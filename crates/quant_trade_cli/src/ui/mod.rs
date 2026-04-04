pub mod theme;
pub mod chart;
pub mod overview;
pub mod dashboard;
pub mod checklist;

use ratatui::Frame;
use ratatui::layout::Rect;
use crate::app::{App, Mode};

pub fn render(f: &mut Frame, app: &App) {
    let size = f.area();

    use ratatui::layout::{Layout, Constraint};
    let chunks = Layout::default()
        .constraints([
            Constraint::Length(1),  // header (tabs)
            Constraint::Min(0),     // main content
            Constraint::Length(1),  // footer (status)
        ])
        .split(size);

    render_header(f, chunks[0], app);
    render_footer(f, chunks[2], app);

    match app.mode {
        Mode::Overview => overview::render(f, chunks[1], app),
        Mode::Dashboard => dashboard::render(f, chunks[1], app),
    }
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    use ratatui::widgets::Paragraph;
    use ratatui::style::{Style, Color, Modifier};
    use ratatui::text::{Span, Line};

    let mut spans: Vec<Span> = Vec::new();

    // Overview tab
    let overview_active = app.mode == Mode::Overview;
    spans.push(Span::styled(
        " 總覽 ",
        if overview_active {
            Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        },
    ));

    spans.push(Span::raw("│"));

    // Stock tabs 1-5
    for (i, sd) in app.stocks.iter().enumerate() {
        let is_active = app.mode == Mode::Dashboard && app.current_stock == i;
        let label = format!(" [{}]{} ", i + 1, sd.name);

        let style = if is_active {
            match sd.whale.as_ref().map(|w| &w.whale_direction) {
                Some(crate::data::types::WhaleDirection::Bull) => {
                    Style::default().fg(Color::Black).bg(theme::BUY_COLOR).add_modifier(Modifier::BOLD)
                }
                Some(crate::data::types::WhaleDirection::Bear) => {
                    Style::default().fg(Color::Black).bg(theme::SELL_COLOR).add_modifier(Modifier::BOLD)
                }
                _ => {
                    Style::default().fg(Color::Black).bg(Color::Cyan).add_modifier(Modifier::BOLD)
                }
            }
        } else {
            Style::default().fg(Color::DarkGray)
        };

        spans.push(Span::styled(label, style));
        if i < app.stocks.len() - 1 {
            spans.push(Span::raw(" "));
        }
    }

    let para = Paragraph::new(Line::from(spans));
    f.render_widget(para, area);
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    use ratatui::widgets::Paragraph;
    use ratatui::style::{Style, Color};

    let text = if app.loading {
        " 載入資料中...".to_string()
    } else {
        let voice = if app.alert_config.enabled { "開" } else { "關" };
        let keys = match app.mode {
            Mode::Overview => "[j/k]選擇 [Enter]進入 [1-5]切換",
            Mode::Dashboard => "[c]K線 [w]大戶 [s]訊號 [Space]播放 [+/-]速度",
        };
        format!(
            "{}檔 | {} | [v]語音:{} | [0/Esc]總覽 q:離開",
            app.stocks.len(),
            keys,
            voice,
        )
    };
    let para = Paragraph::new(text)
        .style(Style::default().fg(Color::DarkGray).bg(Color::Black));
    f.render_widget(para, area);
}
