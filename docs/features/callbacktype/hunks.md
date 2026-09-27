# `callbacktype` whole-corpus hunk classification

Before/after `kuna decompile-all` over 70 decbench binaries and the 5 callbacktype fixtures (40,724
functions), `--option callbacktype off` against the default, same build (be75f8cc8 on 0096e984d),
every changed function classified by `hunks-corpus.py`. The binaries are the implementer's 12 and
the 9, 12, 14, 12 and 11 that successive reviews chose to be disjoint from the ones before.

A hunk is IN SCOPE only when it lies inside a function the option parked a declared prototype on.
The park round runs after every other decompile of the run and decompiles only the parked
functions again, so any other function that prints differently -- a direct caller of a parked
callback included -- is `UNEXPLAINED-not-parked`, and a parked function that gains a `CONCAT` is
`UNEXPLAINED-concat`. The two cast columns count each function's casts in both arms with the
castbench counter (`castcount.py`, the file's own type vocabulary).

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
| callbacktype_forward_x86_64 | compare_g @0x11a9 | parked | 14 | struct_0 *, struct_0 * | void *, void * | int | int | 8 | 14 |
| callbacktype_forward_x86_64 | compare_c @0x14f0 | parked | 8 | struct_1 *, struct_1 * | void *, void * | int | int | 2 | 6 |
| callbacktype_forward_x86_64 | by_name_g2 @0x17f0 | parked | 9 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| callbacktype_forward_x86_64 | worker_g6 @0x19a0 | parked | 6 | long | void * | long | void * | 6 | 8 |
| callbacktype_narrow_x86_64 | lw @0x1300 | parked | 4 | int * | void * | unsigned int | void * | 2 | 6 |
| callbacktype_narrow_x86_64 | zcmp @0x1310 | parked | 4 | char *, char * | void *, void * | char | int | 0 | 3 |
| callbacktype_narrow_x86_64 | zw @0x1320 | parked | 4 | long * | void * | bool | void * | 0 | 3 |
| callbacktype_narrow_x86_64 | iw @0x1330 | parked | 4 | unsigned int * | void * | unsigned int | void * | 0 | 3 |
| callbacktype_x86_64 | by_key @0x12d0 | parked | 4 | int *, int * | void *, void * | int | int | 0 | 2 |
| callbacktype_x86_64 | on_int @0x12f0 | parked | 2 |  | int | void | void | 0 | 0 |
| callbacktype_x86_64 | by_name @0x1310 | parked | 9 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| callbacktype_x86_64 | by_count @0x1320 | parked | 7 | unsigned long *, unsigned long * | void *, void * | void | int | 2 | 2 |
| callbacktype_x86_64 | warn_path @0x1340 | parked | 7 | char *, int | char *, int | void | int | 0 | 0 |

**128 functions change of 40,724, and every one is a parked callback (115 in the decbench binaries,
13 in the fixtures). No function that is not parked changes at all -- no caller, no parameter
type, no return type, no cast -- and nothing is UNEXPLAINED (0).** The 115 decbench rows are the
same functions with the same diffs as the previous build's parked rows, which also redid 4
callers in the forwarding fixture; those callers are now byte-identical between the arms, and so
are the direct callers in the round-9 review's four constructed pthread builds (run through
`HUNKS_EXTRA`, not listed here). Casts over the changed decbench functions: 553 -> 769, 37 print
more and 1 fewer. Each that prints more reads fields through a parameter that moves from a
guessed pointee to the declared `void *`, which DWARF gives; the one that prints fewer (`xmlwf`'s
`nsattcmp`) returns its difference as the declared `int` without an `(unsigned long)` (see the
evaluation, (f)).

