# 03 — AI 與量化交易技術 (AI & Quant Techniques)

## 1. 特徵工程 (Feature Engineering) — 日內交易

### 1.1 價格特徵 (Price Features)
| 特徵 | 說明 | 用途 |
|------|------|------|
| 報酬率 (Returns)：1分/5分/15分/30分/60分 | 價格變動百分比 | 動量訊號 (Momentum Signal) |
| VWAP 偏離度 (VWAP Deviation) | 現價 vs 量均價 | 均值回歸 (Mean Reversion) |
| 日內高低點範圍 (Intraday Range) | 日內波幅 / ATR | 波動率判斷 (Volatility Regime) |
| 收盤位置 (Close Position) | (收盤-低)/(高-低) | 買賣壓力 (Buying/Selling Pressure) |
| 跳空 (Gap) | 開盤價 vs 前日收盤 | 動量/均值回歸訊號 |

### 1.2 量能特徵 (Volume Features)
| 特徵 | 說明 | 用途 |
|------|------|------|
| 量比 (Volume Ratio) | 成交量 / 20日均量 | 異常活動偵測 (Unusual Activity) |
| 量價分佈 (Volume Profile) | 各價位的成交量 | 支撐壓力 (Support/Resistance) |
| 買賣力道 (Order Imbalance) | (買量-賣量) / 總量 | 方向性壓力 (Directional Pressure) |
| 資金流向 (Money Flow) | 價格 × 成交量方向 | 主力追蹤 (Smart Money Tracking) |

### 1.3 時間特徵 (Time Features)
| 特徵 | 說明 | 用途 |
|------|------|------|
| 時段編碼 (Time of Day, sin/cos) | 開盤後的分鐘數 | 日內季節性 (Intraday Seasonality) |
| 星期幾 (Day of Week, sin/cos) | 週一至週五 | 週規律 (Weekly Pattern) |
| 距收盤時間 (Minutes to Close) | 剩餘交易時間 | 尾盤行為 (Close Behavior) |
| 市場階段 (Market Phase) | 開盤/盤中/尾盤 | 分階段建模 (Phase-Specific Model) |
| 午餐效應 (Lunch Effect, 11:30-12:30) | 量能下降期 | 流動性降低警示 |

### 1.4 截面特徵 (Cross-Sectional Features) — 台灣市場
| 特徵 | 說明 | 用途 |
|------|------|------|
| 類股相對強弱 (Sector Relative Strength) | 個股報酬 vs 類股指數 | 板塊輪動 (Rotation Signal) |
| 指數相關性 (Index Correlation) | 與加權指數連動 | 市場 vs 個股特有訊號 |
| 同業比較 (Peer Comparison) | 與同業表現比較 | 配對交易 (Pairs Trading) |
| 三大法人流向 (Institutional Flows) | 外資/投信/自營商買賣超 | 主力動向 (Smart Money) |
| 融資融券變化 (Margin Balance Change) | 融資融券餘額變動 | 市場情緒 (Sentiment) |

### 1.5 微結構特徵 (Microstructure Features)
| 特徵 | 說明 | 用途 |
|------|------|------|
| 買賣價差 (Bid-Ask Spread) | (Ask-Bid) / 中間價 | 流動性成本 (Liquidity Cost) |
| 委託簿深度 (Order Book Depth) | 最佳 5 檔量能 | 供需失衡 (Supply/Demand Imbalance) |
| 成交量分佈 (Trade Size Distribution) | 大單 vs 小單比例 | 法人 vs 散戶判斷 |
| 訂單流毒性 (Order Flow Toxicity / VPIN) | 資訊交易者比例 | 知情交易偵測 (Informed Trading) |
| 價格衝擊 (Price Impact) | 單位成交量的價格變動 | 市場深度估計 |

## 2. 機器學習模型 (Machine Learning Models)

### 2.1 梯度提升樹 (Gradient Boosted Trees / XGBoost / LightGBM)
**最適合：** 表格特徵 (Tabular Features)、可解釋性 (Interpretability)、快速迭代 (Fast Iteration)

```
輸入特徵：技術指標 + 量能特徵 + 時間特徵 = 50-200 個特徵
目標變數 (Target)：未來 5/15/30 分鐘報酬率方向 (二元分類 Binary Classification)
訓練方式：滾動窗口 (Rolling Window, 例如 6 個月)，每週重新訓練 (Retrain)

優點 (Advantages)：
  - 混合特徵類型處理能力好 (Handles Mixed Feature Types)
  - 訓練和推論速度快 (Fast Training & Inference)
  - 內建特徵重要性 (Built-in Feature Importance)
  - 對離群值 (Outliers) 魯棒 (Robust)
缺點 (Disadvantages)：
  - 無法自然捕捉序列模式 (Can't Capture Sequential Patterns)
  - 需要手動特徵工程 (Manual Feature Engineering)
```

### 2.2 LSTM / GRU (長短期記憶網路 / Recurrent Neural Networks)
**最適合：** 序列模式 (Sequential Patterns)、多步預測 (Multi-step Prediction)、時間依賴性 (Temporal Dependencies)

```
輸入格式：[batch_size, 序列長度(seq_len), 特徵數(features)]
  - seq_len：60-240 分鐘的歷史資料
  - features：OHLCV + 技術指標（10-50 個特徵）
輸出：[batch_size, 預測期數(forecast_horizon)]
  - 預測未來 1/5/15 分鐘的報酬率或價格

架構 (Architecture)：
  Input → LSTM(128) → LSTM(64) → Dense(32) → Dense(1)

關鍵要點：
  - 使用滾動窗口標準化 (Rolling Window Normalization)，不使用全局標準化
  - 使用報酬率/變化量 (Returns/Changes)，不使用絕對價格 (Absolute Prices)
  - 長序列加入注意力機制 (Attention Mechanism)
  - 層間加入 Dropout (0.2-0.5)
```

### 2.3 Transformer 模型
**最適合：** 捕捉長距離依賴 (Long-range Dependencies)、注意力機制 (Attention over Time Series)

```
時間序列變體 (Architecture Variants)：
  - PatchTST：將時間步打包 (Patch) 為 token，大幅提升效率
  - Informer：高效注意力機制，適合長序列
  - Temporal Fusion Transformer (TFT)：多視角預測，可解釋注意力

優點：捕捉長距離依賴、可解釋的注意力權重、多項基準測試最佳
缺點：計算成本高 (Computationally Expensive)、需要更多資料、有限日內資料容易過擬合
```

### 2.4 強化學習 (Reinforcement Learning / RL)
**最適合：** 端到端交易代理 (End-to-End Trading Agent)、學習執行策略 (Execution Strategy)

```
框架：FinRL、Stable-Baselines3、Ray RLlib

代理設定 (Agent Setup)：
  狀態 (State)：  市場特徵 + 投資組合狀態 + 剩餘時間
  動作 (Action)： 買進 / 賣出 / 觀望 + 部位大小 (Position Size)
  獎勵 (Reward)： 損益 + 風險調整（Sharpe / Sortino）

演算法 (Algorithms)：
  - PPO (Proximal Policy Optimization)：最穩定，推薦預設使用
  - DQN (Deep Q-Network)：離散動作空間
  - SAC/TD3：連續動作空間，樣本效率高

建議：用監督式學習 (Supervised Learning) 產生訊號 → 用 RL 優化執行 (Execution)
```

### 2.5 隱藏馬可夫模型 (Hidden Markov Model / HMM)
**最適合：** 市場狀態偵測 (Regime Detection)、多空盤整識別

```
使用場景：辨識多頭 (Bull) / 空頭 (Bear) / 盤整 (Range-bound) 市場狀態
隱藏狀態數 (States)：2-5 個
觀測值 (Observations)：報酬率、成交量、波動率

我們的現有實作 (stock_api_cli)：
  python3 -m stock_api_cli analyze 2330.TW --states 4

應用於當沖：
  - 開盤前：預測今日可能的市場狀態 (Pre-market Regime Prediction)
  - 狀態感知策略 (Regime-aware Strategy)：不同狀態使用不同參數
  - 風險管理：不確定狀態時降低部位 (Reduce Size in Uncertain Regimes)

限制：落後偵測 (Lagging Detection)、高斯假設 (Gaussian Assumption) 常不成立
建議：作為輔助訊號 (Supplementary Signal)，不作為主要訊號
```

### 2.6 NLP 情緒分析 (Sentiment Analysis)
**最適合：** 新聞驅動波動 (News-driven Moves)、財報反應 (Earnings Reactions)

```
台灣資料來源 (Data Sources)：
  - TWSE 重大訊息 (Material Disclosures)
  - 公開資訊觀測站 MOPS 公告
  - 新聞來源 (經濟日報、工商時報)
  - PTT Stock 板（散戶情緒 Retail Sentiment）

模型：
  - BERT (中文)：ckiplab/bert-base-chinese
  - FinBERT：金融情緒分析
  - LLM (GPT/Claude)：摘要 + 情緒評分

應用：開盤前掃描隔夜新聞、盤中即時新聞事件偵測、持倉風險警示
```

## 3. 日內量化策略 (Quantitative Strategies)

### 3.1 日內動量策略 (Intraday Momentum)
```
假說 (Hypothesis)：前 30 分鐘走勢傾向延續 (Continuation)

訊號產生 (Signal Generation)：
  - 計算開盤到 09:30 的報酬率 (Opening 30-min Return)
  - 報酬率 > 門檻值（如 +0.5%）→ 買進 (動量追蹤 Momentum)
  - 報酬率 < 門檻值（如 -0.5%）→ 賣出 (動量追蹤)
  - 13:00 出場或 -0.5% 停損 (Stop-Loss)

AI 加強 (Enhancement)：
  - 用 ML 預測哪些股票的動量會延續
  - 特徵：量比、類股動量、法人流向
  - 標籤 (Label)：10:00-13:00 報酬是否延續 09:00-10:00 方向
```

### 3.2 VWAP 均值回歸 (VWAP Mean Reversion)
```
假說 (Hypothesis)：價格傾向回歸 VWAP

訊號產生：
  - 偏離度 = (現價 - VWAP) / VWAP
  - 偏離度 > +1%：賣出訊號（價格偏高，預期回歸 Sell Signal）
  - 偏離度 < -1%：買進訊號（價格偏低，預期回歸 Buy Signal）
  - 目標：回到 VWAP
  - 停損：偏離度超過 ±2%

AI 加強：
  - ML 預測回歸是否會發生 (Binary Classifier)
  - 特徵：量價分佈、時段、近期波動率
  - 只在 ML 確認回歸機率 > 60% 時交易
```

### 3.3 開盤區間突破 (Opening Range Breakout / ORB)
```
假說 (Hypothesis)：前 15-30 分鐘的區間預測日內方向

訊號產生：
  - 標記開盤區間 (Opening Range)：09:00-09:15 (或 09:30) 的最高最低價
  - 突破區間高點 → 買進 (Breakout Buy)
  - 跌破區間低點 → 賣出 (Breakdown Sell)
  - 停損：回到區間內
  - 目標：2 倍開盤區間 或 時間出場 (Time Exit, 13:00)

AI 加強：
  - ML 預測突破是否會成功
  - 特徵：盤前量能、跳空方向、類股強弱
  - 過濾低機率突破 (Filter Low-Probability Breakouts)
```

### 3.4 配對交易 (Pairs Trading / Intraday)
```
假說 (Hypothesis)：相關股票維持相對價格關係

台股配對範例 (Pairs Examples)：
  - 2330 (台積電) vs 2337 (旺宏) — 半導體
  - 2317 (鴻海) vs 2354 (鴻準) — 鴻海集團
  - 2881 (富邦金) vs 2882 (國泰金) — 金融股

訊號產生：
  - 計算價差 (Spread)：log(P_A) - β × log(P_B)
  - 價差 Z-score > 2：做空 A、做多 B
  - 價差 Z-score < -2：做多 A、做空 B
  - Z-score 回到 0 時出場

AI 加強：ML 預測收斂時間、動態避險比率 (Dynamic Hedge Ratio)、狀態感知門檻
```

### 3.5 訂單流策略 (Order Flow Strategy)
```
假說 (Hypothesis)：大單流向預測價格方向

所需資料 (Data Needed)：
  - Level 2 委託簿 (Order Book, 最佳 5 檔)
  - 逐筆成交資料 (Tick-by-tick Trade Data)
  - 成交方向分類 (Trade Direction Classification)

訊號產生：
  - 訂單失衡 (Order Imbalance) = (買量 - 賣量) / 總量
  - 委託簿失衡 (Book Imbalance) = (買檔深度 - 賣檔深度) / 總深度
  - 成交加速度 (Trade Acceleration)：淨訂單流變化率

AI 加強：
  - LSTM 處理訂單流序列 (Order Flow Sequence)
  - 預測短期 (1-5 分鐘) 價格方向
  - 即時訊號搭配 WebSocket 資料
```

## 4. 策略評估指標 (Strategy Evaluation Metrics)

### 4.1 基本指標
| 指標 | 公式 | 良好門檻 |
|------|------|----------|
| 勝率 (Win Rate) | 獲利次數 / 總交易次數 | > 55% |
| 獲利因子 (Profit Factor) | 總獲利 / 總虧損 | > 1.5 |
| 夏普比率 (Sharpe Ratio) | 平均報酬 / 報酬標準差 × √252 | > 1.5 (年化) |
| 最大回撤 (Max Drawdown) | max(高峰 - 谷底) / 高峰 | < 10% |
| 期望值 (Expectancy) | 平均獲利×勝率 - 平均虧損×敗率 | > 0 |
| 卡瑪比率 (Calmar Ratio) | 年化報酬 / 最大回撤 | > 1.0 |

### 4.2 當沖專用指標
| 指標 | 說明 | 目標 |
|------|------|------|
| 每日損益一致性 (Daily P&L Consistency) | 獲利日佔比 | > 60% |
| 每日平均交易次數 (Avg Trades/Day) | 來回交易數 | 2-10 |
| 持倉時間 (Holding Period) | 平均持倉時間 | 30分 - 3小時 |
| 滑價成本 (Slippage Cost) | 實際成交 vs 預期成交 | < 0.05% |
| 損益兩平準確率 (Breakeven Accuracy) | 需要的最低勝率 | < 50%（風報比好時） |

### 4.3 滾動驗證 (Walk-Forward Validation)
```
重要原則：絕對不要用隨機分割 (Random Split) 處理時間序列資料！

正確做法 (Correct Approach)：
  1. 訓練 (Train)：1月-6月，測試 (Test)：7月（樣本外 Out-of-Sample）
  2. 訓練：2月-7月，測試：8月
  3. 訓練：3月-8月，測試：9月
  4. ...持續向前滾動 (Roll Forward)

  報告所有期間的平均和最差表現 (Average & Worst Case Performance)。

日內驗證額外要點：
  - 使用淨化交叉驗證 (Purged Cross-Validation)：訓練/測試之間留間隔 (Gap)
  - 考慮隔夜跳空 (Overnight Gaps)
  - 在不同市場狀態下測試 (bull/bear/range)
```

## 5. 模型部署考量 (Model Deployment)

### 5.1 延遲需求 (Latency Requirements)
| 策略類型 | 決策頻率 (Decision Freq) | 最大延遲 (Max Latency) |
|----------|-------------------------|----------------------|
| 剝頭皮 (Scalping) | 每筆/每秒 (Per Tick) | < 100ms |
| 動量 (Momentum, 5min bars) | 每 5 分鐘 | < 1s |
| 波段 (Swing, 15-30min) | 每 15-30 分鐘 | < 5s |
| 均值回歸 (Mean Reversion) | 每 1-5 分鐘 | < 500ms |

### 5.2 模型再訓練 (Model Retraining)
- **每日再訓練 (Daily)：** 快速適應模型 (GBM)
- **每週再訓練 (Weekly)：** 中等變化模型 (LSTM)
- **每月再訓練 (Monthly)：** 穩定模型 (Transformer)
- **部署前一律驗證** (Always Validate on Out-of-Sample Before Deploying)

### 5.3 集成方法 (Ensemble Approach)
```
建議：組合多個模型

訊號層 (Signal Layer)：
  Model 1：LightGBM（表格特徵 Tabular Features）→ 機率 (Probability)
  Model 2：LSTM（序列特徵 Sequential Features）→ 機率
  Model 3：Transformer（長距離 Long-range）→ 機率

整合 (Aggregation)：
  - 加權平均 (Weighted Average)：p = w1×p1 + w2×p2 + w3×p3
  - 或：使用元學習器 (Meta-learner / Stacking)

決策層 (Decision Layer)：
  - 只在集成機率 > 65% 時交易（高確信度 High Conviction）
  - 部位大小隨確信度 (Confidence) 調整
```
