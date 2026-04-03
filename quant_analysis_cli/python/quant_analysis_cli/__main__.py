"""CLI entry point: python -m quant_analysis_cli <command> [args]"""

import argparse
import sys

from .cli.analyze_cmd import run_analyze
from .cli.train_cmd import run_train
from .cli.backtest_cmd import run_backtest
from .cli.report_cmd import run_report
from .cli.signals_cmd import run_signals
from .cli.fetch_1m_cmd import run_fetch_1m
from .cli.analyze1m_cmd import run_analyze1m


def main():
    parser = argparse.ArgumentParser(
        prog="quant_analysis_cli",
        description="Quantitative Analysis CLI — HMM regime analysis, backtesting, signals",
    )
    parser.add_argument("--db", default=None,
                        help="SQLite database path (default: ~/.stock_ai/data.db)")

    sub = parser.add_subparsers(dest="command", required=True)

    # ── shared args ──
    def _add_common(p):
        p.add_argument("--input", "-i", default=None,
                       help="Read OHLCV from JSON file (from stock_api_cli fetch -o)")
        p.add_argument("--states", type=int, default=4,
                       help="HMM n_states (default: 4)")

    # ── analyze ──
    p_analyze = sub.add_parser("analyze", help="Full pipeline: data -> HMM -> indicators -> backtest")
    p_analyze.add_argument("symbols", nargs="+", help="Stock symbols (e.g. 2330.TW)")
    _add_common(p_analyze)
    p_analyze.add_argument("--output", "-o", default=None, help="Save JSON to file")
    p_analyze.add_argument("--save", action="store_true", help="Save results to SQLite")
    p_analyze.set_defaults(func=run_analyze)

    # ── train ──
    p_train = sub.add_parser("train", help="Train HMM and persist model")
    p_train.add_argument("symbols", nargs=1, help="Stock symbol")
    _add_common(p_train)
    p_train.add_argument("--save", default=None, help="Also export model to file (e.g. model.pkl)")
    p_train.set_defaults(func=run_train)

    # ── backtest ──
    p_backtest = sub.add_parser("backtest", help="Load saved model and run backtest")
    p_backtest.add_argument("symbols", nargs=1, help="Stock symbol")
    _add_common(p_backtest)
    p_backtest.add_argument("--model", default=None, help="Load model from file")
    p_backtest.add_argument("--model-id", type=int, default=None, help="Load model from SQLite by id")
    p_backtest.set_defaults(func=run_backtest)

    # ── report ──
    p_report = sub.add_parser("report", help="Full pipeline + HTML report")
    p_report.add_argument("symbols", nargs=1, help="Stock symbol")
    _add_common(p_report)
    p_report.add_argument("--output", "-o", default=None, help="HTML output path")
    p_report.set_defaults(func=run_report)

    # ── signals ──
    p_signals = sub.add_parser("signals", help="Compute trading signals")
    p_signals.add_argument("symbols", nargs=1, help="Stock symbol")
    p_signals.add_argument("--input", "-i", default=None, help="Read OHLCV from JSON file")
    p_signals.add_argument("--save", action="store_true", help="Save signals to SQLite")
    p_signals.set_defaults(func=run_signals)

    # ── fetch-1m ──
    p_f1m = sub.add_parser("fetch-1m", help="Fetch 1-minute OHLCV bars (last 7 days, yfinance)")
    p_f1m.add_argument("symbols", nargs=1, help="Stock symbol (e.g. 2330.TW)")
    p_f1m.add_argument("--store", action="store_true", help="Upsert bars into kline_1min SQLite table")
    p_f1m.set_defaults(func=run_fetch_1m)

    # ── analyze1m ──
    p_a1m = sub.add_parser("analyze1m", help="HMM regime analysis on 1-min bars (doc-08 taxonomy)")
    p_a1m.add_argument("symbols", nargs=1, help="Stock symbol (e.g. 2330.TW)")
    p_a1m.add_argument("--states", type=int, default=4, help="HMM n_states (default: 4)")
    p_a1m.set_defaults(func=run_analyze1m)

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
