// decompile2-collab-fuzz.mjs — random live sessions through the page's real
// glue (collab/sync.js with group.js and a real Session per page, see
// collab-sim.mjs), on a virtual clock: 3 to 5 pages, some with the program
// open and changes of their own, some that must receive it; random edits of
// every kind, joins (and joins again after leaving), leaves, undo and redo,
// decompiler-effort changes (also by pages working alone, which join later), links that fail with edits in flight, and pairs
// of pages that cannot link directly, links that lose the first message one
// side sends; every action followed by a random wait,
// so edits land inside the others' 16 ms apply batch, during joins and while a
// program is being opened. Once everything has settled it checks that:
//   - pages linked to each other hold the same registers and send the engine
//     the same directives in the same order;
//   - every page's Session is exactly what its registers make (besides the
//     changes that stay on it), and no join is left hanging;
//   - no page ever sent a write for a field its student did not change in
//     that action (or, when joining, ever), except a joiner's earlier fields
//     written with the oldest clock, which cannot replace anyone's write, and
//     those only where the registers held nothing; a joiner brings no
//     variable change made at another decompiler effort than the session's;
//   - a change the others' pages refuse stays on its page, and its student is
//     told (also for a joiner's earlier changes); no page takes another for
//     one whose clock races ahead;
//   - adding a directive always adds one (never replaces another of the
//     page's own); applying the others' changes never clears what the engine
//     said of a record they did not change; a page out of any session never
//     keeps saving into the shared slot or ordering by a session's births.
// Seeded; `--runs N` (default 400) and `--seed S` (default 1) pick the runs,
// `--verbose` prints each failing run's problems. Needs no build.
//   node integrations/web/test/decompile2-collab-fuzz.mjs [--runs 2000] [--seed 1]
import { Sim, FN } from './collab-sim.mjs';

const arg = (name, fallback) => {
  const i = process.argv.indexOf(name);
  return i >= 0 ? Number(process.argv[i + 1]) : fallback;
};
const RUNS = arg('--runs', 400);
const SEED = arg('--seed', 1);
const VERBOSE = process.argv.includes('--verbose');
const TRACE = process.argv.includes('--trace');

const IDS = ['p0aaaaaa', 'p1bbbbbb', 'p2cccccc', 'p3dddddd', 'p4eeeeee'];
const FNS = ['0x1000', FN, '0x1200'];
const SYMS = ['v1', 'v2', 'v3'];
const NAMES = ['a', 'b', 'count', 'total', 'idx'];
const TYPES = ['int', 'long', 'char *', 'unsigned int'];
const RAWS = ['readonly 0x2000+8', 'volatile 0x3000+4', 'readonly 0x2100+4'];
const MODES = ['auto', 'fast', 'reliable', 'aggressive'];
/** A directive that reads a file: the others' pages refuse it, so it stays on the page that made it. */
const PAGE_ONLY = 'readonly @regions.txt';

/** One random edit of any kind (now and then one the others' pages refuse); 'addRaw' when it added a directive. */
function mutate(rng, s) {
  const fn = rng.pick(FNS);
  switch (rng.int(9)) {
    case 0: s.setVar(fn, rng.pick(SYMS), { name: rng.chance(0.15) ? null : rng.pick(NAMES) }); break;
    case 1: s.setVar(fn, rng.pick(SYMS), { type: rng.chance(0.15) ? null : rng.pick(TYPES) }); break;
    case 2: s.setFunctionName(fn, rng.chance(0.2) ? null : `fn_${rng.pick(NAMES)}`); break;
    case 3: s.setComment(fn, rng.pick(['0x1104', '0x1108']), rng.chance(0.2) ? null : `note ${rng.int(5)}`); break;
    case 4: {
      const addr = rng.pick(['0x4000', '0x4010']);
      if (rng.chance(0.25)) s.setData(addr, null, null);
      else s.setData(addr, rng.pick(TYPES), `g_${rng.pick(NAMES)}`);
      break;
    }
    case 5: s.setByte(`0x${(0x1000 + rng.int(6)).toString(16)}`, rng.int(3) * 0x48, 0); break;
    case 6: {
      const tag = rng.pick(['pa', 'pb']);
      s.setTypedef(tag, rng.chance(0.2) ? null : `struct ${tag} { int x${rng.int(3)}; } ${tag};`);
      break;
    }
    case 7: {
      const raws = [...s.records].filter(([, r]) => r.kind === 'raw').map(([k]) => k);
      if (raws.length && rng.chance(0.4)) s.remove(rng.pick(raws));
      else {
        s.addRaw(rng.chance(0.1) ? PAGE_ONLY : rng.pick(RAWS));
        return 'addRaw';
      }
      break;
    }
    default: s.setProto(fn, rng.chance(0.2) ? null : `int f(int a${rng.int(3)})`); break;
  }
}

async function runOne(seed) {
  const sim = new Sim(seed, { latency: [1, 40], noticeMs: [100, 6000], loseFirst: 0.15 });
  const { rng } = sim;
  const n = 3 + rng.int(3);
  const pages = IDS.slice(0, n).map((id, i) => {
    const program = i === 0 || rng.chance(0.6);
    const edits = rng.chance(0.5) ? 1 + rng.int(4) : 0;
    const page = sim.page(id, { program, own: edits ? (s) => { for (let e = 0; e < edits; e++) mutate(rng, s); } : null });
    if (i && rng.chance(0.3)) page.mode = rng.pick(MODES);
    return page;
  });
  const [host] = pages;
  for (let i = 1; i < n; i++) for (let j = i + 1; j < n; j++) if (rng.chance(0.2)) sim.net.block(IDS[i], IDS[j]);
  host.start();
  await sim.run(rng.int(200));
  const steps = 30 + rng.int(40);
  const log = (...a) => { if (TRACE) console.log(`${sim.clock.now()} ms`, ...a); };
  for (let step = 0; step < steps; step++) {
    const r = rng();
    const shared = pages.filter((p) => p.sync.shared);
    if (r < 0.42) {
      const p = rng.pick(pages.filter((x) => x.program));
      const raws = () => [...p.session.records.values()].filter((x) => x.kind === 'raw').length;
      const before = raws();
      let what = null;
      const keys = p.edit((s) => { what = mutate(rng, s); });
      if (what === 'addRaw' && raws() !== before + 1) sim.violations.push(`${p.id} added a directive but holds ${raws()} (had ${before})`);
      log(p.id, p.sync.phase, 'edit', [...keys].join(' '));
    } else if (r < 0.56) {
      const solo = pages.filter((p) => p.sync.phase === 'solo');
      if (solo.length && shared.length) {
        const [p, h] = [rng.pick(solo), rng.pick(shared)];
        log(p.id, 'joins via', h.id, 'program', !!p.program);
        p.joinVia(h);
      }
    } else if (r < 0.61) {
      const guests = shared.filter((p) => p !== host);
      if (guests.length) {
        const p = rng.pick(guests);
        log(p.id, 'leaves');
        p.leave();
      }
    } else if (r < 0.66) {
      const live = [...sim.net.links].filter((l) => !l.dead);
      if (live.length) sim.net.fail(rng.pick(live));
    } else if (r < 0.76) {
      if (shared.length) rng.pick(shared).undo();
    } else if (r < 0.81) {
      if (shared.length) rng.pick(shared).redo();
    } else if (r < 0.84) {
      const can = pages.filter((p) => p.sync.phase !== 'joining');
      if (can.length) rng.pick(can).setMode(rng.pick(MODES));
    } else {
      await sim.run(rng.int(400));
    }
    await sim.run(rng.int(rng.chance(0.3) ? 20 : 150));
  }
  await sim.run(120000);
  const problems = sim.problems();
  for (const [k, v] of Object.entries(sim.stats)) totals[k] = (totals[k] || 0) + v;
  return problems;
}

const totals = {};

const t0 = Date.now();
let failed = 0;
for (let i = 0; i < RUNS; i++) {
  const seed = SEED + i;
  let problems;
  try {
    problems = await runOne(seed);
  } catch (e) {
    problems = [`threw: ${e.stack}`];
  }
  if (problems.length) {
    failed++;
    console.log(`FAIL seed ${seed}: ${problems[0]}${problems.length > 1 ? ` (+${problems.length - 1} more)` : ''}`);
    if (VERBOSE) for (const p of problems.slice(1, 8)) console.log(`     ${p}`);
    if (failed >= 10) break;
  }
}
const secs = ((Date.now() - t0) / 1000).toFixed(1);
if (VERBOSE) console.log('totals', JSON.stringify(totals));
if (failed) {
  console.log(`DECOMPILE2 COLLAB FUZZ FAIL — ${failed} failing run(s) of seeds ${SEED}..${SEED + RUNS - 1} (${secs} s)`);
  process.exit(1);
}
console.log(`DECOMPILE2 COLLAB FUZZ OK — ${RUNS} runs, seeds ${SEED}..${SEED + RUNS - 1}, 3-5 pages each: registers, directives and Sessions agree, and no page sent a change its student did not make (${secs} s)`);
