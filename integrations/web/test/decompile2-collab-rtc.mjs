// decompile2-collab-rtc.mjs — a live session between two separate Chrome
// processes (two profiles: two people on two machines), over real WebRTC
// with no server: this script carries the invite link to Ben's browser and
// his reply link back to Ana's, the way the two people would, and checks that
// Ben receives the program and that a rename on one page shows on the other.
// The browsers run with raw host candidates (--disable-features=
// WebRtcHideLocalIpsWithMdns): Chrome's default hides them behind mDNS names,
// which need multicast, and CI runners do not reliably have it. First a probe
// connects two plain RTCPeerConnections inside one page: when that gathers no
// candidate or cannot connect, the machine cannot do WebRTC and the test prints
// SKIPPED and exits 0. After that, any failure, a page that could not connect
// included, exits 1. `--late N` applies the reply N seconds after Ben made it
// (a measurement to run by hand, not in CI).
//   integrations/web/build.sh && node integrations/web/test/decompile2-collab-rtc.mjs [--late 60]
import assert from 'node:assert/strict';
import { findChrome, launchChrome, openPage } from './cdp-client.mjs';
import { requireDist, serveStatic } from './worker-harness.mjs';

const chromePath = findChrome();
if (!chromePath || typeof WebSocket !== 'function') {
  console.log(`DECOMPILE2 COLLAB RTC SKIPPED — ${chromePath ? 'this Node has no global WebSocket (need 22+)' : 'no Chrome found (set CHROME=...)'}`);
  process.exit(0);
}
requireDist();

const lateAt = process.argv.indexOf('--late');
const late = lateAt > 0 ? Number(process.argv[lateAt + 1]) : 0;
const sleep = (ms) => new Promise((done) => setTimeout(done, ms));
const server = await serveStatic();
const flags = ['--disable-features=WebRtcHideLocalIpsWithMdns'];
const chromes = [await launchChrome(chromePath, { flags }), await launchChrome(chromePath, { flags })];
const guard = setTimeout(() => {
  console.error('DECOMPILE2 COLLAB RTC FAIL — timed out');
  for (const c of chromes) c.close();
  process.exit(1);
}, 300000 + late * 1000);

const ready = (p) => p.waitFor(`document.getElementById('pick').getAttribute('aria-disabled') === null`, { timeout: 60000 });
const idle = (p) => p.waitFor(`document.getElementById('cancelbtn').disabled`, { timeout: 60000 });
const started = Date.now();

/** Can this machine connect two peer connections at all (in one page, host candidates only)? */
const PROBE = async () => {
  const a = new RTCPeerConnection();
  const b = new RTCPeerConnection();
  const gather = (pc) => new Promise((done) => {
    if (pc.iceGatheringState === 'complete') done();
    pc.addEventListener('icegatheringstatechange', () => pc.iceGatheringState === 'complete' && done());
    setTimeout(done, 3000);
  });
  try {
    const ch = a.createDataChannel('probe');
    await a.setLocalDescription(await a.createOffer());
    await gather(a);
    if (!/a=candidate:/.test(a.localDescription.sdp)) return 'no ICE candidate was gathered';
    await b.setRemoteDescription(a.localDescription);
    await b.setLocalDescription(await b.createAnswer());
    await gather(b);
    await a.setRemoteDescription(b.localDescription);
    const open = await new Promise((done) => {
      ch.onopen = () => done(true);
      setTimeout(() => done(false), 10000);
    });
    return open ? null : `two peer connections in one page did not connect (ICE ${a.iceConnectionState})`;
  } finally {
    a.close();
    b.close();
  }
};
const step = (what) => { if (process.env.COLLAB_VERBOSE) console.log(`${((Date.now() - started) / 1000).toFixed(1)} s  ${what}`); };
let skipped = null;

try {
  const [ana, ben] = await Promise.all(chromes.map((c) => openPage(c.port)));
  for (const p of [ana, ben]) await p.viewport(1280, 860);
  await ana.navigate(`${server.base}/decompile2/`);
  await ready(ana);
  skipped = await ana.call(PROBE);
  if (skipped) throw Object.assign(new Error(skipped), { skip: true });
  await ana.click('#examplebtn');
  await ana.waitFor(`/sum_to/.test(document.getElementById('ccode').textContent)`, { timeout: 60000 });
  await idle(ana);
  await ana.click('#morebtn');
  await ana.click('#collabbtn');
  await ana.waitFor(`document.querySelector('#d2collab input[name=name]')`, { timeout: 20000 });
  await ana.type('Ana');
  await ana.key('Enter');
  await ana.waitFor(`document.querySelector('#d2collab [data-copytext]')?.value.includes('#join=')`, { timeout: 20000 });
  const invite = await ana.evaluate(`document.querySelector('#d2collab [data-copytext]').value`);
  step('Ana has an invite link');

  await ben.navigate(invite);
  await ben.waitFor(`document.querySelector('#d2collab input[name=name]')`, { timeout: 60000 });
  await ben.call(() => { document.querySelector('#d2collab input[name=name]').select(); return true; });
  await ben.type('Ben');
  await ben.key('Enter');
  await ben.waitFor(`document.querySelector('#d2collab [data-copytext]')?.value.includes('#reply=')`, { what: 'the reply link (no tab of Ben\'s browser made the invite)', timeout: 20000 });
  const reply = await ben.evaluate(`document.querySelector('#d2collab [data-copytext]').value`);
  step('Ben has a reply link');
  const t0 = Date.now();
  if (late) {
    console.log(`waiting ${late} s before Ana opens the reply…`);
    await sleep(late * 1000);
  }
  await ana.call((r) => {
    const input = document.querySelector('#d2collab input[name=reply]');
    input.value = r;
    input.form.requestSubmit();
    return true;
  }, reply);
  const applied = Date.now();
  step('Ana applied the reply');
  await ben.waitFor(`document.getElementById('crumbname')?.textContent === 'sample.elf'`, { what: 'Ben receives the program', timeout: 30000 });
  {
    const openMs = Date.now() - applied;
    await ben.waitFor(`/sum_to/.test(document.getElementById('ccode').textContent)`, { timeout: 60000 });
    await idle(ben);
    await ana.evaluate(`document.getElementById('d2collab').close(); true`);
    await ana.click('#c-L5 .t[data-sym="v1"]');
    await ana.key('n');
    await ana.waitFor(`!document.getElementById('d2pop').hidden`);
    await ana.type('total');
    await ana.key('Enter');
    await ben.waitFor(`/\\btotal = sum_to/.test(document.getElementById('ccode').textContent)`, { what: 'Ana\'s rename on Ben\'s page', timeout: 30000 });
    await idle(ben);
    await ben.click('#fnlist .fn[data-addr="0x1161"]');
    await ben.waitFor(`document.getElementById('vname').textContent === 'sum_to'`, { timeout: 60000 });
    await idle(ben);
    await ben.click('#fnrenamebtn');
    await ben.waitFor(`!document.getElementById('d2pop').hidden`);
    await ben.call(() => { const i = document.querySelector('#d2pop input'); i.value = 'summation'; i.dispatchEvent(new Event('input', { bubbles: true })); return true; });
    await ben.key('Enter');
    await ana.waitFor(`/summation\\(add/.test(document.getElementById('ccode').textContent)`, { what: 'Ben\'s rename on Ana\'s page', timeout: 30000 });
    for (const p of [ana, ben]) assert.deepEqual(p.exceptions, [], 'no page exception');
    const lateText = late ? `, reply applied ${Math.round((applied - t0) / 1000)} s after it was made` : '';
    console.log(`DECOMPILE2 COLLAB RTC OK — two Chrome processes over WebRTC, links carried by hand${lateText}: Ben received the program ${openMs} ms after the reply was applied; renames went both ways`);
  }
} catch (e) {
  if (!e.skip) throw e;
} finally {
  clearTimeout(guard);
  for (const c of chromes) c.close();
  await server.close();
}
if (skipped) console.log(`DECOMPILE2 COLLAB RTC SKIPPED — ${skipped}`);
