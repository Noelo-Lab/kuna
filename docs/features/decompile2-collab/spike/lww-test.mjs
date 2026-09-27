// lww-test.mjs — convergence of the replicated session under the delivery a
// peer-to-peer mesh actually gives (every op reaches every peer eventually, in
// any order, some twice, some only through a late-join snapshot), and the
// check that keeps another peer's ops from making the engine read a file.
//   node docs/features/decompile2-collab/spike/lww-test.mjs
import assert from 'node:assert/strict';
import { Replica, validOp } from './lww.mjs';

let seed = 0x2545f491;
const rand = (n) => { seed ^= seed << 13; seed ^= seed >>> 17; seed ^= seed << 5; return (seed >>> 0) % n; };
const pick = (xs) => xs[rand(xs.length)];

const KEYS = [
  'var:0x1198:v1:name', 'var:0x1198:v1:type', 'var:0x1161:a0:name',
  'fn:0x1149', 'fn:0x1161', 'comment:0x1198:0x11b5', 'byte:0x11af', 'byte:0x11b0', 'setting:mode',
];
const VALUES = {
  name: ['total', 'count', 'sum', null], type: ['unsigned long', 'long', null],
  fn: ['adder', 'add_two', null], comment: ['calls add', 'hot path', null], byte: ['05', '90', null],
  setting: ['auto', 'fast', 'aggressive'],
};
const valueFor = (key) => pick(VALUES[key.startsWith('var:') ? key.split(':')[3] : key.split(':')[0]]);

function round(peers, ops, lateJoiner) {
  const reps = peers.map((p) => new Replica(p));
  const log = [];
  for (let i = 0; i < ops; i++) {
    const r = pick(reps);
    const key = pick(KEYS);
    const op = r.set(key, valueFor(key));
    assert.ok(validOp(op), `a page's own op passes the check (${JSON.stringify(op)})`);
    log.push(op);
    if (rand(4) === 0) {
      const other = pick(reps);
      for (const op of log.slice(-3)) other.apply(op);
    }
  }
  for (const r of reps) {
    const order = [...log, ...log.slice(0, rand(log.length))];
    for (let i = order.length - 1; i > 0; i--) { const j = rand(i + 1); [order[i], order[j]] = [order[j], order[i]]; }
    for (const op of order) assert.notEqual(r.receive(JSON.parse(JSON.stringify(op))), 'invalid');
  }
  if (lateJoiner) {
    const late = new Replica('zed');
    for (const op of pick(reps).snapshot()) late.apply(op);
    reps.push(late);
  }
  const want = JSON.stringify(reps[0].directives());
  for (const r of reps) assert.equal(JSON.stringify(r.directives()), want, 'every peer ends with the same directives');
  const regs = JSON.stringify([...reps[0].regs].sort());
  for (const r of reps) assert.equal(JSON.stringify([...r.regs].sort()), regs, 'and the same registers');
}

for (let i = 0; i < 2000; i++) round(['ana', 'ben', 'cy'].slice(0, 2 + rand(2)), 5 + rand(40), rand(2) === 0);

const a = new Replica('ana');
const b = new Replica('ben');
const renamed = a.set('var:0x1198:v1:name', 'total');
const retyped = b.set('var:0x1198:v1:type', 'unsigned long');
a.apply(retyped);
b.apply(renamed);
assert.deepEqual(a.directives(), ['type v1 unsigned long total'], 'a concurrent rename and retype of one local both survive');
assert.deepEqual(b.directives(), a.directives());

const c1 = a.set('fn:0x1149', 'adder');
const c2 = b.set('fn:0x1149', 'add_two');
a.apply(c2);
b.apply(c1);
assert.equal(a.regs.get('fn:0x1149').v, b.regs.get('fn:0x1149').v, 'concurrent renames of one function agree on a winner');

const mine = a.set('comment:0x1198:0x11b5', 'calls add');
b.apply(mine);
const theirs = b.set('comment:0x1198:0x11b5', 'hot path');
a.apply(theirs);
assert.equal(a.undo('comment:0x1198:0x11b5', null, mine.c), null, 'undo will not erase what someone else wrote since');
const mine2 = a.set('byte:0x11af', '05');
const undone = a.undo('byte:0x11af', null, mine2.c);
b.apply(mine2);
b.apply(undone);
assert.equal(b.regs.get('byte:0x11af').v, null, 'my own undo reaches the others as an ordinary write');

const c = (n = 7) => [n, 'eve'];
const HOSTILE = [
  [{ k: 'raw:eve:1', v: '@/work/input.bin', c: c() }, 'a raw directive that is @FILE'],
  [{ k: 'raw:eve:2', v: '   @/work/input.bin', c: c() }, 'the same behind spaces (the page trims raw text)'],
  [{ k: 'raw:eve:3', v: 'bytes 0x11af @/work/input.bin', c: c() }, 'a raw bytes directive whose payload is @FILE'],
  [{ k: 'raw:eve:4', v: 'bytes  0x11af\t@x', c: c() }, 'the same with odd spacing'],
  [{ k: 'comment:0x1198:0x11b5', v: 'fine\nbytes 0x401000 @/home/u/.ssh/id_rsa', c: c() }, 'a newline that would start a directive in an exported .kuna file'],
  [{ k: 'var:0x1198:v1:type', v: 'long @x', c: c() }, 'an @ in a type'],
  [{ k: 'byte:0x11af', v: '9g', c: c() }, 'a byte that is not two hex digits'],
  [{ k: 'byte:0x11af', v: '@x', c: c() }, 'a byte that is a file'],
  [{ k: 'var:0x1198:v1:name', v: 'x y', c: c() }, 'a name that is not an identifier'],
  [{ k: 'var:0x1198:v1:name', v: 'a'.repeat(129), c: c() }, 'an overlong name'],
  [{ k: 'comment:0x1198:0x11b5', v: 'x'.repeat(1025), c: c() }, 'an overlong note'],
  [{ k: 'flow:0x1198', v: 'return', c: c() }, 'a key of a kind the session does not have'],
  [{ k: 'setting:mode', v: 'turbo', c: c() }, 'a mode that does not exist'],
  [{ k: 'fn:0x1149', v: 'ok', c: [-1, 'eve'] }, 'a negative clock'],
  [{ k: 'fn:0x1149', v: 'ok', c: [1, 'Eve!'] }, 'a malformed peer id'],
  [{ k: 'fn:0x1149', v: 'ok', c: c(), extra: 1 }, 'an extra field'],
  [{ k: 'fn:0x1149', v: 7, c: c() }, 'a value that is not a string'],
];
const victim = new Replica('ana');
const before = JSON.stringify(victim.snapshot());
for (const [op, why] of HOSTILE) assert.equal(victim.receive(op), 'invalid', `rejected: ${why}`);
assert.equal(JSON.stringify(victim.snapshot()), before, 'and nothing was applied');
assert.equal(victim.receive({ k: 'comment:0x1198:0x11b5', v: 'see @notes', c: c() }), true, 'an @ inside a note is text, and allowed');
assert.equal(victim.receive({ k: 'raw:eve:5', v: 'flow 0x11b5 return', c: c() }), true, 'a raw directive without a file form is allowed');
assert.ok(victim.directives().every((d) => !d.startsWith('@') && !/^bytes\s+\S+\s+@/.test(d)), 'no directive the page sends can read a file');

console.log(`LWW OK — ${HOSTILE.length} hostile ops rejected (@FILE, bytes @FILE, a newline, bad shapes); `
  + '2000 random rounds (2-3 peers, duplicates, any order, late-join snapshots) converge; rename+retype merge; undo respects later writers');
