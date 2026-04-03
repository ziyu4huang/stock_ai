# Stock AI — Master Development Roadmap

> Single-binary Rust dashboard + Python HMM quant engine + Bun/ECharts frontend.
> Updated: 2026-04-03

---

## Architecture Overview

```
stock_ai/                              ← Cargo workspace root
├── Cargo.toml                         ← workspace config
├── crates/
│   ├── stock-core/                    ← shared types, indicators, DB
│   ├── stock-sources/                 ← data fetchers + cache layer
│   └── stock-server/                  ← Axum binary + build.rs + embedded webui
├── webui/                             ← Bun + ECharts frontend
├── stock_api_cli/                     ← multi-vendor API + HMM quant engine
│   ├── __main__.py                    ← CLI: python -m stock_api_cli <cmd>
│   ├── types.py                       ← shared types (OHLCVBar, Quote)
│   ├── yahoo_finance/                 ← vendor → language matrix
│   │   ├── python/client.py
│   │   ├── bun/client.ts
│   │   └── rust/src/lib.rs
│   ├── alpha_vantage/
│   │   ├── python/client.py
│   │   ├── bun/client.ts
│   │   └── rust/src/lib.rs
│   ├── hmm/                           ← HMM analysis engine
│   │   ├── model.py
│   │   ├── features.py
│   │   └── backtest.py
│   ├── indicators.py                  ← RSI, MACD, Bollinger
│   ├── report.py                      ← HTML report generator
│   └── bench.py                       ← cross-vendor benchmark runner
├── output/                            ← generated reports & analysis JSON
├── scripts/                           ← start.sh, stop.sh
└── docs/plan/                         ← planning documents
```

### Data Flow

```
Browser (:3003)
  │
  ▼
Axum Router (stock-server)
  ├── GET /                → embedded webui.html
  ├── GET /api/watchlist   → stock-core → SQLite watchlist (Phase 1.1)
  ├── POST /api/watchlist  → add symbol to watchlist (Phase 1.1)
  ├── GET /api/history     → stock-sources → SQLite cache → TWSE/Yahoo/AV
  ├── GET /api/quote       → stock-sources → live fetch
  ├── GET /api/indicators  → stock-core → RSI/MACD/BB from cached bars
  ├── GET /api/kline       → stock-sources → SQLite → date range filter
  ├── GET /api/backtest    → python3 -m stock_api_cli analyze → JSON
  └── GET /api/report      → python3 -m stock_api_cli report  → HTML
```

---

## Phase 1: Stock Registry UI + Foundation Polish

**Goal:** Make the app immediately useful with a persistent watchlist and polish existing features.

### 1.1 Stock Registry / Watchlist UI

Persistent watchlist for managing stocks you're interested in. Add/remove with one click, persists across restarts.

| Task | Status | Notes |
|------|--------|-------|
| SQLite `watchlist` table | ☐ | symbol PK, name, market, added_at |
| `GET /api/watchlist` | ☐ | list all watched symbols |
| `POST /api/watchlist` | ☐ | add symbol |
| `DELETE /api/watchlist/:symbol` | ☐ | remove symbol |
| Watchlist sidebar panel (frontend) | ☐ | collapsible, click-to-load |
| Add stock dialog | ☐ | symbol input, auto-detect market |
| Auto-load first watched stock on boot | ☐ | replace hardcoded 2330.TW |
| Pre-populate defaults (2330, 2317, 2454…) | ☐ | seed on first run |

See: `docs/plan/stock-registry-ui.md`

### 1.2 Existing Foundation (done)

| Task | Status |
|------|--------|
| Cargo workspace (3 crates) | ✅ |
| SQLite cache (kline_daily) | ✅ |
| 7 API endpoints | ✅ |
| ECharts candlestick + volume + dataZoom | ✅ |
| Stock tabs (multi-symbol) | ✅ |
| Sidebar stats (RSI, MACD, BB) | ✅ |
| HMM Backtest panel | ✅ |
| HTML Report generation | ✅ |
| Python HMM CLI (analyze/train/backtest/report) | ✅ |

### 1.3 Polish Tasks

| Task | Status | Notes |
|------|--------|-------|
| Custom AppError type | ☐ | IntoResponse, no panics |
| Cache staleness detection | ☐ | re-fetch if latest bar too old |
| Frontend indicator chart overlays | ☐ | RSI/MACD subplots on chart |
| Responsive mobile layout | ☐ | stack stats below chart |

---

## Phase 2: Restructure to `stock_api_cli`

**Goal:** Rename `quant_cli/` → `stock_api_cli/` with vendor × language matrix for API benchmarking.

### 2.1 Folder Restructure

```
quant_cli/                    ← DELETE, rename to stock_api_cli/
stock_api_cli/                ← NEW
├── __init__.py
├── __main__.py               ← CLI: python -m stock_api_cli <cmd>
├── types.py                  ← OHLCVBar, Quote, shared types
│
├── yahoo_finance/            ← vendor directory
│   ├── python/
│   │   ├── __init__.py
│   │   └── client.py         ← YFinanceClient
│   ├── bun/                  ← (future)
│   │   └── client.ts
│   └── rust/                 ← (future)
│       └── src/lib.rs
│
├── alpha_vantage/            ← vendor directory
│   ├── python/
│   │   ├── __init__.py
│   │   └── client.py         ← AlphaVantageClient
│   ├── bun/
│   └── rust/
│
├── hmm/                      ← HMM analysis engine (from quant_cli)
│   ├── __init__.py
│   ├── model.py              ← GaussianHMM + state labeling
│   ├── features.py           ← log_ret, range_pct, vol_change
│   └── backtest.py           ← day-trade backtest, Taiwan fees
│
├── indicators.py             ← RSI, MACD, Bollinger Bands
├── report.py                 ← generate_report() → standalone HTML
└── bench.py                  ← cross-vendor benchmark runner
```

### 2.2 CLI Commands

```bash
# Analysis (unchanged behavior, new name)
python3 -m stock_api_cli analyze 2330.TW [--source yfinance|av] [--period 1y]
python3 -m stock_api_cli train 2330.TW --save model.pkl
python3 -m stock_api_cli backtest 2330.TW --model model.pkl
python3 -m stock_api_cli report 2330.TW [-o report.html]

# NEW: benchmark all vendors for a symbol
python3 -m stock_api_cli bench 2330.TW
```

### 2.3 Migration Checklist

| Task | Status | Notes |
|------|--------|-------|
| Create `stock_api_cli/` structure | ☐ | dirs for vendors + hmm |
| Move `sources/yfinance_src.py` → `yahoo_finance/python/client.py` | ☐ | |
| Move `sources/alpha_vantage.py` → `alpha_vantage/python/client.py` | ☐ | |
| Move `hmm_model.py, features.py, backtest.py` → `hmm/` | ☐ | |
| Move `indicators.py, report.py` to root of `stock_api_cli/` | ☐ | |
| Update `__main__.py` imports | ☐ | new module paths |
| Update `analyze.py` wrapper | ☐ | delegates to stock_api_cli |
| Update Rust handlers (stock-server) | ☐ | `python3 -m stock_api_cli` |
| Add `bench.py` cross-vendor runner | ☐ | |
| Delete `quant_cli/` | ☐ | after all references updated |

### 2.4 Vendor × Language Matrix (future expansion)

| Vendor \ Language | Python | Bun/TS | Rust |
|-------------------|--------|--------|------|
| Yahoo Finance | ✅ current | ☐ Phase 3 | ☐ Phase 3 |
| Alpha Vantage | ✅ current | ☐ Phase 3 | ☐ Phase 3 |
| FMP | ☐ future | ☐ future | ☐ future |
| TWSE OpenAPI | ☐ (in Rust only) | ☐ | ✅ current |

---

## Phase 3: Multi-Language Clients + Enhanced Analysis

**Goal:** Add Bun and Rust clients for API vendors. Enhance chart analysis.

### 3.1 Bun API Clients
- `yahoo_finance/bun/client.ts` — direct fetch from browser
- `alpha_vantage/bun/client.ts` — with API key from env
- Compare latency: Bun direct vs Rust proxy vs Python subprocess

### 3.2 Rust API Clients
- `yahoo_finance/rust/` as a crate under `crates/` or `stock_api_cli/rust/`
- Native reqwest-based clients
- Benchmark against Python equivalents

### 3.3 Enhanced Chart Analysis
- RSI subplot below main chart
- MACD histogram subplot
- Bollinger Bands overlay on candlestick
- HMM regime background coloring
- Walk-forward validation for HMM
- BIC grid search for auto state-count

### 3.4 Backtest Enhancements
- Equity curve chart in report
- Stop-loss / take-profit exits
- Buy & Hold baseline comparison
- Trade log CSV export

---

## Phase 4: Real-Time Inference & Alerts

**Goal:** Live per-minute HMM state prediction with entry/exit signals.

### 4.1 Live Inference Loop
- `python -m stock_api_cli live SYMBOL` — streaming state prediction
- Bullish state detection with N-bar confirmation window
- Entry/exit signal generation (LONG/SHORT/FLAT)

### 4.2 Alert System
- Terminal notification on regime change
- Browser push notification
- Color flash on critical transition

### 4.3 Risk Management
- Max daily trades (default: 3)
- Max daily loss (default: 2% capital)
- Position sizing based on volatility regime

---

## Phase 5: Rust TUI (Ratatui)

**Goal:** Terminal-based dashboard for live monitoring.

### 5.1 IPC Bridge
- Python → Rust via Unix Domain Socket / JSON
- Real-time state + probability push every minute

### 5.2 TUI Dashboard
- Current regime with color indicator
- Sparkline price chart
- State probability bar chart
- Today's trade log
- Keyboard shortcuts: `[r]` refresh, `[q]` quit, `[s]` toggle strategy

---

## Phase 6: Production Hardening

### 6.1 Deployment
- Dockerize (optional)
- Environment config
- Health check endpoint

### 6.2 Monitoring
- Structured JSON logging
- Daily auto-report generation
- Model drift detection

### 6.3 Model Pipeline
- Weekly auto-retrain with latest data
- New vs old model comparison
- Auto-promote if BIC improves

---

## Progress Summary

| Phase | Description | Tasks | Done | Status |
|-------|-------------|-------|------|--------|
| **1** | **Stock Registry + Polish** | **15** | **9** | **60%** |
| 2 | Restructure to stock_api_cli | 10 | 0 | Planning |
| 3 | Multi-Language + Analysis | 12 | 0 | Not started |
| 4 | Real-Time & Alerts | 8 | 0 | Not started |
| 5 | Rust TUI | 6 | 0 | Not started |
| 6 | Production | 8 | 0 | Not started |
| **Total** | | **59** | **9** | **15%** |

---

## Key Design Decisions

| Decision | Choice | Why |
|----------|--------|-----|
| Database | SQLite (rusqlite bundled) | Zero external deps, single file |
| Python integration | subprocess call | No Rust HMM library; quant engine stays independent |
| Chart library | ECharts 5.x | Volume subplot, dataZoom, indicator overlays |
| Frontend bundling | Bun IIFE → build.rs → embedded | Single binary, no separate frontend |
| Workspace | Cargo crates/ | Clean separation, add/remove crates easily |
| API structure | `stock_api_cli/<vendor>/<lang>/` | Compare vendors × languages independently |
| Stock registry | Phase 1 priority | Immediately useful, drives daily usage |
