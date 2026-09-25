// decompile2-session.mjs — the study view's edit session, from the source tree
// with no build: how edits become `--assert` directives (merging, pinning,
// scoping, qualification after a function rename, parameters through
// `prototype`), byte runs, the `.kuna` file round trip, outcomes, undo, the
// C declarators behind it, and the localStorage store.
import assert from 'node:assert/strict';
import { Session, stripComment, shellQuote, cliCommand, isGlobalRaw } from '../decompile2/session.js';
import {
  validateIdent, validateCType, cDeclare, parseSignature, buildPrototype, typeSize, knownTypes, normalizeType,
} from '../decompile2/ctype.js';
import { SessionStore, fnv1a32, hashBytes } from '../decompile2/persist.js';

const checks = [];
const MAIN = '0x1198';
const SUM = '0x1161';
const names = new Map([[MAIN, 'main'], [SUM, 'sum_to']]);

// ── merge and pin rules ────────────────────────────────────────────────────
{
  const s = new Session();
  s.setVar(MAIN, 'v1', { name: 'total' });
  assert.deepEqual(s.assertionsFor(MAIN), ['name v1 total']);
  s.setVar(MAIN, 'v1', { type: 'unsigned long' });
  assert.deepEqual(s.assertionsFor(MAIN), ['type v1 unsigned long total'], 'rename + retype merge into one pinned type');
  s.setVar(MAIN, 'v1', { name: null });
  assert.deepEqual(s.assertionsFor(MAIN), ['type v1 unsigned long v1'], 'a bare retype pins the current name');
  s.setVar(MAIN, 'v1', { type: 'char[16]', name: 'buf' });
  assert.deepEqual(s.assertionsFor(MAIN), ['type v1 char buf[16]'], 'arrays declare the name inside');
  s.setVar(MAIN, 'v1', { type: null, name: null });
  assert.equal(s.size, 0, 'clearing both fields removes the record');
  s.setVar(MAIN, 'v1', { name: 'v1' });
  assert.equal(s.size, 0, 'renaming to the original name is no edit');

  // display name → symbol
  s.setVar(MAIN, 'v1', { name: 'total' });
  assert.equal(s.symbolOf(MAIN, 'total'), 'v1');
  assert.equal(s.symbolOf(MAIN, 'argc'), 'argc');
  assert.equal(s.symbolOf(SUM, 'total'), 'total', 'records are per function');
  s.setVar(MAIN, s.symbolOf(MAIN, 'total'), { name: 'sum' });
  assert.deepEqual(s.assertionsFor(MAIN), ['name v1 sum'], 'renaming a renamed local keys on the symbol');
  checks.push('merge/pin/display-name');
}

// ── scoping: unqualified for inspect, current-name-qualified for export ────
{
  const s = new Session();
  s.setVar(MAIN, 'v1', { name: 'total' });
  s.setVar(SUM, 'v1', { name: 'i' });
  s.setComment(MAIN, '0x11b5', 'calls add first');
  s.setProto(SUM, 'long sum_to(int count)');
  s.setTypedef('pair', 'typedef struct pair { int a; int b; } pair;');
  s.setData('0x4010', 'int', 'counter');
  s.setByte(0x11e1n, 0x90, 0xe8);
  s.addRaw('readonly 0x2000+8');
  s.addRaw('flow sum_to::0x1190 branch');
  assert.deepEqual(s.assertionsFor(MAIN), [
    'typedef typedef struct pair { int a; int b; } pair;',
    'data 0x4010 int counter',
    'prototype 0x1161 long sum_to(int count)',
    'bytes 0x11e1 90',
    'readonly 0x2000+8',
    'name v1 total',
    'comment 0x11b5 calls add first',
  ], 'inspect: globals then this function, unqualified, in replay order');
  assert.deepEqual(s.assertionsFor(SUM).slice(-1), ['name v1 i']);
  assert.deepEqual(s.globalAssertions(), s.assertionsFor('0xdead'), 'list gets only the global directives');
  s.setFunctionName(SUM, 'summation');
  const nameOf = (a) => s.functionName(a) || names.get(a);
  assert.deepEqual(s.allAssertions(nameOf), [
    'typedef typedef struct pair { int a; int b; } pair;',
    'data 0x4010 int counter',
    'function 0x1161=summation',
    'prototype 0x1161 long sum_to(int count)',
    'bytes 0x11e1 90',
    'readonly 0x2000+8',
    'flow sum_to::0x1190 branch',
    'name main::v1 total',
    'name summation::v1 i',
    'comment main::0x11b5 calls add first',
  ], 'export: qualified with the CURRENT name, after the function rename');
  assert.ok(!isGlobalRaw('flow 0x10 return') && !isGlobalRaw('name v1 x') && isGlobalRaw('param f::0 RDI int x') && isGlobalRaw('volatile 0x5+4'));
  checks.push('unqualified/qualified scoping');
}

// ── parameters go through prototype ────────────────────────────────────────
{
  const sig = parseSignature('long sum_to(int n)');
  sig.params[0].name = 'count';
  assert.equal(buildPrototype(sig), 'long sum_to(int count)');
  const main = parseSignature('int main(int argc,char **argv)');
  main.params[1].type = 'char *[4]';
  assert.equal(buildPrototype(main), 'int main(int argc, char *argv[4])');
  assert.equal(buildPrototype(parseSignature('void f(void)')), 'void f(void)');
  assert.equal(buildPrototype(parseSignature('char *strdup(const char *s)')), 'char *strdup(const char *s)');
  assert.equal(buildPrototype(parseSignature('int printf(char *fmt, ...)')), 'int printf(char *fmt, ...)');
  assert.equal(buildPrototype(parseSignature('int g(int (*cb)(int), char buf[16])')), 'int g(int (*cb)(int), char buf[16])');
  assert.equal(parseSignature('void * __stdcall Load(void *p)').conv, '__stdcall');
  assert.equal(parseSignature('not a prototype'), null);
  checks.push('prototype for params');
}

// ── declarators, validation, sizes ─────────────────────────────────────────
assert.equal(cDeclare('char[16]', 'buf'), 'char buf[16]');
assert.equal(cDeclare('int *', 'p'), 'int *p');
assert.equal(cDeclare('char **', 'argv'), 'char **argv');
assert.equal(cDeclare('int (*)(int)', 'fp'), 'int (*fp)(int)');
assert.equal(cDeclare('unsigned  long', 'n'), 'unsigned long n');
assert.equal(normalizeType('char  * *'), 'char **');
assert.equal(validateIdent('total'), null);
assert.match(validateIdent('9lives'), /letters/);
assert.match(validateIdent('int'), /keyword/);
assert.equal(validateCType('unsigned int'), null);
assert.equal(validateCType('struct pair *'), null);
assert.match(validateCType('int; x'), /cannot contain/);
assert.match(validateCType('int int int (('), /not a C type/);
assert.equal(typeSize('unsigned int'), 4);
assert.equal(typeSize('long'), 8);
assert.equal(typeSize('long', { llp64: true }), 4);
assert.equal(typeSize('char *', { bits: 32 }), 4);
assert.equal(typeSize('char[16]'), 16);
assert.equal(typeSize('undefined8'), 8);
assert.equal(typeSize('struct mystery'), null);
assert.ok(knownTypes({ types: [{ name: 'pair' }], typedefs: ['node'] }).includes('node'));
checks.push('cDeclare/validate/typeSize');

// ── byte runs ──────────────────────────────────────────────────────────────
{
  const s = new Session();
  s.setByte(0x11e1n, 0x90, 0xe8);
  s.setByte(0x11e2n, 0x90, 0x6a);
  s.setByte(0x11e4n, 0x90, 0xff);
  assert.deepEqual(s.globalAssertions(), ['bytes 0x11e1 9090', 'bytes 0x11e4 90'], 'contiguous bytes form one run');
  s.setByte(0x11e2n, 0x6a, 0x6a);
  assert.deepEqual(s.globalAssertions(), ['bytes 0x11e1 90', 'bytes 0x11e4 90'], 'writing the original byte removes it');
  s.remove('bytes:0x11e4');
  assert.deepEqual(s.globalAssertions(), ['bytes 0x11e1 90']);
  s.setByte(0xffffffff81000000n, 0xcc, 0);
  assert.deepEqual(s.globalAssertions().at(-1), 'bytes 0xffffffff81000000 cc', '64-bit addresses stay exact');
  checks.push('byte runs');
}

// ── outcomes ───────────────────────────────────────────────────────────────
{
  const s = new Session();
  const k1 = s.setVar(MAIN, 'v1', { name: 'total' });
  const k2 = s.setVar(MAIN, 'v999', { name: 'nope' });
  s.assertionsFor(MAIN);
  assert.equal(s.statusOf(k1), 'pending');
  s.recordOutcomes([
    { directive: 'name v1 total', status: 'applied', detail: null, fatal: false },
    { directive: 'name v999 nope', status: 'rejected', detail: 'No symbol named: v999', fatal: false },
  ]);
  assert.equal(s.statusOf(k1), 'applied');
  assert.equal(s.statusOf(k2), 'rejected');
  const rows = s.entries((a) => names.get(a));
  assert.deepEqual(rows.map((r) => [r.text, r.status, r.detail]), [
    ['name main::v1 total', 'applied', null],
    ['name main::v999 nope', 'rejected', 'No symbol named: v999'],
  ]);
  checks.push('outcomes');
}

// ── undo / redo ────────────────────────────────────────────────────────────
{
  const s = new Session();
  let snap = s.snapshot();
  s.setVar(MAIN, 'v1', { name: 'total' });
  s.pushUndo(snap);
  snap = s.snapshot();
  s.setByte(0x11e1n, 0x90, 0xe8);
  s.pushUndo(snap);
  assert.equal(s.size, 2);
  assert.ok(s.undo());
  assert.deepEqual(s.globalAssertions(), [], 'undo removed the patch');
  assert.equal(s.size, 1);
  assert.ok(s.undo());
  assert.equal(s.size, 0);
  assert.equal(s.undo(), false, 'nothing left to undo');
  assert.ok(s.redo() && s.redo());
  assert.equal(s.size, 2);
  assert.equal(s.redo(), false);
  snap = s.snapshot();
  s.setComment(MAIN, '0x11b5', 'x');
  s.pushUndo(snap);
  assert.equal(s.canRedo, false, 'a new edit drops the redo branch');
  for (let i = 0; i < 150; i++) s.pushUndo(s.snapshot());
  assert.equal(s.undoStack.length, 100, 'undo history is capped');
  checks.push('undo/redo');
}

// ── the .kuna file and the CLI's comment rule ──────────────────────────────
assert.equal(stripComment('name v1 x # worked out from strings'), 'name v1 x');
assert.equal(stripComment('# a comment'), '');
assert.equal(stripComment('   # indented comment'), '');
assert.equal(stripComment('\tname v1 x'), 'name v1 x');
assert.equal(stripComment('comment 0x10 a#b'), 'comment 0x10 a#b', 'a # inside a token is text');
assert.equal(stripComment('comment 0x10 see #3'), 'comment 0x10 see', 'the CLI cuts at " #" even in a comment');
assert.equal(shellQuote('sample.elf'), 'sample.elf');
assert.equal(shellQuote('my file.elf'), "'my file.elf'");
assert.equal(shellQuote("it's"), "'it'\\''s'");
assert.equal(cliCommand('sample.elf', 'main'), 'kuna decompile sample.elf main --assert @sample.elf.kuna');
assert.equal(cliCommand('a b.exe', 'sub_401000'), "kuna decompile 'a b.exe' sub_401000 --assert @'a b.exe.kuna'");
{
  const s = new Session();
  s.setVar(MAIN, 'v1', { name: 'total', type: 'unsigned long' });
  s.setVar(SUM, 'v1', { name: 'i' });
  s.setProto(SUM, 'long sum_to(int count)');
  s.setFunctionName(SUM, 'summation');
  s.setComment(MAIN, '0x11b5', 'calls add first');
  s.setData('0x4010', 'char[8]', 'tag');
  s.setByte(0x11e1n, 0x90, 0xe8);
  s.addRaw('flow summation::0x1190 branch');
  const nameOf = (a) => s.functionName(a) || names.get(a);
  const text = s.toFileText({ binary: 'sample.elf', hash: 'sha256:ab', time: new Date(0), target: 'main', nameOf });
  const lines = text.trim().split('\n');
  assert.equal(lines[0], '# kuna study-view session (one --assert directive per line)');
  assert.ok(lines.includes('# replay: kuna decompile sample.elf main --assert @sample.elf.kuna'));
  assert.ok(lines.includes('# saved: 1970-01-01T00:00:00.000Z'));
  const body = lines.filter((l) => stripComment(l));
  assert.deepEqual(body, [
    'data 0x4010 char tag[8]',
    'function 0x1161=summation',
    'prototype 0x1161 long sum_to(int count)',
    'bytes 0x11e1 90',
    'flow summation::0x1190 branch',
    'type main::v1 unsigned long total',
    'name summation::v1 i',
    'comment main::0x11b5 calls add first',
  ]);
  const byName = new Map([['main', MAIN], ['summation', SUM], ['sum_to', SUM]]);
  const back = new Session();
  const counts = back.importText(text, { resolveFunc: (n) => byName.get(n) || null });
  assert.equal(counts.added, 8);
  assert.equal(counts.raw, 1, 'a directive the page does not model is kept verbatim');
  assert.deepEqual(back.allAssertions((a) => back.functionName(a) || names.get(a)), s.allAssertions(nameOf), 'file round trip');
  const loose = new Session();
  const r = loose.importText('name v1 total\ntype v2 unsigned long\nname v3 x # note\n', { bindTo: MAIN });
  assert.equal(r.unbound, 0);
  assert.deepEqual(loose.assertionsFor(MAIN), ['name v1 total', 'type v2 unsigned long v2', 'name v3 x'], 'unqualified lines bind to the open function');
  const nowhere = new Session();
  assert.equal(nowhere.importText('name v1 total').unbound, 1, 'with nothing open they stay unbound');
  const again = Session.fromJSON(JSON.parse(JSON.stringify(s.toJSON())));
  assert.deepEqual(again.allAssertions(nameOf), s.allAssertions(nameOf), 'JSON round trip (what localStorage holds)');
  assert.equal(Session.fromJSON({ v: 9 }).size, 0, 'an unknown version restores nothing');
  checks.push('.kuna file round trip + stripComment parity + quoting + JSON');
}

// ── persistence ────────────────────────────────────────────────────────────
assert.equal(fnv1a32(new Uint8Array()), '811c9dc5');
assert.equal(fnv1a32(new TextEncoder().encode('a')), 'e40c292c');
assert.equal(fnv1a32(new TextEncoder().encode('foobar')), 'bf9cf968');
if (globalThis.crypto?.subtle) {
  assert.match(await hashBytes(new TextEncoder().encode('abc')), /^sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad$/);
}
assert.match(await hashBytes(new TextEncoder().encode('abc'), null), /^fnv:[0-9a-f]{16}-3$/, 'no WebCrypto: FNV fallback');
{
  const mem = new Map();
  let quota = Infinity;
  const storage = {
    getItem: (k) => (mem.has(k) ? mem.get(k) : null),
    setItem: (k, v) => {
      const used = [...mem].filter(([key]) => key !== k).reduce((n, [, val]) => n + val.length, 0);
      if (used + v.length > quota) throw new Error('QuotaExceededError');
      mem.set(k, v);
    },
    removeItem: (k) => mem.delete(k),
  };
  const store = new SessionStore(storage, { max: 3 });
  for (const h of ['a', 'b', 'c', 'd']) store.save(h, `${h}.elf`, `edits of ${h}`);
  assert.deepEqual(store.index().map((e) => e.hash), ['d', 'c', 'b'], 'LRU keeps the newest three');
  assert.equal(store.load('a'), null, 'the oldest session was evicted');
  assert.equal(store.load('c'), 'edits of c');
  store.save('b', 'b.elf', 'edits of b, again');
  assert.deepEqual(store.index().map((e) => e.hash), ['b', 'd', 'c'], 'saving moves a binary to the front');
  quota = [...mem].reduce((n, [, v]) => n + v.length, 0) + 10;
  assert.ok(store.save('e', 'e.elf', 'x'.repeat(40)), 'a full store evicts until the save fits');
  assert.ok(store.index().length < 4 && store.load('e'));
  quota = 5;
  assert.equal(store.save('f', 'f.elf', 'x'.repeat(100)), false, 'a save that can never fit reports failure');
  store.remove('e');
  assert.equal(store.load('e'), null);
  assert.equal(new SessionStore(null).save('x', 'x', 'y'), false, 'no storage at all is not an error');
  const throwing = new SessionStore({ getItem() { throw new Error('denied'); }, setItem() { throw new Error('denied'); }, removeItem() {} });
  assert.equal(throwing.load('x'), null);
  assert.deepEqual(throwing.index(), []);
  checks.push('store LRU/quota + FNV/SHA-256');
}

console.log(`DECOMPILE2 SESSION OK — ${checks.join('; ')}`);
