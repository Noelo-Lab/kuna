// wire.js — what the pages in a live session say to each other, and the
// check every message from another page passes before this page acts on it:
// a known type, the fields of that type in their expected shapes, and a size
// cap. Invites and replies travel as links whose fragment (never sent to any
// server) is deflated, base64url JSON holding only what a peer needs to
// connect. DOM-free.
import { validSdp } from './sdp.js';
import { CONTROL } from '../session.js';

export const PROTOCOL = 1;
export const MAX_PEERS = 8;
const MAX_FILE = 64 << 20;
export const MAX_MESSAGE = 240 << 10;
export const COLORS = ['#e8404e', '#5fb3e8', '#e6ae5c', '#8cc58e', '#b39ddb', '#f28fb3', '#6cc3c3', '#c8a27a'];
const VIEWS = ['c', 'split', 'asm', 'bytes', 'stack', 'src'];

const PEER = /^[a-z0-9]{8}$/;
const SID = /^[a-z0-9]{12}$/;
const ID = /^[a-z0-9]{10}$/;
const HASH = /^[0-9a-f]{64}$/;
const ADDR = /^0x[0-9a-f]{1,16}$/;

const plain = (v, max) => typeof v === 'string' && v.length <= max && !CONTROL.test(v);
const isName = (v) => plain(v, 40) && v.trim().length > 0;
const isPeer = (v) => typeof v === 'string' && PEER.test(v);
const isSid = (v) => typeof v === 'string' && SID.test(v);
const isAddr = (v) => typeof v === 'string' && ADDR.test(v);
const isColor = (v) => COLORS.includes(v);
const unit = (v) => typeof v === 'number' && Number.isFinite(v) && v >= 0 && v <= 1;
const only = (m, keys) => Object.keys(m).every((k) => keys.includes(k));

/** `c:<line>`, `h:<line>` (a line's heading in the assembly), `a:<insn>`, `b:<byte>` or `s:<stack slot>`. */
export function validAnchor(a) {
  return typeof a === 'string' && /^(?:[ch]:[1-9]\d{0,5}|[ab]:0x[0-9a-f]{1,16}|s:-?\d{1,6})$/.test(a);
}

/** One person in the roster. */
function validMember(m) {
  return !!m && typeof m === 'object' && only(m, ['peer', 'name', 'color']) && isPeer(m.peer) && isName(m.name) && isColor(m.color);
}

/** A program as peers describe it: its name, size and SHA-256. */
function validFile(f) {
  return !!f && typeof f === 'object' && only(f, ['name', 'size', 'hash']) && plain(f.name, 255) && f.name.trim().length > 0 &&
    !/[\\/]/.test(f.name) && Number.isSafeInteger(f.size) && f.size > 0 && f.size <= MAX_FILE && typeof f.hash === 'string' && HASH.test(f.hash);
}

const ops = (list, max) => Array.isArray(list) && list.length <= max;

const CHECKS = {
  hello: (m) => Number.isSafeInteger(m.proto) && isPeer(m.peer) && isName(m.name) && (m.proto !== PROTOCOL || (
    only(m, ['t', 'proto', 'build', 'peer', 'name', 'color', 'sid', 'file']) && typeof m.build === 'string' && HASH.test(m.build) &&
    (m.color === null || isColor(m.color)) && (m.sid === null || isSid(m.sid)) && (m.file === null || validFile(m.file)))),
  welcome: (m) => only(m, ['t', 'sid', 'color', 'roster', 'file', 'example', 'send']) && isSid(m.sid) && isColor(m.color) &&
    Array.isArray(m.roster) && m.roster.length < MAX_PEERS && m.roster.every(validMember) && validFile(m.file) &&
    typeof m.example === 'boolean' && typeof m.send === 'boolean',
  full: (m) => only(m, ['t']),
  bye: (m) => only(m, ['t']),
  snap: (m) => only(m, ['t', 'ops', 'last']) && ops(m.ops, 4000) && typeof m.last === 'boolean',
  ops: (m) => only(m, ['t', 'ops']) && ops(m.ops, 4000) && m.ops.length > 0,
  file: (m) => only(m, ['t', 'name', 'size', 'hash']) && validFile({ name: m.name, size: m.size, hash: m.hash }),
  roster: (m) => only(m, ['t', 'members']) && Array.isArray(m.members) && m.members.length <= MAX_PEERS && m.members.every(validMember),
  where: (m) => only(m, ['t', 'fn', 'view']) && (m.fn === null || isAddr(m.fn)) && VIEWS.includes(m.view),
  ping: (m) => only(m, ['t', 'fn', 'view', 'anchor']) && isAddr(m.fn) && VIEWS.includes(m.view) && validAnchor(m.anchor),
  relay: (m) => only(m, ['t', 'to', 'from', 'kind', 'id', 'd']) && isPeer(m.to) && isPeer(m.from) &&
    (m.kind === 'offer' || m.kind === 'answer') && typeof m.id === 'string' && ID.test(m.id) && validSdp(m.d),
  resync: (m) => only(m, ['t']),
  sum: (m) => only(m, ['t', 'h']) && typeof m.h === 'string' && /^\d{1,7}:[0-9a-f]{16}$/.test(m.h),
  cur: (m) => (only(m, ['t', 'off']) && m.off === true) ||
    (only(m, ['t', 'fn', 'view', 'anchor', 'fx', 'fy', 'col']) && isAddr(m.fn) && VIEWS.includes(m.view) && validAnchor(m.anchor) &&
      unit(m.fx) && unit(m.fy) && (m.col === undefined || (typeof m.col === 'number' && Number.isFinite(m.col) && m.col >= -64 && m.col <= 4096))),
};

/**
 * Whether a message from another page has a known type and a sane shape. The
 * register ops inside `snap` and `ops` are checked one by one when applied
 * (`Replica.receive`), so one bad op does not throw away the rest. A hello
 * from another protocol version is checked only for its sender's id and name,
 * so the page can say whose page is a different version.
 */
export function validMessage(m) {
  if (!m || typeof m !== 'object' || Array.isArray(m) || typeof m.t !== 'string' || !Object.hasOwn(CHECKS, m.t)) return false;
  try {
    return CHECKS[m.t](m) === true;
  } catch (_) {
    return false;
  }
}

/** The largest message of `type` the page reads, in characters of JSON. */
export function maxBytes(type) {
  if (type === 'snap' || type === 'ops') return MAX_MESSAGE;
  if (type === 'cur') return 512;
  if (type === 'welcome') return 16 << 10;
  return 8 << 10;
}

/** Parse one text message from a peer: the message, or null when it is too big, not JSON or not valid. */
export function readMessage(text, channel = 'edits') {
  if (typeof text !== 'string' || text.length > (channel === 'cursor' ? maxBytes('cur') : MAX_MESSAGE)) return null;
  let m;
  try {
    m = JSON.parse(text);
  } catch (_) {
    return null;
  }
  if (!validMessage(m) || text.length > maxBytes(m.t) || (channel === 'cursor') !== (m.t === 'cur')) return null;
  return m;
}

/** A person's name as the others will see it (no control characters or line breaks, at most 40 characters), or ''. */
export function cleanName(v) {
  return String(v ?? '').replace(new RegExp(CONTROL.source, 'g'), ' ').replace(/\s+/g, ' ').trim().slice(0, 40).trim();
}

/**
 * How peers describe a program: `{meta: {name, size, hash}}`, or `{problem}`
 * when it cannot travel (over MAX_FILE). The name loses control characters
 * and path separators and is cut to 255 characters.
 */
export function describeFile(name, size, hash) {
  if (!Number.isSafeInteger(size) || size <= 0) return { problem: 'the program is empty' };
  if (size > MAX_FILE) return { problem: `it is ${(size / (1 << 20)).toFixed(0)} MB, and a program sent to the others can be at most ${MAX_FILE >> 20} MB` };
  const clean = String(name ?? '').replace(new RegExp(CONTROL.source, 'g'), ' ').replace(/[\\/]/g, '_').trim().slice(0, 255).trim();
  const meta = { name: clean || 'program', size, hash };
  return validFile(meta) ? { meta } : { problem: 'the program cannot be described' };
}

/** How many bytes `text` takes as UTF-8 (what a data channel's message limit counts). */
export function utf8Length(text) {
  let n = 0;
  for (let i = 0; i < text.length; i++) {
    const c = text.charCodeAt(i);
    if (c < 0x80) n += 1;
    else if (c < 0x800) n += 2;
    else if (c >= 0xd800 && c <= 0xdbff && i + 1 < text.length) { n += 4; i++; }
    else n += 3;
  }
  return n;
}

// ── invite and reply links ─────────────────────────────────────────────────

const toB64url = (u8) => {
  let s = '';
  for (let i = 0; i < u8.length; i += 0x8000) s += String.fromCharCode(...u8.subarray(i, i + 0x8000));
  return btoa(s).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
};
const fromB64url = (s) => Uint8Array.from(atob(s.replace(/-/g, '+').replace(/_/g, '/')), (c) => c.charCodeAt(0));
const pipe = async (u8, stream) => new Uint8Array(await new Response(new Blob([u8]).stream().pipeThrough(stream)).arrayBuffer());
const CODE = /^[A-Za-z0-9_-]{16,4000}$/;

const SHAPES = {
  invite: (o) => only(o, ['v', 'k', 'id', 'n', 'f', 'z', 'd']) && o.k === 'i' && ID.test(o.id) && isName(o.n) &&
    validFile({ name: o.f, size: o.z, hash: '0'.repeat(64) }) && validSdp(o.d),
  reply: (o) => only(o, ['v', 'k', 'id', 'n', 'd']) && o.k === 'r' && ID.test(o.id) && isName(o.n) && validSdp(o.d),
};

/**
 * An invite (`{id, n: inviter's name, f: file name, z: its size, d: compact
 * offer}`) or a reply (`{id, n: guest's name, d: compact answer}`) as a
 * link-safe code.
 */
export async function encodeCode(kind, fields) {
  const obj = { v: PROTOCOL, k: kind === 'invite' ? 'i' : 'r', ...fields };
  if (!SHAPES[kind](obj)) throw new Error(`not a valid ${kind}`);
  return toB64url(await pipe(new TextEncoder().encode(JSON.stringify(obj)), new CompressionStream('deflate-raw')));
}

/**
 * A code back into its fields: `{ok: true, ...}`, or `{ok: false, why}` with
 * `why` 'version' (another version of Kuna made it) or 'bad'.
 */
export async function decodeCode(code, kind) {
  if (typeof code !== 'string' || !CODE.test(code)) return { ok: false, why: 'bad' };
  try {
    const bytes = await pipe(fromB64url(code), new DecompressionStream('deflate-raw'));
    if (bytes.length > 8000) return { ok: false, why: 'bad' };
    const obj = JSON.parse(new TextDecoder().decode(bytes));
    if (!obj || typeof obj !== 'object' || Array.isArray(obj)) return { ok: false, why: 'bad' };
    if (obj.v !== PROTOCOL) return { ok: false, why: Number.isSafeInteger(obj.v) ? 'version' : 'bad' };
    return SHAPES[kind](obj) ? { ok: true, ...obj } : { ok: false, why: 'bad' };
  } catch (_) {
    return { ok: false, why: 'bad' };
  }
}

/** The code inside a pasted invite or reply link (or a bare code), or null. */
export function codeFrom(text, kind) {
  const t = String(text || '').trim();
  const m = new RegExp(`[#&]${kind === 'invite' ? 'join' : 'reply'}=([A-Za-z0-9_-]+)`).exec(t);
  if (m) return m[1];
  return /^[A-Za-z0-9_-]{16,}$/.test(t) ? t : null;
}

// ── small helpers ──────────────────────────────────────────────────────────

/** A token bucket: `take()` is false once `rate` per second (burst `burst`) is spent. */
export function limiter(rate, burst = rate, now = () => Date.now()) {
  let tokens = burst;
  let last = now();
  return {
    take() {
      const t = now();
      tokens = Math.min(burst, tokens + ((t - last) / 1000) * rate);
      last = t;
      if (tokens < 1) return false;
      tokens -= 1;
      return true;
    },
  };
}

/** Run `fn`, ignoring what it throws (closing what may already be closed). */
export function quietly(fn) {
  try { return fn(); } catch (_) { return undefined; }
}

/** A fresh random id of `n` characters from [a-z0-9]. */
export function randomId(n = 8) {
  const bytes = crypto.getRandomValues(new Uint8Array(n * 2));
  let out = '';
  for (const b of bytes) {
    if (b < 252) out += 'abcdefghijklmnopqrstuvwxyz0123456789'[b % 36];
    if (out.length === n) return out;
  }
  return out.padEnd(n, '0');
}

/** Up to two initials of a name, for the roster. */
export function initials(name) {
  const words = String(name || '').trim().split(/\s+/).filter(Boolean);
  const letters = words.slice(0, 2).map((w) => [...w][0]);
  return (letters.join('') || '?').toUpperCase();
}
