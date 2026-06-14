# Review: w10-pspec-context (Round 1)

**Verdict: ACCEPT_WITH_LOSSES**

Branch `rport/w10-pspec-context` @ `4491152` (merge-base = `rust-port` @ `848fe63`).
Independent verifier (not the porter). Focus: faithfulness of the pspec
`<context_data>` parse+apply, no special-casing, the x86:64 64-bit decode is
real, all gates green.

## What this branch actually is

The merge-base of this branch IS `rust-port` HEAD, so the only delta on the
branch is a single test file (`verify_w10_struct_corpus.rs`, +105 lines, two new
`#[test]` functions). The implementation the commit message describes
(`Architecture::parse_processor_config` -> `ContextInternal::decode_from_spec`,
the `set_pspec_xml` console + datatest wiring, the `VarnodeData::decode_from_attributes`
name path, `SnapshotRegisterLookup`) already landed on `rust-port` via commit
`923da87` (part of the merged `w10-refinement-loops`). This branch re-pins the
property with two regression tests on the loop-NAMED files. I verified the
implementation as it exists at the branch tip (the only meaningful unit of
review) plus the new tests.

## (1) FAITHFULNESS — VERIFIED

The `<context_data>` parse is a faithful 1:1 port of the C++ oracle:

- `ContextInternal::decode_from_spec` (globalcontext.rs:1017) mirrors
  `decodeFromSpec` (globalcontext.cc:531) element-for-element: open
  `ELEM_CONTEXT_DATA`, loop `open_element`, `Range::decode_from_attributes`,
  `get_first_addr`/`get_last_addr_open`, dispatch `ELEM_CONTEXT_SET` ->
  `decode_context` / `ELEM_TRACKED_SET` -> `decode_tracked`, else error
  "Bad <context_data> tag", close.
- `decode_context` (globalcontext.rs:786) mirrors `decodeContext`
  (globalcontext.cc:345): read `val` (`ATTRIB_VAL`), look up the variable by the
  `name` attribute via `get_variable(nm)`, re-zero the default blob per `<set>`
  when `addr1.is_invalid()` (exactly the C++ per-child re-zero), else
  `get_region_for_set` + `set_value`. The truncation `read_unsigned_integer_id(..) as u32`
  matches the C++ `uintb`->`uintm` implicit narrowing.
- `Architecture::parse_processor_config` (architecture.rs:995) wires the
  `parseProcessorConfig` (architecture.cc:1176) `ELEM_CONTEXT_DATA` branch and is
  invoked from `init_post_engine` before `build_typegrp`/the action build —
  before any instruction is decoded, matching the C++ ordering
  (`restoreFromSpec` -> `parseProcessorConfig` at architecture.cc:645).
- The load path is real and generic: both the console `load file` path
  (engine.rs:251) and the datatest bootstrap read the resolved `.pspec`
  (`specs.processorfile`, the C++ `processorfile`) and hand it to
  `set_pspec_xml`. No archid branching.

## (2) NO SPECIAL-CASING — VERIFIED

The context values come entirely from PARSING the pspec, never from constants:

- `grep -rniE 'longmode|addrsize|opsize'` over the production `src/` returns ONLY
  doc-comments and unrelated `getAddrSize()` API calls. ZERO hardcoded context-
  variable names or values in `globalcontext.rs` / `architecture.rs` logic.
- `get_variable(nm)` (globalcontext.rs:825) is a plain `variables.get(nm)` map
  lookup keyed on the `<set name=...>` string — the variable table is populated
  from the `.sla`. `decode_context` reads `val` from the `<set val=...>` attribute.
- `find_child(el, nm)` (architecture.rs:77) is a generic `get_name() == nm` walk.
- The only `x86`/`386` string in a code path is `normalize_processor`
  (sleigh_arch.rs:903): a byte-identical port of upstream `normalizeProcessor`
  (sleigh_arch.cc:483, `if nm.contains("386") -> "x86"`) — upstream Ghidra
  behavior, not a kuna shortcut.
- The parse is generic across processors: ARM (`condconst2`), and any
  context-sensitive processor, decode through the identical path.

## (3) x86:64 64-bit decode is REAL — VERIFIED (independent before/after)

I wrote a throwaway verifier probe that bootstraps `loopcomment` TWICE on the
real render path — once handing the `.pspec` to `set_pspec_xml` (the fix) and
once skipping it (toggle off, the pre-fix state) — and counted markers:

```
AFTER  (pspec applied):  64-bit markers=49  16-bit real-mode markers=0
BEFORE (pspec skipped):  64-bit markers=0   16-bit real-mode markers=35
```

Toggling the pspec parse FLIPS the lift: with it, 64-bit registers (RSP/RBP/RDI/…)
and zero 16-bit real-mode signature; without it, zero 64-bit and 35 real-mode
garbage marks (BX+SI / segment CALLOTHER / 0xfffe). This proves the fix is
load-bearing AND that the 64-bit lift comes from the applied context, not a
hardcode. The probe was removed after confirming (verifier scratch).

The two committed regression tests pass and assert the same property on the
loop-NAMED files (`loopcomment`, `forloop_varused`): 64-bit registers present,
16-bit real-mode signature absent, real C loop keyword emitted (back-edges
formed). A 16-bit real-mode mis-lift could not satisfy any of the three.

## (4) Gates — ALL GREEN

- `cargo test --workspace`: **3180 passed, 0 failed** (matches the commit's
  3178+2 claim).
- `cargo test -p kuna-decomp --test verify_w10_struct_corpus`: 8/8 pass
  (the 2 new pspec-context + 6 refinement-loops adversarial tests).
- `cargo clippy --workspace --lib`: clean. The new test file
  (`verify_w10_struct_corpus`) is clippy-clean. (`--all-targets` shows warnings
  ONLY in pre-existing unrelated test files — `verify_w2core_walkback`,
  `verify_w3_*`, `verify_w5_s3_rules_7`, … — not introduced by this branch.)
- C++ oracle: **207/207 unit + 675/675 datatests, PARITY OK**. The C++ tree diff
  vs `rust-port` is EMPTY (untouched).

## Adversarial tests committed (>=3): MET

On this branch: `verify_w10_pspec_context_loopcomment_lifts_64bit_and_structures`,
`verify_w10_pspec_context_forloop_varused_lifts_64bit_and_structures`. Plus the
refinement-loops adversarial tests in the same file that depend on this exact
fix: `verify_w10_r2_divopt_reciprocal_multiply_is_64bit` (the 64-bit magic
`0x948b0fcd6e9e0653` is impossible under a 16-bit lift),
`verify_w10_r2_divopt_stores_through_rdi_straightline`,
`verify_w10_r2_forloop1_is_bounded_loop_not_bare_keyword`. >=3 satisfied.

## Why ACCEPT_WITH_LOSSES (not ACCEPT)

The `<context_data>` branch is faithfully and generically wired and the decode
fix is real — none of the REJECT conditions (special-cased / hardcoded /
unfaithful / fake fix) hold. But `parse_processor_config` dispatches ONLY the
`context_data` child of C++ `parseProcessorConfig`; it silently skips every other
`<processor_spec>` child (volatile, register_data, jumpassist, segmentop,
data_space, inferptrbounds, …) and drops the C++ `throw` on an unrecognized
element. On the x86-64 corpus the skipped children are cosmetic
(`register_data` = DEBUG/CONTROL register grouping; `programcounter`/`properties`
are C++ skip-elements too), so it is unobservable on the gate — but it is a real
faithfulness scope reduction. Recorded as **LOSS-143**.

## Losses

- **LOSS-143** (new, recorded in MAIN-tree losses.md): `parse_processor_config`
  wires only the `<context_data>` branch of `parseProcessorConfig`; the other
  `<processor_spec>` children are silently skipped and the unknown-element throw
  is dropped.
- LOSS-139..142 (already recorded under the merged `w10-refinement-loops`) carry
  forward: emitBlockGoto over-emit, emitForLoop deferral, splitPieces in-block
  order, SnapshotRegisterLookup name-resolution divergence.

## Blockers

None.
