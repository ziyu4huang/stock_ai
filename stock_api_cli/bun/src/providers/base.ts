/**
 * Base interface for data providers.
 */

import type { OHLCVBar, Quote } from "../types";

export interface DataProvider {
  /** Fetch OHLCV bars for the given period. */
  fetchOHLCV(symbol: string, period: string): Promise<OHLCVBar[]>;

  /** Fetch the latest quote for a symbol. */
  fetchQuote(symbol: string): Promise<Quote>;
}
