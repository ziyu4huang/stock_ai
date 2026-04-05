# FBS Node.js API Tests

Test scripts for 富邦新一代 API (Fubon Neo API) using Node.js SDK v2.2.8.

## Prerequisites

1. Copy `.env.sample` to `.env` and fill in credentials:
   ```
   FUBON_PERSONAL_ID=  # 身分證字號
   FUBON_API_KEY=      # 申請的 API Key
   FUBON_CERT_PATH=    # 憑證路徑 e.g. .cert/fbs_XXXX.p12
   FUBON_CERT_PASS=    # 憑證密碼
   ```

2. Sign the API risk declaration (API使用風險暨聲明書):
   https://wtv.fbs.com.tw/ContractWeb/_Contract

## Scripts

| Script | Description |
|---|---|
| `login.ts` | Verify login and API key authentication |
| `history.ts` | Fetch historical OHLCV data for 2330 (last 30 days) |
| `realtime-books.ts` | Subscribe to live best 5-bid/ask orderbook for 2330 |

## Run

```bash
bun --env-file=.env tests/fbs_nodejs/login.ts
bun --env-file=.env tests/fbs_nodejs/history.ts
bun --env-file=.env tests/fbs_nodejs/realtime-books.ts
```

> `realtime-books.ts` auto-exits if market is closed (Mon-Fri 09:00-13:30 TST).

## Docs

- Full API reference: `docs/fbs/llms-full.txt`
- Update docs: `curl -o docs/fbs/llms-full.txt https://www.fbs.com.tw/TradeAPI/llms-full.txt`
- LLM build guide: https://www.fbs.com.tw/TradeAPI/docs/welcome/build-with-llm
