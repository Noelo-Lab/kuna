# [PROPOSAL] Relocatable-object (`ET_REL`) loader support — `ptx.o::fix_output_parameters`

**Opportunity:** `test_decompiling_ptx_fix_output_parameters::fix_output_parameters`
**angr reference:** CLE `ELF` backend relocatable-object handling (section layout + `.rela.*`
relocation application + symbol binding), angr 9.2.213.
**Status:** large feature — human go/no-go requested before any implementation worker is spent.

## The problem

`ptx.o` is a **relocatable ELF object** (`ET_REL`; `readelf -l` → "There are no program
headers in this file"). kuna's ELF `LoadImage` backend
(`decompiler/crates/kuna-analysis/src/loadimage_object.rs`) builds its memory map **only from
`PT_LOAD` segments** (`for seg in file.segments()`, ~line 205). A `.o` has no program headers,
so the map is empty and **every** function fails to lift:

```
[decomp]> load function fix_output_parameters
Execution error: Unable to load 512 bytes at r0x00000660
```

kuna therefore emits **zero** output for `fix_output_parameters` (and all of `ptx.o`), while
angr decompiles it fully with resolved symbol names. Full analysis: `analysis.md`. Captured
side-by-side: `angr-vs-kuna.txt`.

This is a **loader-tier capability gap**, not a decompiler-pass quality gap. It is *not*
expressible as one option-gated `kuna_*.rs` Action/Rule (those live in `kuna-decomp` S1–S9,
downstream of the loader). The scope decider returned **large** (recorded in `record.json`).

## Why it cannot be the normal single-pass worker feature

1. The fix is in `kuna-analysis` (the loader), upstream of the `kuna-decomp` pass pipeline —
   there is no `Action`/`Rule` seam and no architecture-struct flag the decompiler honors here.
2. Mapping the `SHF_ALLOC` sections alone is insufficient: without relocation application every
   intra-object `call`, every PC-relative `lea` of a global, and every absolute pointer slot
   resolves to `0`, so the output is still not angr-quality.
3. It structurally rewrites a ported-core loader file (`loadimage_object.rs`) well beyond a
   single gated early-return, and adds new data (a relocation model) — Hard-rule-7 LARGE on
   multiple counts.

## Proposed implementation plan (multi-step, for the approved implementation worker)

All in `decompiler/crates/kuna-analysis/` (the `object` crate already exposes everything
needed: `Object::kind()`, `ObjectSection`, `RelocationKind`, `ObjectSymbol`).

1. **Detect `ET_REL`.** In `ObjectLoadImage::open`, when `file.segments()` is empty *and*
   `file.kind() == object::ObjectKind::Relocatable`, take a new layout path instead of the
   `PT_LOAD` snapshot.

2. **Synthesize a section layout.** Assign each `SHF_ALLOC` section
   (`SectionFlags::Elf{ sh_flags } & SHF_ALLOC`, i.e. `.text`, `.text.startup`, `.rodata`,
   `.data`, `.bss`, …) a non-overlapping load address above a synthetic base
   (`0x400000`, matching angr's default so the testcase's `0x400660` lines up), respecting
   each section's alignment. Record a `section-index → load-vma` map. Snapshot
   `PROGBITS`/code/data bytes as `Segment`s; zero-fill `NOBITS` (`.bss`).

3. **Apply relocations.** For each `.rela.<sec>`, walk `section.relocations()`; for each
   `(offset, symbol, kind, addend)` compute the patched value from the laid-out symbol/section
   addresses and write it into the snapshotted bytes of the target section. Start with the
   x86-64 kinds the corpus needs: `R_X86_64_PC32`/`PLT32` (rel32 calls/leas),
   `R_X86_64_64`/`32`/`32S` (absolute slots). Unhandled kinds → a logged warning + skip (so the
   loader degrades, never panics).

4. **Rebase symbols.** Shift each funcsym/datasym from its section-relative value to
   `section_load_vma + value` so `read symbols` resolves `fix_output_parameters` to the new
   `0x400660`, and so global data references render as `reference_max_width`, `input_file_name`,
   etc.

5. **Tests + gates.** Add a `tests/stages/` decompilertest driven by `ptx.o` (or a trimmed
   self-contained `ET_REL` bytechunk) asserting `fix_output_parameters` now lifts and renders;
   keep `make test` PARITY OK (linked-ELF path untouched), `make rust-test` green,
   `make test-stages` clean. Add loader unit tests for the layout + each relocation kind.

## Gating / default decision

The natural gate is **the file type itself**, not a user option: linked `ET_EXEC`/`ET_DYN`
images keep the existing `PT_LOAD` path byte-for-byte; only `ET_REL` (which today produces an
*error*, no output) takes the new path. So this is a pure capability addition with **no
divergence** on any existing-corpus output — but it should still ship behind an
`elf-reloc-objects` option (default decided at implementation time, likely ON since it only
affects files kuna currently cannot load at all) for an explicit off-switch and discoverability.

## Speed / risk

- **Speed:** layout + relocation patching is a one-time O(sections + relocations) pass at load;
  negligible vs decompilation. No per-decompile cost. No impact on the linked-ELF fast path.
- **Risk:** contained to the loader; the linked path is unchanged (guarded on
  `kind()==Relocatable` + empty `segments()`). Main correctness surface is the relocation math
  per `R_X86_64_*` kind — covered by targeted unit tests. Unknown relocation kinds degrade with
  a warning rather than miscompiling silently.

## Proposed option name

`elf-reloc-objects` (`change_kind = structure-recovery` / capability; `source_decompiler = angr`;
`inspiration = "test_decompiling_ptx_fix_output_parameters; CLE ELF relocatable backend; fix_output_parameters"`).
