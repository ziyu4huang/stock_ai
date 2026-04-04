use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Gauge, List, ListItem, Paragraph};

use crate::app::App;
use crate::data::types::{AimingLevel, WhaleDirection};
use crate::ui::checklist;
use crate::ui::theme;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let symbol = match &app.whale_symbol {
        Some(s) => s.clone(),
        None => return,
    };

    let sd = match app.stock_data.get(&symbol) {
        Some(d) => d,
        None => return,
    };

    let whale = match &sd.whale {
        Some(w) => w,
        None => {
            let para = Paragraph::new("No whale data available")
                .block(Block::default().borders(Borders::ALL).title(" Whale "));
            f.render_widget(para, area);
            return;
        }
    };

    // Layout: aiming bar (3 lines) + mid (checklist + brokers) + stop loss (3 lines)
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(5),
            Constraint::Min(0),
            Constraint::Length(5),
        ])
        .split(area);

    render_aiming(f, chunks[0], &whale.aiming, &symbol, &whale.whale_direction);
    render_mid(f, chunks[1], app, &symbol, whale);
    render_stop_loss(f, chunks[2], app, &symbol);
}

fn render_aiming(
    f: &mut Frame,
    area: Rect,
    aiming: &crate::data::types::AimingResult,
    symbol: &str,
    direction: &WhaleDirection,
) {
    let level_color = match aiming.level {
        AimingLevel::Critical => Color::Red,
        AimingLevel::High => Color::Yellow,
        AimingLevel::Medium => Color::Cyan,
        AimingLevel::Low => Color::DarkGray,
    };

    let dir_icon = match direction {
        WhaleDirection::Bull => "▲ BULL",
        WhaleDirection::Bear => "▼ BEAR",
        WhaleDirection::Neutral => "— NEUTRAL",
    };
    let dir_color = match direction {
        WhaleDirection::Bull => theme::BUY_COLOR,
        WhaleDirection::Bear => theme::SELL_COLOR,
        WhaleDirection::Neutral => Color::DarkGray,
    };

    let score_text = format!(
        "{} | Aiming:{} {}  Base:{} Geo:+{} Sector:+{} OB:+{} Pulse:+{}",
        dir_icon,
        aiming.total_score,
        aiming.level.label(),
        aiming.base_score,
        aiming.geo_bonus,
        aiming.sector_bonus,
        aiming.orderbook_bonus,
        aiming.pulse_bonus,
    );

    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!(" {} — Whale Hunting ", symbol))
                .style(Style::default().fg(dir_color)),
        )
        .gauge_style(Style::default().fg(level_color).bg(Color::Black))
        .percent(aiming.total_score as u16)
        .label(score_text);

    f.render_widget(gauge, area);
}

fn render_mid(
    f: &mut Frame,
    area: Rect,
    app: &App,
    symbol: &str,
    whale: &crate::data::types::WhaleAnalysis,
) {
    let mid = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Checklist
    checklist::render(f, mid[0], &whale.checklist);

    // Broker details
    render_brokers(f, mid[1], app, symbol);
}

fn render_brokers(f: &mut Frame, area: Rect, app: &App, symbol: &str) {
    let period = app
        .stock_data
        .get(symbol)
        .and_then(|sd| sd.daily_bars.last())
        .map(|b| {
            chrono::NaiveDateTime::from_timestamp_opt(b.time, 0)
                .map(|dt| dt.format("%Y-%m").to_string())
                .unwrap_or_default()
        })
        .unwrap_or_default();

    let details = crate::data::geographic::GeoHelper::get_broker_details(&app.db, symbol, &period, 5);

    let items: Vec<ListItem> = if details.is_empty() {
        vec![ListItem::new("No broker data (seeded DB)")]
    } else {
        details
            .iter()
            .map(|(name, net)| {
                let color = if *net > 0.0 {
                    theme::BUY_COLOR
                } else {
                    theme::SELL_COLOR
                };
                let text = format!(
                    "{:<16} Net {:>+10.0} NTD",
                    name,
                    net / 1_000_000.0 * 1_000_000.0 // format as-is
                );
                ListItem::new(text).style(Style::default().fg(color))
            })
            .collect()
    };

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Top Brokers "),
    );
    f.render_widget(list, area);
}

fn render_stop_loss(f: &mut Frame, area: Rect, app: &App, symbol: &str) {
    let text = if let Some(state) = app.stop_loss_states.get(symbol) {
        let ts = crate::whale::stop_loss::tick_size(state.entry_price);
        format!(
            "Entry: {:.1}  Dir: {:?}  Time: {}\n2-Tick: {:.1}  15min stop  Trailing: {:.1} (0.5%)",
            state.entry_price,
            state.direction,
            state.entry_time,
            state.entry_price - 2.0 * ts,
            state.entry_price * 0.995,
        )
    } else {
        "No position open — press [a] to open position".into()
    };

    let para = Paragraph::new(text)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Stop Loss "),
        );
    f.render_widget(para, area);
}
