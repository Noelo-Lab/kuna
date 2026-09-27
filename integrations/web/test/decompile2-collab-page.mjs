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
// made; a name with a line separator. Each case runs in fresh tabs (a second
// Chrome process stands in for another person's computer) and is reported;
// any failure exits 1.
//
// Skips (exit 0) when there is no Chrome or no global WebSocket (Node < 22).
// `--only TEXT` runs the cases whose name contains TEXT.
//   integrations/web/build.sh && node integrations/web/test/decompile2-collab-page.mjs
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { findChrome, launchChrome, openPage, openTab } from './cdp-client.mjs';
import { requireDist, serveStatic, fixture } from './worker-harness.mjs';

const chromePath = findChrome();
if (!chromePath || typeof WebSocket !== 'function') {
  console.log(`DECOMPILE2 COLLAB PAGE SKIPPED — ${chromePath ? 'this Node has no global WebSocket (need 22+)' : 'no Chrome found (set CHROME=...)'}`);
  process.exit(0);
}
requireDist();

const sleep = (ms) => new Promise((done) => setTimeout(done, ms));
const server = await serveStatic();
const flags = ['--disable-features=WebRtcHideLocalIpsWithMdns'];
const chrome = await launchChrome(chromePath, { flags });
const chrome2 = await launchChrome(chromePath, { flags });
const guard = setTimeout(() => { console.error('DECOMPILE2 COLLAB PAGE FAIL — timed out'); chrome.close(); chrome2.close(); process.exit(1); }, 900000);
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
  await p.waitFor(`/sum_to/.test(document.getElementById('ccode').textContent)`, { what: `${p.label}: main`, timeout: 60000 });
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
