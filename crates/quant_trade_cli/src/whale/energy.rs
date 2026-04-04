use std::collections::VecDeque;

use crate::data::types::{EnergyPulse, PulsePattern};

/// RMS energy pulse analyzer. Maintains a ring buffer of recent bar volumes
/// and classifies the energy envelope pattern.
pub struct EnergyAnalyzer {
    ring: VecDeque<f64>,
    window: usize,
}

impl EnergyAnalyzer {
    pub fn new(window: usize) -> Self {
        Self {
            ring: VecDeque::with_capacity(window),
            window,
        }
    }

    /// Feed a new volume value and classify the energy pattern.
    pub fn analyze(&mut self, volume: f64, symbol: &str, timestamp: i64) -> EnergyPulse {
        if self.ring.len() >= self.window {
            self.ring.pop_front();
        }
        self.ring.push_back(volume);

        let rms = self.compute_rms();
        let pattern = self.classify();
        let confidence = self.compute_confidence(&pattern);

        EnergyPulse {
            symbol: symbol.to_string(),
            timestamp,
            rms_energy: rms,
            pattern,
            confidence,
            window_bars: self.ring.len(),
        }
    }

    fn compute_rms(&self) -> f64 {
        if self.ring.is_empty() {
            return 0.0;
        }
        let sum_sq: f64 = self.ring.iter().map(|v| v * v).sum();
        (sum_sq / self.ring.len() as f64).sqrt()
    }

    /// Classify the energy envelope pattern.
    fn classify(&self) -> PulsePattern {
        if self.ring.len() < 5 {
            return PulsePattern::Normal;
        }

        let volumes: Vec<f64> = self.ring.iter().copied().collect();
        let n = volumes.len();
        let mean = volumes.iter().sum::<f64>() / n as f64;

        // Check for single spike (one dominant value)
        let max_val = volumes.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let sum_excluding_max: f64 = volumes.iter().filter(|&&v| v < max_val * 0.9).sum();
        if max_val > mean * 4.0 && sum_excluding_max < max_val * 0.5 {
            return PulsePattern::Single;
        }

        // Check for staircase (monotonic increase)
        let mut rising_count = 0;
        for i in 1..n {
            if volumes[i] > volumes[i - 1] {
                rising_count += 1;
            }
        }
        if rising_count as f64 > n as f64 * 0.7 && volumes[n - 1] > volumes[0] * 2.0 {
            return PulsePattern::Staircase;
        }

        // Check for accumulation (sustained moderate with low variance)
        let variance = {
            let sum_sq: f64 = volumes.iter().map(|v| (v - mean).powi(2)).sum();
            sum_sq / n as f64
        };
        let cv = (variance.sqrt()) / mean.max(1.0); // coefficient of variation
        if cv < 0.3 && mean > volumes.iter().sum::<f64>() / n as f64 * 0.8 {
            return PulsePattern::Accumulation;
        }

        // Check for distribution (high energy declining)
        let mut falling_count = 0;
        for i in 1..n {
            if volumes[i] < volumes[i - 1] {
                falling_count += 1;
            }
        }
        let first_half_mean: f64 = volumes[..n / 2].iter().sum::<f64>() / (n / 2) as f64;
        let second_half_mean: f64 = volumes[n / 2..].iter().sum::<f64>() / (n - n / 2) as f64;
        if falling_count as f64 > n as f64 * 0.6 && first_half_mean > second_half_mean * 1.5 {
            return PulsePattern::Distribution;
        }

        PulsePattern::Normal
    }

    fn compute_confidence(&self, pattern: &PulsePattern) -> f64 {
        if self.ring.len() < 5 {
            return 0.0;
        }
        match pattern {
            PulsePattern::Normal => 0.0,
            _ => {
                let fullness = self.ring.len() as f64 / self.window as f64;
                (fullness * 0.8).min(1.0)
            }
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.ring.clear();
    }
}

impl Default for EnergyAnalyzer {
    fn default() -> Self {
        Self::new(20)
    }
}
