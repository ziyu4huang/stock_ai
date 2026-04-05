// i18n — zh_TW / English toggle for Whale Radar WebUI

export type Locale = 'en' | 'zh_TW';

type Dict = Record<string, string>;

const en: Dict = {
  // ── Titles & Headers
  'whale.radar': 'WHALE RADAR',
  'order.book': 'ORDER BOOK',
  'signal.radar': 'SIGNAL RADAR',
  'alerts': 'ALERTS',
  'tick.feed': 'TICK FEED',

  // ── Status
  'live': 'LIVE',
  'paused': 'PAUSED',
  'offline': 'OFFLINE',
  'ws.connected': 'WS: connected',
  'ws.reconnecting': 'WS: reconnecting...',
  'events': 'Ev:',

  // ── Toolbar
  'resume': 'Resume',
  'pause': 'Pause',
  'clear': 'Clear',
  'tip.auto': 'Auto-switch to ignition tab',
  'tip.sound': 'Toggle sound alerts',
  'tip.voice': 'Toggle voice alerts',
  'tip.pause': 'Pause/Resume feed',
  'tip.clear': 'Clear alerts',
  'tip.lang': 'Toggle language',

  // ── Radar
  'confidence': 'Confidence',
  'agree': 'AGREE',
  'div': 'DIV',
  'sl': 'S/L:',
  'sl.dash': 'S/L: \u2014',
  'spoof': '\u26A0 SPOOF: ',
  'spoof.detail': ' vanish + ',
  'surge': ' surge',
  'cluster': '\u26A0 CLUSTER: ',
  'cluster.detail': 'x whale burst (wash?)',
  'whale.buy': 'WhaleBuy \u25B2',
  'whale.sell': 'WhaleSell\u25BC',
  'ign.long': 'IgnLong  \u25B2',
  'ign.short': 'IgnShort \u25BC',
  'ticks': 'Ticks:',
  'velocity': 'Velocity ',
  'tmin': ' t/min ',
  'absorption': 'Absorption ',

  // ── Order book
  'waiting.ob': 'Waiting for order book...',
  'spread': 'spread',
  'flow.2min': '2min flow',
  'five.levels': '5-Level',
  'lot': 'sh',
  'buy.pressure': 'Buy',
  'sell.pressure': 'Sell',
  'flow.direction': 'Flow',
  'absorption.label': 'Absorb',
  'aggression.label': 'Agg',
  'aggression.buy': 'Buy',
  'aggression.sell': 'Sell',
  'spoof.wall.up': 'WALL UP',
  'spoof.wall.gone': 'WALL GONE',
  'spoof.setup': 'spoof setup?',
  'spoof.executed': 'spoof executed?',
  'net.bullish': 'BULLISH',
  'net.bearish': 'BEARISH',
  'flow.net': 'NET',
  'more.suspicious': ' more',
  'no.whale.data': 'No whale data yet',

  // ── Alerts
  'watching': 'Watching for whale activity...',

  // ── Tick feed
  'starting.feed': 'Starting tick feed...',
  'sh': 'sh',

  // ── Market states (from Rust)
  'L-ACCUM': 'L-ACCUM',
  'L-IGNITE': 'L-IGNITE',
  'S-DISTRIB': 'S-DISTRIB',
  'S-IGNITE': 'S-IGNITE',
  'NEUTRAL': 'NEUTRAL',
  'NOISE': 'NOISE',

  // ── HMM states (from Rust)
  'H-ACCUM': 'H-ACCUM',
  'H-IGNITE': 'H-IGNITE',
  'H-DIST': 'H-DIST',
  'H-NOISE': 'H-NOISE',

  // ── Whale tags (from Rust)
  'WHALE': 'WHALE',
  'BIG WHALE': 'BIG WHALE',
  'MEGA WHALE': 'MEGA WHALE',

  // ── Trade side
  'BUY': 'BUY',
  'SELL': 'SELL',

  // ── Lang toggle button text
  'lang.toggle': '\u4E2D',

  // ── Help modal
  'help.title': 'Keyboard Shortcuts',
  'help.key': 'Key',
  'help.action': 'Action',
  'help.1to5': '1 – 5',
  'help.1to5.desc': 'Switch to tab N',
  'help.arrows': '← / →',
  'help.arrows.desc': 'Prev / Next tab',
  'help.p': 'P',
  'help.p.desc': 'Pause / Resume',
  'help.a': 'A',
  'help.a.desc': 'Toggle auto-switch',
  'help.s': 'S',
  'help.s.desc': 'Toggle sound',
  'help.v': 'V',
  'help.v.desc': 'Toggle voice',
  'help.space': 'Space',
  'help.space.desc': 'Clear alerts',
  'help.q': '?',
  'help.q.desc': 'Toggle this help',
  'help.dismiss': 'Press ? or Esc to close',
};

const zh_TW: Dict = {
  // ── Titles & Headers
  'whale.radar': '\u5DE8\u9F8F\u96F7\u9054',
  'order.book': '\u59D4\u8A17\u7C3F',
  'signal.radar': '\u4FE1\u865F\u96F7\u9054',
  'alerts': '\u8B66\u5831',
  'tick.feed': '\u9010\u7B46\u660E\u7D30',

  // ── Status
  'live': '\u5373\u6642',
  'paused': '\u66AB\u505C',
  'offline': '\u96E2\u7DDA',
  'ws.connected': 'WS: \u5DF2\u9023\u7DDA',
  'ws.reconnecting': 'WS: \u91CD\u65B0\u9023\u7DDA...',
  'events': '\u4E8B\u4EF6:',

  // ── Toolbar
  'resume': '\u7E7C\u7E8C',
  'pause': '\u66AB\u505C',
  'clear': '\u6E05\u9664',
  'tip.auto': '\u81EA\u52D5\u5207\u63DB\u81F3\u9EDE\u706B\u5206\u9801',
  'tip.sound': '\u97F3\u6548\u958B\u95DC',
  'tip.voice': '\u8A9E\u97F3\u958B\u95DC',
  'tip.pause': '\u66AB\u505C/\u7E7C\u7E8C\u9910\u6599',
  'tip.clear': '\u6E05\u9664\u8B66\u5831',
  'tip.lang': '\u5207\u63DB\u8A9E\u7CFB',

  // ── Radar
  'confidence': '\u4FE1\u5FC3\u5EA6',
  'agree': '\u4E00\u81F4',
  'div': '\u5206\u6B67',
  'sl': '\u505C\u640D: ',
  'sl.dash': '\u505C\u640D: \u2014',
  'spoof': '\u26A0 \u5047\u55AE: ',
  'spoof.detail': ' \u6D88\u5931 + ',
  'surge': ' \u6E67\u5165',
  'cluster': '\u26A0 \u5BC6\u96C6: ',
  'cluster.detail': 'x \u5DE8\u9F8F\u7FA4 (\u5C0D\u6572?)',
  'whale.buy': '\u5DE8\u9F8F\u8CB7 \u25B2',
  'whale.sell': '\u5DE8\u9F8F\u8CE3\u25BC',
  'ign.long': '\u9EDE\u706B\u591A  \u25B2',
  'ign.short': '\u9EDE\u706B\u7A7A  \u25BC',
  'ticks': '\u7B46\u6578:',
  'velocity': '\u901F\u5EA6 ',
  'tmin': ' \u7B46/\u5206 ',
  'absorption': '\u5438\u6536\u529B ',

  // ── Order book
  'waiting.ob': '\u7B49\u5F85\u59D4\u8A17\u7C3F\u8CC7\u6599...',
  'spread': '\u50F9\u5DEE',
  'flow.2min': '2\u5206\u9418\u6D41\u5411',
  'five.levels': '\u4E94\u6A94',
  'lot': '\u5F35',
  'buy.pressure': '\u8CB7\u58D3',
  'sell.pressure': '\u8CE3\u58D3',
  'flow.direction': '\u6D41\u5411',
  'absorption.label': '\u5438\u6536',
  'aggression.label': '\u653B\u64CA',
  'aggression.buy': '\u8CB7',
  'aggression.sell': '\u8CE3',
  'spoof.wall.up': '\u639B\u7246',
  'spoof.wall.gone': '\u64A4\u55AE',
  'spoof.setup': '\u5047\u55AE\u4F48\u5C40?',
  'spoof.executed': '\u5047\u55AE\u57F7\u884C?',
  'net.bullish': '\u504F\u591A',
  'net.bearish': '\u504F\u7A7A',
  'flow.net': '\u6DE8',
  'more.suspicious': ' \u66F4\u591A',
  'no.whale.data': '\u5C1A\u7121\u5DE8\u9F8F\u6578\u64DA',

  // ── Alerts
  'watching': '\u76E3\u63A7\u5DE8\u9F8F\u6D3B\u52D5\u4E2D...',

  // ── Tick feed
  'starting.feed': '\u7B49\u5F85\u9010\u7B46\u8CC7\u6599...',
  'sh': '\u5F35',

  // ── Market states
  'L-ACCUM': '\u591A\u65B9\u5438\u7C4C',
  'L-IGNITE': '\u591A\u65B9\u9EDE\u706B',
  'S-DISTRIB': '\u7A7A\u65B9\u51FA\u8CA8',
  'S-IGNITE': '\u7A7A\u65B9\u9EDE\u706B',
  'NEUTRAL': '\u76E4\u6574',
  'NOISE': '\u96DC\u8A0A',

  // ── HMM states
  'H-ACCUM': 'H\u5438\u7C4C',
  'H-IGNITE': 'H\u9EDE\u706B',
  'H-DIST': 'H\u51FA\u8CA8',
  'H-NOISE': 'H\u96DC\u8A0A',

  // ── Whale tags
  'WHALE': '\u5DE8\u9F8F',
  'BIG WHALE': '\u5927\u5DE8\u9F8F',
  'MEGA WHALE': '\u8D85\u7D1A\u5DE8\u9F8F',

  // ── Trade side
  'BUY': '\u8CB7',
  'SELL': '\u8CE3',

  // ── Lang toggle button text
  'lang.toggle': 'EN',

  // ── Help modal
  'help.title': '\u9375\u76E4\u5FEB\u6377\u9375',
  'help.key': '\u6309\u9375',
  'help.action': '\u529F\u80FD',
  'help.1to5': '1 – 5',
  'help.1to5.desc': '\u5207\u63DB\u81F3\u7B2C N \u5206\u9801',
  'help.arrows': '\u2190 / \u2192',
  'help.arrows.desc': '\u4E0A\u4E00 / \u4E0B\u4E00\u5206\u9801',
  'help.p': 'P',
  'help.p.desc': '\u66AB\u505C / \u7E7C\u7E8C',
  'help.a': 'A',
  'help.a.desc': '\u81EA\u52D5\u5207\u63DB\u958B\u95DC',
  'help.s': 'S',
  'help.s.desc': '\u97F3\u6548\u958B\u95DC',
  'help.v': 'V',
  'help.v.desc': '\u8A9E\u97F3\u958B\u95DC',
  'help.space': 'Space',
  'help.space.desc': '\u6E05\u9664\u8B66\u5831',
  'help.q': '?',
  'help.q.desc': '\u986F\u793A\u6B64\u8AAA\u660E',
  'help.dismiss': '\u6309 ? \u6216 Esc \u95DC\u9589',
};

// ── Locale management ────────────────────────────────────────────

const STORAGE_KEY = 'whale-radar-locale';
const DICTS: Record<Locale, Dict> = { en, zh_TW };

let currentLocale: Locale = 'zh_TW';

export function getLocale(): Locale {
  return currentLocale;
}

export function setLocale(loc: Locale): void {
  currentLocale = loc;
  try { localStorage.setItem(STORAGE_KEY, loc); } catch {}
}

export function initLocale(): void {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored === 'en' || stored === 'zh_TW') {
      currentLocale = stored;
    }
  } catch {}
}

export function toggleLocale(): Locale {
  const next = currentLocale === 'en' ? 'zh_TW' : 'en';
  setLocale(next);
  return next;
}

// ── Translation function ─────────────────────────────────────────

/** Translate a key. If not found, returns the key itself. */
export function t(key: string): string {
  return DICTS[currentLocale][key] ?? DICTS.en[key] ?? key;
}
