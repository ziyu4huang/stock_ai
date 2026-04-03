// Stock AI — ECharts Frontend
// Candlestick + Volume + dataZoom + Stock Tabs + Backtest Panel

const echarts = (window as any).echarts;

// ── globals ──────────────────────────────────────────────────────────────
let chart: any = null;
const stockTabs: { symbol: string; data: any[] }[] = [];
let activeIdx = 0;
let currentDays = 90;

// ── init ─────────────────────────────────────────────────────────────────
function initChart() {
  const el = document.getElementById("chart")!;
  chart = echarts.init(el, "dark");
  chart.setOption({
    backgroundColor: "transparent",
    animation: false,
    tooltip: { trigger: "axis", axisPointer: { type: "cross" } },
    legend: { data: ["K线", "成交量"], top: 0, textStyle: { color: "#778" } },
    grid: [
      { left: 60, right: 20, top: 40, height: "55%" },
      { left: 60, right: 20, top: "72%", height: "18%" },
    ],
    xAxis: [
      { type: "category", data: [], gridIndex: 0, axisLabel: { show: false }, axisTick: { show: false } },
      { type: "category", data: [], gridIndex: 1, axisLabel: { color: "#556" } },
    ],
    yAxis: [
      { type: "value", gridIndex: 0, scale: true, splitLine: { lineStyle: { color: "#1a1c22" } }, axisLabel: { color: "#778" } },
      { type: "value", gridIndex: 1, splitLine: { show: false }, axisLabel: { color: "#556" } },
    ],
    dataZoom: [
      { type: "inside", xAxisIndex: [0, 1], start: 50, end: 100 },
      { type: "slider", xAxisIndex: [0, 1], bottom: 10, height: 20, borderColor: "#21242e", fillerColor: "rgba(91,141,239,0.15)", handleStyle: { color: "#5b8def" } },
    ],
    series: [
      {
        name: "K线", type: "candlestick", xAxisIndex: 0, yAxisIndex: 0,
        itemStyle: { color: "#ef4444", color0: "#22c55e", borderColor: "#ef4444", borderColor0: "#22c55e" },
        data: [],
      },
      {
        name: "成交量", type: "bar", xAxisIndex: 1, yAxisIndex: 1,
        itemStyle: { color: "#3b4252" }, data: [],
      },
    ],
  });
  const ro = new ResizeObserver(() => chart.resize());
  ro.observe(el);
}

// ── fetch & render ───────────────────────────────────────────────────────
async function loadStock(symbol: string, days: number) {
  const overlay = document.getElementById("overlay")!;
  overlay.textContent = "Loading...";
  overlay.style.display = "flex";

  try {
    const resp = await fetch(`/api/history/${encodeURIComponent(symbol)}?days=${days}`);
    const data = await resp.json();
    if (!data.bars || data.bars.length === 0) {
      overlay.textContent = "No data for " + symbol;
      return;
    }

    // Store in tabs
    const existing = stockTabs.findIndex(t => t.symbol === symbol);
    if (existing >= 0) {
      stockTabs[existing].data = data.bars;
      activeIdx = existing;
    } else {
      stockTabs.push({ symbol, data: data.bars });
      activeIdx = stockTabs.length - 1;
    }

    renderChart(data.bars);
    renderTabs();
    await loadStats(symbol);

    overlay.style.display = "none";
  } catch (err) {
    overlay.textContent = "Error: " + (err as Error).message;
  }
}

function renderChart(bars: any[]) {
  const dates: string[] = [];
  const ohlc: number[][] = [];
  const volumes: number[] = [];
  const volColors: string[] = [];

  for (const b of bars) {
    const d = new Date(b.time * 1000);
    dates.push(d.toISOString().slice(0, 10));
    ohlc.push([b.open, b.close, b.low, b.high]);
    volumes.push(b.volume);
    volColors.push(b.close >= b.open ? "#ef444480" : "#22c55e80");
  }

  chart.setOption({
    xAxis: [{ data: dates }, { data: dates }],
    series: [
      { data: ohlc },
      {
        data: volumes.map((v, i) => ({
          value: v,
          itemStyle: { color: volColors[i] },
        })),
      },
    ],
  });
}

function renderTabs() {
  const el = document.getElementById("stock-tabs")!;
  el.innerHTML = stockTabs
    .map(
      (t, i) =>
        `<div class="tab${i === activeIdx ? " active" : ""}" onclick="switchTab(${i})">${t.symbol}<span class="tab-close" onclick="event.stopPropagation();closeTab(${i})">×</span></div>`
    )
    .join("");
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

function fmtVol(v: number): string {
  if (v >= 1_000_000) return (v / 1_000_000).toFixed(1) + "M";
  if (v >= 1_000) return (v / 1_000).toFixed(1) + "K";
  return v.toString();
}

// ── backtest ─────────────────────────────────────────────────────────────
async function runBacktest() {
  const symbol = stockTabs[activeIdx]?.symbol;
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

// ── exposed to HTML ──────────────────────────────────────────────────────
(window as any).loadStock = () => {
  const input = document.getElementById("search") as HTMLInputElement;
  const sym = input.value.trim().toUpperCase();
  if (!sym) return;
  loadStock(sym, currentDays);
};

(window as any).setPeriod = (days: number, btn: HTMLButtonElement) => {
  currentDays = days;
  document.querySelectorAll(".period-btn").forEach((b) => b.classList.remove("active"));
  btn.classList.add("active");
  const sym = stockTabs[activeIdx]?.symbol;
  if (sym) loadStock(sym, days);
};

(window as any).switchTab = (idx: number) => {
  activeIdx = idx;
  renderTabs();
  renderChart(stockTabs[idx].data);
  loadStats(stockTabs[idx].symbol);
};

(window as any).closeTab = (idx: number) => {
  stockTabs.splice(idx, 1);
  if (activeIdx >= stockTabs.length) activeIdx = Math.max(0, stockTabs.length - 1);
  renderTabs();
  if (stockTabs.length > 0) {
    renderChart(stockTabs[activeIdx].data);
    loadStats(stockTabs[activeIdx].symbol);
  }
};

(window as any).runBacktest = runBacktest;

(window as any).openReport = () => {
  const symbol = stockTabs[activeIdx]?.symbol;
  if (!symbol) return;
  window.open(`/api/report/${encodeURIComponent(symbol)}`, "_blank");
};

// ── boot ─────────────────────────────────────────────────────────────────
initChart();
(document.getElementById("search") as HTMLInputElement).addEventListener("keydown", (e) => {
  if (e.key === "Enter") (window as any).loadStock();
});
// auto-load TSMC
loadStock("2330.TW", currentDays);
