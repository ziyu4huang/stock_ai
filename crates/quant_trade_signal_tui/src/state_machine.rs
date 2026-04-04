use crate::types::{MarketState, Tick, TradeSide};
use std::collections::VecDeque;

/// Rule-based 6-state market classifier covering both long and short 建倉 paths.
///
/// 做多路徑: Noise → LongAccum → LongIgnition
/// 做空路徑: Noise → ShortDistrib → ShortIgnition
/// 其他:     Neutral (整理), Noise (雜訊)
pub struct StockStateTracker {
    recent_prices: VecDeque<f64>,                         // last 60 ticks
    whale_events: VecDeque<(TradeSide, std::time::Instant)>, // rolling 2 min
    pub current_state: MarketState,
}

impl StockStateTracker {
    pub fn new() -> Self {
        Self {
            recent_prices: VecDeque::with_capacity(60),
            whale_events: VecDeque::new(),
            current_state: MarketState::Noise,
        }
    }

    /// Update with a new tick. `is_whale` flags a qualifying large trade.
    /// Returns the new classified state.
    pub fn update(&mut self, tick: &Tick, is_whale: bool) -> MarketState {
        if self.recent_prices.len() >= 60 {
            self.recent_prices.pop_front();
        }
        self.recent_prices.push_back(tick.price);

        let now = std::time::Instant::now();
        self.whale_events
            .retain(|(_, t)| now.duration_since(*t).as_secs() <= 120);
        if is_whale {
            self.whale_events.push_back((tick.side.clone(), now));
        }

        self.current_state = self.classify(tick.price);
        self.current_state.clone()
    }

    fn classify(&self, price: f64) -> MarketState {
        if self.recent_prices.len() < 10 {
            return MarketState::Noise;
        }

        let min_p = self.recent_prices.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_p = self.recent_prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = max_p - min_p;
        let price_pct = if range < 1e-9 {
            0.5
        } else {
            (price - min_p) / range
        };

        let whale_buys = self.whale_events.iter().filter(|(s, _)| *s == TradeSide::Buy).count() as u32;
        let whale_sells = self.whale_events.iter().filter(|(s, _)| *s == TradeSide::Sell).count() as u32;
        let total_whales = whale_buys + whale_sells;
        let buy_ratio = if total_whales == 0 {
            0.5
        } else {
            whale_buys as f64 / total_whales as f64
        };
        let sell_ratio = 1.0 - buy_ratio;

        let trending_up = self.is_trending_up();
        let trending_down = self.is_trending_down();

        // ── Long 建倉 conditions ──────────────────────────────────────────────
        // LongIgnition: price breakout at mid-high range, dominant whale buys, trending up
        if price_pct > 0.5 && buy_ratio >= 0.70 && trending_up && total_whales >= 3 {
            return MarketState::LongIgnition;
        }
        // LongAccum: price at low range, whale buys quietly accumulating (may not be trending yet)
        if price_pct < 0.35 && buy_ratio >= 0.60 && total_whales >= 2 {
            return MarketState::LongAccum;
        }

        // ── Short 建倉 conditions ─────────────────────────────────────────────
        // ShortIgnition: price breakdown at mid-low range, dominant whale sells, trending down
        if price_pct < 0.5 && sell_ratio >= 0.70 && trending_down && total_whales >= 3 {
            return MarketState::ShortIgnition;
        }
        // ShortDistrib: price at high range, whale sells quietly distributing
        if price_pct > 0.65 && sell_ratio >= 0.60 && total_whales >= 2 {
            return MarketState::ShortDistrib;
        }

        // ── Neutral: mid-range with mixed or no whale activity ────────────────
        if price_pct >= 0.35 && price_pct <= 0.65 {
            return MarketState::Neutral;
        }

        MarketState::Noise
    }

    fn is_trending_up(&self) -> bool {
        self.trend_direction() > 0.0
    }

    fn is_trending_down(&self) -> bool {
        self.trend_direction() < 0.0
    }

    /// Positive = recent prices above older mean, negative = below.
    fn trend_direction(&self) -> f64 {
        let n = self.recent_prices.len();
        if n < 10 {
            return 0.0;
        }
        let recent: Vec<f64> = self.recent_prices.iter().rev().take(5).cloned().collect();
        let older: Vec<f64> = self.recent_prices.iter().rev().skip(5).take(15).cloned().collect();
        if older.is_empty() {
            return 0.0;
        }
        let recent_mean = recent.iter().sum::<f64>() / recent.len() as f64;
        let older_mean = older.iter().sum::<f64>() / older.len() as f64;
        recent_mean - older_mean
    }
}
