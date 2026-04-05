# FBS API Setup Reference

## Environment File (.env)

Create `.env` at project root (copy from `.env.sample` if it exists):

```dotenv
FUBON_PERSONAL_ID=A123456789      # 身分證字號 (10 chars)
FUBON_API_KEY=your-api-key-here   # From Fubon account portal
FUBON_CERT_PATH=.cert/fbs_XXXX.p12
FUBON_CERT_PASS=your-cert-password
```

The `.env` file is gitignored — never commit it.

### Getting API Key

1. Log in to Fubon e-banking or the trading portal
2. Navigate to API management / 新一代 API 申請
3. Generate and copy the API Key

### Cert File (.p12)

The `.p12` certificate is the same one used for Fubon's trading system login.

Default storage recommendation: `.cert/` directory at project root (gitignored).

```bash
mkdir -p .cert
# Copy your .p12 file here
cp ~/Downloads/fbs_XXXX.p12 .cert/
```

Update `FUBON_CERT_PATH` in `.env` to match the actual filename.

## SDK Installation

The Fubon Neo SDK is vendored at `vendor/fubon-neo-2.2.8.tgz` and referenced in `package.json`.

```bash
bun install
```

Verify installation:
```bash
bun pm ls 2>/dev/null | grep fubon-neo
# or check node_modules
ls node_modules/fubon-neo 2>/dev/null && echo "installed"
```

### Updating the SDK

When a new SDK version is released:

1. Download the new `.tgz` from Fubon developer portal
2. Replace `vendor/fubon-neo-X.X.X.tgz`
3. Update `package.json`: change the `fubon-neo` version entry filename
4. Run `bun install`

## API Risk Declaration (API使用風險暨聲明書)

**Required before any API call will succeed.**

Sign at: `https://wtv.fbs.com.tw/ContractWeb/_Contract`

- Takes effect: next business day after signing
- Until signed, login returns: "無簽署完成API使用風險暨聲明書帳號"
- If the error message also says "此訊息表連線測試成功": credentials are correct, only permission is pending

## Market Hours (TST = Asia/Taipei)

| Session | Hours |
|---------|-------|
| Regular trading | Mon–Fri 09:00–13:30 |
| Pre-market (盤前) | 08:30–09:00 (some data) |
| Post-market (盤後) | 13:30+ (no realtime) |

The `realtime-books.ts` test auto-exits when market is closed.

## Troubleshooting Quick Reference

```bash
# Check .env loaded correctly
bun -e "console.log(process.env.FUBON_PERSONAL_ID)" --env-file=.env

# Check cert file exists
ls -la $(grep FUBON_CERT_PATH .env | cut -d= -f2)

# Check SDK is installed
ls node_modules/fubon-neo/package.json

# Full test run
bun --env-file=.env tests/fbs_nodejs/login.ts
```
