# undefname -- implementation plan and measurements

Root cause and design rationale: `analysis.md`. This file is the plan as executed plus
the numbers behind it.

## What changed

| file | change |
|---|---|
| `decompiler/crates/kuna-decomp/src/p6_variables/kuna_undefname.rs` | **new** -- the whole fix: `finish_undefined_names` = `assign_scope_default_names` + `refresh_placeholder_high_names` |
| `decompiler/crates/kuna-decomp/src/p6_variables/mod.rs` | declare the module |
| `decompiler/crates/kuna-decomp/src/p6_variables/varmap.rs` | `ScopeLocal::assign_default_names`, a one-line accessor onto the already-ported `Database::assign_default_names` |
| `decompiler/crates/kuna-decomp/src/p6_variables/coreaction_cleanup.rs` | one call at the tail of `name_local_highs_angr`, after the spacebase attach pass |
| `decompiler/crates/kuna-base/src/xml.rs` | corpus file count 190 -> 191 (counted from disk, not arithmetic) |
| `tests/stages/ghdec-undefname.xml` | **new** -- two witnesses, two passes, 8 assertions |
| `docs/baseline-stages.json` | re-recorded from a live run: 386 -> 394 keys, purely the 8 new ones |
| `docs/spec/06-variables-and-merge.md` | section 6.1, "Closing out the undefined names" |
| `docs/history.md` | DIV row |

No option, no `phases.toml` row, no catalog-count bump: an identifier containing `$$`
is not valid C under any naming policy, so there is no decision to expose.

## Decisions

1. **Repair the caches; do not move the gate.** Dropping
   `resolve_default_name_override`'s `sym_off == 0 && size == entry_size` gate so the
   first touch always renames would also fix the render, but it moves *when* a Symbol
   consumes `base` and therefore renumbers every later local in any function where a
   partial high precedes its whole-cover sibling. The gate is also a faithful port of
   C++'s `high->getSymbolOffset() < 0`. Repairing at the tail is the zero-churn form.
2. **Run the tail *after* the spacebase attach pass**, not before. Running the
   catch-all first would give previously-unnamed Symbols a name in time for
   `link_symbol_reference`'s `is_name_undefined` bail to stop firing, turning
   suppressed `&symbol` references into `&vN` renders. That may well be an improvement,
   but it is a second, separable behaviour change; keeping the tail last leaves every
   `&symbol` render byte-identical.
3. **Match on the placeholder shape, not on "re-read every name".** Re-reading the
   Symbol for *all* highs would be closer to upstream's printer but would also overwrite
   names bound by the proto-partial, global and dynamic-symbol paths. Restricting to
   `Symbol::is_name_undefined`'s exact shape (`len == 15 && starts_with("$$undef")`)
   can only ever replace a string that is not a valid C identifier.
4. **Rewrite the name only.** The in-symbol offset and symbol type were bound from the
   same SymbolEntry the re-query returns, so re-setting them would be a no-op; leaving
   them alone keeps the change provably scoped to the identifier.
5. **No option.** `AGENTS.md`: "a strict bug fix that only corrects wrong output needs
   no flag." The ablation agrees -- 0 of 675 upstream assertions move.

## Ablation

`kuna test --all --baseline docs/baseline.json` -> **PARITY OK, 675/675**, with the fix
unconditionally on and `docs/baseline.json` untouched. `make test-stages` -> **PARITY
OK, 394/394**, the stages baseline re-recorded only to add this test's 8 keys (the diff
is 8 added lines and the footer count; no existing key moved).

## Corpus breadth

Stratified sample: one binary per project per optimisation level over the decbench
corpus -- **116 stripped binaries, 90,808 functions**, x86-64 and ARM Cortex-M, `O0` /
`O2` / `O2-noinline`. Both arms are the same tree with and without the four-file change, rebased onto
`208bb155` (so the numbers include #249 through #254), with the per-function watchdog
disabled so a timeout under load cannot blank a function in either arm.

| measure | before | after |
|---|---|---|
| binaries with at least one leaked identifier | 73 | **0** |
| functions with at least one leaked identifier in the emitted C | 371 | **0** |
| total `$$undefNNNNNNNN` occurrences in emitted C | 2142 | **0** |
| of those, reads | 1503 | **0** |
| of those, writes | 639 | **0** |
| functions whose ScopeLocal Symbol table carries a placeholder | 93 | **0** |
| placeholder Symbols in those tables | 125 | **0** |
| functions produced (both arms) | 90808 | **90808** |
| binaries that errored | 0 | **0** |

The **blast radius** is the same set of functions and nothing else. Twelve binaries
(the eight leakiest plus four that never leaked, as controls) were decompiled whole with
both builds and diffed function by function:

| binary | functions | changed | changed **without** a placeholder in the before text |
|---|---|---|---|
| `tar` | 1125 | 13 | 0 |
| `ip` | 1008 | 38 | 0 |
| `e2fsck` | 1421 | 21 | 0 |
| `bash` | 2538 | 13 | 0 |
| `tar` | 1585 | 9 | 0 |
| `ip` | 1912 | 33 | 0 |
| `e2fsck` | 1909 | 21 | 0 |
| `betaflight_STM32F405.elf` | 5797 | 14 | 0 |
| `update-passwd` | 98 | 0 | 0 |
| `bzip2` | 160 | 0 | 0 |
| `ch.elf` | 770 | 0 | 0 |
| `CMSIS_DAP.axf` | 267 | 0 | 0 |
| **total** | **18590** | **162** | **0** |

Every function that changed had a `$$undef` identifier in its before text, and the
function *set* is identical in both arms (the diff asserts it), so nothing stopped
being produced and nothing new appeared.

## Speed

`scripts.pipeline.timeit` flips an **option** on one binary, and this fix has no
option, so it does not apply. The equivalent measurement for a default-path change is
the interleaved two-binary A/B that DIV-52 and DIV-54 used: alternate before/after runs
pair by pair so the box's drifting load cancels, and take the median of each arm.
`decompile-all --mode reliable`, whole binary:

| binary | before (ms) | after (ms) | delta | pairs |
|---|---|---|---|---|
| `tar-O2` | 52,998.0 | 50,085.7 | -5.50% | 7 |
| `ip-O2-noinline` | 30,959.3 | 31,025.3 | +0.21% | 7 |
| `gzip-O2` | 4,733.7 | 4,891.3 | +3.33% | 7 |

The added work is one scope walk plus one high walk per function, and the high walk
exits immediately on the overwhelming majority of functions (no high carries a
placeholder), which is what the measurement shows.

## Benchmark (GED)

GED scores CFG shape, and renaming an identifier cannot change a control-flow graph, so
the expected delta is exactly **0**. The item was mined from the NOVEL pool precisely
because it does not rank by margin: it is worth doing for the 221-in-283 functions whose
C reads an uninitialised variable, not for the metric. See `record.json` for the
recorded rescore.

## Stage test

`tests/stages/ghdec-undefname.xml` carries both mechanisms as real bytes -- `tar`
`xstrcat` (the stale cache) and `ip` `set_qlen` (the unwired catch-all) -- with their
callees stubbed. Pass 1 is the default pipeline; pass 2 sets `option dedupvardecls off`
to pin that neither witness depends on the P9 declaration collapse.
