# Ch 4 — Frontend Enhancement

> Expand the Bun + lightweight-charts frontend within the single-binary model.

---

## 4.1 Multi-File Frontend Structure

**Current:** Single `webui/src/app.tsx` file.

**New structure:**
```
webui/src/
├── app.tsx            # Main layout, state management, composition
├── api.ts             # fetch helpers for all endpoints
├── chart.ts           # lightweight-charts setup & update logic
├── components.tsx     # SearchBar, StockTabs, StatsPanel, BacktestPanel
└── types.ts           # TS interfaces (Bar, Quote, Indicators, BacktestResult)
```

**build.rs update:** Add `rerun-if-changed` for all new files.

---

## 4.2 Period Selector

Add period toggle buttons next to existing range buttons (1M/3M/6M/1Y):

```
[Daily] [Weekly] [Monthly]   |   [1M] [3M] [6M] [1Y]
```

- Period buttons control `?period=daily|weekly|monthly` on `/api/kline`
- Range buttons control `days` parameter (how much history to show)

---

## 4.3 Date Range Picker

Add start/end date inputs to the toolbar:

```
[From: 2025-04-03] [To: 2026-04-03] [Go]
```

- Maps to `from` / `to` query params on API calls
- Default: from = 1 year ago, to = today

---

## 4.4 Volume Histogram

Add volume as a separate series below the candlestick chart:

- Use `lightweight-charts` `HistogramSeries`
- Green bars for up-days, red bars for down-days
- Separate price scale from candlestick

---

## 4.5 Indicator Overlays

Toggle buttons: `[RSI] [MACD] [BOLL]`

- When toggled on, fetch `/api/indicators/:symbol`
- **Bollinger Bands:** add 3 line series (upper/mid/lower) on same price scale
- **RSI:** add separate pane below chart (if supported) or overlay as oscillator
- **MACD:** separate pane or inline display

Note: lightweight-charts v4 supports multiple panes. Use `chart.addPane()` or manual DOM layout.

---

## 4.6 Backtest Panel Enhancement

**Current:** Button triggers backtest, shows JSON result.

**New:**
- "Run Backtest" button with loading spinner
- Show HMM regime states as **colored background bands** on chart:
  - Bull quiet → green shade
  - Bull volatile → light green shade
  - Bear quiet → red shade
  - Bear volatile → light red shade
- Display metrics panel: total return, Sharpe ratio, max drawdown, win rate
- Model params display: states, data source, training period

---

## 4.7 build.rs Watch List Update

Add to `build.rs`:
```rust
println!("cargo:rerun-if-changed=webui/src/api.ts");
println!("cargo:rerun-if-changed=webui/src/chart.ts");
println!("cargo:rerun-if-changed=webui/src/components.tsx");
println!("cargo:rerun-if-changed=webui/src/types.ts");
```

---

## 4.8 Responsive Improvements

- Mobile: stack toolbar vertically, stats panel below chart
- Chart fills available width, auto-resize on orientation change
- Collapsible stats sidebar on narrow screens

---

## Dark Theme (unchanged)

| Element | Color |
|---------|-------|
| Page bg | `#0d0e11` |
| Panel bg | `#13151a` |
| Accent | `#5b8def` |
| Text | `#e6edf3` |
| Muted text | `#8b949e` |

---

## Files to Create/Modify

1. `webui/src/api.ts` — new file
2. `webui/src/chart.ts` — new file
3. `webui/src/components.tsx` — new file
4. `webui/src/types.ts` — new file
5. `webui/src/app.tsx` — rewrite
6. `build.rs` — add rerun-if-changed lines
