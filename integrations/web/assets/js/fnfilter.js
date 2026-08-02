// fnfilter.js — the /decompile sidebar's function filter.
//
// Pure string and count logic, no DOM: the page owns the rows and what they
// look like, this owns what matches and what the header says. Split out of
// decompile/index.html so `test/fnfilter.mjs` can pin the query semantics under
// Node — the DOM half needs a real browser, which the committed tests do not
// have (integrations/web/README.md → Tests).
//
// The query is either a `/regex/flags` literal or whitespace-separated terms,
// ALL of which must appear somewhere in a row's name, aliases, or address.
// Plain terms are case-insensitive; a regex is too unless it names its own
// flags. A stripped binary indexes as `sub_<hex>` rows whose only handle is the
// address, so the address is part of the haystack, not a separate field.

/** Flags a filter regex may carry (`g`/`y` are stateful — `test` would skip). */
const FLAGS = 'imsu';

/** Format a count the way the sidebar header does (locale-pinned for tests). */
const fmt = (n) => n.toLocaleString('en-US');

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

/**
 * Match `rows` (`[{key, stub}]`, sidebar order) against the query text.
 * Returns the parallel `matches` array plus the counts the header renders.
 */
export function summarize(rows, text) {
  const query = compileQuery(text);
  const matches = rows.map((row) => query.test(row.key));
  const count = (stub) =>
    rows.reduce(
      (acc, row, i) => {
        if (!!row.stub !== stub) return acc;
        return { matched: acc.matched + (matches[i] ? 1 : 0), total: acc.total + 1 };
      },
      { matched: 0, total: 0 },
    );
  return {
    matches,
    error: query.error,
    filtered: !query.empty,
    normal: count(false),
    stubs: count(true),
  };
}

/** The sidebar header: totals when idle, `matched of total` when filtering. */
export function headline(summary) {
  const part = (count, noun) =>
    summary.filtered
      ? `${fmt(count.matched)} of ${fmt(count.total)} ${noun}`
      : `${fmt(count.total)} ${noun}`;
  return `${part(summary.normal, 'functions')} · ${part(summary.stubs, 'stubs')}`;
}

/** The `imports & thunks (…)` divider label, counted the same way. */
export function stubDividerLabel(summary) {
  const { matched, total } = summary.stubs;
  return summary.filtered
    ? `imports & thunks (${fmt(matched)} of ${fmt(total)})`
    : `imports & thunks (${fmt(total)})`;
}
