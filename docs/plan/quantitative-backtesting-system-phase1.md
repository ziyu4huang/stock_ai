# 量化回测系统 - 第1期开发计划

## 1. 系统概述

构建一个量化交易回测平台，支持股票 K 线数据展示、技术指标计算、策略回测与绩效评估。第 1 期聚焦于**基础骨架搭建 + K 线数据展示**，为后续回测功能奠定基础。

### 第 1 期目标

- 搭建项目骨架（多模块/多服务结构）
- 实现市场数据获取与本地缓存
- 搭建后台 API 网关（BFF 层）
- 实现前端 K 线图展示页面
- 提供一键启停脚本

### 整体架构（第 1 期）

```
┌──────────────┐     ┌──────────────┐     ┌─────────────────────┐     ┌──────────┐
│  web-app     │────▶│  web-service │────▶│  market-data-service│────▶│ FMP API  │
│  (React/ECharts)  │  (BFF 后台)  │     │  (数据服务+缓存)    │     │          │
│  :3000       │     │  :8181       │     │  :8182              │     └──────────┘
└──────────────┘     └──────────────┘     └─────────┬───────────┘
                                                    │
                                           ┌────────▼────────┐
                                           │  TimescaleDB    │
                                           │  (Docker :5432) │
                                           └─────────────────┘
```

---

## 2. 技术选型

| 层级 | 技术 | 说明 |
|------|------|------|
| **后端** | Java 21 + Spring Boot 4.x | 微服务架构，Maven 多模块管理 |
| **前端** | React 19 + TypeScript 5.9 + Vite 7 | SPA 单页应用 |
| **图表** | ECharts 5 | 专业 K 线图（红涨绿跌、主副图联动） |
| **UI** | Bootstrap 5 | 响应式布局，深色主题 |
| **数据库** | TimescaleDB (PostgreSQL) | 时序数据库，K 线数据缓存 |
| **外部数据** | FMP API (Financial Modeling Prep) | 美股历史 K 线数据源 |
| **HTTP 客户端** | Spring WebClient | 响应式调用外部 API |
| **容器** | Docker | TimescaleDB 运行容器 |

---

## 3. 项目模块结构

```
jzhu-trading/
├── .env                          # 环境变量（API Key、数据库配置）
├── pom.xml                       # 父 POM（统一依赖管理）
├── db/
│   └── init/
│       └── 01_init_kline.sql     # K 线表初始化（日K/周K/月K）
├── trading-common/               # 共享 DTO 库（纯 JAR，非 Spring Boot 应用）
│   ├── pom.xml
│   └── src/main/java/ai/jzhu/trading/common/
│       └── dto/
│           ├── KlineResponse.java    # K 线数据 record
│           └── ErrorResponse.java    # 错误响应 record
├── strategy-core/                # 策略核心（第1期空模块占位，第3期填充）
│   └── pom.xml
├── market-data-service/          # 市场数据服务（数据获取 + TimescaleDB 缓存）
│   ├── pom.xml
│   └── src/main/java/ai/jzhu/trading/marketdata/
│       ├── MarketDataServiceApplication.java
│       ├── domain/
│       │   ├── model/Kline.java
│       │   └── port/
│       │       ├── KlineRepository.java       # 数据库出站端口
│       │       └── MarketDataProvider.java    # 外部 API 出站端口
│       ├── application/
│       │   └── usecase/GetKlineUseCase.java   # 核心用例：缓存优先
│       ├── infrastructure/
│       │   ├── persistence/JdbcKlineRepository.java
│       │   ├── external/
│       │   │   └── FmpMarketDataProvider.java
│       │   └── config/WebClientConfig.java
│       └── presentation/
│           ├── controller/MarketDataController.java
│           └── exception/GlobalExceptionHandler.java
├── web-service/                  # BFF 层（前端唯一直接交互的后端）
│   ├── pom.xml
│   └── src/main/java/ai/jzhu/trading/web/
│       ├── WebServiceApplication.java
│       ├── domain/port/MarketDataPort.java
│       ├── application/usecase/GetKlineUseCase.java
│       ├── infrastructure/
│       │   ├── client/MarketDataClient.java
│       │   └── config/
│       │       ├── WebClientConfig.java
│       │       └── CorsConfig.java
│       └── presentation/
│           ├── controller/KlineController.java
│           └── exception/GlobalExceptionHandler.java
├── web-app/                      # React 前端（不在 Maven 管理中）
│   ├── package.json
│   ├── vite.config.ts
│   └── src/
│       ├── main.tsx
│       ├── App.tsx
│       ├── api/index.ts             # Axios 实例
│       ├── components/
│       │   ├── Navbar.tsx
│       │   ├── SearchBar.tsx
│       │   ├── StockTabs.tsx
│       │   └── KlineChart.tsx        # ECharts K 线图核心组件
│       ├── pages/KlinePage.tsx
│       ├── types/index.ts
│       └── styles/global.css         # 深色主题
└── scripts/
    ├── start-all.sh                 # 一键启动所有服务
    ├── stop-all.sh                  # 一键停止所有服务
    └── README.md                    # 常用命令文档
```

---

## 4. 开发任务分解

### 阶段 1：项目骨架搭建

**目标：** 创建 Maven 多模块项目骨架、共享模块、数据库初始化

| # | 任务 | 说明 |
|---|------|------|
| 1.1 | 创建父 POM | groupId: `ai.jzhu.trading`，Java 21，Spring Boot 4.0.3，统一依赖管理 |
| 1.2 | 创建 `.env` 文件 | FMP_API_KEY、数据库连接配置、Spring profile |
| 1.3 | 创建 `db/init/01_init_kline.sql` | kline_daily/weekly/monthly 三张时序表，hypertable + 唯一索引 |
| 1.4 | 创建 `trading-common` 模块 | 纯 JAR，包含 `KlineResponse` 和 `ErrorResponse` record |
| 1.5 | 创建 `strategy-core` 占位模块 | 空 pom.xml，第 3 期填充 |
| 1.6 | 启动 TimescaleDB Docker 容器 | 端口 5432，持久化数据卷 |
| 1.7 | 执行数据库初始化脚本 | 创建 hypertable 和索引 |

### 阶段 2：市场数据服务（market-data-service）

**目标：** 对接 FMP API 获取美股 K 线数据，TimescaleDB 本地缓存

| # | 任务 | 说明 |
|---|------|------|
| 2.1 | 搭建服务骨架 | Spring Boot 应用，端口 8182，Clean Architecture 包结构 |
| 2.2 | 定义领域层 | `Kline` 领域模型，`KlineRepository` / `MarketDataProvider` 出站端口接口 |
| 2.3 | 实现 `GetKlineUseCase` | 核心：查 DB → 有则返回 → 无则调 API → 写入 DB → 返回 |
| 2.4 | 实现 `JdbcKlineRepository` | JdbcTemplate 查询 + batchUpdate，`ON CONFLICT DO NOTHING` |
| 2.5 | 实现 `FmpMarketDataProvider` | WebClient 调 FMP API，反转数据顺序，错误处理 |
| 2.6 | 实现 REST 控制器 | `GET /api/market-data/kline`，参数：symbol, market, period, startDate, endDate |
| 2.7 | 全局异常处理 | API 限流、无效 symbol、下游不可用等场景 |

**核心逻辑流程：**
```
请求 → GetKlineUseCase
         ├─ 1. 根据 period 选择表 (daily/weekly/monthly)
         ├─ 2. 查 TimescaleDB
         ├─ 3. 有数据 → 直接返回（缓存命中）
         └─ 4. 无数据 → 调 FMP API → 批量写入 DB → 返回（缓存未命中）
```

### 阶段 3：BFF 后台服务（web-service）

**目标：** 前端唯一直接交互的网关层，转发 K 线数据请求

| # | 任务 | 说明 |
|---|------|------|
| 3.1 | 搭建服务骨架 | Spring Boot，端口 8181，依赖 trading-common |
| 3.2 | 定义出站端口 | `MarketDataPort` 接口 |
| 3.3 | 实现 `MarketDataClient` | WebClient 调 market-data-service（端口 8182） |
| 3.4 | 实现 REST 控制器 | `GET /api/web/kline`，透传到 market-data-service |
| 3.5 | CORS 配置 | 允许 `http://localhost:3000`，全方法、全 Header |
| 3.6 | 全局异常处理 | 下游不可用时返回友好错误 |

### 阶段 4：React 前端（web-app）

**目标：** 搜索股票 + K 线图展示（含成交量副图）

| # | 任务 | 说明 |
|---|------|------|
| 4.1 | 初始化 React 项目 | Vite 7 + TypeScript 5.9，端口 3000 |
| 4.2 | 安装依赖 | Bootstrap 5, ECharts 5, Axios |
| 4.3 | 实现深色主题样式 | 背景 `#0d1117`，卡片 `#161b22`，红涨绿跌配色 |
| 4.4 | 实现 `Navbar` | "JZhu Trading" 品牌标识，"K 线回测" 导航 |
| 4.5 | 实现 `SearchBar` | 股票代码、市场、周期、日期范围输入，查询按钮 |
| 4.6 | 实现 `StockTabs` | 多股票标签切换、删除，当前选中高亮 |
| 4.7 | 实现 `KlineChart` | ECharts candlestick + 成交量柱状图，dataZoom 缩放 |
| 4.8 | 实现 `KlinePage` | 整合所有组件，状态管理，API 调用流程 |

**K 线图布局：**
```
┌─────────────────────────────┐
│  主图：K 线 (candlestick)    │  70% 高度
│  红色阳线 #ef4444            │
│  绿色阴线 #22c55e            │
├─────────────────────────────┤
│  副图：成交量 (bar)           │  20% 高度
│  红绿跟随 K 线颜色           │
├─────────────────────────────┤
│  dataZoom 滑动条             │  10% 高度
└─────────────────────────────┘
```

### 阶段 5：一键启停脚本

**目标：** 提供便捷的开发环境启停命令

| # | 任务 | 说明 |
|---|------|------|
| 5.1 | 创建 `scripts/start-all.sh` | 按序启动：TimescaleDB → market-data-service → web-service → web-app |
| 5.2 | 创建 `scripts/stop-all.sh` | 停止所有服务进程 + Docker 容器 |
| 5.3 | 创建常用命令文档 | 启停命令、数据库操作、Docker 命令速查 |

---

## 5. API 设计

### market-data-service（端口 8182）

```
GET /api/market-data/kline
  参数:
    symbol    (String, 必填)  - 股票代码，如 TSLA
    market    (String, 默认 "us") - 市场
    period    (String, 默认 "daily") - K 线周期: daily / weekly / monthly
    startDate (String, 可选)  - yyyy-MM-dd
    endDate   (String, 可选)  - yyyy-MM-dd

  返回: List<KlineResponse>
    [{ date, open, high, low, close, volume }, ...]
```

### web-service（端口 8181）

```
GET /api/web/kline
  参数: 同 market-data-service
  返回: 同 market-data-service（透传）
```

---

## 6. 数据库设计

### TimescaleDB 配置

```bash
docker run -d \
  --name trading-timescaledb \
  -p 5432:5432 \
  -e POSTGRES_DB=trading_platform \
  -e POSTGRES_USER=trading \
  -e POSTGRES_PASSWORD=trading123 \
  -v trading-pgdata:/var/lib/postgresql/data \
  timescale/timescaledb:latest-pg16
```

### K 线表结构

```sql
CREATE TABLE kline_daily (
    time    TIMESTAMPTZ NOT NULL,
    symbol  VARCHAR(20) NOT NULL,
    market  VARCHAR(10) NOT NULL,
    open    DOUBLE PRECISION,
    high    DOUBLE PRECISION,
    low     DOUBLE PRECISION,
    close   DOUBLE PRECISION,
    volume  BIGINT
);
-- hypertable + 唯一索引 (symbol, market, time DESC)
-- kline_weekly / kline_monthly 结构相同
```

---

## 7. 深色主题配色方案

| 用途 | 色值 |
|------|------|
| 页面背景 | `#0d1117` |
| 卡片/面板 | `#161b22` |
| 边框 | `#30363d` |
| 主文字 | `#e6edf3` |
| 次文字 | `#8b949e` |
| 涨/阳线 | `#ef4444`（红色） |
| 跌/阴线 | `#22c55e`（绿色） |
| 按钮蓝 | `#2563eb` |
| 按钮橙 | `#f97316` |

---

## 8. 服务端口分配

| 服务 | 端口 |
|------|------|
| web-app（React） | 3000 |
| web-service（BFF） | 8181 |
| market-data-service | 8182 |
| TimescaleDB | 5432 |

---

## 9. 环境变量

```env
FMP_API_KEY=<your_key>
SPRING_PROFILES_ACTIVE=dev
DB_HOST=localhost
DB_PORT=5432
DB_NAME=trading_platform
DB_USER=trading
DB_PASSWORD=trading123
```

---

## 10. 第 1 期范围说明

### 包含

- 项目骨架（Maven 多模块）
- 市场数据服务（FMP API + TimescaleDB 缓存）
- BFF 网关服务
- React 前端（搜索 + K 线图 + 成交量）
- 一键启停脚本

### 不包含（后续期数）

- 回测面板、策略选择、参数配置
- 技术指标叠加（MA、BOLL、RSI 等）
- 策略引擎与绩效评估
- indicator-service、backtest-service、strategy-service、api-gateway
- 用户认证与权限管理

---

## 11. 后续路线图预览

| 期数 | 主题 | 核心内容 |
|------|------|----------|
| 第 2 期 | 技术指标服务 | MA/EMA/MACD/RSI/BOLL 计算，前端指标叠加 |
| 第 3 期 | 策略引擎 | strategy-core 实现，策略定义、信号生成 |
| 第 4 期 | 回测系统 | backtest-service，历史回测、绩效报告 |
| 第 5 期 | 实盘模拟 | paper trading，实时行情推送 |
| 第 6 期 | 生产部署 | API Gateway、认证、监控、CI/CD |
