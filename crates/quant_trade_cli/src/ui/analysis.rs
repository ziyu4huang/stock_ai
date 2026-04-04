use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Modifier, Style, Color};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table};

use crate::app::App;
use crate::ui::chart;
use crate::ui::theme;

pub fn render(f: &mut Frame, area: Rect, app: &App) {
    let symbol = match &app.analysis_symbol {
        Some(s) => s.clone(),
        None => return,
    };

    let sd = match app.stock_data.get(&symbol) {
        Some(d) => d,
        None => return,
    };

    let bars = &sd.daily_bars;
    let analysis = &sd.analysis;

    // Layout: chart (65%) + bottom (35%)
    let chunks = Layout::default()
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    // Compute indicators for chart
    let closes: Vec<f64> = bars.iter().map(|b| b.close).collect();
    let (_, _, bb_upper) = stock_core::calc_bb_series(&closes, 20, 2.0);
    let (bb_upper, bb_mid, bb_lower) = stock_core::calc_bb_series(&closes, 20, 2.0);
    let ema5 = stock_core::calc_ema(&closes, 5);
    let ema20 = stock_core::calc_ema(&closes, 20);

    // Convert signals for chart
    let core_signals: Vec<stock_core::BarSignals> = analysis.signals.clone();

    chart::render_chart(
        f,
        chunks[0],
        bars,
        &bb_upper,
        &bb_mid,
        &bb_lower,
        &ema5,
        &ema20,
        &core_signals,
        app.analysis_chart_offset,
        &format!(" {} — Daily Analysis ", symbol),
    );

    // Bottom: signals list (left) + stats (right)
    let bottom = Layout::default()
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    render_signals_list(f, bottom[0], analysis);
    render_stats(f, bottom[1], app, &symbol);
}

fn render_signals_list(
    f: &mut Frame,
    area: Rect,
    analysis: &stock_core::DayTradeAnalysis,
) {
    let items: Vec<ListItem> = analysis
        .signals
        .last()
        .map(|bs| {
            bs.signals
                .iter()
                .map(|s| {
                    let color = match &s.direction {
                        stock_core::SignalDirection::Buy => theme::BUY_COLOR,
                        stock_core::SignalDirection::Sell => theme::SELL_COLOR,
                        _ => Color::Gray,
                    };
                    let text = format!(
                        "{:<20} {:>4} {:+}",
                        s.kind.name(),
                        match &s.direction {
                            stock_core::SignalDirection::Buy => "BUY",
                            stock_core::SignalDirection::Sell => "SEL",
                            _ => "—",
                        },
                        s.score
                    );
                    ListItem::new(text).style(Style::default().fg(color))
                })
                .collect()
        })
        .unwrap_or_default();

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Signals "),
    );
    f.render_widget(list, area);
}

fn render_stats(f: &mut Frame, area: Rect, app: &App, symbol: &str) {
    let sd = match app.stock_data.get(symbol) {
        Some(d) => d,
        None => return,
    };
    let bars = &sd.daily_bars;
    let analysis = &sd.analysis;
    let last_price = bars.last().map(|b| b.close).unwrap_or(0.0);

    let closes: Vec<f64> = bars.iter().map(|b| b.close).collect();
    let rsi = stock_core::calc_rsi(&closes, 14);
    let (macd_line, _, _) = stock_core::calc_macd(&closes);
    let (bb_upper, _, bb_lower) = stock_core::calc_bb(&closes, 20, 2.0);

    let score_color = theme::score_color(analysis.latest_score);

    let mut lines = vec![
        format!("Price: {:.1}  Score: {:+}", last_price, analysis.latest_score),
        format!("RSI(14): {:.1}  MACD: {:+.1}", rsi, macd_line),
        format!("BB: [{:.0} - {:.0}]", bb_lower, bb_upper),
    ];

    if let Some(whale) = &sd.whale {
        lines.push(format!(
            "Aiming: {} {}",
            whale.aiming.total_score,
            whale.aiming.level.label()
        ));
    }

    let text = lines.join("\n");
    let para = Paragraph::new(text)
        .style(Style::default().fg(score_color))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Stats "),
        );
    f.render_widget(para, area);
}
