// asm-view.js — the study view's assembly pane: instruction rows annotated
// with the C line each came from, colour bands shared with the C pane, linked
// branch/call targets, stack-slot operands and a branch-arrow gutter.
//
// DOM-free: every function returns data or an escaped HTML string.
import { escapeHtml } from '../assets/js/highlight-c.js';
import { bandOf } from './render-c.js';
import { bare, signedHex } from './addr.js';

export { bandOf };

const CHUNK = 256;
const LANE_W = 8;
const MAX_LANES = 6;

/** An instruction address for display: `abs` (`11ab`), `rel` (`+0x13`) or `both`. */
export function formatAddr(addrHex, startHex, mode = 'abs') {
  const abs = bare(addrHex);
  if (mode === 'abs' || !startHex) return abs;
  const rel = signedHex(BigInt(addrHex) - BigInt(startHex));
  return mode === 'rel' ? rel : `${abs} ${rel}`;
}

/** `8b45ec` → `8b 45 ec`. */
export function spacedBytes(hex) {
  return (hex || '').replace(/(..)(?=.)/g, '$1 ');
}

const linesKey = (insn) => (insn.lines || []).join(',');

/** Consecutive instructions with the same C lines: `[{lines, line, start, end}]`. */
export function groupRuns(instructions) {
  const runs = [];
  instructions.forEach((insn, i) => {
    const key = linesKey(insn);
    const last = runs[runs.length - 1];
    if (last && last.key === key) last.end = i + 1;
    else runs.push({ key, lines: insn.lines || [], line: insn.lines?.[0] ?? null, start: i, end: i + 1 });
  });
  return runs.map(({ key, ...run }) => run);
}

const BRANCH = /^(J[A-Z]*|B|B\.[A-Z]+|B[A-Z]{2}|BR|CB N?Z|CBN?Z|TBN?Z|LOOP[A-Z]*|JMP|JCXZ|JECXZ|JRCXZ|BEQ[A-Z]*|BNE[A-Z]*|BGEZ|BLEZ|BGTZ|BLTZ|BC)$/;
const CALL = /^(CALL|BL|BLX|BLR|JAL|JALR|BCTRL)$/;

/** Is this mnemonic a (conditional or unconditional) jump, not a call? */
export function isBranch(mnemonic) {
  const m = (mnemonic || '').toUpperCase().replace(/^(REP[A-Z]*|LOCK|BND|NOTRACK)\s+/, '');
  return !CALL.test(m) && BRANCH.test(m);
}

export function isCall(mnemonic) {
  return CALL.test((mnemonic || '').toUpperCase().replace(/^(BND|NOTRACK)\s+/, ''));
}

/** The in-function branch target of `insn`, as an index into `instructions`. */
function branchTarget(insn, insnIndex) {
  if (!isBranch(insn.mnemonic)) return null;
  const m = /(?:^|[\s,#])(0x[0-9a-f]+)\s*$/i.exec(insn.operands || '');
  if (!m) return null;
  const hex = '0x' + BigInt(m[1]).toString(16);
  return insnIndex.has(hex) ? insnIndex.get(hex) : null;
}

/**
 * Branch arrows for the gutter: `{lanes, edges:[{from,to,lane}], rows}` where
 * `rows[i]` lists the segments crossing row `i` (`from`/`to`/`pass`, with the
 * direction). Lanes are assigned shortest-span first so nested loops nest.
 */
export function branchArrows(instructions) {
  const insnIndex = new Map(instructions.map((insn, i) => [insn.address_hex, i]));
  const edges = [];
  instructions.forEach((insn, i) => {
    const j = branchTarget(insn, insnIndex);
    if (j !== null && j !== i) edges.push({ from: i, to: j, lo: Math.min(i, j), hi: Math.max(i, j) });
  });
  edges.sort((a, b) => (a.hi - a.lo) - (b.hi - b.lo) || a.lo - b.lo);
  const busy = [];
  for (const e of edges) {
    let lane = 0;
    while (lane < MAX_LANES - 1 && (busy[lane] || []).some((o) => !(e.hi < o.lo || e.lo > o.hi))) lane++;
    (busy[lane] ||= []).push(e);
    e.lane = lane;
  }
  const rows = instructions.map(() => []);
  for (const e of edges) {
    const up = e.to < e.from;
    for (let r = e.lo; r <= e.hi; r++) {
      const kind = r === e.from ? 'from' : r === e.to ? 'to' : 'pass';
      rows[r].push({ lane: e.lane, kind, up });
    }
  }
  const lanes = edges.length ? Math.max(...edges.map((e) => e.lane)) + 1 : 0;
  return { lanes, edges: edges.map(({ from, to, lane }) => ({ from, to, lane })), rows };
}

/** One row's arrow gutter as inline SVG (`height` = the row height in px). */
function arrowSvg(segs, lanes, height = 20) {
  if (!lanes || !segs.length) return '';
  const w = lanes * LANE_W + 6;
  const mid = height / 2;
  let paths = '';
  for (const s of segs) {
    const x = w - 6 - s.lane * LANE_W;
    if (s.kind === 'pass') paths += `M${x} 0V${height}`;
    else {
      // from: the edge leaves this row towards the target; to: it arrives.
      const towardTop = s.kind === 'from' ? s.up : !s.up;
      paths += `M${w} ${mid}H${x}V${towardTop ? 0 : height}`;
    }
  }
  const heads = segs.filter((s) => s.kind === 'to')
    .map(() => `<path d="M${w - 4} ${mid - 3}L${w} ${mid}L${w - 4} ${mid + 3}" fill="none"/>`).join('');
  return `<svg width="${w}" height="${height}" aria-hidden="true" stroke="currentColor" stroke-width="1">` +
    `<path d="${paths}" fill="none"/>${heads}</svg>`;
}

const STACK_REF = /\[\s*(RBP|EBP|RSP|ESP|SP|X29|FP|W29)\s*(?:([+-])\s*(-?0x[0-9a-f]+|-?\d+))?\s*\]/i;
const PTR_PREFIX = /\b(?:byte|word|dword|qword|tword|xmmword|ymmword|zmmword|float|double)\s+ptr\s*$/i;

/** `[RBP + -0x14]` → `{reg:'RBP', disp:-20, start, end}` (start includes a `dword ptr` prefix). */
export function stackOperand(ops) {
  const m = STACK_REF.exec(ops || '');
  if (!m) return null;
  let disp = 0;
  if (m[3]) {
    const neg = m[3].startsWith('-');
    const mag = Number(BigInt(m[3].replace(/^-/, '')));
    disp = (m[2] === '-' ? -1 : 1) * (neg ? -mag : mag);
  }
  const ptr = PTR_PREFIX.exec(ops.slice(0, m.index));
  return { reg: m[1].toUpperCase(), disp, start: ptr ? ptr.index : m.index, end: m.index + m[0].length };
}

/**
 * Operands as HTML: code addresses become `a.xt[data-goto]` links (annotated
 * with the function they start), and a frame-relative memory operand becomes
 * `span.so[data-slot]` carrying its entry-relative stack offset when
 * `ctx.slotOf(reg, disp, insn)` can place it.
 */
export function linkOperands(insn, ctx = {}) {
  const ops = insn.operands || '';
  const linkable = isBranch(insn.mnemonic) || isCall(insn.mnemonic) || /^(LEA|ADR|ADRP)$/i.test(insn.mnemonic);
  const piece = (text) => {
    if (!linkable) return escapeHtml(text);
    return text.replace(/[^]*?\b(0x[0-9a-f]+)\b|[^]+/gi, (chunk, lit) => {
      if (!lit) return escapeHtml(chunk);
      const pre = chunk.slice(0, chunk.length - lit.length);
      const hex = '0x' + BigInt(lit).toString(16);
      const fn = ctx.fnByAddr?.get(hex);
      if (!fn && !ctx.insnIndex?.has(hex)) return escapeHtml(chunk);
      const label = fn && ctx.nameOf ? ` <span class="d2muted">&lt;${escapeHtml(ctx.nameOf(fn))}&gt;</span>` : '';
      return `${escapeHtml(pre)}<a class="xt" data-goto="${hex}">${escapeHtml(lit)}</a>${label}`;
    });
  };
  const so = stackOperand(ops);
  if (!so) return piece(ops);
  const slot = ctx.slotOf ? ctx.slotOf(so.reg, so.disp, insn) : null;
  const slotAttr = slot === null || slot === undefined ? '' : ` data-slot="${slot}"`;
  return piece(ops.slice(0, so.start)) +
    `<span class="so"${slotAttr}>${escapeHtml(ops.slice(so.start, so.end))}</span>` +
    piece(ops.slice(so.end));
}

/** Compact instruction rows for the hover card. */
export function renderInsnRows(insns, { startHex, prefs = {}, max = 12 } = {}) {
  const shown = insns.slice(0, max);
  const mode = prefs.asmAddr || 'abs';
  let out = '';
  for (const insn of shown) {
    out += `<div class="cr"><span class="aa">${escapeHtml(formatAddr(insn.address_hex, startHex, mode))}</span>` +
      (prefs.asmBytes === false ? '' : `<span class="ab">${escapeHtml(spacedBytes(insn.bytes))}</span>`) +
      `<span class="am">${escapeHtml(insn.mnemonic)}</span><span class="ao">${escapeHtml(insn.operands)}</span></div>`;
  }
  return out;
}

const attr = (name, value) => (value == null || value === '' ? '' : ` ${name}="${escapeHtml(String(value))}"`);

function clip(text, n = 56) {
  const t = (text || '').trim();
  return t.length > n ? t.slice(0, n - 1) + '…' : t;
}

/**
 * The assembly pane's HTML. `ctx`: `{prefs, codeLines, fnByAddr, nameOf,
 * slotOf, hints (Map addr -> label), patched (Set of addr)}`.
 */
export function renderAsm(fnData, ctx = {}) {
  const prefs = ctx.prefs || {};
  const insns = fnData.instructions || [];
  if (!insns.length) return '';
  const start = fnData.address_hex;
  const insnIndex = new Map(insns.map((insn, i) => [insn.address_hex, i]));
  const opCtx = { ...ctx, insnIndex };
  const arrows = prefs.asmArrows === false ? { lanes: 0, rows: [] } : branchArrows(insns);
  const codeLines = ctx.codeLines || (fnData.code || '').split('\n');
  const runs = groupRuns(insns);
  const hints = ctx.hints || new Map();
  const patched = ctx.patched || new Set();
  let out = `<div class="d2-chunk" style="--ag-w:${arrows.lanes ? arrows.lanes * LANE_W + 6 : 0}px">`;
  let row = 0;
  for (const run of runs) {
    const lineText = run.line ? codeLines[run.line - 1] : null;
    if (prefs.asmCMode === 'interleave' && run.line) {
      out += `<div class="d2-as" data-line="${run.line}"><b>L${run.lines.join(',')}</b>  ${escapeHtml(clip(lineText, 140))}</div>`;
    }
    for (let i = run.start; i < run.end; i++, row++) {
      if (row && row % CHUNK === 0) out += `</div><div class="d2-chunk" style="--ag-w:${arrows.lanes ? arrows.lanes * LANE_W + 6 : 0}px">`;
      const insn = insns[i];
      const band = run.line ? bandOf(run.line) : null;
      const comment = i === run.start && run.line && prefs.asmCMode !== 'interleave'
        ? `; L${run.lines.join(',')}: ${clip(lineText)}` : '';
      const hint = hints.get(insn.address_hex);
      const cls = ['d2-ar', run.line ? '' : 'nomap', patched.has(insn.address_hex) ? 'pa' : ''].filter(Boolean).join(' ');
      out += `<div class="${cls}" id="a-${insn.address_hex}" role="option" data-i="${i}"` +
        `${attr('data-addr', insn.address_hex)}${attr('data-lines', (insn.lines || []).join(' '))}${attr('data-band', band)}>` +
        `<span class="ag">${arrowSvg(arrows.rows[i] || [], arrows.lanes)}</span>` +
        `<span class="aa"><span class="abs">${escapeHtml(formatAddr(insn.address_hex, start, 'abs'))}</span>` +
        `<span class="rel">${escapeHtml(formatAddr(insn.address_hex, start, 'rel'))}</span></span>` +
        `<span class="ab" title="${escapeHtml(spacedBytes(insn.bytes))}">${escapeHtml(spacedBytes(insn.bytes))}</span>` +
        `<span class="am"${attr('data-mn', insn.mnemonic)}>${escapeHtml(insn.mnemonic)}</span>` +
        `<span class="ao">${linkOperands(insn, opCtx)}</span>` +
        `<span class="ac">${escapeHtml(comment)}${hint && prefs.asmHints !== false ? `<span class="ah">${escapeHtml(hint)}</span>` : ''}</span></div>`;
    }
  }
  out += '</div>';
  if (fnData.instructions_truncated) {
    out += '<div class="d2note">The listing stops here: the engine caps one function\'s rows.</div>';
  }
  return out;
}
