// decompile2-browser.mjs — drive the real /decompile/ page in headless Chrome
// over the DevTools protocol: the start screen, hints off by default and on
// with ?student=true (hover cards with them), load the fixture through the
// file input, open main, hover a line, switch to Assembly, rename a variable,
// patch a byte, reload to see the session restored, the Collaborate button,
// the /decompile2/ redirect, the layout at 1024 and 820 px, and the Strings
// list taking a search for "flag" to the code that uses it, and the type
// definitions shown above a function. Any uncaught page exception fails the run.
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
import { requireDist, serveStatic, fixture, openSample } from './worker-harness.mjs';

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

const loadFixture = () => openSample(page);
const ready = (what = 'page ready') => page.waitFor(`document.getElementById('pick').getAttribute('aria-disabled') === null`, { what, timeout: 60000 });

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
  await page.navigate(`${server.base}/decompile/`);
  await ready('#pick enabled');
  await page.evaluate(`localStorage.clear(); true`);
  await page.navigate(`${server.base}/decompile/`);
  await ready();
  assert.ok(await page.call(() => {
    const zone = document.getElementById('dropzone');
    return zone.offsetParent !== null && getComputedStyle(document.getElementById('sidebar')).display === 'none';
  }), 'the welcome screen shows the drop zone, and no function list yet');
  assert.deepEqual(await page.call(() => [document.querySelector('.d2-welcomein h1').textContent, document.querySelector('.d2-welcomein .d2-lede').textContent]),
    ['Decompile a binary program', 'Open a program to generate source-like code for it running 100% in the web browser using WASM'], 'the start screen says what the page does');
  assert.equal(await count('#examplebtn, #welcomeexample, .d2-steps, .d2-formats, #tab-src'), 0, 'no example, steps, formats line or Original source');
  assert.equal(await text('#welcomeopen'), 'Open file');
  assert.deepEqual(await page.call(() => { const b = document.getElementById('collabbtn'); return [b.disabled, b.title, b.closest('.d2-top') !== null]; }),
    [true, 'Open a program first', true], 'Collaborate sits in the top bar, off until a program is open');
  assert.equal(await page.call(() => document.getElementById('moremenu').querySelector('#collabbtn, [id^=collab]')), null, 'and not in the ⋯ menu');
  assert.deepEqual(await page.call(() => [document.documentElement.dataset.hints, document.getElementById('hintsbox').checked]), ['off', false], 'hints start off');
  await noExceptions('page ready: the start screen, Collaborate off, hints off');

  await loadFixture();
  await page.waitFor(`document.querySelectorAll('#fnlist .fn').length > 5`, { what: 'inventory', timeout: 60000 });
  await page.waitFor(`/sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'main opened without a click', timeout: 60000 });
  await idle('main open');
  assert.equal(await text('#vname'), 'main', 'main is the open function without a click');
  assert.equal((await text('.d2-fntitle')).trim(), 'main', 'the header shows only the function name');
  assert.equal(await count('#vmeta'), 0);
  assert.deepEqual(await page.evaluate(`[...document.querySelectorAll('#tabs [role=tab]')].map((t) => t.textContent)`),
    ['Code', 'Assembly', 'Side by side', 'Bytes', 'Stack'], 'the views, in order');
  assert.equal(await count('#fnlist .fn.sel[data-addr="0x1198"]'), 1);
  assert.ok(await count('#ccode .t[data-kind=variable]') > 0, 'variables are tokens');
  await page.hover('#c-L5 .ct');
  await sleep(900);
  assert.equal(await page.evaluate(`document.getElementById('d2card').hidden`), true, 'with hints off, hovering a line shows no card');
  await page.hover('#c-L5 .t[data-sym="v1"]');
  await sleep(900);
  assert.equal(await page.evaluate(`document.getElementById('d2card').hidden`), true, 'nor a name');
  assert.equal(await count('#c-L5.hl-hover'), 1, 'but the line under the pointer is still highlighted, as its assembly is');
  await page.key('2');
  await page.waitFor(`document.getElementById('a-0x11a4')`, { what: 'the assembly', timeout: 30000 });
  await page.hover('#a-0x11a4');
  await sleep(900);
  assert.equal(await page.evaluate(`document.getElementById('d2card').hidden`), true, 'nor an instruction');
  await page.key('1');
  assert.equal(await page.call(() => { const t = document.getElementById('tip'); return t.hidden || t.getClientRects().length === 0; }), true, 'and no tip strip');
  const collab = await page.call(() => {
    const b = document.getElementById('collabbtn');
    const said = document.getElementById(b.getAttribute('aria-describedby'))?.textContent;
    return [b.disabled, b.title, said];
  });
  assert.equal(collab[0], false, 'Collaborate is on once a program is open');
  assert.match(collab[1], /^Work on this program with others in real time: share an invite link/, 'its tooltip says what it is');
  assert.equal(collab[2], collab[1], 'and so does its accessible description');
  await page.click('#collabbtn');
  await page.waitFor(`document.getElementById('d2collab')?.open`, { what: 'the collaboration dialog', timeout: 10000 });
  await page.key('Escape');
  await page.waitFor(`!document.getElementById('d2collab')?.open`, { what: 'dialog closed', timeout: 10000 });
  await noExceptions('open a program: the name alone in the header, the views in order, no hover card, Collaborate opens the dialog');

  await page.navigate(`${server.base}/decompile/?student=true`);
  await ready('?student=true ready');
  assert.deepEqual(await page.call(() => [document.documentElement.dataset.hints, document.getElementById('hintsbox').checked]), ['on', true], '?student=true turns hints on');
  await loadFixture();
  await page.waitFor(`/sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'main again', timeout: 60000 });
  await idle('main open with hints');
  await noExceptions('?student=true turns hints on');

  assert.equal(await page.call(() => document.documentElement.dataset.theme), 'dark', 'the page opens in the dark theme');
  await page.click('#themebtn');
  assert.equal(await page.call(() => document.documentElement.dataset.theme), 'light', 'the toggle switches to light');
  await page.click('#themebtn');
  assert.equal(await page.call(() => document.documentElement.dataset.theme), 'dark', 'and back to dark');
  await noExceptions('the theme toggle sets data-theme');

  await page.hover('#c-L5 .ct');
  await sleep(700);
  assert.equal(await page.evaluate(`!document.getElementById('d2card').hidden`), true, 'with hints on, the hover card shows');
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

  const shown = (sel) => page.call((s) => { const el = document.querySelector(s); return !!el && el.getClientRects().length > 0; }, sel);
  assert.ok(await page.call(() => document.getElementById('hintsbox').checked), '"Show hints" is on with ?student=true');
  assert.ok(await shown('#hint'), 'the status bar hint shows');
  await page.click('#hintsbox');
  assert.equal(await page.call(() => document.documentElement.dataset.hints), 'off', 'unticking turns hints off');
  assert.ok(!(await shown('#hint')), 'the status bar hint is hidden');
  assert.deepEqual(await page.call(() => { const p = JSON.parse(localStorage.getItem('kuna.d2.prefs')); return [p.hints, p.hintsSet]; }), [false, true], 'and the choice is kept');
  await page.key('4');
  await sleep(200);
  if (await count('#stackframe .d2frame') > 0) {
    assert.ok(!(await shown('.d2-stacklede')), 'the stack view drops its explanation');
    assert.ok(!(await shown('#stackframe tr.ret .sa')), 'and the slot notes that teach');
    assert.ok(await shown('#stackframe tr.ret .sz'), 'but keeps the facts');
  }
  await page.click('#hintsbox');
  assert.equal(await page.call(() => document.documentElement.dataset.hints), 'on', 'ticking turns them back on');
  assert.ok(await shown('#hint'), 'the status bar hint is back');
  if (await count('#stackframe .d2frame') > 0) assert.ok(await shown('.d2-stacklede'), 'so is the stack explanation');
  await page.key('3');
  await sleep(150);
  await noExceptions('"Show hints" hides and restores the teaching notes');

  for (const width of [1024, 820]) {
    await page.viewport(width, 860);
    await sleep(250);
    const overflow = await page.evaluate(`document.documentElement.scrollWidth - document.documentElement.clientWidth`);
    assert.ok(overflow <= 0, `no horizontal page overflow at ${width}px (got ${overflow})`);
  }
  await page.viewport(1280, 860);
  await noExceptions('layout at 1024 and 820 px');

  await page.navigate(`${server.base}/decompile/?student=true`);
  await ready('reload ready');
  await page.call(() => {
    window.restoredToasts = 0;
    new MutationObserver((changes) => {
      for (const c of changes) for (const n of c.addedNodes) if (/Restored/.test(n.textContent)) window.restoredToasts++;
    }).observe(document.getElementById('d2toasts'), { childList: true });
    return true;
  });
  await loadFixture();
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

  await page.navigate(`${server.base}/decompile/`);
  await ready('reload ready');
  await page.call((key, stored) => {
    localStorage.clear();
    localStorage.setItem('kuna.d2.prefs', JSON.stringify({ v: 1, tab: 'c' }));
    localStorage.setItem(key, stored);
    return true;
  }, 'kuna.d2.session.' + sampleHash, JSON.stringify({ v: 1, rawSeq: 1,
    records: [['raw:1', { kind: 'raw', text: 'bytes 0x10 zz' }]], bytes: [] }));
  await loadFixture();
  await page.waitFor(`document.querySelectorAll('#fnlist .fn').length > 5`, { what: 'inventory despite a bad stored directive', timeout: 60000 });
  if (engineHasInspect) {
    await page.waitFor(`/sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'main despite a bad stored directive', timeout: 60000 });
    assert.match(await page.evaluate(`document.querySelector('#sesslist .mk')?.title || ''`), /could not read it/, 'the rail marks the refused directive');
    await page.click('#sesslist [data-act=edit-remove]');
    await page.waitFor(`document.querySelectorAll('#sesslist li').length === 0`, { what: 'refused directive removed', timeout: 60000 });
    await noExceptions('a stored directive the engine refuses does not lock the binary out');
  }

  await page.navigate(`${server.base}/decompile/`);
  await ready('no switch, no choice');
  assert.equal(await page.call(() => document.documentElement.dataset.hints), 'off', 'a version-1 record (hints on by default then) does not turn hints on');
  await page.click('#hintsbox');
  assert.equal(await page.call(() => document.documentElement.dataset.hints), 'on', 'ticking turns them on');
  await page.navigate(`${server.base}/decompile/`);
  await ready('reload after ticking');
  assert.deepEqual(await page.call(() => [document.documentElement.dataset.hints, document.getElementById('hintsbox').checked]), ['on', true], 'the choice is remembered without the parameter');
  await page.click('#hintsbox');
  await page.navigate(`${server.base}/decompile/?student=true`);
  await ready('the switch over a choice');
  assert.equal(await page.call(() => document.documentElement.dataset.hints), 'on', '?student=true wins over having turned them off');
  await page.navigate(`${server.base}/decompile/`);
  await ready('reload after unticking');
  assert.equal(await page.call(() => document.documentElement.dataset.hints), 'off', 'and turning them off is remembered too');
  await noExceptions('an explicit toggle is remembered across reloads; ?student=true wins for its load');

  await page.navigate(`${server.base}/decompile2/?student=true#join=kept-as-is`);
  await page.waitFor(`location.pathname === '/decompile/'`, { what: 'the redirect', timeout: 20000 });
  assert.deepEqual(await page.call(() => [location.pathname, location.search, location.hash]), ['/decompile/', '?student=true', '#join=kept-as-is'],
    '/decompile2/ redirects to /decompile/ with the query and the invite fragment intact');
  const moved = await (await fetch(`${server.base}/decompile2/`)).text();
  assert.match(moved, /http-equiv="refresh" content="0; url=\.\.\/decompile\/"/, 'with a meta refresh for pages without scripts');
  await noExceptions('/decompile2/ redirects, keeping #join=');

  for (const path of ['/', '/dev-viz/']) {
    const html = await (await fetch(`${server.base}${path}`)).text();
    assert.ok(/href="\.\.?\/decompile\/"/.test(html), `${path} links to /decompile/`);
    assert.ok(!/decompile2\//.test(html), `${path} does not link to /decompile2/`);
  }
  assert.ok(!/decompile2\//.test(await (await fetch(`${server.base}/decompile/`)).text()), '/decompile/ does not link to /decompile2/ either');
  done.push('the nav links to /decompile/, and nothing to /decompile2/');

  await page.navigate(`${server.base}/decompile/`);
  await ready('before the crackme');
  await openSample(page, 'crackme.elf');
  await page.waitFor(`/check/.test(document.getElementById('ccode').textContent)`, { what: 'crackme main', timeout: 60000 });
  await idle('crackme open');
  assert.deepEqual(await page.call(() => [document.getElementById('fnpanel').hidden, document.getElementById('strpanel').hidden]), [false, true],
    'the sidebar starts on the functions');
  await page.click('#side-strs');
  await page.waitFor(`document.querySelectorAll('#strlist .str').length > 0`, { what: 'the strings', timeout: 60000 });
  assert.equal(await page.call(() => document.getElementById('fnpanel').hidden), true, 'Strings replaces the function list');
  await page.click('#strfilter');
  await page.type('flag');
  await page.waitFor(`document.querySelectorAll('#strlist .str').length === 4`, { what: 'the search', timeout: 10000 });
  assert.deepEqual(await page.call(() => [...document.querySelectorAll('#strlist .str .sx')].map((b) => b.textContent)),
    ['flag{str1ngs_4re_3asy}', 'Nope, that is not the flag.', 'Enter the flag: ', 'Correct! You found the flag.'], 'what matches "flag", used strings first');
  assert.match(await text('#strlist .str[data-addr="0x2004"] .su'), /^Used in check ×2 through the pointer secret$/);
  const flagLink = '#strlist .str[data-addr="0x2004"] a.xt';
  const sites = (await page.call((s) => document.querySelector(s).dataset.sites, flagLink)).split(' ');
  await page.click(flagLink);
  await page.waitFor(`document.getElementById('vname').textContent === 'check' && document.querySelector('#ccode .d2-cl.hl-sel') !== null`, { what: 'check, at the use', timeout: 60000 });
  assert.match(await text('#ccode .d2-cl.hl-sel'), /secret/, 'the selected line is the one reading the flag pointer');
  assert.equal(await page.call(() => document.getElementById('asmcode').getAttribute('aria-activedescendant')), `a-${sites[0]}`, 'at its first use');
  await page.click(flagLink);
  await page.waitFor(`document.getElementById('asmcode').getAttribute('aria-activedescendant') === 'a-${sites[1]}'`, { what: 'the next use', timeout: 10000 });
  assert.match(await text('#ccode .d2-cl.hl-sel'), /strcmp\(input,secret\)/, 'a second click goes to its next use');
  await page.click('#strlist .str[data-addr="0x2037"] .sx');
  await page.waitFor(`document.getElementById('vname').textContent === 'main' && /Enter the flag/.test(document.querySelector('#ccode .d2-cl.hl-sel')?.textContent || '')`, { what: 'main at the prompt', timeout: 60000 });
  await page.call(() => document.querySelector('#strlist .str[data-addr="0x2004"] .sx').focus());
  await page.key(' ', { code: 'Space' });
  await page.waitFor(`document.getElementById('vname').textContent === 'check'`, { what: 'Space on a string', timeout: 60000 });
  assert.equal(await page.call(() => document.getElementById('tab-c').getAttribute('aria-selected')), 'true', 'Space opens the string, not the Assembly view');
  await page.key('/');
  assert.equal(await page.call(() => document.activeElement.id), 'strfilter', '/ searches the list that is open');
  await page.call(() => { const i = document.getElementById('strfilter'); i.value = 'ld-linux'; i.dispatchEvent(new Event('input')); return true; });
  await page.waitFor(`document.querySelectorAll('#strlist .str').length === 1`, { what: 'an unused string', timeout: 10000 });
  await page.click('#strlist .str .sx');
  await page.waitFor(`document.getElementById('tab-bytes').getAttribute('aria-selected') === 'true'`, { what: 'its bytes', timeout: 30000 });
  await page.click('#side-fns');
  assert.equal(await page.call(() => document.getElementById('fnpanel').hidden), false, 'back to the functions');
  await setSelect('mode', 'fast');
  await page.waitFor(`document.querySelectorAll('#fnlist .fn').length > 3 && document.getElementById('vname').textContent === 'check'`, { what: 'reopened in Fast', timeout: 60000 });
  await idle('before Cancel');
  await page.call(() => {
    [...document.querySelectorAll('#fnlist .fn')].find((row) => row.textContent === 'main').click();
    document.getElementById('side-strs').click();
    document.getElementById('cancelbtn').click();
    return true;
  });
  assert.match(await text('#strnone'), /^Stopped\. Find the strings$/, 'Cancel while the strings wait their turn says so');
  await page.click('#strnone [data-act=strings-load]');
  await page.waitFor(`document.querySelectorAll('#strlist .str').length > 0`, { what: 'the strings again', timeout: 60000 });
  await noExceptions('Strings: search "flag", go to each use, show an unused string\'s bytes');

  await setSelect('mode', 'auto');
  await openSample(page, 'structs.elf');
  await page.waitFor(`[...document.querySelectorAll('#fnlist .fn')].some((row) => row.textContent === 'make_item')`, { what: 'structs inventory', timeout: 60000 });
  await idle('structs open');
  await page.call(() => { [...document.querySelectorAll('#fnlist .fn')].find((row) => row.textContent === 'make_item').click(); return true; });
  await page.click('#tab-c');
  await page.waitFor(`document.getElementById('vname').textContent === 'make_item' && document.querySelector('#ccode .d2-tyhead')`, { what: 'make_item with its types', timeout: 60000 });
  assert.equal(await text('#ccode .d2-tyhead'), 'Types this function uses');
  const shaded = await page.call(() => [...document.querySelectorAll('#ccode .d2-cl.d2-ty')].map((row) => row.querySelector('.ct').textContent));
  assert.equal(shaded[0], 'typedef struct struct_0 struct_0;');
  assert.ok(shaded.includes('struct struct_0 {') && shaded.includes('    unsigned long field_0x8;'), 'struct_0 is defined above make_item');
  assert.match(await page.call((n) => document.getElementById(`c-L${n + 1}`).textContent, shaded.length), /make_item\(/, 'the function starts right after them');
  assert.equal(await count('#ccode .d2-ty .t'), 0, 'nothing in the definitions is a renamable name');
  await page.click('#c-L4 .ct');
  assert.match(await text('#railbody .x-card'), /Part of the definition of struct_0, a type this function uses\..*field_0x8 is the field 0x8 bytes from the start/s,
    'a definition line explains itself');
  await noExceptions('the struct the decompiler worked out is defined above the function');

  console.log(`DECOMPILE2 BROWSER OK — ${done.join('; ')}` + (skipped.length ? `; SKIPPED: ${skipped.join('; ')}` : ''));
} finally {
  clearTimeout(guard);
  page?.close();
  chrome.close();
  await server.close();
}
