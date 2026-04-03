"""analyze1m: HMM regime analysis on 1-minute bars (doc-08 taxonomy).

Output JSON:
  symbol, interval, bars_trained,
  current_state, current_label, current_color,
  state_info: [...],
  timeline: [{ts, state}, ...]   ← aligned with kline_1min timestamps
"""

import json
import sys

from ..db import get_connection, read_kline_1min
from ..analysis.features import build_features_1m
from ..analysis.hmm import fit_hmm, describe_states_1m

MIN_BARS = 100  # minimum 1-min bars needed to train HMM


def run_analyze1m(args):
    symbol = args.symbols[0].upper()
    conn = get_connection(args.db)

    # 1. Read 1-min bars from DB
    try:
        df_raw = read_kline_1min(conn, symbol)
    except RuntimeError as e:
        json.dump({"error": str(e)}, sys.stdout, ensure_ascii=False)
        print()
        sys.exit(1)

    if len(df_raw) < MIN_BARS:
        json.dump({
            "error": f"Not enough 1-min data for {symbol}: {len(df_raw)} bars "
                     f"(need {MIN_BARS}+). Run: python3 -m quant_analysis_cli fetch-1m {symbol} --store"
        }, sys.stdout, ensure_ascii=False)
        print()
        sys.exit(1)

    print(f"[1/3] Building 1-min features ({len(df_raw)} bars)...", file=sys.stderr)

    # 2. Build 1-min features (log_ret, range_pct, vol_ratio, close_pos)
    features, df = build_features_1m(df_raw)

    print(f"[2/3] Fitting GaussianHMM (n_states={args.states})...", file=sys.stderr)

    # 3. Fit HMM (always fresh — fast on 1m data, deterministic seed)
    model = fit_hmm(features, n_states=args.states)
    states = model.predict(features)
    state_info = describe_states_1m(model, df, states)

    print(f"[3/3] Building timeline...", file=sys.stderr)

    # 4. Build timeline: list of {ts, state} aligned to df index
    timestamps = [int(t.timestamp()) for t in df.index]
    timeline = [{"ts": ts, "state": int(s)} for ts, s in zip(timestamps, states)]

    # 5. Find current state's label/color
    current_state = int(states[-1])
    current_label = "NOISE 雜訊震盪"
    current_color = "#6b7280"
    for si in state_info:
        if si["state"] == current_state:
            current_label = si["label"]
            current_color = si["color"]
            break

    result = {
        "symbol": symbol,
        "interval": "1m",
        "bars_trained": len(df),
        "current_state": current_state,
        "current_label": current_label,
        "current_color": current_color,
        "state_info": state_info,
        "timeline": timeline,
    }

    json.dump(result, sys.stdout, ensure_ascii=False)
    print()
