// groups.js — which part of the sidebar a function belongs in: the student's
// program, the startup and runtime code every compiler adds, or the functions
// it imports from libraries. DOM-free. A name nobody listed stays with the
// program: hiding real code is worse than showing one helper too many.

const STARTUP = new Set([
  '_init', '_start', '_fini', 'deregister_tm_clones', 'register_tm_clones', '__do_global_dtors_aux',
  'frame_dummy', '__libc_csu_init', '__libc_csu_fini', '_dl_relocate_static_pie',
]);
const STARTUP_RE = /^(?:__x86\.get_pc_thunk|_?_?(?:mainCRTStartup|WinMainCRTStartup|wmainCRTStartup|__security_init_cookie|__scrt_))/;
const MAIN = ['main', '_main', 'wmain', 'WinMain', 'wWinMain', '_wmain'];

/** `program` | `startup` | `imports`. */
export function groupOf(fn) {
  if (fn.kind === 'plt' || fn.kind === 'thunk') return 'imports';
  if (STARTUP.has(fn.name) || STARTUP_RE.test(fn.name || '')) return 'startup';
  return 'program';
}

const byAddress = (a, b) => {
  const x = BigInt(a.address_hex), y = BigInt(b.address_hex);
  return x < y ? -1 : x > y ? 1 : 0;
};

const mainRank = (fn) => {
  const at = MAIN.indexOf(fn.name);
  return at < 0 ? MAIN.length : at;
};

/** The three groups, each sorted: the program with `main` first, then by address. */
export function groupFunctions(functions) {
  const out = { program: [], startup: [], imports: [] };
  for (const fn of functions) out[groupOf(fn)].push(fn);
  out.program.sort((a, b) => mainRank(a) - mainRank(b) || byAddress(a, b));
  out.startup.sort(byAddress);
  out.imports.sort(byAddress);
  return out;
}

/** The function to open first: `main` when there is one, else the program's first. */
export function firstFunction(groups) {
  return groups.program[0] || groups.startup[0] || groups.imports[0] || null;
}
