/**
 * Yahoo Finance data provider via yfinance-style HTTP API.
 *
 * Uses Yahoo Finance chart API directly (no Python dependency).
 */

import type { DataProvider } from "./base";
import type { OHLCVBar, Quote } from "../types";

const USER_AGENT =
  "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

const PERIOD_MAP: Record<string, string> = {
  "1mo": "1mo",
  "3mo": "3mo",
  "6mo": "6mo",
  "1y": "1y",
  "2y": "2y",
  "5y": "5y",
  max: "max",
};

const RANGE_INTERVAL: Record<string, string> = {
  "1mo": "1d",
  "3mo": "1d",
  "6mo": "1d",
  "1y": "1d",
  "2y": "1wk",
  "5y": "1wk",
  max: "1mo",
};

export class YahooProvider implements DataProvider {
  async fetchOHLCV(symbol: string, period: string): Promise<OHLCVBar[]> {
    const range = PERIOD_MAP[period] || "1y";
    const interval = RANGE_INTERVAL[range] || "1d";

    const url = `https://query1.finance.yahoo.com/v8/finance/chart/${encodeURIComponent(symbol)}?range=${range}&interval=${interval}`;
    const resp = await fetch(url, {
      headers: { "User-Agent": USER_AGENT },
    });

    if (!resp.ok) {
      throw new Error(`Yahoo Finance HTTP ${resp.status} for ${symbol}`);
    }

    const data = await resp.json();
    const result = data?.chart?.result?.[0];
    if (!result) {
      throw new Error(`No data in Yahoo Finance response for ${symbol}`);
    }

    const timestamps: number[] = result.timestamp || [];
    const quote = result.indicators?.quote?.[0];
    if (!quote) {
      throw new Error(`No quote data in Yahoo Finance response for ${symbol}`);
    }

    const bars: OHLCVBar[] = [];
    for (let i = 0; i < timestamps.length; i++) {
      const o = quote.open?.[i];
      const h = quote.high?.[i];
      const l = quote.low?.[i];
      const c = quote.close?.[i];
      const v = quote.volume?.[i];
      // Skip incomplete bars
      if (o == null || h == null || l == null || c == null || v == null) continue;

      const d = new Date(timestamps[i] * 1000);
      const dateStr = d.toISOString().split("T")[0];

      bars.push({
        date: dateStr,
        open: Math.round(o * 100) / 100,
        high: Math.round(h * 100) / 100,
        low: Math.round(l * 100) / 100,
        close: Math.round(c * 100) / 100,
        volume: Math.round(v),
      });
    }

    if (bars.length === 0) {
      throw new Error(`No bars returned for ${symbol} (Yahoo Finance)`);
    }

    return bars;
  }

  async fetchQuote(symbol: string): Promise<Quote> {
    // Fetch 5-day history to get latest + previous close
    const url = `https://query1.finance.yahoo.com/v8/finance/chart/${encodeURIComponent(symbol)}?range=5d&interval=1d`;
    const resp = await fetch(url, {
      headers: { "User-Agent": USER_AGENT },
    });

    if (!resp.ok) {
      throw new Error(`Yahoo Finance HTTP ${resp.status} for ${symbol}`);
    }

    const data = await resp.json();
    const result = data?.chart?.result?.[0];
    if (!result) {
      throw new Error(`No data in Yahoo Finance response for ${symbol}`);
    }

    const meta = result.meta;
    const timestamps: number[] = result.timestamp || [];
    const quote = result.indicators?.quote?.[0];

    const price = meta?.regularMarketPrice ?? 0;
    const prevClose = meta?.chartPreviousClose ?? meta?.previousClose ?? 0;
    const change = price - prevClose;
    const changePct = prevClose > 0 ? (change / prevClose) * 100 : 0;

    // Get last bar's OHLCV
    let high = 0, low = 0, open = 0, volume = 0;
    if (quote && timestamps.length > 0) {
      const last = timestamps.length - 1;
      high = quote.high?.[last] ?? 0;
      low = quote.low?.[last] ?? 0;
      open = quote.open?.[last] ?? 0;
      volume = quote.volume?.[last] ?? 0;
    }

    return {
      symbol,
      name: meta?.shortName ?? meta?.longName ?? symbol,
      price: Math.round(price * 100) / 100,
      change: Math.round(change * 100) / 100,
      change_pct: Math.round(changePct * 100) / 100,
      volume: Math.round(volume),
      high: Math.round(high * 100) / 100,
      low: Math.round(low * 100) / 100,
      open: Math.round(open * 100) / 100,
      prev_close: Math.round(prevClose * 100) / 100,
    };
  }
}
