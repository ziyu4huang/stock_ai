# Ch 5 — Startup Scripts

> One-click start/stop for all services + command reference.

Reference prompt: [`docs/vibe-coding-05-quick-start-script.txt`](../vibe-coding-05-quick-start-script.txt)

---

## 5.1 start-all.sh

**File:** `scripts/start-all.sh`

Start order (dependencies first):

1. **TimescaleDB Docker** — check if running, skip if already up
2. **market-data-service** — `mvn spring-boot:run -pl market-data-service` (background)
3. **web-service** — `mvn spring-boot:run -pl web-service` (background)
4. **web-app** — `cd web-app && npm run dev` (background)

**Features:**
- Wait for each service health check before starting the next
- Color-coded output: green = started, yellow = waiting, red = failed
- PID tracking for clean shutdown
- Respect `.env` file via spring-dotenv

---

## 5.2 stop-all.sh

**File:** `scripts/stop-all.sh`

- Kill tracked Java processes (market-data-service, web-service)
- Kill tracked Node process (web-app)
- Optionally stop TimescaleDB Docker container (with flag `--with-db`)
- Clean up PID files

---

## 5.3 scripts/README.md — Common Commands

**File:** `scripts/README.md`

Contents to include:

### Docker
```bash
# Start TimescaleDB
docker start trading-timescaledb
# Stop TimescaleDB
docker stop trading-timescaledb
# View logs
docker logs -f trading-timescaledb
# Reset DB (warning: deletes data)
docker rm -f trading-timescaledb
docker run -d ...  # recreate
```

### Maven Services
```bash
# Start single service
mvn spring-boot:run -pl market-data-service
mvn spring-boot:run -pl web-service

# Build all
mvn clean package -DskipTests

# Run tests
mvn test
```

### Frontend
```bash
cd web-app
npm run dev      # dev server :3000
npm run build    # production build
npm run preview  # preview production build
```

### Database
```bash
# Connect to DB
docker exec -it trading-timescaledb psql -U trading -d trading_platform

# Run init script
docker exec -i trading-timescaledb psql -U trading -d trading_platform < db/init/01_init_kline.sql

# Check table data
docker exec -it trading-timescaledb psql -U trading -d trading_platform \
  -c "SELECT symbol, COUNT(*) FROM kline_daily GROUP BY symbol;"
```

### One-click
```bash
# Start everything
./scripts/start-all.sh

# Stop everything
./scripts/stop-all.sh

# Stop including DB
./scripts/stop-all.sh --with-db
```
