// xrefs-view.js — a function's references for the rail: who calls it (each
// linked to the calling instruction), what it calls (linked to the call
// site), and the data it touches. When the engine has no `xrefs` command the
// callees can still be read off this function's own CALL instructions. DOM-free.
import { escapeHtml } from '../assets/js/highlight-c.js';
import { isCall } from './asm-view.js';

const link = (hex, label) => `<a class="xt" data-goto="${escapeHtml(hex)}">${escapeHtml(label)}</a>`;

function list(title, rows) {
  if (!rows.length) return `<p class="label" style="margin:8px 0 2px">${escapeHtml(title)}</p><p class="d2muted">none</p>`;
  return `<p class="label" style="margin:8px 0 2px">${escapeHtml(title)} (${rows.length})</p><ul class="d2edits">${rows.join('')}</ul>`;
}

/**
 * The engine's `xrefs` document as HTML. `nameOf(addrHex, fallback)` gives
 * the student's name for a function.
 */
const KIND = { call: '', jump: 'jumps to', data: 'takes the address', read: 'reads', write: 'writes' };

function kindNote(kind) {
  const note = KIND[kind] ?? kind;
  return note ? ` <span class="d2muted">${escapeHtml(note)}</span>` : '';
}

const insnNote = (text) => (text ? ` <span class="d2muted">${escapeHtml(text)}</span>` : '');

/**
 * The engine's `xrefs` document as HTML: callers link to the calling
 * function and the calling instruction, callees and data to the target and
 * the instruction that references it. `nameOf(addrHex, name)` gives the
 * student's name for a function (the engine's `name` may be null).
 */
export function renderXrefs(result, { nameOf = (a, n) => n || a } = {}) {
  const callers = (result.callers || []).map((c) => `<li><span class="tx">${link(c.address_hex, nameOf(c.address_hex, c.name))}` +
    `${kindNote(c.kind)} <span class="d2muted">at</span> ${link(c.from_hex, c.from_hex)}${insnNote(c.instruction)}</span></li>`);
  const callees = (result.callees || []).map((c) => `<li><span class="tx">${link(c.address_hex, nameOf(c.address_hex, c.name))}` +
    `${kindNote(c.kind)} <span class="d2muted">from</span> ${link(c.at_hex, c.at_hex)}${insnNote(c.instruction)}</span></li>`);
  const data = (result.data_refs || []).map((d) => {
    const target = d.address_hex || '';
    return `<li><span class="tx">${escapeHtml(d.name || target)} <span class="d2muted">${escapeHtml(target)}</span>` +
      `${kindNote(d.kind)} <span class="d2muted">at</span> ${link(d.at_hex, d.at_hex)}${insnNote(d.instruction)}</span></li>`;
  });
  return list('callers', callers) + list('callees', callees) + list('data', data);
}

/**
 * Callees read from this function's own CALL rows, for an engine without
 * `xrefs`: `[{address_hex, name, at_hex}]` (indirect calls are skipped).
 */
export function localCallees(fnData, fnByAddr) {
  const out = [];
  for (const insn of fnData.instructions || []) {
    if (!isCall(insn.mnemonic)) continue;
    const m = /^\s*(0x[0-9a-f]+)\s*$/i.exec(insn.operands || '');
    if (!m) continue;
    const hex = '0x' + BigInt(m[1]).toString(16);
    out.push({ address_hex: hex, name: fnByAddr?.get(hex)?.name || hex, at_hex: insn.address_hex, kind: 'call' });
  }
  return out;
}
