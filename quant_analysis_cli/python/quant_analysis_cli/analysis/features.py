"""Feature engineering for HMM input."""

from typing import Tuple
import numpy as np
import pandas as pd


def build_features(df: pd.DataFrame) -> Tuple[np.ndarray, pd.DataFrame]:
    """Compute HMM features from OHLCV data.

    Features: log return, intraday range %, volume log change.
    Returns (feature_matrix, aligned_dataframe_with_features).
    """
    df = df.copy()
    df["log_ret"] = np.log(df["close"] / df["close"].shift(1))
    df["range_pct"] = (df["high"] - df["low"]) / df["close"]
    df["vol_change"] = np.log(df["volume"] / df["volume"].shift(1))
    df.replace([np.inf, -np.inf], np.nan, inplace=True)
    df.dropna(inplace=True)
    features = df[["log_ret", "range_pct", "vol_change"]].values
    return features, df


def build_features_1m(df: pd.DataFrame) -> Tuple[np.ndarray, pd.DataFrame]:
    """HMM features for 1-minute bars — optimized for whale/regime detection.

    Features:
      log_ret    — per-minute log return (direction signal)
      range_pct  — (high-low)/close (intrabar volatility)
      vol_ratio  — volume / rolling-20-min avg (detects abnormal order flow)
      close_pos  — (close-low)/(high-low) (buying vs selling pressure within bar)
    """
    df = df.copy()
    df["log_ret"] = np.log(df["close"] / df["close"].shift(1))
    df["range_pct"] = (df["high"] - df["low"]) / df["close"].clip(lower=1e-9)
    vol_ma = df["volume"].rolling(20, min_periods=5).mean().clip(lower=1)
    df["vol_ratio"] = df["volume"] / vol_ma
    hl = (df["high"] - df["low"]).clip(lower=1e-9)
    df["close_pos"] = (df["close"] - df["low"]) / hl
    df.replace([np.inf, -np.inf], np.nan, inplace=True)
    df.dropna(inplace=True)
    features = df[["log_ret", "range_pct", "vol_ratio", "close_pos"]].values
    return features, df


def normalize(features: np.ndarray) -> np.ndarray:
    """Z-score normalization (mean=0, std=1)."""
    return (features - features.mean(axis=0)) / features.std(axis=0)
