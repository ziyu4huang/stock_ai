# Ch 2 — Market Data Service

> FMP API integration + TimescaleDB local cache. Port 8182.

Reference prompt: [`docs/prompt-02-market-data-service.txt`](../prompt-02-market-data-service.txt)

---

## 2.1 Service Skeleton

**File:** `market-data-service/pom.xml`

- Spring Boot app, port **8182**
- Deps: spring-boot-starter-web, spring-boot-starter-webflux, spring-boot-starter-data-jdbc, postgresql, trading-common

**Package:** `ai.jzhu.trading.marketdata`

**application.yml:**
```yaml
server:
  port: 8182
spring:
  application:
    name: market-data-service
  datasource:
    url: jdbc:postgresql://${DB_HOST:localhost}:${DB_PORT:5432}/${DB_NAME:trading_platform}
    username: ${DB_USER:trading}
    password: ${DB_PASSWORD:trading123}
    driver-class-name: org.postgresql.Driver
  jdbc:
    template:
      query-timeout: 30s
fmp:
  api-key: ${FMP_API_KEY}
  base-url: https://financialmodelingprep.com/api/v3
```

---

## 2.2 Domain Layer

**Package:** `ai.jzhu.trading.marketdata.domain`

### Model

| Class | Type | Fields |
|-------|------|--------|
| `Kline` | record | `String date`, `double open/high/low/close`, `long volume` |

### Outbound Ports

| Interface | Method | Purpose |
|-----------|--------|---------|
| `KlineRepository` | `List<Kline> findBySymbolAndMarketAndPeriodBetween(symbol, market, period, start, end)` | DB read |
| `KlineRepository` | `void saveAll(List<Kline>, period)` | DB batch write |
| `MarketDataProvider` | `List<Kline> fetchKline(symbol, market, period, start, end)` | External API |

---

## 2.3 GetKlineUseCase

**Package:** `application.usecase`

**Core logic (cache-first):**

```
1. Map period → table name (daily/weekly/monthly)
2. Query DB via KlineRepository
3. If rows > 0 → return (cache HIT, log it)
4. Else → call MarketDataProvider (FMP API)
5. Batch insert into DB with ON CONFLICT DO NOTHING
6. Return data (cache MISS, log it)
```

---

## 2.4 JdbcKlineRepository

**Package:** `infrastructure.persistence`

- Uses `JdbcTemplate`
- Query: `SELECT * FROM {table} WHERE symbol=? AND market=? AND time BETWEEN ? AND ? ORDER BY time ASC`
- Batch insert: `JdbcTemplate.batchUpdate` with `ON CONFLICT DO NOTHING`

---

## 2.5 FmpMarketDataProvider

**Package:** `infrastructure.external`

- WebClient call: `GET {base-url}/historical-price-full/{symbol}?apikey={key}&from={start}&to={end}`
- FMP response mapping (`FmpHistoricalResponse`):
  - `String symbol`
  - `List<FmpDailyPrice> historical`
  - `@JsonIgnoreProperties(ignoreUnknown = true)` on nested DTO
- **Reverse** the `historical` list (FMP returns newest-first)
- Error handling: rate-limit → friendly message, invalid symbol → empty list

---

## 2.6 MarketDataController

**Package:** `presentation.controller`

```
GET /api/market-data/kline
  Params:
    symbol    (String, required)  — e.g. TSLA
    market    (String, default "us")
    period    (String, default "daily") — daily / weekly / monthly
    startDate (String, optional) — yyyy-MM-dd
    endDate   (String, optional) — yyyy-MM-dd
  Returns: List<KlineResponse>
```

---

## 2.7 GlobalExceptionHandler

**Package:** `presentation.exception`

Handles:
- FMP API rate-limit → 429 with friendly message
- Invalid symbol → 200 with empty list
- Downstream timeout → 503

---

## Clean Architecture Package Map

```
ai.jzhu.trading.marketdata
├── MarketDataServiceApplication.java
├── domain/
│   ├── model/Kline.java
│   └── port/
│       ├── KlineRepository.java
│       └── MarketDataProvider.java
├── application/
│   └── usecase/GetKlineUseCase.java
├── infrastructure/
│   ├── persistence/JdbcKlineRepository.java
│   ├── external/
│   │   ├── FmpMarketDataProvider.java
│   │   └── dto/FmpHistoricalResponse.java
│   └── config/WebClientConfig.java
└── presentation/
    ├── controller/MarketDataController.java
    └── exception/GlobalExceptionHandler.java
```

---

## Key Technical Points

- `GetKlineUseCase` depends only on interfaces (ports), not implementations
- WebClient configured as Bean with base-url injection
- FMP API key via `@Value("${fmp.api-key}")`
- Log distinction: "from database" vs "from FMP API" for cache hit/miss visibility
- Batch write uses `ON CONFLICT DO NOTHING` to avoid duplicates
