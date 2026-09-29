// decompile2-collab-page.mjs — live-session cases in the real /decompile2/
// page, one per defect a review found in how the page drives a session (the
// number is the review's): a cancelled or superseded edit keeps the others'
// changes; a language switch before a guest joins; a guest's own stored
// changes kept apart and back after leaving, and saved with their own names;
// following someone into a function that is still loading; a new invite after
// a failed join; Undo during another person's re-decompile, and Cancel
// keeping it queued; opening a program during a pending join; the restored-
// changes banner once a session starts; keyboard focus in the roster; two
// tabs of one browser without a false warning; a connection that cannot be
// made; a name with a line separator. Then a second review's (numbered as it
// numbers them): a guest that has the program open with changes of its own;
// leaving and joining again, with the guest's own changes kept apart or not;
// an edit made while another person's change waits to be applied; an inviter
// whose tab closes in the middle of a join; following someone who opens a
// function they had open before; a view setting changed while another
// person's change waits; the "use the session's changes" offer after opening
// another program; a tab joining a tab of its own browser that keeps a
// student's changes apart; Stop while a new decompiler effort reloads the
// program; changes stored by an earlier version. Then a third review's: two
// records with one unparseable directive (no re-decompile loop, alone), a
// join that fails while the received program opens, another person's change
// queued behind an open a cached function replaced, a join whose build id
// cannot be worked out after the connection was made, the back/forward
// cache, and a session's saved copy offered when its program is opened again.
// And a fourth review's: opening a cached function while another person's
// change is being decompiled, the site deployed again while a session is on
// (a Stop restarts the engine), no solo undo copies in a session, no offer of
// an old session's copy to a page joining a new one, a guest
// that renamed a variable at another
// decompiler effort, and a program received while an edit of another program
// is in flight. Each case runs in fresh tabs (a second Chrome process
// stands in for another person's computer) and is reported; any failure exits 1.
//
// Skips (exit 0) when there is no Chrome or no global WebSocket (Node < 22).
// `--only TEXT` runs the cases whose name contains TEXT.
//   integrations/web/build.sh && node integrations/web/test/decompile2-collab-page.mjs
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { findChrome, launchChrome, openPage, openTab } from './cdp-client.mjs';
import { requireDist, serveStatic, fixture } from './worker-harness.mjs';
import { legacyKey } from '../decompile2/persist.js';

const chromePath = findChrome();
if (!chromePath || typeof WebSocket !== 'function') {
  console.log(`DECOMPILE2 COLLAB PAGE SKIPPED — ${chromePath ? 'this Node has no global WebSocket (need 22+)' : 'no Chrome found (set CHROME=...)'}`);
  process.exit(0);
}
requireDist();

const sleep = (ms) => new Promise((done) => setTimeout(done, ms));
let wasmTag = null;
let wasmFetches = 0;
const server = await serveStatic(undefined, 0, {
  onRequest: (rel) => { if (rel.endsWith('.wasm')) wasmFetches++; },
  headers: (rel) => (rel.endsWith('.wasm') && wasmTag ? { etag: wasmTag } : {}),
});
const flags = ['--disable-features=WebRtcHideLocalIpsWithMdns'];
const chrome = await launchChrome(chromePath, { flags });
const chrome2 = await launchChrome(chromePath, { flags });
const guard = setTimeout(() => { console.error('DECOMPILE2 COLLAB PAGE FAIL — timed out'); chrome.close(); chrome2.close(); process.exit(1); }, 2400000);
const SAMPLE_HASH = 'sha256:' + createHash('sha256').update(readFileSync(fixture('sample.elf'))).digest('hex');

/** Worker answers arrive `window.__kunaDelay` ms late, so an engine request can be caught in flight; `__kunaWorkers` counts engine starts. */
const DELAY_SHIM = `(() => {
  const Real = window.Worker;
  window.__kunaDelay = 0;
  window.__kunaWorkers = 0;
  window.Worker = function (url, opts) {
    const w = new Real(url, opts);
    window.__kunaWorkers++;
    let handler = null;
    w.addEventListener('message', (ev) => { const d = window.__kunaDelay || 0; if (d) setTimeout(() => handler && handler(ev), d); else if (handler) handler(ev); });
    Object.defineProperty(w, 'onmessage', { get() { return handler; }, set(fn) { handler = fn; }, configurable: true });
    return w;
  };
  window.Worker.prototype = Real.prototype;
})();`;

/** The page's apply batch (a 16 ms timer) waits 6 s instead, so an edit can be made inside it. */
const WIDE_APPLY = `(() => {
  const real = window.setTimeout;
  window.setTimeout = function (fn, ms, ...rest) { return real.call(this, fn, ms === 16 ? 6000 : ms, ...rest); };
})();`;

/** The program, and the message announcing it, sent between tabs of this browser never arrive. */
const DROP_FILE = `(() => {
  const post = BroadcastChannel.prototype.postMessage;
  BroadcastChannel.prototype.postMessage = function (m) {
    if (m && m.k === 'm' && (m.d instanceof ArrayBuffer || (typeof m.d === 'string' && m.d.startsWith('{"t":"file"')))) return undefined;
    return post.call(this, m);
  };
})();`;

/** Every request the page makes of the engine, by method, in `window.__kunaCalls`. */
const COUNT_CALLS = `(() => {
  window.__kunaCalls = [];
  const post = Worker.prototype.postMessage;
  Worker.prototype.postMessage = function (m, t) {
    window.__kunaCalls.push(m && m.method);
    return post.call(this, m, t);
  };
})();`;

/** The directives of every \`list\` request the page makes, in \`window.__lists\`. */
const LIST_ARGS = `(() => {
  window.__lists = [];
  const post = Worker.prototype.postMessage;
  Worker.prototype.postMessage = function (m, t) {
    if (m && m.method === 'list') window.__lists.push(m.params.assertions);
    return post.call(this, m, t);
  };
})();`;

/** The engine cannot say which build it is (as when the site changed after the page loaded). */
const BUILD_FAILS = `(() => {
  const Real = window.Worker;
  window.Worker = function (url, opts) {
    const w = new Real(url, opts);
    let handler = null;
    w.addEventListener('message', (ev) => handler && handler(ev));
    Object.defineProperty(w, 'onmessage', { get() { return handler; }, set(fn) { handler = fn; }, configurable: true });
    const post = w.postMessage.bind(w);
    w.postMessage = (m, t) => {
      if (m && m.method === 'build') {
        setTimeout(() => handler && handler({ data: { id: m.id, ok: false, error: 'the site was updated after this page loaded; reload the page' } }), 10);
        return undefined;
      }
      return post(m, t);
    };
    return w;
  };
  window.Worker.prototype = Real.prototype;
})();`;

/** The browser takes 4 s to register each of this page's Web Lock requests (as it can on a busy machine). */
const LOCKS_LATE = `(() => {
  if (!navigator.locks) return;
  const request = navigator.locks.request.bind(navigator.locks);
  navigator.locks.request = (...args) => new Promise((resolve, reject) => setTimeout(() => request(...args).then(resolve, reject), 4000));
})();`;

const OWN_KEY = `kuna.d2.session.${SAMPLE_HASH}`;
const stored = (records) => JSON.stringify({ v: 1, rawSeq: 0, bytes: [], records });
const MAIN = '0x1198';
const SUM = '0x1161';
const fnRec = (addr, name) => [`fn:${addr}`, { kind: 'fn', addr, name }];
const varRec = (func, sym, name) => [`var:${func}:${sym}`, { kind: 'var', func, sym, name, type: null }];
const noteRec = (func, addr, text) => [`comment:${func}:${addr}`, { kind: 'comment', func, addr, text }];
const rawRec = (n, text) => [`raw:${n}`, { kind: 'raw', text }];

let tabs = [];
let firstUsed = { 1: false, 2: false };
async function tab(label, { other = false, script = null, width = 1280 } = {}) {
  const c = other ? chrome2 : chrome;
  const which = other ? 2 : 1;
  const page = firstUsed[which] ? await openTab(c.port) : await openPage(c.port);
  firstUsed[which] = true;
  page.label = label;
  await page.viewport(width, 860);
  if (script) await page.send('Page.addScriptToEvaluateOnNewDocument', { source: script });
  tabs.push(page);
  return page;
}

const ready = (p) => p.waitFor(`document.getElementById('pick').getAttribute('aria-disabled') === null`, { what: `${p.label} ready`, timeout: 60000 });
const idle = (p) => p.waitFor(`document.getElementById('cancelbtn').disabled`, { what: `${p.label} idle`, timeout: 90000 });
const text = (p, sel) => p.call((s) => document.querySelector(s)?.textContent ?? '', sel);
const code = (p) => text(p, '#ccode');
const toasts = (p) => p.evaluate(`[...document.querySelectorAll('.d2-toast')].map((t) => t.textContent)`);
const closeDialog = (p) => p.evaluate(`(() => { const d = document.getElementById('d2collab'); if (d?.open) d.close(); return true; })()`);
const setSelect = (p, id, value) => p.call((i, v) => { const s = document.getElementById(i); s.value = v; s.dispatchEvent(new Event('change')); return true; }, id, value);
const front = (p) => p.send('Page.bringToFront');

/** Open the page with this browser's storage emptied first (then `seed`), so no case inherits another's settings. */
async function open(p, { seed = null } = {}) {
  await p.navigate(`${server.base}/kuna-web.js`);
  await p.call((entries) => {
    localStorage.clear();
    for (const [k, v] of entries) localStorage.setItem(k, v);
    return true;
  }, seed || []);
  await p.navigate(`${server.base}/decompile2/`);
  await ready(p);
}

async function example(p) {
  await p.click('#examplebtn');
  await p.waitFor(`document.getElementById('crumbname')?.textContent === 'sample.elf' && document.getElementById('ccode').textContent.includes('add(')`, { what: `${p.label}: main`, timeout: 60000 });
  await idle(p);
}

async function nameAndGo(p, name) {
  await p.waitFor(`document.getElementById('d2collab')?.open && document.querySelector('#d2collab input[name=name]')`, { what: `${p.label} name field`, timeout: 30000 });
  await p.call(() => { const i = document.querySelector('#d2collab input[name=name]'); i.focus(); i.select(); return true; });
  await p.type(name);
  await p.key('Enter');
}

async function inviteLink(p, name = null) {
  if (!(await p.evaluate(`!!document.getElementById('d2collab')?.open`))) {
    await p.click('#morebtn');
    await p.click('#collabbtn');
    await p.waitFor(`document.getElementById('d2collab')?.open`, { what: 'collab dialog' });
  }
  if (name) await nameAndGo(p, name);
  else await p.click('#d2collab [data-act=invite]');
  await p.waitFor(`document.querySelector('#d2collab [data-copytext]')?.value.includes('#join=')`, { what: 'invite link', timeout: 20000 });
  return p.evaluate(`document.querySelector('#d2collab [data-copytext]').value`);
}

/** Ana (first browser, the example open) and Ben (a tab of the same browser, or `other`: a second browser), joined. */
async function pair({ other = false, anaScript = null, benScript = null, benSeed = null, anaSeed = null } = {}) {
  const ana = await tab('Ana', { script: anaScript });
  await open(ana, { seed: anaSeed });
  await example(ana);
  const link = await inviteLink(ana, 'Ana');
  const ben = await tab('Ben', { other, script: benScript });
  if (benSeed || other) await open(ben, { seed: benSeed });
  await ben.navigate(link);
  await nameAndGo(ben, 'Ben');
  if (other) {
    await ben.waitFor(`document.querySelector('#d2collab [data-copytext]')?.value.includes('#reply=')`, { what: 'Ben\'s reply link', timeout: 20000 });
    const reply = await ben.evaluate(`document.querySelector('#d2collab [data-copytext]').value`);
    await ana.call((r) => { const i = document.querySelector('#d2collab input[name=reply]'); i.value = r; i.form.requestSubmit(); return true; }, reply);
  }
  await ben.waitFor(`document.getElementById('crumbname')?.textContent === 'sample.elf' && /sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'Ben has the program', timeout: 60000 });
  await idle(ben);
  await ana.waitFor(`document.querySelectorAll('#d2roster .d2-who:not(.wait)').length === 1`, { what: 'Ana sees Ben', timeout: 20000 });
  await closeDialog(ana);
  await closeDialog(ben);
  return { ana, ben };
}

/** Open `link`'s invite in `ben`, a page that is already open (only the #fragment changes). */
async function joinByHash(ben, link) {
  await ben.evaluate(`location.hash = ${JSON.stringify('#join=' + link.split('#join=')[1])}; true`);
  await nameAndGo(ben, 'Ben');
}

/** Ben (another browser) made a reply link: Ana pastes it into her invite. */
async function carryReply(ana, ben) {
  await ben.waitFor(`document.querySelector('#d2collab [data-copytext]')?.value.includes('#reply=')`, { what: 'Ben\'s reply link', timeout: 20000 });
  const reply = await ben.evaluate(`document.querySelector('#d2collab [data-copytext]').value`);
  if (!(await ana.evaluate(`!!document.querySelector('#d2collab input[name=reply]')`))) {
    await ana.click('#morebtn');
    await ana.click('#collabbtn');
    await ana.waitFor(`document.querySelector('#d2collab input[name=reply]')`, { what: 'Ana\'s paste box' });
  }
  await ana.call((r) => { const i = document.querySelector('#d2collab input[name=reply]'); i.value = r; i.form.requestSubmit(); return true; }, reply);
}

const joinedBoth = async (ana, ben, n = 1) => {
  await ben.waitFor(`document.querySelectorAll('#d2roster .d2-who:not(.wait)').length === ${n} && !document.getElementById('d2collab')?.open`, { what: 'Ben joined', timeout: 30000 });
  await ana.waitFor(`document.querySelectorAll('#d2roster .d2-who:not(.wait)').length === ${n}`, { what: 'Ana sees Ben', timeout: 20000 });
};
const rail = (p) => text(p, '#sesslist');

async function popover(p, selector, key, value) {
  await p.click(selector);
  await p.key(key);
  await p.waitFor(`!document.getElementById('d2pop').hidden`, { what: `${p.label} popover` });
  await p.call((v) => { const i = document.querySelector('#d2pop input, #d2pop textarea'); i.value = v; i.dispatchEvent(new Event('input', { bubbles: true })); return true; }, value);
}

/** What a page shows, for a failed case's report. */
const SUMMARY = `JSON.stringify({
  fn: document.getElementById('vname')?.textContent, view: document.querySelector('#tabs [aria-selected=true]')?.dataset.tab,
  status: document.getElementById('status')?.textContent, busy: !document.getElementById('cancelbtn')?.disabled,
  code: (document.getElementById('ccode')?.textContent || '').slice(0, 80),
  toasts: [...document.querySelectorAll('.d2-toast')].map((t) => t.textContent.slice(0, 90)),
  dialog: document.getElementById('d2collab')?.open ? document.getElementById('d2collab').textContent.slice(0, 160) : null,
})`;
const only = process.argv.includes('--only') ? process.argv[process.argv.indexOf('--only') + 1] : null;
const CASE_MS = 150000;
const results = [];
async function test(name, fn) {
  if (only && !name.includes(only)) return;
  const t0 = Date.now();
  let timer;
  try {
    await Promise.race([fn(), new Promise((_, fail) => { timer = setTimeout(() => fail(new Error(`the case took over ${CASE_MS / 1000} s`)), CASE_MS); })]);
    for (const p of tabs) assert.deepEqual(p.exceptions, [], `no page exception in ${p.label}`);
    results.push([true, name]);
    console.log(`ok   ${name} (${((Date.now() - t0) / 1000).toFixed(1)} s)`);
  } catch (e) {
    results.push([false, name, e.message.split('\n')[0]]);
    console.log(`FAIL ${name} (${((Date.now() - t0) / 1000).toFixed(1)} s) — ${e.message.split('\n')[0]}`);
    for (const p of tabs) console.log(`     ${p.label}: ${await p.evaluate(SUMMARY).catch((err) => err.message)}`);
  } finally {
    clearTimeout(timer);
    for (const p of tabs) await p.closeTab().catch(() => {});
    tabs = [];
    firstUsed = { 1: true, 2: true };
    await sleep(300);
  }
}

try {
  await test('#1 a cancelled edit keeps the edit and the other person\'s change, on both pages', async () => {
    const { ana, ben } = await pair({ anaScript: DELAY_SHIM });
    await front(ben);
    await popover(ben, '#ccode .t[data-sym="v1"]', 'n', 'bens_name');
    await front(ana);
    await popover(ana, '#ccode .t[data-sym="argc"]', 'n', 'anas_name');
    await ana.evaluate('window.__kunaDelay = 4000; true');
    await ana.key('Enter');
    await ben.key('Enter');
    await sleep(1500);
    assert.equal(await ana.evaluate(`!document.getElementById('cancelbtn').disabled`), true, 'Ana\'s edit is in flight');
    await ana.evaluate(`document.getElementById('cancelbtn').click(); window.__kunaDelay = 0; true`);
    for (const p of [ana, ben]) {
      await p.waitFor(`[...document.querySelectorAll('#sesslist li')].length === 2`, { what: `${p.label}: both changes kept`, timeout: 20000 });
    }
    await idle(ben);
    await ben.waitFor(`/anas_name/.test(document.getElementById('ccode').textContent) && /bens_name/.test(document.getElementById('ccode').textContent)`, { what: 'Ben shows both', timeout: 30000 });
    assert.ok(!(await toasts(ben)).some((t) => /put back/.test(t)), 'nobody is told a change was put back');
  });

  await test('#1 a new decompiler effort from someone else does not take back an edit in flight', async () => {
    const { ana, ben } = await pair({ anaScript: DELAY_SHIM });
    await front(ana);
    await popover(ana, '#ccode .t[data-sym="argc"]', 'n', 'anas_name');
    await ana.evaluate('window.__kunaDelay = 3000; true');
    await ana.key('Enter');
    await sleep(800);
    await setSelect(ben, 'mode', 'fast');
    await ana.waitFor(`document.getElementById('mode').value === 'fast'`, { what: 'Ana takes the new effort', timeout: 20000 });
    await ana.evaluate('window.__kunaDelay = 0; true');
    for (const p of [ana, ben]) {
      await idle(p);
      await p.waitFor(`/anas_name/.test(document.getElementById('ccode').textContent)`, { what: `${p.label} keeps Ana's rename`, timeout: 60000 });
    }
  });

  await test('#4 a guest joins after the inviter switched Show code as (a re-index) since making the link', async () => {
    const ana = await tab('Ana');
    await open(ana);
    await example(ana);
    const link = await inviteLink(ana, 'Ana');
    await closeDialog(ana);
    await setSelect(ana, 'lang', 'rust');
    await ana.waitFor(`/fn main|unsafe/.test(document.getElementById('ccode').textContent)`, { what: 'Ana in Rust', timeout: 60000 });
    await idle(ana);
    const ben = await tab('Ben');
    await ben.navigate(link);
    await nameAndGo(ben, 'Ben');
    await ben.waitFor(`document.getElementById('crumbname')?.textContent === 'sample.elf'`, { what: 'Ben receives the program', timeout: 30000 });
  });

  await test('#6 #24 a guest\'s own changes are kept apart, come back on leaving, and save with their own names', async () => {
    const own = JSON.stringify({ v: 1, rawSeq: 0, bytes: [], records: [
      ['fn:0x1161', { kind: 'fn', addr: '0x1161', name: 'mine_sum' }],
      ['var:0x1161:v1', { kind: 'var', func: '0x1161', sym: 'v1', name: 'bens_own', type: null }],
    ] });
    const { ana, ben } = await pair({ other: true, benSeed: [[`kuna.d2.session.${SAMPLE_HASH}`, own]] });
    await front(ana);
    await ana.click('#fnlist .fn[data-addr="0x1161"]');
    await idle(ana);
    await ana.click('#fnrenamebtn');
    await ana.waitFor(`!document.getElementById('d2pop').hidden`);
    await ana.call(() => { const i = document.querySelector('#d2pop input'); i.value = 'summation'; i.dispatchEvent(new Event('input', { bubbles: true })); return true; });
    await ana.key('Enter');
    await ben.waitFor(`/summation\\(add/.test(document.getElementById('ccode').textContent)`, { what: 'Ben sees the session', timeout: 30000 });
    await sleep(500);
    assert.equal(await ben.evaluate(`localStorage.getItem(${JSON.stringify(`kuna.d2.session.${SAMPLE_HASH}`)})`), own, 'Ben\'s own stored changes are untouched');
    assert.match(await ben.evaluate(`localStorage.getItem(${JSON.stringify(`kuna.d2.shared.${SAMPLE_HASH}`)}) || ''`), /summation/, 'the session is saved apart');
    await ben.click('#d2roster [data-act=collab-open]');
    await ben.waitFor(`document.querySelector('#d2collab [data-act=save-aside]')`, { what: 'the offer to save them, in the dialog', timeout: 10000 });
    await ben.evaluate(`(() => { window.__saved = null; const real = URL.createObjectURL; URL.createObjectURL = (b) => { b.text().then((t) => { window.__saved = t; }); return real.call(URL, b); }; return true; })()`);
    await ben.click('#d2collab [data-act=save-aside]');
    await ben.waitFor('window.__saved', { what: 'the saved file' });
    const saved = await ben.evaluate('window.__saved');
    assert.match(saved, /name mine_sum::v1 bens_own/, 'qualified with the guest\'s own function names');
    assert.ok(!/summation/.test(saved), 'not the session\'s');
    await ben.click('#d2collab [data-act=leave]');
    await ben.waitFor(`/bens_own/.test(document.getElementById('ccode').textContent + document.getElementById('sesslist').textContent)`, { what: 'Ben\'s own changes are back', timeout: 30000 });
    assert.ok((await toasts(ben)).some((t) => /Your own changes to sample\.elf are back/.test(t)));
  });

  await test('#10 a follower\'s page finishes opening a function while the others keep moving', async () => {
    const { ana, ben } = await pair({ benScript: DELAY_SHIM });
    const starts = await ben.evaluate('window.__kunaWorkers');
    await ben.evaluate('window.__kunaDelay = 2500; true');
    await ben.click('#d2roster .d2-who');
    await front(ana);
    await ana.click('#fnlist .fn[data-addr="0x1161"]');
    for (let i = 0; i < 6; i++) {
      await sleep(700);
      await ana.click(i % 2 ? '#tab-c' : '#tab-asm');
    }
    await ben.waitFor(`document.getElementById('vname').textContent === 'sum_to' && /for \\(/.test(document.getElementById('ccode').textContent)`, { what: 'Ben shows sum_to', timeout: 30000 });
    assert.equal(await ben.evaluate('window.__kunaWorkers'), starts, 'Ben\'s open was never cancelled and restarted');
  });

  await test('#12 after a failed join, the tab can join another invite', async () => {
    const ana = await tab('Ana');
    await open(ana);
    await example(ana);
    const used = await inviteLink(ana, 'Ana');
    await closeDialog(ana);
    const cy = await tab('Cy');
    await cy.navigate(used);
    await nameAndGo(cy, 'Cy');
    await cy.waitFor(`document.getElementById('crumbname')?.textContent === 'sample.elf'`, { what: 'Cy joined', timeout: 30000 });
    const ben = await tab('Ben');
    await ben.navigate(used);
    await nameAndGo(ben, 'Ben');
    await ben.waitFor(`/already used/.test(document.getElementById('d2collab')?.textContent || '')`, { what: 'Ben is told the link was used', timeout: 20000 });
    const fresh = await inviteLink(ana);
    await closeDialog(ana);
    await closeDialog(ben);
    await ben.evaluate(`location.hash = ${JSON.stringify('#join=' + fresh.split('#join=')[1])}; true`);
    await ben.waitFor(`document.querySelector('#d2collab input[name=name]')`, { what: 'a join form for the new invite', timeout: 20000 });
    await nameAndGo(ben, 'Ben');
    await ben.waitFor(`document.getElementById('crumbname')?.textContent === 'sample.elf'`, { what: 'Ben joins', timeout: 30000 });
  });

  await test('#20 Undo works while another person\'s change is being decompiled', async () => {
    const { ana, ben } = await pair({ anaScript: DELAY_SHIM });
    await front(ana);
    await popover(ana, '#ccode .t[data-sym="argc"]', 'n', 'anas_name');
    await ana.key('Enter');
    await idle(ana);
    await ana.evaluate('window.__kunaDelay = 3000; true');
    await front(ben);
    await popover(ben, '#ccode .t[data-sym="v1"]', 'n', 'bens_name');
    await ben.key('Enter');
    await ana.waitFor(`!document.getElementById('cancelbtn').disabled`, { what: 'Ana decompiles Ben\'s change', timeout: 10000 });
    await front(ana);
    await ana.evaluate(`document.body.focus(); true`);
    await ana.key('u');
    await ana.evaluate('window.__kunaDelay = 0; true');
    for (const p of [ana, ben]) {
      await p.waitFor(`!/anas_name/.test(document.getElementById('sesslist').textContent)`, { what: `${p.label}: Ana's rename undone`, timeout: 30000 });
    }
  });

  await test('#20 Cancel keeps another person\'s change queued for the code on screen', async () => {
    const { ana, ben } = await pair({ anaScript: DELAY_SHIM });
    await ana.evaluate('window.__kunaDelay = 4000; true');
    await front(ana);
    await ana.click('#ccode');
    await ana.key('x');
    await ana.waitFor(`!document.getElementById('cancelbtn').disabled`, { what: 'Ana looks for callers', timeout: 10000 });
    await front(ben);
    await popover(ben, '#ccode .t[data-sym="v1"]', 'n', 'bens_name');
    await ben.key('Enter');
    await sleep(1200);
    await ana.evaluate(`window.__kunaDelay = 0; document.getElementById('cancelbtn').click(); true`);
    await ana.waitFor(`/bens_name/.test(document.getElementById('ccode').textContent)`, { what: 'Ana\'s code shows Ben\'s rename', timeout: 30000 });
  });

  await test('#21 opening a program while a join waits asks first', async () => {
    const ana = await tab('Ana');
    await open(ana);
    await example(ana);
    const link = await inviteLink(ana, 'Ana');
    const ben = await tab('Ben', { other: true });
    await open(ben);
    await ben.navigate(link);
    await nameAndGo(ben, 'Ben');
    await ben.waitFor(`document.querySelector('#d2collab [data-copytext]')?.value.includes('#reply=')`, { what: 'Ben waits with a reply link', timeout: 20000 });
    await closeDialog(ben);
    await ben.click('#examplebtn');
    await ben.waitFor(`!document.getElementById('d2pop').hidden && /You are joining Ana's session/.test(document.getElementById('d2pop').textContent)`, { what: 'Ben is asked first', timeout: 10000 });
  });

  await test('#22 the restored-changes banner goes once a session starts', async () => {
    const stored = JSON.stringify({ v: 1, rawSeq: 1, bytes: [], records: [['raw:1', { kind: 'raw', text: 'readonly 0x2000+8' }]] });
    const ana = await tab('Ana');
    await open(ana, { seed: [[`kuna.d2.session.${SAMPLE_HASH}`, stored]] });
    await example(ana);
    await ana.waitFor(`document.querySelector('.d2banner')`, { what: 'the restored banner', timeout: 20000 });
    await inviteLink(ana, 'Ana');
    await ana.waitFor(`!document.querySelector('.d2banner')`, { what: 'the banner goes', timeout: 10000 });
  });

  await test('#23 the roster keeps keyboard focus while the others move', async () => {
    const { ana, ben } = await pair();
    await ben.evaluate(`document.querySelector('#d2roster .d2-who').focus(); true`);
    await front(ana);
    for (const t of ['#tab-asm', '#tab-c', '#tab-asm']) {
      await ana.click(t);
      await sleep(300);
    }
    assert.equal(await ben.evaluate(`document.activeElement?.classList.contains('d2-who') === true`), true);
  });

  await test('#25 two tabs of one browser: no warning about the guest\'s own earlier changes', async () => {
    const ana = await tab('Ana');
    await open(ana);
    await example(ana);
    await popover(ana, '#ccode .t[data-sym="v1"]', 'n', 'total');
    await ana.key('Enter');
    await idle(ana);
    const link = await inviteLink(ana, 'Ana');
    const ben = await tab('Ben');
    await ben.navigate(link);
    await nameAndGo(ben, 'Ben');
    await ben.waitFor(`document.getElementById('crumbname')?.textContent === 'sample.elf'`, { what: 'Ben joined', timeout: 30000 });
    await sleep(800);
    assert.ok(!(await toasts(ben)).some((t) => /earlier changes|kept as they were/.test(t)), JSON.stringify(await toasts(ben)));
  });

  await test('#26 a connection this browser cannot make is shown, not swallowed', async () => {
    const ana = await tab('Ana', { script: 'window.RTCPeerConnection = function () { throw new Error("blocked by the browser"); };' });
    await open(ana);
    await example(ana);
    await ana.click('#morebtn');
    await ana.click('#collabbtn');
    await nameAndGo(ana, 'Ana');
    await ana.waitFor(`/Could not make an invite link: blocked by the browser/.test(document.querySelector('#d2collab [data-status]')?.textContent || '')`, { what: 'the reason', timeout: 10000 });
  });

  await test('#18 a name with a line separator is cleaned, not a silent failure', async () => {
    const ana = await tab('Ana');
    await open(ana);
    await example(ana);
    await ana.click('#morebtn');
    await ana.click('#collabbtn');
    await nameAndGo(ana, 'A\u2028na');
    await ana.waitFor(`document.querySelector('#d2collab [data-copytext]')?.value.includes('#join=')`, { what: 'an invite link', timeout: 10000 });
  });

  // ── a second review ──────────────────────────────────────────────────────

  await test('second review #1 a guest that has the program open with changes of its own joins: nothing of the inviter\'s is lost', async () => {
    const ana = await tab('Ana');
    await open(ana, { seed: [[OWN_KEY, stored([fnRec(SUM, 'summation'), noteRec(MAIN, '0x11b5', 'ana note')])]] });
    await example(ana);
    const link = await inviteLink(ana, 'Ana');
    const ben = await tab('Ben', { other: true });
    await open(ben, { seed: [[OWN_KEY, stored([fnRec(SUM, 'bens_sum'), varRec(MAIN, 'v1', 'bens_total')])]] });
    await example(ben);
    await joinByHash(ben, link);
    await carryReply(ana, ben);
    await joinedBoth(ana, ben);
    for (const p of [ana, ben]) {
      await p.waitFor(`/summation\\(add/.test(document.getElementById('ccode').textContent) && /bens_total/.test(document.getElementById('ccode').textContent)`, { what: `${p.label} shows Ana's rename and Ben's`, timeout: 30000 });
      await p.waitFor(`/ana note/.test(document.getElementById('sesslist').textContent)`, { what: `${p.label} keeps Ana's note`, timeout: 10000 });
      assert.ok(!/bens_sum/.test(await code(p) + await rail(p)), `the session's name for sum_to wins on ${p.label}'s page`);
    }
    assert.ok((await toasts(ben)).some((t) => /replaced 1 of yours/.test(t)), 'Ben is told, and offered his own as a file');
  });

  await test('second review #2 a guest whose own changes were kept apart leaves, then joins again with the program open: the session is not erased', async () => {
    const ana = await tab('Ana');
    await open(ana, { seed: [[OWN_KEY, stored([fnRec(SUM, 'summation'), noteRec(MAIN, '0x11b5', 'ana note')])]] });
    await example(ana);
    const first = await inviteLink(ana, 'Ana');
    const ben = await tab('Ben', { other: true });
    await open(ben, { seed: [[OWN_KEY, stored([varRec(MAIN, 'v1', 'bens_own')])]] });
    await ben.navigate(first);
    await nameAndGo(ben, 'Ben');
    await carryReply(ana, ben);
    await ben.waitFor(`/summation\\(add/.test(document.getElementById('ccode').textContent)`, { what: 'Ben has the session', timeout: 30000 });
    await idle(ben);
    await closeDialog(ana);
    await ben.click('#d2roster [data-act=collab-open]');
    await ben.waitFor(`document.querySelector('#d2collab [data-act=leave]')`, { what: 'the session dialog' });
    await ben.click('#d2collab [data-act=leave]');
    await ben.waitFor(`/bens_own/.test(document.getElementById('ccode').textContent) && !/summation/.test(document.getElementById('ccode').textContent)`, { what: 'Ben\'s own changes are back', timeout: 30000 });
    await idle(ben);
    await ana.waitFor(`document.querySelectorAll('#d2roster .d2-who').length === 0`, { what: 'Ana sees Ben leave', timeout: 20000 });
    const again = await inviteLink(ana);
    await joinByHash(ben, again);
    await carryReply(ana, ben);
    await joinedBoth(ana, ben);
    for (const p of [ana, ben]) {
      await p.waitFor(`/summation\\(add/.test(document.getElementById('ccode').textContent) && /bens_own/.test(document.getElementById('ccode').textContent)`, { what: `${p.label} has the session and Ben's own rename`, timeout: 30000 });
      await p.waitFor(`/ana note/.test(document.getElementById('sesslist').textContent)`, { what: `${p.label} keeps Ana's note`, timeout: 10000 });
    }
  });

  await test('second review #2 a guest leaves, both go on alone, and it joins the same session again: both sides\' changes meet', async () => {
    const { ana, ben } = await pair();
    await ben.click('#d2roster [data-act=collab-open]');
    await ben.waitFor(`document.querySelector('#d2collab [data-act=leave]')`, { what: 'the session dialog' });
    await ben.click('#d2collab [data-act=leave]');
    await ana.waitFor(`document.querySelectorAll('#d2roster .d2-who').length === 0`, { what: 'Ana sees Ben leave', timeout: 20000 });
    await front(ana);
    await popover(ana, '#ccode .t[data-sym="argc"]', 'n', 'while_away');
    await ana.key('Enter');
    await idle(ana);
    await front(ben);
    await popover(ben, '#ccode .t[data-sym="v1"]', 'n', 'bens_alone');
    await ben.key('Enter');
    await idle(ben);
    const again = await inviteLink(ana);
    await closeDialog(ben);
    await joinByHash(ben, again);
    await joinedBoth(ana, ben);
    for (const p of [ana, ben]) {
      await p.waitFor(`/while_away/.test(document.getElementById('ccode').textContent) && /bens_alone/.test(document.getElementById('ccode').textContent)`, { what: `${p.label} has both`, timeout: 30000 });
    }
  });

  await test('second review #3 an edit made while another person\'s change waits to be applied keeps that change', async () => {
    const { ana, ben } = await pair({ benScript: WIDE_APPLY });
    await front(ana);
    await popover(ana, '#ccode .t[data-sym="v1"]', 'n', 'anas_name');
    await ana.key('Enter');
    await sleep(600);
    assert.ok(!/anas_name/.test(await code(ben)), 'Ben\'s page has not applied it yet');
    await front(ben);
    await popover(ben, '#ccode .t[data-sym="argc"]', 'n', 'bens_name');
    await ben.key('Enter');
    for (const p of [ana, ben]) {
      await p.waitFor(`/anas_name/.test(document.getElementById('ccode').textContent) && /bens_name/.test(document.getElementById('ccode').textContent)`, { what: `${p.label} has both`, timeout: 30000 });
    }
    await sleep(2000);
    for (const p of [ana, ben]) assert.match(await code(p), /anas_name/, `${p.label} still has Ana's rename`);
  });

  await test('second review #4 the inviter\'s tab closes after the welcome, before the program arrives: the join fails and says so', async () => {
    const ana = await tab('Ana', { script: DROP_FILE });
    await open(ana);
    await example(ana);
    const link = await inviteLink(ana, 'Ana');
    const ben = await tab('Ben');
    await ben.navigate(link);
    await nameAndGo(ben, 'Ben');
    await ben.waitFor(`/Receiving/.test(document.getElementById('d2collab')?.textContent || '')`, { what: 'Ben waits for the program', timeout: 20000 });
    await ana.closeTab();
    tabs = tabs.filter((t) => t !== ana);
    await ben.waitFor(`/closed/.test((document.getElementById('d2collab')?.textContent || '') + [...document.querySelectorAll('.d2-toast')].map((t) => t.textContent).join(' '))`, { what: 'Ben is told', timeout: 20000 });
    await closeDialog(ben);
    await ben.click('#examplebtn');
    await ben.waitFor(`/sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'Ben opens a program without being asked to stop a join', timeout: 60000 });
  });

  await test('second review #7 following someone who opens a function they had open before: the follower lands where they are', async () => {
    const { ana, ben } = await pair({ anaScript: DELAY_SHIM });
    await ben.click('#d2roster .d2-who');
    await front(ana);
    await ana.evaluate('window.__kunaDelay = 3000; true');
    await ana.click(`#fnlist .fn[data-addr="${SUM}"]`);
    await sleep(400);
    await ana.click(`#fnlist .fn[data-addr="${MAIN}"]`);
    await ana.evaluate('window.__kunaDelay = 0; true');
    await sleep(3500);
    assert.equal(await text(ana, '#vname'), 'main');
    assert.equal(await text(ben, '#vname'), 'main', 'Ben follows Ana to main, not to the function she stopped opening');
    assert.match(await ben.evaluate(`document.querySelector('#d2roster .d2-who')?.title || ''`), /Ana: main/);
  });

  await test('second review #8 a view setting changed while another person\'s change waits: the change still reaches the code', async () => {
    const { ana, ben } = await pair({ anaScript: DELAY_SHIM });
    await front(ana);
    await ana.evaluate('window.__kunaDelay = 10000; true');
    await ana.click('#ccode');
    await ana.key('x');
    await ana.waitFor(`!document.getElementById('cancelbtn').disabled`, { what: 'Ana is busy', timeout: 10000 });
    await front(ben);
    await popover(ben, '#ccode .t[data-sym="v1"]', 'n', 'bens_name');
    await ben.key('Enter');
    await front(ana);
    await ana.waitFor(`/bens_name/.test(document.getElementById('sesslist').textContent)`, { what: 'Ben\'s change reached Ana\'s session', timeout: 10000 });
    await sleep(600);
    await ana.evaluate(`(() => {
      document.getElementById('viewbtn').click();
      const i = document.querySelector('#viewmenu input[name=asmInfer]');
      i.checked = !i.checked;
      i.dispatchEvent(new Event('change', { bubbles: true }));
      return true;
    })()`);
    assert.equal(await ana.evaluate(`!document.getElementById('cancelbtn').disabled`), true, 'the re-decompile is still waiting for Ana\'s request');
    assert.ok(!/bens_name/.test(await code(ana)), 'and the code does not have Ben\'s change yet');
    await ana.evaluate('window.__kunaDelay = 0; true');
    await ana.waitFor(`/bens_name/.test(document.getElementById('ccode').textContent)`, { what: 'Ana\'s code shows Ben\'s rename', timeout: 30000 });
  });

  await test('second review #9 "use the session\'s changes" after opening another program does not touch that program', async () => {
    const ana = await tab('Ana');
    await open(ana, { seed: [[OWN_KEY, stored([fnRec(SUM, 'summation')])]] });
    await example(ana);
    const link = await inviteLink(ana, 'Ana');
    const ben = await tab('Ben', { other: true });
    await open(ben, { seed: [[OWN_KEY, stored([varRec(MAIN, 'v1', 'bens_own')])]] });
    await ben.navigate(link);
    await nameAndGo(ben, 'Ben');
    await carryReply(ana, ben);
    await ben.waitFor(`/summation\\(add/.test(document.getElementById('ccode').textContent)`, { what: 'Ben has the session', timeout: 30000 });
    await idle(ben);
    await ben.click('#d2roster [data-act=collab-open]');
    await ben.waitFor(`document.querySelector('#d2collab [data-act=leave]')`, { what: 'the session dialog' });
    await ben.click('#d2collab [data-act=leave]');
    await ben.waitFor(`[...document.querySelectorAll('.d2-toast button')].some((b) => /session's changes instead/.test(b.textContent))`, { what: 'the offer', timeout: 20000 });
    await idle(ben);
    await ben.call((b64) => {
      const bytes = Uint8Array.from(atob(b64), (c) => c.charCodeAt(0));
      const dt = new DataTransfer();
      dt.items.add(new File([bytes], 'sample_macho.o'));
      const input = document.getElementById('file');
      input.files = dt.files;
      input.dispatchEvent(new Event('change', { bubbles: true }));
      return true;
    }, readFileSync(fixture('sample_macho.o')).toString('base64'));
    await ben.waitFor(`document.getElementById('crumbname')?.textContent === 'sample_macho.o'`, { what: 'another program', timeout: 60000 });
    await idle(ben);
    await ben.call(() => { [...document.querySelectorAll('.d2-toast button')].find((b) => /session's changes instead/.test(b.textContent)).click(); return true; });
    await sleep(800);
    assert.ok(!/Replace mine/.test(await ben.evaluate(`document.getElementById('d2pop')?.hidden ? '' : document.getElementById('d2pop')?.textContent || ''`)), 'no offer to replace the other program\'s changes');
    assert.ok((await toasts(ben)).some((t) => /another program/.test(t)), 'it says why');
  });

  await test('second review #11 a tab joining a tab of its own browser keeps the student\'s own changes apart when that tab does', async () => {
    const hana = await tab('Hana', { other: true });
    await open(hana);
    await example(hana);
    const link = await inviteLink(hana, 'Hana');
    const own = stored([varRec(MAIN, 'v1', 'students_own')]);
    const ana = await tab('Ana');
    await open(ana, { seed: [[OWN_KEY, own]] });
    await ana.navigate(link);
    await nameAndGo(ana, 'Ana');
    await carryReply(hana, ana);
    await ana.waitFor(`document.getElementById('crumbname')?.textContent === 'sample.elf' && /kept as they were/.test([...document.querySelectorAll('.d2-toast')].map((t) => t.textContent).join(' '))`, { what: 'Ana keeps her own apart', timeout: 30000 });
    await idle(ana);
    const second = await inviteLink(ana);
    const cy = await tab('Cy');
    await cy.navigate(second);
    await nameAndGo(cy, 'Cy');
    await cy.waitFor(`document.getElementById('crumbname')?.textContent === 'sample.elf' && /sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'Cy joined', timeout: 30000 });
    await idle(cy);
    await front(cy);
    await popover(cy, '#ccode .t[data-sym="argc"]', 'n', 'cys_name');
    await cy.key('Enter');
    await idle(cy);
    await sleep(500);
    assert.equal(await cy.evaluate(`localStorage.getItem(${JSON.stringify(OWN_KEY)})`), own, 'the student\'s own stored changes are untouched');
  });

  await test('second review #12 Stop while a new decompiler effort reloads the program leaves the session cleanly', async () => {
    const { ana, ben } = await pair({ anaScript: DELAY_SHIM });
    await ana.evaluate('window.__kunaDelay = 5000; true');
    await setSelect(ben, 'mode', 'fast');
    await ana.waitFor(`document.getElementById('mode').value === 'fast' && !document.getElementById('cancelbtn').disabled`, { what: 'Ana reloads the program', timeout: 20000 });
    await ana.evaluate(`document.getElementById('cancelbtn').click(); window.__kunaDelay = 0; true`);
    await ana.waitFor(`(() => { const r = document.getElementById('d2roster'); return !r || r.hidden; })()`, { what: 'Ana is out of the session', timeout: 10000 });
    assert.ok((await toasts(ana)).some((t) => /left the session/.test(t)));
    await ben.waitFor(`document.querySelectorAll('#d2roster .d2-who').length === 0`, { what: 'Ben sees Ana go', timeout: 20000 });
  });

  await test('second review #13 changes stored by an earlier version (under its old key) are kept apart like any others', async () => {
    const ana = await tab('Ana');
    await open(ana);
    await example(ana);
    const link = await inviteLink(ana, 'Ana');
    const ben = await tab('Ben', { other: true });
    const old = `kuna.d2.session.${legacyKey(readFileSync(fixture('sample.elf')))}`;
    await open(ben, { seed: [[old, stored([varRec(MAIN, 'v1', 'from_before')])]] });
    await ben.navigate(link);
    await nameAndGo(ben, 'Ben');
    await carryReply(ana, ben);
    await ben.waitFor(`document.getElementById('crumbname')?.textContent === 'sample.elf'`, { what: 'Ben joined', timeout: 30000 });
    await ben.waitFor(`/kept as they were/.test([...document.querySelectorAll('.d2-toast')].map((t) => t.textContent).join(' '))`, { what: 'Ben\'s earlier changes are kept apart', timeout: 10000 });
    assert.match(await ben.evaluate(`localStorage.getItem(${JSON.stringify(OWN_KEY)}) || ''`), /from_before/, 'and moved to the current key');
  });

  // ── a third review ───────────────────────────────────────────────────────

  await test('third review #2 two records with one directive the engine cannot read: no endless re-decompiling, working alone', async () => {
    const ana = await tab('Ana', { script: COUNT_CALLS });
    await open(ana, { seed: [[OWN_KEY, stored([rawRec(1, 'bytes 0x10 zz'), rawRec(2, 'bytes 0x10 zz')])]] });
    await example(ana);
    await sleep(1500);
    const before = await ana.evaluate('window.__kunaCalls.length');
    await sleep(3000);
    const after = await ana.evaluate('window.__kunaCalls.length');
    assert.equal(after, before, `the page kept asking the engine: ${JSON.stringify(await ana.evaluate('window.__kunaCalls.slice(-6)'))}`);
    assert.match(await rail(ana), /bytes 0x10 zz/);
  });

  await test('third review #4 a join that fails while the received program opens gives the student\'s own changes back', async () => {
    const ana = await tab('Ana');
    await open(ana, { seed: [[OWN_KEY, stored([fnRec(SUM, 'summation')])]] });
    await example(ana);
    const link = await inviteLink(ana, 'Ana');
    const ben = await tab('Ben', { other: true, script: DELAY_SHIM });
    await open(ben, { seed: [[OWN_KEY, stored([varRec(MAIN, 'v1', 'bens_own')])]] });
    await ben.navigate(link);
    await nameAndGo(ben, 'Ben');
    await ben.evaluate('window.__kunaDelay = 4000; true');
    await carryReply(ana, ben);
    await ben.waitFor(`/Finding the functions in sample.elf/.test(document.getElementById('status').textContent)`, { what: 'Ben opens the program', timeout: 20000 });
    await ana.closeTab();
    tabs = tabs.filter((t) => t !== ana);
    await ben.waitFor(`/closed the connection|stopped answering/.test((document.getElementById('d2collab')?.textContent || '') + [...document.querySelectorAll('.d2-toast')].map((t) => t.textContent).join(' '))`, { what: 'Ben is told', timeout: 30000 });
    await ben.evaluate('window.__kunaDelay = 0; true');
    await closeDialog(ben);
    await ben.waitFor(`/bens_own/.test(document.getElementById('ccode').textContent)`, { what: 'Ben\'s own changes show', timeout: 30000 });
    assert.ok(!/summation/.test(await code(ben)), 'not the session\'s');
    assert.equal(await ben.evaluate(`localStorage.getItem(${JSON.stringify(`kuna.d2.shared.${SAMPLE_HASH}`)})`), null, 'and nothing is saved as a session\'s copy');
  });

  await test('third review #5 another person\'s change queued behind an open that a cached function replaced still reaches the code', async () => {
    const { ana, ben } = await pair({ anaScript: DELAY_SHIM });
    await front(ben);
    await ben.click(`#fnlist .fn[data-addr="${SUM}"]`);
    await ben.waitFor(`document.getElementById('vname').textContent === 'sum_to' && document.getElementById('cancelbtn').disabled`, { what: 'Ben on sum_to', timeout: 30000 });
    await front(ana);
    await ana.evaluate('window.__kunaDelay = 4000; true');
    await ana.click(`#fnlist .fn[data-addr="${SUM}"]`);
    await ana.waitFor(`!document.getElementById('cancelbtn').disabled`, { what: 'Ana opens sum_to', timeout: 10000 });
    await front(ben);
    await popover(ben, '#ccode .t[data-sym="v1"]', 'n', 'in_sum');
    await ben.key('Enter');
    await front(ana);
    await ana.waitFor(`/in_sum/.test(document.getElementById('sesslist').textContent)`, { what: 'Ben\'s change reached Ana', timeout: 10000 });
    await sleep(800);
    assert.equal(await ana.evaluate(`!document.getElementById('cancelbtn').disabled`), true, 'Ana is still opening sum_to, so the re-decompile waits');
    await ana.click(`#fnlist .fn[data-addr="${MAIN}"]`);
    await ana.evaluate('window.__kunaDelay = 0; true');
    await ana.waitFor(`document.getElementById('vname').textContent === 'main' && document.getElementById('cancelbtn').disabled`, { what: 'Ana back on main', timeout: 20000 });
    await front(ben);
    await ben.click(`#fnlist .fn[data-addr="${MAIN}"]`);
    await ben.waitFor(`document.getElementById('vname').textContent === 'main' && document.getElementById('cancelbtn').disabled`, { what: 'Ben on main', timeout: 30000 });
    await popover(ben, '#ccode .t[data-sym="argc"]', 'n', 'later');
    await ben.key('Enter');
    await ana.waitFor(`/later/.test(document.getElementById('ccode').textContent)`, { what: 'Ana\'s code shows Ben\'s later rename', timeout: 30000 });
  });

  await test('third review #6 a join that cannot work out its build id after connecting closes the link, and the inviter hears', async () => {
    const ana = await tab('Ana');
    await open(ana);
    await example(ana);
    const link = await inviteLink(ana, 'Ana');
    const ben = await tab('Ben', { script: BUILD_FAILS });
    await ben.navigate(link);
    await nameAndGo(ben, 'Ben');
    await ben.waitFor(`/Could not join|could not make a connection/.test(document.getElementById('d2collab')?.textContent || '')`, { what: 'Ben is told', timeout: 20000 });
    await ana.waitFor(`/did not finish joining/.test(document.querySelector('#d2collab [data-status]')?.textContent || '')`, { what: 'Ana is told', timeout: 20000 });
    assert.equal(await ana.evaluate(`document.querySelectorAll('#d2roster .d2-who').length`), 0);
  });

  await test('third review #10 a page that goes into the back/forward cache leaves the session, and says so when it comes back', async () => {
    const { ana, ben } = await pair();
    await ben.evaluate(`dispatchEvent(new PageTransitionEvent('pagehide', { persisted: true })); true`);
    await ana.waitFor(`document.querySelectorAll('#d2roster .d2-who').length === 0`, { what: 'Ana sees Ben go', timeout: 20000 });
    await ben.evaluate(`dispatchEvent(new PageTransitionEvent('pageshow', { persisted: true })); true`);
    await ben.waitFor(`/left the session when you went to another page/.test([...document.querySelectorAll('.d2-toast')].map((t) => t.textContent).join(' '))`, { what: 'Ben is told', timeout: 10000 });
    assert.match(await text(ben, '#railchanges h3'), /^Your changes/, 'the page is back to working alone');
  });

  await test('a tab whose browser is slow to grant it its own presence lock still stays linked to the tab that invited it', async () => {
    const ana = await tab('Ana');
    await open(ana);
    await example(ana);
    const link = await inviteLink(ana, 'Ana');
    const ben = await tab('Ben', { script: LOCKS_LATE });
    await ben.navigate(link);
    await nameAndGo(ben, 'Ben');
    await ben.waitFor(`document.getElementById('crumbname')?.textContent === 'sample.elf'`, { what: 'Ben joined', timeout: 30000 });
    await ana.waitFor(`document.querySelectorAll('#d2roster .d2-who:not(.wait)').length === 1`, { what: 'Ana sees Ben', timeout: 20000 });
    await sleep(6000);
    assert.equal(await ana.evaluate(`document.querySelectorAll('#d2roster .d2-who').length`), 1, 'Ana still sees Ben');
    assert.ok(!(await toasts(ana)).some((t) => /Lost the connection/.test(t)), JSON.stringify(await toasts(ana)));
  });

  await test('third review #11 a session\'s copy kept apart is offered again when its program is opened later', async () => {
    const ana = await tab('Ana');
    await open(ana, { seed: [[OWN_KEY, stored([fnRec(SUM, 'summation')])]] });
    await example(ana);
    const link = await inviteLink(ana, 'Ana');
    const ben = await tab('Ben', { other: true });
    await open(ben, { seed: [[OWN_KEY, stored([varRec(MAIN, 'v1', 'bens_own')])]] });
    await ben.navigate(link);
    await nameAndGo(ben, 'Ben');
    await carryReply(ana, ben);
    await ben.waitFor(`/summation\\(add/.test(document.getElementById('ccode').textContent)`, { what: 'Ben has the session', timeout: 30000 });
    await idle(ben);
    await ben.click('#d2roster [data-act=collab-open]');
    await ben.waitFor(`document.querySelector('#d2collab [data-act=leave]')`, { what: 'the session dialog' });
    await ben.click('#d2collab [data-act=leave]');
    await ben.waitFor(`/bens_own/.test(document.getElementById('ccode').textContent)`, { what: 'Ben\'s own changes are back', timeout: 30000 });
    await ben.navigate(`${server.base}/decompile2/`);
    await ready(ben);
    await example(ben);
    await ben.waitFor(`[...document.querySelectorAll('.d2-toast button')].some((b) => /Use those instead/.test(b.textContent))`, { what: 'the offer', timeout: 10000 });
    await ben.call(() => { [...document.querySelectorAll('.d2-toast button')].find((b) => /Use those instead/.test(b.textContent)).click(); return true; });
    await ben.waitFor(`!document.getElementById('d2pop').hidden && /Replace mine/.test(document.getElementById('d2pop').textContent)`, { what: 'asked first', timeout: 10000 });
    await ben.evaluate(`document.querySelector('#d2pop form').requestSubmit(); true`);
    await ben.waitFor(`/summation\\(add/.test(document.getElementById('ccode').textContent)`, { what: 'the session\'s changes are Ben\'s now', timeout: 30000 });
  });

  // ── a fourth review ──────────────────────────────────────────────────────

  await test('fourth review #3 a guest who renamed a variable at another decompiler effort joins: the rename stays out of the session, and the guest is told', async () => {
    const ana = await tab('Ana');
    await open(ana);
    await example(ana);
    const link = await inviteLink(ana, 'Ana');
    const ben = await tab('Ben', { other: true });
    await open(ben, { seed: [[OWN_KEY, stored([fnRec(SUM, 'bens_sum'), varRec(MAIN, 'v1', 'bens_fast_total')])]] });
    await example(ben);
    await ben.call(() => { const m = document.getElementById('mode'); m.value = 'fast'; m.dispatchEvent(new Event('change', { bubbles: true })); return true; });
    await idle(ben);
    await joinByHash(ben, link);
    await carryReply(ana, ben);
    await joinedBoth(ana, ben);
    await ben.waitFor(`document.getElementById('mode').value === 'auto'`, { what: 'Ben runs the session\'s effort', timeout: 20000 });
    for (const p of [ana, ben]) {
      await p.waitFor(`/bens_sum/.test(document.getElementById('sesslist').textContent)`, { what: `${p.label} has Ben's function name`, timeout: 30000 });
      await idle(p);
      assert.ok(!/bens_fast_total/.test(await code(p) + await rail(p)), `${p.label} holds no rename Ben made in Fast`);
    }
    assert.ok((await toasts(ben)).some((t) => /One of your variables was changed at another decompiler effort/.test(t) && /Automatic/.test(t)), 'Ben is told, and offered his own as a file');
  });

  await test('fourth review #4 the site is deployed again during a session: a Stop restarts the engine without leaving the session or downloading it again', async () => {
    wasmTag = '"first"';
    try {
      const { ana, ben } = await pair({ anaScript: DELAY_SHIM });
      wasmTag = '"deployed-again"';
      const fetched = wasmFetches;
      const workers = await ana.evaluate('window.__kunaWorkers');
      await ana.evaluate('window.__kunaDelay = 4000; true');
      await front(ana);
      await ana.click(`#fnlist .fn[data-addr="${SUM}"]`);
      await sleep(1000);
      assert.equal(await ana.evaluate(`!document.getElementById('cancelbtn').disabled`), true, 'Ana\'s request is in flight');
      await ana.evaluate(`document.getElementById('cancelbtn').click(); window.__kunaDelay = 0; true`);
      await ana.waitFor(`window.__kunaWorkers > ${workers}`, { what: 'Ana\'s engine restarted', timeout: 20000 });
      await idle(ana);
      await ana.click(`#fnlist .fn[data-addr="${SUM}"]`);
      await ana.waitFor(`document.getElementById('vname').textContent === 'sum_to' && document.getElementById('cancelbtn').disabled`, { what: 'Ana on sum_to', timeout: 30000 });
      await sleep(1000);
      assert.equal(await ana.evaluate(`document.querySelectorAll('#d2roster .d2-who:not(.wait)').length`), 1, 'Ana is still in the session with Ben');
      assert.ok(!(await toasts(ana)).some((t) => /left the session/.test(t)), 'and was not told she left it');
      assert.equal(wasmFetches - fetched, 0, 'the restarted engine is the one the page compiled: no download');
      await front(ben);
      await popover(ben, '#ccode .t[data-sym="v1"]', 'n', 'after_the_stop');
      await ben.key('Enter');
      await ana.waitFor(`/after_the_stop/.test(document.getElementById('sesslist').textContent)`, { what: 'Ana still hears Ben', timeout: 20000 });
    } finally {
      wasmTag = null;
    }
  });

  await test('fourth review #7 a program received while an edit of another program is in flight: that edit is undone in its own program\'s store, and nothing is saved as a session\'s copy of it', async () => {
    const ana = await tab('Ana');
    await open(ana);
    await example(ana);
    const link = await inviteLink(ana, 'Ana');
    const ben = await tab('Ben', { other: true, script: DELAY_SHIM });
    const macho = readFileSync(fixture('sample_macho.o'));
    const machoHash = 'sha256:' + createHash('sha256').update(macho).digest('hex');
    await open(ben, { seed: [[OWN_KEY, stored([varRec(MAIN, 'v1', 'bens_own')])], [`kuna.d2.session.${machoHash}`, stored([noteRec('0x0', '0x0', 'bens macho note')])]] });
    await ben.call((b64) => {
      const bytes = Uint8Array.from(atob(b64), (c) => c.charCodeAt(0));
      const dt = new DataTransfer();
      dt.items.add(new File([bytes], 'sample_macho.o'));
      const input = document.getElementById('file');
      input.files = dt.files;
      input.dispatchEvent(new Event('change', { bubbles: true }));
      return true;
    }, macho.toString('base64'));
    await ben.waitFor(`document.getElementById('crumbname')?.textContent === 'sample_macho.o' && document.querySelector('#fnlist .fn')`, { what: 'Ben has another program open', timeout: 60000 });
    await idle(ben);
    await ben.call(() => { [...document.querySelectorAll('#fnlist .fn')].find((f) => /sum_to/.test(f.textContent)).click(); return true; });
    await ben.waitFor(`/sum_to/.test(document.getElementById('vname').textContent) && document.querySelector('#ccode .t[data-sym]') && document.getElementById('cancelbtn').disabled`, { what: 'Ben on its sum_to', timeout: 60000 });
    await joinByHash(ben, link);
    await ben.waitFor(`document.querySelector('#d2collab [data-copytext]')?.value.includes('#reply=')`, { what: 'Ben\'s reply link', timeout: 20000 });
    const reply = await ben.evaluate(`document.querySelector('#d2collab [data-copytext]').value`);
    await closeDialog(ben);
    await front(ben);
    await popover(ben, '#ccode .t[data-sym]', 'n', 'bens_macho_name');
    await ben.evaluate('window.__kunaDelay = 30000; true');
    await ben.key('Enter');
    await sleep(1000);
    await ben.evaluate('window.__kunaDelay = 0; true');
    assert.equal(await ben.evaluate(`!document.getElementById('cancelbtn').disabled`), true, 'Ben\'s edit is in flight');
    assert.match(await ben.evaluate(`localStorage.getItem(${JSON.stringify(`kuna.d2.session.${machoHash}`)}) || ''`), /bens_macho_name/, 'and saved');
    if (!(await ana.evaluate(`!!document.querySelector('#d2collab input[name=reply]')`))) {
      await ana.click('#morebtn');
      await ana.click('#collabbtn');
      await ana.waitFor(`document.querySelector('#d2collab input[name=reply]')`, { what: 'Ana\'s paste box' });
    }
    await ana.call((r) => { const i = document.querySelector('#d2collab input[name=reply]'); i.value = r; i.form.requestSubmit(); return true; }, reply);
    await ben.waitFor(`document.getElementById('crumbname')?.textContent === 'sample.elf' && /sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'Ben has the session\'s program', timeout: 60000 });
    await idle(ben);
    assert.equal(await ben.evaluate(`localStorage.getItem(${JSON.stringify(`kuna.d2.shared.${machoHash}`)})`), null, 'nothing is saved as a session\'s copy of the other program');
    const own = await ben.evaluate(`localStorage.getItem(${JSON.stringify(`kuna.d2.session.${machoHash}`)}) || ''`);
    assert.ok(/bens macho note/.test(own) && !/bens_macho_name/.test(own), 'its own store keeps Ben\'s note, without the undone edit');
  });

  await test('fifth review #2 a program received while joining never offers an old session\'s copy of it, even after an earlier open was stopped', async () => {
    const ana = await tab('Ana');
    await open(ana);
    await example(ana);
    const link = await inviteLink(ana, 'Ana');
    const ben = await tab('Ben', { other: true, script: DELAY_SHIM });
    await open(ben, { seed: [[`kuna.d2.shared.${SAMPLE_HASH}`, stored([fnRec(SUM, 'from_an_old_session')])]] });
    await ben.evaluate('window.__kunaDelay = 4000; true');
    await ben.click('#examplebtn');
    await sleep(1000);
    assert.equal(await ben.evaluate(`!document.getElementById('cancelbtn').disabled`), true, 'Ben is opening the example');
    await ben.evaluate(`document.getElementById('cancelbtn').click(); window.__kunaDelay = 0; true`);
    await idle(ben);
    await joinByHash(ben, link);
    await carryReply(ana, ben);
    await joinedBoth(ana, ben);
    await idle(ben);
    await sleep(1000);
    const said = await toasts(ben);
    assert.ok(!said.some((t) => /from a live session you were in/.test(t)), `no offer of an old session's copy while in this one: ${JSON.stringify(said)}`);
  });

  await test('fifth review #6 a join that fails while the received program is being listed lists it again with the student\'s own changes', async () => {
    const ana = await tab('Ana');
    await open(ana, { seed: [[OWN_KEY, stored([rawRec(1, 'readonly 0x2000+8')])]] });
    await example(ana);
    const link = await inviteLink(ana, 'Ana');
    const ben = await tab('Ben', { other: true, script: DELAY_SHIM + LIST_ARGS });
    await open(ben, { seed: [[OWN_KEY, stored([varRec(MAIN, 'v1', 'bens_own'), rawRec(1, 'volatile 0x3000+4')])]] });
    await ben.navigate(link);
    await nameAndGo(ben, 'Ben');
    await ben.evaluate('window.__kunaDelay = 4000; true');
    await carryReply(ana, ben);
    await ben.waitFor(`/Finding the functions in sample.elf/.test(document.getElementById('status').textContent)`, { what: 'Ben lists the program', timeout: 20000 });
    await ana.closeTab();
    tabs = tabs.filter((t) => t !== ana);
    await ben.waitFor(`/closed the connection|stopped answering/.test((document.getElementById('d2collab')?.textContent || '') + [...document.querySelectorAll('.d2-toast')].map((t) => t.textContent).join(' '))`, { what: 'Ben is told', timeout: 30000 });
    await ben.evaluate('window.__kunaDelay = 0; true');
    await closeDialog(ben);
    await ben.waitFor(`/bens_own/.test(document.getElementById('ccode').textContent)`, { what: 'Ben\'s own changes show', timeout: 30000 });
    await idle(ben);
    const last = (await ben.evaluate('window.__lists')).at(-1);
    assert.ok(last.includes('volatile 0x3000+4') && !last.includes('readonly 0x2000+8'), `the list shown is the one for Ben's own changes: ${JSON.stringify(last)}`);
  });

  await test('fourth review #10 in a session, an edit or a burst of typed bytes keeps no copy of the whole session for the solo Undo', async () => {
    const { ana, ben } = await pair();
    await ana.evaluate(`import('./session.js').then(({ Session }) => {
      window.__copies = 0;
      const snapshot = Session.prototype.snapshot;
      const pushUndo = Session.prototype.pushUndo;
      Session.prototype.snapshot = function (...a) { window.__copies++; return snapshot.apply(this, a); };
      Session.prototype.pushUndo = function (...a) { window.__copies++; return pushUndo.apply(this, a); };
      return true;
    })`);
    await front(ana);
    await popover(ana, '#ccode .t[data-sym="v1"]', 'n', 'first_name');
    await ana.key('Enter');
    await idle(ana);
    await popover(ana, '#ccode .t[data-sym="argc"]', 'n', 'second_name');
    await ana.key('Enter');
    await idle(ana);
    await ana.key('3');
    await ana.waitFor(`document.querySelector('#hexdump .hb[data-a="0x11e1"]')`, { what: 'the Bytes tab', timeout: 20000 });
    await ana.click('#hexdump .hb[data-a="0x11e1"]');
    await ana.key('9');
    await ana.key('0');
    await ana.key('Escape');
    const patched = `[...document.querySelectorAll('#sesslist .tx')].some((t) => /bytes 0x11e1 90/.test(t.title))`;
    await ben.waitFor(patched, { what: 'Ben has the patch', timeout: 20000 });
    await idle(ana);
    assert.equal(await ana.evaluate('window.__copies'), 0, 'no whole-session copy was made or kept');
    await ana.key('u');
    await ben.waitFor(`!${patched}`, { what: 'the shared Undo still takes the patch back', timeout: 20000 });
  });

  await test('fourth review #1 opening a cached function while another person\'s change is being decompiled stays on it', async () => {
    const { ana, ben } = await pair({ anaScript: DELAY_SHIM });
    await front(ana);
    await ana.click(`#fnlist .fn[data-addr="${SUM}"]`);
    await ana.waitFor(`document.getElementById('vname').textContent === 'sum_to' && document.getElementById('cancelbtn').disabled`, { what: 'Ana on sum_to', timeout: 30000 });
    await ana.click(`#fnlist .fn[data-addr="${MAIN}"]`);
    await ana.waitFor(`document.getElementById('vname').textContent === 'main' && document.getElementById('cancelbtn').disabled`, { what: 'Ana back on main', timeout: 30000 });
    await ana.evaluate('window.__kunaDelay = 4000; true');
    await front(ben);
    await popover(ben, '#ccode .t[data-sym="v1"]', 'n', 'bens_name');
    await ben.key('Enter');
    await front(ana);
    await ana.waitFor(`!document.getElementById('cancelbtn').disabled`, { what: 'Ana decompiles main again for Ben\'s change', timeout: 10000 });
    await ana.click(`#fnlist .fn[data-addr="${SUM}"]`);
    assert.equal(await text(ana, '#vname'), 'sum_to');
    await ana.evaluate('window.__kunaDelay = 0; true');
    await sleep(5000);
    assert.equal(await text(ana, '#vname'), 'sum_to', 'the view stays on the function Ana opened');
    assert.match(await ana.evaluate('location.hash'), /0x1161/);
    assert.match(await ben.evaluate(`document.querySelector('#d2roster .d2-who')?.title || ''`), /Ana: sum_to/, 'and the others see her there');
  });
} finally {
  clearTimeout(guard);
  chrome.close();
  chrome2.close();
  await server.close();
}

const failed = results.filter(([ok]) => !ok);
if (failed.length) {
  console.log(`DECOMPILE2 COLLAB PAGE FAIL — ${failed.length} of ${results.length}`);
  process.exit(1);
}
console.log(`DECOMPILE2 COLLAB PAGE OK — ${results.length} cases`);
process.exit(0);
