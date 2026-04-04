// Order book panel — with delta indicators and suspicious level highlighting

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
  const maxLots = Math.max(...ob.asks.map(l => l.lots), ...ob.bids.map(l => l.lots), 1);
  const delta = v.book_delta;
  const suspiciousSet = buildSuspiciousSet(delta.suspicious_levels);

  let html = '';

  // Asks (reversed: sell5 at top, sell1 at bottom near spread)
  for (let i = ob.asks.length - 1; i >= 0; i--) {
    const l = ob.asks[i];
    const pct = (l.lots / maxLots * 100).toFixed(0);
    const d = delta.ask_deltas[i];
    const isSuspicious = suspiciousSet.has(`ask-${i}`);
    const suspClass = isSuspicious ? ' suspicious' : '';
    const suspIcon = getSuspiciousIcon(suspiciousSet, 'ask', i);
    html += `<div class="ob-row ask${suspClass}"><span class="ob-label">${suspIcon}${l.label}</span><span class="ob-price">${l.price.toFixed(2)}</span><span class="ob-lots">${l.lots}${t('lot')}</span>${formatDelta(d)}<div class="ob-bar" style="width:${pct}%"></div></div>`;
  }

  // Spread row
  html += `<div class="spread-row">${ob.last_price.toFixed(2)} ${ob.last_side === 'Buy' ? '\u25B2' : '\u25BC'}  ${t('spread')} ${ob.spread.toFixed(2)}</div>`;

  // Bids
  for (let i = 0; i < ob.bids.length; i++) {
    const l = ob.bids[i];
    const pct = (l.lots / maxLots * 100).toFixed(0);
    const d = delta.bid_deltas[i];
    const isSuspicious = suspiciousSet.has(`bid-${i}`);
    const suspClass = isSuspicious ? ' suspicious' : '';
    const suspIcon = getSuspiciousIcon(suspiciousSet, 'bid', i);
    html += `<div class="ob-row bid${suspClass}"><span class="ob-label">${suspIcon}${l.label}</span><span class="ob-price">${l.price.toFixed(2)}</span><span class="ob-lots">${l.lots}${t('lot')}</span>${formatDelta(d)}<div class="ob-bar" style="width:${pct}%"></div></div>`;
  }

  // Pressure bar
  html += `<div class="pressure-row"><span style="color:var(--green)">${t('buy.pressure')} ${ob.bid_pressure_pct.toFixed(0)}%</span> / <span style="color:var(--red)">${t('sell.pressure')} ${ob.ask_pressure_pct.toFixed(0)}%</span></div>`;
  html += `<div class="pressure-bar"><div class="bid-bar" style="width:${ob.bid_pressure_pct.toFixed(0)}%"></div><div class="ask-bar" style="width:${ob.ask_pressure_pct.toFixed(0)}%"></div></div>`;

  // Cumulative delta bar (2-min net flow)
  const cumBid = delta.cumulative_bid_delta;
  const cumAsk = delta.cumulative_ask_delta;
  const cumTotal = Math.abs(cumBid) + Math.abs(cumAsk);
  if (cumTotal > 0) {
    const bidPct = (cumBid / (cumBid + Math.abs(cumAsk) + 1) * 100).toFixed(0);
    const askPct = (Math.abs(cumAsk) / (cumBid + Math.abs(cumAsk) + 1) * 100).toFixed(0);
    html += `<div class="cum-delta-label"><span style="color:var(--green)">\u0394+${cumBid}</span> <span style="color:var(--dim)">${t('flow.2min')}</span> <span style="color:var(--red)">${cumAsk}\u0394</span></div>`;
    html += `<div class="pressure-bar cum-bar"><div class="bid-bar" style="width:${bidPct}%"></div><div class="ask-bar" style="width:${askPct}%"></div></div>`;
  }

  obEl.innerHTML = html;
}

/** Build a set of "side-level" keys for fast lookup */
function buildSuspiciousSet(levels: SuspiciousLevel[]): Set<string> {
  const s = new Set<string>();
  for (const l of levels) s.add(`${l.side}-${l.level}`);
  return s;
}

/** Get icon for a suspicious level */
function getSuspiciousIcon(set: Set<string>, side: string, level: number): string {
  const key = `${side}-${level}`;
  if (!set.has(key)) return '';
  // Find the actual suspicious level to determine kind
  // We can't look it up efficiently, so use a simple indicator
  return '\u26A0 '; // ⚠
}

/** Format a delta value as a small colored span */
function formatDelta(d: number): string {
  if (d === 0) return '<span class="ob-delta"></span>';
  const color = d > 0 ? 'var(--green)' : 'var(--red)';
  const sign = d > 0 ? '+' : '';
  return `<span class="ob-delta" style="color:${color}">${sign}${d}</span>`;
}
