# Project Architecture — stock_ai

## Overview

Single-crate Rust binary that serves a stock chart dashboard. The web UI is built
with Bun + TradingView lightweight-charts and embedded into the binary at compile
time via `include_str!`.

## Crate Structure

```
stock_ai/
├── Cargo.toml          # single [[bin]] "stock_ai"
├── build.rs            # bun install → bun build → inline HTML
├── src/
│   └── main.rs         # all server logic (axum routes + data fetching + indicators)
├── webui/
│   ├── package.json    # lightweight-charts dep
│   ├── src/
│   │   └── app.tsx     # frontend SPA
│   └── bun.lock
└── scripts/            # dev/utility scripts (not part of build)
```

## Build Pipeline

1. `build.rs` runs **before** `main.rs` compiles
2. `bun install` in `webui/`
3. `bun build webui/src/app.tsx --format iife --target browser --minify` → `$OUT_DIR/bundle.js`
4. Inline `bundle.js` into an HTML template string → `$OUT_DIR/webui.html`
5. `main.rs` uses `include_str!(concat!(env!("OUT_DIR"), "/webui.html"))` to embed at compile time

## Data Sources (priority order)

| Ticker pattern | Primary source | Fallback |
|---|---|---|
| `*.TW`, `*.TWO` | TWSE OpenAPI (no key needed) | Yahoo Finance chart API |
| Everything else | Alpha Vantage (`AV_API_KEY`) | Yahoo Finance chart API |

## API Routes

| Method | Path | Handler | Returns |
|---|---|---|---|
| GET | `/` | `serve_webui` | Embedded HTML |
| GET | `/api/history/{symbol}?days=N` | `get_history` | `{ symbol, bars: [{time,open,high,low,close,volume}] }` |
| GET | `/api/quote/{symbol}` | `get_quote` | `{ symbol, price, change, change_pct, volume, high, low }` |
| GET | `/api/indicators/{symbol}` | `get_indicators` | `{ symbol, rsi_14, macd, macd_signal, macd_hist, bb_upper, bb_mid, bb_lower }` |

## Technical Indicators

All computed server-side in `main.rs`:
- **RSI(14)** — Wilder's smoothing
- **MACD(12,26,9)** — EMA-based
- **Bollinger Bands(20, 2σ)** — SMA ± 2 std dev

## Runtime

- Binds to `0.0.0.0:3003`
- Graceful shutdown on Ctrl+C
- Shared state: `Arc<AppState>` holding Alpha Vantage key + reqwest client
- CORS: permissive (all origins)
