// Validate the generated public evidence bundle without duplicating its logic.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { runInNewContext } from 'node:vm';

const root = new URL(process.argv[2] || '../dist/', import.meta.url);
const data = JSON.parse(await readFile(new URL('dev-viz/data.json', root), 'utf8'));
const html = await readFile(new URL('dev-viz/index.html', root), 'utf8');
const app = await readFile(new URL('dev-viz/app.js', root), 'utf8');

assert.equal(data.schemaVersion, 1);
assert.match(data.meta.sha, /^[0-9a-f]{40}$/);
assert.equal(data.phases.length, 10);
assert.deepEqual(data.phases.map((p) => p.id), Array.from({length: 10}, (_, i) => `P${i}`));
assert.ok(data.commits.length > 100, 'full non-merge history was not exported');
assert.ok(data.options.length > 50, 'option catalog was not exported');
assert.ok(data.records.length > 20, 'feature records were not exported');
assert.ok(data.triage.length > 10, 'triage records were not exported');
assert.ok(data.baselines.upstream[0] === data.baselines.upstream[1]);
assert.ok(data.baselines.stages[0] === data.baselines.stages[1]);
assert.ok(data.options.every((o) => /^P[0-9]$/.test(o.phase) && o.line > 0));
assert.ok(data.commits.every((c) => /^\d{4}-\d{2}-\d{2}$/.test(c.date)));
assert.ok(data.records.some((r) => r.ged?.before > r.ged?.after));
assert.ok(data.records.some((r) => r.novel));
assert.equal(new Set(data.community.changes.map((c) => c.pr)).size, data.community.changes.length);
assert.ok(data.community.changes.every((c) => c.externalPr || c.issues.length > 0));
assert.ok(data.community.changes.every((c) => data.commits.some((commit) => commit.subject.endsWith(`(#${c.pr})`))));
assert.ok(data.milestones.some((m) => m.date === '2026-08-08' && m.title.includes('optimized C')));
assert.ok(data.milestones.some((m) => m.date === '2026-08-31' && m.commit === '63a124ae'));

const {dailyRows, sourceKey} = runInNewContext(app.replace(/main\(\);\s*$/, '') + '\n({dailyRows, sourceKey})');
const day = (date, added, removed = 0, agent = false) => ({date, added, removed, agent});
const rows = dailyRows({commits: [day('2026-07-04', 10), day('2026-06-30', 3, 2), day('2026-07-01', 4), day('2026-07-04', 5, 2, true)]});
assert.deepEqual(JSON.parse(JSON.stringify(rows)), [
  {date:'2026-06-30', commits:1, agent:0, added:3, removed:2, skipped:0},
  {date:'2026-07-01', commits:1, agent:0, added:4, removed:0, skipped:0},
  {date:'2026-07-04', commits:2, agent:1, added:15, removed:2, skipped:2},
]);
assert.equal(dailyRows(data).reduce((sum, row) => sum + row.commits, 0), data.commits.length);
assert.ok(dailyRows(data).every((row) => row.commits > 0));
assert.equal(sourceKey('ghidra-upstream'), sourceKey('ghidra'));
assert.notEqual(sourceKey('kuna'), sourceKey('ghidra'));

const serialized = JSON.stringify(data);
assert.doesNotMatch(serialized, /\/(Users|home)\//, 'local paths leaked into public data');
assert.doesNotMatch(serialized, /noreply@|simplelogin|@anthropic|@openai/i, 'author identity leaked into public data');
assert.match(html, /id="phase-heatmap"/);
assert.match(html, /id="catalog-search"/);
assert.match(app, /fetch\('\.\/data\.json'\)/);

console.log(`dev-viz OK: ${data.commits.length} commits, ${data.options.length} options, ${data.records.length} records, ${data.community.changes.length} community changes`);
