/**
 * Alpha Vantage data provider (TIME_SERIES_DAILY + GLOBAL_QUOTE).
 *
 * Requires AV_API_KEY env var. Free tier: 5 calls/min, 500/day.
 */

import type { DataProvider } from "./base";
import type { OHLCVBar, Quote } from "../types";

const BASE_URL = "https://www.alphavantage.co/query";

const callTimes: number[] = [];
const RATE_LIMIT = 5;
const RATE_WINDOW = 61_000; // ms

function throttle(): Promise<void> {
  return new Promise((resolve) => {
    const now = Date.now();
    // Clean old timestamps
    while (callTimes.length > 0 && now - callTimes[0]! >= RATE_WINDOW) {
      callTimes.shift();
    }
    if (callTimes.length >= RATE_LIMIT) {
      const wait = RATE_WINDOW - (now - callTimes[0]!) + 500;
      if (wait > 0) {
        console.log(`      [AV] rate-limit: waiting ${Math.round(wait / 1000)}s ...`);
        setTimeout(() => {
          callTimes.push(Date.now());
          resolve();
        }, wait);
        return;
      }
    }
    callTimes.push(Date.now());
    resolve();
  });
}

function checkAVErrors(data: Record<string, unknown>, symbol: string): void {
  if ("Error Message" in data) {
    throw new Error(`AV error for ${symbol}: ${data["Error Message"]}`);
  }
  if ("Note" in data) {
    throw new Error(`AV rate limit: ${data["Note"]}`);
  }
  if ("Information" in data) {
    throw new Error(`AV info: ${data["Information"]}`);
  }
}

export class AlphaVantageProvider implements DataProvider {
  private apiKey: string;

  constructor(apiKey?: string) {
    this.apiKey = apiKey ?? process.env.AV_API_KEY ?? "";
    if (!this.apiKey) {
      throw new Error("AV_API_KEY not set (env var or constructor arg)");
    }
  }

  async fetchOHLCV(symbol: string, period: string): Promise<OHLCVBar[]> {
    await throttle();

    const outputsize = ["2y", "5y", "max"].includes(period) ? "full" : "compact";
    const params = new URLSearchParams({
      function: "TIME_SERIES_DAILY",
      symbol,
      outputsize,
      apikey: this.apiKey,
    });

    const resp = await fetch(`${BASE_URL}?${params}`, { signal: AbortSignal.timeout(30_000) });
    if (!resp.ok) throw new Error(`AV HTTP ${resp.status} for ${symbol}`);

    const data = (await resp.json()) as Record<string, unknown>;
    checkAVErrors(data, symbol);

    const tsKey = "Time Series (Daily)";
    const rows = data[tsKey] as Record<string, Record<string, string>> | undefined;
    if (!rows) throw new Error(`No time series in AV response for ${symbol}`);

    const periodDays: Record<string, number> = {
      "1mo": 30, "3mo": 90, "6mo": 180, "1y": 365, "2y": 730, "5y": 1825,
    };
    const days = periodDays[period] ?? 365;
    const cutoff = new Date();
    cutoff.setDate(cutoff.getDate() - days);

    const bars: OHLCVBar[] = [];
    // Sort dates ascending
    const sortedDates = Object.keys(rows).sort();

    for (const dateStr of sortedDates) {
      const d = new Date(dateStr);
      if (period !== "max" && d < cutoff) continue;

      const vals = rows[dateStr]!;
      bars.push({
        date: dateStr,
        open: Math.round(parseFloat(vals["1. open"]) * 100) / 100,
        high: Math.round(parseFloat(vals["2. high"]) * 100) / 100,
        low: Math.round(parseFloat(vals["3. low"]) * 100) / 100,
        close: Math.round(parseFloat(vals["4. close"]) * 100) / 100,
        volume: parseInt(vals["5. volume"], 10),
      });
    }

    if (bars.length === 0) {
      throw new Error(`No data for ${symbol} (Alpha Vantage)`);
    }

    return bars;
  }

  async fetchQuote(symbol: string): Promise<Quote> {
    await throttle();

    const params = new URLSearchParams({
      function: "GLOBAL_QUOTE",
      symbol,
      apikey: this.apiKey,
    });

    const resp = await fetch(`${BASE_URL}?${params}`, { signal: AbortSignal.timeout(30_000) });
    if (!resp.ok) throw new Error(`AV HTTP ${resp.status} for ${symbol}`);

    const data = (await resp.json()) as Record<string, unknown>;
    checkAVErrors(data, symbol);

    const gq = data["Global Quote"] as Record<string, string> | undefined;
    if (!gq || Object.keys(gq).length === 0) {
      throw new Error(`No quote data from AV for ${symbol}`);
    }

    const price = parseFloat(gq["05. price"] ?? "0");
    const prevClose = parseFloat(gq["08. previous close"] ?? "0");
    const change = parseFloat(gq["09. change"] ?? "0");
    const changePctStr = (gq["10. change percent"] ?? "0%").replace("%", "");

    return {
      symbol,
      name: symbol, // AV GLOBAL_QUOTE doesn't return name
      price: Math.round(price * 100) / 100,
      change: Math.round(change * 100) / 100,
      change_pct: Math.round(parseFloat(changePctStr) * 100) / 100,
      volume: parseInt(gq["06. volume"] ?? "0", 10),
      high: Math.round(parseFloat(gq["03. high"] ?? "0") * 100) / 100,
      low: Math.round(parseFloat(gq["04. low"] ?? "0") * 100) / 100,
      open: Math.round(parseFloat(gq["02. open"] ?? "0") * 100) / 100,
      prev_close: Math.round(prevClose * 100) / 100,
    };
  }
}
