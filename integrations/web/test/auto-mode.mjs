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

console.log(
  'AUTO MODE GLUE OK — list/decompile/project pass an explicit WASM mode and output language',
);
