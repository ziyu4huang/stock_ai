use crate::alert::{self, SoundQueue};
use crate::stock_view::{AlertKind, StockView};
use crate::types::{FeedEvent, TAB_COUNT, STOCKS};
use serde::Serialize;

/// Commands from the web UI to the TUI loop.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(tag = "action")]
pub enum Command {
    #[serde(rename = "switch_tab")]
    SwitchTab { index: usize },
    #[serde(rename = "next_tab")]
    NextTab,
    #[serde(rename = "prev_tab")]
    PrevTab,
    #[serde(rename = "toggle_pause")]
    TogglePause,
    #[serde(rename = "toggle_auto")]
    ToggleAuto,
    #[serde(rename = "toggle_sound")]
    ToggleSound,
    #[serde(rename = "toggle_voice")]
    ToggleVoice,
    #[serde(rename = "clear_alerts")]
    ClearAlerts,
    #[serde(rename = "quit")]
    Quit,
}

pub struct AppState {
    pub views: Vec<StockView>,     // TAB_COUNT entries
    pub active_tab: usize,
    pub auto_switch: bool,         // auto-jump to ignition tab
    pub paused: bool,
    pub total_events: u64,
    /// Tab index of last auto-switch (used for notification display)
    pub last_auto_switch_from: Option<usize>,
    /// Sound effects enabled (afplay)
    pub sound_on: bool,
    /// Voice alerts enabled (macOS say zh_TW)
    pub voice_on: bool,
    /// Unified sound queue with rate-limiting
    pub sound_queue: SoundQueue,
}

/// Serializable snapshot of AppState for the web API.
#[derive(Serialize)]
pub struct AppStateSnapshot {
    pub active_tab: usize,
    pub auto_switch: bool,
    pub paused: bool,
    pub sound_on: bool,
    pub voice_on: bool,
    pub total_events: u64,
    pub tabs: Vec<TabSnapshot>,
}

#[derive(Serialize)]
pub struct TabSnapshot {
    pub symbol: String,
    pub name: String,
    pub has_signal: bool,
    pub signal_dir: Option<String>,
    pub state: String,
    pub state_icon: String,
    pub action: String,
    pub action_label_zh: String,
    pub composite_score: i32,
    pub whale_buy_pts: i32,
    pub whale_sell_pts: i32,
    pub ignition_pts: i32,
    pub pressure_pts: i32,
    pub hmm_state: String,
    pub hmm_state_icon: String,
    pub hmm_agrees: bool,
    pub hmm_pts: i32,
    pub whale_buys: u64,
    pub whale_sells: u64,
    pub long_ignitions: u64,
    pub short_ignitions: u64,
    pub last_price: f64,
    pub change_pct: f64,
    pub total_ticks: u64,
    pub stop_loss: Option<f64>,
    pub last_sound: Option<SoundSnapshot>,
    pub order_book: Option<OrderBookSnapshot>,
    pub alerts: Vec<AlertSnapshot>,
    pub ticks: Vec<TickSnapshot>,
    // ── Anti-spoof metrics ─────────────────────────
    pub whale_buy_volume_m: f64,
    pub whale_sell_volume_m: f64,
    pub aggressive_buys: u32,
    pub aggressive_sells: u32,
    pub cluster_count: u32,
    pub trade_velocity: f64,
    pub suspicious_adds: u32,
    pub suspicious_removes: u32,
    pub absorption_score: f64,
    pub signal_confidence: f64,
    pub book_delta: crate::types::BookDeltaSummary,
}

#[derive(Serialize)]
pub struct SoundSnapshot {
    pub label: String,
    pub timestamp: String,
}

#[derive(Serialize)]
pub struct OrderBookSnapshot {
    pub asks: Vec<OrderLevelSnapshot>,
    pub bids: Vec<OrderLevelSnapshot>,
    pub last_price: f64,
    pub last_side: String,
    pub spread: f64,
    pub bid_pressure_pct: f64,
    pub ask_pressure_pct: f64,
}

#[derive(Serialize)]
pub struct OrderLevelSnapshot {
    pub price: f64,
    pub lots: u64,
    pub label: String,
}

#[derive(Serialize)]
pub struct AlertSnapshot {
    pub timestamp: String,
    pub icon: String,
    pub line1: String,
    pub line2: String,
    pub is_ignition: bool,
}

#[derive(Serialize)]
pub struct TickSnapshot {
    pub timestamp: String,
    pub side: String,
    pub arrow: String,
    pub price: f64,
    pub amount_m: f64,
    pub shares: u64,
    pub whale_tag: String,
}

impl AppState {
    pub fn new() -> Self {
        let views = STOCKS
            .iter()
            .take(TAB_COUNT)
            .map(|s| StockView::new(s.symbol, s.name, s.base_price, s.whale_threshold))
            .collect();

        Self {
            views,
            active_tab: 0,
            auto_switch: true,
            paused: false,
            total_events: 0,
            last_auto_switch_from: None,
            sound_on: true,
            voice_on: true,
            sound_queue: SoundQueue::new(),
        }
    }

    /// Process one feed event. Returns `Some(new_tab)` if tab should switch.
    pub fn process_event(&mut self, event: FeedEvent) -> Option<usize> {
        self.total_events += 1;

        match event {
            FeedEvent::Tick(tick) => {
                let sym = tick.symbol.clone();
                if let Some(idx) = self.views.iter().position(|v| v.symbol == sym) {
                    let (name, threshold) = {
                        let v = &self.views[idx];
                        (v.name.clone(), v.whale_threshold)
                    };

                    let is_whale = tick.amount >= threshold;
                    let tick_price = tick.price;
                    let tick_amount_m = tick.amount_m();
                    let tick_lots = tick.lots();
                    let tick_side = tick.side.clone();

                    let ignition = self.views[idx].process_tick(tick);

                    // Fire sound alert
                    if ignition.is_some() {
                        // Ignition — play ignition sound + TTS
                        let kind = match ignition.as_ref().unwrap().direction {
                            crate::types::PositionDir::Long => AlertKind::LongIgnition,
                            crate::types::PositionDir::Short => AlertKind::ShortIgnition,
                        };
                        let label = alert::fire_alert(
                            &self.sound_queue, self.sound_on, self.voice_on, &name, &kind,
                            0.0, tick_price, 0, None,
                        );
                        self.views[idx].last_sound = Some(crate::types::SoundInfo {
                            label,
                            timestamp: chrono::Local::now(),
                        });

                        // Auto-switch if enabled and not already on that tab
                        if self.auto_switch && idx != self.active_tab {
                            self.last_auto_switch_from = Some(self.active_tab);
                            self.active_tab = idx;
                            self.views[idx].clear_signal();
                            return Some(idx);
                        }
                    } else if is_whale {
                        // Regular whale trade — play whale sound + TTS
                        let kind = match tick_side {
                            crate::types::TradeSide::Buy => AlertKind::WhaleBuy(
                                crate::types::WhaleRank::from_amount(tick_amount_m * 1_000_000.0),
                            ),
                            crate::types::TradeSide::Sell => AlertKind::WhaleSell(
                                crate::types::WhaleRank::from_amount(tick_amount_m * 1_000_000.0),
                            ),
                        };
                        let label = alert::fire_alert(
                            &self.sound_queue, self.sound_on, self.voice_on, &name, &kind,
                            tick_amount_m, tick_price, tick_lots, None,
                        );
                        self.views[idx].last_sound = Some(crate::types::SoundInfo {
                            label,
                            timestamp: chrono::Local::now(),
                        });
                    }
                }
            }
            FeedEvent::Book(book) => {
                let sym = book.symbol.clone();
                if let Some(idx) = self.views.iter().position(|v| v.symbol == sym) {
                    self.views[idx].update_book(book);
                    self.views[idx].recompute();
                }
            }
        }
        None
    }

    /// Process a web command. Returns true if quit requested.
    pub fn process_command(&mut self, cmd: Command) -> bool {
        match cmd {
            Command::SwitchTab { index } => self.switch_tab(index),
            Command::NextTab => self.next_tab(),
            Command::PrevTab => self.prev_tab(),
            Command::TogglePause => self.toggle_pause(),
            Command::ToggleAuto => self.toggle_auto_switch(),
            Command::ToggleSound => self.toggle_sound(),
            Command::ToggleVoice => self.toggle_voice(),
            Command::ClearAlerts => self.clear_alerts(),
            Command::Quit => return true,
        }
        false
    }

    pub fn switch_tab(&mut self, idx: usize) {
        if idx < self.views.len() {
            self.active_tab = idx;
            self.views[idx].clear_signal();
            self.last_auto_switch_from = None;
        }
    }

    pub fn next_tab(&mut self) {
        self.switch_tab((self.active_tab + 1) % self.views.len());
    }

    pub fn prev_tab(&mut self) {
        let n = self.views.len();
        self.switch_tab((self.active_tab + n - 1) % n);
    }

    pub fn toggle_auto_switch(&mut self) {
        self.auto_switch = !self.auto_switch;
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn toggle_sound(&mut self) {
        self.sound_on = !self.sound_on;
    }

    pub fn toggle_voice(&mut self) {
        self.voice_on = !self.voice_on;
    }

    pub fn clear_alerts(&mut self) {
        if let Some(v) = self.views.get_mut(self.active_tab) {
            v.alerts.clear();
        }
    }

    pub fn active_view(&self) -> &StockView {
        &self.views[self.active_tab]
    }

    /// Create a serializable snapshot of the full state for the web API.
    pub fn snapshot(&self) -> AppStateSnapshot {
        AppStateSnapshot {
            active_tab: self.active_tab,
            auto_switch: self.auto_switch,
            paused: self.paused,
            sound_on: self.sound_on,
            voice_on: self.voice_on,
            total_events: self.total_events,
            tabs: self.views.iter().map(|v| {
                TabSnapshot {
                    symbol: v.symbol.clone(),
                    name: v.name.clone(),
                    has_signal: v.has_signal,
                    signal_dir: v.signal_dir.as_ref().map(|d| format!("{:?}", d)),
                    state: v.state.label().to_string(),
                    state_icon: v.state.icon().to_string(),
                    action: v.action.label().to_string(),
                    action_label_zh: v.action.label_zh().to_string(),
                    composite_score: v.composite.value,
                    whale_buy_pts: v.composite.whale_buy_pts,
                    whale_sell_pts: v.composite.whale_sell_pts,
                    ignition_pts: v.composite.ignition_pts,
                    pressure_pts: v.composite.pressure_pts,
                    hmm_state: v.hmm_state.label().to_string(),
                    hmm_state_icon: v.hmm_state.icon().to_string(),
                    hmm_agrees: v.hmm_confirmation.agrees_with_rules,
                    hmm_pts: v.composite.hmm_pts,
                    whale_buys: v.stats.whale_buys,
                    whale_sells: v.stats.whale_sells,
                    long_ignitions: v.stats.long_ignitions,
                    short_ignitions: v.stats.short_ignitions,
                    last_price: v.stats.last_price,
                    change_pct: v.stats.change_pct(),
                    total_ticks: v.stats.total_ticks,
                    stop_loss: v.stop_loss_price(),
                    last_sound: v.last_sound.as_ref().map(|s| SoundSnapshot {
                        label: s.label.clone(),
                        timestamp: s.timestamp.format("%H:%M:%S").to_string(),
                    }),
                    order_book: v.order_book.as_ref().map(|b| {
                        let total_bid: u64 = b.bids.iter().map(|l| l.lots).sum();
                        let total_ask: u64 = b.asks.iter().map(|l| l.lots).sum();
                        let total = (total_bid + total_ask) as f64;
                        let bid_pct = if total > 0.0 { total_bid as f64 / total * 100.0 } else { 50.0 };
                        OrderBookSnapshot {
                            asks: b.asks.iter().enumerate().map(|(i, l)| OrderLevelSnapshot {
                                price: l.price,
                                lots: l.lots,
                                label: if i == 0 { format!("賣1 ◄") } else { format!("賣{}", i + 1) },
                            }).collect(),
                            bids: b.bids.iter().enumerate().map(|(i, l)| OrderLevelSnapshot {
                                price: l.price,
                                lots: l.lots,
                                label: if i == 0 { "買1 ◄".to_string() } else { format!("買{}", i + 1) },
                            }).collect(),
                            last_price: b.last_price,
                            last_side: format!("{:?}", b.last_side),
                            spread: b.asks[0].price - b.bids[0].price,
                            bid_pressure_pct: bid_pct,
                            ask_pressure_pct: 100.0 - bid_pct,
                        }
                    }),
                    alerts: v.alerts.iter().map(|a| AlertSnapshot {
                        timestamp: a.timestamp.format("%H:%M:%S").to_string(),
                        icon: a.kind.icon().to_string(),
                        line1: a.line1.clone(),
                        line2: a.line2.clone(),
                        is_ignition: a.kind.is_ignition(),
                    }).collect(),
                    ticks: v.tick_log.iter().map(|t| {
                        let amount_m = t.amount_m();
                        let whale_tag = if amount_m >= 50.0 { "🐳MEGA" }
                            else if amount_m >= 20.0 { "🐋BIG" }
                            else if amount_m >= 5.0 { "🐋" }
                            else { "" };
                        TickSnapshot {
                            timestamp: t.timestamp.format("%H:%M:%S").to_string(),
                            side: format!("{:?}", t.side),
                            arrow: t.side.arrow().to_string(),
                            price: t.price,
                            amount_m,
                            shares: t.shares,
                            whale_tag: whale_tag.to_string(),
                        }
                    }).collect(),
                    // Anti-spoof metrics
                    whale_buy_volume_m: v.whale_metrics.whale_buy_volume_m,
                    whale_sell_volume_m: v.whale_metrics.whale_sell_volume_m,
                    aggressive_buys: v.whale_metrics.aggressive_buys,
                    aggressive_sells: v.whale_metrics.aggressive_sells,
                    cluster_count: v.whale_metrics.cluster_count,
                    trade_velocity: v.whale_metrics.trade_velocity,
                    suspicious_adds: v.whale_metrics.suspicious_adds,
                    suspicious_removes: v.whale_metrics.suspicious_removes,
                    absorption_score: v.whale_metrics.absorption_score,
                    signal_confidence: v.whale_metrics.signal_confidence,
                    book_delta: v.book_delta.clone(),
                }
            }).collect(),
        }
    }
}
