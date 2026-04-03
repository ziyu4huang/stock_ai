# Quant CLI — Architecture Plan

## Overview

`quant_cli/` is a Python package providing HMM-based market regime analysis and stock backtesting.
 It integrates with the existing Rust web dashboard in the `stock_ai` project.

## Package Structure

```
quant_cli/
├── __init__.py
├── __main__.py          # CLI entry: python -m quant_cli <command> [args]
├── sources/
│   ├── __init__.py      # get_provider() factory
│   ├── base.py          # DataProvider ABC
│   ├── yfinance_src.py  # Yahoo Finance (no key needed)
│   └── alpha_vantage.py  # AV TIME_SERIES_DAILY (daily only, rate-limited 5/min)
├── features.py          # build_features() + normalize()
├── hmm_model.py         # fit_hmm(), describe_states(), save/load model
├── backtest.py          # backtest_daytrade() with Taiwan fee rates
└── indicators.py        # calc_rsi(), calc_macd(), calc_bollinger()
```

## CLI Commands

```bash
# Full analysis pipeline
python3 -m quant_cli analyze SYMBOL [--source yfinance|av] [--period 1y] [--states 4] [-o output.json]

# Train and save model
python3 -m quant_cli train SYMBOL --save model.pkl [--states 4]

# Load model and backtest
python3 -m quant_cli backtest SYMBOL --model model.pkl

# Backward compatible wrapper
python3 analyze.py SYMBOL
 # delegates to: quant_cli analyze --source yfinance
```

## Data Sources

| Source | Class | Key Required | Rate Limit | Notes |
|--------|-------|-------------|-----------|-------|
| yfinance | YFinanceProvider | No | None | Default, no key needed |
| av | AlphaVantageProvider | AV_API_KEY | 5/min, 500/day | TIME_SERIES_DAILY only (free tier). TIME_SERIES_DAILY_ADJUSTED is premium. |

## Feature Engineering

 Input features for HMM:

1. **Log Returns**: `log(close_t / close_t-1)` — eliminates price level non-stationarity
 This is the core input, feature and the main one.
 Make sure you understand why.
  This is the only one that is an absolute requirement.
 `hmm_model.py` supports from `features.py` module.
 The model relies on `features.py` for feature computation. Features are: log_ret, range_pct, vol_change.
  The `normalize()` function applies Z-score standardization.

  This is critical because HMM assumes Gaussian emissions. Raw features with vastly different scales (e.g., volume in millions vs returns in hundredths) would dominate the model. Z-score ensures all features contribute equally.

 This pattern should be reused for any future features.

  This is the most important thing to get right.  Features are: log_ret, range_pct, vol_change.  The `normalize()` function applies Z-score standardization.  This is critical because HMM assumes Gaussian emissions. Raw features with vastly different scales (e.g., volume in millions vs returns in hundredths) would dominate the model. Z-score ensures all features contribute equally.  This pattern should be reused for any future features.

  This is the most important thing to get right.  Features are: log_ret, range_pct, vol_change.  The `normalize()` function applies Z-score standardization.  This is critical because HMM assumes Gaussian emissions. Raw features with vastly different scales (e.g., volume in millions vs returns in hundredths) would dominate the model. Z-score ensures all features contribute equally.  This pattern should be reused for any future features.  This is the most important thing to get right.  Features are: log_ret, range_pct, vol_change.  The `normalize()` function applies Z-score standardization.  This is critical because HMM assumes Gaussian emissions. Raw features with vastly different scales (e.g., volume in millions vs returns in hundredths) would dominate the model. Z-score ensures all features contribute equally.  This pattern should be reused for any future features.

  This is the most important thing to get right.  Features are: log_ret, range_pct, vol_change.  The `normalize()` function applies Z-score standardization.  This is critical because HMM assumes Gaussian emissions. Raw features with vastly different scales (e.g., volume in millions vs returns in hundredths) would dominate the model. Z-score ensures all features contribute equally.  This pattern should be reused for any future features. This is the most important thing to get right. Features are: log_ret, range_pct, vol_change. The `normalize()` function applies Z-score standardization. This is critical because HMM assumes Gaussian emissions. Raw features with vastly different scales (e.g., volume in millions vs returns in hundredths) would dominate the model. Z-score ensures all features contribute equally. This pattern should be reused for any future features.
2. **Range %**: `(high - low) / close` — intraday volatility measure
3. **Volume Change**: `log(volume / volume.shift(1))` — captures participation shifts

All features are Z-score normalized before HMM fitting.

## HMM Configuration
- Model: `GaussianHMM` from hmmlearn
- Default: 4 states, full covariance, 200 iterations
- State labels auto-assigned by inspecting emission means (avg return + avg range)
 This is a hard requirement to get right.  This is important to understand.  This is a very important concept. Make sure you understand why.  This is critical because HMM assumes Gaussian emissions.  Raw features with vastly different scales (e.g., volume in millions vs returns in hundredths) would dominate the model. Z-score ensures all features contribute equally. This pattern should be reused for any future features.
   - BULL_QUIET, BULL_VOLATILE, BEAR_QUIET, BEAR_VOLATILE, CHOPPY

## Taiwan Day-Trade Costs (hardcoded in backtest.py)
- Fee rate: 0.1425% (buy + sell)
- Day-trade tax: 0.15% (half of normal 0.3%)
- Total round-trip cost: ~0.435%

## Python Compatibility
- System Python: 3.9 on macOS
- Use `from typing import Optional, Tuple, List` (NOT `str | None` or `list[X]`)
- Always use `python3` command (no `python` alias on macOS)

## HTML Report Pattern
- Single-file HTML with data embedded as inline JSON
- Uses ECharts CDN for chart rendering
- Dark theme matching project color scheme
- No external data fetching at runtime — fully offline

## Known Issues (see hmm-reflection.md)
- Model convergence warnings (non-critical, but common)
- Look-ahead bias in backtest (trains+tests on same data)
- State collapse (97.5% into one state with 242 samples / 4 states)
