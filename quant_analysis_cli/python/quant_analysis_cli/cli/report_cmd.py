"""report command: full pipeline + HTML report generation."""

import os

from ..db import get_connection, read_ohlcv, read_ohlcv_from_json, load_latest_model
from ..analysis.features import build_features
from ..analysis.hmm import fit_hmm, describe_states
from ..analysis.backtest import backtest_daytrade
from ..analysis.indicators import calc_rsi, calc_macd, calc_bollinger
from ..report.html_report import generate_report
from .analyze_cmd import TAIWAN_TICKERS, OUTPUT_DIR


def run_report(args):
    """Full pipeline + HTML report generation."""
    symbol = args.symbols[0]
    name = TAIWAN_TICKERS.get(symbol, symbol)
    n_states = args.states

    # 1. Read data
    conn = None
    if args.input:
        print(f"[1/5] Reading {symbol} data from file {args.input}...")
        df_raw = read_ohlcv_from_json(args.input)
    else:
        print(f"[1/5] Reading {symbol} data from SQLite...")
        conn = get_connection(args.db)
        df_raw = read_ohlcv(conn, symbol)
    print(f"      Got {len(df_raw)} bars")

    # 2. Features
    print(f"[2/5] Building features...")
    features, df = build_features(df_raw)

    # 3. HMM — try loading saved model first, fall back to fitting new one
    model = None
    if conn is not None:
        try:
            print(f"[3/5] Loading saved model for {symbol}...")
            _, model = load_latest_model(conn, symbol)
        except RuntimeError:
            model = None

    if model is None:
        print(f"[3/5] Fitting GaussianHMM (n_states={n_states})...")
        model = fit_hmm(features, n_states=n_states)

    states = model.predict(features)
    state_info = describe_states(model, df, states)
    bic = model.bic(features)
    score = model.score(features)

    current_state = int(states[-1])
    current_label_obj = next((s for s in state_info if s["state"] == current_state), None)
    current_label = current_label_obj["label"] if current_label_obj else "Unknown"

    # 4. Indicators
    print(f"[4/5] Technical Indicators...")
    rsi = calc_rsi(df["close"])
    macd, signal, hist = calc_macd(df["close"])
    bb_upper, bb_mid, bb_lower = calc_bollinger(df["close"])

    # 5. Backtest
    print(f"[5/5] Day-Trade Backtest...")
    best_state = state_info[0]["state"]
    bt = backtest_daytrade(df, states, best_state)

    # Generate HTML
    html = generate_report(
        symbol=symbol, name=name, df=df, states=states,
        state_info=state_info, current_state=current_state,
        current_label=current_label, rsi=rsi, macd=macd,
        signal=signal, hist=hist, bb_upper=bb_upper,
        bb_mid=bb_mid, bb_lower=bb_lower, backtest=bt,
        bic=bic, score=score,
    )

    # Output
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    default_name = f"report_{symbol.replace('.', '_')}.html"
    output_path = args.output or os.path.join(OUTPUT_DIR, default_name)
    with open(output_path, "w", encoding="utf-8") as f:
        f.write(html)
    print(f"\nReport saved to: {output_path}")
    print(f"Open in browser: file://{os.path.abspath(output_path)}")
