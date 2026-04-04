//! Headless simulation mode: auto-pilots through all views, captures text screenshots.
//!
//! Usage: `cargo run -p quant_trade_cli -- --sim 30`
//! Output: `/tmp/quant_sim/01_overview.txt`, `02_台積電_chart.txt`, ...

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use ratatui::backend::TestBackend;
use ratatui::Terminal;

use crate::app::{App, Mode, SubView};
use crate::capture;
use crate::ui;

const TERM_W: u16 = 140;
const TERM_H: u16 = 48;
const VIEW_DURATION_SECS: u64 = 3;
const OUTPUT_DIR: &str = "/tmp/quant_sim";

/// A scheduled view transition.
struct Transition {
    label: String,       // filename-safe label
    setup: fn(&mut App), // configure app state for this view
}

pub async fn run(total_secs: u64) -> Result<(), Box<dyn std::error::Error>> {
    // Build app and load data
    let mut app = App::new();
    app.load_data().await;

    if app.stocks.is_empty() {
        eprintln!("No stock data loaded — cannot run sim");
        return Ok(());
    }

    let stock_count = app.stocks.len();

    // Build transition schedule
    let mut transitions: Vec<Transition> = Vec::new();

    // 1. Overview
    transitions.push(Transition {
        label: "01_overview".into(),
        setup: |app| {
            app.mode = Mode::Overview;
        },
    });

    // 2-5. For each stock: chart → whale → signals → replay
    for (i, sd) in app.stocks.iter().enumerate() {
        let name = sd.name.clone();
        let idx = i;

        let n = format!("{:02}", (i * 4) + 2);
        let base = format!("{}_{}", n, name);

        transitions.push(Transition {
            label: format!("{}_chart", base),
            setup: move |app| {
                app.mode = Mode::Dashboard;
                app.current_stock = idx;
                app.sub_view = SubView::Chart;
                app.reset_replay();
            },
        });

        let label2 = format!("{}_whale", base);
        transitions.push(Transition {
            label: label2,
            setup: move |app| {
                app.mode = Mode::Dashboard;
                app.current_stock = idx;
                app.sub_view = SubView::Whale;
            },
        });

        let label3 = format!("{}_signals", base);
        transitions.push(Transition {
            label: label3,
            setup: move |app| {
                app.mode = Mode::Dashboard;
                app.current_stock = idx;
                app.sub_view = SubView::Signals;
            },
        });

        let label4 = format!("{}_replay", base);
        transitions.push(Transition {
            label: label4,
            setup: move |app| {
                app.mode = Mode::Dashboard;
                app.current_stock = idx;
                app.sub_view = SubView::Chart;
                app.reset_replay();
                app.replay_playing = true;
                // Advance a few ticks so replay chart has content
                for _ in 0..200 {
                    app.replay_advance();
                }
                app.replay_playing = false;
            },
        });
    }

    // Calculate how many transitions fit in the time budget
    let per_view = Duration::from_secs(VIEW_DURATION_SECS);
    let budget = Duration::from_secs(total_secs);
    let max_views = (budget.as_secs() / per_view.as_secs()) as usize;
    let transitions: Vec<_> = transitions.into_iter().take(max_views).collect();

    if transitions.is_empty() {
        eprintln!("No transitions scheduled (duration too short)");
        return Ok(());
    }

    // Create output dir
    fs::create_dir_all(OUTPUT_DIR)?;

    // Set up TestBackend terminal
    let backend = TestBackend::new(TERM_W, TERM_H);
    let mut terminal = Terminal::new(backend)?;

    let start = Instant::now();
    let mut saved: Vec<String> = Vec::new();

    for (i, t) in transitions.iter().enumerate() {
        // Wait until scheduled time
        let target = start + per_view * (i as u32 + 1);
        let now = Instant::now();
        if target > now {
            tokio::time::sleep(target - now).await;
        }

        // Set up app state
        (t.setup)(&mut app);

        // Render
        terminal.draw(|f| ui::render(f, &app))?;

        // Capture buffer → text
        let buf = terminal.current_buffer_mut().clone();
        let path = PathBuf::from(OUTPUT_DIR).join(format!("{}.txt", t.label));
        capture::save_text(&buf, TERM_W, TERM_H, &path)?;
        saved.push(format!("{}", path.display()));

        eprintln!("[sim] ({}/{}) saved {}", i + 1, transitions.len(), path.display());
    }

    // Write index
    let index_path = PathBuf::from(OUTPUT_DIR).join("index.txt");
    let mut idx_file = fs::File::create(&index_path)?;
    writeln!(idx_file, "quant_trade_cli sim — {} views in {}s", saved.len(), total_secs)?;
    for (i, p) in saved.iter().enumerate() {
        writeln!(idx_file, "  {:02}. {}", i + 1, p)?;
    }

    eprintln!("[sim] Done — {} screenshots saved to {}/", saved.len(), OUTPUT_DIR);
    Ok(())
}
