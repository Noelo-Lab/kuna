// site-theme.mjs — the landing page's light and dark themes in headless Chrome,
// served straight from the source tree (no build needed): every <picture> loads
// its -dark.svg under prefers-color-scheme: dark and its light file otherwise,
// a theme change swaps them without a reload, every image is served and decodes,
// the page never scrolls sideways from 320 to 1920 px in either theme, and
// /dev-viz/ stays light when the system is dark (it does not opt in).
//
// Skips (exit 0) when there is no Chrome or no global WebSocket (Node < 22),
// except under CI (the CI variable is set), where either is a failure.
//
// Usage:  node integrations/web/test/site-theme.mjs
import assert from 'node:assert/strict';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { findChrome, launchChrome, openPage } from './cdp-client.mjs';
import { serveStatic } from './worker-harness.mjs';

const chromePath = findChrome();
if (!chromePath || typeof WebSocket !== 'function') {
  const why = chromePath ? 'this Node has no global WebSocket (need 22+)' : 'no Chrome found (set CHROME=...)';
  console.log(`SITE THEME ${process.env.CI ? 'FAIL' : 'SKIPPED'} — ${why}`);
  process.exit(process.env.CI ? 1 : 0);
}

const site = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const server = await serveStatic(site);
const chrome = await launchChrome(chromePath);
const guard = setTimeout(() => { console.error('SITE THEME FAIL — timed out'); chrome.close(); process.exit(1); }, 120000);

const PAPER = { light: 'rgb(255, 255, 255)', dark: 'rgb(20, 17, 16)' };
const pictures = `[...document.querySelectorAll('picture img')].map((img) => img.currentSrc)`;
const decoded = `[...document.querySelectorAll('picture img')].every((img) => img.complete && img.naturalWidth > 0)`;

try {
  const page = await openPage(chrome.port);
  const scheme = (value) => page.send('Emulation.setEmulatedMedia', { features: [{ name: 'prefers-color-scheme', value }] });
  const paper = () => page.evaluate('getComputedStyle(document.body).backgroundColor');

  await page.viewport(1280);
  await scheme('light');
  await page.navigate(`${server.base}/`);
  await page.waitFor(`${pictures}.length >= 2 && ${pictures}.every(Boolean)`, { what: 'pictures load' });
  const light = await page.evaluate(pictures);
  assert.ok(light.every((src) => !src.endsWith('-dark.svg')), `light theme loaded a dark image: ${light}`);
  await page.waitFor(decoded, { what: 'the light images decode' });
  assert.equal(await paper(), PAPER.light);

  await scheme('dark');
  await page.waitFor(`${pictures}.every((src) => src.endsWith('-dark.svg'))`, { what: 'pictures swap to -dark.svg without a reload' });
  await page.waitFor(decoded, { what: 'the dark images decode' });
  assert.equal(await paper(), PAPER.dark);
  const dark = await page.evaluate(pictures);

  for (const src of new Set([...light, ...dark])) {
    const status = await page.evaluate(`fetch(${JSON.stringify(src)}).then((r) => r.status)`);
    assert.equal(status, 200, `${src} is not served`);
  }

  for (const value of ['light', 'dark']) {
    await scheme(value);
    for (const width of [320, 390, 768, 1280, 1920]) {
      await page.viewport(width);
      const [scroll, inner] = await page.evaluate('[document.documentElement.scrollWidth, innerWidth]');
      assert.ok(scroll <= inner, `${value} at ${width}px scrolls sideways (${scroll} > ${inner})`);
    }
  }
  assert.deepEqual(page.exceptions, [], 'the landing page threw');

  await scheme('dark');
  await page.viewport(1280);
  await page.navigate(`${server.base}/dev-viz/`);
  assert.equal(await paper(), PAPER.light, '/dev-viz/ must stay light');
  assert.deepEqual(page.exceptions, [], '/dev-viz/ threw');
  console.log('SITE THEME OK — light/dark pictures, live swap, 320–1920 px, dev-viz stays light');
} finally {
  clearTimeout(guard);
  chrome.close();
  await server.close();
}
