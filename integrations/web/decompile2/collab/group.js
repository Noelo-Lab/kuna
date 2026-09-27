// group.js — the people in a live session and what passes between their
// pages, with no DOM. Everyone links to everyone (up to MAX_PEERS): a
// newcomer links to whoever invited them and learns the others from the
// roster each page gossips; of each pair not yet linked, the page with the
// smaller id offers, and a page linked to both relays the offer and the
// answer. On every link: a hello (another protocol or build is refused
// politely), then each side's registers, both ways; after that, edits as they
// happen. A page forwards an edit only to members its author has no direct
// link to, so a pair that could not link directly still converges. A
// newcomer without the program receives it in chunks and checks its SHA-256.
// Every message from another page passes `readMessage` (shape and size) and a
// per-page rate limit first; a page that had to drop edits asks for the
// sender's registers again. This page sends within the same limits, sized in
// UTF-8 bytes (what a data channel counts), and the others cannot grow the
// registers past MAX_REGISTERS live ones.
//
// `connect` makes links for introductions ({offer(), answer({id, sdp})});
// `page` is how the group tells the page what happened. A link is
// {send(text), sendCursor(text), sendBinary(bytes), buffered(), drain(n),
// close(), onmessage(data, channel), onclose()}.
import { PROTOCOL, MAX_PEERS, MAX_MESSAGE, COLORS, readMessage, limiter, randomId, utf8Length } from './wire.js';
import { sha256Hex } from '../../sha256.js';

export const MAX_REGISTERS = 100000;
const MAX_STORED = 2 * MAX_REGISTERS;
const CHUNK = 64 << 10;
const HIGH_WATER = 1 << 20;
const INTRO_MS = 25000;
const RETRY_MS = 6000;
const MAX_TRIES = 3;
const FLUSH_MS = 50;
const RESYNC_MS = 5000;

/** Members with distinct colours: of two that share one, the larger id takes the first free colour. */
export function resolveColors(members) {
  const used = new Set();
  const color = new Map();
  const later = [];
  for (const m of [...members].sort((a, b) => (a.peer < b.peer ? -1 : 1))) {
    if (m.color && !used.has(m.color)) {
      used.add(m.color);
      color.set(m.peer, m.color);
    } else later.push(m);
  }
  for (const m of later) {
    const c = COLORS.find((x) => !used.has(x)) || m.color || COLORS[0];
    used.add(c);
    color.set(m.peer, c);
  }
  return members.map((m) => ({ ...m, color: color.get(m.peer) }));
}

const quietly = (fn) => {
  try { return fn(); } catch (_) { return undefined; }
};

export class Group {
  constructor({ me, name, build, replica, connect, page }) {
    this.me = me;
    this.name = name;
    this.build = build;
    this.replica = replica;
    this.connect = connect;
    this.page = page;
    this.sid = null;
    this.color = null;
    this.links = new Set();
    this.peers = new Map();
    this.known = new Map();
    this.intros = new Map();
    this.outbox = new Map();
    this.flushTimer = 0;
    this.rosterTimer = 0;
    this.where = { fn: null, view: 'c' };
    this.closed = false;
  }

  /** Start a session as its first member. */
  create() {
    this.sid = randomId(12);
    this.color = COLORS[0];
  }

  get active() {
    return !!this.sid && !this.closed;
  }

  /** Everyone in the session, this page first: `{peer, name, color, me, linked, where}`. */
  members() {
    const out = [{ peer: this.me, name: this.name, color: this.color, me: true, linked: true, where: this.where }];
    for (const rec of this.peers.values()) {
      if (rec.member) out.push({ peer: rec.peer, name: rec.name, color: rec.color, me: false, linked: true, where: rec.where });
    }
    for (const [peer, k] of this.known) {
      if (!this.peers.has(peer)) out.push({ peer, name: k.name, color: k.color, me: false, linked: false, where: null });
    }
    return resolveColors(out);
  }

  size() {
    return this.members().length;
  }

  /** A member's name and colour, or null. */
  who(peer) {
    return this.members().find((m) => m.peer === peer) || null;
  }

  /** A link from an invite: `joining` on the page that opened the invite. */
  addLink(link, { joining = false, peer = null } = {}) {
    const rec = {
      link, peer, joining, member: false, hello: false, name: '', color: null, where: null, listed: new Set(),
      lim: {
        ops: limiter(20, 20), snap: limiter(200, 400), cur: limiter(30, 45), ping: limiter(1, 2), other: limiter(20, 40),
      },
      out: limiter(18, 18), rx: null, dropped: 0, gone: false, resyncAt: 0,
    };
    this.links.add(rec);
    link.onmessage = (data, channel) => this.#receive(rec, data, channel);
    link.onclose = () => this.#drop(rec);
    this.#send(rec, {
      t: 'hello', proto: PROTOCOL, build: this.build, peer: this.me, name: this.name,
      color: this.color, sid: joining ? null : this.sid, file: this.page.fileMeta(),
    });
    return rec;
  }

  /** This page's own register ops, for every page it is linked to. */
  local(ops) {
    if (ops.length) this.#queue(ops, null);
  }

  setWhere(where) {
    this.where = { fn: where.fn, view: where.view };
    for (const rec of this.peers.values()) if (rec.member) this.#sendWhere(rec);
  }

  sendCursor(cursor) {
    const text = JSON.stringify({ t: 'cur', ...cursor });
    for (const rec of this.peers.values()) if (rec.member) quietly(() => rec.link.sendCursor(text));
  }

  sendPing(ping) {
    for (const rec of this.peers.values()) if (rec.member) this.#send(rec, { t: 'ping', ...ping });
  }

  /** Say goodbye and close every link; the page's session stays as it is. */
  leave() {
    if (this.closed) return;
    this.closed = true;
    clearTimeout(this.flushTimer);
    clearTimeout(this.rosterTimer);
    for (const rec of this.links) {
      this.#send(rec, { t: 'bye' });
      rec.gone = true;
    }
    const links = [...this.links];
    setTimeout(() => { for (const rec of links) quietly(() => rec.link.close()); }, 250);
    for (const intro of this.intros.values()) quietly(() => intro.cancel?.());
    this.links.clear();
    this.peers.clear();
    this.known.clear();
    this.intros.clear();
    this.outbox.clear();
  }

  // ── receiving ────────────────────────────────────────────────────────────

  /** Send a message (or prebuilt JSON text) if it fits in one data-channel message. */
  #send(rec, msg) {
    const text = typeof msg === 'string' ? msg : JSON.stringify(msg);
    if (text.length > MAX_MESSAGE && utf8Length(text) > MAX_MESSAGE) return false;
    return quietly(() => { rec.link.send(text); return true; }) === true;
  }

  #receive(rec, data, channel) {
    if (this.closed || rec.gone) return;
    if (typeof data !== 'string') {
      this.#chunk(rec, data);
      return;
    }
    const m = readMessage(data, channel);
    if (!m || (!rec.hello && m.t !== 'hello')) {
      rec.dropped++;
      return;
    }
    if (!rec.lim[m.t in rec.lim ? m.t : 'other'].take()) {
      rec.dropped++;
      if (m.t === 'ops' || m.t === 'snap') this.#askResync(rec);
      return;
    }
    switch (m.t) {
      case 'hello': this.#hello(rec, m); break;
      case 'welcome': this.#welcome(rec, m); break;
      case 'snap': this.#ops(rec, m.ops, m.last); break;
      case 'ops': this.#ops(rec, m.ops, null); break;
      case 'resync': if (rec.member) this.#sendSnap(rec); break;
      case 'file': this.#file(rec, m); break;
      case 'roster': this.#roster(rec, m); break;
      case 'where':
        if (rec.member) {
          rec.where = { fn: m.fn, view: m.view };
          this.page.where(rec.peer);
        }
        break;
      case 'cur': if (rec.member) this.page.cursor(rec.peer, m); break;
      case 'ping': if (rec.member) this.page.ping(rec.peer, m); break;
      case 'relay': this.#relay(rec, m); break;
      case 'full':
        if (rec.joining && !rec.member) {
          rec.failed = true;
          this.page.event('full', { name: rec.name });
          this.#drop(rec);
        }
        break;
      case 'bye':
        rec.bye = true;
        this.#drop(rec);
        break;
      default: break;
    }
  }

  /** Ask a page for all its registers again (after this page had to drop some of its edits). */
  #askResync(rec) {
    const now = Date.now();
    if (!rec.member || now < rec.resyncAt) return;
    rec.resyncAt = now + RESYNC_MS;
    setTimeout(() => { if (!rec.gone && !this.closed) this.#send(rec, { t: 'resync' }); }, 1000);
  }

  #hello(rec, m) {
    if (rec.hello) return;
    rec.hello = true;
    rec.name = m.name;
    if (m.proto !== PROTOCOL || m.build !== this.build) {
      rec.failed = true;
      this.page.event('mismatch', { name: m.name });
      this.#send(rec, { t: 'bye' });
      this.#drop(rec);
      return;
    }
    if (m.peer === this.me || (rec.peer && rec.peer !== m.peer) || this.peers.has(m.peer)) {
      rec.failed = true;
      this.#drop(rec);
      return;
    }
    rec.peer = m.peer;
    rec.color = m.color;
    this.peers.set(m.peer, rec);
    if (rec.joining) return;
    if (!this.sid || (m.sid && m.sid !== this.sid)) {
      this.#drop(rec);
      return;
    }
    if (m.sid === this.sid) {
      rec.member = true;
      this.known.delete(m.peer);
      this.#sendWhere(rec);
      this.#sendSnap(rec);
      this.#rosterChanged();
      return;
    }
    const file = this.page.fileMeta();
    if (!file) {
      this.#send(rec, { t: 'bye' });
      this.peers.delete(m.peer);
      setTimeout(() => this.#drop(rec), 250);
      return;
    }
    if (this.size() >= MAX_PEERS) {
      this.#send(rec, { t: 'full' });
      this.peers.delete(m.peer);
      setTimeout(() => this.#drop(rec), 250);
      return;
    }
    const taken = new Set(this.members().map((x) => x.color));
    rec.color = COLORS.find((c) => !taken.has(c)) || COLORS[this.size() % COLORS.length];
    rec.member = true;
    const send = m.file?.hash !== file.hash;
    this.#send(rec, {
      t: 'welcome', sid: this.sid, color: rec.color, roster: this.#rosterList().filter((x) => x.peer !== rec.peer),
      file, example: this.page.isExample(), send,
    });
    this.#sendSnap(rec);
    this.#sendWhere(rec);
    if (send) this.#sendFile(rec);
    this.page.event('joined', { peer: rec.peer, name: rec.name });
    this.#rosterChanged();
  }

  #welcome(rec, m) {
    if (!rec.joining || rec.member || this.sid) return;
    rec.member = true;
    rec.sponsor = true;
    rec.program = m.file;
    this.sid = m.sid;
    this.color = m.color;
    rec.listed = new Set(m.roster.map((x) => x.peer));
    for (const x of m.roster) if (x.peer !== this.me && x.peer !== rec.peer) this.#know(x, rec.peer);
    this.page.welcomed({ from: rec.peer, name: rec.name, file: m.file, example: m.example, send: m.send });
    this.#sendSnap(rec);
    this.#sendWhere(rec);
    this.#rosterChanged();
  }

  #ops(rec, list, last) {
    if (!rec.member) return;
    const changes = [];
    const fresh = [];
    for (const op of list) {
      const before = typeof op?.k === 'string' ? this.replica.regs.get(op.k) : undefined;
      if (!before && (this.replica.regs.size >= MAX_STORED || (op?.v != null && this.replica.live >= MAX_REGISTERS))) {
        rec.dropped++;
        continue;
      }
      const prev = before ? { v: before.v, by: before.c[1] } : null;
      const r = this.replica.receive(op);
      if (r === 'invalid') {
        rec.dropped++;
        continue;
      }
      if (!r) continue;
      fresh.push(op);
      changes.push({ key: op.k, value: op.v, by: op.c[1], prev: prev ? prev.v : null, prevBy: prev ? prev.by : null });
    }
    if (fresh.length) this.#queue(fresh, rec);
    if (changes.length) this.page.changed(changes, rec.peer);
    if (last === true && rec.sponsor && !rec.caughtUp) {
      rec.caughtUp = true;
      this.page.caughtUp(rec.peer);
    }
  }

  #file(rec, m) {
    if (!rec.sponsor || rec.rx || rec.gotFile || !rec.program || m.hash !== rec.program.hash || m.size !== rec.program.size) {
      rec.dropped++;
      return;
    }
    rec.rx = { meta: { name: rec.program.name, size: m.size, hash: m.hash }, buf: new Uint8Array(m.size), got: 0 };
    this.page.fileProgress(0, m.size);
  }

  #chunk(rec, data) {
    const rx = rec.rx;
    const u8 = data instanceof ArrayBuffer ? new Uint8Array(data)
      : ArrayBuffer.isView(data) ? new Uint8Array(data.buffer, data.byteOffset, data.byteLength) : null;
    if (!rx || !u8) {
      rec.dropped++;
      return;
    }
    if (rx.got + u8.length > rx.meta.size) {
      rec.rx = null;
      this.page.fileFailed('size');
      return;
    }
    rx.buf.set(u8, rx.got);
    rx.got += u8.length;
    this.page.fileProgress(rx.got, rx.meta.size);
    if (rx.got < rx.meta.size) return;
    rec.rx = null;
    rec.gotFile = true;
    sha256Hex(rx.buf).then((hash) => {
      if (hash === rx.meta.hash) this.page.fileArrived(rx.buf, rx.meta);
      else this.page.fileFailed('hash');
    });
  }

  #roster(rec, m) {
    if (!rec.member) return;
    rec.listed = new Set(m.members.map((x) => x.peer));
    for (const x of m.members) if (x.peer !== this.me && !this.peers.has(x.peer)) this.#know(x, rec.peer);
    for (const [peer, k] of this.known) {
      if (rec.listed.has(peer)) continue;
      k.via.delete(rec.peer);
      if (!k.via.size) this.known.delete(peer);
    }
    this.page.roster();
    this.#mesh();
  }

  #know(x, via) {
    const k = this.known.get(x.peer) || { name: x.name, color: x.color, via: new Set(), tries: 0, retryAt: 0 };
    k.name = x.name;
    k.color = x.color;
    k.via.add(via);
    this.known.set(x.peer, k);
  }

  #relay(rec, m) {
    if (!rec.member) return;
    if (m.to !== this.me) {
      const to = this.peers.get(m.to);
      if (m.from === rec.peer && to && to !== rec && to.member) this.#send(to, m);
      return;
    }
    if (m.kind === 'answer') {
      const intro = this.intros.get(m.from);
      if (intro?.id === m.id && intro.answer) Promise.resolve().then(() => intro.answer(m.d)).catch(() => {});
      return;
    }
    if (m.from === this.me || this.peers.has(m.from) || this.intros.has(m.from) || m.from > this.me) return;
    if (this.size() >= MAX_PEERS && !this.known.has(m.from)) return;
    this.#pair(m.from, rec, async () => {
      const res = await this.connect.answer({ id: m.id, sdp: m.d });
      return { id: m.id, cancel: res.cancel, ready: res.ready, relay: res.sdp ? { kind: 'answer', id: m.id, d: res.sdp } : null };
    });
  }

  #mesh() {
    if (!this.active) return;
    const now = Date.now();
    for (const [peer, k] of this.known) {
      if (this.peers.has(peer) || this.intros.has(peer) || this.me > peer || k.tries >= MAX_TRIES || now < k.retryAt) continue;
      const via = [...k.via].map((p) => this.peers.get(p)).find((r) => r?.member);
      if (!via) continue;
      k.tries++;
      this.#pair(peer, via, async () => {
        const off = await this.connect.offer();
        return { id: off.id, cancel: off.cancel, ready: off.ready, answer: off.answer, relay: { kind: 'offer', id: off.id, d: off.sdp } };
      });
    }
  }

  /**
   * Link to `peer` through `via`: `start()` makes this page's half of the
   * handshake (`{id, cancel, ready, answer?, relay}`), `relay` goes to the
   * peer through `via`, and the link that `ready` brings joins the group.
   */
  async #pair(peer, via, start) {
    const intro = { id: null, answer: null, cancel: null };
    this.intros.set(peer, intro);
    const timer = setTimeout(() => this.#introFailed(peer, intro), INTRO_MS);
    const current = () => this.intros.get(peer) === intro && !this.closed;
    try {
      const half = await start();
      Object.assign(intro, { id: half.id, answer: half.answer || null, cancel: half.cancel });
      if (!current()) {
        quietly(() => half.cancel?.());
        return;
      }
      if (half.relay) this.#send(via, { t: 'relay', to: peer, from: this.me, ...half.relay });
      const link = await half.ready;
      if (!current()) {
        quietly(() => link.close());
        return;
      }
      clearTimeout(timer);
      this.intros.delete(peer);
      this.addLink(link, { peer });
    } catch (_) {
      clearTimeout(timer);
      this.#introFailed(peer, intro);
    }
  }

  #introFailed(peer, intro) {
    if (this.intros.get(peer) !== intro) return;
    this.intros.delete(peer);
    quietly(() => intro.cancel?.());
    const k = this.known.get(peer);
    if (k) k.retryAt = Date.now() + RETRY_MS;
    setTimeout(() => this.#mesh(), RETRY_MS + 50);
  }

  #drop(rec) {
    if (rec.gone) return;
    rec.gone = true;
    this.links.delete(rec);
    quietly(() => rec.link.close());
    if (rec.rx) {
      rec.rx = null;
      this.page.fileFailed('lost');
    }
    const joinFailed = rec.joining && !rec.member && !rec.failed && !this.closed;
    if (!rec.peer || this.peers.get(rec.peer) !== rec) {
      if (joinFailed) this.page.event('closed', { name: rec.name });
      return;
    }
    this.peers.delete(rec.peer);
    this.outbox.delete(rec);
    if (!rec.member) {
      if (joinFailed) this.page.event('closed', { name: rec.name });
      return;
    }
    if (this.closed) return;
    for (const [peer, k] of this.known) {
      k.via.delete(rec.peer);
      if (!k.via.size && !this.peers.has(peer)) {
        this.known.delete(peer);
        this.page.event('lost', { peer, name: k.name, reachable: false });
      }
    }
    if (!rec.bye) {
      for (const other of this.peers.values()) {
        if (other.member && other.listed.has(rec.peer)) this.#know({ peer: rec.peer, name: rec.name, color: rec.color }, other.peer);
      }
    }
    const k = this.known.get(rec.peer);
    if (k) {
      k.tries = 0;
      k.retryAt = Date.now() + RETRY_MS;
      setTimeout(() => this.#mesh(), RETRY_MS + 50);
    }
    this.page.event(rec.bye ? 'left' : 'lost', { peer: rec.peer, name: rec.name, reachable: !!k });
    this.#rosterChanged();
  }

  // ── sending ──────────────────────────────────────────────────────────────

  #rosterList() {
    const out = [{ peer: this.me, name: this.name, color: this.color }];
    for (const rec of this.peers.values()) if (rec.member) out.push({ peer: rec.peer, name: rec.name, color: rec.color });
    return out.slice(0, MAX_PEERS);
  }

  #rosterChanged() {
    this.page.roster();
    if (this.rosterTimer) return;
    this.rosterTimer = setTimeout(() => {
      this.rosterTimer = 0;
      if (this.closed) return;
      const members = this.#rosterList();
      for (const rec of this.peers.values()) if (rec.member) this.#send(rec, { t: 'roster', members });
      this.#mesh();
    }, 0);
  }

  #sendWhere(rec) {
    this.#send(rec, { t: 'where', fn: this.where.fn, view: this.where.view });
  }

  /**
   * `ops` as messages `{text, count}`: the JSON of `head` and a run of ops, each
   * op serialized once and each message at most MAX_MESSAGE bytes of UTF-8.
   */
  #messages(head, ops, last) {
    const out = [];
    let texts = [];
    let size = 0;
    const room = MAX_MESSAGE - 64;
    const close = (end) => out.push({ text: `${head}${texts.join(',')}]${last ? `,"last":${end}` : ''}}`, count: texts.length });
    for (const op of ops) {
      const text = JSON.stringify(op);
      const n = utf8Length(text) + 1;
      if (texts.length && (size + n > room || texts.length >= 4000)) {
        close(false);
        texts = [];
        size = 0;
      }
      texts.push(text);
      size += n;
    }
    if (texts.length || !out.length) close(true);
    return out;
  }

  #sendSnap(rec) {
    for (const { text } of this.#messages('{"t":"snap","ops":[', this.replica.snapshot(), true)) this.#send(rec, text);
  }

  async #sendFile(rec) {
    const bytes = this.page.fileBytes();
    const meta = this.page.fileMeta();
    if (!bytes || !meta) return;
    this.#send(rec, { t: 'file', name: meta.name, size: meta.size, hash: meta.hash });
    for (let at = 0; at < bytes.length; at += CHUNK) {
      while (!rec.gone && rec.link.buffered() > HIGH_WATER) await rec.link.drain(HIGH_WATER / 2);
      if (rec.gone || this.closed) return;
      quietly(() => rec.link.sendBinary(bytes.slice(at, at + CHUNK)));
    }
  }

  /** Queue ops for the members that need them: all, for this page's own; for forwarded ones, those not linked to the author. */
  #queue(ops, from) {
    for (const rec of this.peers.values()) {
      if (!rec.member || rec === from) continue;
      let box = null;
      for (const op of ops) {
        if (from && (op.c[1] === rec.peer || rec.listed.has(op.c[1]))) continue;
        box ||= this.outbox.get(rec) || [];
        box.push(op);
      }
      if (box) this.outbox.set(rec, box);
    }
    if (!this.flushTimer && this.outbox.size) this.flushTimer = setTimeout(() => this.#flush(), FLUSH_MS);
  }

  /** Send what is queued, at most as fast as the others' rate limit takes it. */
  #flush() {
    this.flushTimer = 0;
    for (const [rec, box] of [...this.outbox]) {
      const messages = this.#messages('{"t":"ops","ops":[', box, false);
      let sent = 0;
      let ops = 0;
      while (sent < messages.length && rec.out.take()) {
        this.#send(rec, messages[sent].text);
        ops += messages[sent++].count;
      }
      if (sent === messages.length) this.outbox.delete(rec);
      else this.outbox.set(rec, box.slice(ops));
    }
    if (this.outbox.size) this.flushTimer = setTimeout(() => this.#flush(), 100);
  }
}
