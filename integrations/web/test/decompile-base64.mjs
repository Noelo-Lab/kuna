// decompile-base64.mjs — opening a program pasted as base64 text: which texts
// decode, to which bytes, and the name the page gives them.
//
//   node integrations/web/test/decompile-base64.mjs

import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { decodeBase64, pastedName } from '../decompile/base64.js';

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
