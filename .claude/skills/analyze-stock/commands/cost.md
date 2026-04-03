# /analyze-stock cost

Taiwan day trading cost analysis for a stock.

## Usage

```
/analyze-stock cost 2330.TW
```

## Steps

1. Run `bun run yf-query.ts quote SYMBOL` to get current price
2. Calculate and display:

```
=== 當沖成本分析 (Day Trading Cost Analysis) ===
標的: 2330 台積電
現價: NT$1,810

1 張 (1000 股) 成交值 (Trade Value): NT$1,810,000

標準費率 (Standard Rate 0.1425%):
  買進手續費 (Buy Commission):    NT$2,579
  賣出手續費 (Sell Commission):   NT$2,594
  當沖交易稅 (Day Trade Tax):     NT$2,730
  來回總成本 (Round-trip Cost):   NT$7,903 (0.43%)

議價費率 (Negotiated Rate 0.03%):
  買進手續費:    NT$543
  賣出手續費:   NT$546
  當沖交易稅:   NT$2,730
  來回總成本:   NT$3,819 (0.21%)

損益兩平 (Breakeven):
  需要價格移動 ≥ 0.21% = NT$3.80
  相當於 1 檔 (NT$5) → 實際利潤 NT$1,190 (1 檔 − 成本)
```

3. Show tick friction analysis with the tick size table
4. Recommend strategy type based on tick friction level
