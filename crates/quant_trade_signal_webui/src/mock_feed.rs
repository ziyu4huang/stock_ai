use crate::types::{FeedEvent, OrderBook, OrderLevel, Tick, TradeSide, STOCKS, tick_size};
use chrono::Local;
use rand::prelude::*;
use rand::rngs::StdRng;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

// ── Simulation modes ──────────────────────────────────────────────────────────

#[derive(Clone, Copy)]
enum SimMode { Normal, WhaleAccum, Ignition, Distribution }

struct StockSim {
    symbol: &'static str,
    price: f64,
    whale_threshold: f64,
    mode: SimMode,
    mode_ticks_left: u32,
    ticks_since_book: u32,
    rng: StdRng,
}

impl StockSim {
    fn new(symbol: &'static str, base_price: f64, whale_threshold: f64, seed: u64) -> Self {
        Self {
            symbol,
            price: base_price,
            whale_threshold,
            mode: SimMode::Normal,
            mode_ticks_left: 30,
            ticks_since_book: 0,
            rng: StdRng::seed_from_u64(seed),
        }
    }

    /// Generate next tick + optionally an order book snapshot
    fn next_events(&mut self) -> Vec<FeedEvent> {
        if self.mode_ticks_left == 0 {
            self.mode = self.pick_mode();
            self.mode_ticks_left = match self.mode {
                SimMode::Normal => self.rng.gen_range(20..80),
                SimMode::WhaleAccum => self.rng.gen_range(5..15),
                SimMode::Ignition => self.rng.gen_range(3..8),
                SimMode::Distribution => self.rng.gen_range(5..15),
            };
        }
        self.mode_ticks_left -= 1;

        let ts = tick_size(self.price);
        let (shares, side, ticks_move) = match self.mode {
            SimMode::Normal => {
                let mv: i32 = self.rng.gen_range(-2..=2);
                let sh = self.rng.gen_range(1_000_u64..20_000);
                let side = if self.rng.gen_bool(0.5) { TradeSide::Buy } else { TradeSide::Sell };
                (sh, side, mv)
            }
            SimMode::WhaleAccum => {
                let mv: i32 = self.rng.gen_range(-1..=1);
                let is_whale = self.rng.gen_bool(0.4);
                let min = (self.whale_threshold / self.price) as u64 + 1_000;
                let sh = if is_whale { self.rng.gen_range(min..min * 3) } else { self.rng.gen_range(1_000..10_000) };
                (sh, TradeSide::Buy, mv)
            }
            SimMode::Ignition => {
                let mv: i32 = self.rng.gen_range(1..=3);
                let min = (self.whale_threshold / self.price) as u64 + 1_000;
                let sh = self.rng.gen_range(min..min * 5);
                (sh, TradeSide::Buy, mv)
            }
            SimMode::Distribution => {
                let mv: i32 = self.rng.gen_range(-1..=0);
                let is_whale = self.rng.gen_bool(0.3);
                let min = (self.whale_threshold / self.price) as u64 + 1_000;
                let sh = if is_whale { self.rng.gen_range(min..min * 2) } else { self.rng.gen_range(1_000..15_000) };
                (sh, TradeSide::Sell, mv)
            }
        };

        self.price = ((self.price + ticks_move as f64 * ts) / ts).round() * ts;
        self.price = self.price.max(10.0);

        let tick = Tick {
            symbol: self.symbol.to_string(),
            timestamp: Local::now(),
            price: self.price,
            shares,
            side: side.clone(),
            amount: self.price * shares as f64,
        };

        let mut events = vec![FeedEvent::Tick(tick.clone())];

        // Emit a book snapshot roughly every 3 ticks
        self.ticks_since_book += 1;
        if self.ticks_since_book >= 3 {
            self.ticks_since_book = 0;
            events.push(FeedEvent::Book(self.generate_book(&tick)));
        }

        events
    }

    fn generate_book(&mut self, tick: &Tick) -> OrderBook {
        let ts = tick_size(tick.price);
        let base = tick.price;

        // Max volume scale for context (helps visualize bar proportions)
        let max_lots: u64 = 5000;

        // asks[0] = best ask (lowest) = base + 1 tick if buy, base if sell
        let ask_base = match tick.side {
            TradeSide::Buy => base + ts,
            TradeSide::Sell => base,
        };
        let bid_base = match tick.side {
            TradeSide::Buy => base,
            TradeSide::Sell => base - ts,
        };

        let mut asks = [OrderLevel { price: 0.0, lots: 0 }; 5];
        for i in 0..5 {
            let price = ((ask_base + i as f64 * ts) / ts).round() * ts;
            // Lots decrease away from best ask, with some randomness
            let scale = (5 - i) as f64 / 5.0;
            let lots = (self.rng.gen_range(800_u64..max_lots) as f64 * scale) as u64;
            asks[i] = OrderLevel { price, lots: lots.max(50) };
        }

        let mut bids = [OrderLevel { price: 0.0, lots: 0 }; 5];
        for i in 0..5 {
            let price = ((bid_base - i as f64 * ts) / ts).round() * ts;
            let scale = (5 - i) as f64 / 5.0;
            let lots = (self.rng.gen_range(800_u64..max_lots) as f64 * scale) as u64;
            bids[i] = OrderLevel { price, lots: lots.max(50) };
        }

        // In ignition mode, inflate best bid volume (whale parked order)
        if matches!(self.mode, SimMode::Ignition) {
            bids[0].lots = self.rng.gen_range(3000_u64..8000);
        }
        // In distribution mode, inflate best ask volume
        if matches!(self.mode, SimMode::Distribution) {
            asks[0].lots = self.rng.gen_range(3000_u64..8000);
        }

        OrderBook {
            symbol: tick.symbol.clone(),
            timestamp: tick.timestamp,
            asks,
            bids,
            last_price: tick.price,
            last_side: tick.side.clone(),
        }
    }

    fn pick_mode(&mut self) -> SimMode {
        let r: f64 = self.rng.gen();
        if r < 0.60 { SimMode::Normal }
        else if r < 0.75 { SimMode::WhaleAccum }
        else if r < 0.88 { SimMode::Ignition }
        else { SimMode::Distribution }
    }
}

// ── Public entry point ────────────────────────────────────────────────────────

pub fn spawn(tx: Sender<FeedEvent>) {
    thread::spawn(move || {
        let mut sims: Vec<StockSim> = STOCKS
            .iter()
            .enumerate()
            .map(|(i, s)| StockSim::new(s.symbol, s.base_price, s.whale_threshold, 42 + i as u64))
            .collect();

        let mut rng = StdRng::seed_from_u64(999);

        loop {
            let idx = rng.gen_range(0..sims.len());
            for event in sims[idx].next_events() {
                if tx.send(event).is_err() {
                    return;
                }
            }
            let delay_ms = rng.gen_range(100u64..380);
            thread::sleep(Duration::from_millis(delay_ms));
        }
    });
}
