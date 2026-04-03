"""Tests for stock_api_cli types."""

import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), ".."))

from stock_api_cli.types import OHLCVBar, Quote


def test_ohlcv_bar_creation():
    bar = OHLCVBar(
        date="2024-01-15",
        open=100.0,
        high=105.0,
        low=98.0,
        close=103.0,
        volume=1000000,
    )
    assert bar.date == "2024-01-15"
    assert bar.open == 100.0
    assert bar.high == 105.0
    assert bar.low == 98.0
    assert bar.close == 103.0
    assert bar.volume == 1000000


def test_ohlcv_bar_defaults():
    bar = OHLCVBar(date="2024-01-01", open=0, high=0, low=0, close=0, volume=0)
    assert bar.date == "2024-01-01"


def test_quote_creation():
    q = Quote(
        symbol="2330.TW",
        price=580.0,
        name="TSMC",
        change=5.0,
        change_pct=0.87,
        volume=20000000,
        high=585.0,
        low=572.0,
        open=575.0,
        prev_close=575.0,
    )
    assert q.symbol == "2330.TW"
    assert q.price == 580.0
    assert q.change == 5.0
    assert q.change_pct == 0.87
    assert q.volume == 20000000


def test_quote_defaults():
    q = Quote(symbol="AAPL", price=150.0)
    assert q.name == ""
    assert q.change == 0.0
    assert q.change_pct == 0.0
    assert q.volume == 0
