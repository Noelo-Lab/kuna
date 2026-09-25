// make-inspect-fixtures.mjs — regenerate the study view's contract fixtures
// (test/fixtures/inspect-*.json, list-sample.json) from the NATIVE CLI on
// test/fixtures/sample.elf, so the page's pure renderers can be tested
// without a wasm build.
//
// `inspect` documents are assembled by hand from `kuna decompile --json`
// (code, line_mappings, variables with lines/addresses, types) and `kuna
// disassemble --json` (instruction rows); `offset`, `file_offset` and each
// row's `lines` are derived here, `tokens` is left empty with `tokens_error`
// explaining why. `target` and `known_types` are the contract's shapes filled
// with the values this x86-64 ELF has.
//
// Usage:  make binaries && node integrations/web/test/make-inspect-fixtures.mjs
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const here = dirname(fileURLToPath(import.meta.url));
const repo = resolve(here, '../../..');
const kuna = process.env.KUNA || join(repo, 'decompiler/target/release/kuna');
const elfPath = join(here, 'fixtures/sample.elf');
const env = { ...process.env, KUNA_SPECS: process.env.KUNA_SPECS || join(repo, 'specs') };
const run = (...args) => JSON.parse(execFileSync(kuna, args, { env, maxBuffer: 64 << 20 }).toString());

/** ELF64 little-endian section headers: [{name, address, size, file_offset, flags, type}]. */
function elfSections(buf) {
  const dv = new DataView(buf.buffer, buf.byteOffset, buf.byteLength);
  const shoff = Number(dv.getBigUint64(0x28, true));
  const shentsize = dv.getUint16(0x3a, true);
  const shnum = dv.getUint16(0x3c, true);
  const shstrndx = dv.getUint16(0x3e, true);
  const hdr = (i) => {
    const o = shoff + i * shentsize;
    return {
      nameOff: dv.getUint32(o, true), type: dv.getUint32(o + 4, true),
      flags: Number(dv.getBigUint64(o + 8, true)), addr: Number(dv.getBigUint64(o + 16, true)),
      offset: Number(dv.getBigUint64(o + 24, true)), size: Number(dv.getBigUint64(o + 32, true)),
    };
  };
  const strtab = hdr(shstrndx);
  const name = (off) => {
    let end = strtab.offset + off;
    while (buf[end]) end++;
    return buf.subarray(strtab.offset + off, end).toString('latin1');
  };
  const out = [];
  for (let i = 1; i < shnum; i++) {
    const h = hdr(i);
    if (!(h.flags & 2)) continue; // SHF_ALLOC only: what the loader maps
    out.push({
      name: name(h.nameOff),
      address: h.addr,
      address_hex: '0x' + h.addr.toString(16),
      size: h.size,
      file_offset: h.type === 8 ? null : h.offset, // SHT_NOBITS has no file bytes
      executable: !!(h.flags & 4),
      writable: !!(h.flags & 1),
    });
  }
  return out;
}

const elf = readFileSync(elfPath);
const sections = elfSections(elf);
const fileOffset = (addr) => {
  const s = sections.find((x) => x.file_offset !== null && addr >= x.address && addr < x.address + x.size);
  return s ? s.file_offset + (addr - s.address) : null;
};
const target = { archid: 'x86:LE:64:default:gcc', processor: 'x86', endian: 'little', bits: 64 };

function inspect(name) {
  const dec = run('decompile', elfPath, name, '--json');
  const fn = dec.functions[0];
  const dis = run('disassemble', elfPath, name, '--json');
  const lines = new Map();
  for (const m of fn.line_mappings || []) {
    for (const a of m.addresses) {
      if (!lines.has(a)) lines.set(a, []);
      lines.get(a).push(m.line_number);
    }
  }
  const instructions = dis.instructions.map((i) => ({
    address: i.address,
    address_hex: i.address_hex,
    offset: i.address - fn.address,
    size: i.size,
    bytes: i.bytes,
    mnemonic: i.mnemonic,
    operands: i.operands,
    text: i.text,
    file_offset: fileOffset(i.address),
    lines: (lines.get(i.address) || []).sort((a, b) => a - b),
  }));
  return {
    binary: 'sample.elf',
    language: dec.language,
    target,
    function: {
      name: fn.name,
      address: fn.address,
      address_hex: fn.address_hex,
      aliases: fn.aliases,
      object_location: fn.object_location,
      kind: 'func',
      size: fn.size,
      code: fn.code,
      error: fn.error,
      proto: fn.code.split('\n')[0],
      unstructured_gotos: fn.unstructured_gotos,
      line_mappings: fn.line_mappings,
      variables: fn.variables,
      types: fn.types || [],
      globals: fn.globals || [],
      tokens: [],
      tokens_error: 'fixture assembled from the native CLI, which has no token stream',
      instructions,
      instructions_truncated: dis.truncated,
    },
    assertions: dec.assertions || [],
  };
}

const funcs = run('functions', elfPath, '--json');
const kinds = { '0x1020': 'plt', '0x1040': 'thunk', '0x1050': 'plt' };
const list = {
  binary: 'sample.elf',
  language: 'c-language',
  target,
  count: funcs.functions.length,
  functions: funcs.functions.map((f) => ({
    name: f.name, address: f.address, address_hex: f.address_hex, aliases: f.aliases,
    object_location: f.object_location, size: f.size, kind: kinds[f.address_hex] || 'func',
  })),
  sections,
  known_types: [],
  assertions: [],
};

const write = (file, doc) => writeFileSync(join(here, 'fixtures', file), JSON.stringify(doc, null, 1) + '\n');
for (const name of ['main', 'sum_to', 'add']) write(`inspect-${name}.json`, inspect(name));
write('list-sample.json', list);
console.log('wrote inspect-{main,sum_to,add}.json and list-sample.json');
