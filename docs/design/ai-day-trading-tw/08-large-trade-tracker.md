# 08 — 大單追蹤偵測器：技術實作與 HMM 整合 (Large Trade Tracker)

> **定位：** 自動化交易系統的「雷達站」。
> 在台灣市場，要對抗 **0.21% 的手續費與稅金**，追蹤大戶 (Whales / Institutional Players) 的「掃貨點火」動作是最具勝算的策略。大戶進場時，通常會打破原本的 HMM 平穩狀態，產生非線性的動能。

---

## 1. 偵測器核心邏輯 (The Radar Logic)

大單追蹤不只是看「單筆成交金額」，更重要的是看 **「連續性」** 與 **「主動性」**。

### 1.1 主動買入 vs 被動買入

| 行為 | 定義 | 意義 |
|------|------|------|
| **主動買入 (Aggressive Buy)** | 成交在 **Ask (賣價)** | 大戶不計成本向上掃貨 |
| **被動買入 (Passive Buy)** | 成交在 **Bid (買價)** | 大戶掛單等待成交 |
| **主動賣出 (Aggressive Sell)** | 成成交在 **Bid (買價)** | 大戶不計成本向下倒貨 |

### 1.2 大單定義 (Threshold)

| 門檻類型 | 定義 | 說明 |
|----------|------|------|
| **定額門檻** | 單筆 > 500 萬 NTD | 絕對金額，適合大型權值股 |
| **相對門檻** | 單筆成交量 > 過去 10 分鐘平均每分鐘成交量的 20% | 動態適應不同活躍度 |
| **成交量佔比** | 單筆 > 日均成交量 0.1% | 適合小型股偵測 |

### 1.3 點火序列 (Ignition Sequence)

**定義：** 在 30 秒內出現超過 3 筆大單，且價格推升超過 2 個 Tick Size。

```
Timeline (30s window):
──┬──────┬──────┬──────┬──→
  │ Whale│ Whale│ Whale│
  │  Buy │  Buy │  Buy │
  └──┬───└──┬───└──┬───┘
     │      │      │
  Price +2 ticks minimum = IGNITION DETECTED
```

---

## 2. HMM 整合：大單意圖分類

偵測到大單不代表立刻進場。利用 **HMM** 判斷大單的「本質」，避免被假訊號騙入场。

### 2.1 狀態對應表

| 觀測到的大單特徵 | HMM 推算的隱藏狀態 (Hidden State) | 自動化動作 |
|:---|:---|:---|
| 價格低檔 + 緩慢大單 | **State: Accumulation (吸籌)** | 監控，準備進場 |
| 價格突破 + 連續掃單 | **State: Ignition (點火/拉升)** | **確認買入 (BUY)** |
| 價格高檔 + 頻繁大單但價格不漲 | **State: Distribution (出貨/誘多)** | 避開，準備反向 |
| 隨機大單 | **State: Noise (雜訊)** | 忽略，節省手續費 |

### 2.2 雙重確認流程 (Double Confirmation)

```
大單觸發 (Whale Alert)
        │
        ▼
┌─────────────────┐
│  HMM 狀態檢查    │
│  Current State?  │
└────┬────────┬────┘
     │        │
  Bullish   Bearish/
  Ignition  Distribution
     │        │
     ▼        ▼
  BUY ✅    SKIP ❌
```

---

## 3. 技術實作

### 3.1 數據定義與過濾

```python
import pandas as pd

def detect_whales(tick_data, threshold_amount=5_000_000):
    """
    偵測大單成交

    Args:
        tick_data: DataFrame with columns 'price', 'volume', 'side', 'amount'
        threshold_amount: 大單門檻 (NTD), 預設 500 萬

    Returns:
        buy_pressure, sell_pressure: 買賣壓力 (NTD)
    """
    whales = tick_data[tick_data['amount'] >= threshold_amount]

    buy_pressure = whales[whales['side'] == 'Buy']['amount'].sum()
    sell_pressure = whales[whales['side'] == 'Sell']['amount'].sum()

    return buy_pressure, sell_pressure
```

### 3.2 點火序列偵測

```python
def detect_ignition(tick_data, window_seconds=30, min_whales=3, min_tick_move=2):
    """
    偵測點火序列：短時間內連續大單 + 價格推升

    Args:
        tick_data: 大單 DataFrame (已過濾)
        window_seconds: 時間窗口 (秒)
        min_whales: 最少大單筆數
        min_tick_move: 最少 tick 變動數

    Returns:
        list of ignition events with timestamps and price moves
    """
    ignitions = []
    tick_size = get_tick_size(tick_data['price'].iloc[0])

    for i in range(len(tick_data)):
        window = tick_data.iloc[i:i + min_whales]
        if len(window) < min_whales:
            continue

        time_span = (window.index[-1] - window.index[0]).total_seconds()
        price_move = window['price'].iloc[-1] - window['price'].iloc[0]
        tick_moves = price_move / tick_size

        if time_span <= window_seconds and tick_moves >= min_tick_move:
            ignitions.append({
                'start_time': window.index[0],
                'end_time': window.index[-1],
                'whale_count': len(window),
                'price_move': price_move,
                'tick_moves': tick_moves,
                'total_amount': window['amount'].sum(),
            })

    return ignitions
```

### 3.3 整合 HMM 進行雙重確認

```python
def check_with_hmm(current_features, hmm_model):
    """
    當偵測器發現大單時，呼叫 HMM 檢查是否處於趨勢狀態

    Args:
        current_features: 當前市場特徵 (DataFrame)
        hmm_model: 已訓練的 HMM 模型

    Returns:
        bool: 是否確認進場
    """
    state = hmm_model.predict(current_features)
    # 假設 State 0 = Bullish / Ignition
    if state == 0:
        return True
    return False
```

---

## 4. 系統組件

偵測器需要以下組件才能完整運作：

### 4.1 組件概覽

| 組件 | 技術 | 功能 |
|------|------|------|
| **Tick Streamer** | Rust + Shioaji WebSocket | 訂閱 `Exchange.TSE` 逐筆成交 |
| **State Machine** | Python + HMM | 每分鐘更新 HMM 狀態轉移機率 |
| **Visual Alert** | Ratatui (Rust TUI) | 大單觸發時顯示 `WHALE DETECTED: 2330.TW - BUY IGNITION` |
| **Signal Logger** | SQLite | 記錄所有偵測事件供回測驗證 |

### 4.2 數據流

```
Shioaji WebSocket (Ticks)
        │
        ▼
  ┌─────────────┐
  │ Tick Streamer│ (Rust — 低延遲)
  │ 即時過濾     │
  └──────┬──────┘
         │  Whale Ticks Only
         ▼
  ┌─────────────┐     ┌───────────┐
  │ HMM Engine  │────→│ Decision  │
  │ 狀態分類     │     │ BUY/SKIP  │
  └──────┬──────┘     └─────┬─────┘
         │                   │
         ▼                   ▼
  ┌─────────────┐     ┌───────────┐
  │ SQLite Log  │     │ TUI Alert │
  │ 訊號記錄     │     │ 視覺提示  │
  └─────────────┘     └───────────┘
```

---

## 5. 獲利驗證 (Profitability Verification)

在正式自動化下單前，必須通過以下回測驗證：

### 5.1 驗證指標

| 指標 | 目標 | 說明 |
|------|------|------|
| **5 分鐘平均漲幅** | > 0.21% | 大單出現後的平均價格變動 |
| **勝率** | > 55% | 點火訊號的成功率 |
| **期望值** | 正值 (扣除 0.21% 成本後) | 每筆交易的平均獲利 |
| **最大回撤** | < 2% | 單次點火失敗的最大虧損 |

### 5.2 停損規則

- **主規則：** 若大單點火失敗，價格跌破大單成交的最低價，立即停損。
- **時間停損：** 大單出現後 5 分鐘內未推升價格，平倉出場。
- **日內上限：** 單日最多 3 次停損，觸發後停止交易。

### 5.3 回測腳本範例

```python
def backtest_whale_signals(tick_data, whale_signals, holding_period='5min'):
    """
    回測大單訊號的獲利能力

    Args:
        tick_data: 歷史 tick 數據
        whale_signals: 偵測到的大單訊號列表
        holding_period: 持倉時間

    Returns:
        dict: 勝率, 平均獲利, 期望值, 最大回撤
    """
    results = []
    cost = 0.0021  # 0.21% round-trip cost

    for signal in whale_signals:
        entry_price = signal['price']
        entry_time = signal['timestamp']

        # 找到 holding_period 後的價格
        exit_tick = tick_data[tick_data.index > entry_time].head(holding_period)
        if exit_tick.empty:
            continue

        exit_price = exit_tick.iloc[-1]['price']
        pnl = (exit_price - entry_price) / entry_price - cost

        results.append({
            'entry_time': entry_time,
            'pnl': pnl,
            'exit_price': exit_price,
        })

    return {
        'win_rate': sum(1 for r in results if r['pnl'] > 0) / len(results),
        'avg_pnl': sum(r['pnl'] for r in results) / len(results),
        'max_drawdown': min(r['pnl'] for r in results),
        'total_trades': len(results),
    }
```

---

## 6. 下一步行動

### 6.1 優先順序

1. **歷史 Tick 數據回測** — 先用歷史資料驗證大單與價格變動的相關性
2. **門檻校準** — 根據回測結果調整大單定義門檻
3. **HMM 狀態映射** — 將 HMM 的隱藏狀態對應到 Accumulation / Ignition / Distribution / Noise
4. **即時偵測原型** — 用 Shioaji WebSocket 建立即時 Tick Streamer

### 6.2 建議測試標的

| 股票 | 代號 | 適合原因 |
|------|------|----------|
| 台積電 | 2330.TW | 成交量大，大單頻繁，Tick 數據充足 |
| 鴻海 | 2317.TW | 權值股，大戶進出明顯 |
| 聯發科 | 2454.TW | 半導體題材多，波動度適中 |
| 富邦金 | 2881.TW | 金融股，大單與法人動態相關性高 |
