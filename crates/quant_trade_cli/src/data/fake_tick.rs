use crate::data::tick_source::TickSource;
use crate::data::types::{Bar, Tick};

/// Taiwan stock tick size by price tier.
pub fn tw_tick_size(price: f64) -> f64 {
    match price {
        p if p < 10.0 => 0.01,
        p if p < 50.0 => 0.05,
        p if p < 100.0 => 0.10,
        p if p < 500.0 => 0.50,
        p if p < 1000.0 => 1.00,
        _ => 5.00,
    }
}

/// Snap price to nearest tick grid.
fn snap_tick(price: f64, tick_size: f64) -> f64 {
    (price / tick_size).round() * tick_size
}

/// Intraday volatility multiplier — U-shaped pattern.
/// TWSE: 09:00-13:30 = 270 minutes. High vol at open/close, low at lunch.
fn intraday_vol_multiplier(minute: usize) -> f64 {
    let t = minute as f64 / 270.0;
    let base = 2.0 * (t - 0.5).powi(2) + 0.5;
    let lunch_dip = if (150..210).contains(&minute) { 0.5 } else { 1.0 };
    base * lunch_dip
}

/// Intraday volume fraction — bimodal at open and close.
fn intraday_volume_fraction(minute: usize) -> f64 {
    let t = minute as f64 / 270.0;
    let open_surge = (-((t - 0.05).powi(2)) / 0.01).exp();
    let close_surge = (-((t - 0.95).powi(2)) / 0.01).exp();
    let base = 1.0 / 270.0;
    base + 0.3 * open_surge + 0.4 * close_surge
}

/// Generate simulated intraday ticks for a single daily bar using GBM.
fn generate_day_ticks(bar: &Bar) -> Vec<Tick> {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let mut ticks = Vec::with_capacity(270);

    let daily_range = bar.high - bar.low;
    let daily_vol = if daily_range > 0.0 {
        daily_range / bar.open
    } else {
        0.02 // fallback 2%
    };
    let drift = (bar.close / bar.open).ln() / 270.0;
    let tick_size = tw_tick_size(bar.close);

    let base_ts = bar.time; // midnight UTC for the day
    let open_ts = base_ts + 9 * 3600; // 09:00 UTC+8 approximation

    let mut price = bar.open;

    for minute in 0..270 {
        let local_vol = daily_vol * (1.0 / 270.0_f64).sqrt() * intraday_vol_multiplier(minute);
        let z: f64 = rng.gen_range(-3.0..3.0); // truncated normal approximation
        let ret = drift + local_vol * z;
        price *= ret.exp();

        // Constrain to daily range (soft)
        if price < bar.low * 0.995 {
            price = bar.low * 0.995;
        }
        if price > bar.high * 1.005 {
            price = bar.high * 1.005;
        }

        // Snap to tick grid
        price = snap_tick(price, tick_size);

        // Force close on last minute
        if minute == 269 {
            price = bar.close;
        }

        let spread_ticks = rng.gen_range(1..=3);
        let bid = snap_tick(price - spread_ticks as f64 * tick_size / 2.0, tick_size);
        let ask = snap_tick(price + spread_ticks as f64 * tick_size / 2.0, tick_size);

        let vol_frac = intraday_volume_fraction(minute);
        let minute_vol = ((bar.volume as f64) * vol_frac) as i64;
        let minute_vol = minute_vol.max(1);

        ticks.push(Tick {
            timestamp: open_ts + (minute * 60) as i64,
            price,
            volume: minute_vol,
            bid,
            ask,
            bid_size: rng.gen_range(1..50),
            ask_size: rng.gen_range(1..50),
        });
    }

    ticks
}

/// Fake tick source — generates simulated ticks from real daily bars.
#[allow(dead_code)]
pub struct FakeTickSource {
    pub symbol: String,
    pub daily_bars: Vec<Bar>,
    pub ticks: Vec<Tick>,
    pub cursor: usize,
}

impl FakeTickSource {
    pub fn new(symbol: &str, daily_bars: Vec<Bar>) -> Self {
        let mut ticks = Vec::new();
        for bar in &daily_bars {
            let day_ticks = generate_day_ticks(bar);
            ticks.extend(day_ticks);
        }
        Self {
            symbol: symbol.to_string(),
            daily_bars,
            ticks,
            cursor: 0,
        }
    }

    #[allow(dead_code)]
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    #[allow(dead_code)]
    pub fn daily_bars(&self) -> &[Bar] {
        &self.daily_bars
    }
}

impl TickSource for FakeTickSource {
    fn next_tick(&mut self) -> Option<Tick> {
        if self.cursor < self.ticks.len() {
            let tick = self.ticks[self.cursor].clone();
            self.cursor += 1;
            Some(tick)
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.cursor = 0;
    }

    fn total_ticks(&self) -> usize {
        self.ticks.len()
    }

    fn position(&self) -> usize {
        self.cursor
    }
}
