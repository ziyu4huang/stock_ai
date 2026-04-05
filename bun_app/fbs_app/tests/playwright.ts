import { chromium } from 'playwright';
import assert from 'node:assert';

const BASE = 'http://localhost:3200';

async function main() {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  let passed = 0;
  let failed = 0;

  async function test(name: string, fn: () => Promise<void>) {
    try {
      await fn();
      console.log(`  ✓ ${name}`);
      passed++;
    } catch (err: any) {
      console.log(`  ✗ ${name}: ${err.message}`);
      failed++;
    }
  }

  console.log('\n🧪 FBS App — Playwright Tests\n');

  // --- Test 1: Page loads ---
  await test('Page loads with correct title', async () => {
    await page.goto(BASE, { waitUntil: 'networkidle' });
    const title = await page.title();
    assert.equal(title, 'FBS Stock — Day Chart');
  });

  // --- Test 2: Top bar elements ---
  await test('Top bar has title and symbol input', async () => {
    const titleText = await page.locator('.top-bar .title').textContent();
    assert.equal(titleText, 'FBS Stock');

    const input = page.locator('#symbolInput');
    const value = await input.inputValue();
    assert.equal(value, '2330');
  });

  // --- Test 3: Mode buttons ---
  await test('Mode buttons: LIVE active, History inactive', async () => {
    const liveBtn = page.locator('.mode-btn[data-mode="live"]');
    const histBtn = page.locator('.mode-btn[data-mode="history"]');

    assert.isTrue(await liveBtn.getAttribute('class')?.then(c => c!.includes('active')) ?? false, 'LIVE should be active');
    assert.isFalse(await histBtn.getAttribute('class')?.then(c => c!.includes('active')) ?? true, 'History should not be active');
  });

  // --- Test 4: Timeframe buttons ---
  await test('Timeframe 1min is active by default', async () => {
    const btn1 = page.locator('.tf-btn[data-tf="1"]');
    const cls = await btn1.getAttribute('class');
    assert.ok(cls?.includes('active'), '1min should be active');
  });

  // --- Test 5: Quote header shows placeholders ---
  await test('Quote header has placeholder elements', async () => {
    const priceEl = page.locator('#priceDisplay');
    assert.ok(await priceEl.isVisible(), 'Price display should be visible');

    const ohlc = page.locator('.ohlc');
    assert.ok(await ohlc.isVisible(), 'OHLC should be visible');
  });

  // --- Test 6: Chart container exists ---
  await test('Chart container renders', async () => {
    const chart = page.locator('#chart');
    assert.ok(await chart.isVisible(), 'Chart div should be visible');

    // ECharts creates a child canvas or svg
    const box = await chart.boundingBox();
    assert.ok(box && box.width > 100, 'Chart should have reasonable width');
  });

  // --- Test 7: Footer elements ---
  await test('Footer stats visible', async () => {
    const footer = page.locator('.footer');
    assert.ok(await footer.isVisible(), 'Footer should be visible');
    assert.ok(await page.locator('#fVol').isVisible(), 'Volume stat visible');
    assert.ok(await page.locator('#fVal').isVisible(), 'Value stat visible');
  });

  // --- Test 8: Switch to History mode ---
  await test('Switch to History mode shows date picker', async () => {
    await page.locator('.mode-btn[data-mode="history"]').click();
    const dp = page.locator('#datePicker');
    assert.ok(await dp.isVisible(), 'Date picker should be visible in history mode');
  });

  // --- Test 9: Switch back to Live mode ---
  await test('Switch back to Live hides date picker', async () => {
    await page.locator('.mode-btn[data-mode="live"]').click();
    const dp = page.locator('#datePicker');
    // Wait a tick for the class toggle
    await page.waitForTimeout(100);
    assert.isFalse(await dp.isVisible(), 'Date picker should be hidden in live mode');
  });

  // --- Test 10: Change timeframe ---
  await test('Change timeframe to 5min', async () => {
    await page.locator('.tf-btn[data-tf="5"]').click();
    const btn = page.locator('.tf-btn[data-tf="5"]');
    const cls = await btn.getAttribute('class');
    assert.ok(cls?.includes('active'), '5min should now be active');

    // 1min should be deactivated
    const btn1 = page.locator('.tf-btn[data-tf="1"]');
    const cls1 = await btn1.getAttribute('class');
    assert.ok(!cls1?.includes('active'), '1min should be inactive');
  });

  // --- Test 11: Symbol input change ---
  await test('Change symbol input', async () => {
    const input = page.locator('#symbolInput');
    await input.fill('');
    await input.type('2454');
    await input.press('Enter');

    const val = await input.inputValue();
    assert.equal(val, '2454');
  });

  // --- Test 12: API endpoint /api/intraday/candles ---
  await test('API /api/intraday/candles responds', async () => {
    const resp = await page.goto(`${BASE}/api/intraday/candles?symbol=2330&timeframe=1`);
    assert.ok(resp!.ok(), 'API should respond 2xx');
    const data = await resp!.json();
    // Could be {data: [...]} or {error: "..."}
    assert.ok(data !== null, 'Response should be JSON');
  });

  // --- Test 13: API endpoint /api/intraday/quote ---
  await test('API /api/intraday/quote responds', async () => {
    const resp = await page.goto(`${BASE}/api/intraday/quote?symbol=2330`);
    assert.ok(resp!.ok(), 'Quote API should respond 2xx');
    const data = await resp!.json();
    assert.ok(data !== null, 'Response should be JSON');
  });

  // --- Test 14: API endpoint /api/historical/candles ---
  await test('API /api/historical/candles responds', async () => {
    const resp = await page.goto(`${BASE}/api/historical/candles?symbol=2330&timeframe=1`);
    assert.ok(resp!.ok(), 'Historical API should respond 2xx');
    const data = await resp!.json();
    assert.ok(data !== null, 'Response should be JSON');
  });

  // --- Test 15: Take screenshot for visual check ---
  await test('Screenshot captures page', async () => {
    await page.goto(BASE, { waitUntil: 'networkidle' });
    await page.waitForTimeout(1000); // Let chart render
    await page.screenshot({ path: 'tests/screenshot.png', fullPage: true });
  });

  // --- Summary ---
  console.log(`\n  ${passed} passed, ${failed} failed (${passed + failed} total)\n`);

  await browser.close();

  if (failed > 0) {
    process.exit(1);
  }
}

main().catch(err => {
  console.error('Test runner error:', err);
  process.exit(1);
});
