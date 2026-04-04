use ratatui::Frame;
use ratatui::layout::{Rect, Layout, Constraint};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, Paragraph, Gauge, List, ListItem};
use ratatui::text::{Line, Span};

use crate::app::{App, SubView};
use crate::data::types::{AimingLevel, WhaleDirection};
use crate::ui::theme;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let sd = match app.current() {
        Some(s) => s,
        None => return,
    };

    let chunks = Layout::default()
        .constraints([
            Constraint::Length(3),  // aiming bar
            Constraint::Length(1),  // sub-view tabs
            Constraint::Min(0),     // content
        ])
        .split(area);

    render_aiming_bar(f, chunks[0], sd);
    render_subtabs(f, chunks[1], app);
    render_content(f, chunks[2], app, sd);
}

fn render_aiming_bar(f: &mut Frame, area: Rect, sd: &crate::app::StockData) {
    let (dir_icon, dir_color) = sd.whale.as_ref()
        .map(|w| match &w.whale_direction {
            WhaleDirection::Bull => ("▲多頭", theme::BUY_COLOR),
            WhaleDirection::Bear => ("▼空頭", theme::SELL_COLOR),
            WhaleDirection::Neutral => ("—中性", theme::NEUTRAL),
        })
        .unwrap_or(("—", Color::DarkGray));

    let price = sd.scan.last_price;
    let price_str = format!("{:.1}", price);

    if let Some(w) = &sd.whale {
        let level_color = match &w.aiming.level {
            AimingLevel::Critical => Color::Red,
            AimingLevel::High => Color::Yellow,
            AimingLevel::Medium => Color::Cyan,
            AimingLevel::Low => Color::DarkGray,
        };

        let score_text = format!(
            "{} | 瞄準:{} {}  基礎:{} 地緣:+{} 板塊:+{} 委託:+{} 脈衝:+{}",
            dir_icon,
            w.aiming.total_score,
            w.aiming.level.label(),
            w.aiming.base_score,
            w.aiming.geo_bonus,
            w.aiming.sector_bonus,
            w.aiming.orderbook_bonus,
            w.aiming.pulse_bonus,
        );

        let gauge = Gauge::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!(" {} {} {} ", sd.name, sd.symbol, price_str))
                    .title_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
                    .style(Style::default().fg(dir_color))
            )
            .gauge_style(Style::default().fg(level_color).bg(Color::Black))
            .percent(w.aiming.total_score as u16)
            .label(Span::styled(
                score_text,
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            ));
        f.render_widget(gauge, area);
    } else {
        let para = Paragraph::new(format!(" {} {} {} | 無大戶分析", sd.name, sd.symbol, price_str))
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(para, area);
    }
}

fn render_subtabs(f: &mut Frame, area: Rect, app: &App) {
    let tabs = [
        (" [c]K線 ", SubView::Chart),
        (" [w]大戶 ", SubView::Whale),
        (" [s]訊號 ", SubView::Signals),
    ];

    let mut spans: Vec<Span> = Vec::new();
    for (label, view) in tabs {
        if app.sub_view == view {
            spans.push(Span::styled(
                label,
                Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD),
            ));
        } else {
            spans.push(Span::styled(label, Style::default().fg(Color::DarkGray)));
        }
    }

    let para = Paragraph::new(Line::from(spans));
    f.render_widget(para, area);
}

fn render_content(
    f: &mut Frame,
    area: Rect,
    app: &App,
    sd: &crate::app::StockData,
) {
    match app.sub_view {
        SubView::Chart => render_chart_view(f, area, app, sd),
        SubView::Whale => render_whale_view(f, area, app, sd),
        SubView::Signals => render_signals_view(f, area, sd),
    }
}

fn render_chart_view(
    f: &mut Frame,
    area: Rect,
    app: &App,
    sd: &crate::app::StockData,
) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Percentage(65),
            Constraint::Percentage(35),
        ])
        .split(area);

    // Chart area
    let chart_block = Block::default()
        .borders(Borders::ALL)
        .title(" K線走勢 ")
        .title_style(Style::default().fg(Color::Cyan));
    let chart_inner = chart_block.inner(chunks[0]);
    f.render_widget(chart_block, chunks[0]);

    // Use the existing chart renderer
    crate::ui::chart::render_chart_simple(f, chart_inner, &sd.daily_bars);

    // Bottom: replay controls + stats
    let bottom_chunks = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])

        .split(chunks[1]);

    // Replay controls
    let replay_state = if app.replay_playing { "播放中" } else { "暫停" };
    let progress = if app.replay_ticks.is_empty() {
        0
    } else {
        app.replay_cursor * 100 / app.replay_ticks.len()
    };
    let replay_text = vec![
        Line::from(vec![
            Span::styled("模擬重播: ", Style::default().fg(Color::DarkGray)),
            Span::styled(replay_state, Style::default().fg(if app.replay_playing { Color::Green } else { Color::Yellow })),
        ]),
        Line::from(vec![
            Span::styled("進度: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}%", progress), Style::default().fg(Color::White)),
            Span::styled(format!(" ({}/{})", app.replay_cursor, app.replay_ticks.len()), Style::default().fg(Color::DarkGray)),
        ]),
        Line::from(vec![
            Span::styled("速度: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{}x", app.replay_speed), Style::default().fg(Color::White)),
        ]),
        Line::from(Span::styled(
            format!("損益: {:.1}", app.replay_pnl),
            Style::default().fg(if app.replay_pnl >= 0.0 { theme::BUY_COLOR } else { theme::SELL_COLOR }),
        )),
    ];
    let replay_para = Paragraph::new(replay_text)
        .block(Block::default().borders(Borders::ALL).title(" 重播控制 "));
    f.render_widget(replay_para, bottom_chunks[0]);

    // Quick stats
    let mut stats_lines = vec![
        Line::from(vec![
            Span::styled("收盤: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:.1}", sd.scan.last_price), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("分數: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:+}", sd.scan.score), Style::default().fg(theme::score_color(sd.scan.score))),
        ]),
        Line::from(vec![
            Span::styled("方向: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                match &sd.scan.direction {
                    stock_core::SignalDirection::Buy => "買進",
                    stock_core::SignalDirection::Sell => "賣出",
                    _ => "中立",
                },
                Style::default().fg(theme::score_color(sd.scan.score)),
            ),
        ]),
    ];

    // Add RSI / MACD if available
    if let Some(last) = sd.analysis.signals.last() {
        for sig in last.signals.iter().take(3) {
            stats_lines.push(Line::from(Span::styled(
                format!("• {}", sig.kind.name()),
                Style::default().fg(Color::Gray),
            )));
        }
    }

    let stats_para = Paragraph::new(stats_lines)
        .block(Block::default().borders(Borders::ALL).title(" 統計 "));
    f.render_widget(stats_para, bottom_chunks[1]);
}

fn render_whale_view(
    f: &mut Frame,
    area: Rect,
    app: &App,
    sd: &crate::app::StockData,
) {
    let w = match &sd.whale {
        Some(w) => w,
        None => {
            let msg = Paragraph::new(" 無大戶分析資料")
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(msg, area);
            return;
        }
    };

    let chunks = Layout::default()
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])

        .split(area);

    // ── Checklist ──
    let items: Vec<ListItem> = w.checklist.iter()
        .map(|item| {
            let icon = if item.passed { "✓" } else { "✗" };
            let icon_color = if item.passed { Color::Green } else { Color::Red };
            ListItem::new(Line::from(vec![
                Span::styled(format!(" {} ", icon), Style::default().fg(icon_color)),
                Span::styled(
                    format!("{:<20}", item.label),
                    Style::default().fg(Color::White),
                ),
                Span::styled(&item.detail, Style::default().fg(Color::DarkGray)),
            ]))
        })
        .collect();

    let checklist = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 執行清單 ")
            .title_style(Style::default().fg(Color::Yellow)));
    f.render_widget(checklist, chunks[0]);

    // ── Right side: pulse + stop loss ──
    let right_chunks = Layout::default()
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);

    // Energy pulse
    let pulse_info = if let Some(pulse) = &w.pulse {
        vec![
            Line::from(vec![
                Span::styled("模式: ", Style::default().fg(Color::DarkGray)),
                Span::styled(pulse.pattern.label(), Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("能量: ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{:.2}", pulse.rms_energy), Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("信心: ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{:.1}%", pulse.confidence * 100.0), Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("窗口: ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{}根K線", pulse.window_bars), Style::default().fg(Color::White)),
            ]),
        ]
    } else {
        vec![Line::from(Span::styled(" 無脈衝資料", Style::default().fg(Color::DarkGray)))]
    };
    let pulse_para = Paragraph::new(pulse_info)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 能量脈衝 ")
            .title_style(Style::default().fg(Color::Cyan)));
    f.render_widget(pulse_para, right_chunks[0]);

    // Stop loss status
    let sl_info = app.stop_loss_states.get(&sd.symbol)
        .map(|sl| {
            let pnl = sd.scan.last_price - sl.entry_price;
            let pnl_color = if pnl >= 0.0 { theme::BUY_COLOR } else { theme::SELL_COLOR };
            vec![
                Line::from(vec![
                    Span::styled("進場: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{:.1}", sl.entry_price), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("現價: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{:.1}", sd.scan.last_price), Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("損益: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(format!("{:+.1}", pnl), Style::default().fg(pnl_color)),
                ]),
            ]
        })
        .unwrap_or_else(|| {
            vec![Line::from(Span::styled(" 無部位 — 按 [a] 建立部位", Style::default().fg(Color::DarkGray)))]
        });
    let sl_para = Paragraph::new(sl_info)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 停損監控 ")
            .title_style(Style::default().fg(Color::Red)));
    f.render_widget(sl_para, right_chunks[1]);
}

fn render_signals_view(
    f: &mut Frame,
    area: Rect,
    sd: &crate::app::StockData,
) {
    let chunks = Layout::default()
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(area);

    // ── Signal list ──
    let mut signal_items: Vec<ListItem> = Vec::new();
    for bar_signals in sd.analysis.signals.iter().rev().take(20) {
        let time_str = &bar_signals.date;

        let signals_text = if bar_signals.signals.is_empty() {
            "—".to_string()
        } else {
            bar_signals.signals.iter()
                .map(|s| s.kind.name())
                .collect::<Vec<_>>()
                .join(", ")
        };

        let score = bar_signals.score;
        let score_color = theme::score_color(score);

        signal_items.push(ListItem::new(Line::from(vec![
            Span::styled(format!("{} ", time_str), Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:+4} ", score), Style::default().fg(score_color)),
            Span::styled(signals_text, Style::default().fg(Color::White)),
        ])));
    }

    if signal_items.is_empty() {
        signal_items.push(ListItem::new(Span::styled(" 無訊號資料", Style::default().fg(Color::DarkGray))));
    }

    let signals_list = List::new(signal_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 訊號歷史 ")
            .title_style(Style::default().fg(Color::Yellow)));
    f.render_widget(signals_list, chunks[0]);

    // ── Summary ──
    let mut summary_lines = vec![
        Line::from(vec![
            Span::styled("代碼: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&sd.symbol, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("名稱: ", Style::default().fg(Color::DarkGray)),
            Span::styled(&sd.name, Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("收盤: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:.1}", sd.scan.last_price), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("最新分數: ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:+}", sd.scan.score), Style::default().fg(theme::score_color(sd.scan.score))),
        ]),
    ];

    if let Some(w) = &sd.whale {
        summary_lines.push(Line::from(vec![
            Span::styled("瞄準分數: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} ({})", w.aiming.total_score, w.aiming.level.label()),
                Style::default().fg(Color::White),
            ),
        ]));
        let (dir_icon, dir_color) = match &w.whale_direction {
            WhaleDirection::Bull => ("▲多頭", theme::BUY_COLOR),
            WhaleDirection::Bear => ("▼空頭", theme::SELL_COLOR),
            WhaleDirection::Neutral => ("—中性", theme::NEUTRAL),
        };
        summary_lines.push(Line::from(vec![
            Span::styled("方向: ", Style::default().fg(Color::DarkGray)),
            Span::styled(dir_icon, Style::default().fg(dir_color)),
        ]));
    }

    // Top signals
    summary_lines.push(Line::from(""));
    summary_lines.push(Line::from(Span::styled("近期訊號:", Style::default().fg(Color::Yellow))));
    for sig in sd.scan.top_signals.iter().take(6) {
        summary_lines.push(Line::from(Span::styled(
            format!(" • {}", sig),
            Style::default().fg(Color::Gray),
        )));
    }

    let summary_para = Paragraph::new(summary_lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 摘要 ")
            .title_style(Style::default().fg(Color::Cyan)));
    f.render_widget(summary_para, chunks[1]);
}
