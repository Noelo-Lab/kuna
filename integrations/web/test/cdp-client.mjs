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

/** Launch headless Chrome; resolves `{browser, close}` once DevTools answers. */
export async function launchChrome(chromePath = findChrome(), { width = 1280, height = 860 } = {}) {
  const profile = mkdtempSync(join(tmpdir(), 'kuna-cdp-'));
  const child = spawn(chromePath, [
    '--headless=new', '--no-sandbox', '--disable-gpu', '--no-first-run',
    '--no-default-browser-check', '--disable-extensions', '--hide-scrollbars',
    '--remote-debugging-port=0', `--user-data-dir=${profile}`,
    `--window-size=${width},${height}`, 'about:blank',
  ], { stdio: 'ignore' });
  let port = null;
  for (let i = 0; i < 200 && !port; i++) {
    await sleep(50);
    try { port = readFileSync(join(profile, 'DevToolsActivePort'), 'utf8').split('\n')[0]; } catch (_) { /* not yet */ }
  }
  const close = () => {
    try { child.kill('SIGKILL'); } catch (_) { /* gone */ }
    try { rmSync(profile, { recursive: true, force: true }); } catch (_) { /* busy */ }
  };
  if (!port) { close(); throw new Error('Chrome did not open a DevTools port'); }
  return { port: Number(port), child, close };
}

/** Attach to the first page target; resolves a small session API. */
export async function openPage(port, { onException } = {}) {
  const targets = await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();
  const page = targets.find((t) => t.type === 'page');
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
    pending.set(id, { resolve, reject });
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

  async function navigate(url) {
    const loaded = new Promise((done) => on('Page.loadEventFired', done));
    await send('Page.navigate', { url });
    await loaded;
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

  return { send, on, evaluate, call, waitFor, navigate, click, hover, key, type, viewport, screenshot, exceptions, close: () => ws.close() };
}

const VK = { Enter: 13, Escape: 27, ' ': 32, ArrowLeft: 37, ArrowUp: 38, ArrowRight: 39, ArrowDown: 40, Tab: 9, Backspace: 8 };
