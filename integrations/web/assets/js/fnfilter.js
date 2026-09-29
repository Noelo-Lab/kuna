// fnfilter.js — the /decompile function list's search.
//
// Pure string logic, no DOM: the page owns the rows and what they look like,
// this owns what matches, so `test/fnfilter.mjs` can pin the query semantics
// under Node.
//
// The query is either a `/regex/flags` literal or whitespace-separated terms,
// ALL of which must appear somewhere in a row's name, aliases, or address.
// Plain terms are case-insensitive; a regex is too unless it names its own
// flags. A stripped binary indexes as `sub_<hex>` rows whose only handle is the
// address, so the address is part of the haystack, not a separate field.

/** Flags a filter regex may carry (`g`/`y` are stateful — `test` would skip). */
const FLAGS = 'imsu';

/** The haystack one inventory entry is matched against. */
export function searchKey(fn) {
  const aliases = Array.isArray(fn.aliases) ? fn.aliases : [];
  return [fn.name, ...aliases, fn.address_hex].filter(Boolean).join(' ');
}

/**
 * Compile query text into `{empty, error, test(key)}`. An unparseable regex
 * matches nothing and reports `error` — the page shows it instead of silently
 * filtering everything away.
 */
export function compileQuery(text) {
  const query = (text || '').trim();
  if (!query) return { empty: true, error: null, test: () => true };

  const literal = query.match(/^\/(.+)\/([a-zA-Z]*)$/);
  if (literal) {
    const asked = [...literal[2]].filter((f) => FLAGS.includes(f)).join('');
    try {
      const re = new RegExp(literal[1], literal[2] ? asked : 'i');
      return { empty: false, error: null, test: (key) => re.test(key) };
    } catch (e) {
      return { empty: false, error: e.message, test: () => false };
    }
  }

  const terms = query.toLowerCase().split(/\s+/);
  return {
    empty: false,
    error: null,
    test: (key) => {
      const hay = key.toLowerCase();
      return terms.every((term) => hay.includes(term));
    },
  };
}
