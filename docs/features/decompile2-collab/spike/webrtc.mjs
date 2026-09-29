// webrtc.mjs — can two browsers talk with no server of ours? Two separate
// headless Chrome processes (two profiles: two "users") open /decompile2/,
// build a WebRTC data channel, and exchange the offer and the answer only
// through this script, which stands in for a person pasting an invite link
// and a reply code. It measures the codes' sizes (the whole SDP, and just the
// fields a peer needs), the time to connect, the round-trip time, and the
// throughput of sending a program's bytes.
//
// The pass/fail run uses raw host candidates: Chrome's default hides them
// behind mDNS `.local` names, which need multicast, and CI runners do not
// reliably have it. If ICE cannot connect at all here, it says so and exits 0
// (a machine without a usable network is not a failure of the design).
// `--all` adds the measured-only rows: Chrome's defaults, and a public STUN
// server.
//   integrations/web/build.sh && node docs/features/decompile2-collab/spike/webrtc.mjs [--all]
import { spawn } from 'node:child_process';
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const web = join(dirname(fileURLToPath(import.meta.url)), '../../../../integrations/web');
const { findChrome, openPage } = await import(join(web, 'test/cdp-client.mjs'));
const { serveStatic } = await import(join(web, 'test/worker-harness.mjs'));
const sleep = (ms) => new Promise((done) => setTimeout(done, ms));

async function launch(flags) {
  const profile = mkdtempSync(join(tmpdir(), 'kuna-rtc-'));
  const child = spawn(findChrome(), ['--headless=new', '--no-sandbox', '--disable-gpu', '--no-first-run',
    '--remote-debugging-port=0', `--user-data-dir=${profile}`, ...flags, 'about:blank'], { stdio: 'ignore' });
  let port = null;
  for (let i = 0; i < 200 && !port; i++) {
    await sleep(50);
    try { port = Number(readFileSync(join(profile, 'DevToolsActivePort'), 'utf8').split('\n')[0]); } catch (_) { /* not yet */ }
  }
  return { page: await openPage(port), close: () => { child.kill('SIGKILL'); rmSync(profile, { recursive: true, force: true }); } };
}

const inPage = {
  offer: async function (ice) {
    const t0 = performance.now();
    const pc = new RTCPeerConnection({ iceServers: ice });
    globalThis.rtc = { pc, t0, ch: pc.createDataChannel('edits', { ordered: true }) };
    pc.createDataChannel('cursor', { ordered: false, maxRetransmits: 0 });
    await pc.setLocalDescription(await pc.createOffer());
    await new Promise((done) => {
      if (pc.iceGatheringState === 'complete') done();
      pc.addEventListener('icegatheringstatechange', () => pc.iceGatheringState === 'complete' && done());
      setTimeout(done, 2000);
    });
    return globalThis.packSdp(pc.localDescription.sdp, performance.now() - t0);
  },
  answer: async function (code, ice) {
    const pc = new RTCPeerConnection({ iceServers: ice });
    globalThis.rtc = { pc };
    pc.ondatachannel = ({ channel }) => {
      if (channel.label !== 'edits') return;
      globalThis.rtc.ch = channel;
      channel.binaryType = 'arraybuffer';
      channel.onmessage = ({ data }) => {
        if (typeof data === 'string') channel.send(data);
        else {
          const r = globalThis.rtc;
          r.first ??= performance.now();
          r.got = (r.got || 0) + data.byteLength;
          r.last = performance.now();
        }
      };
    };
    await pc.setRemoteDescription({ type: 'offer', sdp: await globalThis.unpackSdp(code) });
    await pc.setLocalDescription(await pc.createAnswer());
    await new Promise((done) => {
      if (pc.iceGatheringState === 'complete') done();
      pc.addEventListener('icegatheringstatechange', () => pc.iceGatheringState === 'complete' && done());
      setTimeout(done, 2000);
    });
    return globalThis.packSdp(pc.localDescription.sdp, 0);
  },
  connect: async function (code) {
    const { pc, ch, t0 } = globalThis.rtc;
    await pc.setRemoteDescription({ type: 'answer', sdp: await globalThis.unpackSdp(code) });
    const open = await new Promise((done) => {
      if (ch.readyState === 'open') done(true);
      ch.onopen = () => done(true);
      setTimeout(() => done(false), 10000);
    });
    if (!open) return { open: false, state: pc.iceConnectionState };
    const pair = [...(await pc.getStats()).values()].find((s) => s.type === 'candidate-pair' && s.nominated);
    const rtts = [];
    for (let i = 0; i < 50; i++) {
      const s = performance.now();
      await new Promise((done) => { ch.onmessage = () => done(); ch.send(`ping${i}`); });
      rtts.push(performance.now() - s);
    }
    rtts.sort((a, b) => a - b);
    const bytes = new Uint8Array(4 << 20);
    const ts = performance.now();
    ch.bufferedAmountLowThreshold = 1 << 20;
    for (let at = 0; at < bytes.length; at += 16384) {
      if (ch.bufferedAmount > (4 << 20)) await new Promise((done) => { ch.onbufferedamountlow = done; });
      ch.send(bytes.subarray(at, at + 16384));
    }
    return { open: true, ms: performance.now() - t0, rttMedian: rtts[25], rttP95: rtts[47], sendStart: ts, candidate: pair?.localCandidateId || null };
  },
};

const helpers = () => {
  const b64 = (u8) => btoa(String.fromCharCode(...u8)).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
  const unb64 = (s) => Uint8Array.from(atob(s.replace(/-/g, '+').replace(/_/g, '/')), (c) => c.charCodeAt(0));
  const pipe = async (u8, stream) => new Uint8Array(await new Response(new Blob([u8]).stream().pipeThrough(stream)).arrayBuffer());
  globalThis.packSdp = async (sdp, gatherMs) => {
    const code = b64(await pipe(new TextEncoder().encode(sdp), new CompressionStream('deflate-raw')));
    const lines = sdp.split(/\r?\n/);
    const field = (p) => (lines.find((l) => l.startsWith(p)) || '').slice(p.length);
    const cands = lines.filter((l) => l.startsWith('a=candidate:'));
    const minimal = JSON.stringify({ u: field('a=ice-ufrag:'), p: field('a=ice-pwd:'), f: field('a=fingerprint:sha-256 ').replace(/:/g, ''),
      c: cands.map((l) => l.split(' ').slice(4, 8).join(' ')) });
    const minimalCode = b64(await pipe(new TextEncoder().encode(minimal), new CompressionStream('deflate-raw')));
    return { code, sdpBytes: sdp.length, codeChars: code.length, minimalChars: minimalCode.length, gatherMs,
      candidates: cands.map((l) => l.split(' ').slice(4, 8).join(' ')) };
  };
  globalThis.unpackSdp = async (code) => new TextDecoder().decode(await pipe(unb64(code), new DecompressionStream('deflate-raw')));
  return true;
};

const server = await serveStatic();
const CONFIGS = [
  { name: 'no ICE servers, raw host candidates', flags: ['--disable-features=WebRtcHideLocalIpsWithMdns'], ice: [] },
  { name: 'no ICE servers, Chrome defaults (host candidates hidden behind mDNS names)', flags: [], ice: [], extra: true },
  { name: 'public STUN only (stun.l.google.com)', flags: [], ice: [{ urls: 'stun:stun.l.google.com:19302' }], extra: true },
].filter((cfg) => !cfg.extra || process.argv.includes('--all'));
if (!findChrome()) {
  console.log('WEBRTC SKIPPED — no Chrome found (set CHROME=...)');
  process.exit(0);
}
const results = [];
for (const cfg of CONFIGS) {
  const ana = await launch(cfg.flags);
  const ben = await launch(cfg.flags);
  try {
    for (const b of [ana, ben]) {
      await b.page.navigate(`${server.base}/decompile2/`);
      await b.page.call(helpers);
    }
    const offer = await ana.page.call(inPage.offer, cfg.ice);
    const answer = await ben.page.call(inPage.answer, offer.code, cfg.ice);
    const link = await ana.page.call(inPage.connect, answer.code);
    let mbps = null;
    if (link.open) {
      await ben.page.waitFor('(globalThis.rtc.got || 0) >= (4 << 20)', { timeout: 60000, what: '4 MiB received' });
      const span = await ben.page.call(() => globalThis.rtc.last - globalThis.rtc.first);
      mbps = +((4 * 8 * 1.048576) / (span / 1000)).toFixed(0);
    }
    results.push({ config: cfg.name, offer: { sdpBytes: offer.sdpBytes, codeChars: offer.codeChars, minimalChars: offer.minimalChars, gatherMs: Math.round(offer.gatherMs), candidates: offer.candidates },
      answer: { codeChars: answer.codeChars, minimalChars: answer.minimalChars, candidates: answer.candidates },
      open: link.open, connectMs: link.ms && Math.round(link.ms), rttMedianMs: link.rttMedian && +link.rttMedian.toFixed(2), rttP95Ms: link.rttP95 && +link.rttP95.toFixed(2), throughputMbitPerS: mbps, state: link.state });
  } catch (e) {
    results.push({ config: cfg.name, error: String(e.message || e) });
  } finally {
    ana.close();
    ben.close();
  }
}
await server.close();
const main = results[0];
if (process.argv.includes('--all')) console.log(JSON.stringify(results, null, 2));
if (main.error) {
  console.log(`WEBRTC FAILED — ${main.error}`);
  process.exit(1);
}
if (!main.open) {
  console.log(`WEBRTC SKIPPED — ICE could not connect two Chrome processes on this machine (ICE state: ${main.state || 'none'})`);
  process.exit(0);
}
console.log(`WEBRTC OK — two Chrome processes, no ICE servers, codes carried by hand: open in ${main.connectMs} ms, `
  + `RTT ${main.rttMedianMs} ms median / ${main.rttP95Ms} ms p95, invite ${main.offer.codeChars} chars `
  + `(${main.offer.minimalChars} with only the fields a peer needs), reply ${main.answer.codeChars}, 4 MiB at ${main.throughputMbitPerS} Mbit/s`);
