use crate::market::generator::StockGenerator;
use crate::market::twse::{round_to_tick, tick_size};
use crate::types::*;
use chrono::Local;
use rand::prelude::*;

/// Generate a 5-level order book consistent with the current tick and regime.
pub fn generate_book(gen: &StockGenerator, rng: &mut impl Rng) -> BidAskSTKv1 {
    let price = gen.price();
    let ts = tick_size(price);
    let regime = gen.regime();

    // Best bid/ask placement
    let (bid_base, ask_base) = (price, price + ts);

    // Base depth decays with distance from best
    let base_depth = match regime {
        MarketRegime::Normal => 800u64..4000,
        MarketRegime::WhaleAccum => 1500..6000,
        MarketRegime::Ignition => 2000..8000,
        MarketRegime::Distribution => 1000..5000,
    };

    let mut bid_price = [0.0f64; 5];
    let mut bid_volume = [0u64; 5];
    let mut ask_price = [0.0f64; 5];
    let mut ask_volume = [0u64; 5];

    for i in 0..5 {
        let decay = (5 - i) as f64 / 5.0;
        let base = rng.gen_range(base_depth.clone()) as f64;
        let lots = ((base * decay) as u64 / 1000 + 1) * 1000; // align to lots

        bid_price[i] = round_to_tick(bid_base - i as f64 * ts);
        bid_volume[i] = lots;

        ask_price[i] = round_to_tick(ask_base + i as f64 * ts);
        ask_volume[i] = lots;
    }

    // Regime-specific book adjustments
    match regime {
        MarketRegime::WhaleAccum => {
            // Inflated best bid (large resting buy order)
            bid_volume[0] = rng.gen_range(3000u64..10000) * 1000;
        }
        MarketRegime::Ignition => {
            // Thin ask side (being eaten through), fat bid side
            bid_volume[0] = rng.gen_range(5000u64..15000) * 1000;
            for v in ask_volume.iter_mut() {
                *v = (*v / 3).max(1000);
            }
        }
        MarketRegime::Distribution => {
            // Inflated best ask (large resting sell order)
            ask_volume[0] = rng.gen_range(3000u64..10000) * 1000;
        }
        _ => {}
    }

    let bid_total_vol: u64 = bid_volume.iter().sum();
    let ask_total_vol: u64 = ask_volume.iter().sum();

    BidAskSTKv1 {
        code: gen.code().to_string(),
        datetime: Local::now(),
        bid_price,
        bid_volume,
        ask_price,
        ask_volume,
        bid_total_vol,
        ask_total_vol,
    }
}
