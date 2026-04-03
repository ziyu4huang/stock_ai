# HMM Quant Trade Assistant - Development TODO

> Status legend: `[x]` Done | `[ ]` Pending | `[-]` In Progress

---

## Phase 1: Data Pipeline (数据管线)

### 1.1 Data Fetching (数据获取)
- [x] Create `quant_cli/sources/` — dual-source data fetching (yfinance + Alpha Vantage)
- [x] Implement multi-source: `--source yfinance` / `--source av` via DataProvider ABC
- [x] Support TWSE (台股) + US market symbol mapping
- [x] Add retry & rate-limit handling for API calls (AV throttle: 5/min)

### 1.2 Feature Engineering (特征工程)
- [x] Calculate **Log Returns** (`log(close_t / close_t-1)`)
- [x] Calculate **Intraday Volatility** (`(high - low) / close`)
- [x] Calculate **Volume Change** (log volume ratio)
- [x] Normalize all features via Z-score (`features.normalize()`)
- [ ] Add **Rolling Window** statistics (20-min MA, 60-min MA)

### 1.3 Data Validation (数据校验)
- [x] Check for NaN / missing bars and dropna
- [ ] Validate trading hours (09:00–13:30 TW time)
- [ ] Verify data continuity (no gaps > 5 min during session)

---

## Phase 2: HMM Model Training (模型训练)

### 2.1 Model Setup
- [x] Install `hmmlearn` and dependencies (`pip install hmmlearn scikit-learn`)
- [x] Create `quant_cli/hmm_model.py` — model training module
- [x] Implement `GaussianHMM` with configurable state count (default: 4)
- [ ] Add **Grid Search** for optimal state count (BIC/AIC scoring)

### 2.2 State Interpretation (状态解读)
- [x] Map hidden states to **Market Regimes**:
  - BULL_QUIET: Low Vol, Positive Ret — 低波動上漲
  - BULL_VOLATILE: High Vol, Positive Ret — 高波動上漲
  - BEAR_QUIET: Low Vol, Negative Ret — 低波動下跌
  - BEAR_VOLATILE: High Vol, Negative Ret — 高波動崩盤
  - CHOPPY: Neutral — 震盪盤整
- [x] Auto-label states by inspecting avg return & range
- [ ] Generate **Regime Transition Matrix** visualization

### 2.3 Model Persistence
- [x] Save/load model via `joblib` (`hmm_model.save_model` / `load_model`)
- [ ] Add model versioning with timestamp
- [x] Log training metrics: BIC score, state distribution %

### 2.4 Anti-Overfitting Checks
- [ ] **Look-ahead Bias prevention**: strict train/test split by time (no shuffle)
- [ ] **Walk-forward validation**: rolling window retrain
- [ ] Cross-validate with at least 3 different time periods

---

## Phase 3: Day-Trading Assistant (当沖助理)

### 3.1 Real-time Signal Generation
- [ ] Create `assistant.py` — live inference loop
- [ ] Load `market_scanner.pkl` and run per-minute state prediction
- [ ] Implement **Bullish State detection** (>80% posterior probability threshold)
- [ ] Generate signals: `LONG` / `SHORT` / `FLAT`

### 3.2 Entry / Exit Logic (进场/出场逻辑)
- [ ] **Entry**: Bullish state confirmed → simulated Buy at next bar open
- [ ] **Exit — Time-based**: Auto sell at **13:20 TW time** (no overnight positions)
- [ ] **Exit — Stop-loss**: configurable % drop from entry price
- [ ] **Exit — Take-profit**: configurable % gain from entry price
- [ ] **Exit — Regime change**: sell immediately if state flips from Bullish to non-Bullish

### 3.3 Confirmation Window (防闪烁机制)
- [ ] Require state to persist **N consecutive bars** (default: 3 min) before action
- [ ] Add hysteresis: exit threshold slightly different from entry threshold
- [ ] Log all state transitions with timestamps for post-analysis

### 3.4 Risk Management (风险管理)
- [ ] Max daily trades limit (default: 3)
- [ ] Max daily loss limit (default: 2% of capital)
- [ ] Position sizing based on volatility regime
- [ ] Calculate and track **Profit Factor** (must be > transaction costs)

---

## Phase 4: Backtesting (回测验证)

### 4.1 Backtest Engine
- [x] Create `quant_cli/backtest.py` — day-trade backtest engine
- [x] Track: entries, exits, P&L per trade, cumulative return
- [ ] Feed historical data bar-by-bar to HMM (strict no look-ahead)
- [ ] Generate equity curve

### 4.2 Performance Metrics
- [ ] Win rate (%)
- [ ] Profit Factor (gross profit / gross loss)
- [ ] Sharpe Ratio (annualized)
- [ ] Max Drawdown (%)
- [ ] Average holding period (minutes)
- [ ] Transaction cost analysis (手续费 + 交易税 + 滑价)

### 4.3 Reporting
- [ ] Generate backtest report (markdown + charts)
- [ ] Export trade log to CSV
- [ ] Compare: HMM strategy vs Buy & Hold baseline

---

## Phase 5: SawClaw Rust TUI Integration (Rust 可视化)

### 5.1 IPC Bridge (Python ↔ Rust)
- [ ] Define IPC protocol (Unix Domain Socket / JSON messages)
- [ ] Python side: push state + probability every minute
- [ ] Rust side: listen and parse state updates
- [ ] Implement reconnection / heartbeat logic

### 5.2 TUI Dashboard (Ratatui)
- [ ] Display current Market Regime with color indicator:
  - 🟢 `STATE 1: BULLISH` — 适合作当沖
  - 🟡 `STATE 2: NOISE` — 不要进场
  - 🔴 `STATE 3: CRASH` — 考虑避险或放空
- [ ] Show real-time price chart (sparkline)
- [ ] Show state probability bar chart
- [ ] Show today's trade log (entry/exit/P&L)
- [ ] Keyboard shortcuts: `[r]` refresh, `[q]` quit, `[s]` toggle strategy on/off

### 5.3 Alert System
- [ ] Terminal bell / notification on state change
- [ ] Color flash on critical regime transition
- [ ] Configurable sound alert (optional)

---

## Phase 6: Production Hardening (生产强化)

### 6.1 Deployment
- [ ] Dockerize: Python HMM service + Rust TUI in one compose
- [ ] Environment config: API keys, model path, trading hours
- [ ] Health check endpoint for HMM service

### 6.2 Monitoring & Logging
- [ ] Structured JSON logging (timestamp, state, probability, action)
- [ ] Daily auto-report generation (P&L summary)
- [ ] Model drift detection: alert if state distribution shifts > threshold

### 6.3 Model Retraining Pipeline
- [ ] Weekly auto-retrain with latest data
- [ ] Compare new model vs old model on last-week data
- [ ] Auto-promote if BIC improves, else keep old model

---

## Reflection & Improvement Items

### HIGH Priority

#### R1: Walk-Forward Validation (fix look-ahead bias)
- [ ] Implement walk-forward backtest: train on [0:T], predict T+1, slide window day-by-day
- [ ] Report out-of-sample win rate, Sharpe, max drawdown
- [ ] Compare walk-forward vs in-sample metrics side-by-side

**Why:** Current backtest trains+tests on same data → 66.7% win rate is inflated. Real performance will be worse.

#### R2: Fix State Collapse — reduce states + auto-select
- [ ] Change default `--states` from 4 to 3
- [ ] Add `--states auto` mode: run BIC grid search for n_states in [2,6], pick best
- [ ] Print BIC comparison table when using auto mode

**Why:** 97.5% of days collapsed into State 2 with 4 states on 2330.TW. Model cannot distinguish regimes. Fewer states may work better; auto-selection avoids manual tuning.

### MEDIUM Priority

#### R3: Richer Feature Engineering
- [ ] Add rolling realized volatility (20-day std of log returns)
- [ ] Add return momentum (consecutive up/down day count)
- [ ] Add volume-price correlation feature
- [ ] Add ADF stationarity test (fail loudly if features are non-stationary)

**Why:** Only 3 features on 242 samples is too sparse for 4-state HMM. More informative features → better regime separation.

#### R4: Parameterized HTML Report
- [ ] Add `report` subcommand: `python3 -m quant_cli report SYMBOL --output report.html`
- [ ] Generate standalone HTML with embedded data + ECharts for dark theme
- [ ] Include: K-line chart, regime overlay, indicator gauges, backtest equity curve

**Why:** Current `hmm_2330.html` has hardcoded analysis. Need to generate for any symbol on demand.

### LOW Priority

#### R5: Model Versioning & Metadata
- [ ] Save metadata dict alongside model pkl: symbol, date range, features, BIC, state labels
- [ ] Add `python3 -m quant_cli models list` to show saved models with metadata

#### R6: Sophisticated Backtest
- [ ] Add stop-loss / take-profit exits
- [ ] Add short selling in bearish regimes
- [ ] Model slippage (open = actual + random offset)
- [ ] Position sizing based on volatility regime

---

## Quick Reference: File Structure

```
hmm-quant/
├── quant_cli/                # Phase 1-2: Modular Python CLI package
│   ├── __init__.py
│   ├── __main__.py           # CLI entry: python -m quant_cli
│   ├── sources/
│   │   ├── base.py           # DataProvider ABC
│   │   ├── yfinance_src.py   # yfinance implementation
│   │   └── alpha_vantage.py  # AV implementation (daily only)
│   ├── features.py           # Feature engineering (log_ret, range, vol_change)
│   ├── hmm_model.py          # HMM training + state labeling + save/load
│   ├── backtest.py           # Day-trade backtest engine
│   └── indicators.py         # RSI, MACD, Bollinger Bands
├── analyze.py                # Backward-compatible wrapper
├── models/
│   └── *.pkl                 # Saved HMM models
├── requirements.txt
├── sawclaw-tui/              # Phase 5: Rust TUI
│   ├── Cargo.toml
│   └── src/main.rs
├── config.yaml               # 全局配置
└── docs/
    └── hmm-quant-assistant-todo.md  # This file
```

---

## Checklist Summary

| Phase | Total | Done | Progress |
|-------|-------|------|----------|
| 1. Data Pipeline | 11 | 8 | `████████░░░` 73% |
| 2. HMM Training | 10 | 6 | `██████░░░░░` 60% |
| 3. Day-Trading Asst | 13 | 0 | `░░░░░░░░░░░░░` 0% |
| 4. Backtesting | 9 | 2 | `██░░░░░░░░░░░` 22% |
| 5. Rust TUI | 9 | 0 | `░░░░░░░░░░░░░` 0% |
| 6. Production | 8 | 0 | `░░░░░░░░░░░░░` 0% |
| R. Reflection | 12 | 0 | `░░░░░░░░░░░░░` 0% |
| **Total** | **72** | **16** | **22%** |
