// decompile2-render.mjs — the study view's pure renderers, from the source
// tree with no build: the shared highlighter's scan, the C pane (token stream
// and regex fallback), the line/instruction/symbol index, and the diff that
// flashes edited lines.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { highlight, highlightC, highlightRust, scan, escapeHtml } from '../assets/js/highlight-c.js';
import {
  normalizeInspect, tokenLines, fallbackLines, lineSegments, buildIndex, renderC,
  changedLines, localDecls, storageLabel, bandOf,
} from '../decompile2/render-c.js';
import { addrHex, signedHex } from '../decompile2/addr.js';
import { normalizePrefs, cycle, DEFAULT_PREFS, loadPrefs, savePrefs } from '../decompile2/prefs.js';
import {
  formatAddr, groupRuns, linkOperands, branchArrows, renderAsm, renderInsnRows, stackOperand,
  isBranch, isCall, spacedBytes,
} from '../decompile2/asm-view.js';
import { placeOverlay } from '../decompile2/hover.js';
import { expand } from '../decompile2/sync.js';

const fixture = (name) => JSON.parse(readFileSync(new URL(`./fixtures/${name}`, import.meta.url), 'utf8'));
const checks = [];

// ── the shared highlighter: scan() is what highlight() renders ─────────────
const MAIN = 'int main(int argc,char **argv)\n{\n  long v1; // rax\n  v1 = sum_to(add(argc,3));\n  printf("%ld\\n",v1);\n  return (int)v1;\n}';
assert.equal(
  highlight(MAIN),
  '<span class="tok-type">int</span> <span class="tok-fn">main</span>(<span class="tok-type">int</span> argc,' +
  '<span class="tok-type">char</span> **argv)\n{\n  <span class="tok-type">long</span> v1; <span class="tok-com">// rax</span>\n' +
  '  v1 = <span class="tok-fn">sum_to</span>(<span class="tok-fn">add</span>(argc,<span class="tok-num">3</span>));\n' +
  '  <span class="tok-fn">printf</span>(<span class="tok-str">&quot;%ld\\n&quot;</span>,v1);\n' +
  '  <span class="tok-kw">return</span> (<span class="tok-type">int</span>)v1;\n}',
  'highlight() output is pinned byte for byte',
);
assert.equal(
  highlightRust('let mut x: u32 = 0x1f; /* c */'),
  '<span class="tok-kw">let</span> <span class="tok-kw">mut</span> x: <span class="tok-type">u32</span> = ' +
  '<span class="tok-num">0x1f</span>; <span class="tok-com">/* c */</span>',
);
const join = (pieces) => pieces.map((p) => (p.cls ? `<span class="${p.cls}">${escapeHtml(p.text)}</span>` : escapeHtml(p.text))).join('');
for (const code of [MAIN, readFileSync(new URL('./fixtures/sample.c', import.meta.url), 'utf8'), '"<&>"', '/* open', '']) {
  assert.equal(scan(code).map((p) => p.text).join(''), code, 'scan pieces cover the input exactly');
  assert.equal(join(scan(code)), highlight(code), 'highlight == join(scan)');
  assert.equal(join(scan(code, 'c')), highlightC(code), 'highlightC == join(scan c)');
}
checks.push('highlight pinned, == scan join');

// ── normalizeInspect: an inspect document and a plain decompile document ───
const inspectMain = normalizeInspect(fixture('inspect-main.json'));
assert.equal(inspectMain.source, 'inspect');
assert.equal(inspectMain.address_hex, '0x1198');
assert.equal(inspectMain.hasInstructions, true);
assert.equal(inspectMain.instructions[9].address_hex, '0x11b5');
assert.deepEqual(inspectMain.instructions[9].lines, [5]);
const plain = normalizeInspect({
  binary: 'x', count: 1,
  functions: [{ name: 'main', address: 4504, address_hex: '0x1198', code: MAIN, error: null, variables: [] }],
});
assert.equal(plain.source, 'decompile');
assert.equal(plain.hasInstructions, false);
assert.match(plain.tokens_error, /inspect/);
assert.equal(plain.proto, 'int main(int argc,char **argv)');
checks.push('normalizeInspect (inspect + decompile)');

// ── token stream: round trip, per-line fallback, no-token path ─────────────
/** A synthetic engine token stream for `code` (what the engine lane emits). */
function tokensFor(code) {
  const out = [];
  code.split('\n').forEach((text, i) => {
    let col = 0;
    for (const p of scan(text)) {
      if (p.text.trim()) {
        const kind = p.cls === 'tok-type' ? 'type' : p.cls === 'tok-fn' ? 'funcname' : p.word && !p.cls ? 'variable' : 'syntax';
        out.push({ line: i + 1, col, len: p.text.length, kind, color: 'no_color', text: p.text });
      }
      col += p.text.length;
    }
  });
  return out;
}
const toks = tokensFor(MAIN);
const exact = tokenLines(MAIN, toks);
exact.forEach((segs, i) => {
  assert.ok(segs, `line ${i + 1} verifies`);
  assert.equal(segs.map((s) => s.text).join(''), MAIN.split('\n')[i], `line ${i + 1} segments rebuild it`);
});
const drift = toks.map((t) => (t.line === 4 && t.text === 'sum_to' ? { ...t, col: t.col + 1 } : t));
const partly = tokenLines(MAIN, drift);
assert.equal(partly[3], null, 'a drifted token fails its line');
assert.ok(partly[4], 'other lines keep their tokens');
const withTokens = { ...plain, tokens: drift, tokens_error: null };
const seg = lineSegments(withTokens);
assert.equal(seg.fallbackCount, 1, 'exactly the drifted line falls back');
const html = renderC(withTokens, { segs: seg.segs });
const text = html.replace(/<[^>]+>/g, '').replace(/&lt;/g, '<').replace(/&gt;/g, '>').replace(/&quot;/g, '"').replace(/&amp;/g, '&');
assert.ok(text.includes('v1 = sum_to(add(argc,3));'), 'the fallback line still renders its text');
const fb = fallbackLines(MAIN);
assert.equal(fb.length, 7);
assert.ok(fb[3].some((s) => s.tok?.kind === 'variable' && s.text === 'v1'), 'fallback names variables');
assert.ok(fb[3].some((s) => s.tok?.kind === 'funcname' && s.text === 'sum_to'), 'fallback names callees');
assert.ok(fallbackLines('p->len = s.x;')[0].filter((s) => s.tok?.kind === 'field').length === 2, 'fields after -> and .');
checks.push('tokenLines round trip + per-line fallback');

// ── renderC: rows, gutters, attributes, escaping ───────────────────────────
const mainSeg = lineSegments(inspectMain);
const mainIndex = buildIndex(inspectMain, mainSeg.segs);
const fnByName = new Map([['sum_to', { address_hex: '0x1161' }], ['add', { address_hex: '0x1149' }]]);
const mainHtml = renderC(inspectMain, { segs: mainSeg.segs, index: mainIndex, fnByName });
assert.match(mainHtml, /<div class="d2-cl" id="c-L5" role="option" data-line="5" data-addrs="0x11b5 0x11c2" data-band="5">/);
assert.match(mainHtml, /<span class="la">11b5<\/span>/);
assert.match(mainHtml, /data-kind="funcname"[^>]*data-callee="0x1161"[^>]*>sum_to</);
assert.match(mainHtml, /data-kind="variable" data-sym="v1"/);
const evil = normalizeInspect({ functions: [{ name: '<x>', address_hex: '0x1', code: 'int f(void)\n{\n  g("<img src=x onerror=alert(1)>");\n}' }] });
const evilHtml = renderC(evil);
assert.ok(!evilHtml.includes('<img'), 'engine strings are escaped');
assert.ok(evilHtml.includes('&lt;img'), 'escaped text survives');
const errored = renderC(normalizeInspect({ functions: [{ name: 'f', address_hex: '0x1', code: null, error: 'boom <b>' }] }));
assert.match(errored, /decompile error:\n   boom &lt;b&gt;/);
checks.push('renderC rows/attributes/escaping');

// ── buildIndex: every source of line mappings is merged ────────────────────
assert.deepEqual(mainIndex.lineToInsns.get(5), ['0x11b5', '0x11c2']);
assert.deepEqual(mainIndex.insnToLines.get('0x11e1'), [6]);
assert.equal(mainIndex.addrToInsn.get('0x11b5').mnemonic, 'CALL');
assert.deepEqual([...mainIndex.symToLines.get('v1')].sort(), [3, 5, 6, 7]);
const sumTo = normalizeInspect(fixture('inspect-sum_to.json'));
const sumIndex = buildIndex(sumTo);
assert.deepEqual(sumIndex.lineToInsns.get(7), ['0x1174', '0x1186', '0x1190']);
assert.ok(sumIndex.symToAddrs.get('acc').has('0x116c'), 'variable addresses join the symbol index');
checks.push('buildIndex');

// ── declarations, storage, bands, diff, addresses, prefs ──────────────────
assert.deepEqual(localDecls(sumTo.code).map((d) => [d.name, d.type, d.storage]), [['acc', 'long', 'stack - 0x10'], ['v1', 'int', 'stack - 0x14']]);
assert.deepEqual(localDecls('void f(void)\n{\n  char v2 [16]; // stack - 0x18\n  \n}').map((d) => d.type), ['char [16]']);
assert.equal(storageLabel('rax'), 'register RAX');
assert.equal(storageLabel('stack - 0x14'), 'stack entry−0x14');
assert.equal(bandOf(5), 5);
assert.equal(bandOf(6), 0);
assert.deepEqual([...changedLines('a\nb\nc', 'a\nB\nc\nd')], [2, 4]);
assert.deepEqual([...changedLines(MAIN, MAIN.replace(/v1/g, 'total'))], [3, 4, 5, 6]);
assert.equal(changedLines('', 'x').size, 0, 'nothing flashes on a first render');
assert.equal(addrHex(4504), '0x1198');
assert.equal(addrHex('0X11AB'), '0x11ab');
assert.equal(addrHex('0xffffffff81000000'), '0xffffffff81000000');
assert.equal(addrHex(2 ** 60), null, 'an unsafe number is refused, not rounded');
assert.equal(signedHex(-4n), '-0x4');
assert.deepEqual(normalizePrefs({ v: 1, asmAddr: 'nope', hoverDelay: 250, split: 'yes' }), { ...DEFAULT_PREFS, hoverDelay: 250 });
assert.equal(cycle(DEFAULT_PREFS, 'asmAddr').asmAddr, 'rel');
assert.equal(cycle({ ...DEFAULT_PREFS, asmAddr: 'both' }, 'asmAddr').asmAddr, 'abs');
const mem = new Map();
const store = { getItem: (k) => mem.get(k) ?? null, setItem: (k, v) => mem.set(k, v) };
savePrefs(store, { ...DEFAULT_PREFS, asmBytes: false });
assert.equal(loadPrefs(store).asmBytes, false);
assert.equal(loadPrefs({ getItem: () => { throw new Error('denied'); } }).asmBytes, true, 'a throwing store gives defaults');
checks.push('decls/storage/bands/diff/addr/prefs');

// ── the assembly pane ──────────────────────────────────────────────────────
assert.equal(formatAddr('0x11ab', '0x1198', 'abs'), '11ab');
assert.equal(formatAddr('0x11ab', '0x1198', 'rel'), '+0x13');
assert.equal(formatAddr('0x1190', '0x1198', 'rel'), '-0x8', 'offsets may be negative');
assert.equal(formatAddr('0x11ab', '0x1198', 'both'), '11ab +0x13');
assert.equal(
  formatAddr('0xffffffff81000010', '0xffffffff81000000', 'both'),
  'ffffffff81000010 +0x10',
  'addresses above 2^53 stay exact (BigInt)',
);
assert.equal(spacedBytes('8b45ec'), '8b 45 ec');
assert.ok(isBranch('JLE') && isBranch('JMP') && isBranch('b.ne') && !isBranch('CALL') && !isBranch('BL') && isCall('CALL') && isCall('bl'));
assert.deepEqual(groupRuns(sumTo.instructions).map((r) => [r.line, r.start, r.end]), [
  [null, 0, 4], [6, 4, 5], [7, 5, 6], [null, 6, 8], [8, 8, 10], [7, 10, 11], [null, 11, 13], [7, 13, 14], [10, 14, 15], [null, 15, 16], [10, 16, 17],
]);
const arrows = branchArrows(sumTo.instructions);
assert.deepEqual(arrows.edges, [{ from: 6, to: 11, lane: 0 }, { from: 13, to: 7, lane: 1 }], 'JMP forward, JLE back, nested lanes');
assert.deepEqual(arrows.rows[6], [{ lane: 0, kind: 'from', up: false }]);
assert.deepEqual(arrows.rows[9].map((s) => s.kind), ['pass', 'pass']);
assert.equal(branchArrows(inspectMain.instructions).lanes, 0, 'straight-line code has no arrows');
assert.deepEqual(stackOperand('dword ptr [RBP + -0x14],EDI'), { reg: 'RBP', disp: -20, start: 0, end: 23 });
assert.equal(stackOperand('RAX,[0x2004]'), null);
const fnByAddr = new Map([['0x1149', { name: 'add' }], ['0x1161', { name: 'sum<to>' }]]);
const call = linkOperands({ mnemonic: 'CALL', operands: '0x1161' }, { fnByAddr, nameOf: (f) => f.name });
assert.equal(call, '<a class="xt" data-goto="0x1161">0x1161</a> <span class="d2muted">&lt;sum&lt;to&gt;&gt;</span>', 'callee names are escaped');
const jump = linkOperands({ mnemonic: 'JLE', operands: '0x117d' }, { insnIndex: new Map([['0x117d', 7]]) });
assert.equal(jump, '<a class="xt" data-goto="0x117d">0x117d</a>', 'in-function targets link');
assert.equal(linkOperands({ mnemonic: 'MOV', operands: 'ESI,0x1149' }, { fnByAddr }), 'ESI,0x1149', 'immediates are not guessed to be code');
assert.equal(
  linkOperands({ mnemonic: 'MOV', operands: 'dword ptr [RBP + -0x14],EDI' }, { slotOf: (reg, disp) => disp - 8 }),
  '<span class="so" data-slot="-28">dword ptr [RBP + -0x14]</span>,EDI',
);
assert.equal(linkOperands({ mnemonic: 'MOV', operands: 'EAX,<b>' }), 'EAX,&lt;b&gt;', 'operands are escaped');
const asmHtml = renderAsm(sumTo, { prefs: { asmCMode: 'comment' }, fnByAddr });
assert.match(asmHtml, /<div class="d2-ar" id="a-0x116c" role="option" data-i="4" data-addr="0x116c" data-lines="6" data-band="0">/);
assert.match(asmHtml, /<span class="ac">; L6: acc = 0;<\/span>/);
assert.match(asmHtml, /id="a-0x1161"[^>]*>/);
assert.match(asmHtml, /class="d2-ar nomap" id="a-0x1161"/);
assert.equal((asmHtml.match(/class="d2-ar/g) || []).length, 17);
const inter = renderAsm(sumTo, { prefs: { asmCMode: 'interleave' } });
assert.match(inter, /<div class="d2-as" data-line="6"><b>L6<\/b> {2}acc = 0;<\/div>/, 'interleave puts the C line above its run');
assert.ok(!/; L6/.test(inter), 'interleave drops the comment column text');
assert.equal(renderAsm(plain, {}), '', 'no instructions, no rows');
const cardRows = renderInsnRows(sumTo.instructions, { startHex: sumTo.address_hex, prefs: { asmAddr: 'rel', asmBytes: false }, max: 2 });
assert.equal(cardRows, '<div class="cr"><span class="aa">+0x0</span><span class="am">ENDBR64</span><span class="ao"></span></div>' +
  '<div class="cr"><span class="aa">+0x4</span><span class="am">PUSH</span><span class="ao">RBP</span></div>');
checks.push('formatAddr/groupRuns/branchArrows/linkOperands/renderAsm');

// ── hover placement and sync expansion ─────────────────────────────────────
const vp = { width: 1000, height: 800 };
assert.deepEqual(placeOverlay({ left: 100, top: 100, right: 300, bottom: 120 }, { width: 200, height: 100 }, vp), { left: 100, top: 124, placement: 'below' });
assert.deepEqual(placeOverlay({ left: 100, top: 700, right: 300, bottom: 720 }, { width: 200, height: 100 }, vp), { left: 100, top: 596, placement: 'above' });
assert.equal(placeOverlay({ left: 950, top: 10, right: 990, bottom: 30 }, { width: 200, height: 100 }, vp).left, 792, 'clamped inside the right edge');
assert.equal(placeOverlay({ left: -40, top: 10, right: 0, bottom: 30 }, { width: 200, height: 100 }, vp).left, 8, 'clamped inside the left edge');
const tall = placeOverlay({ left: 10, top: 300, right: 50, bottom: 320 }, { width: 100, height: 700 }, vp);
assert.ok(tall.top >= 8 && tall.top + 700 <= 800 - 8 + 700, 'a card taller than both sides is clamped');
const sets = expand({ line: 5 }, mainIndex);
assert.deepEqual([...sets.addrs], ['0x11b5', '0x11c2']);
assert.deepEqual([...expand({ addr: '0x11e1' }, mainIndex).lines], [6]);
assert.ok(expand({ sym: 'v1' }, mainIndex).lines.has(7));
assert.equal(expand(null, mainIndex).lines.size, 0);
checks.push('placeOverlay/expand');

console.log(`DECOMPILE2 RENDER OK — ${checks.join('; ')}`);
