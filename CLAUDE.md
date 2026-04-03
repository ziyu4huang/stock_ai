# CLAUDE.md - Project Knowledge Index

Knowledge base is organized in `.agent/memory/` by category. Read relevant files before working.

## Quick Reference

- **Project:** Stock AI — Rust web dashboard + Python HMM quant CLI
- **Rust stack:** Axum + reqwest + serde + chrono (backend), Bun + lightweight-charts (frontend)
- **Python stack:** yfinance + hmmlearn + pandas + numpy (HMM regime detection)
- **Build:** `cargo build` / `cargo run` (build.rs handles bun install + bundle automatically)
- **Bun path (macOS):** `~/.bun/bin/bun` — auto-detected in build.rs via `$HOME`
- **Bun path (Windows):** `C:/Users/ziyu4/.bun/bin/bun.exe`
- **Ports:** Rust server :3003, Python CLI (no server)
- **Env:** `AV_API_KEY` required (Alpha Vantage API key, in zshrc)
- **Python:** Use `python3` (macOS has no `python` alias, system Python 3.9)
- **APIs:** TWSE OpenAPI (Taiwan stocks, .TW/.TWO), Alpha Vantage (US/global), Yahoo Finance (fallback)
- **API routes (Rust):**
  - `GET /` — serve embedded webui.html
  - `GET /api/history/{symbol}?days=N` — OHLCV bars
  - `GET /api/quote/{symbol}` — latest quote
  - `GET /api/indicators/{symbol}` — RSI(14), MACD, Bollinger Bands
- **CLI commands (Python):**
  - `python3 -m quant_cli analyze SYMBOL [--source yfinance|av] [--states N]`
  - `python3 -m quant_cli train SYMBOL --save model.pkl`
  - `python3 -m quant_cli backtest SYMBOL --model model.pkl`
  - `python3 analyze.py SYMBOL` — backward-compatible wrapper

## Knowledge Files

### develop/ - Project & Development
- [project-architecture](.agent/memory/develop/project-architecture.md) - Crate structure, build pipeline, API routes, data sources

### rust/ - Rust Language & Patterns
_(empty - add notes here as patterns emerge)_

### skill--tooling/ - Tools & Build Pipeline
- [bun-build-pipeline](.agent/memory/skill--tooling/bun-build-pipeline.md) - Bun setup, build.rs integration, `--format iife` requirement

### skill--frontend/ - Frontend Skills
- [lightweight-charts-setup](.agent/memory/skill--frontend/lightweight-charts-setup.md) - TradingView lightweight-charts integration patterns

### quant/ - Quantitative Analysis
- [quant-cli-architecture](.agent/memory/develop/quant-cli-architecture.md) - quant_cli package structure, data sources, CLI commands, Python 3.9 compat

### plan-todo/ - Plans & TODOs
- [hmm-quant-assistant-todo](docs/plan/hmm-quant-assistant-todo.md) - Phase 1-6 TODO (27% done)
- [quantitative-backtesting-system-phase1](docs/plan/quantitative-backtesting-system-phase1.md) - Java/Spring Boot reference architecture (future direction)

 TBD)

## Convention

- Each note: self-contained, category in folder name
- New categories: `.agent/memory/<category>/<kebab-case-name>.md`
- Update this index when adding new files
