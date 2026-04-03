"""fetch-1m: Download 1-minute OHLCV bars and store in kline_1min.

yfinance allows at most 7 days of 1-minute data per request.
Run daily (or more frequently) to accumulate a long history.
All timestamps are stored as UTC Unix seconds.
"""

import json
import sys
from datetime import datetime, timezone

import pandas as pd
import yfinance as yf

from ..db import get_connection, upsert_kline_1min, count_kline_1min


def _fetch_1min_bars(symbol: str) -> list[dict]:
    """Fetch last 7 days of 1-minute bars from yfinance.

    Returns list of dicts: {ts, open, high, low, close, volume}
    where ts is a UTC Unix timestamp (integer seconds).
    """
    ticker = yf.Ticker(symbol)
    df = ticker.history(period="7d", interval="1m", auto_adjust=True)
    if df.empty:
        raise RuntimeError(f"No 1-minute data returned for {symbol} (yfinance)")

    df.columns = [c.lower().replace(" ", "_") for c in df.columns]

    # Ensure the DatetimeIndex is UTC-aware before converting
    if df.index.tz is None:
        df.index = df.index.tz_localize("UTC")
    else:
        df.index = df.index.tz_convert("UTC")

    bars = []
    for ts_ns, row in df.iterrows():
        # pandas Timestamp → Python int (seconds)
        ts = int(ts_ns.timestamp())
        bars.append({
            "ts": ts,
            "open": round(float(row["open"]), 4),
            "high": round(float(row["high"]), 4),
            "low": round(float(row["low"]), 4),
            "close": round(float(row["close"]), 4),
            "volume": int(row.get("volume", 0)),
        })
    return bars


def run_fetch_1m(args):
    symbol = args.symbols[0].upper()
    print(f"[1/2] Fetching 1-min bars for {symbol} (last 7 days)...", file=sys.stderr)

    bars = _fetch_1min_bars(symbol)
    print(f"      {len(bars)} bars fetched.", file=sys.stderr)

    if args.store:
        conn = get_connection(args.db)
        new_rows = upsert_kline_1min(conn, symbol, bars)
        total = count_kline_1min(conn, symbol)
        print(
            f"[2/2] Stored {new_rows} new bars → total {total} bars for {symbol}.",
            file=sys.stderr,
        )
        # Also print summary JSON to stdout for Rust to parse
        result = {
            "symbol": symbol,
            "fetched": len(bars),
            "new_rows": new_rows,
            "total_rows": total,
        }
        json.dump(result, sys.stdout, ensure_ascii=False)
        print()
    else:
        # Print raw bars as JSON (pipe-friendly)
        result = {
            "symbol": symbol,
            "interval": "1m",
            "fetched": len(bars),
            "bars": bars,
        }
        json.dump(result, sys.stdout, indent=2, ensure_ascii=False)
        print()
