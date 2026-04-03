# Ch 1 — Rust Backend Enhancement

> Refactor the existing Axum single-binary into a clean module structure.

---

## Current State

Single `src/main.rs` (319 lines) containing everything: types, DB logic, data sources, indicators, and handlers. Works but needs structure for Phase 1 growth.

---

## 1.1 Module Structure

Refactor into separate files under `src/`:

```
src/
├── main.rs            # entry: init state, build router, bind :3003
├── types.rs           # Bar, AppState, HQ, KQ, AppError
├── handlers.rs        # serve_webui, get_history, get_quote, get_indicators,
│                      #   get_kline, get_backtest, get_report
├── data_sources.rs    # fetch_twse, fetch_av, fetch_yahoo, fetch_history
├── cache.rs           # init_db, db_query, db_upsert, cached_fetch, slice
└── indicators.rs      # calc_rsi, calc_ema, calc_macd, calc_bb
```

**Key change:** `main.rs` becomes ~50 lines — just state init + router.

---

## 1.2 AppState Expansion

```rust
struct AppState {
    av_key: String,
    client: reqwest::Client,
    db: Mutex<rusqlite::Connection>,
    data_dir: String,      // NEW: base dir for output/ etc.
    python_cmd: String,    // NEW: "python3" path
}
```

- `data_dir` defaults to `$HOME/.stock_ai`
- `python_cmd` defaults to `python3`, configurable via env `PYTHON_CMD`

---

## 1.3 SQLite Schema (already exists, verify)

```sql
CREATE TABLE IF NOT EXISTS kline_daily (
    time INTEGER NOT NULL,
    symbol TEXT NOT NULL,
    open REAL, high REAL, low REAL, close REAL, volume INTEGER,
    PRIMARY KEY (symbol, time)
);
CREATE INDEX IF NOT EXISTS idx_kd ON kline_daily(symbol, time ASC);
```

No changes needed — schema is solid.

---

## 1.4 Error Type

```rust
enum AppError {
    Fetch(String),     // external API failure
    Db(String),        // SQLite error
    Python(String),    // subprocess error
    NotFound(String),  // no data
}
```

Implement `IntoResponse` to return `{"error": "message"}` with appropriate HTTP status codes.

---

## Files to Create/Modify

1. `src/types.rs` — extract from main.rs
2. `src/handlers.rs` — extract from main.rs
3. `src/data_sources.rs` — extract from main.rs
4. `src/cache.rs` — extract from main.rs
5. `src/indicators.rs` — extract from main.rs
6. `src/main.rs` — rewrite as thin entry point
