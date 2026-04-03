# /analyze-stock position

Position sizing calculation for day trading a stock.

## Usage

```
/analyze-stock position 2330.TW 500000
/analyze-stock position 2317.TW 1000000
```

Arguments: SYMBOL CAPITAL

## Steps

1. Run `bun run yf-query.ts quote SYMBOL` to get current price
2. Calculate position sizes for different risk levels:

```
=== 部位計算 (Position Sizing) ===
標的: 2330 台積電 @ NT$1,810
資金 (Capital): NT$500,000

假設停損距離 (Stop-loss Distance):
  保守 (Conservative, 0.5%): NT$9/股 (2 檔)
  中等 (Moderate, 1.0%): NT$18/股 (4 檔)
  積極 (Aggressive, 1.5%): NT$27/股 (6 檔)

1% 風險規則 (1% Risk Rule):
  每筆最大風險: NT$5,000

  保守停損: 5,000 / 9 = 555 股 → 需 NT$1,004,550 → ❌ 超過資金 (需零股 Odd Lot)
  中等停損: 5,000 / 18 = 277 股 → 需 NT$501,370 → ❌ 略超
  積極停損: 5,000 / 27 = 185 股 → 需 NT$334,850 → ✅ 可行

建議 (Recommendation):
  → 2330 單價高，NT$50萬資金建議用零股 (Odd Lot) 或改選中價股
  → 2317 鴻海 @ NT$193：1 張 = NT$193,000，NT$50萬可交易 2 張
```

3. If capital is insufficient for 1 lot (1張 = 1000 shares), suggest:
   - Odd lot trading (零股交易)
   - Alternative lower-priced stocks
   - Mini futures (小台指 MTX) as alternative
