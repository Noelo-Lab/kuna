// decompile-data.mjs — the Data tab's rows, from the source tree with no
// build: integers, text, pointers, arrays and structs decoded by their
// layouts from what the native kuna_wasm prints for globals.elf (as the
// symbol table types it, retyped by `data` directives, and stripped, where
// the decompiler's own declarations type it), bytes that start as zeros, the
// unlabelled bytes between items, who uses an item, and that every engine
// string is escaped.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import {
  readUint, readInt, cString, quote, intText, declType, layoutOfDecl, sectionWords, dataModel, childRows,
  gapRows, placeOf, rowOf, usesOf, useWords, renderData, pointerValue, valueOf, dataSections, sizeWords, sectionPicker,
} from '../decompile/data-view.js';

const fixture = (name) => JSON.parse(readFileSync(new URL(`./fixtures/${name}`, import.meta.url)));
const checks = [];
const fnAt = (hex) => ({ '0x1189': 'twice', '0x119b': 'main' }[hex] || null);
const top = (m, name) => m.rows.find((r) => r.name === name) || assert.fail(`${name} is a row: ${m.rows.map((r) => r.name || r.kind)}`);

// ── reading bytes ──────────────────────────────────────────────────────────
const b = Uint8Array.from([0xe8, 0x03, 0, 0, 0xff, 0xff, 0xff, 0xff, 0x68, 0x69, 0, 0x41]);
assert.equal(readUint(b, 0, 4), 1000n);
assert.equal(readUint(b, 0, 2, false), 0xe803n, 'big-endian reads the other way');
assert.equal(readInt(b, 4, 4), -1n);
assert.equal(readUint(b, 4, 4), 0xffffffffn);
assert.equal(readUint(b, 10, 4), null, 'past the end is nothing');
assert.deepEqual(cString(b, 8), { text: 'hi', size: 3 });
assert.equal(cString(b, 8, b.length, 4), null, 'shorter than the minimum');
assert.equal(cString(b, 11), null, 'no NUL, no string');
assert.equal(quote('a"b\n\\'), '"a\\"b\\n\\\\"');
assert.equal(quote('x'.repeat(10), 4), '"xxxx"…');
assert.deepEqual(cString(Uint8Array.from([0x1b, 0x5b, 0x33, 0x6d, 0x41, 0]), 0), { text: '\x1b[3mA', size: 6 }, 'a terminal colour code is text');
assert.equal(quote('\x1b[3mA\x07'), '"\\x1b[3mA\\x07"', 'control characters read as C escapes');
assert.equal(intText(1000n, 4), '1000 · 0x3e8');
assert.equal(intText(7n, 4), '7', 'a digit needs no hex');
assert.equal(intText(-1n, 4), '-1 · 0xffffffff');
const chars = { start: 0n, fileEnd: null, little: true, bytes: Uint8Array.from([0x5c, 0x27, 0x41, 0x0a]) };
assert.deepEqual([0n, 1n, 2n, 3n].map((a) => valueOf(chars, a, { kind: 'char', size: 1 }).value),
  ["'\\\\' · 92", "'\\'' · 39", "'A' · 65", '10'], 'a char reads as a C character literal');
checks.push('bytes, text, integers');

// ── declarations the page knows ────────────────────────────────────────────
assert.equal(declType('int (*dat_4078)(unsigned int)', 'dat_4078'), 'int (*)(unsigned int)');
assert.equal(declType('char *dat_4070', 'dat_4070'), 'char *');
assert.equal(declType('long a.b', 'a.b'), 'long', 'a name with regex characters is taken literally');
assert.equal(declType('char *odd(name', 'odd(name'), 'char *');
assert.equal(declType('int $x', '$x'), 'int');
assert.equal(declType('int ab', 'b'), 'int ab', 'only the whole name');
assert.deepEqual(layoutOfDecl('int dat_4040[]', 'dat_4040'), { kind: 'array', size: 4, type: 'int[]', elem: { kind: 'int', size: 4, type: 'int', signed: true }, count: null, open: true });
assert.equal(layoutOfDecl('char *dat_4070', 'dat_4070').to.kind, 'char');
assert.equal(layoutOfDecl('int (*dat_4078)(unsigned int)', 'dat_4078').to.kind, 'code');
assert.deepEqual(layoutOfDecl('unsigned long n', 'n'), { kind: 'int', size: 8, type: 'unsigned long', signed: false });
assert.equal(layoutOfDecl('unsigned long n', 'n', { longSize: 4 }).size, 4, 'long is 4 bytes on Windows');
assert.equal(layoutOfDecl('short int s', 's').size, 2);
assert.equal(layoutOfDecl('uint16_t w[3]', 'w').size, 6);
assert.equal(layoutOfDecl('struct_0 g', 'g'), null, 'a struct the page has no layout for');
assert.equal(layoutOfDecl('struct_0 g', 'g', { known: new Map([['struct_0', 24]]) }).size, 24);
assert.equal(sectionWords({ executable: false, writable: true, file_offset: 0x3000 }), 'data the program can change');
assert.equal(sectionWords({ executable: false, writable: false, file_offset: 0x2000 }), 'data that never changes');
assert.equal(sectionWords({ executable: false, writable: true, file_offset: null }), 'data that starts as zeros');
checks.push('declarations');

// ── as the symbol table types it ───────────────────────────────────────────
const plain = dataModel(fixture('data-globals.json'), { fnAt });
assert.equal(plain.section.name, '.data');
assert.equal(top(plain, 'greeting').value, '"hello, data"', 'unknown bytes that read as text are text');
assert.equal(top(plain, 'greeting').type, '12 bytes', 'a type nobody knows is its size');
assert.match(top(plain, 'primes').value, /^02 00 00 00 03 00 00 00 05 00 00 00 07 00 00 00 …$/, 'other unknown bytes are hex');
assert.equal(top(plain, 'primes').count, 2, 'that opens to its 16-byte lines');
assert.deepEqual(childRows(plain, top(plain, 'primes')).map((r) => r.value), ['02 00 00 00 03 00 00 00 05 00 00 00 07 00 00 00', '0b 00 00 00 0d 00 00 00 11 00 00 00 13 00 00 00']);
assert.equal(top(plain, 'banner').value, '→ "kuna data tab"', 'an untyped word that holds the address of a string points at it');
assert.deepEqual(top(plain, 'op').link, { hex: '0x1189', kind: 'fn', label: 'twice' }, 'one that holds a function\'s entry points at the function');
assert.equal(top(plain, 'favorite').value, '→ primes + 0xc');
assert.deepEqual(top(plain, 'favorite').link, { hex: '0x404c', kind: 'data', label: 'primes + 0xc' });
const word = plain.rows.find((r) => r.hex === '0x4008');
assert.equal(word.kind, 'bytes', 'a word that points at nothing named stays bytes');
assert.equal(word.value, '08 40 00 00 00 00 00 00');
assert.deepEqual(plain.rows.filter((r) => r.kind === 'zeros').map((r) => [r.hex, r.size]), [['0x4000', 8], ['0x4010', 16], ['0x402c', 20]]);
checks.push('untyped');

// ── retyped by `data` directives ───────────────────────────────────────────
const typed = dataModel(fixture('data-globals-typed.json'), { fnAt });
const primes = top(typed, 'primes');
assert.equal(primes.type, 'int[8]');
assert.equal(primes.value, '{2, 3, 5, 7, 11, 13, 17, 19}');
assert.deepEqual(childRows(typed, primes).map((r) => [r.name, r.value]).slice(2, 5), [['[2]', '5'], ['[3]', '7'], ['[4]', '11 · 0xb']]);
const origin = top(typed, 'origin');
assert.equal(origin.type, 'point');
assert.equal(origin.value, '{x = 3, y = 4, label = "origin"}');
assert.deepEqual(childRows(typed, origin).map((r) => [r.name, r.type, r.value]),
  [['x', 'int', '3'], ['y', 'int', '4'], ['label', 'char *', '→ "origin"']]);
assert.equal(top(typed, 'favorite').value, '→ primes[3]', 'a pointer into an int array names the element');
assert.equal(top(typed, 'op').value, '→ twice');
assert.equal(top(typed, 'banner').value, '→ "kuna data tab"');
assert.deepEqual(placeOf(typed, 0x404cn), { key: '0x4040/3', row: childRows(typed, primes)[3], open: ['0x4040'] }, 'an address inside an array opens it to the element');
assert.equal(placeOf(typed, 0x4068n).key, '0x4060/2', 'and inside a struct to the field');
assert.equal(rowOf(typed, '0x4060/2').name, 'label');
assert.equal(rowOf(typed, '0x4060/9'), null);
checks.push('typed: arrays, structs, pointers');

// ── bytes that start as zeros ──────────────────────────────────────────────
const bss = dataModel(fixture('data-globals-bss.json'), { fnAt });
assert.equal(bss.section.name, '.bss');
assert.equal(top(bss, 'scratch').value, 'uninitialized', 'nothing in the file backs .bss');
assert.equal(top(bss, 'scratch').cls, 'dv-uninit');
assert.match(bss.rows.find((r) => r.kind === 'uninit').value, /^\d+ bytes, uninitialized$/);
assert.match(renderData(bss), /class="dv-v dv-uninit" title="The file holds no bytes for this\./, 'and says why');
checks.push('.bss');

// ── stripped: the decompiler's own declarations ────────────────────────────
const stripped = fixture('inspect-globals-stripped.json').function.globals;
const sm = dataModel(fixture('data-globals-stripped.json'), { fnAt, known: stripped });
assert.equal(top(sm, 'dat_4040').value, '{2, 3, 5, 7, 11, 13, 17, 19}', 'an open array runs to the next item');
assert.equal(top(sm, 'dat_4040').type, 'int[]');
assert.equal(top(sm, 'dat_4078').value, '→ twice');
assert.equal(top(sm, 'dat_4080').value, '→ dat_4040[3]');
assert.equal(top(sm, 'dat_4068').value, '→ "origin"');
assert.equal(top(sm, 's_4020').value, '"hello, data"');
checks.push('stripped');

// ── unlabelled bytes ───────────────────────────────────────────────────────
const raw = (bytes, extra = {}) => dataModel({
  start_hex: '0x1000', span: bytes.length, size: bytes.length, bytes: Buffer.from(bytes).toString('hex'), pointer_size: 8,
  target: { endian: 'little' }, section: { name: '.data', address_hex: '0x1000', size: bytes.length, file_offset: 0, file_size: bytes.length, writable: true },
  items: [], targets: [], ...extra,
}, { fnAt });
const gaps = raw([...Buffer.from('flag{x}\0'), 1, 2, 3, ...Array(12).fill(0), 9]);
assert.deepEqual(gaps.rows.map((r) => [r.kind, r.hex, r.value]), [
  ['text', '0x1000', '"flag{x}"'], ['bytes', '0x1008', '01 02 03 00 00 00 00 00'], ['bytes', '0x1010', '00 00 00 00 00 00 00 09'],
]);
const ptr = raw([0x89, 0x11, 0, 0, 0, 0, 0, 0], { targets: [{ at_hex: '0x1000', to_hex: '0x1189', name: 'twice', offset: 0, text: null }] });
assert.deepEqual(ptr.rows.map((r) => [r.kind, r.value]), [['pointer', '→ twice']]);
assert.equal(pointerValue(ptr, 0x1000n).link.kind, 'fn');
const tail = raw([1, 2, 3, 4], { section: { name: '.data', address_hex: '0x1000', size: 64, file_offset: 0, file_size: 2 } });
assert.deepEqual(gapRows(tail, 0x1000n, 0x1004n).map((r) => [r.kind, r.size]), [['bytes', 2], ['uninit', 2]], 'past the file data the bytes start as zeros');
checks.push('unlabelled bytes');

// ── an array larger than the window ────────────────────────────────────────
const big = dataModel({
  start_hex: '0x2000', span: 16, size: 16, bytes: Buffer.from(Uint32Array.from([100, 101, 102, 103]).buffer).toString('hex'), pointer_size: 8,
  target: { endian: 'little' }, section: { name: '.data', address_hex: '0x1000', size: 0x4000, file_offset: 0, file_size: 0x4000, writable: true },
  items: [{ name: 'table', address_hex: '0x1000', size: 0x2000, type: 'int[2048]', declaration: 'int table[2048]',
    layout: { kind: 'array', size: 0x2000, type: 'int[2048]', count: 2048, elem: { kind: 'int', size: 4, type: 'int', signed: true } } }], targets: [],
}, { fnAt });
assert.equal(big.rows[0].value, '{…, 100, 101, 102, 103, …}', 'the summary shows the elements the window holds');
assert.deepEqual(childRows(big, big.rows[0]).map((r) => [r.name, r.value]), [['[1024]', '100 · 0x64'], ['[1025]', '101 · 0x65'], ['[1026]', '102 · 0x66'], ['[1027]', '103 · 0x67']]);
assert.equal(placeOf(big, 0x2008n).key, '0x1000/2', 'an address deep in it opens to its element');
assert.match(renderData(big, { open: new Set(['0x1000']) }), /2044 more not shown here/);
const blob = dataModel({ ...JSON.parse(JSON.stringify({ start_hex: '0x2000', span: 16, size: 16, bytes: '00112233445566778899aabbccddeeff', pointer_size: 8, target: { endian: 'little' }, targets: [],
  section: { name: '.data', address_hex: '0x1000', size: 0x4000, file_offset: 0, file_size: 0x4000, writable: true } })),
items: [{ name: 'blob', address_hex: '0x1000', size: 0x2000, type: 'char[8192]', declaration: '', layout: { kind: 'array', size: 0x2000, type: 'char[8192]', count: 8192, elem: { kind: 'unknown', size: 1, type: 'char' } } }] }, { fnAt });
assert.equal(blob.rows[0].value, '… 00 11 22 33 44 55 66 77 88 99 aa bb cc dd ee ff …', 'an untyped one shows the bytes the window holds');
assert.deepEqual(childRows(blob, blob.rows[0]).map((r) => [r.hex, r.value]), [['0x2000', '00 11 22 33 44 55 66 77 88 99 aa bb cc dd ee ff']]);
checks.push('an array larger than the window');

// ── who uses it ────────────────────────────────────────────────────────────
const uses = fixture('uses-globals.json').uses;
const names = (hex, name) => (hex === '0x119b' ? 'main' : name);
assert.deepEqual(usesOf(uses, 0x4040n, 32, names).map((g) => [g.name, g.sites.length, [...g.kinds]]), [['main', 1, ['data']]]);
assert.deepEqual(usesOf(uses, 0x4060n, 16, names).map((g) => [g.name, g.sites.length]), [['main', 2]], 'every byte of the item counts');
assert.equal(useWords(new Set(['read'])), 'reads it');
assert.equal(useWords(new Set(['data'])), 'uses its address');
assert.equal(useWords(new Set(['read', 'write'])), 'reads and writes it');
assert.deepEqual(usesOf(uses, 0x4100n, 8, names), []);
checks.push('uses');

// ── HTML ───────────────────────────────────────────────────────────────────
const html = renderData(typed, { selected: '0x4040/3', open: new Set(['0x4040']) });
assert.match(html, /^<div class="dv-rows" style="--nw:\d+ch;--tw:\d+ch"><div class="dv-sec"><b>\.data<\/b>/);
assert.match(html, /data-key="0x4040\/3"[^>]*>.*\[3\]/, 'an open array shows its elements');
assert.match(html, /class="dv-row dv-elem sel"/, 'the selected element is marked');
assert.match(html, /data-act="dv-follow" data-to="0x1189" data-kind="fn"/);
assert.ok(!html.includes('data-key="0x4060/0"'), 'a closed struct hides its fields');
const evil = dataModel({ ...fixture('data-globals.json'), items: [{ name: '<img src=x>', address_hex: '0x4020', size: 12, type: '<b>', declaration: '', layout: { kind: 'unknown', size: 12, type: '<b>' } }] }, { fnAt });
const evilHtml = renderData(evil);
assert.ok(!evilHtml.includes('<img') && !evilHtml.includes('<b>&') && evilHtml.includes('&lt;img src=x&gt;'), 'engine strings are escaped');
checks.push('html');

// ── going to a section ─────────────────────────────────────────────────────
const secs = [
  { name: '.bss', address_hex: '0x4090', size: 0x30, file_offset: null, executable: false, writable: true },
  { name: '.text', address_hex: '0x1060', size: 0x200, file_offset: 0x1060, executable: true, writable: false },
  { name: '.rodata', address_hex: '0x2000', size: 0x40, file_offset: 0x2000, executable: false, writable: false },
  { name: '.data', address_hex: '0x4000', size: 0x88, file_offset: 0x3000, executable: false, writable: true },
  { name: '.tbss', address_hex: '0x3df0', size: 0, file_offset: null, executable: false, writable: true },
];
assert.deepEqual(dataSections(secs).map((s) => s.name), ['.rodata', '.data', '.bss'], 'data sections in address order, no code, none empty');
assert.deepEqual(dataSections(undefined), []);
assert.deepEqual([12, 1, 4608, 3 * 1048576 + 200000].map(sizeWords), ['12 bytes', '1 byte', '4.5 KB', '3.2 MB']);
assert.equal(sectionWords(secs[0]), 'data that starts as zeros');
assert.equal(sectionWords(secs[2]), 'data that never changes');
const pick = sectionPicker(dataSections(secs), { address_hex: '0x4090' });
assert.match(pick, /^<select class="dv-pick" data-act="dv-section"/);
assert.match(pick, /<option value="0x4090" selected>\.bss<\/option>/, 'the section on screen is chosen');
assert.ok(!pick.includes('.text'), 'code is not listed');
const withMenu = renderData(typed, { sections: dataSections([...secs, typed.section]) });
assert.match(withMenu, /<div class="dv-sec"><select class="dv-pick"[^]*<option value="0x4000" selected>\.data<\/option>/, 'the heading is a menu of the sections');
assert.match(renderData(typed, { sections: [typed.section] }), /<div class="dv-sec"><b>\.data<\/b>/, 'one section needs no menu');
assert.ok(sectionPicker([{ name: '<x>', address_hex: '0x10', size: 1 }], null).includes('&lt;x&gt;'), 'section names are escaped');
checks.push('sections');

console.log(`DECOMPILE DATA OK — ${checks.join(' · ')}`);
