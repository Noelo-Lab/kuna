// zip.mjs — structurally validate zip.js's output, with no dependencies.
//
// Builds a STORE-only archive from mixed entries (a multi-KB string plus raw
// Uint8Array bytes), then re-parses the bytes it produced: EOCD signature and
// entry counts, every central-directory record (name, method 0, UTF-8 flag,
// local-header offset), every local header (signature, name, method 0), the
// fixed DOS timestamp, and an INDEPENDENTLY recomputed CRC-32 per payload.
// Also asserts byte-determinism (two builds identical). If a system `unzip`
// exists, additionally runs `unzip -t` as a bonus check (skipped silently
// when absent).
//
// Usage:  node zip.mjs
import { spawnSync } from 'node:child_process';
import { mkdtempSync, writeFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { makeZip } from '../zip.js';

function fail(m) { console.error(`\x1b[31mFAIL\x1b[0m ${m}`); process.exit(1); }
const ok = (m) => console.log(`\x1b[32mOK\x1b[0m   ${m}`);

// Independent CRC-32 (bitwise, table-free — deliberately NOT zip.js's
// table-driven implementation, so a table bug can't self-validate).
function crc32(bytes) {
  let c = 0xffffffff;
  for (let i = 0; i < bytes.length; i++) {
    c ^= bytes[i];
    for (let k = 0; k < 8; k++) c = (c >>> 1) ^ (0xedb88320 & -(c & 1));
  }
  return (c ^ 0xffffffff) >>> 0;
}

const big = 'int sum_to(int n) { int s = 0; while (n > 0) s += n--; return s; }\n'.repeat(120); // ~8 KB
const raw = new Uint8Array(4096);
for (let i = 0; i < raw.length; i++) raw[i] = (i * 7 + 13) & 0xff;
const entries = [
  { name: 'sample.kuna/sample.c', data: big },
  { name: 'sample.kuna/README.md', data: '# sample\n\nexported by kuna\n' },
  { name: 'sample.kuna/raw.bin', data: raw },
];

const zip = makeZip(entries);
const dv = new DataView(zip.buffer, zip.byteOffset, zip.byteLength);
const dec = new TextDecoder();
const enc = new TextEncoder();
const DOS_DATE = ((2026 - 1980) << 9) | (1 << 5) | 1; // fixed 2026-01-01 00:00:00

// EOCD (no archive comment, so it is exactly the last 22 bytes).
const eocd = zip.length - 22;
if (dv.getUint32(eocd, true) !== 0x06054b50) fail('EOCD signature missing');
if (dv.getUint16(eocd + 8, true) !== entries.length) fail('EOCD per-disk entry count wrong');
if (dv.getUint16(eocd + 10, true) !== entries.length) fail('EOCD total entry count wrong');
const cdSize = dv.getUint32(eocd + 12, true);
const cdOff = dv.getUint32(eocd + 16, true);
if (cdOff + cdSize !== eocd) fail('central directory does not abut the EOCD');
ok(`EOCD: ${entries.length} entries, central dir @${cdOff} (+${cdSize})`);

// Walk the central directory in entry order; verify each local header + payload.
let p = cdOff;
for (const e of entries) {
  if (dv.getUint32(p, true) !== 0x02014b50) fail(`central-directory signature missing @${p}`);
  if ((dv.getUint16(p + 8, true) & 0x0800) === 0) fail(`${e.name}: UTF-8 flag (bit 11) unset`);
  if (dv.getUint16(p + 10, true) !== 0) fail(`${e.name}: method != STORE in central dir`);
  if (dv.getUint16(p + 12, true) !== 0 || dv.getUint16(p + 14, true) !== DOS_DATE)
    fail(`${e.name}: DOS timestamp not the fixed 2026-01-01 00:00:00`);
  const crc = dv.getUint32(p + 16, true);
  const csize = dv.getUint32(p + 20, true);
  const usize = dv.getUint32(p + 24, true);
  const nlen = dv.getUint16(p + 28, true);
  const xlen = dv.getUint16(p + 30, true);
  const clen = dv.getUint16(p + 32, true);
  const lho = dv.getUint32(p + 42, true);
  const cname = dec.decode(zip.subarray(p + 46, p + 46 + nlen));
  if (cname !== e.name) fail(`central-directory name '${cname}' != '${e.name}'`);

  if (dv.getUint32(lho, true) !== 0x04034b50) fail(`${e.name}: local-header signature missing @${lho}`);
  if (dv.getUint16(lho + 8, true) !== 0) fail(`${e.name}: method != STORE in local header`);
  const lnlen = dv.getUint16(lho + 26, true);
  const lxlen = dv.getUint16(lho + 28, true);
  const lname = dec.decode(zip.subarray(lho + 30, lho + 30 + lnlen));
  if (lname !== e.name) fail(`local-header name '${lname}' != '${e.name}'`);

  const want = typeof e.data === 'string' ? enc.encode(e.data) : e.data;
  if (csize !== usize || csize !== want.length)
    fail(`${e.name}: sizes wrong (stored ${csize}/${usize}, expected ${want.length})`);
  const payload = zip.subarray(lho + 30 + lnlen + lxlen, lho + 30 + lnlen + lxlen + csize);
  if (Buffer.compare(Buffer.from(payload), Buffer.from(want)) !== 0) fail(`${e.name}: payload bytes differ`);
  const got = crc32(payload);
  if (got !== crc) fail(`${e.name}: recomputed CRC-32 ${got.toString(16)} != stored ${crc.toString(16)}`);
  ok(`${e.name}: local+central agree, STORE, ${want.length} bytes, crc32 ${crc.toString(16).padStart(8, '0')}`);
  p += 46 + nlen + xlen + clen;
}
if (p !== cdOff + cdSize) fail('central-directory walk did not consume exactly cdSize');

// Determinism: a second build must be byte-identical (fixed timestamp et al).
if (Buffer.compare(Buffer.from(zip), Buffer.from(makeZip(entries))) !== 0)
  fail('makeZip is not deterministic (two builds differ)');
ok('deterministic: two builds byte-identical');

// Bonus: a real extractor accepts it (skip silently if `unzip` is absent).
if (!spawnSync('unzip', ['-v'], { stdio: 'ignore' }).error) {
  const dir = mkdtempSync(join(tmpdir(), 'kuna-zip-'));
  const zf = join(dir, 'out.zip');
  writeFileSync(zf, zip);
  const r = spawnSync('unzip', ['-t', zf], { encoding: 'utf8' });
  rmSync(dir, { recursive: true, force: true });
  if (r.status !== 0) fail(`unzip -t rejected the archive:\n${r.stdout}${r.stderr}`);
  ok('bonus: system `unzip -t` accepts the archive');
}

console.log(`\n\x1b[32mZIP OK\x1b[0m — ${entries.length}-entry STORE zip parses back cleanly (headers, names, CRC-32s) and is deterministic.`);
