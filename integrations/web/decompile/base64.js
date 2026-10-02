// base64.js — a program pasted as base64 text, or gzipped first
// (`gzip -9c prog | base64 -w0`). DOM-free, so test/decompile2-base64.mjs
// runs it under plain Node.

const DATA_URL = /^data:[^,]*;base64,/i;
const BODY = /^[A-Za-z0-9+/]+$/;

/**
 * The bytes `text` encodes, or null when it is not base64. Whitespace and line
 * breaks, a `data:…;base64,` prefix, the URL-safe alphabet and missing padding
 * are all accepted, so `base64` output and data URLs paste as they are.
 */
export function decodeBase64(text) {
  const s = String(text ?? '').trim().replace(DATA_URL, '').replace(/\s+/g, '')
    .replace(/-/g, '+').replace(/_/g, '/');
  const body = s.replace(/={1,2}$/, '');
  if (!body || !BODY.test(body) || body.length % 4 === 1) return null;
  if (s.length !== body.length && s.length % 4 !== 0) return null;
  const bin = atob(body + '='.repeat((4 - (body.length % 4)) % 4));
  const bytes = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) bytes[i] = bin.charCodeAt(i);
  return bytes;
}

/** A file name for pasted bytes, from their format's magic number. */
export function pastedName(bytes) {
  const at = (...m) => m.every((b, i) => bytes[i] === b);
  if (at(0x7f, 0x45, 0x4c, 0x46)) return 'pasted.elf';
  if (at(0x4d, 0x5a)) return 'pasted.exe';
  if (at(0xcf, 0xfa, 0xed, 0xfe) || at(0xce, 0xfa, 0xed, 0xfe) || at(0xca, 0xfe, 0xba, 0xbe)) return 'pasted.macho';
  return 'pasted.bin';
}

const isGzip = (b) => b.length >= 18 && b[0] === 0x1f && b[1] === 0x8b && b[2] === 8;

/** The file name a gzip header stores (gzip keeps it unless run with -n or on a pipe), or ''. */
export function gzipName(b) {
  if (!isGzip(b) || !(b[3] & 0x08)) return '';
  const at = b[3] & 0x04 ? 12 + (b[10] | (b[11] << 8)) : 10;
  const end = b.indexOf(0, at);
  if (end < 0 || end - at > 1024) return '';
  return String.fromCharCode(...b.subarray(at, end)).split(/[/\\]/).pop();
}

/**
 * The program in `bytes` as `{name, bytes}`. Gzip data is inflated and named
 * from its header, else from `name` without `.gz`; anything else is returned
 * as it is. A null `name` means pasted, so the format names it.
 */
export async function unpackProgram(bytes, name = null) {
  if (!isGzip(bytes)) return { name: name ?? pastedName(bytes), bytes };
  const inflated = new Blob([bytes]).stream().pipeThrough(new DecompressionStream('gzip'));
  const out = new Uint8Array(await new Response(inflated).arrayBuffer());
  return { name: gzipName(bytes) || name?.replace(/\.gz$/i, '') || pastedName(out), bytes: out };
}
