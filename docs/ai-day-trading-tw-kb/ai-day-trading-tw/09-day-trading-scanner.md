# 09 — Day Trading Scanner (日內交易掃描器)

## 概述

Day Trading Scanner 是 WebUI 的功能模組，用於掃描觀察清單中的股票，偵測買賣信號, 並以視覺化方式呈現。

用戶只需 1D 日線資料 + 贖賣信號偵測系 即可找到潽在的交易候選股票。

所有信號偵測邏輯在 Rust 環計算（效能考量： 掃描 20+ 檔股票需快速），不需 Python）

---

## 三種模式

### 1. Scan List（掃描列表）
- 掃描觀察清單中所有股票， 依複合評分排序
- 顯示： 代碼、名稱、最新價、評分、方向、前三大信號
- 點擊進入 Analysis 訡式

- API: `GET /api/scan-signals`

### 2. Analysis（分析模式）
- 單一股票的完整日線分析圖表
- K 線圖 + BB 帶 + EMA5/20 疊加
- 買入/賣出箭頭標記 (來自伺服器計算的信號)
- 成交量條 (大單量成交以橙色標記)
- 點擊 K 線查看信號詳情 (觸發了哪些指標、強度、原因)
- API: `GET /api/daytrade/:symbol`

### 3. Replay（回放模式）
- 逐根 K 線逐步回放歷史資料, 模擬實盤觀察
- Play/Pause/Step 控制按鈕
- 速度滑桿 (100ms ~ 3s)
- 即時 P&L 追蹤: 買入信號進場、賣出信號出場
- 顯示: 交易次數、勝率、總損益

---

## 信號偵測系統

### 偵測邏輯 (共 11 種信號)

| 信號 | 偵測條件 | 評分 | 方向 |
|------|---------|------|------|
| RSI Reversal | RSI 從 <30 回升 (買) / 從 >70 回落 (賣) | +20/+30 | Buy/Sell |
| MACD Hist Cross | MACD 柱狀圖零線交叉 | +20/-20 | Buy/Sell |
| MACD Signal Cross | MACD 線與信號線交叉 | +15/-15 | Buy/Sell |
| BB Squeeze | 布林帶寬度處於近 60 日底部 20% | +20/-20 | Buy/Sell |
| BB Breakout | 擠壓後價格突破帶狀 | +30/-30 | Buy/Sell |
| Volume Surge | 成交量 > 2x 20日均量 | +10/+20/+30 | Buy/Sell |
| Bull Engulfing | 看漲吞沒型態 | +20 | Buy |
| Bear Engulfing | 看跌吞沒型態 | -20 | Sell |
| Hammer | 鎚子線型態 | +20 | Buy |
| Shooting Star | 射擊之星型態 | -20 | Sell |
| Morning Star | 晨星 (3K線反轉) | +30 | Buy |
| Evening Star | 暮星 (3K線反轉) | -30 | Sell |
| EMA Cross | EMA(5) 與 EMA(20) 交叉 | +20/-20 | Buy/Sell |
| Gap Up/Down | 開盤跳空 > 1% | +10/-10 | Buy/Sell |

### 複合評分計算
```
composite_score = clamp(sum(signal_scores), -100, +100)
```
- 買入信號貢獻正分 (+10 ~ +30)
- 賣出信號貢獻負分 (-10 ~ -30)
- 強度分三級: Weak (±10), Medium (±20), Strong (±30)

### K 線型態判斷邏輯

**Engulfing (吞沒):**
- 前一根為陰/陽, 當前根為陽/陰
- 當前根實體完全包含前一根實體

**Hammer (鎚子):**
- 當前根為陽線 (close > open)
- 下影線 ≥ 2 倍實體
- 上影線 ≤ 0.3 倍實體

**Morning/Evening Star (晨星/暮星):**
- 3 根 K 線組合
- 第一根大陰/陽、第二根小實體、第三根大陽/陰
- 第三根收盤進入第一根實體範圍

---

## API 規格

### GET /api/scan-signals
回傳所有觀察清單股票的掃描結果，依評分排序。

**Response:**
```json
{
  "results": [
    {
      "symbol": "2330.TW",
      "name": "台積電",
      "last_price": 850.0,
      "score": 55,
      "direction": "Buy",
      "top_signals": ["MACD Hist Cross", "Bull Engulfing", "EMA Cross"]
    }
  ]
}
```

### GET /api/daytrade/:symbol
回傳單一股票的完整日交易分析。

**Response:**
```json
{
  "symbol": "2330.TW",
  "bars": [...],
  "signals": [
    {
      "idx": 156,
      "date": "2026-03-15",
      "signals": [
        {
          "kind": "RsiReversal",
          "direction": "Buy",
          "strength": "Strong",
          "score": 30,
          "reason": "RSI(14) crossed up from 24.5 to 31.2"
        }
      ],
      "score": 30
    }
  ],
  "latest_score": 55,
  "latest_direction": "Buy"
}
```

---

## 前端實現
- **Scanner 按鈕:** 在 toolbar 新增 "Scanner" 按鈕切換掃描器視圖
- **子工具列:** Scan List / Analysis / Replay 三個子模式切換
- **掃描表格:** 顏色編碼評分 (綠=買入, 紅=賣出, 灰=中性)
- **分析圖表:** 使用 ECharts，複用全域 chart 實例
- **回放控制:** Play/Pause/Step + 速度滑桿 + P&L 統計

顯示

### 圖表疊加層
- K 線 (Candlestick): 紅漲綠跌
- BB Bands (布林通道): 上/中/下軌道
- EMA(5) / EMA(20): 短期/中期趨勢
- 買入標記: 緑色向上箭頭
- 賣出標記: 紅色向下箭頭
- 成交量: 大單量以橙色標記

---

## 檔案結構

| 檔案 | 說明 |
|------|------|
| `crates/stock-core/src/types.rs` | 信號相關型別定義 |
| `crates/stock-core/src/indicators.rs` | 序列指標函數 + 信號偵測器 + `analyze_daytrade()` |
| `crates/stock-server/src/main.rs` | API 端點 (`api_scan_signals`, `api_daytrade`) |
| `crates/stock-server/build.rs` | HTML + CSS (Scanner UI) |
| `crates/stock-server/webui/src/app.tsx` | 前端邏輯 (掃描/分析/回放) |

---

## 使用流程

1. 在 WebUI 點擊 **Scanner** 按鈕
2. 系統自動掃描觀察清單 → 顯示排序後的候選股列表
3. 點擊有興趣的股票 → 進入 **Analysis** 查看詳細買賣信號
4. 切換到 **Replay** → 點擊 Play 觀看信號如何在歷史中逐根觸發
5. 利用 P&L 統計評估策略有效性

---

## 未來改進方向
- 增加 VWAP 茇標 (需要分鐘線資料)
- 增加 Stochastic RSI
- 加入支撐/阻力位偵測
- 結合 HMM 狀態作為額外信號權重
- 策略回測 (依歷史信號計算累計報酬)
- 即時推送通知 (搭配 WebSocket)
