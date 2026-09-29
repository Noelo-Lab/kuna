// decompile2-collab-cases.mjs — live-session cases that need no browser, one
// per defect a review found (the number is the review's): the order the
// engine gets directives in, alone and shared; a page that joins again
// sending what it changed meanwhile; Undo through several edits of a field;
// field-by-field merging; a global's two halves; values another page refuses;
// a handshake that dies before the welcome; routes through a page that left;
// bounded clocks; messages sized in UTF-8; names and programs that cannot
// travel; the register cap and resync; huge local batches; a late
// BroadcastChannel knock; forwarding only where needed; the shared session
// kept apart in storage; a program over 32 MB sent to a page that lost the
// inviter's first hello; a connection that cannot be set up; a page whose
// clock races ahead. Every case runs and is reported; any failure exits 1.
//   node integrations/web/test/decompile2-collab-cases.mjs
import assert from 'node:assert/strict';
import { createHash, randomBytes } from 'node:crypto';
import * as S from '../decompile2/session.js';
import * as R from '../decompile2/collab/replica.js';
import * as W from '../decompile2/collab/wire.js';
import * as G from '../decompile2/collab/group.js';
import * as P from '../decompile2/persist.js';

globalThis.addEventListener ??= () => {};
globalThis.removeEventListener ??= () => {};
const results = [];
async function test(name, fn) {
  try {
    await fn();
    results.push([true, name]);
  } catch (e) {
    results.push([false, name, e.message.split('\n')[0]]);
  }
}
const sleep = (ms) => new Promise((done) => setTimeout(done, ms));
const MAIN = '0x1198';
const SUM = '0x1161';

// ── the order directives reach the engine ──────────────────────────────────
await test('#2 alone, a page sends each kind in the order it was made: a struct before the struct that uses it', () => {
  const s = new S.Session();
  s.setTypedef('point', 'struct point { int x; int y; } point;');
  s.setTypedef('line', 'struct line { struct point a; struct point b; } line;');
  assert.deepEqual(s.globalAssertions().slice(0, 2), [
    'typedef struct point { int x; int y; } point;',
    'typedef struct line { struct point a; struct point b; } line;',
  ]);
});
await test('#2 alone, the later of two raw directives comes last (raw:10 after raw:2)', () => {
  const s = new S.Session();
  for (let i = 1; i <= 10; i++) s.addRaw(`prototype ${SUM} long sum_to(int v${i})`);
  const list = s.globalAssertions();
  assert.equal(list[list.length - 1], `prototype ${SUM} long sum_to(int v10)`);
});
await test('#2 alone, a rename that frees a name comes before the rename that takes it', () => {
  const s = new S.Session();
  s.setVar(SUM, 'v1', { name: 'i' });
  s.setVar(SUM, 'acc', { name: 'v1' });
  assert.deepEqual(s.assertionsFor(SUM), ['name v1 i', 'name acc v1']);
});
await test('#2 shared, pages that applied the same registers in other orders send one list, in birth order', () => {
  const a = new R.Replica('ana00000');
  const ops = [
    a.set('typedef:point', 'struct point { int x; int y; } point;'),
    a.set('typedef:line', 'struct line { struct point a; struct point b; } line;'),
    a.set(`var:${SUM}:v1:name`, 'i'),
    a.set(`var:${SUM}:acc:name`, 'v1'),
  ];
  const b = new R.Replica('ben00000');
  for (const op of [...ops].reverse()) b.receive(JSON.parse(JSON.stringify(op)));
  const pages = [a, b].map((r) => {
    const s = new S.Session();
    R.applyRegisters(s, r, [...r.regs.keys()].reverse());
    s.orderOf = R.birthOrder(r);
    return s;
  });
  assert.deepEqual(pages[1].allAssertions((x) => x), pages[0].allAssertions((x) => x));
  assert.deepEqual(pages[0].assertionsFor(SUM), ['typedef struct point { int x; int y; } point;',
    'typedef struct line { struct point a; struct point b; } line;', 'name v1 i', 'name acc v1']);
});
await test('#2 shared, a later edit keeps a record\'s place, and a newcomer learns the same birth from a snapshot', () => {
  const a = new R.Replica('ana00000');
  a.set('typedef:point', 'struct point { int x; } point;');
  a.set('typedef:line', 'struct line { struct point a; } line;');
  a.set('typedef:point', 'struct point { int x; int y; } point;');
  const late = new R.Replica('zed00000');
  for (const op of a.snapshot()) late.receive(op);
  assert.deepEqual(late.birth('typedef:point'), a.birth('typedef:point'));
  const s = new S.Session();
  R.applyRegisters(s, late, ['typedef:line', 'typedef:point']);
  s.orderOf = R.birthOrder(late);
  assert.match(s.globalAssertions()[0], /struct point \{ int x; int y; \}/);
});
await test('#2 leaving keeps the session\'s order for the page on its own (and in what it saves)', () => {
  const r = new R.Replica('ana00000');
  r.set('typedef:point', 'struct point { int x; } point;');
  r.set('typedef:line', 'struct line { struct point a; } line;');
  const s = new S.Session();
  R.applyRegisters(s, r, ['typedef:line', 'typedef:point']);
  s.orderOf = R.birthOrder(r);
  assert.deepEqual(s.toJSON().records.map(([k]) => k), ['typedef:point', 'typedef:line'], 'saved in birth order');
  s.reorder();
  s.orderOf = null;
  assert.match(s.globalAssertions()[0], /struct point/, 'and kept that way on its own');
});
await test('#2 births merge as a grow-only minimum, in any order', () => {
  const x = new R.Replica('ana00000');
  const y = new R.Replica('ben00000');
  const ox = x.set('fn:0x10', 'a');
  const oy = y.set('fn:0x10', 'b');
  x.receive(oy);
  y.receive(ox);
  assert.deepEqual(x.birth('fn:0x10'), y.birth('fn:0x10'));
  assert.deepEqual(x.birth('fn:0x10'), [2, 'ana00000']);
});

// ── Undo in a live session ─────────────────────────────────────────────────
const write = (rep, key, value) => {
  const prev = rep.value(key);
  const prevClock = rep.clock(key);
  return { key, prev, prevClock, op: rep.set(key, value) };
};
await test('#5 Undo goes back through two edits of one field, and after a redo', () => {
  const r = new R.Replica('ana00000');
  const h = new R.History();
  const K = `var:${MAIN}:v1:name`;
  h.record([write(r, K, 'total')]);
  h.record([write(r, K, 'sum')]);
  assert.equal(h.undo(r).ops.length, 1);
  assert.equal(r.value(K), 'total');
  const second = h.undo(r);
  assert.deepEqual(second.skipped, [], 'the older step is still this page\'s own');
  assert.equal(r.value(K), null);
  h.redo(r);
  h.redo(r);
  assert.equal(r.value(K), 'sum', 'and redo goes forward through both');
  h.undo(r);
  assert.equal(r.value(K), 'total');
});
await test('#8 a deliberate revert is a step of its own, and Undo takes back only it', () => {
  const r = new R.Replica('ana00000');
  const h = new R.History();
  h.record([write(r, 'fn:0x1149', 'adder')]);
  h.record([write(r, 'comment:0x1198:0x11b5', 'note')]);
  h.record([write(r, 'comment:0x1198:0x11b5', null)]);
  assert.equal(h.undoStack.length, 3);
  h.undo(r);
  assert.equal(r.value('comment:0x1198:0x11b5'), 'note', 'undo restores the note');
  assert.equal(r.value('fn:0x1149'), 'adder', 'and leaves the earlier, unrelated rename');
  h.record([write(r, 'fn:0x1149', 'plus')]);
  assert.equal(h.canRedo, false, 'every recorded change clears redo');
});
await test('#9 a global goes back whole or not at all', () => {
  const a = new R.Replica('ana00000');
  const b = new R.Replica('ben00000');
  const h = new R.History();
  const step = [write(a, 'data:0x4010:type', 'int'), write(a, 'data:0x4010:name', 'counter')];
  h.record(step);
  for (const w of step) b.receive(w.op);
  a.receive(b.set('data:0x4010:name', 'total'));
  const res = h.undo(a);
  assert.equal(res.ops.length, 0, 'nothing written');
  assert.deepEqual(res.skipped.map((x) => x.by), ['ben00000', 'ben00000']);
  assert.equal(a.value('data:0x4010:type'), 'int');
});
await test('second review #10 a global with a half written null is gone on every page, whatever each page held before', () => {
  const r = new R.Replica('ana00000');
  r.set('data:0x4010:type', 'int');
  r.set('data:0x4010:name', 'total');
  const held = new S.Session();
  R.applyRegisters(held, r, ['data:0x4010:type', 'data:0x4010:name']);
  const fresh = new S.Session();
  r.set('data:0x4010:type', null);
  r.set('data:0x4010:name', 'renamed');
  for (const s of [held, fresh]) R.applyRegisters(s, r, ['data:0x4010:type', 'data:0x4010:name']);
  assert.deepEqual([...held.records.keys()], [...fresh.records.keys()], 'both pages make the same Session from the same registers');
  assert.equal(held.records.get('data:0x4010'), undefined, 'a global needs both halves');
  assert.deepEqual([...R.registersOf(held)], [], 'and its registers read back as nothing to send');
});

// ── merging and sharing ────────────────────────────────────────────────────
await test('#7 another page\'s retype of a local keeps this page\'s rename of it', () => {
  const s = new S.Session();
  s.setVar(MAIN, 'v1', { name: 'mine' });
  const group = new R.Replica('ana00000');
  group.set(`var:${MAIN}:v1:type`, 'unsigned long');
  R.applyRegisters(s, group, [`var:${MAIN}:v1:type`]);
  assert.deepEqual(s.assertionsFor(MAIN), ['type v1 unsigned long mine']);
});
await test('#11 a value the others refuse is tried again, and a later valid value of the field is shared', () => {
  const s = new S.Session();
  const r = new R.Replica('ana00000');
  const base = new Map();
  s.setComment(MAIN, '0x11b5', '#hot');
  const first = R.localChanges(R.changedBetween(base, R.registersOf(s)), r);
  assert.deepEqual(first.changes, []);
  assert.equal(first.refused.length, 1);
  s.setComment(MAIN, '0x11b5', 'hot path');
  assert.deepEqual(R.localChanges(R.changedBetween(base, R.registersOf(s)), r).changes, [{ key: `comment:${MAIN}:0x11b5`, value: 'hot path' }]);
  assert.equal(S.directiveTextProblem('#hot') !== null, true, 'and the page\'s own note dialog refuses it too');
});
await test('#16 a counter far past this page\'s is refused, so one op cannot stop everyone', () => {
  const ana = new R.Replica('ana00000');
  const ben = new R.Replica('ben00000');
  assert.equal(ben.receive({ k: 'fn:0x1149', v: 'pwned', c: [Number.MAX_SAFE_INTEGER, 'eve00000'] }), 'invalid');
  assert.equal(ben.receive({ k: 'fn:0x1149', v: 'pwned', c: [2 ** 30, 'eve00000'] }), 'invalid');
  const op = ana.set('fn:0x1161', 'summation');
  assert.equal(ben.receive(op), true, 'syncing goes on');
  assert.ok(Number.isSafeInteger(ana.set('fn:0x1149', 'adder').c[0]));
});
await test('#18 names and programs are cleaned or refused before they travel', () => {
  assert.equal(W.cleanName('A na \u0007Lopez'), 'A na Lopez');
  assert.equal(W.cleanName('\u0000'), '');
  assert.match(W.describeFile('crackme.elf', 70 << 20, 'a'.repeat(64)).problem, /64 MB/);
  assert.deepEqual(W.describeFile('crack\tme/x.elf', 100, 'a'.repeat(64)).meta, { name: 'crack me_x.elf', size: 100, hash: 'a'.repeat(64) });
});
await test('#6 a session joined from someone else is saved apart from the student\'s own', () => {
  const mem = new Map();
  const storage = { getItem: (k) => (mem.has(k) ? mem.get(k) : null), setItem: (k, v) => mem.set(k, v), removeItem: (k) => mem.delete(k) };
  const own = new P.SessionStore(storage);
  const shared = new P.SessionStore(storage, { prefix: 'kuna.d2.shared.', indexKey: 'kuna.d2.shared.index' });
  own.save('sha256:ab', 'a.elf', 'mine');
  shared.save('sha256:ab', 'a.elf', 'theirs');
  assert.equal(own.load('sha256:ab'), 'mine');
  assert.equal(shared.load('sha256:ab'), 'theirs');
});

// ── whole groups over in-memory links ──────────────────────────────────────
const BUILD = 'c'.repeat(64);
const FILE_BYTES = new Uint8Array(randomBytes(20000));
const FILE = { name: 'sample.elf', size: FILE_BYTES.length, hash: createHash('sha256').update(FILE_BYTES).digest('hex') };
let delivered = 0;

function linkPair() {
  const make = () => ({
    onmessage: null, onclose: null, closed: false, texts: [],
    send(text) { this.texts.push(text); this.deliver(text, 'edits'); },
    sendCursor(text) { this.deliver(text, 'cursor'); },
    sendBinary(bytes) { this.deliver(bytes.slice().buffer, 'edits'); },
    buffered() { return 0; },
    drain() { return Promise.resolve(); },
    deliver(data, channel) {
      if (this.closed || this.lossy) return;
      const to = this.other;
      setImmediate(() => { if (!to.closed) { delivered++; to.onmessage?.(data, channel); } });
    },
    close() {
      if (this.closed) return;
      this.closed = true;
      this.onclose?.();
      setImmediate(() => this.other.close());
    },
  });
  const a = make();
  const b = make();
  a.other = b;
  b.other = a;
  return [a, b];
}
const FAKE_SDP = { u: 'abcd', p: 'x'.repeat(24), f: 'A'.repeat(64), c: [] };
const blocked = new Set();
const offers = new Map();
function connector(me) {
  return {
    async offer() {
      const id = W.randomId(10);
      let settle;
      const ready = new Promise((done) => { settle = done; });
      offers.set(id, { from: me, settle });
      return { id, sdp: FAKE_SDP, bc: false, ready, answer: async () => {}, cancel() { offers.delete(id); } };
    },
    async answer({ id }) {
      const off = offers.get(id);
      if (!off) throw new Error('used');
      offers.delete(id);
      if (blocked.has([off.from, me].sort().join('-'))) throw new Error('failed');
      const [a, b] = linkPair();
      off.settle(a);
      return { sdp: null, ready: Promise.resolve(b), cancel() {} };
    },
  };
}
function member(peer, name, { file = null, bytes = null, replica = null, group = {} } = {}) {
  const p = {
    name, file, bytes, events: [],
    fileMeta() { return this.file; }, fileBytes() { return this.bytes; }, isExample() { return false; },
    welcomed() {}, caughtUp() {}, changed() {}, fileProgress() {}, fileFailed() {}, roster() {}, where() {}, cursor() {}, ping() {},
    fileArrived(b, m) { this.file = m; this.bytes = b; },
    event(kind, info) { this.events.push([kind, info.name]); },
  };
  const r = replica || new R.Replica(peer);
  const g = new G.Group({ me: peer, name, build: BUILD, replica: r, connect: connector(peer), page: p, ...group });
  return { g, p, replica: r, peer };
}
function invite(host, guest) {
  const [a, b] = linkPair();
  host.g.addLink(a);
  guest.g.addLink(b, { joining: true });
  return [a, b];
}
const settle = async (ms = 60) => { for (let i = 0; i < 6; i++) await sleep(ms / 6); };
/** Wait until `ok()` (checked every 50 ms), at most `ms`; returns whether it came true. */
const until = async (ok, ms = 5000) => {
  for (const end = Date.now() + ms; Date.now() < end; await sleep(50)) if (ok()) return true;
  return ok();
};
const same = (...ms) => ms.every((m) => JSON.stringify([...m.replica.regs].sort()) === JSON.stringify([...ms[0].replica.regs].sort()));

await test('#3 a page that joins again sends what it changed while it was away', async () => {
  const ana = member('aaaaaaaa', 'Ana', { file: FILE, bytes: FILE_BYTES });
  ana.g.create();
  ana.g.local([ana.replica.set('fn:0x1149', 'adder')]);
  const ben = member('bbbbbbbb', 'Ben');
  const [, b] = invite(ana, ben);
  await settle(150);
  b.close();
  await settle(60);
  ben.g.local([ben.replica.set(`var:${MAIN}:v1:name`, 'ben_offline')]);
  const again = member('bbbbbbbb', 'Ben', { file: FILE, bytes: FILE_BYTES, replica: ben.replica });
  invite(ana, again);
  await settle(200);
  assert.equal(ana.replica.value(`var:${MAIN}:v1:name`), 'ben_offline');
  assert.ok(same(ana, again));
  ana.g.leave();
  again.g.leave();
  ben.g.leave();
});
await test('#13 a guest is told when the inviter\'s page closes the link after its hello but before the welcome', async () => {
  const ben = member('bbbbbbbb', 'Ben');
  const [a, b] = linkPair();
  ben.g.addLink(b, { joining: true });
  a.onmessage = () => {};
  a.send(JSON.stringify({ t: 'hello', proto: W.PROTOCOL, build: BUILD, peer: 'aaaaaaaa', name: 'Ana', color: W.COLORS[0], sid: 'abcdefabcdef', file: FILE }));
  await settle(30);
  a.close();
  await settle(30);
  assert.deepEqual(ben.p.events, [['closed', 'Ana']]);
  ben.g.leave();
});
await test('#14 a member reached only through someone who left is dropped, and the user is told', async () => {
  blocked.add('aaaaaaaa-cccccccc');
  const ana = member('aaaaaaaa', 'Ana', { file: FILE, bytes: FILE_BYTES });
  ana.g.create();
  const ben = member('bbbbbbbb', 'Ben');
  const cy = member('cccccccc', 'Cy');
  invite(ana, ben);
  await settle(150);
  invite(ben, cy);
  await settle(200);
  assert.equal(ana.g.size(), 3);
  ben.g.leave();
  await settle(400);
  assert.equal(ana.g.members().some((m) => m.peer === 'cccccccc'), false, 'Cy is not listed as "connecting…" forever');
  assert.ok(ana.p.events.some(([k, n]) => k === 'lost' && n === 'Cy'));
  blocked.clear();
  ana.g.leave();
  cy.g.leave();
});
await test('#17 every message fits a data channel in UTF-8 bytes, with a session of non-ASCII notes', async () => {
  const ana = member('aaaaaaaa', 'Ana', { file: FILE, bytes: FILE_BYTES });
  ana.g.create();
  for (let i = 0; i < 400; i++) ana.replica.set(`comment:${MAIN}:0x${(0x1000 + i).toString(16)}`, '中'.repeat(1000));
  const ben = member('bbbbbbbb', 'Ben');
  const [a] = invite(ana, ben);
  await settle(300);
  const biggest = Math.max(...a.texts.map((t) => Buffer.byteLength(t)));
  assert.ok(biggest <= W.MAX_MESSAGE, `largest message ${biggest} bytes`);
  assert.ok(same(ana, ben), 'and the newcomer has all of it');
  ana.g.leave();
  ben.g.leave();
});
await test('#19 the cap counts live registers, not deletions', async () => {
  const cap = G.MAX_REGISTERS ?? 100000;
  const ana = member('aaaaaaaa', 'Ana', { file: FILE, bytes: FILE_BYTES });
  ana.g.create();
  const ben = member('bbbbbbbb', 'Ben');
  invite(ana, ben);
  await settle(150);
  for (let i = 0; i < cap; i++) ben.replica.apply({ k: `byte:0x${(0x100000 + i).toString(16)}`, v: null, c: [i + 1, 'zzz00000'] });
  ana.g.local([ana.replica.set('fn:0x1149', 'adder')]);
  await settle(200);
  assert.equal(ben.replica.value('fn:0x1149'), 'adder');
  ana.g.leave();
  ben.g.leave();
});
await test('#19 a page that had to drop edits asks for them again and catches up', async () => {
  const ana = member('aaaaaaaa', 'Ana', { file: FILE, bytes: FILE_BYTES });
  ana.g.create();
  const ben = member('bbbbbbbb', 'Ben');
  const [a] = invite(ana, ben);
  await settle(150);
  for (let i = 0; i < 40; i++) {
    const op = ana.replica.set(`fn:0x${(0x2000 + i).toString(16)}`, `f${i}`);
    a.send(JSON.stringify({ t: 'ops', ops: [op] }));
  }
  await sleep(2600);
  assert.ok(same(ana, ben), `Ben has ${[...ben.replica.regs.keys()].filter((k) => k.startsWith('fn:0x2')).length} of 40`);
  ana.g.leave();
  ben.g.leave();
});
await test('#27 a very large local batch is queued without overflowing the stack', async () => {
  const ana = member('aaaaaaaa', 'Ana', { file: FILE, bytes: FILE_BYTES });
  ana.g.create();
  const ben = member('bbbbbbbb', 'Ben');
  invite(ana, ben);
  await settle(150);
  const ops = [];
  for (let i = 0; i < 150000; i++) ops.push({ k: `byte:0x${(0x300000 + i).toString(16)}`, v: '90', c: [i + 10, 'aaaaaaaa'] });
  ana.g.local(ops);
  ana.g.leave();
  ben.g.leave();
});
await test('D an edit crosses a full mesh once per page, not once per pair', async () => {
  const ana = member('aaaaaaaa', 'Ana', { file: FILE, bytes: FILE_BYTES });
  ana.g.create();
  const ben = member('bbbbbbbb', 'Ben');
  const cy = member('cccccccc', 'Cy');
  const dee = member('dddddddd', 'Dee');
  invite(ana, ben);
  await settle(150);
  invite(ben, cy);
  await settle(250);
  invite(cy, dee);
  await settle(300);
  let copies = 0;
  for (const m of [ben, cy, dee]) {
    for (const rec of m.g.peers.values()) {
      const before = rec.link.onmessage;
      rec.link.onmessage = (data, ch) => { if (typeof data === 'string' && data.includes('"fn:0x7777"')) copies++; before(data, ch); };
    }
  }
  ana.g.local([ana.replica.set('fn:0x7777', 'once')]);
  await settle(300);
  assert.ok(same(ana, ben, cy, dee));
  assert.equal(copies, 3, `copies delivered: ${copies}`);
  for (const m of [ana, ben, cy, dee]) m.g.leave();
});

// ── a second review: what is passed on, and pages comparing notes ──────────
await test('second review #5 a newer write of the same value is passed on, so a later write in between cannot split the pages', async () => {
  blocked.add('bbbbbbbb-cccccccc');
  const quiet = { sumMs: 1e8 };
  const ana = member('aaaaaaaa', 'Ana', { file: FILE, bytes: FILE_BYTES, group: quiet });
  ana.g.create();
  const ben = member('bbbbbbbb', 'Ben', { group: quiet });
  const cy = member('cccccccc', 'Cy', { group: quiet });
  try {
    invite(ana, ben);
    await until(() => ana.g.peers.get('bbbbbbbb')?.member && ben.g.sid);
    invite(ana, cy);
    await until(() => cy.g.members().length === 3 && ben.g.members().length === 3);
    await settle(300);
    assert.ok(!ben.g.peers.has('cccccccc'), 'Ben and Cy cannot link: Cy hears Ben only through Ana');
    const K = 'fn:0x5555';
    ben.g.local([ben.replica.set(K, 'x')]);
    await until(() => cy.replica.value(K) === 'x');
    ben.replica.counter += 10;
    ben.g.local([ben.replica.set(K, 'x')]);
    await until(() => ana.replica.clock(K)?.[0] === ben.replica.clock(K)[0]);
    await settle(300);
    assert.deepEqual(cy.replica.clock(K), ben.replica.clock(K), 'Cy has Ben\'s newer clock, passed on by Ana');
    const between = [ben.replica.clock(K)[0] - 5, 'cccccccc'];
    const op = { k: K, v: 'y', c: between, b: between };
    cy.replica.apply(op);
    cy.g.local([op]);
    await settle(300);
    assert.deepEqual([ana, ben, cy].map((m) => m.replica.value(K)), ['x', 'x', 'x'], 'every page holds the same value');
  } finally {
    blocked.delete('bbbbbbbb-cccccccc');
    for (const m of [ana, ben, cy]) m.g.leave();
  }
});
await test('second review #6 an edit lost on a link that dies reaches that page through the others', async () => {
  const fast = { sumMs: 1e8, quietMs: 100 };
  const ana = member('aaaaaaaa', 'Ana', { file: FILE, bytes: FILE_BYTES, group: fast });
  ana.g.create();
  const ben = member('bbbbbbbb', 'Ben', { group: fast });
  const cy = member('cccccccc', 'Cy', { group: fast });
  try {
    invite(ana, ben);
    await until(() => ana.g.peers.get('bbbbbbbb')?.member && ben.g.sid);
    invite(ana, cy);
    assert.ok(await until(() => ben.g.peers.get('cccccccc')?.member && cy.g.peers.get('bbbbbbbb')?.member), 'a full mesh');
    await settle(300);
    const ab = ana.g.peers.get('bbbbbbbb').link;
    ab.lossy = true;
    ab.other.lossy = true;
    const K = 'comment:0x1198:0x11b5';
    ana.g.local([ana.replica.set(K, 'lost on the way to Ben')]);
    await until(() => cy.replica.value(K) === 'lost on the way to Ben');
    await settle(300);
    assert.equal(cy.replica.value(K), 'lost on the way to Ben');
    assert.equal(ben.replica.value(K), null, 'Ben did not get it: the link to Ana was already failing');
    blocked.add('aaaaaaaa-bbbbbbbb');
    ab.close();
    assert.ok(await until(() => ben.replica.value(K) === 'lost on the way to Ben', 15000), 'Ben has the edit, from Cy, once they compared digests after Ana\'s link went');
    await until(() => same(ana, ben, cy));
    assert.ok(same(ana, ben, cy));
  } finally {
    blocked.delete('aaaaaaaa-bbbbbbbb');
    for (const m of [ana, ben, cy]) m.g.leave();
  }
});

// ── a third review ─────────────────────────────────────────────────────────
await test('a hello lost as the channel opens is sent again, whichever side lost it, and the join completes', async () => {
  for (const lost of ['joiner', 'inviter']) {
    const ana = member('aaaaaaaa', 'Ana', { file: FILE, bytes: FILE_BYTES });
    ana.g.create();
    ana.g.local([ana.replica.set('fn:0x1149', 'adder')]);
    const ben = member('bbbbbbbb', 'Ben');
    const [a, b] = linkPair();
    const end = lost === 'joiner' ? b : a;
    const send = end.send;
    let first = true;
    end.send = function sendButLoseTheFirst(text) {
      if (first) {
        first = false;
        return;
      }
      send.call(this, text);
    };
    ana.g.addLink(a);
    ben.g.addLink(b, { joining: true });
    assert.ok(await until(() => ben.replica.value('fn:0x1149') === 'adder', 8000), `Ben joined although the ${lost}'s hello was lost`);
    assert.ok(await until(() => ana.g.peers.get('bbbbbbbb')?.member));
    ana.g.leave();
    ben.g.leave();
  }
});
await test('third review #14 a message over the channel\'s limit in UTF-8 bytes is not sent, even under it in characters', async () => {
  const big = { name: '€'.repeat(90000), size: 100, hash: 'a'.repeat(64) };
  const ana = member('aaaaaaaa', 'Ana', { file: FILE, bytes: FILE_BYTES });
  ana.g.create();
  const ben = member('bbbbbbbb', 'Ben');
  const [a] = invite(ana, ben);
  Object.assign(ana.p, { file: big, bytes: new Uint8Array(100) });
  await settle(60);
  const over = a.texts.filter((t) => W.utf8Length(t) > W.MAX_MESSAGE);
  assert.ok(a.texts.length > 0);
  assert.deepEqual(over.map((t) => t.slice(0, 20)), [], 'nothing over the limit reached the channel');
  ana.g.leave();
  ben.g.leave();
});
await test('third review #8 a directive added after the registers gave this page\'s own back is added, not written over one', () => {
  const s = new S.Session();
  s.setRaw('raw:ana00000:1', 'readonly 0x2000+8');
  s.addRaw('volatile 0x3000+4');
  R.adoptRawKeys(s, 'ana00000');
  assert.deepEqual([...s.records.values()].map((r) => r.text).sort(), ['readonly 0x2000+8', 'volatile 0x3000+4']);
  s.addRaw('readonly 0x2100+4');
  R.adoptRawKeys(s, 'ana00000');
  assert.equal(s.records.size, 3, 'and the next one too');
});
await test('third review #9 applying registers leaves a record whose value did not change as the engine last saw it', () => {
  const r = new R.Replica('ana00000');
  r.set(`fn:${SUM}`, 'summation');
  r.set(`var:${MAIN}:v1:name`, 'total');
  r.set('data:0x4010:type', 'int');
  r.set('data:0x4010:name', 'counter');
  r.set('byte:0x11e1', '90');
  const s = new S.Session();
  R.applyRegisters(s, r, [...r.regs.keys()]);
  s.assertionsFor(MAIN);
  s.recordOutcomes(s.assertionsFor(MAIN).map((directive) => ({ directive, status: 'applied' })), MAIN);
  const before = [...s.outcomes.keys()].sort();
  assert.ok(before.length >= 4);
  R.applyRegisters(s, r, [...r.regs.keys()]);
  assert.deepEqual([...s.outcomes.keys()].sort(), before, 'no outcome was cleared');
});

// ── the BroadcastChannel knock ─────────────────────────────────────────────
const SDP = 'v=0\r\no=- 1 2 IN IP4 127.0.0.1\r\ns=-\r\nt=0 0\r\na=group:BUNDLE 0\r\nm=application 9 UDP/DTLS/SCTP webrtc-datachannel\r\n' +
  'c=IN IP4 0.0.0.0\r\na=candidate:1 1 udp 2113937151 127.0.0.1 40000 typ host\r\na=ice-ufrag:abcd\r\na=ice-pwd:' + 'x'.repeat(24) +
  '\r\na=fingerprint:sha-256 ' + Array(32).fill('AB').join(':') + '\r\na=setup:actpass\r\na=mid:0\r\na=sctp-port:5000\r\n';
class FakePC {
  static made = [];
  constructor() { this.iceGatheringState = 'complete'; this.localDescription = null; this.listeners = new Map(); FakePC.made.push(this); }
  createDataChannel(label) { return { label, readyState: 'connecting', addEventListener() {}, close() {} }; }
  async createOffer() { return { type: 'offer', sdp: SDP }; }
  async createAnswer() { return { type: 'answer', sdp: SDP.replace('actpass', 'active') }; }
  async setLocalDescription(d) { this.localDescription = d; }
  async setRemoteDescription(d) { this.remote = d; }
  addEventListener(t, f) { this.listeners.set(t, f); }
  close() {}
}
await test('#29 a knock the guest gave up on is not a link: the WebRTC reply still connects', async () => {
  globalThis.RTCPeerConnection = FakePC;
  const L = await import('../decompile2/collab/link.js');
  const offer = await L.makeOffer({ me: 'aaaaaaaa' });
  const ch = new BroadcastChannel(`kuna.d2.link.${offer.id}`);
  const acks = [];
  ch.onmessage = ({ data }) => { if (data?.k === 'ack') acks.push(data); };
  ch.postMessage({ k: 'knock', from: 'bbbbbbbb' });
  await sleep(2000);
  ch.close();
  assert.equal(acks.length, 1, 'the offer answered the knock');
  await offer.answer({ u: 'abcd', p: 'x'.repeat(24), f: 'A'.repeat(64), c: ['127.0.0.1 40001 hu 2113937151'] });
  offer.cancel();
});
await test('#29 a guest that gets an ack too late says so, and goes on with WebRTC', async () => {
  globalThis.RTCPeerConnection = FakePC;
  const L = await import('../decompile2/collab/link.js');
  const id = W.randomId(10);
  const ch = new BroadcastChannel(`kuna.d2.link.${id}`);
  const said = [];
  ch.onmessage = ({ data }) => {
    if (data?.k === 'knock') setTimeout(() => ch.postMessage({ k: 'ack', from: 'aaaaaaaa', to: data.from }), 800);
    if (data?.k === 'yes' || data?.k === 'no') said.push(data.k);
  };
  const res = await L.takeOffer({ me: 'bbbbbbbb', id, sdp: { u: 'abcd', p: 'x'.repeat(24), f: 'A'.repeat(64), c: [] } });
  await sleep(600);
  ch.close();
  assert.ok(res.sdp, 'it made a WebRTC answer');
  assert.deepEqual(said, ['no']);
  assert.match(FakePC.made.at(-1).localDescription.sdp, /a=setup:passive\r\n/, '#28 and the answer it uses is passive');
  res.cancel?.();
});

await test('fourth review #8 a connection that cannot be set up is closed, on either side, and the invite with it', async () => {
  class FailingPC extends FakePC {
    close() { this.closed = true; }
    async setRemoteDescription() { throw new Error('the description was refused'); }
  }
  globalThis.RTCPeerConnection = FailingPC;
  const L = await import('../decompile2/collab/link.js');
  await assert.rejects(L.takeOffer({ me: 'bbbbbbbb', id: W.randomId(10), sdp: { u: 'abcd', p: 'x'.repeat(24), f: 'A'.repeat(64), c: [] } }));
  assert.equal(FakePC.made.at(-1).closed, true, 'the guest closes its connection');
  const offer = await L.makeOffer({ me: 'aaaaaaaa' });
  const pc = FakePC.made.at(-1);
  await assert.rejects(offer.answer({ u: 'abcd', p: 'x'.repeat(24), f: 'A'.repeat(64), c: ['127.0.0.1 40001 hu 2113937151'] }));
  assert.equal(pc.closed, true, 'the inviter closes its connection');
  await assert.rejects(offer.ready, 'and the offer is over');
  const ch = new BroadcastChannel(`kuna.d2.link.${offer.id}`);
  const heard = [];
  ch.onmessage = ({ data }) => { if (data?.k !== 'knock') heard.push(data?.k); };
  ch.postMessage({ k: 'knock', from: 'cccccccc' });
  await sleep(500);
  ch.close();
  assert.deepEqual(heard, [], 'nothing listens for its tabs any more');
});

delete globalThis.RTCPeerConnection;
// ── a fourth review ────────────────────────────────────────────────────────

await test('fourth review #5 a program over 32 MB still arrives when the inviter\'s first hello is lost', async () => {
  const bytes = new Uint8Array((33 << 20) + 5);
  bytes.set(randomBytes(4096), 1000);
  const file = { name: 'big.elf', size: bytes.length, hash: createHash('sha256').update(bytes).digest('hex') };
  const ana = member('aaaaaaaa', 'Ana', { file, bytes });
  ana.g.create();
  ana.g.local([ana.replica.set('fn:0x1149', 'adder')]);
  const ben = member('bbbbbbbb', 'Ben');
  const [a, b] = linkPair();
  const send = a.send;
  let first = true;
  a.send = function sendButLoseTheFirst(text) {
    if (first) {
      first = false;
      return;
    }
    send.call(this, text);
  };
  ana.g.addLink(a);
  ben.g.addLink(b, { joining: true });
  assert.ok(await until(() => ben.p.bytes?.length === bytes.length, 30000), 'Ben received the whole program');
  assert.equal(ben.p.file.hash, file.hash);
  assert.ok(await until(() => ben.replica.value('fn:0x1149') === 'adder', 5000), 'and the registers');
  ana.g.leave();
  ben.g.leave();
});

await test('fourth review #9 a page whose clock races ahead is cut off and the student told; the others\' writes still count', async () => {
  const WINDOW = 2 ** 24;
  const ana = member('aaaaaaaa', 'Ana', { file: FILE, bytes: FILE_BYTES });
  ana.g.create();
  const ben = member('bbbbbbbb', 'Ben', { file: FILE, bytes: FILE_BYTES });
  invite(ana, ben);
  await settle(150);
  const [a, b] = linkPair();
  b.onmessage = () => {};
  ana.g.addLink(a);
  const hello = { t: 'hello', proto: W.PROTOCOL, build: BUILD, peer: 'cccccccc', name: 'Cy', color: null, sid: null, file: FILE };
  b.send(JSON.stringify(hello));
  await settle(60);
  let c = ana.replica.counter;
  for (let m = 0; m < 20; m++) {
    const ops = [];
    for (let i = 0; i < 1000; i++) {
      c += WINDOW - 8;
      ops.push({ k: `comment:${MAIN}:0x${(0x1000 + m * 1000 + i).toString(16)}`, v: 'x', c: [c, 'cccccccc'], b: [c, 'cccccccc'] });
    }
    b.send(JSON.stringify({ t: 'ops', ops }));
  }
  await settle(300);
  assert.ok(ana.replica.counter < 2 * WINDOW, `Ana's clock stays near where it was (${ana.replica.counter})`);
  assert.ok(ben.replica.counter < 2 * WINDOW, `and so does Ben's (${ben.replica.counter})`);
  assert.ok(ana.p.events.some(([k, n]) => k === 'misbehaved' && n === 'Cy'), 'Ana is told');
  assert.equal(a.closed, true, 'and her page stops linking to Cy');
  ana.g.local([ana.replica.set(`fn:${MAIN}`, 'still_counts')]);
  await settle(150);
  assert.equal(ben.replica.value(`fn:${MAIN}`), 'still_counts', 'an honest write still reaches the others');
  const [a2, b2] = linkPair();
  b2.onmessage = () => {};
  ana.g.addLink(a2);
  b2.send(JSON.stringify(hello));
  await settle(60);
  assert.equal(a2.closed, true, 'Cy is not linked to again');
  ana.g.leave();
  ben.g.leave();
});

const failed = results.filter(([ok]) => !ok);
for (const [ok, name, why] of results) console.log(`${ok ? 'ok  ' : 'FAIL'} ${name}${ok ? '' : ` — ${why}`}`);
if (failed.length) {
  console.log(`DECOMPILE2 COLLAB CASES FAIL — ${failed.length} of ${results.length}`);
  process.exit(1);
}
console.log(`DECOMPILE2 COLLAB CASES OK — ${results.length} cases`);
process.exit(0);
