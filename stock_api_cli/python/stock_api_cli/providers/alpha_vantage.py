"""Alpha Vantage data provider (daily OHLCV + global quote)."""

import os
import time
from typing import Optional
import requests
import pandas as pd
from .base import DataProvider
from ..types import Quote

# Rate-limit guard: track last few call timestamps
_call_times: list = []
_RATE_LIMIT = 5        # max calls per window
_RATE_WINDOW = 61.0    # seconds (AV free tier: 5/min, add 1s buffer)


def _throttle():
    """Block until we're within the rate limit."""
    global _call_times
    now = time.time()
    _call_times = [t for t in _call_times if now - t < _RATE_WINDOW]
    if len(_call_times) >= _RATE_LIMIT:
        wait = _RATE_WINDOW - (now - _call_times[0]) + 0.5
        if wait > 0:
            print(f"      [AV] rate-limit: waiting {wait:.0f}s ...")
            time.sleep(wait)
    _call_times.append(time.time())


def _check_av_errors(data: dict, symbol: str):
    """Raise on common AV error responses."""
    if "Error Message" in data:
        raise RuntimeError(f"AV error: {data['Error Message']}")
    if "Note" in data:
        raise RuntimeError(f"AV rate limit: {data['Note']}")
    if "Information" in data:
        raise RuntimeError(f"AV info: {data['Information']}")


class AlphaVantageProvider(DataProvider):
    """Fetch daily OHLCV data via Alpha Vantage TIME_SERIES_DAILY.

    Reads AV_API_KEY from environment. Free tier: 5 calls/min, 500/day.
    """

    BASE_URL = "https://www.alphavantage.co/query"

    def __init__(self, api_key: Optional[str] = None):
        self.api_key = api_key or os.environ.get("AV_API_KEY")
        if not self.api_key:
            raise RuntimeError("AV_API_KEY not set (env var or constructor arg)")

    def fetch_ohlcv(self, symbol: str, period: str = "1y") -> pd.DataFrame:
        _throttle()

        params = {
            "function": "TIME_SERIES_DAILY",
            "symbol": symbol,
            "outputsize": "full" if period in ("2y", "5y", "max") else "compact",
            "apikey": self.api_key,
        }

        resp = requests.get(self.BASE_URL, params=params, timeout=30)
        resp.raise_for_status()
        data = resp.json()
        _check_av_errors(data, symbol)

        ts_key = "Time Series (Daily)"
        if ts_key not in data:
            raise RuntimeError(f"No time series in AV response for {symbol}")

        rows = data[ts_key]
        records = []
        for date_str, vals in rows.items():
            records.append({
                "date": date_str,
                "open": float(vals["1. open"]),
                "high": float(vals["2. high"]),
                "low": float(vals["3. low"]),
                "close": float(vals["4. close"]),
                "volume": int(vals["5. volume"]),
            })

        df = pd.DataFrame(records)
        df["date"] = pd.to_datetime(df["date"])
        df.set_index("date", inplace=True)
        df.sort_index(inplace=True)

        # Trim to requested period
        if period != "max":
            cmap = {"1y": 365, "2y": 730, "5y": 1825}
            days = cmap.get(period, 365)
            cutoff = pd.Timestamp.now().normalize() - pd.Timedelta(days=days)
            df = df[df.index >= cutoff]

        if df.empty:
            raise RuntimeError(f"No data for {symbol} (Alpha Vantage)")

        return df

    def fetch_quote(self, symbol: str) -> Quote:
        _throttle()

        params = {
            "function": "GLOBAL_QUOTE",
            "symbol": symbol,
            "apikey": self.api_key,
        }

        resp = requests.get(self.BASE_URL, params=params, timeout=30)
        resp.raise_for_status()
        data = resp.json()
        _check_av_errors(data, symbol)

        gq = data.get("Global Quote", {})
        if not gq:
            raise RuntimeError(f"No quote data from AV for {symbol}")

        price = float(gq.get("05. price", 0))
        prev = float(gq.get("08. previous close", 0))
        change = float(gq.get("09. change", 0))
        change_pct_str = gq.get("10. change percent", "0%").replace("%", "")
        change_pct = float(change_pct_str)

        return Quote(
            symbol=symbol,
            price=round(price, 2),
            change=round(change, 2),
            change_pct=round(change_pct, 2),
            volume=int(gq.get("06. volume", 0)),
            high=round(float(gq.get("03. high", 0)), 2),
            low=round(float(gq.get("04. low", 0)), 2),
            open=round(float(gq.get("02. open", 0)), 2),
            prev_close=round(prev, 2),
        )
