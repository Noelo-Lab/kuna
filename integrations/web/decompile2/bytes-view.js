// bytes-view.js — the study view's bytes pane: a function's bytes as a hex
// dump (instruction bytes from the engine, gaps from the page's own copy of
// the file), the student's patches marked, and the patched file built from
// `sections[].file_offset`. DOM-free.
import { escapeHtml } from '../assets/js/highlight-c.js';
import { bare } from './addr.js';

const big = (a) => (typeof a === 'bigint' ? a : BigInt(a));
const hex = (a) => '0x' + big(a).toString(16);

/**
 * The file offset that holds virtual address `addr`, or null (not
 * file-backed). A section's file bytes may be fewer than its size (`file_size`:
 * a PE section's zero-filled tail), and the tail has no offset.
 */
export function fileOffsetFor(addr, sections = []) {
  const a = big(addr);
  for (const s of sections) {
    const start = BigInt(s.address_hex ?? s.address);
    if (a < start || a >= start + BigInt(s.size)) continue;
    if (s.file_offset === null || s.file_offset === undefined) return null;
    const held = BigInt(s.file_size ?? s.size);
    return a - start < held ? Number(BigInt(s.file_offset) + (a - start)) : null;
  }
  return null;
}

/** The byte the file holds at `addr`, or null. */
export function originalByteAt(addr, { fileBytes, sections }) {
  const off = fileOffsetFor(addr, sections);
  return off !== null && fileBytes && off < fileBytes.length ? fileBytes[off] : null;
}

/**
 * One cell per byte of the function `[entry, entry + size)`: `{addr, hex,
 * value, orig, patched, insn, first, gap}`. Instruction bytes come from the
 * engine rows; bytes between rows from the file; `patches` (Map BigInt →
 * byte) is the session's overlay.
 */
export function functionCells(instructions, { entry, size = 0, fileBytes = null, sections = [], patches = new Map() } = {}) {
  const byAddr = new Map();
  instructions.forEach((insn, i) => {
    const base = big(insn.address_hex);
    for (let k = 0; k < insn.size; k++) {
      byAddr.set(base + BigInt(k), { insn: i, first: k === 0, engine: parseInt(insn.bytes.slice(k * 2, k * 2 + 2), 16) });
    }
  });
  let start = entry != null ? big(entry) : instructions.length ? big(instructions[0].address_hex) : 0n;
  let end = start + BigInt(size || 0);
  for (const insn of instructions) {
    const a = big(insn.address_hex);
    if (a < start) start = a;
    if (a + BigInt(insn.size) > end) end = a + BigInt(insn.size);
  }
  const cells = [];
  for (let a = start; a < end; a++) {
    const known = byAddr.get(a);
    const orig = originalByteAt(a, { fileBytes, sections }) ?? (known ? known.engine : null);
    const patched = patches.has(a);
    const value = patched ? patches.get(a) : known ? known.engine : orig;
    cells.push({
      addr: a, hex: hex(a), value, orig, patched,
      insn: known ? known.insn : null, first: !!known?.first, gap: value === null,
    });
  }
  return cells;
}

/** Cells into 16-byte rows aligned to 16: `[{base, cells: (cell|null)[]}]`. */
export function hexRows(cells, width = 16) {
  const rows = [];
  const w = BigInt(width);
  for (const cell of cells) {
    const base = cell.addr - (cell.addr % w);
    let row = rows[rows.length - 1];
    if (!row || row.base !== base) {
      row = { base, cells: new Array(width).fill(null) };
      rows.push(row);
    }
    row.cells[Number(cell.addr - base)] = cell;
  }
  return rows;
}

/**
 * The file with `runs` (`[{addr, values}]`) written in: `{bytes, unmapped}`,
 * where `unmapped` lists the runs (or parts) no file byte backs — those are
 * left out rather than written somewhere wrong.
 */
export function applyPatches(fileBytes, sections, runs) {
  const out = new Uint8Array(fileBytes);
  const unmapped = [];
  for (const run of runs) {
    let missing = 0;
    run.values.forEach((v, i) => {
      const off = fileOffsetFor(big(run.addr) + BigInt(i), sections);
      if (off === null || off >= out.length) missing++;
      else out[off] = v;
    });
    if (missing) unmapped.push({ addr: hex(run.addr), count: missing });
  }
  return { bytes: out, unmapped };
}

/** `"90 90"`, `"0x9090"`, `"90,90"` → Uint8Array; null for anything else. */
export function parseHex(text) {
  const t = (text || '').replace(/0x/gi, '').replace(/[\s,]+/g, '');
  if (!t || t.length % 2 || !/^[0-9a-f]+$/i.test(t)) return null;
  const out = new Uint8Array(t.length / 2);
  for (let i = 0; i < out.length; i++) out[i] = parseInt(t.slice(i * 2, i * 2 + 2), 16);
  return out;
}

/** `sample.elf` → `sample.patched.elf`; `a.out` → `a.patched.out`; `prog` → `prog.patched`. */
export function patchedName(name) {
  const n = name || 'binary';
  const dot = n.lastIndexOf('.');
  if (dot <= 0) return `${n}.patched`;
  return `${n.slice(0, dot)}.patched${n.slice(dot)}`;
}

const byteHex = (v) => (v === null || v === undefined ? '··' : v.toString(16).padStart(2, '0'));
const ascii = (v) => (v >= 0x20 && v < 0x7f ? String.fromCharCode(v) : '.');

/**
 * The hex dump's HTML. `opts`: `{fileBytes, sections, selected (Set of
 * insn indices), editAddr (BigInt)}`. Cells are `.hb[data-a]`, patched ones
 * `.pa`, the selected instruction's `.in`, the byte being typed `.ed`.
 */
export function renderHex(rows, { sections = [], selected = new Set(), editAddr = null } = {}) {
  let out = '';
  for (const row of rows) {
    const off = fileOffsetFor(row.base, sections);
    out += `<div class="d2-hr"><span class="ho" title="${off === null ? 'not stored in the file' : `file offset 0x${off.toString(16)}`}">${escapeHtml(bare(hex(row.base)))}</span><span class="hx">`;
    let text = '';
    row.cells.forEach((c) => {
      if (!c) {
        out += '<span class="hb gap"> </span>';
        text += ' ';
        return;
      }
      const cls = ['hb', c.patched ? 'pa' : '', c.gap ? 'gap' : '', c.insn !== null && selected.has(c.insn) ? 'in' : '',
        editAddr !== null && c.addr === editAddr ? 'ed' : '', c.insn !== null && c.insn % 2 ? 'alt' : ''].filter(Boolean).join(' ');
      const title = c.patched ? ` title="was ${byteHex(c.orig)}"` : '';
      out += `<span class="${cls}" data-a="${c.hex}"${c.insn !== null ? ` data-i="${c.insn}"` : ''}${title}>${byteHex(c.value)}</span>`;
      text += c.value === null ? ' ' : (c.patched ? `\u0001${ascii(c.value)}\u0002` : ascii(c.value));
    });
    const asciiHtml = escapeHtml(text).replace(/\u0001(.*?)\u0002/g, '<i class="pa">$1</i>');
    out += `</span><span class="ha">${asciiHtml}</span></div>`;
  }
  return out;
}
