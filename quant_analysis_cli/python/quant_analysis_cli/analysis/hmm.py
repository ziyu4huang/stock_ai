"""HMM model training, state interpretation, persistence."""

import warnings
from typing import List
import numpy as np
from hmmlearn.hmm import GaussianHMM
import joblib

warnings.filterwarnings("ignore")


def fit_hmm(features: np.ndarray, n_states: int = 4, seed: int = 42) -> GaussianHMM:
    """Fit a GaussianHMM to the feature matrix."""
    model = GaussianHMM(
        n_components=n_states,
        covariance_type="full",
        n_iter=200,
        random_state=seed,
        tol=0.01,
    )
    model.fit(features)
    return model


def describe_states(model: GaussianHMM, df, states: np.ndarray) -> List[dict]:
    """Classify each hidden state with human-readable labels.

    Returns list of dicts sorted by avg_daily_ret descending.
    """
    daily_ret = df["close"].pct_change().fillna(0).values
    info = []

    for s in range(model.n_components):
        mask = states == s
        count = mask.sum()
        if count == 0:
            continue

        avg_ret = daily_ret[mask].mean() * 100
        avg_vol = df["range_pct"].values[mask].mean() * 100
        pct = count / len(states) * 100

        if avg_ret > 0.05 and avg_vol < 2.0:
            label = "BULL_QUIET  低波動上漲"
            emoji = "[G]"
        elif avg_ret > 0.05 and avg_vol >= 2.0:
            label = "BULL_VOLATILE 高波動上漲"
            emoji = "[Y]"
        elif avg_ret < -0.05 and avg_vol < 2.0:
            label = "BEAR_QUIET  低波動下跌"
            emoji = "[O]"
        elif avg_ret < -0.05 and avg_vol >= 2.0:
            label = "BEAR_VOLATILE 高波動崩盤"
            emoji = "[R]"
        else:
            label = "CHOPPY     震盪盤整"
            emoji = "[W]"

        info.append({
            "state": s, "label": label, "emoji": emoji,
            "count": int(count), "pct": round(pct, 1),
            "avg_daily_ret_pct": round(avg_ret, 4),
            "avg_range_pct": round(avg_vol, 4),
        })

    info.sort(key=lambda x: x["avg_daily_ret_pct"], reverse=True)
    return info


def save_model(model: GaussianHMM, path: str) -> None:
    """Persist trained HMM to disk."""
    joblib.dump(model, path)


def load_model(path: str) -> GaussianHMM:
    """Load a saved HMM from disk."""
    return joblib.load(path)
