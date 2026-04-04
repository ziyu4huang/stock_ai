// Status bar component

import { el, setText } from './dom';
import { t } from './i18n';

let wsConnected = false;

export function setWsConnected(connected: boolean): void {
  wsConnected = connected;
}

export function render(paused: boolean, totalEvents: number): void {
  const dotEl = el('status-dot');
  const labelEl = el('status-label');
  const connEl = el('status-conn');

  if (paused) {
    dotEl.className = 'status-dot paused';
    labelEl.textContent = t('paused');
  } else if (wsConnected) {
    dotEl.className = 'status-dot live';
    labelEl.textContent = t('live');
  } else {
    dotEl.className = 'status-dot disconnected';
    labelEl.textContent = t('offline');
  }

  connEl.textContent = wsConnected ? t('ws.connected') : t('ws.reconnecting');
  el('status-events').textContent = `${t('events')}${totalEvents}`;
  el('status-time').textContent = new Date().toLocaleTimeString();
}
