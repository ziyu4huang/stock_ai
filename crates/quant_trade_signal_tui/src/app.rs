use crate::alert;
use crate::stock_view::{AlertKind, StockView};
use crate::types::{FeedEvent, TAB_COUNT, STOCKS};

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
                            self.sound_on, self.voice_on, &name, &kind,
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
                            self.sound_on, self.voice_on, &name, &kind,
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
}
