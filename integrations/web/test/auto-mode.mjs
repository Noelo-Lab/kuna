import { wasmCommandArgs } from '../kuna-web.js';

function assertArgs(actual, expected, label) {
  if (actual.join('\0') !== expected.join('\0')) {
    throw new Error(`${label}: got ${JSON.stringify(actual)}, want ${JSON.stringify(expected)}`);
  }
}

assertArgs(
  wasmCommandArgs('list'),
  ['/work/input.bin', '/specs', 'list', '--mode', 'auto'],
  'list auto mode',
);
assertArgs(
  wasmCommandArgs('decompile', 'main'),
  ['/work/input.bin', '/specs', 'decompile', 'main', '--mode', 'auto'],
  'decompile auto mode',
);
assertArgs(
  wasmCommandArgs('project', 'sample.elf'),
  ['/work/input.bin', '/specs', 'project', 'sample.elf', '--mode', 'auto'],
  'project auto mode',
);
assertArgs(
  wasmCommandArgs('decompile', 'main', 'fast'),
  ['/work/input.bin', '/specs', 'decompile', 'main', '--mode', 'fast'],
  'explicit mode override',
);

console.log('AUTO MODE GLUE OK — list/decompile/project pass an explicit WASM mode');
