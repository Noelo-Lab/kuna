// Validate the generated public evidence bundle without duplicating its logic.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';

const root = new URL('../dist/', import.meta.url);
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

const serialized = JSON.stringify(data);
assert.doesNotMatch(serialized, /\/(Users|home)\//, 'local paths leaked into public data');
assert.doesNotMatch(serialized, /noreply@|simplelogin|@anthropic|@openai/i, 'author identity leaked into public data');
assert.match(html, /id="phase-heatmap"/);
assert.match(html, /id="catalog-search"/);
assert.match(app, /fetch\('\.\/data\.json'\)/);

console.log(`dev-viz OK: ${data.commits.length} commits, ${data.options.length} options, ${data.records.length} records`);
