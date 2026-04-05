// Get historical OHLCV data for 2330, similar to yfinance API
import { FubonSDK, Mode } from 'fubon-neo';

const sdk = new FubonSDK();

sdk.apikeyLogin(
  process.env.FUBON_PERSONAL_ID!,
  process.env.FUBON_API_KEY!,
  process.env.FUBON_CERT_PATH!,
  process.env.FUBON_CERT_PASS!
);

sdk.initRealtime(Mode.Speed);

const rest = sdk.marketdata.restClient.stock;

const to   = new Date().toISOString().slice(0, 10);
const from = new Date(Date.now() - 30 * 24 * 60 * 60 * 1000).toISOString().slice(0, 10);

console.log(`Fetching 2330 history from ${from} to ${to}...`);

const data = await rest.historical.candles({
  symbol: '2330',
  from,
  to,
  fields: 'open,high,low,close,volume,change',
});

if (!data?.data?.length) {
  console.log('No data returned');
  process.exit(1);
}

console.log(`\n2330 歷史資料 (${data.data.length} 筆)\n`);
console.log('date        open    high    low     close   volume      change');
console.log('----------  ------  ------  ------  ------  ----------  ------');
for (const bar of data.data) {
  console.log(
    `${bar.date}  ${String(bar.open).padStart(6)}  ${String(bar.high).padStart(6)}  ${String(bar.low).padStart(6)}  ${String(bar.close).padStart(6)}  ${String(bar.volume).padStart(10)}  ${bar.change}`
  );
}
