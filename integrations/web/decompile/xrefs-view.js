// xrefs-view.js — "Calls and callers" in the Explain panel, in plain words:
// which functions this one calls, which call it (from the engine's `xrefs`,
// loaded on request), and the data it uses. Each name links to the function
// or address; the exact instruction is in its tooltip. When the engine's
// reference walk is not available the callees are read from this function's
// own CALL instructions. DOM-free.
import { escapeHtml } from '../assets/js/highlight-c.js';
import { isCall } from './asm-view.js';

const HOW = { jump: 'jumps to it', data: 'uses its address', read: 'reads it', write: 'writes it' };

function link(hex, label, title) {
  return `<a class="xt" data-goto="${escapeHtml(hex)}"${title ? ` title="${escapeHtml(title)}"` : ''}>${escapeHtml(label)}</a>`;
}

/** `a`, `a and b`, `a, b and c`. */
export function joinWords(items) {
  if (items.length <= 1) return items.join('');
  return `${items.slice(0, -1).join(', ')} and ${items[items.length - 1]}`;
}

/** One link per distinct target, noting how it refers when that is not a call. */
function names(rows, { nameOf, target, at }) {
  const seen = new Map();
  for (const r of rows) {
    const hex = r[target];
    if (!hex) continue;
    if (!seen.has(hex)) seen.set(hex, { r, sites: [] });
    seen.get(hex).sites.push(r[at] + (r.instruction ? `: ${r.instruction}` : ''));
  }
  return [...seen.entries()].map(([hex, { r, sites }]) => {
    const how = HOW[r.kind] ? ` <span class="d2muted">(${HOW[r.kind]})</span>` : '';
    return link(hex, nameOf(hex, r.name) || hex, sites.join('\n')) + how;
  });
}

/**
 * The engine's `xrefs` document as plain sentences. `nameOf(addrHex, name)`
 * gives the student's name for a function (the engine's `name` may be null).
 */
export function renderXrefs(result, { nameOf = (a, n) => n || a } = {}) {
  const calls = names(result.callees || [], { nameOf, target: 'address_hex', at: 'at_hex' });
  const callers = names(result.callers || [], { nameOf, target: 'address_hex', at: 'from_hex' });
  const data = names(result.data_refs || [], { nameOf: (a, n) => n || a, target: 'address_hex', at: 'at_hex' });
  return `<p class="x-sum">${calls.length ? `Calls ${joinWords(calls)}` : 'Calls no other functions'}</p>` +
    `<p class="x-sum">${callers.length ? `Called by ${joinWords(callers)}` : 'Nothing in this program calls it directly'}</p>` +
    (data.length ? `<p class="x-sum">Uses data at ${joinWords(data)}</p>` : '');
}

/** What the page knows without the engine's walk: the calls, and a way to ask for callers. */
export function renderLocalCalls(callees, { nameOf = (a, n) => n || a, note = '' } = {}) {
  const calls = names(callees, { nameOf, target: 'address_hex', at: 'at_hex' });
  return `<p class="x-sum">${calls.length ? `Calls ${joinWords(calls)}` : 'Calls no other functions'}</p>` +
    (note ? `<p class="x-sum d2muted">${escapeHtml(note)}</p>`
      : '<p class="x-sum"><button class="d2-link" data-act="refs-load">Find who calls it</button></p>');
}

/**
 * Callees read from this function's own CALL rows: `[{address_hex, name,
 * at_hex, kind, instruction}]` (indirect calls are skipped).
 */
export function localCallees(fnData, fnByAddr) {
  const out = [];
  for (const insn of fnData.instructions || []) {
    if (!isCall(insn.mnemonic)) continue;
    const m = /^\s*(0x[0-9a-f]+)\s*$/i.exec(insn.operands || '');
    if (!m) continue;
    const hex = '0x' + BigInt(m[1]).toString(16);
    out.push({ address_hex: hex, name: fnByAddr?.get(hex)?.name || hex, at_hex: insn.address_hex, kind: 'call', instruction: insn.text });
  }
  return out;
}

const SETS = (sym) => new RegExp(`(?:\\b${sym}\\b(?:\\s*\\[[^\\]]*\\])*\\s*(?:[-+*/%&|^]|<<|>>)?=(?!=))|(?:\\+\\+|--)\\s*\\b${sym}\\b|\\b${sym}\\b\\s*(?:\\+\\+|--)`);

/** How line `text` uses `sym`: `declares`, `sets` (an assignment or ++/--) or `reads`. */
export function useKind(text, sym, { decl = false } = {}) {
  if (decl) return 'declares';
  if (!/^\w+$/.test(sym)) return 'reads';
  return SETS(sym).test(text) ? 'sets' : 'reads';
}

const USE_WORDS = { declares: 'declared', sets: 'set', reads: 'read', input: 'input' };

function lineRow(r) {
  return `<li class="xr-row" tabindex="-1" data-line="${r.line}" title="Go to line ${r.line}">` +
    `<span class="xr-at">${r.line}</span><span class="xr-how ${r.how}">${USE_WORDS[r.how] || ''}</span>` +
    `<code class="xr-code">${escapeHtml(r.text)}</code></li>`;
}

function siteRow(r) {
  const how = r.how || (r.kind === 'call' ? 'calls' : HOW[r.kind] || r.kind || '');
  return `<li class="xr-row" tabindex="-1" data-fn="${escapeHtml(r.fn)}" data-site="${escapeHtml(r.site)}" title="${escapeHtml(r.instruction || '')}">` +
    `<span class="xr-at">${escapeHtml(r.siteLabel)}</span><span class="xr-how">${escapeHtml(how)}</span>` +
    `<span class="xr-name">${escapeHtml(r.name)}</span><code class="xr-code">${escapeHtml(r.instruction || '')}</code></li>`;
}

/**
 * The cross-references dialog: `{title, sections: [{heading, rows, kind:
 * 'lines'|'sites', loading, empty, note}]}`. Line rows: `{line, how, text}`;
 * site rows: `{fn, site, siteLabel, name, kind, how?, instruction}` (`fn` is the
 * function to open, `site` the instruction to show in it, `how` replaces the
 * words for `kind`).
 */
export function renderRefsDialog(model) {
  const sections = model.sections.map((s) => {
    const head = `<h3>${escapeHtml(s.heading)}${s.rows?.length ? ` <span class="x-count">(${s.rows.length})</span>` : ''}</h3>`;
    if (s.loading) return head + `<p class="xr-empty">${escapeHtml(s.loading)}</p>`;
    const body = s.rows?.length
      ? `<ul class="xr-list">${s.rows.map(s.kind === 'lines' ? lineRow : siteRow).join('')}</ul>`
      : `<p class="xr-empty">${escapeHtml(s.empty || 'None.')}</p>`;
    return head + body + (s.note ? `<p class="xr-empty">${escapeHtml(s.note)}</p>` : '');
  }).join('');
  return `<div class="hh"><h2 id="xrefstitle">${escapeHtml(model.title)}</h2>` +
    '<button class="d2-iconbtn small" data-act="xrefs-close" aria-label="Close" title="Close (Esc)">×</button></div>' +
    `<div class="hb2">${model.sub ? `<p class="xr-sub">${escapeHtml(model.sub)}</p>` : ''}${sections}</div>`;
}
