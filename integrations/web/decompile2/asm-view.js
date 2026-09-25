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

const REGISTER = new RegExp('^(?:' + [
  '[re]?(?:ax|bx|cx|dx|si|di|sp|bp|ip)', '[abcd][lh]', '(?:si|di|sp|bp)l', 'r(?:[89]|1[0-5])[dwb]?',
  '[xyz]mm(?:[0-9]|[12][0-9]|3[01])', '[cdefgs]s', 'st[0-7]?', 'cr[0-8]', 'dr[0-7]', 'k[0-7]', '[re]?flags', 'mm[0-7]',
  '[xw](?:[0-9]|[12][0-9]|30)', '[xw]zr', 'w?sp', 'lr', 'fp', 'pc', '[bhsdqv](?:[0-9]|[12][0-9]|3[01])', 'nzcv',
  'r(?:[0-9]|1[0-5])', 'ip', 'sb', 'sl', '[acs]psr',
].join('|') + ')$', 'i');
const PROTECTED = /<[^>]*>|"(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'/g;

function easyPlain(text) {
  return text
    .replace(/\s*\+\s*-\s*(?=0x|\d)/g, ' - ')
    .replace(/,(?! )/g, ', ')
    .replace(/\b[A-Za-z_]\w*\b/g, (word) => (REGISTER.test(word) ? word.toLowerCase() : word));
}

/**
 * Operands spelled the way objdump and gdb print them: register names in lower
 * case, a space after each comma, `+ -0x14` as `- 0x14`. Symbols in `<…>` and
 * string or character literals are left exactly as they are. Display only:
 * every lookup (links, stack slots, notes) reads the engine's own text.
 */
export function easyOperands(ops) {
  if (!ops) return ops || '';
  let out = '';
  let at = 0;
  for (const m of ops.matchAll(PROTECTED)) {
    out += easyPlain(ops.slice(at, m.index)) + m[0];
    at = m.index + m[0].length;
  }
  return out + easyPlain(ops.slice(at));
}

/** One instruction in the chosen spelling: `easy` (the default) or `exact`. Data rows stay as decoded. */
export function spellInsn(insn, spelling = 'easy') {
  const mnemonic = insn.mnemonic || '';
  if (spelling === 'exact' || mnemonic.startsWith('.')) {
    return { mnemonic, operands: insn.operands || '', text: insn.text || [mnemonic, insn.operands].filter(Boolean).join(' ') };
  }
  const m = mnemonic.toLowerCase();
  const operands = easyOperands(insn.operands || '');
  return { mnemonic: m, operands, text: operands ? `${m} ${operands}` : m };
}
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
  const easy = ctx.spelling !== 'exact' && !(insn.mnemonic || '').startsWith('.');
  const spell = (text) => escapeHtml(easy ? easyOperands(text) : text);
  const piece = (text) => {
    if (!linkable) return spell(text);
    return text.replace(/[^]*?\b(0x[0-9a-f]+)\b|[^]+/gi, (chunk, lit) => {
      if (!lit) return spell(chunk);
      const pre = chunk.slice(0, chunk.length - lit.length);
      const hex = '0x' + BigInt(lit).toString(16);
      const fn = ctx.fnByAddr?.get(hex);
      if (!fn && !ctx.insnIndex?.has(hex)) return spell(chunk);
      const label = fn && ctx.nameOf ? ` <span class="d2muted">&lt;${escapeHtml(ctx.nameOf(fn))}&gt;</span>` : '';
      return `${spell(pre)}<a class="xt" data-goto="${hex}">${escapeHtml(lit)}</a>${label}`;
    });
  };
  const so = stackOperand(ops);
  if (!so) return piece(ops);
  const slot = ctx.slotOf ? ctx.slotOf(so.reg, so.disp, insn) : null;
  const slotAttr = slot === null || slot === undefined ? '' : ` data-slot="${slot}"`;
  return piece(ops.slice(0, so.start)) +
    `<span class="so"${slotAttr}>${spell(ops.slice(so.start, so.end))}</span>` +
    piece(ops.slice(so.end));
}

const ARG_REG = /^(?:[RE]?(?:DI|SI|DX|CX)|DIL|SIL|DL|CL|R[89][DWB]?|XMM[0-7]|[WX][0-7]|[DSQ][0-7])$/i;
const FRAME_MEM = /\[\s*(?:[RE]BP|[RE]SP|SP|X29)\b/i;

/** Does `insn` belong to the function's opening frame setup (x86 / AArch64)? */
function isPrologueInsn(insn, family) {
  const m = (insn.mnemonic || '').toUpperCase();
  const ops = (insn.operands || '').replace(/\s+/g, '');
  const [dst, src] = ops.split(/,(?![^[]*\])/);
  if (family === 'x86') {
    if (/^ENDBR(32|64)$/.test(m) || m === 'NOP') return true;
    if (m === 'PUSH') return /^[RE]?[A-Z0-9]+$/i.test(ops);
    if (m === 'MOV' && /^[RE]BP,[RE]SP$/i.test(ops)) return true;
    if ((m === 'SUB' || m === 'AND') && /^[RE]SP,/i.test(ops)) return true;
    if (/^MOV/.test(m) && /FS:\[0x28\]/i.test(ops)) return true;
    if (/^(MOV|MOVSS|MOVSD|MOVAPS|MOVUPS|MOVQ|MOVD)$/.test(m) && FRAME_MEM.test(dst || '') && ARG_REG.test(src || '')) return true;
    return false;
  }
  if (family === 'aarch64') {
    const lm = m.toLowerCase();
    if (lm === 'bti' || lm === 'paciasp' || lm === 'nop') return true;
    if ((lm === 'stp' || lm === 'str' || lm === 'stur') && FRAME_MEM.test(ops)) return /^[wxqds]\d/i.test(ops);
    if (lm === 'mov' && /^x29,sp$/i.test(ops)) return true;
    if (lm === 'sub' && /^sp,sp/i.test(ops)) return true;
    return false;
  }
  return true;
}

const JUMP = /^(JMP|B|B\.AL)$/i;

/**
 * Attribute the instructions the engine left unmapped (`lines: []`) to a C
 * line, the way objdump -S reads them: an instruction between a mapped A (line
 * a) and the next mapped B (line b) belongs to b when b >= a (it sets up B),
 * else to a (it finishes A); an unconditional jump in that gap, and anything
 * before it, finishes A. Before the first mapped instruction the frame setup is
 * `prologue` and the rest sets up the first line; after the last one is
 * `epilogue`. Returns one `{lines, inferred, role}` per instruction.
 */
export function inferLines(instructions, family = 'x86') {
  const out = instructions.map((insn) => ({ lines: insn.lines || [], inferred: false, role: null }));
  const mapped = [];
  instructions.forEach((insn, i) => { if ((insn.lines || []).length) mapped.push(i); });
  if (!mapped.length) return out;
  const lo = (i) => Math.min(...instructions[i].lines);
  const hi = (i) => Math.max(...instructions[i].lines);
  const infer = (i, line) => { out[i] = { lines: [line], inferred: true, role: null }; };
  let p = 0;
  while (p < mapped[0] && isPrologueInsn(instructions[p], family)) out[p++].role = 'prologue';
  for (let i = p; i < mapped[0]; i++) infer(i, lo(mapped[0]));
  for (let k = 0; k + 1 < mapped.length; k++) {
    const a = mapped[k], b = mapped[k + 1];
    const la = hi(a), lb = lo(b);
    let cut = a;
    for (let i = a + 1; i < b; i++) if (JUMP.test(instructions[i].mnemonic || '')) cut = i;
    for (let i = a + 1; i < b; i++) infer(i, i <= cut || lb < la ? la : lb);
  }
  for (let i = mapped[mapped.length - 1] + 1; i < instructions.length; i++) out[i].role = 'epilogue';
  return out;
}

/** Compact instruction rows for the hover card. */
export function renderInsnRows(insns, { startHex, prefs = {}, max = 12, inferred = new Set() } = {}) {
  const shown = insns.slice(0, max);
  const mode = prefs.asmAddr || 'abs';
  let out = '';
  for (const insn of shown) {
    const sp = spellInsn(insn, prefs.asmSpelling);
    out += `<div class="cr${inferred.has(insn.address_hex) ? ' inf' : ''}"><span class="aa">${escapeHtml(formatAddr(insn.address_hex, startHex, mode))}</span>` +
      (prefs.asmBytes === false ? '' : `<span class="ab">${escapeHtml(spacedBytes(insn.bytes))}</span>`) +
      `<span class="am">${escapeHtml(sp.mnemonic)}</span><span class="ao">${escapeHtml(sp.operands)}</span></div>`;
  }
  return out;
}

const attr = (name, value) => (value == null || value === '' ? '' : ` ${name}="${escapeHtml(String(value))}"`);

function clip(text, n = 72) {
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
  const opCtx = { ...ctx, insnIndex, spelling: prefs.asmSpelling };
  const arrows = prefs.asmArrows === false ? { lanes: 0, rows: [] } : branchArrows(insns);
  const codeLines = ctx.codeLines || (fnData.code || '').split('\n');
  const inferred = ctx.inferred || null;
  const effective = inferred ? insns.map((insn, i) => ({ ...insn, lines: inferred[i].lines })) : insns;
  const runs = groupRuns(effective);
  const hints = ctx.hints || new Map();
  const patched = ctx.patched || new Set();
  const widest = (mode) => Math.max(...insns.map((insn) => formatAddr(insn.address_hex, start, mode).length));
  const chunkStyle = `--ag-w:${arrows.lanes ? arrows.lanes * LANE_W + 6 : 0}px;` +
    `--aw-abs:${widest('abs') + 3}ch;--aw-rel:${widest('rel') + 3}ch;--aw-both:${widest('both') + 3}ch`;
  let out = `<div class="d2-chunk" style="${chunkStyle}">`;
  const headings = prefs.asmCMode === 'heading' || prefs.asmCMode === 'interleave';
  let row = 0;
  for (const run of runs) {
    const lineText = run.line ? codeLines[run.line - 1] : null;
    if (headings && run.line) {
      out += `<div class="d2-as" data-line="${run.line}"><span class="asn">${run.lines.join(', ')}</span>` +
        `<span class="ast">${escapeHtml(clip(lineText, 160))}</span></div>`;
    }
    for (let i = run.start; i < run.end; i++, row++) {
      if (row && row % CHUNK === 0) out += `</div><div class="d2-chunk" style="${chunkStyle}">`;
      const insn = insns[i];
      const inf = inferred?.[i];
      const band = run.line ? bandOf(run.line) : null;
      const roleStart = inf?.role && (i === 0 || inferred[i - 1].role !== inf.role);
      if (headings && roleStart) {
        out += `<div class="d2-as role" data-role="${inf.role}">${inf.role === 'prologue' ? 'Function setup' : 'Function cleanup'}</div>`;
      }
      const role = roleStart && !headings ? `; ${inf.role}` : '';
      const comment = i === run.start && run.line && !headings
        ? `; L${run.lines.join(',')}: ${clip(lineText)}` : role;
      const hint = hints.get(insn.address_hex);
      const cls = ['d2-ar', run.line ? '' : 'nomap', patched.has(insn.address_hex) ? 'pa' : ''].filter(Boolean).join(' ');
      out += `<div class="${cls}" id="a-${insn.address_hex}" role="option" data-i="${i}"` +
        `${attr('data-addr', insn.address_hex)}${attr('data-lines', run.lines.join(' '))}${attr('data-band', band)}` +
        `${inf?.inferred ? ' data-inferred="1"' : ''}${attr('data-role', inf?.role)}${attr('title', insn.text)}>` +
        `<span class="ag">${arrowSvg(arrows.rows[i] || [], arrows.lanes)}</span>` +
        `<span class="aa"><span class="abs">${escapeHtml(formatAddr(insn.address_hex, start, 'abs'))}</span>` +
        `<span class="rel">${escapeHtml(formatAddr(insn.address_hex, start, 'rel'))}</span></span>` +
        `<span class="ab" title="${escapeHtml(spacedBytes(insn.bytes))}">${escapeHtml(spacedBytes(insn.bytes))}</span>` +
        `<span class="am"${attr('data-mn', insn.mnemonic)}>${escapeHtml(spellInsn(insn, prefs.asmSpelling).mnemonic)}</span>` +
        `<span class="ao"${attr('title', insn.operands)}>${linkOperands(insn, opCtx)}</span>` +
        `<span class="ac">${escapeHtml(comment)}${hint && prefs.asmHints !== false ? `<span class="ah">${escapeHtml(hint)}</span>` : ''}</span></div>`;
    }
  }
  out += '</div>';
  if (fnData.instructions_truncated) {
    out += '<div class="d2note">The listing stops here: the engine caps one function\'s rows.</div>';
  }
  return out;
}
