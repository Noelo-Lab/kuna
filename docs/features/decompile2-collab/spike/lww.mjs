// lww.mjs — the replicated session the proposal describes, as a
// last-writer-wins register map. Every field a student can change is one
// register: a local's name and its type are separate registers, one patched
// byte is one register, a comment is one register. A write carries a Lamport
// clock `[counter, peer]`; the larger clock wins everywhere, so peers that have
// seen the same writes hold the same map whatever order the writes arrived in.
// A deletion is a write of `null` (a tombstone), so it wins or loses like any
// other write. The decompiler effort is a register too (`setting:mode`),
// because the engine's symbols depend on it. Ops from other peers pass
// `validOp` first: a key must have a known shape and a value its kind's shape,
// with no control characters (a newline would split a directive in an
// exported .kuna file) and never a form that makes the engine read a file
// (`@FILE`, `bytes ADDR @FILE`). DOM-free; the spike's tests drive it.
//
// This is the spike, kept as the proposal measured it. The page's own copy,
// integrations/web/decompile2/collab/replica.js, has moved on: its `validOp`
// also bounds the clock counter, carries each register's birth clock, and
// takes its text limits and rules from session.js. Read that one for what a
// page accepts.

const newer = (a, b) => a[0] !== b[0] ? a[0] > b[0] : a[1] > b[1];

const HEX = '0x[0-9a-f]{1,16}';
const IDENT = /^[A-Za-z_][A-Za-z0-9_]{0,127}$/;
const TEXT = (max) => (v) => v.length <= max && !/[\u0000-\u001f\u007f]/.test(v);
const DECL = (v) => TEXT(512)(v) && !v.includes('@');
const KINDS = [
  [new RegExp(`^var:${HEX}:[A-Za-z_][A-Za-z0-9_]{0,63}:name$`), (v) => IDENT.test(v)],
  [new RegExp(`^var:${HEX}:[A-Za-z_][A-Za-z0-9_]{0,63}:type$`), DECL],
  [new RegExp(`^fn:${HEX}$`), (v) => IDENT.test(v)],
  [new RegExp(`^(proto|data):${HEX}$`), DECL],
  [/^typedef:[A-Za-z_][A-Za-z0-9_]{0,63}$/, DECL],
  [new RegExp(`^comment:${HEX}:${HEX}$`), TEXT(1024)],
  [new RegExp(`^byte:${HEX}$`), (v) => /^[0-9a-f]{2}$/.test(v)],
  [/^raw:[a-z0-9]{1,16}:[0-9]{1,9}$/, (v) => TEXT(1024)(v) && !v.split(/\s+/).some((t) => t.startsWith('@'))],
  [/^setting:mode$/, (v) => ['auto', 'fast', 'reliable', 'aggressive'].includes(v)],
];

/** Whether an op from another peer may be applied. */
export function validOp(op) {
  if (!op || typeof op !== 'object' || Object.keys(op).length !== 3) return false;
  const { k, v, c } = op;
  if (typeof k !== 'string' || k.length > 200) return false;
  if (!Array.isArray(c) || c.length !== 2 || !Number.isSafeInteger(c[0]) || c[0] < 1 || typeof c[1] !== 'string' || !/^[a-z0-9]{1,16}$/.test(c[1])) return false;
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

  /** A local write; returns the op to broadcast. */
  set(key, value) {
    const op = { k: key, v: value, c: [++this.counter, this.peer] };
    this.apply(op);
    return op;
  }

  /** Apply an op from another peer: 'invalid', or whether it changed the register. */
  receive(op) {
    return validOp(op) ? this.apply(op) : 'invalid';
  }

  /** Apply a local or trusted op; true when it changed the register. */
  apply(op) {
    if (op.c[0] > this.counter) this.counter = op.c[0];
    const cur = this.regs.get(op.k);
    if (cur && !newer(op.c, cur.c)) return false;
    this.regs.set(op.k, { v: op.v, c: op.c });
    return true;
  }

  /** Everything, as ops a newcomer applies to catch up. */
  snapshot() {
    return [...this.regs].map(([k, { v, c }]) => ({ k, v, c }));
  }

  /** Undo my last write to `key` by writing `previous` back, unless someone else wrote since. */
  undo(key, previous, myClock) {
    const cur = this.regs.get(key);
    if (!cur || cur.c[0] !== myClock[0] || cur.c[1] !== myClock[1]) return null;
    return this.set(key, previous);
  }

  /**
   * The session this map describes, as `--assert` directives in a canonical
   * order (by kind, then key), so every peer sends the engine the same list.
   */
  directives() {
    const vars = new Map();
    const out = [];
    for (const [key, { v }] of [...this.regs].sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0))) {
      if (v === null) continue;
      const [kind, ...rest] = key.split(':');
      if (kind === 'setting') continue;
      if (kind === 'var') {
        const [func, sym, field] = rest;
        const id = `${func}:${sym}`;
        const rec = vars.get(id) || { func, sym };
        rec[field] = v;
        vars.set(id, rec);
      } else if (kind === 'fn') out.push(`function ${rest[0]}=${v}`);
      else if (kind === 'comment') out.push(`comment ${rest[1]} ${v}`);
      else if (kind === 'byte') out.push(`bytes ${rest[0]} ${v}`);
      else if (kind === 'raw') out.push(v);
    }
    for (const rec of vars.values()) {
      if (rec.type) out.push(`type ${rec.sym} ${rec.type} ${rec.name || rec.sym}`);
      else if (rec.name) out.push(`name ${rec.sym} ${rec.name}`);
    }
    return out;
  }
}
