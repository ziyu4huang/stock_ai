"""CLI entry point: python -m stock_api_cli <command> [args]"""

import argparse
import json
import sys

from .providers import get_provider


def cmd_fetch(args):
    """Fetch OHLCV history for a symbol."""
    provider = get_provider(args.source)
    df = provider.fetch_ohlcv(args.symbol, period=args.period)

    bars = []
    for i in range(len(df)):
        row = df.iloc[i]
        bars.append({
            "date": str(row.name.date()) if hasattr(row.name, "date") else str(row.name),
            "open": round(float(row["open"]), 2),
            "high": round(float(row["high"]), 2),
            "low": round(float(row["low"]), 2),
            "close": round(float(row["close"]), 2),
            "volume": int(row["volume"]),
        })

    result = {
        "symbol": args.symbol,
        "source": args.source,
        "period": args.period,
        "bars": bars,
    }

    if args.output:
        with open(args.output, "w") as f:
            json.dump(result, f, indent=2, ensure_ascii=False)
        print(f"Saved {len(bars)} bars to {args.output}")
    else:
        json.dump(result, sys.stdout, indent=2, ensure_ascii=False)
        print()


def cmd_quote(args):
    """Fetch latest quote for a symbol."""
    provider = get_provider(args.source)
    q = provider.fetch_quote(args.symbol)
    result = {
        "symbol": q.symbol,
        "name": q.name,
        "price": q.price,
        "change": round(q.change, 2),
        "change_pct": round(q.change_pct, 2),
        "volume": q.volume,
        "high": q.high,
        "low": q.low,
        "open": q.open,
        "prev_close": q.prev_close,
    }

    if args.output:
        with open(args.output, "w") as f:
            json.dump(result, f, indent=2, ensure_ascii=False)
        print(f"Quote saved to {args.output}")
    else:
        json.dump(result, sys.stdout, indent=2, ensure_ascii=False)
        print()


def main():
    parser = argparse.ArgumentParser(
        prog="stock_api_cli",
        description="Stock API CLI — multi-source data fetch connector",
    )
    sub = parser.add_subparsers(dest="command", required=True)

    # ── fetch ──
    p_fetch = sub.add_parser("fetch", help="Fetch OHLCV history")
    p_fetch.add_argument("symbol", help="Stock symbol (e.g. 2330.TW, AAPL)")
    p_fetch.add_argument("--source", default="yfinance", choices=["yfinance", "av"],
                         help="Data source (default: yfinance)")
    p_fetch.add_argument("--period", default="1y", help="Period: 1mo,3mo,6mo,1y,2y,5y,max")
    p_fetch.add_argument("--output", "-o", help="Save JSON to file")
    p_fetch.set_defaults(func=cmd_fetch)

    # ── quote ──
    p_quote = sub.add_parser("quote", help="Get latest quote")
    p_quote.add_argument("symbol", help="Stock symbol")
    p_quote.add_argument("--source", default="yfinance", choices=["yfinance", "av"],
                         help="Data source (default: yfinance)")
    p_quote.add_argument("--output", "-o", help="Save JSON to file")
    p_quote.set_defaults(func=cmd_quote)

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
