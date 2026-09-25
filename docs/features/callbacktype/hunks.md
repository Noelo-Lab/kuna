# `callbacktype` whole-corpus hunk classification

Before/after `kuna decompile-all` over 70 decbench binaries and the forwarding fixture (40,613 functions),
`--option callbacktype off` against the default, same build (99f849ef6, rebased onto 850e8c692:
#722 castarith, #724 castsign, #723 globalref, #727 castindex, #728 castternary, #726 callpush on,
#720 structheadless off), every changed function classified by `hunks-corpus.py`. The first 12
binaries are the implementer's set, then 9 from the round-2 review, 12 from the round-3 review, 14
from the round-4 review, 12 from the round-7 review and 11 from the round-8 review, each chosen to
be disjoint from the ones before; the last input is `callbacktype_forward_x86_64`, whose callers
are the two shapes the round-7 and round-8 reviews found.

A hunk is IN SCOPE when it lies inside a function the option parked a declared prototype on, or
inside a function that CALLS one without growing an argument at any call to a parked function or
a line of its own. A caller that grows an argument or a line is `UNEXPLAINED-caller-grew`: an
argument is what a declaration longer than the body invents (the round-2 `x6`, `x7`), and a line
is what a declared return narrower than the caller's read of it splits the register into. A call
that passed more than the declared list and now passes exactly the list dropped a phantom
argument (`caller-dropped-an-argument`, in scope). Any changed function that gains a `CONCAT` is
`UNEXPLAINED-concat`, and a function that is not itself parked whose OWN parameter types change is
`UNEXPLAINED-own-params`: the declaration rebuilds a call, never what the caller was handed. The
one exception is `caller-gained-a-forwarded-parameter`: a caller whose first decompile saw fewer
parameters than it passes on keeps every type it had and gains, after them, only parameters it
hands straight to a parked function whose call grew to exactly the declared list. The return
columns are the printed return types; a function that is not parked whose return moves is listed
below the table. The two cast columns count each function's casts in both arms with the castbench
counter (`castcount.py`, the file's own type vocabulary).

| binary | function | kind | +/- lines | params off | params on | return off | return on | casts off | casts on |
|---|---|---|---|---|---|---|---|---|---|
| O0/bzip2/stripped/bzip2 | sub_3b72 @0x3b72 | parked | 2 |  | int | void | void | 0 | 0 |
| O0/bzip2/stripped/bzip2 | sub_3bb0 @0x3bb0 | parked | 2 |  | int | void | void | 0 | 0 |
| O0/coreutils/stripped/cut | sub_3a72 @0x3a72 | parked | 9 | unsigned long *, unsigned long * | void *, void * | unsigned long | int | 4 | 9 |
| O0/coreutils/stripped/ptx | sub_34f6 @0x34f6 | parked | 32 | struct_2 *, struct_2 * | void *, void * | unsigned int | int | 19 | 32 |
| O0/coreutils/stripped/ptx | sub_3663 @0x3663 | parked | 8 | struct_2 *, struct_2 * | void *, void * | unsigned int | int | 4 | 5 |
| O0/coreutils/stripped/sort | sub_590b @0x590b | parked | 9 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| O0/coreutils/stripped/sort | sub_c481 @0xc481 | parked | 6 | unsigned long * | void * | unsigned long | void * | 0 | 7 |
| O0/dash/stripped/dash | sub_1196d @0x1196d | parked | 9 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| O0/dpkg/stripped/dpkg-query | sub_7fa4 @0x7fa4 | parked | 2 | void * | void * | unsigned long | int | 3 | 3 |
| O0/dpkg/stripped/dpkg-query | sub_10b3b @0x10b3b | parked | 2 | void * | void * | unsigned long | int | 3 | 3 |
| O0/dpkg/stripped/dpkg-trigger | sub_64ae @0x64ae | parked | 2 | void * | void * | unsigned long | int | 3 | 3 |
| O0/e2fsprogs/stripped/e2fsck | sub_2780c @0x2780c | parked | 10 | struct_166 *, struct_166 * | void *, void * | int4 | int4 | 2 | 8 |
| O0/e2fsprogs/stripped/e2fsck | sub_3d7a3 @0x3d7a3 | parked | 4 | void *, void * | void *, void * | unsigned long | int4 | 4 | 4 |
| O0/e2fsprogs/stripped/e2fsck | sub_5448d @0x5448d | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O0/e2fsprogs/stripped/e2fsck | sub_8218b @0x8218b | parked | 8 | uint4 *, uint4 * | void *, void * | unsigned long | int4 | 0 | 4 |
| O0/e2fsprogs/stripped/e2fsck | sub_8adcf @0x8adcf | parked | 16 | struct_62 * | void * | unsigned long | void * | 0 | 7 |
| O0/gnutls/stripped/certtool | sub_37e6f @0x37e6f | parked | 18 | struct_24 *, struct_24 * | void *, void * | unsigned int | int | 6 | 15 |
| O0/grep/stripped/grep | sub_161d8 @0x161d8 | parked | 4 | long *, long * | void *, void * | int | int | 2 | 6 |
| O0/iproute2/stripped/ip | sub_8c84d @0x8c84d | parked | 8 | uint8 *, uint8 * | void *, void * | unsigned long | int4 | 0 | 4 |
| O0/kmod/stripped/kmod | sub_12bf4 @0x12bf4 | parked | 9 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| O0/libselinux/stripped/libselinux.so.1 | sub_d127 @0xd127 | parked | 9 | void * | void * | unsigned long | int | 4 | 4 |
| O0/libselinux/stripped/libselinux.so.1 | sub_1ddc7 @0x1ddc7 | parked | 2 |  | void * | void | void | 0 | 0 |
| O0/libselinux/stripped/libselinux.so.1 | sub_1e961 @0x1e961 | parked | 2 |  | void * | void | void | 10 | 10 |
| O0/libselinux/stripped/libselinux.so.1 | sub_2399c @0x2399c | parked | 82 | char * | void * | unsigned long | void * | 39 | 57 |
| O0/libselinux/stripped/libselinux.so.1 | sub_26db8 @0x26db8 | parked | 2 |  | void * | void | void | 6 | 6 |
| O0/rsyslog/stripped/rsyslogd | sub_46af0 @0x46af0 | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O0/rsyslog/stripped/rsyslogd | sub_707a6 @0x707a6 | parked | 4 | unsigned long *, unsigned long * | void *, void * | int4 | int4 | 2 | 4 |
| O0/rsyslog/stripped/rsyslogd | sub_75298 @0x75298 | parked | 6 | void * | void * | unsigned long | void * | 55 | 55 |
| O0/rsyslog/stripped/rsyslogd | sub_893fc @0x893fc | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O0/rsyslog/stripped/rsyslogd | sub_8942b @0x8942b | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O0/rsyslog/stripped/rsyslogd | sub_8945a @0x8945a | parked | 13 | uint4 *, uint4 * | void *, void * | int4 | int4 | 0 | 4 |
| O0/rsyslog/stripped/rsyslogd | sub_89493 @0x89493 | parked | 13 | uint4 *, uint4 * | void *, void * | int4 | int4 | 0 | 4 |
| O0/rsyslog/stripped/rsyslogd | sub_894cc @0x894cc | parked | 8 | char *, unsigned long * | void *, void * | void | int4 | 1 | 1 |
| O0/rsyslog/stripped/rsyslogd | sub_894f8 @0x894f8 | parked | 8 | char *, unsigned long * | void *, void * | void | int4 | 1 | 1 |
| O0/rsyslog/stripped/rsyslogd | sub_8acbf @0x8acbf | parked | 4 | void * | void * | unsigned long | void * | 11 | 11 |
| O0/rsyslog/stripped/rsyslogd | sub_9ef8e @0x9ef8e | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O0/rsyslog/stripped/rsyslogd | sub_9f0f3 @0x9f0f3 | parked | 8 | char *, unsigned long * | void *, void * | void | int4 | 1 | 1 |
| O0/tar/stripped/tar | sub_20a2b @0x20a2b | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O0/tar/stripped/tar | sub_3550c @0x3550c | parked | 2 | unsigned int | int4 | void | void | 0 | 0 |
| O2-noinline/coreutils/stripped/cut | sub_38d0 @0x38d0 | parked | 11 | int *, int * | void *, void * | unsigned long | int | 0 | 4 |
| O2-noinline/coreutils/stripped/sort | sub_5930 @0x5930 | parked | 9 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| O2-noinline/coreutils/stripped/sort | sub_a710 @0xa710 | parked | 6 | unsigned long * | void * | unsigned long | void * | 0 | 7 |
| O2-noinline/dash/stripped/dash | sub_ed90 @0xed90 | parked | 9 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| O2-noinline/dash/stripped/dash | sub_15fd0 @0x15fd0 | parked | 6 | unsigned long *, unsigned long * | void *, void * | int | int | 2 | 2 |
| O2-noinline/dpkg/stripped/dpkg-query | sub_7a10 @0x7a10 | parked | 2 | void * | void * | unsigned long | int | 5 | 5 |
| O2-noinline/dpkg/stripped/dpkg-query | sub_e2c0 @0xe2c0 | parked | 2 | void * | void * | unsigned long | int | 3 | 3 |
| O2-noinline/e2fsprogs/stripped/e2fsck | sub_1e780 @0x1e780 | parked | 10 | struct_155 *, struct_155 * | void *, void * | int4 | int4 | 2 | 8 |
| O2-noinline/e2fsprogs/stripped/e2fsck | sub_414e0 @0x414e0 | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O2-noinline/e2fsprogs/stripped/e2fsck | sub_605e0 @0x605e0 | parked | 11 | uint4 *, uint4 * | void *, void * | uint8 | int4 | 0 | 4 |
| O2-noinline/e2fsprogs/stripped/e2fsck | sub_66e40 @0x66e40 | parked | 16 | struct_46 * | void * | unsigned long | void * | 0 | 7 |
| O2-noinline/gnutls/stripped/systemkey | sub_1f290 @0x1f290 | parked | 15 | struct_3 *, struct_3 * | void *, void * | int | int | 2 | 5 |
| O2-noinline/kmod/stripped/kmod | sub_e3a0 @0xe3a0 | parked | 9 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| O2-noinline/libselinux/stripped/libselinux.so.1 | sub_c5b0 @0xc5b0 | parked | 13 | void * | void * | bool | int | 3 | 3 |
| O2-noinline/libselinux/stripped/libselinux.so.1 | sub_17d90 @0x17d90 | parked | 2 |  | void * | void | void | 0 | 0 |
| O2-noinline/libselinux/stripped/libselinux.so.1 | sub_18d90 @0x18d90 | parked | 2 |  | void * | void | void | 10 | 10 |
| O2-noinline/libselinux/stripped/libselinux.so.1 | sub_1c7e0 @0x1c7e0 | parked | 80 | char * | void * | unsigned long | void * | 37 | 58 |
| O2-noinline/libselinux/stripped/libselinux.so.1 | sub_1e8d0 @0x1e8d0 | parked | 2 |  | void * | void | void | 6 | 6 |
| O2-noinline/rsyslog/stripped/rsyslogd | sub_39620 @0x39620 | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O2-noinline/rsyslog/stripped/rsyslogd | sub_56e30 @0x56e30 | parked | 4 | unsigned long *, int4 * | void *, void * | int4 | int4 | 1 | 3 |
| O2-noinline/rsyslog/stripped/rsyslogd | sub_5aa10 @0x5aa10 | parked | 8 | void * | void * | unsigned long | void * | 48 | 48 |
| O2-noinline/rsyslog/stripped/rsyslogd | sub_68570 @0x68570 | parked | 6 | uint4 *, uint4 * | void *, void * | int4 | int4 | 0 | 4 |
| O2-noinline/rsyslog/stripped/rsyslogd | sub_68590 @0x68590 | parked | 6 | uint4 *, uint4 * | void *, void * | int4 | int4 | 0 | 4 |
| O2-noinline/rsyslog/stripped/rsyslogd | sub_68820 @0x68820 | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O2-noinline/rsyslog/stripped/rsyslogd | sub_68830 @0x68830 | parked | 8 | char *, unsigned long * | void *, void * | void | int4 | 1 | 1 |
| O2-noinline/rsyslog/stripped/rsyslogd | sub_68840 @0x68840 | parked | 8 | char *, unsigned long * | void *, void * | void | int4 | 1 | 1 |
| O2-noinline/rsyslog/stripped/rsyslogd | sub_68850 @0x68850 | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O2-noinline/rsyslog/stripped/rsyslogd | sub_697e0 @0x697e0 | parked | 6 | void * | void * | unsigned long | void * | 9 | 9 |
| O2-noinline/rsyslog/stripped/rsyslogd | sub_77910 @0x77910 | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O2-noinline/rsyslog/stripped/rsyslogd | sub_77920 @0x77920 | parked | 8 | char *, unsigned long * | void *, void * | void | int4 | 1 | 1 |
| O2-noinline/shadow/stripped/sulogin | sub_2aa0 @0x2aa0 | parked | 2 |  | int | void | void | 0 | 0 |
| O2-noinline/sysvinit/stripped/sulogin | sub_3510 @0x3510 | parked | 2 |  | int | void | void | 0 | 0 |
| O2-noinline/tar/stripped/tar | sub_1d420 @0x1d420 | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O2-noinline/tar/stripped/tar | sub_2be90 @0x2be90 | parked | 2 | unsigned int | int4 | void | void | 0 | 0 |
| O2/bzip2/stripped/bzip2 | sub_33d0 @0x33d0 | parked | 2 |  | int | void | void | 0 | 0 |
| O2/bzip2/stripped/bzip2 | sub_34c0 @0x34c0 | parked | 2 |  | int | void | void | 0 | 0 |
| O2/coreutils/stripped/numfmt | sub_5930 @0x5930 | parked | 11 | int *, int * | void *, void * | unsigned long | int | 0 | 4 |
| O2/coreutils/stripped/sort | sub_7300 @0x7300 | parked | 9 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| O2/coreutils/stripped/sort | sub_ba80 @0xba80 | parked | 6 | unsigned long * | void * | unsigned long | void * | 0 | 7 |
| O2/cronie/stripped/crontab | sub_4920 @0x4920 | parked | 2 |  | int | void | void | 0 | 0 |
| O2/dash/stripped/dash | sub_15a90 @0x15a90 | parked | 10 | unsigned long *, unsigned long * | void *, void * | int | int | 4 | 4 |
| O2/dpkg/stripped/dpkg | sub_1df20 @0x1df20 | parked | 2 | void * | void * | unsigned long | int | 5 | 5 |
| O2/dpkg/stripped/dpkg | sub_261f0 @0x261f0 | parked | 2 | void * | void * | unsigned long | int | 3 | 3 |
| O2/dpkg/stripped/dpkg | sub_2d4c0 @0x2d4c0 | parked | 9 | long *, long * | void *, void * | void | int | 2 | 4 |
| O2/dpkg/stripped/dpkg-divert | sub_7750 @0x7750 | parked | 2 | void * | void * | unsigned long | int | 5 | 5 |
| O2/dpkg/stripped/dpkg-divert | sub_e150 @0xe150 | parked | 2 | void * | void * | unsigned long | int | 3 | 3 |
| O2/dpkg/stripped/dpkg-statoverride | sub_8090 @0x8090 | parked | 2 | void * | void * | unsigned long | int | 3 | 3 |
| O2/e2fsprogs/stripped/e2fsck | sub_1de20 @0x1de20 | parked | 10 | struct_126 *, struct_126 * | void *, void * | int4 | int4 | 0 | 8 |
| O2/e2fsprogs/stripped/e2fsck | sub_40200 @0x40200 | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O2/e2fsprogs/stripped/e2fsck | sub_60260 @0x60260 | parked | 11 | uint4 *, uint4 * | void *, void * | uint8 | int4 | 0 | 4 |
| O2/e2fsprogs/stripped/e2fsck | sub_66060 @0x66060 | parked | 16 | struct_42 * | void * | unsigned long | void * | 0 | 7 |
| O2/gnutls/stripped/certtool | sub_26620 @0x26620 | parked | 15 | struct_15 *, struct_15 * | void *, void * | int | int | 2 | 5 |
| O2/grep/stripped/grep | sub_f930 @0xf930 | parked | 4 | long *, long * | void *, void * | int | int | 2 | 6 |
| O2/libedit/stripped/libedit.so.0.0.70 | sub_1fc80 @0x1fc80 | parked | 9 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| O2/libexpat/stripped/xmlwf | sub_34d0 @0x34d0 | parked | 42 | long *, long * | void *, void * | unsigned long | int | 3 | 6 |
| O2/libexpat/stripped/xmlwf | sub_3a40 @0x3a40 | parked | 9 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| O2/libexpat/stripped/xmlwf | sub_3a50 @0x3a50 | parked | 8 | unsigned long *, unsigned long * | void *, void * | int | int | 3 | 2 |
| O2/libselinux/stripped/libselinux.so.1 | sub_c6d0 @0xc6d0 | parked | 13 | void * | void * | bool | int | 3 | 3 |
| O2/libselinux/stripped/libselinux.so.1 | sub_17f70 @0x17f70 | parked | 2 |  | void * | void | void | 2 | 2 |
| O2/libselinux/stripped/libselinux.so.1 | sub_18f70 @0x18f70 | parked | 2 |  | void * | void | void | 10 | 10 |
| O2/libselinux/stripped/libselinux.so.1 | sub_1f7c0 @0x1f7c0 | parked | 2 |  | void * | void | void | 6 | 6 |
| O2/openssh-portable/stripped/scp | sub_30a20 @0x30a20 | parked | 9 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| O2/openssh-portable/stripped/scp | sub_30a30 @0x30a30 | parked | 9 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| O2/rsyslog/stripped/rsyslogd | sub_39bd0 @0x39bd0 | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O2/rsyslog/stripped/rsyslogd | sub_57640 @0x57640 | parked | 4 | unsigned long *, int4 * | void *, void * | int4 | int4 | 1 | 3 |
| O2/rsyslog/stripped/rsyslogd | sub_5bae0 @0x5bae0 | parked | 6 | void * | void * | unsigned long | void * | 50 | 50 |
| O2/rsyslog/stripped/rsyslogd | sub_68c00 @0x68c00 | parked | 6 | uint4 *, uint4 * | void *, void * | int4 | int4 | 0 | 4 |
| O2/rsyslog/stripped/rsyslogd | sub_68c20 @0x68c20 | parked | 6 | uint4 *, uint4 * | void *, void * | int4 | int4 | 0 | 4 |
| O2/rsyslog/stripped/rsyslogd | sub_68d30 @0x68d30 | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O2/rsyslog/stripped/rsyslogd | sub_68d40 @0x68d40 | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O2/rsyslog/stripped/rsyslogd | sub_69cf0 @0x69cf0 | parked | 6 | void * | void * | unsigned long | void * | 19 | 19 |
| O2/rsyslog/stripped/rsyslogd | sub_781f0 @0x781f0 | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O2/shadow/stripped/login | sub_52f0 @0x52f0 | parked | 2 |  | int | void | void | 0 | 0 |
| O2/tar/stripped/tar | sub_1e2b0 @0x1e2b0 | parked | 10 | unsigned long *, unsigned long * | void *, void * | void | int4 | 2 | 2 |
| O2/tar/stripped/tar | sub_2d500 @0x2d500 | parked | 2 | unsigned int | int4 | void | void | 0 | 0 |
| O2/tar/stripped/tar | sub_3ac00 @0x3ac00 | parked | 223 | struct_70 *, struct_70 * | void *, void * | uint8 | int4 | 58 | 73 |
| callbacktype_forward_x86_64 | compare_g @0x1189 | parked | 14 | struct_0 *, struct_0 * | void *, void * | int | int | 8 | 14 |
| callbacktype_forward_x86_64 | compare_c @0x14a0 | parked | 8 | struct_1 *, struct_1 * | void *, void * | int | int | 2 | 6 |
| callbacktype_forward_x86_64 | compare_z @0x1600 | parked | 8 | struct_0 *, struct_0 * | void *, void * | int | int | 8 | 12 |
| callbacktype_forward_x86_64 | search_z @0x16d0 | caller-dropped-an-argument | 2 | struct_0 *, char **, long | struct_0 *, char **, long | unsigned int | unsigned int | 1 | 1 |
| callbacktype_forward_x86_64 | by_name_g2 @0x17a0 | parked | 9 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| callbacktype_forward_x86_64 | minimum_g2 @0x17d0 | caller-of-parked | 2 | unsigned long *, int | unsigned long *, int | unsigned long * | unsigned long * | 3 | 2 |
| callbacktype_forward_x86_64 | ent_before_c2 @0x1880 | caller-gained-a-forwarded-parameter | 4 |  | unsigned long *, unsigned long * | unsigned int | unsigned int | 1 | 1 |
| callbacktype_forward_x86_64 | by_name_c2 @0x1890 | parked | 9 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| callbacktype_forward_x86_64 | minimum_c2 @0x18a0 | caller-of-parked | 2 | unsigned long *, unsigned int | unsigned long *, unsigned int | unsigned long * | unsigned long * | 3 | 2 |

**124 functions change of 40,613: 120 parked callbacks and 4 callers, all 4 callers in the fixture. In
the 70 decbench binaries every changed function is a parked callback. No function that is not
parked changes a parameter type it had or its return type, and nothing is UNEXPLAINED (0).** The
run parks 120 functions (115 in decbench binaries).

**Callers.** In the 70 decbench binaries no caller changes. The round-7 review found one on an
earlier build: `ptx -O0`'s `search_table` (`sub_36cb`), a bsearch helper that calls the parked
comparator `compare_words` directly with its own `WORD *`. It was decompiled again because it uses
the comparator's result, and its parameter printed `void *` where it had `struct_2 *` (DWARF:
`WORD *`), because the only thing that typed it was the comparator's own reads, which the declared
`void *` replaced. The comparator's first-decompile statement is now kept for its call sites, and
where the slot says only `void *` it still types what a caller passes, so `search_table` prints
byte-identical to `off`.

The round-8 review found the same class with a primitive pointee. A comparator that hands only the
first word of each record to `strcmp` states `undefined8 *` (printed `unsigned long *`) for both
parameters and prints `void`, and a caller that forwards its own two pointers to it, or scans an
array of 24-byte records with it, took `unsigned long *` from that statement. The declared `int`
differs from the printed `void`, so each such caller that uses the result is decompiled again, and
the kept statement then refused an unknown pointee as saying no more than `void *`: both callers
took `void *` for their own parameters and return, and indexed the array as
`&((char *)a0)[0x18]` where `off` prints `&a0[3]`. The kept statement now carries any pointer but
`void *`, which is what `protoorder` voted at the same call before the park. None of the 70 binaries
holds that shape; the fixture (`callbacktype_forward_x86_64`, the `tests/cli` probe) pins both
shapes at gcc -O0, -O2 and clang -O0, -O2, and its four changed callers are the ones the table lists:

- `minimum_g2` and `minimum_c2` keep `unsigned long *` and `&a0[3]`. Their one change is that the
  comparator's result is read as the declared `int`: `(int)by_name_g2(v2,a0) <= -1` loses its
  `(int)` (3 -> 2 casts each).
- `ent_before_c2` (clang -O2) printed `unsigned int ent_before_c2(void)` and called the comparator
  with no arguments; the declared two-parameter call passes the two registers it forwards, so it
  gains `unsigned long *a0,unsigned long *a1` after the none it had, typed by the same kept vote.
  gcc's `ent_before_g2` already had both and prints byte-identical.
- `search_z` (clang -O0) drops the `idiv` remainder its call passed in `rdx`.

A caller that hands the result straight back can also take the declared return type. No function
in the corpus does; a constructed `int next_cmp(struct ent *p) { return p->next ? by_name(p,
p->next) : 0; }` at clang -O0 is the one seen, `unsigned int` -> `int`, which is the source's type
and the same 32 bits.

A caller is also decompiled again less often: one that uses the result is redone only when the
declared return differs from the return the callback already printed. Before the redo was bounded,
`libselinux -O2-noinline`'s `sub_1ce10` and `selinux_restorecon_xattr` were decompiled again for
their calls to a parked `pthread_once` routine and printed `a1` as `unsigned int`; that routine now
agrees with its body and is not parked.

Twenty-five binaries print no changed function: `coreutils/fmt -O2`, `ls -O0` and `-O2`, `du -O2`,
`timeout -O2`, `ginstall -O2`, `ptx -O2` and `-O2-noinline`, `diffutils/diff -O2` and `sdiff -O2`,
`findutils/find -O0` and `-O2-noinline`, `gzip -O2`, `shadow/su -O0` and `-O2`, `useradd -O2` and
`-O2-noinline`, `userdel -O0` and `-O2`, `chfn -O0`, `sysvinit/init -O2` and `shutdown -O2`,
`cronie/crond -O0`, `gnutls/gnutls-serv -O2` and `openssh-portable/sftp -O0`. `su`'s `kill_child` handler is the `recovered-fewer-inputs` refusal
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

**Casts outside castbench.** Over the 115 changed functions of the 70 decbench binaries the count
goes from 553 to 769 (+216): 37 functions print more, 1 fewer, 77 the same. The one with fewer is
`xmlwf -O2`'s `nsattcmp`, whose `return (unsigned long)(v5 - v4);` becomes the declared `int`'s
`return v5 - v4;`. In every one of the 37 the body's own guess for the parameter was a pointer to
something (a synthesized `struct_N *`, a `char *`, an `unsigned long *`, a `long *`, an `int *`, a
`uint4 *`) and the slot declares `void *`; DWARF agrees with the slot in every one (`void *(void *)`
for the thread routines, `int (const void *, const void *)` for the comparators, including the six
the round-8 review's binaries add: `xmlwf`'s `notationCmp`, `numfmt`'s `compare_ranges`, `dpkg`'s
`treenode_cmp`, and `rsyslogd -O2-noinline`'s `cmp`, `qs_arrcmp_uint32_index_val` and
`qs_arrcmp_sprsArrtab`). With the fixture the totals are 583 -> 811 over 124 functions, the fixture's
two `minimum_*` callers among the three with fewer. The source
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
