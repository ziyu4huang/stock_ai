// Subscribe to real-time best 5-bid/ask (books) for 2330
// Exits with a message if market is closed
import { FubonSDK, Mode } from 'fubon-neo';

const SYMBOL = '2330';
const MARKET_OPEN_HOUR  = 9;
const MARKET_CLOSE_HOUR = 13;
const MARKET_CLOSE_MIN  = 30;
const WAIT_SECONDS      = 10;

function isTradingHours(): boolean {
  const now = new Date();
  const day = now.getDay(); // 0=Sun, 6=Sat
  if (day === 0 || day === 6) return false;
  const h = now.getHours();
  const m = now.getMinutes();
  const totalMin = h * 60 + m;
  return totalMin >= MARKET_OPEN_HOUR * 60 && totalMin < MARKET_CLOSE_HOUR * 60 + MARKET_CLOSE_MIN;
}

if (!isTradingHours()) {
  const now = new Date().toLocaleString('zh-TW', { timeZone: 'Asia/Taipei' });
  console.log(`Market is closed (${now})`);
  console.log('Trading hours: Mon-Fri 09:00-13:30 TST');
  process.exit(0);
}

const sdk = new FubonSDK();

sdk.apikeyLogin(
  process.env.FUBON_PERSONAL_ID!,
  process.env.FUBON_API_KEY!,
  process.env.FUBON_CERT_PATH!,
  process.env.FUBON_CERT_PASS!
);

sdk.initRealtime(Mode.Speed);

const stock = sdk.marketdata.webSocketClient.stock;

let dataReceived = false;

stock.on('message', (message: string) => {
  const msg = JSON.parse(message);
  if (msg.channel !== 'books') return;

  dataReceived = true;
  const { bids, asks, time } = msg.data;
  const ts = new Date(time / 1000).toLocaleTimeString('zh-TW', { timeZone: 'Asia/Taipei' });

  console.log(`\n[${ts}] ${SYMBOL} 五檔`);
  console.log('  委賣 asks:');
  [...(asks ?? [])].reverse().forEach((a: any) => console.log(`    ${a.price} × ${a.size}`));
  console.log('  --------');
  console.log('  委買 bids:');
  (bids ?? []).forEach((b: any) => console.log(`    ${b.price} × ${b.size}`));
});

stock.on('connect',    () => console.log('connected'));
stock.on('disconnect', () => console.log('disconnected'));
stock.on('error',      (err: any) => console.error('error:', err));

await stock.connect();
stock.subscribe({ channel: 'books', symbol: SYMBOL });
console.log(`Subscribed to ${SYMBOL} books, waiting for data...`);

// If no data after WAIT_SECONDS, assume market just closed
setTimeout(() => {
  if (!dataReceived) {
    console.log(`\nNo data received after ${WAIT_SECONDS}s — market may have just closed.`);
    process.exit(0);
  }
}, WAIT_SECONDS * 1000);
