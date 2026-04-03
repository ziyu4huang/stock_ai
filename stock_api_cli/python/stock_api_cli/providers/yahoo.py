"""Yahoo Finance data provider (via yfinance)."""

import pandas as pd
import yfinance as yf
from .base import DataProvider
from ..types import Quote


class YFinanceProvider(DataProvider):
    """Fetch OHLCV data and quotes via yfinance (Yahoo Finance)."""

    def fetch_ohlcv(self, symbol: str, period: str = "1y") -> pd.DataFrame:
        ticker = yf.Ticker(symbol)
        df = ticker.history(period=period, auto_adjust=True)
        if df.empty:
            raise RuntimeError(f"No data for {symbol} (yfinance)")
        df.columns = [c.lower().replace(" ", "_") for c in df.columns]
        return df

    def fetch_quote(self, symbol: str) -> Quote:
        ticker = yf.Ticker(symbol)
        info = ticker.info

        price = info.get("currentPrice") or info.get("regularMarketPrice") or 0.0
        prev = info.get("previousClose") or info.get("regularMarketPreviousClose") or 0.0
        change = price - prev if prev > 0 else 0.0
        change_pct = (change / prev * 100) if prev > 0 else 0.0

        return Quote(
            symbol=symbol,
            name=info.get("shortName", symbol),
            price=round(price, 2),
            change=round(change, 2),
            change_pct=round(change_pct, 2),
            volume=info.get("volume", 0),
            high=round(info.get("dayHigh", 0.0), 2),
            low=round(info.get("dayLow", 0.0), 2),
            open=round(info.get("open", 0.0), 2),
            prev_close=round(prev, 2),
        )
