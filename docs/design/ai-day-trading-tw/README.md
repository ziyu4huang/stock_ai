# AI 當沖知識庫 — 台灣市場

以量化 (Quantitative) + AI 技術進行日內交易 (Intraday Trading / 當沖) 的知識庫。

## 概述

**目標：** 在台股市場使用 AI + 量化策略進行當日沖銷 (Day Trading / 當沖)，不留過夜部位 (No Overnight Position)，僅支付價差 (Spread) + 手續費，不需支付全额股價。

**核心概念：** 台股允許「先買後賣」或「先賣後買」的當沖交易，券商會在收盤時自動沖銷 (Netting)，你只需要付出買賣價差 + 手續費 + 交易稅。

## 文件清單

| # | 檔案 | 主題 |
|---|------|-------|
| 01 | [01-tw-day-trading-mechanics.md](01-tw-day-trading-mechanics.md) | 台股當沖機制、費用、結算 |
| 02 | [02-trading-instruments-comparison.md](02-trading-instruments-comparison.md) | 股票/期貨/選擇權/權證 比較 |
| 03 | [03-ai-quant-techniques.md](03-ai-quant-techniques.md) | ML 模型、特徵工程、交易策略 |
| 04 | [04-data-and-infrastructure.md](04-data-and-infrastructure.md) | 資料來源、券商 API、回測工具 |
| 05 | [05-risks-and-principles.md](05-risks-and-principles.md) | 風險管理、法規、常見陷阱 |
| 06 | [06-system-architecture.md](06-system-architecture.md) | 系統架構設計、專案結構、實作階段 |
| 07 | [07-tooling-guide-shioaji-lgmb-vectorbt.md](07-tooling-guide-shioaji-lgmb-vectorbt.md) | **Shioaji + LightGBM + vectorbt 實作指南** |
| 08 | [08-large-trade-tracker.md](08-large-trade-tracker.md) | **大單追蹤偵測器：HMM 整合與點火偵測** |

## 快速參考

### 台股交易時段 (Trading Hours)
- **開盤前撮合 (Pre-market Auction)：** 08:30 – 09:00
- **盤中逐筆交易 (Continuous Trading)：** 09:00 – 13:30
- **收盤集合競價 (Closing Auction)：** 13:25 – 13:30
- **盤後定價 (After-hours Fixed Price)：** 14:00 – 14:30

### 成本結構 (Cost Structure) — 當沖股票

| 費用項目 | 費率 | 說明 |
|----------|------|------|
| 券商手續費 (Broker Commission) | 0.1425%（可議價至 0.02-0.06%） | 買進 + 賣出各收一次 |
| 證券交易稅 (Securities Transaction Tax) | **0.15%**（當沖優惠） | 僅賣出時收取，正常為 0.3% |
| 來回總成本 (Round-trip Cost) | 約 0.3-0.5% | 買手續費 + 賣手續費 + 交易稅 |

### 各工具資金需求 (Capital Requirements)

| 工具 (Instrument) | 最低資金 | 槓桿 (Leverage) |
|-------------------|----------|-----------------|
| 股票當沖 (Stock Day Trading) | 帳戶餘額 ~NT$10萬+ | 無（T+0 沖銷） |
| 台指期 TX (Index Futures) | 保證金 ~NT$28.8萬 | 約 22x |
| 小型台指期 MTX (Mini Futures) | 保證金 ~NT$7.2萬 | 約 22x |
| 台指選擇權 TXO (Index Options) | 權利金 NT$1-5萬/口 | 高（非線性） |

### 建議閱讀順序
1. 從 `01-tw-day-trading-mechanics.md` 了解市場機制
2. 讀 `02-trading-instruments-comparison.md` 選擇交易工具
3. 研讀 `03-ai-quant-techniques.md` 學習策略設計
4. 搭配 `07-tooling-guide-shioaji-lgmb-vectorbt.md` 實際動手
5. 查看 `04-data-and-infrastructure.md` 了解工具鏈
6. 內化 `05-risks-and-principles.md` 的風險管理原則
7. 用 `06-system-architecture.md` 規劃系統架構
