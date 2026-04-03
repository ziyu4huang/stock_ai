#!/usr/bin/env bun
/**
 * Quick Yahoo Finance query CLI for Claude Code.
 * Usage: bun run yf-query.ts <command> <symbol> [options]
 * Commands: quote, history, info
 * Options: --no-cache  force fresh fetch (bypass cache)
 */
import YahooFinance from "yahoo-finance2";
import { existsSync, mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const yf = new YahooFinance({ suppressNotices: ["yahooSurvey", "ripHistorical"] });
const [,, cmd, symbol, ...opts] = process.argv;
const noCache = opts.includes("--no-cache");

if (!cmd || !symbol) {
  console.error("Usage: bun run yf-query.ts <quote|history|info> <SYMBOL> [period] [--no-cache]");
  process.exit(1);
}

// --- Cache layer ---
const CACHE_DIR = join(import.meta.dir, ".cache", "analyze-stock");
const TTL_MS: Record<string, number> = {
  quote:   5 * 60 * 1000,      // 5 min
  history: 15 * 60 * 1000,     // 15 min
  info:    24 * 60 * 60 * 1000, // 24 hours
};

function cachePath(key: string): string {
  const safeName = symbol.replace(/[^a-zA-Z0-9._-]/g, "_");
  return join(CACHE_DIR, cmd, `${safeName}_${key}.json`);
}

function readCache(key: string): any | null {
  if (noCache) return null;
  const file = cachePath(key);
  if (!existsSync(file)) return null;
  try {
    const raw = JSON.parse(readFileSync(file, "utf-8"));
    if (Date.now() - new Date(raw.timestamp).getTime() < (TTL_MS[cmd] ?? 0)) {
      return raw.data;
    }
  } catch { /* corrupted cache, re-fetch */ }
  return null;
}

function writeCache(key: string, data: any): void {
  const file = cachePath(key);
  mkdirSync(join(CACHE_DIR, cmd), { recursive: true });
  writeFileSync(file, JSON.stringify({ timestamp: new Date().toISOString(), data }, null, 2));
}

// --- Commands ---
async function main() {
  switch (cmd) {
    case "quote": {
      const cached = readCache("latest");
      if (cached) { console.log(JSON.stringify(cached, null, 2)); return; }
      const q = await yf.quote(symbol);
      const data = {
        symbol: q.symbol,
        name: q.shortName,
        price: q.regularMarketPrice,
        change: q.regularMarketChange,
        changePct: q.regularMarketChangePercent,
        open: q.regularMarketOpen,
        high: q.regularMarketDayHigh,
        low: q.regularMarketDayLow,
        volume: q.regularMarketVolume,
        prevClose: q.regularMarketPreviousClose,
        marketCap: q.marketCap,
        pe: q.trailingPE,
        eps: q.epsTrailingTwelveMonths,
        fiftyTwoWeekHigh: q.fiftyTwoWeekHigh,
        fiftyTwoWeekLow: q.fiftyTwoWeekLow,
      };
      writeCache("latest", data);
      console.log(JSON.stringify(data, null, 2));
      break;
    }
    case "history": {
      const days = parseInt(opts.find(o => !o.startsWith("--")) ?? "30") || 30;
      const cacheKey = `${days}d`;
      const cached = readCache(cacheKey);
      if (cached) { console.log(JSON.stringify(cached, null, 2)); return; }
      const period1 = new Date(Date.now() - days * 86400000);
      const result = await yf.chart(symbol, { period1, interval: "1d" });
      const rows = (result.quotes ?? []).map((r: any) => ({
        date: new Date(r.date).toISOString().slice(0, 10),
        open: r.open, high: r.high, low: r.low, close: r.close,
        volume: r.volume,
      }));
      writeCache(cacheKey, rows);
      console.log(JSON.stringify(rows, null, 2));
      break;
    }
    case "info": {
      const cached = readCache("summary");
      if (cached) { console.log(JSON.stringify(cached, null, 2)); return; }
      const summary = await yf.quoteSummary(symbol, {
        modules: ["price", "summaryDetail", "financialData", "defaultKeyStatistics"]
      });
      writeCache("summary", summary);
      console.log(JSON.stringify(summary, null, 2));
      break;
    }
    default:
      console.error(`Unknown command: ${cmd}. Use quote, history, or info.`);
      process.exit(1);
  }
}
main().catch(e => { console.error(e.message); process.exit(1); });
