// decompile-wrap.mjs — the C pane's line wrapping, kuna's end-of-line notes
// in words, the `x` dialog's helpers and instruction grouping by basic block,
// from the source tree (no build, no browser).
//
//   node integrations/web/test/decompile-wrap.mjs
import assert from 'node:assert/strict';
import { wrapBreaks, applyBreaks } from '../decompile/wrap-c.js';
import { lineHtml } from '../decompile/render-c.js';
import { lineTags, commentStart } from '../decompile/tags.js';
import { useKind, renderRefsDialog } from '../decompile/xrefs-view.js';
import { inferLines } from '../decompile/asm-view.js';

const checks = [];

// ── wrapping ───────────────────────────────────────────────────────────────
const LONG = '        if ((((*a0 == 3) && (a0[1] == 3)) && (*(unsigned char *)&a0[2] & 0x20)) && ((a0[6] == 10 && ' +
  '(v1 = *a2, v2 = a2[1], v3 = a2[2], v4 = a2[3], sub_4012f9(a0) == ((unsigned int)v3 << 0x10 | (unsigned int)v4 << 0x18 | ' +
  '(unsigned int)v1 | (unsigned int)v2 << 8))))) {';
/** Undo the breaks: every row's indent goes, every eaten character comes back. */
const unwrap = (text, breaks) => {
  let out = '';
  let at = 0;
  for (const b of breaks) {
    out += text.slice(at, b.at) + text.slice(b.at, b.at + b.eat);
    at = b.at + b.eat;
  }
  return out + text.slice(at);
};
assert.deepEqual(wrapBreaks('  x = 1;', 80), [], 'a line that fits is left alone');
for (const cols of [120, 90, 70, 50]) {
  const breaks = wrapBreaks(LONG, cols);
  const rows = applyBreaks(LONG, breaks).split('\n');
  assert.ok(rows.length > 1, `${cols} columns: the line wraps`);
  assert.ok(rows.every((r) => r.length <= cols), `${cols} columns: every row fits`);
  assert.equal(unwrap(LONG, breaks), LONG, `${cols} columns: only spaces are replaced`);
  assert.ok(breaks.every((b) => b.eat === 0 || LONG[b.at] === ' '), 'a break only eats a space');
}
assert.deepEqual(applyBreaks(LONG, wrapBreaks(LONG, 90)).split('\n').slice(0, 3), [
  '        if ((((*a0 == 3) && (a0[1] == 3)) && (*(unsigned char *)&a0[2] & 0x20)) &&',
  '            ((a0[6] == 10 &&',
  '              (v1 = *a2,',
], 'the condition breaks at its outer && first, aligned after the opening bracket');
const assign = '    v5 = some_long_function_name(first_argument_value + 0x10,second_argument_value,(char *)third);';
assert.deepEqual(applyBreaks(assign, wrapBreaks(assign, 80)).split('\n'), [
  '    v5 = some_long_function_name(first_argument_value + 0x10,',
  '                                 second_argument_value,',
  '                                 (char *)third);',
], 'an assignment keeps its = when breaking the call fits');
const ternary = '  v3 = (v1 < 0x10) ? lookup_table_entry_for_the_value(v1,v2) : default_value_function_name(v2,v1,0);';
assert.deepEqual(applyBreaks(ternary, wrapBreaks(ternary, 60)).split('\n'), [
  '  v3 = (v1 < 0x10) ?',
  '      lookup_table_entry_for_the_value(v1,v2) :',
  '      default_value_function_name(v2,v1,0);',
], 'a ternary breaks at ? and :, not inside a call');
const str = '  puts("a string, with commas && operators that is far too long to fit on one row of the pane");';
assert.ok(wrapBreaks(str, 40).every((b) => b.at < 7 || b.at > str.length - 3), 'a string never breaks');
const cmt = '  f(a,b); // a comment, with commas && operators && more text';
assert.deepEqual(wrapBreaks(cmt, 20), [{ at: 6, eat: 0, indent: 4 }], 'a comment never breaks');
const segs = [
  { text: '  ', tok: null }, { text: 'f', tok: { kind: 'funcname', text: 'f' } }, { text: '(', tok: { kind: 'syntax', text: '(' } },
  { text: 'a', tok: { kind: 'variable', text: 'a' } }, { text: ' ', tok: null }, { text: '&&', tok: { kind: 'syntax', text: '&&' } },
  { text: ' ', tok: null }, { text: 'b', tok: { kind: 'variable', text: 'b' } }, { text: ',', tok: { kind: 'syntax', text: ',' } },
  { text: 'c', tok: { kind: 'variable', text: 'c' } }, { text: ');', tok: { kind: 'syntax', text: ');' } },
];
const plain = (h) => h.split('<').map((part, i) => (i ? part.slice(part.indexOf('>') + 1) : part)).join('').split('&amp;').join('&');
const html = lineHtml(segs, {}, [{ at: 8, eat: 1, indent: 4 }, { at: 11, eat: 0, indent: 4 }]);
assert.equal(plain(html), '  f(a &&\n    b,\n    c);', 'breaks become a newline and the indent, tokens stay whole');
assert.equal((html.match(/data-sym="a"/g) || []).length, 1);
assert.equal(plain(lineHtml(segs, {}, [{ at: 6, eat: 0, indent: 2 }])), '  f(a \n  && b,c);', 'a break before a token');
assert.equal(plain(lineHtml(segs, {}, [{ at: 7, eat: 0, indent: 2 }])), '  f(a && b,c);', 'a break inside a token is left out');
checks.push('wrapBreaks/applyBreaks/lineHtml');

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
