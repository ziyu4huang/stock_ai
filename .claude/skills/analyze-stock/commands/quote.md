# /analyze-stock quote

Quick price quote for a stock symbol.

## Usage

```
/analyze-stock quote 2330.TW
```

## Steps

```bash
bun run yf-query.ts quote SYMBOL
```

Show concise output:
- SYMBOL 名稱 (Name)
- 現價 (Price): NT$X, 漲跌 (Change): +/−NT$Y (Z%)
- 開 (Open) / 高 (High) / 低 (Low)
- 成交量 (Volume)
- 前收 (Prev Close)
- 本益比 (P/E), 每股盈餘 (EPS)
- 52 週區間 (52W Range)
