use crate::detector::{IgnitionDetector, WhaleDetector};
use crate::state_machine::StockStateTracker;
use crate::types::{
    ActionAdvice, CompositeScore, IgnitionEvent, MarketState, OrderBook, PositionDir,
    SoundInfo, Tick, TradeSide, WhaleTrade, WhaleRank,
};
use chrono::{DateTime, Local};
use ratatui::style::Color;
use std::collections::VecDeque;

// ── Alert types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum AlertKind {
    WhaleBuy(WhaleRank),
    WhaleSell(WhaleRank),
    LongIgnition,
    ShortIgnition,
}

impl AlertKind {
    pub fn color(&self) -> Color {
        match self {
            AlertKind::WhaleBuy(r) => r.color(),
            AlertKind::WhaleSell(WhaleRank::Mega) => Color::Magenta,
            AlertKind::WhaleSell(WhaleRank::Large) => Color::Yellow,
            AlertKind::WhaleSell(_) => Color::Cyan,
            AlertKind::LongIgnition => Color::LightGreen,
            AlertKind::ShortIgnition => Color::LightRed,
        }
    }
    pub fn icon(&self) -> &'static str {
        match self {
            AlertKind::WhaleBuy(WhaleRank::Mega) | AlertKind::WhaleSell(WhaleRank::Mega) => "🐳",
            AlertKind::WhaleBuy(_) | AlertKind::WhaleSell(_) => "🐋",
            AlertKind::LongIgnition => "🔥▲",
            AlertKind::ShortIgnition => "🔥▼",
        }
    }
    pub fn is_ignition(&self) -> bool {
        matches!(self, AlertKind::LongIgnition | AlertKind::ShortIgnition)
    }
}

#[derive(Debug, Clone)]
pub struct AlertEntry {
    pub timestamp: DateTime<Local>,
    pub kind: AlertKind,
    pub line1: String,
    pub line2: String,
}

// ── Per-stock statistics ──────────────────────────────────────────────────────

#[derive(Debug)]
pub struct StockStats {
    pub total_ticks: u64,
    pub whale_buys: u64,
    pub whale_sells: u64,
    pub long_ignitions: u64,
    pub short_ignitions: u64,
    pub open_price: f64,
    pub last_price: f64,
}

impl StockStats {
    pub fn new(base_price: f64) -> Self {
        Self {
            total_ticks: 0,
            whale_buys: 0,
            whale_sells: 0,
            long_ignitions: 0,
            short_ignitions: 0,
            open_price: base_price,
            last_price: base_price,
        }
    }

    pub fn change_pct(&self) -> f64 {
        if self.open_price == 0.0 { return 0.0; }
        (self.last_price - self.open_price) / self.open_price * 100.0
    }
}

// ── Per-stock view ────────────────────────────────────────────────────────────

pub struct StockView {
    pub symbol: String,
    pub name: String,
    pub tick_log: VecDeque<Tick>,      // last 60 ticks
    pub alerts: VecDeque<AlertEntry>,  // last 40 alerts, newest first
    pub order_book: Option<OrderBook>,
    pub state: MarketState,
    pub stats: StockStats,
    /// Set when an ignition fires; cleared when the tab is focused
    pub has_signal: bool,
    pub signal_dir: Option<PositionDir>,
    /// Composite signal score (-100 to +100)
    pub composite: CompositeScore,
    /// Current action recommendation
    pub action: ActionAdvice,
    /// Last sound alert info for UI display
    pub last_sound: Option<SoundInfo>,

    state_tracker: StockStateTracker,
    ignition_detector: IgnitionDetector,
    pub whale_threshold: f64,
}

impl StockView {
    pub fn new(symbol: &str, name: &str, base_price: f64, whale_threshold: f64) -> Self {
        Self {
            symbol: symbol.to_string(),
            name: name.to_string(),
            tick_log: VecDeque::with_capacity(60),
            alerts: VecDeque::with_capacity(40),
            order_book: None,
            state: MarketState::Noise,
            stats: StockStats::new(base_price),
            has_signal: false,
            signal_dir: None,
            composite: CompositeScore::new(),
            action: ActionAdvice::StandAside,
            last_sound: None,
            state_tracker: StockStateTracker::new(),
            ignition_detector: IgnitionDetector::default(),
            whale_threshold,
        }
    }

    /// Returns `Some(IgnitionEvent)` if an ignition fired this tick.
    pub fn process_tick(&mut self, tick: Tick) -> Option<IgnitionEvent> {
        self.stats.total_ticks += 1;
        self.stats.last_price = tick.price;

        let whale_opt = WhaleDetector::check(&tick, self.whale_threshold);
        let is_whale = whale_opt.is_some();

        if is_whale {
            match tick.side {
                TradeSide::Buy => self.stats.whale_buys += 1,
                TradeSide::Sell => self.stats.whale_sells += 1,
            }
        }

        self.state = self.state_tracker.update(&tick, is_whale);

        let mut ignition_result = None;

        if let Some(ref whale) = whale_opt {
            self.push_alert(build_whale_alert(whale));
            let now = Local::now();
            if let Some(ig) = self.ignition_detector.push(whale, now) {
                match ig.direction {
                    PositionDir::Long => self.stats.long_ignitions += 1,
                    PositionDir::Short => self.stats.short_ignitions += 1,
                }
                self.has_signal = true;
                self.signal_dir = Some(ig.direction.clone());
                self.push_alert(build_ignition_alert(&ig));
                ignition_result = Some(ig);
            }
        }

        if self.tick_log.len() >= 60 { self.tick_log.pop_back(); }
        self.tick_log.push_front(tick);

        ignition_result
    }

    pub fn update_book(&mut self, book: OrderBook) {
        self.order_book = Some(book);
    }

    pub fn clear_signal(&mut self) {
        self.has_signal = false;
        self.signal_dir = None;
    }

    /// Recalculate composite score and action advice from current stats.
    pub fn recompute(&mut self) {
        let mut score = 0i32;

        // Whale buy/sell contributions
        let wb = self.stats.whale_buys;
        let ws = self.stats.whale_sells;
        let whale_net = wb as i32 - ws as i32;
        let whale_pts = whale_net * 5;
        self.composite.whale_buy_pts = (wb as i32 * 5).min(30);
        self.composite.whale_sell_pts = (ws as i32 * 5).min(30);
        score += whale_pts;

        // Ignition bonus
        let li = self.stats.long_ignitions as i32;
        let si = self.stats.short_ignitions as i32;
        self.composite.ignition_pts = (li * 20) - (si * 20);
        score += self.composite.ignition_pts;

        // Order book pressure
        if let Some(ref book) = self.order_book {
            let total_bid: u64 = book.bids.iter().map(|l| l.lots).sum();
            let total_ask: u64 = book.asks.iter().map(|l| l.lots).sum();
            let total = (total_bid + total_ask) as f64;
            if total > 0.0 {
                let bias = (total_bid as f64 / total - 0.5) * 40.0;
                self.composite.pressure_pts = bias as i32;
                score += bias as i32;
            }
        }

        self.composite.value = CompositeScore::clamp(score);

        // Derive action advice
        self.action = self.derive_action();
    }

    fn derive_action(&self) -> ActionAdvice {
        let s = self.composite.value;
        match self.state {
            MarketState::LongIgnition if s >= 50 => ActionAdvice::FollowBull,
            MarketState::LongAccum if s >= 20 => ActionAdvice::WatchBull,
            MarketState::ShortIgnition if s <= -50 => ActionAdvice::FollowBear,
            MarketState::ShortDistrib if s <= -20 => ActionAdvice::WatchBear,
            _ if s >= 60 => ActionAdvice::WatchBull,
            _ if s <= -60 => ActionAdvice::WatchBear,
            _ => ActionAdvice::StandAside,
        }
    }

    /// Suggested stop-loss price based on current action.
    pub fn stop_loss_price(&self) -> Option<f64> {
        match self.action {
            ActionAdvice::FollowBull | ActionAdvice::WatchBull => {
                // 2 ticks below last price
                let ts = crate::types::tick_size(self.stats.last_price);
                Some(self.stats.last_price - ts * 2.0)
            }
            ActionAdvice::FollowBear | ActionAdvice::WatchBear => {
                let ts = crate::types::tick_size(self.stats.last_price);
                Some(self.stats.last_price + ts * 2.0)
            }
            ActionAdvice::StandAside => None,
        }
    }

    fn push_alert(&mut self, alert: AlertEntry) {
        if self.alerts.len() >= 40 { self.alerts.pop_back(); }
        self.alerts.push_front(alert);
    }
}

// ── Alert builders ────────────────────────────────────────────────────────────

fn build_whale_alert(whale: &WhaleTrade) -> AlertEntry {
    let kind = match whale.tick.side {
        TradeSide::Buy => AlertKind::WhaleBuy(whale.rank.clone()),
        TradeSide::Sell => AlertKind::WhaleSell(whale.rank.clone()),
    };
    let dir_tag = match whale.tick.side {
        TradeSide::Buy => "▲ 做多訊號",
        TradeSide::Sell => "▼ 做空訊號",
    };
    AlertEntry {
        timestamp: whale.tick.timestamp,
        kind,
        line1: format!(
            "{} {:.1}M NTD  {}",
            whale.rank.label(), whale.tick.amount_m(), dir_tag
        ),
        line2: format!(
            "  {} @ {:.1}  {:>6}sh / {:>4}張",
            whale.tick.side.label(), whale.tick.price,
            whale.tick.shares, whale.tick.lots()
        ),
    }
}

fn build_ignition_alert(ev: &IgnitionEvent) -> AlertEntry {
    let (kind, dir_label, tick_label) = match ev.direction {
        PositionDir::Long => (
            AlertKind::LongIgnition,
            "🔥▲ LONG 點火 — 建多倉",
            format!("+{:.1} ticks ▲", ev.price_move_ticks),
        ),
        PositionDir::Short => (
            AlertKind::ShortIgnition,
            "🔥▼ SHORT 點火 — 建空倉",
            format!("{:.1} ticks ▼", ev.price_move_ticks),
        ),
    };
    AlertEntry {
        timestamp: ev.timestamp,
        kind,
        line1: dir_label.to_string(),
        line2: format!(
            "  {}x whale  {:.1}M NTD  {}",
            ev.whale_count, ev.total_amount_m, tick_label
        ),
    }
}
