// decompile2-base64.mjs — opening a program pasted as base64 text, gzipped or
// not: which texts decode, to which bytes, and the name the page gives them.
//
//   node integrations/web/test/decompile2-base64.mjs

import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { gzipSync } from 'node:zlib';
import { decodeBase64, gzipName, pastedName, unpackProgram } from '../decompile/base64.js';

const ok = (msg) => console.log(`\x1b[32mOK\x1b[0m   ${msg}`);

const elf = new Uint8Array(readFileSync(new URL('./fixtures/sample.elf', import.meta.url)));
const b64 = Buffer.from(elf).toString('base64');
const same = (got, want = elf) => assert.deepEqual(got && [...got], [...want]);

same(decodeBase64(b64));
same(decodeBase64(b64.match(/.{1,76}/g).join('\n')));
same(decodeBase64(`\r\n  ${b64.match(/.{1,64}/g).join('\r\n')}  \n`));
ok('plain, wrapped (base64 -w76, PEM-style CRLF) and padded-with-whitespace text decode to the file');

same(decodeBase64(`data:application/octet-stream;base64,${b64}`));
same(decodeBase64(Buffer.from(elf).toString('base64url')));
ok('a data: URL and the URL-safe alphabet without padding decode to the file');

for (const [text, bytes] of [['QQ==', [0x41]], ['QQ', [0x41]], ['QUI=', [0x41, 0x42]], ['QUI', [0x41, 0x42]], ['QUJD', [0x41, 0x42, 0x43]]]) {
  same(decodeBase64(text), bytes);
}
ok('every padding length decodes, with or without its = signs');

for (const bad of ['', '   ', 'Q', 'QUJDR', 'QQ===', 'Q=Q=', 'QUJD!', 'int main(void) { return 0; }', '0x401000;', null, undefined]) {
  assert.equal(decodeBase64(bad), null, JSON.stringify(bad));
}
ok('empty text, C code, stray symbols, an impossible length and bad padding are not base64');

assert.equal(pastedName(elf), 'pasted.elf');
assert.equal(pastedName(Uint8Array.of(0x4d, 0x5a, 0x90, 0)), 'pasted.exe');
assert.equal(pastedName(Uint8Array.of(0xcf, 0xfa, 0xed, 0xfe)), 'pasted.macho');
assert.equal(pastedName(Uint8Array.of(1, 2, 3)), 'pasted.bin');
assert.equal(pastedName(new Uint8Array(0)), 'pasted.bin');
ok('the pasted program is named after its format');

/** `gz` with the FNAME (and optionally FEXTRA) fields `gzip` writes for a named file. */
function named(gz, name, extra = null) {
  const head = Uint8Array.from(gz.subarray(0, 10));
  head[3] |= 0x08 | (extra ? 0x04 : 0);
  const xfield = extra ? [extra.length & 0xff, extra.length >> 8, ...extra] : [];
  return Uint8Array.from([...head, ...xfield, ...Buffer.from(name, 'latin1'), 0, ...gz.subarray(10)]);
}
const gz = new Uint8Array(gzipSync(elf, { level: 9 }));

let p = await unpackProgram(decodeBase64(Buffer.from(named(gz, 'check')).toString('base64')));
same(p.bytes);
assert.equal(p.name, 'check');
ok('gzip -9c check | base64 -w0 opens as the program, named check from the gzip header');

p = await unpackProgram(gz);
same(p.bytes);
assert.equal(p.name, 'pasted.elf');
assert.equal((await unpackProgram(gz, 'check.gz')).name, 'check');
assert.equal((await unpackProgram(gz, 'CHECK.GZ')).name, 'CHECK');
ok('gzip from a pipe (no stored name) is named by format when pasted, by the file name minus .gz when opened');

assert.equal(gzipName(named(gz, '/challenge/check')), 'check');
assert.equal(gzipName(named(gz, 'C:\\tmp\\a.exe')), 'a.exe');
assert.equal(gzipName(named(gz, 'babyrev', [0x41, 0x50, 2, 0, 1, 2])), 'babyrev');
same((await unpackProgram(named(gz, 'babyrev', [0x41, 0x50, 2, 0, 1, 2]))).bytes);
assert.equal(gzipName(gz), '');
assert.equal(gzipName(elf), '');
ok('the stored name loses its directories and is found past an extra field');

p = await unpackProgram(elf, 'a.out');
assert.equal(p.name, 'a.out');
assert.equal(p.bytes, elf);
const torn = gz.slice(0, gz.length >> 1);
await assert.rejects(unpackProgram(torn), 'a truncated gzip stream is refused');
const bad = gz.slice();
bad[40] ^= 0xff;
await assert.rejects(unpackProgram(bad), 'a corrupted gzip stream is refused');
ok('bytes that are not gzip pass through untouched; damaged gzip data is refused');
