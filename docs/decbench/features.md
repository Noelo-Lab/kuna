# decbench campaign — feature clusters (from the 28-case triage)

Generated from `docs/decbench/triage/*.md` (2026-07-04, 28/28 triaged). One row per
planned PR; member cases link the evidence. Statuses across the queue:
9 feature-candidate, 7 covered-by-option, 11 metric-artifact, 1 needs-proposal.

| # | slug | scope | member cases | mechanism | est. pool impact |
|---|------|-------|--------------|-----------|------------------|
| F1 | `benchlisting` (Listing on for decompile-all) | small | beyond, abort_gzip, cleanup_exit, xalloc_die, ck_fopen, pubkey_info, xheader_set_option (+ mktemp_proto partial) | `option listing on` alone closes all 7: the default-on `noreturn_propagate` pass is a **no-op without the Listing**, and the benchmark surface (`decompile-all`) never enables it. Enable the Listing by default on the decompile-all path (opt-out preserved); measure whole-binary wall-time delta. | **very large** — the noreturn overrun poisons the victim function AND every swallowed neighbor; most of the 296-case kuna-specific bucket |
| F2 | `noreturn_propagate` robustness (existing option) | small | ssh_tun_confirm, spawn_shell, pkg_infodb_remove_file | Three angr-anchored relaxations of `s1_noreturn_propagate` (angr `CFGBase._determine_function_returning`): (a) CFG-closure instead of the address-last-instruction rule (cold tails after the call), (b) looping-noreturn wrapper bodies (exec/longjmp retry loops, no ret), (c) reach-propagation through wrappers defeated by nop padding. | medium |
| F3 | `whiledo_overflow` is_complex port fix | small, **correctness** | lookup_flag_data_by_name | kuna emits **INVALID C** (a `return` inside `while(...)` parens): `is_complex` (s8_structure/blockaction.rs:2001 + region_structurer.rs:821) wrongly delegates BlockList/BlockIf to their front leaf; upstream `FlowBlock::isComplex()` returns true for them → overflow syntax. Restore upstream dispatch. | broad — any while-loop whose condition block is compound; also un-breaks Joern parsing of kuna output |
| F4 | `returnsplit_eager` (return duplication without gotos) | small | factor, parse_ilname, ext4_fc_handle_add_extent | Ported `ActionReturnSplit` only splits shared RETURN blocks reached by **gotos**; angr's `ReturnDuplicatorLow` duplicates shared returns generally, restoring dual-return source shapes (and unlocking `foldcallret` on the freed condition). Also stops `rule_block_or` over-condensation swallowing early-return guards (factor). | medium — classic O0 guard shapes corpus-wide |
| F5 | `iteregion` (ITE diamond → ternary) | small-medium | print_link_flags (+ partially output_one_dumb_line) | Port angr's `ITERegionConverter` core schema: ConditionalJump with two single-assignment children converging on one tail → `x = c ? a : b`, then fold single-use temps into call args. kuna has no ternary-recovery knob today (`branchflip` normalizes these very diamonds but can't collapse them). | large on O0 flag/format-style code (~20 diamonds in one function here) |
| F6 | `cstyle-null-cmp` emission (Joern-parse fix) | small | shell_initialize | kuna's rendering of a NULL-compare form breaks Joern's parse (kuna output is CFG-identical to angr yet scores 72) — fix the emission form so standard C parsers read it. | broad metric lift + LLM-readability |
| P1 | `[PROPOSAL]` full ITE/expr-folding pipeline | proposal | output_one_dumb_line | Full ITERegionConverter + expression folding + the S6 over-merge fix (two different conditions printed from one merged variable). Draft PR after F5 lands to size the residual. | large |
| P2 | `[PROPOSAL]` ARM Cortex-M discovery | proposal | (from WS-1) | Entry seeding/vector tables, Thumb prologue patterns, TMode context (engine already decodes ARM — u-boot proof). | ~90 cps binaries |
| D2 | decbench metric hardening (decbench repo) | small | ce_inc_search, tok_wline, packages, print... 11 artifact cases | (a) source-CFG matcher picks wrong TU / a prototype → `source_nodes<=1` must mean "no usable source CFG", not a scorable 1-node graph; (b) detect truncated decompilations (angr prologue-pattern mis-split: `write_fatal_details` starts at entry+6) instead of rewarding them with GED 0; (c) investigate the `filesavespackage` drift (recorded 99, recomputes 25). | benchmark integrity — 10+ of 28 sampled cases were scoring artifacts |

## Status (shipped)

| Feature | PR | State | Notes |
|---|---|---|---|
| F1 `benchlisting` | #121 | ✅ merged | DIV-15; 13 cases → GED 0 |
| F3 `whiledo` isComplex | #122 | ✅ merged | correctness (invalid C → valid) |
| F2 `noreturn_error` | #126 | ✅ merged | DIV-16; `error(nonzero,…)` wrapper slice — sip GED 347→0. The (a) CFG-closure / (b) looping-wrapper / (c) nop-padding relaxations remain a follow-up. |
| F4 `returndup` | #124 | ✅ merged | angr `ReturnDuplicatorHigh`, `returndup` option (default-off opt-in) |
| F5 `iteregion` | #125 | ✅ merged | **DIV-17, default-ON** (per user directive); GED 140→11; 0/675, −2.6% speed; still `--option iteregion off` per-function |
| ghidra-beats-kuna triage + runtime-choice registry | #123 | ✅ merged | `docs/decbench/triage-ghidra/`, `docs/decbench/runtime-choices.md` |
| F6 `cstyle-null-cmp` | — | open | flippable-emission follow-up |
| P1 full ITE/expr-folding | — | proposal | size residual after F5 (`output_one_dumb_line`) |
| P2 ARM Cortex-M | — | proposal | discovery-tier |

## Sequencing (original plan)

1. **F1 + F3 + D2 first** (all small, independent): F1 is the single biggest kuna win;
   F3 is a correctness bug (invalid C); D2 restores trust in the metric.
2. **F2, F4** next (both extend existing pass families; F2 rides the existing
   `noreturn_propagate` option).
3. **F5** then **P1** sizing; **P2** drafted anytime.

Every PR follows docs/decbench-loop.md: option registered (or existing option
extended), two-pass stage test, `timeit` speed block, `rescore.py --record --siblings`
GED block, default-on only if the 0/675 ablation + ≤5% speed policy allows,
DIV entry + per-test opt-outs when default-on.
