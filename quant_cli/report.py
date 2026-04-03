"""Generate standalone HTML reports with inline ECharts visualization."""

import json
from datetime import datetime


STATE_COLORS = {
    "BULL_QUIET": "#22c55e",
    "BULL_VOLATILE": "#eab308",
    "BEAR_QUIET": "#f97316",
    "BEAR_VOLATILE": "#ef4444",
    "CHOPPY": "#8b949e",
}

STATE_BG = {
    "BULL_QUIET": "regime-bull",
    "BULL_VOLATILE": "regime-choppy",
    "BEAR_QUIET": "regime-choppy",
    "BEAR_VOLATILE": "regime-bear",
    "CHOPPY": "regime-choppy",
}

INDICATOR_TAGS = {
    "OVERBOUGHT": "tag-red",
    "OVERSOLD": "tag-green",
    "NEUTRAL": "tag-blue",
    "BULLISH": "tag-green",
    "BEARISH": "tag-red",
    "ABOVE UPPER": "tag-red",
    "BELOW LOWER": "tag-green",
    "WITHIN BANDS": "tag-blue",
}


def _state_short_label(label: str) -> str:
    """Extract the short state name (e.g. BULL_QUIET from 'BULL_QUIET  低波動上漲')."""
    return label.split()[0] if label else "UNKNOWN"


def _state_color_for_label(label: str) -> str:
    short = _state_short_label(label)
    for key, color in STATE_COLORS.items():
        if key in short:
            return color
    return "#8b949e"


def generate_report(symbol: str, name: str, df, states, state_info: list,
                    current_state: int, current_label: str,
                    rsi: float, macd: float, signal: float, hist: float,
                    bb_upper: float, bb_mid: float, bb_lower: float,
                    backtest: dict, bic: float = 0, score: float = 0) -> str:
    """Generate a standalone HTML report string.

    All data is inlined — the HTML file is self-contained and can be opened offline.
    """
    price = float(df["close"].iloc[-1])
    now = datetime.now().strftime("%Y-%m-%d")
    short_label = _state_short_label(current_label)
    regime_class = STATE_BG.get(short_label, "regime-choppy")
    regime_color = _state_color_for_label(current_label)

    # Build kline data array
    kline_rows = []
    for i in range(len(df)):
        row = df.iloc[i]
        st = int(states[i]) if i < len(states) else -1
        kline_rows.append({
            "date": str(row.name.date()) if hasattr(row.name, 'date') else str(row.name),
            "open": round(float(row["open"]), 2),
            "high": round(float(row["high"]), 2),
            "low": round(float(row["low"]), 2),
            "close": round(float(row["close"]), 2),
            "volume": int(row["volume"]),
            "state": st,
        })
    kline_json = json.dumps(kline_rows, ensure_ascii=False)

    # State distribution bar
    bar_segments = []
    for si in state_info:
        short = _state_short_label(si["label"])
        color = STATE_COLORS.get(short, "#8b949e")
        bar_segments.append(
            f'<div style="width:{si["pct"]}%;background:{color}" '
            f'title="{short}">{si["state"]}</div>'
        )
    bar_html = "\n        ".join(bar_segments)

    # State table rows
    state_rows = []
    for si in state_info:
        short = _state_short_label(si["label"])
        color = _state_color_for_label(si["label"])
        ret_cls = "win" if si["avg_daily_ret_pct"] > 0 else "loss"
        state_rows.append(
            f'<tr>'
            f'<td>{si["state"]}</td>'
            f'<td style="color:{color}">{short}</td>'
            f'<td>{si["count"]}</td>'
            f'<td class="{ret_cls}">{"+" if si["avg_daily_ret_pct"] > 0 else ""}'
            f'{si["avg_daily_ret_pct"]:.2f}%</td>'
            f'</tr>'
        )
    state_table = "\n        ".join(state_rows)

    # Current state info
    cs_info = next((s for s in state_info if s["state"] == current_state), None)

    # Indicator signals
    rsi_sig = "OVERSOLD" if rsi < 30 else "OVERBOUGHT" if rsi > 70 else "NEUTRAL"
    macd_sig = "BULLISH" if hist > 0 else "BEARISH"
    bb_pos = ("ABOVE UPPER" if price > bb_upper else
              "BELOW LOWER" if price < bb_lower else "WITHIN BANDS")
    rsi_tag = INDICATOR_TAGS.get(rsi_sig, "tag-blue")
    macd_tag = INDICATOR_TAGS.get(macd_sig, "tag-blue")
    bb_tag = INDICATOR_TAGS.get(bb_pos, "tag-blue")

    # Backtest section
    best_state = state_info[0] if state_info else {}
    bt_trades = backtest.get("last_5_trades", [])
    trade_rows = []
    for t in bt_trades:
        cls = "win" if t["net_pct"] > 0 else "loss"
        sign = "+" if t["net_pct"] > 0 else ""
        trade_rows.append(
            f'<tr><td>{t["date"]}</td><td>{t["entry"]}</td>'
            f'<td>{t["exit"]}</td><td class="{cls}">{sign}{t["net_pct"]:.2f}%</td></tr>'
        )
    trade_table = "\n        ".join(trade_rows)

    # Recommendation
    rec_class = "rec-bull" if "BULL" in current_label else (
        "rec-bear" if "BEAR" in current_label else "rec-choppy"
    )
    if "BULL" in current_label:
        rec_text = (
            f'<div style="font-weight:700;color:#22c55e;margin-bottom:4px">'
            f'>> BULLISH — 考慮當沖做多</div>'
            f'Scalp intraday moves, close before 13:30<br>'
            f'Hard stop: ALL positions by 13:20'
        )
    elif "BEAR" in current_label:
        rec_text = (
            f'<div style="font-weight:700;color:#ef4444;margin-bottom:4px">'
            f'>> BEARISH — 觀望或考慮做空</div>'
            f'High risk regime, avoid long positions<br>'
            f'Hard stop: ALL positions by 13:20'
        )
    else:
        rec_text = (
            f'<div style="font-weight:700;color:#eab308;margin-bottom:4px">'
            f'>> CHOPPY — 震盪盤整，觀望為宜</div>'
            f'Whipsaw risk, reduce position size<br>'
            f'Hard stop: ALL positions by 13:20'
        )

    # State color map for JS
    unique_states = set(s["state"] for s in state_info)
    state_color_map = {}
    for si in state_info:
        short = _state_short_label(si["label"])
        state_color_map[si["state"]] = _state_color_for_label(si["label"])
    state_color_map[-1] = "#8b949e"
    sc_map_js = json.dumps(state_color_map)

    # State label map for JS
    state_label_map = {}
    for si in state_info:
        state_label_map[si["state"]] = _state_short_label(si["label"])
    state_label_map[-1] = "N/A"
    sl_map_js = json.dumps(state_label_map)

    html = f"""<!DOCTYPE html>
<html lang="zh-TW">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{symbol} {name} — HMM Regime Analysis</title>
<script src="https://cdn.jsdelivr.net/npm/echarts@5/dist/echarts.min.js"></script>
<style>
* {{ margin:0; padding:0; box-sizing:border-box; }}
body {{ background:#0d1117; color:#e6edf3; font-family:'SF Mono',Menlo,Consolas,monospace; }}
.grid {{ display:grid; grid-template-columns:1fr 340px; gap:16px; padding:16px; max-width:1440px; margin:auto; }}
.card {{ background:#161b22; border:1px solid #30363d; border-radius:8px; padding:16px; }}
h1 {{ font-size:20px; font-weight:700; margin-bottom:4px; }}
.subtitle {{ color:#8b949e; font-size:13px; margin-bottom:12px; }}
#chart-main {{ height:520px; }}
.sidebar {{ display:flex; flex-direction:column; gap:12px; }}
.indicator-row {{ display:flex; justify-content:space-between; padding:4px 0; border-bottom:1px solid #21262d; font-size:13px; }}
.indicator-label {{ color:#8b949e; }}
.indicator-value {{ font-weight:600; }}
.tag {{ display:inline-block; padding:2px 8px; border-radius:4px; font-size:11px; font-weight:600; }}
.tag-green {{ background:#22c55e22; color:#22c55e; border:1px solid #22c55e44; }}
.tag-red {{ background:#ef444422; color:#ef4444; border:1px solid #ef444444; }}
.tag-yellow {{ background:#eab30822; color:#eab308; border:1px solid #eab30844; }}
.tag-blue {{ background:#2563eb22; color:#2563eb; border:1px solid #2563eb44; }}
.regime-box {{ text-align:center; padding:12px; border-radius:8px; margin:4px 0; }}
.regime-bull {{ background:#22c55e15; border:1px solid #22c55e44; }}
.regime-bear {{ background:#ef444415; border:1px solid #ef444444; }}
.regime-choppy {{ background:#eab30815; border:1px solid #eab30844; }}
.state-bar {{ display:flex; height:24px; border-radius:4px; overflow:hidden; margin:8px 0; }}
.state-bar div {{ display:flex; align-items:center; justify-content:center; font-size:10px; font-weight:600; }}
table {{ width:100%; border-collapse:collapse; font-size:12px; }}
th {{ text-align:left; color:#8b949e; padding:6px 8px; border-bottom:1px solid #30363d; }}
td {{ padding:6px 8px; border-bottom:1px solid #21262d; }}
.win {{ color:#22c55e; }}
.loss {{ color:#ef4444; }}
.rec-box {{ padding:12px; border-radius:8px; margin-top:8px; font-size:13px; line-height:1.6; }}
.rec-bull {{ background:#22c55e10; border:1px solid #22c55e33; }}
.rec-bear {{ background:#ef444410; border:1px solid #ef444433; }}
.rec-choppy {{ background:#eab30810; border:1px solid #eab30833; }}
.full-width {{ grid-column:1/-1; }}
@media(max-width:900px) {{ .grid {{ grid-template-columns:1fr; }} }}
</style>
</head>
<body>
<div class="grid">
  <!-- Header -->
  <div class="card full-width" style="display:flex;justify-content:space-between;align-items:center">
    <div>
      <h1>{symbol} — {name}</h1>
      <div class="subtitle">HMM Regime Analysis | Source: yfinance | Generated: {now}</div>
    </div>
    <div style="text-align:right">
      <div style="font-size:28px;font-weight:700">{price:,.2f}</div>
      <div class="subtitle">Latest Close</div>
    </div>
  </div>

  <!-- Main Chart -->
  <div class="card">
    <div style="font-size:14px;font-weight:600;margin-bottom:8px">K-Line + HMM Regime Overlay</div>
    <div id="chart-main"></div>
  </div>

  <!-- Sidebar -->
  <div class="sidebar">
    <!-- Current Regime -->
    <div class="card">
      <div style="font-size:12px;color:#8b949e;margin-bottom:8px">CURRENT REGIME</div>
      <div class="regime-box {regime_class}">
        <div style="font-size:22px;font-weight:700;color:{regime_color}">{short_label}</div>
        <div style="font-size:13px;color:{regime_color}aa">{current_label.split()[-1] if current_label else ""}</div>
      </div>
      <div style="text-align:center;margin-top:6px;font-size:12px;color:#8b949e">
        State {current_state} | {cs_info['pct'] if cs_info else 0}% of days |
        Avg Ret {"+" if (cs_info or {}).get("avg_daily_ret_pct", 0) > 0 else ""}{(cs_info or {}).get("avg_daily_ret_pct", 0):.2f}%
      </div>
    </div>

    <!-- State Distribution -->
    <div class="card">
      <div style="font-size:12px;color:#8b949e;margin-bottom:6px">STATE DISTRIBUTION ({len(state_info)} states)</div>
      <div class="state-bar">
        {bar_html}
      </div>
      <table style="margin-top:4px">
        <tr><th>State</th><th>Label</th><th>Days</th><th>Avg Ret</th></tr>
        {state_table}
      </table>
    </div>

    <!-- Technical Indicators -->
    <div class="card">
      <div style="font-size:12px;color:#8b949e;margin-bottom:6px">TECHNICAL INDICATORS</div>
      <div class="indicator-row">
        <span class="indicator-label">RSI(14)</span>
        <span>{rsi:.1f} <span class="tag {rsi_tag}">{rsi_sig}</span></span>
      </div>
      <div class="indicator-row">
        <span class="indicator-label">MACD</span>
        <span>{macd:.2f}</span>
      </div>
      <div class="indicator-row">
        <span class="indicator-label">Signal</span>
        <span>{signal:.2f}</span>
      </div>
      <div class="indicator-row">
        <span class="indicator-label">Histogram</span>
        <span style="color:{'#22c55e' if hist > 0 else '#ef4444'}">{hist:+.2f} <span class="tag {macd_tag}">{macd_sig}</span></span>
      </div>
      <div class="indicator-row">
        <span class="indicator-label">Bollinger</span>
        <span>{bb_lower:.0f} / {bb_mid:.0f} / {bb_upper:.0f}</span>
      </div>
      <div class="indicator-row">
        <span class="indicator-label">BB Position</span>
        <span><span class="tag {bb_tag}">{bb_pos}</span></span>
      </div>
    </div>

    <!-- Backtest -->
    <div class="card">
      <div style="font-size:12px;color:#8b949e;margin-bottom:6px">BACKTEST (Long in best regime)</div>
      <div class="indicator-row">
        <span class="indicator-label">Best State</span>
        <span style="color:{_state_color_for_label(best_state.get('label', ''))}">{best_state.get('state', '?')} ({_state_short_label(best_state.get('label', ''))})</span>
      </div>
      <div class="indicator-row">
        <span class="indicator-label">Trades</span>
        <span style="font-weight:700">{backtest.get('total_trades', 0)}</span>
      </div>
      <div class="indicator-row">
        <span class="indicator-label">Win Rate</span>
        <span class="{'win' if backtest.get('win_rate', 0) >= 50 else 'loss'}" style="font-weight:700">{backtest.get('win_rate', 0)}%</span>
      </div>
      <div class="indicator-row">
        <span class="indicator-label">Avg Net (after fees)</span>
        <span class="{'win' if backtest.get('avg_net_ret_pct', 0) > 0 else 'loss'}">{"+" if backtest.get('avg_net_ret_pct', 0) > 0 else ""}{backtest.get('avg_net_ret_pct', 0):.3f}%</span>
      </div>
      <div class="indicator-row">
        <span class="indicator-label">Total P&L</span>
        <span class="{'win' if backtest.get('total_net_pct', 0) > 0 else 'loss'}" style="font-weight:700">{"+" if backtest.get('total_net_pct', 0) > 0 else ""}{backtest.get('total_net_pct', 0):.2f}%</span>
      </div>
      <div class="indicator-row">
        <span class="indicator-label">Cost/trade</span>
        <span>{backtest.get('cost_per_trade_pct', 0):.3f}%</span>
      </div>
      <table style="margin-top:8px">
        <tr><th>Date</th><th>Entry</th><th>Exit</th><th>Net</th></tr>
        {trade_table}
      </table>
    </div>

    <!-- Recommendation -->
    <div class="card">
      <div class="rec-box {rec_class}">
        {rec_text}
      </div>
    </div>
  </div>
</div>

<script>
// Inline data
const klineData = {kline_json};
const stateColors = {sc_map_js};
const stateLabels = {sl_map_js};

const dates = klineData.map(d => d.date);
const ohlc = klineData.map(d => [d.open, d.close, d.low, d.high]);
const volumes = klineData.map(d => d.volume);

const chart = echarts.init(document.getElementById('chart-main'), null, {{ renderer: 'canvas' }});

const option = {{
  backgroundColor: 'transparent',
  animation: false,
  tooltip: {{
    trigger: 'axis',
    axisPointer: {{ type: 'cross' }},
    backgroundColor: '#161b22ee',
    borderColor: '#30363d',
    textStyle: {{ color: '#e6edf3', fontSize: 12 }},
    formatter: function(params) {{
      const k = params[0];
      if (!k) return '';
      const idx = k.dataIndex;
      const d = klineData[idx];
      const st = d.state;
      return '<b>' + d.date + '</b><br/>' +
        'Open: ' + d.open + '  Close: ' + d.close + '<br/>' +
        'High: ' + d.high + '  Low: ' + d.low + '<br/>' +
        'Vol: ' + (d.volume/1e6).toFixed(1) + 'M<br/>' +
        '<span style="color:' + (stateColors[st] || '#8b949e') + '">State ' + st + ': ' + (stateLabels[st] || 'N/A') + '</span>';
    }}
  }},
  axisPointer: {{ link: [{{ xAxisIndex: 'all' }}] }},
  grid: [
    {{ left: '8%', right: '4%', top: '4%', height: '58%' }},
    {{ left: '8%', right: '4%', top: '72%', height: '16%' }}
  ],
  xAxis: [
    {{ type: 'category', data: dates, gridIndex: 0, axisLine: {{ lineStyle: {{ color: '#30363d' }} }}, axisLabel: {{ show: false }} }},
    {{ type: 'category', data: dates, gridIndex: 1, axisLine: {{ lineStyle: {{ color: '#30363d' }} }}, axisLabel: {{ color: '#8b949e', fontSize: 10 }} }}
  ],
  yAxis: [
    {{ type: 'value', gridIndex: 0, scale: true, splitLine: {{ lineStyle: {{ color: '#21262d' }} }}, axisLabel: {{ color: '#8b949e' }} }},
    {{ type: 'value', gridIndex: 1, splitLine: {{ lineStyle: {{ color: '#21262d' }} }}, axisLabel: {{ color: '#8b949e', formatter: function(v) {{ return (v/1e6).toFixed(0)+'M'; }} }} }}
  ],
  dataZoom: [
    {{ type: 'inside', xAxisIndex: [0, 1], start: 0, end: 100 }},
    {{ type: 'slider', xAxisIndex: [0, 1], bottom: '2%', height: 16, borderColor: '#30363d', fillerColor: '#2563eb33', handleStyle: {{ color: '#2563eb' }}, textStyle: {{ color: '#8b949e' }} }}
  ],
  series: [
    {{
      name: 'K-Line',
      type: 'candlestick',
      xAxisIndex: 0,
      yAxisIndex: 0,
      data: ohlc,
      itemStyle: {{
        color: '#ef4444',
        color0: '#22c55e',
        borderColor: '#ef4444',
        borderColor0: '#22c55e'
      }}
    }},
    {{
      name: 'Volume',
      type: 'bar',
      xAxisIndex: 1,
      yAxisIndex: 1,
      data: volumes.map(function(v, i) {{
        return {{
          value: v,
          itemStyle: {{ color: klineData[i].close >= klineData[i].open ? '#ef444466' : '#22c55e66' }}
        }};
      }})
    }}
  ]
}};

chart.setOption(option);
window.addEventListener('resize', function() {{ chart.resize(); }});
</script>
</body>
</html>"""
    return html
