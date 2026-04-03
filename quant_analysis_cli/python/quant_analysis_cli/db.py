"""SQLite data access layer for quant_analysis_cli.

Shared database: ~/.stock_ai/data.db (same as Rust server).
Tables:
  - kline_daily   (populated by stock_api_cli fetch --store)
  - kline_1min    (populated by quant_analysis_cli fetch-1m --store)
  - hmm_models    (populated by quant_analysis_cli train)
  - analysis_results (populated by quant_analysis_cli analyze --save)
  - signal_log    (populated by quant_analysis_cli signals)
"""

import io
import json
import os
import sqlite3
from datetime import datetime
from typing import Optional, List

import joblib
import pandas as pd

_DEFAULT_DB = os.path.join(os.environ.get("HOME", "."), ".stock_ai", "data.db")

_SCHEMA = """
CREATE TABLE IF NOT EXISTS kline_1min (
    symbol TEXT NOT NULL,
    ts     INTEGER NOT NULL,
    open   REAL NOT NULL,
    high   REAL NOT NULL,
    low    REAL NOT NULL,
    close  REAL NOT NULL,
    volume INTEGER NOT NULL,
    PRIMARY KEY (symbol, ts)
);
CREATE INDEX IF NOT EXISTS idx_k1m ON kline_1min(symbol, ts DESC);

CREATE TABLE IF NOT EXISTS hmm_models (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    n_states INTEGER NOT NULL DEFAULT 4,
    source TEXT NOT NULL DEFAULT 'yfinance',
    period TEXT NOT NULL DEFAULT '1y',
    bars_used INTEGER,
    bic REAL,
    score REAL,
    model_blob BLOB NOT NULL,
    created_at TEXT NOT NULL,
    UNIQUE(symbol, n_states, source, period)
);
CREATE INDEX IF NOT EXISTS idx_hmm_symbol ON hmm_models(symbol);

CREATE TABLE IF NOT EXISTS analysis_results (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    model_id INTEGER REFERENCES hmm_models(id),
    close_price REAL,
    current_state INTEGER,
    current_label TEXT,
    rsi REAL,
    macd REAL,
    macd_signal REAL,
    macd_hist REAL,
    bb_upper REAL,
    bb_mid REAL,
    bb_lower REAL,
    state_info TEXT NOT NULL,
    backtest_result TEXT,
    analyzed_at TEXT NOT NULL,
    UNIQUE(symbol, analyzed_at)
);
CREATE INDEX IF NOT EXISTS idx_ar_symbol ON analysis_results(symbol);

CREATE TABLE IF NOT EXISTS signal_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    symbol TEXT NOT NULL,
    date TEXT NOT NULL,
    signal_type TEXT NOT NULL,
    regime_state INTEGER,
    confidence REAL NOT NULL DEFAULT 0,
    details TEXT,
    created_at TEXT NOT NULL,
    UNIQUE(symbol, date, signal_type)
);
CREATE INDEX IF NOT EXISTS idx_sig_sym ON signal_log(symbol, date DESC);
"""


def get_connection(db_path: Optional[str] = None) -> sqlite3.Connection:
    """Open SQLite connection, creating analysis tables if needed."""
    path = db_path or os.environ.get("STOCK_AI_DB", _DEFAULT_DB)
    os.makedirs(os.path.dirname(path), exist_ok=True)
    conn = sqlite3.Connection(path)
    conn.row_factory = sqlite3.Row
    conn.executescript(_SCHEMA)
    return conn


def read_ohlcv(conn: sqlite3.Connection, symbol: str,
               from_ts: Optional[int] = None, to_ts: Optional[int] = None) -> pd.DataFrame:
    """Read OHLCV from kline_daily, return DataFrame with DatetimeIndex.

    Raises RuntimeError if no data found.
    """
    sql = "SELECT time, open, high, low, close, volume FROM kline_daily WHERE symbol = ?"
    params = [symbol]
    if from_ts is not None:
        sql += " AND time >= ?"
        params.append(from_ts)
    if to_ts is not None:
        sql += " AND time <= ?"
        params.append(to_ts)
    sql += " ORDER BY time"

    rows = conn.execute(sql, params).fetchall()
    if not rows:
        raise RuntimeError(
            f"No OHLCV data for {symbol} in database. "
            f"Run: PYTHONPATH=stock_api_cli/python python3 -m stock_api_cli fetch {symbol} --store"
        )
    df = pd.DataFrame(rows, columns=["time", "open", "high", "low", "close", "volume"])
    df["date"] = pd.to_datetime(df["time"], unit="s")
    df.set_index("date", inplace=True)
    return df[["open", "high", "low", "close", "volume"]]


def read_ohlcv_from_json(path: str) -> pd.DataFrame:
    """Read OHLCV from a JSON file produced by stock_api_cli fetch.

    Supports the --input flag for file-based pipeline.
    """
    with open(path) as f:
        data = json.load(f)
    bars = data.get("bars", data) if isinstance(data, dict) else data
    df = pd.DataFrame(bars)
    df["date"] = pd.to_datetime(df["date"])
    df.set_index("date", inplace=True)
    for col in ["open", "high", "low", "close"]:
        df[col] = df[col].astype(float)
    df["volume"] = df["volume"].astype(int)
    return df[["open", "high", "low", "close", "volume"]]


def save_model(conn: sqlite3.Connection, symbol: str, n_states: int,
               source: str, period: str, bars_used: int,
               bic: float, score: float, model) -> int:
    """Serialize and save an HMM model, return the row id."""
    buf = io.BytesIO()
    joblib.dump(model, buf)
    buf.seek(0)
    blob = buf.read()
    now = datetime.now().isoformat()

    conn.execute(
        """INSERT OR REPLACE INTO hmm_models
           (symbol, n_states, source, period, bars_used, bic, score, model_blob, created_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)""",
        (symbol, n_states, source, period, bars_used, bic, score, blob, now),
    )
    conn.commit()
    row = conn.execute(
        "SELECT id FROM hmm_models WHERE symbol=? AND n_states=? AND source=? AND period=?",
        (symbol, n_states, source, period),
    ).fetchone()
    return row["id"]


def load_model_blob(conn: sqlite3.Connection, model_id: int):
    """Load a serialized HMM model from SQLite by id."""
    row = conn.execute(
        "SELECT model_blob FROM hmm_models WHERE id = ?", (model_id,)
    ).fetchone()
    if not row:
        raise RuntimeError(f"No model found with id={model_id}")
    return joblib.load(io.BytesIO(row["model_blob"]))


def load_latest_model(conn: sqlite3.Connection, symbol: str):
    """Load the most recent model for a symbol."""
    row = conn.execute(
        "SELECT id, model_blob FROM hmm_models WHERE symbol = ? ORDER BY created_at DESC LIMIT 1",
        (symbol,),
    ).fetchone()
    if not row:
        raise RuntimeError(f"No trained model found for {symbol}. Run: train {symbol}")
    return row["id"], joblib.load(io.BytesIO(row["model_blob"]))


def save_analysis_result(conn: sqlite3.Connection, result: dict) -> int:
    """Store an analysis result, return the row id."""
    now = datetime.now().isoformat()
    conn.execute(
        """INSERT OR REPLACE INTO analysis_results
           (symbol, close_price, current_state, current_label,
            rsi, macd, macd_signal, macd_hist,
            bb_upper, bb_mid, bb_lower,
            state_info, backtest_result, analyzed_at)
           VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)""",
        (
            result["symbol"],
            result.get("close"),
            result.get("current_state"),
            result.get("current_label"),
            result.get("rsi"),
            result.get("macd"),
            result.get("macd_signal"),
            result.get("macd_hist"),
            result.get("bb_upper"),
            result.get("bb_mid"),
            result.get("bb_lower"),
            json.dumps(result.get("states", []), ensure_ascii=False),
            json.dumps(result.get("backtest", {}), ensure_ascii=False),
            now,
        ),
    )
    conn.commit()
    row = conn.execute(
        "SELECT id FROM analysis_results WHERE symbol=? AND analyzed_at=?",
        (result["symbol"], now),
    ).fetchone()
    return row["id"]


def read_latest_analysis(conn: sqlite3.Connection, symbol: str) -> Optional[dict]:
    """Read the most recent analysis for a symbol."""
    row = conn.execute(
        "SELECT * FROM analysis_results WHERE symbol = ? ORDER BY analyzed_at DESC LIMIT 1",
        (symbol,),
    ).fetchone()
    if not row:
        return None
    d = dict(row)
    d["state_info"] = json.loads(d["state_info"])
    d["backtest_result"] = json.loads(d.get("backtest_result") or "{}")
    return d


# ── kline_1min ────────────────────────────────────────────────────────────

def upsert_kline_1min(conn: sqlite3.Connection, symbol: str, bars: list) -> int:
    """Insert 1-minute bars, silently skip duplicates (INSERT OR IGNORE).

    Args:
        bars: list of dicts with keys ts, open, high, low, close, volume
              ts is a Unix timestamp (UTC integer seconds).

    Returns:
        Number of newly inserted rows.
    """
    before = conn.execute("SELECT COUNT(*) FROM kline_1min WHERE symbol=?", (symbol,)).fetchone()[0]
    conn.executemany(
        "INSERT OR IGNORE INTO kline_1min (symbol, ts, open, high, low, close, volume) "
        "VALUES (?, ?, ?, ?, ?, ?, ?)",
        [(symbol, b["ts"], b["open"], b["high"], b["low"], b["close"], b["volume"]) for b in bars],
    )
    conn.commit()
    after = conn.execute("SELECT COUNT(*) FROM kline_1min WHERE symbol=?", (symbol,)).fetchone()[0]
    return after - before


def read_kline_1min(conn: sqlite3.Connection, symbol: str,
                    from_ts: Optional[int] = None,
                    to_ts: Optional[int] = None) -> pd.DataFrame:
    """Read 1-min bars from kline_1min into a UTC-indexed DataFrame.

    Raises RuntimeError if no data found.
    """
    sql = "SELECT ts, open, high, low, close, volume FROM kline_1min WHERE symbol=?"
    params: list = [symbol]
    if from_ts is not None:
        sql += " AND ts >= ?"
        params.append(from_ts)
    if to_ts is not None:
        sql += " AND ts <= ?"
        params.append(to_ts)
    sql += " ORDER BY ts"

    rows = conn.execute(sql, params).fetchall()
    if not rows:
        raise RuntimeError(
            f"No 1-min data for {symbol}. "
            f"Run: python3 -m quant_analysis_cli fetch-1m {symbol} --store"
        )
    df = pd.DataFrame(rows, columns=["ts", "open", "high", "low", "close", "volume"])
    df.index = pd.to_datetime(df["ts"], unit="s", utc=True)
    df.index.name = "datetime"
    return df[["open", "high", "low", "close", "volume"]]


def count_kline_1min(conn: sqlite3.Connection, symbol: str) -> int:
    """Return total number of stored 1-min bars for a symbol."""
    row = conn.execute("SELECT COUNT(*) FROM kline_1min WHERE symbol=?", (symbol,)).fetchone()
    return row[0] if row else 0
