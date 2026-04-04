use crate::callback::{OnBidAskStkV1, OnTickStkV1};
use crate::config::SimulationConfig;
use crate::market::book;
use crate::market::generator::StockGenerator;
use crate::types::StockContract;
use rand::prelude::*;
use rand::rngs::StdRng;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// The simulation engine drives all subscribed stock generators
/// and dispatches events to registered callbacks.
pub struct SimulationEngine {
    generators: Vec<StockGenerator>,
    config: SimulationConfig,
    tick_callbacks: Vec<Arc<dyn OnTickStkV1>>,
    bidask_callbacks: Vec<Arc<dyn OnBidAskStkV1>>,
}

impl SimulationEngine {
    pub fn new(contracts: Vec<StockContract>, config: SimulationConfig) -> Self {
        let generators = contracts
            .into_iter()
            .enumerate()
            .map(|(i, c)| StockGenerator::new(c, &config, config.seed + i as u64))
            .collect();

        Self {
            generators,
            config,
            tick_callbacks: Vec::new(),
            bidask_callbacks: Vec::new(),
        }
    }

    pub fn on_tick(&mut self, cb: Arc<dyn OnTickStkV1>) {
        self.tick_callbacks.push(cb);
    }

    pub fn on_bid_ask(&mut self, cb: Arc<dyn OnBidAskStkV1>) {
        self.bidask_callbacks.push(cb);
    }

    /// Run the main simulation loop on the current thread.
    pub fn run(mut self) {
        let mut rng = StdRng::seed_from_u64(self.config.seed + 9999);
        let book_every = self.config.book_every_n_ticks;
        let tick_interval_ms = (1000.0 / self.config.tick_rate / self.config.speed) as u64;

        loop {
            // Pick a random stock to generate the next tick
            let idx = rng.gen_range(0..self.generators.len());
            let gen = &mut self.generators[idx];

            let tick = gen.next_tick();

            // Dispatch to tick callbacks
            for cb in &self.tick_callbacks {
                cb.on_tick(&tick);
            }

            // Periodically emit a book snapshot
            if gen.tick_count() % book_every as u64 == 0 {
                let ba = book::generate_book(gen, &mut rng);
                for cb in &self.bidask_callbacks {
                    cb.on_bid_ask(&ba);
                }
            }

            // Inter-tick delay with jitter
            let jitter = rng.gen_range(0..tick_interval_ms / 2);
            thread::sleep(Duration::from_millis(tick_interval_ms + jitter));
        }
    }
}
