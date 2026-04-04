use crate::types::{BidAskSTKv1, TickSTKv1};

/// Callback trait for tick events (mirrors Shioaji `@api.on_tick_stk_v1`).
pub trait OnTickStkV1: Send + Sync {
    fn on_tick(&self, tick: &TickSTKv1);
}

/// Callback trait for bid/ask events (mirrors Shioaji `@api.on_bid_ask_stk_v1`).
pub trait OnBidAskStkV1: Send + Sync {
    fn on_bid_ask(&self, ba: &BidAskSTKv1);
}
