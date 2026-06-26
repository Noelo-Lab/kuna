# analysis — ifelseflatten-clientloop-171db8

**Opportunity:** `test_ifelseflatten_clientloop :: client_request_tun_fwd`
**Binary:** `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/decompiler/clientloop.o` (x86_64 ELF **ET_REL** object)
**Selector:** `client_request_tun_fwd` @ `0x405170` (.text+0x5170, 641 bytes)

## TL;DR — the opportunity is mislabeled

This is **not** an "if-else flattening" structuring gap. **kuna emits zero output** for this
function: it hard-fails in the **loader**, long before any S7/S8 structuring runs. The real,
underlying angr-vs-kuna gap is a **loader/analysis-tier** one, so it cannot be closed by the
single decompiler Action/Rule (`kuna_<slug>.rs`, stages S2–S9) this worker is scoped to.
Per Hard rule 7 this is a **large / different-tier** change → **PROPOSAL** (see `proposal.md`).

## What angr does

angr decompiles the function cleanly (see `angr-vs-kuna.txt`). The construct the test name
points at is the deeply-nested `if (!v8) { ...; if (!v8) { ... } }` chain over the
`sshpkt_start` / `sshpkt_put_*` / `sshpkt_send` calls, with a shared `sshpkt_fatal` /
`__stack_chk_fail` failure tail. Critically, angr reaches that output at all because its CLE
loader backs the **extern object** (undefined external symbols) with a **zero-filled, readable**
region — so a read of an undefined-extern global just returns 0 and decompilation proceeds.

## What kuna does

```
[decomp]> load function client_request_tun_fwd
Execution error: Unable to load 512 bytes at r0x0040cb60
[decomp]> decompile
Execution error: No function selected
```

No C output, in both addr-mode (`0x405170`) and name-mode.

## Root cause (pinned)

The function body contains:

```asm
5276:  cmpl   $0x1, 0x0(%rip)        # options+0x1373   (R_X86_64_PC32 -> options)
```

`options` is an **undefined external data global** (ELF `SHN_UNDEF`, `STT_NOTYPE`, `st_value==0`).
kuna's ET_REL loader assigns each *undefined* referenced symbol a **16-byte synthetic slot** in an
"extern area" above the laid-out sections, but **adds no readable backing segment** for that area:

- `decompiler/crates/kuna-analysis/src/s1_loader/elf_reloc.rs:144` — `extern_cursor = align_up(cursor + 0x1000, 0x1000)`
- `decompiler/crates/kuna-analysis/src/s1_loader/elf_reloc.rs:282` — `*extern_cursor = extern_cursor.wrapping_add(16)` (16-byte slot, no segment)

The extern slots feed `funcsyms` (named **call targets**, resolved by name — no bytes ever read),
which is fine for *function* externs (`sshlog`, `tun_open`, …). But `options` is read as **data**:
`options+0x1373` resolves to ≈`0x40cb60`, which is `0x1373` bytes past the 16-byte slot and not
covered by any `Segment`. The LoadImage fill (`loadimage_object.rs:597`) takes the
"initial address not mapped" path and returns `Unable to load 512 bytes at r0x0040cb60`.

The skipped-reloc warnings the pipeline prints (`unhandled kind GotRelative (skipped)` at
`0x400785` etc.) are in **other** functions and are unrelated to this hard-fail.

## Owning stage

**S1 / code-data-partition** (the loader), `kuna-analysis` crate — the same tier as the existing
`relocobjects` (DIV-7) and `i386_pie_plt` options. `relocobjects` already binds undefined externs
as call targets; what is missing is **backing the synthetic extern area with zero-filled readable
memory** so undefined-extern *data* reads return 0 instead of aborting.

## Why this is not the assigned (ifelseflatten) feature

The assigned deliverable is a decompiler structuring Action/Rule. There is **no kuna output to
restructure** here — the pipeline dies in the loader. Shipping a `kuna_ifelseflatten*.rs` Action
would never execute on this input and would not touch the actual fault. After the loader fix lands,
whether a genuine *structural* ifelseflatten gap remains is unknown and would be a **separate**
feature. Hence: proposal for the loader fix.

## Hypothesis for the fix

Add a **zero-filled, readable** segment covering the synthetic extern area in
`elf_reloc::layout_relocatable` (so any read inside the extern region returns 0), gated by a new
loader option (env-var bridge, like `relocobjects`). Re-run `compare --entry
test_ifelseflatten_clientloop` to confirm kuna then produces output, and re-assess any remaining
structural delta. Risk is contained to the ET_REL path (linked ET_EXEC/ET_DYN images are untouched).
