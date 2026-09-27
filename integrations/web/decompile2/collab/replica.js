// replica.js — the shared session as last-writer-wins registers. Every field a
// student can change is one register: a local's name and its type, a
// function's name, a signature, a global's type and its name, a type
// definition, a note, one patched byte, one directive the page does not model,
// and the decompiler effort (the engine's symbols depend on it). A write
// carries a Lamport clock `[counter, peer]` and the larger clock wins, so
// pages that have seen the same writes hold the same registers whatever order
// they arrived in; a deletion is a write of null.
//
// `registersOf` reads a Session as registers and `applyRegisters` writes
// registers back into one. Ops from other pages pass `validOp` first: a key of
// a known shape, a value of its kind's shape, no control characters (a
// newline would start a second directive in an exported .kuna file) and never
// a form that makes the engine read a file (`@FILE`, `bytes ADDR @FILE`).
// DOM-free.

export const MODES = ['auto', 'fast', 'reliable', 'aggressive'];

const newer = (a, b) => (a[0] !== b[0] ? a[0] > b[0] : a[1] > b[1]);

const HEX = '0x[0-9a-f]{1,16}';
const SYM = '[A-Za-z_][A-Za-z0-9_]{0,199}';
const PEER = '[a-z0-9]{1,16}';
const IDENT = /^[A-Za-z_][A-Za-z0-9_]{0,199}$/;
const plain = (max) => (v) => v.length > 0 && v.length <= max && !/[\u0000-\u001f\u007f-\u009f\u2028\u2029]/.test(v) && !/(^|\s)#/.test(v);
const decl = (max) => (v) => plain(max)(v) && !v.includes('@');
const raw = (v) => plain(4096)(v) && !v.trim().split(/\s+/).some((t) => t.startsWith('@'));
const KINDS = [
  [new RegExp(`^var:${HEX}:${SYM}:name$`), (v) => IDENT.test(v)],
  [new RegExp(`^var:${HEX}:${SYM}:type$`), decl(512)],
  [new RegExp(`^fn:${HEX}$`), (v) => IDENT.test(v)],
  [new RegExp(`^proto:${HEX}$`), decl(2048)],
  [new RegExp(`^data:${HEX}:type$`), decl(512)],
  [new RegExp(`^data:${HEX}:name$`), (v) => IDENT.test(v)],
  [new RegExp(`^typedef:${SYM}$`), decl(8192)],
  [new RegExp(`^comment:${HEX}:${HEX}$`), plain(4096)],
  [new RegExp(`^byte:${HEX}$`), (v) => /^[0-9a-f]{2}$/.test(v)],
  [new RegExp(`^raw:${PEER}:[0-9]{1,9}$`), raw],
  [new RegExp(`^rawf:${HEX}:${PEER}:[0-9]{1,9}$`), raw],
  [/^setting:mode$/, (v) => MODES.includes(v)],
];

/** Whether an op from another page may be applied. */
export function validOp(op) {
  if (!op || typeof op !== 'object' || Array.isArray(op) || Object.keys(op).length !== 3) return false;
  const { k, v, c } = op;
  if (typeof k !== 'string' || k.length > 300) return false;
  if (!Array.isArray(c) || c.length !== 2 || !Number.isSafeInteger(c[0]) || c[0] < 1 ||
      typeof c[1] !== 'string' || !/^[a-z0-9]{1,16}$/.test(c[1])) return false;
  const kind = KINDS.find(([re]) => re.test(k));
  if (!kind) return false;
  return v === null || (typeof v === 'string' && kind[1](v));
}

export class Replica {
  constructor(peer) {
    this.peer = peer;
    this.counter = 0;
    this.regs = new Map();
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

  /** A local write; returns the op to send. */
  set(key, value) {
    const op = { k: key, v: value, c: [++this.counter, this.peer] };
    this.apply(op);
    return op;
  }

  /** An op from another page: 'invalid', or whether it changed the register. */
  receive(op) {
    return validOp(op) ? this.apply(op) : 'invalid';
  }

  /** Apply a local or already-checked op; true when it changed the register. */
  apply(op) {
    if (op.c[0] > this.counter) this.counter = op.c[0];
    const cur = this.regs.get(op.k);
    if (cur && !newer(op.c, cur.c)) return false;
    this.regs.set(op.k, { v: op.v, c: [op.c[0], op.c[1]] });
    return true;
  }

  /** Every register as an op, for a page catching up. */
  snapshot() {
    return [...this.regs].map(([k, { v, c }]) => ({ k, v, c: [c[0], c[1]] }));
  }

  /** The registers that hold a value (tombstones left out). */
  live() {
    const out = new Map();
    for (const [k, { v }] of this.regs) if (v !== null) out.set(k, v);
    return out;
  }
}

const hexOf = (big) => '0x' + big.toString(16);

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

function rawRegister(key, rec, me) {
  const own = /^raw:(\d+)$/.exec(key);
  const theirs = /^raw:([a-z0-9]{1,16}):(\d+)$/.exec(key);
  const [peer, n] = own ? [me, own[1]] : theirs ? [theirs[1], theirs[2]] : [null, null];
  if (!peer) return null;
  return rec.func ? `rawf:${rec.func}:${peer}:${n}` : `raw:${peer}:${n}`;
}

/** The session as registers, `key → value` (values are strings). */
export function registersOf(session, me) {
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
        out.set(`data:${rec.addr}:type`, rec.type);
        out.set(`data:${rec.addr}:name`, rec.name);
        break;
      case 'typedef': out.set(`typedef:${rec.tag}`, rec.decl); break;
      case 'comment': out.set(`comment:${rec.func}:${rec.addr}`, rec.text); break;
      case 'raw': {
        const reg = rawRegister(key, rec, me);
        if (reg) out.set(reg, rec.text);
        break;
      }
      default: break;
    }
  }
  for (const [addr, value] of session.bytes) out.set(`byte:${hexOf(addr)}`, value.toString(16).padStart(2, '0'));
  return out;
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
 * Write the registers `keys` (from `replica`) into `session`. A local's name
 * and type, and a global's type and name, are read together, since the
 * session keeps each pair in one record.
 */
export function applyRegisters(session, replica, keys) {
  const done = new Set();
  let bytes = false;
  for (const key of keys) {
    const parts = key.split(':');
    const v = replica.value(key);
    switch (parts[0]) {
      case 'var': {
        const id = `${parts[1]}:${parts[2]}`;
        if (done.has('var:' + id)) break;
        done.add('var:' + id);
        session.setVar(parts[1], parts[2], {
          name: replica.value(`var:${id}:name`), type: replica.value(`var:${id}:type`),
        });
        break;
      }
      case 'fn': session.setFunctionName(parts[1], v); break;
      case 'proto': session.setProto(parts[1], v); break;
      case 'data': {
        if (done.has('data:' + parts[1])) break;
        done.add('data:' + parts[1]);
        session.setData(parts[1], replica.value(`data:${parts[1]}:type`), replica.value(`data:${parts[1]}:name`));
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
 * Redo does the same forwards. A change that exactly takes back the last step
 * (an edit the engine could not apply, undone by the page) removes that step.
 */
export class History {
  constructor(max = 100) {
    this.undoStack = [];
    this.redoStack = [];
    this.max = max;
  }

  get canUndo() { return this.undoStack.length > 0; }

  get canRedo() { return this.redoStack.length > 0; }

  /** One local edit: `[{key, prev, prevClock, op}]` (`op` is the write that replaced `prev`). */
  record(changes) {
    if (!changes.length) return;
    const last = this.undoStack[this.undoStack.length - 1];
    const same = (a, b) => !!a && !!b && a[0] === b[0] && a[1] === b[1];
    if (last && last.length === changes.length &&
        changes.every((ch) => last.some((e) => e.key === ch.key && e.prev === ch.op.v && same(e.c, ch.prevClock)))) {
      this.undoStack.pop();
      return;
    }
    this.undoStack.push(changes.map((ch) => ({ key: ch.key, prev: ch.prev, next: ch.op.v, c: ch.op.c })));
    if (this.undoStack.length > this.max) this.undoStack.shift();
    this.redoStack = [];
  }

  /** Undo the last step on `replica`: `{ops, skipped: [{key, by}]}`, or null when there is none. */
  undo(replica) {
    return this.#step(replica, this.undoStack, this.redoStack, 'prev');
  }

  redo(replica) {
    return this.#step(replica, this.redoStack, this.undoStack, 'next');
  }

  clear() {
    this.undoStack = [];
    this.redoStack = [];
  }

  #step(replica, from, to, field) {
    const step = from.pop();
    if (!step) return null;
    const ops = [];
    const skipped = [];
    const back = [];
    for (const e of step) {
      const cur = replica.clock(e.key);
      if (!cur || cur[0] !== e.c[0] || cur[1] !== e.c[1]) {
        skipped.push({ key: e.key, by: replica.writer(e.key) });
        continue;
      }
      const op = replica.set(e.key, e[field]);
      ops.push(op);
      back.push({ key: e.key, prev: e.prev, next: e.next, c: op.c });
    }
    if (back.length) to.push(back);
    return { ops, skipped };
  }
}
