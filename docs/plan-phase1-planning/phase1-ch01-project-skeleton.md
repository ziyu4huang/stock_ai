# Ch 1 — Project Skeleton

> Maven multi-module project skeleton, shared DTO library, TimescaleDB init.

Reference prompt: [`docs/prompt-01-project-skeleton.txt`](../prompt-01-project-skeleton.txt)

---

## 1.1 Parent POM

**File:** `jzhu-trading/pom.xml`

```
groupId:    ai.jzhu.trading
artifactId: jzhu-trading
version:    1.0.0-SNAPSHOT
```

- Parent: `spring-boot-starter-parent` 4.0.3
- Java 21
- Sub-modules: `trading-common`, `strategy-core`, `web-service`, `market-data-service`
- `dependencyManagement`:
  - `spring-boot-starter-web`
  - `spring-boot-starter-webflux`
  - `spring-boot-starter-data-jdbc`
  - `lombok`
  - `jackson-databind`
  - `me.paulschwarz:spring-dotenv:4.0.0`
  - `org.postgresql:postgresql`
- Default deps (all children inherit): spring-dotenv, lombok, spring-boot-starter-test

---

## 1.2 Environment File

**File:** `jzhu-trading/.env`

```env
FMP_API_KEY=
SPRING_PROFILES_ACTIVE=dev
DB_HOST=localhost
DB_PORT=5432
DB_NAME=trading_platform
DB_USER=trading
DB_PASSWORD=trading123
```

---

## 1.3 Database Init Script

**File:** `db/init/01_init_kline.sql`

- `CREATE EXTENSION IF NOT EXISTS timescaledb;`
- Timezone: `America/New_York`
- Table `kline_daily`:
  - `time TIMESTAMPTZ NOT NULL`
  - `symbol VARCHAR(20) NOT NULL`
  - `market VARCHAR(10) NOT NULL`
  - `open, high, low, close DOUBLE PRECISION`
  - `volume BIGINT`
- Hypertable on `time`
- Unique index: `(symbol, market, time DESC)`
- `kline_weekly` / `kline_monthly`: `LIKE kline_daily INCLUDING ALL` + hypertable

**Execute:**
```bash
docker exec -i trading-timescaledb psql -U trading -d trading_platform < db/init/01_init_kline.sql
```

---

## 1.4 trading-common Module

**File:** `trading-common/pom.xml`

- Pure JAR (packaging: jar, NO spring-boot-maven-plugin)
- Deps: lombok, jackson-annotations (lightweight — no Spring starters)

**Package:** `ai.jzhu.trading.common.dto`

| Class | Fields |
|-------|--------|
| `KlineResponse` (record) | `String date`, `double open/high/low/close`, `long volume` |
| `ErrorResponse` (record) | `int status`, `String message`, `String timestamp` |

---

## 1.5 strategy-core Placeholder

**File:** `strategy-core/pom.xml`

- Empty module — content fills in Phase 3
- Package: `ai.jzhu.strategy`

---

## 1.6 TimescaleDB Docker

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

---

## 1.7 Init DB

```bash
docker exec -i trading-timescaledb psql -U trading -d trading_platform < db/init/01_init_kline.sql
```

---

## Files to Generate

1. `jzhu-trading/pom.xml`
2. `jzhu-trading/.env`
3. `db/init/01_init_kline.sql`
4. `trading-common/pom.xml`
5. `trading-common/src/main/java/ai/jzhu/trading/common/dto/KlineResponse.java`
6. `trading-common/src/main/java/ai/jzhu/trading/common/dto/ErrorResponse.java`
7. `strategy-core/pom.xml`
