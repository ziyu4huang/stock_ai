"""Technical indicators: RSI, MACD, Bollinger Bands."""

from typing import Tuple
import pandas as pd


def calc_rsi(series: pd.Series, period: int = 14) -> float:
    delta = series.diff()
    gain = delta.where(delta > 0, 0).rolling(period).mean()
    loss = (-delta.where(delta < 0, 0)).rolling(period).mean()
    rs = gain / loss
    rsi = 100 - (100 / (1 + rs))
    return float(rsi.iloc[-1])


def calc_macd(series: pd.Series) -> Tuple[float, float, float]:
    ema12 = series.ewm(span=12).mean()
    ema26 = series.ewm(span=26).mean()
    macd = ema12 - ema26
    signal = macd.ewm(span=9).mean()
    hist = macd - signal
    return float(macd.iloc[-1]), float(signal.iloc[-1]), float(hist.iloc[-1])


def calc_bollinger(series: pd.Series, period: int = 20, std_mult: float = 2.0) -> Tuple[float, float, float]:
    mid = series.rolling(period).mean().iloc[-1]
    std = series.rolling(period).std().iloc[-1]
    return float(mid + std_mult * std), float(mid), float(mid - std_mult * std)
