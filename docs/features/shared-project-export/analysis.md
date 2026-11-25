# Analysis: the browser export has drifted from `decompile-project`

## Evidence

All measurements were made at upstream main `a3351f838`, with the release builds of
`kuna` and of the native `kuna_wasm` binary that the first command makes.

```bash
cd decompiler && cargo build --release -p kuna-cli -p kuna-wasm && cd ..
F=decompiler/crates/kuna-analysis/tests/fixtures/fauxware
decompiler/target/release/kuna decompile-project $F -o /tmp/fx >/dev/null
grep -n 'accepted(void)' /tmp/fx/fauxware.h
#   84:int accepted(void);
decompiler/target/release/kuna_wasm $F specs project | jq -r '.files["fauxware.h"]' | grep -n 'accepted(void)'
#   84:void accepted(void);
```

In the `.c`, the CLI prints `return puts("Welcome to the admin console, trusted user!");` in
`int accepted(void)`, and the browser prints the bare call in `void accepted(void)`.

The artifacts of `decompile-project` and of `kuna_wasm ... project` differ, measured as the
`+`/`-` lines of Python's `difflib.unified_diff(a, b, n=0)` (GNU `diff -U0` finds a shorter
diff on `mcount_x86_64`: 36,019 `.c` lines and 2,568 `.h` lines):

| Fixture | `.c` lines (CLI) | `.c` differing | `.h` lines (CLI) | `.h` differing |
|---|---|---|---|---|
| `fauxware` | 190 | 4 | 92 | 2 |
| `mcount_x86_64` | 172,079 | 36,213 | 3,638 | 3,210 |

On `mcount_x86_64`, the differences are:

- record names: the CLI's `struct_16` is the browser's `struct_31`;
- argument and local types: `void *v1` against `unsigned long v1`;
- 146 more header lines on the CLI side;
- 2,009 `.asm` lines, in the `; arg:` and `; stack:` comments and the data labels.

These are the effects `docs/cli.md` attributes to the callee-first order
(`--option protoorder`): which record a layout becomes, and the types callers learn from the
prototypes their callees state. With the order off, the CLI writes the browser's export:

```bash
decompiler/target/release/kuna decompile-project $F -o /tmp/fx-off --option protoorder off >/dev/null
decompiler/target/release/kuna_wasm $F specs project | jq -j '.files["fauxware.c"]' | cmp - /tmp/fx-off/fauxware.c && echo same
#   same
```

On both fixtures the `.c`, `.h` and `.asm` are then byte-identical, and the README differs
only in its `Path` row.

## Why

- **`kuna-wasm/src/lib.rs` (`fn project`) is "the `kuna decompile-project` flow
  (`decompile_project.rs::decompile_project`) with the folder write replaced".** It calls
  `decompile_export_targets` directly, so it decompiles in address order.
- **`decompile_project.rs` asks `decompile_all::callee_first_decision` and takes
  `decompile_callee_first` on a whole binary.** That is the default, so the two diverge on
  every whole-binary export.
- **The browser cannot call the CLI's path.** `callee_first` and the `CallGraph` it walks
  live in `kuna-cli`, which is a binary crate. `kuna-wasm` does not depend on it, and could
  not on wasm32: its `--jobs` spawns processes.
- **No test compares the two.** `parity.mjs` compares the wasm build with the native
  `kuna_wasm`, never with `kuna`. `docs/web-integration.md` documents the address order as
  one of two known differences from the CLI export (the other is the README's `Path` row).

One more difference does not come from the order and does not show on these fixtures. The
CLI's `load_program` sets `errortoomanyinstructions off` on its decompiling surfaces, and
`kuna_wasm`'s does not. A function past the 100,000-instruction budget (a
`.rept 110000; nop` body) is a truncated body with a warning in the CLI export and an
`error:` record in the browser's.

## What callee-first needs

- **The call graph.** `decompile_callee_first` (`decompile_all/callee_first.rs`, 959 lines)
  builds a `CallGraph`, plans with `callee_first_plan`, and asks seven queries:
  `open_entries`, `direct_callers`, `direct_call_sites`, `called_directly`,
  `callback_refs`, `arm_closed_sites` and `address_taken_refs`. The graph is built in
  `kuna-cli/src/callgraph*` (1,411 lines), which reads the image through
  `kuna-cli/src/image.rs`.
- **`decompile_all/custom_reg_callers.rs`** (329 lines, `calleereginputs custom`).
  `callee_first` calls its `analyze` and `decide`, and it asks the graph two more queries,
  `entry_extents` and `reg_call_sites`.
- **These `decompile_all::Args` fields.** `decompile_callee_first` reads `binary`, and
  `slice` and `target` through `slice_pref()`. It also reads `max_fn_seconds` and
  `max_fn_memory` through `arm_fn_budgets`. `decompile-all`'s wrapper adds `no_vars` and
  `wants_provenance()`, and `callee_first_decision` reads `options` and `raw_image`.
  (`recording` and `stated` belong to the architecture's `kuna_callerargs` ledger, not to
  `Args`.)
- **`ResultStore`** (`decompile_all/result_store.rs`), which encodes each result with
  `jobs::wire`'s `encode_result` and `decode_result`.
- **`kuna_allocrelease::after_analysis`**, a `malloc_trim` on Linux glibc only.

None of these spawns a process or a thread. The one wasm hazard is the result store's spool.
`ResultStore::new` calls `tempfile::tempfile()`, which reads `std::env::temp_dir()`, and on
`wasm32-wasip1` that panics with `no filesystem on wasm`. The store already keeps a result
in memory when it has no file, so the caller should supply the spool, and the browser
should supply none.
