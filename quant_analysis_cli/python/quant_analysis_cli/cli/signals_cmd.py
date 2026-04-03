"""signals command: compute and store trading signals."""

import json
from datetime import datetime

from ..db import (get_connection, read_ohlcv, read_ohlcv_from_json,
                  load_latest_model, save_analysis_result)
from ..analysis.features import build_features
from ..analysis.hmm import describe_states
from ..analysis.indicators import calc_rsi, calc_macd, calc_bollinger
from .analyze_cmd import TAIWAN_TICKERS


def run_signals(args):
    """Compute trading signals from latest HMM model."""
    symbol = args.symbols[0]
    name = TAIWAN_TICKERS.get(symbol, symbol)

    conn = get_connection(args.db)

    # Read data
    if args.input:
        print(f"Reading {symbol} data from file {args.input}...")
        df_raw = read_ohlcv_from_json(args.input)
    else:
        print(f"Reading {symbol} data from SQLite...")
        df_raw = read_ohlcv(conn, symbol)

    # Load model
    print(f"Loading latest model for {symbol}...")
    model_id, model = load_latest_model(conn, symbol)

    features, df = build_features(df_raw)
    states = model.predict(features)
    state_info = describe_states(model, df, states)

    current_state = int(states[-1])
    current_label_obj = next((s for s in state_info if s["state"] == current_state), None)
    current_label = current_label_obj["label"] if current_label_obj else "Unknown"

    # Indicators
    rsi = calc_rsi(df["close"])
    macd, signal, hist = calc_macd(df["close"])
    bb_upper, bb_mid, bb_lower = calc_bollinger(df["close"])
    price = float(df["close"].iloc[-1])

    # Determine signal
    confidence = 0.0
    if "BULL" in current_label:
        signal_type = "LONG"
        confidence = 0.6
        if rsi < 30:
            confidence += 0.2  # oversold bounce
        if hist > 0:
            confidence += 0.1  # MACD bullish
    elif "BEAR" in current_label:
        signal_type = "SHORT"
        confidence = 0.5
        if rsi > 70:
            confidence += 0.2
        if hist < 0:
            confidence += 0.1
    else:
        signal_type = "HOLD"
        confidence = 0.3

    confidence = min(confidence, 1.0)

    details = json.dumps({
        "rsi": round(rsi, 1),
        "macd_hist": round(hist, 2),
        "bb_position": "above" if price > bb_upper else "below" if price < bb_lower else "within",
        "regime_label": current_label,
    })

    print(f"\n  Signal: {signal_type} (confidence: {confidence:.0%})")
    print(f"  Regime: State {current_state} — {current_label}")
    print(f"  RSI: {rsi:.1f}  MACD Hist: {hist:+.2f}  Price: {price:.2f}")

    # Save to SQLite
    if getattr(args, "save", False):
        now = datetime.now().isoformat()
        today = datetime.now().strftime("%Y-%m-%d")
        conn.execute(
            """INSERT OR REPLACE INTO signal_log
               (symbol, date, signal_type, regime_state, confidence, details, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?)""",
            (symbol, today, signal_type, current_state, confidence, details, now),
        )
        conn.commit()
        print(f"  Signal saved to SQLite.")
