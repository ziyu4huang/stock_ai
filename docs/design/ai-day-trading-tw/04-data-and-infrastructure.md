# 04 — 資料來源、券商 API 與基礎設施 (Data & Infrastructure)

## 1. 台灣市場資料來源 (Data Sources)

### 1.1 TWSE OpenAPI（免費 Free）
**網址：** `twse.com.tw` 各端點

| 資料 | 端點 (Endpoint) | 頻率 (Frequency) | 格式 |
|------|----------------|-----------------|------|
| 日 OHLCV | `/exchangeReport/STOCK_DAY` | 每日 (Daily) | JSON |
| 5 秒統計 | `/exchangeReport/MI_5MINS_INDEX` | 5 秒 (5-sec) | JSON |
| 三大法人 | `/fund/T86` | 每日 | JSON/CSV |
| 融資融券 | `/exchangeReport/MI_MARGN` | 每日 | JSON |
| 本益比殖利率 | `/exchangeReport/BWIBBU_d` | 每日 | JSON/CSV |
| 當沖統計 | `/exchangeReport/MI_DAYTRADING` | 每日 | JSON |

**限制 (Limitations)：**
- 5 秒資料為彙總 (Aggregated)，非逐筆 (Not Tick-by-tick)
- 有請求頻率限制 (Rate Limit) — 請求間加入延遲 (Delay)
- 歷史資料有限（通常 1-2 年 via API）
- 無即時 Level 2 委託簿 (No Real-time Level 2 Order Book)

### 1.2 台灣期貨交易所 (Taifex)
**網址：** `taifex.com.tw`

| 資料 | 說明 |
|------|------|
| TX/MTX 日 OHLCV | 期貨日資料 (Futures Daily Data) |
| TX/MTX 日內 | 1 分鐘或逐筆 (1-min or Tick Data) |
| TXO 選擇權日資料 | 每日結算價 (Daily Settlement)、未平倉 (OI) |
| 三大法人 (期貨) | 法人部位 (Institutional Positioning) |

### 1.3 Yahoo Finance（免費）
- **股票代碼格式：** `2330.TW`（上市 TWSE）、`2330.TWO`（上櫃 OTC）
- **日內資料：** `yfinance` 支援 `interval='1m'`（最多 7 天）、`5m`、`15m`、`60m`
- **歷史日資料：** 完整歷史可用
- **限制：** 日內資料僅限最近 7 天，非即時 (Not Real-time)

### 1.4 付費資料供應商 (Paid Providers)

| 供應商 | 資料 | 費用 | 備註 |
|--------|------|------|------|
| **XQ 全球贏家** (嘉實) | 即時 + 歷史、逐筆 (Tick Data) | ~NT$500-3000/月 | 台灣最流行 |
| **永豐金 Shioaji** | 逐筆 + 委託簿 + 交易 | 開戶免費 (Free with Account) | 最適合開發者 |
| **康和證券 API** | 逐筆 + 委託簿 | 開戶免費 | |
| **CMoney** | 財務資料、法人流向 | 訂閱制 | 基本面好用 |
| **TEJ** (台灣經濟新報) | 完整金融資料 | 貴 (NT$1萬+/月) | 機構等級 |

## 2. 券商交易 API — Python

### 2.1 永豐金 Shioaji — ★★★★★（開發者首選）

> **詳盡實作指南請參考 [07-tooling-guide.md](07-tooling-guide-shioaji-lgmb-vectorbt.md)**

**GitHub：** `Sinotrade/Shioaji` | **文件：** `sinotrade.github.io`

**特色：**
- Python 原生 API（無需 COM、無需 DLL、無需 Windows）
- WebSocket 即時報價 (Real-time Quote via WebSocket)
- 逐筆資料串流 (Tick-level Data Streaming)
- 委託簿最佳 5 檔 (Best 5 Levels of Order Book)
- 支援股票 (Stocks)、期貨 (Futures)、選擇權 (Options)
- **活躍社群 (Active Community)**，定期更新
- **免費**（只需開設永豐金帳戶）
- 首個支援 AI Coding Agent (Claude Code / Cursor / Copilot) 的券商 API
- Docker 映像可用 (Docker Images Available)
- 跨平台 (Cross-platform)：Linux、macOS、Windows

### 2.2 群益 Keystone API
- COM-based（僅 Windows，或用 Wine/VM）
- 功能完整：逐筆資料、委託簿、交易
- C# / Python wrapper 可用
- 使用者多但需要 Windows 環境

### 2.3 元大 YesTrader API
- VBA/COM-based
- 完整交易功能 (Full Trading Capabilities)
- 需要 Windows
- 使用者基數大 (Large User Base)

### 2.4 富邦 Neo API
- REST-based API（現代架構）
- 成長中的開發者社群 (Growing Developer Community)
- 網頁交易平台

### 2.5 國際券商 — Interactive Brokers (IBKR)
- **國際券商**，支援台灣市場
- **TWS API：** Java、C++、Python
- **ib_insync** (Python)：IB API 的非同步包裝器 (Async Wrapper)
- 支援全球股票、期貨、選擇權
- **最低手續費：** ~USD 1-2/筆
- **適合：** 跨市場策略 (Multi-market Strategies)

### 2.6 比較總覽

| 券商 (Broker) | 語言 (Language) | 平台 (Platform) | 即時數據 (Real-time) | 交易 (Trading) | Python 原生 |
|---------------|----------------|----------------|---------------------|----------------|-------------|
| **永豐 Shioaji** | Python | Linux/Mac/Win | WebSocket | ✓ | ★★★★★ |
| 群益 Keystone | COM | 僅 Windows | ✓ | ✓ | 需要 wrapper |
| 元大 YesTrader | COM | 僅 Windows | ✓ | ✓ | 需要 wrapper |
| 富邦 Neo | REST | 任意平台 | REST/WS | ✓ | 良好 |
| IBKR | Multi | 任意平台 | ✓ | ✓ | 良好 (ib_insync) |

**推薦：** **永豐金 Shioaji** — Python 原生、Mac/Linux 相容、免費

## 3. 即時資料架構 (Real-time Data Architecture)

### 3.1 當沖資料管線 (Data Pipeline)

```
┌─────────────┐    WebSocket     ┌──────────────┐
│  券商 API    │ ──────────────→  │  資料路由器    │
│  (Shioaji)   │   逐筆資料       │  (Python)     │
└─────────────┘   tick data      └──────┬───────┘
                                        │
                     ┌──────────────────┼──────────────────┐
                     ▼                  ▼                  ▼
              ┌───────────┐    ┌──────────────┐   ┌────────────┐
              │  特徵引擎   │    │    儲存       │   │  儀表板     │
              │  Feature   │    │  (SQLite/    │   │  (Grafana/ │
              │  Engine    │    │   Parquet)   │   │  Terminal) │
              └─────┬─────┘    └──────────────┘   └────────────┘
                    │
                    ▼
              ┌───────────┐
              │  ML 模型    │
              │  推論       │
              │  Inference │
              └─────┬─────┘
                    │
                    ▼
              ┌───────────┐
              │  訊號產生器 │
              │  Signal    │
              │  Generator │
              └─────┬─────┘
                    │
                    ▼
              ┌───────────┐
              │  風控管理   │──→ 超出風險參數則拒絕
              │  Risk      │
              │  Manager   │
              └─────┬─────┘
                    │
                    ▼
              ┌───────────┐
              │  訂單執行器 │──→ 券商 API (Shioaji)
              │  Order     │
              │  Executor  │
              └───────────┘
```

### 3.2 延遲預算 (Latency Budget)
```
逐筆資料到達 (Tick Arrival) → 特徵計算 (Feature Compute) → 模型推論 (Model Inference) → 風控檢查 (Risk Check) → 訂單提交 (Order Submission)

目標（台股當沖，非高頻 HFT）：
  Tick → 特徵：     < 50ms
  特徵 → 訊號：     < 100ms
  訊號 → 訂單：     < 50ms
  總計 (Total)：     < 200ms

台股每 5 秒更新一次，即使 1 秒延遲也足夠（非剝頭皮策略 Scalping）。
Python + 現代 Mac 即可達成。
```

## 4. 回測框架 (Backtesting Frameworks)

### 4.1 vectorbt ★★★★★ — 詳見 [07-tooling-guide.md](07-tooling-guide-shioaji-lgmb-vectorbt.md)
**最適合：** 快速向量化回測 (Fast Vectorized Backtesting)、參數優化 (Parameter Optimization)

- 極快（NumPy/Numba 加速 Vectorized）
- 數秒測試數千種參數組合 (Thousands of Parameter Combos in Seconds)
- 豐富分析指標（Sharpe、Drawdown、交易分析）
- 內建資料連接器 (Built-in Data Connectors)

### 4.2 Backtrader ★★★★
**最適合：** 事件驅動回測 (Event-driven Backtesting)、逼真模擬 (Realistic Simulation)

- 事件驅動（真實訂單模擬 Realistic Order Simulation）
- 內建券商模擬 + 手續費計算 (Built-in Broker Simulation with Commissions)
- 可模擬當沖（強制 13:30 前平倉 Force Close at 13:30）
- Pandas 資料來源支援

### 4.3 FinRL ★★★
**最適合：** 強化學習交易代理 (RL Trading Agents)

- FinRL-X (v3.0)：生產導向 (Production-oriented)
  - ML 選股 + DRL 擇時 + 可擴展策略基礎
  - `bt` 回測引擎 (Backtesting Engine)
  - 多帳戶風控 (Multi-account Risk Controls)
  - Pydantic 設定 (Type-safe Config)
- 支援 14+ 資料來源包括 yfinance、Alpaca、WRDS、Binance
- 演算法 (Algorithms)：PPO、A2C、DDPG、SAC、TD3

### 4.4 自建回測器 (Custom Backtester)
**台灣特定規則需要自建：**

```python
class TaiwanDayTradeBacktester:
    def __init__(self):
        self.tax_rate = 0.0015         # 0.15% 當沖交易稅
        self.commission = 0.0003        # 0.03% 議價手續費
        self.min_commission = 20        # NT$20 最低手續費
        self.tick_sizes = {...}         # 台股檔位規則
        self.trading_hours = (9, 0, 13, 30)  # 09:00-13:30

    def calculate_costs(self, trade_value):
        """計算來回成本 (Round-trip Cost)"""
        commission = max(trade_value * self.commission, self.min_commission)
        tax = trade_value * self.tax_rate   # 僅賣出
        return commission * 2 + tax          # 買賣各一次 + 稅

    def apply_tick_size(self, price, direction):
        """套用台股升降檔位規則"""
        ...
```

## 5. 開發技術棧推薦 (Recommended Tech Stack)

### 5.1 核心技術棧 (Core Stack)
| 元件 (Component) | 工具 (Tool) | 原因 |
|-----------------|------------|------|
| 語言 (Language) | Python 3.10+ | ML 生態系、券商 API 支援 |
| 券商 API (Broker API) | Shioaji (永豐金) | 台灣最佳 Python API |
| 資料儲存 (Data Storage) | SQLite + Parquet | 日內逐筆存 Parquet，日資料存 SQLite |
| ML — 表格 (Tabular) | LightGBM | 快速迭代、高準確度 |
| ML — 序列 (Sequential) | PyTorch LSTM | 捕捉時間依賴 |
| ML — 狀態 (Regime) | hmmlearn | 市場狀態偵測 |
| 回測 (Backtesting) | vectorbt | 快速、支援日內 |
| 特徵工程 (Feature Store) | Pandas + Polars | 快速資料處理 |
| 排程 (Scheduling) | APScheduler / Cron | 每日模型再訓練、盤前分析 |
| 監控 (Monitoring) | Grafana + SQLite | 即時損益儀表板 |
| 警報 (Alerts) | Telegram bot | 即時交易通知 |
| 模型追蹤 (Model Tracking) | MLflow | 追蹤模型指標、版本比較 |
