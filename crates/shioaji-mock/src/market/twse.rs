/// TWSE tick size table.
pub fn tick_size(price: f64) -> f64 {
    if price < 10.0 { 0.01 }
    else if price < 50.0 { 0.05 }
    else if price < 100.0 { 0.1 }
    else if price < 500.0 { 0.5 }
    else if price < 1000.0 { 1.0 }
    else { 5.0 }
}

/// Round price to nearest valid tick.
pub fn round_to_tick(price: f64) -> f64 {
    let ts = tick_size(price);
    (price / ts).round() * ts
}

/// Enforce daily price limit (+/-10% of reference).
pub fn clamp_price(price: f64, reference: f64) -> f64 {
    let limit_up = (reference * 1.1 * 100.0).round() / 100.0;
    let limit_down = (reference * 0.9 * 100.0).round() / 100.0;
    price.max(limit_down).min(limit_up)
}

/// Minimum lot size in shares (TWSE = 1000).
pub const LOT_SIZE: u64 = 1000;

/// Align share count to lot boundary.
pub fn align_lot(shares: u64) -> u64 {
    ((shares + LOT_SIZE - 1) / LOT_SIZE) * LOT_SIZE
}
