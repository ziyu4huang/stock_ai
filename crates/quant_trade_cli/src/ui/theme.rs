use ratatui::style::{Color, Modifier, Style};

// Taiwan convention: red = up/buy, green = down/sell
#[allow(dead_code)]
pub const BG: Color = Color::Reset;
#[allow(dead_code)]
pub const TEXT: Color = Color::White;
#[allow(dead_code)]
pub const DIM: Color = Color::DarkGray;
#[allow(dead_code)]
pub const ACCENT: Color = Color::Cyan;
pub const BUY_COLOR: Color = Color::Red;
pub const SELL_COLOR: Color = Color::Green;
pub const NEUTRAL: Color = Color::Gray;
#[allow(dead_code)]
pub const HIGHLIGHT_BG: Color = Color::DarkGray;

#[allow(dead_code)]
pub fn score_style(score: i32) -> Style {
    if score > 30 {
        Style::default().fg(BUY_COLOR).add_modifier(Modifier::BOLD)
    } else if score > 0 {
        Style::default().fg(BUY_COLOR)
    } else if score < -30 {
        Style::default().fg(SELL_COLOR).add_modifier(Modifier::BOLD)
    } else if score < 0 {
        Style::default().fg(SELL_COLOR)
    } else {
        Style::default().fg(NEUTRAL)
    }
}

pub fn score_color(score: i32) -> Color {
    if score > 0 {
        BUY_COLOR
    } else if score < 0 {
        SELL_COLOR
    } else {
        NEUTRAL
    }
}

#[allow(dead_code)]
pub fn selected_style() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(Color::Cyan)
        .add_modifier(Modifier::BOLD)
}
