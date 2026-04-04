use std::collections::HashMap;

/// Global simulation configuration.
#[derive(Debug, Clone)]
pub struct SimulationConfig {
    /// Virtual clock speed multiplier. 1.0 = real-time, 10.0 = 10x speed.
    pub speed: f64,
    /// Base ticks per second per stock.
    pub tick_rate: f64,
    /// Emit a book snapshot every N ticks per stock.
    pub book_every_n_ticks: u32,
    /// Annualized volatility (default ~0.25).
    pub volatility: f64,
    /// Per-stock volatility overrides.
    pub volatility_by_stock: HashMap<String, f64>,
    /// Whale probability per tick (large trade appears).
    pub whale_probability: f64,
    /// Whale threshold in NTD.
    pub whale_threshold: f64,
    /// RNG seed for reproducibility.
    pub seed: u64,
    /// Markov regime transition matrix [from][to], indexed as Normal=0, WhaleAccum=1, Ignition=2, Distribution=3.
    pub markov: [[f64; 4]; 4],
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            speed: 1.0,
            tick_rate: 3.0,
            book_every_n_ticks: 3,
            volatility: 0.25,
            volatility_by_stock: HashMap::new(),
            whale_probability: 0.05,
            whale_threshold: 5_000_000.0,
            seed: 42,
            markov: [
                // Normal → [Normal, WhaleAccum, Ignition, Distribution]
                [0.92, 0.04, 0.025, 0.015],
                // WhaleAccum →
                [0.08, 0.70, 0.15, 0.07],
                // Ignition →
                [0.30, 0.05, 0.55, 0.10],
                // Distribution →
                [0.10, 0.05, 0.05, 0.80],
            ],
        }
    }
}

impl SimulationConfig {
    pub fn volatility_for(&self, code: &str) -> f64 {
        self.volatility_by_stock.get(code).copied().unwrap_or(self.volatility)
    }

    pub fn whale_threshold_for(&self, code: &str) -> f64 {
        // Lower threshold for cheaper stocks
        let _ = code;
        self.whale_threshold
    }
}
