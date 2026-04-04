use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Gauge, Paragraph};

use crate::app::App;
use crate::data::types::Bar;
use crate::ui::chart;
use crate::ui::theme;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let symbol = match &app.replay_symbol {
        Some(s) => s.clone(),
        None => return,
    };

    // Layout: chart (65%) + bottom (35%)
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    // Convert replay bars to chart format
    let bars: Vec<Bar> = app
        .replay_bars
        .iter()
        .map(|ib| Bar {
            time: ib.time,
            open: ib.open,
            high: ib.high,
            low: ib.low,
            close: ib.close,
            volume: ib.volume,
        })
        .collect();

    if !bars.is_empty() {
        let closes: Vec<f64> = bars.iter().map(|b| b.close).collect();
        let (bb_upper, bb_mid, bb_lower) = stock_core::calc_bb_series(&closes, 20, 2.0);
        let ema5 = stock_core::calc_ema(&closes, 5);
        let ema20 = stock_core::calc_ema(&closes, 20);

        chart::render_chart(
            f,
            chunks[0],
            &bars,
            &bb_upper,
            &bb_mid,
            &bb_lower,
            &ema5,
            &ema20,
            &[],
            0,
            &format!(" {} — Replay ", symbol),
        );
    } else {
        let para = Paragraph::new("Press [n] or [Space] to start replay")
            .block(Block::default().borders(Borders::ALL).title(" Replay "));
        f.render_widget(para, chunks[0]);
    }

    // Bottom: controls (left) + P&L (right)
    let bottom = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    render_controls(f, bottom[0], app);
    render_pnl(f, bottom[1], app);
}

fn render_controls(f: &mut Frame, area: Rect, app: &App) {
    let total = app.replay_ticks.len();
    let cursor = app.replay_cursor;
    let pct = if total > 0 {
        (cursor as f64 / total as f64 * 100.0) as u16
    } else {
        0
    };

    let play_state = if app.replay_playing { "▶ PLAYING" } else { "⏸ PAUSED" };

    let text = format!(
        "{}  Speed: {}x\nCursor: {}/{} ({}%)\n[Space]Play/Pause [n]Step [c]Reset [+/-]Speed",
        play_state,
        app.replay_speed,
        cursor,
        total,
        pct,
    );

    let para = Paragraph::new(text)
        .style(Style::default().fg(Color::Cyan))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Controls "),
        );
    f.render_widget(para, area);
}

fn render_pnl(f: &mut Frame, area: Rect, app: &App) {
    let total_trades = app.replay_trades.len();
    let closed_trades: Vec<_> = app
        .replay_trades
        .iter()
        .filter(|t| t.pnl.is_some())
        .collect();
    let wins = closed_trades.iter().filter(|t| t.pnl.unwrap_or(0.0) > 0.0).count();
    let win_rate = if !closed_trades.is_empty() {
        wins as f64 / closed_trades.len() as f64 * 100.0
    } else {
        0.0
    };

    let pnl_color = if app.replay_pnl > 0.0 {
        theme::BUY_COLOR
    } else if app.replay_pnl < 0.0 {
        theme::SELL_COLOR
    } else {
        Color::Gray
    };

    let last_trade = app.replay_trades.last();

    let text = format!(
        "Trades: {}  Wins: {}  Rate: {:.0}%\nP&L: {:+.1} pts\n{}",
        total_trades,
        wins,
        win_rate,
        app.replay_pnl,
        last_trade.map(|t| {
            if t.exit_price.is_some() {
                format!(
                    "Last: {}@{:.1} → {:.1}",
                    match &t.direction {
                        stock_core::SignalDirection::Buy => "BUY",
                        _ => "SELL",
                    },
                    t.entry_price,
                    t.exit_price.unwrap()
                )
            } else {
                format!(
                    "Open: {}@{:.1}",
                    match &t.direction {
                        stock_core::SignalDirection::Buy => "BUY",
                        _ => "SELL",
                    },
                    t.entry_price
                )
            }
        }).unwrap_or_default(),
    );

    let para = Paragraph::new(text)
        .style(Style::default().fg(pnl_color))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" P&L "),
        );
    f.render_widget(para, area);
}
