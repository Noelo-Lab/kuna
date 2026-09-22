# calleevote — analysis

## 1. The measured symptom

Round E's O2 census (`docs/decbench/typecampaign/final-e/o2census/`) ranked the
O2 blockers. The largest O2-noinline bucket is a program struct pointer that kuna
declares `void *` in a function that is otherwise right: 143 functions at
O2-noinline and 20 at O2 where fixing that one variable makes the function
perfect under decbench#93 crediting, 66 of them gnulib `Hash_table` getters.

```
$ kuna decompile-all O2-noinline/coreutils/stripped/chcon --addr 0x68f0
unsigned long sub_68f0(void *a0)
{
  return *(unsigned long *)((long)a0 + 0x10);
}
```

DWARF: `size_t hash_get_n_buckets (const Hash_table *table)`.

The census named two causes. Both were measured before anything was built
(`census-bucket.py`, `census-callers.py`, `census-votes.py` in this folder).

## 2. What each candidate fix reaches

**(a) The reverse vote.** For the 166 solo functions of the bucket (O2 +
O2-noinline, round-E rows): 150 have callers, 11 have none, 5 have their address
taken. What the callers pass in that argument, read off their own C:

| callers pass | functions |
|---|---:|
| no argument at all (the caller forwards the register untouched) | 44 |
| `void *` (the caller's own parameter) | 31 |
| mixed / disagreeing | 34 |
| `unsigned long *`, `int *`, `long`, locals of other types | ~30 |
| a `struct_N *` | 4 |

So a caller-to-callee vote reaches at most ~8 of the 166: the callers of a
`Hash_table` getter hold `void *` too. It is still worth building, for a
different population. Over every wrong argument at O2/O2-noinline, a parameter
the callee typed only `void *` or an integer, all of whose callers pass one
committed pointer, is 131 wrong-to-right under the pinned metric (mostly
`char *`: `gzip_error (char const *m)`-shaped wrappers whose callers pass
literals), 41 more under #93 crediting, and 18 where the callers' type is wrong.

**(b) A lone field.** `structsynth` declines a pointer read at one offset: it
wants two distinct offsets including 0 (`kuna_structsynth.rs` `accepts_record`).
Relaxing that alone did nothing, and the reason is a second blocker the census
did not name. `ActionStructSynth` fires only once `ActionInferTypes` reports a
pass that changed nothing (`kuna_infertypes_settled`). A short function's main
loop ends on the pass that first typed it (nothing after it changes an op, so no
second pass runs) and the synthesizer is never offered the function at all.
With both the relaxation and one forced settle pass, over the 296 O2/O2-noinline
slices:

| arm | pinned perfect | #93-credited perfect | worse |
|---|---:|---:|---:|
| main | 455 | 557 | - |
| settle pass only | 455 | 557 | 0 |
| settle pass + lone field (any width) | 455 | 704 | 24 |
| settle pass + lone field (width >= 4) | 455 | 694 | 22 |

All 22-24 worse rows are one shape: a `qsort` comparator or hash-table callback
(`proctab_hasher`, `wd_comparator`, `name_compare`, ...) whose contract declares
`void const *` and which reads one field of what it is handed exactly as a
getter does. What separates them is not in the function: their address is taken.

**(c) Both, gated on the complete caller set.** A whole-binary run already has
the call graph `protoorder` orders by. A function whose callers are all known
direct calls (its address taken nowhere: not in code, not as a pointer-width
word of any section the image loads, not by a Mach-O chained fixup, a dynamic
relocation, an exported symbol or the entry point; and only in an x86-64 image;
sections 5 and 6 have what the first two versions of this check missed) is
"closed". The reverse vote needs that anyway, and the lone-field rule is
admitted only there. That removes every callback false positive:

| arm (296 O2 slices) | pinned perfect | credited perfect | improved / worse |
|---|---:|---:|---:|
| main | 455 | 557 | - |
| `types` (reverse vote) | 475 | 590 | 145 / 2 |
| `fields` (vote + closed lone field) | 475 | 716 | 145 / 2 pinned, 589 / 2 credited |

Of the census bucket's 166 solo functions, 97 now take a struct pointer and all
97 are perfect under #93 crediting. The rest are callbacks, byte fields and
parameters that are only forwarded.

## 3. Things measured and not shipped

* **Refusing a value the callee hands to a declared `void *` parameter.** It
  would keep gnulib `xmemdup (void const *p, size_t s)` right (3 functions), but
  a wrapper around `memcpy`/`fwrite` is exactly what `dired_outbuf` and
  `samedir_template` are, with `char *` parameters: on the 444 slices it lost 8
  perfect functions and 28 improvements to gain those 3.
* **A byte or halfword lone field.** +10 credited perfect, 2 more worse rows
  (bzip2 `copyFileName` reads a `Char *` at a fixed index); a byte at a fixed
  offset is as often a character of a buffer as a field.
* **A settle pass for every structsynth candidate.** Measured alone it moves no
  scored variable and only changes C text, so it is not part of this option.

## 4. A pre-existing instability the redo exposed

Decompiling a function twice in one session can give a different prototype the
second time. `bash` -O2 `sub_7ea70` gains an unused `unsigned int a6` on its
second decompile with every option at its default and `calleevote` off:

```
load file bash / read symbols / load addr 0x7ea70 / decompile / print C / decompile / print C
int8 sub_7ea70(unsigned long a0,int8 a1,unsigned long a2,char *a3,uint4 a4,uint4 a5)
int8 sub_7ea70(unsigned long a0,int8 a1,unsigned long a2,char *a3,uint4 a4,uint4 a5,unsigned int a6)
```

Turning off each of 24 call/arity options one at a time does not remove it
(`Funcdata::clear` still stubs `localmap->clearUnlocked`). It is not this
option's to fix. The redo guards against it instead: a redo whose parameter or
variable count differs from the first decompile's keeps the first body and drops
that function's records, so nothing is decided from a discarded body
(`same_arity`, `Ledger::forget`). Without the guard two bash functions gained a
parameter; with it none does, on any of the 12 swept binaries or the 444 slices.

## 5. What the first closed-set check missed

The first version read only raw words of non-executable sections plus dynamic
relocations. Three shapes of stored callback escaped it, and in each a
callback that is also called directly was voted on:

| shape | witness | before the fix | after |
|---|---|---|---|
| a relocatable object: the table word is 0 in the file and an `R_X86_64_64` fills it | `calleevote_ops_x86_64.o` `get_c` (also `cx3.o`, `cx5.o`) | `get_c(struct_0 *a0)`, `fwd(char *a0)` | `get_c(void *a0)`, `fwd(unsigned long a0)`, as `off` |
| an ARM Thumb address stored as `addr\|1` | crazyflie O2-noinline `cf2.elf` `sub_801d6a4`, `sub_801d6a8`, `sub_801d6b0`, `sub_801d658` | `struct_105 *a0` etc. | `void *a0`, as `off` |
| a const ops table placed in `.text` | the same four: their table sits at `0x8047604` in `.text` | | |

The check then scanned every allocated section, code included, on the pointer
grid, read each word a second time with bit 0 cleared on ARM and MIPS (section
6 replaces that: no ARM or MIPS function is closed at all), added
Mach-O chained-fixup rebase targets, and declared no function closed in a
relocatable object (whose data references the reference walk does not classify
at all) or an image whose sections hold none of its entries. A word that only
happens to equal an entry leaves that function open, the safe direction. On the
444 slices this moved nothing: every x86-64 PIE table was already a data word.

Two readability shapes the review found are fixed as well:

* **A `char *` over a wide constant store.** cf2 `sub_801ab08` printed
  `*(unsigned int *)((int)a0 + 0x34) = 0xffffffff;` as four `a0[0x34] = '\xff';`
  stores and `sub_8016760` lost its `unsigned short *`. The vote is now refused
  when the callee stores a constant wider than a byte through the value
  (`kuna_protoorder::input_refuses`); both functions print as `off` again, and
  over 21 whole binaries no new run of byte stores appears.
* **An array walked from a constant index.** dash O2-noinline `sub_7460`
  (`char **argv`) read `argv[1]` and then stepped `argv + 2` through a loop; the
  lone-field rule made that a record `{char field_0x0[8]; char *field_0x8;}`
  stepped at the record's size. A base from which an address at a constant
  offset reaches a phi is no longer a lone field, and `sub_7460` now takes the
  `char **` its callers pass. A broader guard (any use of such an address but a
  load or store, and any second access the prune dropped) was measured first
  and cost 21 functions whose record decbench credits (`file_has_acl_cache`,
  `getfilecon_cache`, `xheader_store`, ...: they pass `&f->member` to a call),
  so only the loop is refused.

What stays, and is documented as the option's effect: an array read at one
constant index and nothing else (`return a[2];`) reads exactly like a getter
and becomes a one-field record; where the callers' record is smaller than what
the function reaches through a pointer derived from the value, the access
prints as an index past the record (kmod `0x10ae0`
`*(unsigned long *)v1[1].field_0x0 = a1;`, dpkg `0x13570`), which the review
checked equivalent in 7 functions over 12 binaries; and a caller local whose `void *` came only
from the callee's old `void *` through `protoorder` keeps no pointer type when
the callee's new record does not fit what the caller does with it, so it is
declared as an integer and may merge with another local (dash `sub_eb30`).

## 6. Addresses built from two instructions, and code at address 0

The second review found two more ways a callback reached the closed set.

* **An address taken in two instructions.** The cross-reference walk reads one
  instruction at a time. AArch64 takes a function's address as `adrp`+`add`,
  MIPS (non-PIC) as `lui`+`addiu`, ARM as `movw`+`movt`, PowerPC and RISC-V
  as two halves, i386 PIC as a base register plus an offset; neither half is a
  reference to the function, so a callback also called directly counted as
  closed. AArch64 `sg.c`: `run(getc16, buf)` plus two direct calls printed
  `unsigned long sub_401018(struct_0 *a0)`; MIPS `calleevote_callback_mipsel`
  printed `unsigned int get_c(struct_0 *a0)` and gave `run` a `char *`
  parameter from its one caller. Joining the halves would mean either a
  per-architecture pair matcher in the walk or reading the constants out of
  every decompiled body, which is complete only when every function in the
  image decompiles and every switch table is read. The option is measured and
  scored on x86-64 only, so it now closes functions only in an x86-64 image,
  where every address code takes is one instruction's operand (a RIP-relative
  `lea`, an immediate). Every other architecture prints exactly as `off`.
* **A table in code loaded at address 0.** The stored-word scan skipped every
  section at address 0, meaning to skip the sections the image does not load;
  firmware loads `.text` at 0, and a const ops table in it was never read
  (`sub_20(struct_0 *a0)`, `sub_30(struct_1 *a0)`). Sections are now chosen by
  `SHF_ALLOC` on ELF, whatever their address. Exported symbols (`exports()`,
  which also covers PE and Mach-O export tables) are open as well.

Witnesses: `calleevote_callback_mipsel` and `calleevote_zero_x86_64` (fixtures,
`tests/cli` probes, `calleevote_stored_tests`). The reviewer's AArch64, MIPS and
zero-address builds (`sg`, `sf`, `se`, `si`) now print exactly as with the
option off. What is still not seen, and is documented: an address kept as an
offset from another one (a 32-bit offset table, a relative C++ vtable, a
self-relative pointer).

### The redo's cost, and a vote the rounds dropped

The review measured kmod O2-noinline at +11.3% (min of 9; +7.4% median),
outside the four budget binaries. kmod's first round decompiled 73 of its 541
functions again, and 42 of those printed exactly as before. Most were a getter
whose one-field record `protoorder` had handed to its callers, which then
passed the same record back: a statement that repeats the callee's own type.
Such a first statement is no longer made (round 1: 73 -> 49 redos).

Reading the rounds for that turned up a real loss. Each round decided against
the parameters the LAST decompile recovered, so a parameter that had taken a
vote was no longer a candidate, fell out of the next statement (which
replaces the old one), and lost its type in the next redo; a third round
sometimes stated it again and sometimes had no round left. tar O2-noinline
`0x35910` needed all three rounds; coreutils `ginstall` `need_copy` (O0) and
`copy` (O2-noinline) ended with `unsigned long` where every caller passes a
`char *`. Every round now decides against the parameters of the function's
first decompile. Over the 21 binaries the default output is byte-identical to
the build before either change; on the 444 slices the two `ginstall`
functions improve (334 up / 3 down, was 332 / 3).

What the second decompile still costs (interleaved min of 15, same build, the
option off vs default, on main with #712): fmt, ls, sort and bash -O2 +0.42,
+0.93, +0.88 and +0.63%; kmod -O2-noinline +11.3% (55 of its 541 functions
decompiled again) and cmp -O0 +7.9% (9 of 214, most of it one 217-line
function whose `char *` votes its own uses refuse), at load 2-3. On the base
before #712 the same build measured kmod +9.8% and cmp +7.7-7.9%. Whether a vote will be
refused is known only by running the pipeline with it, since the refusals read
the function's own uses in the redo, so no cheaper filter was found.

