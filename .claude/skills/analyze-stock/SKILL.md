---
name: analyze-stock
description: >
  Use when: "analyze stock", "stock price", "check quote", "2330 price", "stock analysis",
  "compare stocks", "tick friction", "day trading cost", "position sizing".
  Triggers on: analyze stock, stock quote, stock price, compare stocks, 當沖成本, 檔位摩擦.
metadata:
  version: 1.0.0
---

# /analyze-stock — Stock Analysis (股票分析)

Analyze stocks using real-time Yahoo Finance data via Bun. Uses `yf-query.ts` (~0.9s, 37x faster than MCP).

## Default Invocation

When `/analyze-stock` is invoked, analyze the given symbol(s) with full report.

```
/analyze-stock 2330.TW                          # Single stock full analysis
/analyze-stock 2330.TW 2317.TW 2454.TW          # Compare multiple stocks
/analyze-stock AAPL                              # US stock
/analyze-stock 2330.TW --history 30             # With 30-day history
```

## Commands

- `/analyze-stock quote SYMBOL` — Quick price quote
- `/analyze-stock compare SYM1 SYM2 SYM3` — Side-by-side comparison
- `/analyze-stock cost SYMBOL` — Taiwan day trading cost analysis
- `/analyze-stock position SYMBOL CAPITAL` — Position sizing for given capital

## Data Source

Always use Bun (fastest):

```bash
# Quote (price, change, volume, PE, etc.)
bun run yf-query.ts quote SYMBOL

# History (OHLCV)
bun run yf-query.ts history SYMBOL DAYS

# Detailed info (financial statements)
bun run yf-query.ts info SYMBOL
```

Fallback if Bun fails:

```bash
python3 -c "import yfinance as yf, json, warnings; warnings.filterwarnings('ignore'); t=yf.Ticker('SYMBOL'); print(json.dumps({'price': t.fast_info.last_price, 'market_cap': t.fast_info.market_cap, 'prev_close': t.fast_info.previous_close, '52w_high': t.fast_info.year_high, '52w_low': t.fast_info.year_low}, default=str))"
```

## Output Format

Write in **繁體中文 (Traditional Chinese)** with **English terms** in parentheses.

### 報告結構 (Report Structure)

#### 1. 價格摘要 (Price Summary)
- Current price, change, change %
- Day range, previous close
- 52-week range

#### 2. 估值指標 (Valuation Metrics)
- Market Cap (市值)
- P/E Ratio (本益比)
- EPS (每股盈餘)

#### 3. 台股特有分析 (Taiwan-Specific Analysis) — only for .TW/.TWO symbols

**成本分析 (Day Trading Cost)**:
```
Round-trip cost at negotiated rate (0.03%):
  Buy commission:  PRICE × 1000shares × 0.03%
  Sell commission: PRICE × 1000shares × 0.03%
  Day trade tax:   PRICE × 1000shares × 0.15%
  Total:           0.21% of trade value
```

**檔位摩擦 (Tick Friction)** — use Taiwan tick size rules:
| Price Range | Tick Size |
|---|---|
| < NT$10 | NT$0.01 |
| NT$10-50 | NT$0.05 |
| NT$50-100 | NT$0.10 |
| NT$100-500 | NT$0.50 |
| NT$500-1000 | NT$1.00 |
| > NT$1000 | NT$5.00 |

```
Tick friction = tick_size / current_price × 100%
Breakeven ticks = round_up(0.21% / tick_friction%)
```

**部位計算 (Position Sizing)** — for NT$500K and NT$1M capital:
```
Risk per trade: 1% of capital
Stop-loss distance: N ticks
Max shares = risk_amount / stop_loss_distance
Required capital = max_shares × current_price
```

**當沖適合度 (Day Trading Suitability)**:
- Liquidity: high (daily volume > NT$1B) / medium / low
- Tick friction: low (<0.1%) / medium (0.1-0.3%) / high (>0.3%)
- Strategy fit: scalping / momentum / swing / avoid

#### 4. 比較 (Comparison) — for multiple symbols
Side-by-side table with key metrics.

## Notes
- Round monetary values to whole NT$ (no decimals for prices > NT$100)
- Show percentages to 2 decimal places
- For Taiwan stocks, show Chinese name: `2330 台積電`, `2317 鴻海`, `2454 聯發科`
