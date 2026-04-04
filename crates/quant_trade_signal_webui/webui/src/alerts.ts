// Alerts panel — conditional innerHTML rebuild

import type { TabSnapshot } from './types';
import { el } from './dom';

export function render(v: TabSnapshot): void {
  const alertsEl = el('alerts');
  const countEl = el('alert-count');
  const countText = v.alerts.length > 0 ? `(${v.alerts.length})` : '';
  if (countEl.textContent !== countText) countEl.textContent = countText;

  if (v.alerts.length === 0) {
    const waitText = 'Watching for whale activity...';
    if (alertsEl.children.length === 0 || !alertsEl.querySelector('.alert-waiting')) {
      alertsEl.innerHTML = `<div class="alert-waiting" style="color:var(--dim);padding:8px;text-align:center">${waitText}</div>`;
    }
    return;
  }

  let html = '';
  for (const a of v.alerts) {
    const color = a.is_ignition ? (a.line1.includes('\u25B2') ? 'var(--lgreen)' : 'var(--lred)') : 'var(--cyan)';
    const ignClass = a.is_ignition ? (a.line1.includes('\u25B2') ? ' ignition bull' : ' ignition bear') : '';
    html += `<div class="alert-item${ignClass}"><div class="alert-time">${a.icon} ${a.timestamp}</div><div class="alert-line1" style="color:${color};font-weight:${a.is_ignition ? 'bold' : 'normal'}">${a.line1}</div><div class="alert-line2">${a.line2}</div></div>`;
  }
  alertsEl.innerHTML = html;
}
