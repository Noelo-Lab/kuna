import { wasmCommandArgs } from '../kuna-web.js';

function assertArgs(actual, expected, label) {
  if (actual.join('\0') !== expected.join('\0')) {
    throw new Error(`${label}: got ${JSON.stringify(actual)}, want ${JSON.stringify(expected)}`);
  }
}

// Both policies are the ENGINE's: the glue passes `auto` through rather than
// resolving a byte-size threshold or sniffing the binary in JavaScript, so the
// browser and the CLI cannot drift apart on either one.
assertArgs(
  wasmCommandArgs('list'),
  ['/work/input.bin', '/specs', 'list', '--mode', 'auto', '--language', 'auto'],
  'list auto mode + auto language',
);
assertArgs(
  wasmCommandArgs('decompile', 'main'),
  ['/work/input.bin', '/specs', 'decompile', 'main', '--mode', 'auto', '--language', 'auto'],
  'decompile auto mode + auto language',
);
assertArgs(
  wasmCommandArgs('project', 'sample.elf'),
  ['/work/input.bin', '/specs', 'project', 'sample.elf', '--mode', 'auto', '--language', 'auto'],
  'project auto mode + auto language',
);
assertArgs(
  wasmCommandArgs('decompile', 'main', 'fast'),
  ['/work/input.bin', '/specs', 'decompile', 'main', '--mode', 'fast', '--language', 'auto'],
  'explicit mode override',
);
assertArgs(
  wasmCommandArgs('decompile', 'main', 'auto', 'rust'),
  ['/work/input.bin', '/specs', 'decompile', 'main', '--mode', 'auto', '--language', 'rust'],
  'explicit language override',
);

// The study-view commands: every directive rides as its own `--assert` pair,
// after the mode and language, in session order; `read` takes two positionals;
// and an empty directive list is the argv the flag's absence always produced.
assertArgs(
  wasmCommandArgs('inspect', 'main', 'auto', 'auto', ['name v1 total', 'type v2 unsigned int']),
  ['/work/input.bin', '/specs', 'inspect', 'main', '--mode', 'auto', '--language', 'auto',
    '--assert', 'name v1 total', '--assert', 'type v2 unsigned int'],
  'inspect + two assertions',
);
assertArgs(
  wasmCommandArgs('read', ['0x1149', 16]),
  ['/work/input.bin', '/specs', 'read', '0x1149', '16', '--mode', 'auto', '--language', 'auto'],
  'read positionals',
);
assertArgs(
  wasmCommandArgs('xrefs', '0x1161', 'fast', 'c', ['function 0x1161=summation']),
  ['/work/input.bin', '/specs', 'xrefs', '0x1161', '--mode', 'fast', '--language', 'c',
    '--assert', 'function 0x1161=summation'],
  'xrefs with a function rename',
);
for (const [command, arg] of [['list', undefined], ['decompile', 'main'], ['project', 'sample.elf']]) {
  assertArgs(
    wasmCommandArgs(command, arg, 'auto', 'auto', []),
    wasmCommandArgs(command, arg),
    `${command}: empty assertions == the old argv`,
  );
}

console.log(
  'AUTO MODE GLUE OK — list/decompile/project pass an explicit WASM mode and output language; ' +
  'inspect/read/xrefs argv and --assert ordering pinned',
);
