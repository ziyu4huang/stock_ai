"""CLI entry point: python -m quant_cli <command> [args]"""

import argparse
import sys
import json
from datetime import datetime

from .sources import get_provider
from .features import build_features
from .hmm_model import fit_hmm, describe_states, save_model, load_model
from .backtest import backtest_daytrade
from .indicators import calc_rsi, calc_macd, calc_bollinger
from .report import generate_report

# ── Shared config ──

TAIWAN_TICKERS = {
    "2330.TW": "TSMC 台積電",
    "2317.TW": "Foxconn 鴻海",
    "2454.TW": "MediaTek 聯發科",
    "2308.TW": "Delta 達電",
    "2891.TW": "CTBC 中信金",
}


def run_analyze(args):
    """Full pipeline: fetch → features → HMM → indicators → backtest → print."""
    symbols = args.symbols
    source_name = args.source
    period = args.period
    n_states = args.states

    results = []
    for sym in symbols:
        name = TAIWAN_TICKERS.get(sym, sym)
        try:
            result = _analyze_one(sym, name, source_name, period, n_states)
            results.append(result)
        except Exception as e:
            print(f"\n  ERROR for {sym}: {e}\n")
            import traceback; traceback.print_exc()

    if args.output:
        with open(args.output, "w") as f:
            json.dump(results, f, indent=2, ensure_ascii=False)
        print(f"Results saved to {args.output}")


def _analyze_one(symbol, name, source_name, period, n_states):
    print(f"\n{'='*60}")
    print(f"  {name} ({symbol}) — HMM Regime Analysis")
    print(f"  Source: {source_name}  |  Generated: {datetime.now().strftime('%Y-%m-%d %H:%M')}")
    print(f"{'='*60}\n")

    # 1. Fetch
    provider = get_provider(source_name)
    print(f"[1/5] Fetching {symbol} data ({period}) via {source_name}...")
    df_raw = provider.fetch_ohlcv(symbol, period=period)
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
    print(f"      BIC: {model.bic(features):.1f}  |  Score: {model.score(features):.1f}")

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

    return {
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


def run_train(args):
    """Train HMM and save to file."""
    symbol = args.symbols[0]
    name = TAIWAN_TICKERS.get(symbol, symbol)

    provider = get_provider(args.source)
    print(f"Fetching {symbol} ({name}) via {args.source}, period={args.period}...")
    df_raw = provider.fetch_ohlcv(symbol, period=args.period)
    print(f"  {len(df_raw)} bars")

    features, df = build_features(df_raw)
    print(f"Fitting HMM (n_states={args.states})...")
    model = fit_hmm(features, n_states=args.states)
    states = model.predict(features)
    state_info = describe_states(model, df, states)

    print(f"  BIC: {model.bic(features):.1f}")
    for si in state_info:
        print(f"  {si['emoji']} State {si['state']}: {si['label']}  ({si['pct']}%)")

    save_model(model, args.save)
    print(f"Model saved to {args.save}")


def run_backtest(args):
    """Load saved model and run backtest."""
    symbol = args.symbols[0]
    name = TAIWAN_TICKERS.get(symbol, symbol)

    provider = get_provider(args.source)
    print(f"Fetching {symbol} ({name}) via {args.source}, period={args.period}...")
    df_raw = provider.fetch_ohlcv(symbol, period=args.period)
    features, df = build_features(df_raw)

    print(f"Loading model from {args.model}...")
    model = load_model(args.model)
    states = model.predict(features)
    state_info = describe_states(model, df, states)

    best_state = state_info[0]["state"]
    bt = backtest_daytrade(df, states, best_state)

    print(f"\nBacktest results (best state: {best_state}):")
    print(f"  Trades: {bt['total_trades']}  |  Win Rate: {bt['win_rate']}%")
    print(f"  Avg Net (after fees): {bt.get('avg_net_ret_pct', 0):+.4f}%")
    print(f"  Total Net P&L: {bt.get('total_net_pct', 0):+.2f}%")


def run_report(args):
    """Full pipeline + HTML report generation."""
    symbol = args.symbols[0]
    name = TAIWAN_TICKERS.get(symbol, symbol)
    source_name = args.source
    period = args.period
    n_states = args.states

    # 1. Fetch
    provider = get_provider(source_name)
    print(f"[1/5] Fetching {symbol} data ({period}) via {source_name}...")
    df_raw = provider.fetch_ohlcv(symbol, period=period)
    print(f"      Got {len(df_raw)} bars")

    # 2. Features
    print(f"[2/5] Building features...")
    features, df = build_features(df_raw)

    # 3. HMM
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
    output_path = args.output or f"report_{symbol.replace('.', '_')}.html"
    with open(output_path, "w", encoding="utf-8") as f:
        f.write(html)
    print(f"\nReport saved to: {output_path}")
    print(f"Open in browser: file://$(pwd)/{output_path}")


def main():
    parser = argparse.ArgumentParser(
        prog="quant_cli",
        description="HMM Quant Trade Assistant",
    )
    sub = parser.add_subparsers(dest="command", required=True)

    # ── analyze ──
    p_analyze = sub.add_parser("analyze", help="Full pipeline: fetch + HMM + backtest")
    p_analyze.add_argument("symbols", nargs="+", help="Stock symbols (e.g. 2330.TW TSLA)")
    p_analyze.add_argument("--source", default="yfinance", choices=["yfinance", "av"],
                           help="Data source (default: yfinance)")
    p_analyze.add_argument("--period", default="1y", help="Data period (default: 1y)")
    p_analyze.add_argument("--states", type=int, default=4, help="HMM states (default: 4)")
    p_analyze.add_argument("--output", "-o", help="Save JSON results to file")
    p_analyze.set_defaults(func=run_analyze)

    # ── train ──
    p_train = sub.add_parser("train", help="Train HMM and save model")
    p_train.add_argument("symbols", nargs=1, help="Stock symbol")
    p_train.add_argument("--source", default="yfinance", choices=["yfinance", "av"])
    p_train.add_argument("--period", default="1y")
    p_train.add_argument("--states", type=int, default=4)
    p_train.add_argument("--save", required=True, help="Output model path (.pkl)")
    p_train.set_defaults(func=run_train)

    # ── backtest ──
    p_bt = sub.add_parser("backtest", help="Load model and run backtest")
    p_bt.add_argument("symbols", nargs=1, help="Stock symbol")
    p_bt.add_argument("--source", default="yfinance", choices=["yfinance", "av"])
    p_bt.add_argument("--period", default="1y")
    p_bt.add_argument("--model", required=True, help="Saved model path (.pkl)")
    p_bt.set_defaults(func=run_backtest)

    # ── report ──
    p_report = sub.add_parser("report", help="Generate standalone HTML report")
    p_report.add_argument("symbols", nargs=1, help="Stock symbol (e.g. 2330.TW)")
    p_report.add_argument("--source", default="yfinance", choices=["yfinance", "av"])
    p_report.add_argument("--period", default="1y")
    p_report.add_argument("--states", type=int, default=4, help="HMM states (default: 4)")
    p_report.add_argument("--output", "-o", help="Output HTML file path")
    p_report.set_defaults(func=run_report)

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
