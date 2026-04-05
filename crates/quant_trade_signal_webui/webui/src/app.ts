// Entry point: WebSocket + rAF loop + component orchestration

import type { AppStateSnapshot } from './types';
import { StateStore } from './state';
import { el, setText } from './dom';
import * as tabs from './tabs';
import * as orderbook from './orderbook';
import * as radar from './radar';
import * as alerts from './alerts';
import * as tickfeed from './tickfeed';
import * as status from './status';
import * as keyboard from './keyboard';
import { initLocale, getLocale, toggleLocale, t } from './i18n';

// ── Command sender ──────────────────────────────────────────────

function sendCommand(action: string, extra?: Record<string, unknown>): void {
  const body = extra ? { action, ...extra } : { action };
  fetch('/api/command', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  });
}

function switchTab(i: number): void {
  sendCommand('switch_tab', { index: i });
}

// ── WebSocket ───────────────────────────────────────────────────

let pendingState: AppStateSnapshot | null = null;
let animFrameId = 0;

function connect(): void {
  const proto = location.protocol === 'https:' ? 'wss' : 'ws';
  const ws = new WebSocket(`${proto}://${location.host}/ws`);

  ws.onopen = () => {
    status.setWsConnected(true);
  };

  ws.onmessage = (e: MessageEvent) => {
    try {
      pendingState = JSON.parse(e.data) as AppStateSnapshot;
      if (!animFrameId) {
        animFrameId = requestAnimationFrame(renderFrame);
      }
    } catch (err) {
      console.error('parse error', err);
    }
  };

  ws.onclose = () => {
    status.setWsConnected(false);
    setTimeout(connect, 1000);
  };

  ws.onerror = () => {};
}

// ── rAF rendering ───────────────────────────────────────────────

const store = new StateStore();

function renderFrame(): void {
  animFrameId = 0;
  if (!pendingState) return;

  const diff = store.update(pendingState);
  const s = store.curr!;
  pendingState = null;

  // Tabs — only update if tab metadata or active tab changed
  if (diff.tabsChanged) tabs.render(s);

  // Toolbar buttons
  if (diff.toolbarChanged) renderToolbar(s);

  // Active tab data
  const v = s.tabs[s.active_tab];

  // Order book
  if (diff.orderBookChanged) orderbook.render(v);

  // Signal radar
  if (diff.radarChanged) radar.render(v);

  // Alerts
  if (diff.alertsChanged) alerts.render(v);

  // Tick feed
  if (diff.tickFeedChanged) tickfeed.render(v);

  // Status bar
  if (diff.statusChanged) status.render(s.paused, s.total_events);
}

// ── Toolbar rendering ───────────────────────────────────────────

function renderToolbar(s: AppStateSnapshot): void {
  const autoEl = el('ind-auto');
  const autoText = s.auto_switch ? 'A \u25C9' : 'A \u25CB';
  if (autoEl.textContent !== autoText) autoEl.textContent = autoText;
  const autoCls = 'toolbar-btn' + (s.auto_switch ? ' on' : '');
  if (autoEl.className !== autoCls) autoEl.className = autoCls;
  autoEl.style.color = s.auto_switch ? 'var(--green)' : '';

  const soundEl = el('ind-sound');
  const soundText = s.sound_on ? '\u{1F50A}' : '\u{1F507}';
  if (soundEl.textContent !== soundText) soundEl.textContent = soundText;
  const soundCls = 'toolbar-btn' + (s.sound_on ? ' on' : '');
  if (soundEl.className !== soundCls) soundEl.className = soundCls;
  soundEl.style.color = s.sound_on ? 'var(--cyan)' : '';

  const voiceEl = el('ind-voice');
  const voiceText = s.voice_on ? '\u{1F5E3}' : '  ';
  if (voiceEl.textContent !== voiceText) voiceEl.textContent = voiceText;
  const voiceCls = 'toolbar-btn' + (s.voice_on ? ' on' : '');
  if (voiceEl.className !== voiceCls) voiceEl.className = voiceCls;
  voiceEl.style.color = s.voice_on ? 'var(--yellow)' : '';

  // Language toggle
  const langEl = el('ind-lang');
  const langText = t('lang.toggle');
  if (langEl.textContent !== langText) langEl.textContent = langText;

  const pauseEl = el('ind-pause');
  const pauseText = s.paused ? `\u25B6 ${t('resume')}` : `\u23F8 ${t('pause')}`;
  if (pauseEl.textContent !== pauseText) pauseEl.textContent = pauseText;
  const pauseCls = 'toolbar-btn' + (s.paused ? ' pause-active' : '');
  if (pauseEl.className !== pauseCls) pauseEl.className = pauseCls;
}

// ── Toolbar button event listeners ──────────────────────────────

function setupToolbar(): void {
  el('ind-auto').addEventListener('click', () => sendCommand('toggle_auto'));
  el('ind-sound').addEventListener('click', () => sendCommand('toggle_sound'));
  el('ind-voice').addEventListener('click', () => sendCommand('toggle_voice'));
  el('ind-pause').addEventListener('click', () => sendCommand('toggle_pause'));
  el('btn-clear').addEventListener('click', () => sendCommand('clear_alerts'));
  el('btn-help').addEventListener('click', () => toggleHelp());

  // Language toggle — toggles locale and forces full re-render
  el('ind-lang').addEventListener('click', () => {
    toggleLocale();
    forceRerender();
    if (helpVisible) buildHelpModal();
  });
}

/** Force a full re-render after locale change */
function forceRerender(): void {
  const s = store.curr;
  if (!s) return;
  tabs.render(s);
  renderToolbar(s);
  orderbook.render(s.tabs[s.active_tab]);
  radar.render(s.tabs[s.active_tab]);
  alerts.render(s.tabs[s.active_tab]);
  tickfeed.render(s.tabs[s.active_tab]);
  status.render(s.paused, s.total_events);
  // Also update static panel headers
  updatePanelHeaders();
}

/** Update static panel header text that doesn't come from WebSocket data */
function updatePanelHeaders(): void {
  // ob-title is already set by orderbook.render()
  const srEl = document.getElementById('signal-radar-title');
  if (srEl) srEl.textContent = `${t('signal.radar')}`;
  const alEl = document.getElementById('alerts-title');
  if (alEl) alEl.textContent = `${t('alerts')} `;
}

// ── Help modal ──────────────────────────────────────────────────

let helpVisible = false;

function buildHelpModal(): void {
  const modal = document.getElementById('help-modal');
  if (!modal) return;
  const rows = [
    ['help.1to5', 'help.1to5.desc'],
    ['help.arrows', 'help.arrows.desc'],
    ['help.p', 'help.p.desc'],
    ['help.a', 'help.a.desc'],
    ['help.s', 'help.s.desc'],
    ['help.v', 'help.v.desc'],
    ['help.space', 'help.space.desc'],
    ['help.q', 'help.q.desc'],
  ];
  const tbody = rows.map(([k, d]) =>
    `<tr><td style="padding:3px 10px 3px 0;color:var(--cyan);font-weight:bold">${t(k)}</td><td style="padding:3px 0;color:var(--text)">${t(d)}</td></tr>`
  ).join('');
  modal.innerHTML = `<div style="background:var(--bg2);border:1px solid var(--border);border-radius:8px;padding:20px;min-width:300px">
    <div style="color:var(--text);font-size:13px;font-weight:bold;margin-bottom:12px;border-bottom:1px solid var(--border);padding-bottom:8px">${t('help.title')}</div>
    <table style="width:100%;border-collapse:collapse;font-size:11px"><tbody>${tbody}</tbody></table>
    <div style="margin-top:12px;color:var(--dim);font-size:10px;text-align:center">${t('help.dismiss')}</div>
  </div>`;
}

function toggleHelp(): void {
  helpVisible = !helpVisible;
  const modal = document.getElementById('help-modal');
  if (!modal) return;
  if (helpVisible) {
    buildHelpModal();
    modal.style.display = 'flex';
  } else {
    modal.style.display = 'none';
  }
}

// ── Boot ────────────────────────────────────────────────────────

function boot(): void {
  initLocale();
  tabs.init(switchTab);
  setupToolbar();
  keyboard.init(sendCommand, toggleHelp);
  connect();

  // Set tooltips (static)
  el('ind-auto').title = t('tip.auto');
  el('ind-sound').title = t('tip.sound');
  el('ind-voice').title = t('tip.voice');
  el('ind-pause').title = t('tip.pause');
  el('btn-clear').title = t('tip.clear');

  // Initial status
  status.setWsConnected(false);
  status.render(false, 0);
}

boot();
