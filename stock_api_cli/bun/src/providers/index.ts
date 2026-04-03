/**
 * Provider registry — factory to get data providers by name.
 */

export type { DataProvider } from "./base";
import type { DataProvider } from "./base";
import { YahooProvider } from "./yahoo";
import { AlphaVantageProvider } from "./alpha_vantage";

const PROVIDERS: Record<string, () => DataProvider> = {
  yfinance: () => new YahooProvider(),
  av: () => new AlphaVantageProvider(),
};

export function getProvider(name: string): DataProvider {
  const factory = PROVIDERS[name];
  if (!factory) {
    throw new Error(
      `Unknown source '${name}'. Choose from: ${Object.keys(PROVIDERS).join(", ")}`
    );
  }
  return factory();
}
