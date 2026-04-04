// Order book panel — 4-section layout: Flow Summary → Enhanced 5-Level → Spoof Detail → Flow Gauge

import type { TabSnapshot, SuspiciousLevel } from './types';
import { el } from './dom';
import { t } from './i18n';

export function render(v: TabSnapshot): void {
  const titleEl = el('ob-title');
  const newTitle = `${t('order.book')} ${t('five.levels')} ${v.symbol} ${v.name}`;
  if (titleEl.textContent !== newTitle) titleEl.textContent = newTitle;

  const obEl = el('order-book');
  if (!v.order_book) {
    if (obEl.children.length === 0 || !obEl.querySelector('.ob-waiting')) {
      obEl.innerHTML = `<div class="ob-waiting" style="color:var(--dim);padding:12px;text-align:center">${t('waiting.ob')}</div>`;
    }
    return;
  }

  const ob = v.order_book;
  const delta = v.book_delta;
  const maxLots = Math.max(...ob.asks.map(l => l.lots), ...ob.bids.map(l => l.lots), 1);
  const suspMap = buildSuspiciousMap(delta.suspicious_levels);

  let html = '';

  // ── Section A: Flow Summary ──────────────────────
  html += renderFlowSummary(v);

  // ── Section B: Enhanced 5-Level Order Book ───────
  // Asks (reversed: sell5 at top, sell1 at bottom near spread)
  for (let i = ob.asks.length - 1; i >= 0; i--) {
    const l = ob.asks[i];
    const pct = (l.lots / maxLots * 100).toFixed(0);
    const d = delta.ask_deltas[i];
    const info = suspMap.get(`ask-${i}`);
    html += renderOrderRow('ask', i, l, d, info, pct);
  }

  // Spread row
  html += `<div class="spread-row">${ob.last_price.toFixed(2)} ${ob.last_side === 'Buy' ? '\u25B2' : '\u25BC'}  ${t('spread')} ${ob.spread.toFixed(2)}</div>`;

  // Bids
  for (let i = 0; i < ob.bids.length; i++) {
    const l = ob.bids[i];
    const pct = (l.lots / maxLots * 100).toFixed(0);
    const d = delta.bid_deltas[i];
    const info = suspMap.get(`bid-${i}`);
    html += renderOrderRow('bid', i, l, d, info, pct);
  }

  // ── Section C: Spoof Detection Detail ────────────
  if (delta.suspicious_levels.length > 0) {
    html += renderSpoofDetail(delta.suspicious_levels, ob);
  }

  // ── Section D: Flow Delta Gauge ──────────────────
  html += renderFlowGauge(delta);

  obEl.innerHTML = html;
}

// ── Section A: Flow Summary ──────────────────────────

function renderFlowSummary(v: TabSnapshot): string {
  const delta = v.book_delta;
  const cumBid = delta.cumulative_bid_delta;
  const cumAsk = delta.cumulative_ask_delta;

  // Flow direction bar
  const flowTotal = Math.abs(cumBid) + Math.abs(cumAsk);
  const bidPct = flowTotal > 0 ? (Math.abs(cumBid) / flowTotal * 100).toFixed(0) : '50';
  const askPct = flowTotal > 0 ? (Math.abs(cumAsk) / flowTotal * 100).toFixed(0) : '50';
  const netFlow = cumBid + cumAsk;

  // Absorption gauge (0-100)
  const absVal = v.absorption_score;
  const absColor = absVal >= 60 ? 'var(--cyan)' : absVal >= 30 ? 'var(--cyan)' : 'var(--dim)';
  const absBold = absVal >= 60 ? 'font-weight:bold;' : '';

  // Aggression ratio
  const aggTotal = v.aggressive_buys + v.aggressive_sells;
  const aggPct = aggTotal > 0 ? (v.aggressive_buys / aggTotal * 100).toFixed(0) : null;
  const aggColor = aggPct !== null
    ? (parseInt(aggPct) >= 60 ? 'var(--lgreen)' : parseInt(aggPct) <= 40 ? 'var(--lred)' : 'var(--dim)')
    : 'var(--dim)';

  return `<div class="ob-flow-summary">
    <div class="flow-metric">
      <span class="flow-metric-label">${t('flow.direction')}</span>
      <div class="flow-metric-bar"><div class="flow-metric-fill" style="background:var(--green);width:${bidPct}%"></div></div>
      <span class="flow-metric-val" style="color:${netFlow >= 0 ? 'var(--lgreen)' : 'var(--lred)'}">${netFlow >= 0 ? '+' : ''}${netFlow}</span>
    </div>
    <div class="flow-metric">
      <span class="flow-metric-label">${t('absorption.label')}</span>
      <div class="flow-metric-bar"><div class="flow-metric-fill" style="background:${absColor};width:${absVal.toFixed(0)}%"></div></div>
      <span class="flow-metric-val" style="color:${absColor};${absBold}">${absVal.toFixed(0)}%</span>
    </div>
    <div class="flow-metric">
      <span class="flow-metric-label">${t('aggression.label')}</span>
      ${aggPct !== null
        ? `<div class="flow-metric-bar"><div class="flow-metric-fill" style="background:${aggColor};width:${aggPct}%"></div></div>
           <span class="flow-metric-val" style="color:${aggColor}">${aggPct}% ${t('aggression.buy')}</span>`
        : `<div class="flow-metric-bar"><div class="flow-metric-fill" style="background:var(--dim);width:0%"></div></div>
           <span class="flow-metric-val" style="color:var(--dim)">${t('no.whale.data')}</span>`}
    </div>
  </div>`;
}

// ── Section B: Enhanced Order Row ────────────────────

function renderOrderRow(
  side: 'ask' | 'bid',
  idx: number,
  level: { label: string; price: number; lots: number },
  d: number,
  suspInfo: SuspiciousLevel | undefined,
  pct: string,
): string {
  const suspClass = suspInfo ? ' suspicious' : '';
  const badge = suspInfo ? renderBadge(suspInfo) : '';
  const deltaHtml = formatDelta(d, pct);

  return `<div class="ob-row ${side}${suspClass}">
    <span class="ob-label">${level.label}${badge}</span>
    <span class="ob-price">${level.price.toFixed(2)}</span>
    <span class="ob-lots">${level.lots}${t('lot')}</span>
    ${deltaHtml}
    <div class="ob-bar" style="width:${pct}%"></div>
  </div>`;
}

function renderBadge(info: SuspiciousLevel): string {
  if (info.kind === 'sudden_add') {
    return `<span class="ob-badge wall-up">${t('spoof.wall.up')}</span>`;
  }
  return `<span class="ob-badge wall-gone">${t('spoof.wall.gone')}</span>`;
}

function formatDelta(d: number, _pct: string): string {
  if (d === 0) return '<span class="ob-delta"></span>';
  const cls = d > 0 ? 'pos' : 'neg';
  const large = Math.abs(d) >= 50 ? ' large' : '';
  const sign = d > 0 ? '+' : '';
  return `<span class="ob-delta ${cls}${large}">${sign}${d}</span>`;
}

// ── Section C: Spoof Detection Detail ────────────────

function renderSpoofDetail(levels: SuspiciousLevel[], ob: { asks: { label: string }[]; bids: { label: string }[] }): string {
  let html = '<div class="ob-spoof-detail">';
  const max = Math.min(levels.length, 3);
  for (let i = 0; i < max; i++) {
    const s = levels[i];
    const label = s.side === 'ask' ? ob.asks[s.level]?.label : ob.bids[s.level]?.label;
    const badgeCls = s.kind === 'sudden_add' ? 'add' : 'remove';
    const badgeText = s.kind === 'sudden_add' ? t('spoof.wall.up') : t('spoof.wall.gone');
    const hint = s.kind === 'sudden_add' ? t('spoof.setup') : t('spoof.executed');
    const sign = s.kind === 'sudden_add' ? '+' : '';
    html += `<div class="spoof-row">
      <span style="color:var(--yellow)">\u26A0</span>
      <span>${label ?? `${s.side}${s.level}`}</span>
      <span class="spoof-badge ${badgeCls}">${badgeText}</span>
      <span style="color:var(--yellow)">${sign}${s.volume}${t('lot')}</span>
      <span style="color:var(--dim)">${hint}</span>
    </div>`;
  }
  if (levels.length > 3) {
    html += `<div class="spoof-row" style="color:var(--dim)">+${levels.length - 3}${t('more.suspicious')}</div>`;
  }
  html += '</div>';
  return html;
}

// ── Section D: Flow Delta Gauge ──────────────────────

function renderFlowGauge(delta: { cumulative_bid_delta: number; cumulative_ask_delta: number }): string {
  const cumBid = delta.cumulative_bid_delta;
  const cumAsk = delta.cumulative_ask_delta;
  const total = Math.abs(cumBid) + Math.abs(cumAsk);
  const net = cumBid + cumAsk;

  let bidPct: string;
  let askPct: string;
  if (total > 0) {
    bidPct = (cumBid / (cumBid + Math.abs(cumAsk) + 1) * 100).toFixed(0);
    askPct = (Math.abs(cumAsk) / (cumBid + Math.abs(cumAsk) + 1) * 100).toFixed(0);
  } else {
    bidPct = '50';
    askPct = '50';
  }

  const netColor = net > 0 ? 'var(--lgreen)' : net < 0 ? 'var(--lred)' : 'var(--dim)';
  const netLabel = net > 0 ? t('net.bullish') : net < 0 ? t('net.bearish') : '';
  const netSign = net >= 0 ? '+' : '';

  return `<div class="ob-flow-gauge">
    <div class="flow-gauge-bar">
      <div class="bid-bar" style="width:${bidPct}%"></div>
      <div class="ask-bar" style="width:${askPct}%"></div>
      <div class="flow-gauge-center"></div>
    </div>
    <div class="flow-gauge-label">
      <span style="color:var(--green)">\u0394+${cumBid}</span>
      <span style="color:var(--dim)">${t('flow.2min')}</span>
      <span style="color:var(--red)">${cumAsk}\u0394</span>
    </div>
    ${total > 0 ? `<div class="flow-gauge-net" style="color:${netColor}">${t('flow.net')} ${netSign}${net} ${netLabel}</div>` : ''}
  </div>`;
}

// ── Helpers ──────────────────────────────────────────

/** Build a Map keyed by "side-level" for full suspicious level lookup */
function buildSuspiciousMap(levels: SuspiciousLevel[]): Map<string, SuspiciousLevel> {
  const m = new Map<string, SuspiciousLevel>();
  for (const l of levels) m.set(`${l.side}-${l.level}`, l);
  return m;
}
