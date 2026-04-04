use std::collections::HashMap;

use rusqlite::Connection;

use crate::alert;
use crate::data::db;
use crate::data::fake_tick::FakeTickSource;
use crate::data::geographic::GeoHelper;
use crate::data::types::*;
use crate::data::watchlist::default_watchlist;
use crate::whale::aiming::compute_aiming;
use crate::whale::energy::EnergyAnalyzer;
use crate::whale::spike::SpikeDetector;

use crossterm::event::{KeyCode, KeyEvent};
use stock_core::{SignalDirection, ScanResult};

/// Sub-view within a stock dashboard.
#[derive(Debug, Clone, PartialEq)]
pub enum SubView {
    Chart,
    Whale,
    Signals,
}

/// Top-level mode.
#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    /// Overview showing all 5 stocks as cards.
    Overview,
    /// Individual stock dashboard.
    Dashboard,
}

pub struct StockData {
    pub symbol: String,
    pub name: String,
    pub daily_bars: Vec<Bar>,
    pub tick_source: FakeTickSource,
    pub analysis: stock_core::DayTradeAnalysis,
    pub scan: ScanResult,
    pub whale: Option<WhaleAnalysis>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReplayTrade {
    pub entry_idx: usize,
    pub entry_price: f64,
    pub exit_idx: Option<usize>,
    pub exit_price: Option<f64>,
    pub direction: SignalDirection,
    pub pnl: Option<f64>,
}

pub struct App {
    pub mode: Mode,
    pub running: bool,

    // Stock tabs (always 5)
    pub stocks: Vec<StockData>,
    pub current_stock: usize,  // 0..4
    pub sub_view: SubView,

    // Replay state (per-stock)
    pub replay_ticks: Vec<Tick>,
    pub replay_cursor: usize,
    pub replay_playing: bool,
    pub replay_speed: usize,
    pub replay_bars: Vec<IntradayBar>,
    pub replay_trades: Vec<ReplayTrade>,
    pub replay_pnl: f64,

    // Whale detection shared state
    pub spike_detector: SpikeDetector,
    #[allow(dead_code)]
    pub energy_analyzers: HashMap<String, EnergyAnalyzer>,
    pub stop_loss_states: HashMap<String, StopLossState>,

    // Alert
    pub alert_config: AlertConfig,

    // Database
    pub db: Connection,

    // Loading
    pub loading: bool,
    pub status_msg: String,

    // HTTP
    pub client: reqwest::Client,
}

#[allow(dead_code)]
const MAX_STOCKS: usize = 5;

impl App {
    pub fn new() -> Self {
        let db_path = std::env::var("HOME")
            .map(|h| format!("{}/.stock_ai/geo.db", h))
            .unwrap_or_else(|_| ":memory:".into());
        let db = Connection::open(&db_path)
            .and_then(|conn| {
                db::init_db(&conn)?;
                db::seed_data(&conn)?;
                Ok(conn)
            })
            .unwrap_or_else(|_| {
                let conn = Connection::open_in_memory().unwrap();
                let _ = db::init_db(&conn);
                let _ = db::seed_data(&conn);
                conn
            });

        Self {
            mode: Mode::Overview,
            running: true,
            stocks: Vec::new(),
            current_stock: 0,
            sub_view: SubView::Chart,
            replay_ticks: Vec::new(),
            replay_cursor: 0,
            replay_playing: false,
            replay_speed: 5,
            replay_bars: Vec::new(),
            replay_trades: Vec::new(),
            replay_pnl: 0.0,
            spike_detector: SpikeDetector::default(),
            energy_analyzers: HashMap::new(),
            stop_loss_states: HashMap::new(),
            alert_config: AlertConfig {
                enabled: false,
                min_level: AimingLevel::High,
            },
            db,
            loading: true,
            status_msg: "載入中...".into(),
            client: reqwest::Client::new(),
        }
    }

    /// Get current stock data (if loaded).
    pub fn current(&self) -> Option<&StockData> {
        self.stocks.get(self.current_stock)
    }

    /// Get current stock data mutably.
    #[allow(dead_code)]
    pub fn current_mut(&mut self) -> Option<&mut StockData> {
        self.stocks.get_mut(self.current_stock)
    }

    pub async fn load_data(&mut self) {
        let watchlist = default_watchlist();
        let mut handles = Vec::new();
        for entry in &watchlist {
            let client = self.client.clone();
            let symbol = entry.symbol.clone();
            let name = entry.name.clone();
            handles.push(tokio::spawn(async move {
                let bars = fetch_daily_bars(&client, &symbol, 90).await;
                bars.ok().map(|b| (symbol, name, b))
            }));
        }

        let mut results: Vec<(String, String, Vec<Bar>)> = Vec::new();
        for handle in handles {
            if let Ok(Some((symbol, name, bars))) = handle.await {
                if !bars.is_empty() {
                    results.push((symbol, name, bars));
                }
            }
        }

        // Feed all volumes into spike detector
        for (symbol, _, bars) in &results {
            for bar in bars {
                self.spike_detector.check(symbol, bar.volume, bar.time);
            }
        }

        // Build analysis for each stock
        for (symbol, name, bars) in results {
            let tick_source = FakeTickSource::new(&symbol, bars.clone());
            let analysis = analyze_bars(&bars);
            let scan = build_scan_result(&symbol, &name, &bars, &analysis);
            let whale = self.compute_whale(&symbol, &bars);

            if let Some(ref w) = &whale {
                alert::maybe_alert(
                    &self.alert_config,
                    &symbol,
                    &name,
                    w.aiming.total_score,
                    &w.aiming.level,
                    &w.whale_direction,
                );
            }

            self.stocks.push(StockData {
                symbol,
                name,
                daily_bars: bars,
                tick_source,
                analysis,
                scan,
                whale,
            });
        }

        self.loading = false;
        let voice_state = if self.alert_config.enabled { "開" } else { "關" };
        self.status_msg = format!(
            "已載入 {} 檔 | [1-5]切換 [v]語音:{} | q:離開",
            self.stocks.len(),
            voice_state,
        );
    }

    fn compute_whale(&self, symbol: &str, bars: &[Bar]) -> Option<WhaleAnalysis> {
        if bars.is_empty() {
            return None;
        }

        let last_bar = bars.last()?;
        let spike = self.spike_detector.query(symbol, last_bar.volume, last_bar.time);

        let volumes: Vec<f64> = bars.iter().rev().take(20).map(|b| b.volume as f64).collect();
        let mut analyzer = EnergyAnalyzer::default();
        for &v in volumes.iter().rev() {
            analyzer.analyze(v, symbol, last_bar.time);
        }
        let pulse = analyzer.analyze(last_bar.volume as f64, symbol, last_bar.time);

        let period = format_date(last_bar.time);
        let geo_bonus = GeoHelper::geo_bonus(&self.db, symbol, &period);
        let sector_count = GeoHelper::sector_cluster_count(&self.db, symbol, &period);

        let aiming = compute_aiming(spike.as_ref(), Some(&pulse), geo_bonus, sector_count, 1.0);

        let whale_direction = {
            let price_rising = bars.len() >= 3 && {
                let n = bars.len();
                bars[n - 1].close > bars[n - 3].close
            };
            match &pulse.pattern {
                PulsePattern::Staircase | PulsePattern::Accumulation => {
                    if price_rising { WhaleDirection::Bull } else { WhaleDirection::Neutral }
                }
                PulsePattern::Distribution => WhaleDirection::Bear,
                _ => {
                    if price_rising { WhaleDirection::Bull } else { WhaleDirection::Bear }
                }
            }
        };

        let checklist = vec![
            ChecklistItem {
                label: "量能突波 >= 3σ".into(),
                passed: spike.is_some(),
                detail: spike.as_ref().map(|s| format!("z={:.1}", s.z_score)).unwrap_or_else(|| "—".into()),
            },
            ChecklistItem {
                label: "能量脈衝確認".into(),
                passed: pulse.pattern == PulsePattern::Staircase || pulse.pattern == PulsePattern::Accumulation,
                detail: pulse.pattern.label().into(),
            },
            ChecklistItem {
                label: "地緣籌碼匹配".into(),
                passed: geo_bonus > 0,
                detail: format!("+{}", geo_bonus),
            },
            ChecklistItem {
                label: "板塊聚集".into(),
                passed: sector_count >= 2,
                detail: format!("{}/3", sector_count),
            },
            ChecklistItem {
                label: "委託簿支撐".into(),
                passed: true,
                detail: "N/A".into(),
            },
            ChecklistItem {
                label: "無對手大戶".into(),
                passed: true,
                detail: "—".into(),
            },
        ];

        Some(WhaleAnalysis {
            symbol: symbol.to_string(),
            aiming,
            checklist,
            recent_spikes: spike.into_iter().collect(),
            pulse: Some(pulse),
            whale_direction,
        })
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.running = false;
            }
            // Switch to stock tabs 1-5
            KeyCode::Char(c) if c >= '1' && c <= '5' => {
                let idx = (c as usize).saturating_sub('1' as usize);
                if idx < self.stocks.len() {
                    self.current_stock = idx;
                    self.mode = Mode::Dashboard;
                    self.sub_view = SubView::Chart;
                    self.reset_replay();
                }
            }
            KeyCode::Char('0') | KeyCode::Esc => {
                self.mode = Mode::Overview;
                self.replay_playing = false;
            }
            KeyCode::Tab => {
                self.current_stock = (self.current_stock + 1) % self.stocks.len().max(1);
                if self.mode == Mode::Overview {
                    self.mode = Mode::Dashboard;
                    self.sub_view = SubView::Chart;
                }
                self.reset_replay();
            }
            KeyCode::BackTab => {
                if self.current_stock > 0 {
                    self.current_stock -= 1;
                } else {
                    self.current_stock = self.stocks.len().saturating_sub(1);
                }
                if self.mode == Mode::Overview {
                    self.mode = Mode::Dashboard;
                    self.sub_view = SubView::Chart;
                }
                self.reset_replay();
            }
            KeyCode::Char('v') => {
                self.alert_config.enabled = !self.alert_config.enabled;
                let s = if self.alert_config.enabled { "開啟" } else { "關閉" };
                self.status_msg = format!("語音告警：{}", s);
            }
            _ => match self.mode {
                Mode::Overview => self.handle_overview_key(key),
                Mode::Dashboard => self.handle_dashboard_key(key),
            },
        }
    }

    fn handle_overview_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Down | KeyCode::Char('j') => {
                if self.current_stock + 1 < self.stocks.len() {
                    self.current_stock += 1;
                }
            }
            KeyCode::Up | KeyCode::Char('k') => {
                if self.current_stock > 0 {
                    self.current_stock -= 1;
                }
            }
            KeyCode::Enter => {
                self.mode = Mode::Dashboard;
                self.sub_view = SubView::Chart;
                self.reset_replay();
            }
            _ => {}
        }
    }

    fn handle_dashboard_key(&mut self, key: KeyEvent) {
        match key.code {
            // Sub-view switching
            KeyCode::Char('c') | KeyCode::Char('C') => {
                self.sub_view = SubView::Chart;
                self.reset_replay();
            }
            KeyCode::Char('w') | KeyCode::Char('W') => {
                self.sub_view = SubView::Whale;
                self.replay_playing = false;
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.sub_view = SubView::Signals;
                self.replay_playing = false;
            }
            // Chart scrolling
            KeyCode::Left | KeyCode::Char('h') => {
                // handled in chart sub-view if needed
            }
            KeyCode::Right | KeyCode::Char('l') => {}
            // Replay controls (only in chart sub-view)
            KeyCode::Char(' ') => {
                if self.sub_view == SubView::Chart {
                    self.replay_playing = !self.replay_playing;
                }
            }
            KeyCode::Char('n') => {
                if self.sub_view == SubView::Chart {
                    self.replay_step();
                }
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.replay_speed = (self.replay_speed + 1).min(50);
            }
            KeyCode::Char('-') => {
                self.replay_speed = self.replay_speed.saturating_sub(1).max(1);
            }
            KeyCode::Char('r') => {
                if self.sub_view == SubView::Chart {
                    self.replay_playing = false;
                }
            }
            KeyCode::Char('a') => {
                if self.sub_view == SubView::Whale {
                    if let Some(sd) = self.stocks.get(self.current_stock) {
                        let sym = sd.symbol.clone();
                        let close = sd.daily_bars.last().map(|b| b.close);
                        let time = sd.daily_bars.last().map(|b| b.time);
                        let dir = sd.analysis.latest_direction.clone();
                        if let (Some(price), Some(t)) = (close, time) {
                            self.stop_loss_states.insert(
                                sym.clone(),
                                StopLossState {
                                    entry_price: price,
                                    entry_time: t,
                                    direction: dir,
                                    max_favorable: price,
                                },
                            );
                            self.status_msg = format!(
                                "建立部位：{} @ {:.1}",
                                sym, price
                            );
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn reset_replay(&mut self) {
        if let Some(sd) = self.current() {
            self.replay_ticks = sd.tick_source.ticks.clone();
        } else {
            self.replay_ticks.clear();
        }
        self.replay_cursor = 0;
        self.replay_playing = false;
        self.replay_bars.clear();
        self.replay_trades.clear();
        self.replay_pnl = 0.0;
    }

    pub fn replay_step(&mut self) {
        if self.replay_cursor >= self.replay_ticks.len() {
            self.replay_playing = false;
            return;
        }
        let tick = self.replay_ticks[self.replay_cursor].clone();
        self.replay_cursor += 1;

        let bar_time = tick.timestamp - (tick.timestamp % 60);
        if self.replay_bars.is_empty()
            || self.replay_bars.last().map(|b| b.time) != Some(bar_time)
        {
            self.replay_bars.push(IntradayBar {
                time: bar_time,
                open: tick.price,
                high: tick.price,
                low: tick.price,
                close: tick.price,
                volume: tick.volume,
                vwap: tick.price,
            });
        } else {
            let bar = self.replay_bars.last_mut().unwrap();
            bar.high = bar.high.max(tick.price);
            bar.low = bar.low.min(tick.price);
            bar.close = tick.price;
            bar.volume += tick.volume;
        }

        let bar_count = self.replay_bars.len();
        if bar_count > 26 && bar_count % 5 == 0 {
            let analysis = analyze_intraday(&self.replay_bars);
            let score = analysis.latest_score;

            let in_position = self
                .replay_trades
                .last()
                .map(|t| t.exit_price.is_none())
                .unwrap_or(false);

            if !in_position && score > 30 {
                self.replay_trades.push(ReplayTrade {
                    entry_idx: self.replay_cursor,
                    entry_price: tick.price,
                    exit_idx: None,
                    exit_price: None,
                    direction: SignalDirection::Buy,
                    pnl: None,
                });
            } else if in_position && score < -30 {
                if let Some(last) = self.replay_trades.last_mut() {
                    last.exit_idx = Some(self.replay_cursor);
                    last.exit_price = Some(tick.price);
                    last.pnl = Some(tick.price - last.entry_price);
                    self.replay_pnl += tick.price - last.entry_price;
                }
            }
        }
    }

    pub fn replay_advance(&mut self) {
        for _ in 0..self.replay_speed {
            if self.replay_cursor < self.replay_ticks.len() {
                self.replay_step();
            } else {
                self.replay_playing = false;
                break;
            }
        }
    }
}

// ── Helper functions ──────────────────────────────────────────────────────

async fn fetch_daily_bars(
    client: &reqwest::Client,
    symbol: &str,
    days: u32,
) -> Result<Vec<Bar>, String> {
    let url = format!(
        "https://query1.finance.yahoo.com/v8/finance/chart/{}?range={}d&interval=1d",
        symbol, days
    );
    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    let result = json
        .get("chart")
        .and_then(|c| c.get("result"))
        .and_then(|r| r.get(0))
        .ok_or("No chart data")?;

    let timestamps = result.get("timestamp").and_then(|t| t.as_array()).ok_or("No timestamps")?;
    let quotes = result
        .get("indicators")
        .and_then(|i| i.get("quote"))
        .and_then(|q| q.get(0))
        .ok_or("No quotes")?;

    let opens = quotes.get("open").and_then(|v| v.as_array());
    let highs = quotes.get("high").and_then(|v| v.as_array());
    let lows = quotes.get("low").and_then(|v| v.as_array());
    let closes = quotes.get("close").and_then(|v| v.as_array());
    let volumes = quotes.get("volume").and_then(|v| v.as_array());

    let (Some(opens), Some(highs), Some(lows), Some(closes), Some(volumes)) =
        (opens, highs, lows, closes, volumes)
    else {
        return Ok(vec![]);
    };

    let mut bars = Vec::new();
    for i in 0..timestamps.len() {
        let time = timestamps[i].as_i64().unwrap_or(0);
        let open = opens[i].as_f64().unwrap_or(0.0);
        let high = highs[i].as_f64().unwrap_or(0.0);
        let low = lows[i].as_f64().unwrap_or(0.0);
        let close = closes[i].as_f64().unwrap_or(0.0);
        let volume = volumes[i].as_i64().unwrap_or(0);
        if open > 0.0 && close > 0.0 {
            bars.push(Bar { time, open, high, low, close, volume });
        }
    }
    Ok(bars)
}

fn analyze_bars(bars: &[Bar]) -> stock_core::DayTradeAnalysis {
    let core_bars: Vec<stock_core::Bar> = bars.iter().map(|b| b.into()).collect();
    stock_core::analyze_daytrade(&core_bars)
}

fn analyze_intraday(bars: &[IntradayBar]) -> stock_core::DayTradeAnalysis {
    let core_bars: Vec<stock_core::Bar> = bars
        .iter()
        .map(|b| stock_core::Bar {
            time: b.time,
            open: b.open,
            high: b.high,
            low: b.low,
            close: b.close,
            volume: b.volume,
        })
        .collect();
    stock_core::analyze_daytrade(&core_bars)
}

fn build_scan_result(
    symbol: &str,
    name: &str,
    bars: &[Bar],
    analysis: &stock_core::DayTradeAnalysis,
) -> ScanResult {
    let last_price = bars.last().map(|b| b.close).unwrap_or(0.0);
    let top_signals: Vec<String> = analysis
        .signals
        .last()
        .map(|bs| bs.signals.iter().map(|s| s.kind.name().to_string()).collect())
        .unwrap_or_default();

    ScanResult {
        symbol: symbol.to_string(),
        name: name.to_string(),
        last_price,
        score: analysis.latest_score,
        direction: analysis.latest_direction.clone(),
        top_signals,
    }
}

fn format_date(timestamp: i64) -> String {
    chrono::DateTime::from_timestamp(timestamp, 0)
        .map(|dt| dt.format("%Y-%m").to_string())
        .unwrap_or_default()
}
