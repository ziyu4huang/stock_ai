# Quantitative Backtesting System — Phase 1 (Architecture Reference)

> This is the original full-plan reference.
> For actionable chapter-by-chapter plans, see `phase1-todo.md` and `phase1-ch01..05` files.
> Plans have been rewritten to fit the **actual** Rust/Axum + Bun + SQLite + Python stack.

---

## System Architecture

```
┌─────────────────────────────────────────────────────────┐
│  cargo run  →  single Rust binary (:3003)               │
│                                                         │
│  Axum routes:                                           │
│    GET /              → embedded webui.html             │
│    GET /api/history   → OHLCV bars (SQLite cached)      │
│    GET /api/quote     → latest quote                     │
│    GET /api/indicators→ RSI(14), MACD, Bollinger Bands   │
│    GET /api/kline     → kline data (period + date filter)│
│    GET /api/backtest  → Python subprocess → JSON         │
│    GET /api/report    → Python subprocess → HTML         │
│                                                         │
│  SQLite: ~/.stock_ai/data.db                            │
│  Data sources: TWSE / Alpha Vantage / Yahoo Finance     │
└─────────────────────────────────────────────────────────┘

  webui/  (Bun + TradingView lightweight-charts)
    build.rs → bun build → IIFE → embedded in binary

  quant_cli/  (Python 3.9+)
    HMM regime detection + backtesting engine
```

## Tech Stack

| Layer | Technology | Role |
|-------|-----------|------|
| Backend | Rust + Axum + reqwest + rusqlite | Web server, API, caching, indicators |
| Frontend | Bun + lightweight-charts + TypeScript | Chart dashboard |
| Quant | Python + hmmlearn + pandas + numpy | HMM analysis, backtesting |
| Build | cargo + build.rs + bun | Single binary output |
| Storage | SQLite | Local OHLCV cache |

## Phase 1 Goals

1. Refactor Rust backend into clean module structure
2. Add weekly/monthly kline aggregation
3. Improve Python HMM integration with parameterized endpoints
4. Enhance frontend with volume overlay, indicator toggles, backtest visualization
5. Add dev convenience scripts

## Future Phases (unchanged)

| Phase | Theme |
|-------|-------|
| Phase 2 | Technical indicator service, multi-timeframe analysis |
| Phase 3 | Strategy engine, signal generation |
| Phase 4 | Full backtesting with performance reporting |
| Phase 5 | Real-time data streaming, paper trading |
| Phase 6 | Production deployment, monitoring |
