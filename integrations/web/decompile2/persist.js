// persist.js — keep each binary's edit session in localStorage, keyed by a
// hash of the binary's bytes, so reloading the page (or loading the same file
// again) restores the student's renames, retypes and patches. A live session
// joined from someone else is kept apart (`kuna.d2.shared.`), so it never
// overwrites the student's own. DOM-free: the storage object is passed in;
// every write survives a full or throwing store.
import { sha256Hex } from '../sha256.js';

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

/** A stable identity for a binary: `sha256:<hex>` (WebCrypto, or ../sha256.js outside a secure context). */
export async function hashBytes(bytes, subtle = globalThis.crypto?.subtle) {
  return 'sha256:' + await sha256Hex(bytes, subtle);
}

/** The key earlier versions used outside a secure context (two FNV-1a passes and the length), to find what they stored. */
export function legacyKey(bytes) {
  return `fnv:${fnv1a32(bytes)}${fnv1a32(bytes, 0x050c5d1f)}-${bytes.length}`;
}

/**
 * A bounded LRU of sessions (`max` binaries) in `storage`, under `prefix`
 * (the index under `indexKey`). When the storage is full, `spare` (another
 * store, holding copies worth less) gives up its entries first, then this
 * store its own least recently used.
 */
export class SessionStore {
  constructor(storage, { max = 20, prefix = SESSION_PREFIX, indexKey = INDEX_KEY, spare = null } = {}) {
    this.storage = storage;
    this.max = max;
    this.prefix = prefix;
    this.indexKey = indexKey;
    this.spare = spare;
  }

  index() {
    try {
      const list = JSON.parse(this.storage?.getItem(this.indexKey) || '[]');
      return Array.isArray(list) ? list.filter((e) => e && typeof e.hash === 'string') : [];
    } catch (_) {
      return [];
    }
  }

  #writeIndex(list) {
    this.storage.setItem(this.indexKey, JSON.stringify(list));
  }

  load(hash) {
    try {
      return this.storage?.getItem(this.prefix + hash) ?? null;
    } catch (_) {
      return null;
    }
  }

  /**
   * What is saved for `hash`, moving what an earlier version saved for the
   * same `bytes` under its FNV-1a key. That key (two passes over the whole
   * program) is worked out only when such an entry exists.
   */
  loadMoving(hash, bytes, name) {
    const text = this.load(hash);
    if (text || !bytes || !this.#hasLegacy()) return text;
    const old = legacyKey(bytes);
    const found = this.load(old);
    if (found && this.save(hash, name, found)) this.remove(old);
    return found;
  }

  #hasLegacy() {
    const legacy = `${this.prefix}fnv:`;
    try {
      if (typeof this.storage.key === 'function' && Number.isInteger(this.storage.length)) {
        for (let i = 0; i < this.storage.length; i++) if (this.storage.key(i)?.startsWith(legacy)) return true;
        return false;
      }
    } catch (_) { /* fall back to the index */ }
    return this.index().some((e) => e.hash.startsWith('fnv:'));
  }

  /** Drop the least recently used entry; false when there is none. */
  evictOne() {
    const list = this.index();
    const last = list.pop();
    if (!last) return false;
    this.#drop(last.hash);
    try { this.#writeIndex(list); } catch (_) { /* full */ }
    return true;
  }

  /** Save `text` for `hash`; makes room from `spare`, then from this store's oldest sessions. */
  save(hash, name, text) {
    if (!this.storage) return false;
    let list = this.index().filter((e) => e.hash !== hash);
    list.unshift({ hash, name, saved: Date.now() });
    while (list.length > this.max) this.#drop(list.pop().hash);
    for (;;) {
      try {
        this.storage.setItem(this.prefix + hash, text);
        this.#writeIndex(list);
        return true;
      } catch (_) {
        if (this.spare?.evictOne()) continue;
        if (list.length <= 1) {
          try { this.storage.removeItem(this.prefix + hash); } catch (__) { /* nothing */ }
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
    try { this.storage.removeItem(this.prefix + hash); } catch (_) { /* gone */ }
  }
}
