mod alert;
mod app;
mod detector;
mod hmm;
mod mock_feed;
mod state_machine;
mod stock_view;
mod types;
mod ui;
mod web;

use app::{AppState, Command};
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::sync::{Arc, RwLock, mpsc};
use std::time::Duration;
use types::{FeedEvent, OrderBook, OrderLevel, Tick, TradeSide};

#[derive(Parser, Debug)]
#[command(name = "quant_trade_signal_tui", about = "🐋 Whale Radar — TUI + Web debug")]
struct Args {
    /// Run web server only (no terminal UI)
    #[arg(long)]
    web_only: bool,

    /// Disable web server (TUI only)
    #[arg(long)]
    no_web: bool,
}

/// Convert ShioajiEvent → TUI FeedEvent.
fn convert_shioaji_event(ev: shioaji_mock::ShioajiEvent) -> FeedEvent {
    match ev {
        shioaji_mock::ShioajiEvent::Tick(t) => {
            let side = match t.tick_type {
                shioaji_mock::types::TickType::Buy => TradeSide::Buy,
                shioaji_mock::types::TickType::Sell => TradeSide::Sell,
                shioaji_mock::types::TickType::Unknown => TradeSide::Buy,
            };
            FeedEvent::Tick(Tick {
                symbol: t.code,
                timestamp: t.datetime,
                price: t.close,
                shares: t.volume,
                side,
                amount: t.amount,
            })
        }
        shioaji_mock::ShioajiEvent::BidAsk(ba) => {
            let convert = |prices: &[f64; 5], volumes: &[u64; 5]| -> [OrderLevel; 5] {
                let mut levels = [OrderLevel { price: 0.0, lots: 0 }; 5];
                for i in 0..5 {
                    levels[i] = OrderLevel {
                        price: prices[i],
                        lots: volumes[i] / 1000,
                    };
                }
                levels
            };
            let ts = types::tick_size(ba.ask_price[0]);
            FeedEvent::Book(OrderBook {
                symbol: ba.code,
                timestamp: ba.datetime,
                asks: convert(&ba.ask_price, &ba.ask_volume),
                bids: convert(&ba.bid_price, &ba.bid_volume),
                last_price: ba.ask_price[0] - ts,
                last_side: TradeSide::Buy,
            })
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let app_state = Arc::new(RwLock::new(AppState::new()));

    // Feed events channel — receives from shioaji-mock
    let (feed_tx, feed_rx) = mpsc::channel::<FeedEvent>();

    // Commands from web → TUI
    let (cmd_tx, cmd_rx) = mpsc::channel::<Command>();

    // Spawn shioaji-mock simulator + conversion thread
    let tx_feed = feed_tx;
    std::thread::spawn(move || {
        let shioaji_rx = shioaji_mock::spawn();
        for ev in shioaji_rx {
            let feed_ev = convert_shioaji_event(ev);
            if tx_feed.send(feed_ev).is_err() {
                return;
            }
        }
    });

    // Start web server (if not disabled)
    let web_running = if !args.no_web {
        let web_state = Arc::new(web::WebState {
            app: Arc::clone(&app_state),
            cmd_tx,
        });
        std::thread::spawn(move || web::run_server(web_state));
        true
    } else {
        false
    };

    if args.web_only {
        // Headless mode: just process events, no TUI
        eprintln!("🐋 Whale Radar running in web-only mode");
        if web_running {
            eprintln!("   Press Ctrl+C to quit");
        }
        web_only_loop(&app_state, &feed_rx, &cmd_rx);
    } else {
        // TUI mode (optionally with web server too)
        tui_loop(&app_state, &feed_rx, &cmd_rx)?;
    }

    Ok(())
}

fn web_only_loop(
    app_state: &Arc<RwLock<AppState>>,
    feed_rx: &mpsc::Receiver<FeedEvent>,
    cmd_rx: &mpsc::Receiver<Command>,
) {
    loop {
        // Process feed events
        {
            let mut app = app_state.write().unwrap();
            if !app.paused {
                while let Ok(event) = feed_rx.try_recv() {
                    app.process_event(event);
                }
            }
        }

        // Process web commands
        while let Ok(cmd) = cmd_rx.try_recv() {
            let mut app = app_state.write().unwrap();
            if app.process_command(cmd) {
                return; // quit
            }
        }

        std::thread::sleep(Duration::from_millis(16));
    }
}

fn tui_loop(
    app_state: &Arc<RwLock<AppState>>,
    feed_rx: &mpsc::Receiver<FeedEvent>,
    cmd_rx: &mpsc::Receiver<Command>,
) -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        // Render TUI
        {
            let app = app_state.read().unwrap();
            terminal.draw(|f| ui::render(f, &app))?;
        }

        // Keyboard input (non-blocking)
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let mut app = app_state.write().unwrap();
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => break,
                        KeyCode::Char('p') | KeyCode::Char('P') => app.toggle_pause(),
                        KeyCode::Char('a') | KeyCode::Char('A') => app.toggle_auto_switch(),
                        KeyCode::Char('s') | KeyCode::Char('S') => app.toggle_sound(),
                        KeyCode::Char('v') | KeyCode::Char('V') => app.toggle_voice(),
                        KeyCode::Char(' ') => app.clear_alerts(),
                        KeyCode::Char('1') => app.switch_tab(0),
                        KeyCode::Char('2') => app.switch_tab(1),
                        KeyCode::Char('3') => app.switch_tab(2),
                        KeyCode::Char('4') => app.switch_tab(3),
                        KeyCode::Char('5') => app.switch_tab(4),
                        KeyCode::Right | KeyCode::Tab => app.next_tab(),
                        KeyCode::Left => app.prev_tab(),
                        _ => {}
                    }
                }
            }
        }

        // Process feed events
        {
            let mut app = app_state.write().unwrap();
            if !app.paused {
                while let Ok(event) = feed_rx.try_recv() {
                    app.process_event(event);
                }
            }
        }

        // Process web commands
        while let Ok(cmd) = cmd_rx.try_recv() {
            let mut app = app_state.write().unwrap();
            if app.process_command(cmd) {
                disable_raw_mode()?;
                execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                terminal.show_cursor()?;
                return Ok(());
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
