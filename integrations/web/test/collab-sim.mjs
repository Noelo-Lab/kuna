// collab-sim.mjs — live sessions without a browser: pages that each hold a
// real Session and a real Sync (collab/sync.js) with its Group (group.js),
// joined by in-memory links, on a virtual clock that runs every timer the
// protocol sets (the 16 ms apply batch, flushes, digests, retries). A page
// stands in for app.js the way the study view drives Sync: every edit is
// followed by `sessionChanged` (Sync.local, then saving), the others' changes
// come back through `remoteChanged`, a program another page sends is opened
// with `openShared` (the Session is swapped at once and the load finishes a
// moment later, false when a later open replaced it, as in app.js), and
// leaving gives a session kept apart back to the student's own changes
// (`endShared`). Links deliver in order with a latency,
// can fail (what was in flight is lost, and each side notices later), can
// lose the first message one side sends (as a data channel may as it opens),
// and some pairs of pages can be made unable to link directly.
//
// Used by decompile2-collab-sync.mjs (scripted cases) and
// decompile2-collab-fuzz.mjs (random runs).
import { Session } from '../decompile2/session.js';
import { Sync } from '../decompile2/collab/sync.js';
import { Group } from '../decompile2/collab/group.js';
import { Replica, adoptRawKeys, applyRegisters, birthOrder, changedBetween, registersOf } from '../decompile2/collab/replica.js';
import { sha256Js } from '../sha256.js';

export const BUILD = 'b'.repeat(64);
export const PROGRAM_BYTES = new Uint8Array(Array.from({ length: 48 }, (_, i) => (i * 37 + 11) & 0xff));
export const PROGRAM_HASH = sha256Js(PROGRAM_BYTES);
export const PROGRAM = { name: 'prog.elf', bytes: PROGRAM_BYTES, hash: `sha256:${PROGRAM_HASH}` };
export const FN = '0x1100';

/** A seeded generator (mulberry32): `rng()` in [0, 1), `int(n)`, `pick(list)`, `chance(p)`. */
export function prng(seed) {
  let a = seed >>> 0;
  const rng = () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = a;
    t = Math.imul(t ^ (t >>> 15), t | 1);
    t ^= t + Math.imul(t ^ (t >>> 7), t | 61);
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
  rng.int = (n) => Math.floor(rng() * n);
  rng.pick = (list) => list[rng.int(list.length)];
  rng.chance = (p) => rng() < p;
  return rng;
}

/** A virtual clock: `setTimeout`/`clearTimeout`/`now` for the code under test, run by `run(ms)`. */
export class Clock {
  constructor() {
    this.t = 0;
    this.heap = [];
    this.seq = 0;
    this.live = new Map();
    this.onTask = null;
  }

  now() { return this.t; }

  setTimeout(fn, ms = 0) {
    const id = ++this.seq;
    const e = { at: this.t + Math.max(0, Math.floor(ms) || 0), id, fn };
    this.live.set(id, e);
    this.#push(e);
    return id;
  }

  clearTimeout(id) {
    const e = this.live.get(id);
    if (e) e.fn = null;
    this.live.delete(id);
  }

  get timers() {
    return { setTimeout: (fn, ms) => this.setTimeout(fn, ms), clearTimeout: (id) => this.clearTimeout(id), now: () => this.now() };
  }

  #less(a, b) { return a.at !== b.at ? a.at < b.at : a.id < b.id; }

  #push(e) {
    const h = this.heap;
    h.push(e);
    let i = h.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (!this.#less(h[i], h[p])) break;
      [h[i], h[p]] = [h[p], h[i]];
      i = p;
    }
  }

  #pop() {
    const h = this.heap;
    const top = h[0];
    const last = h.pop();
    if (h.length) {
      h[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < h.length && this.#less(h[l], h[m])) m = l;
        if (r < h.length && this.#less(h[r], h[m])) m = r;
        if (m === i) break;
        [h[i], h[m]] = [h[m], h[i]];
        i = m;
      }
    }
    return top;
  }

  /** Run every timer due in the next `ms` (and the promises they settle), then move the clock to the end. */
  async run(ms) {
    const end = this.t + ms;
    while (this.heap.length && this.heap[0].at <= end) {
      const e = this.#pop();
      if (!e.fn) continue;
      this.live.delete(e.id);
      this.t = Math.max(this.t, e.at);
      const fn = e.fn;
      e.fn = null;
      await this.task(fn, e.label);
    }
    this.t = end;
  }

  /** Run `fn` as one task: its promise continuations settle before the next one starts. */
  async task(fn, label = null) {
    this.onTask?.(label, true);
    try {
      fn();
    } finally {
      for (let i = 0; i < 20; i++) await null;
      this.onTask?.(label, false);
    }
  }

  /** Schedule `fn` with a label the task hook sees (a message delivery names its page). */
  at(ms, fn, label) {
    const id = this.setTimeout(fn, ms);
    this.live.get(id).label = label;
    return id;
  }
}

/** The network: link pairs with latency and failures, and which pairs of pages can link directly. */
export class Net {
  constructor(clock, rng, { latency = [1, 30], noticeMs = [50, 6000], loseFirst = 0 } = {}) {
    this.clock = clock;
    this.rng = rng;
    this.latency = latency;
    this.noticeMs = noticeMs;
    this.loseFirst = loseFirst;
    this.blocked = new Set();
    this.links = new Set();
    this.offers = new Map();
    this.seq = 0;
  }

  block(a, b) {
    this.blocked.add([a, b].sort().join('|'));
  }

  canLink(a, b) {
    return !this.blocked.has([a, b].sort().join('|'));
  }

  #lat() {
    const [lo, hi] = this.latency;
    return lo + this.rng.int(hi - lo + 1);
  }

  /** Two ends of one link, `a` at page `pa` and `b` at page `pb`. */
  pair(pa, pb) {
    const pair = { pa, pb, dead: false, ends: [] };
    const make = (me, other) => {
      const end = {
        page: me, onmessage: null, onclose: null, closed: false, lastAt: 0,
        send: (text) => this.#deliver(pair, end, text),
        sendCursor: () => {},
        sendBinary: (bytes) => this.#deliver(pair, end, bytes.slice().buffer),
        buffered: () => 0,
        drain: () => Promise.resolve(),
        close: () => this.#close(pair, end, 0),
      };
      return end;
    };
    const a = make(pa, pb);
    const b = make(pb, pa);
    a.pair = pair;
    b.pair = pair;
    pair.ends = [a, b];
    if (this.loseFirst && this.rng.chance(this.loseFirst)) (this.rng.chance(0.5) ? a : b).loseNext = true;
    this.links.add(pair);
    return [a, b];
  }

  #deliver(pair, from, data) {
    if (pair.dead || from.closed) return;
    if (from.loseNext) {
      from.loseNext = false;
      return;
    }
    const to = pair.ends[0] === from ? pair.ends[1] : pair.ends[0];
    const at = Math.max(this.clock.now() + this.#lat(), from.lastAt);
    from.lastAt = at;
    this.clock.at(at - this.clock.now(), () => {
      if (!pair.dead && !to.closed) to.onmessage?.(data, 'edits');
    }, to.page);
  }

  /** One side closes: the other notices after a moment (what was still on the way is lost). */
  #close(pair, end, notice) {
    if (end.closed) return;
    end.closed = true;
    const other = pair.ends[0] === end ? pair.ends[1] : pair.ends[0];
    const first = !pair.dead;
    pair.dead = true;
    this.links.delete(pair);
    end.onclose?.();
    if (first && !other.closed) {
      this.clock.at(notice || this.#lat(), () => {
        if (other.closed) return;
        other.closed = true;
        other.onclose?.();
      }, other.page);
    }
  }

  /** The connection fails: nothing more gets through, and each side notices within `noticeMs`. */
  fail(pair) {
    if (pair.dead) return;
    pair.dead = true;
    this.links.delete(pair);
    const [lo, hi] = this.noticeMs;
    for (const end of pair.ends) {
      this.clock.at(lo + this.rng.int(hi - lo + 1), () => {
        if (end.closed) return;
        end.closed = true;
        end.onclose?.();
      }, end.page);
    }
  }

  #fakeSdp(token) {
    return { u: token, p: 'p'.repeat(22), f: 'A'.repeat(64), c: [] };
  }

  /** What group.js uses to link two members through a third (`connect`), for page `me`. */
  connect(me) {
    return {
      offer: async () => {
        const id = String(++this.seq).padStart(10, '0');
        const token = `o${id}`;
        let ok;
        let no;
        const ready = new Promise((res, rej) => { ok = res; no = rej; });
        ready.catch(() => {});
        const offer = { me, id, ok, no, done: false };
        this.offers.set(token, offer);
        return {
          id, sdp: this.#fakeSdp(token), ready,
          answer: async (d) => {
            const answer = this.offers.get(d.u);
            if (!answer || offer.done) return;
            offer.done = true;
            const [a, b] = this.pair(offer.me, answer.me);
            this.offers.delete(d.u);
            this.offers.delete(token);
            ok(a);
            answer.ok(b);
          },
          cancel: () => {
            this.offers.delete(token);
            no(new Error('cancelled'));
          },
        };
      },
      answer: async ({ sdp }) => {
        const offer = this.offers.get(sdp.u);
        const token = `a${String(++this.seq).padStart(10, '0')}`;
        let ok;
        const ready = new Promise((res) => { ok = res; });
        if (offer && this.canLink(offer.me, me)) this.offers.set(token, { me, ok });
        return { sdp: this.#fakeSdp(token), ready, cancel: () => this.offers.delete(token) };
      },
    };
  }
}

/** One page: what app.js does around Sync, with its own storage. */
export class Page {
  constructor(sim, id, { program = true, own = null, name = null, browser = null, openMs = null } = {}) {
    this.sim = sim;
    this.id = id;
    this.openMs = openMs;
    this.name = name || `P${id.slice(0, 3)}`;
    this.program = program ? PROGRAM : null;
    this.session = new Session();
    this.mode = 'auto';
    this.slot = 'own';
    this.store = browser?.store || new Map();
    this.sharedStore = browser?.sharedStore || new Map();
    this.touched = new Set();
    this.opening = 0;
    this.events = [];
    this.joinFailures = [];
    this.replacedCopies = [];
    if (own) {
      own(this.session);
      adoptRawKeys(this.session, id);
      for (const k of registersOf(this.session).keys()) this.touched.add(k);
      this.store.set(PROGRAM.hash, JSON.stringify(this.session.toJSON()));
      if (!this.program) this.session = new Session();
    }
    this.sync = new Sync({
      me: id, app: this.#app(), ui: this.#ui(), timers: sim.clock.timers, hash: (b) => Promise.resolve(sha256Js(b)),
    });
  }

  #app() {
    return {
      session: () => this.session,
      binary: () => (this.program ? { ...this.program, example: false } : null),
      fileMeta: () => (this.program ? { name: this.program.name, size: this.program.bytes.length, hash: this.program.hash.slice(7) } : null),
      mode: () => this.mode,
      target: () => FN,
      nameOf: (a) => a,
      toast: () => {},
      remoteChanged: ({ mode }) => {
        this.checkOutcomes();
        if (mode) this.mode = mode;
        this.sessionChanged();
      },
      clearUndo: () => {},
      shareStarted: () => {},
      refresh: () => {},
      endShared: () => this.#endShared(),
      ownSession: (hash) => {
        const own = this.#restore(hash);
        return own.size ? own : null;
      },
      openShared: ({ session, slot, mode, still = () => true }) => {
        if (!still()) return Promise.resolve(false);
        this.program = PROGRAM;
        this.session = session;
        this.slot = slot;
        if (mode) this.mode = mode;
        const open = ++this.opening;
        return new Promise((done) => {
          this.sim.clock.at(this.openMs ?? this.sim.rng.int(200), () => done(this.opening === open), this.id);
        });
      },
    };
  }

  #ui() {
    return {
      nameOf: (peer) => peer,
      joined: () => this.sim.stats.joined++,
      joinFailed: (reason) => {
        this.sim.stats.failedJoins++;
        this.joinFailures.push(reason);
      },
      event: (kind) => this.events.push(kind),
      mergeReplaced: ({ copy }) => this.replacedCopies.push(copy),
    };
  }

  #restore(hash) {
    const text = this.store.get(hash);
    return text ? Session.fromJSON(JSON.parse(text)) : new Session();
  }

  #endShared() {
    this.session.orderOf = null;
    if (this.slot !== 'shared' || !this.program) {
      this.slot = 'own';
      return;
    }
    const hash = this.program.hash;
    const own = this.#restore(hash);
    this.slot = 'own';
    if (!own.size) {
      this.persist();
      this.sharedStore.delete(hash);
      return;
    }
    this.session = own;
    this.sessionChanged();
  }

  persist() {
    if (!this.program) return;
    const where = this.slot === 'shared' ? this.sharedStore : this.store;
    if (this.session.size) where.set(this.program.hash, JSON.stringify(this.session.toJSON()));
    else where.delete(this.program.hash);
  }

  /** app.js sessionChanged: Sync first, then save; then every record is as the engine last applied it. */
  sessionChanged() {
    this.sync.local();
    this.persist();
    this.seen = new Map();
    for (const [key, rec] of this.session.records) {
      this.session.outcomes.set(key, { status: 'applied', detail: null, fatal: false });
      this.seen.set(key, JSON.stringify(rec));
    }
    this.seenSession = this.session;
  }

  /** The others' changes reached the Session: a record they did not change keeps its outcome. */
  checkOutcomes() {
    if (this.seenSession !== this.session || !this.seen) return;
    for (const [key, rec] of this.session.records) {
      if (this.seen.get(key) === JSON.stringify(rec) && !this.session.outcomes.has(key)) {
        this.sim.violations.push(`${this.id}: applying the others' changes cleared the outcome of ${key}, which did not change`);
      }
    }
  }

  /** A student's edit: `mutate(session)`, then sessionChanged; returns the registers it changed. */
  edit(mutate) {
    adoptRawKeys(this.session, this.id);
    const before = registersOf(this.session);
    mutate(this.session);
    adoptRawKeys(this.session, this.id);
    const keys = new Set(changedBetween(before, registersOf(this.session)).keys());
    for (const k of keys) this.touched.add(k);
    this.sim.act(this, keys, () => this.sessionChanged());
    return keys;
  }

  undo() {
    const step = this.sync.history.undoStack.at(-1) || [];
    this.sim.act(this, new Set(step.map((e) => e.key)), () => {
      if (this.sync.undo()) this.sessionChanged();
    });
  }

  redo() {
    const step = this.sync.history.redoStack.at(-1) || [];
    this.sim.act(this, new Set(step.map((e) => e.key)), () => {
      if (this.sync.redo()) this.sessionChanged();
    });
  }

  setMode(mode) {
    this.touched.add('setting:mode');
    this.sim.act(this, new Set(['setting:mode']), () => {
      this.mode = mode;
      this.sync.modeChanged(mode);
    });
  }

  start() {
    const keys = new Set([...registersOf(this.session).keys(), 'setting:mode']);
    for (const k of keys) this.touched.add(k);
    this.sim.act(this, keys, () => this.sync.start({ build: BUILD, name: this.name, connect: this.sim.net.connect(this.id) }));
  }

  /** Open `host`'s invite (a link carried by hand). */
  joinVia(host) {
    const [a, b] = this.sim.net.pair(host.id, this.id);
    this.sim.act(this, new Set(), () => {
      host.sync.addLink(a);
      this.sync.beginJoin({ build: BUILD, name: this.name, connect: this.sim.net.connect(this.id), link: b });
    });
  }

  leave() {
    this.sim.act(this, new Set(), () => this.sync.leave());
  }

  /** The directives this page sends, in order (every function qualified by address). */
  directives() {
    return this.session.allAssertions((a) => a);
  }
}

/** A simulation: pages, the clock, the network, and the check on what each page sends as its own. */
export class Sim {
  constructor(seed = 1, options = {}) {
    this.rng = prng(seed);
    this.clock = new Clock();
    this.net = new Net(this.clock, this.rng, options);
    this.pages = new Map();
    this.violations = [];
    this.ctx = null;
    this.stats = { joined: 0, failedJoins: 0, ops: 0, sharedAtEnd: 0, pages: 0 };
    this.clock.onTask = (label, on) => {
      if (!on) {
        this.ctx = null;
        return;
      }
      const page = label ? this.pages.get(label) : null;
      this.ctx = page ? { kind: 'deliver', page, joining: page.sync.joining } : { kind: 'timer' };
    };
    Sim.current = this;
  }

  page(id, options) {
    const p = new Page(this, id, options);
    this.pages.set(id, p);
    return p;
  }

  /** Run a student's action on `page`; it may send ops only for `allow`. */
  act(page, allow, fn) {
    this.ctx = { kind: 'act', page, allow };
    try {
      fn();
    } finally {
      this.ctx = null;
    }
  }

  /** Group.local, checked: a page sends a write only for a key its student touched now (or, joining, ever). */
  check(me, ops) {
    const page = this.pages.get(me);
    if (!page) return;
    this.stats.ops += ops.length;
    const ctx = this.ctx;
    for (const op of ops) {
      let ok = false;
      if (ctx?.kind === 'act' && ctx.page === page) ok = ctx.allow.has(op.k);
      else if (ctx?.kind === 'deliver' && ctx.page === page && ctx.joining) ok = page.touched.has(op.k) || (op.c[0] === 1 && op.b?.[0] === 1);
      if (!ok) this.violations.push(`${me} sent ${op.k}=${JSON.stringify(op.v)} (${ctx?.kind || 'outside'}${ctx?.page && ctx.page !== page ? ` of ${ctx.page.id}` : ''}) at ${this.clock.now()} ms`);
    }
  }

  run(ms) {
    return this.clock.run(ms);
  }

  /** Pages in a session linked to each other (both sides count the other a member), grouped. */
  components() {
    const shared = [...this.pages.values()].filter((p) => p.sync.shared && p.sync.group?.active);
    const linked = (p, q) => p.sync.group.peers.get(q.id)?.member && q.sync.group.peers.get(p.id)?.member;
    const seen = new Set();
    const out = [];
    for (const p of shared) {
      if (seen.has(p)) continue;
      const comp = [p];
      seen.add(p);
      for (let i = 0; i < comp.length; i++) {
        for (const q of shared) if (!seen.has(q) && linked(comp[i], q)) { seen.add(q); comp.push(q); }
      }
      out.push(comp);
    }
    return out;
  }

  /** What must hold once everything settled; returns the problems found. */
  problems() {
    const out = [...this.violations];
    for (const p of this.pages.values()) {
      if (p.sync.phase === 'joining') out.push(`${p.id} is still joining after everything settled`);
      if (p.sync.phase !== 'solo') continue;
      if (p.slot === 'shared') out.push(`${p.id} is out of any session but still saves into the shared slot`);
      if (p.session.orderOf) out.push(`${p.id} is out of any session but still orders its directives by a session's births`);
    }
    this.stats.pages = this.pages.size;
    this.stats.sharedAtEnd = [...this.pages.values()].filter((p) => p.sync.shared).length;
    for (const comp of this.components()) {
      const [first, ...rest] = comp;
      const regs = (p) => JSON.stringify([...p.sync.replica.regs].sort(([a], [b]) => (a < b ? -1 : 1)));
      for (const p of rest) {
        if (p.sync.group.sid !== first.sync.group.sid) out.push(`${p.id} and ${first.id} are linked but in different sessions`);
        else if (regs(p) !== regs(first)) out.push(`${p.id} and ${first.id} hold different registers`);
        else if (JSON.stringify(p.directives()) !== JSON.stringify(first.directives())) out.push(`${p.id} and ${first.id} send different directives`);
      }
    }
    for (const p of this.pages.values()) {
      if (!p.sync.shared) continue;
      const r = p.sync.replica;
      const fresh = new Session();
      applyRegisters(fresh, r, [...r.regs.keys()]);
      fresh.orderOf = birthOrder(r);
      const mine = JSON.stringify([...registersOf(p.session)].sort());
      const theirs = JSON.stringify([...registersOf(fresh)].sort());
      if (mine !== theirs) out.push(`${p.id}'s Session is not its registers: ${mine} vs ${theirs}`);
      else if (JSON.stringify(p.directives()) !== JSON.stringify(fresh.allAssertions((a) => a))) out.push(`${p.id}'s directives are not in the registers' order`);
      if (p.mode !== (r.value('setting:mode') || p.mode)) out.push(`${p.id} runs ${p.mode}, the session ${r.value('setting:mode')}`);
    }
    return out;
  }
}

/** A joiner's earlier field (the oldest clock, `[1, page]`) is written only where the registers hold nothing. */
const apply = Replica.prototype.apply;
const receive = Replica.prototype.receive;
Replica.prototype.receive = function receiving(op) {
  this.receiving = true;
  try {
    return receive.call(this, op);
  } finally {
    this.receiving = false;
  }
};
Replica.prototype.apply = function checkedApply(op) {
  if (!this.receiving && op.c?.[0] === 1 && op.c[1] === this.peer && this.regs.has(op.k)) {
    Sim.current?.violations.push(`${this.peer} wrote its earlier ${op.k}=${JSON.stringify(op.v)} over a field the session holds`);
  }
  return apply.call(this, op);
};

/** Every Group.local call goes through the current simulation's check. */
const local = Group.prototype.local;
Group.prototype.local = function checkedLocal(ops) {
  Sim.current?.check(this.me, ops);
  return local.call(this, ops);
};
