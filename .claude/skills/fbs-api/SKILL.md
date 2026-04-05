---
name: fbs-api
description: >
  This skill should be used when the user asks to "/fbs_api", "run fbs", "test fbs api",
  "fbs test", "fubon api", "富邦 api", "run fbs api test", "verify fbs", "setup fbs",
  "fbs login", "fbs history", "fbs realtime", "check fubon connection", or wants to
  run, verify, or set up the Fubon Neo SDK integration.
metadata:
  version: 1.0.0
---

# /fbs_api — Fubon Neo SDK: Setup, Verify & Test

Run, verify, and troubleshoot the Fubon Neo API (富邦新一代 API) integration.
Tests live in `tests/fbs_nodejs/` and use the SDK from `vendor/fubon-neo-2.2.8.tgz`.

## Quick Invocation

```
/fbs_api              # Run all tests + summary report (default)
/fbs_api setup        # Check prerequisites and guide setup
/fbs_api login        # Run login test only
/fbs_api history      # Run history (OHLCV) test only
/fbs_api realtime     # Run realtime books test only
```

## Prerequisites Checklist

Before running tests, verify:

1. `.env` file exists at project root with all 4 required keys:
   - `FUBON_PERSONAL_ID` — 身分證字號
   - `FUBON_API_KEY` — API Key (from Fubon account)
   - `FUBON_CERT_PATH` — Path to `.p12` cert (e.g., `.cert/fbs_XXXX.p12`)
   - `FUBON_CERT_PASS` — Cert password

2. SDK installed: `bun install` (installs from `vendor/fubon-neo-2.2.8.tgz`)

3. Risk declaration signed at: `https://wtv.fbs.com.tw/ContractWeb/_Contract`
   (required for API access; takes effect next business day after signing)

To check setup, run:
```bash
ls .env && bun pm ls 2>/dev/null | grep fubon-neo
```

For full setup details, read `references/setup.md`.

## Running Tests

### Run All + Summary (default for /fbs_api)

Use the combined test runner script:

```bash
bun --env-file=.env .claude/skills/fbs-api/scripts/run-all.ts
```

### Individual Tests

```bash
bun --env-file=.env tests/fbs_nodejs/login.ts
bun --env-file=.env tests/fbs_nodejs/history.ts
bun --env-file=.env tests/fbs_nodejs/realtime-books.ts
```

## Interpreting Results

### Login Test

| Result | Meaning | Action |
|--------|---------|--------|
| `Login OK: {...}` | Fully authenticated | Ready to trade |
| `無簽署完成API使用風險暨聲明書帳號` | Not yet signed risk declaration | Sign at Fubon website; takes effect next day. **Note: if message also says "連線測試成功", the credentials are correct — only the permission is pending.** |
| `Login failed: invalid api key` | Wrong API key | Check `FUBON_API_KEY` in `.env` |
| `Login failed: cert error` | Wrong cert path or password | Check `FUBON_CERT_PATH` and `FUBON_CERT_PASS` |

### History Test

- Returns 20-30 rows of daily OHLCV for 2330 (TSMC)
- Columns: `date open high low close volume change`
- Success: data rows printed to stdout
- Failure: `No data returned` — check login and initRealtime

### Realtime Books Test

- Auto-exits cleanly when market is closed (Mon–Fri 09:00–13:30 TST)
- During market hours: prints live 五檔 (best 5 bid/ask) updates every few seconds
- `No data received after 10s` = market just closed

## Summary Report Format

After running all tests, produce a markdown table:

```
| Test            | Status | Notes                          |
|-----------------|--------|--------------------------------|
| login.ts        | PASS   | Account: XXXXXXXX              |
| history.ts      | PASS   | 2330: 20 bars returned         |
| realtime-books.ts | SKIP | Market closed (HH:MM TST)     |
```

Status values: **PASS** / **FAIL** / **SKIP** (market closed)

## Common Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `Cannot find module 'fubon-neo'` | SDK not installed | Run `bun install` |
| `ENOENT .cert/...` | Cert file missing | Check `FUBON_CERT_PATH` |
| `process.env.FUBON_* is undefined` | Missing `.env` | Check `.env` exists and has all keys |
| `initRealtime` error | Network/SDK issue | Retry; check internet connection |

## Additional Resources

- **`references/setup.md`** — Full setup guide: .env config, cert creation, SDK update
- **`scripts/run-all.ts`** — Combined test runner with summary table
- **Full API docs:** `docs/fbs/llms-full.txt` (61k lines, LLM-optimized) — read for FBS API questions
- **Update docs:** `curl -o docs/fbs/llms-full.txt https://www.fbs.com.tw/TradeAPI/llms-full.txt`
- **Update SDK:** replace `vendor/fubon-neo-*.tgz`, update `package.json`, run `bun install`
