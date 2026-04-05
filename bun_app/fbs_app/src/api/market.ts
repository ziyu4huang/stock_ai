import { getSDK } from '../sdk';

export async function handleIntradayCandles(req: Request): Promise<Response> {
  const url = new URL(req.url);
  const symbol = url.searchParams.get('symbol') || '2330';
  const timeframe = url.searchParams.get('timeframe') || '1';

  const sdk = await getSDK();
  const client = sdk.marketdata.restClient;

  try {
    const data = await client.stock.intraday.candles({ symbol, timeframe });
    return Response.json(data);
  } catch (err: any) {
    return Response.json({ error: err.message }, { status: 500 });
  }
}

export async function handleIntradayQuote(req: Request): Promise<Response> {
  const url = new URL(req.url);
  const symbol = url.searchParams.get('symbol') || '2330';

  const sdk = await getSDK();
  const client = sdk.marketdata.restClient;

  try {
    const data = await client.stock.intraday.quote({ symbol });
    return Response.json(data);
  } catch (err: any) {
    return Response.json({ error: err.message }, { status: 500 });
  }
}

export async function handleHistoricalCandles(req: Request): Promise<Response> {
  const url = new URL(req.url);
  const symbol = url.searchParams.get('symbol') || '2330';
  const timeframe = url.searchParams.get('timeframe') || '1';
  const from = url.searchParams.get('from') || new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString().slice(0, 10);
  const to = url.searchParams.get('to') || new Date().toISOString().slice(0, 10);

  const sdk = await getSDK();
  const client = sdk.marketdata.restClient;

  try {
    const data = await client.stock.historical.candles({
      symbol,
      timeframe,
      from,
      to,
      fields: 'open,high,low,close,volume,change',
    });
    return Response.json(data);
  } catch (err: any) {
    return Response.json({ error: err.message }, { status: 500 });
  }
}
