// session.js — the student's edits, held as the `--assert` directives the
// engine replays on every request (each wasm call is a fresh process, so the
// session IS the directive list). Records are keyed by what they describe,
// so a rename and a retype of one local merge into one pinned `type`
// directive, and writing a byte back to its original value removes the patch.
// The same records export as a `.kuna` file the native CLI replays verbatim:
//   kuna decompile <binary> <function> --assert @<binary>.kuna
// DOM-free.
import { cDeclare, parseParam } from './ctype.js';

const ORDER = ['typedef', 'data', 'fn', 'proto', 'bytes', 'raw', 'var', 'comment'];
const UNDO_MAX = 100;

/** Strip a `#` comment the way the CLI's `@FILE` reader does (` #`, or a leading `#`). */
export function stripComment(line) {
  const at = line.indexOf(' #');
  if (at >= 0) return line.slice(0, at).trim();
  const t = line.trim();
  return t.startsWith('#') ? '' : t;
}

/** POSIX-shell quoting for one argument. */
export function shellQuote(s) {
  const text = String(s);
  if (/^[A-Za-z0-9_./@:=+,%-]+$/.test(text)) return text;
  return `'${text.replace(/'/g, `'\\''`)}'`;
}

/** The native command that replays a session file against one function. */
export function cliCommand(fileName, target, sessionFile = `${fileName}.kuna`) {
  return `kuna decompile ${shellQuote(fileName)} ${shellQuote(target)} --assert @${shellQuote(sessionFile)}`;
}

const hex = (big) => '0x' + big.toString(16);

/** `name S N`, or `type S <T N>` (the name pinned so a retype cannot renumber it). */
function varDirective(rec, qualifier) {
  const sym = qualifier ? `${qualifier}::${rec.sym}` : rec.sym;
  if (rec.type) return `type ${sym} ${cDeclare(rec.type, rec.name || rec.sym)}`;
  return `name ${sym} ${rec.name}`;
}

function recordDirective(rec, qualifier) {
  switch (rec.kind) {
    case 'var': return varDirective(rec, qualifier);
    case 'comment': return `comment ${qualifier ? `${qualifier}::` : ''}${rec.addr} ${rec.text}`;
    case 'proto': return `prototype ${rec.addr} ${rec.decl}`;
    case 'fn': return `function ${rec.addr}=${rec.name}`;
    case 'data': return `data ${rec.addr} ${cDeclare(rec.type, rec.name)}`;
    case 'typedef': return `typedef ${rec.decl}`;
    case 'raw': return rec.text;
    default: throw new Error(`unknown record kind ${rec.kind}`);
  }
}

export class Session {
  constructor() {
    this.records = new Map();
    this.bytes = new Map();
    this.outcomes = new Map();
    this.sent = new Map();
    this.undoStack = [];
    this.redoStack = [];
    this.rawSeq = 0;
  }

  get size() {
    return this.records.size + this.byteRuns().length;
  }

  // ── edits ────────────────────────────────────────────────────────────────

  /** Rename and/or retype a local (`undefined` leaves a field, `null` clears it). */
  setVar(func, sym, { name, type } = {}) {
    const key = `var:${func}:${sym}`;
    const rec = { kind: 'var', func, sym, name: null, type: null, ...(this.records.get(key) || {}) };
    if (name !== undefined) rec.name = name && name !== sym ? name : null;
    if (type !== undefined) rec.type = type || null;
    if (!rec.name && !rec.type) this.records.delete(key);
    else this.records.set(key, rec);
    return key;
  }

  setProto(addr, decl) {
    return this.#set(`proto:${addr}`, decl ? { kind: 'proto', addr, decl } : null);
  }

  setFunctionName(addr, name) {
    return this.#set(`fn:${addr}`, name ? { kind: 'fn', addr, name } : null);
  }

  setData(addr, type, name) {
    return this.#set(`data:${addr}`, type && name ? { kind: 'data', addr, type, name } : null);
  }

  setTypedef(tag, decl) {
    return this.#set(`typedef:${tag}`, decl ? { kind: 'typedef', tag, decl } : null);
  }

  setComment(func, addr, text) {
    return this.#set(`comment:${func}:${addr}`, text ? { kind: 'comment', func, addr, text } : null);
  }

  addRaw(text) {
    const key = `raw:${++this.rawSeq}`;
    this.records.set(key, { kind: 'raw', text: text.trim() });
    return key;
  }

  /** Patch one byte; writing the `original` back removes the patch. */
  setByte(addr, value, original) {
    const a = BigInt(addr);
    if (value === original) this.bytes.delete(a);
    else this.bytes.set(a, value & 0xff);
  }

  remove(key) {
    if (key.startsWith('bytes:')) {
      const run = this.byteRuns().find((r) => `bytes:${hex(r.addr)}` === key);
      for (let i = 0; i < (run?.values.length || 0); i++) this.bytes.delete(run.addr + BigInt(i));
      return;
    }
    this.records.delete(key);
    this.outcomes.delete(key);
  }

  clear() {
    this.records.clear();
    this.bytes.clear();
    this.outcomes.clear();
  }

  #set(key, rec) {
    if (rec) this.records.set(key, rec);
    else this.records.delete(key);
    return key;
  }

  // ── queries ──────────────────────────────────────────────────────────────

  /** The record whose current display name in `func` is `displayName`. */
  varRecord(func, displayName) {
    for (const rec of this.records.values()) {
      if (rec.kind === 'var' && rec.func === func && (rec.name || rec.sym) === displayName) return rec;
    }
    return null;
  }

  /** The symbol the engine knows a displayed local by. */
  symbolOf(func, displayName) {
    return this.varRecord(func, displayName)?.sym ?? displayName;
  }

  functionName(addr) {
    return this.records.get(`fn:${addr}`)?.name;
  }

  proto(addr) {
    return this.records.get(`proto:${addr}`)?.decl;
  }

  comment(func, addr) {
    return this.records.get(`comment:${func}:${addr}`)?.text;
  }

  typedefTags() {
    return [...this.records.values()].filter((r) => r.kind === 'typedef').map((r) => r.tag);
  }

  /** Patched bytes as contiguous runs `[{addr: BigInt, values: number[]}]`. */
  byteRuns() {
    const addrs = [...this.bytes.keys()].sort((a, b) => (a < b ? -1 : a > b ? 1 : 0));
    const runs = [];
    for (const a of addrs) {
      const last = runs[runs.length - 1];
      if (last && last.addr + BigInt(last.values.length) === a) last.values.push(this.bytes.get(a));
      else runs.push({ addr: a, values: [this.bytes.get(a)] });
    }
    return runs;
  }

  #bytesDirectives() {
    return this.byteRuns().map((r) => ({
      key: `bytes:${hex(r.addr)}`,
      text: `bytes ${hex(r.addr)} ${r.values.map((v) => v.toString(16).padStart(2, '0')).join('')}`,
    }));
  }

  /** Every directive in replay order; `qualify(rec)` names var/comment records' function. */
  #directives({ func = null, qualify = null } = {}) {
    const groups = new Map(ORDER.map((k) => [k, []]));
    for (const [key, rec] of this.records) {
      if (rec.kind === 'var' || rec.kind === 'comment') {
        if (qualify) groups.get(rec.kind).push({ key, text: recordDirective(rec, qualify(rec.func)) });
        else if (rec.func === func) groups.get(rec.kind).push({ key, text: recordDirective(rec, null) });
      } else if (rec.kind === 'raw') {
        if (qualify || isGlobalRaw(rec.text)) groups.get('raw').push({ key, text: rec.text });
      } else {
        groups.get(rec.kind).push({ key, text: recordDirective(rec, null) });
      }
    }
    groups.set('bytes', this.#bytesDirectives());
    const out = ORDER.flatMap((k) => groups.get(k));
    for (const d of out) this.sent.set(d.text, d.key);
    return out;
  }

  /** Directives for one `inspect`: every global one, plus this function's, unqualified. */
  assertionsFor(addrHex) {
    return this.#directives({ func: addrHex }).map((d) => d.text);
  }

  /** Directives that describe no single function body (for `list`). */
  globalAssertions() {
    return this.#directives({ func: null }).map((d) => d.text);
  }

  /** Every directive, function-scoped ones qualified with `nameOf(addrHex)` (project, export). */
  allAssertions(nameOf) {
    return this.#directives({ qualify: nameOf }).map((d) => d.text);
  }

  /** Attach the engine's `assertions[]` rows to the records that produced them. */
  recordOutcomes(rows) {
    for (const row of rows || []) {
      const key = this.sent.get((row.directive || '').trim());
      if (key) this.outcomes.set(key, { status: row.status, detail: row.detail || null, fatal: !!row.fatal });
    }
  }

  statusOf(key) {
    return this.outcomes.get(key)?.status || 'pending';
  }

  /** The session as rail rows: `[{key, kind, text, status, detail}]`, in replay order. */
  entries(nameOf) {
    return this.#directives({ qualify: nameOf }).map((d) => ({
      key: d.key,
      kind: d.key.split(':')[0],
      text: d.text,
      status: this.statusOf(d.key),
      detail: this.outcomes.get(d.key)?.detail || null,
      fatal: this.outcomes.get(d.key)?.fatal || false,
    }));
  }

  // ── undo ─────────────────────────────────────────────────────────────────

  snapshot() {
    return {
      records: [...this.records].map(([k, v]) => [k, { ...v }]),
      bytes: [...this.bytes],
      rawSeq: this.rawSeq,
    };
  }

  restore(snap) {
    this.records = new Map(snap.records.map(([k, v]) => [k, { ...v }]));
    this.bytes = new Map(snap.bytes);
    this.rawSeq = snap.rawSeq;
  }

  /** Record `snap` (the state before a successful edit) as one undo step. */
  pushUndo(snap) {
    this.undoStack.push(snap);
    if (this.undoStack.length > UNDO_MAX) this.undoStack.shift();
    this.redoStack = [];
  }

  get canUndo() { return this.undoStack.length > 0; }
  get canRedo() { return this.redoStack.length > 0; }

  undo() {
    if (!this.canUndo) return false;
    this.redoStack.push(this.snapshot());
    this.restore(this.undoStack.pop());
    return true;
  }

  redo() {
    if (!this.canRedo) return false;
    this.undoStack.push(this.snapshot());
    this.restore(this.redoStack.pop());
    return true;
  }

  /** The records as plain JSON (what the page persists per binary). */
  toJSON() {
    return {
      v: 1,
      records: [...this.records].map(([k, v]) => [k, { ...v }]),
      bytes: [...this.bytes].map(([a, v]) => [hex(a), v]),
      rawSeq: this.rawSeq,
    };
  }

  static fromJSON(doc) {
    const s = new Session();
    if (!doc || doc.v !== 1) return s;
    for (const [k, v] of doc.records || []) if (typeof k === 'string' && v?.kind) s.records.set(k, { ...v });
    for (const [a, v] of doc.bytes || []) s.bytes.set(BigInt(a), v & 0xff);
    s.rawSeq = Number(doc.rawSeq) || 0;
    return s;
  }

  // ── the .kuna file ───────────────────────────────────────────────────────

  /** The session as an `@FILE` for `kuna ... --assert @file`. */
  toFileText({ binary = 'binary', hash = '', time = new Date(), target = 'main', nameOf = (a) => a } = {}) {
    const file = `${binary}.kuna`;
    const head = [
      '# kuna study-view session (one --assert directive per line)',
      `# binary: ${binary}`,
      hash ? `# hash: ${hash}` : null,
      `# saved: ${time.toISOString()}`,
      `# replay: ${cliCommand(binary, target, file)}`,
      `#     or: kuna decompile-all ${shellQuote(binary)} --assert @${shellQuote(file)}`,
    ].filter(Boolean);
    return [...head, ...this.allAssertions(nameOf)].join('\n') + '\n';
  }

  /**
   * Read a `.kuna` file back into records. `resolveFunc(name)` maps a
   * qualifier to a function address; `bindTo` receives unqualified
   * function-scoped directives. Returns `{added, raw, unbound}` counts.
   */
  importText(text, { resolveFunc = () => null, bindTo = null } = {}) {
    let added = 0, raw = 0, unbound = 0;
    for (const line of text.split(/\r?\n/)) {
      const d = stripComment(line);
      if (!d) continue;
      const kind = this.#importOne(d, resolveFunc, bindTo);
      if (kind === 'raw') raw++;
      if (kind === 'unbound') unbound++;
      added++;
    }
    return { added, raw, unbound };
  }

  #importOne(d, resolveFunc, bindTo) {
    const [keyword] = d.split(/\s+/, 1);
    const rest = d.slice(keyword.length).trim();
    const scope = (tok) => {
      const at = tok.lastIndexOf('::');
      if (at <= 0) return { func: bindTo, operand: tok, bound: !!bindTo };
      const func = resolveFunc(tok.slice(0, at));
      return { func, operand: tok.slice(at + 2), bound: !!func };
    };
    let m;
    if (keyword === 'function' && (m = /^(0x[0-9a-f]+|[0-9a-f]+)=(\S+)$/i.exec(rest))) {
      this.setFunctionName('0x' + BigInt('0x' + m[1].replace(/^0x/i, '')).toString(16), m[2]);
      return 'fn';
    }
    if (keyword === 'prototype' && (m = /^(0x[0-9a-f]+)\s+(.+)$/i.exec(rest))) {
      this.setProto('0x' + BigInt(m[1]).toString(16), m[2]);
      return 'proto';
    }
    if (keyword === 'typedef') {
      const tag = /\b(?:struct|union|enum)\s+([A-Za-z_]\w*)/.exec(rest)?.[1] || /([A-Za-z_]\w*)\s*;?\s*$/.exec(rest)?.[1] || rest;
      this.setTypedef(tag, rest);
      return 'typedef';
    }
    if (keyword === 'bytes' && (m = /^(0x[0-9a-f]+|[0-9a-f]+)\s+(?:0x)?([0-9a-f]+)$/i.exec(rest)) && m[2].length % 2 === 0) {
      const base = BigInt('0x' + m[1].replace(/^0x/i, ''));
      for (let i = 0; i < m[2].length / 2; i++) this.bytes.set(base + BigInt(i), parseInt(m[2].slice(i * 2, i * 2 + 2), 16));
      return 'bytes';
    }
    if (keyword === 'name' && (m = /^(\S+)\s+([A-Za-z_]\w*)$/.exec(rest))) {
      const s = scope(m[1]);
      if (s.bound) { this.setVar(s.func, s.operand, { name: m[2] }); return 'var'; }
      this.addRaw(d);
      return m[1].includes('::') ? 'raw' : 'unbound';
    }
    if (keyword === 'type' && (m = /^(\S+)\s+(.+)$/.exec(rest))) {
      const s = scope(m[1]);
      if (s.bound) {
        const decl = parseParam(m[2]);
        this.setVar(s.func, s.operand, { type: decl.type, name: decl.name || undefined });
        return 'var';
      }
      this.addRaw(d);
      return m[1].includes('::') ? 'raw' : 'unbound';
    }
    if (keyword === 'comment' && (m = /^(\S+)\s+(.+)$/.exec(rest))) {
      const s = scope(m[1]);
      if (s.bound && /^(0x)?[0-9a-f]+$/i.test(s.operand)) {
        this.setComment(s.func, '0x' + BigInt('0x' + s.operand.replace(/^0x/i, '')).toString(16), m[2]);
        return 'comment';
      }
    }
    if (keyword === 'data' && (m = /^(0x[0-9a-f]+|[0-9a-f]+)\s+(.+)$/i.exec(rest))) {
      const decl = parseParam(m[2]);
      if (decl.name) {
        this.setData('0x' + BigInt('0x' + m[1].replace(/^0x/i, '')).toString(16), decl.type, decl.name);
        return 'data';
      }
    }
    this.addRaw(d);
    return 'raw';
  }
}

/** Does a raw directive hold for the whole program (safe to send with any request)? */
export function isGlobalRaw(text) {
  const [keyword, first = ''] = text.trim().split(/\s+/, 2);
  if (['name', 'type', 'comment', 'flow'].includes(keyword)) return false;
  if (['param', 'return'].includes(keyword)) return first.includes('::');
  return true;
}
