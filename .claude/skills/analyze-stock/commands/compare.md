# /analyze-stock compare

Compare multiple stocks side-by-side.

## Usage

```
/analyze-stock compare 2330.TW 2317.TW 2454.TW
```

## Steps

1. Run `bun run yf-query.ts quote SYMBOL` for each symbol (in parallel if possible)
2. Present a comparison table:

```
| 指標 (Metric) | 2330 台積電 | 2317 鴻海 | 2454 聯發科 |
|---|---|---|---|
| 現價 (Price) | NT$1,810 | NT$193 | NT$1,465 |
| 漲跌幅 (Change) | -2.43% | -2.03% | 0.00% |
| 市值 (Market Cap) | NT$46.9T | NT$2.69T | NT$2.34T |
| 本益比 (P/E) | 27.3 | 14.4 | 22.2 |
| 每股盈餘 (EPS) | NT$66.25 | NT$13.39 | NT$66.07 |
| 成交量 (Volume) | 28.1M | 37.3M | 9.3M |
| 檔位摩擦 (Tick Friction) | 0.28% | 0.26% | 0.34% |
| 損益兩平檔數 (Breakeven Ticks) | ~1 檔 | ~1 檔 | ~1 檔 |
```

3. Add brief commentary on which is best for different strategy types.
