# `charptr` — where the `char *` class gap actually is

Measured on `main` 2e28ece4a (round C), over the 444-slice decbench set
(projects coreutils grep gzip diffutils bzip2 findutils tar shadow, `--opt O0
--opt O2 --opt O2-noinline`), metric pinned to decbench 625e892 the way
`docs/decbench/typecampaign/final-c/pindb.py` pins it.

## The class

Ground-truth `char *` is the largest single class in the corpus: **14,645** of
the 65,715 scored ground-truth variables on these slices (25.9% of the whole
134,209-variable ground truth). kuna matches **4,204 = 28.7%**.

## Split by where the variable lives — this is the whole story

| GT `char *` storage | n | kuna | binja | ida | ghidra |
|---|---|---|---|---|---|
| argument | 4,186 | 2,717 (64.9%) | 2,907 (71.5%) | 1,948 (47.8%) | 1,428 (34.3%) |
| stack local | 3,316 | 1,487 (44.8%) | 2,365 (71.4%) | 1,590 (47.9%) | 1,244 (37.5%) |
| register only | 7,143 | 0 (0.0%) | 51 (0.7%) | 5 (0.1%) | 3 (0.0%) |

(The rivals' own function sets differ by a few dozen slices; their `n` is
4,064/3,311/7,094 for binja.)

Three facts follow, and they decided this feature:

1. **Half the class is unreachable for everyone.** 7,143 of the 14,645 GT
   variables are register-only locals. kuna exports no register locals at all,
   and the best rival scores 0.7% on them.
2. **kuna's argument recovery is nearly saturated.** 64.9% against binja's
   71.5% is 190 variables of headroom on the whole corpus.
3. **Nine tenths of the gap is stack locals** — 878 variables — and it is not
   mainly a typing failure.

## What kuna spells instead, for the misses that have a kuna variable

| kuna spelling | argument misses | stack misses |
|---|---|---|
| `undefined8` | 0 | 932 |
| `unsigned long` | 806 | 216 |
| `void *` | 247 | 177 |
| `long` | 180 | 134 |
| `undefined16` | 0 | 79 |
| `int8` | 28 | 17 |
| `unsigned long *` | 16 | 8 |
| `stat *` | 9 | 9 |
| everything else | 35 | 76 |
| no kuna variable matched | 148 | 181 |

**The 932 `undefined8` stack misses are not a type-inference failure.** They are
`framelayout` filler slots: the frame slot exists in the union `Funcdata`
records per restructure pass, and the value that lived there has been
copy-propagated into a register `HighVariable` that kuna does not export.
Witness, `coreutils -O0 basename::perform_basename`: ground truth `char *name`
at `DW_OP_fbreg -24` (= `rbp-40`), kuna reports `local_28  undefined8  -40`,
and kuna's own body says `char *v2; // rax`. Making `record_frame_slots` prefer
a later pass's *committed* type over the first pass's uncommitted one was tried
and changes nothing — measured, no slot moves — because by the time types are
inferred the symbol is already gone. Closing that bucket is a chapter-06
(variables/merge) item, not a chapter-05 one.

## The evidence census over the population this rule can reach

Collected with `KUNA_CHARPTR_CENSUS=1`, which prints one line per candidate the
rule considers (a function input the prototype model could place a parameter in,
or a Varnode in the stack space, of pointer width, whose current vote is not
already a pointer at something named). Deduplicated per (function, storage);
`coreutils -O0 cat`, 126 functions, 298 candidates:

| what the walk found | candidates |
|---|---|
| (e) no character evidence at all | 240 |
| refused: callee declares a scalar there | 22 |
| refused: an element step or a store wider than a byte | 18 |
| refused: integer arithmetic / shift | 4 |
| refused: compared against a non-zero literal | 2 |
| refused: a load wider than a byte | 1 |
| (c) byte-width dereference only | 8 |
| (a) a declared `char *` callee parameter (all three also had byte evidence) | 3 |
| (b) a `%s` conversion argument | 0 |
| (d) a character-array constant | 0 |

The shape holds across the corpus: **the dominant category is (e) — none of the
above.** A parameter such as `cat`'s `simple_cat(char *buf, size_t bufsize)`
only ever forwards `buf` to `safe_read` and `full_write`, whose own libc
descent is `read(int, void *, size_t)`; nothing in the program says those bytes
are characters, and binja calls it `int64_t` too.

The cases where a declared `char *` *is* reachable are largely cases the
ordinary propagation already handles, because the declaration lands directly on
the argument Varnode. What is left for this rule is the set where a hop stands
in between — an `-O0` spill, a `MULTIEQUAL`, a `p + k` — plus the `void *`
placeholders `ptrfromuse` leaves.

## What that is worth

444-slice sweep, 10,748 functions, `charptr on` against the same build with the
option off:

- perfect `type_match` 1,349 -> 1,353, aggregate 3657.76 -> 3662.69
- 4 onto perfect, 14 improved, **6 worse, 0 off perfect**
- across 2,290 functions of the 8-binary corpus diff: 14 signatures change,
  **0 arities change**, 11 functions gain or lose one local declaration

Three variants were measured and rejected.

- **The declared-callee half alone** (a call argument the callee declares
  `char *`, and nothing else): +0.14 aggregate, 1 function improved, 0 onto
  perfect, over the whole 444 slices. That is why it does not ship as its own
  strength: nearly every reachable declaration already lands on the argument
  Varnode, where ordinary propagation takes it.
- **Counting `protoorder`'s recovered callee vote** as evidence rather than only
  declared prototypes: +0.95 aggregate against +1.78 on the same 812-function
  subset. A recovered `char *` is itself a guess, and the walk would launder it
  into a commitment.
- **Without the fixed-offset guard** (any one-byte dereference counts, wherever
  it sits): +8 onto perfect and 26 improved, but 13 worse instead of 6. Four of
  the extra losses convert a *correct* `void *` into `char *` off a one-byte
  struct field — coreutils `ginstall`'s
  `announce_mkdir(char const *dir, void *options)` reads the `bool` at
  `options + 0x3c`. More metric, less truth; rejected.

The six remaining losses are read individually in `record.json`, and they are
three functions seen in more than one build.

- `od::print_long_double` (O2 and O2-noinline): a *correct* `void *block`
  parameter refined to `char *`.
- `head::elide_tail_bytes_pipe` (O2-noinline): a GT `__off_t`, an integer the
  body also uses as an address.
- `tar::check_compressed_archive` (O0, O2, O2-noinline): the byte at
  `fbreg -41` is GT `_Bool temp`; kuna reports it `undefined1`, which the metric
  credits on width alone, and typing the pointer it is read through knocks the
  byte on to `char`, which the metric then scores exactly and rejects. That is
  the same cost in the other direction: committing a type forfeits the
  width-only free pass a placeholder gets.

One further shape is visible in the corpus diff without being a metric loss:
coreutils `cp` -O0 `sub_f8ce` holds a `NAME_MAX` limit in a slot the body also
steps through by one byte, and the option turns `long v6; v6 = 0xff;` into
`char *v6; v6 = (char *)0xff;`. The emitted C stays equivalent — every widening
and comparison picks up an explicit cast — but it reads worse, and it is the
same shape as `head::elide_tail_bytes_pipe`.

## The counterexample that class did not cover (review round 2)

The paragraph above claimed the `cp` shape was value-preserving and left it at
that. It is not true of the class. diffutils `diff` at `-O0`, `sub_2180a` (an
`iconv` loop) holds `outbytesleft = 0x1000` — 4096, the size of the `char
v1[4104]` output buffer — and with the option on the assignment printed a string:

```
$ kuna decompile-all O0/diffutils/stripped/diff --addr 0x2180a
    v2 = (char *)0x1000;
    v8 = iconv(a2,&v5,&v4,&v3,&v2);
$ kuna decompile-all O0/diffutils/stripped/diff --addr 0x2180a --option charptr on
    v2 = "5";                       <-- before the fix
    v8 = iconv(a2,&v5,&v4,&v3,&v2);
```

`readelf -S` puts `.dynsym` at `0x400..0x10d8`, so `0x1000` is inside the
dynamic symbol table and the bytes there are `35 00` — the `st_name` field of a
symbol-table entry, read as the one-character string `"5"`. The same shape gave
two more sites in the `-O2` build (`v4 = "\x1e\x03";`).

**It is not charptr's vote.** The JSON `variables[]` is identical in both arms —
`v2` is `char *` with or without the option — and only the *constant's*
rendering moves. `PrintC::pushPtrCharConstant` replaces a constant with the
characters at its address, and the only thing it asks first is
`Scope::isReadOnly`; `ElfFormat::section_bits` painted `.dynsym` read-only
because it is allocated and not writable. charptr commits the *neighbouring*
slot (`stack@-4192`, `ev=byte`, the correct `iconv` output buffer) and that is
enough to reorder the fold so the constant picks up the `char *` its destination
already had.

The fix is therefore outside this rule, and ships in the same PR as the strict
fix it is: the dynamic loader's tables no longer carry the read-only bit. Two
*default-output* defects on main fall out with it — `bash`'s
`rl_do_lowercase_version` returned `"_ungets"` for `0x1869f` and coreutils `ls`
compared a pointer against `"loc"` for `0x12c7`, both `.dynstr` offsets. See
`record.json` → `wrong_output_found`, the fixture
`loadertablestring_x86_64`, and the probe
`tests/cli/loader-table-bytes-print-as-a-string.json`.

## Layout precision: what a `char *` costs when the object was an aggregate

The item spec asks for this number directly — *a `char *` that was a struct
pointer is a loss* — so it is measured rather than argued. Eight whole binaries,
both arms of the same build, `decompile-all` over every function
(`e2fsprogs e2fsck -O0`, `cronie crond -O0`, `coreutils expr -O0`, `coreutils
ls/mv/wc -O2`, `shadow passwd -O2`, `libedit.so.0.0.70 -O2`, about 2,300
functions), keyed on each variable's storage comment so renumbering cannot fake
a move:

| move | count | what it is |
|---|---|---|
| placeholder → `char *` | 9 | the gain: `void *`, `undefined *`, `uint1 *` |
| scalar → `char *` | 2 | the gain: an integer that held a string |
| **typed pointer → `char *`** | **3** | the loss, all one function |
| `char *` → something else | 8 | the signedness knock-on, four functions |
| `struct_N *` declarations | **1,286 → 1,286** | no synthesized struct pointer ever moves |

The three typed-pointer losses are all `e2fsck::ext2fs_bitcount` (`unsigned
int *a0` and its two stack copies become `char *`), and they happen for the
reason the `unsigned char *` note already gives: the refine guard reads the type
*in flight*, so a pointee that would only have arrived by later propagation is
not there to protect. Upstream declares that function `int
ext2fs_bitcount(const void *addr, int nbytes)`, so in this instance `char *` is
not worse than what it replaced — but the mechanism is the one that would be.

Layout precision proper — a wide field read through a pointer that is now a
character array — moves on exactly one function of the eight binaries. Lines of
the shape `*(T *)&p[k]` with `T` wider than a byte go 1,202 → 1,204, both in
`e2fsck::sub_678f8`, where `v11 = ext2fs_group_desc(...)` (an `int8` with the
option off) commits and the group descriptor's 2-byte field prints as
`*(unsigned short *)&v11[0x1e]`; the same statement's zero store splits into
`v11[0x1e] = '\0'; v11[0x1f] = '\0';`, which is value-preserving and reads
worse. Nothing else in the eight binaries loses a field boundary.

cronie `crond -O0 sub_6715` — the `strcmp(base + 0x13, ".cron.hostname")` on a
`struct dirent *` that the offset guard was added for — is byte-identical in
both arms.

## The offset rule, completed (follow-up to #695)

The offset guard above read a fixed displacement only when it arrived as a
literal `INT_ADD` or a `PTRSUB`. The same displacement has a third spelling: once
an earlier fold has typed the base `void *`, `RulePtrArith` rewrites `d + 0x13`
as `PTRADD(d, 0x13, 1)`, and the `PTRADD` arm counted any one-byte element step
as a character step and forwarded the value as if it were still at the base. A
`struct dirent *` whose `d_name` is walked to its NUL therefore committed to
`char *` on the second fold after declining on the first (census: `ev=none`,
then `ev=byte` three times). dpkg shows it twice: `dpkg-trigger` -O2 `sub_58d0`
and `dpkg-statoverride` -O0 `sub_9aaf` both print `(char *a0)` with the option
on before this change and `(void *a0)` after.

The `PTRADD` arm now reads its index: a constant non-zero index sets the offset
flag, index zero is identity, and a one-byte step taken from an offset value
forwards without counting. Only an index the program computes, from the base,
is character evidence (`kuna_charptr::ptradd_step`, unit-tested).

That removes nearly all of what the option was measured to gain, because the
gain was the same shape. Two correct idioms start at a fixed offset from the
base and cannot be told from a field by anything the walk sees:

* skipping a string's first character before the first read (`p + 1`):
  coreutils `realpath` `path_prefix`, `env` `scan_varname`, `printf`
  `print_esc`, three of the four functions that had moved onto a perfect score;
* writing a number backwards from the end of a caller's buffer, gnulib
  `umaxtostr` (`p = buf + 20; *p = 0; *--p = ...`): tar `sysinttostr`, the
  fourth, and the `umaxtostr` family in tar, ls and sort.

| 444-slice sweep, option on vs off | before (a constant `PTRADD` counted) | after |
|---|---|---|
| perfect `type_match` | 1,349 -> 1,353 | 1,349 -> 1,349 |
| aggregate | +4.93 | -0.57 |
| onto / off perfect | 4 / 0 | 0 / 0 |
| improved / worse | 14 / 6 | 2 / 4 |

The four worse rows are the ones already read above (`head`
`elide_tail_bytes_pipe`, `tar` `check_compressed_archive` in three builds); the
two improved are `tac` `output` and `tar` `read_header`. The base aggregate,
3659.62, is the same as before, so the `.interp`/note loader change beside this
one moves nothing the metric scores.

Corpus check over 16 whole binaries: the default arm is byte-identical to main
on all of them. With the option on, 147 lines in 8 binaries change, and every
hunk is the rule withdrawing a commitment so that the function prints what the
default arm prints: a parameter back from `char *` to `void *`, `&a0[k]` back to
`(char *)((long)a0 + k)`, the `umaxtostr` cursor back in its own `char *v2`, two
tar stores back from `'-'`/`' '` to `0x2d`/`0x20`. No arity change, no new
literal, nothing reordered. What the option still does on those binaries is
mostly signedness (`unsigned char *`/`uint1 *` to `char *`, and a few knock-ons
the other way) plus one store-only `bool *` (`tar` `mode_from_header`'s
`hbits`) that becomes `char *`. `default-on-evaluation.md` records what this
means for the default.
