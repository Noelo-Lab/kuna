// decompile2-bytes.mjs — the study view's bytes pane and patching, from the
// source tree with no build: file offsets from the section table, the patched
// file (and what cannot be written), hex rows, no-op fills per architecture,
// hex parsing and the patched file's name.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import {
  fileOffsetFor, originalByteAt, functionCells, hexRows, applyPatches, parseHex, patchedName, renderHex,
} from '../decompile2/bytes-view.js';
import { archFrom, nopFill } from '../decompile2/arch.js';
import { normalizeInspect } from '../decompile2/render-c.js';

const here = new URL('./fixtures/', import.meta.url);
const list = JSON.parse(readFileSync(new URL('list-sample.json', here), 'utf8'));
const elf = new Uint8Array(readFileSync(new URL('sample.elf', here)));
const main = normalizeInspect(JSON.parse(readFileSync(new URL('inspect-main.json', here), 'utf8')));
const checks = [];

// ── file offsets ───────────────────────────────────────────────────────────
assert.equal(fileOffsetFor('0x1198', list.sections), 0x1198, '.text maps 1:1 in this ELF');
assert.equal(fileOffsetFor(0x2004n, list.sections), 0x2004, '.rodata');
assert.equal(fileOffsetFor('0x4000', list.sections), 0x3000, '.data sits 0x1000 below its address in the file');
assert.equal(fileOffsetFor('0x4010', list.sections), null, '.bss has no file bytes');
assert.equal(fileOffsetFor('0x999999', list.sections), null, 'unmapped');
assert.equal(fileOffsetFor('0x1198', []), null, 'no section table, no offset');
for (const insn of main.instructions) {
  const off = fileOffsetFor(insn.address_hex, list.sections);
  assert.equal(off, insn.file_offset, `${insn.address_hex} offset agrees with the fixture`);
  assert.equal(Buffer.from(elf.subarray(off, off + insn.size)).toString('hex'), insn.bytes, `${insn.address_hex} bytes are the file's`);
}
assert.equal(originalByteAt('0x11e1', { fileBytes: elf, sections: list.sections }), 0xe8);
checks.push('fileOffsetFor/originalByteAt');

// ── cells and rows ─────────────────────────────────────────────────────────
const patches = new Map([[0x11e1n, 0x90]]);
const cells = functionCells(main.instructions, { entry: main.address_hex, size: main.size, fileBytes: elf, sections: list.sections, patches });
assert.equal(cells.length, 84);
const call = cells.find((c) => c.hex === '0x11e1');
assert.deepEqual([call.value, call.orig, call.patched, call.first], [0x90, 0xe8, true, true]);
assert.equal(cells.filter((c) => c.patched).length, 1);
assert.ok(cells.every((c) => !c.gap), 'every byte of main is known');
const rows = hexRows(cells);
assert.deepEqual(rows.map((r) => r.base.toString(16)), ['1190', '11a0', '11b0', '11c0', '11d0', '11e0']);
assert.equal(rows[0].cells.slice(0, 8).every((c) => c === null), true, 'the first row starts at 0x1198');
assert.equal(main.instructions[20].address_hex, '0x11e1');
const html = renderHex(rows, { sections: list.sections, selected: new Set([20]), editAddr: 0x11e2n });
assert.match(html, /<span class="hb pa in" data-a="0x11e1" data-i="20" title="was e8">90<\/span>/);
assert.match(html, /<span class="ha">\.<i class="pa">\.<\/i>j\.\.\.H\.E\.\.\. {4}<\/span>/, 'the ASCII column marks the patched byte');
assert.match(html, /class="hb in ed" data-a="0x11e2"/);
assert.match(html, /<span class="ho" title="file offset 0x11e0">11e0<\/span>/);
assert.ok(!/<(?!\/?(span|div|i)\b)/.test(html), 'only the renderer\'s own tags');
checks.push('functionCells/hexRows/renderHex');

// ── the patched file ───────────────────────────────────────────────────────
const nopped = applyPatches(elf, list.sections, [{ addr: 0x11e1n, values: [0x90, 0x90, 0x90, 0x90, 0x90] }]);
assert.deepEqual(nopped.unmapped, []);
assert.equal(nopped.bytes.length, elf.length, 'the file keeps its length');
const diff = [];
nopped.bytes.forEach((b, i) => { if (b !== elf[i]) diff.push(i); });
assert.deepEqual(diff, [0x11e1, 0x11e2, 0x11e3, 0x11e4, 0x11e5], 'only the patched bytes change');
assert.notEqual(nopped.bytes, elf, 'the input is not modified');
assert.equal(elf[0x11e1], 0xe8);
const partial = applyPatches(elf, list.sections, [{ addr: 0x400en, values: [1, 2, 3, 4] }]);
assert.deepEqual(partial.unmapped, [{ addr: '0x400e', count: 2 }], 'bytes past .data into .bss are reported, not written');
assert.equal(partial.bytes[0x300e], 1);
checks.push('applyPatches (+ unmapped)');

// ── architectures and no-op fills ──────────────────────────────────────────
assert.deepEqual(archFrom({ archid: 'x86:LE:64:default:gcc' }), { family: 'x86', endian: 'little', bits: 64, thumb: false });
assert.equal(archFrom({ archid: 'AARCH64:LE:64:v8A:default' }).family, 'aarch64');
assert.equal(archFrom({ archid: 'ARM:LE:32:v8T' }, [{ size: 2, mnemonic: 'push', operands: '{r4,lr}' }]).thumb, true);
assert.equal(archFrom({ archid: 'ARM:LE:32:v7' }, [{ size: 4, mnemonic: 'mov', operands: 'r0,r1' }]).thumb, false);
assert.deepEqual(archFrom({ archid: 'MIPS:BE:32:default' }), { family: 'mips', endian: 'big', bits: 32, thumb: false });
assert.equal(archFrom({ archid: 'PowerPC:BE:64:default' }).family, 'ppc');
assert.equal(archFrom({ archid: 'RISCV:LE:64:RV64GC' }).family, 'riscv');
assert.equal(archFrom(null, main.instructions).family, 'x86', 'registers identify x86 without a target');
assert.equal(archFrom(null, [{ size: 4, mnemonic: 'ldr', operands: 'x0,[sp, #0x10]' }]).family, 'aarch64');
assert.equal(archFrom(null, []).family, null);
assert.equal(nopFill({ family: 'x86' }, 5), '9090909090');
assert.equal(nopFill({ family: 'aarch64' }, 8), '1f2003d51f2003d5');
assert.equal(nopFill({ family: 'aarch64' }, 6), null, 'no partial AArch64 fill');
assert.equal(nopFill({ family: 'arm', thumb: false }, 4), '0000a0e1');
assert.equal(nopFill({ family: 'arm', thumb: true }, 4), '00bf00bf');
assert.equal(nopFill({ family: 'mips' }, 4), '00000000');
assert.equal(nopFill({ family: 'ppc', endian: 'big' }, 4), '60000000');
assert.equal(nopFill({ family: 'ppc', endian: 'little' }, 4), '00000060');
assert.equal(nopFill({ family: 'riscv' }, 4), '13000000');
assert.equal(nopFill({ family: 'riscv' }, 2), '0100');
assert.equal(nopFill({ family: null }, 4), null);
assert.equal(nopFill({ family: 'x86' }, 0), null);
checks.push('archFrom/nopFill');

// ── hex input and names ────────────────────────────────────────────────────
assert.deepEqual([...parseHex('90 90')], [0x90, 0x90]);
assert.deepEqual([...parseHex('0x9090')], [0x90, 0x90]);
assert.deepEqual([...parseHex('e8,8f, ff')], [0xe8, 0x8f, 0xff]);
assert.equal(parseHex('909'), null);
assert.equal(parseHex('zz'), null);
assert.equal(parseHex(''), null);
assert.equal(patchedName('sample.elf'), 'sample.patched.elf');
assert.equal(patchedName('a.out'), 'a.patched.out');
assert.equal(patchedName('crackme'), 'crackme.patched');
assert.equal(patchedName('.hidden'), '.hidden.patched');
assert.equal(patchedName('v1.2.exe'), 'v1.2.patched.exe');
checks.push('parseHex/patchedName');

console.log(`DECOMPILE2 BYTES OK — ${checks.join('; ')}`);
