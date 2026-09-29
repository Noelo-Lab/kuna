// fnfilter.mjs — the /decompile function search's query semantics.
//
// The search is what makes a whole-binary inventory usable (a 1.1 MiB PE
// indexes 3,158 entries), so its matching rules are pinned here. This is the
// pure half — `assets/js/fnfilter.js` has no DOM dependency precisely so it can
// run under plain Node; the page's row-hiding is covered by the browser suite.
//
//   node integrations/web/test/fnfilter.mjs

import assert from 'node:assert/strict';
import { compileQuery, searchKey } from '../assets/js/fnfilter.js';

const ok = (msg) => console.log(`\x1b[32mOK\x1b[0m   ${msg}`);

const INVENTORY = [
  { name: 'main', address_hex: '0x401000', aliases: [], kind: 'func' },
  { name: 'sub_4e6800', address_hex: '0x4e6800', aliases: ['_dws_ErrNo@0'], kind: 'func' },
  { name: 'ParseHeader', address_hex: '0x4047a5', aliases: [], kind: 'func' },
  { name: 'GlobalFree', address_hex: '0x4e633a', aliases: [], kind: 'plt' },
  { name: 'GlobalLock', address_hex: '0x4e634c', aliases: [], kind: 'plt' },
  { name: 'j_memcpy', address_hex: '0x4e6400', aliases: [], kind: 'thunk' },
];
const KEYS = INVENTORY.map(searchKey);
/** Which inventory rows the query `text` keeps, and its error. */
const match = (text) => {
  const query = compileQuery(text);
  return { matches: KEYS.map((key) => query.test(key)), error: query.error };
};
const named = (summary) =>
  INVENTORY.filter((_, i) => summary.matches[i]).map((fn) => fn.name);

// --- the haystack ----------------------------------------------------------
assert.equal(searchKey(INVENTORY[1]), 'sub_4e6800 _dws_ErrNo@0 0x4e6800');
assert.equal(searchKey({ name: 'x', address_hex: '0x1' }), 'x 0x1');
ok('searchKey covers name, aliases, and address');

// --- plain terms -----------------------------------------------------------
assert.deepEqual(named(match('')), INVENTORY.map((fn) => fn.name));
assert.deepEqual(named(match('   ')), INVENTORY.map((fn) => fn.name));
ok('an empty query keeps every row');

assert.deepEqual(named(match('global')), ['GlobalFree', 'GlobalLock']);
assert.deepEqual(named(match('GLOBAL')), ['GlobalFree', 'GlobalLock']);
assert.deepEqual(named(match('parse')), ['ParseHeader']);
ok('name matching is case-insensitive substring');

assert.deepEqual(named(match('_dws_errno')), ['sub_4e6800']);
ok('an alias matches the row that carries it');

assert.deepEqual(named(match('0x4047a5')), ['ParseHeader']);
assert.deepEqual(named(match('4047a5')), ['ParseHeader']);
assert.deepEqual(named(match('4E6')), ['sub_4e6800', 'GlobalFree', 'GlobalLock', 'j_memcpy']);
ok('addresses match with or without 0x, either case');

assert.deepEqual(named(match('global 0x4e634c')), ['GlobalLock']);
assert.deepEqual(named(match('global parse')), []);
ok('multiple terms are AND, not OR');

// --- /regex/ ---------------------------------------------------------------
assert.deepEqual(named(match('/^Global/')), ['GlobalFree', 'GlobalLock']);
assert.deepEqual(named(match('/^(main|j_)/')), ['main', 'j_memcpy']);
assert.deepEqual(named(match('/free$/')), []);            // key ends with the address
assert.deepEqual(named(match('/globalfree/')), ['GlobalFree']);
assert.deepEqual(named(match('/globalfree/m')), []);      // explicit flags: no implicit `i`
ok('/regex/ matches, case-insensitive unless it names flags');

const bad = match('/foo(/');
assert.equal(bad.matches.some(Boolean), false);
assert.match(bad.error, /group|regular expression|Invalid/i);
assert.equal(match('/ok/').error, null);
ok('an unparseable regex reports its error and matches nothing');

// --- speed -----------------------------------------------------------------
// Every keystroke re-tests every row; the real list is thousands long.
const wide = Array.from({ length: 5000 }, (_, i) => `some_function_name_${i} 0x${(0x400000 + i * 16).toString(16)}`);
const t0 = performance.now();
for (const q of ['s', 'so', 'som', 'some_f', '/^some_function_name_4[0-9]{2} /']) {
  const query = compileQuery(q);
  wide.filter((key) => query.test(key));
}
const dt = performance.now() - t0;
assert.ok(dt < 500, `5 keystrokes over 5,000 rows took ${dt.toFixed(1)} ms`);
ok(`5 keystrokes over 5,000 rows: ${dt.toFixed(1)} ms`);

console.log('\n\x1b[32mFILTER OK\x1b[0m — name/alias/address terms, /regex/, and error reporting.');
