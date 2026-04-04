use crate::app::AppState;
use crate::stock_view::StockView;
use crate::types::{HmmState, PositionDir};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

// ── Top-level render ──────────────────────────────────────────────────────────

pub fn render(f: &mut Frame, app: &AppState) {
    let area = f.area();

    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // tab bar
            Constraint::Min(0),     // main content
            Constraint::Length(1),  // status bar
        ])
        .split(area);

    render_tab_bar(f, app, rows[0]);
    render_main(f, app, rows[1]);
    render_status_bar(f, app, rows[2]);
}

// ── Tab bar ───────────────────────────────────────────────────────────────────

fn render_tab_bar(f: &mut Frame, app: &AppState, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray))
        .title(" 🐋 WHALE RADAR v2 — 做多▲ / 做空▼ ");

    let _inner = block.inner(area);

    let mut spans: Vec<Span> = Vec::new();
    spans.push(Span::raw(" "));

    for (i, view) in app.views.iter().enumerate() {
        let is_active = i == app.active_tab;
        let has_sig = view.has_signal;

        // Signal indicator: 🔥 if ignition, * if any unread alert
        let sig_icon = if has_sig {
            match view.signal_dir {
                Some(PositionDir::Long) => "🔥▲",
                Some(PositionDir::Short) => "🔥▼",
                None => "★",
            }
        } else {
            ""
        };

        let label = format!("[{}:{} {}{}]  ", i + 1, view.symbol, view.name, sig_icon);

        let style = if is_active {
            Style::default()
                .fg(Color::Black)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else if has_sig {
            Style::default()
                .fg(Color::LightRed)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        spans.push(Span::styled(label, style));
    }

    // Auto-switch indicator
    let auto_label = if app.auto_switch {
        "  [A]◉ "
    } else {
        "  [A]○ "
    };
    spans.push(Span::styled(
        auto_label,
        Style::default()
            .fg(if app.auto_switch { Color::Green } else { Color::DarkGray })
            .add_modifier(Modifier::BOLD),
    ));

    // Sound indicator
    let sound_label = if app.sound_on { "🔊" } else { "🔇" };
    spans.push(Span::styled(
        format!(" [S]{}", sound_label),
        Style::default().fg(if app.sound_on { Color::Cyan } else { Color::DarkGray }),
    ));

    // Voice indicator
    let voice_label = if app.voice_on { "🗣" } else { "  " };
    spans.push(Span::styled(
        format!(" [V]{}", voice_label),
        Style::default().fg(if app.voice_on { Color::Yellow } else { Color::DarkGray }),
    ));

    if app.paused {
        spans.push(Span::styled(
            "  ⏸ PAUSED",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
        ));
    }

    let line = Paragraph::new(Line::from(spans)).block(block);
    f.render_widget(line, area);
}

// ── Main content area — 3-column layout ───────────────────────────────────────

fn render_main(f: &mut Frame, app: &AppState, area: Rect) {
    let view = app.active_view();

    // If terminal is wide enough (≥100 cols), use 3-column layout
    // Otherwise fall back to 2-column
    if area.width >= 100 {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(38), Constraint::Percentage(22), Constraint::Percentage(40)])
            .split(area);

        // Left column: order book (full height)
        render_order_book(f, view, cols[0]);

        // Middle column: signal radar top + alerts bottom
        let mid = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(cols[1]);

        render_signal_radar(f, view, mid[0]);
        render_alerts(f, view, mid[1]);

        // Right column: tick feed (full height)
        render_tick_feed(f, view, cols[2]);
    } else {
        // Fallback: 2-column (original layout)
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(42), Constraint::Percentage(58)])
            .split(area);

        let left = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(cols[0]);

        let right = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
            .split(cols[1]);

        render_order_book(f, view, left[0]);
        render_signal_radar(f, view, left[1]);
        render_tick_feed(f, view, right[0]);
        render_alerts(f, view, right[1]);
    }
}

// ── Order book (五檔) ─────────────────────────────────────────────────────────

fn render_order_book(f: &mut Frame, view: &StockView, area: Rect) {
    let title = format!(" ORDER BOOK 五檔  {} {} ", view.symbol, view.name);
    let block = Block::default()
        .title(title.as_str())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let bar_width = (block.inner(area).width as usize).saturating_sub(28).max(10);

    let mut lines: Vec<Line> = Vec::new();

    match &view.order_book {
        None => {
            lines.push(Line::from(Span::styled(
                "  Waiting for order book...",
                Style::default().fg(Color::DarkGray),
            )));
        }
        Some(book) => {
            // Find max lots across all levels for proportional bars
            let max_lots = book.asks.iter().chain(book.bids.iter())
                .map(|l| l.lots)
                .max()
                .unwrap_or(1)
                .max(1);

            lines.push(Line::from(""));

            // ── Asks (賣) — show in reverse: ask5 first, ask1 last ──────────
            for i in (0..5).rev() {
                let level = book.asks[i];
                let bar_len = (bar_width as u64 * level.lots / max_lots).max(1) as usize;
                let bar = "░".repeat(bar_len);
                let rank_label = if i == 0 { "賣1 ◄" } else { &format!("賣{} ", i + 1) };

                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  {:4}  {:>8.2}  {:>5}張  ", rank_label, level.price, level.lots),
                        Style::default().fg(Color::Red),
                    ),
                    Span::styled(bar, Style::default().fg(Color::Red)),
                ]));
            }

            // ── Spread line ──────────────────────────────────────────────────
            let spread = book.asks[0].price - book.bids[0].price;
            let last_arrow = book.last_side.arrow();
            lines.push(Line::from(vec![
                Span::styled(
                    format!("  ──── {:.2} {}  spread {:.2} ────────────────────",
                        book.last_price, last_arrow, spread),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
            ]));

            // ── Bids (買) — show bid1 first ──────────────────────────────────
            for i in 0..5 {
                let level = book.bids[i];
                let bar_len = (bar_width as u64 * level.lots / max_lots).max(1) as usize;
                let bar = "█".repeat(bar_len);
                let rank_label = if i == 0 { "買1 ◄" } else { &format!("買{} ", i + 1) };

                lines.push(Line::from(vec![
                    Span::styled(
                        format!("  {:4}  {:>8.2}  {:>5}張  ", rank_label, level.price, level.lots),
                        Style::default().fg(Color::Green),
                    ),
                    Span::styled(bar, Style::default().fg(Color::Green)),
                ]));
            }

            lines.push(Line::from(""));

            // ── Buy/Sell pressure ratio ──────────────────────────────────────
            let total_bid: u64 = book.bids.iter().map(|l| l.lots).sum();
            let total_ask: u64 = book.asks.iter().map(|l| l.lots).sum();
            let total = (total_bid + total_ask) as f64;
            let bid_pct = if total > 0.0 { total_bid as f64 / total * 100.0 } else { 50.0 };
            let ask_pct = 100.0 - bid_pct;
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(format!("買壓 {:.0}%", bid_pct), Style::default().fg(Color::Green)),
                Span::raw("  /  "),
                Span::styled(format!("賣壓 {:.0}%", ask_pct), Style::default().fg(Color::Red)),
            ]));
        }
    }

    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}

// ── Signal radar (NEW — middle column) ────────────────────────────────────────

fn render_signal_radar(f: &mut Frame, view: &StockView, area: Rect) {
    let score = &view.composite;
    let action = &view.action;
    let state = &view.state;

    let border_color = if score.value > 30 { Color::LightGreen }
        else if score.value < -30 { Color::LightRed }
        else { Color::DarkGray };

    let block = Block::default()
        .title(" SIGNAL RADAR 信號雷達 ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color));

    let inner = block.inner(area);
    let w = inner.width as usize;
    let mut lines: Vec<Line> = Vec::new();

    // ── Composite score bar ──────────────────────────────────────────────
    lines.push(Line::from(""));

    // Score bar: show [-100, +100] as a bar with center marker
    let bar_w = w.saturating_sub(14);
    let mid = bar_w / 2;
    let score_pos = ((score.value as i64 + 100) as usize * bar_w / 200).min(bar_w - 1);
    let mut bar_chars: Vec<char> = vec! ['─'; bar_w];
    bar_chars[mid] = '│';
    if score_pos < bar_w {
        if score.value >= 0 {
            for i in mid..=score_pos {
                if i < bar_w { bar_chars[i] = '▓'; }
            }
        } else {
            for i in score_pos..=mid {
                if i < bar_w { bar_chars[i] = '▓'; }
            }
        }
    }

    let score_color = if score.value > 0 { Color::LightGreen } else if score.value < 0 { Color::LightRed } else { Color::Gray };
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(format!("{:+4}", score.value), Style::default().fg(score_color).add_modifier(Modifier::BOLD)),
        Span::raw(" "),
        Span::styled(
            bar_chars.iter().collect::<String>(),
            Style::default().fg(score_color),
        ),
    ]));

    // ── Market state ─────────────────────────────────────────────────────
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(state.icon(), Style::default().fg(state.color())),
        Span::raw(" "),
        Span::styled(state.label(), Style::default().fg(state.color()).add_modifier(Modifier::BOLD)),
    ]));

    // ── HMM state ────────────────────────────────────────────────────────
    let hmm = &view.hmm_state;
    let agrees = view.hmm_confirmation.agrees_with_rules;
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled("HMM", Style::default().fg(Color::DarkGray)),
        Span::raw(" "),
        Span::styled(hmm.icon(), Style::default().fg(hmm.color())),
        Span::raw(" "),
        Span::styled(hmm.label(), Style::default().fg(hmm.color())),
        Span::raw("  "),
        Span::styled(
            if agrees { "✓ AGREE" } else { "✗ DIV" },
            Style::default().fg(if agrees { Color::Green } else { Color::Yellow }),
        ),
    ]));

    lines.push(Line::from(""));

    // ── Action advice ────────────────────────────────────────────────────
    let action_style = Style::default().fg(action.color()).add_modifier(Modifier::BOLD);
    let action_box = format!(" {} ", action.label_zh());
    lines.push(Line::from(vec![
        Span::raw(" ┌─ ACTION ────────┐"),
    ]));
    lines.push(Line::from(vec![
        Span::raw(" │"),
        Span::styled(action_box, action_style),
        Span::raw("│"),
    ]));

    // Stop-loss
    if let Some(sl) = view.stop_loss_price() {
        let sl_pct = (sl - view.stats.last_price) / view.stats.last_price * 100.0;
        let sl_str = format!(" S/L: {:.2} ({:+.1}%)", sl, sl_pct);
        lines.push(Line::from(vec![
            Span::raw(" │"),
            Span::styled(sl_str, Style::default().fg(Color::Yellow)),
            Span::raw("│"),
        ]));
    } else {
        lines.push(Line::from(vec![
            Span::raw(" │"),
            Span::styled(" S/L: —", Style::default().fg(Color::DarkGray)),
            Span::raw("          │"),
        ]));
    }
    lines.push(Line::from(vec![
        Span::raw(" └─────────────────┘"),
    ]));

    lines.push(Line::from(""));

    // ── Whale counters with mini bars ────────────────────────────────────
    let s = &view.stats;
    let max_w = std::cmp::max(std::cmp::max(s.whale_buys, s.whale_sells),
        std::cmp::max(s.long_ignitions, s.short_ignitions)).max(1);
    let bar_max = (w as usize).saturating_sub(26).max(5);

    lines.push(sline_counter("  WhaleBuy ▲ ", s.whale_buys, max_w, bar_max, Color::LightGreen));
    lines.push(sline_counter("  WhaleSell▼ ", s.whale_sells, max_w, bar_max, Color::LightRed));
    lines.push(sline_counter("  IgnLong  ▲ ", s.long_ignitions, max_w, bar_max, Color::Green));
    lines.push(sline_counter("  IgnShort ▼ ", s.short_ignitions, max_w, bar_max, Color::Red));

    // ── Price info ───────────────────────────────────────────────────────
    lines.push(Line::from(""));
    let chg = s.change_pct();
    let chg_color = if chg > 0.0 { Color::LightGreen } else if chg < 0.0 { Color::LightRed } else { Color::Gray };
    lines.push(Line::from(vec![
        Span::raw("  "),
        Span::styled(format!("{:.2}", s.last_price), Style::default().fg(Color::White).add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled(format!("{:+.2}%", chg), Style::default().fg(chg_color)),
        Span::raw("  "),
        Span::styled(format!("Ticks:{}", s.total_ticks), Style::default().fg(Color::DarkGray)),
    ]));

    // ── Last sound ───────────────────────────────────────────────────────
    if let Some(ref info) = view.last_sound {
        lines.push(Line::from(""));
        let time_str = info.timestamp.format("%H:%M:%S").to_string();
        lines.push(Line::from(vec![
            Span::styled("  🔊 ", Style::default().fg(Color::Cyan)),
            Span::styled(&info.label, Style::default().fg(Color::Cyan)),
            Span::styled(format!(" {}", time_str), Style::default().fg(Color::DarkGray)),
        ]));
    }

    let para = Paragraph::new(lines).block(block);
    f.render_widget(para, area);
}

fn sline_counter<'a>(label: &'a str, count: u64, max_val: u64, bar_max: usize, color: Color) -> Line<'a> {
    let bar_len = if max_val > 0 { (count as usize * bar_max / max_val as usize).min(bar_max) } else { 0 };
    let bar: String = "█".repeat(bar_len);
    Line::from(vec![
        Span::styled(label, Style::default().fg(Color::DarkGray)),
        Span::styled(bar, Style::default().fg(color)),
        Span::styled(format!(" {}", count), Style::default().fg(color).add_modifier(Modifier::BOLD)),
    ])
}

// ── Alerts (per stock) ────────────────────────────────────────────────────────

fn render_alerts(f: &mut Frame, view: &StockView, area: Rect) {
    let block = Block::default()
        .title(" ALERTS ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    let max_rows = inner.height as usize;
    let max_alerts = (max_rows / 3).max(1);

    let mut items: Vec<ListItem> = Vec::new();

    for alert in view.alerts.iter().take(max_alerts) {
        let color = alert.kind.color();
        let icon = alert.kind.icon();
        let time_str = alert.timestamp.format("%H:%M:%S").to_string();

        items.push(ListItem::new(Line::from(Span::styled(
            format!("{} {} ", icon, time_str),
            Style::default().fg(Color::DarkGray),
        ))));
        items.push(ListItem::new(Line::from(Span::styled(
            format!("  {}", alert.line1),
            Style::default().fg(color).add_modifier(
                if alert.kind.is_ignition() { Modifier::BOLD } else { Modifier::empty() }
            ),
        ))));
        items.push(ListItem::new(Line::from(Span::styled(
            alert.line2.clone(),
            Style::default().fg(Color::Gray),
        ))));
    }

    if items.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "  Watching for whale activity...",
            Style::default().fg(Color::DarkGray),
        ))));
    }

    f.render_widget(List::new(items).block(block), area);
}

// ── Tick feed (single-line compact) ───────────────────────────────────────────

fn render_tick_feed(f: &mut Frame, view: &StockView, area: Rect) {
    let feed_title = format!(" TICK FEED  {} ", view.symbol);
    let block = Block::default()
        .title(feed_title.as_str())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let max_ticks = block.inner(area).height as usize;
    let mut items: Vec<ListItem> = Vec::new();

    for tick in view.tick_log.iter().take(max_ticks) {
        let side_color = tick.side.color();
        let amount_m = tick.amount_m();
        let is_whale = amount_m >= 5.0;
        let time_str = tick.timestamp.format("%H:%M:%S").to_string();

        let whale_tag = if amount_m >= 50.0 { " 🐳MEGA" }
            else if amount_m >= 20.0 { " 🐋BIG " }
            else if amount_m >= 5.0 { " 🐋    " }
            else { "" };

        // Compact single line: time ▲ price  whale_tag  amount  shares
        items.push(ListItem::new(Line::from(vec![
            Span::styled(format!("{} ", time_str), Style::default().fg(Color::DarkGray)),
            Span::styled(
                tick.side.arrow(),
                Style::default().fg(side_color).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("{:>8.2}", tick.price),
                Style::default().fg(side_color),
            ),
            Span::styled(
                whale_tag,
                Style::default().fg(if amount_m >= 50.0 { Color::Magenta }
                    else if amount_m >= 20.0 { Color::Yellow }
                    else if is_whale { Color::Cyan }
                    else { Color::DarkGray }),
            ),
            Span::styled(
                format!("{:>5.1}M", amount_m),
                Style::default().fg(if is_whale { Color::Yellow } else { Color::DarkGray }),
            ),
            Span::styled(
                format!(" {:>5}sh", tick.shares),
                Style::default().fg(Color::DarkGray),
            ),
        ])));
    }

    if items.is_empty() {
        items.push(ListItem::new(Line::from(Span::styled(
            "  Starting tick feed...",
            Style::default().fg(Color::DarkGray),
        ))));
    }

    f.render_widget(List::new(items).block(block), area);
}

// ── Status bar ────────────────────────────────────────────────────────────────

fn render_status_bar(f: &mut Frame, app: &AppState, area: Rect) {
    let now = chrono::Local::now().format("%H:%M:%S").to_string();
    let status = if app.paused { "[PAUSED]" } else { "[LIVE]  " };
    let status_color = if app.paused { Color::Yellow } else { Color::Green };

    let line = Line::from(vec![
        Span::styled(status, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled("[1-5]Tab", Style::default().fg(Color::Gray)),
        Span::raw("  "),
        Span::styled("[←→]", Style::default().fg(Color::Gray)),
        Span::raw("  "),
        Span::styled("[A]Auto", Style::default().fg(Color::Gray)),
        Span::raw("  "),
        Span::styled(
            format!("[S]Sound{}", if app.sound_on { "◉" } else { "○" }),
            Style::default().fg(if app.sound_on { Color::Cyan } else { Color::DarkGray }),
        ),
        Span::raw("  "),
        Span::styled(
            format!("[V]Voice{}", if app.voice_on { "◉" } else { "○" }),
            Style::default().fg(if app.voice_on { Color::Yellow } else { Color::DarkGray }),
        ),
        Span::raw("  "),
        Span::styled("[P]Pause", Style::default().fg(Color::Gray)),
        Span::raw("  "),
        Span::styled("[Space]Clear", Style::default().fg(Color::Gray)),
        Span::raw("  "),
        Span::styled("[Q]Quit", Style::default().fg(Color::Gray)),
        Span::raw("  "),
        Span::styled(
            format!("Ev:{} {}", app.total_events, now),
            Style::default().fg(Color::DarkGray),
        ),
    ]);

    f.render_widget(Paragraph::new(line).style(Style::default().bg(Color::Black)), area);
}
