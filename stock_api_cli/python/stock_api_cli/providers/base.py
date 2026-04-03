"""Abstract base for data providers."""

from abc import ABC, abstractmethod
import pandas as pd

from ..types import Quote


class DataProvider(ABC):
    """Standard interface for stock data sources.

    All OHLCV implementations must return a DataFrame with columns:
    open, high, low, close, volume — and a DatetimeIndex.
    """

    @abstractmethod
    def fetch_ohlcv(self, symbol: str, period: str = "1y") -> pd.DataFrame:
        """Fetch OHLCV bars for the given period.

        Args:
            symbol: Stock ticker (e.g. "2330.TW", "AAPL")
            period: One of "1mo", "3mo", "6mo", "1y", "2y", "5y", "max"

        Returns:
            DataFrame with DatetimeIndex and columns: open, high, low, close, volume
        """
        ...

    @abstractmethod
    def fetch_quote(self, symbol: str) -> Quote:
        """Fetch the latest quote for a symbol.

        Args:
            symbol: Stock ticker

        Returns:
            Quote with current price, change, volume, etc.
        """
        ...
