// replica.js — the shared session as last-writer-wins registers. Every field a
// student can change is one register: a local's name and its type, a
// function's name, a signature, a global's type and its name, a type
// definition, a note, one patched byte, one directive the page does not model,
// and the decompiler effort (the engine's symbols depend on it). A write
// carries a Lamport clock `[counter, peer]` and the larger clock wins, so
// pages that have seen the same writes hold the same registers whatever order
// they arrived in; a deletion is a write of null. Each register also keeps its
// birth clock, the smallest clock any page has written it with (merged as a
// grow-only minimum, sent along with every op), which orders the session's
// directives the same way on every page. A replica keeps a digest of all its
// registers (order-free, updated with each write), so two pages can tell
// cheaply whether they hold the same session.
//
// `registersOf` reads a Session as registers, `changedBetween` and
// `localChanges` find what a page changed since a base, and `applyRegisters`
// writes registers back into a Session, field by field. Ops from other pages
// pass `validOp` first: a key of a known shape, a value of its kind's shape
// (the same text rules as the page's own dialogs, from session.js), and a
// clock within bounds. DOM-free.
import { addrHex } from '../addr.js';
import { TEXT_LIMITS, UNDO_MAX, directiveTextProblem, declarationProblem } from '../session.js';

export const MODES = ['auto', 'fast', 'reliable', 'aggressive'];
const MAX_COUNTER = 2 ** 48;
const COUNTER_WINDOW = 2 ** 24;

/** Whether clock `a` is later than clock `b`. */
export const newer = (a, b) => (a[0] !== b[0] ? a[0] > b[0] : a[1] > b[1]);

/** The clock of a write older than any a page makes (`[1, peer]`: every page's own writes start at 2). */
export const oldestClock = (peer) => [1, peer];

const HEX = '0x[0-9a-f]{1,16}';
const SYM = '[A-Za-z_][A-Za-z0-9_]{0,199}';
const PEER = '[a-z0-9]{1,16}';
const IDENT = /^[A-Za-z_][A-Za-z0-9_]{0,199}$/;
const text = (max) => (v) => v.length > 0 && !directiveTextProblem(v, max);
const decl = (max) => (v) => v.length > 0 && !declarationProblem(v, max);
const raw = (v) => text(TEXT_LIMITS.raw)(v) && !v.trim().split(/\s+/).some((t) => t.startsWith('@'));
const KINDS = [
  [new RegExp(`^var:${HEX}:${SYM}:name$`), (v) => IDENT.test(v)],
  [new RegExp(`^var:${HEX}:${SYM}:type$`), decl(TEXT_LIMITS.type)],
  [new RegExp(`^fn:${HEX}$`), (v) => IDENT.test(v)],
  [new RegExp(`^proto:${HEX}$`), decl(TEXT_LIMITS.decl)],
  [new RegExp(`^data:${HEX}:type$`), decl(TEXT_LIMITS.type)],
  [new RegExp(`^data:${HEX}:name$`), (v) => IDENT.test(v)],
  [new RegExp(`^typedef:${SYM}$`), decl(TEXT_LIMITS.typedef)],
  [new RegExp(`^comment:${HEX}:${HEX}$`), text(TEXT_LIMITS.comment)],
  [new RegExp(`^byte:${HEX}$`), (v) => /^[0-9a-f]{2}$/.test(v)],
  [new RegExp(`^raw:${PEER}:[0-9]{1,9}$`), raw],
  [new RegExp(`^rawf:${HEX}:${PEER}:[0-9]{1,9}$`), raw],
  [/^setting:mode$/, (v) => MODES.includes(v)],
];

/** FNV-1a of `text` from `seed`, 32 bits. */
function fnv(text, seed) {
  let h = seed;
  for (let i = 0; i < text.length; i++) {
    h ^= text.charCodeAt(i);
    h = Math.imul(h, 0x01000193) >>> 0;
  }
  return h >>> 0;
}

const hex8 = (n) => n.toString(16).padStart(8, '0');

const clockOk = (c) => Array.isArray(c) && c.length === 2 && Number.isSafeInteger(c[0]) && c[0] >= 1 && c[0] <= MAX_COUNTER &&
  typeof c[1] === 'string' && /^[a-z0-9]{1,16}$/.test(c[1]);

/** Whether `value` may be written to `key` (the check every op from another page passes). */
function validValue(key, value) {
  if (typeof key !== 'string' || key.length > 300) return false;
  const kind = KINDS.find(([re]) => re.test(key));
  return !!kind && (value === null || (typeof value === 'string' && kind[1](value)));
}

/** Whether an op from another page may be applied. */
export function validOp(op) {
  if (!op || typeof op !== 'object' || Array.isArray(op)) return false;
  const keys = Object.keys(op);
  if (keys.length !== 3 && !(keys.length === 4 && 'b' in op)) return false;
  if (!clockOk(op.c) || (op.b !== undefined && (!clockOk(op.b) || newer(op.b, op.c)))) return false;
  return validValue(op.k, op.v);
}

export class Replica {
  constructor(peer) {
    this.peer = peer;
    this.counter = 1;
    this.regs = new Map();
    this.live = 0;
    this.sum = [0, 0];
  }

  /** Fold one register into (or out of) the digest: XOR, so order does not matter. */
  #mix(key, r) {
    const text = `${key}\u0000${r.v === null ? '\u0001' : r.v}\u0000${r.c[0]}.${r.c[1]}\u0000${r.b[0]}.${r.b[1]}`;
    this.sum[0] = (this.sum[0] ^ fnv(text, 0x811c9dc5)) >>> 0;
    this.sum[1] = (this.sum[1] ^ fnv(text, 0x050c5d1f)) >>> 0;
  }

  /** The same text on two pages when they hold the same registers (values, clocks and births). */
  digest() {
    return `${this.regs.size}:${hex8(this.sum[0])}${hex8(this.sum[1])}`;
  }

  value(key) {
    return this.regs.get(key)?.v ?? null;
  }

  /** The page that last wrote `key`, or null. */
  writer(key) {
    return this.regs.get(key)?.c[1] ?? null;
  }

  clock(key) {
    return this.regs.get(key)?.c ?? null;
  }

  birth(key) {
    return this.regs.get(key)?.b ?? null;
  }

  /** A local write (its counter is always past 1, which is kept for `OLDEST`); returns the op to send. */
  set(key, value) {
    const c = [++this.counter, this.peer];
    const op = { k: key, v: value, c, b: this.regs.get(key)?.b || c };
    this.apply(op);
    return op;
  }

  /**
   * An op from another page: 'invalid', or whether it changed the register
   * (its value, or its birth). A counter far past this page's is refused, so
   * one bad op cannot push every page's clock to the end of its range.
   */
  receive(op) {
    if (!validOp(op) || op.c[0] > this.counter + COUNTER_WINDOW) return 'invalid';
    return this.apply(op);
  }

  /**
   * Apply a local or already-checked op; true when the register took it (a
   * newer clock, even with the same value, or an earlier birth). A page passes
   * on every op its register took, so the pages it forwards to end with the
   * same clocks, not only the same values.
   */
  apply(op) {
    if (op.c[0] > this.counter) this.counter = op.c[0];
    const cur = this.regs.get(op.k);
    const b = op.b || op.c;
    if (!cur) {
      const r = { v: op.v, c: [op.c[0], op.c[1]], b: [b[0], b[1]] };
      this.regs.set(op.k, r);
      this.#mix(op.k, r);
      if (op.v !== null) this.live++;
      return true;
    }
    const earlier = newer(cur.b, b);
    const later = newer(op.c, cur.c);
    if (!earlier && !later) return false;
    this.#mix(op.k, cur);
    if (earlier) cur.b = [b[0], b[1]];
    if (later) {
      this.live += (op.v !== null) - (cur.v !== null);
      cur.v = op.v;
      cur.c = [op.c[0], op.c[1]];
    }
    this.#mix(op.k, cur);
    return true;
  }

  /** Every register as an op, for a page catching up. */
  snapshot() {
    return [...this.regs].map(([k, { v, c, b }]) => ({ k, v, c: [c[0], c[1]], b: [b[0], b[1]] }));
  }

  /** How many registers hold a value (tombstones left out). */
  liveCount() {
    return this.live;
  }
}

/**
 * Give this page's own raw directives (`raw:<n>`) their shared name
 * (`raw:<me>:<n>`), so a page that joins again later under another id does
 * not send them twice. Returns how many it renamed.
 */
export function adoptRawKeys(session, me) {
  let n = 0;
  for (const [key, rec] of [...session.records]) {
    const own = /^raw:(\d+)$/.exec(key);
    if (!own || rec.kind !== 'raw') continue;
    const shared = `raw:${me}:${own[1]}`;
    session.records.delete(key);
    session.records.set(shared, rec);
    if (session.outcomes.has(key)) session.outcomes.set(shared, session.outcomes.get(key));
    session.outcomes.delete(key);
    if (session.refused.delete(key)) session.refused.add(shared);
    n++;
  }
  return n;
}

/** The session as registers, `key → value` (values are strings). Raw records must have their shared names. */
export function registersOf(session) {
  const out = new Map();
  for (const [key, rec] of session.records) {
    switch (rec.kind) {
      case 'var':
        if (rec.name) out.set(`var:${rec.func}:${rec.sym}:name`, rec.name);
        if (rec.type) out.set(`var:${rec.func}:${rec.sym}:type`, rec.type);
        break;
      case 'fn': out.set(`fn:${rec.addr}`, rec.name); break;
      case 'proto': out.set(`proto:${rec.addr}`, rec.decl); break;
      case 'data':
        if (rec.type) out.set(`data:${rec.addr}:type`, rec.type);
        if (rec.name) out.set(`data:${rec.addr}:name`, rec.name);
        break;
      case 'typedef': out.set(`typedef:${rec.tag}`, rec.decl); break;
      case 'comment': out.set(`comment:${rec.func}:${rec.addr}`, rec.text); break;
      case 'raw': {
        const m = /^raw:([a-z0-9]{1,16}):(\d+)$/.exec(key);
        if (m) out.set(rec.func ? `rawf:${rec.func}:${m[1]}:${m[2]}` : `raw:${m[1]}:${m[2]}`, rec.text);
        break;
      }
      default: break;
    }
  }
  for (const [addr, value] of session.bytes) out.set(`byte:${addrHex(addr)}`, value.toString(16).padStart(2, '0'));
  return out;
}

/**
 * The registers that differ from `base` to `now` (two `registersOf` maps), as
 * `key → value` (null where `now` no longer has one): what a student changed
 * since the Session last matched the registers. The decompiler effort is not
 * a Session field.
 */
export function changedBetween(base, now) {
  const out = new Map();
  for (const [key, value] of now) if (base.get(key) !== value) out.set(key, value);
  for (const key of base.keys()) if (!now.has(key) && !key.startsWith('setting:')) out.set(key, null);
  return out;
}

/**
 * Which of the writes `want` (`key → value`) to send: `{changes, same,
 * refused}`. One the registers already hold is `same`; a value another page
 * would refuse, or a new register past `maxLive` live ones, is refused (and
 * tried again at the next change, so a later valid value is shared).
 */
export function localChanges(want, replica, { maxLive = Infinity } = {}) {
  const changes = [];
  const same = [];
  const refused = [];
  let live = replica.liveCount();
  for (const [key, value] of want) {
    const had = replica.value(key);
    if (had === value) {
      same.push(key);
      continue;
    }
    if (value !== null && !validValue(key, value)) {
      refused.push({ key, value, why: 'invalid' });
      continue;
    }
    if (value !== null && had === null) {
      if (live >= maxLive) {
        refused.push({ key, value, why: 'full' });
        continue;
      }
      live++;
    }
    changes.push({ key, value });
  }
  return { changes, same, refused };
}

/** The session record key a register lives in (`bytes` for a byte). */
export function recordKeyOf(register) {
  const p = register.split(':');
  switch (p[0]) {
    case 'var': return `var:${p[1]}:${p[2]}`;
    case 'data': return `data:${p[1]}`;
    case 'raw': return `raw:${p[1]}:${p[2]}`;
    case 'rawf': return `raw:${p[2]}:${p[3]}`;
    case 'byte': return 'bytes';
    default: return register;
  }
}

/**
 * Write the registers `keys` (from `replica`) into `session`, field by field:
 * a register the replica has never held leaves its field alone, so applying
 * another page's retype of a local keeps this page's rename of it. A global
 * is a type and a name together: it exists only while both hold a value (a
 * half written null, as when one page deletes a global another renames at the
 * same time, removes it), so every page makes the same Session from the same
 * registers whatever it held before.
 */
export function applyRegisters(session, replica, keys) {
  const done = new Set();
  const field = (k) => (replica.regs.has(k) ? replica.value(k) : undefined);
  let bytes = false;
  for (const key of keys) {
    const parts = key.split(':');
    const v = replica.value(key);
    switch (parts[0]) {
      case 'var': {
        const id = `${parts[1]}:${parts[2]}`;
        if (done.has('var:' + id)) break;
        done.add('var:' + id);
        session.setVar(parts[1], parts[2], { name: field(`var:${id}:name`), type: field(`var:${id}:type`) });
        break;
      }
      case 'fn': session.setFunctionName(parts[1], v); break;
      case 'proto': session.setProto(parts[1], v); break;
      case 'data': {
        const addr = parts[1];
        if (done.has('data:' + addr)) break;
        done.add('data:' + addr);
        const type = field(`data:${addr}:type`);
        const name = field(`data:${addr}:name`);
        if (type === undefined && name === undefined) break;
        session.setData(addr, type || null, name || null);
        break;
      }
      case 'typedef': session.setTypedef(parts[1], v); break;
      case 'comment': session.setComment(parts[1], parts[2], v); break;
      case 'byte': {
        const addr = BigInt(parts[1]);
        if (v === null) session.bytes.delete(addr);
        else session.bytes.set(addr, parseInt(v, 16));
        bytes = true;
        break;
      }
      case 'raw':
      case 'rawf': {
        const func = parts[0] === 'rawf' ? parts[1] : null;
        const [peer, n] = parts.slice(-2);
        session.setRaw(`raw:${peer}:${n}`, v, func);
        break;
      }
      default: break;
    }
  }
  if (bytes) session.touchBytes();
}

/**
 * Each record's place and author in the registers: `recordKey → {birth,
 * clock}`, the earliest birth of its registers and the newest write that
 * holds a value (null when every register of it is deleted).
 */
export function recordIndex(replica) {
  const index = new Map();
  for (const [key, r] of replica.regs) {
    const rk = recordKeyOf(key);
    const e = index.get(rk);
    if (!e) index.set(rk, { birth: r.b, clock: r.v === null ? null : r.c });
    else {
      if (newer(e.birth, r.b)) e.birth = r.b;
      if (r.v !== null && (!e.clock || newer(r.c, e.clock))) e.clock = r.c;
    }
  }
  return index;
}

/**
 * The order of a session's records in a live session (`Session.orderOf`):
 * each record's birth, from `recordIndex` of a replica (or an index already
 * made); null for a record the registers do not hold yet, which then comes last.
 */
export function birthOrder(source) {
  const index = source instanceof Map ? source : recordIndex(source);
  return (recordKey) => index.get(recordKey)?.birth || null;
}

/** What a register change did, in words, for "Ben … after you" (`prev`: the value it replaced). */
export function describeRegister(key, value, prev = null, nameOf = (a) => a) {
  const p = key.split(':');
  switch (p[0]) {
    case 'var':
      if (p[3] === 'name') return value ? `renamed ${prev || p[2]} to ${value}` : `put back the name ${p[2]}`;
      return value ? `changed the type of ${p[2]} to ${value}` : `put back the type of ${p[2]}`;
    case 'fn': return value ? `renamed ${prev || nameOf(p[1])} to ${value}` : `put back the name of ${nameOf(p[1])}`;
    case 'proto': return `changed the signature of ${nameOf(p[1])}`;
    case 'data': return p[2] === 'name' && value ? `named the global at ${p[1]} ${value}` : `changed the global at ${p[1]}`;
    case 'typedef': return value ? `changed the type ${p[1]}` : `removed the type ${p[1]}`;
    case 'comment': return value ? `changed the note at ${p[2]}` : `removed the note at ${p[2]}`;
    case 'byte': return value ? `changed the byte at ${p[1]}` : `put back the byte at ${p[1]}`;
    case 'setting': return `switched the decompiler effort to ${value}`;
    default: return 'changed the same thing';
  }
}

/**
 * This page's own edits, for Undo in a live session. A step is the registers
 * one edit wrote, with the values they held before and the clocks of this
 * page's writes; Undo writes the old values back, except where another page
 * has written since (that register's clock is no longer this page's), and
 * Redo does the same forwards. A global's two registers go back together or
 * not at all. When a step writes a register, the next step that expects the
 * value it wrote takes the new clock, so Undo goes back through several edits
 * of one field.
 */
export class History {
  constructor(max = UNDO_MAX) {
    this.undoStack = [];
    this.redoStack = [];
    this.max = max;
  }

  get canUndo() { return this.undoStack.length > 0; }

  get canRedo() { return this.redoStack.length > 0; }

  /** One local edit: `[{key, prev, op}]` (`op` is the write that replaced `prev`). */
  record(changes) {
    if (!changes.length) return;
    this.undoStack.push(changes.map((ch) => ({ key: ch.key, prev: ch.prev, next: ch.op.v, c: ch.op.c })));
    if (this.undoStack.length > this.max) this.undoStack.shift();
    this.redoStack = [];
  }

  /** Undo the last step on `replica`: `{ops, skipped: [{key, by}]}`, or null when there is none. */
  undo(replica) {
    return this.#step(replica, this.undoStack, this.redoStack, 'prev', 'next');
  }

  redo(replica) {
    return this.#step(replica, this.redoStack, this.undoStack, 'next', 'prev');
  }

  clear() {
    this.undoStack = [];
    this.redoStack = [];
  }

  #step(replica, from, to, write, expect) {
    const step = from.pop();
    if (!step) return null;
    const mine = (e) => {
      const cur = replica.clock(e.key);
      return !!cur && cur[0] === e.c[0] && cur[1] === e.c[1];
    };
    const blocked = new Map();
    for (const e of step) if (e.key.startsWith('data:') && !mine(e)) blocked.set(recordKeyOf(e.key), replica.writer(e.key));
    const ops = [];
    const skipped = [];
    const back = [];
    for (const e of step) {
      if (!mine(e) || blocked.has(recordKeyOf(e.key))) {
        skipped.push({ key: e.key, by: mine(e) ? blocked.get(recordKeyOf(e.key)) : replica.writer(e.key) });
        continue;
      }
      const op = replica.set(e.key, e[write]);
      ops.push(op);
      back.push({ key: e.key, prev: e.prev, next: e.next, c: op.c });
      for (let i = from.length - 1; i >= 0; i--) {
        const later = from[i].find((x) => x.key === e.key);
        if (!later) continue;
        if (later[expect] === e[write]) later.c = op.c;
        break;
      }
    }
    if (back.length) to.push(back);
    return { ops, skipped };
  }
}
