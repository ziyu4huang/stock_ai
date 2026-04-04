mod app;
mod data;
mod whale;
mod alert;
mod ui;
mod capture;

use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

use crate::app::App;
use crate::ui::render;

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn std::error::Error>> {
    crossterm::terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(
        stdout,
        crossterm::terminal::EnterAlternateScreen,
        crossterm::event::EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

/// Simulation mode config.
struct SimConfig {
    duration: Duration,
    tab_dwell: Duration,
    view_dwell: Duration,
}

/// Run simulation with ratatui TestBackend — no TTY needed.
async fn run_sim(sim_duration: Duration) -> Result<(), Box<dyn std::error::Error>> {
    let backend = ratatui::backend::TestBackend::new(220, 50);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    app.load_data().await;

    let screenshot_dir = std::path::PathBuf::from("/tmp/quant_sim");
    let _ = std::fs::create_dir_all(&screenshot_dir);

    let sim = SimConfig {
        duration: sim_duration,
        tab_dwell: Duration::from_millis(800),  // fast for screenshot capture
        view_dwell: Duration::from_millis(400),
    };

    let start_time = Instant::now();
    let mut last_tab_switch = Instant::now();
    let mut last_view_switch = Instant::now();
    let mut screenshot_count: u32 = 0;

    loop {
        if !app.running {
            break;
        }

        let elapsed = start_time.elapsed();
        if elapsed >= sim.duration {
            app.running = false;
            break;
        }

        // Auto-switch sub-views
        if last_view_switch.elapsed() >= sim.view_dwell {
            last_view_switch = Instant::now();
            use crate::app::SubView;
            app.sub_view = match app.sub_view {
                SubView::Chart => SubView::Whale,
                SubView::Whale => SubView::Signals,
                SubView::Signals => SubView::Chart,
            };
        }

        // Auto-switch stock tabs
        if last_tab_switch.elapsed() >= sim.tab_dwell {
            last_tab_switch = Instant::now();
            app.current_stock = (app.current_stock + 1) % app.stocks.len().max(1);
            app.mode = crate::app::Mode::Dashboard;
            app.sub_view = crate::app::SubView::Chart;
        }

        // Render
        terminal.draw(|f| render(f, &app))?;

        // Capture screenshot as ASCII text
        let buf = terminal.backend().buffer().clone();
        let label = format!(
            "{:03}_{}_{}",
            screenshot_count,
            match app.mode {
                crate::app::Mode::Overview => "overview",
                crate::app::Mode::Dashboard => "dashboard",
            },
            match app.sub_view {
                crate::app::SubView::Chart => "chart",
                crate::app::SubView::Whale => "whale",
                crate::app::SubView::Signals => "signals",
            },
        );
        let path = screenshot_dir.join(format!("{}.txt", label));
        crate::capture::save_text(&buf, 220, 50, &path)?;
        screenshot_count += 1;
    }

    // Print summary
    println!("模擬完成，共載入 {} 檔股票", app.stocks.len());
    println!("截圖已保存至 /tmp/quant_sim/ (共{}張)", screenshot_count);
    for sd in &app.stocks {
        let dir = sd.whale.as_ref().map(|w| w.whale_direction.label()).unwrap_or("N/A");
        let aiming = sd.whale.as_ref().map(|w| w.aiming.total_score).unwrap_or(0);
        println!("  {} {} | 分數:{} | 方向:{} | 瞄準:{}", sd.symbol, sd.name, sd.scan.score, dir, aiming);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Check for --sim argument (supports --sim, --sim=N)
    let sim_arg = std::env::args().find(|a| a == "--sim" || a.starts_with("--sim="));
    let sim_mode = sim_arg.is_some();
    let sim_duration_secs: u64 = sim_arg
        .and_then(|a| {
            if a.starts_with("--sim=") {
                a.trim_start_matches("--sim=").parse().ok()
            } else {
                None
            }
        })
        .unwrap_or(120); // default 120 seconds

    if sim_mode {
        // Use TestBackend — no TTY required
        run_sim(Duration::from_secs(sim_duration_secs)).await?;
        return Ok(());
    }

    // Normal interactive mode — needs real TTY
    let mut terminal = setup_terminal()?;
    let mut app = App::new();
    app.load_data().await;

    // Event loop
    loop {
        if !app.running {
            break;
        }

        let timeout = Duration::from_millis(100);

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                app.handle_key(key);
            }
        }

        if app.replay_playing {
            app.replay_advance();
        }

        terminal.draw(|f| render(f, &app))?;
    }

    restore_terminal(&mut terminal)?;
    Ok(())
}
