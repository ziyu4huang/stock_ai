"""Abstract base for data providers."""

from abc import ABC, abstractmethod
import pandas as pd


class DataProvider(ABC):
    """Standard interface for OHLCV data sources.

    All implementations must return a DataFrame with columns:
    open, high, low, close, volume — and a DatetimeIndex.
    """

    @abstractmethod
    def fetch_ohlcv(self, symbol: str, period: str = "1y") -> pd.DataFrame:
        ...
