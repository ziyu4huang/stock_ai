# stock_api_cli — Architecture (replaces quant_cli)

> Multi-vendor, multi-language market data API framework + HMM quant engine.
> Current name: `quant_cli/` — will be renamed to `stock_api_cli/` in Phase 2.

---

## Current State: quant_cli/

```
quant_cli/
├── __init__.py
├── __main__.py              # CLI: python -m quant_cli <command>
├── sources/
│   ├── __init__.py          # get_provider() factory
│   ├── base.py              # DataProvider ABC
│   ├── yfinance_src.py      # Yahoo Finance
│   └── alpha_vantage.py     # Alpha Vantage
├── features.py              # build_features() + normalize()
├── hmm_model.py             # fit_hmm(), describe_states(), save/load
├── backtest.py              # backtest_daytrade() with Taiwan fees
├── indicators.py            # calc_rsi(), calc_macd(), calc_bollinger()
└── report.py                # generate_report() → standalone HTML
```

## Target State: stock_api_cli/

```
stock_api_cli/
├── __init__.py
├── __main__.py               ← CLI: python -m stock_api_cli <cmd>
├── types.py                  ← shared: OHLCVBar, Quote, MarketInfo
│
├── yahoo_finance/            ← vendor: Yahoo Finance
│   ├── __init__.py
│   └── python/
│       ├── __init__.py
│       └── client.py         ← YFinanceClient(StockAPIClient)
│   ├── bun/                  ← (Phase 3)
│   │   └── client.ts
│   └── rust/                 ← (Phase 3)
│       └── src/lib.rs
│
├── alpha_vantage/            ← vendor: Alpha Vantage
│   ├── __init__.py
│   └── python/
│       ├── __init__.py
│       └── client.py         ← AlphaVantageClient(StockAPIClient)
│   ├── bun/
│   └── rust/
│
├── hmm/                      ← HMM analysis engine
│   ├── __init__.py
│   ├── model.py              ← GaussianHMM + state labeling + save/load
│   ├── features.py           ← log_ret, range_pct, vol_change + normalize
│   └── backtest.py           ← day-trade backtest, Taiwan fee rates
│
├── indicators.py             ← calc_rsi(), calc_macd(), calc_bollinger()
├── report.py                 ← generate_report() → standalone ECharts HTML
└── bench.py                  ← cross-vendor benchmark runner
```

---

## Provider Interface

```python
# stock_api_cli/types.py
class OHLCVBar:
    date: str        # "2026-04-03"
    open: float
    high: float
    low: float
    close: float
    volume: int

class Quote:
    symbol: str
    price: float
    change: float
    change_pct: float
    volume: int
    high: float
    low: float
```

```python
# Each vendor/python/client.py implements:
class StockAPIClient(ABC):
    @abstractmethod
    def fetch_daily(self, symbol: str, period: str = "1y") -> List[OHLCVBar]: ...

    @abstractmethod
    def fetch_quote(self, symbol: str) -> Optional[Quote]: ...

    @property
    @abstractmethod
    def vendor_name(self) -> str: ...

    @property
    @abstractmethod
    def rate_limit(self) -> str: ...
```

---

## CLI Commands

```bash
# Analysis
python3 -m stock_api_cli analyze SYMBOL [--source yfinance|av] [--period 1y] [-o result.json]
python3 -m stock_api_cli report SYMBOL [--period 1y] [-o report.html]

# HMM
python3 -m stock_api_cli train SYMBOL --save model.pkl
python3 -m stock_api_cli backtest SYMBOL --model model.pkl

# NEW: Benchmark
python3 -m stock_api_cli bench SYMBOL [--vendors yfinance,alpha_vantage]
```

---

## Vendor × Language Matrix

| Vendor \ Language | Python | Bun/TS | Rust |
|-------------------|--------|--------|------|
| Yahoo Finance | ✅ current | ☐ Phase 3 | ☐ Phase 3 |
| Alpha Vantage | ✅ current | ☐ Phase 3 | ☐ Phase 3 |
| FMP | ☐ future | ☐ | ☐ |
| TWSE OpenAPI | ☐ | ☐ | ✅ in Rust |

---

## Benchmark Output

```json
{
  "symbol": "2330.TW",
  "date": "2026-04-03",
  "results": {
    "yfinance": {
      "bars": 242, "latency_ms": 320, "latest": "2026-04-02",
      "rate_limit": "none", "cost": "free"
    },
    "alpha_vantage": {
      "bars": 100, "latency_ms": 580, "latest": "2026-04-02",
      "rate_limit": "5/min", "cost": "free tier"
    }
  }
}
```

---

## Feature Engineering (unchanged)

Critical HMM inputs:

1. **Log Returns**: `log(close_t / close_t-1)` — eliminates price level non-stationarity
2. **Range %**: `(high - low) / close` — intraday volatility
3. **Volume Change**: `log(volume / volume.shift(1))` — participation shifts

Z-score normalized before HMM fitting (Gaussian emission assumption).

## HMM Config

- `GaussianHMM` from hmmlearn
- Default: 4 states, full covariance, 200 iterations
- Labels: BULL_QUIET, BULL_VOLATILE, BEAR_QUIET, BEAR_VOLATILE, CHOPPY

## Taiwan Day-Trade Costs

- Fee: 0.1425% × 2 (buy + sell)
- Tax: 0.15% (day-trade rate)
- Total round-trip: ~0.435%

## Python Compat

- System Python 3.9 on macOS
- `from typing import Optional, Tuple, List` (not `str | None`)
- Always `python3`

## Known Issues

- Model convergence warnings (non-critical)
- Look-ahead bias (trains+tests on same data)
- State collapse (97.5% into one state with limited samples)
