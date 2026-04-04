# 06 — 系統架構設計 (System Architecture)

## 1. 高階架構 (High-Level Architecture)

```
┌──────────────────────────────────────────────────────────────┐
│                   AI 當沖系統 (AI Day Trading System)          │
├──────────────────────────────────────────────────────────────┤
│                                                              │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────────┐ │
│  │  資料層       │  │  ML 管線      │  │  執行層            │ │
│  │  Data Layer   │  │  ML Pipeline  │  │  Execution Layer  │ │
│  │              │  │              │  │                   │ │
│  │ ┌──────────┐│  │ ┌──────────┐ │  │ ┌───────────────┐ │ │
│  │ │ 即時行情   ││  │ │ 特徵引擎   │ │  │ │ 訊號產生器    │ │ │
│  │ │ Real-time ││──→│ │ Feature  │ │  │ │ Signal        │ │ │
│  │ │ (Shioaji) ││  │ │ Engine   │ │  │ │ Generator     │ │ │
│  │ └──────────┘│  │ └────┬─────┘ │  │ └───────┬───────┘ │ │
│  │ ┌──────────┐│  │      │       │  │         │         │ │
│  │ │ 歷史資料   ││  │ ┌────▼─────┐ │  │ ┌───────▼───────┐ │ │
│  │ │ Historical││──→│ │ 模型推論   │ │  │ │ 風控管理      │ │ │
│  │ │ (SQLite/  ││  │ │ Inference│ │  │ │ Risk Manager  │ │ │
│  │ │  Parquet) ││  │ │(LightGBM │ │  │ │ (部位限制,    │ │ │
│  │ └──────────┘│  │ │ +LSTM)   │ │  │ │  停損,        │ │ │
│  │ ┌──────────┐│  │ └──────────┘ │  │ │  每日上限)    │ │ │
│  │ │ 新聞情緒   ││  │              │  │ └───────┬───────┘ │ │
│  │ │ News &   ││  │ ┌──────────┐ │  │         │         │ │
│  │ │ Sentiment││──→│ │ 再訓練    │ │  │ ┌───────▼───────┐ │ │
│  │ │ (NLP)    ││  │ │ Retrain  │ │  │ │ 訂單執行器    │ │ │
│  │ └──────────┘│  │ │ Pipeline │ │  │ │ Order         │ │ │
│  └──────────────┘  │ │ (每日)    │ │  │ │ Executor      │ │ │
│                    │ └──────────┘ │  │ │ (Shioaji API) │ │ │
│                    └──────────────┘  │ └───────────────┘ │ │
│                                      └───────────────────┘ │
│  ┌───────────────────────────────────────────────────────┐ │
│  │                   監控層 (Monitoring Layer)              │ │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────┐ │ │
│  │  │ 損益儀表板│  │ 模型監控  │  │ 系統健康  │  │電報   │ │ │
│  │  │ P&L Dash │  │ Model    │  │ System   │  │Alerts │ │ │
│  │  │(Grafana) │  │ Monitor  │  │ Health   │  │(TG)   │ │ │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────┘ │ │
│  └───────────────────────────────────────────────────────┘ │
└──────────────────────────────────────────────────────────────┘
```

## 2. 元件設計 (Component Design)

### 2.1 資料層 (Data Layer)

#### 即時行情服務 (Real-time Market Data Service)
```
職責 (Responsibilities)：
  - 連接券商 WebSocket (Shioaji WebSocket)
  - 接收逐筆成交資料 (Tick-by-tick Data)
  - 建構即時 OHLCV K 線 (Build Real-time OHLCV Bars)：1分/5分/15分
  - 串流至特徵引擎 (Stream to Feature Engine)
  - 儲存逐筆資料至 Parquet (Store Ticks to Parquet)

技術 (Technology)：
  - Python asyncio + WebSocket
  - 環形緩衝區 (Ring Buffer) 存最新 N 根 K 線
  - Parquet 檔案按日期分區 (Partition by Date)

資料流 (Data Flow)：
  Shioaji WS → 逐筆解析器 (Tick Parser) → K線建構器 (Bar Builder) → 特徵引擎
                                          ↓
                                     Parquet 儲存（每日一檔 Daily File）
```

#### 歷史資料服務 (Historical Data Service)
```
職責：
  - 下載並快取歷史日/日內資料 (Download & Cache Historical Data)
  - TWSE OpenAPI + yfinance 作為來源
  - 提供回測與模型訓練的資料

儲存佈局 (Storage Layout)：
  data/
    daily/                          # 日資料
      2330.TW.parquet
      2317.TW.parquet
    intraday/                       # 日內資料
      2024-01-15/
        2330.TW_1m.parquet          # 1 分鐘 K 線
        2330.TW_tick.parquet        # 逐筆資料
    fundamental/                    # 基本面
      institutional_flows.parquet   # 三大法人
      margin_balance.parquet        # 融資融券
```

### 2.2 ML 管線 (ML Pipeline)

#### 特徵引擎 (Feature Engine)
```
輸入 (Input)：即時 OHLCV K 線 (1 分鐘)
輸出 (Output)：特徵向量 (Feature Vector, 50-200 個特徵)

特徵類別 (Feature Categories)：
  1. 價格特徵 (Price)：報酬率 (Returns)、動量 (Momentum)、日內位置 (Range Position)
  2. 量能特徵 (Volume)：相對量 (Relative Volume)、資金流 (Money Flow)
  3. 波動特徵 (Volatility)：已實現波動率 (Realized Vol)、ATR、區間 (Range)
  4. 時間特徵 (Time)：sin/cos 編碼時段、市場階段 (Market Phase)
  5. 截面特徵 (Cross-sectional)：vs 指數、vs 類股 (vs Index, vs Sector)

更新頻率：每 1 分鐘（對齊 K 線收盤 Bar Close）
延遲目標：< 100ms 完成全部特徵計算
```

#### 模型推論服務 (Model Inference Service)
```
模型集成 (Model Ensemble)：
  1. LightGBM：表格特徵 → 機率 (Tabular Features → Probability)
  2. LSTM (PyTorch)：序列特徵 → 機率 (Sequential Features → Probability)
  3. HMM：狀態分類 → 市場狀態 (Regime Classification → Market State)

訊號整合 (Signal Aggregation)：
  p_final = 0.5 × p_lgbm + 0.3 × p_lstm + 0.2 × regime_signal
  （權重透過滾動優化調校 Weights Tuned via Walk-Forward Optimization）

輸出 (Output)：
  - 方向 (Direction)：BUY 買進 / SELL 賣出 / HOLD 觀望
  - 確信度 (Confidence)：0-100%
  - 預期報酬 (Expected Return)：±X%
  - 建議持倉時間 (Recommended Holding Time)：N 分鐘
```

#### 再訓練管線 (Retrain Pipeline)
```
排程 (Schedule)：每日 15:00（收盤後 After Market Close）

步驟 (Steps)：
  1. 取得今日資料和實際結果 (Fetch Today's Data & Outcomes)
  2. 標記 (Label)：訊號是否正確？(實際報酬 vs 預測 Actual vs Predicted)
  3. 加入訓練集 (Append to Training Set)：滾動 6 個月 (Rolling 6 Months)
  4. 重新訓練 LightGBM（5 分鐘 Retrain）
  5. 重新訓練 LSTM（30 分鐘，需要時 Only if Needed）
  6. 在保留集上驗證 (Validate on Holdout Set)
  7. 若績效改善 → 部署新模型 (Deploy New Model if Improved)
  8. 若績效退化 → 保留舊模型，發出警報 (Keep Old + Alert if Degraded)

模型版本管理 (Model Versioning)：
  - 存在 models/ 目錄，加上日期戳記 (Date Stamp)
  - 用 MLflow 追蹤指標 (Track Metrics)
  - 實盤表現下降時快速回滾 (Quick Rollback)
```

### 2.3 執行層 (Execution Layer)

#### 訊號產生器 (Signal Generator)
```python
class SignalGenerator:
    """將模型輸出轉為交易訊號 (Convert Model Output to Trading Signal)"""

    def generate_signal(self, features, model_output):
        direction = model_output.direction      # BUY / SELL
        confidence = model_output.confidence     # 0-1

        # 只在高確信度時交易 (Only Trade on High Conviction)
        if confidence < 0.65:
            return Signal.HOLD

        # 檢查市場狀態 (Check Regime)
        if regime == "uncertain":
            return Signal.HOLD

        return Signal(direction, confidence)
```

#### 風控管理器 (Risk Manager)
```python
class RiskManager:
    """交易前風險檢查 (Pre-trade Risk Check)"""

    def check(self, signal, portfolio):
        # 部位數量限制 (Position Limit)
        if portfolio.open_positions >= MAX_POSITIONS:
            return Reject("達到最大持倉數 max positions reached")

        # 每日虧損限制 (Daily Loss Limit)
        if portfolio.daily_pnl < -MAX_DAILY_LOSS:
            return Reject("觸及每日虧損上限 daily loss limit hit")
            # 同時：平掉所有部位並停止交易 (Also: Close All & Stop Trading)

        # 每筆風險 (Per-trade Risk)
        max_risk = portfolio.capital * 0.01       # 1% per trade
        position_size = max_risk / (signal.entry - signal.stop_loss)

        # 集中度限制 (Concentration Limit)
        if portfolio.exposure_to(signal.symbol) > 0.3:
            return Reject("集中度超限 concentration limit")

        # 時間檢查 (Time Check)：13:00 後不開新部位
        if current_time > time(13, 0):
            return Reject("接近收盤 too close to market close")

        return Approve(position_size)
```

#### 訂單執行器 (Order Executor)
```python
class OrderExecutor:
    """執行交易訂單 (Execute Trading Orders)"""

    def execute(self, signal, position_size):
        # 使用限價單獲得更好的成交價 (Use Limit Orders for Better Fills)
        order = LimitOrder(
            symbol=signal.symbol,
            side=signal.direction,
            quantity=position_size,
            price=current_price,              # 當前價限價 at-the-money limit
            time_in_force="IOC",              # 立即或取消 Immediate or Cancel
        )
        result = broker.place_order(order)

        # 若未成交，嘗試市價單（有滑價保護）
        if not result.filled and signal.confidence > 0.80:
            result = broker.place_order(MarketOrder(...))

        # 設定停損 (Set Stop-Loss)
        if result.filled:
            broker.place_order(StopOrder(
                stop_price=signal.stop_loss,
                ...
            ))

        return result

    def force_close_all(self):
        """13:25 前強制平倉所有部位 (Force Close All Before 13:25)"""
        for position in portfolio.open_positions:
            broker.place_order(MarketOrder(
                symbol=position.symbol,
                side=opposite(position.side),
                quantity=position.quantity,
            ))
```

## 3. 技術棧 (Technology Stack)

| 元件 (Component) | 技術 (Tech) | 替代方案 (Alternative) |
|-----------------|------------|---------------------|
| 語言 (Language) | Python 3.10+ | - |
| 券商 API (Broker) | Shioaji (永豐金) | 富邦 Neo |
| 資料儲存 (Storage) | SQLite + Parquet | PostgreSQL |
| ML — 表格 (Tabular ML) | LightGBM | XGBoost, CatBoost |
| ML — 序列 (Sequential ML) | PyTorch LSTM | TensorFlow |
| ML — 狀態 (Regime ML) | hmmlearn | 自建 HMM (Custom) |
| 回測 (Backtesting) | vectorbt | Backtrader |
| 特徵工程 (Features) | Pandas + Polars | DuckDB |
| 排程 (Scheduling) | APScheduler | Cron |
| 監控 (Monitoring) | Grafana + SQLite | Streamlit |
| 警報 (Alerts) | python-telegram-bot | Discord webhook |
| 模型追蹤 (Model Tracking) | MLflow | Weights & Biases |
| 設定 (Config) | YAML + Pydantic | TOML |

## 4. 專案結構 (Project Structure)

```
stock_ai/
├── crates/                              # Rust 後端（既有）
├── webui/                               # 前端（既有）
├── docs/
│   └── design/
│       └── ai-day-trading-tw/           # 本知識庫
├── daytrader/                           # 新增：當沖模組
│   ├── __init__.py
│   ├── config/
│   │   ├── settings.py                 # Pydantic 設定
│   │   └── instruments.yaml            # 股票/期貨設定
│   ├── data/
│   │   ├── realtime.py                 # WebSocket 即時數據
│   │   ├── historical.py               # 歷史資料下載
│   │   ├── bar_builder.py              # 逐筆 → OHLCV 聚合
│   │   └── storage.py                  # Parquet/SQLite 儲存
│   ├── features/
│   │   ├── technical.py                # 技術指標 (Technical Indicators)
│   │   ├── volume.py                   # 量能特徵
│   │   ├── microstructure.py           # 委託簿特徵 (Order Book Features)
│   │   └── cross_sectional.py          # 截面特徵
│   ├── models/
│   │   ├── lgbm_model.py               # LightGBM 包裝器 (Wrapper)
│   │   ├── lstm_model.py               # PyTorch LSTM
│   │   ├── hmm_regime.py               # HMM 狀態偵測
│   │   └── ensemble.py                 # 模型集成 (Ensemble)
│   ├── strategy/
│   │   ├── signal_generator.py         # 訊號產生 (Signal Generation)
│   │   ├── risk_manager.py             # 部位大小 + 風控
│   │   └── order_executor.py           # 訂單管理 (Order Management)
│   ├── backtest/
│   │   ├── engine.py                   # vectorbt 回測器
│   │   ├── cost_model.py               # 台灣特有成本模型
│   │   └── walk_forward.py             # 滾動驗證 (Walk-Forward)
│   ├── monitoring/
│   │   ├── pnl_tracker.py              # 即時損益追蹤
│   │   ├── model_monitor.py            # 模型績效監控
│   │   └── telegram_bot.py             # 交易警報
│   ├── main.py                         # 主程式入口 (Main Entry Point)
│   └── train.py                        # 模型訓練腳本
├── data/                                # 資料目錄
│   ├── daily/
│   ├── intraday/
│   └── models/
└── tests/
    └── daytrader/
```

## 5. 實作階段 (Implementation Phases)

### 階段一：基礎建設 (Phase 1, 第 1-4 週)
- [ ] 設定 Shioaji 連線和資料收集 (Setup Shioaji + Data Collection)
- [ ] 建構歷史資料管線 (Build Historical Data Pipeline)：TWSE + yfinance
- [ ] 實作特徵引擎（基礎技術指標）(Implement Feature Engine)
- [ ] 建構簡單 LightGBM 模型預測下一 15 分鐘 (Build LightGBM for Next-15min)
- [ ] 回測框架搭配台灣成本模型 (Backtesting + TW Cost Model)
- [ ] 模擬交易模式（不下真實訂單）(Paper Trading Mode)

### 階段二：強化 (Phase 2, 第 5-8 週)
- [ ] 加入 LSTM 模型處理序列特徵 (Add LSTM for Sequential Features)
- [ ] 實作 HMM 狀態偵測 (Implement HMM Regime Detection)
- [ ] 建構集成模型 (Build Ensemble Model)
- [ ] 即時監控儀表板 (Real-time Monitoring Dashboard)
- [ ] 風控管理系統 (Risk Management System)
- [ ] Telegram 交易警報 (Telegram Alerts)

### 階段三：實盤交易 (Phase 3, 第 9-12 週)
- [ ] 連接 Shioaji 進行即時下單 (Connect Shioaji for Live Order Execution)
- [ ] 從最小部位開始（零股 / 1 股）(Start with Minimum Size)
- [ ] 監控實盤 vs 回測表現 (Monitor Live vs Backtest)
- [ ] 持續迭代模型和策略 (Iterate on Model & Strategy)
- [ ] 逐步增加部位大小 (Gradually Increase Position Size)

### 階段四：優化 (Phase 4, 第 13 週+)
- [ ] 加入台指期 (TX/MTX) 支援 (Add Futures Support)
- [ ] 多股票策略 (Multi-stock Strategy)
- [ ] 進階訂單執行（TWAP 等）(Advanced Order Execution)
- [ ] 自動模型再訓練管線 (Auto Retrain Pipeline)
- [ ] 效能優化（降低延遲）(Performance Optimization)
