// decompile2-collab-sync.mjs — the page's glue in a live session
// (collab/sync.js), case by case, through the same simulated pages as the fuzz
// test (collab-sim.mjs: a real Session, Sync and Group per page, in-memory
// links, a virtual clock). One case per defect a second review found in how a
// page keeps its Session and the registers in step (the number is the
// review's): a guest that has the program open when it joins, leaving and
// joining again, an edit made while another person's change waits to be
// applied, an inviter that goes away in the middle of a join, a tab of the
// same browser keeping a guest's own changes apart, the shared order; then a
// third review's: two joiners bringing the same field, a slow open of the
// received program, a join that fails while the program opens; and a fourth
// review's: a guest's variable changes made at another decompiler effort, a
// guest's earlier change the others' pages refuse. Each
// case also checks that no page sent a change its student did not make, and
// that the pages agree once settled. Needs no build.
//   node integrations/web/test/decompile2-collab-sync.mjs
import assert from 'node:assert/strict';
import { Sim, FN, PROGRAM } from './collab-sim.mjs';
import { birthOrder, recordKeyOf } from '../decompile2/collab/replica.js';

const results = [];
async function test(name, fn) {
  try {
    await fn();
    results.push([true, name]);
  } catch (e) {
    results.push([false, name, e.message.split('\n')[0]]);
  }
}

const ANA = 'anaaaaaa';
const BEN = 'benbbbbb';
const CY = 'cycccccc';
const HANA = 'hanaaaaa';
const settled = (sim) => {
  const problems = sim.problems();
  assert.deepEqual(problems, [], problems[0]);
};
const shows = (page, text) => page.directives().some((d) => d.includes(text));

await test('#1 a guest that has the program open with changes of its own joins: the inviter\'s changes stay on every page, the guest\'s own join the session', async () => {
  const sim = new Sim(1);
  const ana = sim.page(ANA, { own: (s) => { s.setFunctionName(FN, 'anas_fn'); s.setComment(FN, '0x1104', 'ana note'); s.setVar(FN, 'v2', { name: 'anas_v2' }); } });
  const ben = sim.page(BEN, { own: (s) => { s.setFunctionName(FN, 'bens_fn'); s.setVar(FN, 'v1', { name: 'bens_v1' }); } });
  ana.start();
  await sim.run(50);
  ben.joinVia(ana);
  await sim.run(2000);
  assert.equal(ben.sync.phase, 'shared');
  for (const p of [ana, ben]) {
    assert.ok(shows(p, 'function 0x1100=anas_fn'), `${p.name} keeps Ana's function name`);
    assert.ok(shows(p, 'ana note'), `${p.name} keeps Ana's note`);
    assert.ok(shows(p, 'anas_v2'), `${p.name} keeps Ana's rename`);
    assert.ok(shows(p, 'bens_v1'), `${p.name} has Ben's own rename`);
    assert.ok(!shows(p, 'bens_fn'), `the session's name for ${FN} wins on ${p.name}'s page`);
  }
  assert.equal(ben.replacedCopies.length, 1, 'Ben is offered his own changes as they were');
  assert.ok(ben.replacedCopies[0].allAssertions((a) => a).includes('function 0x1100=bens_fn'));
  await sim.run(60000);
  settled(sim);
});

await test('#1 the guest edits while it is still joining: that edit is the newest', async () => {
  const sim = new Sim(2);
  const ana = sim.page(ANA, { own: (s) => s.setVar(FN, 'v1', { name: 'anas' }) });
  const ben = sim.page(BEN, {});
  ana.start();
  await sim.run(50);
  ben.joinVia(ana);
  await sim.run(1);
  ben.edit((s) => s.setVar(FN, 'v1', { name: 'bens_now' }));
  await sim.run(3000);
  for (const p of [ana, ben]) assert.ok(shows(p, 'bens_now'), `${p.name} has Ben's newest name`);
  await sim.run(60000);
  settled(sim);
});

await test('#2 a guest leaves, both go on alone, and it joins the same session again: nothing is erased, both sides\' changes meet', async () => {
  const sim = new Sim(3);
  const ana = sim.page(ANA, { own: (s) => { s.setFunctionName(FN, 'shared_fn'); s.setComment(FN, '0x1104', 'shared note'); } });
  const ben = sim.page(BEN, { program: false });
  ana.start();
  await sim.run(50);
  ben.joinVia(ana);
  await sim.run(3000);
  assert.ok(shows(ben, 'shared note'));
  ben.leave();
  await sim.run(500);
  ana.edit((s) => s.setVar(FN, 'v2', { name: 'while_away' }));
  ben.edit((s) => s.setVar(FN, 'v3', { name: 'bens_alone' }));
  ben.edit((s) => s.setComment(FN, '0x1104', null));
  await sim.run(500);
  ben.joinVia(ana);
  await sim.run(3000);
  assert.equal(ben.sync.phase, 'shared');
  for (const p of [ana, ben]) {
    assert.ok(shows(p, 'shared_fn'), `${p.name} keeps the session's function name`);
    assert.ok(shows(p, 'while_away'), `${p.name} has what Ana did meanwhile`);
    assert.ok(shows(p, 'bens_alone'), `${p.name} has what Ben did meanwhile`);
    assert.ok(!shows(p, 'shared note'), `${p.name}: Ben's removal of the note after leaving is the newest change`);
  }
  await sim.run(60000);
  settled(sim);
});

await test('#2 a guest whose own changes were kept apart leaves (they come back) and joins again: the session is not erased', async () => {
  const sim = new Sim(4);
  const ana = sim.page(ANA, { own: (s) => { s.setFunctionName(FN, 'shared_fn'); s.setComment(FN, '0x1104', 'shared note'); s.setVar(FN, 'v2', { name: 'anas_v2' }); } });
  const ben = sim.page(BEN, { program: false, own: (s) => s.setVar(FN, 'v1', { name: 'bens_own' }) });
  ana.start();
  await sim.run(50);
  ben.joinVia(ana);
  await sim.run(3000);
  assert.equal(ben.slot, 'shared', 'the session is kept apart from Ben\'s own changes');
  assert.ok(!shows(ben, 'bens_own'));
  ben.leave();
  await sim.run(500);
  assert.ok(shows(ben, 'bens_own') && !shows(ben, 'shared note'), 'Ben\'s own changes are back');
  ben.joinVia(ana);
  await sim.run(3000);
  for (const p of [ana, ben]) {
    assert.ok(shows(p, 'shared_fn') && shows(p, 'shared note') && shows(p, 'anas_v2'), `${p.name} keeps the whole session`);
    assert.ok(shows(p, 'bens_own'), `${p.name} has Ben's own rename, which the session never held`);
  }
  await sim.run(60000);
  settled(sim);
});

await test('#3 an edit made while another person\'s change waits to be applied does not take that change back', async () => {
  const sim = new Sim(5);
  const ana = sim.page(ANA, {});
  const ben = sim.page(BEN, { program: false });
  ana.start();
  await sim.run(50);
  ben.joinVia(ana);
  await sim.run(3000);
  ana.edit((s) => s.setFunctionName(FN, 'anas_name'));
  for (let i = 0; i < 400 && ben.sync.replica.value(`fn:${FN}`) !== 'anas_name'; i++) await sim.run(1);
  assert.equal(ben.sync.replica.value(`fn:${FN}`), 'anas_name', 'Ben\'s page has Ana\'s change');
  assert.ok(!shows(ben, 'anas_name') && ben.sync.pendingTimer, 'and it is still waiting to be applied');
  ben.edit((s) => s.setVar(FN, 'v1', { name: 'bens' }));
  await sim.run(3000);
  for (const p of [ana, ben]) assert.ok(shows(p, 'anas_name') && shows(p, 'bens'), `${p.name} has both changes`);
  await sim.run(60000);
  settled(sim);
});

await test('#4 the inviter goes away after the welcome, before the program arrives: the join fails and stops holding', async () => {
  const sim = new Sim(6);
  const ana = sim.page(ANA, {});
  const ben = sim.page(BEN, { program: false });
  ana.start();
  await sim.run(50);
  const [a, b] = sim.net.pair(ANA, BEN);
  a.sendBinary = () => {};
  sim.act(ben, new Set(), () => {
    ana.sync.addLink(a);
    ben.sync.beginJoin({ build: 'b'.repeat(64), name: 'Ben', connect: sim.net.connect(BEN), link: b });
  });
  await sim.run(1000);
  assert.equal(ben.sync.phase, 'joining', 'Ben was welcomed and waits for the program');
  sim.net.fail(a.pair);
  await sim.run(8000);
  assert.equal(ben.sync.phase, 'solo');
  assert.deepEqual(ben.joinFailures, ['lost']);
});

await test('#4 the inviter goes away after the welcome, before its registers have all arrived: the join fails', async () => {
  const sim = new Sim(11);
  const ana = sim.page(ANA, { own: (s) => s.setFunctionName(FN, 'anas') });
  const ben = sim.page(BEN, { own: (s) => s.setVar(FN, 'v1', { name: 'bens' }) });
  ana.start();
  await sim.run(50);
  const [a, b] = sim.net.pair(ANA, BEN);
  const send = a.send;
  a.send = (text) => { if (!text.startsWith('{"t":"snap"')) send(text); };
  sim.act(ben, new Set(), () => {
    ana.sync.addLink(a);
    ben.sync.beginJoin({ build: 'b'.repeat(64), name: 'Ben', connect: sim.net.connect(BEN), link: b });
  });
  await sim.run(1000);
  assert.equal(ben.sync.phase, 'joining', 'Ben was welcomed and waits for Ana\'s registers');
  sim.net.fail(a.pair);
  await sim.run(8000);
  assert.equal(ben.sync.phase, 'solo');
  assert.deepEqual(ben.joinFailures, ['sponsor']);
  ben.edit((s) => s.setVar(FN, 'v2', { name: 'later' }));
  assert.ok(ben.directives().includes('name 0x1100::v2 later'), 'Ben goes on alone with his own changes');
});

await test('#4 a join that hears nothing more after the welcome fails after a while', async () => {
  const sim = new Sim(7);
  const ana = sim.page(ANA, {});
  const ben = sim.page(BEN, { program: false });
  ana.start();
  await sim.run(50);
  const [a, b] = sim.net.pair(ANA, BEN);
  a.sendBinary = () => {};
  sim.act(ben, new Set(), () => {
    ana.sync.addLink(a);
    ben.sync.beginJoin({ build: 'b'.repeat(64), name: 'Ben', connect: sim.net.connect(BEN), link: b });
  });
  await sim.run(1000);
  assert.equal(ben.sync.phase, 'joining');
  await sim.run(25000);
  assert.equal(ben.sync.phase, 'solo');
  assert.deepEqual(ben.joinFailures, ['stalled']);
});

await test('#11 a tab that joins a tab of its own browser keeps the student\'s own changes apart when that tab does', async () => {
  const sim = new Sim(8);
  const browser = { store: new Map(), sharedStore: new Map() };
  const hana = sim.page(HANA, { own: (s) => s.setFunctionName(FN, 'hanas_fn') });
  const ana = sim.page(ANA, { program: false, browser, own: (s) => s.setVar(FN, 'v1', { name: 'students_own' }) });
  const ownBefore = browser.store.get(PROGRAM.hash);
  const cy = sim.page(CY, { program: false, browser });
  hana.start();
  await sim.run(50);
  ana.joinVia(hana);
  await sim.run(3000);
  assert.equal(ana.slot, 'shared');
  cy.joinVia(ana);
  await sim.run(3000);
  assert.equal(cy.slot, 'shared', 'the joining tab keeps the session apart too');
  cy.edit((s) => s.setVar(FN, 'v3', { name: 'cys' }));
  await sim.run(1000);
  assert.equal(browser.store.get(PROGRAM.hash), ownBefore, 'the student\'s own stored changes are untouched');
  assert.match(browser.sharedStore.get(PROGRAM.hash), /cys/);
  await sim.run(60000);
  settled(sim);
});

await test('#11 two tabs of one browser, the inviter keeping the session as its own: no false setting apart', async () => {
  const sim = new Sim(9);
  const browser = { store: new Map(), sharedStore: new Map() };
  const ana = sim.page(ANA, { browser, own: (s) => s.setVar(FN, 'v1', { name: 'total' }) });
  const ben = sim.page(BEN, { program: false, browser });
  ana.start();
  await sim.run(50);
  ben.joinVia(ana);
  await sim.run(3000);
  assert.equal(ben.slot, 'own');
  assert.equal(ben.sync.aside, null);
  settled(sim);
});

await test('#15 the order the page sends in a session is birthOrder over its registers', async () => {
  const sim = new Sim(10);
  const ana = sim.page(ANA, { own: (s) => { s.setTypedef('pb', 'struct pb { int x; } pb;'); s.addRaw('readonly 0x2000+8'); } });
  const ben = sim.page(BEN, { own: (s) => { s.setTypedef('pa', 'struct pa { int y; } pa;'); s.addRaw('volatile 0x3000+4'); } });
  ana.start();
  await sim.run(50);
  ben.joinVia(ana);
  await sim.run(3000);
  for (const p of [ana, ben]) {
    const order = birthOrder(p.sync.replica);
    for (const key of p.sync.replica.regs.keys()) assert.deepEqual(p.sync.orderOf(recordKeyOf(key)), order(recordKeyOf(key)));
    assert.equal(p.session.orderOf, p.sync.orderOf, 'the Session sorts with it');
  }
  settled(sim);
});

// ── a third review ─────────────────────────────────────────────────────────

await test('third review #1 a second joiner\'s earlier value does not replace the first joiner\'s', async () => {
  const sim = new Sim(21);
  const ana = sim.page(ANA, {});
  const ben = sim.page(BEN, { own: (s) => s.setFunctionName(FN, 'bens_name') });
  const cy = sim.page(CY, { own: (s) => s.setFunctionName(FN, 'cys_stale') });
  ana.start();
  await sim.run(50);
  ben.joinVia(ana);
  await sim.run(3000);
  assert.ok(shows(ana, 'function 0x1100=bens_name'), 'Ben brought a name the session did not hold');
  cy.joinVia(ana);
  await sim.run(3000);
  for (const p of [ana, ben, cy]) assert.ok(shows(p, 'function 0x1100=bens_name'), `${p.name} keeps Ben's name`);
  await sim.run(60000);
  settled(sim);
});

await test('third review #3 a slow open of the received program is not failed as stalled when another person edits meanwhile', async () => {
  const sim = new Sim(22);
  const ana = sim.page(ANA, {});
  const ben = sim.page(BEN, { program: false, openMs: 45000 });
  ana.start();
  await sim.run(50);
  ben.joinVia(ana);
  for (let i = 0; i < 400 && !ben.program; i++) await sim.run(5);
  assert.ok(ben.program && ben.sync.phase === 'joining', 'Ben is opening the program');
  ana.edit((s) => s.setVar(FN, 'v1', { name: 'while_it_opens' }));
  await sim.run(50000);
  assert.deepEqual(ben.joinFailures, []);
  assert.equal(ben.sync.phase, 'shared');
  assert.ok(shows(ben, 'name 0x1100::v1 while_it_opens'));
  await sim.run(60000);
  settled(sim);
});

await test('third review #4 a join that fails while the received program opens gives the student\'s own changes back', async () => {
  const sim = new Sim(23);
  const ana = sim.page(ANA, { own: (s) => s.setFunctionName(FN, 'session_name') });
  const ben = sim.page(BEN, { program: false, openMs: 8000, own: (s) => s.setVar(FN, 'v1', { name: 'bens_own' }) });
  ana.start();
  await sim.run(50);
  ben.joinVia(ana);
  for (let i = 0; i < 400 && !ben.program; i++) await sim.run(5);
  assert.ok(ben.program && ben.sync.phase === 'joining', 'Ben is opening the program with the session\'s changes');
  for (const l of [...sim.net.links]) sim.net.fail(l);
  await sim.run(10000);
  assert.deepEqual(ben.joinFailures, ['sponsor']);
  assert.equal(ben.slot, 'own', 'Ben saves into his own slot again');
  assert.ok(shows(ben, 'bens_own') && !shows(ben, 'session_name'), 'and sees his own changes');
  assert.equal(ben.session.orderOf, null);
  settled(sim);
});

// ── a fourth review ────────────────────────────────────────────────────────

await test('fourth review #3 a guest\'s variable changes made at another decompiler effort stay out of the session', async () => {
  const sim = new Sim(31);
  const ana = sim.page(ANA, { own: (s) => s.setVar(FN, 'v2', { name: 'anas_v2' }) });
  const ben = sim.page(BEN, {
    own: (s) => {
      s.setFunctionName(FN, 'bens_fn');
      s.setComment(FN, '0x1104', 'bens note');
      s.setVar(FN, 'v1', { name: 'bens_fast_v1' });
      s.setVar(FN, 'v2', { type: 'long' });
    },
  });
  ben.mode = 'fast';
  ana.start();
  await sim.run(50);
  ben.joinVia(ana);
  await sim.run(1);
  ben.edit((s) => s.setVar(FN, 'v3', { name: 'renamed_while_joining' }));
  await sim.run(3000);
  assert.equal(ben.sync.phase, 'shared');
  assert.equal(ben.mode, 'auto', 'Ben now runs the session\'s effort');
  for (const p of [ana, ben]) {
    assert.ok(shows(p, 'function 0x1100=bens_fn') && shows(p, 'bens note'), `${p.name} has Ben's function name and note (they do not depend on the effort)`);
    assert.ok(shows(p, 'anas_v2'), `${p.name} keeps Ana's rename`);
    assert.ok(!shows(p, 'bens_fast_v1') && !shows(p, 'renamed_while_joining') && !shows(p, 'long'),
      `${p.name} holds none of the variable changes Ben made in Fast (at Automatic they would change other variables)`);
  }
  assert.equal(ben.merges.length, 1, 'Ben is told');
  assert.equal(ben.merges[0].apart, 3, 'three of his variables stayed out');
  assert.equal(ben.merges[0].mode, 'auto');
  const copy = ben.merges[0].copy.allAssertions((a) => a);
  assert.ok(copy.includes('name 0x1100::v1 bens_fast_v1') && copy.includes('name 0x1100::v3 renamed_while_joining'), 'and offered his own changes as they were');
  await sim.run(60000);
  settled(sim);
});

await test('fourth review #3 at the same effort a guest\'s variable changes join the session as before', async () => {
  const sim = new Sim(32);
  const ana = sim.page(ANA, {});
  const ben = sim.page(BEN, { own: (s) => s.setVar(FN, 'v1', { name: 'bens_v1' }) });
  ben.mode = 'fast';
  ana.start();
  await sim.run(50);
  ana.setMode('fast');
  await sim.run(50);
  ben.joinVia(ana);
  await sim.run(3000);
  for (const p of [ana, ben]) assert.ok(shows(p, 'bens_v1'), `${p.name} has Ben's rename`);
  assert.deepEqual(ben.merges, [], 'nothing to tell Ben');
  await sim.run(60000);
  settled(sim);
});

await test('fourth review #6 a guest\'s earlier change the others\' pages refuse stays on its page, it is told, and it is tried again', async () => {
  const sim = new Sim(33);
  const ana = sim.page(ANA, {});
  const ben = sim.page(BEN, { own: (s) => { s.addRaw('readonly @regions.txt'); s.setFunctionName(FN, 'bens_fn'); } });
  ana.start();
  await sim.run(50);
  ben.joinVia(ana);
  await sim.run(3000);
  assert.equal(ben.sync.phase, 'shared');
  assert.ok(shows(ana, 'function 0x1100=bens_fn'), 'Ben\'s other change joined the session');
  assert.ok(shows(ben, 'readonly @regions.txt') && !shows(ana, '@regions.txt'), 'the one that reads a file stays on Ben\'s page');
  assert.ok(ben.toasts.includes('One of your changes stays on this page only.'), 'and Ben is told');
  const key = [...ben.session.records].find(([, r]) => r.kind === 'raw')[0];
  assert.ok(!ben.sync.base.has(key), 'it is not taken as shared, so the next change tries it again');
  ben.edit((s) => s.replaceWith(key, 'readonly 0x2000+8'));
  await sim.run(2000);
  assert.ok(shows(ana, 'readonly 0x2000+8'), 'made valid, it is shared');
  await sim.run(60000);
  settled(sim);
});

const failed = results.filter(([ok]) => !ok);
for (const [ok, name, why] of results) console.log(`${ok ? 'ok  ' : 'FAIL'} ${name}${ok ? '' : ` — ${why}`}`);
if (failed.length) {
  console.log(`DECOMPILE2 COLLAB SYNC FAIL — ${failed.length} of ${results.length}`);
  process.exit(1);
}
console.log(`DECOMPILE2 COLLAB SYNC OK — ${results.length} cases`);
