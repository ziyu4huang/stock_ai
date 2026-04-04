use crate::detector::{IgnitionDetector, WhaleDetector};
use crate::hmm::{HmmModel, HmmObservationBuilder};
use crate::state_machine::StockStateTracker;
use crate::types::{
    ActionAdvice, BookDeltaSummary, CompositeScore, HmmConfirmation, HmmState, IgnitionEvent,
    MarketState, OrderBook, PositionDir, SoundInfo, SuspiciousLevel, Tick, TradeSide,
    WhaleQualityMetrics, WhaleTrade, WhaleRank,
};
use chrono::{DateTime, Local};
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
    pub fn color(&self) -> &'static str {
        match self {
            AlertKind::WhaleBuy(r) => r.color(),
            AlertKind::WhaleSell(WhaleRank::Mega) => "#e040fb",
            AlertKind::WhaleSell(WhaleRank::Large) => "#ffd600",
            AlertKind::WhaleSell(_) => "#00bcd4",
            AlertKind::LongIgnition => "#69f0ae",
            AlertKind::ShortIgnition => "#ff5252",
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
    /// HMM confirmation layer
    pub hmm_state: HmmState,
    pub hmm_confirmation: HmmConfirmation,

    state_tracker: StockStateTracker,
    ignition_detector: IgnitionDetector,
    hmm_model: HmmModel,
    hmm_builder: HmmObservationBuilder,
    pub whale_threshold: f64,

    // ── Anti-spoof tracking ───────────────────────────────────────
    /// Previous order book lots per level (for delta computation)
    prev_bid_lots: [u64; 5],
    prev_ask_lots: [u64; 5],
    /// Latest computed book delta summary for the frontend
    pub book_delta: BookDeltaSummary,
    /// Whale quality metrics
    pub whale_metrics: WhaleQualityMetrics,
    /// Rolling tick timestamps for velocity calculation (last 120s)
    tick_timestamps: VecDeque<DateTime<Local>>,
    /// Whale trade timestamps for cluster detection
    whale_timestamps: VecDeque<DateTime<Local>>,
    /// Cumulative bid/ask deltas over last 2 minutes
    cum_bid_delta: i64,
    cum_ask_delta: i64,
    /// Timestamps of cumulative delta entries for expiry
    cum_delta_history: VecDeque<(DateTime<Local>, i64, i64)>, // (time, bid_delta, ask_delta)
}

impl StockView {
    pub fn new(symbol: &str, name: &str, base_price: f64, whale_threshold: f64) -> Self {
        let hmm_model = HmmModel::default_tuned();
        let initial_state = HmmState::Noise;
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
            hmm_state: initial_state.clone(),
            hmm_confirmation: HmmConfirmation::new(initial_state, &MarketState::Noise, 0),
            state_tracker: StockStateTracker::new(),
            ignition_detector: IgnitionDetector::default(),
            hmm_model,
            hmm_builder: HmmObservationBuilder::new(),
            whale_threshold,

            // Anti-spoof tracking
            prev_bid_lots: [0; 5],
            prev_ask_lots: [0; 5],
            book_delta: BookDeltaSummary::empty(),
            whale_metrics: WhaleQualityMetrics::default(),
            tick_timestamps: VecDeque::with_capacity(500),
            whale_timestamps: VecDeque::with_capacity(50),
            cum_bid_delta: 0,
            cum_ask_delta: 0,
            cum_delta_history: VecDeque::with_capacity(120),
        }
    }

    /// Returns `Some(IgnitionEvent)` if an ignition fired this tick.
    pub fn process_tick(&mut self, tick: Tick) -> Option<IgnitionEvent> {
        self.stats.total_ticks += 1;
        self.stats.last_price = tick.price;

        let now = Local::now();
        let tick_amount_m = tick.amount_m();

        // Track tick timestamps for velocity
        self.tick_timestamps.push_front(now);
        let cutoff = now - chrono::Duration::seconds(120);
        while self.tick_timestamps.back().map_or(false, |t| *t < cutoff) {
            self.tick_timestamps.pop_back();
        }
        // Trade velocity = ticks per minute
        let secs_span = self.tick_timestamps.front().unwrap()
            .signed_duration_since(*self.tick_timestamps.back().unwrap())
            .num_seconds().max(1) as f64;
        self.whale_metrics.trade_velocity = self.tick_timestamps.len() as f64 / (secs_span / 60.0);

        let whale_opt = WhaleDetector::check(&tick, self.whale_threshold);
        let is_whale = whale_opt.is_some();

        if is_whale {
            match tick.side {
                TradeSide::Buy => {
                    self.stats.whale_buys += 1;
                    self.whale_metrics.whale_buy_volume_m += tick_amount_m;
                    // Check if aggressive (at or above best ask)
                    if let Some(ref book) = self.order_book {
                        if tick.price >= book.asks[0].price {
                            self.whale_metrics.aggressive_buys += 1;
                        }
                    }
                }
                TradeSide::Sell => {
                    self.stats.whale_sells += 1;
                    self.whale_metrics.whale_sell_volume_m += tick_amount_m;
                    // Check if aggressive (at or below best bid)
                    if let Some(ref book) = self.order_book {
                        if tick.price <= book.bids[0].price {
                            self.whale_metrics.aggressive_sells += 1;
                        }
                    }
                }
            }

            // Whale cluster detection: 3+ whales within 5 seconds
            self.whale_timestamps.push_front(now);
            let whale_cutoff = now - chrono::Duration::seconds(5);
            while self.whale_timestamps.back().map_or(false, |t| *t < whale_cutoff) {
                self.whale_timestamps.pop_back();
            }
            if self.whale_timestamps.len() >= 3 {
                self.whale_metrics.cluster_count += 1;
            }
        }

        self.state = self.state_tracker.update(&tick, is_whale);

        // HMM observation update (runs every BATCH_INTERVAL ticks)
        let is_whale_buy = is_whale && tick.side == TradeSide::Buy;
        let is_whale_sell = is_whale && tick.side == TradeSide::Sell;
        if let Some(_obs) = self.hmm_builder.push_tick(tick.price, tick.shares as f64, is_whale_buy, is_whale_sell) {
            let history = self.hmm_builder.obs_history();
            if history.len() >= 2 {
                let path = self.hmm_model.viterbi(&history.iter().copied().collect::<Vec<_>>());
                if let Some(&last_state) = path.last() {
                    let hmm_state = match last_state {
                        0 => HmmState::Accumulation,
                        1 => HmmState::Ignition,
                        2 => HmmState::Distribution,
                        _ => HmmState::Noise,
                    };
                    self.hmm_state = hmm_state.clone();
                    self.hmm_confirmation = HmmConfirmation::new(hmm_state, &self.state, history.len());
                }
            }
        }

        let mut ignition_result = None;

        if let Some(ref whale) = whale_opt {
            self.push_alert(build_whale_alert(whale));
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
        let now = Local::now();

        // Compute deltas from previous book
        let mut bid_deltas = [0i64; 5];
        let mut ask_deltas = [0i64; 5];
        let mut suspicious = Vec::new();

        for i in 0..5 {
            let new_bid = book.bids[i].lots as i64;
            let old_bid = self.prev_bid_lots[i] as i64;
            bid_deltas[i] = new_bid - old_bid;

            let new_ask = book.asks[i].lots as i64;
            let old_ask = self.prev_ask_lots[i] as i64;
            ask_deltas[i] = new_ask - old_ask;

            // Detect suspicious sudden adds (≥3x previous volume)
            if old_bid > 0 && new_bid > (old_bid * 3) as i64 {
                suspicious.push(SuspiciousLevel {
                    side: "bid".into(),
                    level: i,
                    kind: "sudden_add".into(),
                    volume: new_bid - old_bid,
                });
                self.whale_metrics.suspicious_adds += 1;
            }
            if old_ask > 0 && new_ask > (old_ask * 3) as i64 {
                suspicious.push(SuspiciousLevel {
                    side: "ask".into(),
                    level: i,
                    kind: "sudden_add".into(),
                    volume: new_ask - old_ask,
                });
                self.whale_metrics.suspicious_adds += 1;
            }

            // Detect suspicious sudden removes (≥80% gone without trade consumption)
            if old_bid > 10 && new_bid < old_bid / 5 {
                suspicious.push(SuspiciousLevel {
                    side: "bid".into(),
                    level: i,
                    kind: "sudden_remove".into(),
                    volume: old_bid - new_bid,
                });
                self.whale_metrics.suspicious_removes += 1;
            }
            if old_ask > 10 && new_ask < old_ask / 5 {
                suspicious.push(SuspiciousLevel {
                    side: "ask".into(),
                    level: i,
                    kind: "sudden_remove".into(),
                    volume: old_ask - new_ask,
                });
                self.whale_metrics.suspicious_removes += 1;
            }

            // Update cumulative deltas
            self.cum_bid_delta += bid_deltas[i];
            self.cum_ask_delta += ask_deltas[i];
        }

        // Track cumulative delta history (for 2-min rolling window)
        let total_bid_d: i64 = bid_deltas.iter().sum();
        let total_ask_d: i64 = ask_deltas.iter().sum();
        self.cum_delta_history.push_front((now, total_bid_d, total_ask_d));

        // Expire entries older than 2 minutes
        let cutoff = now - chrono::Duration::seconds(120);
        while self.cum_delta_history.back().map_or(false, |(t, _, _)| *t < cutoff) {
            if let Some((_, bd, ad)) = self.cum_delta_history.pop_back() {
                self.cum_bid_delta -= bd;
                self.cum_ask_delta -= ad;
            }
        }

        // Update book delta summary
        self.book_delta = BookDeltaSummary {
            bid_deltas,
            ask_deltas,
            suspicious_levels: suspicious,
            cumulative_bid_delta: self.cum_bid_delta,
            cumulative_ask_delta: self.cum_ask_delta,
        };

        // Save current lots for next delta comparison
        for i in 0..5 {
            self.prev_bid_lots[i] = book.bids[i].lots;
            self.prev_ask_lots[i] = book.asks[i].lots;
        }

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

                // Absorption score: if large pressure but price hasn't moved much,
                // the pressure is being absorbed (real demand/supply)
                let price_range = (book.asks[0].price - book.bids[0].price).abs();
                let pressure_imbalance = (total_bid as f64 - total_ask as f64).abs() / total;
                if pressure_imbalance > 0.3 && price_range < crate::types::tick_size(book.last_price) * 2.0 {
                    self.whale_metrics.absorption_score = (pressure_imbalance * 200.0).min(100.0);
                } else {
                    self.whale_metrics.absorption_score *= 0.9; // decay
                }
            }
        }

        // HMM confirmation bonus/penalty
        self.composite.hmm_pts = self.compute_hmm_pts();
        score += self.composite.hmm_pts;

        self.composite.value = CompositeScore::clamp(score);

        // Compute signal confidence (0-100)
        self.whale_metrics.signal_confidence = self.compute_confidence();

        // Derive action advice
        self.action = self.derive_action();
    }

    /// Compute signal confidence 0-100 based on signal quality indicators.
    fn compute_confidence(&self) -> f64 {
        let mut conf = 70.0; // base confidence

        let m = &self.whale_metrics;

        // Boost: aggressive whales (at best bid/ask) are more credible
        let total_whales = self.stats.whale_buys + self.stats.whale_sells;
        if total_whales > 0 {
            let aggressive_ratio = (m.aggressive_buys + m.aggressive_sells) as f64 / total_whales as f64;
            conf += aggressive_ratio * 15.0;
        }

        // Boost: HMM agrees with rule-based signals
        if self.hmm_confirmation.agrees_with_rules {
            conf += 10.0;
        }

        // Boost: high absorption (real pressure, not spoofing)
        conf += m.absorption_score * 0.1;

        // Penalty: suspicious order book activity
        let spoof_penalty = (m.suspicious_adds + m.suspicious_removes) as f64 * 5.0;
        conf -= spoof_penalty;

        // Penalty: clustered whales (potential wash trading)
        conf -= m.cluster_count as f64 * 10.0;

        // Penalty: very high trade velocity (could be manipulation)
        if m.trade_velocity > 60.0 {
            conf -= (m.trade_velocity - 60.0) * 0.3;
        }

        conf.max(0.0).min(100.0)
    }

    fn derive_action(&self) -> ActionAdvice {
        let s = self.composite.value;
        let hmm_boost = if self.hmm_confirmation.agrees_with_rules { 10 } else { 0 };

        match self.state {
            MarketState::LongIgnition if s >= 50 - hmm_boost => ActionAdvice::FollowBull,
            MarketState::LongAccum if s >= 20 - hmm_boost => ActionAdvice::WatchBull,
            MarketState::ShortIgnition if s <= -50 + hmm_boost => ActionAdvice::FollowBear,
            MarketState::ShortDistrib if s <= -20 + hmm_boost => ActionAdvice::WatchBear,
            _ if s >= 60 => ActionAdvice::WatchBull,
            _ if s <= -60 => ActionAdvice::WatchBear,
            // HMM ignition override: promote if HMM strongly agrees
            _ if self.hmm_state == HmmState::Ignition && s >= 40 => ActionAdvice::FollowBull,
            _ if self.hmm_state == HmmState::Distribution && s <= -40 => ActionAdvice::FollowBear,
            _ => ActionAdvice::StandAside,
        }
    }

    /// Compute HMM confirmation points: ±15 max.
    fn compute_hmm_pts(&self) -> i32 {
        if self.hmm_confirmation.agrees_with_rules {
            match self.hmm_state {
                HmmState::Ignition => 15,
                HmmState::Accumulation => 8,
                HmmState::Distribution => -15,
                HmmState::Noise => -5,
            }
        } else {
            match self.hmm_state {
                HmmState::Ignition => 5,
                HmmState::Noise => -10,
                _ => 0,
            }
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
