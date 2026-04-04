# CLAUDE.md - Project Knowledge Index

Knowledge base is organized in `.agent/memory/` by category. Read relevant files before working.

## Quick Reference

- **Project:** Stock AI — Cargo workspace Rust binary + Python stock data & quant analysis CLI
- **Rust stack:** Axum + reqwest + serde + chrono + rusqlite (backend), Bun + ECharts (frontend)
- **Python stack:** yfinance + hmmlearn + pandas + numpy (stock data, analysis, reporting)
- **Workspace:** `crates/stock-core` (types, indicators, SQLite, fetchers) + `crates/stock-server` (Axum binary)
- **Build:** `cargo build` / `cargo run` (build.rs handles bun install + bundle automatically)
- **Bun path (macOS):** `~/.bun/bin/bun` — auto-detected in build.rs via `$HOME`
- **Bun path (Windows):** `C:/Users/ziyu4/.bun/bin/bun.exe`
- **Ports:** Rust server :3003, Python CLI (no server)
- **Env:** `AV_API_KEY` required (Alpha Vantage API key, in zshrc)
- **Python:** Use `python3` (macOS has no `python` alias, system Python 3.9)
- **APIs:** TWSE OpenAPI (Taiwan stocks, .TW/.TWO), Alpha Vantage (US/global), Yahoo Finance (fallback)
- **JS runtime:** Always use Bun (not npm). `bun install`, `bun run`
- **API routes (Rust):**
  - `GET /` — serve embedded webui.html
  - `GET /api/history/{symbol}?days=N` — OHLCV bars (cached)
  - `GET /api/quote/{symbol}` — latest quote
  - `GET /api/indicators/{symbol}` — RSI(14), MACD, Bollinger Bands
  - `GET /api/kline/{symbol}?period=&from=&to=` — filtered kline
  - `GET /api/backtest/{symbol}` — spawns quant_analysis_cli analyze
  - `GET /api/report/{symbol}` — spawns quant_analysis_cli report (HTML)
  - `GET|POST|DELETE /api/watchlist` — watchlist CRUD
  - `GET|POST|DELETE /api/strategies` — strategy CRUD
  - `GET /api/signals/{symbol}` — latest signals from SQLite
  - `POST /api/scan` — parallel scan all watchlist stocks
- **CLI commands (stock_api_cli):**
  - Python: `PYTHONPATH=stock_api_cli/python python3 -m stock_api_cli fetch SYMBOL [--source yfinance|av] [--period 1y]`
  - Python: `PYTHONPATH=stock_api_cli/python python3 -m stock_api_cli quote SYMBOL [--source yfinance|av]`
  - Bun: `bun run stock_api_cli/bun/src/cli.ts fetch SYMBOL [--source yfinance|av] [--period 1y]`
  - Bun: `bun run stock_api_cli/bun/src/cli.ts quote SYMBOL [--source yfinance|av]`
- **stock_api_cli structure:** `stock_api_cli/{python/stock_api_cli, bun/src, rust/}`
- **CLI commands (quant_analysis_cli):**
  - Python: `PYTHONPATH=quant_analysis_cli/python python3 -m quant_analysis_cli analyze SYMBOL [--input data.json] [--save]`
  - Python: `PYTHONPATH=quant_analysis_cli/python python3 -m quant_analysis_cli train SYMBOL [--save model.pkl]`
  - Python: `PYTHONPATH=quant_analysis_cli/python python3 -m quant_analysis_cli backtest SYMBOL [--model model.pkl | --model-id ID]`
  - Python: `PYTHONPATH=quant_analysis_cli/python python3 -m quant_analysis_cli report SYMBOL [-o report.html]`
  - Python: `PYTHONPATH=quant_analysis_cli/python python3 -m quant_analysis_cli signals SYMBOL [--save]`
- **quant_analysis_cli structure:** `quant_analysis_cli/{python/quant_analysis_cli, bun/src, rust/}`
- **Data pipeline:** `stock_api_cli fetch --store` → SQLite → `quant_analysis_cli analyze` (or `--input data.json` for file-based)
- **SQLite:** `~/.stock_ai/data.db` — tables: kline_daily, watchlist, strategies, signal_log, hmm_models, analysis_results
- **Pipeline doc:** [docs/plan/pipeline-architecture.md](docs/plan/pipeline-architecture.md) — full architecture with entry points

## Knowledge Files

### develop/ - Project & Development
- [project-architecture](.agent/memory/develop/project-architecture.md) - Workspace structure, build pipeline, API routes, data sources

### rust/ - Rust Language & Patterns
_(empty - add notes here as patterns emerge)_

### skill--tooling/ - Tools & Build Pipeline
- [bun-build-pipeline](.agent/memory/skill--tooling/bun-build-pipeline.md) - Bun setup, build.rs integration, `--format iife` requirement, Bun vs npm
- [slidev-presentations](.agent/memory/skill--tooling/slidev-presentations.md) - Slidev setup with Bun, slide QA with Playwright, formatting tips

### skill--frontend/ - Frontend Skills
- [echarts-setup](.agent/memory/skill--frontend/echarts-setup.md) - ECharts candlestick + volume, dark theme, data flow

### quant/ - Quantitative Analysis
- [stock-api-cli-architecture](.agent/memory/develop/stock-api-cli-architecture.md) - stock_api_cli package structure, providers, analysis, CLI commands

### plan-todo/ - Plans & TODOs
- [hmm-quant-assistant-todo](docs/plan/hmm-quant-assistant-todo.md) - HMM TODO (Phase 1-6)
- [quantitative-backtesting-system-phase1](docs/plan/quantitative-backtesting-system-phase1.md) - Full plan reference

### Phase 1 — Quantitative Backtesting System (Rust/Axum + Bun + SQLite + Python)
- [phase1-todo](docs/plan/phase1-todo.md) - Master TODO checklist (29 tasks, 59% done)
- [phase1-ch01-backend-enhancement](docs/plan/phase1-ch01-backend-enhancement.md) - Workspace refactor, AppState, error types
- [phase1-ch02-market-data](docs/plan/phase1-ch02-market-data.md) - Weekly/monthly aggregation, smart cache, date filtering
- [phase1-ch03-python-integration](docs/plan/phase1-ch03-python-integration.md) - Parameterized backtest API, model management endpoints
- [phase1-ch04-frontend](docs/plan/phase1-ch04-frontend.md) - Bun/ECharts, volume overlay, indicator toggles, backtest viz
- [phase1-ch05-dev-scripts](docs/plan/phase1-ch05-dev-scripts.md) - setup.sh, run.sh, command reference

### AI Day Trading — Taiwan Market Knowledge Base
- [README](docs/design/ai-day-trading-tw/README.md) - Overview, quick reference, cost structure
- [01-tw-day-trading-mechanics](docs/design/ai-day-trading-tw/01-tw-day-trading-mechanics.md) - 當沖 rules, fees, settlement, market microstructure
- [02-trading-instruments-comparison](docs/design/ai-day-trading-tw/02-trading-instruments-comparison.md) - Stocks, futures, options, warrants comparison
- [03-ai-quant-techniques](docs/design/ai-day-trading-tw/03-ai-quant-techniques.md) - ML models, features, strategies, evaluation
- [04-data-and-infrastructure](docs/design/ai-day-trading-tw/04-data-and-infrastructure.md) - Data sources, broker APIs (Shioaji), backtesting tools
- [05-risks-and-principles](docs/design/ai-day-trading-tw/05-risks-and-principles.md) - Risk management, regulations, pitfalls, daily checklist
- [06-system-architecture](docs/design/ai-day-trading-tw/06-system-architecture.md) - System design, component architecture, project structure, implementation phases
- [08-large-trade-tracker](docs/design/ai-day-trading-tw/08-large-trade-tracker.md) - 大單追蹤偵測器, HMM 意圖分類, 點火序列偵測, Tick Streamer
- [09-day-trading-scanner](docs/design/ai-day-trading-tw/09-day-trading-scanner.md) - 日內交易掃描器, 買賣信號偵測, 複合評分, 歷史回放
- [10-geo-whale-hunting](docs/design/ai-day-trading-tw/10-geo-whale-hunting.md) - 地緣大戶狙擊, SurrealDB 籌碼地理, 3-Sigma DSP 脈衝, Welford 演算法

### presentations/
- [phase1-goal-set](docs/presentation/phase1-goal-set/slides.md) - Slidev deck: architecture, goals, roadmap (14 slides)

## Convention

- Each note: self-contained, category in folder name
- New categories: `.agent/memory/<category>/<kebab-case-name>.md`
- Update this index when adding new files
