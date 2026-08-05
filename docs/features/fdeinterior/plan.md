# fdeinterior - implementation plan

Owning phase: **P1 code/data partition** (`docs/spec/01-program-prep.md` section
1.5). Option: `fdeinterior`, tier `analysis`, `change_kind`
`analysis-enablement`, default-**ON** (DIV-61).

## 1. The pass - `analyzers/entry/kuna_fdeinterior.rs` (new module)

A child module of `entry/`, so it reuses that module's private `.eh_frame` reader
(`read_u32`/`read_u64`, `parse_cie_fde_encoding`, `encoded_size`,
`decode_eh_pointer`) and its funcsym set (`existing_function_addrs`) without
widening any API.

* `scan_fde_bodies(file) -> Vec<(u64, u64)>` - `scan_eh_frame_starts`'s record
  walk, extended by the one field it skips. `pcRange` sits immediately after
  `pcBegin` in the CIE's `R` encoding but is a LENGTH, so it is decoded with the
  format bits only (`enc & 0x0f`), never the pc-relative application. Zero-length
  and malformed records are dropped; an overlapping result set drops the whole
  stream.
* `eligible_fde_bodies(file, bytes)` - the single-function filter: no other named
  function start strictly inside, no other FDE `pcBegin` strictly inside, no
  overlap with a linker-stub section. This is what keeps the linker's one
  whole-PLT FDE from deleting every import.
* `is_interior(bodies, vma)` - one binary search; the ranges are sorted and
  disjoint by construction.
* `suppress_interior_entries(entries, bodies)` - `Vec::retain`, returning what it
  dropped.
* `FdeInteriorPass` (`id() == "fdeinterior"`, `Phase::P1`) - reports the eligible
  bodies in the new `AnalysisOutput::fde_bodies` field.

## 2. Wiring

* `analyzers/entry/mod.rs`: `pub mod kuna_fdeinterior;`.
* `pass.rs`: new `AnalysisOutput::fde_bodies` field + its `merge` arm.
* `passes.rs`: register `FdeInteriorPass` after the other entry passes.
* `kuna-console/src/engine.rs`: the `"fdeinterior" => arch.analysis_fdeinterior`
  gate arm, and ONE call to `suppress_interior_entries` on the fully merged output
  immediately before `commit_analysis_output`. The placement is load-bearing: the
  deferred Listing consumers (`aif`) merge after the load-time passes, and one of
  the three producers is `aif`.
* `architecture.rs` / `options.rs` / `phases.toml`: the option, default `on`.

The stash-at-load / gate-at-commit shape is the established one for an
output-changing analysis flag (`eh_frame_full`, `cortexmvectors`): with the gate
off the fact stream is dropped and `fde_bodies` is empty, which makes the
suppression a literal no-op.

## 3. Tests

`decompiler/crates/kuna-console/tests/verify_fdeinterior.rs`, two-pass over the
vendored `eh_lsda_x86_64` fixture (`guarded` = FDE `[0x4012d6, 0x40137a)`, catch
dispatch at `0x4012e2`), with `eh_frame_full` on in both arms:

1. `fdeinterior off` - `sub_4012e2` registers and decompiles (the bug, and the
   proof that off restores the old set),
2. default - `sub_4012e2` is absent while the FDE start `sub_4012d6` still
   decompiles (the fix),
3. no named function is lost (the PLT property).

Plus unit tests in the module for the strictness of `is_interior` and the
empty-bodies no-op. No `tests/stages/*.xml`: the XML `<binaryimage>` datatest path
never runs the analyzer tier, so the corpus cannot express this - the same reason
`tailcallentry` (#259) and `cortexmvectors` shipped cross-crate e2e tests instead.

## 4. Counts and docs

`kuna_phases/tests.rs` (settables 90 -> 91, analysis tier 29 -> 30, catalog rows
89 -> 90 -- derived from a fresh catalog capture, never incremented),
`catalog_bytecompat.rs` (90 -> 91) and the regenerated
`tests/fixtures/phase_catalog.json`; `docs/options.md` regenerated;
`docs/spec/01-program-prep.md` section 1.5 gains the prose; DIV-61 in
`docs/history.md`.
