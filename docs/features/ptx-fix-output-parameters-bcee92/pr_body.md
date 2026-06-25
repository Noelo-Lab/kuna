# `relocobjects`: load ELF relocatable objects (`ET_REL` / `.o`)

Closes the angr-vs-kuna gap `test_decompiling_ptx_fix_output_parameters::fix_output_parameters`
(`ptx.o`, angr 9.2.213). Analysis: [`analysis.md`](analysis.md) · approved proposal:
[`proposal.md`](proposal.md).

## The gap

`ptx.o` is a **relocatable ELF object** (`ET_REL`) with no `PT_LOAD` program headers. kuna's
faithful `LoadImageBfd` port (`kuna-analysis/loadimage_object.rs`) builds its byte map **only** from
`PT_LOAD` segments, so a `.o` mapped **zero bytes** and *every* function failed to lift
(`Unable to load N bytes at ...`) — kuna emitted **no output at all**. angr's CLE ELF relocatable
backend decompiles it fully with resolved names.

## Mechanism

A new kuna file `kuna-analysis/src/s1_loader/elf_reloc.rs`. For an `ET_REL` object
(`kind()==Relocatable` + empty `segments()`), `ObjectLoadImage::from_bytes` takes a new
`from_relocatable` path that:

1. **lays out** each `SHF_ALLOC` section at a non-overlapping VMA above `0x400000` (angr's CLE
   default — `fix_output_parameters` lands at `0x400660`, matching angr), respecting alignment;
   `.bss`/`NOBITS` is zero-filled;
2. **applies** the `.rela.*` relocations into the snapshotted bytes — `R_X86_64_PC32`/`PLT32`
   (`S + A − P`), `R_X86_64_64`/`32`/`32S` (`S + A`); an unhandled kind degrades with a logged
   warning rather than miscompiling;
3. **rebases** defined symbols to their load VMA and **binds undefined externs** (PLT-relative call
   targets are named even when gcc emits them `STT_NOTYPE`) to synthetic addresses, so calls render
   by name (`strlen(...)`, `__sprintf_chk(...)`, `__ctype_b_loc(...)`, …).

The linked `ET_EXEC`/`ET_DYN` `PT_LOAD` path is untouched (byte-identical).

## Option

`relocobjects` (default **on** — it only affects `.o` files, which the loader otherwise cannot load
at all). The loader runs at `load file`, **upstream** of the per-function option machinery, so the
toggle is bridged to the loader by the `KUNA_RELOC_OBJECTS` (`RELOC_OBJECTS_ENV`) process env var
that `Architecture::set_kuna_option` and `kuna decompile` write:

```
kuna decompile ptx.o fix_output_parameters                       # default: loads + decompiles
kuna decompile ptx.o fix_output_parameters --option relocobjects off   # upstream PT_LOAD-only loader (errors)
KUNA_RELOC_OBJECTS=0 kuna decompile ptx.o fix_output_parameters         # same, via env
```

Discoverable via `kuna catalog --json` (`source_decompiler: angr`, `change_kind:
structure-recovery`). New divergence **DIV-7**.

## Ablation / parity / speed

- **0 of 675** upstream datatest assertions change — the XML datatest path never constructs an
  `ObjectLoadImage`, and the linked-ELF path is byte-identical. `make test` → **PARITY OK**.
- `make test-stages` → **PARITY OK** (159/159; the two `kuna-catalog.xml` provenance counts bumped
  3→4 `angr` / 2→3 `structure-recovery` for the new settable).
- **Speed within budget**: median `fix_output_parameters` decompile `−6.97%` (on vs the off/error
  path), well under the +5% budget; the layout+relocation work is a one-time O(sections+relocs) pass
  at load.

## Tests

Cargo workspace (`make rust-test`) — **no XML stage test is possible** for a loader feature (the
datatest path bypasses `ObjectLoadImage` entirely):

- `s1_loader::elf_reloc` — a hand-assembled `ET_REL` exercising section layout + each relocation
  kind + defined/extern symbol binding;
- `loadimage_object` — the vendored real `ptx.o` fixture: `fix_output_parameters` rebases to
  `0x400660`, its bytes load, and externs (`strlen`/`dcgettext`/`error`) resolve.

> Note: `make rust-test` has one **pre-existing** failure on this branch
> (`verify_w10_proto_unlock`'s const-return direction-check, a stale DIV-6 oracle assertion) that
> fails identically on the base commit and is unrelated to this feature; left untouched.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
