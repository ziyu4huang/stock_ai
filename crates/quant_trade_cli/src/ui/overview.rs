use ratatui::Frame;
use ratatui::layout::{Rect, Layout, Constraint};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Gauge};
use ratatui::text::{Line, Span};

use crate::app::App;
use crate::data::types::{AimingLevel, WhaleDirection};
use crate::ui::theme;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    if app.stocks.is_empty() {
        let msg = Paragraph::new(" 載入中...")
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(msg, area);
        return;
    }

    // Split into equal columns for each stock
    let constraints: Vec<Constraint> = app.stocks.iter()
        .map(|_| Constraint::Percentage(20))
        .collect();
    let columns = Layout::default()
        .constraints(constraints)
        .split(area);

    for (i, sd) in app.stocks.iter().enumerate() {
        render_stock_card(f, columns[i], sd, i, i == app.current_stock);
    }
}

fn render_stock_card(
    f: &mut Frame,
    area: Rect,
    sd: &crate::app::StockData,
    index: usize,
    selected: bool,
) {
    let border_color = if selected { Color::Cyan } else { Color::DarkGray };
    let border_style = Style::default().fg(border_color);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" [{}] {} ", index + 1, sd.name))
        .border_style(border_style);

    let inner = block.inner(area);
    f.render_widget(block, area);

    if inner.height < 8 {
        return;
    }

    let chunks = Layout::default()
        .constraints([
            Constraint::Length(1),  // price
            Constraint::Length(1),  // score
            Constraint::Length(1),  // direction + aiming
            Constraint::Length(1),  // aiming gauge
            Constraint::Min(0),     // signals
        ])
        .split(inner);

    // ── Price line ──
    let price = sd.scan.last_price;
    let change = sd.daily_bars.len().ge(&2).then(|| {
        let n = sd.daily_bars.len();
        sd.daily_bars[n - 1].close - sd.daily_bars[n - 2].close
    }).unwrap_or(0.0);
    let change_pct = sd.daily_bars.len().ge(&2).then(|| {
        let n = sd.daily_bars.len();
        (sd.daily_bars[n - 1].close - sd.daily_bars[n - 2].close) / sd.daily_bars[n - 2].close * 100.0
    }).unwrap_or(0.0);

    let (change_icon, change_color) = if change > 0.0 {
        ("▲", theme::BUY_COLOR)
    } else if change < 0.0 {
        ("▼", theme::SELL_COLOR)
    } else {
        ("—", theme::NEUTRAL)
    };

    let price_line = Line::from(vec![
        Span::styled(format!("{:.1} ", price), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::styled(
            format!("{}{:.1}%", change_icon, change_pct.abs()),
            Style::default().fg(change_color),
        ),
    ]);
    f.render_widget(Paragraph::new(price_line), chunks[0]);

    // ── Score line ──
    let score = sd.scan.score;
    let score_color = theme::score_color(score);
    let score_line = Line::from(vec![
        Span::styled("分數:", Style::default().fg(Color::DarkGray)),
        Span::styled(format!("{:+}", score), Style::default().fg(score_color).add_modifier(Modifier::BOLD)),
    ]);
    f.render_widget(Paragraph::new(score_line), chunks[1]);

    // ── Direction + Aiming line ──
    let (dir_icon, dir_color, aiming_str, aiming_color) = sd.whale.as_ref()
        .map(|w| {
            let (icon, color) = match &w.whale_direction {
                WhaleDirection::Bull => ("▲多頭", theme::BUY_COLOR),
                WhaleDirection::Bear => ("▼空頭", theme::SELL_COLOR),
                WhaleDirection::Neutral => ("—中性", theme::NEUTRAL),
            };
            let level = &w.aiming.level;
            let marker = match level {
                AimingLevel::Critical | AimingLevel::High => "■",
                _ => "□",
            };
            let lcolor = match level {
                AimingLevel::Critical => Color::Red,
                AimingLevel::High => Color::Yellow,
                AimingLevel::Medium => Color::Cyan,
                AimingLevel::Low => Color::DarkGray,
            };
            (icon, color, format!("{}{}", w.aiming.total_score, marker), lcolor)
        })
        .unwrap_or(("—", Color::DarkGray, "—".into(), Color::DarkGray));

    let dir_line = Line::from(vec![
        Span::styled(format!("{} ", dir_icon), Style::default().fg(dir_color).add_modifier(Modifier::BOLD)),
        Span::styled(&aiming_str, Style::default().fg(aiming_color)),
    ]);
    f.render_widget(Paragraph::new(dir_line), chunks[2]);

    // ── Aiming gauge ──
    if let Some(w) = &sd.whale {
        let level_color = match &w.aiming.level {
            AimingLevel::Critical => Color::Red,
            AimingLevel::High => Color::Yellow,
            AimingLevel::Medium => Color::Cyan,
            AimingLevel::Low => Color::DarkGray,
        };
        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(level_color).bg(Color::Black))
            .percent(w.aiming.total_score as u16);
        f.render_widget(gauge, chunks[3]);
    }

    // ── Signals ──
    let signals: Vec<Line> = sd.scan.top_signals.iter()
        .take(4)
        .map(|s| Line::from(Span::styled(
            format!("• {}", truncate_signal(s)),
            Style::default().fg(Color::Gray),
        )))
        .collect();
    let signals_para = Paragraph::new(signals);
    f.render_widget(signals_para, chunks[4]);
}

fn truncate_signal(s: &str) -> String {
    if s.len() > 18 {
        format!("{}...", &s[..15])
    } else {
        s.to_string()
    }
}
