# Analysis — `ptx.o::fix_output_parameters` (angr `test_decompiling_ptx_fix_output_parameters`)

## TL;DR

The target binary `ptx.o` is a **relocatable ELF object file** (`ET_REL`, "ELF 64-bit LSB
relocatable"). angr decompiles `fix_output_parameters` (and every other function) fine.
**kuna produces no output at all** — not worse output, *zero* output:

```
[decomp]> load function fix_output_parameters
Execution error: Unable to load 512 bytes at r0x00000660
[decomp]> decompile
Execution error: No function selected
```

This is **not a decompiler-pass quality gap**. It is a **loader-tier gap**: kuna's ELF
`LoadImage` backend cannot map the bytes of a relocatable object, so the SLEIGH front-end
has nothing to lift.

## Reproduction

```
kuna decompile /home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/ptx.o fix_output_parameters
# -> Execution error: Unable to load 512 bytes at r0x00000660
```

Every function fails identically (`compare_words` @0x10, `unescape_string` @0x1b0,
`digest_word_file` @0x550, `print_spaces` @0x1590, …) — i.e. the whole image is unmapped,
not just this function.

## Root cause (pinned to one line)

`decompiler/crates/kuna-analysis/src/loadimage_object.rs`

```rust
// Snapshot the loadable segments (PT_LOAD), copying their RAM bytes.
let mut segments: Vec<Segment> = Vec::new();
for seg in file.segments() {          // <-- line ~205
    ...
    segments.push(Segment { vma, data: data.to_vec() });
}
```

The loader's *unit of mapping is the ELF loadable segment* (`PT_LOAD`), faithfully matching
the C++ `LoadImageBfd` for a real process image (header note, lines 15–27, and the explicit
"PARTIAL scope", lines 46–52). A **relocatable `.o` has no program headers and therefore no
`PT_LOAD` segments** (`readelf -l ptx.o` → "There are no program headers in this file").
So `file.segments()` yields nothing, `segments` stays empty, `find_section`/`loadFill` find
no containing region, and `loadFill` hits its "initial address not mapped → break →
DataUnavailError" path → **"Unable to load N bytes at …"**.

The symbol table *is* read (`read symbols` resolves `fix_output_parameters` → `0x660`); only
the **bytes** are missing.

## What angr does better

angr's CLE loader has a dedicated `ELF`/relocatable backend that, for `ET_REL`:

1. **Lays out the allocatable sections** (`SHF_ALLOC`: `.text`, `.data`, `.rodata`, `.bss`,
   `.text.startup`, …), assigning each a non-overlapping load address (a synthetic base,
   commonly `0x400000`), instead of leaving every section at its on-disk section address `0`.
2. **Applies the relocations** from `.rela.text` / `.rela.*` so intra-object calls, PC-relative
   `lea`s of globals, and absolute pointer slots resolve to the laid-out addresses rather than
   to `0`.
3. **Binds local + global symbols** to those addresses, which is why the angr listing reads
   `reference_max_width`, `input_file_name`, `line_width`, `truncation_string`, `xmalloc`,
   `__sprintf_chk`, `strlen`, … instead of raw offsets.

The full angr rendering of `fix_output_parameters` is in `angr-vs-kuna.txt` (kuna side is the
load error).

## Owning stage

This is **S1 / loader** (`kuna-analysis`, the "Run Analysis" / program-prep tier), *upstream of
the decompiler stage model* — `decompiler/crates/kuna-analysis/src/loadimage_object.rs`
(`docs/stage-mapping.md`: loader/markup tier). It is **not** in the `kuna-decomp` S1–S9 pass
pipeline, so it cannot be modeled as one option-gated `Action`/`Rule` à la
`kuna_loweredswitch.rs`.

## Why this is a PROPOSAL, not a one-pass feature

Closing this gap requires loader **infrastructure**, all in the analysis tier, not a single
gated decompiler pass:

- a new **`ET_REL` section-layout** path in `loadimage_object.rs` (synthesize segments from
  `SHF_ALLOC` sections when `file.segments()` is empty), **plus**
- **relocation application** (`.rela.*` → patch the snapshotted bytes), **plus**
- **symbol rebasing** so funcsyms/datasyms point at the laid-out addresses.

Without (2)+(3), mapping the sections alone yields a function whose every call/global-ref
targets `0` — i.e. still not the angr-quality result the testcase wants. This is multi-part,
touches a ported-core loader file structurally (well beyond a single gated early-return), and
introduces a *capability* rather than a presentation/structure toggle. Per **Hard rule 7** this
is a **large** feature → `[PROPOSAL]` draft PR for human go/no-go. See `proposal.md`.
