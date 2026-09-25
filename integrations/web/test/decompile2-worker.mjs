// decompile2-worker.mjs — the study view's RPC surface through the real module
// Worker: `inspect`, `read`, `xrefs` and per-call `--assert` directives.
//
// Skips (exit 0, with a message) while the built wasm predates those commands,
// so it is green before the engine side lands and meaningful after it.
//
// Usage:  integrations/web/build.sh && node integrations/web/test/decompile2-worker.mjs
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { requireDist, serveStatic, workerClient, fixture } from './worker-harness.mjs';

requireDist();
const server = await serveStatic();
const client = workerClient(server.base);
const elf = new Uint8Array(await readFile(fixture('sample.elf')));

/** Every line rebuilt from its tokens (gaps as spaces) equals the code line. */
function assertTokensRebuild(fn) {
  const lines = fn.code.split('\n');
  const byLine = new Map();
  for (const t of fn.tokens) {
    assert.ok(t.line >= 1 && t.line <= lines.length, `token line ${t.line} in range`);
    if (!byLine.has(t.line)) byLine.set(t.line, []);
    byLine.get(t.line).push(t);
  }
  lines.forEach((text, i) => {
    const toks = (byLine.get(i + 1) || []).sort((a, b) => a.col - b.col);
    let rebuilt = '';
    for (const t of toks) {
      assert.ok(t.col >= rebuilt.length, `tokens on line ${i + 1} do not overlap`);
      rebuilt += ' '.repeat(t.col - rebuilt.length) + t.text;
    }
    assert.equal(rebuilt.trimEnd(), text.trimEnd(), `line ${i + 1} rebuilds from its tokens`);
  });
}

function unknownCommand(error, name) {
  return new RegExp(`unknown command: "?${name}`).test(error?.message || '');
}

const checks = [];
try {
  await client.ready();
  const inventory = await client.load(elf, { fileName: 'sample.elf' });
  let main;
  try {
    main = await client.inspect('main');
  } catch (error) {
    if (unknownCommand(error, 'inspect')) {
      console.log('DECOMPILE2 WORKER SKIPPED — this wasm has no `inspect` command yet; ' +
        'rebuild integrations/web/build.sh on an engine with inspect/read/--assert and rerun');
      process.exit(0);
    }
    throw error;
  }

  // list: sections + target + known types ride on the inventory.
  assert.ok(Array.isArray(inventory.sections), 'list carries sections[]');
  const text = inventory.sections.find((s) => s.name === '.text');
  assert.ok(text && text.executable === true && Number.isInteger(text.file_offset), '.text is file-backed code');
  assert.equal(typeof inventory.target?.archid, 'string', 'list carries target.archid');
  assert.ok(Array.isArray(inventory.known_types), 'list carries known_types[]');
  checks.push('list sections/target/known_types');

  // inspect == decompile, and every map points somewhere real.
  const plain = await client.decompile('main');
  const fn = main.function;
  assert.equal(fn.code, plain.functions[0].code, 'inspect main code == decompile main code');
  const lineCount = fn.code.split('\n').length;
  const insnAddrs = new Set(fn.instructions.map((i) => i.address_hex));
  for (const insn of fn.instructions) {
    assert.equal(insn.bytes.length, insn.size * 2, `${insn.address_hex} carries its bytes`);
    assert.ok(Array.isArray(insn.lines), `${insn.address_hex} has lines[]`);
    for (const line of insn.lines) assert.ok(line >= 1 && line <= lineCount, 'instruction line in range');
  }
  for (const m of fn.line_mappings) {
    for (const a of m.addresses) {
      assert.ok(insnAddrs.has('0x' + a.toString(16)), `line ${m.line_number} address 0x${a.toString(16)} is an instruction`);
    }
  }
  if (fn.tokens.length) assertTokensRebuild(fn);
  else assert.equal(typeof fn.tokens_error, 'string', 'an empty token stream says why');
  checks.push(`inspect main (${fn.instructions.length} insns, ${fn.tokens.length} tokens)`);

  const sumTo = (await client.inspect('sum_to')).function;
  if (sumTo.tokens.length) assertTokensRebuild(sumTo);
  assert.ok(sumTo.instructions.some((i) => i.mnemonic === 'RET'), 'sum_to ends in RET');
  checks.push('inspect sum_to');

  // Directives: a rename applies, a bad symbol is a rejected row with a body.
  const local = /\b(v\d+)\b/.exec(fn.code)?.[1];
  assert.ok(local, 'main has a vN local to rename');
  const renamed = await client.inspect('main', { assertions: [`name ${local} total`] });
  assert.equal(renamed.assertions[0].status, 'applied', 'rename applied');
  assert.match(renamed.function.code, /\btotal\b/, 'renamed local in the code');
  // main's local lives in RAX (8 bytes): a same-size retype applies, a narrowing one is refused.
  const retyped = await client.inspect('main', { assertions: [`type ${local} unsigned long ${local}`] });
  assert.equal(retyped.assertions[0].status, 'applied', 'pinned retype applied');
  assert.match(retyped.function.code, new RegExp(`unsigned long ${local}\\b`), 'retyped local in the code');
  const narrowed = await client.inspect('main', { assertions: [`type ${local} unsigned int ${local}`] });
  assert.equal(narrowed.assertions[0].status, 'rejected', 'a size-changing retype is rejected, with a body');
  const rejected = await client.inspect('main', { assertions: ['name v999 nope'] });
  assert.equal(rejected.assertions[0].status, 'rejected', 'unknown symbol is rejected');
  assert.ok(rejected.function.code, 'a rejected directive still returns a body');
  await assert.rejects(
    client.inspect('main', { assertions: ['name'] }),
    (error) => /--assert/.test(error.message) && error.detail?.exitCode !== 0,
    'an unparseable directive is a process error with detail',
  );
  checks.push('name/type applied, unknown symbol rejected, parse error detailed');

  // read: raw bytes, overlays, and the file offset round trip.
  const first = sumTo.instructions[0];
  const raw = await client.read(first.address_hex, first.size);
  assert.equal(raw.bytes, first.bytes, 'read == first instruction bytes');
  assert.equal(
    Buffer.from(elf.subarray(raw.file_offset, raw.file_offset + raw.size)).toString('hex'),
    raw.bytes,
    'file_offset points at the same bytes in the file',
  );
  const patched = await client.read(first.address_hex, 4, {
    assertions: [`bytes ${first.address_hex} 90909090`],
  });
  assert.equal(patched.bytes, '90909090', 'a bytes overlay is visible to read');
  checks.push('read + overlay + file_offset');

  // function rename reaches the inventory; a qualified rename reaches the project.
  const renamedList = await client.list({ assertions: [`function ${sumTo.address_hex}=summation`] });
  assert.ok(
    renamedList.functions.some((f) => f.name === 'summation' || (f.aliases || []).includes('summation')),
    'function rename reaches list',
  );
  const project = await client.project('sample.elf', { assertions: [`name main::${local} total`] });
  assert.ok(Buffer.from(project.bytes).includes('total'), 'qualified rename reaches the project export');
  checks.push('list function rename, project qualified rename');

  // xrefs: callers link to the calling instruction, callees to the call site.
  const refs = await client.xrefs('sum_to');
  assert.equal(refs.function.address_hex, sumTo.address_hex);
  const caller = refs.callers.find((c) => c.address_hex === '0x1198');
  assert.ok(caller && caller.from_hex === '0x11c2' && caller.kind === 'call', 'main calls sum_to at 0x11c2');
  const mainRefs = await client.xrefs('main');
  assert.deepEqual(mainRefs.callees.map((c) => [c.name, c.at_hex]), [['add', '0x11b5'], ['sum_to', '0x11c2'], ['printf', '0x11e1']]);
  assert.ok(mainRefs.data_refs.some((d) => d.address_hex === '0x2004' && d.at_hex === '0x11d2'), 'the format string');
  const renamedRefs = await client.xrefs('main', { assertions: [`function ${sumTo.address_hex}=summation`] });
  assert.ok(renamedRefs.callees.some((c) => c.name === 'summation'), 'a function rename reaches xrefs');
  checks.push('xrefs callers/callees/data + rename');

  // The page names a refused directive from this error: exit code, and the directive quoted.
  await assert.rejects(
    client.list({ assertions: ['bytes 0x10 zz'] }),
    (error) => error.detail?.exitCode === 1 && error.detail.stderr.includes(`--assert ${JSON.stringify('bytes 0x10 zz')}:`),
    'an unparseable directive fails the request and names itself',
  );
  checks.push('refusal error contract');

  console.log(`DECOMPILE2 WORKER OK — ${checks.join('; ')}`);
} finally {
  client.close();
  await server.close();
}
