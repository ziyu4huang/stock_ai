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


def normalize(features: np.ndarray) -> np.ndarray:
    """Z-score normalization (mean=0, std=1)."""
    return (features - features.mean(axis=0)) / features.std(axis=0)
