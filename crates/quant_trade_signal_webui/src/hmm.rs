//! Rust-native 4-state GaussianHMM for market regime detection.
//!
//! States: Accumulation, Ignition, Distribution, Noise
//! Observations: [log_return, whale_imbalance, volume_zscore, price_position]

use std::collections::VecDeque;

// ── Constants ──────────────────────────────────────────────────────────────────

const N_STATES: usize = 4;
const N_FEATURES: usize = 4;
const OBS_WINDOW: usize = 20; // Viterbi runs on last N observations
const BATCH_INTERVAL: u32 = 5; // run HMM every K ticks
const PRICE_WINDOW: usize = 60;

// State indices
const ACCUM: usize = 0;
const IGNIT: usize = 1;
const DISTR: usize = 2;
const NOISE: usize = 3;

// ── HMM Model ──────────────────────────────────────────────────────────────────

/// 4-state Gaussian HMM with hand-tuned emission parameters.
pub struct HmmModel {
    prior: [f64; N_STATES],
    trans: [[f64; N_STATES]; N_STATES],
    means: [[f64; N_FEATURES]; N_STATES],
    /// Diagonal covariance (variances only) for simplicity and numerical stability.
    vars: [[f64; N_FEATURES]; N_STATES],
}

impl HmmModel {
    /// Domain-knowledge hand-tuned model for Taiwan market microstructure.
    ///
    /// Accumulation: low returns (+), positive whale imbalance, moderate volume, lower price range
    /// Ignition:     positive returns, strong positive whale imbalance, high volume, mid-upper range
    /// Distribution: near-zero returns, negative whale imbalance, high volume, upper range
    /// Noise:        near-zero returns, neutral imbalance, normal volume, mid range
    pub fn default_tuned() -> Self {
        let prior = [0.4, 0.1, 0.1, 0.4];

        // Transition matrix — high self-loop, occasional jumps
        // Accum stays put, can transition to Ignition or Noise
        // Ignition is transient, likely to Noise after
        // Distribution is transient, likely to Noise after
        // Noise is sticky
        let trans = [
            [0.60, 0.20, 0.05, 0.15], // from Accum
            [0.15, 0.30, 0.05, 0.50], // from Ignition
            [0.05, 0.05, 0.30, 0.60], // from Distribution
            [0.20, 0.05, 0.10, 0.65], // from Noise
        ];

        // Emission means [log_return, whale_imbalance, volume_zscore, price_position]
        let means = [
            [0.0001, 0.3, 0.5, 0.25], // Accum: slight positive return, positive whale imbalance, moderate volume, low price range
            [0.002, 0.7, 2.0, 0.70],  // Ignition: positive return, strong buy imbalance, high volume, upper range
            [-0.0005, -0.5, 1.5, 0.80], // Distribution: slight negative, strong sell imbalance, high volume, high range
            [0.0, 0.0, 0.0, 0.50],   // Noise: flat, neutral, normal volume, mid range
        ];

        // Diagonal variances (wider = more tolerant)
        let vars = [
            [0.0001, 0.15, 0.8, 0.03], // Accum
            [0.002, 0.10, 1.0, 0.02],  // Ignition
            [0.001, 0.12, 1.0, 0.02],  // Distribution
            [0.0003, 0.20, 0.8, 0.06], // Noise
        ];

        Self { prior, trans, means, vars }
    }

    /// Run Viterbi on an observation sequence, return most likely state indices.
    pub fn viterbi(&self, obs: &[Observation]) -> Vec<usize> {
        let t = obs.len();
        if t == 0 { return vec![]; }

        let mut trellis = vec![[0.0f64; N_STATES]; t];
        let mut backptr = vec![[0usize; N_STATES]; t];

        // Initialize
        for s in 0..N_STATES {
            trellis[0][s] = self.prior[s].ln() + self.emit_ln(s, &obs[0]);
        }

        // Recurse
        for ti in 1..t {
            for s in 0..N_STATES {
                let (best_prev, best_val) = (0..N_STATES)
                    .map(|ps| (ps, trellis[ti - 1][ps] + self.trans[ps][s].ln()))
                    .fold((0usize, f64::NEG_INFINITY), |acc, (ps, val)| {
                        if val > acc.1 { (ps, val) } else { acc }
                    });
                trellis[ti][s] = best_val + self.emit_ln(s, &obs[ti]);
                backptr[ti][s] = best_prev;
            }
        }

        // Backtrace
        let mut path = vec![0usize; t];
        let (best_last, _) = (0..N_STATES)
            .map(|s| (s, trellis[t - 1][s]))
            .fold((0usize, f64::NEG_INFINITY), |acc, (s, v)| {
                if v > acc.1 { (s, v) } else { acc }
            });
        path[t - 1] = best_last;
        for ti in (0..t - 1).rev() {
            path[ti] = backptr[ti + 1][path[ti + 1]];
        }
        path
    }

    /// Log-likelihood of emitting observation `o` from state `s`.
    /// Uses diagonal Gaussian: sum of per-feature log-N(x; mu, var).
    fn emit_ln(&self, s: usize, o: &Observation) -> f64 {
        let mut ll = 0.0;
        for f in 0..N_FEATURES {
            let diff = o[f] - self.means[s][f];
            let var = self.vars[s][f];
            ll += -0.5 * (2.0 * std::f64::consts::PI * var).ln() - diff * diff / (2.0 * var);
        }
        ll
    }
}

// ── Observation ────────────────────────────────────────────────────────────────

/// 4-dimensional observation vector.
pub type Observation = [f64; N_FEATURES];

// ── Observation Builder ────────────────────────────────────────────────────────

/// Accumulates tick data and produces HMM observation vectors every BATCH_INTERVAL ticks.
pub struct HmmObservationBuilder {
    prices: VecDeque<f64>,
    volumes: VecDeque<f64>,
    whale_buys: u32,
    whale_sells: u32,
    tick_count: u32,
    /// Rolling observation history for Viterbi window.
    obs_history: VecDeque<Observation>,
    /// Welford online stats for volume z-score.
    vol_count: f64,
    vol_mean: f64,
    vol_m2: f64,
}

impl HmmObservationBuilder {
    pub fn new() -> Self {
        Self {
            prices: VecDeque::with_capacity(PRICE_WINDOW),
            volumes: VecDeque::with_capacity(PRICE_WINDOW),
            whale_buys: 0,
            whale_sells: 0,
            tick_count: 0,
            obs_history: VecDeque::with_capacity(OBS_WINDOW),
            vol_count: 0.0,
            vol_mean: 0.0,
            vol_m2: 0.0,
        }
    }

    /// Push a tick. Returns `Some(observation)` every BATCH_INTERVAL ticks.
    pub fn push_tick(
        &mut self,
        price: f64,
        volume: f64,
        is_whale_buy: bool,
        is_whale_sell: bool,
    ) -> Option<Observation> {
        // Update Welford for volume
        self.vol_count += 1.0;
        let delta = volume - self.vol_mean;
        self.vol_mean += delta / self.vol_count;
        let delta2 = volume - self.vol_mean;
        self.vol_m2 += delta * delta2;

        if self.prices.len() >= PRICE_WINDOW {
            self.prices.pop_front();
        }
        self.prices.push_back(price);

        if self.volumes.len() >= PRICE_WINDOW {
            self.volumes.pop_front();
        }
        self.volumes.push_back(volume);

        if is_whale_buy { self.whale_buys += 1; }
        if is_whale_sell { self.whale_sells += 1; }

        self.tick_count += 1;
        if self.tick_count % BATCH_INTERVAL != 0 {
            return None;
        }

        let obs = self.build_observation();

        if self.obs_history.len() >= OBS_WINDOW {
            self.obs_history.pop_front();
        }
        self.obs_history.push_back(obs);

        // Reset per-batch whale counts
        self.whale_buys = 0;
        self.whale_sells = 0;

        Some(obs)
    }

    fn build_observation(&self) -> Observation {
        // 1. log_return
        let log_return = if self.prices.len() >= 2 {
            let prev = *self.prices.iter().rev().nth(1).unwrap();
            let curr = *self.prices.back().unwrap();
            if prev > 0.0 { (curr / prev).ln() } else { 0.0 }
        } else {
            0.0
        };

        // 2. whale_imbalance: [-1, +1]
        let total_whales = self.whale_buys + self.whale_sells;
        let whale_imbalance = if total_whales > 0 {
            (self.whale_buys as f64 - self.whale_sells as f64) / total_whales as f64
        } else {
            0.0
        };

        // 3. volume_zscore
        let vol_std = if self.vol_count >= 2.0 {
            (self.vol_m2 / (self.vol_count - 1.0)).sqrt()
        } else {
            1.0
        };
        let vol_zscore = if vol_std > 1e-9 {
            (*self.volumes.back().unwrap_or(&0.0) - self.vol_mean) / vol_std
        } else {
            0.0
        };

        // 4. price_position: [0, 1]
        let price_position = {
            if self.prices.is_empty() {
                0.5
            } else {
                let curr = *self.prices.back().unwrap();
                let min = self.prices.iter().cloned().fold(f64::INFINITY, f64::min);
                let max = self.prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let range = max - min;
                if range < 1e-9 { 0.5 } else { (curr - min) / range }
            }
        };

        [log_return, whale_imbalance, vol_zscore, price_position]
    }

    /// Get the observation history for Viterbi decoding.
    pub fn obs_history(&self) -> &VecDeque<Observation> {
        &self.obs_history
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_model_valid_probabilities() {
        let model = HmmModel::default_tuned();
        // Prior sums to ~1
        let prior_sum: f64 = model.prior.iter().sum();
        assert!((prior_sum - 1.0).abs() < 0.01, "prior sum = {}", prior_sum);
        // Each transition row sums to ~1
        for row in &model.trans {
            let row_sum: f64 = row.iter().sum();
            assert!((row_sum - 1.0).abs() < 0.01, "trans row sum = {}", row_sum);
        }
        // Variances are positive
        for s in 0..N_STATES {
            for f in 0..N_FEATURES {
                assert!(model.vars[s][f] > 0.0, "var[{}][{}] = {}", s, f, model.vars[s][f]);
            }
        }
    }

    #[test]
    fn test_viterbi_single_step() {
        let model = HmmModel::default_tuned();
        // A noise-like observation
        let obs = [[0.0, 0.0, 0.0, 0.5]];
        let path = model.viterbi(&obs);
        assert_eq!(path.len(), 1);
        // Should be Accum or Noise (both have high prior)
        assert!(path[0] == ACCUM || path[0] == NOISE, "expected Accum/Noise, got {}", path[0]);
    }

    #[test]
    fn test_viterbi_ignition_sequence() {
        let model = HmmModel::default_tuned();
        // Simulate ignition: rising prices, strong whale buys, high volume, upper range
        let obs: Vec<Observation> = (0..10)
            .map(|i| {
                let progress = i as f64 / 10.0;
                [
                    0.003 + progress * 0.001, // rising returns
                    0.8,                       // strong buy imbalance
                    2.5,                       // high volume zscore
                    0.6 + progress * 0.03,     // rising through upper range
                ]
            })
            .collect();
        let path = model.viterbi(&obs);
        // At least the last few states should be Ignition
        let ignition_count = path.iter().rev().take(5).filter(|&&s| s == IGNIT).count();
        assert!(ignition_count >= 2, "expected ignition states at end, got {:?}", path);
    }

    #[test]
    fn test_observation_builder_batch_interval() {
        let mut builder = HmmObservationBuilder::new();
        let mut obs_count = 0;
        for i in 0..20 {
            let price = 100.0 + i as f64 * 0.5;
            if builder.push_tick(price, 5000.0, i % 3 == 0, false).is_some() {
                obs_count += 1;
            }
        }
        // Should produce observations at ticks 5, 10, 15, 20
        assert_eq!(obs_count, 4, "expected 4 observations, got {}", obs_count);
    }

    #[test]
    fn test_observation_builder_features() {
        let mut builder = HmmObservationBuilder::new();
        let mut last_obs = None;
        // Push 5 ticks with rising prices, whale buys
        for i in 0..5 {
            let price = 100.0 + i as f64;
            let is_whale_buy = i >= 3;
            if let Some(obs) = builder.push_tick(price, 10000.0, is_whale_buy, false) {
                last_obs = Some(obs);
            }
        }
        let obs = last_obs.expect("should have produced an observation at tick 5");
        // log_return should be positive (prices rising)
        assert!(obs[0] > 0.0, "log_return = {}", obs[0]);
        // whale_imbalance should be positive (2 buys at ticks 3,4)
        assert!(obs[1] > 0.0, "whale_imbalance = {}", obs[1]);
        // price_position should be high (latest is highest)
        assert!(obs[3] > 0.5, "price_position = {}", obs[3]);
    }
}
