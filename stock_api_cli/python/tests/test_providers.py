"""Tests for provider registry and base class."""

import sys
import os
import pytest
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from stock_api_cli.providers import get_provider
from stock_api_cli.providers.base import DataProvider
from stock_api_cli.providers.yahoo import YFinanceProvider
from stock_api_cli.providers.alpha_vantage import AlphaVantageProvider


def test_get_provider_yfinance():
    p = get_provider("yfinance")
    assert isinstance(p, YFinanceProvider)
    assert isinstance(p, DataProvider)


def test_get_provider_av():
    if not os.environ.get("AV_API_KEY"):
        pytest.skip("AV_API_KEY not set")
    p = get_provider("av")
    assert isinstance(p, AlphaVantageProvider)
    assert isinstance(p, DataProvider)


def test_get_provider_unknown():
    try:
        get_provider("nonexistent")
        assert False, "Should have raised ValueError"
    except ValueError as e:
        assert "nonexistent" in str(e)


def test_provider_has_fetch_methods():
    p = get_provider("yfinance")
    assert hasattr(p, "fetch_ohlcv")
    assert hasattr(p, "fetch_quote")
    assert callable(p.fetch_ohlcv)
    assert callable(p.fetch_quote)


def test_yfinance_fetch_ohlcv_live():
    """Live test: fetch 2330.TW via yfinance (requires internet)."""
    p = get_provider("yfinance")
    df = p.fetch_ohlcv("2330.TW", period="1mo")
    assert not df.empty
    assert "open" in df.columns
    assert "high" in df.columns
    assert "low" in df.columns
    assert "close" in df.columns
    assert "volume" in df.columns
    assert len(df) > 0


def test_yfinance_fetch_quote_live():
    """Live test: quote 2330.TW via yfinance (requires internet)."""
    p = get_provider("yfinance")
    q = p.fetch_quote("2330.TW")
    assert q.symbol == "2330.TW"
    assert q.price > 0
