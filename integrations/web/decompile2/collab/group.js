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
// A data channel can lose what one side sends the moment it opens (the
// other side may not be listening yet), so each page repeats its hello until
// the other page shows it got it: a hello saying `seen` (sent on receiving
// a hello from a page not yet told), or anything else. Until then nothing else
// goes out on that link (the welcome, the registers and the program wait), so
// a page never receives more than a few messages before the other's hello;
// what does arrive first is held and read once the hello comes.
// Every message from another page passes `readMessage` (shape and size) and a
// per-page rate limit first; a page that had to drop edits asks for the
// sender's registers again. This page sends within the same limits, sized in
// UTF-8 bytes (what a data channel counts), and the others cannot grow the
// registers past MAX_REGISTERS live ones.
//
// An edit can still go missing (a link that dies while it carries one, a
// member reached only through others), so pages also compare digests of their
// registers: every SUM_MS, and soon after anyone joins or leaves, each page
// tells each page it is linked to its digest once its own edits have been
// quiet for a moment, and two pages that differ send each other all their
// registers (merging them is harmless, the newer write wins).
//
// `connect` makes links for introductions ({offer(), answer({id, sdp})});
// `page` is how the group tells the page what happened; `timers` and `hash`
// can be replaced (tests run the protocol on a clock of their own). A link is
// {send(text), sendCursor(text), sendBinary(bytes), buffered(), drain(n),
// close(), onmessage(data, channel), onclose()}.
import { PROTOCOL, MAX_PEERS, MAX_MESSAGE, COLORS, readMessage, limiter, randomId, utf8Length, quietly } from './wire.js';
import { sha256Hex } from '../../sha256.js';
import { COUNTER_WINDOW, validOp } from './replica.js';

export const MAX_REGISTERS = 100000;
const MAX_STORED = 2 * MAX_REGISTERS;
const CHUNK = 64 << 10;
const HIGH_WATER = 1 << 20;
const INTRO_MS = 25000;
const RETRY_MS = 6000;
const MAX_TRIES = 3;
const FLUSH_MS = 50;
const RESYNC_MS = 5000;
const SUM_MS = 10000;
const HELLO_MS = 1000;
const HELLO_TRIES = 30;
const EARLY_MAX = 500;
/** What a page may send before the other page has its hello. */
const BEFORE_HELLO = new Set(['hello', 'bye', 'full']);
const SUM_SOON_MS = 1500;
const QUIET_MS = 2000;
/** How fast one link may move this page's clock on: COUNTER_WINDOW per RAISE_MS, at most COUNTER_WINDOW at once. */
const RAISE_MS = 60000;
/** The real clock (tests pass one of their own). */
export const TIMERS = {
  setTimeout: (fn, ms) => setTimeout(fn, ms),
  clearTimeout: (id) => clearTimeout(id),
  now: () => Date.now(),
};

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


export class Group {
  constructor({ me, name, build, replica, connect, page, timers = TIMERS, hash = sha256Hex, sumMs = SUM_MS, quietMs = QUIET_MS }) {
    this.me = me;
    this.name = name;
    this.build = build;
    this.replica = replica;
    this.connect = connect;
    this.page = page;
    this.timers = timers;
    this.hash = hash;
    this.sumMs = sumMs;
    this.quietMs = quietMs;
    this.sumTimer = 0;
    this.soonTimer = 0;
    this.lastActivity = 0;
    this.sid = null;
    this.color = null;
    this.links = new Set();
    this.peers = new Map();
    this.known = new Map();
    this.intros = new Map();
    this.blocked = new Set();
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
    this.#sumEvery();
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

  /** A link from an invite: `joining` on the page that opened the invite; `invite` names the invite on the page that made it. */
  addLink(link, { joining = false, peer = null, invite = null } = {}) {
    const rec = {
      link, peer, joining, invite, member: false, hello: false, name: '', color: null, where: null, listed: new Set(),
      lim: {
        ops: limiter(20, 20, this.timers.now), snap: limiter(200, 400, this.timers.now), cur: limiter(30, 45, this.timers.now),
        ping: limiter(1, 2, this.timers.now), other: limiter(20, 40, this.timers.now),
      },
      out: limiter(18, 18, this.timers.now), rx: null, dropped: 0, gone: false, resyncAt: 0, snapAt: 0, sumAt: 0,
      early: [], acked: false, told: false, waiting: [], onAck: [], raise: { room: COUNTER_WINDOW, at: this.timers.now() },
    };
    this.links.add(rec);
    link.onmessage = (data, channel) => this.#receive(rec, data, channel);
    link.onclose = () => this.#drop(rec);
    this.#sayHello(rec, 0);
    return rec;
  }

  /**
   * Send this page's hello, and again every HELLO_MS until the other page
   * shows it got one; a link whose other page never does is dropped.
   */
  #sayHello(rec, tries) {
    if (rec.gone || rec.failed || rec.acked || this.closed) return;
    if (tries > HELLO_TRIES) {
      this.#drop(rec);
      return;
    }
    this.#helloNow(rec);
    this.timers.setTimeout(() => this.#sayHello(rec, tries + 1), HELLO_MS);
  }

  #helloNow(rec) {
    const sent = this.#send(rec, {
      t: 'hello', proto: PROTOCOL, build: this.build, peer: this.me, name: this.name,
      color: this.color, sid: rec.joining ? null : this.sid, file: this.page.fileMeta(), seen: rec.hello,
    });
    if (sent && rec.hello) rec.told = true;
  }

  /** The other page has this page's hello: send what waited for it. */
  #acked(rec) {
    if (rec.acked) return;
    rec.acked = true;
    const waiting = rec.waiting;
    rec.waiting = [];
    if (waiting.length) rec.told = true;
    for (const text of waiting) quietly(() => rec.link.send(text));
    const onAck = rec.onAck;
    rec.onAck = [];
    for (const done of onAck) done();
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
    this.timers.clearTimeout(this.flushTimer);
    this.timers.clearTimeout(this.rosterTimer);
    this.timers.clearTimeout(this.sumTimer);
    this.timers.clearTimeout(this.soonTimer);
    for (const rec of this.links) {
      this.#send(rec, { t: 'bye' });
      rec.gone = true;
    }
    const links = [...this.links];
    this.timers.setTimeout(() => { for (const rec of links) quietly(() => rec.link.close()); }, 250);
    for (const intro of this.intros.values()) quietly(() => intro.cancel?.());
    this.links.clear();
    this.peers.clear();
    this.known.clear();
    this.intros.clear();
    this.outbox.clear();
  }

  // ── receiving ────────────────────────────────────────────────────────────

  /**
   * Send a message (or prebuilt JSON text) if it fits in one data-channel
   * message; until the other page has this page's hello, it waits.
   */
  #send(rec, msg) {
    const text = typeof msg === 'string' ? msg : JSON.stringify(msg);
    if (text.length * 3 > MAX_MESSAGE && utf8Length(text) > MAX_MESSAGE) return false;
    const hello = typeof msg === 'object' && BEFORE_HELLO.has(msg.t);
    if (!rec.acked && !hello) {
      rec.waiting.push(text);
      return true;
    }
    if (!hello) rec.told = true;
    return quietly(() => { rec.link.send(text); return true; }) === true;
  }

  #receive(rec, data, channel) {
    if (this.closed || rec.gone) return;
    if (typeof data !== 'string') {
      if (rec.hello) this.#chunk(rec, data);
      else this.#hold(rec, data, channel);
      return;
    }
    const m = readMessage(data, channel);
    if (!m) {
      rec.dropped++;
      return;
    }
    if (!rec.hello && m.t !== 'hello') {
      if (channel === 'edits') this.#hold(rec, data, channel);
      return;
    }
    if (m.t !== 'hello') this.#acked(rec);
    if (!rec.lim[m.t in rec.lim ? m.t : 'other'].take()) {
      rec.dropped++;
      if (m.t === 'ops' || m.t === 'snap') this.#askResync(rec);
      return;
    }
    switch (m.t) {
      case 'hello':
        this.#hello(rec, m);
        if (!rec.hello || rec.gone || rec.failed) break;
        if (m.seen) this.#acked(rec);
        if (!m.seen || !rec.told) this.#helloNow(rec);
        this.#replay(rec);
        break;
      case 'welcome': this.#welcome(rec, m); break;
      case 'snap': this.#ops(rec, m.ops, m.last); break;
      case 'ops': this.#ops(rec, m.ops, null); break;
      case 'resync': this.#resync(rec); break;
      case 'sum': this.#sum(rec, m.h); break;
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

  /** Something the other page sent before its hello arrived: kept, to read once it does. */
  #hold(rec, data, channel) {
    if (rec.early.length < EARLY_MAX) rec.early.push([data, channel]);
    else rec.dropped++;
  }

  #replay(rec) {
    const early = rec.early;
    rec.early = [];
    for (const [data, channel] of early) this.#receive(rec, data, channel);
  }

  /** Ask a page for all its registers again (after this page had to drop some of its edits). */
  #askResync(rec) {
    const now = this.timers.now();
    if (!rec.member || now < rec.resyncAt) return;
    rec.resyncAt = now + RESYNC_MS;
    this.timers.setTimeout(() => { if (!rec.gone && !this.closed) this.#send(rec, { t: 'resync' }); }, 1000);
  }

  /** A page asked for all of this page's registers: at most once per RESYNC_MS each. */
  #resync(rec) {
    const now = this.timers.now();
    if (!rec.member || !this.replica || now < rec.snapAt) return;
    rec.snapAt = now + RESYNC_MS;
    this.#sendSnap(rec);
  }

  // ── digests: do two pages hold the same registers? ───────────────────────

  #activity() {
    this.lastActivity = this.timers.now();
  }

  /** Nothing of this page's is waiting to go out, and no edit came or went for a moment. */
  #quiet() {
    return !this.outbox.size && this.timers.now() - this.lastActivity >= this.quietMs;
  }

  #sumEvery() {
    this.timers.clearTimeout(this.sumTimer);
    this.sumTimer = this.timers.setTimeout(() => {
      this.sumTimer = 0;
      if (!this.active) return;
      this.#sumRound();
      this.#sumEvery();
    }, this.sumMs);
  }

  /** Someone joined or left: compare digests soon rather than at the next round. */
  #sumSoon() {
    if (this.soonTimer || !this.active) return;
    this.soonTimer = this.timers.setTimeout(() => {
      this.soonTimer = 0;
      this.#sumRound();
    }, SUM_SOON_MS);
  }

  #sumRound() {
    if (!this.active || !this.replica || !this.#quiet()) return;
    const h = this.replica.digest();
    for (const rec of this.peers.values()) if (rec.member) this.#send(rec, { t: 'sum', h });
  }

  /** Another page's digest: when it differs (and both pages are quiet), send ours and ask for theirs. */
  #sum(rec, h) {
    if (!rec.member || !this.replica || !this.#quiet() || h === this.replica.digest()) return;
    const now = this.timers.now();
    if (now < rec.sumAt) return;
    rec.sumAt = now + this.sumMs;
    this.#sendSnap(rec);
    this.#send(rec, { t: 'resync' });
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
    if (m.peer === this.me || (rec.peer && rec.peer !== m.peer) || this.peers.has(m.peer) || this.blocked.has(m.peer)) {
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
      this.timers.setTimeout(() => this.#drop(rec), 250);
      return;
    }
    if (this.size() >= MAX_PEERS) {
      rec.failed = true;
      this.#send(rec, { t: 'full' });
      this.peers.delete(m.peer);
      this.timers.setTimeout(() => this.#drop(rec), 250);
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
    this.page.event('joined', { peer: rec.peer, name: rec.name, invite: rec.invite });
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
    this.page.welcomed({ from: rec.peer, name: rec.name, file: m.file, example: m.example, send: m.send, sid: m.sid });
    if (this.closed || rec.gone) return;
    this.#sendSnap(rec);
    this.#sendWhere(rec);
    this.#sumEvery();
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
      const raise = validOp(op) ? op.c[0] - this.replica.counter : 0;
      if (raise > 0 && raise <= COUNTER_WINDOW && !this.#spend(rec, raise)) {
        this.#misbehaved(rec);
        break;
      }
      const prev = before ? { v: before.v, by: before.c[1] } : null;
      const r = this.replica.receive(op);
      if (r === 'invalid') {
        rec.dropped++;
        continue;
      }
      if (!r) continue;
      const now = this.replica.regs.get(op.k);
      fresh.push({ k: op.k, v: now.v, c: [now.c[0], now.c[1]], b: [now.b[0], now.b[1]] });
      changes.push({ key: op.k, value: now.v, by: now.c[1], prev: prev ? prev.v : null, prevBy: prev ? prev.by : null });
    }
    if (fresh.length) {
      this.#activity();
      this.#queue(fresh, rec);
    }
    if (changes.length) this.page.changed(changes, rec.peer);
    if (this.closed || rec.gone) return;
    if (last === true && rec.sponsor && !rec.caughtUp) {
      rec.caughtUp = true;
      this.page.caughtUp(rec.peer);
    }
  }

  /**
   * Charge `n` (how far an op moves this page's clock on) to the link it came
   * by. Pages' clocks move by one per edit, so a link that moves it faster
   * than anyone edits is a page not working as it should: without this, one
   * could push every page's clock to the end of its range in minutes, after
   * which no one's edits would be accepted.
   */
  #spend(rec, n) {
    const now = this.timers.now();
    const r = rec.raise;
    r.room = Math.min(COUNTER_WINDOW, r.room + ((now - r.at) / RAISE_MS) * COUNTER_WINDOW);
    r.at = now;
    if (n > r.room) return false;
    r.room -= n;
    return true;
  }

  /** A page that sent what no page working as it should sends: stop linking to it, and say so. */
  #misbehaved(rec) {
    if (rec.gone) return;
    rec.misbehaved = true;
    if (rec.peer) {
      this.blocked.add(rec.peer);
      this.known.delete(rec.peer);
    }
    this.#drop(rec);
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
    Promise.resolve(this.hash(rx.buf)).then((hash) => {
      if (this.closed || rec.gone) return;
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
    this.#sumSoon();
  }

  #know(x, via) {
    if (this.blocked.has(x.peer)) return;
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
    if (m.from === this.me || this.peers.has(m.from) || this.intros.has(m.from) || m.from > this.me || this.blocked.has(m.from)) return;
    if (this.size() >= MAX_PEERS && !this.known.has(m.from)) return;
    this.#pair(m.from, rec, async () => {
      const res = await this.connect.answer({ id: m.id, sdp: m.d });
      return { id: m.id, cancel: res.cancel, ready: res.ready, relay: res.sdp ? { kind: 'answer', id: m.id, d: res.sdp } : null };
    });
  }

  #mesh() {
    if (!this.active) return;
    const now = this.timers.now();
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
    const timer = this.timers.setTimeout(() => this.#introFailed(peer, intro), INTRO_MS);
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
      this.timers.clearTimeout(timer);
      this.intros.delete(peer);
      this.addLink(link, { peer });
    } catch (_) {
      this.timers.clearTimeout(timer);
      this.#introFailed(peer, intro);
    }
  }

  #introFailed(peer, intro) {
    if (this.intros.get(peer) !== intro) return;
    this.intros.delete(peer);
    quietly(() => intro.cancel?.());
    const k = this.known.get(peer);
    if (k) k.retryAt = this.timers.now() + RETRY_MS;
    this.timers.setTimeout(() => this.#mesh(), RETRY_MS + 50);
  }

  #drop(rec) {
    if (rec.gone) return;
    rec.gone = true;
    rec.waiting = [];
    for (const done of rec.onAck.splice(0)) done();
    this.links.delete(rec);
    quietly(() => rec.link.close());
    if (rec.rx) {
      rec.rx = null;
      this.page.fileFailed('lost');
    }
    const joinFailed = rec.joining && !rec.member && !rec.failed && !this.closed;
    const unjoined = !rec.joining && rec.invite && !rec.member && !rec.failed && !this.closed;
    const early = () => {
      if (joinFailed) this.page.event('closed', { name: rec.name });
      else if (unjoined) this.page.event('unjoined', { invite: rec.invite, name: rec.name });
    };
    if (!rec.peer || this.peers.get(rec.peer) !== rec) {
      early();
      return;
    }
    this.peers.delete(rec.peer);
    this.outbox.delete(rec);
    if (!rec.member) {
      early();
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
    if (!rec.bye && !rec.misbehaved) {
      for (const other of this.peers.values()) {
        if (other.member && other.listed.has(rec.peer)) this.#know({ peer: rec.peer, name: rec.name, color: rec.color }, other.peer);
      }
    }
    const k = this.known.get(rec.peer);
    if (k) {
      k.tries = 0;
      k.retryAt = this.timers.now() + RETRY_MS;
      this.timers.setTimeout(() => this.#mesh(), RETRY_MS + 50);
    }
    this.page.event(rec.misbehaved ? 'misbehaved' : rec.bye ? 'left' : 'lost', { peer: rec.peer, name: rec.name, reachable: !!k });
    if (this.closed) return;
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
    this.#sumSoon();
    if (this.rosterTimer) return;
    this.rosterTimer = this.timers.setTimeout(() => {
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
    if (!rec.acked) await new Promise((done) => rec.onAck.push(done));
    if (rec.gone || this.closed) return;
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
    this.#activity();
    if (!this.flushTimer && this.outbox.size) this.flushTimer = this.timers.setTimeout(() => this.#flush(), FLUSH_MS);
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
    if (this.outbox.size) this.flushTimer = this.timers.setTimeout(() => this.#flush(), 100);
  }
}
