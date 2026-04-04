# Plan: Stock Registry & Watchlist Management UI

## Context

Users need a way to manage which stocks they're interested in — add/remove symbols, see a persistent watchlist, and quickly load any stock without typing. Currently the only way to view a stock is to type the symbol manually. A registry/watchlist provides quick access and persistence.

## Features

### 1. SQLite Watchlist Table

New table in `~/.stock_ai/data.db`:

```sql
CREATE TABLE IF NOT EXISTS watchlist (
    symbol TEXT PRIMARY KEY,
    name TEXT,
    market TEXT DEFAULT 'TW',   -- TW, US
    added_at INTEGER,           -- unix timestamp
    notes TEXT DEFAULT ''
);
```

### 2. API Endpoints (in `crates/stock-server/src/main.rs`)

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/watchlist` | Return all watched symbols with metadata |
| `POST` | `/api/watchlist` | Add symbol `{ symbol, name, market }` |
| `DELETE` | `/api/watchlist/:symbol` | Remove symbol from watchlist |

### 3. DB Functions (in `crates/stock-core/src/lib.rs`)

- `watchlist_get_all(conn) -> Vec<WatchlistEntry>`
- `watchlist_add(conn, symbol, name, market)`
- `watchlist_remove(conn, symbol)`

### 4. Frontend UI (in `webui/src/app.tsx` + HTML template)

**Watchlist Sidebar Panel** (replaces or supplements current stats sidebar):
- Collapsible panel on the left side or a dropdown from toolbar
- Shows list of watched stocks with:
  - Symbol + name
  - Last price (fetched on load)
  - Mini sparkline or just +/- change indicator
  - × button to remove
- Click any stock to load its chart
- "+" button at bottom to add new symbol
- Pre-populated with Taiwan market favorites: 2330.TW, 2317.TW, 2454.TW, 2308.TW, 2891.TW

**Add Stock Dialog**:
- Input field for symbol (e.g. "AAPL" or "2330.TW")
- Auto-detect market (TW if ends in .TW/.TWO, else US)
- Optional name field
- Add button → POST `/api/watchlist`

### 5. Auto-load on Startup

Instead of hardcoded `loadStock("2330.TW")`:
1. Fetch `GET /api/watchlist` on boot
2. Load first stock in watchlist (if any)
3. Show watchlist in sidebar

## Implementation Steps

1. Add `watchlist` table to `init_db()` in `stock-core`
2. Add `WatchlistEntry` struct + DB functions to `stock-core`
3. Add 3 API handlers to `stock-server` (GET/POST/DELETE watchlist)
4. Add watchlist panel to frontend HTML template in `build.rs`
5. Add watchlist JS functions to `app.tsx`
6. Replace hardcoded auto-load with watchlist-based auto-load

## Files to Modify

| File | Change |
|------|--------|
| `crates/stock-core/src/lib.rs` | Add watchlist table + DB functions + WatchlistEntry struct |
| `crates/stock-server/src/main.rs` | Add 3 watchlist API handlers + routes |
| `crates/stock-server/build.rs` | Add watchlist panel HTML to template |
| `webui/src/app.tsx` | Add watchlist JS: fetch, render, add, remove, click-to-load |
