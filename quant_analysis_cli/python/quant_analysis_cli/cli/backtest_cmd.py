"""backtest command: load saved model and run backtest."""

from ..db import get_connection, read_ohlcv, read_ohlcv_from_json, load_model_blob, load_latest_model
from ..analysis.features import build_features
from ..analysis.hmm import load_model as load_model_file, describe_states
from ..analysis.backtest import backtest_daytrade
from .analyze_cmd import TAIWAN_TICKERS


def run_backtest(args):
    """Load saved model and run backtest."""
    symbol = args.symbols[0]
    name = TAIWAN_TICKERS.get(symbol, symbol)

    # Read data
    if args.input:
        print(f"Reading {symbol} data from file {args.input}...")
        df_raw = read_ohlcv_from_json(args.input)
    else:
        print(f"Reading {symbol} data from SQLite...")
        conn = get_connection(args.db)
        df_raw = read_ohlcv(conn, symbol)
    features, df = build_features(df_raw)

    # Load model
    if args.model:
        print(f"Loading model from file {args.model}...")
        model = load_model_file(args.model)
    elif getattr(args, "model_id", None):
        conn = get_connection(args.db)
        print(f"Loading model from SQLite (id={args.model_id})...")
        model = load_model_blob(conn, args.model_id)
    else:
        conn = get_connection(args.db)
        print(f"Loading latest model from SQLite...")
        model_id, model = load_latest_model(conn, symbol)
        print(f"  Loaded model id={model_id}")

    states = model.predict(features)
    state_info = describe_states(model, df, states)

    best_state = state_info[0]["state"]
    bt = backtest_daytrade(df, states, best_state)

    print(f"\nBacktest results (best state: {best_state}):")
    print(f"  Trades: {bt['total_trades']}  |  Win Rate: {bt['win_rate']}%")
    print(f"  Avg Net (after fees): {bt.get('avg_net_ret_pct', 0):+.4f}%")
    print(f"  Total Net P&L: {bt.get('total_net_pct', 0):+.2f}%")
