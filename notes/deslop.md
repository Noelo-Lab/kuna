# Repository cleanup

One cumulative PR on `refactor/deslop`, starting at `632437155`.
Preserve CLI output, option semantics, decompiler defaults, and regression
expectations. Commit verified milestones; do not re-pin baselines to hide failures.

## Current changes

- Option dispatch and its 245-name frontend allowlist now come from the same
  declaration in `p0_knowledge/kuna_option_dispatch.rs`. The original name order
  and all handler bodies are preserved. Catalog metadata remains independent;
  a test checks names and uniqueness against the implemented handlers.
- CLI subprocess and in-process loading share environment bindings and value
  conversions in `loadtime.rs`. Tests cover legacy token behavior, final-option
  precedence, explicit removal, and restoration after unwinding.
- Removed obsolete porting instructions and historical count-change logs from
  the touched option code. Count assertions remain intact.
- CLI integration tests exercise the executable instead of compiling private
  module copies. They parse output with an independent JSON implementation.
  Removed missing-command and missing-spec success paths throughout the CLI
  suite, including wrappers that returned `Option` only to support those skips.
  Missing processor specs now fail tests; optional C-compiler checks are retained.

## Evidence

- Untouched baseline: 675/675 upstream assertions. The stage suite initially
  failed six Cortus APS3 assertions because its compiled spec predated the
  source. Rebuilding that one artifact made the baseline pass 1467/1467.
- Refactor: upstream and stage parity pass; catalog JSON is byte-identical;
  `catalog --check` and `make check-spec` pass; 268/268 CLI probes pass.
  Text/JSON decompilation and JSON function listings match the baseline's stdout
  and stderr with repeated and mixed loader options. The full workspace suite
  and generated-options freshness test pass.
- Eight interleaved warm runs of `decompile fauxware main` with loader overrides:
  median 133 ms before, 120 ms after. Other tests were running concurrently, so
  this is a check for a noticeable slowdown, not evidence of a speedup.
- Baseline executables retained outside the repository at
  `/tmp/kuna-deslop-baseline.5KlEYM` for comparison.
- CLI test cleanup: all four gates and 268/268 CLI probes pass again. Running
  the modulo CLI regression with an empty specs directory fails as intended;
  missing specs can no longer turn that test green.

## Audit still open

These are investigation targets, not a claim that the repository review is done.

| Area | Evidence / next check |
|---|---|
| CLI test structure | Private module copies and missing-command/spec skips are removed; the full workspace passes. Other suites still contain ad hoc JSON field extraction and duplicate process helpers. |
| Option plumbing | Loader options still use process-wide environment variables; inspect the loader API before replacing ambient configuration. Numerous per-option modules repeat boolean parsing. |
| Parsing and serialization | `jsonfmt.rs` contains a permissive handwritten JSON parser; `build.rs` contains a handwritten TOML subset. Check compatibility requirements before choosing replacements. |
| CLI responsibilities | `decompile_all.rs` and `jobs.rs` combine discovery, scheduling, serialization, and process lifecycle. Identify ownership boundaries and duplicated policies. |
| Collection policy | `cargo clippy --workspace --lib --bins` stops in `kuna-decomp` with 222 denied `HashMap`/`HashSet` findings. The configured rationale points to a missing ADR. Review iteration semantics and lookup costs before replacing collections, then check the remaining crates and enforce the gate. |
| Engine boundaries | `kuna_addcarrychain`, `kuna_arraystride`, `ruleaction_3`, and `ruleaction_4` duplicate `new_unique_out`. The real method additionally assigns high variables and checks register lanes, so replacing these requires behavioral tests. Ninety engine files still contain wave-era STUB notes. |
| Analysis, SLEIGH, Python, integrations | Broader structural and error-handling review still required. |

Before each commit: `make test`, `make test-stages`, `make rust-test`,
`make check-spec`. Also run the catalog check and relevant CLI probes. Preserve
the single-PR scope across subsequent milestones.
