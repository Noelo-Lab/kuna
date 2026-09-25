// decompile2-browser.mjs — drive the real /decompile2/ page (and /decompile/)
// in headless Chrome over the DevTools protocol: load the example through the
// file input, open main, hover a line, switch to Assembly, rename a variable,
// patch a byte, reload to see the session restored, and check the layout at
// 1024 and 820 px. Any uncaught page exception fails the run.
//
// Skips (exit 0) when there is no Chrome or no global WebSocket (Node < 22).
// Steps that need the engine's `inspect`/`--assert` surface assert the page's
// honest fallback instead when the built wasm predates it, and say so.
//
// Usage:  integrations/web/build.sh && node integrations/web/test/decompile2-browser.mjs
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { findChrome, launchChrome, openPage } from './cdp-client.mjs';
import { requireDist, serveStatic, fixture } from './worker-harness.mjs';

const chromePath = findChrome();
if (!chromePath || typeof WebSocket !== 'function') {
  console.log(`DECOMPILE2 BROWSER SKIPPED — ${chromePath ? 'this Node has no global WebSocket (need 22+)' : 'no Chrome found (set CHROME=...)'}`);
  process.exit(0);
}
requireDist();

const sleep = (ms) => new Promise((done) => setTimeout(done, ms));
const server = await serveStatic();
const chrome = await launchChrome(chromePath);
const guard = setTimeout(() => { console.error('DECOMPILE2 BROWSER FAIL — timed out'); chrome.close(); process.exit(1); }, 240000);
const done = [];
const skipped = [];
let page;

async function noExceptions(step) {
  assert.deepEqual(page.exceptions, [], `no page exception after: ${step}`);
  done.push(step);
}

/** Load the page's own example through the real file input, as a user would. */
const LOAD_EXAMPLE = `(async () => {
  const bytes = await (await fetch('./examples/sample.elf')).arrayBuffer();
  const dt = new DataTransfer();
  dt.items.add(new File([bytes], 'sample.elf'));
  const input = document.getElementById('file');
  input.files = dt.files;
  input.dispatchEvent(new Event('change', { bubbles: true }));
  return true;
})()`;

const text = (sel) => page.call((s) => document.querySelector(s)?.textContent ?? '', sel);
const count = (sel) => page.call((s) => document.querySelectorAll(s).length, sel);
const idle = (what) => page.waitFor(`document.getElementById('cancelbtn').disabled`, { what, timeout: 60000 });
const toasts = () => page.evaluate(`[...document.querySelectorAll('.d2-toast')].map((t) => t.textContent)`);
const setSelect = (id, value) => page.call((i, v) => {
  const sel = document.getElementById(i);
  sel.value = v;
  sel.dispatchEvent(new Event('change'));
  return true;
}, id, value);
const sampleHash = 'sha256:' + createHash('sha256').update(readFileSync(fixture('sample.elf'))).digest('hex');

try {
  page = await openPage(chrome.port);
  await page.viewport(1280, 860);
  await page.navigate(`${server.base}/decompile2/`);
  await page.waitFor(`document.getElementById('pick').getAttribute('aria-disabled') === null`, { what: '#pick enabled', timeout: 60000 });
  await page.evaluate(`localStorage.clear(); true`);
  assert.ok(await page.call(() => {
    const zone = document.getElementById('dropzone');
    return zone.offsetParent !== null && getComputedStyle(document.getElementById('sidebar')).display === 'none';
  }), 'the welcome screen shows the drop zone, and no function list yet');
  await noExceptions('page ready (welcome screen with a drop zone)');

  await page.evaluate(LOAD_EXAMPLE);
  await page.waitFor(`document.querySelectorAll('#fnlist .fn').length > 5`, { what: 'inventory', timeout: 60000 });
  await page.waitFor(`/sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'main opened without a click', timeout: 60000 });
  assert.equal(await text('#vname'), 'main', 'main is the open function without a click');
  assert.equal(await count('#fnlist .fn.sel[data-addr="0x1198"]'), 1);
  assert.ok(await count('#ccode .t[data-kind=variable]') > 0, 'variables are tokens');
  await noExceptions('load the example; main opens by itself');

  assert.equal(await page.call(() => document.documentElement.dataset.theme), 'dark', 'the page opens in the dark theme');
  await page.click('#themebtn');
  assert.equal(await page.call(() => document.documentElement.dataset.theme), 'light', 'the toggle switches to light');
  await page.click('#themebtn');
  assert.equal(await page.call(() => document.documentElement.dataset.theme), 'dark', 'and back to dark');
  await noExceptions('the theme toggle sets data-theme');

  await page.hover('#c-L5 .ct');
  await sleep(700);
  assert.equal(await page.evaluate(`!document.getElementById('d2card').hidden`), true, 'the hover card shows');
  assert.match(await text('#d2card'), /^Line 5/);
  const card = await text('#d2card');
  await page.key('Escape');
  await noExceptions('hover a line');

  await page.click('#ccode');
  await page.key(' ', { code: 'Space' });
  await sleep(300);
  const engineHasInspect = await count('#asmcode .d2-ar') > 0;
  if (engineHasInspect) {
    assert.match(await text('#asmcode .d2-as[data-line="5"]'), /^5v1 = sum_to\(add\(argc,3\)\);$/, 'each C line heads the instructions it became');
    assert.equal(await count('#asmcode .d2-as[data-role="prologue"]'), 1, 'the prologue has one "Function setup" heading');
    assert.deepEqual(await page.call(() => {
      const row = document.getElementById('a-0x11a4');
      return [row.querySelector('.am').textContent, row.querySelector('.ao').textContent, row.title];
    }), ['mov', 'dword ptr [rbp - 0x14], edi', 'MOV dword ptr [RBP + -0x14],EDI'], 'easy spelling on screen, the exact text in the title');
    assert.match(card, /Line 5 → 2 instructions \+6 that set it up/, 'the card counts the inferred set-up');
    assert.ok(await count('#asmcode .d2-ar[data-inferred="1"][data-band="5"]') === 6, 'the argument set-up rows share line 5\'s band, dashed');
    assert.ok(await count('#asmcode .d2-ar[data-role="prologue"]') === 6);
    await noExceptions('Space to Assembly (inferred attribution)');
  } else {
    assert.match(await text('#asmcode'), /inspect/, 'the Assembly tab explains what it waits for');
    skipped.push('Assembly rows (the built wasm has no inspect)');
  }
  await page.key(' ', { code: 'Space' });
  await sleep(150);

  await page.click('#c-L5 .t[data-sym="v1"]');
  await page.key('n');
  await page.waitFor(`!document.getElementById('d2pop').hidden`, { what: 'rename popover' });
  await page.type('total');
  await page.key('Enter');
  await page.waitFor(`document.getElementById('sesslist')?.children.length === 1`, { what: 'edit recorded', timeout: 60000 });
  await page.waitFor(`document.getElementById('cancelbtn').disabled`, { what: 'edit finished', timeout: 60000 });
  const mark = await page.evaluate(`document.querySelector('#sesslist .mk')?.title`);
  if (engineHasInspect && mark === 'applied') {
    assert.match(await text('#ccode'), /\btotal\b/, 'the renamed local is in the C');
    await noExceptions('rename v1 to total (applied)');

    await page.click('#c-L5 .t[data-sym="total"]');
    await page.key('ArrowDown');
    assert.equal(await count('#c-L6.hl-sel'), 1, 'ArrowDown from a selected variable steps to the next line');
    assert.equal(await count('#c-L1.hl-sel'), 0);
    await noExceptions('ArrowDown steps from the selected variable');

    await page.click('#c-L6 .ct');
    await page.key(';');
    await page.waitFor(`!document.getElementById('d2pop').hidden`, { what: 'comment popover' });
    await page.type('calls printf');
    await page.key('Enter');
    await page.waitFor(`/calls printf/.test(document.getElementById('ccode').textContent)`, { what: 'comment rendered', timeout: 60000 });
    await idle('comment applied');
    await page.click('#sesslist li:last-child [data-act=edit-edit]');
    await page.waitFor(`!document.getElementById('d2pop').hidden`, { what: 'rail edit popover' });
    await page.type('prints the sum');
    await page.key('Enter');
    await page.waitFor(`/prints the sum/.test(document.getElementById('ccode').textContent)`, { what: 'edited comment rendered', timeout: 60000 });
    await idle('rail edit applied');
    const rows = await page.evaluate(`[...document.querySelectorAll('#sesslist li')].map((li) => li.querySelector('.mk').title + ' ' + li.querySelector('.tx').textContent)`);
    assert.deepEqual(rows.filter((r) => /Note at/.test(r)), ['applied Note at 11e1: prints the sum'], 'editing a note from the panel keeps one typed, applied record');
    await noExceptions('edit a comment from the rail');

    await page.key('x');
    await page.waitFor(`/Called by/.test(document.getElementById('railrefsbody')?.textContent || '')`, { what: 'references', timeout: 60000 });
    assert.match(await text('#railrefsbody'), /Calls add, sum_to and printf/, 'what main calls');
    assert.match(await text('#railrefsbody'), /Called by _start \(uses its address\)/, 'main is referenced from _start');
    await noExceptions('references from the engine');

    await page.key('s');
    await sleep(200);
    assert.equal(await count('#panes.split #asmcode.no-bytes'), 1, 'split view hides the bytes column by default');
    assert.ok(await count('#asmcode .ao[title]') > 0, 'operands keep their full text as a title');
    await page.key('b');
    assert.equal(await count('#asmcode.no-bytes'), 0, 'b shows it in split view');
    await page.key('b');
    await page.key('s');
    await sleep(150);
    assert.equal(await count('#asmcode.no-bytes'), 1, 'the single view keeps its own setting (off by default)');
    await noExceptions('side by side bytes column');
  } else {
    assert.match(await text('#sesslist'), /name main::v1 total/, 'the edit is kept in the session');
    assert.equal(mark, engineHasInspect ? 'applied' : 'not yet sent', 'the rail is honest about the outcome');
    skipped.push('rename applied by the engine (the built wasm has no --assert)');
  }

  await page.key('3');
  await sleep(200);
  if (await count('#hexdump .hb[data-a]') > 0) {
    await page.click('#hexdump .hb[data-a="0x11e1"]');
    await page.key('9');
    await page.key('0');
    await page.waitFor(`document.querySelectorAll('#hexdump .hb.pa').length > 0`, { what: 'patched byte' });
    await page.waitFor(`!document.getElementById('patchbtn').disabled`, { what: '#patchbtn enabled' });
    await page.key('Escape');
    await page.waitFor(`document.getElementById('cancelbtn').disabled`, { what: 'patch sent', timeout: 60000 });
    await noExceptions('type 90 in the Bytes tab');
  } else {
    assert.match(await text('#hexdump'), /inspect/);
    skipped.push('byte patching (the built wasm has no inspect)');
  }

  for (const width of [1024, 820]) {
    await page.viewport(width, 860);
    await sleep(250);
    const overflow = await page.evaluate(`document.documentElement.scrollWidth - document.documentElement.clientWidth`);
    assert.ok(overflow <= 0, `no horizontal page overflow at ${width}px (got ${overflow})`);
  }
  await page.viewport(1280, 860);
  await noExceptions('layout at 1024 and 820 px');

  await page.navigate(`${server.base}/decompile2/`);
  await page.waitFor(`document.getElementById('pick').getAttribute('aria-disabled') === null`, { what: 'reload ready', timeout: 60000 });
  await page.call(() => {
    window.restoredToasts = 0;
    new MutationObserver((changes) => {
      for (const c of changes) for (const n of c.addedNodes) if (/Restored/.test(n.textContent)) window.restoredToasts++;
    }).observe(document.getElementById('d2toasts'), { childList: true });
    return true;
  });
  await page.evaluate(LOAD_EXAMPLE);
  await page.waitFor(`document.querySelector('.d2banner')`, { what: 'restored-session banner', timeout: 60000 });
  assert.match(await text('.d2banner'), /Restored \d+ changes? from last time/);
  await noExceptions('reload restores the session');

  if (engineHasInspect) {
    await page.waitFor(`document.querySelector('#fnlist .fn.sel') && document.getElementById('cancelbtn').disabled`, { what: 'restored main', timeout: 60000 });
    await page.key('1');
    await page.waitFor(`/add\\(argc,3\\)/.test(document.getElementById('ccode').textContent)`, { what: 'restored main in C', timeout: 60000 });
    await setSelect('lang', 'rust');
    await page.waitFor(`/unsafe fn main/.test(document.getElementById('ccode').textContent)`, { what: 'Rust view', timeout: 60000 });
    await idle('Rust view idle');
    assert.equal(await page.call(() => window.restoredToasts), 1, 'a re-index does not toast the restore again');
    assert.equal(await count('.d2banner'), 0, 'nor show its banner');
    assert.match(await text('#railvars'), /argc\s*i32/, 'the rail reads the Rust signature');
    await page.click('#ccode .t[data-sym="argc"]');
    await page.key('y');
    await sleep(200);
    assert.ok((await toasts()).some((t) => /Retyping needs the C view/.test(t)), 'retyping in the Rust view says it needs C');
    assert.equal(await page.evaluate(`document.getElementById('d2pop').hidden`), true, 'and opens no dialog');
    await noExceptions('Rust view: no re-toast, C-only edits refused with a hint');
  }

  await page.navigate(`${server.base}/decompile2/`);
  await page.waitFor(`document.getElementById('pick').getAttribute('aria-disabled') === null`, { what: 'reload ready', timeout: 60000 });
  await page.call((key, stored) => {
    localStorage.clear();
    localStorage.setItem('kuna.d2.prefs', JSON.stringify({ v: 1, tab: 'c' }));
    localStorage.setItem(key, stored);
    return true;
  }, 'kuna.d2.session.' + sampleHash, JSON.stringify({ v: 1, rawSeq: 1,
    records: [['raw:1', { kind: 'raw', text: 'bytes 0x10 zz' }]], bytes: [] }));
  await page.evaluate(LOAD_EXAMPLE);
  await page.waitFor(`document.querySelectorAll('#fnlist .fn').length > 5`, { what: 'inventory despite a bad stored directive', timeout: 60000 });
  if (engineHasInspect) {
    await page.waitFor(`/sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'main despite a bad stored directive', timeout: 60000 });
    assert.match(await page.evaluate(`document.querySelector('#sesslist .mk')?.title || ''`), /could not read it/, 'the rail marks the refused directive');
    await page.click('#sesslist [data-act=edit-remove]');
    await page.waitFor(`document.querySelectorAll('#sesslist li').length === 0`, { what: 'refused directive removed', timeout: 60000 });
    await noExceptions('a stored directive the engine refuses does not lock the binary out');
  }

  await page.navigate(`${server.base}/decompile/`);
  await page.waitFor(`document.getElementById('pick').getAttribute('aria-disabled') === null`, { what: '/decompile ready', timeout: 60000 });
  await page.evaluate(LOAD_EXAMPLE.replace('./examples/sample.elf', '../decompile2/examples/sample.elf'));
  const clickMain = `(() => { const row = [...document.querySelectorAll('#fnlist .fn')].find((r) => r.querySelector('.nm').textContent === 'main'); row?.click(); return !!row; })()`;
  await page.waitFor(clickMain, { what: '/decompile lists main', timeout: 60000 });
  await page.waitFor(`/sum_to\\(add\\(/.test(document.getElementById('code').textContent)`, { what: '/decompile renders main', timeout: 60000 });
  await page.evaluate(`(() => { const s = document.getElementById('lang'); s.value = 'rust'; s.dispatchEvent(new Event('change')); return true; })()`);
  await page.waitFor(clickMain, { what: '/decompile re-lists main', timeout: 60000 });
  await page.waitFor(`/fn main|let mut|unsafe/.test(document.getElementById('code').textContent)`, { what: '/decompile Language: Rust', timeout: 60000 });
  await noExceptions('/decompile still works and its Language control changes the output');

  for (const path of ['/', '/decompile/', '/dev-viz/']) {
    const html = await (await fetch(`${server.base}${path}`)).text();
    assert.ok(!/decompile2/.test(html), `${path} does not link to the unlisted study view`);
  }
  done.push('no other page links to /decompile2');

  console.log(`DECOMPILE2 BROWSER OK — ${done.join('; ')}` + (skipped.length ? `; SKIPPED: ${skipped.join('; ')}` : ''));
} finally {
  clearTimeout(guard);
  page?.close();
  chrome.close();
  await server.close();
}
