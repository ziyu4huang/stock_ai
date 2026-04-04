use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Block, Borders, Row, Table};

use crate::app::App;
use crate::data::types::AimingLevel;
use crate::ui::theme;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let header = Row::new(vec!["Symbol", "Name", "Price", "Score", "Dir", "Aiming", "Signals"])
        .style(Style::default().add_modifier(Modifier::BOLD));

    let rows: Vec<Row> = app
        .scan_results
        .iter()
        .enumerate()
        .map(|(i, r)| {
            let score_str = format!("{:+}", r.score);
            let dir_str = match &r.direction {
                stock_core::SignalDirection::Buy => "BUY".to_string(),
                stock_core::SignalDirection::Sell => "SEL".to_string(),
                _ => "—".to_string(),
            };
            let signals_str = r.top_signals.join(", ");

            // Aiming score from whale data
            let (aiming_str, aiming_color) = app
                .stock_data
                .get(&r.symbol)
                .and_then(|sd| sd.whale.as_ref())
                .map(|w| {
                    let level = &w.aiming.level;
                    let score = w.aiming.total_score;
                    let marker = match level {
                        AimingLevel::Critical | AimingLevel::High => "■",
                        _ => "□",
                    };
                    let color = match level {
                        AimingLevel::Critical => ratatui::style::Color::Red,
                        AimingLevel::High => ratatui::style::Color::Yellow,
                        AimingLevel::Medium => ratatui::style::Color::Cyan,
                        AimingLevel::Low => ratatui::style::Color::DarkGray,
                    };
                    (format!("{}{}", score, marker), color)
                })
                .unwrap_or(("—".into(), ratatui::style::Color::DarkGray));

            let aiming_cell = ratatui::widgets::Cell::from(
                ratatui::text::Span::styled(aiming_str, ratatui::style::Style::default().fg(aiming_color))
            );
            let row = Row::new(vec![
                ratatui::widgets::Cell::from(r.symbol.as_str()),
                ratatui::widgets::Cell::from(r.name.as_str()),
                ratatui::widgets::Cell::from(format!("{:.1}", r.last_price)),
                ratatui::widgets::Cell::from(score_str),
                ratatui::widgets::Cell::from(dir_str),
                aiming_cell,
                ratatui::widgets::Cell::from(signals_str),
            ]);

            if i == app.scan_selected {
                row.style(theme::selected_style())
            } else {
                row.style(theme::score_style(r.score))
            }
        })
        .collect();

    let widths = [
        ratatui::layout::Constraint::Length(10),
        ratatui::layout::Constraint::Length(12),
        ratatui::layout::Constraint::Length(10),
        ratatui::layout::Constraint::Length(6),
        ratatui::layout::Constraint::Length(4),
        ratatui::layout::Constraint::Length(7),
        ratatui::layout::Constraint::Min(20),
    ];

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Scan List — Day Trading Scanner "),
        )
        .highlight_style(theme::selected_style());

    f.render_widget(table, area);
}
