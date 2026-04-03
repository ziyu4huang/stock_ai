"""Shared types for stock_api_cli."""

from dataclasses import dataclass
from typing import Optional


@dataclass
class OHLCVBar:
    """Single OHLCV candle."""
    date: str
    open: float
    high: float
    low: float
    close: float
    volume: int


@dataclass
class Quote:
    """Latest quote for a symbol."""
    symbol: str
    price: float
    name: str = ""
    change: float = 0.0
    change_pct: float = 0.0
    volume: int = 0
    high: float = 0.0
    low: float = 0.0
    open: float = 0.0
    prev_close: float = 0.0
