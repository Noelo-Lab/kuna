// data-view.js — the Data tab: a window of the program's data as rows, one per
// labelled item (address, name, type, value) and per run of unlabelled bytes
// (text, a pointer, zeros, hex), each value decoded from the window's bytes by
// the item's layout. The engine's `data` document gives the items it knows;
// the declarations the page knows (the decompiler's own `dat_` globals) fill
// the gaps. Bytes a section's file data does not reach start as zeros.
// DOM-free: every engine string reaches HTML through escapeHtml.
import { escapeHtml } from '../assets/js/highlight-c.js';
import { bare } from './addr.js';
import { normalizeType } from './ctype.js';

const hx = (a) => '0x' + a.toString(16);
const MAX_TEXT = 200;
const SUMMARY_ITEMS = 8;
export const MAX_CHILDREN = 256;
const LINE = 16;
/** What bytes no file data backs read as: `.bss`, a section's zero-filled tail. */
export const UNINIT = 'uninitialized';
/** Its tooltip. */
export const UNINIT_WHY = 'The file holds no bytes for this. It starts as zeros, unless the loader fills it in (a library variable such as stderr).';

const SCALARS = new Map([
  ['char', ['char', 1, true]], ['signed char', ['char', 1, true]], ['unsigned char', ['char', 1, false]],
  ['short', ['int', 2, true]], ['unsigned short', ['int', 2, false]], ['int', ['int', 4, true]],
  ['unsigned int', ['int', 4, false]], ['unsigned', ['int', 4, false]], ['long long', ['int', 8, true]],
  ['unsigned long long', ['int', 8, false]], ['float', ['float', 4]], ['double', ['float', 8]],
  ['_Bool', ['bool', 1, false]], ['bool', ['bool', 1, false]], ['wchar_t', ['int', 4, true]],
  ['int8_t', ['int', 1, true]], ['int16_t', ['int', 2, true]], ['int32_t', ['int', 4, true]], ['int64_t', ['int', 8, true]],
  ['uint8_t', ['int', 1, false]], ['uint16_t', ['int', 2, false]], ['uint32_t', ['int', 4, false]], ['uint64_t', ['int', 8, false]],
  ['byte', ['int', 1, false]], ['word', ['int', 2, false]], ['dword', ['int', 4, false]], ['qword', ['int', 8, false]],
]);

/** The unsigned integer of `size` bytes at `off` (BigInt), or null past the end. */
export function readUint(bytes, off, size, little = true) {
  if (off < 0 || off + size > bytes.length) return null;
  let v = 0n;
  for (let i = 0; i < size; i++) v = (v << 8n) | BigInt(bytes[off + (little ? size - 1 - i : i)]);
  return v;
}

/** The signed integer of `size` bytes at `off` (BigInt), or null past the end. */
export function readInt(bytes, off, size, little = true) {
  const v = readUint(bytes, off, size, little);
  if (v === null) return null;
  const top = 1n << BigInt(size * 8 - 1);
  return v & top ? v - (top << 1n) : v;
}

const printable = (b) => (b >= 0x20 && b < 0x7f) || b === 0x09 || b === 0x0a || b === 0x0d;
const textByte = (b) => printable(b) || b === 0x1b;

/**
 * The NUL-terminated text at `off` (within `end`), as `{text, size}` with the
 * NUL, or null: printable characters, line breaks, tabs and the escape
 * character that starts a terminal colour code.
 */
export function cString(bytes, off, end = bytes.length, min = 1) {
  let k = off;
  while (k < end && k < bytes.length && textByte(bytes[k])) k++;
  if (k >= end || k >= bytes.length || bytes[k] !== 0 || k - off < min) return null;
  return { text: String.fromCharCode(...bytes.subarray(off, k)), size: k - off + 1 };
}

/** Text as a C string literal, cut at `max` characters. */
export function quote(text, max = MAX_TEXT) {
  const cut = text.length > max;
  const body = (cut ? text.slice(0, max) : text)
    .replace(/\\/g, '\\\\').replace(/"/g, '\\"').replace(/\n/g, '\\n').replace(/\r/g, '\\r').replace(/\t/g, '\\t')
    .replace(/[\x00-\x1f\x7f]/g, (c) => `\\x${c.charCodeAt(0).toString(16).padStart(2, '0')}`);
  return `"${body}"${cut ? '…' : ''}`;
}

/** An integer for reading: decimal, then hex when they differ (`1000 · 0x3e8`, `-1 · 0xffffffff`). */
export function intText(value, size) {
  const raw = value < 0n ? value & ((1n << BigInt(size * 8)) - 1n) : value;
  return value >= 0n && value < 10n ? value.toString() : `${value} · ${hx(raw)}`;
}

/** The type of a C declaration without its name (`int (*op)(int)` → `int (*)(int)`). */
export function declType(declaration, name) {
  const d = (declaration || '').trim().replace(/;$/, '');
  if (!name) return d;
  const re = new RegExp(`(?<![\\w$])${name.replace(/[\\^$.*+?()[\]{}|]/g, '\\$&')}(?![\\w$])`);
  return normalizeType(d.replace(re, '').replace(/\(\s*\*\s*\)/, '(*)').trim());
}

/**
 * A C declaration the page knows (`int dat_4040[]`, `char *dat_4070`,
 * `int (*dat_4078)(unsigned int)`) as a layout `{kind, size, type, signed,
 * to, elem, count, open}` (`open`: an array of unknown length); null when the
 * type is not one of C's scalars, a pointer or an array of them. `known` maps
 * other type names to their sizes.
 */
export function layoutOfDecl(declaration, name, { pointerSize = 8, longSize = 8, known = new Map() } = {}) {
  const type = declType(declaration, name);
  return layoutOfType(type, { pointerSize, longSize, known });
}

function layoutOfType(type, ctx) {
  const t = normalizeType(type).replace(/\b(const|volatile|static|extern)\s+/g, '').trim();
  if (!t) return null;
  if (/\(\*\)\s*\(/.test(t)) return { kind: 'pointer', size: ctx.pointerSize, type: t, to: { kind: 'code', size: 1, type: 'code' } };
  const arr = /^(.*?)\[(\d*)\]$/.exec(t);
  if (arr) {
    const elem = layoutOfType(arr[1], ctx);
    if (!elem || !elem.size) return null;
    if (arr[2] === '') return { kind: 'array', size: elem.size, type: t, elem, count: null, open: true };
    const count = Number(arr[2]);
    return { kind: 'array', size: elem.size * count, type: t, elem, count };
  }
  if (/\*$/.test(t)) {
    const to = layoutOfType(t.replace(/\s*\*$/, ''), ctx) || { kind: 'unknown', size: 1, type: t.replace(/\s*\*$/, '') };
    return { kind: 'pointer', size: ctx.pointerSize, type: t, to };
  }
  const base = t.replace(/^signed$/, 'int').replace(/^signed (?!char)/, '').replace(/^unsigned$/, 'unsigned int')
    .replace(/\b(short|long) int$/, '$1').trim();
  if (/^(unsigned )?long$/.test(base)) return { kind: 'int', size: ctx.longSize, type: t, signed: !base.startsWith('unsigned') };
  const s = SCALARS.get(base);
  if (s) return { kind: s[0], size: s[1], type: t, ...(s[2] === undefined ? {} : { signed: s[2] }) };
  const undef = /^undefined(\d*)$/.exec(base);
  if (undef) return { kind: 'unknown', size: Number(undef[1] || 1), type: t };
  const sized = /^(u?)int(\d+)$/.exec(base);
  if (sized && [1, 2, 4, 8].includes(Number(sized[2]))) return { kind: 'int', size: Number(sized[2]), type: t, signed: !sized[1] };
  const named = ctx.known.get(base.replace(/^(struct|union|enum) /, ''));
  return named ? { kind: 'unknown', size: named, type: t } : null;
}

/** A section's kind in words. */
export function sectionWords(section) {
  if (!section) return 'memory outside any section';
  if (section.executable) return 'code';
  if (section.file_offset === null || section.file_offset === undefined) return 'data that starts as zeros';
  return section.writable ? 'data the program can change' : 'data that never changes';
}

const secAddr = (s) => BigInt(s.address_hex ?? s.address);

/** The sections the Data tab can show (none of them code, none empty), in address order. */
export function dataSections(sections) {
  return (sections || []).filter((s) => !s.executable && s.size > 0)
    .sort((a, b) => (secAddr(a) < secAddr(b) ? -1 : secAddr(a) > secAddr(b) ? 1 : 0));
}

/** A byte count in words: `12 bytes`, `4.5 KB`, `2.1 MB`. */
export function sizeWords(n) {
  if (n < 1024) return `${n} byte${n === 1 ? '' : 's'}`;
  if (n < 1024 * 1024) return `${(n / 1024).toFixed(n < 10240 ? 1 : 0)} KB`;
  return `${(n / 1048576).toFixed(1)} MB`;
}

/** A menu of `sections` with `current` chosen, for going straight to one. */
export function sectionPicker(sections, current) {
  const at = current ? secAddr(current) : null;
  const opts = sections.map((s) => {
    const here = at !== null && secAddr(s) === at;
    return `<option value="${escapeHtml(hx(secAddr(s)))}"${here ? ' selected' : ''}>${escapeHtml(s.name || bare(hx(secAddr(s))))}</option>`;
  }).join('');
  return `<select class="dv-pick" data-act="dv-section" aria-label="Section" title="Go to another section">${opts}</select>`;
}

function hexBytes(bytes, off, n) {
  const parts = [];
  for (let k = Math.max(0, -off); k < n && off + k < bytes.length; k++) parts.push(bytes[off + k].toString(16).padStart(2, '0'));
  return parts.join(' ');
}

/**
 * The window as a model: `{start, end, little, pointerSize, section, items,
 * rows}`. `doc` is the engine's `data` document; `known` the page's own
 * `{name, address_hex, declaration, size}` globals; `fnAt(hex)` the page's
 * name for a function entry (or null).
 */
export function dataModel(doc, { known = [], fnAt = () => null, longSize = 8, knownTypes = new Map() } = {}) {
  const start = BigInt(doc.start_hex ?? doc.start);
  const span = BigInt(doc.span ?? doc.size ?? 0);
  const end = start + span;
  const bytes = parseBytes(doc.bytes);
  const little = (doc.target?.endian || 'little') !== 'big';
  const pointerSize = doc.pointer_size || (doc.target?.bits ? doc.target.bits / 8 : 8);
  const section = doc.section || null;
  const secStart = section ? BigInt(section.address_hex ?? section.address) : start;
  const fileEnd = !section ? null
    : section.file_offset === null || section.file_offset === undefined ? secStart
      : secStart + BigInt(Math.min(section.file_size ?? section.size, section.size));
  const ctx = { pointerSize, longSize, known: knownTypes };
  const byAddr = new Map();
  for (const it of doc.items || []) {
    const addr = BigInt(it.address_hex ?? it.address);
    const prev = byAddr.get(addr);
    const item = { addr, name: it.name, size: Number(it.size) || 0, type: it.type, declaration: it.declaration, layout: it.layout, aliases: [], from: 'engine' };
    if (!prev) byAddr.set(addr, item);
    else if (rank(item) > rank(prev)) { item.aliases = [...prev.aliases, prev.name]; byAddr.set(addr, item); }
    else prev.aliases.push(item.name);
  }
  for (const g of known) {
    const addr = BigInt(g.address_hex ?? g.address);
    if (addr < start || addr >= end) continue;
    const layout = layoutOfDecl(g.declaration, g.name, ctx);
    if (!layout) continue;
    const prev = byAddr.get(addr);
    if (prev && (prev.name === g.name || prev.layout?.kind !== 'unknown')) {
      if (!prev.aliases.includes(g.name) && prev.name !== g.name) prev.aliases.push(g.name);
      continue;
    }
    const item = { addr, name: g.name, size: layout.size, type: layout.type, declaration: g.declaration, layout, aliases: [], from: 'page' };
    if (prev) { item.name = prev.name; item.aliases = prev.aliases; item.size = Math.max(layout.size, prev.size); item.declaration = prev.declaration; }
    byAddr.set(addr, item);
  }
  const items = [...byAddr.values()].sort((a, b) => (a.addr < b.addr ? -1 : 1));
  items.forEach((it, i) => {
    if (it.layout?.open) {
      const next = items[i + 1]?.addr ?? end;
      const room = Number(next - it.addr) / it.layout.elem.size;
      const count = Math.max(1, Math.min(MAX_CHILDREN, Math.floor(room)));
      it.layout = { ...it.layout, count, size: count * it.layout.elem.size };
      it.size = it.layout.size;
    }
  });
  const targets = new Map((doc.targets || []).map((t) => [BigInt(t.at_hex ?? t.at), t]));
  const model = { start, end, bytes, little, pointerSize, section, fileEnd, items, targets, fnAt, rows: [] };
  model.rows = topRows(model);
  return model;
}

function rank(item) {
  const k = item.layout?.kind;
  return (k && k !== 'unknown' ? 4 : 0) + (item.size > 1 ? 2 : 0) + (/^(s_|dat_|_)/.test(item.name) ? 0 : 1);
}

function parseBytes(text) {
  const t = text || '';
  const out = new Uint8Array(t.length >> 1);
  for (let i = 0; i < out.length; i++) out[i] = parseInt(t.slice(i * 2, i * 2 + 2), 16);
  return out;
}

const offOf = (m, addr) => Number(addr - m.start);
export const isUninit = (m, addr) => m.fileEnd !== null && addr >= m.fileEnd;

function topRows(m) {
  const rows = [];
  let pos = m.start;
  const covered = [];
  for (const it of m.items) {
    const end = it.addr + BigInt(Math.max(it.size, 1));
    if (end <= m.start || it.addr >= m.end) continue;
    if (it.addr > pos) rows.push(...gapRows(m, pos, it.addr));
    if (it.addr < pos && covered.length) { rows.push(itemRow(m, it, true)); continue; }
    rows.push(itemRow(m, it));
    covered.push(it);
    if (end > pos) pos = end;
  }
  if (pos < m.end) rows.push(...gapRows(m, pos, m.end));
  return rows;
}

/** A labelled item's row: its value and, for arrays, structs and long unknowns, how many children it opens to. */
function itemRow(m, it, overlap = false) {
  const layout = it.layout || { kind: 'unknown', size: it.size, type: it.type };
  const row = {
    kind: 'item', addr: it.addr, hex: hx(it.addr), size: it.size, name: it.name, aliases: it.aliases,
    type: typeLabel(layout, it), layout, overlap, item: it,
  };
  Object.assign(row, valueOf(m, it.addr, layout, it.size));
  return row;
}

function typeLabel(layout, it) {
  if (layout.kind === 'unknown' || (layout.kind === 'array' && layout.elem?.kind === 'unknown')) {
    return `${it.size} byte${it.size === 1 ? '' : 's'}`;
  }
  return layout.type || it.type || '';
}

/** `{value, cls, link, count}` of `layout` at `addr`: what the row shows and how many children it has. */
export function valueOf(m, addr, layout, size = layout.size) {
  const off = offOf(m, addr);
  if (isUninit(m, addr)) return { value: UNINIT, cls: 'dv-uninit', count: 0 };
  if ((off < 0 || off >= m.bytes.length) && layout.kind !== 'array') {
    return { value: '…', cls: 'dv-muted', count: layout.kind === 'struct' || layout.kind === 'union' ? childCount(layout, size) : 0 };
  }
  const k = layout.kind;
  if (k === 'int' || k === 'enum' || (k === 'char' && size > 1)) {
    const v = layout.signed === false ? readUint(m.bytes, off, layout.size, m.little) : readInt(m.bytes, off, layout.size, m.little);
    if (v === null) return { value: hexBytes(m.bytes, off, size), cls: 'dv-bytes', count: 0 };
    return { value: intText(v, layout.size), cls: 'dv-num', count: 0 };
  }
  if (k === 'bool') {
    const v = readUint(m.bytes, off, layout.size, m.little);
    return { value: v === null ? '' : v === 0n ? 'false' : v === 1n ? 'true' : `true (${v})`, cls: 'dv-num', count: 0 };
  }
  if (k === 'char') {
    const b = m.bytes[off];
    return { value: printable(b) && b >= 0x20 ? `'${String.fromCharCode(b).replace(/['\\]/g, '\\$&')}' · ${b}` : `${b}`, cls: 'dv-num', count: 0 };
  }
  if (k === 'float') {
    if (off + layout.size > m.bytes.length) return { value: '', count: 0 };
    const view = new DataView(m.bytes.buffer, m.bytes.byteOffset + off, layout.size);
    const v = layout.size === 4 ? view.getFloat32(0, m.little) : layout.size === 8 ? view.getFloat64(0, m.little) : null;
    return { value: v === null ? hexBytes(m.bytes, off, layout.size) : String(v), cls: 'dv-num', count: 0 };
  }
  if (k === 'pointer') return pointerValue(m, addr, layout);
  if (k === 'array') return arrayValue(m, addr, layout, size);
  if (k === 'struct' || k === 'union') return structValue(m, addr, layout);
  return unknownValue(m, addr, size);
}

/** What a pointer word at `addr` points at: a function, an item (with its element or field), a string, NULL. */
export function pointerValue(m, addr, layout = { size: m.pointerSize }) {
  const off = offOf(m, addr);
  const v = readUint(m.bytes, off, layout.size || m.pointerSize, m.little);
  if (v === null) return { value: '', count: 0 };
  if (v === 0n) return { value: 'NULL', cls: 'dv-num', count: 0 };
  const t = m.targets.get(addr);
  const to = hx(v);
  const fn = m.fnAt(to);
  if (fn) return { value: `→ ${fn}`, cls: 'dv-ptr', link: { hex: to, kind: 'fn', label: fn }, count: 0 };
  if (t) {
    const label = t.text != null ? quote(t.text, 60) : targetLabel(m, v, t);
    if (label) return { value: `→ ${label}`, cls: t.text != null ? 'dv-str' : 'dv-ptr', link: { hex: to, kind: 'data', label }, count: 0 };
    return { value: `→ ${bare(to)}`, cls: 'dv-ptr', link: { hex: to, kind: 'data', label: bare(to) }, count: 0 };
  }
  return { value: intText(v, layout.size || m.pointerSize), cls: 'dv-num', count: 0 };
}

/** Whether the word at `addr` of an untyped item reads as a pointer: to a function, a named item or a string. */
function pointsSomewhere(m, addr) {
  const t = m.targets.get(addr);
  if (!t) return false;
  const to = BigInt(t.to_hex ?? t.to);
  return !!(m.fnAt(hx(to)) || t.name || t.text != null || m.items.some((x) => x.addr === to));
}

/** `primes[3]`, `origin.label`, `name + 0xc` or `name` for an address inside a known item. */
export function targetLabel(m, value, t = null) {
  const it = m.items.find((x) => x.addr <= value && value < x.addr + BigInt(Math.max(x.size, 1)));
  const name = it?.name ?? t?.name;
  if (!name) return null;
  const base = it?.addr ?? value - BigInt(t?.offset ?? 0);
  const delta = Number(value - base);
  if (!delta) return name;
  const l = it?.layout;
  if (l?.kind === 'array' && l.elem?.size && l.elem.kind !== 'unknown' && delta % l.elem.size === 0) return `${name}[${delta / l.elem.size}]`;
  const field = (l?.fields || []).find((f) => f.offset === delta);
  if (field) return `${name}.${field.name}`;
  return `${name} + ${hx(BigInt(delta))}`;
}

function childCount(layout, size) {
  if (layout.kind === 'array') return layout.count ?? Math.floor(size / (layout.elem?.size || 1));
  return (layout.fields || []).length;
}

/** The first element of an array at `addr` the window holds, and how many of its elements it holds from there. */
function elemSpan(m, addr, elem, count) {
  const size = BigInt(elem.size || 1);
  const first = addr >= m.start ? 0 : Math.min(count, Number((m.start - addr + size - 1n) / size));
  const last = Math.min(count, Number((m.start + BigInt(m.bytes.length) - addr) / size));
  return { first, held: Math.max(0, last - first) };
}

function arrayValue(m, addr, layout, size) {
  const elem = layout.elem || { kind: 'unknown', size: 1 };
  const count = layout.count ?? Math.floor(size / (elem.size || 1));
  const off = offOf(m, addr);
  if (off < 0 && (elem.kind === 'char' || elem.kind === 'unknown')) return unknownValue(m, addr, size);
  if (elem.kind === 'char' || (elem.kind === 'unknown' && elem.size === 1)) {
    const s = cString(m.bytes, off, Math.min(m.bytes.length, off + count), elem.kind === 'char' ? 0 : 2);
    if (s) return { value: quote(s.text), cls: 'dv-str', count: 0 };
    if (elem.kind === 'unknown') return unknownValue(m, addr, size);
  }
  if (elem.kind === 'unknown') return unknownValue(m, addr, size);
  const { first, held } = elemSpan(m, addr, elem, count);
  const parts = [];
  for (let i = first; i < first + Math.min(held, SUMMARY_ITEMS); i++) {
    parts.push(shortValue(m, addr + BigInt(i * elem.size), elem));
  }
  return { value: `{${first ? '…, ' : ''}${parts.join(', ')}${first + Math.min(held, SUMMARY_ITEMS) < count ? ', …' : ''}}`, cls: 'dv-agg', count };
}

function structValue(m, addr, layout) {
  const fields = layout.fields || [];
  const parts = fields.slice(0, 4).map((f) => `${f.name} = ${shortValue(m, addr + BigInt(f.offset), f.type)}`);
  return { value: `{${parts.join(', ')}${fields.length > 4 ? ', …' : ''}}`, cls: 'dv-agg', count: fields.length };
}

/** An element or field in a summary: the number, the string, the pointer's target, or `{…}`. */
function shortValue(m, addr, layout) {
  if (isUninit(m, addr)) return '0';
  const k = layout?.kind;
  if (k === 'struct' || k === 'union' || (k === 'array' && layout.elem?.kind !== 'char')) return '{…}';
  const v = valueOf(m, addr, layout);
  return v.value.replace(/ · 0x[0-9a-f]+$/, '').replace(/^→ /, '');
}

function unknownValue(m, addr, size) {
  const off = offOf(m, addr);
  if (off < 0) {
    return { value: `… ${hexBytes(m.bytes, 0, Math.min(LINE, size + off))} …`, cls: 'dv-bytes', count: Math.ceil(size / LINE) };
  }
  if ([1, 2, 4, 8].includes(size)) {
    if (size === m.pointerSize && pointsSomewhere(m, addr)) return pointerValue(m, addr, { size });
    const v = readUint(m.bytes, off, size, m.little);
    if (v !== null) return { value: intText(v, size), cls: 'dv-num', count: 0 };
  }
  const shown = Math.min(size, LINE);
  return {
    value: hexBytes(m.bytes, off, shown) + (size > LINE ? ' …' : ''),
    cls: 'dv-bytes',
    count: size > LINE ? Math.ceil(size / LINE) : 0,
  };
}

/** The rows an item opens to: elements, fields or 16-byte lines; at most MAX_CHILDREN. */
export function childRows(m, row) {
  const { layout, addr } = row;
  const out = [];
  if (layout.kind === 'array' && layout.elem && layout.elem.kind !== 'unknown') {
    const { first, held } = elemSpan(m, addr, layout.elem, row.count);
    const shown = isUninit(m, addr) ? row.count - first : held;
    for (let i = first; i < first + Math.min(shown, MAX_CHILDREN); i++) {
      const a = addr + BigInt(i * layout.elem.size);
      out.push({ kind: 'elem', addr: a, hex: hx(a), size: layout.elem.size, name: `[${i}]`, type: layout.elem.type, layout: layout.elem, ...valueOf(m, a, layout.elem) });
    }
  } else if (layout.kind === 'struct' || layout.kind === 'union') {
    for (const f of (layout.fields || []).slice(0, MAX_CHILDREN)) {
      const a = addr + BigInt(f.offset);
      out.push({ kind: 'field', addr: a, hex: hx(a), size: f.type.size, name: f.name, type: f.type.type, layout: f.type, ...valueOf(m, a, f.type) });
    }
  } else {
    const held = Number(m.start + BigInt(m.bytes.length) - addr);
    const lines = Math.min(Math.ceil(row.size / LINE), Math.max(0, Math.ceil(held / LINE)));
    const from = addr >= m.start ? 0 : Math.min(lines, Number((m.start - addr) / BigInt(LINE)));
    for (let i = from; i < Math.min(lines, from + MAX_CHILDREN); i++) {
      const a = addr + BigInt(i * LINE);
      const len = Math.min(LINE, row.size - i * LINE);
      out.push({ kind: 'line', addr: a, hex: hx(a), size: len, name: '', type: '', layout: { kind: 'unknown', size: len }, value: isUninit(m, a) ? UNINIT : hexBytes(m.bytes, offOf(m, a), len), cls: isUninit(m, a) ? 'dv-uninit' : 'dv-bytes', count: 0 });
    }
  }
  return out;
}

/**
 * Rows for the unlabelled bytes `[from, to)`: a pointer word the engine
 * resolved, a NUL-terminated text of 4 or more characters, a run of 8 or more
 * zeros, bytes that start as zeros, else hex lines of up to 16 bytes.
 */
export function gapRows(m, from, to) {
  const rows = [];
  let a = from;
  const limit = m.start + BigInt(m.bytes.length);
  const push = (kind, at, size, extra) => rows.push({ kind, addr: at, hex: hx(at), size, name: '', count: 0, ...extra });
  let pending = null;
  const flush = () => {
    if (!pending) return;
    push('bytes', pending.addr, pending.size, { type: '', value: hexBytes(m.bytes, offOf(m, pending.addr), pending.size), cls: 'dv-bytes' });
    pending = null;
  };
  while (a < to) {
    if (isUninit(m, a)) {
      flush();
      push('uninit', a, Number(to - a), { type: '', value: `${Number(to - a)} bytes, ${UNINIT}`, cls: 'dv-uninit' });
      break;
    }
    if (a >= limit) {
      flush();
      push('missing', a, Number(to - a), { type: '', value: 'not in the file', cls: 'dv-muted' });
      break;
    }
    const off = offOf(m, a);
    const ps = BigInt(m.pointerSize);
    if (a % ps === 0n && a + ps <= to && pointsSomewhere(m, a)) {
      flush();
      push('pointer', a, m.pointerSize, { type: 'pointer', layout: { kind: 'pointer', size: m.pointerSize }, ...pointerValue(m, a) });
      a += ps;
      continue;
    }
    const s = cString(m.bytes, off, Math.min(m.bytes.length, Number(to - m.start)), 4);
    if (s && (a === from || m.bytes[off - 1] === 0 || !printable(m.bytes[off - 1]))) {
      flush();
      push('text', a, s.size, { type: 'text', value: quote(s.text), cls: 'dv-str' });
      a += BigInt(s.size);
      continue;
    }
    let z = 0;
    while (a + BigInt(z) < to && off + z < m.bytes.length && m.bytes[off + z] === 0) z++;
    if (z >= 8 && (a % ps === 0n || !pending)) {
      flush();
      push('zeros', a, z, { type: '', value: `${z} zero bytes`, cls: 'dv-muted' });
      a += BigInt(z);
      continue;
    }
    if (!pending) pending = { addr: a, size: 0 };
    pending.size++;
    a++;
    if (pending.size === LINE || (a % BigInt(LINE) === 0n && pending.size >= 4)) flush();
  }
  flush();
  return rows;
}

/** The top-level row whose bytes hold `addr`, or null. */
export function rowAt(model, addr) {
  const a = typeof addr === 'bigint' ? addr : BigInt(addr);
  return model.rows.find((r) => !r.overlap && r.addr <= a && a < r.addr + BigInt(Math.max(r.size, 1))) || null;
}

/**
 * Where `addr` is in the window: `{key, row, open}` — the key of the deepest
 * row holding it (`<item hex>/<child index>` inside an array or struct that
 * opens to it), its row, and the keys that must be open to show it.
 */
export function placeOf(model, addr) {
  const a = typeof addr === 'bigint' ? addr : BigInt(addr);
  let row = rowAt(model, a);
  if (!row) return null;
  let key = row.hex;
  const open = [];
  for (let depth = 0; depth < 4 && row.count && a !== row.addr; depth++) {
    const kids = childRows(model, row);
    const i = kids.findIndex((k) => k.addr <= a && a < k.addr + BigInt(Math.max(k.size, 1)));
    if (i < 0) break;
    open.push(key);
    key = `${key}/${i}`;
    row = kids[i];
  }
  return { key, row, open };
}

/** The row a key names (`<item hex>`, then `/<child index>` per level), or null. */
export function rowOf(model, key) {
  const parts = (key || '').split('/');
  let row = model.rows.find((r) => r.hex === parts[0]) || null;
  for (const i of parts.slice(1)) {
    if (!row?.count) return null;
    row = childRows(model, row)[Number(i)] || null;
  }
  return row;
}

/**
 * The uses of an item (`[{to_hex, name, address_hex, at_hex, kind, via}]`
 * from the `uses` document) grouped by function in first-use order:
 * `[{fn, name, sites:[at_hex], kinds:Set, via}]`.
 */
export function usesOf(uses, addr, size, nameOf = (hex, name) => name || hex) {
  const lo = typeof addr === 'bigint' ? addr : BigInt(addr);
  const hi = lo + BigInt(Math.max(size, 1));
  const groups = new Map();
  for (const u of uses || []) {
    const to = BigInt(u.to_hex ?? u.to);
    if (to < lo || to >= hi) continue;
    const fn = (u.address_hex || '').toLowerCase();
    let g = groups.get(fn);
    if (!g) groups.set(fn, (g = { fn, name: nameOf(fn, u.name), sites: [], kinds: new Set(), via: null }));
    if (!g.sites.includes(u.at_hex)) g.sites.push(u.at_hex);
    g.kinds.add(u.kind);
    if (u.via && !g.via) g.via = u.via.name || bare(u.via.address_hex);
  }
  return [...groups.values()];
}

/** How a function uses the data, in words. */
export function useWords(kinds) {
  const k = [...kinds];
  if (k.includes('write') && k.includes('read')) return 'reads and writes it';
  if (k.includes('write')) return 'writes it';
  if (k.includes('read')) return 'reads it';
  return 'uses its address';
}

const ROW_WORDS = { text: 'text', pointer: 'pointer', bytes: '', zeros: '', uninit: '', missing: '' };

/** One row as HTML (a `.dv-row` grid line; `open` when its children show). */
export function rowHtml(row, { depth = 0, selected = false, open = false, key = row.hex, id = true } = {}) {
  const tw = row.count ? `<button class="dv-tw" data-act="dv-toggle" aria-expanded="${open}" aria-label="${open ? 'Fold' : 'Show'} ${row.count} ${row.layout?.kind === 'struct' ? 'fields' : row.layout?.kind === 'array' && row.layout.elem?.kind !== 'unknown' ? 'elements' : 'lines'}"></button>` : '<span class="dv-tw0"></span>';
  const name = row.name ? escapeHtml(row.name) : '';
  const aliases = row.aliases?.length ? ` title="${escapeHtml(`Also called ${row.aliases.join(', ')}`)}"` : '';
  const type = row.type ?? ROW_WORDS[row.kind] ?? '';
  const value = row.link
    ? `<a class="dv-link" data-act="dv-follow" data-to="${escapeHtml(row.link.hex)}" data-kind="${row.link.kind}" href="#" title="${row.link.kind === 'fn' ? 'Open this function' : 'Go to this data'}">${escapeHtml(row.value)}</a>`
    : escapeHtml(row.value ?? '');
  return `<div class="dv-row dv-${row.kind}${selected ? ' sel' : ''}${row.overlap ? ' dv-over' : ''}"${id ? ` id="dv-${row.hex}"` : ''} role="option" aria-selected="${selected}" data-a="${row.hex}" data-key="${escapeHtml(key)}" data-size="${row.size}" style="--d:${depth}">` +
    `<span class="dv-a">${bare(row.hex)}</span>` +
    `<span class="dv-n"${aliases}>${tw}<span class="dv-nm">${name}</span></span>` +
    `<span class="dv-t">${escapeHtml(type)}</span>` +
    `<span class="dv-v ${row.cls || ''}"${row.cls === 'dv-uninit' ? ` title="${escapeHtml(UNINIT_WHY)}"` : ''}>${value}</span></div>`;
}

/**
 * The whole window as HTML: the section heading (a menu of `sections` when
 * there are several), then every row, the open ones followed by their children. `open` and `selected` are row keys
 * (`<hex>` for a top-level row, `<parent key>/<index>` below it).
 */
export function renderData(model, { selected = null, open = new Set(), sections = [] } = {}) {
  const s = model.section;
  const range = s ? ` · ${bare(s.address_hex)}–${bare(hx(BigInt(s.address_hex) + BigInt(s.size)))}` : '';
  const listed = s && sections.length > 1 && sections.some((x) => secAddr(x) === secAddr(s));
  const title = listed ? sectionPicker(sections, s) : `<b>${escapeHtml(s?.name || 'Memory')}</b>`;
  let html = `<div class="dv-sec">${title} <span>${escapeHtml(sectionWords(s) + range)}</span></div>`;
  let nameCh = 6;
  let typeCh = 4;
  const measure = (rows, depth) => {
    for (const r of rows) {
      nameCh = Math.max(nameCh, (r.name || '').length + depth * 2 + 2);
      typeCh = Math.max(typeCh, (r.type ?? ROW_WORDS[r.kind] ?? '').length);
    }
  };
  measure(model.rows, 0);
  const walk = (rows, depth, parent) => {
    for (const [i, row] of rows.entries()) {
      const key = parent ? `${parent}/${i}` : row.hex;
      const isOpen = !!row.count && open.has(key);
      html += rowHtml(row, { depth, selected: selected === key, open: isOpen, key, id: !parent });
      if (isOpen) {
        const kids = childRows(model, row);
        measure(kids, depth + 1);
        walk(kids, depth + 1, key);
        if (row.count > kids.length) html += `<div class="dv-row dv-rest" style="--d:${depth + 1}"><span class="dv-a"></span><span class="dv-n"></span><span class="dv-t"></span><span class="dv-v dv-muted">${row.count - kids.length} more not shown here</span></div>`;
      }
    }
  };
  walk(model.rows, 0, '');
  return `<div class="dv-rows" style="--nw:${Math.min(nameCh, 32)}ch;--tw:${Math.min(typeCh, 26)}ch">${html}</div>`;
}
