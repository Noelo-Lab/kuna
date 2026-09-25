# `callbacktype` whole-corpus hunk classification

Before/after `kuna decompile-all` over 59 binaries (35,291 functions), `--option callbacktype off`
against the default, same build (bfa1d32e0, rebased onto 850e8c692: #722 castarith, #724 castsign,
#723 globalref, #727 castindex, #728 castternary, #726 callpush on, #720 structheadless off), every
changed function classified by `hunks-corpus.py`. The first 12 binaries are the implementer's set,
then 9 from the round-2 review, 12 from the round-3 review, 14 from the round-4 review and 12 from
the round-7 review, each chosen to be disjoint from the ones before.

A hunk is IN SCOPE when it lies inside a function the option parked a declared prototype on, or
inside a function that CALLS one without growing an argument at any call to a parked function or
a line of its own. A caller that grows an argument or a line is `UNEXPLAINED-caller-grew`: an
argument is what a declaration longer than the body invents (the round-2 `x6`, `x7`), and a line
is what a declared return narrower than the caller's read of it splits the register into. A call
that passed more than the declared list and now passes exactly the list dropped a phantom
argument (`caller-dropped-an-argument`, in scope). Any changed function that gains a `CONCAT` is
`UNEXPLAINED-concat`, and a function that is not itself parked whose OWN parameter types change is
`UNEXPLAINED-own-params`: the declaration rebuilds a call, never what the caller was handed. The two
cast columns count each function's casts in both arms with the castbench counter (`castcount.py`,
the file's own type vocabulary); the parameter columns are the printed signature's types.

| binary | function | kind | +/- lines | params off | params on | casts off | casts on |
|---|---|---|---|---|---|---|---|
| O0/bzip2/stripped/bzip2 | sub_3b72 @0x3b72 | parked | 2 |  | int | 0 | 0 |
| O0/bzip2/stripped/bzip2 | sub_3bb0 @0x3bb0 | parked | 2 |  | int | 0 | 0 |
| O0/coreutils/stripped/cut | sub_3a72 @0x3a72 | parked | 9 | unsigned long *, unsigned long * | void *, void * | 4 | 9 |
| O0/coreutils/stripped/ptx | sub_34f6 @0x34f6 | parked | 32 | struct_2 *, struct_2 * | void *, void * | 19 | 32 |
| O0/coreutils/stripped/ptx | sub_3663 @0x3663 | parked | 8 | struct_2 *, struct_2 * | void *, void * | 4 | 5 |
| O0/coreutils/stripped/sort | sub_590b @0x590b | parked | 9 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O0/coreutils/stripped/sort | sub_c481 @0xc481 | parked | 6 | unsigned long * | void * | 0 | 7 |
| O0/dash/stripped/dash | sub_1196d @0x1196d | parked | 9 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O0/e2fsprogs/stripped/e2fsck | sub_2780c @0x2780c | parked | 10 | struct_166 *, struct_166 * | void *, void * | 2 | 8 |
| O0/e2fsprogs/stripped/e2fsck | sub_3d7a3 @0x3d7a3 | parked | 4 | void *, void * | void *, void * | 4 | 4 |
| O0/e2fsprogs/stripped/e2fsck | sub_5448d @0x5448d | parked | 10 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O0/e2fsprogs/stripped/e2fsck | sub_8218b @0x8218b | parked | 8 | uint4 *, uint4 * | void *, void * | 0 | 4 |
| O0/e2fsprogs/stripped/e2fsck | sub_8adcf @0x8adcf | parked | 16 | struct_62 * | void * | 0 | 7 |
| O0/gnutls/stripped/certtool | sub_37e6f @0x37e6f | parked | 18 | struct_24 *, struct_24 * | void *, void * | 6 | 15 |
| O0/grep/stripped/grep | sub_161d8 @0x161d8 | parked | 4 | long *, long * | void *, void * | 2 | 6 |
| O0/iproute2/stripped/ip | sub_8c84d @0x8c84d | parked | 8 | uint8 *, uint8 * | void *, void * | 0 | 4 |
| O0/kmod/stripped/kmod | sub_12bf4 @0x12bf4 | parked | 9 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O0/libselinux/stripped/libselinux.so.1 | sub_d127 @0xd127 | parked | 9 | void * | void * | 4 | 4 |
| O0/libselinux/stripped/libselinux.so.1 | sub_1ddc7 @0x1ddc7 | parked | 2 |  | void * | 0 | 0 |
| O0/libselinux/stripped/libselinux.so.1 | sub_1e961 @0x1e961 | parked | 2 |  | void * | 10 | 10 |
| O0/libselinux/stripped/libselinux.so.1 | sub_2399c @0x2399c | parked | 82 | char * | void * | 39 | 57 |
| O0/libselinux/stripped/libselinux.so.1 | sub_26db8 @0x26db8 | parked | 2 |  | void * | 6 | 6 |
| O0/rsyslog/stripped/rsyslogd | sub_46af0 @0x46af0 | parked | 10 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O0/rsyslog/stripped/rsyslogd | sub_707a6 @0x707a6 | parked | 4 | unsigned long *, unsigned long * | void *, void * | 2 | 4 |
| O0/rsyslog/stripped/rsyslogd | sub_75298 @0x75298 | parked | 6 | void * | void * | 55 | 55 |
| O0/rsyslog/stripped/rsyslogd | sub_893fc @0x893fc | parked | 10 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O0/rsyslog/stripped/rsyslogd | sub_8942b @0x8942b | parked | 10 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O0/rsyslog/stripped/rsyslogd | sub_8945a @0x8945a | parked | 13 | uint4 *, uint4 * | void *, void * | 0 | 4 |
| O0/rsyslog/stripped/rsyslogd | sub_89493 @0x89493 | parked | 13 | uint4 *, uint4 * | void *, void * | 0 | 4 |
| O0/rsyslog/stripped/rsyslogd | sub_894cc @0x894cc | parked | 8 | char *, unsigned long * | void *, void * | 1 | 1 |
| O0/rsyslog/stripped/rsyslogd | sub_894f8 @0x894f8 | parked | 8 | char *, unsigned long * | void *, void * | 1 | 1 |
| O0/rsyslog/stripped/rsyslogd | sub_8acbf @0x8acbf | parked | 4 | void * | void * | 11 | 11 |
| O0/rsyslog/stripped/rsyslogd | sub_9ef8e @0x9ef8e | parked | 10 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O0/rsyslog/stripped/rsyslogd | sub_9f0f3 @0x9f0f3 | parked | 8 | char *, unsigned long * | void *, void * | 1 | 1 |
| O0/tar/stripped/tar | sub_20a2b @0x20a2b | parked | 10 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O0/tar/stripped/tar | sub_3550c @0x3550c | parked | 2 | unsigned int | int4 | 0 | 0 |
| O2-noinline/coreutils/stripped/cut | sub_38d0 @0x38d0 | parked | 11 | int *, int * | void *, void * | 0 | 4 |
| O2-noinline/coreutils/stripped/sort | sub_5930 @0x5930 | parked | 9 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2-noinline/coreutils/stripped/sort | sub_a710 @0xa710 | parked | 6 | unsigned long * | void * | 0 | 7 |
| O2-noinline/dash/stripped/dash | sub_ed90 @0xed90 | parked | 9 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2-noinline/dash/stripped/dash | sub_15fd0 @0x15fd0 | parked | 6 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2-noinline/dpkg/stripped/dpkg-query | sub_7a10 @0x7a10 | parked | 2 | void * | void * | 5 | 5 |
| O2-noinline/dpkg/stripped/dpkg-query | sub_e2c0 @0xe2c0 | parked | 2 | void * | void * | 3 | 3 |
| O2-noinline/e2fsprogs/stripped/e2fsck | sub_1e780 @0x1e780 | parked | 10 | struct_155 *, struct_155 * | void *, void * | 2 | 8 |
| O2-noinline/e2fsprogs/stripped/e2fsck | sub_414e0 @0x414e0 | parked | 10 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2-noinline/e2fsprogs/stripped/e2fsck | sub_605e0 @0x605e0 | parked | 11 | uint4 *, uint4 * | void *, void * | 0 | 4 |
| O2-noinline/e2fsprogs/stripped/e2fsck | sub_66e40 @0x66e40 | parked | 16 | struct_46 * | void * | 0 | 7 |
| O2-noinline/gnutls/stripped/systemkey | sub_1f290 @0x1f290 | parked | 15 | struct_3 *, struct_3 * | void *, void * | 2 | 5 |
| O2-noinline/kmod/stripped/kmod | sub_e3a0 @0xe3a0 | parked | 9 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2-noinline/libselinux/stripped/libselinux.so.1 | sub_c5b0 @0xc5b0 | parked | 13 | void * | void * | 3 | 3 |
| O2-noinline/libselinux/stripped/libselinux.so.1 | sub_17d90 @0x17d90 | parked | 2 |  | void * | 0 | 0 |
| O2-noinline/libselinux/stripped/libselinux.so.1 | sub_18d90 @0x18d90 | parked | 2 |  | void * | 10 | 10 |
| O2-noinline/libselinux/stripped/libselinux.so.1 | sub_1c7e0 @0x1c7e0 | parked | 80 | char * | void * | 37 | 58 |
| O2-noinline/libselinux/stripped/libselinux.so.1 | sub_1e8d0 @0x1e8d0 | parked | 2 |  | void * | 6 | 6 |
| O2-noinline/tar/stripped/tar | sub_1d420 @0x1d420 | parked | 10 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2-noinline/tar/stripped/tar | sub_2be90 @0x2be90 | parked | 2 | unsigned int | int4 | 0 | 0 |
| O2/bzip2/stripped/bzip2 | sub_33d0 @0x33d0 | parked | 2 |  | int | 0 | 0 |
| O2/bzip2/stripped/bzip2 | sub_34c0 @0x34c0 | parked | 2 |  | int | 0 | 0 |
| O2/coreutils/stripped/sort | sub_7300 @0x7300 | parked | 9 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2/coreutils/stripped/sort | sub_ba80 @0xba80 | parked | 6 | unsigned long * | void * | 0 | 7 |
| O2/cronie/stripped/crontab | sub_4920 @0x4920 | parked | 2 |  | int | 0 | 0 |
| O2/dash/stripped/dash | sub_15a90 @0x15a90 | parked | 10 | unsigned long *, unsigned long * | void *, void * | 4 | 4 |
| O2/dpkg/stripped/dpkg-divert | sub_7750 @0x7750 | parked | 2 | void * | void * | 5 | 5 |
| O2/dpkg/stripped/dpkg-divert | sub_e150 @0xe150 | parked | 2 | void * | void * | 3 | 3 |
| O2/dpkg/stripped/dpkg-statoverride | sub_8090 @0x8090 | parked | 2 | void * | void * | 3 | 3 |
| O2/e2fsprogs/stripped/e2fsck | sub_1de20 @0x1de20 | parked | 10 | struct_126 *, struct_126 * | void *, void * | 0 | 8 |
| O2/e2fsprogs/stripped/e2fsck | sub_40200 @0x40200 | parked | 10 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2/e2fsprogs/stripped/e2fsck | sub_60260 @0x60260 | parked | 11 | uint4 *, uint4 * | void *, void * | 0 | 4 |
| O2/e2fsprogs/stripped/e2fsck | sub_66060 @0x66060 | parked | 16 | struct_42 * | void * | 0 | 7 |
| O2/gnutls/stripped/certtool | sub_26620 @0x26620 | parked | 15 | struct_15 *, struct_15 * | void *, void * | 2 | 5 |
| O2/grep/stripped/grep | sub_f930 @0xf930 | parked | 4 | long *, long * | void *, void * | 2 | 6 |
| O2/libedit/stripped/libedit.so.0.0.70 | sub_1fc80 @0x1fc80 | parked | 9 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2/libselinux/stripped/libselinux.so.1 | sub_c6d0 @0xc6d0 | parked | 13 | void * | void * | 3 | 3 |
| O2/libselinux/stripped/libselinux.so.1 | sub_17f70 @0x17f70 | parked | 2 |  | void * | 2 | 2 |
| O2/libselinux/stripped/libselinux.so.1 | sub_18f70 @0x18f70 | parked | 2 |  | void * | 10 | 10 |
| O2/libselinux/stripped/libselinux.so.1 | sub_1f7c0 @0x1f7c0 | parked | 2 |  | void * | 6 | 6 |
| O2/openssh-portable/stripped/scp | sub_30a20 @0x30a20 | parked | 9 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2/openssh-portable/stripped/scp | sub_30a30 @0x30a30 | parked | 9 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2/rsyslog/stripped/rsyslogd | sub_39bd0 @0x39bd0 | parked | 10 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2/rsyslog/stripped/rsyslogd | sub_57640 @0x57640 | parked | 4 | unsigned long *, int4 * | void *, void * | 1 | 3 |
| O2/rsyslog/stripped/rsyslogd | sub_5bae0 @0x5bae0 | parked | 6 | void * | void * | 50 | 50 |
| O2/rsyslog/stripped/rsyslogd | sub_68c00 @0x68c00 | parked | 6 | uint4 *, uint4 * | void *, void * | 0 | 4 |
| O2/rsyslog/stripped/rsyslogd | sub_68c20 @0x68c20 | parked | 6 | uint4 *, uint4 * | void *, void * | 0 | 4 |
| O2/rsyslog/stripped/rsyslogd | sub_68d30 @0x68d30 | parked | 10 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2/rsyslog/stripped/rsyslogd | sub_68d40 @0x68d40 | parked | 10 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2/rsyslog/stripped/rsyslogd | sub_69cf0 @0x69cf0 | parked | 6 | void * | void * | 19 | 19 |
| O2/rsyslog/stripped/rsyslogd | sub_781f0 @0x781f0 | parked | 10 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2/shadow/stripped/login | sub_52f0 @0x52f0 | parked | 2 |  | int | 0 | 0 |
| O2/tar/stripped/tar | sub_1e2b0 @0x1e2b0 | parked | 10 | unsigned long *, unsigned long * | void *, void * | 2 | 2 |
| O2/tar/stripped/tar | sub_2d500 @0x2d500 | parked | 2 | unsigned int | int4 | 0 | 0 |
| O2/tar/stripped/tar | sub_3ac00 @0x3ac00 | parked | 223 | struct_70 *, struct_70 * | void *, void * | 58 | 73 |

**91 functions change of 35,291; all 91 are the parked callbacks. No caller changes, no function that
is not itself parked changes its own parameter types, and nothing is UNEXPLAINED.** The run parks
91 functions.

**No caller changes.** The round-7 review found one on the build before this one: `ptx -O0`'s
`search_table` (`sub_36cb`), a bsearch helper that calls the parked comparator `compare_words`
directly with its own `WORD *`. It was decompiled again because it uses the comparator's result, and
its parameter printed `void *` where it had `struct_2 *` (DWARF: `WORD *`), because the only thing
that typed it was the comparator's own reads, which the declared `void *` replaced. The
comparator's first-decompile statement is now kept for its call sites, and where the slot says only
`void *` it still types what a caller passes, so `search_table` prints byte-identical to `off`. The
same review's fixture (`callbacktype_forward_x86_64`, the `tests/cli` probe) pins the shape at gcc
-O0, clang -O2 and clang -O0. A caller is also decompiled again less often: one that uses the result
is redone only when the declared return differs from the return the callback already printed.
Before the redo was bounded, `libselinux -O2-noinline`'s `sub_1ce10` and `selinux_restorecon_xattr`
were decompiled again for their calls to a parked `pthread_once` routine and printed `a1` as
`unsigned int`; that routine now agrees with its body and is not parked.

Twenty-two binaries print no changed function: `coreutils/fmt -O2`, `ls -O0` and `-O2`, `du -O2`,
`timeout -O2`, `ginstall -O2`, `ptx -O2` and `-O2-noinline`, `diffutils/diff -O2` and `sdiff -O2`,
`findutils/find -O0` and `-O2-noinline`, `gzip -O2`, `shadow/su -O0` and `-O2`, `useradd -O2` and
`-O2-noinline`, `sysvinit/init -O2` and `shutdown -O2`, `cronie/crond -O0`, `gnutls/gnutls-serv -O2`
and `openssh-portable/sftp -O0`. `su`'s `kill_child` handler is the `recovered-fewer-inputs` refusal
(the program also calls it directly), `diff -O2`'s `compare_names_for_qsort` is the
`returns-no-value` refusal of a value that reaches its RETURN through a join, and `ptx`'s pair at -O2
is the width refusal: `compare_occurs` hands `compare_words`' whole `rax` on through its own `ret`
(`caller-reads-a-wider-return` for `compare_words`), and the value it hands back on that path is a
call's full register, which nothing bounds (`recovered-wider-output` for `compare_occurs`). DWARF
declares both `int`, so these are misses, not wrong answers.

What moves inside a parked function: the declaration and the body rendering that follows from it.
A `void *` parameter prints its dereferences with an explicit cast where an `unsigned long *`
printed `*a0`, and a comparator that gains its `int` return renders the call's result through a
temporary. The largest hunk, `tar -O2`'s `sub_3ac00` (223 +/- lines, most of them locals
renumbered), is a comparator that went from `uint8 sub_3ac00(struct_70 *a0,struct_70 *a1)` to the
declared `int4 sub_3ac00(void *a0,void *a1)`: `return 0xffffffff` prints as `return -1`, the
synthesized record's field reads print as casts off the `void *` (the one place `callbacktype` and
`structsynth` disagree), and one `if (v) return v; return 1;` renders as the equivalent
`if (!v) v = 1; return v;`.

**Casts outside castbench.** Over the 91 changed functions the count goes from 457 to 655 (+198):
31 functions print more, 0 fewer, 60 the same. In every one of the 31 the body's own guess for the
parameter was a pointer to something (a synthesized `struct_N *`, a `char *`, an `unsigned long *`,
a `uint4 *`) and the slot declares `void *`; DWARF agrees with the slot in every one (`void *(void
*)` for the thread routines, `int (const void *, const void *)` for the comparators). The source
converts that `void *` to its record type on entry (`const struct x *p = a;`), which no stripped
binary shows, so each field read prints its own cast off the `void *`, as IDA's does. The largest
are `libselinux`'s `selinux_restorecon_thread` (`char *` -> `void *`, +21 at -O2-noinline, +18 at
-O0), `tar`'s `hol_entry_qcmp` (+15), `ptx -O0`'s `compare_words` (+13), `certtool -O0`'s
`setof_compar` (+9), `e2fsck`'s `process_inode_cmp` (+8 at -O2, +6 at -O0 and -O2-noinline) and
`read_bitmaps_thread` (0 -> 7 at every level), and `sort`'s `sortlines_thread` (+7 at each level,
which is IDA's own count for that function). One of `process_inode_cmp`'s six is an address passed
to an import whose recovered parameter is an integer: `ext2fs_const_inode(&a0->field_0x4[0x14])`
prints `ext2fs_const_inode((int8)a0 + 0x18)`, the same value, in the `(long)v` form castarith
leaves to integer arithmetic. Nothing is weakened: the parameter moves from an inferred pointee to
the declared type, the return from a wrong `unsigned long`/`uint8` to the declared `void *`/`int`.

Raw rows, with the per-function diff, both parameter lists and both cast counts: `hunks.json`.
