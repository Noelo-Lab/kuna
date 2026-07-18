// zip.js — a minimal, dependency-free ZIP writer (STORE only, method 0).
//
// Emits local file headers, a central directory, and the end-of-central-
// directory record per the PKWARE APPNOTE (§4.3), with CRC-32 checksums
// (RFC 1952 §8 polynomial), UTF-8 names (general-purpose flag bit 11), and a
// FIXED DOS timestamp (2026-01-01 00:00:00) so output is byte-deterministic.
// No compression — entries are stored verbatim, which is fine for the small
// text projects this ships and keeps the writer trivially verifiable.

// Standard CRC-32 table (reflected polynomial 0xEDB88320).
const CRC_TABLE = (() => {
  const t = new Uint32Array(256);
  for (let n = 0; n < 256; n++) {
    let c = n;
    for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1;
    t[n] = c >>> 0;
  }
  return t;
})();

function crc32(bytes) {
  let c = 0xffffffff;
  for (let i = 0; i < bytes.length; i++) c = CRC_TABLE[(c ^ bytes[i]) & 0xff] ^ (c >>> 8);
  return (c ^ 0xffffffff) >>> 0;
}

// DOS-format date/time for 2026-01-01 00:00:00 (fixed => deterministic zips).
const DOS_TIME = 0; // hours<<11 | minutes<<5 | seconds/2
const DOS_DATE = ((2026 - 1980) << 9) | (1 << 5) | 1; // (year-1980)<<9 | month<<5 | day

/**
 * Build a STORE-only zip from `entries` = [{name, data}], where `data` is a
 * string (UTF-8 encoded) or a Uint8Array. Names may contain '/' to nest
 * (e.g. 'proj.kuna/proj.c'). Returns the archive as a Uint8Array.
 */
export function makeZip(entries) {
  const enc = new TextEncoder();
  const files = entries.map(({ name, data }) => {
    const bytes = typeof data === 'string' ? enc.encode(data) : data;
    return { name: enc.encode(name), bytes, crc: crc32(bytes), offset: 0 };
  });

  const chunks = [];
  let offset = 0;
  const push = (c) => { chunks.push(c); offset += c.length; };

  // Local file headers, each followed by its stored payload.
  for (const f of files) {
    f.offset = offset;
    const h = new Uint8Array(30 + f.name.length);
    const v = new DataView(h.buffer);
    v.setUint32(0, 0x04034b50, true); // local file header signature
    v.setUint16(4, 20, true); // version needed to extract (2.0)
    v.setUint16(6, 0x0800, true); // flags: bit 11 = UTF-8 name
    v.setUint16(8, 0, true); // compression method 0 = STORE
    v.setUint16(10, DOS_TIME, true);
    v.setUint16(12, DOS_DATE, true);
    v.setUint32(14, f.crc, true);
    v.setUint32(18, f.bytes.length, true); // compressed size (== raw for STORE)
    v.setUint32(22, f.bytes.length, true); // uncompressed size
    v.setUint16(26, f.name.length, true);
    v.setUint16(28, 0, true); // extra field length
    h.set(f.name, 30);
    push(h);
    push(f.bytes);
  }

  // Central directory.
  const cdStart = offset;
  for (const f of files) {
    const h = new Uint8Array(46 + f.name.length);
    const v = new DataView(h.buffer);
    v.setUint32(0, 0x02014b50, true); // central directory header signature
    v.setUint16(4, 20, true); // version made by
    v.setUint16(6, 20, true); // version needed to extract
    v.setUint16(8, 0x0800, true); // flags: bit 11 = UTF-8 name
    v.setUint16(10, 0, true); // method STORE
    v.setUint16(12, DOS_TIME, true);
    v.setUint16(14, DOS_DATE, true);
    v.setUint32(16, f.crc, true);
    v.setUint32(20, f.bytes.length, true);
    v.setUint32(24, f.bytes.length, true);
    v.setUint16(28, f.name.length, true);
    // 30..41: extra/comment lengths, disk number, internal/external attrs — all 0.
    v.setUint32(42, f.offset, true); // local header offset
    h.set(f.name, 46);
    push(h);
  }
  const cdSize = offset - cdStart;

  // End of central directory record.
  const e = new Uint8Array(22);
  const v = new DataView(e.buffer);
  v.setUint32(0, 0x06054b50, true); // EOCD signature
  v.setUint16(8, files.length, true); // entries on this disk
  v.setUint16(10, files.length, true); // entries total
  v.setUint32(12, cdSize, true);
  v.setUint32(16, cdStart, true);
  push(e);

  const out = new Uint8Array(offset);
  let p = 0;
  for (const c of chunks) { out.set(c, p); p += c.length; }
  return out;
}
