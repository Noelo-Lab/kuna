# [PROPOSAL] Relocatable ELF object (`ET_REL` `.o`) loading

**Opportunity:** `test_decompiling_tr_O2_parse_str::parse_str`
**Binary:** `binaries/tests/x86_64/decompiler/tr_O2.o` (ELF `ET_REL`, x86_64)
**Proposed option:** `relocatable_object_loader` (default off until validated)
**Scope:** LARGE — loader infrastructure (Hard Rule 7). Stops at design for human go/no-go.

## The problem

kuna cannot decompile any function from an ELF **relocatable object** (`.o`,
`e_type == ET_REL`). For `tr_O2.o::parse_str`, `decomp_dbg` fails at load time:

```
[decomp]> load function parse_str
Execution error: Unable to load 512 bytes at r0x00000a20
```

angr (via CLE) loads the same object and produces a complete, correct decompilation with
all external calls resolved (`xmalloc`, `strncmp`, `dcgettext`, …). Full side-by-side and
root-cause in [`analysis.md`](./analysis.md).

### Root cause (verified)

`kuna-analysis/src/loadimage_object.rs` builds its loadable byte map **exclusively from
`PT_LOAD` program-header segments** (`for seg in file.segments()`, line 204). An `ET_REL`
object has **no program headers** (`readelf -l` → "There are no program headers"), so the
segment list is empty, `find_section` always returns `None`, and every byte load fails.
Sections are snapshotted for flags only (line 224) and are never consulted for bytes.
There is no relocation application anywhere in the tree.

## angr reference

CLE's `cle.backends.elf` (the static ELF backend) plus `cle.backends.elf.relocation`:
for `ET_REL` it maps each allocatable section into the load space at a distinct base and
then applies the section's relocations (`R_X86_64_PC32`, `PLT32`, `64`, `32S`, …),
synthesizing extern/PLT stubs for undefined symbols. angr's decompiler then sees a fully
linked-in-memory image, which is why its `parse_str` call targets resolve.

## Proposed implementation plan (multi-step)

All in `kuna-analysis` (loader tier); gated behind `--option relocatable_object_loader`,
default OFF, so existing `ET_EXEC`/`ET_DYN` output stays byte-identical.

1. **Detect `ET_REL` / empty-segment case.** In `ObjectLoadImage::new`, when
   `file.segments()` is empty (or `file.kind() == Relocatable`), enter a new section-based
   layout path instead of the segment path.
2. **Section layout (new module `reloc_layout.rs`).** Assign each allocatable
   `PROGBITS`/`NOBITS` section a distinct, non-overlapping base address (chosen base, e.g.
   `0x400000 + cumulative`, page-aligned). Record a `section_index → base` map so symbol
   vmas (`st_shndx` + `st_value`) translate consistently. Build the `segments`/`sections`
   byte map and the `funcsyms` list from this layout (`parse_str` lands at
   `text_base + 0xa20`).
3. **Relocation engine (new module `reloc_apply.rs`).** Parse `.rela.<sec>` and apply the
   x86_64 relocation types present (`PC32`/`PLT32`/`64`/`32S`/`32`), patching the loaded
   bytes. For undefined externs (`xmalloc`, `strncmp`, `dcgettext`, `error`, …) synthesize
   a small PLT/extern-stub region and point the relocations at named stub addresses so the
   decompiler renders the call names. Per-arch reloc tables behind a seam (x86_64 first).
4. **Symbol/markup consistency.** Ensure the strings/DWARF/entry walks in
   `kuna-analysis/src/s1_*` use the same per-section bases (they already key off the
   loadimage symbol/section info, so feeding the layout through is the main work).
5. **Validation.** A `tests/stages/ghangr-tr-o2-parse-str-7b9159.xml` two-pass test:
   off ⇒ current "Unable to load" / empty; on ⇒ `parse_str` renders with resolved calls.
   Plus a regression check that an existing `ET_EXEC` ELF datatest is byte-identical with
   the flag on (the flag must be inert for non-`ET_REL` inputs).

## Speed / risk assessment

- **Speed:** load-time work only (section layout + a single relocation pass), bounded by
  reloc count; negligible vs decompilation. The target is currently *unmeasurable* (kuna
  emits nothing), so there is no regression risk to existing binaries — the flag is inert
  unless the input is `ET_REL`.
- **Risk (moderate–high):**
  - Must not perturb the existing `PT_LOAD` path — default output byte-identical for all
    current binaries (gate strictly on `ET_REL` + flag).
  - Relocation tables are per-architecture; start x86_64-only with a clear seam, others
    return "unsupported" rather than mis-patching.
  - Layout/PLT-stub address choices propagate to every downstream symbol/xref; a
    consistency bug **silently corrupts** call resolution rather than failing loudly →
    needs the byte-for-byte regression gate above and careful review.
  - This is the first kuna feature that lays down *synthetic* extern stubs; the address
    policy should be reviewed so it doesn't collide with real section bases.

## Why this needs go/no-go

It is a new loader subsystem (>1 new module, a new pass type, design choices around
layout/stub policy and per-arch reloc tables) — outside the one-Action/Rule feature model
and squarely a Hard Rule 7 "large" feature. Requesting human approval before an
implementation worker is spent on the branch.
