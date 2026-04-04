// Tab bar component — persistent buttons, update-in-place

import type { AppStateSnapshot } from './types';
import { el } from './dom';

let onSwitch: (index: number) => void;
const TAB_COUNT = 5;

export function init(switchFn: (index: number) => void): void {
  onSwitch = switchFn;
  const container = el('tabs');
  for (let i = 0; i < TAB_COUNT; i++) {
    const btn = document.createElement('button');
    btn.className = 'tab-btn';
    btn.dataset.index = String(i);
    btn.addEventListener('click', () => onSwitch(i));
    container.appendChild(btn);
  }
}

export function render(s: AppStateSnapshot): void {
  const container = el('tabs');
  const buttons = container.children;
  for (let i = 0; i < s.tabs.length && i < buttons.length; i++) {
    const btn = buttons[i] as HTMLElement;
    const t = s.tabs[i];
    const active = i === s.active_tab;
    const icon = t.has_signal
      ? (t.signal_dir === 'Long' ? ' \u{1F525}\u25B2' : t.signal_dir === 'Short' ? ' \u{1F525}\u25BC' : ' \u2605')
      : '';

    const text = `${i + 1} ${t.symbol} ${t.name}${icon}`;
    if (btn.textContent !== text) btn.textContent = text;

    let cls = 'tab-btn';
    if (active) cls += ' active';
    if (t.has_signal && !active) cls += ' has-signal';
    if (btn.className !== cls) btn.className = cls;
  }
}
