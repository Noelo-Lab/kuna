# One project export for the CLI and the browser

This proposal is docs first: it contains no production change. The browser's project export
(`kuna_wasm <binary> <specs> project`, which decompile.run's "C code" download calls)
re-implements the `kuna decompile-project` export, and the two have drifted. The browser
never takes the callee-first order (`protoorder`), which the CLI takes by default on a whole
binary. So the same binary exports different C on the two surfaces: `accepted`'s prototype
on `fauxware`, and 36,213 lines of `.c` and 3,210 of `.h` on `mcount_x86_64`
(`analysis.md`). The fix is one export, which both surfaces call.

## What moves

1. **The call graph and the callee-first decompile move from `kuna-cli` into `kuna-console`,
   unchanged.**
   - `kuna-cli/src/callgraph*` (1,411 lines) and `decompile_all/callee_first.rs` (959 lines)
     become `kuna-console` modules.
   - `kuna-cli` re-exports them, so its call sites do not change.
   - `callee_first` takes the six `Args` fields it reads (`binary`, `no_vars`, `recording`,
     `slice_pref`, `stated`, `wants_provenance`) as a small options struct.
   - The `kuna_allocrelease::after_analysis` hook moves with it, or is passed in as a
     closure.
   - This is a pure move, so the gates should show byte-identical output.

2. **`kuna_console::project::export(prog, targets, order, file_name) -> Export` is the
   export.**
   - It covers what `decompile_project.rs` does after resolving the targets:
     - the decompile, in callee-first or address order;
     - the address sort;
     - `print_c_recompile_prelude` and `print_c_types`;
     - `build_header`, `build_c`, `build_asm` and the README facts.
   - `order` is decided by today's `callee_first_decision`, which moves with step 1.
   - Both `decompile_project` and `kuna_wasm::project` call it. The browser then exports
     what the CLI exports.

The CLI-only surfaces stay where they are: `--jobs` (worker processes), `--stream` and
`--progress` (incremental files), and the folder write. The two `load_program`s also stay
separate, since WASI paths and the browser's option set differ. Only the export
converges.

## Tests

- **A parity test.** For each fixture in the export tests (`fauxware` and at least one
  stripped and one C++ binary), the native `kuna_wasm` `project` artifacts must equal
  `kuna decompile-project`'s, byte for byte, except the README's path row. It fails today.
- **Gates.** `make test`, `make test-stages` and the workspace suite show no change, since
  both steps are moves for the CLI.
- **Speed.** Unchanged for the CLI. The browser export pays the callee-first order, as the
  CLI already does. That cost is measured and recorded before un-drafting.

## Open questions

- Is `kuna-console` the right home for the call graph, or should it go in a small crate both
  depend on? `kuna-console` already holds the program and the project builders, so the
  proposal starts there.
- Should the browser keep an address-order escape hatch, as `--option protoorder off` is
  on the CLI? The browser already accepts `--option`, so the same switch should apply
  unchanged.
