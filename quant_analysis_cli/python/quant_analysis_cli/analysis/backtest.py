"""Day-trade backtest engine."""

FEE_RATE = 0.001425           # buy + sell
TAX_RATE = 0.0015             # day-trade tax
TOTAL_COST_RATE = FEE_RATE * 2 + TAX_RATE  # ~0.435% round-trip


def backtest_daytrade(df, states, best_state: int) -> dict:
    """Go long only on days in best_state, close at EOD."""
    n = min(len(df), len(states))
    trades = []
    for i in range(n):
        if states[i] == best_state:
            entry = float(df.iloc[i]["open"])
            exit_ = float(df.iloc[i]["close"])
            if entry <= 0:
                continue
            pnl = (exit_ - entry) / entry * 100
            cost = TOTAL_COST_RATE * 100
            trades.append({
                "date": str(df.index[i].date()),
                "entry": round(entry, 2), "exit": round(exit_, 2),
                "gross_pct": round(pnl, 3),
                "cost_pct": round(cost, 3),
                "net_pct": round(pnl - cost, 3),
            })

    if not trades:
        return {"total_trades": 0, "win_rate": 0, "total_net_pct": 0}

    wins = [t for t in trades if t["net_pct"] > 0]
    total_net = sum(t["net_pct"] for t in trades)
    avg_net = total_net / len(trades)
    avg_gross = sum(t["gross_pct"] for t in trades) / len(trades)

    return {
        "total_trades": len(trades),
        "win_rate": round(len(wins) / len(trades) * 100, 1),
        "avg_gross_ret_pct": round(avg_gross, 4),
        "avg_net_ret_pct": round(avg_net, 4),
        "total_net_pct": round(total_net, 2),
        "cost_per_trade_pct": round(TOTAL_COST_RATE * 100, 4),
        "last_5_trades": trades[-5:],
    }
