/**
 * CLI entry point: bun run src/cli.ts <command> [args]
 *
 * Commands:
 *   fetch SYMBOL  --source yfinance|av --period 1y
 *   quote SYMBOL  --source yfinance|av
 */

import { getProvider } from "./providers";
import type { FetchResult, Quote } from "./types";

// ── arg parsing ──

function parseArgs(args: string[]) {
  const command = args[0];
  if (!command || !["fetch", "quote"].includes(command)) {
    console.error("Usage: bun run src/cli.ts <fetch|quote> SYMBOL [--source yfinance|av] [--period 1y] [-o file]");
    process.exit(1);
  }

  const positional: string[] = [];
  const opts: Record<string, string> = {};

  let i = 1;
  while (i < args.length) {
    const a = args[i];
    if (a === "--source" || a === "--period" || a === "-o" || a === "--output") {
      const key = a.replace(/^-+/, "").replace(/^o$/, "output");
      opts[key] = args[i + 1];
      i += 2;
    } else if (a.startsWith("--")) {
      const [k, ...rest] = a.slice(2).split("=");
      opts[k] = rest.join("=") || args[i + 1];
      if (!rest.length) i += 2;
      else i += 1;
    } else {
      positional.push(a);
      i += 1;
    }
  }

  return { command, positional, opts };
}

// ── commands ──

async function cmdFetch(positional: string[], opts: Record<string, string>) {
  const symbol = positional[0];
  if (!symbol) {
    console.error("Error: SYMBOL is required");
    process.exit(1);
  }

  const source = opts.source || "yfinance";
  const period = opts.period || "1y";

  const provider = getProvider(source);
  const bars = await provider.fetchOHLCV(symbol, period);

  const result: FetchResult = {
    symbol,
    source,
    period,
    bars,
  };

  const json = JSON.stringify(result, null, 2);

  if (opts.output) {
    const fs = await import("fs/promises");
    await fs.writeFile(opts.output, json, "utf-8");
    console.log(`Saved ${bars.length} bars to ${opts.output}`);
  } else {
    console.log(json);
  }
}

async function cmdQuote(positional: string[], opts: Record<string, string>) {
  const symbol = positional[0];
  if (!symbol) {
    console.error("Error: SYMBOL is required");
    process.exit(1);
  }

  const source = opts.source || "yfinance";
  const provider = getProvider(source);
  const q: Quote = await provider.fetchQuote(symbol);

  const json = JSON.stringify(q, null, 2);

  if (opts.output) {
    const fs = await import("fs/promises");
    await fs.writeFile(opts.output, json, "utf-8");
    console.log(`Quote saved to ${opts.output}`);
  } else {
    console.log(json);
  }
}

// ── main ──

const { command, positional, opts } = parseArgs(process.argv.slice(2));

if (command === "fetch") {
  await cmdFetch(positional, opts);
} else if (command === "quote") {
  await cmdQuote(positional, opts);
}
