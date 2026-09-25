// render-c.js — the study view's C pane: turns one function's code (plus the
// engine's token stream when it has one) into escaped HTML rows, and builds
// the line <-> instruction <-> symbol index every linked view shares.
//
// DOM-free. Every engine string reaches HTML through escapeHtml. A line whose
// tokens do not reproduce its text exactly falls back to the regex scanner
// (assets/js/highlight-c.js `scan`) for that line only.
import { scan, escapeHtml } from '../assets/js/highlight-c.js';
import { addrHex, bare } from './addr.js';

const CHUNK = 256;
const MAX_TOKEN_SPANS = 200000;

/** Colour band of a C line (shared with the assembly rows it maps to). */
export function bandOf(line) {
  return ((line % 6) + 6) % 6;
}

/** The first line of a C body when it reads like a signature. */
function signatureLine(code) {
  const first = (code || '').split('\n')[0] || '';
  return first.includes('(') ? first.trim() : null;
}

function normalizeInsn(insn, entry) {
  const address_hex = addrHex(insn.address_hex ?? insn.address);
  let offset = Number.isFinite(insn.offset) ? insn.offset : null;
  if (offset === null && address_hex && entry) {
    const d = BigInt(address_hex) - BigInt(entry);
    offset = Number(d);
  }
  return {
    ...insn,
    address_hex,
    offset,
    size: insn.size ?? (insn.bytes ? insn.bytes.length / 2 : 0),
    bytes: (insn.bytes || '').toLowerCase(),
    mnemonic: insn.mnemonic || '',
    operands: insn.operands || '',
    text: insn.text || [insn.mnemonic, insn.operands].filter(Boolean).join(' '),
    file_offset: Number.isInteger(insn.file_offset) ? insn.file_offset : null,
    lines: Array.isArray(insn.lines) ? insn.lines : [],
  };
}

/**
 * One function as the page works with it, from either an `inspect` document or
 * a plain `decompile` document (an engine build without `inspect`): the latter
 * has no tokens and no instructions, and says so through `tokens_error` and
 * `hasInstructions`.
 */
export function normalizeInspect(result) {
  const fromInspect = !!result?.function;
  const fn = fromInspect ? result.function : (result?.functions || [])[0];
  if (!fn) throw new Error('the engine returned no function');
  const address_hex = addrHex(fn.address_hex ?? fn.address);
  const code = fn.code ?? '';
  const tokens = Array.isArray(fn.tokens) ? fn.tokens : [];
  return {
    source: fromInspect ? 'inspect' : 'decompile',
    name: fn.name,
    address_hex,
    aliases: fn.aliases || [],
    kind: fn.kind || 'func',
    size: fn.size ?? 0,
    code,
    error: fn.error ?? null,
    proto: fn.proto ?? signatureLine(code),
    language: result.language || (/^#\[allow\(|\blet mut\b/m.test(code) ? 'rust-language' : 'c-language'),
    target: result.target || null,
    unstructured_gotos: fn.unstructured_gotos ?? 0,
    line_mappings: fn.line_mappings || [],
    variables: fn.variables || [],
    types: fn.types || [],
    globals: fn.globals || [],
    tokens,
    tokens_error: fn.tokens_error ?? (fromInspect || tokens.length
      ? null
      : 'no token stream: this engine build answers `decompile`, not `inspect`'),
    hasInstructions: Array.isArray(fn.instructions),
    instructions: (fn.instructions || []).map((insn) => normalizeInsn(insn, address_hex)),
    instructions_truncated: !!fn.instructions_truncated,
    assertions: result.assertions || [],
  };
}

/**
 * Per-line segments from the engine tokens: `[{text, tok}]` covering each line
 * exactly (gaps are `tok: null`), or `null` for a line the tokens do not
 * reproduce (overlap, text mismatch, uncovered non-blank text).
 */
export function tokenLines(code, tokens) {
  const lines = code.split('\n');
  const byLine = lines.map(() => []);
  for (const t of tokens || []) {
    const i = t.line - 1;
    if (i >= 0 && i < lines.length && Number.isInteger(t.col) && typeof t.text === 'string') byLine[i].push(t);
  }
  return lines.map((text, i) => {
    const toks = byLine[i].sort((a, b) => a.col - b.col);
    if (!toks.length) return text.trim() === '' ? (text ? [{ text, tok: null }] : []) : null;
    const segs = [];
    let at = 0;
    for (const t of toks) {
      const len = Number.isInteger(t.len) ? t.len : t.text.length;
      if (t.col < at || len !== t.text.length || text.slice(t.col, t.col + len) !== t.text) return null;
      const gap = text.slice(at, t.col);
      if (gap.trim() !== '') return null;
      if (gap) segs.push({ text: gap, tok: null });
      segs.push({ text: t.text, tok: t });
      at = t.col + len;
    }
    const tail = text.slice(at);
    if (tail.trim() !== '') return null;
    if (tail) segs.push({ text: tail, tok: null });
    return segs;
  });
}

const PSEUDO_KIND = {
  'tok-com': 'comment', 'tok-str': 'value', 'tok-chr': 'value', 'tok-num': 'value',
  'tok-kw': 'syntax', 'tok-type': 'type', 'tok-fn': 'funcname', 'tok-glob': 'variable',
};

/**
 * Per-line segments from the regex scanner, for code the engine sent without
 * tokens (or a line whose tokens failed verification). Identifiers become
 * pseudo-tokens so selection and renaming still work: after `.`/`->` a field,
 * before `(` a function name, a `LAB_…:` a label, anything else a variable.
 */
export function fallbackLines(code, { language } = {}) {
  const dialect = /rust/.test(language || '') ? 'rust' : 'auto';
  const lines = [[]];
  let prevText = '';
  for (const p of scan(code, dialect)) {
    const parts = p.text.split('\n');
    parts.forEach((part, k) => {
      if (k > 0) lines.push([]);
      if (!part) return;
      let tok = null;
      if (p.word) {
        let kind = PSEUDO_KIND[p.cls] || null;
        if (!kind && /^[A-Za-z_]/.test(part)) {
          if (/(\.|->)\s*$/.test(prevText)) kind = 'field';
          else if (/^LAB_/.test(part)) kind = 'label';
          else kind = 'variable';
        }
        if (kind) tok = { kind, text: part, cls: p.cls, fallback: true };
      }
      lines[lines.length - 1].push({ text: part, tok });
    });
    prevText = p.text;
  }
  return lines;
}

/** Segments for every line: engine tokens where they verify, the scanner elsewhere. */
export function lineSegments(fnData) {
  const code = fnData.code || '';
  const fallback = fallbackLines(code, { language: fnData.language });
  if (!fnData.tokens?.length || fnData.tokens.length > MAX_TOKEN_SPANS) {
    return { segs: fallback, fallbackCount: fallback.length, tokenCount: 0 };
  }
  const exact = tokenLines(code, fnData.tokens);
  let fallbackCount = 0;
  const segs = exact.map((line, i) => {
    if (line) return line;
    fallbackCount++;
    return fallback[i] || [];
  });
  return { segs, fallbackCount, tokenCount: fnData.tokens.length };
}

/**
 * Local declarations kuna prints with a storage comment
 * (`  long v1; // rax`, `  int v1; // stack - 0x14`), from the block that
 * opens the body. Register locals appear nowhere else.
 */
export function localDecls(code) {
  const lines = (code || '').split('\n');
  const out = [];
  let inBody = false;
  for (let i = 0; i < lines.length; i++) {
    const text = lines[i];
    if (!inBody) {
      if (/^\{\s*$/.test(text)) inBody = true;
      continue;
    }
    if (text.trim() === '') break;
    const m = /^\s+(.+?)\s*\b([A-Za-z_]\w*)\s*((?:\[[^\]]*\]\s*)*);\s*\/\/\s*(.+?)\s*$/.exec(text);
    if (!m) continue;
    out.push({ name: m[2], type: (m[1] + (m[3] ? ' ' + m[3].replace(/\s+/g, '') : '')).trim(), storage: m[4], line: i + 1 });
  }
  return out;
}

/** `rax` → `register RAX`; `stack - 0x14` → `stack −0x14`. */
export function storageLabel(storage) {
  if (!storage) return '';
  const stack = /^stack\s*([-+])\s*(0x[0-9a-f]+|\d+)$/i.exec(storage.trim());
  if (stack) return `stack ${stack[1] === '-' ? '−' : '+'}${stack[2]}`;
  if (/^[a-z][a-z0-9]*$/i.test(storage.trim())) return `register ${storage.trim().toUpperCase()}`;
  return storage;
}

const addToMap = (map, key, value) => {
  if (key == null || value == null) return;
  let set = map.get(key);
  if (!set) map.set(key, (set = new Set()));
  set.add(value);
};

/**
 * The shared index: `lineToInsns` / `insnToLines` / `addrToInsn` / `insnIndex`
 * / `symToLines` / `symToAddrs`. Line mappings come from every source the
 * engine gives — instruction `lines`, `line_mappings`, token addresses — since
 * each alone can be sparse.
 */
export function buildIndex(fnData, segs = lineSegments(fnData).segs) {
  const lineToInsns = new Map();
  const insnToLines = new Map();
  const addrToInsn = new Map();
  const insnIndex = new Map();
  const symToLines = new Map();
  const symToAddrs = new Map();
  const link = (line, hex) => {
    if (!hex || !Number.isInteger(line)) return;
    addToMap(lineToInsns, line, hex);
    addToMap(insnToLines, hex, line);
  };
  fnData.instructions.forEach((insn, i) => {
    addrToInsn.set(insn.address_hex, insn);
    insnIndex.set(insn.address_hex, i);
    for (const line of insn.lines) link(line, insn.address_hex);
  });
  for (const m of fnData.line_mappings) {
    for (const a of m.addresses || []) link(m.line_number, addrHex(a));
  }
  segs.forEach((line, i) => {
    for (const s of line) {
      const t = s.tok;
      if (!t) continue;
      const hex = addrHex(t.address_hex ?? t.address);
      if (hex && t.kind !== 'comment') link(i + 1, hex);
      if (t.kind === 'variable') {
        addToMap(symToLines, t.text, i + 1);
        if (hex) addToMap(symToAddrs, t.text, hex);
      }
    }
  });
  for (const v of fnData.variables) {
    for (const a of v.addresses || []) addToMap(symToAddrs, v.name, addrHex(a));
  }
  const order = (set) => [...set].sort((a, b) => {
    const ia = insnIndex.get(a), ib = insnIndex.get(b);
    if (ia != null && ib != null) return ia - ib;
    return BigInt(a) < BigInt(b) ? -1 : BigInt(a) > BigInt(b) ? 1 : 0;
  });
  const sorted = new Map();
  for (const [line, set] of lineToInsns) sorted.set(line, order(set));
  const lines = new Map();
  for (const [hex, set] of insnToLines) lines.set(hex, [...set].sort((a, b) => a - b));
  return {
    lineToInsns: sorted,
    insnToLines: lines,
    addrToInsn,
    insnIndex,
    symToLines,
    symToAddrs,
    lineCount: segs.length,
  };
}

const COLOR_CLASS = [
  [/keyword/, 'tok-kw'], [/comment/, 'tok-com'], [/type/, 'tok-type'], [/funcname/, 'tok-fn'],
  [/global/, 'tok-glob'], [/param/, 'tk-param'], [/error/, 'tk-err'], [/special/, 'tok-kw'],
];

/** The CSS class of one token: its `tok-*` colour plus `t` and `tk-<kind>`. */
function tokenClass(t) {
  let color = t.cls || '';
  if (!color && t.color) {
    for (const [re, cls] of COLOR_CLASS) if (re.test(t.color)) { color = cls; break; }
    if (!color && /const/.test(t.color)) {
      color = t.text.startsWith('"') ? 'tok-str' : t.text.startsWith("'") ? 'tok-chr' : 'tok-num';
    }
  }
  if (!color && t.kind === 'type') color = 'tok-type';
  if (!color && t.kind === 'funcname') color = 'tok-fn';
  if (!color && t.kind === 'comment') color = 'tok-com';
  return ['t', `tk-${t.kind || 'syntax'}`, color].filter(Boolean).join(' ');
}

const attr = (name, value) => (value == null || value === '' ? '' : ` ${name}="${escapeHtml(String(value))}"`);

function tokenHtml(t, ctx) {
  const hex = addrHex(t.address_hex ?? t.address);
  let callee = addrHex(t.callee_hex ?? t.callee);
  if (!callee && t.kind === 'funcname') callee = ctx.fnByName?.get(t.text)?.address_hex || null;
  let gaddr = null;
  if (t.kind === 'variable') {
    gaddr = ctx.globalsByName?.get(t.text) || null;
    if (!gaddr) {
      const m = /^dat_([0-9a-f]+)$/i.exec(t.text);
      if (m) gaddr = addrHex('0x' + m[1]);
    }
  }
  const sym = t.kind === 'variable' ? t.text : null;
  const typeText = t.type ?? (t.kind === 'type' ? t.text : null);
  return `<span class="${tokenClass(t)}"${attr('data-kind', t.kind)}${attr('data-sym', sym)}` +
    `${attr('data-addr', hex)}${attr('data-callee', callee)}${attr('data-gaddr', gaddr)}` +
    `${attr('data-type', typeText)}${attr('data-decl', t.decl)}${attr('data-var', Number.isInteger(t.var) ? t.var : null)}` +
    `${t.fallback ? ' data-fb="1"' : ''}>${escapeHtml(t.text)}</span>`;
}

/**
 * The C pane's HTML: one `.d2-cl#c-L<n>` row per line with a line-number and
 * address gutter, tokens as `.t` spans carrying their kind, symbol, address,
 * callee, global address and type. `ctx`: `{index, segs, fnByName, globalsByName}`.
 */
export function renderC(fnData, ctx = {}) {
  const segs = ctx.segs || lineSegments(fnData).segs;
  const index = ctx.index || buildIndex(fnData, segs);
  if (!fnData.code && fnData.error) {
    return `<div class="d2-cl d2-err"><span class="ct">${escapeHtml(`/* ${fnData.name} — decompile error:\n   ${fnData.error} */`)}</span></div>`;
  }
  let out = '<div class="d2-chunk">';
  segs.forEach((line, i) => {
    if (i && i % CHUNK === 0) out += '</div><div class="d2-chunk">';
    const n = i + 1;
    const addrs = index.lineToInsns.get(n) || [];
    const band = addrs.length ? bandOf(n) : null;
    out += `<div class="d2-cl" id="c-L${n}" role="option" data-line="${n}"` +
      `${attr('data-addrs', addrs.join(' '))}${attr('data-band', band)}>` +
      `<span class="ln">${n}</span><span class="la">${escapeHtml(bare(addrs[0]))}</span><span class="ct">`;
    for (const s of line) out += s.tok ? tokenHtml(s.tok, ctx) : escapeHtml(s.text);
    out += '</span></div>';
  });
  return out + '</div>';
}

/**
 * The 1-based lines of `newCode` that are not part of a longest common
 * subsequence with `oldCode` (what an edit changed). Large bodies skip the
 * quadratic table and report nothing rather than stall the page.
 */
export function changedLines(oldCode, newCode) {
  const a = (oldCode || '').split('\n');
  const b = (newCode || '').split('\n');
  const changed = new Set();
  if (!oldCode) return changed;
  if (a.length * b.length > 4e6) return changed;
  const w = b.length + 1;
  const dp = new Uint32Array((a.length + 1) * w);
  for (let i = a.length - 1; i >= 0; i--) {
    for (let j = b.length - 1; j >= 0; j--) {
      dp[i * w + j] = a[i] === b[j] ? dp[(i + 1) * w + j + 1] + 1 : Math.max(dp[(i + 1) * w + j], dp[i * w + j + 1]);
    }
  }
  let i = 0, j = 0;
  while (j < b.length) {
    if (i < a.length && a[i] === b[j]) { i++; j++; }
    else if (i < a.length && dp[(i + 1) * w + j] >= dp[i * w + j + 1]) i++;
    else { changed.add(j + 1); j++; }
  }
  return changed;
}
