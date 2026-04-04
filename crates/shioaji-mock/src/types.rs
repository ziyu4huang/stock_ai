use chrono::{DateTime, Local};

// ── Exchange ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Exchange {
    TSE, // 台灣證券交易所 (上市)
    OTC, // 櫃買中心 (上櫃)
}

// ── Tick type ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TickType {
    Buy,
    Sell,
    Unknown,
}

// ── Change type ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChangeType {
    Up,
    Down,
    Unchanged,
    Open,
}

// ── Quote subscription type ─────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum QuoteType {
    Tick,
    BidAsk,
    KLine,
}

// ── Shioaji-compatible stock tick ───────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TickSTKv1 {
    pub code: String,
    pub datetime: DateTime<Local>,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: u64,         // this-tick volume (shares)
    pub total_volume: u64,   // cumulative day volume
    pub amount: f64,         // tick monetary value
    pub total_amount: f64,   // cumulative day amount
    pub bid_side_total_vol: u64,
    pub ask_side_total_vol: u64,
    pub avg_price: f64,      // VWAP
    pub tick_type: TickType,
    pub chg_type: ChangeType,
    pub price_chg: f64,      // absolute change from reference
    pub pct_chg: f64,        // percentage change
    pub simtrade: bool,
}

// ── Shioaji-compatible bid/ask snapshot ─────────────────────────────────────

#[derive(Debug, Clone)]
pub struct BidAskSTKv1 {
    pub code: String,
    pub datetime: DateTime<Local>,
    pub bid_price: [f64; 5],
    pub bid_volume: [u64; 5],
    pub ask_price: [f64; 5],
    pub ask_volume: [u64; 5],
    pub bid_total_vol: u64,
    pub ask_total_vol: u64,
}

// ── Stock contract ──────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct StockContract {
    pub exchange: Exchange,
    pub code: String,
    pub symbol: String,
    pub name: String,
    pub unit: u64,            // 1000 shares per lot
    pub reference_price: f64, // previous close
    pub limit_up: f64,
    pub limit_down: f64,
}

impl StockContract {
    pub fn tse(code: &str, name: &str, reference_price: f64) -> Self {
        let limit_up = (reference_price * 1.1 * 100.0).round() / 100.0;
        let limit_down = (reference_price * 0.9 * 100.0).round() / 100.0;
        Self {
            exchange: Exchange::TSE,
            code: code.to_string(),
            symbol: code.to_string(),
            name: name.to_string(),
            unit: 1000,
            reference_price,
            limit_up,
            limit_down,
        }
    }
}

// ── Market regime ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MarketRegime {
    Normal,
    WhaleAccum,
    Ignition,
    Distribution,
}

// ── Predefined stocks (mirrors TUI STOCKS) ──────────────────────────────────

pub const STOCKS: &[(&str, &str, f64)] = &[
    ("2330", "台積電", 840.0),
    ("2317", "鴻海",   112.0),
    ("2454", "聯發科", 1200.0),
    ("2881", "富邦金", 88.0),
    ("2882", "國泰金", 52.0),
    ("2303", "聯電",   48.0),
    ("2308", "台達電", 320.0),
    ("2412", "中華電", 128.0),
];
