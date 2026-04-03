/**
 * Shared types for stock_api_cli (Bun/TypeScript).
 */

export interface OHLCVBar {
  date: string;
  open: number;
  high: number;
  low: number;
  close: number;
  volume: number;
}

export interface Quote {
  symbol: string;
  name: string;
  price: number;
  change: number;
  change_pct: number;
  volume: number;
  high: number;
  low: number;
  open: number;
  prev_close: number;
}

export interface FetchResult {
  symbol: string;
  source: string;
  period: string;
  bars: OHLCVBar[];
}
