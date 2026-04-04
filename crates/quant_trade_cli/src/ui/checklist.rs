use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListItem};

use crate::data::types::ChecklistItem;

#[allow(dead_code)]
pub fn render(f: &mut Frame, area: Rect, items: &[ChecklistItem]) {
    let list_items: Vec<ListItem> = items
        .iter()
        .map(|item| {
            let check = if item.passed { "[x]" } else { "[ ]" };
            let color = if item.passed {
                Color::Green
            } else {
                Color::DarkGray
            };
            let text = format!(
                "{} {:<26} {}",
                check,
                item.label,
                item.detail
            );
            ListItem::new(text).style(Style::default().fg(color))
        })
        .collect();

    let list = List::new(list_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Execution Checklist "),
    );
    f.render_widget(list, area);
}
