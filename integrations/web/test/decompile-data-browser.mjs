// decompile-data-browser.mjs — the Data tab in the real /decompile/ page in
// headless Chrome: double-clicking a global in the code opens its data (the
// row selected and in view, its value decoded) instead of renaming it, a
// local still renames and a call still opens the function; a pointer to a
// function opens it and "Used by" opens the function at the using
// instruction, with Back returning each time; renaming and retyping stay one
// click away and the retype decodes the values (an int table, a struct); a
// .bss buffer starts as zeros; the section menu goes to .rodata and the
// empty tab lists the sections; a data address in the assembly goes there
// too; the stripped twin shows the decompiler's own globals; and the tab
// keeps its layout in the light theme and at narrow widths. Any uncaught page
// exception fails the run.
//
// Skips (exit 0) when there is no Chrome or no global WebSocket (Node < 22).
//
// Usage:  integrations/web/build.sh && node integrations/web/test/decompile-data-browser.mjs [--shots DIR]
import assert from 'node:assert/strict';
import { writeFileSync, mkdirSync } from 'node:fs';
import { join } from 'node:path';
import { findChrome, launchChrome, openPage } from './cdp-client.mjs';
import { requireDist, serveStatic, openSample } from './worker-harness.mjs';

const chromePath = findChrome();
if (!chromePath || typeof WebSocket !== 'function') {
  console.log(`DECOMPILE DATA BROWSER SKIPPED — ${chromePath ? 'this Node has no global WebSocket (need 22+)' : 'no Chrome found (set CHROME=...)'}`);
  process.exit(0);
}
requireDist();
const shotsAt = process.argv.indexOf('--shots');
const shots = shotsAt > 0 ? process.argv[shotsAt + 1] : null;
if (shots) mkdirSync(shots, { recursive: true });

const sleep = (ms) => new Promise((done) => setTimeout(done, ms));
const server = await serveStatic();
const chrome = await launchChrome(chromePath);
const guard = setTimeout(() => { console.error('DECOMPILE DATA BROWSER FAIL — timed out'); chrome.close(); process.exit(1); }, 300000);
const done = [];
let page;

async function noExceptions(step) {
  assert.deepEqual(page.exceptions, [], `no page exception after: ${step}`);
  done.push(step);
}
const ready = () => page.waitFor(`document.getElementById('pick').getAttribute('aria-disabled') === null`, { what: 'page ready', timeout: 60000 });
const idle = (what) => page.waitFor(`document.getElementById('cancelbtn').disabled`, { what, timeout: 60000 });
const text = (sel) => page.call((s) => document.querySelector(s)?.textContent ?? '', sel);
const view = () => page.evaluate(`document.querySelector('#tabs [aria-selected=true]').dataset.tab`);
const secHead = () => page.call(() => {
  const s = document.querySelector('#datalist .dv-sec');
  const pick = s?.querySelector('select.dv-pick');
  return `${pick ? pick.selectedOptions[0]?.textContent : s?.querySelector('b')?.textContent} ${s?.querySelector('span')?.textContent}`;
});
const rowValue = (hex) => text(`#datalist .dv-row[data-key="${hex}"] .dv-v`);
const shot = async (name) => {
  if (shots) writeFileSync(join(shots, `data-${name}.png`), Buffer.from(await page.screenshot(), 'base64'));
};
const inView = (sel) => page.call((s) => {
  const el = document.querySelector(s);
  const pane = document.getElementById('datalist');
  if (!el || !pane) return false;
  const r = el.getBoundingClientRect();
  const p = pane.getBoundingClientRect();
  return r.top >= p.top && r.bottom <= p.bottom && pane.offsetParent !== null;
}, sel);
const dataShown = (hex, what) => page.waitFor(`document.querySelector('#datalist .dv-row.sel[data-a="${hex}"]') && document.querySelector('#tabs [aria-selected=true]').dataset.tab === 'data'`, { what, timeout: 60000 });
const usesShown = () => page.waitFor(`document.querySelector('#databar .dv-used a[data-act=dv-use]') || /No code uses/.test(document.querySelector('#databar .dv-used')?.textContent || '')`, { what: 'Used by', timeout: 60000 });
const dblclickToken = (name) => page.call((n) => {
  const t = [...document.querySelectorAll('#ccode .t')].find((x) => x.textContent === n && x.dataset.decl == null);
  if (!t) return false;
  t.scrollIntoView({ block: 'center' });
  const r = t.getBoundingClientRect();
  const opts = { bubbles: true, clientX: r.left + 2, clientY: r.top + r.height / 2 };
  t.dispatchEvent(new MouseEvent('click', { ...opts, detail: 1 }));
  t.dispatchEvent(new MouseEvent('click', { ...opts, detail: 2 }));
  t.dispatchEvent(new MouseEvent('dblclick', { ...opts, detail: 2 }));
  return true;
}, name);
const edit = async (button, value) => {
  await page.click(`#databar [data-act=${button}]`);
  await page.waitFor(`!document.getElementById('d2pop').hidden`, { what: `${button} popover` });
  await page.call(() => { document.querySelector('#d2pop input').select(); return true; });
  await page.type(value);
  await page.key('Enter');
};

try {
  page = await openPage(chrome.port);
  await page.viewport(1440, 900);
  await page.navigate(`${server.base}/decompile/`);
  await ready();
  await page.evaluate(`localStorage.clear(); true`);
  await page.navigate(`${server.base}/decompile/`);
  await ready();
  await openSample(page, 'globals.elf');
  await page.waitFor(`document.querySelector('#ccode .t[data-sym="primes"]')`, { what: 'main open', timeout: 60000 });
  await idle('main decompiled');
  assert.equal(await page.evaluate(`document.querySelector('#ccode .t[data-sym="primes"]').dataset.gaddr`), '0x4040', 'a global in the code carries its address');
  assert.equal(await page.evaluate(`document.querySelector('#ccode .t[data-sym="total"]:not([data-decl])').dataset.gaddr ?? null`), null, 'a local does not');
  await noExceptions('globals.elf: main open, its globals carry their addresses');

  await page.key('5');
  assert.equal(await view(), 'data');
  assert.match(await text('#datalist .dv-empty'), /^Double-click a global variable in the code to see its data here/, 'the Data tab says how to fill it');
  assert.ok(await page.evaluate(`[...document.querySelectorAll('#datalist .dv-empty a')].some((a) => a.textContent === 'primes')`), 'and lists the data this function uses');
  const listed = await page.evaluate(`[...document.querySelectorAll('#datalist .dv-slist li')].map((li) => li.textContent)`);
  for (const want of [/^\.rodata data that never changes · \d/, /^\.data data the program can change · 136 bytes$/, /^\.bss data that starts as zeros · \d+ bytes$/]) {
    assert.ok(listed.some((t) => want.test(t)), `the sections are listed: ${want} in ${JSON.stringify(listed)}`);
  }
  assert.ok(!listed.some((t) => t.startsWith('.text')), 'code is not');
  await page.key('1');
  assert.equal(await view(), 'c');
  await noExceptions('the Data tab before any data: what to do, the data main uses, and the sections to look through');

  assert.ok(await dblclickToken('primes'));
  await dataShown('0x4040', 'Data tab at primes');
  assert.equal(await page.evaluate(`document.getElementById('d2pop').hidden`), true, 'double-clicking a global opens no rename dialog');
  assert.equal(await page.evaluate(`document.getElementById('pane-data').hidden`), false);
  assert.ok(await inView('#datalist .dv-row.sel'), 'the selected row is in view');
  assert.match(await text('#databar .dv-what'), /^primes 32 bytes at 4040 in \.data/);
  assert.match(await secHead(), /^\.data data the program can change/);
  assert.equal(await rowValue('0x4020'), '"hello, data"', 'text reads as text');
  assert.equal(await rowValue('0x4070'), '→ "kuna data tab"', 'a pointer to a string shows the string');
  assert.equal(await rowValue('0x4078'), '→ twice', 'a pointer to a function names it');
  assert.equal(await rowValue('0x4080'), '→ primes + 0xc');
  await usesShown();
  assert.equal(await text('#databar .dv-used'), 'Used by main', 'who uses it');
  await shot('dark-wide');
  await noExceptions('double-click a global: the Data tab, the row selected and in view, values decoded, who uses it');

  await page.click('#databar a[data-act=dv-use]');
  await page.waitFor(`document.querySelector('#tabs [aria-selected=true]').dataset.tab === 'c' && document.querySelector('#asmcode #a-0x11cb, #ccode .d2-cl.hl-sel')`, { what: 'Used by → main', timeout: 60000 });
  await idle('main');
  assert.ok(await page.evaluate(`!!document.querySelector('#ccode .d2-cl.hl-sel .t[data-sym="primes"]')`), 'the line that uses it is selected');
  await page.key('ArrowLeft', { modifiers: 1 });
  await dataShown('0x4040', 'Back to the data');
  await page.key('ArrowLeft', { modifiers: 1 });
  await page.waitFor(`document.querySelector('#tabs [aria-selected=true]').dataset.tab === 'c'`, { what: 'Back to the code', timeout: 30000 });
  await page.key('ArrowRight', { modifiers: 1 });
  await dataShown('0x4040', 'Forward to the data again');
  await noExceptions('"Used by" opens main at the line; Back and Forward come back');

  await page.click('#datalist .dv-row[data-key="0x4078"] a[data-act=dv-follow]');
  await page.waitFor(`document.getElementById('vname').textContent === 'twice'`, { what: 'the pointer opens twice', timeout: 60000 });
  assert.notEqual(await view(), 'data', 'a function opens in the code');
  await page.key('ArrowLeft', { modifiers: 1 });
  await page.waitFor(`document.getElementById('vname').textContent === 'main'`, { what: 'back in main', timeout: 60000 });
  await dataShown('0x4040', 'and on its data');
  await page.click('#datalist .dv-row[data-key="0x4080"] a[data-act=dv-follow]');
  await page.waitFor(`document.querySelector('#datalist .dv-row.sel')?.dataset.a === '0x4040'`, { what: 'a pointer to data moves within the Data tab' });
  await page.click('#datalist .dv-row[data-key="0x4070"]');
  await page.key('ArrowDown');
  assert.equal(await page.evaluate(`document.querySelector('#datalist .dv-row.sel').dataset.key`), '0x4078', '↓ moves to the next row');
  await page.key('Enter');
  await page.waitFor(`document.getElementById('vname').textContent === 'twice'`, { what: 'Enter follows the pointer', timeout: 60000 });
  await page.key('ArrowLeft', { modifiers: 1 });
  await dataShown('0x4078', 'and Back returns to it');
  await page.click('#datalist .dv-row[data-key="0x4080"] a[data-act=dv-follow]');
  await page.waitFor(`document.querySelector('#datalist .dv-row.sel')?.dataset.a === '0x4040'`, { what: 'back at primes' });
  await noExceptions('a pointer to a function opens it; Back returns; a pointer to data moves within the tab; ↓ and Enter');

  await edit('dv-retype', 'int[8]');
  await page.waitFor(`/^\\{2, 3, 5, 7, 11, 13, 17, 19\\}$/.test(document.querySelector('#datalist .dv-row[data-key="0x4040"] .dv-v')?.textContent || '')`, { what: 'int table decoded', timeout: 60000 });
  await idle('retype');
  assert.equal(await text('#datalist .dv-row[data-key="0x4040"] .dv-t'), 'int[8]');
  if (await page.evaluate(`document.querySelector('#datalist .dv-row[data-key="0x4040"] [data-act=dv-toggle]').getAttribute('aria-expanded')`) === 'true') {
    await page.click('#datalist .dv-row[data-key="0x4040"] [data-act=dv-toggle]');
    await page.waitFor(`!document.querySelector('#datalist .dv-row[data-key="0x4040/7"]')`, { what: 'elements folded' });
  }
  await page.click('#datalist .dv-row[data-key="0x4040"] [data-act=dv-toggle]');
  await page.waitFor(`document.querySelector('#datalist .dv-row[data-key="0x4040/7"]')`, { what: 'elements shown' });
  assert.equal(await rowValue('0x4040/7'), '19 · 0x13');
  await page.click('#datalist .dv-row[data-key="0x4060"]');
  await edit('dv-retype', 'point');
  await page.waitFor(`/^\\{x = 3, y = 4, label = "origin"\\}$/.test(document.querySelector('#datalist .dv-row[data-key="0x4060"] .dv-v')?.textContent || '')`, { what: 'struct decoded', timeout: 60000 });
  await idle('retype origin');
  await page.click('#datalist .dv-row[data-key="0x4060"]');
  await edit('dv-rename', 'center');
  await page.waitFor(`document.querySelector('#datalist .dv-row[data-key="0x4060"] .dv-nm')?.textContent === 'center'`, { what: 'renamed in the Data tab', timeout: 60000 });
  await idle('rename origin');
  assert.deepEqual(await page.evaluate(`[...document.querySelectorAll('#sesslist li .tx')].map((li) => li.title.split('\\n')[0])`),
    ['data 0x4040 int primes[8]', 'data 0x4060 point center'], 'one change per global: its type and name');
  await shot('dark-typed');
  await page.key('1');
  await page.waitFor(`/center\.x/.test(document.getElementById('ccode').textContent)`, { what: 'the code reads the renamed struct\'s field' });
  assert.match(await text('#ccode'), /primes\[v2\]/, 'and the table by element');
  await page.key('5');
  assert.equal(await view(), 'data', '5 is the Data tab');
  assert.ok(await page.evaluate(`!!document.querySelector('#datalist .dv-row.sel')`), 'which kept its place');
  await noExceptions('Change type decodes an int table and a struct; Rename renames it everywhere');

  await page.click('#tab-c');
  await page.click('#ccode .t[data-sym="primes"]');
  await page.key('n');
  await page.waitFor(`!document.getElementById('d2pop').hidden`, { what: 'rename a global from the code' });
  assert.match(await text('#d2pop h3'), /global variable at 4040/, 'N on a global in the code still names it');
  await page.key('Escape');
  assert.match(await text('#railsel'), /global variable/, 'the Explain card calls it a global');
  assert.ok(await page.evaluate(`!!document.querySelector('#railsel [data-act=sel-data]')`), 'with a way to see its data');
  assert.ok(await dblclickToken('total'));
  await page.waitFor(`!document.getElementById('d2pop').hidden`, { what: 'double-click on a local still renames' });
  assert.match(await text('#d2pop h3'), /^Rename total/);
  await page.key('Escape');
  assert.equal(await view(), 'c');
  await noExceptions('a global renames with N from the code; a local still renames on double-click');

  assert.ok(await dblclickToken('scratch'));
  await dataShown('0x40c0', 'scratch in .bss');
  assert.match(await secHead(), /^\.bss data that starts as zeros/);
  assert.equal(await rowValue('0x40c0'), 'uninitialized', 'nothing in the file backs it');
  assert.equal(await text('#datalist [data-act=dv-more]'), 'Previous section: .data', 'the section before is one click away');
  await noExceptions('a .bss buffer starts as zeros');

  await page.call(() => {
    const pick = document.querySelector('#datalist select.dv-pick');
    pick.value = [...pick.options].find((o) => o.textContent === '.rodata').value;
    pick.dispatchEvent(new Event('change', { bubbles: true }));
    return true;
  });
  await page.waitFor(`document.querySelector('#datalist select.dv-pick')?.selectedOptions[0]?.textContent === '.rodata' && !document.querySelector('#datalist.dv-busy')`, { what: 'the section menu goes to .rodata', timeout: 60000 });
  assert.match(await secHead(), /^\.rodata data that never changes/);
  assert.ok(await page.evaluate(`[...document.querySelectorAll('#datalist .dv-row .dv-v')].some((v) => v.textContent === '"kuna data tab"')`), 'its strings read as text');
  await shot('rodata');
  await page.key('ArrowLeft', { modifiers: 1 });
  await dataShown('0x40c0', 'Back returns to scratch');
  await noExceptions('the section menu goes to .rodata, its strings read as text, and Back returns');

  await page.key('2');
  await page.waitFor(`document.querySelector('#asmcode a.xt.xd[data-goto="0x4078"]')`, { what: 'a data operand is a link', timeout: 30000 });
  assert.match(await text('#a-0x11fb .ao'), /<op>/, 'with the name of the data');
  await page.click('#asmcode a.xt.xd[data-goto="0x4078"]');
  await dataShown('0x4078', 'the assembly goes to the data');
  await noExceptions('a data address in the assembly goes to the Data tab');

  await page.click('#themebtn');
  await sleep(200);
  const light = await page.call(() => [document.documentElement.dataset.theme, getComputedStyle(document.querySelector('#datalist .dv-row.sel')).backgroundColor, getComputedStyle(document.getElementById('datalist')).color]);
  assert.equal(light[0], 'light');
  assert.notEqual(light[1], 'rgba(0, 0, 0, 0)', 'the selected row is marked in the light theme');
  await shot('light-wide');
  for (const width of [1024, 820, 600]) {
    await page.viewport(width, 860);
    await sleep(250);
    const overflow = await page.evaluate(`[document.documentElement.scrollWidth - document.documentElement.clientWidth, document.getElementById('datalist').scrollWidth - document.getElementById('datalist').clientWidth]`);
    assert.ok(overflow[1] <= 1, `the Data tab has no horizontal overflow at ${width}px (got ${overflow[1]})`);
    if (width > 600) assert.ok(overflow[0] <= 0, `no horizontal page overflow at ${width}px (got ${overflow[0]})`);
    if (width === 600) await shot('light-narrow');
  }
  await page.click('#themebtn');
  await sleep(150);
  await shot('dark-narrow');
  await page.viewport(1440, 900);
  await noExceptions('light theme; the Data tab at 1024, 820 and 600 px without horizontal overflow');

  await openSample(page, 'globals-stripped.elf');
  await page.waitFor(`document.getElementById('crumbname').textContent === 'globals-stripped.elf' && document.getElementById('vname').textContent === 'main'`, { what: 'stripped main', timeout: 60000 });
  await idle('stripped main');
  assert.equal(await view(), 'asm', 'a new program opens in the view the code was last read in, not the Data tab');
  assert.match(await text('#a-0x11cb .ao'), /<dat_4040>/, 'the assembly names the decompiler\'s own globals');
  await page.key('1');
  await page.waitFor(`document.querySelector('#ccode .t[data-sym="dat_4040"]')`, { what: 'stripped C' });
  assert.ok(await dblclickToken('dat_4040'));
  await dataShown('0x4040', 'Data tab at dat_4040');
  assert.equal(await rowValue('0x4040'), '{2, 3, 5, 7, 11, 13, 17, 19}', 'the decompiler\'s own int table decodes');
  assert.equal(await rowValue('0x4078'), '→ sub_1189', 'a function pointer the decompiler typed names its function');
  await shot('stripped');
  await noExceptions('the stripped twin: the decompiler\'s own globals');

  clearTimeout(guard);
  console.log(`DECOMPILE DATA BROWSER OK — ${done.join(' · ')}`);
} catch (e) {
  console.error('DECOMPILE DATA BROWSER FAIL —', e.message);
  if (page?.exceptions?.length) console.error('page exceptions:', page.exceptions);
  await shot('fail').catch(() => {});
  process.exitCode = 1;
} finally {
  clearTimeout(guard);
  chrome.close();
  server.close();
}
