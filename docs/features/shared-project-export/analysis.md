# Analysis: the browser export has drifted from `decompile-project`

## Evidence

All measurements were made at upstream main `a3351f838`, with test-profile builds of
`kuna` and of the native `kuna_wasm` binary.

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
`+`/`-` lines of a zero-context unified diff:

| Fixture | `.c` lines (CLI) | `.c` differing | `.h` lines (CLI) | `.h` differing |
|---|---|---|---|---|
| `fauxware` | 190 | 4 | 92 | 2 |
| `mcount_x86_64` | 172,079 | 36,213 | 3,638 | 3,210 |

On `mcount_x86_64`, the differences are:

- record names: the CLI's `struct_16` is the browser's `struct_31`;
- argument and local types: `void *v1` against `unsigned long v1`;
- 146 more header lines on the CLI side.

These are the effects `docs/cli.md` attributes to the callee-first order
(`--option protoorder`): which record a layout becomes, and the types callers learn from the
prototypes their callees state.

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
  `kuna_wasm`, never with `kuna`.

## What callee-first needs

- **The call graph.** `decompile_all/callee_first.rs` (959 lines) uses eight `CallGraph`
  queries: `open_entries`, `is_ok`, `direct_callers`, `direct_call_sites`,
  `called_directly`, `callback_refs`, `arm_closed_sites` and `address_taken_refs`. That
  graph is built in `kuna-cli/src/callgraph*` (1,411 lines).
- **Six fields of `decompile_all::Args`:** `binary`, `no_vars`, `recording`, `slice_pref`,
  `stated` and `wants_provenance`.
- **`ResultStore`.**
- **`kuna_allocrelease::after_analysis`.**

None of these touches processes or files outside the result store's temporary file, which
`kuna-console` can own as well.
