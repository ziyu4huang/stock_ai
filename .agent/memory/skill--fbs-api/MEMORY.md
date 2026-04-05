# Fubon Neo SDK (FBS) — Memory Index

Sliced from `docs/fbs/llms-full.txt` (61670 lines, v2.2.8) to support `/fbs_api` skill.
Source update: `curl -o docs/fbs/llms-full.txt https://www.fbs.com.tw/TradeAPI/llms-full.txt`

## Slices

### Overview & Setup
- [sdk-overview-setup](sdk-overview-setup.md) — SDK v2.2.8 download, install, version compat, API Key, cert management (L1-475)

### Market Data — Stock (台股)
- [stock-webapi-market-data](stock-webapi-market-data.md) — Historical candles/stats, intraday candles/quote/ticker/trades/volumes, snapshots (actives/movers/quotes), technical indicators (BBands/KDJ/MACD/RSI/SMA) (L476-3985)
- [stock-websocket-market-data](stock-websocket-market-data.md) — Realtime WebSocket: aggregates, books (5檔), candles, indices, trades, mode switching (L3986-6845)

### Market Data — Futures (期貨)
- [futures-market-data](futures-market-data.md) — Intraday candles/quote/ticker/trades/volumes, products list; WebSocket: aggregates, books, candles, trades (L6846-8561)

### Condition Orders (條件單)
- [stock-condition-orders](stock-condition-orders.md) — Stock condition orders: TPSL, trailing stop, time-volume split, day-trade conditions, queries (L8562-18210)
- [futures-condition-orders](futures-condition-orders.md) — Futures condition orders: TPSL, trailing stop, time-volume split, day-trade conditions (L18211-23288)

### Trading — Stock (證券交易)
- [stock-trading-guide](stock-trading-guide.md) — Tutorial: login, accounting (inventory/P&L/margin), non-blocking orders, callbacks (lightweight + standard), reconnect, error/status codes, full buy/sell flow (L23289-31067)
- [stock-trading-api-ref](stock-trading-api-ref.md) — API reference: all endpoints (C++/Python/Node.js/C#/Go). Login, place/cancel/modify orders, batch orders, accounting queries, stock quotes, price limits (L31068-48823)

### Trading — Futures (期貨交易)
- [futures-trading](futures-trading.md) — Tutorial + API reference: login, place/cancel/modify orders, batch orders, accounting (position/equity/margin), callbacks, error/status codes (L48824-60085)

### Trading — Options (選擇權)
- [options-trading](options-trading.md) — Options trading: login, place/cancel/modify orders, batch orders, position queries, margin, convert symbol (L60086-61553)

### Quick Reference
- [quickstart](quickstart.md) — Getting started guide, test environment, LLM-assisted development (L61554-61670)

## Node.js Quick Reference (project uses this)

```js
import { FubonSDK } from 'fubon-neo';
const sdk = new FubonSDK();
const accounts = sdk.login({ id, password, certPath, certPass });
// or API Key login (>=v2.2.7):
// const accounts = sdk.login({ apiKey, apiSecret, certPath, certPass });
```

## Language Sections in API Refs

The API reference files contain repeated sections for each language. Within `stock-trading-api-ref.md`:
- C++: first ~1000 lines
- Python: next ~3500 lines
- Node.js: next ~2500 lines
- Go: next ~4000 lines
- C#: next ~3500 lines

Use `grep` with language-specific patterns (e.g., `bankRemain` for Node.js, `BankRemain` for C#/Go, `bank_remain` for Python) to find the right section.
