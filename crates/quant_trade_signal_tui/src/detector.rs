use crate::types::{IgnitionEvent, PositionDir, Tick, TradeSide, WhaleTrade, WhaleRank, tick_size};
use chrono::{DateTime, Local};
use std::collections::VecDeque;

// ── Whale detector ─────────────────────────────────────────────────────────────

pub struct WhaleDetector;

impl WhaleDetector {
    pub fn check(tick: &Tick, threshold: f64) -> Option<WhaleTrade> {
        if tick.amount >= threshold {
            Some(WhaleTrade {
                rank: WhaleRank::from_amount(tick.amount),
                tick: tick.clone(),
            })
        } else {
            None
        }
    }
}

// ── Ignition detector — dual-direction (做多 + 做空) ──────────────────────────
//
// Bull ignition (多頭點火):  ≥3 whale BUYs within 30s + price rises ≥ 2 ticks
// Bear ignition (空頭點火):  ≥3 whale SELLs within 30s + price drops ≥ 2 ticks
//
// Symmetric logic ensures we detect both long AND short 建倉 opportunities.

pub struct IgnitionDetector {
    /// Sliding window of whale BUY trades (for long/bull ignition)
    bull_window: VecDeque<WhaleTrade>,
    /// Sliding window of whale SELL trades (for short/bear ignition)
    bear_window: VecDeque<WhaleTrade>,
    window_secs: i64,
    min_whales: usize,
    min_tick_moves: f64,
}

impl Default for IgnitionDetector {
    fn default() -> Self {
        Self {
            bull_window: VecDeque::new(),
            bear_window: VecDeque::new(),
            window_secs: 30,
            min_whales: 3,
            min_tick_moves: 2.0,
        }
    }
}

impl IgnitionDetector {
    /// Push a whale trade, evict stale entries, check both directions.
    /// Returns `Some(IgnitionEvent)` with the correct `direction` if triggered.
    /// On the same tick we can only fire one direction to avoid double-alerting.
    pub fn push(&mut self, whale: &WhaleTrade, now: DateTime<Local>) -> Option<IgnitionEvent> {
        self.evict(now);

        match whale.tick.side {
            TradeSide::Buy => {
                self.bull_window.push_back(whale.clone());
                self.check_window(&self.bull_window.clone(), PositionDir::Long, &whale.tick.symbol, now)
            }
            TradeSide::Sell => {
                self.bear_window.push_back(whale.clone());
                self.check_window(&self.bear_window.clone(), PositionDir::Short, &whale.tick.symbol, now)
            }
        }
    }

    fn check_window(
        &mut self,
        window: &VecDeque<WhaleTrade>,
        dir: PositionDir,
        symbol: &str,
        now: DateTime<Local>,
    ) -> Option<IgnitionEvent> {
        if window.len() < self.min_whales {
            return None;
        }

        let prices: Vec<f64> = window.iter().map(|w| w.tick.price).collect();
        let min_p = prices.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_p = prices.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let ts = tick_size(min_p);
        let raw_move = (max_p - min_p) / ts;

        // For long: price must be higher at end than start (rising)
        // For short: price must be lower at end than start (falling)
        let first_price = window.front().unwrap().tick.price;
        let last_price = window.back().unwrap().tick.price;
        let directional_ok = match dir {
            PositionDir::Long => last_price > first_price,
            PositionDir::Short => last_price < first_price,
        };

        if raw_move >= self.min_tick_moves && directional_ok {
            let total_amount_m: f64 = window.iter().map(|w| w.tick.amount_m()).sum();
            let price_move_ticks = match dir {
                PositionDir::Long => raw_move,
                PositionDir::Short => -raw_move, // negative = price fell
            };

            let event = IgnitionEvent {
                symbol: symbol.to_string(),
                timestamp: now,
                whale_count: window.len(),
                price_move_ticks,
                total_amount_m,
                direction: dir.clone(),
            };

            // Clear the fired window to avoid repeat triggers on same burst
            match dir {
                PositionDir::Long => self.bull_window.clear(),
                PositionDir::Short => self.bear_window.clear(),
            }

            return Some(event);
        }

        None
    }

    fn evict(&mut self, now: DateTime<Local>) {
        let secs = self.window_secs;
        self.bull_window.retain(|w| {
            (now - w.tick.timestamp).num_seconds() <= secs
        });
        self.bear_window.retain(|w| {
            (now - w.tick.timestamp).num_seconds() <= secs
        });
    }
}
