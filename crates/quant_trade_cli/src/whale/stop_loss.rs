use crate::data::types::{StopLossState, StopLossTrigger};
use stock_core::SignalDirection;

/// Tick size table for Taiwan stocks (price → tick size in NTD).
#[allow(dead_code)]
pub fn tick_size(price: f64) -> f64 {
    if price < 10.0 { 0.01 }
    else if price < 50.0 { 0.05 }
    else if price < 100.0 { 0.1 }
    else if price < 500.0 { 0.5 }
    else if price < 1000.0 { 1.0 }
    else { 5.0 }
}

/// Check all three stop-loss rules simultaneously.
#[allow(dead_code)]
pub fn check_stop_loss(state: &StopLossState, current_price: f64, current_time: i64) -> StopLossTrigger {
    let ts = tick_size(current_price);

    // Rule 1: 2-tick reverse
    match state.direction {
        SignalDirection::Buy => {
            if current_price <= state.entry_price - 2.0 * ts {
                return StopLossTrigger::TickReverse;
            }
        }
        SignalDirection::Sell => {
            if current_price >= state.entry_price + 2.0 * ts {
                return StopLossTrigger::TickReverse;
            }
        }
        _ => {}
    }

    // Rule 2: 15-minute timeout
    if current_time - state.entry_time >= 900 {
        let favorable = match state.direction {
            SignalDirection::Buy => current_price - state.entry_price,
            SignalDirection::Sell => state.entry_price - current_price,
            _ => 0.0,
        };
        if favorable < ts {
            return StopLossTrigger::Timeout;
        }
    }

    // Rule 3: 0.5% trailing stop
    match state.direction {
        SignalDirection::Buy => {
            if state.max_favorable > state.entry_price * 1.005 {
                let trail_stop = state.max_favorable * 0.995;
                if current_price <= trail_stop {
                    return StopLossTrigger::Trailing;
                }
            }
        }
        SignalDirection::Sell => {
            if state.max_favorable < state.entry_price * 0.995 {
                let trail_stop = state.max_favorable * 1.005;
                if current_price >= trail_stop {
                    return StopLossTrigger::Trailing;
                }
            }
        }
        _ => {}
    }

    StopLossTrigger::None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_reverse_buy() {
        let state = StopLossState {
            entry_price: 100.0,
            entry_time: 0,
            direction: SignalDirection::Buy,
            max_favorable: 100.0,
        };
        let result = check_stop_loss(&state, 99.7, 100);
        assert_eq!(result, StopLossTrigger::TickReverse);
    }

    #[test]
    fn test_no_trigger() {
        let state = StopLossState {
            entry_price: 100.0,
            entry_time: 0,
            direction: SignalDirection::Buy,
            max_favorable: 100.0,
        };
        let result = check_stop_loss(&state, 100.5, 100);
        assert_eq!(result, StopLossTrigger::None);
    }
}
