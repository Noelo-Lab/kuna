// decompile-strings.mjs — the sidebar's Strings list, from the source tree
// with no build: which group a string lands in, how its text and its users
// are shown, that every engine string is escaped, and how a search narrows
// the list and opens the groups it matches in.
import assert from 'node:assert/strict';
import { compileQuery } from '../assets/js/fnfilter.js';
import {
  stringGroup, showText, stringKey, usersOf, renderStringRow, renderStringList, ROW_CAP,
} from '../decompile/strings-view.js';

const checks = [];
const use = (name, address_hex, at_hex, via = null) => ({ name, address_hex, at_hex, kind: via ? 'read' : 'data', instruction: '', via });
const str = (text, address_hex, uses = [], extra = {}) => ({ text, address_hex, length: text.length, encoding: 'ascii', section: '.rodata', in_code: false, uses, ...extra });

const flag = str('flag{str1ngs_4re_3asy}', '0x2004', [
  use('check', '0x1209', '0x1229', { name: 'secret', address_hex: '0x4010' }),
  use('check', '0x1209', '0x1253', { name: 'secret', address_hex: '0x4010' }),
]);
const nope = str('Nope, that is not the flag.', '0x201b', [use('check', '0x1209', '0x123d'), use('main', '0x1272', '0x1306')]);
const junk = str('AWAVAUATUSH', '0x1100', [], { section: '.text', in_code: true });
const interp = str('/lib64/ld-linux-x86-64.so.2', '0x318', [], { section: '.interp' });
const doc = [interp, junk, flag, nope];

// ── groups and text ────────────────────────────────────────────────────────
assert.equal(stringGroup(flag), 'used');
assert.equal(stringGroup(interp), 'unused');
assert.equal(stringGroup(junk), 'code', 'unused text inside code is set apart');
assert.equal(stringGroup({ ...junk, uses: [use('f', '0x1', '0x2')] }), 'used', 'a use wins over where it lives');
assert.equal(showText('a\nb\tc\\d\x01'), 'a\\nb\\tc\\\\d\\x01', 'control characters read as C escapes on one line');
assert.ok(stringKey(str('line\none', '0x10')).includes('line\\none'), 'a search for the escape finds the text');
assert.ok(stringKey(flag).includes('0x2004'), 'and the address is searchable');
checks.push('groups + one-line text');

// ── who uses it ────────────────────────────────────────────────────────────
assert.deepEqual(usersOf(flag), [{ address_hex: '0x1209', name: 'check', at_hex: '0x1229', sites: ['0x1229', '0x1253'], via: 'secret' }],
  'one entry per function, its uses in order, and the pointer it reads');
assert.deepEqual(usersOf(nope).map((u) => u.name), ['check', 'main']);
assert.deepEqual(usersOf(str('both', '0x30', [use('f', '0x10', '0x14'), use('f', '0x10', '0x14', { name: 'p', address_hex: '0x40' })]))[0].sites,
  ['0x14'], 'an instruction that reaches it both ways is one place');
assert.deepEqual(usersOf(str('x', '0x1', [use(null, '0x5000', '0x5000', { name: null, address_hex: '0x4018' })])),
  [{ address_hex: '0x5000', name: null, at_hex: '0x5000', sites: ['0x5000'], via: '0x4018' }], 'an unnamed pointer by address');

const row = renderStringRow(flag, { nameOf: (a, n) => (a === '0x1209' ? 'check_flag' : n) });
assert.match(row, /<button class="sx"[^>]*>flag\{str1ngs_4re_3asy\}<\/button>/);
assert.match(row, /data-fn="0x1209" data-sites="0x1229 0x1253"/, 'the link carries the function and every use');
assert.match(row, />check_flag<\/a>/, 'the student\'s name for the function');
assert.match(row, /×2/);
assert.match(row, /through the pointer secret/);
assert.match(row, /click again for the next/);
const unused = renderStringRow(interp);
assert.ok(!unused.includes('<a '), 'an unused string has no links');
assert.match(unused, /0x318 · \.interp/, 'it says where it is instead');
assert.match(renderStringRow(str('wide', '0x10', [], { encoding: 'utf16' })), /wide text \(UTF-16\)/);
assert.match(renderStringRow(flag, { selected: true }), /class="str sel"/);
checks.push('users + row');

// ── escaping ───────────────────────────────────────────────────────────────
const evil = str('<img src=x onerror=alert(1)>"\'&', '0x3000', [use('<b>f</b>', '0x1"', '0x2"', { name: '<i>p</i>', address_hex: '0x4' })]);
const html = renderStringRow(evil);
assert.ok(!/<img|<b>|<i>/.test(html), 'no engine text becomes markup');
assert.ok(!html.includes('0x1"') && !html.includes('0x2"'), 'attributes are escaped too');
checks.push('escaping');

// ── the list ───────────────────────────────────────────────────────────────
const all = renderStringList(doc, { query: compileQuery('') });
assert.equal(all.matches, 4);
assert.match(all.html, /<details class="d2-group" data-group="used" open><summary>Used by the code <span class="d2-count">\(2\)<\/span>/);
assert.match(all.html, /data-group="unused"><summary>Not used directly <span class="d2-count">\(1\)/, 'closed unless opened');
assert.match(all.html, /data-group="code"><summary>Inside machine code/);
assert.ok(all.html.indexOf('data-group="used"') < all.html.indexOf('data-group="unused"'), 'used strings first');
const kept = renderStringList(doc, { query: compileQuery(''), open: { used: false, code: true } });
assert.match(kept.html, /data-group="used">/, 'the student\'s closed group stays closed');
assert.match(kept.html, /data-group="code" open>/);

const found = renderStringList(doc, { query: compileQuery('FLAG'), open: { used: false } });
assert.equal(found.matches, 2, 'a search is case-insensitive');
assert.match(found.html, /data-group="used" open>.*\(2 of 2\)/, 'a group with matches opens during a search');
assert.ok(!found.html.includes('data-group="unused"') && !found.html.includes('data-group="code"'), 'groups without matches are left out');
assert.equal(renderStringList(doc, { query: compileQuery('/^nope/') }).matches, 1, 'a /regex/ works as in the function search');
assert.equal(renderStringList(doc, { query: compileQuery('password') }).matches, 0);
assert.equal(renderStringList([], { query: compileQuery('') }).html, '');

const many = Array.from({ length: ROW_CAP + 5 }, (_, i) => str(`message number ${i}`, `0x${(0x1000 + i).toString(16)}`, [use('main', '0x10', '0x20')]));
const capped = renderStringList(many, { query: compileQuery('') });
assert.equal((capped.html.match(/class="str"/g) || []).length, ROW_CAP, 'a long list draws a page of rows');
assert.match(capped.html, new RegExp(`Showing ${ROW_CAP} of ${ROW_CAP + 5}\\. Search to find the others\\.`));
assert.equal(capped.matches, ROW_CAP + 5);
assert.ok(!renderStringList(many, { query: compileQuery('number 7') }).html.includes('Showing'), 'a search under the cap draws them all');
checks.push('list: groups, search, cap');

console.log(`DECOMPILE STRINGS OK — ${checks.join(' · ')}`);
