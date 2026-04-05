/**
 * FBS API — Combined Test Runner
 * Runs login, history, and realtime-books tests in sequence.
 * Prints a summary report at the end.
 *
 * Usage: bun --env-file=.env .claude/skills/fbs-api/scripts/run-all.ts
 */

import { spawnSync } from 'child_process';
import { join } from 'path';

const ROOT = join(import.meta.dirname, '../../../..');

interface TestResult {
  name: string;
  status: 'PASS' | 'FAIL' | 'SKIP';
  notes: string;
  output: string;
}

function runTest(script: string, timeoutMs = 20000): TestResult {
  const name = script.replace('tests/fbs_nodejs/', '');
  const result = spawnSync(
    'bun',
    [`--env-file=${join(ROOT, '.env')}`, join(ROOT, script)],
    { timeout: timeoutMs, encoding: 'utf8' }
  );

  const stdout = result.stdout?.trim() ?? '';
  const stderr = result.stderr?.trim() ?? '';
  const combined = [stdout, stderr].filter(Boolean).join('\n');

  // login.ts
  if (name === 'login.ts') {
    if (result.status === 0 && stdout.includes('Login OK')) {
      const match = stdout.match(/Login OK:\s*(.+)/);
      return { name, status: 'PASS', notes: match?.[1]?.slice(0, 60) ?? 'authenticated', output: combined };
    }
    if (stderr.includes('連線測試成功') || stdout.includes('連線測試成功')) {
      return { name, status: 'FAIL', notes: '連線OK，待簽署 API 風險聲明書 (次日生效)', output: combined };
    }
    return { name, status: 'FAIL', notes: combined.slice(0, 80), output: combined };
  }

  // history.ts
  if (name === 'history.ts') {
    const barMatch = stdout.match(/(\d+)\s*筆/);
    if (result.status === 0 && barMatch) {
      return { name, status: 'PASS', notes: `2330: ${barMatch[1]} bars returned`, output: combined };
    }
    return { name, status: 'FAIL', notes: combined.slice(0, 80), output: combined };
  }

  // realtime-books.ts
  if (name === 'realtime-books.ts') {
    if (stdout.includes('Market is closed')) {
      const timeMatch = stdout.match(/\((.+)\)/);
      return { name, status: 'SKIP', notes: `市場收盤 ${timeMatch?.[1] ?? ''}`, output: combined };
    }
    if (stdout.includes('五檔') || stdout.includes('bids')) {
      return { name, status: 'PASS', notes: 'Realtime data received', output: combined };
    }
    if (stdout.includes('No data received')) {
      return { name, status: 'SKIP', notes: 'No data (market just closed)', output: combined };
    }
    return { name, status: result.status === 0 ? 'PASS' : 'FAIL', notes: combined.slice(0, 80), output: combined };
  }

  return { name, status: result.status === 0 ? 'PASS' : 'FAIL', notes: combined.slice(0, 80), output: combined };
}

// ── Run all tests ────────────────────────────────────────────────────────────

const tests = ['tests/fbs_nodejs/login.ts', 'tests/fbs_nodejs/history.ts', 'tests/fbs_nodejs/realtime-books.ts'];
const results: TestResult[] = [];

for (const t of tests) {
  process.stdout.write(`Running ${t.replace('tests/fbs_nodejs/', '')}... `);
  const r = runTest(t);
  process.stdout.write(`${r.status}\n`);
  results.push(r);
}

// ── Summary Report ───────────────────────────────────────────────────────────

const statusIcon = (s: TestResult['status']) => ({ PASS: '✓', FAIL: '✗', SKIP: '–' }[s]);
const col = (s: string, w: number) => s.padEnd(w).slice(0, w);

console.log('\n## FBS API Test Summary\n');
console.log(`| ${'Test'.padEnd(22)} | ${'Status'.padEnd(6)} | Notes |`);
console.log(`|${'-'.repeat(24)}|${'-'.repeat(8)}|${'-'.repeat(45)}|`);

for (const r of results) {
  console.log(`| ${col(r.name, 22)} | ${statusIcon(r.status)} ${col(r.status, 4)} | ${col(r.notes, 43)} |`);
}

const passed = results.filter(r => r.status === 'PASS').length;
const failed = results.filter(r => r.status === 'FAIL').length;
const skipped = results.filter(r => r.status === 'SKIP').length;

console.log(`\nPASS ${passed}  FAIL ${failed}  SKIP ${skipped}`);

if (failed > 0) {
  console.log('\n--- Details for failed tests ---');
  for (const r of results.filter(r => r.status === 'FAIL')) {
    console.log(`\n[${r.name}]\n${r.output}`);
  }
  process.exit(1);
}
