// link.js — the connection between two pages. Across machines it is WebRTC,
// with the offer and the answer carried by the people (in the invite and
// reply links) or relayed by another member of the session; between tabs of
// one browser it is a BroadcastChannel with the same interface. The page that
// answers an offer knocks on the offer's BroadcastChannel first, and uses
// WebRTC only when no tab of this browser answers. A link has two channels:
// `edits` (ordered, reliable) and `cursor` (unordered, never resent).
// Gathering stops after 1.5 s, the ICE servers come from the page's
// STUN/TURN setting (none by default), and the answer is passive (sdp.js).
import { compactSdp, expandSdp, passiveAnswer } from './sdp.js';
import { randomId } from './wire.js';

const GATHER_MS = 1500;
const KNOCK_MS = 500;
const OPEN_MS = 20000;
const quietly = (fn) => {
  try { return fn(); } catch (_) { return undefined; }
};

/** Resolve once `pc` has its candidates, or after `ms`. */
function gathered(pc, ms = GATHER_MS) {
  return new Promise((done) => {
    if (pc.iceGatheringState === 'complete') { done(); return; }
    const timer = setTimeout(done, ms);
    pc.addEventListener('icegatheringstatechange', () => {
      if (pc.iceGatheringState === 'complete') { clearTimeout(timer); done(); }
    });
  });
}

class RtcLink {
  constructor(pc, edits) {
    this.kind = 'rtc';
    this.pc = pc;
    this.edits = null;
    this.cursor = null;
    this.onmessage = null;
    this.onclose = null;
    this.closed = false;
    this.attach(edits);
    const watch = () => {
      const s = pc.connectionState;
      if (s === 'failed' || s === 'closed') this.close();
      else if (s === 'disconnected') {
        clearTimeout(this.lost);
        this.lost = setTimeout(() => { if (pc.connectionState === 'disconnected') this.close(); }, 6000);
      }
    };
    pc.addEventListener('connectionstatechange', watch);
  }

  attach(channel) {
    channel.binaryType = 'arraybuffer';
    const which = channel.label === 'cursor' ? 'cursor' : 'edits';
    this[which] = channel;
    channel.onmessage = ({ data }) => this.onmessage?.(data, which);
    if (which === 'edits') channel.addEventListener('close', () => this.close());
  }

  send(text) {
    if (this.edits?.readyState === 'open') this.edits.send(text);
  }

  sendCursor(text) {
    if (this.cursor?.readyState === 'open') this.cursor.send(text);
  }

  sendBinary(bytes) {
    if (this.edits?.readyState === 'open') this.edits.send(bytes);
  }

  buffered() {
    return this.edits?.bufferedAmount || 0;
  }

  drain(threshold) {
    return new Promise((done) => {
      const ch = this.edits;
      if (!ch || ch.readyState !== 'open' || ch.bufferedAmount <= threshold) { done(); return; }
      ch.bufferedAmountLowThreshold = threshold;
      ch.addEventListener('bufferedamountlow', () => done(), { once: true });
      ch.addEventListener('close', () => done(), { once: true });
    });
  }

  close() {
    if (this.closed) return;
    this.closed = true;
    clearTimeout(this.lost);
    quietly(() => this.edits?.close());
    quietly(() => this.cursor?.close());
    quietly(() => this.pc.close());
    this.onclose?.();
  }
}

/** Is the page `peer` still open? Every page holds a Web Lock named after itself until it goes away. */
const lockName = (peer) => `kuna.d2.peer.${peer}`;
let heldFor = null;

/** Hold this page's own lock (once), so tabs linked to it over BroadcastChannel notice when it is gone. */
export function holdPresenceLock(me) {
  if (heldFor || !globalThis.navigator?.locks) return;
  heldFor = me;
  navigator.locks.request(lockName(me), () => new Promise(() => {})).catch(() => {});
}

class BcLink {
  constructor(channel, me, peer) {
    this.kind = 'bc';
    this.ch = channel;
    this.me = me;
    this.peer = peer;
    this.onmessage = null;
    this.onclose = null;
    this.closed = false;
    channel.onmessage = ({ data }) => {
      if (data?.k === 'knock' && typeof data.from === 'string') {
        quietly(() => channel.postMessage({ k: 'taken', to: data.from }));
        return;
      }
      if (!data || data.from !== peer || (data.to && data.to !== me)) return;
      if (data.k === 'm') this.onmessage?.(data.d, data.c === 'c' ? 'cursor' : 'edits');
      else if (data.k === 'x') this.close(false);
    };
    this.bye = () => this.close();
    addEventListener('pagehide', this.bye);
    if (globalThis.navigator?.locks) {
      this.watch = new AbortController();
      navigator.locks.request(lockName(peer), { signal: this.watch.signal }, () => this.close(false)).catch(() => {});
    }
  }

  post(c, d) {
    if (!this.closed) quietly(() => this.ch.postMessage({ k: 'm', from: this.me, to: this.peer, c, d }));
  }

  send(text) { this.post('e', text); }

  sendCursor(text) { this.post('c', text); }

  sendBinary(bytes) {
    const copy = bytes.byteOffset === 0 && bytes.byteLength === bytes.buffer.byteLength ? bytes.buffer : bytes.slice().buffer;
    this.post('e', copy);
  }

  buffered() { return 0; }

  drain() { return Promise.resolve(); }

  close(tell = true) {
    if (this.closed) return;
    if (tell) quietly(() => this.ch.postMessage({ k: 'x', from: this.me, to: this.peer }));
    this.closed = true;
    removeEventListener('pagehide', this.bye);
    quietly(() => this.watch?.abort());
    quietly(() => this.ch.close());
    this.onclose?.();
  }
}

/**
 * Start a link: an offer to hand to one other page. Resolves `{id, sdp, bc,
 * ready, answer(sdp), cancel(), onstate}`: `ready` settles with the link once
 * the other page answers (a knock from a tab of this browser, or `answer`
 * with its WebRTC reply), or fails.
 */
export async function makeOffer({ me, iceServers = [], bc = true } = {}) {
  const id = randomId(10);
  const pc = new RTCPeerConnection({ iceServers });
  const edits = pc.createDataChannel('edits', { ordered: true });
  const cursor = pc.createDataChannel('cursor', { ordered: false, maxRetransmits: 0 });
  let settle;
  let fail;
  const ready = new Promise((resolve, reject) => { settle = resolve; fail = reject; });
  ready.catch(() => {});
  let done = false;
  let channel = null;
  const offer = { id, bc, ready, onstate: null, answered: false };
  const finish = (link) => {
    if (done) return;
    done = true;
    clearTimeout(offer.timer);
    settle(link);
  };
  const stop = (why) => {
    if (done) return;
    done = true;
    clearTimeout(offer.timer);
    quietly(() => pc.close());
    quietly(() => channel?.close());
    fail(new Error(why));
  };
  edits.addEventListener('open', () => {
    quietly(() => channel?.close());
    const link = new RtcLink(pc, edits);
    link.attach(cursor);
    finish(link);
  });
  pc.addEventListener('iceconnectionstatechange', () => {
    offer.onstate?.(pc.iceConnectionState);
    if (pc.iceConnectionState === 'failed') stop('failed');
  });
  await pc.setLocalDescription(await pc.createOffer());
  await gathered(pc);
  offer.sdp = compactSdp(pc.localDescription.sdp);
  if (!offer.sdp) {
    stop('no description');
    throw new Error('this browser gave no usable connection details');
  }
  if (bc && globalThis.BroadcastChannel) {
    channel = new BroadcastChannel(`kuna.d2.link.${id}`);
    channel.onmessage = ({ data }) => {
      if (data?.k !== 'knock' || typeof data.from !== 'string') return;
      if (done || offer.answered) {
        quietly(() => channel.postMessage({ k: 'taken', to: data.from }));
        return;
      }
      offer.answered = true;
      quietly(() => channel.postMessage({ k: 'ack', from: me, to: data.from }));
      const link = new BcLink(channel, me, data.from);
      quietly(() => pc.close());
      finish(link);
    };
  }
  offer.answer = async (compact) => {
    if (done || offer.answered) throw new Error('used');
    offer.answered = true;
    await pc.setRemoteDescription({ type: 'answer', sdp: expandSdp(compact, 'answer') });
    offer.timer = setTimeout(() => stop('timeout'), OPEN_MS);
  };
  offer.cancel = () => stop('cancelled');
  return offer;
}

/**
 * Answer an offer. Resolves `{link}` when a tab of this browser made it (over
 * BroadcastChannel), else `{sdp, ready, cancel(), onstate}` with the WebRTC
 * answer to send back. Throws Error('used') when the offer was taken.
 */
export async function takeOffer({ me, id, sdp, iceServers = [], bc = true } = {}) {
  if (bc && globalThis.BroadcastChannel) {
    const channel = new BroadcastChannel(`kuna.d2.link.${id}`);
    const reply = await new Promise((done) => {
      const timer = setTimeout(() => done(null), KNOCK_MS);
      channel.onmessage = ({ data }) => {
        if (data?.to !== me) return;
        if (data.k === 'ack' && typeof data.from === 'string') { clearTimeout(timer); done(data); }
        if (data.k === 'taken') { clearTimeout(timer); done({ k: 'taken' }); }
      };
      channel.postMessage({ k: 'knock', from: me });
    });
    if (reply?.k === 'ack') return { link: new BcLink(channel, me, reply.from) };
    channel.close();
    if (reply?.k === 'taken') throw new Error('used');
  }
  const pc = new RTCPeerConnection({ iceServers });
  let settle;
  let fail;
  const ready = new Promise((resolve, reject) => { settle = resolve; fail = reject; });
  ready.catch(() => {});
  const res = { ready, onstate: null };
  let link = null;
  pc.addEventListener('datachannel', ({ channel }) => {
    if (channel.label === 'edits') {
      const open = () => {
        link = new RtcLink(pc, channel);
        if (res.pendingCursor) link.attach(res.pendingCursor);
        settle(link);
      };
      if (channel.readyState === 'open') open();
      else channel.addEventListener('open', open, { once: true });
    } else if (channel.label === 'cursor') {
      if (link) link.attach(channel);
      else res.pendingCursor = channel;
    }
  });
  pc.addEventListener('iceconnectionstatechange', () => res.onstate?.(pc.iceConnectionState));
  await pc.setRemoteDescription({ type: 'offer', sdp: expandSdp(sdp, 'offer') });
  const answer = await pc.createAnswer();
  await pc.setLocalDescription({ type: 'answer', sdp: passiveAnswer(answer.sdp) });
  await gathered(pc);
  res.sdp = compactSdp(pc.localDescription.sdp);
  res.cancel = () => {
    quietly(() => pc.close());
    fail(new Error('cancelled'));
  };
  if (!res.sdp) {
    res.cancel();
    throw new Error('this browser gave no usable connection details');
  }
  return res;
}
