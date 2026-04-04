# Ch 2 — Market Data & Caching

> Strengthen data fetching, add period aggregation, improve cache logic.

---

## 2.1 Weekly/Monthly Aggregation

Currently only `kline_daily` exists. Add aggregation:

**New tables:**
```sql
CREATE TABLE IF NOT EXISTS kline_weekly (
    time INTEGER NOT NULL, symbol TEXT NOT NULL,
    open REAL, high REAL, low REAL, close REAL, volume INTEGER,
    PRIMARY KEY (symbol, time)
);
CREATE TABLE IF NOT EXISTS kline_monthly (LIKE kline_daily ...);
```

**Aggregation logic (in `cache.rs`):**
- Weekly: group daily bars by ISO week, take first open, max high, min low, last close, sum volume
- Monthly: group by year-month, same aggregation

**API impact:**
- `GET /api/kline/:symbol?period=weekly` → query `kline_weekly`
- `GET /api/kline/:symbol?period=monthly` → query `kline_monthly`
- `period=daily` (default) → unchanged

---

## 2.2 Date Range Filtering

Already partially implemented in `get_kline` handler via `from`/`to` query params.

**Extend to `/api/history/:symbol`:**
- Add `from` and `to` optional query params
- Filter the cached bars before slicing by `days`

---

## 2.3 Smart Cache Invalidation

**Current behavior:** Cache hit if `cached.len() > 5`, always returns stale.

**New behavior:**
1. Check if latest cached bar date < last trading day
2. If stale: fetch only the delta (from last cached date → now)
3. Upsert delta into SQLite
4. Return full cached range

This avoids re-fetching years of data when only 1 day is missing.

---

## 2.4 Market Info Endpoints (Nice-to-have)

```
GET /api/markets
  Returns: [{ "id": "tw", "name": "Taiwan" }, { "id": "us", "name": "US/Global" }]

GET /api/search?q=TSLA
  Returns: [{ "symbol": "TSLA", "name": "Tesla Inc", "market": "us" }]
```

Low priority — can use hardcoded list for now.

---

## Data Source Priority (unchanged)

| Ticker | Primary | Fallback |
|--------|---------|----------|
| `*.TW`, `*.TWO` | TWSE OpenAPI | Yahoo Finance |
| Others | Alpha Vantage | Yahoo Finance |

---

## Files to Modify

1. `src/cache.rs` — add aggregation functions, smart invalidation
2. `src/handlers.rs` — extend get_kline with period routing, add from/to to get_history
