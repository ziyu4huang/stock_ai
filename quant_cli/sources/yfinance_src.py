"""yfinance data provider."""

import pandas as pd
import yfinance as yf
from .base import DataProvider


class YFinanceProvider(DataProvider):
    """Fetch OHLCV data via yfinance (Yahoo Finance)."""

    def fetch_ohlcv(self, symbol: str, period: str = "1y") -> pd.DataFrame:
        ticker = yf.Ticker(symbol)
        df = ticker.history(period=period, auto_adjust=True)
        if df.empty:
            raise RuntimeError(f"No data for {symbol} (yfinance)")
        df.columns = [c.lower().replace(" ", "_") for c in df.columns]
        return df
