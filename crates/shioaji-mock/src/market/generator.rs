use crate::config::SimulationConfig;
use crate::market::twse::{align_lot, clamp_price, round_to_tick};
use crate::types::*;
use chrono::Local;
use rand::prelude::*;
use rand::rngs::StdRng;

/// Per-stock simulation generator. Produces TickSTKv1 events using:
/// - Mean-reverting (Ornstein-Uhlenbeck) price process
/// - Power-law volume distribution
/// - Markov regime switching
pub struct StockGenerator {
    contract: StockContract,
    rng: StdRng,

    // Price state
    price: f64,
    open_price: f64,
    high: f64,
    low: f64,

    // Accumulators
    total_volume: u64,
    total_amount: f64,
    bid_side_total_vol: u64,
    ask_side_total_vol: u64,
    tick_count: u64,

    // OU process state (mean-reversion)
    ou_x: f64, // current OU deviation from mean

    // Regime
    regime: MarketRegime,
    regime_ticks_left: u32,

    // Config references
    volatility: f64,
    whale_threshold: f64,
    markov: [[f64; 4]; 4],
}

impl StockGenerator {
    pub fn new(contract: StockContract, config: &SimulationConfig, seed: u64) -> Self {
        let price = contract.reference_price;
        let volatility = config.volatility_for(&contract.code);
        let whale_threshold = config.whale_threshold_for(&contract.code);
        let markov = config.markov;
        Self {
            contract,
            rng: StdRng::seed_from_u64(seed),
            price,
            open_price: 0.0,
            high: price,
            low: price,
            total_volume: 0,
            total_amount: 0.0,
            bid_side_total_vol: 0,
            ask_side_total_vol: 0,
            tick_count: 0,
            ou_x: 0.0,
            regime: MarketRegime::Normal,
            regime_ticks_left: 30,
            volatility,
            whale_threshold,
            markov,
        }
    }

    pub fn code(&self) -> &str {
        &self.contract.code
    }

    pub fn regime(&self) -> MarketRegime {
        self.regime
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn tick_count(&self) -> u64 {
        self.tick_count
    }

    /// Generate the next tick event.
    pub fn next_tick(&mut self) -> TickSTKv1 {
        // Maybe switch regime
        if self.regime_ticks_left == 0 {
            self.regime = self.sample_regime();
            self.regime_ticks_left = match self.regime {
                MarketRegime::Normal => self.rng.gen_range(30..100),
                MarketRegime::WhaleAccum => self.rng.gen_range(10..30),
                MarketRegime::Ignition => self.rng.gen_range(5..12),
                MarketRegime::Distribution => self.rng.gen_range(10..25),
            };
        }
        self.regime_ticks_left -= 1;

        // OU price step: dx = theta * (0 - x) * dt + sigma * dW
        // theta = mean reversion speed, sigma = vol * sqrt(dt)
        let dt: f64 = 1.0_f64 / 252.0 / 390.0; // one tick in trading days (~390 min/day, ~3 ticks/min)
        let theta = 5.0; // mean reversion speed
        let sigma: f64 = self.volatility * dt.sqrt();
        let d_w: f64 = self.rng.gen_range(-1.0..1.0); // approx N(0,1) via uniform
        self.ou_x += theta * (-self.ou_x) * dt + sigma * d_w;

        // Regime bias on price
        let regime_drift = match self.regime {
            MarketRegime::Normal => 0.0,
            MarketRegime::WhaleAccum => sigma * 0.5,   // slight upward bias
            MarketRegime::Ignition => sigma * 2.0,      // strong upward
            MarketRegime::Distribution => -sigma * 1.0, // moderate downward
        };

        let raw_price = self.contract.reference_price * (1.0 + self.ou_x + regime_drift);
        let new_price = clamp_price(round_to_tick(raw_price), self.contract.reference_price);
        self.price = new_price.max(self.contract.limit_down);

        // First tick sets open
        if self.tick_count == 0 {
            self.open_price = self.price;
        }
        self.high = self.high.max(self.price);
        self.low = self.low.min(self.price);

        // Volume: power-law distribution for realistic trade sizes
        let shares = self.gen_volume();
        let amount = self.price * shares as f64;

        // Tick type (buy/sell)
        let tick_type = self.gen_side();
        match tick_type {
            TickType::Buy => self.bid_side_total_vol += shares,
            TickType::Sell => self.ask_side_total_vol += shares,
            TickType::Unknown => {}
        }

        self.total_volume += shares;
        self.total_amount += amount;
        self.tick_count += 1;

        let avg_price = if self.total_volume > 0 {
            self.total_amount / self.total_volume as f64
        } else {
            self.price
        };

        let price_chg = self.price - self.contract.reference_price;
        let pct_chg = if self.contract.reference_price > 0.0 {
            price_chg / self.contract.reference_price * 100.0
        } else {
            0.0
        };

        let chg_type = if self.tick_count <= 1 {
            ChangeType::Open
        } else if price_chg > 0.0 {
            ChangeType::Up
        } else if price_chg < 0.0 {
            ChangeType::Down
        } else {
            ChangeType::Unchanged
        };

        TickSTKv1 {
            code: self.contract.code.clone(),
            datetime: Local::now(),
            open: self.open_price,
            close: self.price,
            high: self.high,
            low: self.low,
            volume: shares,
            total_volume: self.total_volume,
            amount,
            total_amount: self.total_amount,
            bid_side_total_vol: self.bid_side_total_vol,
            ask_side_total_vol: self.ask_side_total_vol,
            avg_price,
            tick_type,
            chg_type,
            price_chg,
            pct_chg,
            simtrade: true,
        }
    }

    /// Power-law volume distribution: P(X > x) ~ x^(-alpha).
    /// Produces mostly small trades with a heavy tail of large (whale) trades.
    fn gen_volume(&mut self) -> u64 {
        let base_shares = self.power_law_sample(1.5); // alpha = 1.5
        let shares = align_lot((base_shares * 1000.0) as u64);

        // In whale/ignition modes, occasionally inject extra-large trades
        match self.regime {
            MarketRegime::WhaleAccum | MarketRegime::Distribution => {
                if self.rng.gen_bool(0.15) {
                    let whale_shares = ((self.whale_threshold / self.price) as u64 / 1000 + 1) * 1000;
                    let multiplier = self.rng.gen_range(1u64..4);
                    return align_lot(whale_shares * multiplier);
                }
            }
            MarketRegime::Ignition => {
                if self.rng.gen_bool(0.3) {
                    let whale_shares = ((self.whale_threshold / self.price) as u64 / 1000 + 1) * 1000;
                    let multiplier = self.rng.gen_range(2u64..6);
                    return align_lot(whale_shares * multiplier);
                }
            }
            _ => {}
        }

        shares.max(1000)
    }

    /// Sample from Pareto distribution: X ~ x_min * U^(-1/alpha).
    fn power_law_sample(&mut self, alpha: f64) -> f64 {
        let u: f64 = self.rng.gen(); // (0, 1]
        let u = u.max(0.001); // avoid infinity
        u.powf(-1.0 / alpha)
    }

    /// Generate buy/sell side based on regime.
    fn gen_side(&mut self) -> TickType {
        let buy_prob = match self.regime {
            MarketRegime::Normal => 0.50,
            MarketRegime::WhaleAccum => 0.70,
            MarketRegime::Ignition => 0.85,
            MarketRegime::Distribution => 0.25,
        };
        if self.rng.gen_bool(buy_prob) {
            TickType::Buy
        } else {
            TickType::Sell
        }
    }

    /// Markov regime transition.
    fn sample_regime(&mut self) -> MarketRegime {
        let from = match self.regime {
            MarketRegime::Normal => 0,
            MarketRegime::WhaleAccum => 1,
            MarketRegime::Ignition => 2,
            MarketRegime::Distribution => 3,
        };
        let r: f64 = self.rng.gen();
        let row = self.markov[from];
        let mut cum = 0.0;
        for (i, &p) in row.iter().enumerate() {
            cum += p;
            if r < cum {
                return match i {
                    0 => MarketRegime::Normal,
                    1 => MarketRegime::WhaleAccum,
                    2 => MarketRegime::Ignition,
                    3 => MarketRegime::Distribution,
                    _ => MarketRegime::Normal,
                };
            }
        }
        MarketRegime::Normal
    }
}
