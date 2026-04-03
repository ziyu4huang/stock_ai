"""train command: train HMM and save to SQLite (+ optional file)."""

from ..db import get_connection, read_ohlcv, read_ohlcv_from_json, save_model
from ..analysis.features import build_features
from ..analysis.hmm import fit_hmm, describe_states
from .analyze_cmd import TAIWAN_TICKERS


def run_train(args):
    """Train HMM and save to SQLite."""
    symbol = args.symbols[0]
    name = TAIWAN_TICKERS.get(symbol, symbol)
    n_states = args.states

    # Read data
    if args.input:
        print(f"Reading {symbol} data from file {args.input}...")
        df_raw = read_ohlcv_from_json(args.input)
    else:
        print(f"Reading {symbol} data from SQLite...")
        conn = get_connection(args.db)
        df_raw = read_ohlcv(conn, symbol)
    print(f"  {len(df_raw)} bars")

    features, df = build_features(df_raw)
    print(f"Fitting HMM (n_states={n_states})...")
    model = fit_hmm(features, n_states=n_states)
    states = model.predict(features)
    state_info = describe_states(model, df, states)
    bic = model.bic(features)
    score = model.score(features)

    print(f"  BIC: {bic:.1f}  |  Score: {score:.1f}")
    for si in state_info:
        print(f"  {si['emoji']} State {si['state']}: {si['label']}  ({si['pct']}%)")

    # Save to SQLite
    conn = get_connection(args.db)
    model_id = save_model(conn, symbol, n_states, "yfinance", args.input or "1y",
                          len(df), bic, score, model)
    print(f"Model saved to SQLite (id={model_id})")

    # Optional file export
    if args.save:
        from ..analysis.hmm import save_model as save_model_file
        save_model_file(model, args.save)
        print(f"Model also saved to file: {args.save}")
