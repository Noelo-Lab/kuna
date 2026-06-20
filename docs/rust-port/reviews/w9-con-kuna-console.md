# w9-con-kuna-console
verdict: ACCEPT-WITH-LOSSES
verifier: Claude Opus 4.8 (1M context)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-console` -> ok, 109 passed; 0 failed (104 pre-existing + 5 verifier-added)

## Scope

C++ in scope: `decompiler/cpp/kuna_console.cc`, `decompiler/cpp/kuna_console.hh`
Rust port: `rust/crates/kuna-console/src/kuna_console.rs` (+ `kuna_console/tests.rs`).
The port leans on `kuna-decomp`'s `kuna_assert` (`validate_assertion`/`Dispatch`/
`AssertLog`) and `kuna_stages` (`emit_catalog_json[_one]`, lookups) — both
out-of-scope but read to establish the boundary; the JSON catalog emitter and the
`kassert` validation/render core they own are byte-faithful to the C++ they port.

## Mechanical pass

- grep `kuna_console.rs` for `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable`/bare-`as`/`panic!`: **none**. Two `.expect()` calls
  (kuna_console.rs:752,791) are each guarded by an immediately-preceding
  `dcp.fd.is_none()` check — sound.
- `cargo clippy -p kuna-console --tests -- -D warnings`: **clean**.
- gate `cargo test -p kuna-console`: **109 passed, 0 failed**.

## Hunt list

- **Signedness**: clean. The only `int4`/usize boundary is the registry-count
  loops (`for i in 0..kuna_num_substages()` etc.), all `usize` index vs `usize`
  count; no signed/unsigned comparison.
- **Integer widths**: clean. No arithmetic on `uintb`-lineage values in scope; the
  only numeric format is `i.to_string()` in the (out-of-scope) `render_list`,
  which matches C++ `dec << i`.
- **Wrapping**: clean — no arithmetic in scope. (The `temp.last += 1` upper-bound
  key bump lives in `interface.rs::restrict_com`, out of scope, and uses
  `wrapping_add`.)
- **Comparator totality**: n/a — no comparators defined in scope.
- **Iteration-order provenance**: clean. Every loop is `for i in 0..kuna_num_*()`
  over a vector-index registry (insertion order = registry order = C++
  `for(i=0;i<kunaNum*();++i)`); `STAGES_IN_ORDER`/`PIPELINE_VARIANTS` are fixed
  arrays matching the C++ `for(i=0;i<=9)` / `PIPELINE_VARIANTS[]`. No HashMap/HashSet.
- **Off-by-one / do-while / reverse iterators**: clean. The `stage map` join loop
  and the `kassert` `tokenizeRest` loop are `while !s.eof() { read_token; skip_ws;
  if empty break }`, a faithful port of C++ `s >> ws; while(!s.eof()){ s>>word>>ws;
  if(empty)break }` — verified the eof-latching matches via the `CommandStream`
  tokenizer (`read_token`/`skip_ws` latch eof at buffer end exactly like
  `istream >> word >> ws`). Adversarial test
  `stage_map_collapses_irregular_whitespace` pins this.
- **Erase-while-iterating**: n/a — no container mutation during traversal in scope.
- **Exception -> Result partial-state parity**: clean. Every error path is a
  pre-mutation guard (`No load image present` / `No function selected` / parse
  errors / `engine_unavailable`); no partial state is left because the store
  mutation itself is the SEAM (LOSS-130). The `kassert` `assert_log` push happens
  only on the (unreachable-today) success path, after the store write — matching
  C++ `assertLog.push_back(rec)` placement after the dispatch.

## Item-specific (this wave's focus)

- **Exact console output text**: `stage list`/`stage map`/`stage catalog`/
  `pipeline list` headers, the per-row `        artifact:`/`        decision:`/
  `   rewind:`/`        exposure:` indentation, the `[Band B]`/`[orthogonal
  plane]`/`(LATENT)`/`(HARD)`/`(HINT)` tags, and the guard strings
  (`No load image present`, `No function selected`, `No code for <name>`,
  `No structured blocks for <name> (decompile first)`, `Unknown
  group/surface/sub-stage: <t>`, `Unknown settable option: <o> (try
  `stage catalog`)`, `Unknown pipeline variant: <n> (try `pipeline list`)`) are
  all transcribed character-for-character against the `.cc`. Verified.
- **Test-runner / driver error-prefix grammar**: the engine errors render under
  `Execution error: ` and parse errors under `Command parsing error: `, byte-equal
  to the real driver (`ifacedecomp.rs:1765/1767`, itself faithful to C++
  `ifacedecomp.cc:3624/3627`). The kuna_console tests' `run_one` helper mirrors it.
- **Command prefix-expansion**: `register_kuna_commands` registers the 11 token
  sequences byte-identical to the C++ `registerCom` calls; `stage cat` →
  `stage catalog` and the rest resolve (`kuna_command_prefixes_expand`). The
  multi-word `stage map force goto` surface key (datatest KUNA-CONSOLE #5) is
  rebuilt by the join loop, not collapsed to the first token — adversarial test
  `stage_map_joins_multiword_surface_key`.
- **Datatest stringmatch counting**: the in-tree datatests that string-match these
  commands (`tests/stages/kuna-console.xml` #1..#8, `kuna-assert.xml` #1..#3,
  plus `kuna-pipeline-quality.xml` / `kuna-restarts.xml` / `kuna-regions.xml`) run
  against the **C++ binary** and exercise the engine surface (e.g. KUNA-CONSOLE #7
  `pipeline variant: decompile`, #8 `compareform: original`, the routed `kassert`
  effect). The Rust console is not yet wired into that harness (SEAM); those rows
  are the engine-dependent half routed through `engine_unavailable` and are the
  substance of LOSS-130. The expressible halves the same XMLs depend on (the
  `[Band B]` count, `[S3] dead-definition-gate`, `(LATENT)` count, `group analysis
  -> S3`, `surface "force goto" -> S7 ... sub-stage edge-virtualization`,
  `sub-stage comparison-canonicalization -> S3`) are all reproduced byte-for-byte
  and unit-pinned.

## Findings

- F1 (loss, accepted as LOSS-130): every engine-touching kuna command body is
  routed through `engine_unavailable` after its expressible guards — `stage
  status` (the 3-line `pipeline variant`/`compareform`/`arraynotation` output),
  `restarts`, `pipeline <variant>` (the decompile drive), `quality` (the goto
  walk), the `region tree/blocks/walk` adapter, and every *routable* `kassert`
  store mutation. Same documented SEAM pattern as LOSS-127 (ifacedecomp) and
  LOSS-129 (codedata). Within this, the routable-`kassert` arms collapse the C++
  per-substage `IfaceParseError` (arg-count) and function-scoped
  `IfaceExecutionError("No function selected")` into one `engine_unavailable`
  execution error — but every such input is unreachable in any unit-tested or
  Python-harness path because the top-of-command `No load image present` guard
  fires first (no program is loaded in any console unit test), and the routable
  mutation only exists once the W5 stores land.
       cpp: decompiler/cpp/kuna_console.cc:160,318,347,421 ; decompiler/cpp/kuna_assert.cc:99-233
       rust: rust/crates/kuna-console/src/kuna_console.rs:136,425,674,728,785,625-651,835

No blockers, no majors. No silent output corruption: every unported surface is a
self-describing execution error naming its exact missing C++ entry point, never a
wrong value and never an "Invalid command" (which would mean the registration /
prefix surface drifted — explicitly guarded by `registers_all_eleven_kuna_commands`
and the resolve tests).

## Adversarial tests (added, committed on the branch)

In `rust/crates/kuna-console/src/kuna_console/tests.rs`, prefixed
`w9_con_kuna_console_`:
- `stage_map_joins_multiword_surface_key` — the `force goto` join (KUNA-CONSOLE #5
  surface key); must not collapse to the first token.
- `stage_map_collapses_irregular_whitespace` — `stage map   force   goto   `
  collapses to the single-space key (eof-latching / `>> ws` parity).
- `kassert_image_guard_precedes_validation` — the image guard short-circuits both a
  valid+`hint` request and a bogus-stage request before any tokenize/validate.
- `stage_catalog_single_option_skips_leading_ws` — `s >> ws >> option` extracts the
  option across leading whitespace and emits the single-row (not full-array) form.
- `stage_catalog_bare_is_full_array` — empty-option branch selects the full W4
  emitter array (the `kuna.catalog` parser contract).

All 5 pass; crate at 109 passing, clippy clean.

## losses

LOSS-130 (appended to docs/rust-port/losses.md). Cross-references the identical
LOSS-127 / LOSS-129 SEAM pattern.
