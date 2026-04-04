// Stock AI — ECharts Frontend v3
// Candlestick + Volume + Indicator Subplots + Watchlist + Signals

import * as echarts from "echarts";
(window as any).echarts = echarts;

// ── globals ──────────────────────────────────────────────────────────────
let chart: any = null;
let currentSymbol = "";
let currentData: any[] = [];
let currentDays = 7;
const activeIndicators = new Set<string>(); // "rsi" | "macd" | "boll"
let currentSignals: any[] = [];

// 1m live view
let viewMode: "daily" | "1m" = "daily";
let liveTimer: ReturnType<typeof setInterval> | null = null;

// ── init chart ────────────────────────────────────────────────────────────
function initChart() {
  const el = document.getElementById("chart")!;
  chart = echarts.init(el, "dark");
  const ro = new ResizeObserver(() => chart.resize());
  ro.observe(el);
}

// ── indicator calculations (client-side) ──────────────────────────────────
function calcRSI(closes: number[], period = 14): (number | null)[] {
  const result: (number | null)[] = new Array(closes.length).fill(null);
  if (closes.length <= period) return result;
  let gains = 0, losses = 0;
  for (let i = 1; i <= period; i++) {
    const d = closes[i] - closes[i - 1];
    if (d > 0) gains += d; else losses -= d;
  }
  let avgGain = gains / period, avgLoss = losses / period;
  result[period] = avgLoss === 0 ? 100 : 100 - 100 / (1 + avgGain / avgLoss);
  for (let i = period + 1; i < closes.length; i++) {
    const d = closes[i] - closes[i - 1];
    avgGain = (avgGain * (period - 1) + Math.max(d, 0)) / period;
    avgLoss = (avgLoss * (period - 1) + Math.max(-d, 0)) / period;
    result[i] = avgLoss === 0 ? 100 : 100 - 100 / (1 + avgGain / avgLoss);
  }
  return result;
}

function calcEMA(vals: number[], period: number): number[] {
  const k = 2 / (period + 1);
  const out = [vals[0]];
  for (let i = 1; i < vals.length; i++) out.push(vals[i] * k + out[i - 1] * (1 - k));
  return out;
}

function calcMACD(closes: number[]) {
  const ema12 = calcEMA(closes, 12), ema26 = calcEMA(closes, 26);
  const macd = ema12.map((v, i) => v - ema26[i]);
  const signal = calcEMA(macd, 9);
  const hist = macd.map((v, i) => v - signal[i]);
  return { macd, signal, hist };
}

function calcBB(closes: number[], period = 20, mult = 2) {
  const upper: (number | null)[] = [], mid: (number | null)[] = [], lower: (number | null)[] = [];
  for (let i = 0; i < closes.length; i++) {
    if (i < period - 1) { upper.push(null); mid.push(null); lower.push(null); continue; }
    const sl = closes.slice(i - period + 1, i + 1);
    const mean = sl.reduce((a, b) => a + b, 0) / period;
    const std = Math.sqrt(sl.reduce((a, b) => a + (b - mean) ** 2, 0) / period);
    upper.push(+(mean + mult * std).toFixed(4));
    mid.push(+mean.toFixed(4));
    lower.push(+(mean - mult * std).toFixed(4));
  }
  return { upper, mid, lower };
}

// ── dynamic grid layout ───────────────────────────────────────────────────
function getGridLayout(subCount: number) {
  if (subCount === 0) return [
    { left: 60, right: 20, top: 30, height: "57%" },
    { left: 60, right: 20, top: "72%", height: "18%" },
  ];
  if (subCount === 1) return [
    { left: 60, right: 20, top: 30, height: "43%" },
    { left: 60, right: 20, top: "57%", height: "11%" },
    { left: 60, right: 20, top: "71%", height: "16%" },
  ];
  return [
    { left: 60, right: 20, top: 30, height: "32%" },
    { left: 60, right: 20, top: "46%", height: "10%" },
    { left: 60, right: 20, top: "59%", height: "14%" },
    { left: 60, right: 20, top: "76%", height: "13%" },
  ];
}

// ── chart rendering ───────────────────────────────────────────────────────
async function loadStock(symbol: string, days: number) {
  const overlay = document.getElementById("overlay")!;
  overlay.textContent = `Loading ${symbol}...`;
  overlay.style.display = "flex";
  chart.clear();

  // Yahoo interval constraints: 1m≤7d, 5m/15m/30m≤60d, 60m≤730d, 1d unlimited
  const interval = days <= 30 ? "15m" : days <= 90 ? "60m" : "1d";

  try {
    const resp = await fetch(`/api/history/${encodeURIComponent(symbol)}?days=${days}&interval=${interval}`);
    const data = await resp.json();
    if (!data.bars || data.bars.length === 0) {
      overlay.textContent = "No data for " + symbol;
      return;
    }

    currentSymbol = symbol;
    currentData = data.bars;

    await loadStats(symbol);
    await loadSignals(symbol);
    if (viewMode === "1m") {
      await refresh1mView();
    } else {
      renderChart(data.bars);
    }

    overlay.style.display = "none";
  } catch (err) {
    overlay.textContent = "Error: " + (err as Error).message;
  }
}

function renderChart(bars: any[]) {
  // Detect if bars are intraday (multiple bars per day)
  const daySet = new Set(bars.map(b => new Date(b.time * 1000).toISOString().slice(0, 10)));
  const isIntraday = daySet.size > 0 && bars.length / daySet.size > 2;
  const dates = bars.map(b => {
    const d = new Date(b.time * 1000);
    if (isIntraday) {
      return `${d.getMonth()+1}/${d.getDate()} ${String(d.getHours()).padStart(2,"0")}:${String(d.getMinutes()).padStart(2,"0")}`;
    }
    return d.toISOString().slice(0, 10);
  });
  const ohlc = bars.map(b => [b.open, b.close, b.low, b.high]);
  const volumes = bars.map(b => b.volume);
  const closes = bars.map(b => b.close);

  const hasRSI = activeIndicators.has("rsi");
  const hasMACD = activeIndicators.has("macd");
  const hasBoll = activeIndicators.has("boll");
  const subCount = (hasRSI ? 1 : 0) + (hasMACD ? 1 : 0);

  const grids = getGridLayout(subCount);

  // xAxis per grid — labels only on bottom grid
  const xAxis: any[] = grids.map((_, i) => ({
    type: "category", data: dates, gridIndex: i,
    axisLabel: { show: i === grids.length - 1, color: "#556" },
    axisTick: { show: false }, axisLine: { lineStyle: { color: "#21242e" } },
  }));

  // yAxis — candle (0), volume (1), then subplots
  const yAxis: any[] = [
    { type: "value", gridIndex: 0, scale: true, splitLine: { lineStyle: { color: "#1a1c22" } }, axisLabel: { color: "#778" } },
    { type: "value", gridIndex: 1, splitLine: { show: false }, axisLabel: { show: false } },
  ];

  // Base series (no annotations — signals only on 1D day-trading mode)
  const series: any[] = [
    {
      name: "K线", type: "candlestick", xAxisIndex: 0, yAxisIndex: 0,
      itemStyle: { color: "#ef4444", color0: "#22c55e", borderColor: "#ef4444", borderColor0: "#22c55e" },
      data: ohlc,
    },
    {
      name: "成交量", type: "bar", xAxisIndex: 1, yAxisIndex: 1,
      data: volumes.map((v, i) => ({
        value: v,
        itemStyle: { color: ohlc[i][1] >= ohlc[i][0] ? "#ef444466" : "#22c55e66" },
      })),
    },
  ];

  // Bollinger Bands on main chart (no extra grid needed)
  if (hasBoll) {
    const bb = calcBB(closes);
    const bbLine = (color: string, dash?: boolean) => ({
      type: "line", xAxisIndex: 0, yAxisIndex: 0,
      symbol: "none", lineStyle: { color, width: 1, type: dash ? "dashed" : "solid" },
    });
    series.push(
      { ...bbLine("#5b8def88"), name: "BB上", data: bb.upper },
      { ...bbLine("#5b8defaa", true), name: "BB中", data: bb.mid },
      { ...bbLine("#5b8def88"), name: "BB下", data: bb.lower },
    );
  }

  const dzXIdx = [0, 1];
  let nextGrid = 2;

  // RSI subplot
  if (hasRSI) {
    const gi = nextGrid++;
    dzXIdx.push(gi);
    yAxis.push({
      type: "value", gridIndex: gi, min: 0, max: 100, splitNumber: 2,
      splitLine: { lineStyle: { color: "#1a1c22" } },
      axisLabel: { color: "#556", fontSize: 10 },
    });
    series.push({
      name: "RSI(14)", type: "line", xAxisIndex: gi, yAxisIndex: gi,
      data: calcRSI(closes), symbol: "none",
      lineStyle: { color: "#f59e0b", width: 1.5 },
      markLine: {
        silent: true, symbol: "none",
        lineStyle: { color: "#f59e0b30", type: "dashed" },
        data: [{ yAxis: 70 }, { yAxis: 30 }],
      },
    });
  }

  // MACD subplot
  if (hasMACD) {
    const gi = nextGrid++;
    dzXIdx.push(gi);
    const { macd, signal, hist } = calcMACD(closes);
    yAxis.push({
      type: "value", gridIndex: gi, scale: true, splitNumber: 2,
      splitLine: { lineStyle: { color: "#1a1c22" } },
      axisLabel: { color: "#556", fontSize: 10 },
    });
    series.push(
      {
        name: "MACD", type: "line", xAxisIndex: gi, yAxisIndex: gi,
        data: macd, symbol: "none", lineStyle: { color: "#5b8def", width: 1.5 },
      },
      {
        name: "Signal", type: "line", xAxisIndex: gi, yAxisIndex: gi,
        data: signal, symbol: "none", lineStyle: { color: "#f59e0b", width: 1, type: "dashed" },
      },
      {
        name: "Hist", type: "bar", xAxisIndex: gi, yAxisIndex: gi,
        data: hist.map(v => ({ value: +v.toFixed(4), itemStyle: { color: v >= 0 ? "#22c55e88" : "#ef444488" } })),
      },
    );
  }

  chart.setOption({
    backgroundColor: "transparent",
    animation: false,
    tooltip: { trigger: "axis", axisPointer: { type: "cross" } },
    legend: {
      data: ["K线", "成交量", ...(hasBoll ? ["BB上", "BB中", "BB下"] : []),
        ...(hasRSI ? ["RSI(14)"] : []), ...(hasMACD ? ["MACD", "Signal", "Hist"] : [])],
      top: 0, textStyle: { color: "#778" },
      selected: { "BB中": false, "BB下": false, "Signal": false, "Hist": false },
    },
    grid: grids,
    xAxis,
    yAxis,
    dataZoom: [
      { type: "inside", xAxisIndex: dzXIdx, start: 0, end: 100 },
      { type: "slider", xAxisIndex: dzXIdx, bottom: 10, height: 20, borderColor: "#21242e", fillerColor: "rgba(91,141,239,0.15)", handleStyle: { color: "#5b8def" } },
    ],
    series,
  }, true);
}

async function loadStats(symbol: string) {
  try {
    const [qResp, iResp] = await Promise.all([
      fetch(`/api/quote/${encodeURIComponent(symbol)}`),
      fetch(`/api/indicators/${encodeURIComponent(symbol)}`),
    ]);
    const q = await qResp.json();
    const ind = await iResp.json();

    if (q.error) {
      document.getElementById("s-price")!.textContent = "--";
      return;
    }

    const cls = q.change >= 0 ? "up" : "dn";
    const sign = q.change >= 0 ? "+" : "";
    document.getElementById("s-sym")!.textContent = symbol;
    document.getElementById("s-price")!.textContent = q.price?.toFixed(2) ?? "--";
    document.getElementById("s-price")!.className = "stat-value " + cls;
    document.getElementById("s-change")!.innerHTML = `${sign}${(q.change ?? 0).toFixed(2)} (${sign}${(q.change_pct ?? 0).toFixed(2)}%)`;
    document.getElementById("s-change")!.className = "stat-sub " + cls;
    document.getElementById("s-vol")!.textContent = fmtVol(q.volume ?? 0);
    document.getElementById("s-hl")!.textContent = `${(q.high ?? 0).toFixed(2)} / ${(q.low ?? 0).toFixed(2)}`;

    const rsi = ind.rsi_14 ?? 50;
    document.getElementById("s-rsi")!.textContent = rsi.toFixed(1);
    const rsiCls = rsi > 70 ? "dn" : rsi < 30 ? "up" : "";
    document.getElementById("s-rsi")!.className = "stat-value " + rsiCls;

    const hist = ind.macd_hist ?? 0;
    const mc = hist >= 0 ? "up" : "dn";
    document.getElementById("s-macd")!.innerHTML =
      `<span class="${mc}">${(ind.macd ?? 0).toFixed(2)} / ${(ind.macd_signal ?? 0).toFixed(2)}</span>`;
  } catch {
    // ignore
  }
}

// ── signals panel ─────────────────────────────────────────────────────────
async function loadSignals(symbol: string) {
  try {
    const resp = await fetch(`/api/signals/${encodeURIComponent(symbol)}`);
    const data = await resp.json();
    const signals = data.signals ?? [];
    currentSignals = signals;
    const el = document.getElementById("signal-list")!;
    if (signals.length === 0) {
      el.innerHTML = '<div style="color:#445;font-size:11px">No signals yet</div>';
      return;
    }
    el.innerHTML = signals.map((s: any) => {
      const color = s.signal_type === "LONG" ? "#22c55e" : s.signal_type === "SHORT" ? "#ef4444" : "#eab308";
      const bg = s.signal_type === "LONG" ? "#22c55e15" : s.signal_type === "SHORT" ? "#ef444415" : "#eab30815";
      return `<div class="signal-item" style="border-left:3px solid ${color};background:${bg}">
        <span class="signal-type" style="color:${color}">${s.signal_type}</span>
        <span class="signal-date">${s.date}</span>
        <span class="signal-conf">${(s.confidence * 100).toFixed(0)}%</span>
      </div>`;
    }).join("");
  } catch {
    // ignore
  }
}

function fmtVol(v: number): string {
  if (v >= 1_000_000) return (v / 1_000_000).toFixed(1) + "M";
  if (v >= 1_000) return (v / 1_000).toFixed(1) + "K";
  return v.toString();
}

// ── series-level indicator helpers (for 1m day-trading signals) ───────────
function calcRSI(closes: number[], period = 14): (number | null)[] {
  const result: (number | null)[] = new Array(closes.length).fill(null);
  if (closes.length <= period) return result;
  let gains = 0, losses = 0;
  for (let i = 1; i <= period; i++) {
    const d = closes[i] - closes[i - 1];
    if (d > 0) gains += d; else losses -= d;
  }
  let avgGain = gains / period, avgLoss = losses / period;
  result[period] = avgLoss === 0 ? 100 : 100 - 100 / (1 + avgGain / avgLoss);
  for (let i = period + 1; i < closes.length; i++) {
    const d = closes[i] - closes[i - 1];
    avgGain = (avgGain * (period - 1) + Math.max(d, 0)) / period;
    avgLoss = (avgLoss * (period - 1) + Math.max(-d, 0)) / period;
    result[i] = avgLoss === 0 ? 100 : 100 - 100 / (1 + avgGain / avgLoss);
  }
  return result;
}

function calcMACD(closes: number[]) {
  const e12 = calcEMA(closes, 12), e26 = calcEMA(closes, 26);
  const ml: number[] = e12.map((v: number, i: number) => v - e26[i]);
  const sl = calcEMA(ml, 9);
  const hist = ml.map((v: number, i: number) => v - sl[i]);
  return { macd: ml, signal: sl, hist };
}

// ── 1m live view ─────────────────────────────────────────────────────────

function renderChart1m(bars: any[], hmm: any) {
  // bars: [{ts, dt, open, high, low, close, volume}]
  // x-axis: "MM-DD HH:MM" for cross-day readability
  const dates = bars.map(b => b.dt.slice(5, 16).replace("T", " "));
  const ohlc = bars.map(b => [b.open, b.close, b.low, b.high]);
  const volumes = bars.map(b => b.volume);

  // Build ts→index map for regime markArea alignment
  const tsIndex: Record<number, number> = {};
  bars.forEach((b, i) => { tsIndex[b.ts] = i; });

  // Collapse consecutive same-state runs into markArea rectangles
  const markAreaData: any[] = [];
  if (hmm?.timeline && hmm?.state_info) {
    const colorMap: Record<number, string> = {};
    const labelMap: Record<number, string> = {};
    for (const si of (hmm.state_info as any[])) {
      colorMap[si.state] = si.color;
      labelMap[si.state] = si.label;
    }

    // Only mark bars that exist in our chart window
    const tl: { ts: number; state: number }[] = (hmm.timeline as any[])
      .filter(e => e.ts in tsIndex);

    let run = 0;
    while (run < tl.length) {
      const state = tl[run].state;
      const color = colorMap[state] || "#6b7280";
      const label = labelMap[state] || "";
      let end = run;
      while (end + 1 < tl.length && tl[end + 1].state === state) end++;
      if (!label.includes("NOISE")) {
        markAreaData.push([
          { xAxis: dates[tsIndex[tl[run].ts]], itemStyle: { color: color + "28" }, label: { show: false } },
          { xAxis: dates[tsIndex[tl[end].ts]] },
        ]);
      }
      run = end + 1;
    }
  }

  // ── Day-trading buy/sell signal detection on 1-min bars ───────────────────
  const closes1m = bars.map((b: any) => b.close);
  const rsi1m = calcRSI(closes1m) as (number | null)[];
  const macd1m = calcMACD(closes1m);
  const dtBuy: any[] = [];
  const dtSell: any[] = [];

  for (let i = 1; i < bars.length; i++) {
    // RSI cross
    if (rsi1m[i - 1] != null && rsi1m[i] != null) {
      if (rsi1m[i - 1]! >= 30 && rsi1m[i]! < 30)
        dtBuy.push({ coord: [dates[i], ohlc[i][2]], value: "RSI<30" });
      if (rsi1m[i - 1]! <= 70 && rsi1m[i]! > 70)
        dtSell.push({ coord: [dates[i], ohlc[i][3]], value: "RSI>70" });
    }
    // MACD histogram cross
    const mh = macd1m.hist;
    if (mh[i - 1] < 0 && mh[i] >= 0)
      dtBuy.push({ coord: [dates[i], ohlc[i][2]], value: "MACD+" });
    if (mh[i - 1] >= 0 && mh[i] < 0)
      dtSell.push({ coord: [dates[i], ohlc[i][3]], value: "MACD-" });
  }

  const candleSeries: any = {
    name: "1m K", type: "candlestick", xAxisIndex: 0, yAxisIndex: 0,
    itemStyle: { color: "#ef4444", color0: "#22c55e", borderColor: "#ef4444", borderColor0: "#22c55e" },
    data: ohlc,
    markArea: markAreaData.length > 0 ? { silent: true, data: markAreaData } : undefined,
    markPoint: {
      symbol: "arrow", symbolSize: 24,
      label: { fontSize: 8, color: "#fff", formatter: (p: any) => p.value },
      animation: false,
      data: [
        ...dtBuy.map((p: any) => ({ ...p, symbolRotate: 0, itemStyle: { color: "#22c55ecc" } })),
        ...dtSell.map((p: any) => ({ ...p, symbolRotate: 180, itemStyle: { color: "#ef4444cc" } })),
      ],
    },
  };

  chart.setOption({
    backgroundColor: "transparent",
    animation: false,
    tooltip: {
      trigger: "axis", axisPointer: { type: "cross" },
      formatter: (params: any[]) => {
        const p = params.find((x: any) => Array.isArray(x.data));
        if (!p) return "";
        const [o, c, l, h] = p.data as number[];
        const color = c >= o ? "#22c55e" : "#ef4444";
        return `${p.name}<br/>O:${o} H:${h} L:${l} <b style="color:${color}">C:${c}</b>`;
      },
    },
    legend: { show: false },
    grid: [
      { left: 60, right: 20, top: 30, height: "57%" },
      { left: 60, right: 20, top: "72%", height: "18%" },
    ],
    xAxis: [
      { type: "category", data: dates, gridIndex: 0, axisLabel: { show: false }, axisTick: { show: false }, axisLine: { lineStyle: { color: "#21242e" } } },
      { type: "category", data: dates, gridIndex: 1, axisLabel: { color: "#556", fontSize: 9, interval: "auto", rotate: 30 }, axisLine: { lineStyle: { color: "#21242e" } } },
    ],
    yAxis: [
      { type: "value", gridIndex: 0, scale: true, splitLine: { lineStyle: { color: "#1a1c22" } }, axisLabel: { color: "#778" } },
      { type: "value", gridIndex: 1, splitLine: { show: false }, axisLabel: { show: false } },
    ],
    dataZoom: [
      { type: "inside", xAxisIndex: [0, 1], start: 70, end: 100 },
      { type: "slider", xAxisIndex: [0, 1], bottom: 10, height: 20, borderColor: "#21242e", fillerColor: "rgba(91,141,239,0.15)", handleStyle: { color: "#5b8def" } },
    ],
    series: [
      candleSeries,
      {
        name: "成交量", type: "bar", xAxisIndex: 1, yAxisIndex: 1,
        data: volumes.map((v, i) => ({
          value: v,
          itemStyle: { color: ohlc[i][1] >= ohlc[i][0] ? "#ef444466" : "#22c55e66" },
        })),
      },
    ],
  }, true);
}

async function refresh1mView() {
  const symbol = currentSymbol;
  if (!symbol || viewMode !== "1m") return;
  const statusEl = document.getElementById("live-status");
  const regimeEl = document.getElementById("live-regime");
  const statesEl = document.getElementById("live-states");
  if (statusEl) statusEl.textContent = "⟳ Fetching 1m data...";
  // Reset regime badge immediately to prevent stale data from previous stock
  if (regimeEl) { regimeEl.textContent = "⟳"; regimeEl.style.color = "#778"; }
  if (statesEl) statesEl.innerHTML = "";

  // Step 1: fetch fresh bars from yfinance → store
  let fetchErr = "";
  try {
    const r = await fetch(`/api/fetch1m/${encodeURIComponent(symbol)}`, { method: "POST" });
    if (!r.ok) {
      const d = await r.json().catch(() => ({}));
      fetchErr = d.error ?? `HTTP ${r.status}`;
    }
  } catch (e) {
    fetchErr = (e as Error).message;
  }

  // Step 2: get stored 1m bars (7-day window)
  let bars: any[] = [];
  let klineErr = "";
  try {
    const r = await fetch(`/api/kline1m/${encodeURIComponent(symbol)}?hours=168`);
    const d = await r.json();
    if (d.data) {
      bars = d.data;
    } else if (d.error) {
      klineErr = d.error;
    }
  } catch (e) {
    klineErr = (e as Error).message;
  }

  // Step 3: run HMM analysis
  let hmm: any = null;
  if (bars.length > 0) {
    try {
      const r = await fetch(`/api/hmm1m/${encodeURIComponent(symbol)}`);
      hmm = await r.json();
      if (hmm?.error !== undefined || !hmm?.current_label) hmm = null;
    } catch { /* ignore */ }
  }

  // Guard: bail if the user switched to a different stock while we were fetching
  if (currentSymbol !== symbol) return;

  // Step 4: render or show error
  if (bars.length > 0) {
    renderChart1m(bars, hmm);
  } else {
    const msg = fetchErr || klineErr || "No 1m data available";
    if (statusEl) statusEl.textContent = `⚠ ${msg}`;
    return;
  }

  // Update status bar
  const now = new Date().toLocaleTimeString();
  if (statusEl) statusEl.textContent = `Updated ${now}  •  ${bars.length} bars${fetchErr ? "  ⚠ fetch: " + fetchErr : ""}`;

  // Update regime badge (statesEl already declared at top)
  if (regimeEl && hmm) {
    const label = hmm.current_label ?? "NOISE";
    const color = hmm.current_color ?? "#6b7280";
    regimeEl.textContent = label;
    regimeEl.style.background = color + "22";
    regimeEl.style.color = color;
    regimeEl.style.borderColor = color + "55";
    if (statesEl && hmm.state_info) {
      statesEl.innerHTML = (hmm.state_info as any[]).map((s: any) =>
        `<span style="color:${s.color};font-size:10px">${s.label.split(" ")[0]} ${s.pct}%</span>`
      ).join(" · ");
    }
  } else if (regimeEl) {
    // HMM unavailable (not enough bars) — reset badge
    regimeEl.textContent = `-- (${bars.length} bars, need 100+)`;
    regimeEl.style.background = "#6b728018";
    regimeEl.style.color = "#6b7280";
    regimeEl.style.borderColor = "#33444455";
    if (statesEl) statesEl.innerHTML = "";
  }
}

function setLiveLayout(on: boolean) {
  const main = document.getElementById("main")!;
  main.style.top = on ? "72px" : "44px";
}

function toggleLive1m(btn: HTMLButtonElement) {
  const liveBar = document.getElementById("live-bar")!;
  if (viewMode === "1m") {
    // Switch back to daily
    viewMode = "daily";
    btn.classList.remove("active");
    btn.textContent = "1m Live";
    if (liveTimer) { clearInterval(liveTimer); liveTimer = null; }
    liveBar.style.display = "none";
    setLiveLayout(false);
    showSignalLegend(false);
    if (currentSymbol && currentData.length > 0) renderChart(currentData);
  } else {
    // Switch to 1m
    viewMode = "1m";
    btn.classList.add("active");
    btn.textContent = "⏹ 1m Live";
    liveBar.style.display = "flex";
    setLiveLayout(true);
    showSignalLegend(true);
    document.getElementById("live-status")!.textContent = "⟳ Loading...";
    refresh1mView();
    liveTimer = setInterval(() => refresh1mView(), 60_000);
  }
}

function showSignalLegend(on: boolean) {
  const title = document.getElementById("signal-legend-title");
  const legend = document.getElementById("signal-legend");
  if (title) title.style.display = on ? "block" : "none";
  if (legend) legend.style.display = on ? "block" : "none";
}

// ── indicator toggle ───────────────────────────────────────────────────────
function toggleIndicator(name: string, btn: HTMLButtonElement) {
  if (activeIndicators.has(name)) {
    activeIndicators.delete(name);
    btn.classList.remove("active");
  } else {
    activeIndicators.add(name);
    btn.classList.add("active");
  }
  if (currentData.length > 0) renderChart(currentData);
}

// ── backtest ─────────────────────────────────────────────────────────────
async function runBacktest() {
  const symbol = currentSymbol;
  if (!symbol) return;
  const panel = document.getElementById("backtest-panel")!;
  const content = document.getElementById("bt-content")!;
  panel.style.display = panel.style.display === "none" ? "flex" : "none";
  content.innerHTML = '<div style="color:#556">Running HMM analysis...</div>';

  try {
    const resp = await fetch(`/api/backtest/${encodeURIComponent(symbol)}`);
    const data = await resp.json();
    if (data.error) {
      content.innerHTML = `<div style="color:#e84848">Error: ${data.error}</div>`;
      return;
    }

    const st = data.states || [];
    const bt = data.backtest || {};
    content.innerHTML = `
      <div style="margin-bottom:8px;font-weight:600;color:#dde;font-size:14px">${data.name || symbol} — ${data.current_label || "?"}</div>
      <div style="color:#778;font-size:12px;margin-bottom:6px">Close: ${data.close} | State ${data.current_state}</div>
      ${st.map((s: any) => `<div style="font-size:12px;color:#99a;margin:2px 0">State ${s.state}: ${s.label} (${s.pct}%) avg ${s.avg_daily_ret_pct > 0 ? "+" : ""}${s.avg_daily_ret_pct}%</div>`).join("")}
      <div style="margin-top:8px;border-top:1px solid #21242e;padding-top:6px;font-size:12px;color:#99a">
        Trades: ${bt.total_trades ?? 0} | Win: ${bt.win_rate ?? 0}%<br>
        Avg Net: ${((bt.avg_net_ret_pct ?? 0) > 0 ? "+" : "")}${(bt.avg_net_ret_pct ?? 0).toFixed(4)}%<br>
        Total P&L: ${((bt.total_net_pct ?? 0) > 0 ? "+" : "")}${(bt.total_net_pct ?? 0).toFixed(2)}%
      </div>
    `;
  } catch (e) {
    content.innerHTML = `<div style="color:#e84848">Failed: ${(e as Error).message}</div>`;
  }
}

// ── watchlist sidebar ─────────────────────────────────────────────────────
async function loadWatchlist() {
  try {
    const resp = await fetch("/api/watchlist");
    const data = await resp.json();
    const items = data.watchlist ?? [];
    const el = document.getElementById("wl-items")!;
    if (items.length === 0) {
      el.innerHTML = '<div style="color:#445;font-size:11px">Empty — add stocks above</div>';
      return;
    }
    el.innerHTML = items.map((w: any) => {
      const signalColor = w.latest_signal === "LONG" ? "#22c55e" : w.latest_signal === "SHORT" ? "#ef4444" : "transparent";
      return `<div class="wl-item" onclick="loadStockFromWatchlist('${w.symbol}')">
        <span class="wl-sym">${w.symbol}</span>
        <span class="wl-name">${w.name}</span>
        <span class="wl-dot" style="background:${signalColor}"></span>
        <span class="wl-del" onclick="event.stopPropagation();removeWatch('${w.symbol}')">×</span>
      </div>`;
    }).join("");
  } catch {
    // ignore
  }
}

async function addToWatchlist() {
  const input = document.getElementById("wl-add-input") as HTMLInputElement;
  const sym = input.value.trim().toUpperCase();
  if (!sym) return;
  await fetch("/api/watchlist", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ symbol: sym, name: sym }),
  });
  input.value = "";
  loadWatchlist();
}

async function removeWatch(symbol: string) {
  await fetch(`/api/watchlist/${encodeURIComponent(symbol)}`, { method: "DELETE" });
  loadWatchlist();
}

// ── scan all watchlist ────────────────────────────────────────────────────
async function scanAll() {
  const btn = document.getElementById("scan-btn") as HTMLButtonElement;
  btn.textContent = "Scanning...";
  btn.disabled = true;
  try {
    const resp = await fetch("/api/scan", { method: "POST" });
    const data = await resp.json();
    const results = data.scanned ?? [];
    const el = document.getElementById("scan-results")!;
    el.innerHTML = results.map((r: any) => {
      const color = r.success ? "#22c55e" : "#ef4444";
      return `<div style="font-size:11px;color:${color};margin:2px 0">${r.symbol}: ${r.success ? "OK" : "FAIL"}</div>`;
    }).join("");
    loadWatchlist();
    if (currentSymbol) loadSignals(currentSymbol);
  } catch (e) {
    console.error("scan failed", e);
  }
  btn.textContent = "Scan All";
  btn.disabled = false;
}

// ── exposed to HTML ──────────────────────────────────────────────────────
(window as any).setPeriod = (days: number, btn: HTMLButtonElement) => {
  // If in live mode, switch back to daily first
  if (viewMode === "1m") {
    viewMode = "daily";
    const liveBar = document.getElementById("live-bar")!;
    liveBar.style.display = "none";
    setLiveLayout(false);
    showSignalLegend(false);
    if (liveTimer) { clearInterval(liveTimer); liveTimer = null; }
    // Reset 1D button text
    document.querySelectorAll(".period-btn").forEach((b) => {
      if ((b as HTMLButtonElement).textContent?.includes("1D")) (b as HTMLButtonElement).textContent = "1D";
    });
  }
  currentDays = days;
  document.querySelectorAll(".period-btn").forEach((b) => b.classList.remove("active"));
  btn.classList.add("active");
  if (currentSymbol) loadStock(currentSymbol, days);
};

(window as any).toggleLive1d = (btn: HTMLButtonElement) => {
  if (viewMode === "1m") {
    // Already live — switch back to daily
    viewMode = "daily";
    btn.classList.remove("active");
    btn.textContent = "1D";
    const liveBar = document.getElementById("live-bar")!;
    liveBar.style.display = "none";
    setLiveLayout(false);
    showSignalLegend(false);
    if (liveTimer) { clearInterval(liveTimer); liveTimer = null; }
    if (currentSymbol && currentData.length > 0) renderChart(currentData);
    // Reactivate the previously selected period button
    document.querySelectorAll(".period-btn").forEach((b) => {
      if (b !== btn && (b as HTMLButtonElement).textContent !== "1D") {
        const d = parseInt((b as HTMLButtonElement).getAttribute("onclick")?.match(/\d+/)?.[0] || "0");
        if (d === currentDays) b.classList.add("active");
      }
    });
  } else {
    // Switch to live 1m intraday
    viewMode = "1m";
    document.querySelectorAll(".period-btn").forEach((b) => b.classList.remove("active"));
    btn.classList.add("active");
    btn.textContent = "⏹ 1D";
    const liveBar = document.getElementById("live-bar")!;
    liveBar.style.display = "flex";
    setLiveLayout(true);
    showSignalLegend(true);
    document.getElementById("live-status")!.textContent = "⟳ Loading...";
    refresh1mView();
    liveTimer = setInterval(() => refresh1mView(), 60_000);
  }
};

(window as any).toggleIndicator = (name: string, btn: HTMLButtonElement) => toggleIndicator(name, btn);
(window as any).refresh1mNow = () => refresh1mView();

(window as any).runBacktest = runBacktest;

(window as any).openReport = () => {
  if (!currentSymbol) return;
  window.open(`/api/report/${encodeURIComponent(currentSymbol)}`, "_blank");
};

(window as any).toggleWatchlist = () => {
  document.getElementById("watchlist")!.classList.toggle("open");
};

(window as any).addToWatchlist = addToWatchlist;
(window as any).removeWatch = removeWatch;
(window as any).scanAll = scanAll;
(window as any).loadStockFromWatchlist = (symbol: string) => loadStock(symbol, currentDays);

// ── config panel ──────────────────────────────────────────────────────────
async function loadConfig() {
  try {
    const resp = await fetch("/api/config");
    const cfg = await resp.json();
    const sel = document.getElementById("cfg-backend") as HTMLSelectElement;
    if (sel && cfg.fetch_backend) sel.value = cfg.fetch_backend;
  } catch { /* ignore */ }
}

(window as any).toggleConfig = () => {
  document.getElementById("cfg-panel")!.classList.toggle("open");
};

(window as any).setConfig = async (backend: string) => {
  const statusEl = document.getElementById("cfg-status")!;
  try {
    const resp = await fetch("/api/config", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ fetch_backend: backend }),
    });
    const cfg = await resp.json();
    statusEl.textContent = `Saved: ${cfg.fetch_backend}`;
    setTimeout(() => { statusEl.textContent = ""; }, 2000);
  } catch {
    statusEl.textContent = "Failed to save";
  }
};

// ── boot ─────────────────────────────────────────────────────────────────
initChart();
(document.getElementById("wl-add-input") as HTMLInputElement).addEventListener("keydown", (e) => {
  if (e.key === "Enter") addToWatchlist();
});
loadWatchlist();
loadConfig();
loadStock("2330.TW", currentDays);

// ═══════════════════════════════════════════════════════════════════════════
// SCANNER — Day Trading Signal Capture
// ═══════════════════════════════════════════════════════════════════════════

interface ScanRow {
  symbol: string; name: string; last_price: number;
  score: number; direction: string; top_signals: string[];
}

interface DaySig {
  kind: string; direction: string; strength: string;
  score: number; reason: string;
}

interface BarSig {
  idx: number; date: string; signals: DaySig[]; score: number;
}

interface DayTrade {
  symbol: string; bars: any[]; signals: BarSig[];
  latest_score: number; latest_direction: string;
}

let scannerOpen = false;
let scannerSub: "list" | "analysis" | "replay" = "list";
let scanResults: ScanRow[] = [];
let daytradeData: DayTrade | null = null;
let daytradeSymbol = "";
let replayIdx = 0;
let replayTimer: ReturnType<typeof setInterval> | null = null;
let replaySpeed = 1000;
let replayPnl = { trades: 0, wins: 0, totalPct: 0, position: null as null | "long", entryPrice: 0 };

// ── scanner toggle ───────────────────────────────────────────────────────
function toggleScanner() {
  scannerOpen = !scannerOpen;
  const sv = document.getElementById("scanner-view")!;
  const mainEl = document.getElementById("main")!;
  if (scannerOpen) {
    sv.style.display = "flex";
    mainEl.style.display = "none";
    loadScanResults();
  } else {
    sv.style.display = "none";
    mainEl.style.display = "flex";
    if (replayTimer) { clearInterval(replayTimer); replayTimer = null; }
    if (currentSymbol && currentData.length > 0) renderChart(currentData);
  }
}

function setScannerSub(sub: "list" | "analysis" | "replay") {
  scannerSub = sub;
  document.getElementById("sub-list-btn")!.classList.toggle("active", sub === "list");
  document.getElementById("sub-analysis-btn")!.classList.toggle("active", sub === "analysis");
  document.getElementById("sub-replay-btn")!.classList.toggle("active", sub === "replay");
  document.getElementById("scan-table")!.style.display = sub === "list" ? "block" : "none";
  document.getElementById("scanner-chart-wrap")!.style.display = sub !== "list" ? "flex" : "none";
  document.getElementById("replay-controls")!.style.display = sub === "replay" ? "flex" : "none";
  document.getElementById("signal-detail")!.style.display = "none";
  if (sub === "replay" && daytradeData) startReplay();
}

// ── scan list ────────────────────────────────────────────────────────────
async function loadScanResults() {
  const el = document.getElementById("scan-table")!;
  el.innerHTML = '<div style="color:#556;padding:20px;text-align:center">Scanning watchlist...</div>';
  try {
    const resp = await fetch("/api/scan-signals");
    const data = await resp.json();
    scanResults = data.results ?? [];
    if (scanResults.length === 0) {
      el.innerHTML = '<div style="color:#445;padding:20px;text-align:center">No results. Add stocks to watchlist first.</div>';
      return;
    }
    renderScanTable();
  } catch (e) {
    el.innerHTML = `<div style="color:#e84848;padding:20px">Error: ${(e as Error).message}</div>`;
  }
}

function scoreColor(score: number): string {
  if (score >= 30) return "#22c55e";
  if (score >= 10) return "#86efac";
  if (score <= -30) return "#ef4444";
  if (score <= -10) return "#fca5a5";
  return "#778";
}

function renderScanTable() {
  const el = document.getElementById("scan-table")!;
  el.innerHTML = `
    <table style="width:100%;border-collapse:collapse;font-size:12px;">
      <thead><tr style="color:#445;font-size:10px;text-transform:uppercase;border-bottom:1px solid #1e2028">
        <th style="text-align:left;padding:6px 8px">Symbol</th>
        <th style="text-align:left;padding:6px 4px">Name</th>
        <th style="text-align:right;padding:6px 4px">Price</th>
        <th style="text-align:right;padding:6px 4px">Score</th>
        <th style="text-align:center;padding:6px 4px">Signal</th>
        <th style="text-align:left;padding:6px 4px">Top Signals</th>
      </tr></thead>
      <tbody>${scanResults.map((r, i) => `
        <tr style="cursor:pointer;border-bottom:1px solid #13151a;${i % 2 ? "" : "background:#0f1014"}" onclick="openAnalysis('${r.symbol}')">
          <td style="padding:6px 8px;font-weight:600;color:#dde">${r.symbol}</td>
          <td style="padding:6px 4px;color:#556;font-size:11px">${r.name || r.symbol}</td>
          <td style="text-align:right;padding:6px 4px;color:#99a">${r.last_price.toFixed(2)}</td>
          <td style="text-align:right;padding:6px 4px;font-weight:700;color:${scoreColor(r.score)}">${r.score > 0 ? "+" : ""}${r.score}</td>
          <td style="text-align:center;padding:6px 4px">
            <span style="padding:2px 8px;border-radius:10px;font-size:10px;font-weight:600;
              background:${r.direction === "Buy" ? "#22c55e20" : r.direction === "Sell" ? "#ef444420" : "#3333"};
              color:${r.direction === "Buy" ? "#22c55e" : r.direction === "Sell" ? "#ef4444" : "#556"}">${r.direction}</span>
          </td>
          <td style="padding:6px 4px;color:#667;font-size:10px">${r.top_signals.join(", ")}</td>
        </tr>`).join("")}</tbody>
    </table>`;
}

// ── analysis view ────────────────────────────────────────────────────────
async function openAnalysis(symbol: string) {
  scannerSub = "analysis";
  setScannerSub("analysis");
  daytradeSymbol = symbol;

  const wrap = document.getElementById("scanner-chart-wrap")!;
  const existingChart = document.getElementById("scanner-chart")!;
  existingChart.innerHTML = "";

  // Init a second chart for scanner if needed
  let sc = echarts.getInstanceByDom(existingChart) || echarts.init(existingChart, "dark");
  new ResizeObserver(() => sc.resize()).observe(existingChart);

  const overlay = document.createElement("div");
  overlay.textContent = `Loading ${symbol}...`;
  overlay.style.cssText = "position:absolute;inset:0;display:flex;align-items:center;justify-content:center;color:#556;z-index:5";
  wrap.appendChild(overlay);

  try {
    const resp = await fetch(`/api/daytrade/${encodeURIComponent(symbol)}`);
    daytradeData = await resp.json();
    wrap.removeChild(overlay);
    renderAnalysisChart(sc);
  } catch (e) {
    overlay.textContent = `Error: ${(e as Error).message}`;
    overlay.style.color = "#e84848";
  }
}

function renderAnalysisChart(sc: any) {
  if (!daytradeData || !daytradeData.bars.length) return;
  const bars = daytradeData.bars;
  const dates = bars.map((b: any) => {
    const d = new Date(b.time * 1000);
    return d.toISOString().slice(0, 10);
  });
  const ohlc = bars.map((b: any) => [b.open, b.close, b.low, b.high]);
  const closes = bars.map((b: any) => b.close);
  const volumes = bars.map((b: any) => b.volume);

  // Compute BB & EMA client-side for overlay
  const bb = calcBB(closes);
  const ema5 = calcEMA(closes, 5);
  const ema20 = calcEMA(closes, 20);

  // Build signal index map
  const sigMap = new Map<number, BarSig>();
  for (const bs of daytradeData.signals) sigMap.set(bs.idx, bs);

  // Buy/sell markers
  const buyPts: any[] = [];
  const sellPts: any[] = [];
  for (const bs of daytradeData.signals) {
    if (bs.score > 0) {
      buyPts.push({ coord: [dates[bs.idx], ohlc[bs.idx][2]], value: `+${bs.score}`, sigIdx: bs.idx });
    } else if (bs.score < 0) {
      sellPts.push({ coord: [dates[bs.idx], ohlc[bs.idx][3]], value: `${bs.score}`, sigIdx: bs.idx });
    }
  }

  // Volume surge highlight: bars where volume > 2x 20-day avg
  const volColors = volumes.map((v: number, i: number) => {
    if (i < 20) return ohlc[i][1] >= ohlc[i][0] ? "#ef444466" : "#22c55e66";
    const avg = volumes.slice(i - 20, i).reduce((a: number, b: number) => a + b, 0) / 20;
    const isSurge = v > avg * 2;
    if (isSurge) return "#f59e0b88"; // orange for surge
    return ohlc[i][1] >= ohlc[i][0] ? "#ef444466" : "#22c55e66";
  });

  sc.setOption({
    backgroundColor: "transparent",
    animation: false,
    tooltip: { trigger: "axis", axisPointer: { type: "cross" } },
    legend: {
      data: ["K线", "成交量", "BB上", "BB中", "BB下", "EMA5", "EMA20"],
      top: 0, textStyle: { color: "#556", fontSize: 10 },
      selected: { "BB中": false, "BB下": false },
    },
    grid: [
      { left: 60, right: 20, top: 35, height: "58%" },
      { left: 60, right: 20, top: "73%", height: "17%" },
    ],
    xAxis: [
      { type: "category", data: dates, gridIndex: 0, axisLabel: { show: false }, axisTick: { show: false }, axisLine: { lineStyle: { color: "#21242e" } } },
      { type: "category", data: dates, gridIndex: 1, axisLabel: { color: "#556", fontSize: 9 }, axisLine: { lineStyle: { color: "#21242e" } } },
    ],
    yAxis: [
      { type: "value", gridIndex: 0, scale: true, splitLine: { lineStyle: { color: "#1a1c22" } }, axisLabel: { color: "#778" } },
      { type: "value", gridIndex: 1, splitLine: { show: false }, axisLabel: { show: false } },
    ],
    dataZoom: [
      { type: "inside", xAxisIndex: [0, 1], start: 70, end: 100 },
      { type: "slider", xAxisIndex: [0, 1], bottom: 10, height: 18, borderColor: "#21242e", fillerColor: "rgba(91,141,239,0.15)", handleStyle: { color: "#5b8def" } },
    ],
    series: [
      {
        name: "K线", type: "candlestick", xAxisIndex: 0, yAxisIndex: 0,
        itemStyle: { color: "#ef4444", color0: "#22c55e", borderColor: "#ef4444", borderColor0: "#22c55e" },
        data: ohlc,
        markPoint: {
          symbol: "arrow", symbolSize: 20,
          label: { fontSize: 8, color: "#fff", formatter: (p: any) => p.value },
          animation: false,
          data: [
            ...buyPts.map(p => ({ ...p, symbolRotate: 0, symbolSize: 18, itemStyle: { color: "#22c55ecc" } })),
            ...sellPts.map(p => ({ ...p, symbolRotate: 180, symbolSize: 18, itemStyle: { color: "#ef4444cc" } })),
          ],
        },
      },
      {
        name: "成交量", type: "bar", xAxisIndex: 1, yAxisIndex: 1,
        data: volumes.map((v: number, i: number) => ({ value: v, itemStyle: { color: volColors[i] } })),
      },
      {
        name: "BB上", type: "line", xAxisIndex: 0, yAxisIndex: 0,
        data: bb.upper, symbol: "none", lineStyle: { color: "#5b8def55", width: 1 },
      },
      {
        name: "BB中", type: "line", xAxisIndex: 0, yAxisIndex: 0,
        data: bb.mid, symbol: "none", lineStyle: { color: "#5b8def88", width: 1, type: "dashed" },
      },
      {
        name: "BB下", type: "line", xAxisIndex: 0, yAxisIndex: 0,
        data: bb.lower, symbol: "none", lineStyle: { color: "#5b8def55", width: 1 },
      },
      {
        name: "EMA5", type: "line", xAxisIndex: 0, yAxisIndex: 0,
        data: ema5, symbol: "none", lineStyle: { color: "#f59e0b88", width: 1 },
      },
      {
        name: "EMA20", type: "line", xAxisIndex: 0, yAxisIndex: 0,
        data: ema20, symbol: "none", lineStyle: { color: "#8b5cf688", width: 1 },
      },
    ],
  }, true);

  // Click handler for signal detail
  sc.off("click");
  sc.on("click", (params: any) => {
    const di = params.dataIndex;
    if (di == null) return;
    const bs = sigMap.get(di);
    if (!bs) {
      document.getElementById("signal-detail")!.style.display = "none";
      return;
    }
    showSignalDetail(bs, dates[di], closes[di]);
  });
}

function showSignalDetail(bs: BarSig, date: string, price: number) {
  const el = document.getElementById("signal-detail")!;
  const title = document.getElementById("sd-title")!;
  const list = document.getElementById("sd-list")!;
  title.textContent = `${date} — ${price.toFixed(2)} (score: ${bs.score > 0 ? "+" : ""}${bs.score})`;
  list.innerHTML = bs.signals.map(s => {
    const c = s.direction === "Buy" ? "#22c55e" : s.direction === "Sell" ? "#ef4444" : "#556";
    return `<div class="sd-item">
      <span class="sd-kind" style="color:${c}">${s.direction} ${s.kind}</span>
      <span style="color:${c};font-size:10px;margin-left:4px">${s.strength} (${s.score > 0 ? "+" : ""}${s.score})</span>
      <div class="sd-reason">${s.reason}</div>
    </div>`;
  }).join("");
  el.style.display = "block";
}

// ── replay mode ──────────────────────────────────────────────────────────
function startReplay() {
  if (!daytradeData || !daytradeData.bars.length) return;
  replayIdx = 30; // start after warmup
  replayPnl = { trades: 0, wins: 0, totalPct: 0, position: null, entryPrice: 0 };
  if (replayTimer) { clearInterval(replayTimer); replayTimer = null; }
  renderReplayChart();
  updateReplayStats();
}

function renderReplayChart() {
  if (!daytradeData) return;
  const bars = daytradeData.bars.slice(0, replayIdx + 1);
  const dates = bars.map((b: any) => new Date(b.time * 1000).toISOString().slice(0, 10));
  const ohlc = bars.map((b: any) => [b.open, b.close, b.low, b.high]);
  const closes = bars.map((b: any) => b.close);
  const volumes = bars.map((b: any) => b.volume);

  // Only show signals up to current replayIdx
  const sigs = daytradeData.signals.filter(bs => bs.idx <= replayIdx);
  const buyPts: any[] = [];
  const sellPts: any[] = [];
  for (const bs of sigs) {
    const localIdx = bs.idx;
    if (localIdx >= dates.length) continue;
    if (bs.score > 0) buyPts.push({ coord: [dates[localIdx], ohlc[localIdx][2]], value: `+${bs.score}` });
    else if (bs.score < 0) sellPts.push({ coord: [dates[localIdx], ohlc[localIdx][3]], value: `${bs.score}` });
  }

  // Check if current bar has a signal
  const currentSig = daytradeData.signals.find(bs => bs.idx === replayIdx);

  const existingChart = document.getElementById("scanner-chart")!;
  let sc = echarts.getInstanceByDom(existingChart) || echarts.init(existingChart, "dark");

  sc.setOption({
    backgroundColor: "transparent",
    animation: false,
    tooltip: { trigger: "axis", axisPointer: { type: "cross" } },
    legend: { show: false },
    grid: [
      { left: 60, right: 20, top: 30, height: "60%" },
      { left: 60, right: 20, top: "73%", height: "17%" },
    ],
    xAxis: [
      { type: "category", data: dates, gridIndex: 0, axisLabel: { show: false }, axisTick: { show: false }, axisLine: { lineStyle: { color: "#21242e" } } },
      { type: "category", data: dates, gridIndex: 1, axisLabel: { color: "#556", fontSize: 9 }, axisLine: { lineStyle: { color: "#21242e" } } },
    ],
    yAxis: [
      { type: "value", gridIndex: 0, scale: true, splitLine: { lineStyle: { color: "#1a1c22" } }, axisLabel: { color: "#778" } },
      { type: "value", gridIndex: 1, splitLine: { show: false }, axisLabel: { show: false } },
    ],
    dataZoom: [
      { type: "inside", xAxisIndex: [0, 1], start: 0, end: 100 },
      { type: "slider", xAxisIndex: [0, 1], bottom: 10, height: 18, borderColor: "#21242e", fillerColor: "rgba(91,141,239,0.15)", handleStyle: { color: "#5b8def" } },
    ],
    series: [
      {
        name: "K线", type: "candlestick", xAxisIndex: 0, yAxisIndex: 0,
        itemStyle: { color: "#ef4444", color0: "#22c55e", borderColor: "#ef4444", borderColor0: "#22c55e" },
        data: ohlc,
        markPoint: {
          symbol: "arrow", symbolSize: 22, animation: false,
          label: { fontSize: 9, color: "#fff", formatter: (p: any) => p.value },
          data: [
            ...buyPts.map((p: any) => ({ ...p, symbolRotate: 0, itemStyle: { color: "#22c55ecc" } })),
            ...sellPts.map((p: any) => ({ ...p, symbolRotate: 180, itemStyle: { color: "#ef4444cc" } })),
          ],
        },
      },
      {
        name: "成交量", type: "bar", xAxisIndex: 1, yAxisIndex: 1,
        data: volumes.map((v: number, i: number) => ({
          value: v,
          itemStyle: { color: ohlc[i][1] >= ohlc[i][0] ? "#ef444466" : "#22c55e66" },
        })),
      },
    ],
  }, true);

  // Show signal detail if current bar has one
  if (currentSig) {
    const d = dates[dates.length - 1];
    showSignalDetail(currentSig, d, closes[closes.length - 1]);
    // Track P&L
    updatePnl(currentSig, closes[closes.length - 1]);
  } else {
    document.getElementById("signal-detail")!.style.display = "none";
  }
}

function updatePnl(bs: BarSig, price: number) {
  if (bs.score > 0 && replayPnl.position !== "long") {
    // Buy signal → enter long
    replayPnl.position = "long";
    replayPnl.entryPrice = price;
  } else if (bs.score < 0 && replayPnl.position === "long") {
    // Sell signal → exit long
    const pnl = (price - replayPnl.entryPrice) / replayPnl.entryPrice * 100;
    replayPnl.trades++;
    if (pnl > 0) replayPnl.wins++;
    replayPnl.totalPct += pnl;
    replayPnl.position = null;
  }
  updateReplayStats();
}

function updateReplayStats() {
  const el = document.getElementById("replay-stats")!;
  const wr = replayPnl.trades > 0 ? (replayPnl.wins / replayPnl.trades * 100).toFixed(0) : "--";
  const pnlSign = replayPnl.totalPct >= 0 ? "+" : "";
  const pos = replayPnl.position === "long" ? " [LONG]" : "";
  el.innerHTML = `<span style="color:#99a">Trades: ${replayPnl.trades}</span> · ` +
    `<span style="color:#99a">Win: ${wr}%</span> · ` +
    `<span style="color:${replayPnl.totalPct >= 0 ? "#22c55e" : "#ef4444"}">P&L: ${pnlSign}${replayPnl.totalPct.toFixed(2)}%</span>` +
    `<span style="color:#22c55e">${pos}</span>`;
}

function replayPlay() {
  if (replayTimer) return;
  replayTimer = setInterval(() => {
    if (replayIdx >= (daytradeData?.bars.length ?? 0) - 1) {
      replayPause();
      return;
    }
    replayIdx++;
    renderReplayChart();
  }, replaySpeed);
}

function replayPause() {
  if (replayTimer) { clearInterval(replayTimer); replayTimer = null; }
}

function replayStep() {
  if (replayIdx >= (daytradeData?.bars.length ?? 0) - 1) return;
  replayIdx++;
  renderReplayChart();
}

function setReplaySpeed(val: number) {
  replaySpeed = val;
  // If currently playing, restart with new speed
  if (replayTimer) {
    clearInterval(replayTimer);
    replayTimer = null;
    replayPlay();
  }
}

// ── expose scanner to HTML ───────────────────────────────────────────────
(window as any).toggleScanner = toggleScanner;
(window as any).setScannerSub = setScannerSub;
(window as any).loadScanResults = loadScanResults;
(window as any).openAnalysis = openAnalysis;
(window as any).replayPlay = replayPlay;
(window as any).replayPause = replayPause;
(window as any).replayStep = replayStep;
(window as any).setReplaySpeed = setReplaySpeed;
