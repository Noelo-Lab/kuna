// persist.js — keep each binary's edit session in localStorage, keyed by a
// hash of the binary's bytes, so reloading the page (or loading the same file
// again) restores the student's renames, retypes and patches. DOM-free: the
// storage object is passed in; every write survives a full or throwing store.

const SESSION_PREFIX = 'kuna.d2.session.';
const INDEX_KEY = 'kuna.d2.index';

/** FNV-1a (32-bit) of `bytes`, as 8 hex digits. */
export function fnv1a32(bytes, seed = 0x811c9dc5) {
  let h = seed >>> 0;
  for (let i = 0; i < bytes.length; i++) {
    h ^= bytes[i];
    h = Math.imul(h, 0x01000193) >>> 0;
  }
  return h.toString(16).padStart(8, '0');
}

/**
 * A stable identity for a binary: `sha256:<hex>` through WebCrypto, or — outside
 * a secure context, where `crypto.subtle` is missing — two FNV-1a passes and
 * the length (`fnv:<hex>-<len>`).
 */
export async function hashBytes(bytes, subtle = globalThis.crypto?.subtle) {
  if (subtle?.digest) {
    try {
      const digest = new Uint8Array(await subtle.digest('SHA-256', bytes));
      return 'sha256:' + [...digest].map((b) => b.toString(16).padStart(2, '0')).join('');
    } catch (_) { /* fall through */ }
  }
  return `fnv:${fnv1a32(bytes)}${fnv1a32(bytes, 0x050c5d1f)}-${bytes.length}`;
}

/** A bounded LRU of sessions (`max` binaries) in `storage`. */
export class SessionStore {
  constructor(storage, { max = 20 } = {}) {
    this.storage = storage;
    this.max = max;
  }

  index() {
    try {
      const list = JSON.parse(this.storage?.getItem(INDEX_KEY) || '[]');
      return Array.isArray(list) ? list.filter((e) => e && typeof e.hash === 'string') : [];
    } catch (_) {
      return [];
    }
  }

  #writeIndex(list) {
    this.storage.setItem(INDEX_KEY, JSON.stringify(list));
  }

  load(hash) {
    try {
      return this.storage?.getItem(SESSION_PREFIX + hash) ?? null;
    } catch (_) {
      return null;
    }
  }

  /** Save `text` for `hash`; evicts the oldest sessions to make room. */
  save(hash, name, text) {
    if (!this.storage) return false;
    let list = this.index().filter((e) => e.hash !== hash);
    list.unshift({ hash, name, saved: Date.now() });
    while (list.length > this.max) this.#drop(list.pop().hash);
    for (;;) {
      try {
        this.storage.setItem(SESSION_PREFIX + hash, text);
        this.#writeIndex(list);
        return true;
      } catch (_) {
        if (list.length <= 1) {
          try { this.storage.removeItem(SESSION_PREFIX + hash); } catch (__) { /* nothing */ }
          return false;
        }
        this.#drop(list.pop().hash);
      }
    }
  }

  remove(hash) {
    if (!this.storage) return;
    this.#drop(hash);
    try { this.#writeIndex(this.index().filter((e) => e.hash !== hash)); } catch (_) { /* full */ }
  }

  #drop(hash) {
    try { this.storage.removeItem(SESSION_PREFIX + hash); } catch (_) { /* gone */ }
  }
}
