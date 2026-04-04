# Phase 1 — Finish Document

> What was actually built. Updated: 2026-04-04

---

## Workspace Overview

```
stock_ai/
├── Cargo.toml                          ← workspace root
├── crates/
│   ├── stock-core/                     ← types, indicators, SQLite, fetchers
│   ├── stock-server/                   ← Axum :3003 + embedded Bun/ECharts webui
│   ├── quant_trade_cli/                ← (reserved)
│   ├── quant_trade_signal_tui/         ← 🐋 Whale Radar TUI + Web debug (ratatui + Axum :3004)
│   ├── quant_trade_signal_webui/       ← 🐋 Whale Radar Web-only UI (Axum :3005, no ratatui)
│   └── shioaji-mock/                   ← Simulated Taiwan stock tick + bid/ask feed
├── stock_api_cli/                      ← Multi-vendor fetcher (Python + Bun + Rust)
│   ├── python/stock_api_cli/           ← yfinance, alpha_vantage providers
│   ├── bun/src/                        ← Bun/TS client
│   └── rust/                           ← (reserved)
├── scripts/                            ← start.sh, stop.sh
├── output/                             ← generated reports
├── docs/                               ← documentation
│   ├── phase1-finish.md               ← THIS FILE
│   ├── archive/                       ← archived plans & design docs
│   ├── api_doc/                       ← API reference (SinoTrade)
│   └── presentation/                  ← Slidev decks
└── requirements.txt                    ← Python dependencies
```

---

## Crates

### stock-core

Shared library: types, technical indicators, SQLite CRUD, data fetchers.

- **Types:** OHLCVBar, Quote, IndicatorResult, WatchlistEntry, Strategy, Signal
- **Indicators:** RSI(14), MACD(12,26,9), Bollinger Bands(20,2)
- **SQLite tables:** kline_daily, watchlist, strategies, signal_log, hmm_models, analysis_results
- **Fetchers:** TWSE OpenAPI, Yahoo Finance (via stock_api_cli subprocess), Alpha Vantage
- **Smart cache:** daily cache 4h TTL, auto-revalidate, intraday intervals for short ranges

### stock-server

Axum binary with embedded Bun/ECharts frontend. Build.rs handles `bun install` + bundle automatically.

- **Port:** 3003
- **Frontend:** Bun + ECharts candlestick chart + volume overlay + indicator subplots
- **API routes:** history, quote, indicators, kline, backtest, report, watchlist CRUD, strategies CRUD, signals, scan

### quant_trade_signal_tui

Terminal + Web dual-mode whale radar for Taiwan stocks.

- **TUI mode:** ratatui + crossterm, keyboard-driven
- **Web mode:** Axum :3004, WebSocket real-time push (100ms)
- **Features:** 5-stock tabs, 3-column layout, order book, signal radar, tick feed, alerts

### quant_trade_signal_webui

Web-only whale radar (pure Axum, no ratatui).

- **Port:** 3005 (configurable via `--port`)
- **No terminal UI** — runs headless, browser-only
- **Enhanced web UX:** clickable toolbar buttons, hover effects, pulse animations, responsive layout, connection status indicator
- **Same core logic** as TUI version: whale detection, ignition detection, HMM regime, composite scoring, sound/voice alerts

### shioaji-mock

Simulated Taiwan stock market feed.

- Generates Tick + BidAsk events for 8 stocks (2330, 2317, 2454, 2881, 2882, 2303, 2308, 2412)
- Simulates market modes: Normal, WhaleAccum, Ignition, Distribution
- Tick-level price simulation with proper Taiwan tick sizes

---

## Python Packages

### stock_api_cli

Multi-vendor market data CLI.

```bash
PYTHONPATH=stock_api_cli/python python3 -m stock_api_cli fetch SYMBOL [--source yfinance|av] [--period 1y]
PYTHONPATH=stock_api_cli/python python3 -m stock_api_cli quote SYMBOL [--source yfinance|av]
```

- **Providers:** yfinance, alpha_vantage
- **Languages:** Python (primary), Bun/TS (secondary), Rust (reserved)

---

## Whale Radar — Architecture

The core trading signal system shared by both TUI and WebUI crates:

```
shioaji-mock → FeedEvent (Tick | Book)
                    │
                    ▼
              AppState (Arc<RwLock>)
              ├── StockView × 5 (one per tab)
              │   ├── WhaleDetector     — threshold-based large trade detection
              │   ├── IgnitionDetector   — ≥3 whales in 30s + directional price move
              │   ├── StockStateTracker  — 6-state rule-based classifier
              │   ├── HmmModel           — 4-state Gaussian HMM (Viterbi)
              │   └── CompositeScore     — whale + ignition + pressure + HMM
              ├── SoundQueue             — afplay + macOS say zh_TW
              └── snapshot() → JSON → WebSocket → Browser
```

### Detection Pipeline

1. **Whale Detection:** trade amount ≥ threshold (5M/3M NTD by stock)
2. **Ignition Detection:** ≥3 whale trades in 30s window + price moves ≥2 ticks directionally
3. **State Machine (6 states):** LongAccum → LongIgnition, ShortDistrib → ShortIgnition, Neutral, Noise
4. **HMM (4 states):** Accumulation, Ignition, Distribution, Noise — 4-feature observation [log_return, whale_imbalance, volume_zscore, price_position]
5. **Composite Score:** -100 to +100 = whale_net×5 + ignition×20 + order_book_pressure×40 + HMM_confirm×15

### Signal Output

| Action | Chinese | Condition |
|--------|---------|-----------|
| FollowBull | 跟多進場 ▲ | L-IGNITE + score ≥ 50 |
| WatchBull | 觀察做多 ▲ | L-ACCUM + score ≥ 20 |
| FollowBear | 跟空進場 ▼ | S-IGNITE + score ≤ -50 |
| WatchBear | 觀察做空 ▼ | S-DISTRIB + score ≤ -20 |
| StandAside | 觀望 — | no clear signal |

### Alert System

- **Sound:** macOS `afplay` with system sounds (Ping=Bull, Basso=Bear, Sosumi=Mega, Hero=Ignition)
- **Voice:** macOS `say -v Meijia -r 200` with Chinese TTS alerts
- **Rate limiting:** per-group cooldown (whale 3s, ignition 8s, voice 4s)
- **Priority queue:** Ignition > MegaWhale > Voice > regular Whale

---

## How to Run

```bash
# Stock dashboard (ECharts webui)
cargo run -p stock-server          # http://localhost:3003

# Whale Radar — TUI mode
cargo run -p quant_trade_signal_tui           # TUI + web :3004
cargo run -p quant_trade_signal_tui -- --web-only  # web-only :3004

# Whale Radar — Web UI (no terminal)
cargo run -p quant_trade_signal_webui          # http://localhost:3005
cargo run -p quant_trade_signal_webui -- --port 3006  # custom port

# Python CLI
PYTHONPATH=stock_api_cli/python python3 -m stock_api_cli fetch 2330.TW --source yfinance
```

---

## Key Design Decisions

| Decision | Choice | Why |
|----------|--------|-----|
| Database | SQLite (rusqlite bundled) | Zero external deps, single file at ~/.stock_ai/data.db |
| Frontend | Bun IIFE → build.rs → embedded | Single binary, no separate frontend server |
| HMM | Hand-tuned Gaussian in Rust | No Python dependency for real-time; 4 states from domain knowledge |
| Python integration | subprocess for batch, Rust-native for real-time | Quant engine stays independent; real-time path avoids IPC latency |
| Mock data | shioaji-mock crate | Simulates Taiwan market microstructure without live broker connection |
| Dual crates (TUI + WebUI) | Shared core, different UI layers | TUI for terminal workflow; WebUI for browser-native mouse interaction |

---

## Tech Stack

| Layer | Technology |
|-------|-----------|
| Backend | Rust + Axum + reqwest + serde + chrono + rusqlite |
| Frontend (dashboard) | Bun + ECharts (embedded in binary) |
| Frontend (whale radar) | Vanilla HTML/CSS/JS (embedded in binary) |
| Quant (batch) | Python + hmmlearn + pandas + numpy |
| Quant (real-time) | Rust-native Gaussian HMM + Viterbi |
| Build | cargo + build.rs + bun |
| Sound | macOS afplay + say (zh_TW) |
