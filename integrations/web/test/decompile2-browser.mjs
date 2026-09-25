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
import { findChrome, launchChrome, openPage } from './cdp-client.mjs';
import { requireDist, serveStatic } from './worker-harness.mjs';

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

const text = (sel) => page.evaluate(`document.querySelector(${JSON.stringify(sel)})?.textContent ?? ''`);
const count = (sel) => page.evaluate(`document.querySelectorAll(${JSON.stringify(sel)}).length`);

try {
  page = await openPage(chrome.port);
  await page.viewport(1280, 860);
  await page.navigate(`${server.base}/decompile2/`);
  await page.waitFor(`document.getElementById('pick').getAttribute('aria-disabled') === null`, { what: '#pick enabled', timeout: 60000 });
  await page.evaluate(`localStorage.clear(); true`);
  await noExceptions('page ready');

  await page.evaluate(LOAD_EXAMPLE);
  await page.waitFor(`document.querySelectorAll('#fnlist .fn').length > 5`, { what: 'inventory', timeout: 60000 });
  await page.click('#fnlist .fn[data-addr="0x1198"]');
  await page.waitFor(`/sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'main decompiled', timeout: 60000 });
  assert.ok(await count('#ccode .t[data-kind=variable]') > 0, 'variables are tokens');
  await noExceptions('load the example and open main');

  await page.hover('#c-L5 .ct');
  await sleep(700);
  assert.equal(await page.evaluate(`!document.getElementById('d2card').hidden`), true, 'the hover card shows');
  assert.match(await text('#d2card'), /^L5/);
  await page.key('Escape');
  await noExceptions('hover a line');

  await page.click('#ccode');
  await page.key(' ', { code: 'Space' });
  await sleep(300);
  const engineHasInspect = await count('#asmcode .d2-ar') > 0;
  if (engineHasInspect) {
    const comments = await page.evaluate(`[...document.querySelectorAll('.d2-ar .ac')].map((e) => e.textContent).join('\\n')`);
    assert.match(comments, /L\d/, 'assembly rows name their C line');
    await noExceptions('Space to Assembly');
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
  await page.evaluate(LOAD_EXAMPLE);
  await page.waitFor(`document.querySelector('.d2banner')`, { what: 'restored-session banner', timeout: 60000 });
  assert.match(await text('.d2banner'), /Restored \d+ edit/);
  await noExceptions('reload restores the session');

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

  console.log(`DECOMPILE2 BROWSER OK — ${done.join('; ')}` + (skipped.length ? `; SKIPPED: ${skipped.join('; ')}` : ''));
} finally {
  clearTimeout(guard);
  page?.close();
  chrome.close();
  await server.close();
}
