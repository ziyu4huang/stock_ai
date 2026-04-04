use serde::{Deserialize, Serialize};

/// Daily OHLCV bar (local version with Deserialize for JSON parsing)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    pub time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
}

/// Convert local Bar to stock_core::Bar for indicator calculations
impl From<&Bar> for stock_core::Bar {
    fn from(b: &Bar) -> Self {
        Self {
            time: b.time,
            open: b.open,
            high: b.high,
            low: b.low,
            close: b.close,
            volume: b.volume,
        }
    }
}

/// Simulated intraday tick
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Tick {
    pub timestamp: i64,
    pub price: f64,
    pub volume: i64,
    pub bid: f64,
    pub ask: f64,
    pub bid_size: i64,
    pub ask_size: i64,
}

/// Aggregated bar from ticks
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct IntradayBar {
    pub time: i64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
    pub vwap: f64,
}

/// Stock info entry
#[derive(Debug, Clone)]
pub struct StockEntry {
    pub symbol: String,
    pub name: String,
}

// ── Whale Hunting Types (Chapter 10) ──────────────────────────────────────

/// A detected 3-sigma volume spike event.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct VolumeSpike {
    pub symbol: String,
    pub timestamp: i64,
    pub volume: i64,
    pub mean: f64,
    pub sigma: f64,
    pub z_score: f64,
}

/// Energy pulse classification.
#[derive(Debug, Clone, PartialEq)]
pub enum PulsePattern {
    Single,
    Staircase,
    Accumulation,
    Distribution,
    Normal,
}

impl PulsePattern {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Single => "Single",
            Self::Staircase => "Staircase",
            Self::Accumulation => "Accumulation",
            Self::Distribution => "Distribution",
            Self::Normal => "Normal",
        }
    }
}

/// RMS energy pulse analysis result.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EnergyPulse {
    pub symbol: String,
    pub timestamp: i64,
    pub rms_energy: f64,
    pub pattern: PulsePattern,
    pub confidence: f64,
    pub window_bars: usize,
}

/// Aiming level classification.
#[derive(Debug, Clone, PartialEq)]
pub enum AimingLevel {
    Critical,
    High,
    Medium,
    Low,
}

impl AimingLevel {
    pub fn from_score(score: i32) -> Self {
        match score {
            s if s >= 90 => Self::Critical,
            s if s >= 70 => Self::High,
            s if s >= 50 => Self::Medium,
            _ => Self::Low,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Critical => "CRITICAL",
            Self::High => "HIGH",
            Self::Medium => "MEDIUM",
            Self::Low => "LOW",
        }
    }
}

/// Full aiming result for a stock.
#[derive(Debug, Clone)]
pub struct AimingResult {
    pub base_score: i32,
    pub geo_bonus: i32,
    pub sector_bonus: i32,
    pub orderbook_bonus: i32,
    pub pulse_bonus: i32,
    #[allow(dead_code)]
    pub penalty: i32,
    pub total_score: i32,
    pub level: AimingLevel,
}

/// Execution checklist item (Chapter 10, 6-item FIRE/HOLD checklist).
#[derive(Debug, Clone)]
pub struct ChecklistItem {
    pub label: String,
    pub passed: bool,
    pub detail: String,
}

/// Stop loss state for an open position.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct StopLossState {
    pub entry_price: f64,
    pub entry_time: i64,
    pub direction: stock_core::SignalDirection,
    pub max_favorable: f64,
}

impl StopLossState {
    #[allow(dead_code)]
    pub fn new(price: f64, time: i64, direction: stock_core::SignalDirection) -> Self {
        Self {
            entry_price: price,
            entry_time: time,
            direction,
            max_favorable: price,
        }
    }
}

/// Stop loss trigger type.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum StopLossTrigger {
    TickReverse,
    Timeout,
    Trailing,
    None,
}

/// Bull/Bear direction of whale activity.
#[derive(Debug, Clone, PartialEq)]
pub enum WhaleDirection {
    Bull,
    Bear,
    Neutral,
}

impl WhaleDirection {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Bull => "BULL",
            Self::Bear => "BEAR",
            Self::Neutral => "NEUTRAL",
        }
    }
}

/// Complete whale hunting analysis for one stock.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct WhaleAnalysis {
    pub symbol: String,
    pub aiming: AimingResult,
    pub checklist: Vec<ChecklistItem>,
    pub recent_spikes: Vec<VolumeSpike>,
    pub pulse: Option<EnergyPulse>,
    pub whale_direction: WhaleDirection,
}

/// Alert configuration (opt-in voice alerts).
#[derive(Debug, Clone)]
pub struct AlertConfig {
    pub enabled: bool,
    pub min_level: AimingLevel,
}
