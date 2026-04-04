// Tick feed panel — conditional rebuild

import type { TabSnapshot } from './types';
import { el } from './dom';

export function render(v: TabSnapshot): void {
  const feedEl = el('tick-feed');
  if (v.ticks.length === 0) {
    const waitText = 'Starting tick feed...';
    if (feedEl.children.length === 0 || !feedEl.querySelector('.tick-waiting')) {
      feedEl.innerHTML = `<div class="tick-waiting" style="color:var(--dim);padding:8px;text-align:center">${waitText}</div>`;
    }
    return;
  }

  let html = '';
  for (const t of v.ticks) {
    const sideColor = t.side === 'Buy' ? 'var(--lgreen)' : 'var(--lred)';
    const tagColor = t.amount_m >= 50 ? 'var(--magenta)' : t.amount_m >= 20 ? 'var(--yellow)' : t.amount_m >= 5 ? 'var(--cyan)' : 'var(--dim)';
    const amtColor = t.amount_m >= 5 ? 'var(--yellow)' : 'var(--dim)';
    const isWhale = t.amount_m >= 5;
    html += `<div class="tick-item${isWhale ? ' whale' : ''}"><span class="tick-time">${t.timestamp}</span><span class="tick-arrow" style="color:${sideColor}">${t.arrow}</span><span class="tick-price" style="color:${sideColor}">${t.price.toFixed(2)}</span><span class="tick-tag" style="color:${tagColor}">${t.whale_tag}</span><span class="tick-amount" style="color:${amtColor}">${t.amount_m.toFixed(1)}M</span><span class="tick-shares">${t.shares}sh</span></div>`;
  }
  feedEl.innerHTML = html;
}
