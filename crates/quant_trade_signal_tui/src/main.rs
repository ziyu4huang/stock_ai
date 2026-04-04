mod alert;
mod app;
mod detector;
mod mock_feed;
mod state_machine;
mod stock_view;
mod types;
mod ui;

use app::AppState;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{io, sync::mpsc, time::Duration};
use types::FeedEvent;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, rx) = mpsc::channel::<FeedEvent>();
    mock_feed::spawn(tx);

    let mut app = AppState::new();

    loop {
        terminal.draw(|f| ui::render(f, &app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        KeyCode::Char('p') | KeyCode::Char('P') => app.toggle_pause(),
                        KeyCode::Char('a') | KeyCode::Char('A') => app.toggle_auto_switch(),
                        KeyCode::Char('s') | KeyCode::Char('S') => app.toggle_sound(),
                        KeyCode::Char('v') | KeyCode::Char('V') => app.toggle_voice(),
                        KeyCode::Char(' ') => app.clear_alerts(),
                        // Tab switching by number
                        KeyCode::Char('1') => app.switch_tab(0),
                        KeyCode::Char('2') => app.switch_tab(1),
                        KeyCode::Char('3') => app.switch_tab(2),
                        KeyCode::Char('4') => app.switch_tab(3),
                        KeyCode::Char('5') => app.switch_tab(4),
                        // Arrow keys for prev/next tab
                        KeyCode::Right | KeyCode::Tab => app.next_tab(),
                        KeyCode::Left => app.prev_tab(),
                        _ => {}
                    }
                }
            }
        }

        if !app.paused {
            while let Ok(event) = rx.try_recv() {
                app.process_event(event);
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
