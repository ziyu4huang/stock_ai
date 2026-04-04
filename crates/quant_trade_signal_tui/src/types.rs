use chrono::{DateTime, Local};
// re-export DateTime for SoundInfo
use ratatui::style::Color;

// ── Trade side ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum TradeSide {
    Buy,
    Sell,
}
impl TradeSide {
    pub fn label(&self) -> &'static str { match self { TradeSide::Buy => "BUY", TradeSide::Sell => "SELL" } }
    pub fn arrow(&self) -> &'static str { match self { TradeSide::Buy => "▲", TradeSide::Sell => "▼" } }
    pub fn color(&self) -> Color { match self { TradeSide::Buy => Color::LightGreen, TradeSide::Sell => Color::LightRed } }
}

// ── Position direction ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum PositionDir { Long, Short }
impl PositionDir {
    pub fn label(&self) -> &'static str { match self { PositionDir::Long => "做多 LONG", PositionDir::Short => "做空 SHORT" } }
    pub fn color(&self) -> Color { match self { PositionDir::Long => Color::LightGreen, PositionDir::Short => Color::LightRed } }
}

// ── Raw tick ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Tick {
    pub symbol: String,
    pub timestamp: DateTime<Local>,
    pub price: f64,
    pub shares: u64,
    pub side: TradeSide,
    pub amount: f64, // NTD = price × shares
}
impl Tick {
    pub fn amount_m(&self) -> f64 { self.amount / 1_000_000.0 }
    pub fn lots(&self) -> u64 { self.shares / 1000 }
}

// ── Whale trade ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct WhaleTrade { pub tick: Tick, pub rank: WhaleRank }

#[derive(Debug, Clone, PartialEq)]
pub enum WhaleRank { Medium, Large, Mega }
impl WhaleRank {
    pub fn from_amount(a: f64) -> Self {
        if a >= 50_000_000.0 { WhaleRank::Mega }
        else if a >= 20_000_000.0 { WhaleRank::Large }
        else { WhaleRank::Medium }
    }
    pub fn label(&self) -> &'static str {
        match self { WhaleRank::Medium => "WHALE", WhaleRank::Large => "BIG WHALE", WhaleRank::Mega => "MEGA WHALE" }
    }
    pub fn color(&self) -> Color {
        match self { WhaleRank::Medium => Color::Cyan, WhaleRank::Large => Color::Yellow, WhaleRank::Mega => Color::Magenta }
    }
}

// ── Ignition event ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct IgnitionEvent {
    pub symbol: String,
    pub timestamp: DateTime<Local>,
    pub whale_count: usize,
    pub price_move_ticks: f64, // positive = up (long), negative = down (short)
    pub total_amount_m: f64,
    pub direction: PositionDir,
}

// ── Market state (6-state, symmetric long/short) ──────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum MarketState {
    LongAccum,     // 低位吸籌 → 做多準備
    LongIgnition,  // 多頭點火 → 建多倉
    ShortDistrib,  // 高位出貨 → 做空準備
    ShortIgnition, // 空頭點火 → 建空倉
    Neutral,       // 整理中
    Noise,         // 雜訊
}
impl MarketState {
    pub fn label(&self) -> &'static str {
        match self {
            MarketState::LongAccum => "L-ACCUM",
            MarketState::LongIgnition => "L-IGNITE",
            MarketState::ShortDistrib => "S-DISTRIB",
            MarketState::ShortIgnition => "S-IGNITE",
            MarketState::Neutral => "NEUTRAL",
            MarketState::Noise => "NOISE",
        }
    }
    pub fn icon(&self) -> &'static str {
        match self {
            MarketState::LongAccum => "📈",
            MarketState::LongIgnition => "🔥▲",
            MarketState::ShortDistrib => "📉",
            MarketState::ShortIgnition => "🔥▼",
            MarketState::Neutral => "➖",
            MarketState::Noise => "〰",
        }
    }
    pub fn color(&self) -> Color {
        match self {
            MarketState::LongAccum => Color::Green,
            MarketState::LongIgnition => Color::LightGreen,
            MarketState::ShortDistrib => Color::Red,
            MarketState::ShortIgnition => Color::LightRed,
            MarketState::Neutral => Color::Gray,
            MarketState::Noise => Color::DarkGray,
        }
    }
    pub fn position_dir(&self) -> Option<PositionDir> {
        match self {
            MarketState::LongAccum | MarketState::LongIgnition => Some(PositionDir::Long),
            MarketState::ShortDistrib | MarketState::ShortIgnition => Some(PositionDir::Short),
            _ => None,
        }
    }
}

// ── Order book (五檔) ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct OrderLevel {
    pub price: f64,
    pub lots: u64, // 張 (1 lot = 1000 shares)
}

#[derive(Debug, Clone)]
pub struct OrderBook {
    pub symbol: String,
    pub timestamp: DateTime<Local>,
    /// asks[0] = best ask (lowest ask = 賣1)
    pub asks: [OrderLevel; 5],
    /// bids[0] = best bid (highest bid = 買1)
    pub bids: [OrderLevel; 5],
    pub last_price: f64,
    pub last_side: TradeSide,
}

// ── Feed event ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum FeedEvent {
    Tick(Tick),
    Book(OrderBook),
}

// ── Stock config ──────────────────────────────────────────────────────────────

pub struct StockConfig {
    pub symbol: &'static str,
    pub name: &'static str,
    pub base_price: f64,
    pub whale_threshold: f64,
}

/// All available stocks (first MONITORED_COUNT are shown in tabs)
pub const STOCKS: &[StockConfig] = &[
    StockConfig { symbol: "2330", name: "台積電", base_price: 840.0,  whale_threshold: 5_000_000.0 },
    StockConfig { symbol: "2317", name: "鴻海",   base_price: 112.0,  whale_threshold: 5_000_000.0 },
    StockConfig { symbol: "2454", name: "聯發科", base_price: 1200.0, whale_threshold: 5_000_000.0 },
    StockConfig { symbol: "2881", name: "富邦金", base_price: 88.0,   whale_threshold: 3_000_000.0 },
    StockConfig { symbol: "2882", name: "國泰金", base_price: 52.0,   whale_threshold: 3_000_000.0 },
    StockConfig { symbol: "2303", name: "聯電",   base_price: 48.0,   whale_threshold: 3_000_000.0 },
    StockConfig { symbol: "2308", name: "台達電", base_price: 320.0,  whale_threshold: 5_000_000.0 },
    StockConfig { symbol: "2412", name: "中華電", base_price: 128.0,  whale_threshold: 3_000_000.0 },
];

/// Number of stocks shown as tabs (first N from STOCKS)
pub const TAB_COUNT: usize = 5;

pub fn tick_size(price: f64) -> f64 {
    if price < 10.0 { 0.01 }
    else if price < 50.0 { 0.05 }
    else if price < 100.0 { 0.1 }
    else if price < 500.0 { 0.5 }
    else if price < 1000.0 { 1.0 }
    else { 5.0 }
}

// ── Composite score & action advice ─────────────────────────────────────────

/// Composite signal score: -100 (strong bear) to +100 (strong bull).
#[derive(Debug, Clone)]
pub struct CompositeScore {
    pub value: i32,
    pub whale_buy_pts: i32,
    pub whale_sell_pts: i32,
    pub ignition_pts: i32,
    pub pressure_pts: i32,
}

impl CompositeScore {
    pub fn new() -> Self {
        Self { value: 0, whale_buy_pts: 0, whale_sell_pts: 0, ignition_pts: 0, pressure_pts: 0 }
    }

    /// Clamp to [-100, +100].
    pub fn clamp(val: i32) -> i32 {
        val.max(-100).min(100)
    }
}

/// Action recommendation derived from composite score + market state.
#[derive(Debug, Clone, PartialEq)]
pub enum ActionAdvice {
    FollowBull,      // 跟多 — whale bull + ignition
    WatchBull,       // 觀多 — whale bull accumulating
    FollowBear,      // 跟空 — whale bear + ignition
    WatchBear,       // 觀空 — whale bear distributing
    StandAside,      // 觀望 — no clear signal
}

impl ActionAdvice {
    pub fn label_zh(&self) -> &'static str {
        match self {
            ActionAdvice::FollowBull => "跟多進場 ▲",
            ActionAdvice::WatchBull => "觀察做多 ▲",
            ActionAdvice::FollowBear => "跟空進場 ▼",
            ActionAdvice::WatchBear => "觀察做空 ▼",
            ActionAdvice::StandAside => "觀望 —",
        }
    }
    pub fn color(&self) -> Color {
        match self {
            ActionAdvice::FollowBull => Color::LightGreen,
            ActionAdvice::WatchBull => Color::Green,
            ActionAdvice::FollowBear => Color::LightRed,
            ActionAdvice::WatchBear => Color::Red,
            ActionAdvice::StandAside => Color::DarkGray,
        }
    }
    pub fn is_bull(&self) -> bool {
        matches!(self, ActionAdvice::FollowBull | ActionAdvice::WatchBull)
    }
}

/// Last sound alert info for UI display.
#[derive(Debug, Clone)]
pub struct SoundInfo {
    pub label: String,
    pub timestamp: DateTime<Local>,
}
