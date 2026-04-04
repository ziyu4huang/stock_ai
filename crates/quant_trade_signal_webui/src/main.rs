mod alert;
mod app;
mod detector;
mod hmm;
mod mock_feed;
mod state_machine;
mod stock_view;
mod types;
mod web;

use app::{AppState, Command};
use clap::Parser;
use std::sync::{Arc, RwLock, mpsc};
use std::time::Duration;
use types::{FeedEvent, OrderBook, OrderLevel, Tick, TradeSide};

#[derive(Parser, Debug)]
#[command(name = "quant_trade_signal_webui", about = "🐋 Whale Radar — Web UI")]
struct Args {
    /// Port for the web server
    #[arg(long, default_value = "3005")]
    port: u16,
}

/// Convert ShioajiEvent → FeedEvent.
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

    // Commands from web → app
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

    // Start web server
    {
        let web_state = Arc::new(web::WebState {
            app: Arc::clone(&app_state),
            cmd_tx,
        });
        let port = args.port;
        std::thread::spawn(move || web::run_server(web_state, port));
    }

    eprintln!("🐋 Whale Radar running — http://localhost:{}", args.port);
    eprintln!("   Press Ctrl+C to quit");

    // Headless event loop
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
                return Ok(());
            }
        }

        std::thread::sleep(Duration::from_millis(16));
    }
}
