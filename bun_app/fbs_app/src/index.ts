import { handleIntradayCandles, handleIntradayQuote, handleIntradayTrades, handleIntradayVolumes, handleHistoricalCandles } from './api/market';
import { readFileSync, writeFileSync } from 'fs';
import { resolve } from 'path';

const PORT = parseInt(process.env.PORT || '3200');
const HTML_PATH = import.meta.dir + '/public/index.html';
const WL_PATH = resolve(import.meta.dir, '..', 'data', 'watchlist.json');

const DEFAULT_WL = { groups: [{ name: '自選一', stocks: [] }], activeGroup: 0 };

function loadWL() {
  try { return JSON.parse(readFileSync(WL_PATH, 'utf-8')); } catch { return { ...DEFAULT_WL }; }
}
function saveWL(data: any) {
  writeFileSync(WL_PATH, JSON.stringify(data, null, 2), 'utf-8');
}

const server = Bun.serve({
  port: PORT,
  async fetch(req) {
    const url = new URL(req.url);

    // API routes
    if (url.pathname === '/api/intraday/candles') return handleIntradayCandles(req).catch(e => Response.json({ error: e.message }, { status: 500 }));
    if (url.pathname === '/api/intraday/quote') return handleIntradayQuote(req).catch(e => Response.json({ error: e.message }, { status: 500 }));
    if (url.pathname === '/api/intraday/trades') return handleIntradayTrades(req).catch(e => Response.json({ error: e.message }, { status: 500 }));
    if (url.pathname === '/api/intraday/volumes') return handleIntradayVolumes(req).catch(e => Response.json({ error: e.message }, { status: 500 }));
    if (url.pathname === '/api/historical/candles') return handleHistoricalCandles(req).catch(e => Response.json({ error: e.message }, { status: 500 }));

    // Watchlist CRUD
    if (url.pathname === '/api/watchlist' && req.method === 'GET') {
      return Response.json(loadWL());
    }
    if (url.pathname === '/api/watchlist' && req.method === 'POST') {
      const body = await req.json();
      saveWL(body);
      return Response.json({ ok: true });
    }

    // Serve index.html for everything else
    const file = Bun.file(HTML_PATH);
    return new Response(file, {
      headers: { 'Content-Type': 'text/html; charset=utf-8' },
    });
  },
});

console.log(`FBS Stock App running at http://localhost:${server.port}`);
