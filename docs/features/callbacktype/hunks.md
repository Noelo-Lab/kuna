# `callbacktype` whole-corpus hunk classification

Before/after `kuna decompile-all` over 47 binaries (22,749 functions), `--option callbacktype off`
against the default, same build (fe6672881, rebased onto f434b6a60: #722 castarith, #724 castsign, #723 globalref, #727 castindex, #728 castternary, #720 structheadless off; the runs on fcf175aac and on d10b3586c, before castindex, gave the identical table), every changed function
classified by `hunks-corpus.py`. The first 12 binaries are the implementer's set, then 9 from the
round-2 review, 12 from the round-3 review and 14 from the round-4 review, each chosen to be
disjoint from the ones before.

A hunk is IN SCOPE when it lies inside a function the option parked a declared prototype on, or
inside a function that CALLS one without changing the number of arguments any call to a parked
function passes or growing the number of lines the function prints. A caller that grows an
argument or a line is `UNEXPLAINED-caller-grew`: an argument is what a declaration longer than
the body invents (the round-2 `x6`, `x7`), and a line is what a declared return narrower than the
caller's read of it splits the register into. A caller that prints fewer lines is in scope and is
read by hand below. Any changed function that gains a `CONCAT` is `UNEXPLAINED-concat`. The two cast columns count
each function's casts in both arms with the castbench counter (`castcount.py`, the file's own
type vocabulary).

| binary | function | kind | +/- lines | casts off | casts on |
|---|---|---|---|---|---|
| O0/bzip2/stripped/bzip2 | sub_3b72 @0x3b72 | parked | 2 | 0 | 0 |
| O0/bzip2/stripped/bzip2 | sub_3bb0 @0x3bb0 | parked | 2 | 0 | 0 |
| O0/coreutils/stripped/cut | sub_3a72 @0x3a72 | parked | 9 | 4 | 9 |
| O0/coreutils/stripped/sort | sub_590b @0x590b | parked | 9 | 2 | 2 |
| O0/coreutils/stripped/sort | sub_c481 @0xc481 | parked | 6 | 0 | 7 |
| O0/dash/stripped/dash | sub_1196d @0x1196d | parked | 9 | 2 | 2 |
| O0/grep/stripped/grep | sub_161d8 @0x161d8 | parked | 4 | 2 | 6 |
| O0/kmod/stripped/kmod | sub_12bf4 @0x12bf4 | parked | 9 | 2 | 2 |
| O0/libselinux/stripped/libselinux.so.1 | sub_1ddc7 @0x1ddc7 | parked | 2 | 0 | 0 |
| O0/libselinux/stripped/libselinux.so.1 | sub_1e961 @0x1e961 | parked | 2 | 10 | 10 |
| O0/libselinux/stripped/libselinux.so.1 | sub_2399c @0x2399c | parked | 82 | 39 | 57 |
| O0/libselinux/stripped/libselinux.so.1 | sub_26db8 @0x26db8 | parked | 2 | 6 | 6 |
| O0/libselinux/stripped/libselinux.so.1 | sub_d127 @0xd127 | parked | 9 | 4 | 4 |
| O0/rsyslog/stripped/rsyslogd | sub_46af0 @0x46af0 | parked | 10 | 2 | 2 |
| O0/rsyslog/stripped/rsyslogd | sub_707a6 @0x707a6 | parked | 4 | 2 | 4 |
| O0/rsyslog/stripped/rsyslogd | sub_75298 @0x75298 | parked | 6 | 55 | 55 |
| O0/rsyslog/stripped/rsyslogd | sub_893fc @0x893fc | parked | 10 | 2 | 2 |
| O0/rsyslog/stripped/rsyslogd | sub_8942b @0x8942b | parked | 10 | 2 | 2 |
| O0/rsyslog/stripped/rsyslogd | sub_8945a @0x8945a | parked | 13 | 0 | 4 |
| O0/rsyslog/stripped/rsyslogd | sub_89493 @0x89493 | parked | 13 | 0 | 4 |
| O0/rsyslog/stripped/rsyslogd | sub_894cc @0x894cc | parked | 8 | 1 | 1 |
| O0/rsyslog/stripped/rsyslogd | sub_894f8 @0x894f8 | parked | 8 | 1 | 1 |
| O0/rsyslog/stripped/rsyslogd | sub_8acbf @0x8acbf | parked | 4 | 11 | 11 |
| O0/rsyslog/stripped/rsyslogd | sub_9ef8e @0x9ef8e | parked | 10 | 2 | 2 |
| O0/rsyslog/stripped/rsyslogd | sub_9f0f3 @0x9f0f3 | parked | 8 | 1 | 1 |
| O0/tar/stripped/tar | sub_20a2b @0x20a2b | parked | 10 | 2 | 2 |
| O0/tar/stripped/tar | sub_3550c @0x3550c | parked | 2 | 0 | 0 |
| O2-noinline/coreutils/stripped/cut | sub_38d0 @0x38d0 | parked | 11 | 0 | 4 |
| O2-noinline/coreutils/stripped/sort | sub_5930 @0x5930 | parked | 9 | 2 | 2 |
| O2-noinline/coreutils/stripped/sort | sub_a710 @0xa710 | parked | 6 | 0 | 7 |
| O2-noinline/dash/stripped/dash | sub_15fd0 @0x15fd0 | parked | 6 | 2 | 2 |
| O2-noinline/dash/stripped/dash | sub_ed90 @0xed90 | parked | 9 | 2 | 2 |
| O2-noinline/dpkg/stripped/dpkg-query | sub_7a10 @0x7a10 | parked | 2 | 5 | 5 |
| O2-noinline/dpkg/stripped/dpkg-query | sub_e2c0 @0xe2c0 | parked | 2 | 3 | 3 |
| O2-noinline/gnutls/stripped/systemkey | sub_1f290 @0x1f290 | parked | 15 | 2 | 5 |
| O2-noinline/kmod/stripped/kmod | sub_e3a0 @0xe3a0 | parked | 9 | 2 | 2 |
| O2-noinline/libselinux/stripped/libselinux.so.1 | sub_17d90 @0x17d90 | parked | 2 | 0 | 0 |
| O2-noinline/libselinux/stripped/libselinux.so.1 | sub_18d90 @0x18d90 | parked | 2 | 10 | 10 |
| O2-noinline/libselinux/stripped/libselinux.so.1 | sub_1c7e0 @0x1c7e0 | parked | 80 | 37 | 58 |
| O2-noinline/libselinux/stripped/libselinux.so.1 | sub_1e8d0 @0x1e8d0 | parked | 2 | 6 | 6 |
| O2-noinline/libselinux/stripped/libselinux.so.1 | sub_c5b0 @0xc5b0 | parked | 13 | 3 | 3 |
| O2/bzip2/stripped/bzip2 | sub_33d0 @0x33d0 | parked | 2 | 0 | 0 |
| O2/bzip2/stripped/bzip2 | sub_34c0 @0x34c0 | parked | 2 | 0 | 0 |
| O2/coreutils/stripped/sort | sub_7300 @0x7300 | parked | 9 | 2 | 2 |
| O2/coreutils/stripped/sort | sub_ba80 @0xba80 | parked | 6 | 0 | 7 |
| O2/cronie/stripped/crontab | sub_4920 @0x4920 | parked | 2 | 0 | 0 |
| O2/dash/stripped/dash | sub_15a90 @0x15a90 | parked | 10 | 4 | 4 |
| O2/dpkg/stripped/dpkg-divert | sub_7750 @0x7750 | parked | 2 | 5 | 5 |
| O2/dpkg/stripped/dpkg-divert | sub_e150 @0xe150 | parked | 2 | 3 | 3 |
| O2/dpkg/stripped/dpkg-statoverride | sub_8090 @0x8090 | parked | 2 | 3 | 3 |
| O2/e2fsprogs/stripped/e2fsck | sub_1de20 @0x1de20 | parked | 10 | 0 | 8 |
| O2/e2fsprogs/stripped/e2fsck | sub_40200 @0x40200 | parked | 10 | 2 | 2 |
| O2/e2fsprogs/stripped/e2fsck | sub_60260 @0x60260 | parked | 11 | 0 | 4 |
| O2/e2fsprogs/stripped/e2fsck | sub_66060 @0x66060 | parked | 16 | 0 | 7 |
| O2/gnutls/stripped/certtool | sub_26620 @0x26620 | parked | 15 | 2 | 5 |
| O2/grep/stripped/grep | sub_f930 @0xf930 | parked | 4 | 2 | 6 |
| O2/libselinux/stripped/libselinux.so.1 | sub_17f70 @0x17f70 | parked | 2 | 2 | 2 |
| O2/libselinux/stripped/libselinux.so.1 | sub_18f70 @0x18f70 | parked | 2 | 10 | 10 |
| O2/libselinux/stripped/libselinux.so.1 | sub_1f7c0 @0x1f7c0 | parked | 2 | 6 | 6 |
| O2/libselinux/stripped/libselinux.so.1 | sub_c6d0 @0xc6d0 | parked | 13 | 3 | 3 |
| O2/shadow/stripped/login | sub_52f0 @0x52f0 | parked | 2 | 0 | 0 |
| O2/tar/stripped/tar | sub_1e2b0 @0x1e2b0 | parked | 10 | 2 | 2 |
| O2/tar/stripped/tar | sub_2d500 @0x2d500 | parked | 2 | 0 | 0 |
| O2/tar/stripped/tar | sub_3ac00 @0x3ac00 | parked | 223 | 58 | 73 |

**64 functions change of 22,749: 64 parked callbacks and 0 callers of one. UNEXPLAINED: 0.**
The run parks 64 functions and every one of them changes. Another 25 callbacks pass every refusal
but are not parked, because the declaration is exactly the signature their body already printed
(trace token `body-agrees`, most of them `pthread_once` routines recovered `void f(void)`). Of
those 25, 23 printed the same text parked or not. The other two changed only in
rendering: `e2fsck -O2`'s `sub_2f400` reshuffled one `if`/`return` into a temporary, and
`gnutls-serv -O2`'s `terminate` handler printed one division as `(long)a0 / 10` instead of
`a0 / 10` (the same quotient, one cast more). Both now print exactly what `off` prints.

No caller changes. Before the redo was bounded, `libselinux -O2-noinline`'s `sub_1ce10` and
`selinux_restorecon_xattr` were decompiled again because they call a parked `pthread_once` routine
directly (and `sub_1ce10` the parked `pthread_create` routine), and each printed its `a1` as
`unsigned int` instead of `unsigned long`, with the same values. That `pthread_once` routine's
declaration now agrees with its body and is not parked, and neither caller's calls change: each
passes the declared number of arguments and ignores the result, so neither is decompiled again.
Redoing them was 13% of that library's run.

Nineteen binaries print no changed function: `coreutils/fmt -O2`, `ls -O2`, `timeout -O2`,
`ginstall -O2`, `ptx -O2` and `-O2-noinline`, `diffutils/diff -O2` and `sdiff -O2`, `findutils/find
-O0` and `-O2-noinline`, `gzip -O2`, `shadow/su -O0` and `-O2`, `useradd -O2` and `-O2-noinline`,
`sysvinit/init -O2` and `shutdown -O2`, `cronie/crond -O0`, and now `gnutls/gnutls-serv -O2`. `su`'s
`kill_child` handler is the `recovered-fewer-inputs` refusal (the program also calls it directly),
`diff -O2`'s `compare_names_for_qsort` is the `returns-no-value` refusal of a value that reaches its
RETURN through a join, and `ptx`'s pair is the width refusal: `compare_occurs` hands
`compare_words`' whole `rax` on through its own `ret` (`caller-reads-a-wider-return` for
`compare_words`), and the value it hands back on that path is a call's full register, which nothing
bounds (`recovered-wider-output` for `compare_occurs`). DWARF declares both `int`, so these are
misses, not wrong answers.

What moves inside a parked function: the declaration and the body rendering that follows from it.
A `void *` parameter prints its dereferences with an explicit cast where an `unsigned long *`
printed `*a0`, and a comparator that gains its `int` return renders the call's result through a
temporary. The largest hunk, `tar -O2`'s `sub_3ac00` (223 +/- lines, most of them locals
renumbered), is a comparator that went from `uint8 sub_3ac00(struct_70 *a0,struct_70 *a1)` to the
declared `int4 sub_3ac00(void *a0,void *a1)`: `return 0xffffffff` prints as `return -1`, the
synthesized record's field reads print as casts off the `void *` (the one place `callbacktype` and
`structsynth` disagree), and one `if (v) return v; return 1;` renders as the equivalent
`if (!v) v = 1; return v;`.

**Casts outside castbench.** Over the 64 changed functions the count goes from 330 to 457 (+127):
18 functions print more, 0 fewer, 46 the same. In every one of the 18 the body's own
guess for the parameter was a pointer to something (a synthesized `struct_N *`, a `char *`, an
`unsigned long *`) and the slot declares `void *`; DWARF agrees with the slot in every one
(`void *(void *)` for the thread routines, `int (const void *, const void *)` for the comparators).
The source converts that `void *` to its record type on entry (`const struct x *p = a;`), which no
stripped binary shows, so each field read prints its own cast off the `void *`, as IDA's does. The
largest are `libselinux`'s `selinux_restorecon_thread` (`char *` -> `void *`, +21 at -O2-noinline,
+18 at -O0), `tar`'s `hol_entry_qcmp` (+15), `e2fsck`'s `process_inode_cmp` (+8) and
`read_bitmaps_thread` (`struct_42 *` -> `void *`, 0 -> 7), and `sort`'s `sortlines_thread` (+7 at
each level, which is IDA's own count for that function). Nothing is weakened: the parameter moves
from an inferred pointee to the declared type, the return from a wrong `unsigned long`/`uint8` to
the declared `void *`/`int`.

Raw rows, with the per-function diff and both cast counts: `hunks.json`.
