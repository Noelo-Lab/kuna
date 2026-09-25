// decompile2-learn.mjs — the study view's teaching aids, from the source tree
// with no build: mnemonic explanations (every mnemonic in the fixtures is
// covered), compiler idioms, the stack-frame model and its callouts, stack
// operands, references and the help text.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { explain, idioms, TABLES } from '../decompile2/mnemonics.js';
import { frameModel, operandSlot, callouts, renderFrame, slotIndex, prologue, belowText } from '../decompile2/stack-frame.js';
import { renderXrefs, renderLocalCalls, localCallees, joinWords } from '../decompile2/xrefs-view.js';
import { helpHtml, HELP_KEYS, HELP_GLOSSARY } from '../decompile2/help.js';
import { normalizeInspect } from '../decompile2/render-c.js';

const fx = (name) => normalizeInspect(JSON.parse(readFileSync(new URL(`./fixtures/inspect-${name}.json`, import.meta.url), 'utf8')));
const main = fx('main');
const sumTo = fx('sum_to');
const add = fx('add');
const checks = [];
const X86 = { family: 'x86', bits: 64 };

// ── mnemonics ──────────────────────────────────────────────────────────────
for (const fn of [main, sumTo, add]) {
  for (const insn of fn.instructions) assert.ok(explain(insn.mnemonic, 'x86'), `${insn.mnemonic} is explained`);
}
assert.match(explain('JLE'), /^jump if less or equal \(signed <=\)/);
assert.match(explain('jae'), /^jump if above or equal \(unsigned >=\)/, 'case-insensitive');
assert.match(explain('SETNE'), /^set the byte to 1 if not equal/);
assert.match(explain('CMOVL'), /^copy only if less \(signed <\)/);
assert.match(explain('REP STOSQ'), /^repeat RCX times: store RAX/);
assert.match(explain('LOCK CMPXCHG'), /^atomically: compare with RAX/);
assert.match(explain('b.ne', 'aarch64'), /^branch if not equal/);
assert.match(explain('LDR', 'aarch64'), /^load a register/);
assert.match(explain('addeq', 'arm'), /^add two values, only if equal$/);
assert.match(explain('subs', 'aarch64'), /set the flags/);
assert.match(explain('movs', 'arm'), /and set the flags$/);
assert.equal(explain('FROBNICATE'), null);
assert.equal(explain(''), null);
for (const [name, table] of Object.entries(TABLES)) {
  for (const [k, v] of Object.entries(table)) assert.ok(typeof v === 'string' && v.length >= 3, `${name}.${k} has text`);
}
checks.push('explain (every fixture mnemonic, Jcc/SETcc/CMOVcc, prefixes, ARM conditions)');

// ── idioms ─────────────────────────────────────────────────────────────────
assert.deepEqual([...idioms(sumTo.instructions)], [
  ['0x1161', 'landing pad (CET)'],
  ['0x1165', 'prologue: save the caller\'s frame pointer'],
  ['0x1166', 'prologue: RBP now marks this frame'],
  ['0x1180', 'int to long: sign-extend EAX into RAX'],
  ['0x1196', 'epilogue: restore the caller\'s frame pointer'],
  ['0x1197', 'return to the caller'],
]);
const mainIdioms = idioms(main.instructions);
assert.equal(mainIdioms.get('0x11a0'), 'prologue: reserve 32 bytes for locals');
assert.equal(mainIdioms.get('0x11dc'), 'variadic call: AL = 0 vector registers used');
assert.equal(mainIdioms.get('0x11ea'), 'epilogue: RSP = RBP; POP RBP');
const synth = [
  { address_hex: '0x10', mnemonic: 'MOV', operands: 'RAX,qword ptr FS:[0x28]' },
  { address_hex: '0x19', mnemonic: 'XOR', operands: 'EAX,EAX' },
  { address_hex: '0x1b', mnemonic: 'TEST', operands: 'EDI,EDI' },
  { address_hex: '0x1d', mnemonic: 'SUB', operands: 'RDX,qword ptr FS:[0x28]' },
  { address_hex: '0x26', mnemonic: 'CALL', operands: '0x1070' },
];
assert.deepEqual([...idioms(synth, 'x86', { nameAt: (h) => (h === '0x1070' ? '__stack_chk_fail' : null) })].map(([, v]) => v), [
  'stack canary: load the guard value', 'zero EAX (shorter than MOV EAX,0)', 'compare EDI with 0',
  'stack canary: check the guard before returning', 'stack canary mismatch: abort',
]);
assert.equal(idioms([{ address_hex: '0x0', mnemonic: 'stp', operands: 'x29, x30, [sp, #-0x20]!' }], 'aarch64').get('0x0'),
  'prologue: save the frame pointer and return address');
checks.push('idioms (prologue/epilogue/canary/zeroing/variadic)');

// ── the stack frame ────────────────────────────────────────────────────────
assert.deepEqual(prologue(sumTo.instructions), { pushes: [{ reg: 'RBP', offset: -8 }], fp: -8, reserve: 0, sp: -8, word: 8 });
const f = frameModel(sumTo, X86);
const slot = (off) => f.slots.find((s) => s.offset === off);
assert.deepEqual([slot(0).kind, slot(0).size], ['ret', 8], 'return address at entry+0');
assert.deepEqual([slot(-8).kind, slot(-8).alias], ['fp', 'local_8'], 'saved RBP at −8 (the engine\'s local_8)');
assert.deepEqual([slot(-16).name, slot(-16).size, slot(-16).type], ['acc', 8, 'long']);
assert.deepEqual([slot(-20).name, slot(-20).size], ['v1', 4], 'the loop counter at −20 (DWARF i; the engine names it v1)');
assert.deepEqual([slot(-24).kind, slot(-24).size], ['pad', 4]);
assert.deepEqual([slot(-28).name, slot(-28).size], ['n', 4], 'the parameter\'s spill slot');
assert.equal(f.redZone, true, 'a leaf with no SUB RSP uses the red zone');
assert.match(callouts(f)[0], /red zone/);
assert.equal(operandSlot('dword ptr [RBP + -0xc]', f), -20, '[RBP-0xc] is entry −20');
assert.equal(operandSlot('qword ptr [RBP + -0x8],RAX', f), -16);
assert.equal(operandSlot('EAX,EDX', f), null);
assert.equal(slotIndex(f).symToSlot.get('acc'), -16);
assert.equal(slotIndex(f).slotToSym.get(-20), 'v1');
const mf = frameModel(main, X86);
assert.equal(mf.reserve, 32);
assert.equal(mf.redZone, false);
assert.ok(mf.slots.find((s) => s.name === 'x').dim, 'DWARF-only locals are dimmed');
assert.ok(!mf.slots.find((s) => s.name === 'argc').dim);
const arr = frameModel({
  ...main, code: 'void f(void)\n{\n  char buf[16];\n  gets(buf);\n}',
  variables: [{ name: 'buf', type: 'char[16]', kind: 'stack', stack_offset: -24, size: 16 }],
}, X86);
const note = callouts(arr).find((n) => /buf/.test(n));
assert.match(note, /buf \(char\[16\]\) ends 8 bytes below the return address/);
assert.match(note, /saved RBP, then return address/);
assert.match(renderFrame(arr), /<tr class="var arr" data-sym="buf" data-slot="-24">/);
assert.match(renderFrame(f), /<td class="off" title="entry−0x14">20<\/td>/, 'offsets read as bytes below the return address; the raw one is the tooltip');
assert.match(renderFrame(f), /<span class="sn">return address<\/span><span class="sa">put there by the CALL that ran this function<\/span>/);
assert.match(renderFrame(f), /^<p class="d2-stacklede">Higher addresses are at the top\. Each box is one thing the function keeps on the stack\.<\/p>/);
assert.deepEqual([belowText(-28), belowText(0), belowText(16)], ['28', '0', '16 above']);
assert.equal(frameModel(sumTo, { family: 'aarch64' }).supported, false);
assert.match(renderFrame(frameModel(sumTo, { family: 'aarch64' })), /not x86/);
const evil = frameModel({ ...sumTo, variables: [{ name: '<img src=x>', type: '"x"', kind: 'stack', stack_offset: -16, size: 8 }] }, X86);
assert.ok(!renderFrame(evil).includes('<img') && renderFrame(evil).includes('&lt;img src=x&gt;'), 'names are escaped');
checks.push('frameModel (sum_to, main, array callout, red zone, escaping)');

// ── references ─────────────────────────────────────────────────────────────
const fnByAddr = new Map([['0x1149', { name: 'add' }], ['0x1161', { name: 'sum_to' }], ['0x1050', { name: 'printf' }]]);
assert.deepEqual(localCallees(main, fnByAddr).map((c) => [c.name, c.at_hex]), [['add', '0x11b5'], ['sum_to', '0x11c2'], ['printf', '0x11e1']]);
const real = renderXrefs({
  binary: 'sample.elf', function: { name: 'main', address: 4504, address_hex: '0x1198' },
  callers: [{ name: null, address: 4192, address_hex: '0x1060', from: 4216, from_hex: '0x1078', kind: 'data', instruction: 'LEA RDI,[0x1198]' }],
  callees: [{ name: 'add', address: 4425, address_hex: '0x1149', at: 4533, at_hex: '0x11b5', kind: 'call', instruction: 'CALL 0x1149' }],
  data_refs: [{ name: 's_2004', address: 8196, address_hex: '0x2004', at: 4562, at_hex: '0x11d2', kind: 'data', instruction: 'LEA RAX,[0x2004]' }],
  assertions: [],
});
assert.match(real, /<p class="x-sum">Called by <a class="xt" data-goto="0x1060" title="0x1078: LEA RDI,\[0x1198\]">0x1060<\/a> <span class="d2muted">\(uses its address\)<\/span><\/p>/,
  'a caller with no name falls back to its address, says how it refers, and keeps the site in its tooltip');
assert.match(real, /<p class="x-sum">Calls <a class="xt" data-goto="0x1149" title="0x11b5: CALL 0x1149">add<\/a><\/p>/);
assert.match(real, /Uses data at <a class="xt" data-goto="0x2004" title="0x11d2: LEA RAX,\[0x2004\]">s_2004<\/a>/);
const refs = renderXrefs({
  callers: [{ name: 'main', address_hex: '0x1198', from_hex: '0x11c2', kind: 'call', instruction: 'CALL 0x1161' },
    { name: 'main', address_hex: '0x1198', from_hex: '0x11d0', kind: 'call', instruction: 'CALL 0x1161' },
    { name: 'other', address_hex: '0x1300', from_hex: '0x1310', kind: 'call' }],
  callees: [],
  data_refs: [{ name: '<s>', address_hex: '0x2004', at_hex: '0x11d2', kind: 'read' }],
}, { nameOf: (a, n) => (a === '0x1198' ? 'entry_main' : n) });
assert.match(refs, /Called by <a class="xt" data-goto="0x1198" title="0x11c2: CALL 0x1161\n0x11d0: CALL 0x1161">entry_main<\/a> and <a class="xt" data-goto="0x1300"/,
  'one link per caller (the student\'s name), every call site in its tooltip, joined in words');
assert.match(refs, /Calls no other functions/);
assert.match(refs, /&lt;s&gt;/);
assert.ok(!refs.includes('<s>'));
assert.equal(joinWords(['a', 'b', 'c']), 'a, b and c');
assert.equal(joinWords(['a']), 'a');
const local = renderLocalCalls(localCallees(main, fnByAddr), { nameOf: (a, n) => n });
assert.match(local, /Calls <a[^>]*>add<\/a>, <a[^>]*>sum_to<\/a> and <a[^>]*>printf<\/a>/);
assert.match(local, /data-act="refs-load">Find who calls it</, 'callers are one click away');
assert.match(renderLocalCalls([], { note: 'Finding callers needs a newer version.' }), /Calls no other functions.*Finding callers needs a newer version\./);
checks.push('xrefs render + local callees');

// ── help ───────────────────────────────────────────────────────────────────
const help = helpHtml();
for (const term of ['v1, v2', 'dat_4010', 'sub_401000', 'LAB_', 'undefined4', '// rax', '// stack - 0x14', 'CONCAT44', 'SEXT48', '._4_8_']) {
  assert.ok(help.includes(term), `glossary covers ${term}`);
}
for (const k of ['/', 'Space', 'n', 'y', ';', 'g', 'x', '?', 'Esc']) assert.ok(HELP_KEYS.some(([key]) => key.split(/ · | /).includes(k)), `key ${k} documented`);
assert.ok(HELP_GLOSSARY.length >= 12);
assert.match(help, /id="helptitle"/);
checks.push('help glossary + keys');

console.log(`DECOMPILE2 LEARN OK — ${checks.join('; ')}`);
