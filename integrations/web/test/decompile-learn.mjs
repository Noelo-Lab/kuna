// decompile-learn.mjs — the study view's teaching aids, from the source tree
// with no build: mnemonic explanations (every mnemonic in the fixtures is
// covered), compiler idioms, the stack-frame model and its callouts, stack
// operands, references and the help text.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { explain, idioms, TABLES } from '../decompile/mnemonics.js';
import { frameModel, operandSlot, callouts, renderFrame, slotIndex, prologue, belowText, canaryOffset } from '../decompile/stack-frame.js';
import { renderXrefs, renderLocalCalls, localCallees, joinWords } from '../decompile/xrefs-view.js';
import { helpHtml, HELP_KEYS, HELP_GLOSSARY } from '../decompile/help.js';
import { normalizeInspect, localDecls, stackPieces } from '../decompile/render-c.js';
import { parseSignature, parseParam, rustTypeSize } from '../decompile/ctype.js';

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
const insns = (list) => list.map(([mnemonic, operands]) => ({ mnemonic, operands }));
assert.deepEqual(prologue(insns([['PUSH', 'R12'], ['MOV', 'ESI,0x40'], ['PUSH', 'RBP'], ['SUB', 'RSP,0x48'], ['MOV', 'RDX,qword ptr [0x4010]'], ['MOV', 'RBP,RSP'], ['CALL', '0x1050']])),
  { pushes: [{ reg: 'R12', offset: -8 }, { reg: 'RBP', offset: -16 }], fp: null, reserve: 0x48, sp: -0x58, word: 8 },
  'an -O2 prologue interleaves other instructions with its pushes, and a later MOV RBP,RSP makes RBP a pointer, not a frame pointer');
assert.deepEqual(prologue(insns([['PUSH', 'EBP'], ['MOV', 'EBP,ESP'], ['SUB', 'ESP,0x18'], ['MOV', 'EAX,dword ptr [EBP + 0x8]'], ['PUSH', 'EAX'], ['CALL', '0x1050']]), 32),
  { pushes: [{ reg: 'EBP', offset: -4 }], fp: -4, reserve: 0x18, sp: -0x1c, word: 4 }, 'an argument pushed for a call is not a saved register');
assert.deepEqual(prologue(insns([['TEST', 'RDI,RDI'], ['JZ', '0x1290'], ['PUSH', 'RBX'], ['ADD', 'RSP,-0x80'], ['SUB', 'RSP,0x8'], ['MOV', 'RBX,RDI'], ['CALL', '0x1050']])),
  { pushes: [{ reg: 'RBX', offset: -8 }], fp: null, reserve: 0x88, sp: -0x90, word: 8 }, 'past a shrink-wrapped early exit; ADD RSP,-N and a second SUB reserve too');
const deeper = frameModel({ ...sumTo, instructions: insns([['PUSH', 'RBP'], ['MOV', 'RBP,RSP'], ['MOV', 'EAX,0x1000'], ['CALL', '0x1050']]),
  frame: { frame_pointer: { register: 'rbp', offset: -8 }, stack_pointer: { register: 'rsp', offset: -0x1010 }, base: 'frame_pointer' } }, X86);
assert.equal(deeper.size, 0x1010, 'the frame reaches as deep as the engine\'s stack pointer, even past what the scan read');
const f = frameModel(sumTo, X86);
const slot = (off) => f.slots.find((s) => s.offset === off);
assert.deepEqual([slot(0).kind, slot(0).size], ['ret', 8], 'return address at entry+0');
assert.deepEqual([slot(-8).kind, slot(-8).alias], ['fp', undefined], 'saved RBP at −8, and no engine-only name (local_8) for it');
assert.deepEqual([slot(-16).name, slot(-16).size, slot(-16).type], ['acc', 8, 'long']);
assert.deepEqual([slot(-20).name, slot(-20).size], ['v1', 4], 'the loop counter at −20 (DWARF i; the code names it v1)');
assert.deepEqual([slot(-24).kind, slot(-24).size, slot(-24).name], ['pad', 4, 'unnamed space']);
assert.deepEqual([slot(-28).name, slot(-28).size, slot(-28).input], ['n', 4, 1], 'the parameter\'s saved copy, under the parameter\'s name');
assert.deepEqual(f.slots.map((s) => f.where(s.offset)), ['rbp + 0x8', 'rbp + 0x0', 'rbp - 0x8', 'rbp - 0xc', 'rbp - 0x10', 'rbp - 0x14'],
  'every slot at the location the comments use');
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
assert.deepEqual(mf.slots.filter((s) => s.kind === 'var' || s.kind === 'arg').map((s) => [s.name, mf.where(s.offset)]), [['argc', 'rbp - 0x14'], ['argv', 'rbp - 0x20']],
  'only names the code shows: the inputs\' saved copies, not the debug info\'s x and s nor the engine\'s local_30');
assert.ok(!mf.slots.some((s) => /^(x|s|local_\w+)$/.test(s.name)));
assert.ok(!renderFrame(mf).includes('debug info'));

/** Every declared stack location lies in a box of the same name that starts at one of them, and every named box is in the code. */
function matchesComments(fnData) {
  const frame = frameModel(fnData, X86);
  const shown = frame.slots.filter((s) => s.kind === 'var' || s.kind === 'arg').concat(frame.slots.filter((s) => s.alias).map((s) => ({ ...s, name: s.alias })));
  const decls = localDecls(fnData.code);
  const sig = parseSignature(fnData.proto || '');
  for (const d of decls) {
    const pieces = stackPieces(d.storage, fnData.frame);
    for (const off of pieces) {
      assert.ok(shown.some((s) => s.name === d.name && off >= s.offset && off < s.offset + s.size && pieces.includes(s.offset)), `${fnData.name}: ${d.name} at ${off}`);
    }
    const home = fnData.variables.find((v) => v.name === d.name && v.kind !== 'arg' && pieces.includes(v.stack_offset));
    if (home) assert.ok(shown.some((s) => s.name === d.name && s.offset === home.stack_offset), `${fnData.name}: ${d.name} starts where the engine places it`);
  }
  for (const s of shown) {
    assert.ok(decls.some((d) => d.name === s.name) || sig?.params.some((p) => p.name === s.name), `${fnData.name}: ${s.name} is in the code`);
  }
  return shown.length;
}
for (const fn of [main, sumTo, add, fx('make_item'), fx('globals-stripped')]) matchesComments(fn);
const item = frameModel({
  ...fx('make_item'), code: 'struct_0 * make_item(unsigned int a0)\n{\n  struct_0 *p; // rax | rbp - 0x8\n  \n  p = malloc(0x28);\n  return p;\n}',
  proto: 'struct_0 * make_item(unsigned int a0)',
  variables: [{ name: 'param_1', type: 'unsigned int', kind: 'arg', arg_index: 0, stack_offset: null, size: 4 },
    { name: 'local_1c', type: 'undefined12', kind: 'stack', stack_offset: -28, size: 12 },
    { name: 'local_10', type: 'struct_0 *', kind: 'stack', stack_offset: -16, size: 8 }],
}, X86);
assert.deepEqual(item.slots.filter((s) => s.kind === 'var').map((s) => [s.name, item.where(s.offset), s.size]), [['p', 'rbp - 0x8', 8]],
  'a local the engine lists as local_10 is drawn under the name the code gives it, and local_1c not at all');
matchesComments({ ...fx('make_item'), code: 'struct_0 * make_item(unsigned int a0)\n{\n  struct_0 *p; // rax | rbp - 0x8\n  \n}', proto: 'struct_0 * make_item(unsigned int a0)' });

const canaried = {
  ...main, code: 'int f(void)\n{\n  char buf [64]; // rbp - 0x50\n  \n  gets(buf);\n}', variables: [], proto: 'int f(void)',
  instructions: [
    { mnemonic: 'ENDBR64', operands: '' }, { mnemonic: 'PUSH', operands: 'RBP' }, { mnemonic: 'MOV', operands: 'RBP,RSP' },
    { mnemonic: 'SUB', operands: 'RSP,0x50' }, { mnemonic: 'MOV', operands: 'RAX,qword ptr FS:[0x28]' },
    { mnemonic: 'MOV', operands: 'qword ptr [RBP + -0x8],RAX' }, { mnemonic: 'XOR', operands: 'EAX,EAX' }, { mnemonic: 'CALL', operands: '0x1060' },
  ],
  frame: { frame_pointer: { register: 'rbp', offset: -8 }, stack_pointer: { register: 'rsp', offset: -0x58 } },
};
const cf = frameModel(canaried, X86);
assert.deepEqual(cf.slots.map((s) => [s.name, cf.where(s.offset), s.size]), [
  ['return address', 'rbp + 0x8', 8], ['saved RBP', 'rbp + 0x0', 8], ['stack canary', 'rbp - 0x8', 8],
  ['unnamed space', 'rbp - 0x10', 8], ['buf', 'rbp - 0x50', 64],
], 'the canary slot is named, and buf sits where its comment says');
assert.match(callouts(cf)[0], /runs upward into stack canary, then saved RBP, then return address/);
assert.match(callouts(cf)[0], /The stack canary is checked before RET/);
assert.match(renderFrame(cf), /<tr class="var arr" data-sym="buf" data-slot="-88"><td class="off" title="88 bytes below the return address">rbp - 0x50<\/td>/,
  'the location as the comment writes it; how far below the return address is the tooltip');
assert.match(renderFrame(cf), /<tr class="canary" data-slot="-16">/);
assert.equal(canaryOffset(canaried.instructions.slice(0, 5), cf.slotOf), null, 'no store, no canary');
const noFp = frameModel({ ...canaried, instructions: [{ mnemonic: 'SUB', operands: 'RSP,0x58' }, { mnemonic: 'CALL', operands: '0x1060' }],
  code: 'int f(void)\n{\n  char buf [64]; // rsp + 0x8\n  \n  gets(buf);\n}', frame: { frame_pointer: null, stack_pointer: { register: 'rsp', offset: -0x60 } } }, X86);
assert.deepEqual(noFp.slots.filter((s) => s.kind === 'var').map((s) => [s.name, noFp.where(s.offset)]), [['buf', 'rsp + 0x8']], 'no frame pointer: from RSP, as the comment says');
assert.equal(operandSlot('qword ptr [RSP + 0x8]', noFp), -0x58, 'operands map to slots through the engine\'s frame');
assert.equal(operandSlot('dword ptr [RBP + -0xc]', noFp), null, 'RBP is no frame pointer here');
assert.equal(noFp.fp, null);
assert.deepEqual(cf.fp, { register: 'rbp', offset: -8 });
const RBP8 = { frame_pointer: { register: 'rbp', offset: -8 }, stack_pointer: { register: 'rsp', offset: -0x58 }, base: 'frame_pointer' };
const folded = frameModel({ ...canaried, frame: RBP8, instructions: canaried.instructions.slice(0, 4),
  code: 'int f(void)\n{\n  item v1; // rbp - 0x30 | rbp - 0x28 | rbp - 0x20\n  unsigned int v2; // rbp - 0x44 | rbp - 0x40\n  int v3 [4]; // rbp + 0x0 | rbp - 0x50\n  \n}',
  types: [{ name: 'item', definition: 'struct item {...}', size: 24 }] }, X86);
assert.deepEqual(folded.slots.filter((s) => s.kind === 'var').map((s) => [s.name, folded.where(s.offset), s.size]),
  [['v1', 'rbp - 0x30', 24], ['v2', 'rbp - 0x40', 4], ['v2', 'rbp - 0x44', 4], ['v3', 'rbp - 0x50', 16]],
  'locations inside one object fold into its box; separate homes keep a box each');
assert.equal(folded.slots.find((s) => s.kind === 'fp').alias, 'v3', 'a second home is cut off below the return address, and lands on the saved RBP');
const anchored = frameModel({ ...canaried, frame: RBP8, instructions: canaried.instructions.slice(0, 4),
  code: 'int f(void)\n{\n  item v1; // rbp - 0x18 | rbp - 0x20 | rbp - 0x40\n  \n}', types: [{ name: 'item', definition: '', size: 24 }],
  variables: [{ name: 'v1', type: 'item', kind: 'stack', stack_offset: -0x28, size: 24 }, { name: 'local_48', type: 'undefined8', kind: 'stack', stack_offset: -0x48, size: 8 }] }, X86);
assert.deepEqual(anchored.slots.filter((s) => s.kind === 'var').map((s) => [s.name, anchored.where(s.offset), s.size]), [['v1', 'rbp - 0x20', 24], ['v1', 'rbp - 0x40', 8]],
  'the object starts where the engine places it; another home is no bigger than the engine\'s slot there');
assert.equal(callouts(folded).filter((n) => /^v3 /.test(n)).length, 1, 'no callout for a copy that reaches above the return address');
const rustFrame = frameModel({ ...canaried, frame: RBP8, language: 'rust-language', proto: 'unsafe fn f()',
  code: 'unsafe fn f()\n{\n  let mut v1: i64; // rbp - 0x10\n  let mut v2: [u8; 40]; // rbp - 0x40\n  let mut v3: *mut u8; // rbp - 0x18\n  \n}' }, X86);
assert.deepEqual(rustFrame.slots.filter((s) => s.kind === 'var').map((s) => [s.name, s.size, !!s.array]), [['v1', 8, false], ['v3', 8, false], ['v2', 40, true]], 'Rust types have their sizes');
const rustStruct = frameModel({ ...canaried, frame: RBP8, language: 'rust-language', proto: 'unsafe fn f()', types: [],
  code: 'unsafe fn f()\n{\n  let mut sb: stat; // rbp - 0x98\n  \n}', variables: [{ name: 'local_a0', type: 'undefined', kind: 'stack', stack_offset: -0xa0, size: 144 }] }, X86);
assert.deepEqual(rustStruct.slots.filter((s) => s.kind === 'var').map((s) => [s.name, s.size]), [['sb', 144]], 'a type of unknown size takes the engine\'s slot at its location');
assert.deepEqual([rustTypeSize('i32'), rustTypeSize('*const u8', { bits: 32 }), rustTypeSize('[[u16; 4]; 3]'), rustTypeSize('Foo'), rustTypeSize('Foo', { known: new Map([['Foo', 12]]) })], [4, 4, 24, null, 12]);
assert.deepEqual(parseParam('char (*abmon)[128]'), { type: 'char (*)[128]', name: 'abmon' }, 'a pointer to an array is named too');
const i386 = frameModel({ ...canaried, frame: { frame_pointer: { register: 'ebp', offset: -4 }, stack_pointer: { register: 'esp', offset: -4 }, base: 'frame_pointer' },
  instructions: insns([['PUSH', 'EBP'], ['MOV', 'EBP,ESP'], ['MOV', 'EAX,dword ptr [EBP + 0x8]'], ['INC', 'EAX'], ['PUSH', 'EAX'], ['CALL', '0x1050']]),
  code: 'int f(int a0)\n{\n  int v1; // eax | ebp - 0x4\n  \n}', proto: 'int f(int a0)', variables: [] }, { family: 'x86', bits: 32 });
assert.deepEqual(i386.slots.map((s) => [s.kind, s.name]), [['ret', 'return address'], ['fp', 'saved EBP'], ['var', 'v1']], 'an argument pushed below the engine\'s stack pointer is no saved register');
const gs = frameModel({ ...canaried, frame: { frame_pointer: { register: 'ebp', offset: -4 }, stack_pointer: { register: 'esp', offset: -0x2c }, base: 'frame_pointer' },
  instructions: insns([['PUSH', 'EBP'], ['MOV', 'EBP,ESP'], ['SUB', 'ESP,0x28'], ['MOV', 'EAX,GS:[0x14]'], ['MOV', 'dword ptr [EBP + -0xc],EAX'], ['XOR', 'EAX,EAX']]),
  code: 'int f(void)\n{\n  char v1 [16]; // ebp - 0x1c\n  \n}', proto: 'int f(void)', variables: [] }, { family: 'x86', bits: 32 });
assert.deepEqual(gs.slots.filter((s) => s.kind === 'canary').map((s) => gs.where(s.offset)), ['ebp - 0xc'], 'the i386 canary (MOV EAX,GS:[0x14])');
const padded = frameModel({ ...canaried, frame: { frame_pointer: null, stack_pointer: { register: 'rsp', offset: -0x18 }, base: 'stack_pointer' },
  instructions: insns([['TEST', 'RDI,RDI'], ['JZ', '0x1290'], ['PUSH', 'RBP'], ['PUSH', 'RBX'], ['PUSH', 'RAX'], ['CALL', '0x1050']]), code: 'int f(void)\n{\n  \n}', proto: 'int f(void)', variables: [] }, X86);
assert.deepEqual(padded.slots.map((s) => [s.kind, s.name]), [['ret', 'return address'], ['saved', 'saved RBP'], ['saved', 'saved RBX'], ['pad', 'unnamed space']],
  'PUSH RAX only pads the stack, and RBP is no frame pointer where the engine finds none');
const twice = frameModel({ ...canaried, code: 'int f(void)\n{\n  long v1; // rbp - 0x8\n  long v2; // rbp - 0x8\n  char buf [64]; // rbp - 0x50\n  \n}' }, X86);
assert.deepEqual(twice.slots.filter((s) => s.offset === -16).map((s) => s.alias || s.name), ['v1', 'v2'], 'a second code name on the canary slot is not lost');
assert.match(renderFrame(twice), /<tr class="canary" data-sym="v1" data-slot="-16">/);
const oldEngine = frameModel({ ...canaried, frame: null, code: 'int f(void)\n{\n  char buf [64]; // stack - 0x58\n  \n  gets(buf);\n}' }, X86);
assert.deepEqual(oldEngine.slots.filter((s) => s.kind === 'var').map((s) => [s.name, oldEngine.where(s.offset)]), [['buf', 'stack - 0x58']], 'an engine without frame reads the comments as written');

const arr = frameModel({
  ...main, code: 'void f(void)\n{\n  char buf [16]; // rbp - 0x10\n  gets(buf);\n}', variables: [],
}, X86);
const note = callouts(arr).find((n) => /buf/.test(n));
assert.match(note, /buf \(char \[16\]\) ends 8 bytes below the return address/);
assert.match(note, /saved RBP, then return address/);
assert.match(renderFrame(arr), /<tr class="var arr" data-sym="buf" data-slot="-24">/);
assert.match(renderFrame(f), /<div class="sl"><span class="sn">return address<\/span><span class="sa d2-teach" title="put there by the CALL that ran this function">put there by the CALL that ran this function<\/span><span class="sz">8 bytes<\/span><\/div>/,
  'each slot is one row: name, description (full text in its title, and a teaching note "Show hints" can hide), size last');
assert.match(renderFrame(f), /^<p class="d2-stacklede">Higher addresses are at the top\. Each box is one thing the function keeps on the stack, at the location the code's comments give\.<\/p>/);
assert.match(renderFrame(f), /<th class="off">Location<\/th>/);
assert.deepEqual([belowText(-28), belowText(0), belowText(16)], ['28 bytes below the return address', 'the return address', '16 bytes above the return address']);
assert.equal(frameModel(sumTo, { family: 'aarch64' }).supported, false);
assert.match(renderFrame(frameModel(sumTo, { family: 'aarch64' })), /not x86/);
const evil = frameModel({ ...sumTo, code: 'long f(void)\n{\n  <script> v9; // rbp - 0x8\n  \n}' }, X86);
assert.ok(!renderFrame(evil).includes('<script') && renderFrame(evil).includes('&lt;script&gt;'), 'types are escaped');
checks.push('frameModel (sum_to, main, only the code\'s names at the comments\' locations, canary, no frame pointer, array callout, red zone, escaping)');

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
for (const term of ['v1, v2', 'dat_4010', 'struct_0', 'field_0x8', 'sub_401000', 'LAB_', 'undefined4', '// rax', '// rbp - 0x14', 'CONCAT44', 'SEXT48', '._4_8_']) {
  assert.ok(help.includes(term), `glossary covers ${term}`);
}
for (const k of ['/', 'Space', 'n', 'y', ';', 'g', 'x', 'e', '?', 'Esc']) assert.ok(HELP_KEYS.some(([key]) => key.split(/ · | /).includes(k)), `key ${k} documented`);
assert.ok(HELP_GLOSSARY.length >= 12);
assert.match(help, /id="helptitle"/);
checks.push('help glossary + keys');

console.log(`DECOMPILE LEARN OK — ${checks.join('; ')}`);
