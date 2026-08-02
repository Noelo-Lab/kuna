// fnfilter.mjs — the /decompile sidebar filter's query semantics and counts.
//
// The filter is what makes a whole-binary inventory usable (a 1.1 MiB PE
// indexes 3,158 entries), so its matching rules are pinned here. This is the
// pure half — `assets/js/fnfilter.js` has no DOM dependency precisely so it can
// run under plain Node; the page's row-hiding/focus wiring needs a browser and
// is covered by the optional Chrome smoke test (README → Optional).
//
//   node integrations/web/test/fnfilter.mjs

import assert from 'node:assert/strict';
import {
  compileQuery,
  headline,
  searchKey,
  stubDividerLabel,
  summarize,
} from '../assets/js/fnfilter.js';

const ok = (msg) => console.log(`\x1b[32mOK\x1b[0m   ${msg}`);

const INVENTORY = [
  { name: 'main', address_hex: '0x401000', aliases: [], kind: 'func' },
  { name: 'sub_4e6800', address_hex: '0x4e6800', aliases: ['_dws_ErrNo@0'], kind: 'func' },
  { name: 'ParseHeader', address_hex: '0x4047a5', aliases: [], kind: 'func' },
  { name: 'GlobalFree', address_hex: '0x4e633a', aliases: [], kind: 'plt' },
  { name: 'GlobalLock', address_hex: '0x4e634c', aliases: [], kind: 'plt' },
  { name: 'j_memcpy', address_hex: '0x4e6400', aliases: [], kind: 'thunk' },
];
const ROWS = INVENTORY.map((fn) => ({
  key: searchKey(fn),
  stub: fn.kind === 'plt' || fn.kind === 'thunk',
}));
const named = (summary) =>
  INVENTORY.filter((_, i) => summary.matches[i]).map((fn) => fn.name);

// --- the haystack ----------------------------------------------------------
assert.equal(searchKey(INVENTORY[1]), 'sub_4e6800 _dws_ErrNo@0 0x4e6800');
assert.equal(searchKey({ name: 'x', address_hex: '0x1' }), 'x 0x1');
ok('searchKey covers name, aliases, and address');

// --- plain terms -----------------------------------------------------------
assert.deepEqual(named(summarize(ROWS, '')), INVENTORY.map((fn) => fn.name));
assert.deepEqual(named(summarize(ROWS, '   ')), INVENTORY.map((fn) => fn.name));
ok('an empty query keeps every row');

assert.deepEqual(named(summarize(ROWS, 'global')), ['GlobalFree', 'GlobalLock']);
assert.deepEqual(named(summarize(ROWS, 'GLOBAL')), ['GlobalFree', 'GlobalLock']);
assert.deepEqual(named(summarize(ROWS, 'parse')), ['ParseHeader']);
ok('name matching is case-insensitive substring');

assert.deepEqual(named(summarize(ROWS, '_dws_errno')), ['sub_4e6800']);
ok('an alias matches the row that carries it');

assert.deepEqual(named(summarize(ROWS, '0x4047a5')), ['ParseHeader']);
assert.deepEqual(named(summarize(ROWS, '4047a5')), ['ParseHeader']);
assert.deepEqual(named(summarize(ROWS, '4E6')), ['sub_4e6800', 'GlobalFree', 'GlobalLock', 'j_memcpy']);
ok('addresses match with or without 0x, either case');

assert.deepEqual(named(summarize(ROWS, 'global 0x4e634c')), ['GlobalLock']);
assert.deepEqual(named(summarize(ROWS, 'global parse')), []);
ok('multiple terms are AND, not OR');

// --- /regex/ ---------------------------------------------------------------
assert.deepEqual(named(summarize(ROWS, '/^Global/')), ['GlobalFree', 'GlobalLock']);
assert.deepEqual(named(summarize(ROWS, '/^(main|j_)/')), ['main', 'j_memcpy']);
assert.deepEqual(named(summarize(ROWS, '/free$/')), []);            // key ends with the address
assert.deepEqual(named(summarize(ROWS, '/globalfree/')), ['GlobalFree']);
assert.deepEqual(named(summarize(ROWS, '/globalfree/m')), []);      // explicit flags: no implicit `i`
ok('/regex/ matches, case-insensitive unless it names flags');

const bad = summarize(ROWS, '/foo(/');
assert.equal(bad.matches.some(Boolean), false);
assert.match(bad.error, /group|regular expression|Invalid/i);
assert.equal(summarize(ROWS, '/ok/').error, null);
ok('an unparseable regex reports its error and matches nothing');

// --- counts ----------------------------------------------------------------
const idle = summarize(ROWS, '');
assert.equal(idle.filtered, false);
assert.deepEqual(idle.normal, { matched: 3, total: 3 });
assert.deepEqual(idle.stubs, { matched: 3, total: 3 });
assert.equal(headline(idle), '3 functions · 3 stubs');
assert.equal(stubDividerLabel(idle), 'imports & thunks (3)');

const filtered = summarize(ROWS, 'global');
assert.equal(filtered.filtered, true);
assert.deepEqual(filtered.normal, { matched: 0, total: 3 });
assert.deepEqual(filtered.stubs, { matched: 2, total: 3 });
assert.equal(headline(filtered), '0 of 3 functions · 2 of 3 stubs');
assert.equal(stubDividerLabel(filtered), 'imports & thunks (2 of 3)');
ok('counts and labels split normal rows from stubs');

const big = summarize(
  Array.from({ length: 3158 }, (_, i) => ({ key: `sub_${i.toString(16)} 0x${i.toString(16)}`, stub: i >= 1952 })),
  'sub_4e6',
);
assert.equal(headline(big), '1 of 1,952 functions · 0 of 1,206 stubs');
ok('a whole-binary inventory formats its counts with separators');

// --- speed -----------------------------------------------------------------
// Every keystroke re-tests every row; the real list is thousands long.
const wide = Array.from({ length: 5000 }, (_, i) => ({
  key: `some_function_name_${i} 0x${(0x400000 + i * 16).toString(16)}`,
  stub: i % 3 === 0,
}));
const t0 = performance.now();
for (const q of ['s', 'so', 'som', 'some_f', '/^some_function_name_4[0-9]{2} /']) summarize(wide, q);
const dt = performance.now() - t0;
assert.ok(dt < 500, `5 keystrokes over 5,000 rows took ${dt.toFixed(1)} ms`);
ok(`5 keystrokes over 5,000 rows: ${dt.toFixed(1)} ms`);

console.log('\n\x1b[32mFILTER OK\x1b[0m — name/alias/address terms, /regex/, counts, and error reporting.');
