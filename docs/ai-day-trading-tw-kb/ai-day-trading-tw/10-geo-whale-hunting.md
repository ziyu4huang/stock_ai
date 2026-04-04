# 10 — 地緣大戶狙擊系統：從籌碼地理到即時偵查 (Geographic Whale Hunting)

> **定位：** 自動化交易系統的「指紋辨識庫」。
> 第八章的大單追蹤器是「雷達」，本章則是「身份資料庫」。
> 當雷達掃到異常脈衝時，我們用 **地緣關係 (Geographic Proximity)** 來判斷：
> 這是大戶的「計畫性行動」，還是散戶的「隨機噪音」？
> 結合 **SurrealDB 圖形資料庫** + **Rust DSP 引擎** + **Shioaji 盤中資料**，
> 建立一套完整的數位偵查工作流。

---

## 1. 為什麼「地緣」是台股的獨特優勢

### 1.1 台灣股市的地理特性

台灣股市有一個全球獨特的現象：**公司總部附近的券商分點，往往是「最內行」的資金聚集地**。

原因：
- 台灣面積僅 36,000 平方公里，高科技業集中在 **新竹科學園區**（竹科）
- 公司高管、上下游供應商、產業分析師，往往在同一個生活圈
- 內線交易雖然違法，但「接近資訊源」的人確實具有 **資訊不對稱 (Information Asymmetry)** 優勢
- 歷史數據反覆驗證：**地緣分點的買賣行為，對股價有顯著的預測力**

### 1.2 經典案例（公開資訊可查）

| 案例 | 地緣分點 | 行為特徵 | 後續走勢 |
|------|----------|----------|----------|
| 聯發科 (2454) | 竹科周邊分點 | 連續 3 日緩慢買超 | 一波段漲幅 15-20% |
| 台積電 (2330) | 新竹/台中特定分點 | 單日大買超 + 隔日沖比例低 | 中線看多訊號 |
| 鴻海 (2317) | 土城周邊分點 | 突然放量買進 | 通常伴隨利多消息 |

> **關鍵概念：** 「地緣」不是內線，而是 **資訊在地化的自然結果**。
> 就像你住在新竹，對竹科公司的動態自然比台北的投資人更敏銳。

### 1.3 反面教材：隔日沖大戶

並非所有大單都值得跟隨。台股市場有一批「隔日沖 (Day-Trade Flip)」大戶：
- **特徵：** 今日大量買進，明日開盤即賣出
- **常見分點：** 凱基-台北、富邦-建國、元大-忠明 等
- **危害：** 跟單的散戶往往買在最高點，次日被倒貨

> 這正是為什麼需要「地緣分析」：**區分「產業大戶」與「隔日沖炒手」**。

---

## 2. 第一階段：建立「地緣籌碼」知識庫 (SurrealDB)

### 2.1 為什麼選 SurrealDB

| 特性 | SurrealDB | 傳統關聯式 DB |
|------|-----------|--------------|
| 圖形查詢 | 內建 (Record Links) | 需要額外join |
| 嵌入式模式 | 支援 (RocksDB 後端) | 需要獨立進程 |
| Schema-less | 彈性欄位 | 固定 Schema |
| 查詢語言 | SurrealQL (類SQL) | SQL |

對於「公司—分點」的地理關聯查詢，圖形資料庫是天然選擇。

### 2.2 資料模型設計

```
┌─────────────┐     NEARBY      ┌─────────────┐
│  Company    │ ◄──────────────► │  Broker     │
│  (公司)     │   (距離 < 5km)   │  (券商分點)  │
├─────────────┤                  ├─────────────┤
│ code: 2330  │                  │ code: 1020  │
│ name: 台積電│                  │ name: 凱基-新竹│
│ hq_address  │                  │ address     │
│ hq_lat/lng  │                  │ lat/lng     │
│ sector      │                  │ tags[]      │
│ hq_city     │                  │ whale_type  │
└─────────────┘                  └─────────────┘
       │
       │ INDUSTRY (同產業)
       ▼
┌─────────────┐
│  Company    │
│  (供應鏈)   │
└─────────────┘
```

### 2.3 SurrealQL Schema 定義

```sql
-- 公司節點
DEFINE TABLE company SCHEMAFULL;
DEFINE FIELD code ON company TYPE string;        -- 股票代碼 e.g. "2330"
DEFINE FIELD name ON company TYPE string;         -- 公司名稱
DEFINE FIELD sector ON company TYPE string;       -- 產業別 e.g. "半導體"
DEFINE FIELD hq_address ON company TYPE string;   -- 總部地址
DEFINE FIELD hq_lat ON company TYPE float;        -- 緯度
DEFINE FIELD hq_lng ON company TYPE float;        -- 經度
DEFINE FIELD hq_city ON company TYPE string;      -- 城市 e.g. "新竹市"

-- 券商分點節點
DEFINE TABLE broker SCHEMAFULL;
DEFINE FIELD code ON broker TYPE string;          -- 分點代碼 e.g. "1020"
DEFINE FIELD name ON broker TYPE string;          -- 分點名稱
DEFINE FIELD address ON broker TYPE string;
DEFINE FIELD lat ON broker TYPE float;
DEFINE FIELD lng ON broker TYPE float;
DEFINE FIELD tags ON broker TYPE set;             -- 標籤
DEFINE FIELD whale_type ON broker TYPE string;    -- "geo" | "dayflip" | "swing" | "unknown"

-- 地緣關係 (距離 < 5km)
DEFINE TABLE nearby SCHEMAFULL;
DEFINE FIELD in ON nearby TYPE record<company>;
DEFINE FIELD out ON nearby TYPE record<broker>;
DEFINE FIELD distance_km ON nearby TYPE float;
```

### 2.4 關鍵分點分類系統

| 類型 | 定義 | 識別方式 | 交易意義 |
|------|------|----------|----------|
| **地緣大戶 (Geo Whale)** | 公司總部 5km 內的分點 | 地理計算 + 長期買超紀錄 | **高勝率跟單目標** |
| **隔日沖炒手 (Day Flipper)** | 已知高頻隔日沖的分點 | 歷史統計：T+1 賣出率 > 70% | **避開或反向操作** |
| **波段大戶 (Swing Whale)** | 在特定股票勝率極高的分點 | 歷史回測：30 日勝率 > 65% | **中線佈局參考** |
| **外資分點 (Foreign)** | 外資專用分點 | 證交所公開資訊 | **趨勢確認指標** |

#### 台灣知名隔日沖分點（公開資訊）

| 分點名稱 | 代碼 | 特徵 |
|----------|------|------|
| 凱基-台北 | — | 市場最大隔日沖分點之一 |
| 富邦-建國 | — | 高頻當沖 |
| 元大-忠明 | — | 活躍度高 |
| 美林 (Merrill Lynch) | — | 外資大單 |
| 摩根大通 (JPMorgan) | — | 外資大單 |

> **注意：** 分點行為會隨時間改變。建議每月更新分點「角色」統計。

### 2.5 資料來源與建庫流程

```
證交所公開資訊                  Google Maps API
    │                               │
    ▼                               ▼
┌──────────┐    ┌──────────┐    ┌──────────┐
│ 公司基本  │    │ 券商分點  │    │ 地址轉   │
│ 資料      │    │ 名冊     │    │ 經緯度   │
└────┬─────┘    └────┬─────┘    └────┬─────┘
     │               │               │
     └───────────────┴───────────────┘
                     │
                     ▼
            ┌─────────────────┐
            │  Haversine 距離  │
            │  計算 (Rust)     │
            │  產生 NEARBY edge │
            └────────┬────────┘
                     │
                     ▼
              ┌──────────────┐
              │  SurrealDB   │
              │  地緣知識庫    │
              └──────────────┘
```

**Haversine 公式（計算兩點球面距離）：**

$$d = 2r \cdot \arcsin\left(\sqrt{\sin^2\left(\frac{\Delta\varphi}{2}\right) + \cos(\varphi_1)\cos(\varphi_2)\sin^2\left(\frac{\Delta\lambda}{2}\right)}\right)$$

其中 $r$ = 6371 km（地球半徑），$\varphi$ 為緯度，$\lambda$ 為經度。

---

## 3. 第二階段：Rust 高效能分析引擎 (DSP Core)

### 3.1 為什麼用 Rust

台股盤中有 ~1,700 檔股票同時交易，每秒產生數千筆 Tick。要在 **單機** 上即時監控所有股票的成交量異常，需要：
- **零成本抽象 (Zero-cost Abstraction)：** Rust 編譯後效能接近 C
- **無 GC 停頓：** 不像 Java/Go 有垃圾回收延遲
- **SIMD 加速：** Apple M 系列晶片的 AMX 單元可被 `Accelerate` 框架直接利用
- **記憶體安全：** 不用擔心 Buffer Overflow

### 3.2 Welford 演算法：即時統計量計算

傳統做法需要保存所有歷史數據來計算平均與標準差。**Welford 演算法** 可以在 O(1) 記憶體下即時更新：

```
初始化: count=0, mean=0, M2=0

每收到新值 x_n:
  count += 1
  delta = x_n - mean
  mean += delta / count
  delta2 = x_n - mean
  M2 += delta * delta2

方差 = M2 / count
標準差 σ = √(M2 / count)
```

**優勢：**
- 每檔股票只需要 3 個變數 (count, mean, M2) = 24 bytes
- 1,700 檔股票 × 24 bytes = **40.8 KB** — 完全可放入 L1 Cache
- 數值穩定性優於 two-pass 演算法

### 3.3 滑動視窗 3-Sigma 監控器

**核心思想：** 散戶的成交量是「常態分佈的噪音」，大戶的進場是「脈衝 (Spike)」。
用 3-Sigma 閾值過濾掉 99.7% 的散戶行為，只留下大戶的痕跡。

$$\text{Spike Alert} = \text{Volume}_i > \mu_{200} + 3\sigma_{200}$$

| 參數 | 值 | 說明 |
|------|-----|------|
| 視窗大小 | 200 筆成交 | 約 5-15 分鐘的即時歷史 |
| 閾值 | $\mu + 3\sigma$ | 過濾 99.7% 常態噪音 |
| 更新頻率 | 每筆 Tick | 即時更新 Welford 統計量 |

### 3.4 Apple Accelerate (vDSP) 硬體加速

在 M5 Max 上，Rust 可以透過 `accelerate` crate 呼叫 Apple 的 vDSP 函式庫：

```rust
// 概念示意：使用 Accelerate 進行批次運算
use accelerate::vdsp;

// 同時計算 1700 檔股票的滑動平均
// vDSP_vsadd 可以一次對整個陣列做向量加法
// 在 M5 Max 上，這會被編譯為 AMX/SIMD 指令
vdsp::vadd(&tick_volumes, &baseline, &mut result, 1700);
```

**硬體優勢（M5 Max）：**
- 16 核 CPU（12 performance + 4 efficiency）
- 128 GB 統一記憶體 — 可將全日 Tick 載入 RAM
- AMX 矩陣運算單元 — 向量運算加速
- 即使同時監控 1,700 檔，CPU 負擔 < 5%

### 3.5 多執行緒架構

```
Tick Stream (Shioaji WebSocket)
         │
         ▼
┌─────────────────────┐
│  Dispatcher (tokio)  │  ← 非同步接收，零拷貝分發
└──┬──┬──┬──┬──┬──┬───┘
   │  │  │  │  │  │
   ▼  ▼  ▼  ▼  ▼  ▼
  [Worker 0..15]        ← 16 個 rayon worker threads
   │  每個 worker 負責   │
   │  ~106 檔股票        │
   │  Welford + 3σ      │
   ▼
┌─────────────────────┐
│  Spike Channel       │  ← 只有觸發 3σ 的 event 才送出
│  (crossbeam mpsc)    │
└────────┬────────────┘
         │
         ▼
   地緣分析 (下一節)
```

---

## 4. 第三階段：地緣狙擊邏輯 (The Aiming Logic)

當 Rust 引擎偵測到 **3-Sigma Spike** 時，執行「聯合法醫鑑定」—— 三重過濾確認。

### 4.1 第一重：分點身份識別

盤中即時確認分點的方法：

| 方法 | 精確度 | 延遲 | 說明 |
|------|--------|------|------|
| **盤後明細 (最佳)** | 100% | T+1 | 證交所公開的「券商買賣超」 |
| **五檔推估 (盤中)** | ~70% | 即時 | 觀察異常大的委託量變化 |
| **歷史行為比對** | ~60% | 即時 | 比對該分點過去的交易模式 |

**盤中五檔推估邏輯：**

```
當某檔股票出現 3-Sigma Spike 時：
  1. 檢查 Ask 側 (賣方) 的掛單量變化
     - 如果 Ask 1 的掛單突然消失 → 可能是「外盤成交」(主動買)
     - 如果 Ask 1 掛單量暴增 → 可能是有人在「壓盤」
  2. 檢查 Bid 側 (買方) 的掛單量變化
     - Bid 1 出現大單 → 「支撐」信號
     - Bid 1 被吃掉 → 可能是「急殺」
  3. 結合 Spike 方向，推估可能的分點類型
```

### 4.2 第二重：能量脈衝驗證 (DSP Pulse Analysis)

單筆大單可能是散戶手滑、程式交易觸發、或隔日沖的假動作。真正的「大戶進場」會呈現特定的脈衝模式：

```
散戶手滑 (Single Pulse)：
Volume ▁▁▁▁▁▁█▁▁▁▁▁▁  ← 單發脈衝，後繼無力

大戶急拉 (Rising Edge Staircase)：
Volume ▁▁▁▁▂▃▅▇█▇▅▃  ← 階梯狀上升，能量遞增

大戶吸籌 (Accumulation Envelope)：
Volume ▁▁▂▃▃▃▃▂▁▁▁▁  ← 平緩的包絡線，持續數分鐘

出貨倒貨 (Distribution)：
Volume ▃▃▅▇████▇▅▃  ← 高檔放量但價格不漲
```

**能量包絡線 (Energy Envelope) 計算：**

$$E(t) = \sqrt{\frac{1}{N}\sum_{i=t-N+1}^{t} v_i^2}$$

其中 $v_i$ 是第 $i$ 筆成交量，$N$ 是視窗大小。這是 DSP 中的 RMS (Root Mean Square) 能量計算。

**判定規則：**

| 脈衝模式 | $E(t)$ 特徵 | 判定 | 動作 |
|----------|------------|------|------|
| 單發脈衝 | $E$ 瞬間暴增後驟降 | Noise | 忽略 |
| 階梯上升 | $E$ 持續遞增 3+ 步 | Ignition | **準備進場** |
| 平緩包絡 | $E$ 穩定偏高 | Accumulation | 監控 |
| 高檔放量 | $E$ 高但 $\Delta P \approx 0$ | Distribution | **避開** |

### 4.3 第三重：地緣加權 (Geographic Weighting)

結合 SurrealDB 的地緣知識庫，對 Spike 進行加權評分：

```sql
-- SurrealQL：查詢某公司的地緣分點
SELECT
    broker.name,
    broker.whale_type,
    nearby.distance_km
FROM nearby
WHERE in = company:2330
AND distance_km < 5.0
ORDER BY distance_km;
```

**Aiming Level 計算：**

```
Base Score = 3-Sigma Spike 觸發 → 60 分

加權因素：
  + 地緣分點活躍 (geo_whale)     → +20 分
  + 產業群聚效應 (同產業 >3 檔觸發) → +10 分
  + 五檔偏買 (Order Book Imbalance > 0.6) → +10 分

扣分因素：
  - 隔日沖分點 (dayflip)         → -30 分
  - 大盤弱勢 (TAIEX < 5min MA)   → -15 分
  - 接近漲停板但未鎖死            → -10 分
```

| Aiming Level | 分數 | 動作 |
|:---|:---|:---|
| **CRITICAL** | 90-100 | 立即關注，準備進場 |
| **HIGH** | 70-89 | 加入觀察名單 |
| **MEDIUM** | 50-69 | 記錄，等待更多確認 |
| **LOW** | < 50 | 忽略，可能是噪音 |

### 4.4 產業群聚效應 (Sector Cluster)

台股的產業集中度極高。當竹科某大廠有利多時，往往整條供應鏈同時受惠：

```
台積電 (2330) 異常放量
    │
    ├── 聯發科 (2454)   ← 同為竹科，連動性高
    ├── 環球晶 (6488)   ← 矽晶圓供應商
    ├── 日月光投控 (3711) ← 封測大廠
    └── 信驊 (5274)     ← IP 概念股

→ 如果 3 檔以上同時觸發 Spike → 產業群聚確認 → Aiming Level 提升
```

---

## 5. 第四階段：系統環境與 AI 整合

### 5.1 記憶體配置 (M5 Max 128GB)

```
┌──────────────────────────────────────────┐
│            128 GB Unified Memory          │
├────────────────┬─────────────────────────┤
│ Rust Tick Cache│ 32 GB                   │
│ (全日 Tick)    │ ~1700 檔 × 每檔 50MB    │
├────────────────┼─────────────────────────┤
│ SurrealDB      │ 8 GB                    │
│ (圖形資料庫)    │ 全台公司+分點+關係      │
├────────────────┼─────────────────────────┤
│ OS + 其他      │ 88 GB                   │
│ (macOS, 瀏覽器) │                         │
└────────────────┴─────────────────────────┘
```

### 5.2 AI 助手整合

讓 AI (Claude Code) 自動讀取 Rust 引擎輸出的「大戶日誌」，結合歷史分點行為進行即時分析：

**範例對話：**

```
User: 「分析 2330 剛剛那波 5000 張的攻擊，是否符合地緣大戶進場特徵？」

AI 分析流程：
  1. 讀取 Spike Log → 2330 在 10:23 觸發 3-Sigma Spike
  2. 查詢 SurrealDB → 新竹地緣分點：凱基-新竹、元大-新竹
  3. 比對歷史 → 凱基-新竹過去 30 次買超，25 次後續上漲 (83% 勝率)
  4. 能量脈衝 → 階梯上升型，連續 4 步遞增
  5. 產業群聚 → 2454/6488 同時觸發 Medium Spike
  → 結論：HIGH CONFIDENCE 地緣大戶進場信號
```

### 5.3 macOS 通知系統

```rust
// 使用 macOS 原生通知
fn notify(title: &str, body: &str) {
    // 透過 osascript 發送系統通知
    Command::new("osascript")
        .arg("-e")
        .arg(format!("display notification \"{}\" with title \"{}\"", body, title))
        .spawn()
        .ok();
}

// 當 Aiming Level = CRITICAL 時觸發
notify("🎯 CRITICAL: 2330 台積電", "地緣大戶 + 階梯脈衝 + 竹科群聚");
```

---

## 6. 第五階段：主觀交易執行清單 (Execution Checklist)

當系統發出 **CRITICAL** 警示時，這是你扣下板機前的最終確認：

| # | 檢查項目 | FIRE (放行) | HOLD (暫停) |
|---|----------|-------------|-------------|
| 1 | **大盤環境 (TAIEX)** | 指數在 5 分鐘均線上方 | 大盤正在跳水 (>1% 跌幅) |
| 2 | **大戶能量 (DSP)** | 連續上升沿 (Rising Edge) | 單筆大單後繼無力 |
| 3 | **地緣關係** | 關鍵地緣分點長期吃貨中 | 該分點是著名「隔日沖」 |
| 4 | **產業群聚** | 同產業 ≥3 檔觸發 | 僅個股單獨異常 |
| 5 | **心理關卡** | 突破今日高點或整數關卡 | 接近 10% 漲停但未鎖死 |
| 6 | **時間因素** | 09:30-11:30 (主力時段) | 13:00+ (尾盤風險高) |

### 6.1 停損設定

| 情境 | 停損點 | 說明 |
|------|--------|------|
| 進場後反向 2 個 Tick | 立即出場 | 大戶也可能看錯 |
| 進場後 15 分鐘無突破 | 平倉出場 | 時間成本太高 |
| 進場後獲利 0.5% | 移動停損到成本價 | 保護利潤 |

---

## 7. 具體行動指南 (Action Plan)

### Phase 1：資料建庫（預計 2-3 天）

| 步驟 | 工作項目 | 產出 |
|------|----------|------|
| 1.1 | 安裝 SurrealDB（Docker 或 Binary） | 資料庫服務 |
| 1.2 | 爬取證交所「公司基本資料」 | Company Nodes |
| 1.3 | 爬取證交所「券商分點名冊」 | Broker Nodes |
| 1.4 | 地址 → 經緯度 (Geocoding) | 座標資料 |
| 1.5 | Haversine 距離計算 → NEARBY Edges | 地緣關聯圖 |
| 1.6 | 標記已知隔日沖/波段分點 | whale_type 標籤 |

### Phase 2：Rust 分析引擎（預計 3-5 天）

| 步驟 | 工作項目 | 產出 |
|------|----------|------|
| 2.1 | 實作 Welford 統計引擎 | 即時 μ, σ 計算 |
| 2.2 | 3-Sigma Spike 偵測器 | Spike Alert 事件 |
| 2.3 | 能量包絡線 (RMS) 計算 | 脈衝模式分類 |
| 2.4 | 整合 `accelerate` crate | SIMD 硬體加速 |
| 2.5 | 多執行緒 Dispatcher | 16 worker 並行 |

### Phase 3：Shioaji 整合（預計 2-3 天）

| 步驟 | 工作項目 | 產出 |
|------|----------|------|
| 3.1 | Shioaji WebSocket 串接 | 即時 Tick Stream |
| 3.2 | 五檔即時監控 | Order Book Imbalance |
| 3.3 | 盤後分點資料匯入 | 每日更新 whale 行為 |

### Phase 4：前端與通知（預計 1-2 天）

| 步驟 | 工作項目 | 產出 |
|------|----------|------|
| 4.1 | macOS 系統通知 | 即時 Alert |
| 4.2 | Aiming Dashboard (Web) | 視覺化監控面板 |
| 4.3 | AI 日誌分析串接 | Claude Code 整合 |

---

## 8. 核心優勢總結

這套系統的核心價值：

1. **不追高殺低** — 在大戶「剛開始推擠價格」的幾秒鐘內就偵測到
2. **地緣驗證** — 用地理關係確認「不是散戶的隨機行為」
3. **DSP 過濾** — 用數學脈衝分析區分「真突破」和「假動作」
4. **硬體加速** — M5 Max 的 SIMD/AMX 讓運算成本趨近於零
5. **AI 輔助** — Claude Code 即時解讀大戶日誌，降低認知負擔

> **這套系統的哲學：** 你不需要比大戶更聰明，你只需要比大戶更早「看見」他們。
