# passthrough — every gained parameter and return, against DWARF

The question round F left open: the option adds parameters and return values the
`type_match` metric cannot check. This is that check, over **two corpora**, with
every function that gains something checked against its unstripped twin's DWARF
prototype — not a sample.

* **corpus A — 444 slices, 8 projects** (coreutils, grep, gzip, diffutils,
  bzip2, findutils, tar, shadow), O0/O2/O2-noinline: the decbench measurement
  corpus this campaign is scored on.
* **corpus B — 130 slices, 17 projects DISJOINT from A** (openssh-portable,
  e2fsprogs, dpkg, kmod, dash, cronie, sysvinit, base-passwd, libacl, libbsd,
  libselinux, zlib, libexpat, libedit, rsyslog, iproute2, gnutls), O2 and
  O2-noinline. Corpus A is eight GNU projects built by one toolchain from one
  source idiom; B exists because a rule measured only on A is measured on that
  idiom, and the first version of this rule was.

Method: `kuna decompile-all <stripped> --json` twice per slice (`--option
passthrough off` and the shipped default), the recovered argument list read from
`variables[]` `kind == "arg"`, joined by `low_pc` to the twin's
`DW_TAG_subprogram` (following `DW_AT_abstract_origin` for gcc's concrete
out-of-line instances — without that hop 433 unnamed instances read as `void`
and inflate the contradicted count tenfold). Drivers: `.scratch/armsweep.py`,
`.scratch/dwarfjoin.py`, `.scratch/dwarfproto.py` in this lane's worktree; the
verdicts below are the ones they print.

A gained parameter is **confirmed** when the twin's prototype has a parameter at
that index, **contradicted** when the twin's prototype is shorter and not
variadic, and **no twin** when the twin has no `DW_TAG_subprogram` at that
address at all.

## What corpus B changed about the rule

Measured on corpus B, the first version of this rule contradicted DWARF on **159
of 1,691 checkable parameters (9.4%)** — against 4 of 2,787 (0.14%) on corpus A.
Every one of the 159 was openssh, and every one was one shape:

```text
unsigned long sub_42ff0(uint8 a0,uint8 a1,unsigned long a2) {     // xcalloc
  if ((!a1) || (!a0))
    sub_3aa40("xmalloc.c","xcalloc",0x34,0,1,0,"xcalloc: zero size",a2);
```

`xcalloc(size_t, size_t)` is recovered with a third parameter **in both arms**,
because the call to the variadic `sshfatal` is set up with `push %rdx; ...; push
%rax; xor %eax,%eax`, and that `push %rdx` is gcc's stack-alignment filler —
`rdx` holds nothing at all. `passthrough` then handed that parameter to every
caller that forwards the register (`tohex`, `pwcopy`, `init_hostkeys`,
`umac_new`, …). gnulib `open_safer(char const *, int, ...)` is the same shape
with the variadic declared: it reads `rdx` only to pass it to `open`'s `mode`
slot, so `savewd_save` — corpus A's 4 contradicted rows — gained a third
parameter too.

Three refusals were added, and one detection bug fixed:

1. **vararg tail** (`p4_calls/kuna_varargtail.rs`). A callee parameter whose
   value, through value-preserving operations, only ever reaches an argument slot
   of a variadic call — one the callee set up as variadic, or a declared `...`
   prototype past its named parameters — is recorded with the statement and never
   claimed. The ABI lets a caller leave that register unset, so the callee's read
   of it says nothing about what its callers put there.
2. **no hole** (`kuna_passthrough::no_hole_before`). Parameters are positional: a
   claim at the third stated register gives the function three parameters. Every
   earlier stated register must be one this function could be carrying — claimed
   here too, or one its own entry walk does not prove it WRITES before reading.
   The `protoorder` fixture's `overrec` sets `rsi` to 16 and forwards `rdx`; it
   rendered `void overrec(unsigned long *a0,unsigned long a1,long a2)` with `a1`
   in no statement, against the call site `overrec(v1)`.
3. **the variadic-call detection that never fired.** `set_up_as_variadic` counted
   the writing instruction's own reads, and `xor %eax,%eax` — the idiom gcc
   actually emits for the vector count — reads the register it zeroes, once as
   its operand and again in `ZF = (EAX == 0)`. Over ssh-keygen it recognized 0
   calls. A read at the same instruction address as the write is now that
   instruction's own.

A fourth, on the return side: `computes_everywhere` answered `true` when its node
budget ran out, and `true` there STATES a callee's return to its callers on a
body the walk never finished reading. It answers `false`.

## Parameters, after the tightening

### corpus A — 444 slices, 8 projects

| | rows | confirmed | contradicted | no twin |
|---|---:|---:|---:|---:|
| **all** | **2,851** | **2,728 (95.7%)** | **0** | **123 (4.3%)** |
| O0 | 0 | 0 | 0 | 0 |
| O2 | 1,093 | 1,086 | 0 | 7 |
| O2-noinline | 1,758 | 1,642 | 0 | 116 |
| coreutils | 2,063 | 1,975 | 0 | 88 |
| shadow | 372 | 358 | 0 | 14 |
| diffutils | 133 | 128 | 0 | 5 |
| tar | 132 | 128 | 0 | 4 |
| findutils | 96 | 88 | 0 | 8 |
| grep | 50 | 46 | 0 | 4 |
| gzip | 5 | 5 | 0 | 0 |

### corpus B — 130 slices, 17 disjoint projects

| | rows | confirmed | contradicted | no twin |
|---|---:|---:|---:|---:|
| **all** | **1,495** | **1,379 (92.2%)** | **0** | **116 (7.8%)** |
| O2 | 626 | 596 | 0 | 30 |
| O2-noinline | 869 | 783 | 0 | 86 |
| openssh-portable | 557 | 557 | 0 | 0 |
| e2fsprogs | 333 | 266 | 0 | 67 |
| iproute2 | 209 | 201 | 0 | 8 |
| zlib | 126 | 101 | 0 | 25 |
| dpkg | 80 | 80 | 0 | 0 |
| rsyslog | 60 | 56 | 0 | 4 |
| gnutls | 45 | 45 | 0 | 0 |
| libselinux | 37 | 36 | 0 | 1 |
| kmod | 16 | 8 | 0 | 8 |
| libedit | 14 | 14 | 0 | 0 |
| dash | 11 | 11 | 0 | 0 |
| libbsd | 6 | 3 | 0 | 3 |
| libacl | 1 | 1 | 0 | 0 |

**4,346 gained parameters over 574 slices — 4,107 DWARF-confirmed, 0
contradicted, 239 with no twin, and 0 lost**
(`lost = 0` in both joins; `lostargs.py` reports `call_lost_args=0
param_lost=0`). 2,225 functions gain a parameter. Nothing moves at O0, where the
register is already named by an op.

The tightening costs 55 confirmed parameters on corpus A (2,783 → 2,728) and 153
on corpus B (1,532 → 1,379) — 5% of the gain, to remove every contradicted row.

Of the 4,107 confirmed parameters, 3,782 also agree with DWARF on the type's
*class*; the 325 that do not are almost all a real pointer parameter kuna types
`long`/`unsigned long` (`chroot` `0x6d10` takes DWARF `char *`, kuna says
`long`). That is the `ptr_char` gap the campaign already tracks, not a
fabrication: the parameter exists, and with the option off it was missing
entirely.

### The 239 rows with no DWARF twin

Single-statement forwarding thunks (`unsigned long sub_b3a0(unsigned long a0,
unsigned char a1) { return sub_b360(a0,a1); }`), every one outside every
`DW_TAG_subprogram` range in the twin — the toolchain emitted them with no debug
entry, so DWARF can neither confirm nor contradict them. Their gained list is
exactly the callee's own recovered list, which is the rule's whole statement.
They concentrate in coreutils/O2-noinline (88), e2fsprogs (67) and zlib (25):
the selinux, `*_safer` and zlib-compat wrapper families.

## Return values — the arm that is a judgement

The first pass of this sweep measured **212 contradicted returns of 4,267** on
corpus A — one shape, replicated: gnulib `version_etc_ar` is DWARF `void`, and
kuna recovers its callee `version_etc_arn` (also `void`) as returning `long`,
because that function's fallthrough path ends in `__fprintf_chk` and keeps the
call's `RAX` clobber. `recovered_output` now refuses to state a return the callee
did not compute: every live RETURN must hand back a value produced in **every
byte and on every path** (`kuna_returnuncomputed::every_return_computes`).

| | rows | confirmed | contradicted | no twin |
|---|---:|---:|---:|---:|
| **corpus A** | **4,087** | **4,012 (98.2%)** | **6 (0.15%)** | **69** |
| **corpus B** | **1,659** | **1,446 (87.2%)** | **151 (9.5% of checkable)** | **62** |
| B: openssh-portable | 675 | 552 | 123 | 0 |
| B: e2fsprogs | 233 | 177 | 24 | 32 |
| B: kmod | 15 | 9 | 2 | 4 |
| B: iproute2 | 267 | 262 | 2 | 3 |
| B: every other project | 469 | 446 | 0 | 23 |

**This rate is not a bug the rule can fix, and it is the option's known cost.**
Every one of the 151 is the same shape, and the shape is undecidable from the
machine code:

```text
-void ext2fs_fast_mark_block_bitmap(void)  { ext2fs_mark_generic_bitmap(); }
+unsigned long ext2fs_fast_mark_block_bitmap(struct_31 *a0,uint4 a1)
+{ return ext2fs_mark_generic_bitmap(a0,a1); }        // DWARF: void
```

A source-`void` wrapper that tail-calls a value-returning function compiles to
exactly the same `jmp` as a wrapper that returns what it calls, and `rax` holds
the callee's result at the RETURN either way. gzip's `char *gzip_base_name(char
const *)` — the witness this option was written for — is that second shape, and
nothing in either body distinguishes them. The claim is that a tail-call wrapper
hands back what it calls: right for 5,458 of 5,615 checkable returns across both
corpora (97.2%), wrong for 157, and concentrated wherever a project writes many
one-line `void` wrappers around value-returning primitives (e2fsprogs's bitmap
accessors, openssh's `blf_key`/`ssh_packet_close` families). `--option
passthrough off` is the way out for a reader who would rather have the `void`.

## Verdict

The **parameter** arm is what the flip rests on, and after the tightening it is
0 contradicted of 4,107 checkable over 574 slices in 25 projects, with nothing
ever lost.
The **return** arm is 97.2% confirmed and its 2.8% is a single undecidable shape,
named here and in the option's `use_when` rather than measured away.
