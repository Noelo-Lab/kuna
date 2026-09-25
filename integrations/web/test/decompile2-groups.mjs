// decompile2-groups.mjs — which sidebar group a function lands in, from the
// source tree with no build: the student's program first (with main on top),
// the startup and runtime helpers every compiler adds, and imported functions.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { groupOf, groupFunctions, firstFunction } from '../decompile2/groups.js';

const fn = (name, address_hex, kind = 'func') => ({ name, address_hex, kind });
const checks = [];

// ── the sample program's real inventory ────────────────────────────────────
const list = JSON.parse(readFileSync(new URL('./fixtures/list-sample.json', import.meta.url), 'utf8'));
const g = groupFunctions(list.functions);
assert.deepEqual(g.program.map((f) => f.name), ['main', 'add', 'sum_to'], 'main first, then by address');
assert.deepEqual(g.startup.map((f) => f.name), ['_init', '_start', 'deregister_tm_clones', 'register_tm_clones', '__do_global_dtors_aux', 'frame_dummy', '_fini']);
assert.deepEqual(g.imports.map((f) => f.name), ['sub_1020', '__cxa_finalize', 'printf'], 'the plt and thunk kinds');
assert.equal(firstFunction(g).name, 'main', 'the page opens main, not _init');
checks.push('sample.elf groups + main first');

// ── the startup list and its patterns ──────────────────────────────────────
for (const name of ['_init', '_start', '_fini', 'deregister_tm_clones', 'register_tm_clones', '__do_global_dtors_aux',
  'frame_dummy', '__libc_csu_init', '__libc_csu_fini', '_dl_relocate_static_pie', '__x86.get_pc_thunk.bx',
  'mainCRTStartup', 'WinMainCRTStartup', 'wmainCRTStartup', '__security_init_cookie', '__scrt_common_main_seh',
  '_mainCRTStartup', '__mainCRTStartup']) {
  assert.equal(groupOf(fn(name, '0x10')), 'startup', `${name} is startup code`);
}
for (const name of ['main', 'initialize', 'start_server', 'frame_dummy2', 'my__scrt_thing', 'x86_get_pc', 'sub_401000', 'FUN_00401000']) {
  assert.equal(groupOf(fn(name, '0x10')), 'program', `${name} stays with the program`);
}
assert.equal(groupOf(fn('_start', '0x10', 'thunk')), 'imports', 'the kind decides imports before the name');
assert.equal(groupOf(fn('puts', '0x10', 'plt')), 'imports');
checks.push('startup names and patterns; unknown names stay in the program');

// ── ordering and the first function ────────────────────────────────────────
const stripped = groupFunctions([fn('sub_2000', '0x2000'), fn('_start', '0x1000'), fn('sub_1800', '0x1800'), fn('puts', '0x900', 'plt')]);
assert.deepEqual(stripped.program.map((f) => f.name), ['sub_1800', 'sub_2000'], 'no main: by address');
assert.equal(firstFunction(stripped).name, 'sub_1800', 'the first program function when there is no main');
const macho = groupFunctions([fn('_helper', '0x100003f00'), fn('_main', '0x100003f50')]);
assert.equal(macho.program[0].name, '_main', 'Mach-O `_main` leads');
const big = groupFunctions([fn('b', '0xffffffff81000010'), fn('a', '0xffffffff81000000')]);
assert.deepEqual(big.program.map((f) => f.name), ['a', 'b'], '64-bit addresses order exactly');
assert.equal(firstFunction(groupFunctions([fn('_start', '0x10')])).name, '_start', 'only startup code: open that');
assert.equal(firstFunction(groupFunctions([])), null);
checks.push('ordering, main variants, fallbacks');

console.log(`DECOMPILE2 GROUPS OK — ${checks.join('; ')}`);
