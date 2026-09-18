# `libctypes glibc` as the default — evaluation, not flipped

**Verdict: the default stays `opaque`.** #660 held `glibc` back for named
reasons, and every one of them still reproduces on `d3617d64` (main after
`structsynth`, `hideshadow`, `signedness`, `boolbyte`, the stdio stream slots,
`mulblob` and the goto-label fix). The failing criterion is the precondition of
the flip procedure itself: the reason #660 gave is not gone. Two of the
evaluation's explicit watch items fail as well — `PTRSUB(` does not stay 0,
and a by-value named local is swallowed by its neighbour.

Everything below is one release build of this branch (`make binaries` on
`d3617d64`), both arms from the same binary: `--option libctypes opaque` (the
shipped default) against `--option libctypes glibc`.

## 1. The wrong answer #660 named is still there

coreutils `copy_internal` at `-O2 -fno-inline` — the same function in `cp`,
`ginstall` and `mv` — takes three file NAMES (`src_name`, `dst_name`,
`dst_relname`, all `char const *` in the source). `glibc` types them `stat *`:

```
$ kuna decompile-all O2-noinline/coreutils/stripped/cp --addr 0x9470 --option libctypes opaque
unsigned long sub_9470(char *a0,char *a1,int a2,char *a3,unsigned int a4,long *a5,long *a6,int *a7,...)
$ kuna decompile-all O2-noinline/coreutils/stripped/cp --addr 0x9470 --option libctypes glibc
unsigned long sub_9470(stat *a0,stat *a1,int a2,stat *a3,unsigned int a4,unsigned long *a5,unsigned long *a6,int *a7,...)
...
-          if (*a0 != '/') {
+          if ((char)a0->st_dev != '/') {
```

`ginstall` `sub_a7b0` and `mv` `sub_9170` change the same way. The DWARF twin
says `bool copy_internal(char *src_name,char *dst_name,int4 dst_dirfd,char
*dst_relname,...)`; `a0` is handed to `renameat`, `sub_75b0(char *,...)` and a
quoting helper, and its only dereference is that first-character test.

**The shape, now understood.** It is not the published fields reaching the
parameter directly. `copy_internal` has a real `struct stat dst_sb` at
`-0x1f8` (the DWARF twin places it there); `opaque` leaves that slot
`char v20 [144]`, `glibc` recovers it as `stat v19` — a correct recovery on its
own. But kuna merges one register variable across three unrelated values: the
source name, `&dst_sb`, and the function's `bool` return:

```
  stat *v37;
    v37 = a0;                           /* the name */
    v10 = sub_14b40(4,v37);             /* quote(name) */
    v37 = &v19;                         /* &dst_sb */
    v6 = sub_6360(v55,v54,v37,v32);     /* follow_fstatat(dirfd,relname,&dst_sb,..) */
      v37 = (stat *)0x1;                /* the bool return */
    return (unsigned long)v37 & 0xffffffff;
```

Under `opaque` the same variable is `char *v38` because `&v20` is a `char *`.
Once `&dst_sb` is a `stat *`, the merged variable takes that type and the
`v37 = a0` copy carries it back onto the parameter. So the propagation #660
asked to be bounded is the over-merged register variable, not the table: a
flip wants that merge split (or a copy into a merged variable kept from
voting on a parameter) first.

## 2. The watch items, re-measured on this base

`decompile-all` both arms over #660's 26 binaries plus `cp`/`ginstall`/`mv`
`-O2 -fno-inline` (29 binaries), counted with the feature's own
`census.py` and `byvalue.py`:

| | `opaque` | `glibc` | |
|---|---:|---:|---|
| `PTRSUB(` functional form | 0 | **2** | watch item: must stay 0 — fails |
| `field_0x` accesses | 1,019 | 52 | the intended effect |
| piece reads `vN._off_size_` | 2,984 | 2,114 | net better |
| cross-field piece writes | 0 | **51** | not better |
| `undefined8` declarations | 68 | 34 | |
| stack declarations | 15,426 | 15,416 | the swallowed-slot class, below |
| by-value named-aggregate locals | 296 | 302 | a net count, see useradd |

Each of #660's named counterexamples reproduces with the same text:

* **`PTRSUB(` 0 → 2** — `O2/libselinux/libselinux.so.1` `sub_13530`:
  `v50 = (int *)v14;` / `v61 = (int *)v14;` become `v50 = PTRSUB(v14,0);` /
  `v61 = PTRSUB(v14,0);` (`v14` is a `FILE *`; `glibc` puts `_flags` at offset 0,
  so the PTRSUB is no longer removed and the printer falls back to the functional
  form). No other binary moves.
* **By-value `stat` lost** — `O2/shadow/useradd` `main` @ `0x6480`: `opaque`
  keeps `stat v75; // stack - 0x218`; under `glibc` the neighbouring
  `char v44 [128]; // stack - 0x268` swallows it and the call reads
  `stat("/var/log/lastlog",(stat *)&v44[0x50])`. The by-value count's net +6
  hides this loss.
* **Plain pointers swallowed** — `O2/openssh-portable/ssh-keygen` @ `0x14bc0`:
  `char *v26`, `char *v27`, `char v29 [8]` (`-0x100/-0x108/-0x110`) merge into
  `char v26 [24]` and `v27 = &v32[strspn(v32," \t")];` becomes
  `v26._8_8_ = &v30[strspn(v30," \t")];`.
* **Index-then-member mis-names** — `O2/e2fsprogs/e2fsck` @ `0x339d0`:
  `*(unsigned long *)&a0[3] = sbrk(0);` becomes `a0[3].tv_sec = sbrk(0);` (the
  source is `track->brk_start = sbrk(0)`; `a0` is a `resource_track *` whose first
  three members are timevals). `O2-noinline/diffutils/diff` @ `0x8e70`:
  `v16 = (char *)&v16[2].field_0x10;` becomes
  `v15 = (unsigned long *)&v15[2].st_nlink;` (pointer arithmetic to the next
  `file_data`, not a read of `st_nlink`).
* **Cross-field piece writes 0 → 51** — the same 51 as #660 (tar 12, find 7,
  ls O2 7, ls O0 6, stat 6, du 6, diff 6, e2fsck 1).

## 3. The typesweep

TYPESWEEP_PLACEHOLDER

## What was not run, and why

The flip procedure's gates (a) `make test`, (b) `make test-stages`,
(c) `make test-cli`, (e) the speed pairs and (g) the `modes.rs` check apply to a
flip commit; there is none, so they were not run. (f) was run as the 29-binary
census above rather than a hunk classification, because the counterexamples it
has to find were already named. #660's own speed pairs (`fmt`/`ls`
`decompile-all`, worst +0.38% on the minimum) are the current evidence that
`glibc` costs nothing measurable; speed is not what holds it back.

## What would unblock a flip

1. Stop the over-merged register variable in `copy_internal` from voting a
   `stat *` onto the name parameters (section 1). That removes the three
   worsened typesweep rows.
2. Print a matching zero-offset PTRSUB as `&p->_flags` (or undo it as `opaque`
   does) so `PTRSUB(` stays 0 on `libselinux.so.1`.
3. Keep a recovered by-value aggregate from being absorbed by a neighbouring
   slot's growth (`useradd`, `ssh-keygen`).

The index-then-member mis-names are inherent to naming a pointee from one call
site; they are the one cost a flip would have to accept knowingly.
