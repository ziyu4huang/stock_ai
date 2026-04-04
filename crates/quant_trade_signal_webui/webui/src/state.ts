// State store with diff tracking

import type { AppStateSnapshot, TabSnapshot } from './types';

export interface DiffFlags {
  tabsChanged: boolean;
  toolbarChanged: boolean;
  activeTabChanged: boolean;
  orderBookChanged: boolean;
  radarChanged: boolean;
  alertsChanged: boolean;
  tickFeedChanged: boolean;
  statusChanged: boolean;
}

export class StateStore {
  private prev: AppStateSnapshot | null = null;
  curr: AppStateSnapshot | null = null;

  /** Update state and return diff flags */
  update(snapshot: AppStateSnapshot): DiffFlags {
    this.prev = this.curr;
    this.curr = snapshot;

    if (!this.prev) {
      // First render — everything changed
      return {
        tabsChanged: true,
        toolbarChanged: true,
        activeTabChanged: true,
        orderBookChanged: true,
        radarChanged: true,
        alertsChanged: true,
        tickFeedChanged: true,
        statusChanged: true,
      };
    }

    const p = this.prev;
    const c = this.curr;
    const activeTabChanged = p.active_tab !== c.active_tab;
    const prevTab = p.tabs[p.active_tab];
    const currTab = c.tabs[c.active_tab];

    // Check if tab metadata (has_signal, signal_dir) changed
    let tabsMetaChanged = activeTabChanged || p.tabs.length !== c.tabs.length;
    if (!tabsMetaChanged) {
      for (let i = 0; i < c.tabs.length; i++) {
        if (p.tabs[i].has_signal !== c.tabs[i].has_signal ||
            p.tabs[i].signal_dir !== c.tabs[i].signal_dir) {
          tabsMetaChanged = true;
          break;
        }
      }
    }

    // Toolbar: auto_switch, sound_on, voice_on, paused
    const toolbarChanged =
      p.auto_switch !== c.auto_switch ||
      p.sound_on !== c.sound_on ||
      p.voice_on !== c.voice_on ||
      p.paused !== c.paused;

    // Order book changed?
    const orderBookChanged = activeTabChanged || !orderBookEqual(prevTab.order_book, currTab.order_book);

    // Radar: score, state, action, hmm, counters, price
    const radarChanged = activeTabChanged ||
      prevTab.composite_score !== currTab.composite_score ||
      prevTab.state !== currTab.state ||
      prevTab.action !== currTab.action ||
      prevTab.action_label_zh !== currTab.action_label_zh ||
      prevTab.hmm_state !== currTab.hmm_state ||
      prevTab.hmm_agrees !== currTab.hmm_agrees ||
      prevTab.whale_buys !== currTab.whale_buys ||
      prevTab.whale_sells !== currTab.whale_sells ||
      prevTab.long_ignitions !== currTab.long_ignitions ||
      prevTab.short_ignitions !== currTab.short_ignitions ||
      prevTab.last_price !== currTab.last_price ||
      prevTab.change_pct !== currTab.change_pct ||
      prevTab.total_ticks !== currTab.total_ticks ||
      prevTab.stop_loss !== currTab.stop_loss ||
      prevTab.state_icon !== currTab.state_icon ||
      prevTab.hmm_state_icon !== currTab.hmm_state_icon ||
      prevTab.hmm_pts !== currTab.hmm_pts ||
      !soundEqual(prevTab.last_sound, currTab.last_sound);

    // Alerts changed?
    const alertsChanged = activeTabChanged || !alertsEqual(prevTab.alerts, currTab.alerts);

    // Tick feed changed?
    const tickFeedChanged = activeTabChanged ||
      prevTab.ticks.length !== currTab.ticks.length ||
      (currTab.ticks.length > 0 && prevTab.ticks.length > 0 && prevTab.ticks[0].timestamp !== currTab.ticks[0].timestamp);

    // Status bar
    const statusChanged = p.total_events !== c.total_events || p.paused !== c.paused;

    return {
      tabsChanged: tabsMetaChanged,
      toolbarChanged,
      activeTabChanged,
      orderBookChanged,
      radarChanged,
      alertsChanged,
      tickFeedChanged,
      statusChanged,
    };
  }
}

function orderBookEqual(a: TabSnapshot['order_book'], b: TabSnapshot['order_book']): boolean {
  if (a === b) return true;
  if (!a || !b) return false;
  if (a.last_price !== b.last_price || a.spread !== b.spread ||
      a.bid_pressure_pct !== b.bid_pressure_pct || a.ask_pressure_pct !== b.ask_pressure_pct) return false;
  for (let i = 0; i < a.asks.length; i++) {
    if (a.asks[i].lots !== b.asks[i].lots) return false;
  }
  for (let i = 0; i < a.bids.length; i++) {
    if (a.bids[i].lots !== b.bids[i].lots) return false;
  }
  return true;
}

function soundEqual(a: TabSnapshot['last_sound'], b: TabSnapshot['last_sound']): boolean {
  if (a === b) return true;
  if (!a || !b) return false;
  return a.label === b.label && a.timestamp === b.timestamp;
}

function alertsEqual(a: TabSnapshot['alerts'], b: TabSnapshot['alerts']): boolean {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    if (a[i].line1 !== b[i].line1 || a[i].line2 !== b[i].line2) return false;
  }
  return true;
}
