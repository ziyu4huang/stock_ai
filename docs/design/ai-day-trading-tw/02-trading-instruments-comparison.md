# 02 — 交易工具比較 (Trading Instruments Comparison)

## 適用當沖的工具總覽

台股當沖有多種工具可選，各有不同的資金需求 (Capital Requirement)、流動性 (Liquidity) 和適合的量化策略類型。

---

## 1. 股票當沖 (Stock Day Trading / 現股當沖)

### 機制
- 同一天內買進並賣出（或先賣後買）同一檔股票
- 不留過夜部位 (No Overnight Position)
- 只支付價差 (Spread) + 手續費 (Commission) + 交易稅 (Tax)

### 優點
- **無槓桿風險 (No Leverage Risk)** — 不會虧超過帳戶資金
- **交易稅減半優惠 (Tax Reduction)：** 0.15%（正常 0.3%）
- **機制簡單** — 方向性賭注 (Directional Bet)
- **可選標的多** — ~1000+ 檔上市櫃股票可當沖
- **無追繳保證金風險 (No Margin Call)**

### 缺點
- **檔位摩擦 (Tick Friction)** — NT$1000 以上 1 檔 = NT$5（2330 @ NT$1,810 的 1 檔 = 0.28%），詳細說明見 [01-tw-day-trading-mechanics.md §6.5](01-tw-day-trading-mechanics.md)
- **平盤下不得融券賣出** — 限制了「先賣後買」的方向
- **流動性差異大** — 中小型股 (Mid/Small Cap) 價差 (Spread) 大
- **交易時間有限** — 09:00-13:30（4.5 小時）

### 資金需求
- **最低：** ~NT$5-10萬（零股或低價股）
- **建議：** NT$50-100萬（才有意義的部位大小）
- **1張 2330 @ NT$1,810（2026/4/2）：** 成交值 NT$181萬，但只需帳戶有足夠餘額

### 最適合
- 初學者、保守型策略
- 大型股動量/突破策略 (Momentum / Breakout Strategy)
- AI 模型預測日內方向 (Intraday Direction Prediction)

---

## 2. 台指期貨 TX (Taiwan Index Futures / 台指期)

### 合約規格 (Contract Specs)
- **標的 (Underlying)：** 加權股價指數 (TAIEX / 發行量加權股價指數)
- **契約乘數 (Multiplier)：** 每點 NT$200
- **契約月份 (Contract Months)：** 近月 + 次月 + 2個季月
- **交易時段 (Trading Hours)：**
  - 日盤 (Regular)：08:45 – 13:45
  - 夜盤 (After-hours)：15:00 – 隔日 05:00
- **漲跌幅限制 (Price Limit)：** ±10%（動態計算）
- **最小跳動點 (Tick Size)：** 1 點 = NT$200

### 保證金 (Margin) — 2025/2026
- **原始保證金 (Initial Margin)：** ~NT$288,000
- **維持保證金 (Maintenance Margin)：** ~NT$221,000
- **槓桿倍數 (Leverage)：** 指數 ~32,500 點 × NT$200 / NT$288K ≈ **22x**

### 小型台指期 (Mini TX / MTX)
- **乘數 (Multiplier)：** 每點 NT$50（TX 的 1/4）
- **保證金 (Margin)：** ~NT$72,000
- 相同交易時段和契約月份
- **小資本首選**

### 優點
- **高流動性 (High Liquidity)** — 亞洲最活躍的指數期貨之一
- **可作多也可作空 (Long & Short)** — 無平盤限制
- **夜盤交易 (Extended Hours)** — 直到隔日 05:00
- **高槓桿 (~22x)** — 小波動 = 大盈虧
- **單一商品 (Single Instrument)** — 專注一個訊號，不需選股 (Stock Selection)
- **低檔位摩擦 (Low Tick Friction)** — 1 點 / 32,500 = 0.003%

### 缺點
- **追繳風險 (Margin Call Risk)** — 可能虧損超過原始保證金
- **每日 mark-to-market** — 必須維持保證金水位
- **期貨特有成本：** 買賣價差 (Bid-Ask Spread) ~1-2 點 = NT$200-400
- **轉倉成本 (Rollover Cost)** — 近到期日須換月 (Roll to Next Month)
- **心理壓力大**

### 費用
- **交易稅 (Transaction Tax)：** 0.002%（百萬分之廿）
- **經手費/期交費：** NT$10-20/口
- **券商手續費 (Broker Fee)：** 可議價，通常 NT$100-300/來回
- **來回總成本：** ~NT$400-600（約合約值 0.01%）

### 最適合
- 指數方向性策略 (Index Directional Strategy)
- 均值回歸策略 (Mean Reversion)
- 高頻/剝頭皮策略 (Scalping)
- AI 模型預測大盤方向

---

## 3. 台指選擇權 TXO (Taiwan Index Options / 台指選擇權)

### 合約規格
- **標的 (Underlying)：** 加權股價指數 (TAIEX)
- **契約規格 (Contract Size)：** 指數點 × NT$50
- **履約價間距 (Strike Intervals)：** 近月 50 點，遠月 100 點
- **到期日 (Expiration)：** 月選擇權 + 週選擇權
- **交易時段：** 與 TX 期貨相同

### 當沖方式
- 買方 (Buyer)：支付權利金 (Premium)，不需保證金
- 賣方 (Seller)：需要保證金
- 日內買賣權利金賺價差

### 優點
- **買方風險有限 (Defined Risk)：** 最大虧損 = 權利金 (Premium Paid)
- **高槓桿 (High Leverage)：** 小額權利金控制大額名目 (Notional)
- **非線性報酬 (Non-linear Payoff)：** 凸性報酬 (Convex Returns)
- **可交易波動率 (Volatility Trading)：** 買賣 Straddle / Strangle

### 缺點
- **時間耗損 (Time Decay / Theta Decay)：** 每天都在虧損價值
- **流動性差 (Poor Liquidity)：** 只有近價 (Near ATM) 的履約價有流動性
- **買賣價差大 (Wide Bid-Ask Spread)：** 深價外 (Deep OTM) 特別嚴重
- **定價複雜 (Complex Pricing)：** 需要了解 Greeks (Delta, Gamma, Theta, Vega)
- **Gamma 風險** — 臨近到期日波動劇烈
- **較不適合簡單方向性 AI 模型**

### 費用
- **交易稅 (Tax)：** 權利金 0.1%（買方）或結算價 0.1%（賣方）
- **手續費：** NT$20-50/口/邊

### 最適合
- 波動率策略 (Volatility Strategy) — 預測波動而非方向
- 避險 (Hedging) 期貨部位
- 限定風險投機 (Defined-risk Speculation)
- 進階量化策略

---

## 4. 認購售權證 (Warrants / 權證)

### 機制
- 由券商 (Issuer) 發行的衍生性商品
- 認購權證 (Call Warrant)：標的上漲時獲利
- 認售權證 (Put Warrant)：標的下跌時獲利
- 存續期間 (Tenor)：通常 6 個月 – 2 年

### 優點
- **資金需求極低 (Very Low Capital)：** 每單位 NT$0.5-5
- **無平盤限制** — 認售權證也可放空
- **內建槓桿 (Built-in Leverage)：** 2-10x

### 缺點
- **發行商定價不透明 (Opaque Pricing)** — 券商控制 Spread
- **買賣價差大 (Wide Spread)：** 0.05-0.5 NT$ = 很大的百分比
- **時間耗損 (Time Decay)** — 類似選擇權
- **流動性風險 (Liquidity Risk)** — 部分權證幾乎無交易量
- **不適合系統化/量化策略 (Not Ideal for Systematic Trading)**

### 最適合
- 極小資本交易者
- 特定股票方向性賭注
- **不建議用於 AI/量化交易**（定價不透明）

---

## 5. 綜合比較矩陣 (Comparison Matrix)

| 特性 (Feature) | 股票當沖 (Stock) | 台指期 TX (Futures) | 小台指 MTX (Mini) | 台指選 TXO (Options) | 權證 (Warrants) |
|---|---|---|---|---|---|
| **最低資金** | ~NT$10萬 | ~NT$30萬 | ~NT$8萬 | ~NT$5千(買) | ~NT$5千 |
| **槓桿 (Leverage)** | 無 | ~22x | ~22x | 高 | 2-10x |
| **可放空 (Shortable)** | 受限(平盤上) | 隨時 | 隨時 | 隨時(Put) | 隨時(Put) |
| **稅率 (Tax Rate)** | 0.15% | 0.002% | 0.002% | 0.1%權利金 | 0.15% |
| **流動性 (Liquidity)** | 因股而異 | 極高 | 高 | 中等 | 低-中 |
| **複雜度 (Complexity)** | 低 | 中 | 中 | 高 | 中 |
| **AI 適合度** | ★★★★ | ★★★★★ | ★★★★★ | ★★★ | ★★ |
| **檔位摩擦** | 中 | 極低 | 極低 | 中 | 高 |
| **過夜風險** | 無 | 無(日內) | 無(日內) | 無(日內) | 無(日內) |
| **交易時間** | 9:00-13:30 | 8:45-13:45+夜盤 | 同TX | 同TX | 9:00-13:30 |

## 6. AI 當沖工具建議 (Recommendations)

### 初學者 (Phase 1)：
**從股票當沖開始** — 選擇流動性高的大型股（2330 台積電、2317 鴻海、2454 聯發科 等）
- 風險最低，最容易理解
- 建立資料管線 (Data Pipeline) 和模型基礎設施
- 學習市場微結構 (Market Microstructure)

### 進階者 (Phase 2)：
**加入台指期/小台指 (TX/MTX)** — 指數方向性策略
- 更高槓桿 (Higher Leverage)、更低成本 (Lower Cost)
- 單一標的簡化 AI 模型
- 夜盤提供更多數據點 (Extended Hours = More Data)

### 高階者 (Phase 3)：
**考慮選擇權策略 (Options)** — 波動率交易
- 最複雜但也最多機會
- 需要 Greeks 和波動率曲面 (Vol Surface) 的知識
- 可結合股票當沖做 Delta-Neutral 策略
