// Status bar component

import { el, setText } from './dom';

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
    labelEl.textContent = 'PAUSED';
  } else if (wsConnected) {
    dotEl.className = 'status-dot live';
    labelEl.textContent = 'LIVE';
  } else {
    dotEl.className = 'status-dot disconnected';
    labelEl.textContent = 'OFFLINE';
  }

  connEl.textContent = wsConnected ? 'WS: connected' : 'WS: reconnecting...';
  el('status-events').textContent = `Ev:${totalEvents}`;
  el('status-time').textContent = new Date().toLocaleTimeString();
}
