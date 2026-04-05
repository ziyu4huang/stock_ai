import { chromium } from 'playwright';

const BASE = 'http://localhost:3200';

function assert(cond, msg) {
  if (!cond) throw new Error(msg || 'Assertion failed');
}

async function main() {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  let passed = 0;
  let failed = 0;

  // Collect console logs and network errors from the page
  const pageLogs = [];
  const pageErrors = [];
  const networkRequests = [];
  page.on('console', msg => pageLogs.push(`[${msg.type()}] ${msg.text()}`));
  page.on('pageerror', err => pageErrors.push(err.message));
  page.on('requestfailed', req => networkRequests.push(`FAIL ${req.method()} ${req.url()}: ${req.failure()?.errorText}`));

  async function test(name, fn) {
    try {
      await fn();
      console.log(`  ✓ ${name}`);
      passed++;
    } catch (err) {
      console.log(`  ✗ ${name}: ${err.message}`);
      // Dump debug info on failure
      if (pageErrors.length) console.log(`    page errors: ${pageErrors.slice(-3).join('; ')}`);
      if (networkRequests.length) console.log(`    network failures: ${networkRequests.slice(-3).join('; ')}`);
      failed++;
    }
  }

  console.log('\n🧪 FBS App — Playwright Tests\n');

  // ========================================
  // SECTION 1: Page load & UI structure
  // ========================================

  await test('Page loads with correct title', async () => {
    await page.goto(BASE, { waitUntil: 'networkidle' });
    const title = await page.title();
    assert(title === 'FBS Stock', `Expected "FBS Stock", got "${title}"`);
  });

  await test('Tab bar has logo and symbol input', async () => {
    const logoText = await page.locator('.tab-bar .logo').textContent();
    assert(logoText === 'FBS Stock', `Expected "FBS Stock", got "${logoText}"`);
    const value = await page.locator('#symbolInput').inputValue();
    assert(value === '2330', `Expected "2330", got "${value}"`);
  });

  await test('Mode buttons: LIVE active, History inactive', async () => {
    const liveCls = await page.locator('.mode-btn[data-mode="live"]').getAttribute('class');
    const histCls = await page.locator('.mode-btn[data-mode="history"]').getAttribute('class');
    assert(liveCls.includes('active'), 'LIVE should be active');
    assert(!histCls.includes('active'), 'History should not be active');
  });

  await test('Timeframe 1min is active by default', async () => {
    const cls = await page.locator('.tf-btn[data-tf="1"]').getAttribute('class');
    assert(cls.includes('active'), '1min should be active');
  });

  await test('Quote header has placeholder elements', async () => {
    assert(await page.locator('#priceDisplay').isVisible(), 'Price display should be visible');
    assert(await page.locator('.ohlc').isVisible(), 'OHLC should be visible');
  });

  await test('Chart container renders', async () => {
    const chart = page.locator('#chart');
    assert(await chart.isVisible(), 'Chart div should be visible');
    const box = await chart.boundingBox();
    assert(box && box.width > 100, 'Chart should have reasonable width');
  });

  await test('Footer stats visible', async () => {
    assert(await page.locator('.footer').isVisible(), 'Footer should be visible');
    assert(await page.locator('#fVol').isVisible(), 'Volume stat visible');
    assert(await page.locator('#fVal').isVisible(), 'Value stat visible');
  });

  // ========================================
  // SECTION 2: Mode switching
  // ========================================

  await test('Switch to History mode shows date picker with last trade date', async () => {
    await page.locator('.mode-btn[data-mode="history"]').click();
    await page.waitForTimeout(200);
    assert(await page.locator('#datePicker').isVisible(), 'Date picker should be visible in history mode');
    const dpVal = await page.locator('#datePicker').inputValue();
    assert(dpVal.length === 10, `Date picker should have a date, got "${dpVal}"`);
    // Should be a weekday (Mon-Fri)
    const d = new Date(dpVal);
    const dow = d.getDay();
    assert(dow >= 1 && dow <= 5, `Date should be a weekday, got day-of-week=${dow}`);
    console.log(`    default date: ${dpVal} (${['Sun','Mon','Tue','Wed','Thu','Fri','Sat'][dow]})`);
  });

  await test('Switch back to Live hides date picker', async () => {
    await page.locator('.mode-btn[data-mode="live"]').click();
    await page.waitForTimeout(200);
    assert(!(await page.locator('#datePicker').isVisible()), 'Date picker should be hidden in live mode');
  });

  // ========================================
  // SECTION 3: Timeframe & symbol changes
  // ========================================

  await test('Change timeframe to 5min', async () => {
    await page.locator('.tf-btn[data-tf="5"]').click();
    const cls5 = await page.locator('.tf-btn[data-tf="5"]').getAttribute('class');
    const cls1 = await page.locator('.tf-btn[data-tf="1"]').getAttribute('class');
    assert(cls5.includes('active'), '5min should now be active');
    assert(!cls1.includes('active'), '1min should be inactive');
  });

  await test('Change symbol input', async () => {
    const input = page.locator('#symbolInput');
    await input.fill('');
    await input.type('2454');
    await input.press('Enter');
    const val = await input.inputValue();
    assert(val === '2454', `Expected "2454", got "${val}"`);
  });

  // ========================================
  // SECTION 4: API endpoints
  // ========================================

  await test('API /api/intraday/candles returns JSON', async () => {
    const resp = await page.goto(`${BASE}/api/intraday/candles?symbol=2330&timeframe=1`);
    const contentType = resp.headers()['content-type'] || '';
    assert(contentType.includes('json'), `Expected JSON, got "${contentType}"`);
    const body = await resp.json();
    // SDK login may fail — check structure only
    assert(body !== null, 'Body should be non-null');
  });

  await test('API /api/intraday/quote returns JSON', async () => {
    const resp = await page.goto(`${BASE}/api/intraday/quote?symbol=2330`);
    const contentType = resp.headers()['content-type'] || '';
    assert(contentType.includes('json'), `Expected JSON, got "${contentType}"`);
    const body = await resp.json();
    assert(body !== null, 'Body should be non-null');
  });

  await test('API /api/historical/candles returns JSON with from/to', async () => {
    const resp = await page.goto(`${BASE}/api/historical/candles?symbol=2330&timeframe=1&from=2025-03-01&to=2025-03-31`);
    const contentType = resp.headers()['content-type'] || '';
    assert(contentType.includes('json'), `Expected JSON, got "${contentType}"`);
    const body = await resp.json();
    assert(body !== null, 'Body should be non-null');
    console.log(`    response keys: ${Object.keys(body).join(', ')}`);
    if (body.data) console.log(`    data count: ${body.data.length}`);
    if (body.error) console.log(`    error: ${body.error}`);
  });

  // ========================================
  // SECTION 5: History mode E2E flow
  // ========================================

  await test('History mode: pick date triggers API call', async () => {
    await page.goto(BASE, { waitUntil: 'networkidle' });
    // Switch to history mode
    await page.locator('.mode-btn[data-mode="history"]').click();
    await page.waitForTimeout(200);
    assert(await page.locator('#datePicker').isVisible(), 'Date picker visible');

    // Listen for API request
    const apiPromise = page.waitForRequest(req => req.url().includes('/api/historical/candles'), { timeout: 5000 }).catch(() => null);
    const apiResponsePromise = page.waitForResponse(resp => resp.url().includes('/api/historical/candles'), { timeout: 5000 }).catch(() => null);

    // Pick a date
    await page.locator('#datePicker').fill('2025-04-01');

    const req = await apiPromise;
    const resp = await apiResponsePromise;
    assert(req !== null, 'Should trigger /api/historical/candles request');
    if (req) {
      const url = new URL(req.url());
      assert(url.searchParams.get('from') !== null, `from param should be set, url: ${req.url()}`);
      assert(url.searchParams.get('to') !== null, `to param should be set, url: ${req.url()}`);
      console.log(`    API called: ${req.url()}`);
    }
    if (resp) {
      const status = resp.status();
      console.log(`    response status: ${status}`);
      const body = await resp.json().catch(() => null);
      if (body?.data) console.log(`    data count: ${body.data.length}`);
      if (body?.error) console.log(`    error: ${body.error}`);
    }
  });

  // ========================================
  // SECTION 6: Live mode E2E flow
  // ========================================

  await test('Live mode: auto-fetch on load triggers API', async () => {
    // Reload in live mode (default)
    const candlesPromise = page.waitForResponse(resp => resp.url().includes('/api/intraday/candles'), { timeout: 5000 }).catch(() => null);
    const quotePromise = page.waitForResponse(resp => resp.url().includes('/api/intraday/quote'), { timeout: 5000 }).catch(() => null);

    await page.goto(BASE, { waitUntil: 'networkidle' });

    const candlesResp = await candlesPromise;
    const quoteResp = await quotePromise;
    assert(candlesResp !== null, 'Should fetch intraday candles on load');
    assert(quoteResp !== null, 'Should fetch intraday quote on load');

    if (candlesResp) {
      const body = await candlesResp.json().catch(() => null);
      console.log(`    candles status: ${candlesResp.status()}, has data: ${!!body?.data}`);
      if (body?.error) console.log(`    candles error: ${body.error}`);
    }
    if (quoteResp) {
      const body = await quoteResp.json().catch(() => null);
      console.log(`    quote status: ${quoteResp.status()}, keys: ${body ? Object.keys(body).slice(0, 5).join(',') : 'null'}`);
      if (body?.error) console.log(`    quote error: ${body.error}`);
    }
  });

  // ========================================
  // SECTION 7: Screenshot
  // ========================================

  await test('Screenshot captures page', async () => {
    await page.goto(BASE, { waitUntil: 'networkidle' });
    await page.waitForTimeout(1000);
    await page.screenshot({ path: 'tests/screenshot.png', fullPage: true });
  });

  // --- Summary ---
  console.log(`\n  ${passed} passed, ${failed} failed (${passed + failed} total)\n`);

  await browser.close();

  if (failed > 0) process.exit(1);
}

main().catch(err => {
  console.error('Test runner error:', err);
  process.exit(1);
});
