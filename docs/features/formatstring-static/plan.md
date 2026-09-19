# `formatstring static` — plan

## Shape

The option becomes three-valued: `off | static | full`, default `static`. `on`
stays accepted as a spelling of `full` (that is what it meant when the option was
`on|off`, and a run that asked for `on` asked for the loop).

| value | load-time resolver | decompile→override→re-decompile loop |
|---|---|---|
| `off` | – | – |
| `static` (default) | yes | – |
| `full` (= `on`) | yes | yes, for the sites `static` declined |

## Pieces

| file | what |
|---|---|
| `kuna-decomp/src/p1_partition/kuna_formatstring.rs` | `FormatStringMode` + `parse_formatstring_mode` (the `cppsig` value-option idiom) |
| `kuna-analysis/src/analyzers/formatstring/kuna_fmtstatic.rs` | `FormatStringStaticPass` — the load-time resolver |
| `kuna-analysis/src/analyzers/protos/mod.rs` | `variadic_format_prototype` (the declaration-driven format-function predicate) + `effective_libctypes_layout` |
| `kuna-analysis/src/analyzers/formatstring/apply.rs` | override parameters are anonymous |
| `kuna-analysis/src/pass.rs` | `FormatSiteFact` + `AnalysisOutput::format_sites` |
| `kuna-analysis/src/passes.rs` | the pass joins `listing_consumer_passes` |
| `kuna-console/src/engine.rs` | the commit parks the sites on the architecture, keyed by caller |
| `kuna-decomp/src/infra/architecture.rs` | `analysis_formatstring: FormatStringMode`, `format_call_overrides` |
| `kuna-console/src/decompile_step.rs` | parked overrides feed the FIRST drive; `audit_parked_format_sites` withdraws any the drive contradicts and drives once more; the loop and its read-only propagation are `full`-only |
| `kuna-decomp/src/p0_knowledge/modes.rs` | `formatstring` leaves the `aggressive` exclusion list |

## Why this can be default-ON

- **The parity corpora cannot see it.** The static resolver needs a Listing, and
  neither the XML datatest path nor the console parity path builds one. 0/675 and
  stages PARITY OK are structural, not lucky.
- **It costs no decompile.** The work is one xref lookup per format callee plus a
  bounded fold per call site, at load, next to a Listing that is already built.
  The one exception is a function where the first drive contradicts a parked
  site (a jump-table join the Listing cannot see, an `alloca` frame): that
  function is driven a second time without the contradicted override.
- **`full` keeps everything that was there.** Nothing is withdrawn: the loop is a
  value away, and `on` still selects it.

## Verification

- two-pass stage test `tests/stages/kuna-formatstring-static.xml` (off = the bug,
  default = the fix, plus a `full` pass that reaches the same answer without a
  Listing), and three more passes over `fmtjoin_x86_64` pinning the two sites
  the audit must withdraw (a format chosen on a jump table; an `alloca` frame);
- `tests/cli/printf-varargs-render-untyped.json`;
- unit tests: the full C99 conversion matrix, every length modifier over `d`/`u`,
  `*` width/precision (and `scanf`'s suppression `*`), the format-function
  predicate over 21 accepted and 12 refused names, and the fold's arithmetic
  (`truncate` / `sign_extend` / overlapping-write kill / `PIECE` at the low
  input's width / shifts of 64 or more);
- whole-corpus `decompile-all` off→static over 10 binaries, every hunk classified;
- off→static over every O2 coreutils binary plus 31 others, every format call
  whose format the C names checked for vararg count against conversion count,
  and every function's arity compared;
- interleaved min-of-15 `decompile-all` wall time, off vs static vs full;
- the 444-slice decbench `type_match` typesweep, static vs off and full vs off.
