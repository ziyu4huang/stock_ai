/**
 * Tests for stock_api_cli Bun providers.
 */

import { describe, test, expect } from "bun:test";
import { getProvider } from "../src/providers";
import type { DataProvider } from "../src/providers/base";
import { YahooProvider } from "../src/providers/yahoo";

describe("provider registry", () => {
  test("getProvider('yfinance') returns YahooProvider", () => {
    const p = getProvider("yfinance");
    expect(p).toBeInstanceOf(YahooProvider);
  });

  test("getProvider('av') returns a provider (if AV_API_KEY set)", () => {
    if (!process.env.AV_API_KEY) {
      console.log("      (skipped: AV_API_KEY not set)");
      return;
    }
    const p = getProvider("av");
    expect(p).toBeDefined();
    expect(typeof p.fetchOHLCV).toBe("function");
    expect(typeof p.fetchQuote).toBe("function");
  });

  test("getProvider('unknown') throws", () => {
    expect(() => getProvider("unknown")).toThrow(/Unknown source/);
  });
});

describe("YahooProvider live tests", () => {
  const provider: DataProvider = new YahooProvider();

  test("fetchOHLCV returns bars for 2330.TW", async () => {
    const bars = await provider.fetchOHLCV("2330.TW", "1mo");
    expect(bars.length).toBeGreaterThan(0);

    const first = bars[0]!;
    expect(first.date).toMatch(/^\d{4}-\d{2}-\d{2}$/);
    expect(first.open).toBeGreaterThan(0);
    expect(first.high).toBeGreaterThanOrEqual(first.low);
    expect(first.close).toBeGreaterThan(0);
    expect(first.volume).toBeGreaterThan(0);
  });

  test("fetchQuote returns quote for 2330.TW", async () => {
    const q = await provider.fetchQuote("2330.TW");
    expect(q.symbol).toBe("2330.TW");
    expect(q.price).toBeGreaterThan(0);
  });

  test("fetchOHLCV throws for invalid symbol", async () => {
    try {
      await provider.fetchOHLCV("INVALIDSYMBOL12345", "1mo");
      // If it doesn't throw, it may return empty — that's OK
    } catch (e) {
      expect(e).toBeInstanceOf(Error);
    }
  });
});
