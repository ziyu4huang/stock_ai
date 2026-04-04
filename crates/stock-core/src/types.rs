use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Clone, Debug)]
pub struct Bar {
    pub time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
}

pub struct AppState {
    pub av_key: String,
    pub client: reqwest::Client,
    pub db: Mutex<rusqlite::Connection>,
    pub project_dir: String,
    pub fetch_backend: Mutex<String>, // "yahoo" (default, AV fallback) | "av" | "yahoo-only"
}

#[derive(Deserialize)]
pub struct HQ {
    pub days: Option<u64>,
    pub interval: Option<String>,
}

#[derive(Deserialize)]
pub struct KQ {
    pub period: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

// ── Watchlist types ───────────────────────────────────────────────────────

#[derive(Serialize, Clone, Debug)]
pub struct WatchlistItem {
    pub symbol: String,
    pub name: String,
    pub strategy_id: Option<i64>,
    pub added_at: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Strategy {
    pub id: Option<i64>,
    pub name: String,
    pub stype: String,        // "hmm_regime", "rsi_reversal", "macd_cross", "bb_bounce"
    pub params: String,       // JSON: {"n_states":4, "best_regime_only":true, ...}
    pub enabled: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct SignalLog {
    pub id: i64,
    pub symbol: String,
    pub date: String,
    pub signal_type: String,  // "LONG", "SHORT", "EXIT", "HOLD"
    pub regime_state: Option<i64>,
    pub confidence: f64,
    pub details: Option<String>,
    pub created_at: String,
}

pub fn is_tw(symbol: &str) -> bool {
    symbol.ends_with(".TW") || symbol.ends_with(".TWO")
}

// ── Day Trading Signal types ──────────────────────────────────────────────

#[derive(Serialize, Clone, Debug, PartialEq)]
pub enum SignalDirection {
    Buy,
    Sell,
    Neutral,
}

#[derive(Serialize, Clone, Debug, PartialEq)]
pub enum SignalStrength {
    Weak,
    Medium,
    Strong,
}

#[derive(Serialize, Clone, Debug)]
pub enum SignalKind {
    RsiReversal,
    MacdHistCross,
    MacdSignalCross,
    BbSqueeze,
    BbBreakout,
    VolumeSurge,
    EngulfingBull,
    EngulfingBear,
    Hammer,
    ShootingStar,
    MorningStar,
    EveningStar,
    EmaCross,
    GapUp,
    GapDown,
}

impl SignalKind {
    pub fn name(&self) -> &'static str {
        match self {
            Self::RsiReversal => "RSI Reversal",
            Self::MacdHistCross => "MACD Hist Cross",
            Self::MacdSignalCross => "MACD Signal Cross",
            Self::BbSqueeze => "BB Squeeze",
            Self::BbBreakout => "BB Breakout",
            Self::VolumeSurge => "Volume Surge",
            Self::EngulfingBull => "Bull Engulfing",
            Self::EngulfingBear => "Bear Engulfing",
            Self::Hammer => "Hammer",
            Self::ShootingStar => "Shooting Star",
            Self::MorningStar => "Morning Star",
            Self::EveningStar => "Evening Star",
            Self::EmaCross => "EMA Cross",
            Self::GapUp => "Gap Up",
            Self::GapDown => "Gap Down",
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct DaySignal {
    pub kind: SignalKind,
    pub direction: SignalDirection,
    pub strength: SignalStrength,
    pub score: i32,
    pub reason: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct BarSignals {
    pub idx: usize,
    pub date: String,
    pub signals: Vec<DaySignal>,
    pub score: i32,
}

#[derive(Serialize, Clone, Debug)]
pub struct DayTradeAnalysis {
    pub symbol: String,
    pub bars: Vec<Bar>,
    pub signals: Vec<BarSignals>,
    pub latest_score: i32,
    pub latest_direction: SignalDirection,
}

#[derive(Serialize, Clone, Debug)]
pub struct ScanResult {
    pub symbol: String,
    pub name: String,
    pub last_price: f64,
    pub score: i32,
    pub direction: SignalDirection,
    pub top_signals: Vec<String>,
}
