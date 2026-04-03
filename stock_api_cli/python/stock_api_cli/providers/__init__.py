"""Data source providers — registry and factory."""

from .base import DataProvider
from .yahoo import YFinanceProvider
from .alpha_vantage import AlphaVantageProvider

_PROVIDERS = {
    "yfinance": YFinanceProvider,
    "av": AlphaVantageProvider,
}


def get_provider(name: str) -> DataProvider:
    """Instantiate a data provider by name ('yfinance' or 'av')."""
    cls = _PROVIDERS.get(name)
    if cls is None:
        raise ValueError(f"Unknown source '{name}'. Choose from: {list(_PROVIDERS)}")
    return cls()
