// Tick feed panel — conditional rebuild

import type { TabSnapshot } from './types';
import { el } from './dom';
import { t } from './i18n';

export function render(v: TabSnapshot): void {
  const feedEl = el('tick-feed');
  if (v.ticks.length === 0) {
    const waitText = t('starting.feed');
    if (feedEl.children.length === 0 || !feedEl.querySelector('.tick-waiting')) {
      feedEl.innerHTML = `<div class="tick-waiting" style="color:var(--dim);padding:8px;text-align:center">${waitText}</div>`;
    }
    return;
  }

  el('tick-title').textContent = `${t('tick.feed')}  ${v.symbol}`;
  const shLabel = t('sh');
  let html = '';
  for (const tick of v.ticks) {
    const sideColor = tick.side === 'Buy' ? 'var(--lgreen)' : 'var(--lred)';
    const tagColor = tick.amount_m >= 50 ? 'var(--magenta)' : tick.amount_m >= 20 ? 'var(--yellow)' : tick.amount_m >= 5 ? 'var(--cyan)' : 'var(--dim)';
    const amtColor = tick.amount_m >= 5 ? 'var(--yellow)' : 'var(--dim)';
    const isWhale = tick.amount_m >= 5;
    html += `<div class="tick-item${isWhale ? ' whale' : ''}"><span class="tick-time">${tick.timestamp}</span><span class="tick-arrow" style="color:${sideColor}">${tick.arrow}</span><span class="tick-price" style="color:${sideColor}">${tick.price.toFixed(2)}</span><span class="tick-tag" style="color:${tagColor}">${tick.whale_tag}</span><span class="tick-amount" style="color:${amtColor}">${tick.amount_m.toFixed(1)}M</span><span class="tick-shares">${tick.shares}${shLabel}</span></div>`;
  }
  feedEl.innerHTML = html;
}
