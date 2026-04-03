# Ch 3 — Web Service (BFF)

> Backend-for-Frontend layer. Only backend the React app talks to. Port 8181.

Reference prompt: [`docs/prompt-03-backend-service.txt`](../prompt-03-backend-service.txt)

---

## 3.1 Service Skeleton

**File:** `web-service/pom.xml`

- Spring Boot app, port **8181**
- Deps: spring-boot-starter-web, spring-boot-starter-webflux, trading-common

**Package:** `ai.jzhu.trading.web`

**application.yml:**
```yaml
server:
  port: 8181
spring:
  application:
    name: web-service
service:
  market-data:
    url: http://localhost:8182
```

---

## 3.2 MarketDataPort (Outbound Interface)

**Package:** `domain.port`

```java
public interface MarketDataPort {
    List<KlineResponse> getKline(String symbol, String market,
                                  String period, String startDate, String endDate);
}
```

---

## 3.3 MarketDataClient

**Package:** `infrastructure.client`

- Implements `MarketDataPort`
- Injects `@Value("${service.market-data.url}")` for base URL
- Uses `WebClient` to call `GET /api/market-data/kline` on market-data-service
- Passes all query parameters through
- Error handling: downstream unavailable → return friendly error

---

## 3.4 GetKlineUseCase (Web Layer)

**Package:** `application.usecase`

- Simple pass-through: delegates to `MarketDataPort`
- No business logic at this stage (future phases add aggregation)

---

## 3.5 KlineController

**Package:** `presentation.controller`

```
GET /api/web/kline
  Params: same as market-data-service
    symbol, market, period, startDate, endDate
  Returns: List<KlineResponse> (transparent passthrough)
```

---

## 3.6 CorsConfig

**Package:** `infrastructure.config`

```
Allowed Origins:      http://localhost:3000
Allowed Methods:      GET, POST, PUT, DELETE, OPTIONS
Allowed Headers:      *
Allow Credentials:    true
```

---

## 3.7 GlobalExceptionHandler

**Package:** `presentation.exception`

- Catches downstream service errors → returns `ErrorResponse` with friendly message

---

## Clean Architecture Package Map

```
ai.jzhu.trading.web
├── WebServiceApplication.java
├── domain/
│   └── port/
│       └── MarketDataPort.java
├── application/
│   └── usecase/
│       └── GetKlineUseCase.java
├── infrastructure/
│   ├── client/
│   │   └── MarketDataClient.java
│   └── config/
│       ├── WebClientConfig.java
│       └── CorsConfig.java
└── presentation/
    ├── controller/
    │   └── KlineController.java
    └── exception/
        └── GlobalExceptionHandler.java
```

---

## Request Flow

```
React (:3000)
  → GET /api/web/kline?symbol=TSLA&...
  → web-service (:8181) KlineController
  → GetKlineUseCase
  → MarketDataClient (WebClient)
  → market-data-service (:8182) GET /api/market-data/kline?symbol=TSLA&...
  → GetKlineUseCase (market-data)
  → TimescaleDB or FMP API
  → response bubbles back
```
