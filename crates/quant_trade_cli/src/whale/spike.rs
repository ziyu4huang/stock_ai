use std::collections::HashMap;

use crate::whale::welford::WelfordTracker;
use crate::data::types::VolumeSpike;

/// 3-Sigma volume spike detector. Maintains per-symbol Welford trackers.
pub struct SpikeDetector {
    trackers: HashMap<String, WelfordTracker>,
    threshold: f64,
    window_size: usize,
}

impl SpikeDetector {
    pub fn new(window_size: usize, threshold: f64) -> Self {
        Self {
            trackers: HashMap::new(),
            threshold,
            window_size,
        }
    }

    /// Feed a new volume value. Returns Some(VolumeSpike) if threshold exceeded.
    pub fn check(&mut self, symbol: &str, volume: i64, timestamp: i64) -> Option<VolumeSpike> {
        let tracker = self
            .trackers
            .entry(symbol.to_string())
            .or_insert_with(|| WelfordTracker::new(self.window_size));

        let vol_f = volume as f64;
        let mean = tracker.mean;
        let sigma = tracker.std_dev();
        let z = tracker.sigma_n(vol_f);

        tracker.update(vol_f);

        if z > self.threshold && tracker.n > 10 {
            Some(VolumeSpike {
                symbol: symbol.to_string(),
                timestamp,
                volume,
                mean,
                sigma,
                z_score: z,
            })
        } else {
            None
        }
    }

    /// Feed volume for all symbols (batch). Returns spikes found.
    #[allow(dead_code)]
    pub fn check_batch(&mut self, entries: &[(&str, i64, i64)]) -> Vec<VolumeSpike> {
        let mut spikes = Vec::new();
        for (symbol, volume, timestamp) in entries {
            if let Some(spike) = self.check(symbol, *volume, *timestamp) {
                spikes.push(spike);
            }
        }
        spikes
    }

    /// Reset a symbol's tracker.
    #[allow(dead_code)]
    pub fn reset(&mut self, symbol: &str) {
        self.trackers.remove(symbol);
    }

    /// Get current stats for a symbol.
    #[allow(dead_code)]
    pub fn stats(&self, symbol: &str) -> Option<(f64, f64)> {
        self.trackers.get(symbol).map(|t| (t.mean, t.std_dev()))
    }

    /// Query the latest value against current stats without updating.
    /// Use this to check if the last bar would be a spike after feeding all history.
    pub fn query(&self, symbol: &str, volume: i64, timestamp: i64) -> Option<VolumeSpike> {
        let tracker = self.trackers.get(symbol)?;
        if tracker.n < 10 {
            return None;
        }
        let vol_f = volume as f64;
        let mean = tracker.mean;
        let sigma = tracker.std_dev();
        let z = if sigma > 0.0 { (vol_f - mean) / sigma } else { 0.0 };

        if z > self.threshold {
            Some(VolumeSpike {
                symbol: symbol.to_string(),
                timestamp,
                volume,
                mean,
                sigma,
                z_score: z,
            })
        } else {
            None
        }
    }
}

/// Default: 200-tick window, 3-sigma threshold.
impl Default for SpikeDetector {
    fn default() -> Self {
        Self::new(200, 3.0)
    }
}
