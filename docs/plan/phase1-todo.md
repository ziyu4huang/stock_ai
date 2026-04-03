# Phase 1 — Stock Registry + API Restructure + Polish

> Target: Persistent watchlist, restructure quant_cli → stock_api_cli, polish frontend.
> Updated: 2026-04-03

---

## 1.1 Stock Registry / Watchlist

Persistent watchlist for managing stocks you're interested in.

- [ ] **1.1.1** SQLite `watchlist` table in stock-core
  - `symbol TEXT PRIMARY KEY, name TEXT, market TEXT, added_at INTEGER, notes TEXT`
- [ ] **1.1.2** DB functions in stock-core: `watchlist_get_all`, `watchlist_add`, `watchlist_remove`
- [ ] **1.1.3** `GET /api/watchlist` — list all watched symbols
- [ ] **1.1.4** `POST /api/watchlist` — add `{ symbol, name, market }`
- [ ] **1.1.5** `DELETE /api/watchlist/:symbol` — remove symbol
- [ ] **1.1.6** Watchlist sidebar panel (frontend)
  - Collapsible left panel, symbol + name + last price
  - Click to load chart, × to remove, "+" to add
- [ ] **1.1.7** Auto-load first watched stock on boot
- [ ] **1.1.8** Seed defaults on first run: 2330.TW, 2317.TW, 2454.TW, 2308.TW, 2891.TW

## 1.2 Restructure: quant_cli → stock_api_cli

Restructure into provider/function/client grouping.

- [x] **1.2.1** Create `stock_api_cli/` with subdirs: `providers/`, `analysis/`, `report/`, `cli/`
- [x] **1.2.2** Move `sources/` → `providers/` (base.py, yahoo.py, alpha_vantage.py)
- [x] **1.2.3** Move analysis modules → `analysis/` (hmm.py, features.py, backtest.py, indicators.py)
- [x] **1.2.4** Move `report.py` → `report/html_report.py`
- [x] **1.2.5** Add `types.py` (OHLCVBar, Quote, BacktestResult, AnalysisResult)
- [x] **1.2.6** Split `__main__.py` into `cli/` commands (analyze_cmd, train_cmd, backtest_cmd, report_cmd)
- [x] **1.2.7** Update Rust handlers: `python3 -m stock_api_cli`
- [x] **1.2.8** Delete `quant_cli/`

## 1.3 Foundation (done)

- [x] **1.3.1** Cargo workspace (3 crates)
- [x] **1.3.2** SQLite cache (kline_daily)
- [x] **1.3.3** 7 API endpoints
- [x] **1.3.4** ECharts candlestick + volume + dataZoom
- [x] **1.3.5** Stock tabs (multi-symbol)
- [x] **1.3.6** Sidebar stats (price, change, RSI, MACD)
- [x] **1.3.7** HMM Backtest panel + HTML Report
- [x] **1.3.8** Python HMM CLI (analyze/train/backtest/report)
- [x] **1.3.9** Start/stop scripts

## 1.4 Polish

- [ ] **1.4.1** Custom `AppError` type → `IntoResponse`
- [ ] **1.4.2** Cache staleness detection
- [ ] **1.4.3** Frontend indicator chart overlays (RSI/MACD subplots)
- [ ] **1.4.4** Responsive mobile layout

---

## Progress

| Section | Tasks | Done | Status |
|---------|-------|------|--------|
| 1.1 Watchlist | 8 | 0 | Next up |
| 1.2 Restructure | 8 | 8 | **Done** |
| 1.3 Foundation | 9 | 9 | **Done** |
| 1.4 Polish | 4 | 0 | Pending |
| **Total** | **29** | **17** | **59%** |

---

## Target Structure

```
stock_ai/
├── Cargo.toml
├── crates/
│   ├── stock-core/          # types, indicators, DB (watchlist + kline)
│   ├── stock-sources/       # TWSE/Yahoo/AV fetchers + cache
│   └── stock-server/        # Axum binary + build.rs + webui
├── webui/                   # Bun + ECharts
├── stock_api_cli/           # replaces quant_cli/
│   ├── __init__.py
│   ├── __main__.py          # slim CLI entry
│   ├── types.py             # OHLCVBar, Quote, BacktestResult
│   ├── providers/           # data sources (base, yahoo, alpha_vantage)
│   ├── analysis/            # hmm, features, backtest, indicators
│   ├── report/              # html_report.py
│   └── cli/                 # analyze_cmd, train_cmd, backtest_cmd, report_cmd
├── output/
├── scripts/
└── docs/plan/
```
