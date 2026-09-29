// cdp-client.mjs — a minimal Chrome DevTools Protocol driver over Node's
// built-in WebSocket: launch headless Chrome, open a page, evaluate, wait,
// click, type, screenshot. No puppeteer.
import { spawn } from 'node:child_process';
import { existsSync, mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

const CANDIDATES = [
  process.env.CHROME,
  '/usr/bin/google-chrome',
  '/usr/bin/google-chrome-stable',
  '/usr/bin/chromium',
  '/usr/bin/chromium-browser',
  '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome',
].filter(Boolean);

export function findChrome() {
  return CANDIDATES.find((path) => existsSync(path)) || null;
}

const sleep = (ms) => new Promise((done) => setTimeout(done, ms));

/**
 * Launch headless Chrome (with any extra `flags`); resolves `{port, child,
 * close}` once it publishes its DevTools port. A cold start on a busy CI
 * runner can take well over ten seconds, so it waits up to a minute.
 */
export async function launchChrome(chromePath = findChrome(), { width = 1280, height = 860, flags = [], startupTimeoutMs = 60000 } = {}) {
  if (!chromePath) throw new Error('Chrome executable not found');
  if (!Number.isFinite(startupTimeoutMs) || startupTimeoutMs < 0) {
    throw new Error('Chrome startup timeout must be a nonnegative finite number');
  }
  const profile = mkdtempSync(join(tmpdir(), 'kuna-cdp-'));
  let child;
  const cleanup = () => {
    try { rmSync(profile, { recursive: true, force: true }); } catch (_) { /* retried when the child closes */ }
  };
  const close = () => {
    try { child?.kill('SIGKILL'); } catch (_) { /* gone */ }
    child?.stderr?.destroy();
    cleanup();
  };
  let stderr = '';
  const failure = (message) => new Error(`${message}${stderr.trim() ? `\n${stderr.trim()}` : ''}`);
  try {
    child = spawn(chromePath, [
      '--headless=new', '--no-sandbox', '--disable-gpu', '--no-first-run',
      '--no-default-browser-check', '--disable-extensions', '--hide-scrollbars',
      '--remote-debugging-port=0', `--user-data-dir=${profile}`,
      `--window-size=${width},${height}`, ...flags, 'about:blank',
    ], { stdio: ['ignore', 'ignore', 'pipe'] });
    child.once('close', cleanup);
    child.stderr.setEncoding('utf8');
    child.stderr.on('data', (chunk) => { stderr = (stderr + chunk).slice(-8192); });
    let spawnError;
    let exited;
    child.once('error', (error) => { spawnError = error; });
    child.once('exit', (code, signal) => { exited = { code, signal }; });
    const deadline = Date.now() + startupTimeoutMs;
    for (;;) {
      if (spawnError) throw failure(`Could not start Chrome: ${spawnError.message}`);
      if (exited) {
        throw failure(`Chrome exited before publishing its DevTools port (${exited.signal ? `signal ${exited.signal}` : `code ${exited.code}`})`);
      }
      let portFile = '';
      try { portFile = readFileSync(join(profile, 'DevToolsActivePort'), 'utf8'); }
      catch (error) { if (error.code !== 'ENOENT') throw error; }
      const newline = portFile.indexOf('\n');
      if (newline >= 0 && child.exitCode === null && child.signalCode === null) {
        const value = portFile.slice(0, newline).trim();
        const port = /^\d+$/.test(value) ? Number(value) : NaN;
        if (!Number.isInteger(port) || port < 1 || port > 65535) {
          throw failure(`Chrome wrote an invalid DevTools port: ${JSON.stringify(value)}`);
        }
        return { port, child, close };
      }
      const remaining = deadline - Date.now();
      if (remaining <= 0) throw failure(`Chrome did not open a DevTools port within ${startupTimeoutMs} ms`);
      await sleep(Math.min(50, remaining));
    }
  } catch (error) {
    close();
    throw error;
  }
}

/** Attach to a page target (the first, or `targetId`); resolves a small session API. */
export async function openPage(port, { onException, targetId = null } = {}) {
  let page = null;
  for (let tries = 0; !page && tries < 100; tries++) {
    if (tries) await sleep(100);
    const targets = await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();
    page = targets.find((t) => t.type === 'page' && (!targetId || t.id === targetId));
  }
  if (!page) throw new Error('Chrome listed no page to attach to');
  const ws = new WebSocket(page.webSocketDebuggerUrl);
  await new Promise((done, fail) => { ws.onopen = done; ws.onerror = fail; });
  let seq = 0;
  const pending = new Map();
  const listeners = new Map();
  ws.onmessage = ({ data }) => {
    const msg = JSON.parse(data);
    if (msg.id && pending.has(msg.id)) {
      const { resolve, reject } = pending.get(msg.id);
      pending.delete(msg.id);
      if (msg.error) reject(new Error(`${msg.error.message}`));
      else resolve(msg.result);
    } else if (msg.method) {
      for (const fn of listeners.get(msg.method) || []) fn(msg.params);
    }
  };
  const send = (method, params = {}) => new Promise((resolve, reject) => {
    const id = ++seq;
    const timer = setTimeout(() => {
      pending.delete(id);
      reject(new Error(`DevTools did not answer ${method} within 90 s (is the page busy?)`));
    }, 90000);
    pending.set(id, {
      resolve: (v) => { clearTimeout(timer); resolve(v); },
      reject: (e) => { clearTimeout(timer); reject(e); },
    });
    ws.send(JSON.stringify({ id, method, params }));
  });
  const on = (method, fn) => {
    if (!listeners.has(method)) listeners.set(method, []);
    listeners.get(method).push(fn);
  };
  const exceptions = [];
  on('Runtime.exceptionThrown', (p) => {
    const text = p.exceptionDetails?.exception?.description || p.exceptionDetails?.text || 'exception';
    exceptions.push(text);
    onException?.(text);
  });
  await send('Page.enable');
  await send('Runtime.enable');

  async function evaluate(expression) {
    const res = await send('Runtime.evaluate', { expression, awaitPromise: true, returnByValue: true });
    if (res.exceptionDetails) {
      throw new Error(res.exceptionDetails.exception?.description || res.exceptionDetails.text);
    }
    return res.result.value;
  }

  /** Run `fn` in the page with `args` passed as values, never spliced into source. */
  async function call(fn, ...args) {
    const { result: global } = await send('Runtime.evaluate', { expression: 'globalThis' });
    const res = await send('Runtime.callFunctionOn', {
      functionDeclaration: fn.toString(), objectId: global.objectId,
      arguments: args.map((value) => ({ value })), awaitPromise: true, returnByValue: true,
    });
    if (res.exceptionDetails) {
      throw new Error(res.exceptionDetails.exception?.description || res.exceptionDetails.text);
    }
    return res.result.value;
  }

  async function waitFor(expression, { timeout = 30000, what = expression } = {}) {
    const end = Date.now() + timeout;
    while (Date.now() < end) {
      try { if (await evaluate(expression)) return true; } catch (_) { /* retry */ }
      await sleep(100);
    }
    throw new Error(`timed out waiting for: ${what}`);
  }

  /** Go to `url` and wait for its load event (a change of the #fragment alone loads nothing, so it returns at once). */
  async function navigate(url) {
    const loaded = new Promise((done) => on('Page.loadEventFired', done));
    const res = await send('Page.navigate', { url });
    if (res.loaderId || res.errorText) await loaded;
  }

  async function center(selector) {
    const box = await call((sel) => {
      const el = document.querySelector(sel);
      if (!el) return null;
      el.scrollIntoView({ block: 'nearest' });
      const r = el.getBoundingClientRect();
      return { x: r.left + Math.min(r.width / 2, 12), y: r.top + r.height / 2 };
    }, selector);
    if (!box) throw new Error(`no element for ${selector}`);
    return box;
  }

  async function mouse(type, { x, y }, extra = {}) {
    await send('Input.dispatchMouseEvent', { type, x, y, button: 'left', ...extra });
  }

  async function click(selector, { count = 1 } = {}) {
    const at = await center(selector);
    await mouse('mouseMoved', at);
    for (let n = 1; n <= count; n++) {
      await mouse('mousePressed', at, { clickCount: n });
      await mouse('mouseReleased', at, { clickCount: n });
    }
    return at;
  }

  async function hover(selector) {
    const at = await center(selector);
    await mouse('mouseMoved', at);
    return at;
  }

  async function key(keyName, { code = keyName, text, modifiers = 0 } = {}) {
    const printable = text ?? (keyName === 'Enter' ? '\r' : keyName.length === 1 ? keyName : undefined);
    await send('Input.dispatchKeyEvent', {
      type: printable ? 'keyDown' : 'rawKeyDown', key: keyName, code, text: printable, modifiers,
      windowsVirtualKeyCode: VK[keyName] ?? (keyName.length === 1 ? keyName.toUpperCase().charCodeAt(0) : 0),
    });
    await send('Input.dispatchKeyEvent', { type: 'keyUp', key: keyName, code, modifiers });
  }

  async function type(text) {
    await send('Input.insertText', { text });
  }

  async function viewport(width, height = 860) {
    await send('Emulation.setDeviceMetricsOverride', { width, height, deviceScaleFactor: 1, mobile: false });
  }

  async function screenshot() {
    const { data } = await send('Page.captureScreenshot', { format: 'png' });
    return Buffer.from(data, 'base64');
  }

  const closeTab = async () => {
    ws.close();
    await fetch(`http://127.0.0.1:${port}/json/close/${page.id}`).catch(() => {});
  };
  return { send, on, evaluate, call, waitFor, navigate, click, hover, key, type, viewport, screenshot, exceptions, close: () => ws.close(), closeTab };
}

/** Open another tab in the same browser (same profile: tabs share storage and BroadcastChannel). */
export async function openTab(port, options = {}) {
  const res = await fetch(`http://127.0.0.1:${port}/json/new?about:blank`, { method: 'PUT' });
  const target = await res.json();
  return openPage(port, { ...options, targetId: target.id });
}

const VK = { Enter: 13, Escape: 27, ' ': 32, ArrowLeft: 37, ArrowUp: 38, ArrowRight: 39, ArrowDown: 40, Tab: 9, Backspace: 8 };
