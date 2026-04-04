# Pipeline Architecture — Stock AI

## Overview

```
Human (user)
  │
  ▼
WebUI (ECharts + Bun)  ─────► crates/stock-server (Axum :3003)
  │                                │
  │                                ├─► crates/stock-core (types, indicators, SQLite, fetchers)
  │                                │       │
  │                                │       ├── kline_daily table (OHLCV cache)
  │                                │       ├── watchlist table (user stocks)
  │                                │       ├── strategies table (trading rules)
  │                                │       └── signal_log table (generated signals)
  │                                │
  │                                ├─► stock_api_cli (Python) ── fetch data, write to SQLite
  │                                │
  │                                └─► quant_analysis_cli (Python) ── HMM, backtest, signals
  │                                        (runs in parallel per stock)
```

---

## Data Flow — Entry Points

### 1. User adds stock to watchlist

```
WebUI                → POST /api/watchlist {symbol, name}
stock-server/main.rs → api_watchlist_add()
stock-core/lib.rs    → watchlist_add(conn, symbol, name, strategy_id)
SQLite               → INSERT INTO watchlist
```

### 2. User loads stock chart

```
WebUI                → GET /api/history/{symbol}?days=90
stock-server/main.rs → get_history() → cached_fetch()
stock-core/lib.rs    → 1. Check SQLite cache (kline_daily)
                       2. Cache miss → fetch_yahoo() or fetch_av()
                       3. db_upsert() to cache
                       4. Return bars
```

### 3. User runs HMM backtest (single stock)

```
WebUI                → GET /api/backtest/{symbol}
stock-server/main.rs → python_cmd("quant_analysis_cli", ["analyze", symbol, "--save"])
                       PYTHONPATH=stock_api_cli/python:quant_analysis_cli/python
quant_analysis_cli   → db.read_ohlcv(symbol) from SQLite
                     → build_features() → fit_hmm() → describe_states()
                     → calc_rsi() → calc_macd() → calc_bollinger()
                     → backtest_daytrade()
                     → save_analysis_result() to SQLite (--save flag)
                     → JSON output to stdout
stock-server         → Parse JSON, return to webui
```

### 4. User scans all watchlist stocks (parallel)

```
WebUI                → POST /api/scan
stock-server/main.rs → api_scan()
                       1. watchlist_get_all() → [symbol1, symbol2, ...]
                       2. For each symbol: tokio::spawn(python_cmd("quant_analysis_cli", ["signals", sym, "--save"]))
                       3. All runs execute IN PARALLEL via tokio tasks
quant_analysis_cli   → Per stock: load_latest_model() → predict states → compute signal → signal_insert()
SQLite               → INSERT INTO signal_log for each stock
stock-server         → Return scan results to webui
WebUI                → Update signal indicators on watchlist
```

### 5. User generates HTML report

```
WebUI                → GET /api/report/{symbol}
stock-server/main.rs → python_cmd("quant_analysis_cli", ["report", symbol])
quant_analysis_cli   → read_ohlcv() → full pipeline → generate_report()
                     → Writes output/report_{symbol}.html
stock-server         → Read HTML file, serve as text/html
```

---

## API Routes (Rust server — :3003)

### Market Data
| Method | Route | Handler | Description |
|--------|-------|---------|-------------|
| GET | `/` | `serve_webui` | Serve embedded webui.html |
| GET | `/api/history/{symbol}?days=N` | `get_history` | OHLCV bars (cached) |
| GET | `/api/quote/{symbol}` | `get_quote` | Latest price |
| GET | `/api/indicators/{symbol}` | `get_indicators` | RSI, MACD, BB |
| GET | `/api/kline/{symbol}?period=&from=&to=` | `get_kline` | Filtered kline |

### Quant Analysis
| Method | Route | Handler | Spawns Python |
|--------|-------|---------|---------------|
| GET | `/api/backtest/{symbol}` | `get_backtest` | `quant_analysis_cli analyze` |
| GET | `/api/report/{symbol}` | `get_report` | `quant_analysis_cli report` |

### Watchlist
| Method | Route | Handler | SQLite op |
|--------|-------|---------|-----------|
| GET | `/api/watchlist` | `api_watchlist_get` | `watchlist_get_all` |
| POST | `/api/watchlist` | `api_watchlist_add` | `watchlist_add` |
| DELETE | `/api/watchlist/{symbol}` | `api_watchlist_delete` | `watchlist_remove` |

### Strategies
| Method | Route | Handler | SQLite op |
|--------|-------|---------|-----------|
| GET | `/api/strategies` | `api_strategies_get` | `strategy_get_all` |
| POST | `/api/strategies` | `api_strategies_add` | `strategy_add` |
| DELETE | `/api/strategies/{id}` | `api_strategies_delete` | `strategy_delete` |

### Signals & Scan
| Method | Route | Handler | Description |
|--------|-------|---------|-------------|
| GET | `/api/signals/{symbol}` | `api_signals_get` | Latest signals from SQLite |
| POST | `/api/scan` | `api_scan` | Parallel scan all watchlist stocks |

---

## SQLite Schema (~/.stock_ai/data.db)

### kline_daily (written by stock-core fetchers, read by quant_analysis_cli)
```sql
CREATE TABLE kline_daily (
    time INTEGER NOT NULL,
    symbol TEXT NOT NULL,
    open REAL, high REAL, low REAL, close REAL, volume INTEGER,
    PRIMARY KEY (symbol, time)
);
```

### watchlist (written/read by stock-server)
```sql
CREATE TABLE watchlist (
    symbol TEXT PRIMARY KEY,
    name TEXT NOT NULL DEFAULT '',
    strategy_id INTEGER,
    added_at TEXT NOT NULL DEFAULT (datetime('now'))
);
```

### strategies (user-defined trading strategies)
```sql
CREATE TABLE strategies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    stype TEXT NOT NULL DEFAULT 'hmm_regime',
    params TEXT NOT NULL DEFAULT '{}',
    enabled INTEGER NOT NULL DEFAULT 1
);
```

### signal_log (written by quant_analysis_cli, read by stock-server)
```sql
CREATE TABLE signal_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    date TEXT NOT NULL,
    signal_type TEXT NOT NULL,      -- LONG, SHORT, EXIT, HOLD
    regime_state INTEGER,
    confidence REAL NOT NULL DEFAULT 0,
    details TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(symbol, date, signal_type)
);
```

---

## CLI Entry Points

### stock_api_cli (Data Fetcher)
```bash
PYTHONPATH=stock_api_cli/python python3 -m stock_api_cli fetch SYMBOL [--source yfinance|av] [--period 1y] [-o data.json]
PYTHONPATH=stock_api_cli/python python3 -m stock_api_cli quote SYMBOL [--source yfinance|av] [-o quote.json]
```
- **Entry**: `stock_api_cli/python/stock_api_cli/__main__.py`
- **Output**: JSON to stdout or file

### quant_analysis_cli (Analysis Engine)
```bash
PYTHONPATH=quant_analysis_cli/python:stock_api_cli/python python3 -m quant_analysis_cli analyze SYMBOL [--input data.json] [--save] [-o result.json]
PYTHONPATH=quant_analysis_cli/python:stock_api_cli/python python3 -m quant_analysis_cli train SYMBOL [--save model.pkl]
PYTHONPATH=quant_analysis_cli/python:stock_api_cli/python python3 -m quant_analysis_cli backtest SYMBOL [--model file.pkl | --model-id N]
PYTHONPATH=quant_analysis_cli/python:stock_api_cli/python python3 -m quant_analysis_cli report SYMBOL [-o report.html]
PYTHONPATH=quant_analysis_cli/python:stock_api_cli/python python3 -m quant_analysis_cli signals SYMBOL [--save]
```
- **Entry**: `quant_analysis_cli/python/quant_analysis_cli/__main__.py`
- **Data source**: SQLite `kline_daily` (primary) or `--input` JSON file (fallback)
- **Output**: JSON stdout + SQLite writes (with `--save` flag) + HTML files (report)

### Rust Server
```bash
cargo run   # starts on :3003
```
- **Entry**: `crates/stock-server/src/main.rs`
- **Config**: `AV_API_KEY` (env), `DATA_DIR` (env, default `~/.stock_ai`)

---

## Parallel Execution Strategy

The `POST /api/scan` endpoint runs analysis on ALL watchlist stocks in parallel:

```
api_scan()
  ├── tokio::spawn(quant_analysis_cli signals 2330.TW --save)
  ├── tokio::spawn(quant_analysis_cli signals 2317.TW --save)
  ├── tokio::spawn(quant_analysis_cli signals 2454.TW --save)
  └── tokio::spawn(quant_analysis_cli signals AAPL --save)
       ↓
  All results collected → returned to webui
```

Each spawn runs an independent Python process. SQLite handles concurrent writes via its built-in locking.

---

## Key Files Summary

| Component | Entry Point | Key Files |
|-----------|-------------|-----------|
| WebUI | `crates/stock-server/webui/src/app.tsx` | ECharts chart, watchlist sidebar, signal alerts |
| HTML template | `crates/stock-server/build.rs` | Embedded HTML + CSS |
| Rust server | `crates/stock-server/src/main.rs` | All API routes, Python CLI invocation |
| Core library | `crates/stock-core/src/lib.rs` | Types, indicators, SQLite CRUD, fetchers |
| stock_api_cli | `stock_api_cli/python/stock_api_cli/__main__.py` | fetch, quote commands |
| quant_analysis_cli | `quant_analysis_cli/python/quant_analysis_cli/__main__.py` | analyze, train, backtest, report, signals |
| Analysis modules | `quant_analysis_cli/python/quant_analysis_cli/analysis/` | features.py, hmm.py, indicators.py, backtest.py |
| SQLite layer | `quant_analysis_cli/python/quant_analysis_cli/db.py` | read_ohlcv, save_model, save_analysis, signals |
| Types | `quant_analysis_cli/python/quant_analysis_cli/types.py` | AnalysisResult, BacktestResult, Signal |
| HTML reports | `quant_analysis_cli/python/quant_analysis_cli/report/html_report.py` | Standalone ECharts HTML |

---

## Future Enhancements

1. **Phase C**: Add `--store` flag to `stock_api_cli fetch` → writes directly to SQLite `kline_daily`
2. **Phase D**: Add direct SQLite read routes in Rust server (avoid spawning Python for cached analysis)
3. **SSE/WebSocket**: Push signal updates to webui in real-time during scan
4. **Scheduled scan**: Cron-like background scanning every N minutes during market hours
5. **More strategies**: RSI reversal, MACD crossover, Bollinger bounce — beyond HMM regime
6. **Risk manager**: Position sizing, stop-loss rules, max daily loss limits
7. **Paper trading**: Simulate orders based on signals, track virtual P&L
