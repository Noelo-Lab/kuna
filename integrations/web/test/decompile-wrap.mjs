// decompile-wrap.mjs — the C pane's line wrapping, kuna's end-of-line notes
// in words, the `x` dialog's helpers and instruction grouping by basic block,
// from the source tree (no build, no browser).
//
//   node integrations/web/test/decompile-wrap.mjs
import assert from 'node:assert/strict';
import { wrapPlan, applyPlan } from '../decompile/wrap-c.js';
import { lineHtml } from '../decompile/render-c.js';
import { lineTags, commentStart } from '../decompile/tags.js';
import { useKind, renderRefsDialog } from '../decompile/xrefs-view.js';
import { inferLines } from '../decompile/asm-view.js';

const checks = [];

// ── wrapping (the Hex-Rays rules) ──────────────────────────────────────────
const lay = (text, margin) => applyPlan(text, wrapPlan(text, margin)).split('\n');
const BSWAP = '  return (unsigned int)a0 & 0xffff0000 | (unsigned int)((unsigned short)a0 << 8 | (unsigned short)a0 >> 8) & 0xffff;';
assert.deepEqual(wrapPlan(BSWAP, 60).breaks, [], 'arithmetic never splits: it soft-wraps instead');
assert.deepEqual(wrapPlan('  x = 1;', 80).breaks, [], 'a line that fits is left alone');
const GUARD = '        if ((((*a0 == 3) && (a0[1] == 3)) && (*(unsigned char *)&a0[2] & 0x20)) && (a0[6] == 10)) {';
assert.deepEqual(wrapPlan(GUARD, 120).breaks, [], 'within the 120-column margin nothing splits');
assert.deepEqual(lay(GUARD, 60), [
  '        if ((*a0 == 3)',
  '            && (a0[1] == 3)',
  '            && (*(unsigned char *)&a0[2] & 0x20)',
  '            && (a0[6] == 10)) {',
], 'a chain splits before each operator, its redundant brackets dropped');
const FOLD = '        if ((a0[6] == 10 && (v1 = *a2, v2 = a2[1], v3 = a2[2], sub_4012f9(a0) == ((unsigned int)v3 << 0x10 | (unsigned int)v1))) || (dat_1 == 2)) {';
assert.deepEqual(lay(FOLD, 70), [
  '        if ((a0[6] == 10',
  '             && (v1 = *a2,',
  '                 v2 = a2[1],',
  '                 v3 = a2[2],',
  '                 sub_4012f9(a0) == ((unsigned int)v3 << 0x10 | (unsigned int)v1)))',
  '            || (dat_1 == 2)) {',
], 'a nested chain lines up after its bracket, a comma list one element per row');
const CALL = '  __fprintf_chk(v3,1,dcgettext(NULL,"License GPLv3+: GNU GPL version 3 or later <%s>.\\n",5),"https://gnu.org/licenses/gpl.html");';
assert.deepEqual(lay(CALL, 80), [
  '  __fprintf_chk(',
  '    v3,',
  '    1,',
  '    dcgettext(NULL,"License GPLv3+: GNU GPL version 3 or later <%s>.\\n",5),',
  '    "https://gnu.org/licenses/gpl.html");',
], 'a call puts each argument on its row, two columns in from its name; an argument that fits stays whole');
const SIG = 'unsigned long sub_401435(int *a0,unsigned char a1,unsigned char *a2) // early-return x10';
assert.deepEqual(lay(SIG, 50), [
  'unsigned long sub_401435(',
  '        int *a0,',
  '        unsigned char a1,',
  '        unsigned char *a2) // early-return x10',
], 'a signature puts each parameter eight columns in; the comment rides on the last row');
assert.deepEqual(lay('  (*dat_403fd8)(main,v2,0,0,a2,some_long_argument_name,another_long_argument_name);', 60).slice(0, 2),
  ['  (*dat_403fd8)(', '    main,'], 'a call through a pointer is a call');
assert.deepEqual(lay('  v = (**(code **)(p + 8))(v56,v36,dat_3d9da || dat_3da90,another_long_argument_name);', 60), [
  '  v = (**(code **)(p + 8))(',
  '        v56,',
  '        v36,',
  '        dat_3d9da || dat_3da90,',
  '        another_long_argument_name);',
], 'commas split before a || inside an argument');
const str = '  puts("a string, with commas && operators that is far too long to fit on one row of the pane");';
assert.deepEqual(wrapPlan(str, 40).breaks, [], 'a string never splits');
for (const [text, margin] of [[GUARD, 60], [FOLD, 70], [CALL, 80], [SIG, 50]]) {
  const plan = wrapPlan(text, margin);
  assert.ok(plan.breaks.every((b) => b.eat === 0 || text[b.at] === ' '), 'a break only eats a space');
  assert.ok([...plan.drops].every((d) => text[d] === '(' || text[d] === ')'), 'only brackets are dropped');
}
const segs = [
  { text: '  ', tok: null }, { text: 'f', tok: { kind: 'funcname', text: 'f' } }, { text: '(', tok: { kind: 'syntax', text: '(' } },
  { text: 'a', tok: { kind: 'variable', text: 'a' } }, { text: ' ', tok: null }, { text: '&&', tok: { kind: 'syntax', text: '&&' } },
  { text: ' ', tok: null }, { text: 'b', tok: { kind: 'variable', text: 'b' } }, { text: ',', tok: { kind: 'syntax', text: ',' } },
  { text: 'c', tok: { kind: 'variable', text: 'c' } }, { text: ');', tok: { kind: 'syntax', text: ');' } },
];
const rowsOf = (h) => h.split('<span class="wl" ').slice(1).map((r) => [/--i:(\d+)/.exec(r)[1],
  r.split('<').map((part, i) => (i ? part.slice(part.indexOf('>') + 1) : '')).join('').split('&amp;').join('&')]);
assert.deepEqual(rowsOf(lineHtml(segs, {}, { breaks: [{ at: 5, eat: 1, indent: 6 }, { at: 11, eat: 0, indent: 4 }], drops: new Set() })),
  [['2', 'f(a'], ['6', '&& b,'], ['4', 'c);']], 'each row is a block with its indent; the indent spaces become padding');
assert.equal((lineHtml(segs, {}, { breaks: [], drops: new Set() }).match(/data-sym="a"/g) || []).length, 1, 'tokens stay whole');
assert.deepEqual(rowsOf(lineHtml(segs, {}, { breaks: [{ at: 7, eat: 0, indent: 2 }], drops: new Set([3]) })), [['2', 'fa && b,c);']],
  'a break inside a token is left out; a dropped bracket is gone');
assert.ok(!/class="wl"/.test(lineHtml(segs, {}, null)), 'no plan: the line renders as before');
checks.push('wrapPlan/applyPlan/lineHtml');

// ── kuna's end-of-line notes ───────────────────────────────────────────────
assert.equal(commentStart('  puts("a // b"); // no-return'), 18, 'a // inside a string is not a comment');
assert.deepEqual(lineTags('    if (0x31 <= a1) { // branch-flip').map((t) => t.tag), ['branch-flip']);
assert.match(lineTags('    if (0x31 <= a1) { // branch-flip')[0].words, /flipped this if/);
assert.match(lineTags('unsigned long f(int *a0) // early-return x10')[0].words, /10 return statements/);
assert.deepEqual(lineTags('  long v1; // rax'), [], 'a register comment is not a note');
assert.deepEqual(lineTags('  usage(1); // no-return, warn: odd, really, here, branch-flip').map((t) => t.tag),
  ['no-return', 'warn: odd, really, here', 'branch-flip'], 'a warning keeps its commas');
assert.match(lineTags('  x = 1; // jt: return-dupe x2')[0].words, /^About this switch: the machine code/);
assert.match(lineTags('  f(); // inlined: memcpy')[0].words, /memcpy/);
checks.push('lineTags');

// ── the x dialog ───────────────────────────────────────────────────────────
assert.equal(useKind('  v1 = *a2;', 'v1'), 'sets');
assert.equal(useKind('  if (v1 == 3) {', 'v1'), 'reads');
assert.equal(useKind('  v3 += 1;', 'v3'), 'sets');
assert.equal(useKind('  v3++;', 'v3'), 'sets');
assert.equal(useKind('  f(v3 <= 2);', 'v3'), 'reads');
assert.equal(useKind('  long v1;', 'v1', { decl: true }), 'declares');
const dlg = renderRefsDialog({
  title: 'Uses of <v1>', sections: [
    { heading: 'In main', kind: 'lines', rows: [{ line: 5, how: 'sets', text: 'v1 = <x>;' }] },
    { heading: 'Called by', kind: 'sites', rows: [], loading: 'Looking…' },
    { heading: 'Calls', kind: 'sites', rows: [{ fn: '0x10', site: '', siteLabel: '10', name: 'f', kind: 'call', instruction: 'CALL 0x10' }] },
  ],
});
assert.match(dlg, /<li class="xr-row" tabindex="-1" data-line="5"/);
assert.match(dlg, /Uses of &lt;v1&gt;/);
assert.match(dlg, /v1 = &lt;x&gt;;/);
assert.match(dlg, /Looking…/);
assert.match(dlg, /data-fn="0x10" data-site=""/);
checks.push('useKind/renderRefsDialog');

// ── grouping by basic block, from the engine's flow facts ──────────────────
const I = (addr, text, lines, flow = null, targets = []) => {
  const [mnemonic, ...rest] = text.split(' ');
  return { address_hex: addr, mnemonic, operands: rest.join(' '), text, lines, flow, targets_hex: targets };
};
const fn = [
  I('0x0', 'PUSH RBP', []), I('0x1', 'MOV RBP,RSP', []), I('0x4', 'PUSH RBX', []), I('0x5', 'MOV RBX,RDI', []),
  I('0x8', 'CMP ESI,0x1', [4]), I('0xb', 'JZ 0x20', [], 'cjump', ['0x20']),
  I('0xd', 'LEA RDI,[0x400]', []), I('0x14', 'CALL 0x100', [5], 'call', ['0x100']), I('0x19', 'MOV EAX,0x1', []), I('0x1e', 'JMP 0x30', [], 'jump', ['0x30']),
  I('0x20', 'LEA RDI,[0x410]', []), I('0x27', 'CALL 0x100', [8], 'call', ['0x100']), I('0x2c', 'MOV EAX,0x0', []),
  I('0x30', 'POP RBX', []), I('0x31', 'POP RBP', []), I('0x32', 'RET', [6, 9], 'return'),
];
const rows = inferLines(fn, 'x86', { sigLine: 1, endLine: 10 });
const show = (r) => (r.role ? `${r.role}:` : '') + (r.inferred ? '~' : '') + r.lines.join(',');
assert.deepEqual(rows.map(show), [
  'prologue:~1', 'prologue:~1', 'prologue:~1', 'prologue:~1',
  '4', '~4',
  '~5', '5', '~6', '~6',
  '~8', '8', '~9',
  'epilogue:~6,9', 'epilogue:~6,9', '6,9',
], 'the frame set-up is the signature; a jump finishes its test; the return value before a shared epilogue is the return it reaches');
assert.ok(rows.every((r) => r.lines.length), 'every instruction has a line');
const voidFn = [I('0x0', 'PUSH RBP', []), I('0x1', 'CALL 0x100', [3], 'call', ['0x100']), I('0x6', 'POP RBP', []), I('0x7', 'RET', [], 'return')];
assert.deepEqual(inferLines(voidFn, 'x86', { sigLine: 1, endLine: 4 }).map(show), ['prologue:~1', '3', 'epilogue:~4', '~4'],
  'a return the engine left unmapped is the closing brace');
checks.push('inferLines by basic block');

console.log(`DECOMPILE WRAP OK — ${checks.join('; ')}`);
