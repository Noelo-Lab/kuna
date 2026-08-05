# fdeinterior - an FDE interior is not a function start

kuna promotes C++ exception landing pads, aggressive-gap-walk starts and stray
prologue-pattern hits to **top-level functions**, because nothing in the commit
path can tell that they sit in the middle of a function that already exists. The
`.eh_frame` FDE table is exactly the missing information, and using it removes
1,030 spurious entries across 72 x86-64 C binaries and 150 of 599 on one
`g++ -O0` C++ binary while losing **zero** real functions.

---

## 1. The measured defect

A purpose-built `g++ -O0 -g` probe: `probe_try_catch` spans `[0x2acf, 0x2ba7)` -
one function, one `.cfi_startproc`/`.cfi_endproc` pair, one FDE. kuna's function
list contains three extra entries inside it.

```
$ kuna functions probe_O0 --json
  0x2acf probe_try_catch
  0x2b0b sub_2b0b        <- .gcc_except_table landing pad (catch dispatch)
  0x2b1a sub_2b1a        <- second byte of the `je 2b6b` at 0x2b19
  0x2b8a sub_2b8a        <- .gcc_except_table landing pad (cleanup)
```

They survive stripping (`probe_O0.stripped` has the same three), so a benchmark
that scores per function sees them.

Decompiled, `sub_2b0b` is what a mid-frame entry always is - the parent's frame
pointer is live on entry, so every local is a dereference of an undefined value:

```c
unsigned int sub_2b0b(unsigned long a0,unsigned long a1,int8 a2)
{
  int8 v3; // rbp                      <- never assigned
  ...
    *(unsigned int *)(v3 + -0x20) = 0xffffffff;
    printf("caught: %s\n",(*v1)(v2));
```

The rivals:

| decompiler | verdict on `0x2b0b` |
|---|---|
| **IDA Pro 9.2** | `get_func(0x2b0b).start_ea == 0x2acf` - part of the parent, taken from the FDE |
| **Ghidra 12.1** | `getFunctionContaining(0x102b0b)` returns none (its `GccExceptionAnalyzer` errored on this PIE - a Ghidra bug, not a model) |
| **kuna (before)** | three separate functions |

IDA's model is the target.

## 2. Root cause - three producers, one missing fact

`kuna functions` / `decompile-all` resolve `--mode auto`, and `auto` picks
**`aggressive`** for anything under 500 KiB (`p0_knowledge/modes.rs`), which turns
on `eh_frame_full`, `aif`, `funcstart_patterns`, `listing` and more. Instrumenting
the commit boundary (`engine.rs (commit_pending_analysis)`) attributes the three
entries exactly:

```
DBGENTRY  pass=eh_frame_full on=true addr=0x2b0b
DBGENTRY  pass=eh_frame_full on=true addr=0x2b8a
DBGENTRY2 pass=aif           on=true addr=0x2b1a
```

* `eh_frame_full` is *designed* to do this: "emit each exception-handler landing
  pad ... as a discovered function entry ... a landing pad sits mid-function".
* `aif` (the ported `ArmAggressiveInstructionFinderAnalyzer`) starts a candidate at
  the first byte of a region the recursive-descent walk never reached. The
  exception region `[0x2b0b, 0x2ba1)` is reachable only through the unwinder, so
  the walk misses part of it and `aif` starts at a byte that is not even an
  instruction boundary.
* the prologue oracles (`entry_disc` oracle 5, `funcstart_patterns`) match an
  aligned `push rbp; mov rbp,rsp` wherever it occurs, including inside a body.

Flipping `--option eh_frame_full off` alone does not fix the list (that was
checked first) - it removes two of the three and leaves `aif`'s.

The common cause is one missing fact. A kuna `FunctionSymbol` is an **entry
address with no extent**: `Database::add_function` takes an address, and
`find_function` answers "is a function registered AT this address", never "is this
address INSIDE one". Every oracle's dedup is therefore an equality test, and a
candidate 12 bytes into a body passes it.

## 3. The fix - `.eh_frame` FDE bodies

Each FDE records `pcBegin` and `pcRange` for exactly one function; the compiler
emits one per `.cfi_startproc`/`.cfi_endproc` pair. That is the extent the symbol
table never carried, available on any ELF built by gcc/clang whether or not it is
stripped. So:

> a discovered entry that falls **strictly inside** an FDE body is not a function.

`kuna_fdeinterior.rs` reuses `scan_eh_frame_starts`'s record walk and additionally
decodes `pcRange` - the field immediately after `pcBegin`, in the CIE's `R`
encoding but as a *length*, so it is decoded with the encoding's format bits only
and never its pc-relative application. The commit filters the **fully merged**
entry set (after the deferred Listing consumers, so `aif` is covered) against those
bodies.

### 3.1 The PLT, and what makes a range eligible

Not every FDE describes one function. The first prototype deleted every import:

```
probe_O0.stripped  fdes=123  entries=189  interior=61
    0x2280 inside 0x2270..0x24a0     <- .plt.sec, one FDE for the whole section
    0x2290 inside 0x2270..0x24a0
    ...
```

The linker emits a **single FDE covering the whole PLT**, and every stub inside it
is a real function. A range is therefore used only when it is a single-function
frame:

1. no already-named function start (`.symtab`/`.dynsym` FUNC + resolved PLT stubs,
   via the existing `existing_function_addrs`) lies strictly inside it,
2. no other FDE's `pcBegin` lies strictly inside it,
3. it does not overlap `.plt` / `.plt.sec` / `.plt.got` / `.iplt` / `.MIPS.stubs`.

Guard 1 alone rejects the PLT on every binary measured (its stubs carry
relocation-derived names even when stripped); 2 and 3 are the cheap
belt-and-suspenders for an image whose stubs are anonymous. An overlapping FDE set
(never observed) drops the whole fact stream rather than guessing.

An entry **at** an FDE `pcBegin` is never suppressed, so oracle 3's own product -
the FDE-start function list, the most valuable oracle on a stripped C/C++ binary -
is untouched by construction.

## 4. Breadth - 72 x86-64 C binaries, 0 real functions lost

`kuna functions <stripped>` with the option off vs on, over the decbench O0 and O2
trees (coreutils, bash, gzip, tar, grep, diffutils, openssh, e2fsprogs, gnutls,
dpkg, zlib, bzip2, findutils, kmod). Every dropped entry is classified against the
matching **unstripped** binary's `FUNC` symbols (address *and* size, so "inside a
real body" is checkable).

| | |
|---|---|
| binaries | 72 |
| entries, option off | 36,568 |
| entries, option on | 35,538 |
| dropped | 1,030 |
| **added** | **0** |
| **dropped that are a real function's entry** | **0** |
| **dropped that carry a name (PLT stub, symbol)** | **0** |
| dropped that fall strictly inside a real function body | 1,030 |
| ...of which are not even an instruction boundary | 52 |

Every removed entry is provably interior to a real function. Nothing is added, so
this can only ever shrink the list.

The C++ population, where the landing pads live (`fdeinterior` off -> on):

| binary | off | on | dropped | landing pads | other interior | real lost |
|---|---|---|---|---|---|---|
| `probe_O0.stripped` (g++ -O0) | 189 | 162 | 27 | 14 | 13 | **0** |
| `probe_O2.stripped` | 71 | 64 | 7 | 4 | 3 | **0** |
| `big_O0.stripped` (g++ -O0, STL + exceptions) | 599 | 449 | 150 | 98 | 52 | **0** |
| `big_O2.stripped` (`--mode aggressive`) | 161 | 86 | 75 | 74 | 1 | **0** |

On `big_O0` **25% of the reported function list was junk**.

Other populations:

| target | off | on | note |
|---|---|---|---|
| static glibc `st_O2.stripped`, `--mode aggressive` | 1,358 | 1,087 | 271 dropped, 0 real, **0 named** - the static `.plt`/`.iplt` is not touched |
| `mydoom.exe` (PE) | 280 | 280 | ELF-only => structurally inert |
| `dexter.dll` (PE) | 270 | 270 | inert |

### 4.1 The emitted C

`kuna decompile-all` before/after, comparing every surviving function's body:

| binary | off | on | removed | added | **bodies changed** | **names changed** |
|---|---|---|---|---|---|---|
| `big_O0.stripped` (C++) | 599 | 449 | 150 | 0 | **0** | **0** |
| `probe_O0.stripped` (C++) | 189 | 162 | 27 | 0 | **0** | **0** |
| `probe_O0` (C++, unstripped) | 190 | 163 | 27 | 0 | **0** | **0** |
| `gzip` (C, O0) | 263 | 248 | 15 | 0 | **0** | **0** |
| `b2sum` (C, O0) | 228 | 216 | 12 | 0 | **0** | **0** |

The change is a pure removal: of the functions that survive, not one changes its text or
its name by a byte. Both arms are captured on this branch, so #262's C++ callee-name
rendering (DIV-59) is identical in both and cancels out of the diff.

## 5. #259 (`tailcallentry`) non-regression - measured, not argued

`tailcallentry` (#259) recovers +561 ARM entries; `ptrentry` (#255) and
`cortexmvectors` add more. This option removes entries, so the composition was
measured on the same corpus and the same surface `docs/features/tailcallentry/`
used - `kuna functions --mode aggressive`, **with `--option tailcallentry on`**,
`fdeinterior` off vs on, over every ARM image in the three optimisation trees:

```
ARM TOTAL: images=98  failed=0  entries_off=57578  entries_on=57578
           images_with_any_change=0
```

**Byte-identical entry sets on all 98 images.** The mechanism is structural rather
than lucky: bare-metal ARM unwinds through `.ARM.exidx`, not `.eh_frame`. A scan of
the corpus finds `.eh_frame` FDEs in **3 of 98** images (riot-os `hello-world.elf`,
2 FDEs each), and those two FDEs contain no discovered entry. The same argument
covers the in-flight ARM entry-granularity work in draft PR #239: it operates on a
population this option cannot see.

## 6. Why default-ON

* The entries removed are, by measurement, never functions - 0 of 1,030 on the C
  corpus, 0 of 259 on the C++ probes, 0 of 271 on static glibc.
* The entries *kept* include every named function and every FDE start; the filter
  has no path to remove either.
* The datatest corpus is structurally untouched: the XML `<binaryimage>` bootstrap
  never runs the analyzer tier (`run_default_analyses` is bound to the real-object
  path), so `make test` is 675/675 and `make test-stages` 394/394 with no re-pin
  and no per-test opt-out.
* Leaving it off would leave the default surface - `auto` -> `aggressive` under
  500 KiB - producing garbage entries on every C++ binary kuna is pointed at.

`--option fdeinterior off` restores the previous discovery set exactly (asserted
end-to-end in `verify_fdeinterior.rs`).

## 7. Rejected alternatives

* **Just turn `eh_frame_full` off by default.** It is already off by default; the
  `aggressive` preset turns it on, and it is not the only producer - `aif`'s
  mid-instruction start survives that change.
* **Suppress only landing pads.** Narrower, and it leaves `aif`'s and the prologue
  patterns' interior hits. The FDE is the general fact and costs the same scan.
* **Give `FunctionSymbol` a real extent.** The correct long-term fix, and far out
  of scope: extents would have to come from the decompiled flow of every function,
  which is a whole-program dependency at load time. `.eh_frame` is extent evidence
  already in the file.
* **Suppress interiors of *every* FDE.** Deletes the whole PLT (section 3.1).

## 8. Provenance, and the two rebases

Every number in this document was re-measured on this branch **rebased onto `890096b6`
(#264 `cppproto`, DIV-60)**, itself on `1cff8ecb` (#262 `cppcallnames`, DIV-59). Neither
upstream change is inert with respect to this one, so both were re-measured rather than
carried forward:

* #262 changed C++ callee-name rendering everywhere and fixed a discovered entry installing
  a duplicate generic `sub_<addr>` that shadowed a real symbol.
* #264 ingests C++ prototypes from DWARF through `DW_AT_specification`, **recovering
  out-of-line member and namespace definitions that were previously dropped entirely**. On
  an unstripped C++ binary far more addresses now carry a real named function symbol, which
  is precisely the input to this option's "range holds no other named function start"
  condition.

The headline invariant holds against the new symbol set. On `big_O0` **447 of the 450
surviving entries now carry a real name** (161 of 163 on `probe_O0`), and the filter still
drops 150 with **0 real functions lost and 0 named functions lost**:

| unstripped C++ | off | on | dropped | real lost | named lost | named entries after |
|---|---|---|---|---|---|---|
| `big_O0` | 600 | 450 | 150 | **0** | **0** | 447 / 450 |
| `big_O2` | 87 | 87 | 0 | **0** | **0** | — |
| `probe_O0` | 190 | 163 | 27 | **0** | **0** | 161 / 163 |
| `probe_O2` | 72 | 65 | 7 | **0** | **0** | — |

The stripped counterparts are byte-identical to the pre-#264 measurement (449 and 162
entries), verified rather than assumed — a stripped image carries no DWARF for `cppproto`
to read.

Both arms of every off/on comparison are the same build, so both upstream changes cancel out
of the diff; the `surviving_names_changed` / `NAMED_LOST` counters added for this check are
**0 everywhere**. #264 also altered 9 plain-C functions in coreutils `fmt` via its
`MAX_TYPE_DEPTH` change, so `fmt` was decompiled whole in both arms specifically to keep the
two effects unconflated: `fmt`-O0 198 -> 191 entries with **0 surviving bodies changed**,
`fmt`-O2 151 -> 151 with none.

Catalog counters were re-derived from a fresh capture of the post-rebase build, not by
adding one to the previous value: **91 settables, 20 core / 41 transform / 30 analysis, 90
catalog rows**. (#264 and this PR made an identical off-by-one edit to the same three
assertions, so the merge resolution would have kept the wrong number had it been trusted.)

## 9. One disclosed interaction

`eh_frame_full`'s product is now constrained by this gate: with both on, a landing
pad inside a single-function FDE is no longer committed. That is the intended
composition (the pad IS the thing being rejected), and the existing
`verify_eh_frame_full` end-to-end test now flips `fdeinterior off` in its
option-on arm so it keeps testing what the LSDA decode *produces*.
