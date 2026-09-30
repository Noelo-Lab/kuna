# Footprint: a value read sign-sensitively keeps its own variable (volreuse)

`decompile-all --json` before and after, every changed function classified. The
first two sections measure the first revision against main `63dfb436c`; the last
measures what following the value through `+`, `-`, `*`, the bitwise operators,
`~`, unary `-` and `<<` adds on top of it (main `1f01d62e6`). Classes:

- **split** — a value an operation reads sign-sensitively is no longer printed as
  the global it is stored to; the store prints as `global = vN;` at the binary's
  store. Some large functions also restructure because the kept store statement
  makes its block non-trivial (a short-circuit fold is declined, a return block is
  duplicated differently); the statements are the same, differently nested.
- **move** — the same statements; a store to a global now prints where the binary
  stores (main printed it where `Merge` had merged or trimmed it).
- **renumber** — only `struct_N` numbering changed, because `sub_2d3a0` in
  O2-noinline tar (argp's `parse_opt`) now types its `state` parameter as a
  synthesized struct at the same offsets (0, 0x28, 0x30) instead of `int8 *`.

Store positions checked against `objdump`: dash O2 `0x7dd0` (`1f414` before
`1f40c`), `0xe1c0` (`1f038` right after `malloc`), dpkg-divert O2 `0xdcf0` (one
store pair at the end), crontab O2 `0x8ca0` (store after the `*a0` load), init O2
`0x7e50` (`d101` before `d100`), rtmon O2-noinline `*_a2n` (store before
`*a0 = ...`), gzip O2 `0x45c0` (stores at `469f`/`46b5` on the slow path only),
diff O2 `0xd860` (`26730` before `26728`), grep O2 `0x4ff0` (`2b838` before
`2b858`).

## 16 disjoint binaries: 25 of 4,135 functions

dash (O0, O2), cronie crond O0 / crontab O2, kmod O2, libexpat xmlwf (O0, O2),
sysvinit init (O0, O2) / shutdown O2-noinline, zlib minigzip O2 / example O0,
base-passwd update-passwd O0, dpkg-divert O2, iproute2 rtmon O2-noinline,
openssh sftp-server O2-noinline.

- split (20): crond O0 `0x41bd` `0x8499` `0xd6a9`; dash O0 `0x6e8c`; init O0
  `0x90ae`; crontab O2 `0x8ca0`; dash O2 `0x6d90` `0x6e90`; dpkg-divert O2
  `0x8450` `0xb1a0` `0xd450` `0xddf0` `0xdf10`; rtmon `0xb3a0` `0xb550` `0xb700`
  `0xba90` `0xbe30` `0xc040`; sftp-server `0x24dd0`.
- split + store order (3): dpkg-divert O2 `0xdcf0`; init O2 `0x7e50` `0x87b0`.
- move (2): dash O2 `0x7dd0` `0xe1c0`.

## 45 coreutils/diffutils/findutils/grep/gzip/tar binaries: 151 of 20,230 functions

- split (110; 13 with larger restructuring: O2/gzip/gzip@0x4bc0, O2/gzip/gzip@0x6e90, O2/gzip/gzip@0xa090, O2/gzip/gzip@0xd9d0, O2/tar/tar@0x25c10, O0/tar/tar@0xbdb0, O0/tar/tar@0xd818, O0/tar/tar@0x3321b, O2-noinline/gzip/gzip@0x4ed0, O2-noinline/gzip/gzip@0x7b60, O2-noinline/gzip/gzip@0xa5a0, O2-noinline/gzip/gzip@0xda60, O2-noinline/tar/tar@0x24710):
  O2/gzip/gzip@0x43b0, O2/gzip/gzip@0x44d0, O2/gzip/gzip@0x45c0, O2/gzip/gzip@0x4bc0, O2/gzip/gzip@0x6a80, O2/gzip/gzip@0x6e90, O2/gzip/gzip@0x9a50, O2/gzip/gzip@0xa090, O2/gzip/gzip@0xb160, O2/gzip/gzip@0xbe90, O2/gzip/gzip@0xc590, O2/gzip/gzip@0xcb20, O2/gzip/gzip@0xd6c0, O2/gzip/gzip@0xd9d0, O2/diffutils/cmp@0x2900, O2/diffutils/diff@0x4f20, O2/diffutils/diff@0xbbc0, O2/diffutils/diff@0xe830, O2/diffutils/sdiff@0x3b00, O2/coreutils/fmt@0x26a0, O2/coreutils/ls@0x4d10, O2/coreutils/sort@0x3ec0, O2/coreutils/wc@0x2860, O2/tar/tar@0xa9f0, O2/tar/tar@0xdd80, O2/tar/tar@0xf710, O2/tar/tar@0x10cf0, O2/tar/tar@0x11680, O2/tar/tar@0x1a0b0, O2/tar/tar@0x203b0, O2/tar/tar@0x23e80, O2/tar/tar@0x25c10, O2/tar/tar@0x2c180, O2/tar/tar@0x2c570, O2/tar/tar@0x2e4c0, O2/grep/grep@0x8710, O2/grep/grep@0x9130, O0/gzip/gzip@0x4677, O0/gzip/gzip@0x4a9e, O0/gzip/gzip@0x5b3e, O0/gzip/gzip@0x5f11, O0/gzip/gzip@0xb868, O0/gzip/gzip@0xc45d, O0/gzip/gzip@0xd67f, O0/gzip/gzip@0xd846, O0/gzip/gzip@0xe055, O0/gzip/gzip@0xe29d, O0/gzip/gzip@0xe655, O0/diffutils/cmp@0x2d6d, O0/diffutils/diff@0xfa13, O0/diffutils/diff@0x11b50, O0/diffutils/sdiff@0x4573, O0/coreutils/fmt@0x2bde, O0/coreutils/fmt@0x312d, O0/coreutils/fmt@0x3a4c, O0/coreutils/fmt@0x42aa, O0/coreutils/ls@0x70af, O0/coreutils/sort@0x5ba8, O0/tar/tar@0xb2cf, O0/tar/tar@0xbdb0, O0/tar/tar@0xd386, O0/tar/tar@0xd818, O0/tar/tar@0x106d8, O0/tar/tar@0x10c82, O0/tar/tar@0x182fd, O0/tar/tar@0x2a571, O0/tar/tar@0x3321b, O0/tar/tar@0x3399b, O0/tar/tar@0x36c56, O2-noinline/gzip/gzip@0x4290, O2-noinline/gzip/gzip@0x43b0, O2-noinline/gzip/gzip@0x44a0, O2-noinline/gzip/gzip@0x4aa0, O2-noinline/gzip/gzip@0x4db0, O2-noinline/gzip/gzip@0x4ed0, O2-noinline/gzip/gzip@0x7070, O2-noinline/gzip/gzip@0x7b60, O2-noinline/gzip/gzip@0xa5a0, O2-noinline/gzip/gzip@0xb290, O2-noinline/gzip/gzip@0xbe40, O2-noinline/gzip/gzip@0xc040, O2-noinline/gzip/gzip@0xc740, O2-noinline/gzip/gzip@0xcc20, O2-noinline/gzip/gzip@0xd720, O2-noinline/gzip/gzip@0xd830, O2-noinline/gzip/gzip@0xda60, O2-noinline/diffutils/cmp@0x2940, O2-noinline/diffutils/diff@0x4fa0, O2-noinline/diffutils/diff@0xc8f0, O2-noinline/diffutils/diff@0xe2b0, O2-noinline/diffutils/sdiff@0x3b60, O2-noinline/coreutils/fmt@0x2fd0, O2-noinline/coreutils/fmt@0x3150, O2-noinline/coreutils/ls@0xa550, O2-noinline/coreutils/ls@0xbda0, O2-noinline/coreutils/sort@0x5e70, O2-noinline/coreutils/sort@0x8b60, O2-noinline/tar/tar@0xda00, O2-noinline/tar/tar@0xeed0, O2-noinline/tar/tar@0xf0f0, O2-noinline/tar/tar@0x108d0, O2-noinline/tar/tar@0x10dd0, O2-noinline/tar/tar@0x19b30, O2-noinline/tar/tar@0x24710, O2-noinline/tar/tar@0x2af40, O2-noinline/tar/tar@0x2b310, O2-noinline/tar/tar@0x2d3a0, O2-noinline/tar/tar@0x2e690, O2-noinline/grep/grep@0x8ff0, O2-noinline/grep/grep@0x9630
- move (16): O2/gzip/gzip@0x5750, O2/gzip/gzip@0xd770, O2/diffutils/diff@0xd860, O2/coreutils/du@0x3c50, O2/coreutils/ls@0x7ff0, O2/grep/grep@0x4ff0, O2/grep/grep@0x8d20, O2/findutils/find@0x165e0, O0/gzip/gzip@0x72f1, O0/gzip/gzip@0xef1d, O0/grep/grep@0xa7d1, O2-noinline/gzip/gzip@0x5b50, O2-noinline/diffutils/diff@0xd3c0, O2-noinline/tar/tar@0x1e470, O2-noinline/grep/grep@0x5090, O2-noinline/findutils/find@0x16170
- renumber (25): O2-noinline/tar/tar@0x34680, O2-noinline/tar/tar@0x35450, O2-noinline/tar/tar@0x35910, O2-noinline/tar/tar@0x35a60, O2-noinline/tar/tar@0x35c50, O2-noinline/tar/tar@0x35cd0, O2-noinline/tar/tar@0x39b40, O2-noinline/tar/tar@0x3c7e0, O2-noinline/tar/tar@0x41580, O2-noinline/tar/tar@0x416b0, O2-noinline/tar/tar@0x416e0, O2-noinline/tar/tar@0x41740, O2-noinline/tar/tar@0x41930, O2-noinline/tar/tar@0x419f0, O2-noinline/tar/tar@0x41a40, O2-noinline/tar/tar@0x41b00, O2-noinline/tar/tar@0x43510, O2-noinline/tar/tar@0x4c3b0, O2-noinline/tar/tar@0x4dc90, O2-noinline/tar/tar@0x4e2f0, O2-noinline/tar/tar@0x55330, O2-noinline/tar/tar@0x55440, O2-noinline/tar/tar@0x56130, O2-noinline/tar/tar@0x565e0, O2-noinline/tar/tar@0x5c6f0

## Following the value through `+`, `^` and the other sign-passing operators

Main `1f01d62e6` against this revision, and the previous revision rebased onto
the same main (`a7514a801`) against this one, so the second number is exactly
what the walk through expressions changes.

27 disjoint binaries (the 16 above plus setfacl O2, getfacl O0, chage O2,
gpasswd O2-noinline, bzip2 O2, psktool O2, mirai O2, chfn O0, mkbuiltins O2,
e2fsck O2, rsyslogd O2): main to head 39 of 8,369 functions, previous revision to
head 7:

- split (2): chage O2 `0x4cc0` (`v12 = sub_7040(optarg); dat_130e8 = v12;`, then
  `v12 <= -2`), init O2 `0x87b0` (`dat_d650` is no longer read back after the
  call's result is stored; the binary keeps `eax`).
- store at the binary's position (2): dash O2 `0xe290` (`1f038` before the
  `1f3e0` decrement, `e347`/`e355`); rsyslogd O2 `0x29fc0` (`b99d8` before
  `b99d0`, `2a0e5`/`2a0ec`; `b99f0` before the pointer store on the `ferror`
  path, `2cb61`/`2cb6b`).
- equivalent reorder, not the binary's order: rsyslogd O2 `0x28f20` (two adjacent
  stores of registers to `b99d0`/`b99d8`, nothing between); `0x29fc0` (`b99dc`
  now ahead of two loads of `b99f8`/`b9a08`; `b99f0` of the same `v24` below
  `label_2cb72`, whose only `goto` has stored the same value).
- equivalent re-coalescing (1): crontab O2 `0x3bc0` (`v1 = optind` holds on the
  path that dropped `v3 = optind`).
- comment anchor only (1): e2fsck O2 `0x2b7e0` (`// branch-flip` moves from the
  store to the `if` it annotates).

45 coreutils/diffutils/findutils/grep/gzip/tar binaries (castbench): main to head
185 of 20,230 functions, previous revision to head 58. For every global whose
printed store count changed, the count was compared with the store instructions
to that address in the function (`objdump`), and the store orders below were
checked there.

- split (33): the value is no longer read back from the global it was stored to.
  O2 gzip `0x4bc0` `0xadd0` `0xb920` `0xc590` `0xd9d0` `0x9a50`, cmp O0 `0x2d6d`,
  diff O2 `0x4f20`, fmt O2 `0x3060`, sort O2 `0x3ec0`, wc O2 `0x2860`, tar O2
  `0xdd80`, grep O2 `0x9130`, tar O0 `0x17069` `0x3b734`, the thirteen O0
  `__progname` / `program_invocation_short_name` setters (cmp `0x4a4b`, diff
  `0x19a53`, diff3 `0x637f`, sdiff `0x632f`, cp `0x16d05`, du `0x11fda`, fmt
  `0x498f`, ls `0x18e89`, sort `0x1345f`, tail `0xccd0`, wc `0x5c1b`, tar
  `0x5c844`, find `0x2dd35`), O2-noinline gzip `0x4aa0` `0x4ed0` `0xbe40`
  `0xc9f0` `0xda60`, cmp `0x2940`, diff `0x4fa0`, ls `0xa550`, tar `0xda00`,
  grep `0x9630`. Where a store count changed it now equals the binary's (wc
  `d12c` 1, diff `26430` 1, gzip `dfe80` 1, `dfba0` 3, `1a004` 6, grep `2a840` 1)
  or moves toward it (tar `82a34` 7 to 4 of 3, the other two unchanged; grep
  `2b840` 4 to 3 of 2). `wc`, `diff` and `tar` now print one store at the join,
  as the binary makes it (`2b14`, `e045`); `ls` `0xa550` prints `max = MAX(max,
  w)` as the binary's conditional store instead of re-storing an unchanged
  `2733c` on every path.
- store order now the binary's (4): diff O2 `0xbbc0` (`ca0e`..`ca3c`), grep O2
  `0x9130` (`840`, `848`, then the byte store), gzip O2 `0x8460` (`dca10` before
  `dca0c`, `899d`/`89a4`), O2-noinline gzip `0x8640`.
- equivalent reorder of accesses to distinct globals with no call or pointer
  store between (12): gzip O2 `0x92b0`, cmp O2 `0x2900`, du O2 `0x3c50`, tar O2
  `0x27340` `0x3c8f0`, grep O2 `0x8d20`, gzip O0 `0x979a` `0x9d74` (`dd870` after
  `de218`/`de220`, not the binary's order) `0xa191` (a load), ls O0 `0x70af`,
  O2-noinline gzip `0x8840` `0x94b0`, tar `0x30600` (the load of `84c70` moves up
  past no store to it), grep `0x8560`.
- equivalent, with a redundant copy of an unchanged value (5 functions, some
  also in the split list): gzip O2 `0x4bc0` and O2-noinline `0x4aa0` `0x4ed0`
  (`v5 = dat_1909c;` right after `dat_1909c = v5`), gzip O2 `0xadd0` and
  O2-noinline `0xb070` (`v9 = dat_dca30,` repeated inside the condition), gzip
  O2 `0x9a50` (`dat_dca10 = v10;` twice after `v10 = dat_dca10;`), O2-noinline
  tar `0xdfd0` (`dat_83f48 = dat_83f58;` again on a path that stored it at
  `e061`), tar O2 `0x308a0` (obstack_finish's temporary store goes through
  `82c70` instead of `82c78`; both end equal).

The 12 further binaries of the next section add one knock-on of this revision
that is not a split: scp O2 `0xb730` prints `dat_442fc = 0;` in the `else` arm, a
store the binary never makes (`bde3: xor %edx,%edx` keeps the 0 in a register).
Upstream's forced marker merge joins that register phi (the global's value or 0)
into the global now that the computed value `v3` keeps its own variable, and the
constant input becomes a `COPY` into it. The binary's one store, `b83b`, overwrites
it on every path, and no call or pointer store lies between (`bde3` jumps to
`b7e7`, and the `bd50` divide path back to `b804`), so the only read in between,
`if (dat_442fc)`, sees the 0 the register held: equivalent.

Casts on the 4,815 shared functions: 32,074 on main `1f01d62e6` (32,073 on the
campaign arm `632437155`) to 32,020, 0.847x IDA, 26.9 per 100 statements. Thirty
functions lose 60 casts, mostly the `(u64)`/`(long)` around a re-read global. Six
gain one each: five print the same text on both arms (the counter's vocabulary is
per file), and tar O2 `0x2c570` is a split (`strchr` and `open` results keep
their own variables instead of being read back from `dat_82d78`/`dat_82a34`). Typesweep (444 slices, 10,748
functions): 10,748 same, 0 improved, 0 worse, 1,674 perfect on both arms, mean
0.3752; the exported variable count is unchanged in every function, and the 58
whose exported variables differ differ only in renumbered names.

## Readers a later rule makes sign-sensitive

This revision's walk also counts a short or byte `==`/`!=` reached through `+`,
`-`, `*`, `^`, `~`, unary `-` or `<<` whatever its constant, a carry, the low half
of a concatenation, and two compares `RuleRangeMeld` would join into an ordered
compare. The previous revision (`c55043175`) against this one:

- 39 disjoint binaries (the 27 above, plus scp O2, ssh-add O0, chsh O2, rtmon
  O0, csplit O2, cksum O0, chmod O2, srptool O2, mksyntax O2, faillog O2, comm O2,
  basename O2): 0 of 11,656 functions change.
- 45 castbench binaries: the printed C is byte-identical in all 45 files.

Main `1f01d62e6` against this revision on the 12 added binaries: 14 of 3,287
functions, all already changed by the previous revision.

- split (11): cksum O0 `0x149a0` (`program_invocation_short_name`), rtmon O0
  `0xc3bb` `0xc66c` `0xc91d` `0xd009` `0xd505` `0xd8f5` (the `*_a2n` family:
  `v2 = strtoul(...); dat_204d0 = v2;`, then `v2 <= 0xff`), ssh-add O0 `0xb180`
  (`v2 < 0`) and `0x5d698` (getopt: `v5 = sub_60810(dat_8b204,1); dat_8b204 = v5;
  if (a0 <= v5)`), scp O2 `0x5150` (`v3 <= 0`) and `0x30430` (the same getopt
  shape).
- split with an equivalent reorder (1): scp O2 `0x8520` prints `dat_442d8 = v14;`
  one register copy before `dat_442d0 = v15;`; the binary stores `442d0` (`902c`)
  then `442d8` (`9036`), with no call or pointer store between.
- renumber (1): basename O2 `0x2580`, local variable numbering only.
- equivalent extra store (1): scp O2 `0xb730`, above.

Two near misses print as before because the walk asks only what a rule can
actually do: a flag tested `!= 0` through `&` (tar O2 `0x27340`; no fold moves a
constant across `&`), and `v6 == 10 || v6 == -1` in fmt O2 `0x3700` (the two
ranges do not join, so `RuleRangeMeld` never rewrites it).
