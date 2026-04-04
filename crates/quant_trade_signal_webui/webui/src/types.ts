// TypeScript interfaces mirroring Rust AppStateSnapshot and friends

export interface AppStateSnapshot {
  active_tab: number;
  auto_switch: boolean;
  paused: boolean;
  sound_on: boolean;
  voice_on: boolean;
  total_events: number;
  tabs: TabSnapshot[];
}

export interface TabSnapshot {
  symbol: string;
  name: string;
  has_signal: boolean;
  signal_dir: string | null;
  state: string;
  state_icon: string;
  action: string;
  action_label_zh: string;
  composite_score: number;
  hmm_state: string;
  hmm_state_icon: string;
  hmm_agrees: boolean;
  hmm_pts: number;
  whale_buys: number;
  whale_sells: number;
  long_ignitions: number;
  short_ignitions: number;
  last_price: number;
  change_pct: number;
  total_ticks: number;
  stop_loss: number | null;
  last_sound: SoundSnapshot | null;
  order_book: OrderBookSnapshot | null;
  alerts: AlertSnapshot[];
  ticks: TickSnapshot[];
  // Anti-spoof metrics
  whale_buy_volume_m: number;
  whale_sell_volume_m: number;
  aggressive_buys: number;
  aggressive_sells: number;
  cluster_count: number;
  trade_velocity: number;
  suspicious_adds: number;
  suspicious_removes: number;
  absorption_score: number;
  signal_confidence: number;
  book_delta: BookDeltaSummary;
}

export interface SoundSnapshot {
  label: string;
  timestamp: string;
}

export interface OrderBookSnapshot {
  asks: OrderLevelSnapshot[];
  bids: OrderLevelSnapshot[];
  last_price: number;
  last_side: string;
  spread: number;
  bid_pressure_pct: number;
  ask_pressure_pct: number;
}

export interface OrderLevelSnapshot {
  price: number;
  lots: number;
  label: string;
}

export interface AlertSnapshot {
  timestamp: string;
  icon: string;
  line1: string;
  line2: string;
  is_ignition: boolean;
}

export interface TickSnapshot {
  timestamp: string;
  side: string;
  arrow: string;
  price: number;
  amount_m: number;
  shares: number;
  whale_tag: string;
}

export interface BookDeltaSummary {
  bid_deltas: number[];  // [5]
  ask_deltas: number[];  // [5]
  suspicious_levels: SuspiciousLevel[];
  cumulative_bid_delta: number;
  cumulative_ask_delta: number;
}

export interface SuspiciousLevel {
  side: string;   // "bid" or "ask"
  level: number;  // 0-4
  kind: string;   // "sudden_add" or "sudden_remove"
  volume: number;
}
