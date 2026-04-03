"""analyze command: full pipeline — read data -> features -> HMM -> indicators -> backtest."""

import json
import os
from datetime import datetime

from ..db import get_connection, read_ohlcv, read_ohlcv_from_json
from ..analysis.features import build_features
from ..analysis.hmm import fit_hmm, describe_states
from ..analysis.backtest import backtest_daytrade
from ..analysis.indicators import calc_rsi, calc_macd, calc_bollinger

OUTPUT_DIR = os.path.join(
    os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(
        os.path.abspath(__file__))))), "output"
)

TAIWAN_TICKERS = {
    "2330.TW": "TSMC 台積電",
    "2317.TW": "Foxconn 鴻海",
    "2454.TW": "MediaTek 聯發科",
    "2308.TW": "Delta 達電",
    "2891.TW": "CTBC 中信金",
}


def run_analyze(args):
    """Full pipeline: read data -> features -> HMM -> indicators -> backtest -> print."""
    symbols = args.symbols
    n_states = args.states

    results = []
    for sym in symbols:
        name = TAIWAN_TICKERS.get(sym, sym)
        try:
            result = _analyze_one(sym, name, args, n_states)
            results.append(result)
        except Exception as e:
            print(f"\n  ERROR for {sym}: {e}\n")
            import traceback; traceback.print_exc()

    if args.output:
        out_path = args.output
    else:
        os.makedirs(OUTPUT_DIR, exist_ok=True)
        out_path = os.path.join(OUTPUT_DIR, "analysis_result.json")
    with open(out_path, "w") as f:
        json.dump(results, f, indent=2, ensure_ascii=False)
    print(f"Results saved to {out_path}")


def _get_ohlcv(args, symbol: str):
    """Load OHLCV data from either JSON file (--input) or SQLite."""
    if args.input:
        return read_ohlcv_from_json(args.input)
    conn = get_connection(args.db)
    return read_ohlcv(conn, symbol)


def _analyze_one(symbol, name, args, n_states):
    print(f"\n{'='*60}")
    print(f"  {name} ({symbol}) — HMM Regime Analysis")
    print(f"  Generated: {datetime.now().strftime('%Y-%m-%d %H:%M')}")
    print(f"{'='*60}\n")

    # 1. Read data
    source_desc = f"file {args.input}" if args.input else "SQLite"
    print(f"[1/5] Reading {symbol} data from {source_desc}...")
    df_raw = _get_ohlcv(args, symbol)
    print(f"      Got {len(df_raw)} bars: {df_raw.index[0].date()} -> {df_raw.index[-1].date()}")
    print(f"      Latest close: {df_raw['close'].iloc[-1]:.2f}")

    # 2. Features
    print(f"\n[2/5] Building features (log return, range, volume change)...")
    features, df = build_features(df_raw)
    print(f"      Feature matrix: {features.shape}")

    # 3. HMM
    print(f"\n[3/5] Fitting GaussianHMM (n_states={n_states})...")
    model = fit_hmm(features, n_states=n_states)
    states = model.predict(features)
    state_info = describe_states(model, df, states)
    bic = model.bic(features)
    score = model.score(features)
    print(f"      BIC: {bic:.1f}  |  Score: {score:.1f}")

    print(f"\n      +--- Market Regimes -----------------------------------+")
    for si in state_info:
        print(f"      | {si['emoji']} State {si['state']}: {si['label']}")
        print(f"      |    Days: {si['count']} ({si['pct']}%)  "
              f"Avg Ret: {si['avg_daily_ret_pct']:+.4f}%  "
              f"Avg Range: {si['avg_range_pct']:.4f}%")
    print(f"      +--------------------------------------------------------+")

    # Current regime
    current_state = int(states[-1])
    current_label = next((s for s in state_info if s["state"] == current_state), None)
    cl = current_label["label"] if current_label else "Unknown"
    ce = current_label["emoji"] if current_label else "[?]"
    print(f"\n      >> Current State: {current_state} -> {ce} {cl}")
    print(f"         Last 5 states: {[int(s) for s in states[-5:]]}")

    # 4. Indicators
    print(f"\n[4/5] Technical Indicators...")
    rsi = calc_rsi(df["close"])
    macd, signal, hist = calc_macd(df["close"])
    bb_upper, bb_mid, bb_lower = calc_bollinger(df["close"])
    price = df["close"].iloc[-1]

    rsi_sig = "OVERSOLD" if rsi < 30 else "OVERBOUGHT" if rsi > 70 else "NEUTRAL"
    macd_sig = "BULLISH" if hist > 0 else "BEARISH"
    bb_pos = ("ABOVE UPPER" if price > bb_upper else
              "BELOW LOWER" if price < bb_lower else "WITHIN BANDS")

    print(f"      RSI(14):  {rsi:.1f}  [{rsi_sig}]")
    print(f"      MACD:     {macd:.2f}  Signal: {signal:.2f}  Hist: {hist:+.2f}  [{macd_sig}]")
    print(f"      Bollinger: {bb_lower:.2f} / {bb_mid:.2f} / {bb_upper:.2f}  [{bb_pos}]")

    # 5. Backtest
    print(f"\n[5/5] Day-Trade Backtest (long only in best regime)...")
    best_state = state_info[0]["state"]
    bt = backtest_daytrade(df, states, best_state)
    print(f"      Best regime: State {best_state}")
    print(f"      Trades: {bt['total_trades']}  |  Win Rate: {bt['win_rate']}%")
    print(f"      Avg Gross: {bt.get('avg_gross_ret_pct',0):+.4f}%  |  "
          f"Avg Net (after fees): {bt.get('avg_net_ret_pct',0):+.4f}%")
    print(f"      Total Net P&L: {bt.get('total_net_pct',0):+.2f}%")
    print(f"      Cost per trade: {bt.get('cost_per_trade_pct',0):.4f}%")
    if bt.get("last_5_trades"):
        print(f"\n      Last 5 trades:")
        for t in bt["last_5_trades"]:
            tag = "WIN " if t["net_pct"] > 0 else "LOSS"
            print(f"        {tag} {t['date']}  Entry: {t['entry']}  Exit: {t['exit']}  "
                  f"Net: {t['net_pct']:+.3f}%")

    # Recommendation
    print(f"\n{'='*60}")
    print(f"  RECOMMENDATION for {symbol} ({name})")
    print(f"{'='*60}")
    if current_label:
        if "BULL" in current_label["label"]:
            print(f"  >> Regime is BULLISH — consider day-trade LONG today")
            print(f"     Target: scalp intraday moves, close before 13:30")
        elif "BEAR" in current_label["label"]:
            print(f"  >> Regime is BEARISH — STAY OUT or short only")
        else:
            print(f"  >> Regime is CHOPPY — high risk of whipsaw, stay on hands")
    print(f"  Hard stop: close ALL positions by 13:20 to avoid settlement risk")
    print(f"{'='*60}\n")

    result = {
        "symbol": symbol, "name": name,
        "close": round(float(price), 2),
        "current_state": current_state,
        "current_label": cl,
        "rsi": round(rsi, 1),
        "macd": round(macd, 2),
        "macd_signal": round(signal, 2),
        "macd_hist": round(hist, 2),
        "bb_upper": round(bb_upper, 2),
        "bb_mid": round(bb_mid, 2),
        "bb_lower": round(bb_lower, 2),
        "backtest": bt,
        "states": state_info,
    }

    # Save to SQLite if --save
    if getattr(args, "save", False):
        conn = get_connection(args.db)
        from ..db import save_analysis_result, save_model as db_save_model
        db_save_model(conn, symbol, n_states, "quant_analysis_cli", args.input or "sqlite",
                       args.input or "1y", len(df), bic, score, model)
        save_analysis_result(conn, result)
        print(f"  Results saved to SQLite.")

    return result
