# TradingView Lightweight Charts — stock_ai

## Library

- Package: `lightweight-charts` v4.1+
- Docs: https://tradingview.github.io/lightweight-charts/
- Used in `webui/src/app.tsx`

## Pattern

The frontend is a single TSX file compiled by Bun into an IIFE bundle. It:
1. Creates a `chart` instance attached to `#chart` div
2. Adds a candlestick series
3. Fetches `/api/history/{symbol}?days=N` and sets the data
4. Fetches `/api/indicators/{symbol}` for RSI/MACD/Bollinger values
5. Displays stats in the `#stats` sidebar

## UI Layout

- Fixed toolbar at top (48px) — brand, search input, Go button, period buttons (1M/3M/6M/1Y)
- Main area: chart (flex:1) + stats sidebar (280px, right side)
- Dark theme: background `#0d0e11`, panels `#13151a`, accent `#5b8def`
- Period buttons toggle `active` class, default = 30 days (1M)

## Data Flow

```
User enters ticker → loadStock() → fetch /api/history/{symbol}?days=N
                                   fetch /api/quote/{symbol}
                                   fetch /api/indicators/{symbol}
                → chart.setData(bars)
                → update sidebar stats
```
