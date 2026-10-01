import assert from 'node:assert/strict';
import { EventEmitter, once } from 'node:events';
import childProcess from 'node:child_process';
import { syncBuiltinESMExports } from 'node:module';
import { PassThrough } from 'node:stream';
import { mock } from 'node:test';
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

const turn = () => new Promise(resolve => setImmediate(resolve));

async function controlled(check, { startupTimeoutMs = 4000, mode } = {}) {
  const stderr = new PassThrough();
  const child = Object.assign(new EventEmitter(), {
    stderr, exitCode: null, signalCode: null,
    kill() {
      if (this.exitCode === null && this.signalCode === null) exit(null, 'SIGKILL');
      return true;
    },
  });
  let closed = false;
  const maybeClose = () => {
    if (!closed && stderr.closed && (child.exitCode !== null || child.signalCode !== null)) {
      closed = true;
      child.emit('close', child.exitCode, child.signalCode);
    }
  };
  const exit = (code = 7, signal = null) => {
    child.exitCode = code;
    child.signalCode = signal;
    child.emit('exit', code, signal);
    maybeClose();
  };
  stderr.once('close', maybeClose);
  const timers = new Set();
  let outcome;
  let settlements = 0;
  mock.timers.enable({ apis: ['setTimeout', 'Date'], now: 0 });
  const schedule = globalThis.setTimeout;
  const cancel = globalThis.clearTimeout;
  mock.method(globalThis, 'setTimeout', (callback, delay, ...args) => {
    const timer = schedule(() => { timers.delete(timer); callback(...args); }, delay);
    timers.add(timer);
    return timer;
  });
  mock.method(globalThis, 'clearTimeout', timer => { timers.delete(timer); cancel(timer); });
  mock.method(childProcess, 'spawn', (_path, args) => {
    if (mode === 'throw') throw new Error('synthetic spawn exception');
    if (mode === 'ready' || mode === 'invalid') {
      const profile = args.find(arg => arg.startsWith('--user-data-dir=')).slice(16);
      writeFileSync(join(profile, 'DevToolsActivePort'), mode === 'ready' ? '12345\n' : 'invalid\n');
    }
    return child;
  });
  syncBuiltinESMExports();
  const pending = launchChrome('controlled-browser', { startupTimeoutMs }).then(
    chrome => { settlements++; outcome = chrome; },
    error => { settlements++; outcome = error; },
  );
  const advance = async ms => { mock.timers.tick(ms); await turn(); };
  try {
    await check({ child, stderr, exit, advance, result: () => outcome });
    assert.ok(outcome, 'startup must settle');
    await pending;
    if (!(outcome instanceof Error)) {
      outcome.close();
      outcome.close();
    }
    await turn();
    assert.equal(settlements, 1);
    assert.equal(timers.size, 0, 'startup must dispose its timers');
    noProfiles();
    if (mode !== 'throw') {
      assert.equal(stderr.destroyed, true);
      for (const event of ['data', 'end', 'close', 'error']) {
        assert.equal(stderr.listenerCount(event), 0, `stderr ${event} listener leaked`);
      }
      for (const event of ['exit', 'close', 'error']) {
        assert.equal(child.listenerCount(event), 0, `child ${event} listener leaked`);
      }
    }
  } finally {
    stderr.destroy();
    await turn();
    mock.restoreAll();
    mock.timers.reset();
    syncBuiltinESMExports();
  }
}

async function controlledCases() {
  let count = 0;
  await controlled(async ({ stderr, exit, advance, result }) => {
    exit();
    await advance(50);
    assert.equal(result(), undefined, 'wait for diagnostics delivered after exit');
    stderr.end('delayed-startup-sentinel\n');
    await advance(0);
    assert.match(result().message, /code 7/);
    assert.equal(result().message.match(/delayed-startup-sentinel/g)?.length, 1);
    assert.equal(Date.now(), 50, 'EOF should finish without awaiting the grace deadline');
  });
  count++;

  for (const mode of ['buffered', 'late-buffered', 'ended', 'held', 'delayed', 'flood', 'signal', 'error-before', 'error-during', 'closed']) {
    await controlled(async ({ stderr, exit, advance, result }) => {
      if (mode === 'buffered') { stderr.pause(); stderr.write('buffered-sentinel\n'); }
      if (mode === 'ended') { stderr.end('ended-sentinel\n'); await advance(0); }
      if (mode === 'error-before') { stderr.pause(); stderr.write('before-error-sentinel\n'); stderr.destroy(new Error('synthetic pipe failure')); await advance(0); }
      if (mode === 'closed') { stderr.pause(); stderr.write('closed-sentinel\n'); stderr.destroy(); await advance(0); }
      exit(mode === 'signal' ? null : 7, mode === 'signal' ? 'SIGTERM' : null);
      await advance(50);
      if (['ended', 'error-before', 'closed'].includes(mode)) {
        assert.ok(result() instanceof Error, `${mode} must finish without the grace`);
      } else {
        assert.equal(result(), undefined);
        if (mode === 'error-during') {
          stderr.pause();
          stderr.write('before-error-sentinel\n');
          stderr.destroy(new Error('synthetic pipe failure'));
          await advance(0);
          assert.match(result().message, /before-error-sentinel/);
        } else {
          for (let i = 0; i < 4; i++) {
            await advance(50);
            if (mode === 'flood') stderr.write('x'.repeat(20000) + 'tail-sentinel\n');
          }
          if (mode === 'delayed' || mode === 'signal') stderr.write('delayed-sentinel\n');
          await advance(49);
          if (mode === 'late-buffered') { stderr.pause(); stderr.write('buffered-sentinel\n'); }
          assert.equal(result(), undefined, 'held pipe waits for the fixed grace');
          await advance(1);
          assert.ok(result() instanceof Error, 'data cannot extend the grace');
          assert.equal(Date.now(), 300);
        }
      }
      assert.match(result().message, mode === 'signal' ? /signal SIGTERM/ : /code 7/);
      if (mode === 'buffered' || mode === 'late-buffered') assert.equal(result().message.match(/buffered-sentinel/g)?.length, 1);
      if (mode === 'error-before') assert.match(result().message, /before-error-sentinel/);
      if (mode === 'closed') assert.match(result().message, /closed-sentinel/);
      if (mode === 'ended') assert.match(result().message, /ended-sentinel/);
      if (mode === 'delayed' || mode === 'signal') assert.match(result().message, /delayed-sentinel/);
      if (mode === 'flood') {
        assert.ok(result().message.endsWith('tail-sentinel'));
        assert.ok(result().message.length < 9000);
      }
    });
    count++;
  }

  await controlled(async ({ stderr, exit, advance, result }) => {
    await advance(50);
    exit();
    await advance(50);
    assert.equal(result(), undefined);
    await advance(200);
    stderr.write('past-startup-deadline\n');
    await advance(50);
    assert.match(result().message, /code 7[\s\S]*past-startup-deadline/);
    assert.equal(Date.now(), 350);
  }, { startupTimeoutMs: 100 });
  count++;

  for (const mode of ['timeout', 'ready', 'invalid', 'spawn-error', 'throw']) {
    await controlled(async ({ child, advance, result }) => {
      if (mode === 'spawn-error') child.emit('error', new Error('synthetic spawn error'));
      await advance(mode === 'timeout' || mode === 'spawn-error' ? 50 : 0);
      if (mode === 'ready') assert.equal(result().port, 12345);
      else assert.match(result().message, {
        timeout: /within 50 ms/, invalid: /invalid DevTools port/,
        'spawn-error': /Could not start Chrome: synthetic spawn error/, throw: /synthetic spawn exception/,
      }[mode]);
    }, { startupTimeoutMs: 50, mode });
    count++;
  }
  return count;
}

let cases = 0;
try {
  cases += await controlledCases();
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
