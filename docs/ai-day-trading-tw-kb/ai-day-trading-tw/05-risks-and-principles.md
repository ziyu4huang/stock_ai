# 05 — 風險管理、原則與法規 (Risks, Principles & Regulations)

## 1. AI 當沖的核心原則 (Core Principles)

### 原則一：邊際優勢必須真實，不是過擬合 (The Edge Must Be Real, Not Overfit)
```
過擬合 (Overfitting) 是量化交易系統的頭號殺手。

規則：
  - 一律使用樣本外測試 (Out-of-Sample Test) — 滾動驗證 (Walk-Forward)
  - 如果策略在 2020-2023 有效但 2024 失敗 → 過擬合 (Overfit)
  - 簡單模型 (Simpler Models) 常在實盤 (Live Trading) 表現更好
  - 對日內預測 > 60% 準確率保持懷疑 (Be Suspicious)
  - 上線前至少模擬交易 (Paper Trade) 1 個月

過擬合的紅旗 (Red Flags of Overfitting)：
  - 參數太多 (> 20 個調校參數 Tuned Parameters)
  - 參數微調就導致績效大幅變化 (Performance Sensitive to Small Changes)
  - 只在特定股票或特定時期有效 (Only Works on Specific Stocks/Periods)
  - 產生訊號的規則/條件太多 (Too Many Rules/Conditions)
```

### 原則二：風險管理 > Alpha 生成 (Risk Management > Alpha Generation)
```
部位大小 (Position Sizing) 比訊號準確率 (Signal Accuracy) 更重要。

凱利公式 (Kelly Criterion) — 當沖簡化版：
  f* = (bp - q) / b
  其中 b = 風報比 (Reward/Risk Ratio)，p = 勝率 (Win Rate)，q = 1 - p

實用規則 (Practical Rules)：
  - 每筆交易最大風險 (Max Risk per Trade)：資金的 1-2%
  - 每日最大風險 (Max Daily Risk)：資金的 5%（到達就停止交易）
  - 一定要設停損單 (Always Use Stop-Loss Orders)
  - 部位大小 = 風險金額 / (進場價 - 停損價)

範例 (Example)：

  範例 A — 高價股（需較大資金）：
  資金 (Capital)：NT$1,000,000
  每筆最大風險：1% = NT$10,000
  標的：2330 @ NT$1,810，停損 NT$1,795（風險 NT$15/股）
  部位 (Position)：NT$10,000 / NT$15 = 666 股（零股 Odd Lot）
  所需資金：666 × NT$1,810 = NT$1,205,460 → 超過資金，需縮小部位
  調整後部位：500 股，風險 NT$7,500（0.75%，在限額內）

  範例 B — 中價股（小資本友善）：
  資金 (Capital)：NT$500,000
  每筆最大風險：1% = NT$5,000
  標的：2317 鴻海 @ NT$193，停損 NT$189（風險 NT$4/股）
  部位 (Position)：NT$5,000 / NT$4 = 1,250 股 ≈ 1.25張
  所需資金：1,250 × NT$193 = NT$241,250 → 資金足夠
```

### 原則三：成本吃掉你的 Alpha (Costs Eat Your Alpha)
```
台灣當沖成本（議價費率 Negotiated Commission）：

  來回成本 = 買手續費 + 賣手續費 + 賣出交易稅
            = 0.03% + 0.03% + 0.15%
            = 0.21%

這意味著：
  - 你需要 > 0.21% 的價格變動才能獲利
  - 2330 @ NT$1,810：需要 ~NT$3.8 的移動（< 1 檔 NT$5）
  - 2317 @ NT$193：需要 ~NT$0.41 的移動（< 1 檔 NT$0.5）
  - NT$50 的股票：需要 ~NT$0.11 的移動（約 2 檔）

對策略設計的影響：
  - 不要交易太頻繁 (Don't Trade Too Frequently) — 每筆成本 0.21%
  - 瞄準較大的波動 (Target Larger Moves, 1-3%)
  - 只在高確信度時交易 (Only Trade on High Conviction)
  - 專注流動性高的股票 (Focus on Liquid Stocks, 較低滑價 Lower Slippage)
```

### 原則四：市場狀態會改變 (Market Regimes Change)
```
多頭市場 (Bull Market) 有效的策略，空頭市場 (Bear Market) 可能失效。

台灣市場狀態範例：
  - 多頭趨勢 (Trending Up)：動量策略 (Momentum) 有效
  - 空頭趨勢 (Trending Down)：均值回歸 (Mean Reversion) 可能有效
  - 盤整區間 (Range-bound)：突破策略 (Breakout) 失效，均值回歸有效
  - 高波動 (High Volatility)：降低部位 (Reduce Size)，放寬停損 (Widen Stops)
  - 低波動 (Low Volatility)：可縮小停損 (Tighter Stops)，加大部位

AI 特有風險 (AI-specific Risk)：
  - 在一個狀態 (Regime) 訓練的模型可能在另一個狀態失效
  - 使用狀態偵測 (HMM Regime Detection) 作為後設訊號 (Meta-Signal)
  - 頻繁再訓練 (Retrain Frequently, 至少每週)
  - 監控實盤表現 vs 回測預期 (Monitor Live vs Backtest)
```

### 原則五：從小開始，逐步擴大 (Start Small, Scale Gradually)
```
分階段執行 (Phased Approach)：

  階段一 (Phase 1, 第 1-2 月)：僅模擬交易 (Paper Trading Only)
    - 實作訊號產生器 (Signal Generator)
    - 追蹤模擬損益 (Track Simulated P&L)
    - 優化特徵與模型 (Refine Features & Model)

  階段二 (Phase 2, 第 3-4 月)：微型實盤 (Micro Live Trading)
    - 交易 1 股（零股 Odd Lot）或最小單位
    - 專注執行品質 (Focus on Execution Quality)
    - 驗證實盤成交與回測假設一致 (Validate Live Fills Match Backtest)

  階段三 (Phase 3, 第 5-6 月)：小規模 (Small Scale)
    - 每個部位 1-5 張 (1-5 Lots)
    - 監控滑價與市場衝擊 (Monitor Slippage & Market Impact)
    - 建立對系統的信心 (Build Confidence)

  階段四 (Phase 4, 第 7 月+)：逐步放大 (Scale Up)
    - 逐漸增加部位大小 (Gradually Increase Position Size)
    - 加入更多工具（期貨 Futures 等）
    - 持續監控與改進 (Continuous Monitoring & Improvement)
```

## 2. 風險分類 (Risk Categories)

### 2.1 模型風險 (Model Risk)
| 風險 | 說明 | 緩解方式 (Mitigation) |
|------|------|----------------------|
| 過擬合 (Overfitting) | 模型記住噪音 (Noise) 而非訊號 (Signal) | 滾動驗證 (Walk-Forward)、集成 (Ensemble) |
| 狀態轉變 (Regime Change) | 市場結構改變 (Market Structure Shift) | 頻繁再訓練 (Retrain)、監控漂移 (Monitor Drift) |
| 特徵衰減 (Feature Decay) | 訊號強度隨時間減弱 | 定期特徵重要性審計 (Feature Importance Audit) |
| 資料洩漏 (Data Leakage) | 訓練時使用了未來資訊 (Future Information) | 淨化交叉驗證 (Purged Cross-Validation) |
| 前瞻偏誤 (Look-ahead Bias) | 使用了預測時點不可用的資訊 | 嚴格時間分割 (Strict Temporal Split) |

### 2.2 執行風險 (Execution Risk)
| 風險 | 說明 | 緩解方式 |
|------|------|----------|
| 滑價 (Slippage) | 實際成交價 ≠ 訊號價格 | 使用限價單 (Limit Orders)、回測中預估滑價 |
| 部分成交 (Partial Fill) | 訂單未完全成交 | IOC 訂單、監控成交狀態 (Monitor Fills) |
| 延遲 (Latency) | 訊號到達太晚 | 優化管線 (Optimize Pipeline)、使用限價單 |
| 系統故障 (System Failure) | 軟體/硬體當機 | 心跳監控 (Heartbeat)、斷線自動撤單 (Auto-cancel) |
| 券商 API 當機 (Broker API Down) | API 無法使用 | 準備備用券商 (Backup Broker)、手動覆蓋 (Manual Override) |

### 2.3 市場風險 (Market Risk)
| 風險 | 說明 | 緩解方式 |
|------|------|----------|
| 過夜跳空 (Overnight Gap) | 部位未平倉時的跳空風險 | 13:00 前強制平倉 (Force Close Before 13:00) |
| 流動性枯竭 (Liquidity Dry-up) | 無法出場 | 只交易日成交量 > NT$10 億的股票 |
| 閃崩 (Flash Crash) | 突然的大幅波動 | 漲跌幅限制意識 (Circuit Breaker Awareness)、停損 |
| 新聞事件 (News Event) | 意外公告導致價格劇烈變動 | 新聞監控 (News Monitoring)、事件前降低部位 |
| 暫停交易 (Trading Halt) | 股票被暫停交易 | 監控台交所公告 (Monitor TWSE Announcements) |

### 2.4 營運風險 (Operational Risk)
| 風險 | 說明 | 緩解方式 |
|------|------|----------|
| 下單邏輯 Bug | 錯誤的數量/價格/方向 | 先模擬交易 (Paper Trade First)、單元測試 (Unit Tests)、緊急停止 (Kill Switch) |
| 停損設定錯誤 | 停損未觸發 | 下單後確認委託狀態 (Verify Order Status) |
| 重複下單 (Duplicate Orders) | 同一訊號觸發兩次 | 冪等性檢查 (Idempotency Check)、訂單追蹤 |
| 時鐘不同步 (Clock Sync) | 系統時間偏移 | NTP 同步、使用券商時間戳記 (Broker Timestamp) |
| 帳戶額度超限 (Account Limit) | 超過購買力 (Buying Power) | 交易前風控檢查 (Pre-trade Risk Check) |

## 3. 台灣演算法交易法規 (Regulations)

### 3.1 散戶可以使用演算法交易嗎？
**可以！** 台灣的散戶 (Retail Investor) 可以合法使用演算法交易 (Algorithmic Trading)。個人自有帳戶的程式交易不需要特殊許可。

**關鍵區別：** 如果你用自己的帳戶透過券商 API 交易 → 不需要牌照 (No License Needed)。如果你提供演算法交易服務給他人或管理他人的資金 → 需要投資顧問牌照 (證券投資顧問事業 License)。

### 3.2 重要法規
| 法規 | 說明 |
|------|------|
| **證券交易法** | 證券交易法 — 一般交易規則 |
| **證券商受託買賣有價證券** | 券商受託買賣規則 |
| **當日沖銷交易** | 當沖交易規則 (TWSE/TPEx) |
| **漲跌幅限制** | 價格限制 ±10% (一般)、±5% (部分上櫃) |
| **異常交易監視** | 台交所監視異常交易模式 |
| **處置措施** | 台交所可限制異常股票交易 |

### 3.3 絕對要避免的行為 (Things to Avoid)
1. **晃價 (Spoofing)：** 下單後打算取消以操縱價格 → **違法**
2. **分層下單 (Layering)：** 在不同價位下多筆訂單製造假象 → **違法**
3. **跑單 (Front-running)：** 在自己的客戶訂單前先交易（如果你是券商）
4. **洗售 (Wash Trading)：** 與自己交易製造成交量 → **違法**
5. **過度撤單 (Excessive Order Cancellation)：** 可能觸發券商審查 (> 50% 撤單率)

### 3.4 稅務 (Tax Implications)
| 項目 | 處理方式 |
|------|----------|
| 當沖獲利 (Day Trading Profit) | **免稅**（證券交易所得停徵 No Income Tax） |
| 當沖虧損 (Day Trading Loss) | 不可扣抵 (Not Deductible) |
| 證券交易稅 (Securities Transaction Tax) | 0.15%（當沖）、0.3%（一般）— 已由券商代扣 |
| 期貨交易獲利 (Futures Trading Profit) | **免稅**（期貨交易所得停徵） |
| 期貨交易稅 (Futures Transaction Tax) | 0.002%（TX）、視商品而定 |
| ⚠️ 稅法可能變動，請諮詢稅務專業人員。 |

## 4. 常見當沖陷阱 (Common Pitfalls)

### 4.1 心理陷阱 (Psychological Traps)
```
1. 報復性交易 (Revenge Trading)：虧損後加大部位想「贏回來」
   → 嚴格每日虧損上限 (Strict Daily Loss Limit, 5%)

2. 死抱虧損 (Holding Losers)：拒絕停損，期待反轉 (Hoping for Reversal)
   → 硬性停損 (Hard Stop-Loss)、13:00 強制平倉

3. 過度交易 (Overtrading)：交易太頻繁，手續費吃掉利潤
   → 每日最多 5-10 筆，重質不重量 (Quality over Quantity)

4. 錯失恐懼 (FOMO)：追漲熱門股但沒有訊號
   → 堅持系統規則 (Stick to System Rules)

5. 確認偏誤 (Confirmation Bias)：只看支持自己部位的證據
   → 使用系統化規則 (Systematic Rules)，不用自由裁量 (Discretion)
```

### 4.2 台灣市場特有陷阱
```
1. 平盤下融券限制：不能在股價低於前日收盤價時先賣後買
   → 某些天只能做單方向 (Single Direction Only)

2. 漲停/跌停 (Price Limit Hit)：價格鎖定時無法出場
   → 避免接近漲跌幅限制的股票 (Avoid Stocks Near Price Limits)

3. 處置股 (Sanctioned Stocks)：台交所限制異常股票交易
   → 每日查看「注意交易資訊」和「處置資訊」

4. 當沖沖不掉 (Failed Close)：13:30 前無法平倉
   → 13:00 強制平倉 (Force Close)，絕不留到 13:30

5. 借券成本 (Borrowing Cost)：如果無法回補空單，借券費可能很高
   → 只在流動性極高的股票做先賣後買

6. 開獎行情 (Ex-dividend/Ex-rights)：除權息日股價跳空
   → 查看除權息行程表 (Check Ex-dividend Schedule)
```

## 5. AI 當沖每日檢查清單 (Daily Checklist)

### 盤前 (Pre-Market, 08:00-09:00)
- [ ] 確認系統正常運作 (System Health Check)
- [ ] 驗證券商 API 連線 (Verify Broker API Connection)
- [ ] 下載更新的模型 (Download Updated Model, if retrained overnight)
- [ ] 檢查隔夜新聞/情緒 (Check Overnight News/Sentiment)
- [ ] 確認今日可當沖標的 (Identify Eligible Day Trading Stocks)
- [ ] 檢查處置股 (Check 處置股) — 避開
- [ ] 檢查除權息行程 (Check 除權息 Schedule) — 注意跳空
- [ ] 設定每日風險限額 (Set Daily Risk Limits)
- [ ] 啟動即時數據連線 (Warm up Real-time Data Connection)

### 盤中 (Intraday, 09:00-13:30)
- [ ] 監控系統健康 (Monitor System Health)
- [ ] 驗證訂單執行 (Verify Order Execution)
- [ ] 即時追蹤損益 (Track P&L in Real-time)
- [ ] 監控部位大小 vs 風險限額 (Monitor Position Size vs Risk Limits)
- [ ] 檢查影響持倉的新聞 (Check News Affecting Open Positions)
- [ ] 13:00 前強制平倉所有部位 (Force Close All Positions by 13:00)

### 盤後 (Post-Market, 13:30-15:00)
- [ ] 確認所有部位已平倉 (Verify All Positions Closed — 避免違約交割！)
- [ ] 計算實際損益並與預期比較 (Calculate Actual vs Expected P&L)
- [ ] 記錄所有交易供分析 (Log All Trades for Analysis)
- [ ] 檢查模型表現 vs 實際結果 (Check Model Performance vs Actual)
- [ ] 如需要則再訓練模型 (Retrain Model if Needed)
- [ ] 若市場狀態改變則更新風險參數 (Update Risk Parameters if Regime Changed)
