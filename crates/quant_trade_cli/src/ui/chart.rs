use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Color;
use ratatui::symbols::Marker;
use ratatui::widgets::canvas::{Canvas, Line, Rectangle, Context};
use ratatui::widgets::{Block, Borders};

use crate::data::types::Bar;
use crate::ui::theme;

/// Simplified chart renderer using only bar data.
/// Computes simple moving averages inline for overlay.
pub fn render_chart_simple(f: &mut Frame, area: Rect, bars: &[Bar]) {
    if bars.is_empty() {
        return;
    }
    let n = bars.len();

    // Simple moving averages
    let ema5 = compute_ema(&bars.iter().map(|b| b.close).collect::<Vec<_>>(), 5);
    let ema20 = compute_ema(&bars.iter().map(|b| b.close).collect::<Vec<_>>(), 20);

    // Bollinger Bands (20-period, 2σ)
    let bb_upper = vec![0.0; n];
    let bb_mid = vec![0.0; n];
    let bb_lower = vec![0.0; n];

    render_chart(f, area, bars, &bb_upper, &bb_mid, &bb_lower, &ema5, &ema20, &[], 0, " K線 ");
}

fn compute_ema(data: &[f64], period: usize) -> Vec<f64> {
    if data.len() < period {
        return vec![0.0; data.len()];
    }
    let k = 2.0 / (period as f64 + 1.0);
    let mut result = vec![0.0; data.len()];
    let mut sum = 0.0;
    for i in 0..period {
        sum += data[i];
    }
    result[period - 1] = sum / period as f64;
    for i in period..data.len() {
        result[i] = data[i] * k + result[i - 1] * (1.0 - k);
    }
    result
}

/// Render a candlestick chart with BB bands, EMA overlays, and signal markers.
pub fn render_chart(
    f: &mut Frame,
    area: Rect,
    bars: &[Bar],
    bb_upper: &[f64],
    bb_mid: &[f64],
    bb_lower: &[f64],
    ema5: &[f64],
    ema20: &[f64],
    signals: &[stock_core::BarSignals],
    offset: usize,
    title: &str,
) {
    let width = area.width as usize;
    let candle_width = 4;
    let visible_count = (width / candle_width).max(1).min(bars.len());

    let start = offset.min(bars.len().saturating_sub(visible_count));
    let end = (start + visible_count).min(bars.len());
    if start >= end {
        return;
    }

    let visible_bars = &bars[start..end];

    let mut y_min = f64::MAX;
    let mut y_max = f64::MIN;
    for i in start..end {
        if i < bb_upper.len() {
            y_min = y_min.min(bb_lower[i]);
            y_max = y_max.max(bb_upper[i]);
        }
        y_min = y_min.min(visible_bars[i - start].low);
        y_max = y_max.max(visible_bars[i - start].high);
    }
    let padding = (y_max - y_min) * 0.05;
    y_min -= padding;
    y_max += padding;
    if y_max <= y_min {
        y_max = y_min + 1.0;
    }

    let vol_max = visible_bars.iter().map(|b| b.volume).max().unwrap_or(1) as f64;
    let vol_height = (y_max - y_min) * 0.15;

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(title))
        .x_bounds([0.0, visible_count as f64])
        .y_bounds([y_min, y_max])
        .marker(Marker::HalfBlock)
        .paint(move |ctx: &mut Context| {
            // Volume bars
            for i in 0..visible_bars.len() {
                let x = i as f64;
                let vol_h = (visible_bars[i].volume as f64 / vol_max) * vol_height;
                ctx.draw(&Rectangle {
                    x,
                    y: y_min,
                    width: 0.8,
                    height: vol_h,
                    color: Color::DarkGray,
                });
            }

            // BB bands
            draw_line_series(ctx, start, bb_upper, Color::Yellow, visible_count);
            draw_line_series(ctx, start, bb_mid, Color::DarkGray, visible_count);
            draw_line_series(ctx, start, bb_lower, Color::Yellow, visible_count);

            // EMA lines
            draw_line_series(ctx, start, ema5, Color::Cyan, visible_count);
            draw_line_series(ctx, start, ema20, Color::Magenta, visible_count);

            // Candlesticks
            for i in 0..visible_bars.len() {
                let x = i as f64;
                let bar = &visible_bars[i];
                let bullish = bar.close >= bar.open;
                let color = if bullish { theme::BUY_COLOR } else { theme::SELL_COLOR };

                // Wick
                ctx.draw(&Line {
                    x1: x + 0.4, y1: bar.low,
                    x2: x + 0.4, y2: bar.high,
                    color,
                });

                // Body
                let body_top = bar.close.max(bar.open);
                let body_bot = bar.close.min(bar.open);
                let body_h = (body_top - body_bot).max((y_max - y_min) * 0.001);
                ctx.draw(&Rectangle {
                    x,
                    y: body_bot,
                    width: 0.8,
                    height: body_h,
                    color,
                });
            }

            // Signal markers
            for bs in signals {
                if bs.idx >= start && bs.idx < end {
                    let x = (bs.idx - start) as f64 + 0.4;
                    let bar = &bars[bs.idx];
                    for sig in &bs.signals {
                        let (marker_y, marker_color) = match sig.direction {
                            stock_core::SignalDirection::Buy => {
                                (bar.low - (y_max - y_min) * 0.02, theme::BUY_COLOR)
                            }
                            stock_core::SignalDirection::Sell => {
                                (bar.high + (y_max - y_min) * 0.02, theme::SELL_COLOR)
                            }
                            _ => continue,
                        };
                        ctx.print(x, marker_y, ratatui::text::Span::styled("▲", ratatui::style::Style::default().fg(marker_color)));
                    }
                }
            }
        });

    f.render_widget(canvas, area);
}

fn draw_line_series(
    ctx: &mut Context,
    start: usize,
    data: &[f64],
    color: Color,
    visible_count: usize,
) {
    for i in 0..visible_count.saturating_sub(1) {
        let idx1 = start + i;
        let idx2 = start + i + 1;
        if idx1 >= data.len() || idx2 >= data.len() {
            continue;
        }
        if data[idx1] == 0.0 || data[idx2] == 0.0 {
            continue;
        }
        ctx.draw(&Line {
            x1: i as f64 + 0.4,
            y1: data[idx1],
            x2: (i + 1) as f64 + 0.4,
            y2: data[idx2],
            color,
        });
    }
}
