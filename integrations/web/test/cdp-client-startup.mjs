import assert from 'node:assert/strict';
import { once } from 'node:events';
import { existsSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { performance } from 'node:perf_hooks';
import { launchChrome } from './cdp-client.mjs';

const directory = mkdtempSync(join(tmpdir(), 'kuna-cdp-tests-'));
const originalTmp = process.env.TMPDIR;
process.env.TMPDIR = directory;
const fixture = `#!/usr/bin/env node
import { spawn } from 'node:child_process';
import { appendFileSync, mkdirSync, writeFileSync } from 'node:fs';
import { basename, dirname, join } from 'node:path';
const mode = basename(process.argv[1], '.mjs');
const profile = process.argv.find(arg => arg.startsWith('--user-data-dir=')).slice(16);
writeFileSync(join(dirname(process.argv[1]), mode + '.json'), JSON.stringify({profile, args: process.argv.slice(2)}));
const portFile = join(profile, 'DevToolsActivePort');
if (mode === 'early') {
  process.stderr.write('sentinel-chrome-startup-failure\\n');
  process.exitCode = 7;
} else if (mode === 'inherited-stderr') {
  const holder = spawn(process.execPath, ['-e', 'setInterval(() => {}, 1000)'], {
    stdio: ['ignore', 'ignore', 2],
  });
  writeFileSync(join(dirname(process.argv[1]), mode + '.pid'), String(holder.pid));
  holder.unref();
  process.stderr.write('inherited-stderr-sentinel\\n', () => process.exit(7));
} else if (mode === 'flood') {
  process.stderr.write('x'.repeat(20000) + 'tail-sentinel\\n');
  process.exitCode = 9;
} else {
  if (mode === 'ready') writeFileSync(portFile, '12345\\nfake-browser\\n');
  if (mode === 'partial') {
    writeFileSync(portFile, '12345');
    setTimeout(() => appendFileSync(portFile, '\\nfake-browser\\n'), 120);
  }
  if (mode === 'invalid') writeFileSync(portFile, 'not-a-port\\nfake-browser\\n');
  if (mode === 'read-error') mkdirSync(portFile);
  if (mode === 'stall') process.stderr.write('waiting-without-port\\n');
  setInterval(() => {}, 1000);
}
`;

function executable(mode) {
  const path = join(directory, `${mode}.mjs`);
  writeFileSync(path, fixture, { mode: 0o755 });
  return path;
}

function noProfiles() {
  assert.deepEqual(readdirSync(directory).filter(name => name.startsWith('kuna-cdp-')), []);
}

async function rejected(path, pattern, options = {}) {
  let failure;
  let chrome;
  try {
    chrome = await launchChrome(path, options);
  } catch (error) {
    failure = error;
  }
  if (chrome) {
    const closed = once(chrome.child, 'close');
    chrome.close();
    await closed;
  }
  assert.ok(failure, 'startup must reject');
  assert.match(failure.message, pattern);
  noProfiles();
  return failure;
}

let cases = 0;
try {
  const started = performance.now();
  const early = await rejected(executable('early'), /code 7/);
  assert.match(early.message, /sentinel-chrome-startup-failure/);
  assert.ok(performance.now() - started < 2000, 'an exited browser must not await the deadline');
  cases++;

  try {
    const started = performance.now();
    const inherited = await rejected(executable('inherited-stderr'), /code 7/, { startupTimeoutMs: 4000 });
    assert.match(inherited.message, /inherited-stderr-sentinel/);
    assert.ok(performance.now() - started < 2000, 'exit detection must not wait for inherited stderr to close');
    cases++;
  } finally {
    const pidFile = join(directory, 'inherited-stderr.pid');
    if (existsSync(pidFile)) {
      const pid = Number(readFileSync(pidFile, 'utf8'));
      assert.ok(Number.isInteger(pid) && pid > 1);
      try { process.kill(pid, 'SIGKILL'); }
      catch (error) { if (error.code !== 'ESRCH') throw error; }
    }
  }

  await rejected(join(directory, 'missing'), /Could not start Chrome:.*ENOENT/);
  cases++;
  await rejected({}, /file.*string/i);
  cases++;
  await rejected(null, /Chrome executable not found/);
  cases++;
  await rejected(executable('ready'), /timeout.*finite/, { startupTimeoutMs: NaN });
  cases++;

  for (const mode of ['ready', 'partial']) {
    const started = performance.now();
    const chrome = await launchChrome(executable(mode), { width: 800, height: 600 });
    const closed = once(chrome.child, 'close');
    let record;
    try {
      record = JSON.parse(readFileSync(join(directory, `${mode}.json`)));
      assert.equal(chrome.port, 12345);
      assert.ok(record.args.includes('--window-size=800,600'));
      assert.ok(record.args.includes('--remote-debugging-port=0'));
      if (mode === 'partial') assert.ok(performance.now() - started >= 120, 'wait for the complete port line');
      assert.ok(existsSync(record.profile));
    } finally {
      chrome.close();
      chrome.close();
      assert.equal(chrome.child.stderr.destroyed, true);
      await closed;
    }
    assert.equal(existsSync(record.profile), false);
    noProfiles();
    cases++;
  }

  await rejected(executable('invalid'), /invalid DevTools port/);
  cases++;
  const stalled = await rejected(executable('stall'), /within 1000 ms/, { startupTimeoutMs: 1000 });
  assert.match(stalled.message, /waiting-without-port/);
  cases++;
  const flooded = await rejected(executable('flood'), /code 9/);
  assert.match(flooded.message, /tail-sentinel/);
  assert.ok(flooded.message.length < 9000, 'diagnostics are bounded');
  cases++;
  await rejected(executable('read-error'), /EISDIR/);
  cases++;
  console.log(`CDP STARTUP OK — ${cases} cases`);
} finally {
  if (originalTmp === undefined) delete process.env.TMPDIR;
  else process.env.TMPDIR = originalTmp;
  rmSync(directory, { recursive: true, force: true });
}
