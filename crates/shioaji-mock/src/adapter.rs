use crate::config::SimulationConfig;
use crate::engine::SimulationEngine;
use crate::types::{StockContract, STOCKS};
use std::sync::{Arc, mpsc};
use std::thread;

/// Shioaji-native event emitted by the simulation.
#[derive(Debug, Clone)]
pub enum ShioajiEvent {
    Tick(crate::types::TickSTKv1),
    BidAsk(crate::types::BidAskSTKv1),
}

struct EventForwarder {
    tx: mpsc::Sender<ShioajiEvent>,
}

impl crate::callback::OnTickStkV1 for EventForwarder {
    fn on_tick(&self, tick: &crate::types::TickSTKv1) {
        let _ = self.tx.send(ShioajiEvent::Tick(tick.clone()));
    }
}

impl crate::callback::OnBidAskStkV1 for EventForwarder {
    fn on_bid_ask(&self, ba: &crate::types::BidAskSTKv1) {
        let _ = self.tx.send(ShioajiEvent::BidAsk(ba.clone()));
    }
}

/// Spawn the simulation engine on a background thread.
/// Returns a receiver of `ShioajiEvent` for the caller to convert.
pub fn spawn() -> mpsc::Receiver<ShioajiEvent> {
    spawn_with_config(SimulationConfig::default())
}

/// Spawn with custom configuration.
pub fn spawn_with_config(config: SimulationConfig) -> mpsc::Receiver<ShioajiEvent> {
    let (tx, rx) = mpsc::channel::<ShioajiEvent>();

    let contracts: Vec<StockContract> = STOCKS
        .iter()
        .map(|(code, name, price)| StockContract::tse(code, name, *price))
        .collect();

    let mut engine = SimulationEngine::new(contracts, config);

    let forwarder = Arc::new(EventForwarder { tx });
    engine.on_tick(forwarder.clone());
    engine.on_bid_ask(forwarder);

    thread::spawn(move || engine.run());

    rx
}
