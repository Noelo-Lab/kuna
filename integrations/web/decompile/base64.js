// base64.js — a program pasted as base64 text. DOM-free, so
// test/decompile-base64.mjs runs it under plain Node.

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
