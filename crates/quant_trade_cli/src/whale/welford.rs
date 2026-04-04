use std::collections::VecDeque;

/// Welford online algorithm for O(1) streaming mean and variance.
/// Maintains a sliding window of the last `max_window` values.
pub struct WelfordTracker {
    pub n: u64,
    pub mean: f64,
    m2: f64,
    window: VecDeque<f64>,
    pub max_window: usize,
}

impl WelfordTracker {
    pub fn new(max_window: usize) -> Self {
        Self {
            n: 0,
            mean: 0.0,
            m2: 0.0,
            window: VecDeque::with_capacity(max_window),
            max_window,
        }
    }

    /// Update with a new value. If window is full, remove the oldest first.
    pub fn update(&mut self, x: f64) {
        // Remove oldest if window is full
        if self.window.len() >= self.max_window {
            if let Some(old) = self.window.pop_front() {
                self.remove_value(old);
            }
        }

        // Welford add
        self.n += 1;
        let delta = x - self.mean;
        self.mean += delta / self.n as f64;
        let delta2 = x - self.mean;
        self.m2 += delta * delta2;

        self.window.push_back(x);
    }

    /// Inverse Welford update (remove a value).
    fn remove_value(&mut self, x: f64) {
        if self.n <= 1 {
            self.n = 0;
            self.mean = 0.0;
            self.m2 = 0.0;
            return;
        }
        let delta = x - self.mean;
        self.n -= 1;
        self.mean -= delta / self.n as f64;
        let delta2 = x - self.mean;
        self.m2 -= delta * delta2;
        if self.m2 < 0.0 {
            self.m2 = 0.0;
        }
    }

    pub fn variance(&self) -> f64 {
        if self.n == 0 {
            0.0
        } else {
            self.m2 / self.n as f64
        }
    }

    pub fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }

    pub fn sigma_n(&self, x: f64) -> f64 {
        let sd = self.std_dev();
        if sd == 0.0 {
            return 0.0;
        }
        (x - self.mean) / sd
    }

    /// Number of values in the current window.
    #[allow(dead_code)]
    pub fn window_len(&self) -> usize {
        self.window.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welford_basic() {
        let mut t = WelfordTracker::new(200);
        let values: Vec<f64> = (1..=10).map(|i| i as f64).collect();
        for v in &values {
            t.update(*v);
        }
        assert!((t.mean - 5.5).abs() < 0.01);
        assert!(t.std_dev() > 0.0);
        assert_eq!(t.n, 10);
    }

    #[test]
    fn test_welford_window() {
        let mut t = WelfordTracker::new(5);
        for v in &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0] {
            t.update(*v);
        }
        // Window should have last 5 values: 3,4,5,6,7
        assert_eq!(t.window_len(), 5);
        assert_eq!(t.n, 5);
        assert!((t.mean - 5.0).abs() < 0.01);
    }
}
