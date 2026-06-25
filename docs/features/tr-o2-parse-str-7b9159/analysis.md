# analysis — tr-o2-parse-str-7b9159

- **Opportunity (angr testcase):** `test_decompiling_tr_O2_parse_str::parse_str`
- **Binary:** `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/tr_O2.o`  (arch `x86_64`)
- **Selector:** `parse_str` @ section-offset `0xa20`, size 2082 bytes

## What angr does better

angr (via CLE) produces a full, correct decompilation of `parse_str` with **resolved
external calls** — `xmalloc`, `strncmp`, `dcgettext`, `error`, `make_printable_str`,
`quote`, `xstrtoumax`, `free`, … — and correct control flow. See
[`angr-vs-kuna.txt`](./angr-vs-kuna.txt).

kuna produces **no output at all**. `decomp_dbg`:

```
[decomp]> load file .../tr_O2.o
.../tr_O2.o successfully loaded: x86:LE:64:default:gcc
[decomp]> read symbols
[decomp]> load function parse_str
Execution error: Unable to load 512 bytes at r0x00000a20
[decomp]> decompile
Execution error: No function selected
```

This is not a *structuring* gap — kuna cannot **load the bytes** of the function.

## The exact construct / root cause

`tr_O2.o` is an **ELF `ET_REL` relocatable object** (`.o`):

```
$ readelf -h tr_O2.o   →  Type: REL (Relocatable file);  Number of program headers: 0
$ readelf -l tr_O2.o   →  There are no program headers in this file.
$ readelf -S tr_O2.o   →  .text  PROGBITS  addr 0x0  size 0x1821   (every section has sh_addr = 0)
$ readelf -sW tr_O2.o  →  parse_str  value 0xa20  size 2082  FUNC LOCAL  (section .text)
```

kuna's `ObjectLoadImage` (the `LoadImageBfd` analog) builds its byte map **only from
`PT_LOAD` program-header segments**:

- `decompiler/crates/kuna-analysis/src/loadimage_object.rs:204` —
  `for seg in file.segments() { … segments.push(Segment { vma, data }) }`.
- A relocatable `.o` has **no program headers**, so `file.segments()` is empty →
  `segments` is empty → `find_section` (`loadimage_object.rs:334`) always returns
  `None` → `load_fill` (`loadimage_object.rs:375`) breaks on the first byte
  ("Initial address not mapped") → `Unable to load N bytes`.
- Sections are snapshotted (`loadimage_object.rs:224`) only for the info/flags walk; the
  loaded byte image never consults them. There is **no relocation application** anywhere.

So kuna can load **no** function from **any** `ET_REL` object, not just `parse_str`.

## Owning stage / tier

This is entirely in the **analysis/loader tier** (`kuna-analysis`, stage S1 loader —
`docs/stage-mapping.md` → the loadimage/markup layer), specifically
`loadimage_object.rs` (the upstream `LoadImageBfd` analog). **No S1–S9 decompiler stage
owns this**; there is no Action/Rule in `kuna-decomp` that could fix it via a gated
early-return.

## Hypothesis for the kuna change

When `file.segments()` yields nothing, build the byte map from **allocatable
PROGBITS/NOBITS sections**, laying each section out at a distinct non-overlapping base
(all `sh_addr == 0` in `ET_REL`, so a layout must be chosen), record per-section bases so
symbol vmas map consistently, then **apply ELF relocations** (`.rela.text`:
`R_X86_64_PC32`/`PLT32`/`64`/`32S`, with synthesized PLT/GOT stubs for undefined externs)
by patching the loaded bytes. Only then will `parse_str`'s call targets resolve the way
angr's do.

## Scope verdict — **LARGE** (proposal fork, Hard Rule 7)

This is **loader infrastructure**, not one option-gated `kuna-decomp` Action/Rule:

- It needs a **new code path + subsystem** in the loader (section layout + a relocation
  engine + consistent symbol-address mapping) — a new pass *type*, >1 new module — which
  trips Hard Rule 7's "new infrastructure" / ">1 new module" triggers.
- It lives in `kuna-analysis`, not `kuna-decomp`; the feature template
  (`kuna_*.rs` Action + Architecture flag + `universalaction.rs` registration) does not
  apply.
- It carries real design choices (layout policy, PLT-stub synthesis, per-arch relocation
  tables) that warrant human go/no-go.

A decider subagent independently confirmed `scope: large` (recorded verbatim in
`record.json`). Per the protocol, this stops at design with a `[PROPOSAL]` draft PR.
