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

## What is NOT laid out

`DIR`, `sigaction`, `sigset_t`, `mbstate_t`, `termios`, `sockaddr` and
`pthread_mutex_t` keep the sized opaque shell under `glibc` too: glibc publishes
no layout for `DIR` at all, and the rest are unions of reserved words no caller
reads by name.

## Two rules the tables obey

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

## Whole-corpus sweep — `opaque` vs `glibc`

Eight binaries, whole-binary `kuna decompile-all` in both arms, every hunk
classified. Raw outputs and the classifier are in `.scratch/` of the branch; the
classification is reproduced in `corpus-hunk-classification-glibc.txt` beside
this file.

| | `opaque` | `glibc` |
|---|---:|---:|
| `PTRSUB(` (the functional form the sizing rule exists to prevent) | 0 | **0** |
| `field_0x<hex>` accesses | 483 | **1** |
| `undefined8` declarations | 18 | **2** |
| declarations | 27,196 | 27,207 |
| by-value named-aggregate locals | 74 | 74 |
| piece reads (`vN._<off>_<size>_`) | 688 | **494** |

The single remaining `field_0x` is `v2._36_4_ = *(unsigned int *)&a1->field_0x24`
in findutils `find`: the four padding bytes inside `struct tm` that a by-value
copy of it moves. It is a hole in the published layout, and a hole is what it
should read as.

Hunk classes, over the 111 functions whose text changed:

| class | hunks |
|---|---:|
| a field name replaces an offset (`->field_0x8` → `->_IO_read_ptr`, `._0_8_` → `.st_dev`) | 389 + the 140 by-value forms |
| a declaration takes the field's type (`long *` → `unsigned long *`, `undefined8` → `long`) | 9 + the layout-driven stack retypes |
| a cast or a literal is respelled (`*v2 = 10` → `*v2 = '\n'`, `0xffffffffffffffff` → `-1`) | 4 |
| anything else | **0** |

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

## Type recovery — the sweep, and why the default does not move

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
