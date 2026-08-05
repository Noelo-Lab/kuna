# dallparity -- plan

Driver/CLI-tier correctness fix. **No `--option`, no stage XML, no catalog
counters** (nothing in the phase model changes, and `phases.toml` gains no row);
it does need a **DIV row**, a **`docs/spec/` touch** and **Rust integration
tests** -- the shape PR #262 (DIV-59) established for this tier. Classification
was checked, not assumed: the change is entirely inside `kuna-console`'s driver
seam, it adds no decision point, and it makes an existing option (`formatstring`)
behave the same on both surfaces instead of introducing a new behaviour. There is
therefore nothing to gate: a surface that ignores the option the caller set is a
bug, and the option itself is already the switch.

## Step 1 -- establish the true scope before fixing anything

1. Code-diff the two paths end to end: `kuna decompile` (CLI ->
   `decomp_dbg` script -> `IfcDecompile`) against `kuna decompile-all`
   (`load_program` -> `resolve_targets` -> `decompile_targets`). Enumerate every
   asymmetry, including the deliberate ones.
2. Output-diff them: for 13 real binaries (C/C++, stripped/unstripped, O0/O2,
   x86-64 exe + shared library + ARM Cortex-M firmware) x 4 modes, sample
   addresses out of `decompile-all --json` and re-decompile each with
   `kuna decompile --addr`, normalizing only the function name. Classify every
   difference; a class the code-diff did not predict is the real finding.

## Step 2 -- fix, by making the shared pipeline the single source of truth

Extract the format-string override / re-decompile loop and its scoped
`readonlypropagate` flip out of `IfcDecompile` into a new
`kuna-console/src/decompile_step.rs`, and route BOTH surfaces through it. Do not
copy the loop into `decompile_targets`: the divergence exists because there were
two call sites, so the fix must leave one.

Design constraints:

- The console's extra facts (`map addr`, `parse line extern`, `map hash`,
  `type varnode`, `override flow`/`prototype`, `map param`) must stay expressible;
  they go through one `DecompileSeed` struct with a `plain()` constructor for the
  whole-binary caller.
- Discovered call-site overrides are returned, not persisted globally: persisting
  them across functions in a whole-binary run would make the output
  order-dependent and would itself be a new divergence from `kuna decompile`.
- With `formatstring` off the step must be byte-identical to the old drive call,
  so every parity gate stays green with no re-pin.

## Step 3 -- prove it

- **Parity**: re-run the 182-record sweep; the target is 0 divergent.
- **Sweep (standing requirement)**: whole-binary `decompile-all` before vs after
  over 33 cases, with a third arm (new build, `--option formatstring off`) that
  must be byte-identical to the old build -- that arm is what proves the refactor
  itself is inert and isolates the intended change.
- **Gates**: `make test` (675/675, no re-pin), `make test-stages`,
  `make rust-test`, `make check-spec` (lenient + strict), `kuna catalog --check`.
- **Speed**: the honest risk is that a function calling printf now decompiles
  twice. Measure `decompile-all` whole-binary, interleaved, min of >= 13 pairs,
  on small binaries (where `auto` -> `aggressive` turns `formatstring` on and the
  cost is paid) and on large ones in `reliable`/`fast` (where it must be flat).
  If the honest fix exceeds the 5% budget, say so and gate rather than hide it.
  **Outcome: it does.** The opt-in costs +43% to +75% on printf-heavy binaries,
  100% of it the second decompile; the redundant-override skip was built,
  measured (crontab 25 -> 24 re-decompiles) and rejected. `formatstring` therefore
  leaves `AGGRESSIVE_OVERRIDES` for `EXCLUDED_ON_PURPOSE`, which makes the shipped
  default free (worst +0.16%) and keeps the capability one flag away on both
  surfaces.
- **Benchmark**: decbench GED + `type_match`, same scorer both arms
  (`325046f`, `cache_version` 5, `DECBENCH_NO_CACHE=1`), before = the base-commit
  `kuna`, after = this build.

## Step 4 -- tests

`kuna-console/tests/verify_decompile_all_parity.rs`, over the four vendored
`fmt_<arch>` fixtures (x86-64 / AArch64 / ARM / RISC-V, so the ARM literal-pool
case is covered):

1. two-pass on the whole-binary surface -- `formatstring` off leaves the varargs
   untyped (the bug), on types the `%s` argument as `char *` (the fix);
2. parity -- in BOTH option states the whole-binary loop's C for `main` is
   byte-identical to the console command's.

No stage XML: every `tests/datatests` and `tests/stages` file is an XML
`<binaryimage>` whose bootstrap never produces an `AnalysisOutput`, so
`formatstring` (an analysis-tier option) cannot be reached from there at all.

## Out of scope, recorded rather than folded in

`kuna decompile-all` cannot load an unstripped C++ binary containing an anonymous
namespace (`Non-global scope has empty name`). Verified to be its own load-tier
root cause in `kuna-analysis`'s demangler, failing identically on BOTH surfaces --
so it is not a divergence, does not belong in this PR, and is reported in the PR
body with the root cause and the one-line shape of the fix.
