# cortexmvectors — implementation plan

Owning phase: **P1 code/data partition**, entry discovery oracle 6
(`docs/spec/01-program-prep.md`). Option: `cortexmvectors`, tier `analysis`,
`change_kind` `analysis-enablement`, default-OFF.

## 1. The widened signature — `entry/kuna_cortexmvectors.rs` (new module)

* `vector_table(file, widen)` — the shipped signature first, the widened scan only as a
  fallback. That ordering is the "never removes an entry" guarantee.
* `widened_scan(file)` — ARM-gated; candidates are `executable_sections` then
  `phdr_executable_sections` then `other_allocated_sections` (the new tier), preserving the
  shipped priority order so an already-matching image keeps its section.
* `is_vector_table(sec_addr, data, le, in_exec)` — `word[0]` in
  `0x1000_0000..=0x3FFF_FFFF`, and `harvest_vector_slots(..., relocated = true)` yields
  >= `MIN_CONFORMING_HANDLERS` (3) slots.

## 2. The harvest split — `entry/mod.rs`

* `cortexm_vector_table` -> `cortexm_vector_table_shipped` (unchanged body); the new
  `cortexm_vector_table(file, widen)` delegates to the module above.
* `harvest_vector_words` keeps its sorted/deduped contract and now wraps a new
  `harvest_vector_slots`, which returns one entry per accepted **slot** in table order. The
  signature counts slots; the oracle seeds from the deduped set.
* `harvest_vector_slots` takes `relocated: bool`. When set, the start-of-code stop only
  applies if the lowest handler is at or above the table base (the betaflight RAM-linked
  table). Clear on the shipped path, so option-off behaviour is byte-identical.
* `cortexm_vector_entries` / `cortexm_thumb_paints` take the same `widen` flag; every
  existing caller passes `false`.

## 3. Wiring — `CortexMVectorsPass`

A standalone `AnalysisPass` (`id() == "cortexmvectors"`), registered in
`passes::passes_for` right after `EntryDiscoveryPass`, gated at commit by
`engine.rs::analysis_pass_enabled`. It emits:

* `entries` — the widened harvest, filtered exactly as `collect_entries` filters every
  oracle (non-zero, in an executable section, not already a funcsym);
* `context_paints` — the whole-image `TMode=1` region paint a confirmed table unlocks.

It returns empty when the shipped signature already matched (those facts are `entry_disc`'s),
on non-ARM, and when no widened table is found.

**Rejected, with measurements in `analysis.md` section 4:** reading the flag inside
`EntryDiscoveryPass::run` (load-time passes run before `--option` is applied, so it silently
did nothing), and feeding the widened handlers to `passes::listing_seeds` /
`listing::context::ContextPainter` (both measured as net recall *losses* through the walk's
tail-call absorption).

## 4. Registration

* `Architecture::analysis_cortexmvectors` (field, `reset_defaults` = false, `set_kuna_option`
  arm).
* `p0_knowledge/options.rs` `KUNA_OPTION_NAMES`.
* `phases.toml` `[[settable]]` row, every field populated,
  `source_decompiler = "kuna"` (this came from kuna's own recall measurement, not a rival),
  `inspiration` prefixed `decbench:`.
* Hard-coded catalog counts: `kuna_phases/tests.rs` (settable 83->84, analysis tier 25->26,
  the `PASS_GATES` list, the JSON row count 82->83) and `catalog_bytecompat.rs`
  (83->84 plus the recaptured `tests/fixtures/phase_catalog.json`).
  `tests/stages/kuna-catalog.xml` needs no edit: its exact-count asserts key on
  `source_decompiler = "angr"` and the `structure-recovery` / `opt-in-tool` change_kinds,
  none of which this row joins; the `use_when` / `change_kind` asserts are ranges.

## 5. Tests

* Unit (`kuna_cortexmvectors.rs`): the cleanflight/betaflight shape, the RAM-relocated
  table, the `word[1] != e_entry` shape, the `.data` false-positive floor and the
  two-vs-three handler threshold, the non-ARM no-op, shipped-match monotonicity, and a pin
  on the committed fixture.
* e2e (`kuna-console/tests/verify_cortexmvectors.rs`): two passes over
  `tests/fixtures/cortexm_ccm_vectors_le32` — option off, the four handlers are in no symbol
  table and even the one `e_entry` function produces no C (A32 misdecode); option on, all
  four are registered and every body decompiles to its real constant.

**No `tests/stages/*.xml`.** The oracle keys entirely on ELF structure (`sh_flags`,
`PT_LOAD` flags, `e_entry`, section bytes at a section base). The datatest harness embeds
raw `<bytechunk>`s and never constructs an `ObjectLoadImage`, so the option is unobservable
on that path. Every sibling analysis-tier discovery flag (`funcstart_patterns`, `aif`,
`eh_frame_full`, `i386_pie_plt`) is gated the same way. Consequently
`kuna-base/src/xml.rs`' corpus file count and `docs/baseline-stages.json` are untouched.

## 6. Fixture

`decompiler/crates/kuna-analysis/tests/fixtures/cortexm_ccm_vectors_le32` (357 bytes) plus
its generator `cortexm_ccm_vectors_le32.py` and a fixtures-README row. Hand-assembled
because no cross toolchain on this host emits a bare-metal STM32 link layout; it fails all
three shipped predicates at once, so one fixture covers all four affected projects' shapes.

## 7. Docs

`docs/spec/01-program-prep.md` (the oracle-6 prose, in the same section as the shipped
signature), regenerated `docs/options.md`. No `docs/history.md` DIV row: default-OFF, so
default output is byte-identical.
