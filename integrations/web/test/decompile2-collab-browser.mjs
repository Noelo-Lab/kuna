// decompile2-collab-browser.mjs — live sessions in the real /decompile2/ page,
// in tabs of one headless Chrome: Ana makes an invite link; Ben opens it in
// another tab (the tabs meet over BroadcastChannel), receives the program and
// sees it open by itself; a rename on one page shows on the other; a rename
// and a retype made at the same moment both survive; a new decompiler effort
// re-decompiles everyone; Ana's pointer lands on the same line of C in Ben's
// window at 1440 and 1024 px, in C code and side by side, and hides when Ben
// opens another function; an Alt+click pings, and Go there opens it; a third
// person, in a second Chrome process, joins over WebRTC through the reply-link
// hand-off (a tab of Ana's browser opens the reply link and hands it to Ana's
// tab) and is introduced to Ben by the group; Undo leaves what someone else
// changed since; malformed messages are dropped and the page stays usable;
// leaving shows on the others' pages. Any uncaught page exception fails the run.
//
// Skips (exit 0) when there is no Chrome or no global WebSocket (Node < 22).
//   integrations/web/build.sh && node integrations/web/test/decompile2-collab-browser.mjs [--shots DIR]
import assert from 'node:assert/strict';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { findChrome, launchChrome, openPage, openTab } from './cdp-client.mjs';
import { requireDist, serveStatic } from './worker-harness.mjs';
import { decodeCode } from '../decompile2/collab/wire.js';

const chromePath = findChrome();
if (!chromePath || typeof WebSocket !== 'function') {
  console.log(`DECOMPILE2 COLLAB BROWSER SKIPPED — ${chromePath ? 'this Node has no global WebSocket (need 22+)' : 'no Chrome found (set CHROME=...)'}`);
  process.exit(0);
}
requireDist();

const shotsAt = process.argv.indexOf('--shots');
const shots = shotsAt > 0 ? process.argv[shotsAt + 1] : null;
if (shots) mkdirSync(shots, { recursive: true });
const sleep = (ms) => new Promise((done) => setTimeout(done, ms));
const server = await serveStatic();
const flags = ['--disable-features=WebRtcHideLocalIpsWithMdns'];
const chrome = await launchChrome(chromePath, { flags });
let chrome2 = null;
const guard = setTimeout(() => {
  console.error(`DECOMPILE2 COLLAB BROWSER FAIL — timed out after: ${done.join('; ')}`);
  chrome.close();
  chrome2?.close();
  process.exit(1);
}, 540000);
const done = [];
const pages = [];

/** Every toast a page shows, kept in `window.__toastLog`, so a failure can say what was said. */
const TOAST_LOG = `(() => {
  window.__toastLog = [];
  new MutationObserver((list) => {
    for (const m of list) for (const n of m.addedNodes) if (n.classList?.contains('d2-toast')) window.__toastLog.push(n.textContent);
  }).observe(document, { childList: true, subtree: true });
})();`;

/** A tab in the first browser, or (`other`) the first page of a second browser: another person's computer. */
async function tab(name, width = 1280, { other = false } = {}) {
  if (other) chrome2 ||= await launchChrome(chromePath, { flags });
  const page = other ? await openPage(chrome2.port) : pages.length ? await openTab(chrome.port) : await openPage(chrome.port);
  page.label = name;
  await page.viewport(width, 860);
  await page.send('Page.addScriptToEvaluateOnNewDocument', { source: TOAST_LOG });
  pages.push(page);
  return page;
}

const started = Date.now();
async function ok(step) {
  for (const p of pages) assert.deepEqual(p.exceptions, [], `no page exception in ${p.label} after: ${step}`);
  done.push(step);
  if (process.env.COLLAB_VERBOSE) console.log(`${((Date.now() - started) / 1000).toFixed(1)} s  ${step}`);
}

/**
 * Bring a tab to the front, as the window a person is using would be: Chrome
 * delivers pointer moves with animation frames, which a background tab does
 * not get, so a move there only arrives with the next input.
 */
const front = (p) => p.send('Page.bringToFront');

async function shot(page, name) {
  if (!shots) return;
  await front(page);
  await sleep(250);
  writeFileSync(join(shots, `${name}.png`), await page.screenshot());
}

const ready = (p) => p.waitFor(`document.getElementById('pick').getAttribute('aria-disabled') === null`, { what: `${p.label} ready`, timeout: 60000 });
const idle = (p) => p.waitFor(`document.getElementById('cancelbtn').disabled`, { what: `${p.label} idle`, timeout: 60000 });
const text = (p, sel) => p.call((s) => document.querySelector(s)?.textContent ?? '', sel);
const toasts = (p) => p.evaluate(`[...document.querySelectorAll('.d2-toast')].map((t) => t.textContent)`);
const code = (p) => text(p, '#ccode');
const setSelect = (p, id, value) => p.call((i, v) => {
  const sel = document.getElementById(i);
  sel.value = v;
  sel.dispatchEvent(new Event('change'));
  return true;
}, id, value);

/** Type a name into the open collab dialog's name field (replacing what is there) and press Enter. */
async function nameAndGo(p, name) {
  await p.waitFor(`document.getElementById('d2collab')?.open && document.querySelector('#d2collab input[name=name]')`, { what: `${p.label} name field` });
  await p.call(() => { const i = document.querySelector('#d2collab input[name=name]'); i.focus(); i.select(); return true; });
  await p.type(name);
  await p.key('Enter');
}

/** Ana's page: ⋯ → Work together… → a fresh invite link. */
async function inviteLink(p, name = null) {
  const open = await p.evaluate(`!!document.getElementById('d2collab')?.open`);
  if (!open) {
    await p.click('#morebtn');
    await p.click('#collabbtn');
    await p.waitFor(`document.getElementById('d2collab')?.open`, { what: 'collab dialog' });
  }
  if (name) await nameAndGo(p, name);
  else await p.click('#d2collab [data-act=invite]');
  await p.waitFor(`document.querySelector('#d2collab [data-copytext]')?.value.includes('#join=')`, { what: 'invite link', timeout: 20000 });
  return p.evaluate(`document.querySelector('#d2collab [data-copytext]').value`);
}

const closeDialog = (p) => p.evaluate(`(() => { const d = document.getElementById('d2collab'); if (d?.open) d.close(); return true; })()`);

/** The pointer `peer` draws on page `p`: its tip, and the rect of what it should be on. */
const pointer = (p, sel) => p.call((s) => {
  const n = document.querySelector('.d2-ptr:not([hidden])');
  if (!n) return null;
  const tip = n.querySelector('.d2-ptr-arrow').getBoundingClientRect();
  const target = document.querySelector(s)?.getBoundingClientRect();
  return { x: tip.left + 1, y: tip.top + 1, anchor: n.dataset.anchor, target: target && [target.left, target.top, target.right, target.bottom] };
}, sel);
const inside = (pt, r, slack = 2) => !!pt && !!r && pt.x >= r[0] - slack && pt.x <= r[2] + slack && pt.y >= r[1] - slack && pt.y <= r[3] + slack;

try {
  // ── Ana opens the example and makes an invite link ─────────────────────────
  const ana = await tab('Ana');
  await ana.navigate(`${server.base}/decompile2/`);
  await ready(ana);
  await ana.evaluate(`localStorage.clear(); true`);
  await ana.click('#examplebtn');
  await ana.waitFor(`/sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'Ana: main', timeout: 60000 });
  await idle(ana);
  const link1 = await inviteLink(ana, 'Ana');
  assert.match(link1, /\/decompile2\/#join=[A-Za-z0-9_-]+$/, 'an invite is a link to the page');
  assert.ok(link1.length < 600, `short enough to paste anywhere (${link1.length} characters)`);
  assert.match(await text(ana, '#d2collab'), /Send this link to one person/);
  await shot(ana, 'invite');
  await ok('Ana makes an invite link');

  // ── Ben opens it in another tab, with no program open ──────────────────────
  const ben = await tab('Ben', 1440);
  await ben.navigate(link1);
  await ben.waitFor(`document.getElementById('d2collab')?.open`, { what: 'join dialog', timeout: 60000 });
  assert.match(await text(ben, '#d2collab'), /Ana invites you to work on sample\.elf/);
  assert.match(await text(ben, '#d2collab'), /sends you a copy of the program/, 'the guest is told the program comes to them');
  assert.equal(await ben.evaluate('location.hash'), '', 'the code is taken out of the address');
  await nameAndGo(ben, 'Ben');
  await ben.waitFor(`document.getElementById('crumbname')?.textContent === 'sample.elf'`, { what: 'Ben receives the program', timeout: 60000 });
  await ben.waitFor(`/sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'Ben: main opens by itself', timeout: 60000 });
  await idle(ben);
  assert.ok((await toasts(ben)).some((t) => /You joined Ana's session/.test(t)));
  await ana.waitFor(`document.querySelectorAll('#d2roster .d2-who').length === 1`, { what: 'Ana sees Ben', timeout: 20000 });
  assert.match(await ana.evaluate(`document.querySelector('#d2roster .d2-who').title`), /^Ben: main, C code/, 'the roster says where Ben is');
  assert.equal(await text(ben, '#d2roster .d2-who'), 'A', 'initials in the top bar');
  assert.equal(await ben.evaluate(`document.getElementById('d2collab').open`), false, 'the join dialog closes once in');
  await closeDialog(ana);
  await ok('Ben opens the link in another tab, receives the program and it opens by itself');

  // ── edits ──────────────────────────────────────────────────────────────────
  await ana.click('#c-L5 .t[data-sym="v1"]');
  await ana.key('n');
  await ana.waitFor(`!document.getElementById('d2pop').hidden`, { what: 'rename popover' });
  await ana.type('total');
  await ana.key('Enter');
  await idle(ana);
  await ben.waitFor(`/\\btotal = sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'the rename on Ben\'s page', timeout: 30000 });
  await idle(ben);
  assert.match(await text(ben, '#status'), /Ana renamed v1 to total/);
  assert.match(await text(ben, '#railchanges h3'), /^Changes/, '"Your changes" becomes "Changes"');
  assert.match(await ben.evaluate(`document.querySelector('#sesslist li').getAttribute('style') || ''`), /--who:#e8404e/, 'each change in its author\'s colour');
  assert.equal(await ben.evaluate(`document.querySelector('#sesslist li .tx').firstChild?.textContent`), 'Ana', 'the author\'s name leads the row, on its first line');
  await ok('a rename on Ana\'s page shows on Ben\'s');

  await ana.click('#c-L5 .t[data-sym="total"]');
  await ana.key('n');
  await ben.click('#c-L5 .t[data-sym="total"]');
  await ben.key('y');
  await ana.waitFor(`!document.getElementById('d2pop').hidden`, { what: 'Ana rename popover' });
  await ben.waitFor(`!document.getElementById('d2pop').hidden`, { what: 'Ben retype popover' });
  await ana.call(() => { const i = document.querySelector('#d2pop input'); i.value = 'sum'; i.dispatchEvent(new Event('input', { bubbles: true })); return true; });
  await ben.call(() => { const i = document.querySelector('#d2pop input'); i.value = 'unsigned long'; i.dispatchEvent(new Event('input', { bubbles: true })); return true; });
  await Promise.all([ana.key('Enter'), ben.key('Enter')]);
  for (const p of [ana, ben]) {
    await p.waitFor(`/unsigned long sum;/.test(document.getElementById('ccode').textContent)`, { what: `${p.label}: rename and retype both kept`, timeout: 30000 });
  }
  await idle(ana);
  await idle(ben);
  await ok('a rename and a retype made at the same moment both survive on both pages');

  await setSelect(ana, 'mode', 'fast');
  await ben.waitFor(`document.getElementById('mode').value === 'fast'`, { what: 'Ben takes the new effort', timeout: 30000 });
  await ben.waitFor(`document.querySelectorAll('#fnlist .fn').length > 5 && /sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'Ben re-decompiled', timeout: 60000 });
  await idle(ben);
  assert.ok((await toasts(ben)).some((t) => /Ana switched the decompiler effort to fast/.test(t)));
  await setSelect(ben, 'mode', 'auto');
  await ana.waitFor(`document.getElementById('mode').value === 'auto'`, { what: 'and back, from Ben', timeout: 30000 });
  await idle(ana);
  await idle(ben);
  await ben.waitFor(`/unsigned long sum;/.test(document.getElementById('ccode').textContent)`, { what: 'Ben: main again', timeout: 60000 });
  await ana.waitFor(`/unsigned long sum;/.test(document.getElementById('ccode').textContent)`, { what: 'Ana: main again', timeout: 60000 });
  await ok('the decompiler effort is shared: a change re-decompiles the other page');

  // ── pointers ───────────────────────────────────────────────────────────────
  const target = '#c-L6 .t[data-kind=funcname]';
  await front(ana);
  for (const [width, view] of [[1440, 'c'], [1024, 'c'], [1440, 'split'], [1024, 'split']]) {
    await ben.viewport(width, 860);
    if ((await ben.evaluate(`document.querySelector('#tabs [aria-selected=true]').dataset.tab`)) !== view) {
      await ben.click(view === 'split' ? '#splitbtn' : '#tab-c');
    }
    await ana.hover('#c-L5 .ct');
    await ben.waitFor(`document.querySelector('.d2-ptr:not([hidden])')?.dataset.anchor === 'c:5'`, { what: `Ana's pointer on line 5 at ${width} px, ${view}`, timeout: 10000 });
    await ana.hover(target);
    let pt = null;
    for (let i = 0; i < 40 && !inside(pt, pt?.target, 3); i++) {
      await sleep(100);
      pt = await pointer(ben, target);
    }
    assert.ok(inside(pt, pt?.target, 3), `Ana's arrow lands on printf in Ben's window at ${width} px, ${view} (${JSON.stringify(pt)})`);
  }
  await ben.viewport(1440, 860);
  await ben.click('#tab-c');
  await ana.hover(target);
  await ben.waitFor(`document.querySelector('.d2-ptr:not([hidden])') && !document.querySelector('.d2-ptr.quiet')`, { what: 'the name tag shows while the pointer moves', timeout: 5000 });
  await shot(ben, 'pointer');
  await ben.waitFor(`document.querySelector('.d2-ptr.quiet:not([hidden])')`, { what: 'and fades once it rests', timeout: 5000 });
  assert.equal(await ben.evaluate(`getComputedStyle(document.querySelector('.d2-ptr .d2-ptr-arrow path')).fillOpacity`), '0.45', 'the arrow stays at its translucency');
  await front(ana);
  await ana.hover('#c-L5 .ct');
  await ben.waitFor(`document.querySelector('.d2-ptr:not([hidden]):not(.quiet)')?.dataset.anchor === 'c:5'`, { what: 'the tag comes back on the next move', timeout: 5000 });
  await ana.hover(target);
  const noArrow = `!document.querySelector('.d2-ptr:not([hidden])')`;
  await ben.click('#fnlist .fn[data-addr="0x1161"]');
  await ben.waitFor(`document.getElementById('vname').textContent === 'sum_to'`, { what: 'Ben opens sum_to', timeout: 60000 });
  await ben.waitFor(noArrow, { what: 'the arrow hides while Ben\'s page decompiles another function', timeout: 3000 });
  await idle(ben);
  await sleep(1200);
  assert.equal(await pointer(ben, target), null, 'and stays hidden there, though that function has a line 6 too');
  assert.match(await ana.evaluate(`document.querySelector('#d2roster .d2-who').title`), /^Ben: sum_to, C code/, 'and Ana\'s roster says where he is');
  await ok('Ana\'s pointer lands on the same name at 1440 and 1024 px, in C code and side by side, and hides elsewhere');

  // ── following ──────────────────────────────────────────────────────────────
  const benTab = () => ben.evaluate(`document.querySelector('#tabs [aria-selected=true]').dataset.tab`);
  await ben.click('#d2roster .d2-who');
  await ben.waitFor(`document.getElementById('vname').textContent === 'main'`, { what: 'a click on Ana\'s initials follows her to main', timeout: 60000 });
  assert.equal(await ben.evaluate(`document.querySelector('#d2roster .d2-who').getAttribute('aria-pressed')`), 'true');
  await ana.click('#tab-asm');
  await ben.waitFor(`document.querySelector('#tabs [aria-selected=true]').dataset.tab === 'asm'`, { what: 'and into the assembly', timeout: 10000 });
  await ben.key('Escape');
  await ana.click('#tab-c');
  await sleep(1200);
  assert.equal(await benTab(), 'asm', 'a key press stops following');
  await ben.click('#tab-c');
  await idle(ben);
  await ok('a click on someone\'s initials follows them until you press a key');

  // ── pings ──────────────────────────────────────────────────────────────────
  await front(ana);
  const p7 = await ana.call(() => { const r = document.querySelector('#c-L7 .ct').getBoundingClientRect(); return { x: r.left + 40, y: r.top + r.height / 2 }; });
  await ana.send('Input.dispatchMouseEvent', { type: 'mouseMoved', x: p7.x, y: p7.y });
  await ana.send('Input.dispatchMouseEvent', { type: 'mousePressed', x: p7.x, y: p7.y, button: 'left', clickCount: 1, modifiers: 1 });
  await ana.send('Input.dispatchMouseEvent', { type: 'mouseReleased', x: p7.x, y: p7.y, button: 'left', clickCount: 1, modifiers: 1 });
  await ben.waitFor(`[...document.querySelectorAll('.d2-toast')].some((t) => /Ana pinged line 7 of main/.test(t.textContent))`, { what: 'the ping toast', timeout: 10000 });
  assert.equal(await ana.evaluate(`document.querySelectorAll('#ccode .hl-sel').length`), 0, 'an Alt+click pings without selecting');
  assert.deepEqual(await ana.evaluate(`[...document.querySelectorAll('.d2-ping')].map((r) => r.dataset.anchor)`), ['c:7'], 'the sender sees exactly the one thing pinged ringed');
  await ben.call(() => {
    const toast = [...document.querySelectorAll('.d2-toast')].find((t) => /Ana pinged line 7/.test(t.textContent));
    toast.querySelector('[data-act=toast]').click();
    return true;
  });
  await ben.waitFor(`document.getElementById('vname').textContent === 'main'`, { what: 'Go there opens main', timeout: 60000 });
  await ben.waitFor(`document.querySelector('.d2-ping[data-anchor="c:7"]')`, { what: 'line 7 rings on Ben\'s page', timeout: 5000 });
  assert.equal(await ben.evaluate(`document.querySelectorAll('.d2-ping').length`), 1, 'exactly one ring for one ping, after Go there too');
  await front(ana);
  await ana.hover('#c-L6 .ct');
  await ana.key('p');
  await ben.waitFor(`document.querySelector('.d2-ping[data-anchor="c:6"]')`, { what: 'p pings the line under the mouse', timeout: 5000 });
  assert.deepEqual(await ben.evaluate(`[...document.querySelectorAll('.d2-ping')].map((r) => r.dataset.anchor)`), ['c:6'], 'a new ping from Ana replaces her last ring');
  await ben.waitFor(`document.querySelector('.d2-ptr:not([hidden])')?.dataset.anchor === 'c:6'`, { what: 'Ana\'s pointer where she pinged', timeout: 5000 });
  await shot(ben, 'session');
  await ok('Alt+click (and p) pings; Go there opens the pinged line');

  // ── a third person, in another browser, over WebRTC and the reply-link hand-off ──
  const link2 = await inviteLink(ana);
  const cy = await tab('Cy', 1280, { other: true });
  await cy.navigate(link2);
  await nameAndGo(cy, 'Cy');
  await cy.waitFor(`document.querySelector('#d2collab [data-copytext]')?.value.includes('#reply=')`, { what: 'Cy\'s reply link', timeout: 20000 });
  assert.match(await text(cy, '#d2collab'), /Send this reply link back to Ana/);
  await shot(cy, 'reply-link');
  const reply = await cy.evaluate(`document.querySelector('#d2collab [data-copytext]').value`);
  const hand = await tab('reply tab');
  await hand.navigate(reply);
  await hand.waitFor(`/Connected — you can close this tab/.test(document.getElementById('d2collab')?.textContent || '')`, { what: 'the reply tab hands over and connects', timeout: 40000 });
  await shot(hand, 'reply-tab');
  const again = await tab('reply again');
  await again.navigate(reply);
  await again.waitFor(`/This reply is for an invite that is no longer open/.test(document.getElementById('d2collab')?.textContent || '')`, { what: 'a reply opened twice says the invite is closed', timeout: 20000 });
  await cy.waitFor(`document.getElementById('crumbname')?.textContent === 'sample.elf' && /sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'Cy has the program', timeout: 60000 });
  await idle(cy);
  for (const p of [ana, ben, cy]) {
    await p.waitFor(`document.querySelectorAll('#d2roster .d2-who:not(.wait)').length === 2`, { what: `${p.label} sees the two others`, timeout: 30000 });
  }
  assert.match(await text(ana, '#d2collab [data-status]'), /Cy joined/, 'Ana\'s invite says who came in');
  await ana.click('#d2collab [data-act=invite]');
  await ana.waitFor(`document.querySelector('#d2collab input[name=reply]')`, { what: 'a fresh invite\'s reply box' });
  await ana.call((r) => { const i = document.querySelector('#d2collab input[name=reply]'); i.value = r; i.form.requestSubmit(); return true; }, reply);
  await ana.waitFor(`/This reply is for an invite that is no longer open/.test(document.querySelector('#d2collab [data-status]')?.textContent || '')`, { what: 'an old reply pasted into a new invite', timeout: 10000 });
  await closeDialog(ana);
  for (const width of [1024, 820]) {
    await ana.viewport(width, 860);
    await sleep(200);
    const overflow = await ana.evaluate(`document.documentElement.scrollWidth - document.documentElement.clientWidth`);
    assert.ok(overflow <= 0, `no horizontal page overflow at ${width} px with three people (got ${overflow})`);
  }
  await ana.viewport(1280, 860);
  assert.match(await code(cy), /unsigned long sum;/, 'the newcomer has the session\'s changes');
  await cy.click('#fnlist .fn[data-addr="0x1161"]');
  await cy.waitFor(`document.getElementById('vname').textContent === 'sum_to'`, { what: 'Cy opens sum_to', timeout: 60000 });
  await idle(cy);
  await cy.click('#fnrenamebtn');
  await cy.waitFor(`!document.getElementById('d2pop').hidden`, { what: 'function rename' });
  await cy.call(() => { const i = document.querySelector('#d2pop input'); i.value = 'summation'; i.dispatchEvent(new Event('input', { bubbles: true })); return true; });
  await cy.key('Enter');
  for (const p of [ana, ben]) {
    await p.waitFor(`/summation\\(add/.test(document.getElementById('ccode').textContent)`, { what: `${p.label} sees Cy's rename`, timeout: 30000 });
  }
  await ok('a third page joins over WebRTC via the reply-link hand-off, and is introduced to Ben by the group');

  // ── Undo only takes back your own, and not what someone changed since ──────
  await idle(ana);
  const printfLine = await ana.call(() => [...document.querySelectorAll('#ccode .d2-cl')].find((l) => /printf\(/.test(l.textContent)).id);
  await ana.click(`#${printfLine} .ct`);
  await ana.key(';');
  await ana.waitFor(`!document.getElementById('d2pop').hidden`, { what: 'note popover' });
  await ana.type('prints it');
  await ana.key('Enter');
  await ben.waitFor(`/prints it/.test(document.getElementById('ccode').textContent)`, { what: 'the note on Ben\'s page', timeout: 30000 });
  await idle(ben);
  await ben.click('#sesslist li[data-key^="comment:"] [data-act=edit-edit]');
  await ben.waitFor(`!document.getElementById('d2pop').hidden`, { what: 'Ben edits the note from Changes' });
  await ben.call(() => { const i = document.querySelector('#d2pop input'); i.value = 'prints the sum'; i.dispatchEvent(new Event('input', { bubbles: true })); return true; });
  await ben.key('Enter');
  await ana.waitFor(`/prints the sum/.test(document.getElementById('ccode').textContent)`, { what: 'Ben\'s note on Ana\'s page', timeout: 30000 });
  assert.ok((await toasts(ana)).some((t) => /Ben changed the note at .* after you/.test(t)), 'Ana hears that Ben replaced her note');
  await idle(ana);
  await ana.click('#ccode');
  await ana.key('u');
  await ana.waitFor(`[...document.querySelectorAll('.d2-toast')].some((t) => /Ben changed that after you, so it was not undone/.test(t.textContent))`, { what: 'undo says why', timeout: 10000 });
  assert.match(await code(ana), /prints the sum/, 'Ben\'s note stays');
  await shot(ben, 'changes');
  await ok('Undo leaves what someone else changed since, and says so');

  // ── malformed messages from a same-origin tab are dropped ──────────────────
  const spy = await tab('spy');
  await spy.navigate(`${server.base}/decompile2/`);
  const seen = spy.call((id) => new Promise((resolve) => {
    const ch = new BroadcastChannel(`kuna.d2.link.${id}`);
    ch.onmessage = ({ data }) => { if (data?.k === 'm' && data.from && data.to) resolve({ from: data.from, to: data.to }); };
    setTimeout(() => resolve(null), 15000);
  }), (await decodeCode(link1.split('#join=')[1], 'invite')).id);
  await sleep(200);
  await front(ana);
  await ana.hover('#c-L5 .ct');
  await ana.hover('#c-L7 .ct');
  await ana.hover('#c-L5 .ct');
  const pair = await seen;
  assert.ok(pair, 'the spy sees the Ana–Ben link');
  const junk = ['not json', '{"t":"ops","ops":[{"k":"raw:eeeeeeee:1","v":"@/etc/passwd","c":[99999,"eeeeeeee"]}]}',
    JSON.stringify({ t: 'ops', ops: [{ k: 'comment:0x1198:0x11b5', v: 'x\nbytes 0x1 @/etc/passwd', c: [99999, 'eeeeeeee'] }] }),
    JSON.stringify({ t: 'where', fn: 'main', view: 'c' }), 'x'.repeat(300 << 10), JSON.stringify({ t: 'hello', proto: 1 }),
    JSON.stringify({ t: 'ping', fn: '0x1198', view: 'c', anchor: '<img src=x onerror=alert(1)>' }), null];
  await spy.call((p, list) => {
    const ch = new BroadcastChannel(`kuna.d2.link.${p.id}`);
    for (const d of list) ch.postMessage({ k: 'm', from: p.from, to: p.to, c: 'e', d: d === null ? new ArrayBuffer(64) : d });
    for (let i = 0; i < 200; i++) ch.postMessage({ k: 'm', from: p.from, to: p.to, c: 'c', d: JSON.stringify({ t: 'cur', fn: '0x1198', view: 'c', anchor: 'c:3', fx: 0, fy: 0 }) });
    return true;
  }, { ...pair, id: (await decodeCode(link1.split('#join=')[1], 'invite')).id }, junk);
  await sleep(500);
  for (const p of [ana, ben]) {
    assert.ok(!/passwd/.test(await p.evaluate(`JSON.stringify([...document.querySelectorAll('#sesslist li')].map((li) => li.textContent))`)), `${p.label}: nothing hostile reached the session`);
  }
  await ana.click('#fnlist .fn[data-addr="0x1149"]');
  await ana.waitFor(`document.getElementById('vname').textContent === 'add'`, { what: 'Ana opens add', timeout: 60000 });
  await idle(ana);
  await ana.click('#fnrenamebtn');
  await ana.waitFor(`!document.getElementById('d2pop').hidden`, { what: 'rename add' });
  await ana.call(() => { const i = document.querySelector('#d2pop input'); i.value = 'plus'; i.dispatchEvent(new Event('input', { bubbles: true })); return true; });
  await ana.key('Enter');
  await ben.waitFor(`/plus\\(argc/.test(document.getElementById('ccode').textContent)`, { what: 'the link still works after the junk', timeout: 30000 });
  await ok('malformed, oversized and hostile messages are dropped; the pages stay usable');

  // ── leaving ────────────────────────────────────────────────────────────────
  await cy.click('#d2roster [data-act=collab-open]');
  await cy.waitFor(`document.getElementById('d2collab')?.open`, { what: 'Cy session dialog' });
  await cy.click('#d2collab [data-act=leave]');
  for (const p of [ana, ben]) {
    await p.waitFor(`[...document.querySelectorAll('.d2-toast')].some((t) => /Cy left the session/.test(t.textContent))`, { what: `${p.label} sees Cy leave`, timeout: 20000 });
    await p.waitFor(`document.querySelectorAll('#d2roster .d2-who').length === 1`, { what: `${p.label} roster`, timeout: 10000 });
  }
  assert.equal(await cy.evaluate(`document.getElementById('d2roster').hidden`), true, 'Cy\'s page is on its own again');
  assert.match(await code(cy), /summation/, 'and keeps the session\'s changes');
  await ok('leaving shows on the other pages; the one who left keeps the changes');

  console.log(`DECOMPILE2 COLLAB BROWSER OK — ${done.join('; ')}`);
} catch (e) {
  for (const p of pages) {
    await shot(p, `fail-${p.label.replace(/\W+/g, '-')}`).catch(() => {});
    console.error(`${p.label}: status "${await text(p, '#status').catch(() => '?')}", toasts ${JSON.stringify(await toasts(p).catch(() => []))}`);
    console.error(`  every toast: ${JSON.stringify(await p.evaluate('window.__toastLog || []').catch(() => '?'))}`);
    console.error(`  dialog: ${JSON.stringify(await p.evaluate(`document.getElementById('d2collab')?.open ? document.getElementById('d2collab').textContent.slice(0, 300) : null`).catch(() => '?'))}`);
  }
  console.error(`DECOMPILE2 COLLAB BROWSER FAIL after: ${done.join('; ')}`);
  throw e;
} finally {
  clearTimeout(guard);
  for (const p of pages) p.close();
  chrome.close();
  chrome2?.close();
  await server.close();
}
