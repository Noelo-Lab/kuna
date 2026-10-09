// strings-view.js — the sidebar's Strings list: the program's text in three
// groups (what the code uses, what it does not use directly, and text that is
// really machine code), each used string naming the functions that use it.
// A function's link goes to the instruction that uses the string, and to its
// next use there on another click; a use made by reading a pointer to the
// string names that pointer. DOM-free: app.js
// mounts the HTML and owns the clicks.
import { escapeHtml } from '../assets/js/highlight-c.js';
import { bare } from './addr.js';

export const STRING_GROUPS = [
  ['used', 'Used by the code', true],
  ['unused', 'Not used directly', false],
  ['code', 'Inside machine code', false],
];

/** At most this many rows of one group are drawn; a search reaches the rest. */
export const ROW_CAP = 300;

export function stringGroup(s) {
  if (s.uses?.length) return 'used';
  return s.in_code ? 'code' : 'unused';
}

const ESC = { '\n': '\\n', '\t': '\\t', '\r': '\\r', '\\': '\\\\' };

/** The text on one line: control characters and backslashes as C escapes. */
export function showText(text) {
  return String(text).replace(/[\x00-\x1f\x7f\\]/g,
    (c) => ESC[c] || `\\x${c.charCodeAt(0).toString(16).padStart(2, '0')}`);
}

/** What a search matches: the text as shown and as stored, and the address. */
export function stringKey(s) {
  const shown = showText(s.text);
  return shown === s.text ? `${s.text} ${s.address_hex}` : `${s.text} ${shown} ${s.address_hex}`;
}

const keys = new WeakMap();

function keyOf(s) {
  let key = keys.get(s);
  if (key === undefined) keys.set(s, (key = stringKey(s)));
  return key;
}

/**
 * The functions that use `s`, each once, in the order of their first use:
 * `[{address_hex, name, at_hex, sites, via}]`. `address_hex` is the function's
 * entry (the instruction itself when no function holds it), `at_hex` its first
 * use, and `via` the pointer that use reads, by name or address.
 */
export function usersOf(s) {
  const seen = new Map();
  for (const u of s.uses || []) {
    const at = String(u.at_hex).toLowerCase();
    const key = String(u.address_hex || at).toLowerCase();
    const prev = seen.get(key);
    if (prev) {
      if (!prev.sites.includes(at)) prev.sites.push(at);
      continue;
    }
    seen.set(key, {
      address_hex: key,
      name: u.name || null,
      at_hex: at,
      sites: [at],
      via: u.via ? u.via.name || u.via.address_hex : null,
    });
  }
  return [...seen.values()];
}

/** One string: its text (opens its first use, or its bytes), then who uses it. */
export function renderStringRow(s, { nameOf = (a, n) => n || a, selected = false } = {}) {
  const where = [s.address_hex, s.section, s.encoding === 'utf16' ? 'wide text (UTF-16)' : null].filter(Boolean).join(' · ');
  const users = usersOf(s);
  const sub = users.length
    ? 'Used in ' + users.map((u) => {
      const times = u.sites.length > 1 ? ` <span class="d2muted">×${u.sites.length}</span>` : '';
      const via = u.via ? ` <span class="d2muted">through the pointer ${escapeHtml(u.via)}</span>` : '';
      const title = u.sites.length > 1 ? `Go to where it is used (${u.sites.length} places: click again for the next)` : 'Go to where it is used';
      return `<a class="xt" href="#${escapeHtml(u.at_hex)}" data-fn="${escapeHtml(u.address_hex)}" data-sites="${escapeHtml(u.sites.join(' '))}" ` +
        `title="${escapeHtml(title)}">${escapeHtml(nameOf(u.address_hex, u.name) || u.at_hex)}</a>${times}${via}`;
    }).join(', ')
    : `<span class="d2muted">${escapeHtml(where)}</span>`;
  const goes = users.length ? `Go to where ${nameOf(users[0].address_hex, users[0].name) || users[0].at_hex} uses it` : 'Show its bytes';
  const list = users.length ? ' <button class="d2-link sr" data-act="str-refs" title="Every place that uses it (x)">all uses</button>' : '';
  return `<div class="str${selected ? ' sel' : ''}" data-addr="${escapeHtml(s.address_hex)}">` +
    `<button class="sx" title="${escapeHtml(`${s.text}\n${where}\n${goes}\nPress x for every place that uses it`)}">${escapeHtml(showText(s.text))}</button>` +
    `<div class="su">${sub}${list}</div></div>`;
}

/**
 * The cross-references dialog's model for string `s`: every instruction that
 * uses it, with the pointer read when the use goes through one.
 */
export function stringRefsModel(s, { nameOf = (a, n) => n || a } = {}) {
  const rows = (s.uses || []).map((u) => {
    const at = String(u.at_hex).toLowerCase();
    const fn = String(u.address_hex || at).toLowerCase();
    const via = u.via ? u.via.name || u.via.address_hex : null;
    return {
      fn, site: at, siteLabel: bare(at), name: nameOf(fn, u.name) || at, kind: u.kind,
      how: via ? `reads the pointer ${via}` : '',
      instruction: u.instruction || '',
    };
  });
  const text = showText(s.text);
  const where = [bare(s.address_hex), s.section].filter(Boolean).join(' in ');
  return {
    title: `Uses of "${text.length > 60 ? text.slice(0, 57) + '…' : text}"`,
    sub: `text at ${where}`,
    sections: [{ heading: 'Used by', kind: 'sites', rows, empty: 'Nothing in the program uses it directly.' }],
  };
}

/**
 * The whole list for `strings` (the engine's rows) under a compiled `query`
 * ({empty, test}): each group with its count and at most `cap` rows. `open`
 * gives each group's state while nothing is searched; during a search the
 * groups with matches open. Returns `{html, matches}`.
 */
export function renderStringList(strings, { query, open = {}, nameOf, selected = null, cap = ROW_CAP } = {}) {
  const groups = Object.fromEntries(STRING_GROUPS.map(([id]) => [id, { total: 0, hits: [] }]));
  for (const s of strings) {
    const g = groups[stringGroup(s)];
    g.total++;
    if (query.test(keyOf(s))) g.hits.push(s);
  }
  let matches = 0;
  const html = STRING_GROUPS.map(([id, title, byDefault]) => {
    const g = groups[id];
    matches += g.hits.length;
    if (!g.total || (!query.empty && !g.hits.length)) return '';
    const isOpen = query.empty ? (open[id] ?? byDefault) : true;
    const count = query.empty ? `(${g.total})` : `(${g.hits.length} of ${g.total})`;
    const rows = g.hits.slice(0, cap).map((s) => renderStringRow(s, { nameOf, selected: s.address_hex === selected })).join('');
    const more = g.hits.length > cap
      ? `<p class="d2-strmore">Showing ${cap.toLocaleString('en-US')} of ${g.hits.length.toLocaleString('en-US')}. Search to find the others.</p>`
      : '';
    return `<details class="d2-group" data-group="${id}"${isOpen ? ' open' : ''}>` +
      `<summary>${escapeHtml(title)} <span class="d2-count">${count}</span></summary>${rows}${more}</details>`;
  }).join('');
  return { html, matches };
}
