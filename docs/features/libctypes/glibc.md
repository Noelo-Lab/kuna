# `libctypes glibc` — the published field layouts

`opaque`, the shipped default, gives each named libc aggregate its real width
and no members. That is enough to keep the pointer-arithmetic seam honest, and
it says nothing about what is at an offset: the refill body of an inlined `getc`
reads `f->field_0x8`, and the value it loads is `unsigned char *` only because
the cast that used to be there said so.

`glibc` installs the real x86-64 members of the nine aggregates whose layout the
platform publishes in an installed header and an application is meant to read.

```
$ kuna decompile-all O2/coreutils/stripped/fmt --addr 0x2f30 --option libctypes opaque
void sub_2f30(FILE *a0)
{
  ...
  v6 = *(unsigned char **)&a0->field_0x8;
  if (*(unsigned char **)&a0->field_0x10 <= v6)
    v4 = __uflow(a0);
  else {
    *(unsigned char **)&a0->field_0x8 = &v6[1];

$ kuna decompile-all O2/coreutils/stripped/fmt --addr 0x2f30 --option libctypes glibc
void sub_2f30(FILE *a0)
{
  ...
  v6 = (unsigned char *)a0->_IO_read_ptr;
  if (a0->_IO_read_end <= v6)
    v4 = __uflow(a0);
  else {
    a0->_IO_read_ptr = (char *)&v6[1];
```

The second half of the value is that a loaded field carries the FIELD's type,
not the cast's. `stat::st_size` is a signed `long`, so a wrapper that returns
`-1` on failure stops printing its sentinel as an unsigned bit pattern
(`decompiler/crates/kuna-analysis/tests/fixtures/libctypes_glibc_x86_64`, the
stage fixture, console spelling):

```
- unsigned long size_of(char *a0,stat *a1)        + int8 size_of(char *a0,stat *a1)
-   if (!stat(a0,a1))                             +   if (!stat(a0,a1))
-     return *(unsigned long *)&a1->field_0x30;   +     return a1->st_size;
-   return 0xffffffffffffffff;                    +   return -1;
```

## Provenance — every offset was measured, not remembered

`.scratch`-built program, compiled and run on the machine this was written on
against the installed headers. It is reproduced here in full because the tables
in `kuna_libctypes/glibc.rs` are a transcription of its output and nothing else.

```c
#define _GNU_SOURCE
#include <stdio.h>
#include <stddef.h>
#include <sys/stat.h>
#include <sys/time.h>
#include <time.h>
#include <pwd.h>
#include <grp.h>
#include <getopt.h>
#include <dirent.h>
#include <gnu/libc-version.h>

#define F(S,M) printf("%-10s %-22s off=0x%-4zx size=%zu\n", #S, #M, offsetof(S,M), sizeof(((S*)0)->M))
#define SZ(S)  printf("== %-12s sizeof=%zu align=%zu\n", #S, sizeof(S), _Alignof(S))
/* ... one F() per member of FILE, struct stat, struct timespec, struct timeval,
       struct tm, struct passwd, struct group, struct option, struct dirent ... */
```

Output (`glibc 2.35`, x86-64):

```
== FILE         sizeof=216 align=8
FILE       _flags                 off=0x0    size=4
FILE       _IO_read_ptr           off=0x8    size=8
FILE       _IO_read_end           off=0x10   size=8
FILE       _IO_read_base          off=0x18   size=8
FILE       _IO_write_base         off=0x20   size=8
FILE       _IO_write_ptr          off=0x28   size=8
FILE       _IO_write_end          off=0x30   size=8
FILE       _IO_buf_base           off=0x38   size=8
FILE       _IO_buf_end            off=0x40   size=8
FILE       _IO_save_base          off=0x48   size=8
FILE       _IO_backup_base        off=0x50   size=8
FILE       _IO_save_end           off=0x58   size=8
FILE       _markers               off=0x60   size=8
FILE       _chain                 off=0x68   size=8
FILE       _fileno                off=0x70   size=4
FILE       _flags2                off=0x74   size=4
FILE       _old_offset            off=0x78   size=8
FILE       _cur_column            off=0x80   size=2
FILE       _vtable_offset         off=0x82   size=1
FILE       _shortbuf              off=0x83   size=1
FILE       _lock                  off=0x88   size=8
FILE       _offset                off=0x90   size=8
FILE       _codecvt               off=0x98   size=8
FILE       _wide_data             off=0xa0   size=8
FILE       _freeres_list          off=0xa8   size=8
FILE       _freeres_buf           off=0xb0   size=8
FILE       __pad5                 off=0xb8   size=8
FILE       _mode                  off=0xc0   size=4
FILE       _unused2               off=0xc4   size=20
== struct stat  sizeof=144 align=8
struct stat st_dev  off=0x0 size=8      struct stat st_blksize off=0x38 size=8
struct stat st_ino  off=0x8 size=8      struct stat st_blocks  off=0x40 size=8
struct stat st_nlink off=0x10 size=8    struct stat st_atim    off=0x48 size=16
struct stat st_mode off=0x18 size=4     struct stat st_mtim    off=0x58 size=16
struct stat st_uid  off=0x1c size=4     struct stat st_ctim    off=0x68 size=16
struct stat st_gid  off=0x20 size=4     (__pad0 @ 0x24, __glibc_reserved[3] @ 0x78)
struct stat st_rdev off=0x28 size=8
struct stat st_size off=0x30 size=8
== struct timespec sizeof=16  tv_sec @0x0  tv_nsec @0x8
== struct timeval  sizeof=16  tv_sec @0x0  tv_usec @0x8
== struct tm       sizeof=56  tm_sec@0x0 tm_min@0x4 tm_hour@0x8 tm_mday@0xc tm_mon@0x10
                              tm_year@0x14 tm_wday@0x18 tm_yday@0x1c tm_isdst@0x20
                              tm_gmtoff@0x28 tm_zone@0x30
== struct passwd   sizeof=48  pw_name@0x0 pw_passwd@0x8 pw_uid@0x10 pw_gid@0x14
                              pw_gecos@0x18 pw_dir@0x20 pw_shell@0x28
== struct group    sizeof=32  gr_name@0x0 gr_passwd@0x8 gr_gid@0x10 gr_mem@0x18
== struct option   sizeof=32  name@0x0 has_arg@0x8 flag@0x10 val@0x18
== struct dirent   sizeof=280 d_ino@0x0 d_off@0x8 d_reclen@0x10 d_type@0x12 d_name@0x13[256]
```

The four rows this output shows that the tables do NOT carry — `stat::__pad0`,
`stat::__glibc_reserved`, `FILE::__pad5`, `FILE::_unused2` — are measured here on
purpose: the width they occupy has to be accounted for, and the reason they are
left out is a judgement, not an omission. See the first of the three rules below.

## What is NOT laid out

`DIR`, `sigaction`, `sigset_t`, `mbstate_t`, `termios`, `sockaddr` and
`pthread_mutex_t` keep the sized opaque shell under `glibc` too: glibc publishes
no layout for `DIR` at all, and the rest are unions of reserved words no caller
reads by name.

## Three rules the tables obey

**No row glibc reserves for itself.** `stat::__pad0`, `stat::__glibc_reserved`,
`_IO_FILE::__pad5` and `_IO_FILE::_unused2` are measured like every other row and
then deliberately left out, so those offsets stay holes and print in the neutral
offset form. The reason is in the sweep below: over sixteen binaries the reserved
rows fired **28** times, and not one of those 28 was worth having. Nineteen were
whole-struct copies spilling padding, where `v12.__glibc_reserved[0] =
v13.__glibc_reserved[0]` says exactly as much as `v12._120_8_ = v13._120_8_`;
eight were lies (below); one was an address computation the name made harder to
read, `(long)v32.__glibc_reserved + v10 * 4 + -0x78` where the hole gives
`(long)&v32.st_dev + v10 * 4`. A member no program may read has no truthful use
that carries information, so naming it can only cost.

**No self-reference.** `_IO_FILE::_chain` and `_IO_FILE::_freeres_list` are
`struct _IO_FILE *` in the header and `void *` here. Completing a struct re-keys
it into a new `Rc` (`TypeFactory::set_fields_struct`), so a `FILE *` built
against the shell WHILE the shell is being filled would be stranded on the
incomplete one and the program would carry two stream types. Nothing reads
`_chain`; the cost is a name, the alternative is a split type.

**At most one level of by-value nesting.** `stat` holds three `timespec`s by
value and `timespec` holds no aggregate, so the recursive mint terminates by
construction rather than by a depth counter, and `dependent_order`'s
definition-before-use walk stays finite. Both properties are unit-tested
(`glibc_layouts_nest_at_most_one_level`).

## The target gate

A field name is a claim about what is at an offset, so the layouts are installed
only where the claim is checkable: an **ELF** for **x86-64** whose `.dynstr`
names glibc (the `libc.so.6` soname, or a `GLIBC_2.x` symbol version —
`.gnu.version_r`'s version names live in `.dynstr`). musl, another libc, another
architecture and a statically linked image all fall back to the `opaque` shells.
For a static image that costs nothing: these tables are matched against IMPORTED
names only, so a static image has no named aggregate to lay out in the first
place.

Where the image's own debug info already describes the aggregate, the adoption
rule is unchanged — a held definition of the declared width is taken whole and no
published layout is installed over it. That is what `tests/stages/kuna-libctypes.xml`
passes 1-3 pin, and why passes 4/5 use a fixture built without `-g`.

A name the operator declares by hand (`--define-function 0x…=fopen`) arrives long
after load, with the image out of reach, so it cannot re-run that gate and must
not guess: minting a glibc layout on a musl or ARM
image would be the false claim the gate exists to prevent. `live_layout` needs
two facts instead, and both have to hold. The option value has to be `glibc` —
that much is process-wide and still readable, so a run that asked for `opaque`
gets `opaque`. And the program has to carry an aggregate already held under one
of the nine names with the member the table puts at offset 0, which is what
proves the gate passed on *this* image. Either half missing gives the opaque
shell. The option half is the one a musl image's own DWARF `stat` — complete, and
named exactly the same way, because the kernel ABI is shared — cannot get past.

That last case is the one the earlier code got wrong: `LibcTypesPass::run` seeds
from IMPORTED names only, so an image that imports `stat` but not `fopen` reaches
the declared path with no `FILE` held, and a hard-coded `Layout::Opaque` would
have minted a fieldless 216-byte shell beside the program's field-filled `stat`.

## Whole-corpus sweep — `opaque` vs `glibc`

Sixteen binaries, whole-binary `kuna decompile-all` in both arms, every hunk
classified. Eight were picked here; the other eight (`e2fsprogs/e2fsck`,
`coreutils/du`, `base-passwd/update-passwd`, `shadow/useradd`,
`coreutils/sort`, `dpkg/dpkg-statoverride`, `cronie/crond`, `diffutils/diff`)
were picked by an adversarial review, and they are where the two counterexamples
below come from. Raw outputs and the classifiers are in `.scratch/review2/` of
the branch; the classification is reproduced in
`corpus-hunk-classification-glibc.txt` beside this file.

| | `opaque` | `glibc` |
|---|---:|---:|
| `PTRSUB(` (the functional form the sizing rule exists to prevent) | 0 | **0** |
| `field_0x<hex>` accesses | 651 | **9** |
| `undefined8` declarations | 46 | **16** |
| declarations | 37,737 | 37,731 |
| by-value named-aggregate locals | 132 | 134 |
| piece reads (`vN._<off>_<size>_`) | 1,580 | **1,273** |
| named reserved members (`__pad0`, `__glibc_reserved`, …) | 0 | **0** |

179 of 8,290 functions change text. Nine `field_0x` accesses survive: one is the
four padding bytes inside a copied `struct tm` in findutils `find`, three are
address forms in `e2fsck`, and five are the diffutils reads at `field_0x80` and
`field_0x88` that the reserved-row rule exists to keep neutral.

### The array-index hazard, and where it goes wrong

A sized pointee makes `p[k]` step by the aggregate's width, so an index respells
an offset. There are two shapes of that and they do not behave alike.

**Member, then index** — `p->d_name[k]`. 31 sites over the sixteen binaries,
every one of them `dirent::d_name`, and every one an improvement: `d_name` is a
`char[256]`, so findutils' dotfile test goes from three offset fields to the
array it is.

```
- if ((vN->field_0x13 != '.') || ((vN->field_0x14 && (vN->field_0x14 != '.' || vN->field_0x15))))
+ if ((vN->d_name[0] != '.') || ((vN->d_name[1] && (vN->d_name[1] != '.' || vN->d_name[2]))))
-   vN = strlen(&vN->field_0x13);
+   vN = strlen(vN->d_name);
```

**Index, then member** — `p[k].member` with `k` not 0. This is the hazard the
refuters raised, and an earlier version of this file claimed every occurrence of
it was an improvement. **That was wrong**, and the binary that shows it is the
one the item spec named. It fires 10 times; **4 are true and 6 are confident
mis-names.**

e2fsprogs `e2fsck`, `init_resource_track` @ `0x339d0` — nine of the ten sites.
`gettimeofday(a0, NULL)` is the only thing in the stripped binary that says what
`a0` is, so `a0` becomes `timeval *`; it is really a
`struct resource_track *`, whose first three members happen to be `timeval`s.

```
$ kuna decompile-all O2/e2fsprogs/stripped/e2fsck --addr 0x339d0 --option libctypes opaque
  *(unsigned long *)&a0[3] = sbrk(0);
  *(unsigned long *)&a0[3].field_0x8 = 0;
  *(unsigned long *)&a0[4] = 0;
  *(unsigned long *)&a0[1] = v3;              /* and a0[1].field_0x8, a0[2], a0[2].field_0x8 */

$ kuna decompile-all O2/e2fsprogs/stripped/e2fsck --addr 0x339d0 --option libctypes glibc
  a0[3].tv_sec = sbrk(0);
  a0[3].tv_usec = 0;
  a0[4].tv_sec = 0;
  a0[1].tv_sec = v3;                          /* and a0[1].tv_usec, a0[2].tv_sec, a0[2].tv_usec */
```

The unstripped twin (`O2/e2fsprogs/compiled/e2fsck`, `iscan.c:61`) says what
those are:

```c
struct resource_track {
    struct timeval time_start, user_start, system_start;   /* 0x00, 0x10, 0x20 */
    void *brk_start;                                       /* 0x30 */
    unsigned long long bytes_read, bytes_written;          /* 0x38, 0x40 */
};
    track->brk_start = sbrk(0);
    track->bytes_read = 0;
    track->bytes_written = 0;
```

So `a0[1]` and `a0[2]` really are `timeval`s and their four `tv_sec`/`tv_usec`
names are right; `a0[3].tv_sec`, `a0[3].tv_usec` and `a0[4].tv_sec` are
`brk_start`, `bytes_read` and `bytes_written` and their names are wrong. Five
sites, where `opaque` printed a neutral offset.

diffutils `diff` at `-O2 -fno-inline`, `0x8e70` — the tenth. A `stat *` walking
`struct file_data { int desc; char const *name; struct stat stat; … }`
advances by `&v15[2].st_nlink`, which is pointer arithmetic to the next
`file_data`, not a read of `st_nlink`. `opaque` printed
`&v16[2].field_0x10`.

That same function is why the reserved rows are gone. `v15` points at the `stat`
member, so the enclosing struct's own `desc` and `name` sit at `-1` index plus
0x80 and 0x88 — inside `__glibc_reserved`. With those rows carried it read

```
    if ((int)v15[-1].__glibc_reserved[1] != -1)      /* really cmp.file[N].desc */
      v9 = (char *)v15[-1].__glibc_reserved[2];      /* really cmp.file[N].name */
```

and it now reads `*(int *)&v15[-1].field_0x80` / `*(char **)&v15[-1].field_0x88`,
which is what `opaque` says too. Eight sites recovered.

The price is the other nineteen, and they are worth naming because a review
counted four of them as mis-names too. `dpkg-statoverride`'s `v12.__pad0` /
`v12.__glibc_reserved[k]` lines are **not** mis-names: `v12` really is a
`struct stat`, filled by the `fstat(v2,&v12)` three lines above, and the source
assigns the whole struct to a global, so the compiler copies its padding as well.
Those four names were right. The other fifteen are the same shape in coreutils
`stat`, findutils `find` and `tar`. All nineteen go back to the offset form, and
that is the trade — a copy of padding is the one place the name tells a reader
nothing:

```
  dat_100c0 = v12.st_gid;
- dat_100c4 = v12.__pad0;          + dat_100c4 = v12._36_4_;
  dat_100c8 = v12.st_rdev;
```

**The hazard is inherent, not a bug in a table.** A pointee named from one call
site is a claim about the whole object, and when the object is bigger than the
claim, every offset past the aggregate is mis-attributed. The reserved rows made
it worse and are gone; the rest of it (`tv_sec` for `brk_start`) is the cost of
`glibc` over `opaque` and is one of the reasons `opaque` stays the default.

### The rest of the hunks

Field names that actually land, across the sixteen binaries: `tm_*`, `d_name`,
`d_type`, `pw_*`, `gr_*`, `d_ino`, plus the `_IO_*` and `st_*` families.
`option`'s members (`name`, `has_arg`, `flag`, `val`) appear nowhere: a
`getopt_long` table is a static initializer these programs never read back
through the pointer.

| class | hunks |
|---|---:|
| a field name replaces an offset (`->field_0x8` → `->_IO_read_ptr`, `._0_8_` → `.st_dev`) | 613 |
| a cast or a literal is respelled (`*v2 = 10` → `*v2 = '\n'`, `0xffffffffffffffff` → `-1`) | 20 |
| a declaration takes the field's type (`long *` → `unsigned long *`, `undefined8` → `long`) | 12 |
| the classifier's `OTHER` | 239 |

The 239 are what the classifier's declaration regex does not match: declaration-list
churn — a stack slot regrouped, a local retyped, a `// rax` comment gained or lost.
They were reviewed by grouping into 137 distinct normalized shapes; every one is
declaration churn except the three printer respellings below. The index shapes
(41 sites) cut across these classes and are counted above.

### A slot the compiler reuses is named wrong in one half

e2fsck's `ext2fs_add_journal_inode3` @ `0x61c00` holds both a `struct stat` (for
its `fstat`) and a `struct ext2_inode`, and the compiler gives them one stack
slot. kuna types that slot `stat` in **both** arms — the declaration is already
wrong under `opaque` — but `opaque` reads the inode half as neutral offsets and
`glibc` names them.

```
- if (v18._28_4_)            + if (v18.st_uid)                /* i_gid */
- v18._26_2_ = 1;            + v18.st_mode._2_2_ = 1;         /* i_links_count */
- v18._0_2_ = 0x8180;        + v18.st_dev._0_2_ = 0x8180;     /* i_mode */
- v18._32_4_ |= 0x80000;     + v18.st_gid |= 0x80000;         /* i_flags */
```

It is the same failure as the index hazard with a different cause: a claim about
an object, made from one call site, applied past what the claim covers.

### Three printer respellings

One shape in the residue looked like a defect and is not. In coreutils `stat`,
`*(unsigned int *)&v10[3] & 0xf000` becomes `v10[3] & 0xf000` once the pointee
goes from `long` to `unsigned long`. That is a pre-existing property of the
printer, not of this change, and it reproduces on `main` with no libctypes
involved at all:

```c
int f_u(unsigned long *p){ return (*(unsigned int *)&p[3] & 0xf000) == 0x4000; }
int f_s(long *p)         { return (*(unsigned int *)&p[3] & 0xf000) == 0x4000; }
```

```
int4 f_u(uint8 *p) { return (int4)((p[3] & 0xf000) == 0x4000); }
int4 f_s(int8 *p)  { return (int4)((*(uint4 *)&p[3] & 0xf000) == 0x4000); }
```

`*(char *)&vN[4]` → `(char)vN[4]` is the same class. The third is a loss: in
that same e2fsck function a 4-byte-element copy loop walks `&v18.st_rdev`, which
is now 8 bytes wide, so the increment stops being an element step.

```
- v19 = (unsigned int *)&v18.field_0x28;  v2 = &v19[1];                  *v20 = *v19;
+ v19 = &v18.st_rdev;                     v2 = (uint8 *)((int8)v19 + 4);  *v20 = (int4)*v19;
```

### By-value locals: a count, not an invariant

`docs/features/libctypes/byvalue.py` counts by-value named-aggregate stack
locals: **132 → 134** over the sixteen binaries. Read that as a count. It is a
NET, and it hides motion in both directions on the exact `variables[]` surface
decbench scores.

shadow `useradd`, `main` @ `0x6480` is the counterexample. `opaque` recovers a
144-byte `stat` stack symbol at `-0x218`; `glibc` does not — a neighbouring
`char[16]` at `-0x268` grows to `char[128]` and swallows it.

```
$ kuna decompile-all O2/shadow/stripped/useradd --addr 0x6480 --option libctypes opaque
  char v45 [16];   // stack - 0x268
  long v65;        // stack - 0x258    (and v66..v68)
  stat v75;        // stack - 0x218

$ kuna decompile-all O2/shadow/stripped/useradd --addr 0x6480 --option libctypes glibc
  char v44 [128];  // stack - 0x268
    v44._16_8_ = v27 / 0x15180;
    stat("/var/log/lastlog",(stat *)&v44[0x50]);
```

Both arms export 43 `variables[]` rows; the `stat` row is one of them in `opaque`
and none of them in `glibc` (`long` 7 → 4, `undefined8` 10 → 15). Two other
`useradd` functions (`0xfde0`, `0x16520`) gain a by-value `stat` that `opaque`
did not have, which is where the net `+2` comes from.

**The metric cannot see it.** `shadow::O2::useradd` `main` scores
`0.08064516129032258` with `{tp: 5, fp: 4, fn: 53, decomp_vars: 43}` in BOTH
arms, so the row is not in `moved.csv` and a flat sweep is not evidence that
nothing regressed. It is evidence that nothing regressed *that this metric
scores*.

## Type recovery — the sweep, and why the default does not move

This sweep was re-run on the build with the reserved rows dropped and the
live-layout probe in. Every figure is identical — perfect 959 → 959, aggregate
3037.05 → 3037.60, the same 9 improved / 3 worse and the same 12 moved rows.

`scripts.decbench.typesweep`, decbench main's `TypeMatchMetric`,
`DECBENCH_NO_CACHE=1`, `KUNA_BIN` pinned to this branch's release build. Base arm
is the shipped default (`opaque`), test arm `--option libctypes glibc`.
8 projects × `O0` / `O2` / `O2 -fno-inline` = **444 slices, 10,748 scored
functions**.

| | base (`opaque`) | test (`glibc`) |
|---|---:|---:|
| `type_match` PERFECT | 959 | 959 (+0) |
| aggregate `type_match` | 3037.05 | 3037.60 (+0.55) |
| moved ONTO perfect | | 0 |
| moved OFF perfect | | 0 |
| improved (not perfect) | | 9 |
| worsened (not perfect) | | 3 |

Controls: the base arm agrees with the tree's published `type_match` on 9,505 of
10,748 (88.44% — the tree was scored before this campaign's merges); 10,590
functions hand the metric byte-identical `variables[]` in both arms and **0** of
them score differently.

**The metric is flat, and that is the expected shape.** decbench scores
`variables[]` — arguments and stack symbols. A field NAME changes neither. What
it changes is the body, and the type of a value loaded out of a field, and the
register locals that carry those are never exported. `+0.55` over 10,748
functions is noise.

### All twelve moved rows

| direction | slice | function | base → test |
|---|---|---|---|
| better | coreutils::O0::chroot | main | .731 → .769 |
| better | coreutils::O0::od | skip | .500 → .625 |
| better | coreutils::O2::od | skip | .000 → .125 |
| better | coreutils::O2-noinline::od | skip | .000 → .125 |
| better | coreutils::O2::cp | copy_internal | .090 → .130 |
| better | coreutils::O2::ginstall | copy_internal | .090 → .130 |
| better | coreutils::O2::mv | copy_internal | .090 → .130 |
| better | coreutils::O2::ginstall | install_file_in_file | .160 → .200 |
| better | gzip::O0::gzip | streamsavedir | .571 → .643 |
| **worse** | coreutils::O2-noinline::cp | copy_internal | .180 → .148 |
| **worse** | coreutils::O2-noinline::ginstall | copy_internal | .180 → .148 |
| **worse** | coreutils::O2-noinline::mv | copy_internal | .180 → .148 |

### The three worse rows are one function

`cp`, `ginstall` and `mv` are the same source. In `copy_internal` at
`-O2 -fno-inline`, three parameters the ground truth spells `char *` come out
`stat *`:

```
- unsigned long sub_9470(char *a0,char *a1,int a2,char *a3,...)
+ unsigned long sub_9470(stat *a0,stat *a1,int a2,stat *a3,...)

-   if (*a0 != '/') {
+   if ((char)a0->st_dev != '/') {
```

A pointer to a struct outranks a pointer to char in the seed fold
(`SUB_PTR_STRUCT` 4 < `SUB_PTR` 6) in **both** arms; what the published fields
change is how far that `stat *` vote travels before the fold sees it. The
fieldless shell stopped it short of these parameters and the completed one does
not.

**So the default stays `opaque`.** The metric gives no reason to move it, and
this is a small wrong answer the fieldless shell did not produce — which is
exactly the judgement call the option model exists for. A later flip wants that
propagation reach bounded, or this shape understood, and a sweep that is positive
rather than flat.

It also wants the two things this sweep cannot see. The six mis-named index sites
and the `useradd` by-value loss above are both invisible to `type_match` — the
first because the metric does not read the body, the second because it scores the
same `tp`/`fp`/`fn` either way. A flat sweep is evidence that nothing regressed
*that this metric scores*, and a default flip would be a decision about the C
text as much as about the number.

## What it does to a `decompile-project` export

`opaque`'s shells are deliberately INCOMPLETE, so the exported `.h` declares
`typedef struct FILE FILE; /* opaque */` and the exported `.c` then reads fields
out of a type its own header says has none. `glibc` closes that: the header emits
the real bodies, dependency-ordered, and they parse.

```
$ kuna decompile-project <fixture> -o D --option libctypes glibc
$ sed -n '41,55p' D/libctypes_glibc_x86_64.h
typedef struct FILE FILE;
typedef struct timespec timespec;
typedef struct stat stat;

struct FILE {
    int _flags;
    undefined1 _pad4[4];
    char *_IO_read_ptr;
    char *_IO_read_end;
    ...
struct stat {
    ...
    timespec st_atim;
```

`cc -std=c99 -fsyntax-only` over the generated header: **0 errors in all three
arms**. Over the generated `.c` (which has never compiled, in any arm) on the
same fixture: `off` 17 errors, `opaque` 21, `glibc` **18** — the three that
`opaque` adds are the incomplete-type reads, and filling the layout in takes them
back out.

## Speed

Interleaved `opaque`/`glibc` pairs, minimum and median of 15 each, alternating
which arm runs first. The box was shared with another corpus sweep throughout,
which is why the minimum is the number to read.

| case | min `opaque` | min `glibc` | min Δ | median Δ |
|---|---:|---:|---:|---:|
| `fmt decompile-all` (152 fn) | 4070.4 ms | 4086.0 ms | +0.38% | +5.02% |
| `ls decompile-all` (404 fn) | 13498.2 ms | 13417.1 ms | −0.60% | −0.37% |
| `fmt functions` (load only) | 145.8 ms | 145.8 ms | −0.04% | −5.15% |

Flat. The work added is nine struct completions at load, once per program.
These numbers were taken before the reserved rows were dropped; that change
removes four field mints per program and can only subtract work, so they stand
as an upper bound. An independent min-of-N on a different box run reproduced
the same shape: `fmt` +1.17% when the measurement is load-dominated, `du`
whole-binary −1.36% at n=11.
