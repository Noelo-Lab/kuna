# Footprint: a value read sign-sensitively keeps its own variable (volreuse)

`decompile-all --json` on main `0729b8b05` and on this revision, every changed
function classified. Classes:

- **split**: a value that an operation reads sign-sensitively *from the register
  or stack slot the binary computed it in* keeps its own variable, and the store
  prints as `global = vN;` (or `global = <expr>;`) where the binary stores.
- **load**: a load the binary makes of a global after storing to it prints as the
  global, where main printed the stored value.
- **store order**: the same statements, with a store to a global now where the
  binary makes it.
- **equivalent reorder**: two accesses to distinct globals swap with no call or
  pointer store between; not the binary's order.
- **fabricated store gone**: main printed a store the binary never makes (a value
  joined into the global and printed at its definition); this revision prints
  the value.

Earlier revisions of this change also counted a load of the global as a read of
the value: kuna's SSA gives a pointer store no effect on a global, so a load
after a store is the store's `COPY`, and rules had fed the value into it. Those
revisions split such a value and printed the value where the binary reads memory
(`gi = u; *p = k; return gi / 16;` returned `u / 16` for `p == &gi`). Most -O0
"splits" were this: the binary stores and loads right back (useradd O0
`9429`/`9430`, crond O0 `0x41bd`, the rtmon O0 `*_a2n` family), and main was not
wrong there. This revision leaves such loads on the global, so those functions
print exactly as on main.

## 39 disjoint binaries: 41 of 11,656 functions

dash (O0, O2), crond O0, crontab O2, kmod O2, xmlwf (O0, O2), init (O0, O2),
shutdown O2-noinline, minigzip O2, example O0, update-passwd O0, dpkg-divert O2,
rtmon (O0, O2-noinline), sftp-server O2-noinline, setfacl O2, getfacl O0, chage
O2, gpasswd O2-noinline, bzip2 O2, psktool O2, mirai O2, chfn O0, mkbuiltins O2,
e2fsck O2, rsyslogd O2, scp O2, ssh-add O0, chsh O2, cksum O0, csplit O2, chmod
O2, srptool O2, mksyntax O2, faillog O2, comm O2, basename O2.

- split (25): cksum O0 `0x149a0` (`v2 = &v1[3]; program_invocation_short_name =
  v2;`, the binary stores from the stack slot at `14a67`); crontab O2 `0x8ca0`;
  dash O2 `0x6d90` `0x6e90`; dpkg-divert O2 `0x8450` `0xb1a0` `0xd450` `0xddf0`
  `0xdf10`; rtmon O2-noinline `0xb3a0` `0xb550` `0xb700` `0xba90` `0xbe30`
  `0xc040`; sftp-server `0x24dd0`; chage O2 `0x4cc0` and `0x7210` (the yacc
  actions: `v22 = *v26;` then the computed `dat_15180 = ...`; main printed a
  first store `dat_15180 = *v26;` the binary never makes, `79de`..`7a25`);
  mirai O2 `0x7cd0` (`v5 = fork(); dat_16440 = v5; if (v5 <= 0 ...`) and
  `0x10480` (the PRNG keeps its four globals in registers across the loop and
  stores them after it, `10561`..`10575`, which is where they now print; main
  printed a store and a load of each on every iteration); gpasswd `0x9fd0`
  (`v4 = (int)v6 + 100; dat_17348 = v4; v2 = v4 * 8L;`); rsyslogd O2 `0x40b60` (`v10 = dat_b9e28 + 1;
  dat_b9e28 = v10; if (v10 <= 999)`); scp O2 `0x5150` `0x30430`; init O2 `0x87b0`.
- split with store order now the binary's (5): dpkg-divert O2 `0xdcf0`; init O2
  `0x7e50` (`d101` before `d100`); gpasswd `0x8150` (`14141` at `8234` before
  the `lea` of `14140`); e2fsck O2 `0x3c280` (`b1398` stored at `3cb77` before
  the pointer advance); scp O2 `0x8520` prints `dat_442d8 = v14;` one register
  copy before `dat_442d0 = v15;` (binary `902c` then `9036`, nothing between: an
  equivalent reorder).
- store order and load (1): dash O2 `0xe1c0` (`dat_1f038 = v3;` right after
  `malloc` as at `e20b`, and the reload at `e244` prints as `dat_1f038 -= v2;`).
- store order (3): dash O2 `0x7dd0` (`1f414` before `1f40c`); dash O2 `0xe290`
  (`1f038` before the `1f3e0` decrement);
  rsyslogd O2 `0x29fc0` (`b99d8` before `b99d0`, `2a0e5`/`2a0ec`; `b99f0` before
  the pointer store on the `ferror` path; `b99dc` ahead of two loads of distinct
  globals).
- equivalent reorder (2): rsyslogd O2 `0x28f20` (two adjacent stores of
  registers to `b99d0`/`b99d8`); gpasswd `0xcb90` (`173d8` printed before
  `173d0`; the binary stores `cbe0` then `cbe7`).
- fabricated store gone, load (1): scp O2 `0x5fe0`: main printed `dat_44010 =
  v1;`, which the binary never stores (it only loads `44010` at `600a`/`607e`);
  this revision prints `if (2 <= dat_44010) { v1 = dat_44010; kill(dat_44010,
  a0); ...`.
- equivalent extra store (1): scp O2 `0xb730` prints `dat_442fc = 0;` in the
  `else` arm, which the binary never stores (`bde3: xor %edx,%edx`). Upstream's
  forced marker merge joins that register phi into the global now that `v3`
  keeps its own variable; the binary's one store, `b83b`, overwrites it on every
  path with no call or pointer store between, and the one read in between sees
  the same 0.
- equivalent re-coalescing (1): crontab O2 `0x3bc0` (`v1 = optind` holds on the
  path that dropped `v3 = optind`).
- comment anchor only (1): e2fsck O2 `0x2b7e0` (`// branch-flip` moves from the
  store to the `if` it annotates).
- renumber (1): basename O2 `0x2580`, local variable numbering only.

The previous revision (`5baee57bd`, rebased) against this one: 17 functions. 13
are -O0 load splits that now print as on main: crond `0x41bd` `0x8499`
`0xd6a9`, dash `0x6e8c`, init `0x90ae`, rtmon `0xc3bb` `0xc66c` `0xc91d` `0xd009`
`0xd505` `0xd8f5`, ssh-add `0xb180` `0x5d698`. The other four are dash O2
`0xe1c0` and `0xfcb0`, scp O2 `0x5fe0`, and rsyslogd O2 `0x29fc0`, where the
previous revision printed `if (1 <= v6)` and `v15[v6 + -1L]` for `yyleng` while
the binary reloads it (`movslq (%rax),%rax` at `2a2ab`, after the byte store
`v26[-1] = 0`, which may alias it); this revision prints `yyleng` there again.
`0xfcb0` prints as on main: a `PIECE` that joins a stored half into the whole of
an 8-byte global is not treated as a load.

## 21 further binaries: 56 of 5,683 functions

bootlogd, killall5 and last O2; groupmems O2, useradd O0, newusers
O2-noinline; cronnext O2, crond O2-noinline; dpkg-query O2; usart_irq_console
(ARM) and cf2 (ARM Cortex-M) O2; mydoom, x0r-usb and minipig (i386 PE) O2;
minigzip64 (O0, O2); bzip2recover O2; chacl O2; ocsptool O0; expr O2-noinline;
stty O2.

- 53 print what the previous revision printed, and were classified in its
  review: in the class, several with the binary's store positions (cf2
  `0x8018f68` `0x800aba4`, crond O2-noinline `0xad20`); five equivalent reorders
  of distinct globals (crond O2-noinline `0x4960`, newusers `0x11060`, cf2
  `0x800a424` `0x800cd80` `0x801372c`); and cf2 `0x8040c2c`..`0x8045a10`, string
  data decoded as code.
- load (3): ocsptool O0 `0xb0a2` (`dat_202e8 = a0; dat_202e0 =
  realloc(dat_202e0,dat_202e8);`: one store at `b132` after the `cmov`, the
  reload at `b139` prints as the global; main printed three stores); cf2
  `0x80423ec` and `0x80440cc` (string data decoded as code, loads after pointer
  stores).
- now as on main (5): cf2 `0x803c764` (FreeRTOS `vTaskSwitchContext`: the
  previous revision printed `dat_20014454 = v1;` where the binary reloads
  `0x20014450` after `str r1,[r4,#84]`; this revision prints `dat_20014454 =
  dat_20014450;` again), useradd O0 `0x919b` `0x19900`, usart_irq_console
  `0x80005c8`, killall5 `0x3680`.

## 45 coreutils/diffutils/findutils/grep/gzip/tar binaries (castbench)

Main `0729b8b05` to this revision: 153 of 20,230 functions. The previous
revision changed 185; 32 of those print as on main again, all functions whose
binary loads the global back after storing it (sdiff O2 stores `fork()`'s pid at
`40d7` and reloads it at `40dd` and `40eb`):
O0 fmt `0x2bde` `0x312d`, sort `0x5ba8`, cmp `0x2d6d`, diff `0x11b50` `0xfa13`,
sdiff `0x4573`, gzip `0x4677` `0x5b3e` `0x5f11` `0xb868` `0xc45d` `0xd67f`
`0xe055` `0xe29d` `0xe655`, tar `0x106d8` `0x10c82` `0x17069` `0x182fd`
`0x2a571` `0x3321b` `0x3399b` `0x36c56` `0x3b734` `0xb2cf` `0xbdb0` `0xd386`
`0xd818`; O2 sdiff `0x3b00`, O2-noinline sdiff `0x3b60` and gzip `0xb070`.

Of the 153, 141 print what the previous revision printed and were classified in
its review (split; store order the binary's; equivalent reorders of distinct
globals with no call or pointer store between; a redundant copy of an unchanged
value; `struct_N` renumbering in O2-noinline tar). The other 12 differ from the
previous revision only where the binary loads the global back, which now prints
as the global: O0 ls `0x70af`, grep `0xa7d1` (`sysconf` result reloaded from
`33818`), gzip `0x4a9e` `0xd846` (`dat_19098 &= 0x1f;` then the reloads); O2
gzip `0x4bc0` and O2-noinline `0x4aa0` (`if (!dat_1909c)`), O2 gzip `0xadd0`,
tar `0x203b0` (`if ((int8)dat_82b08 < 0)`); and four where a store printed on
both paths of a branch is printed once before it, moving the printed store count
toward the binary's (diff O2 `0xbbc0` `26498`: 2 to 1, the binary's 1; gzip O2
`0xbe90` `1a00c` and O2-noinline `0xc040` `1b00c`: 4 to 3, main's 3, the
binary's 2; gzip O2 `0x9a50` `dca10`: 5 to 4, main's 3, the binary's 2, the
extra one storing an unchanged value).

Casts on the 4,815 shared functions: 32,010 on main to 31,961, 0.845x IDA, 26.8
per 100 statements (O0 0.814x, O2 0.901x, O2-noinline 0.815x). 26 functions lose
54 casts, mostly the `(unsigned long)`/`(long)` around a re-read global. Five gain
one each: four print the same text on both arms (the counter's vocabulary is per
file), and tar O2 `0x2c570` is a split (`strchr` and `open` results keep their
own variables instead of being read back from `dat_82d78`/`dat_82a34`).

Typesweep (444 slices, 10,748 functions, main `0729b8b05` against this
revision): 10,748 same, 0 improved, 0 worse, 1,674 perfect on both arms, mean
0.3753; the exported variable count is unchanged in every function.

## On main `5eb973320` (after #798, which keeps a pointer value apart)

The same 39 binaries plus tar O0, O2 and O2-noinline: 92 of 15,936 functions.

- 39 binaries, 40 functions: 34 print the change listed above line for line,
  five differ only in local numbering or in main's own new casts (crontab
  `0x3bc0`, chage `0x4cc0` `0x7210`, and the two copies of `getopt_internal`,
  sftp-server `0x24dd0` and scp `0x30430`, where main now keeps the
  `nextchar` pointer apart itself), and one is new: dash O2 `0xf440` stores
  `dat_21a50` before `dat_21a58`, as the binary does at `f4f8`/`f4ff` (store
  order). The equivalent reorders gpasswd `0xcb90` and rsyslogd `0x28f20` no
  longer change.
- tar, 52 functions: all in the castbench set above with the same change. tar
  O2 `0xa9f0` `0x27340` and O2-noinline `0x2e690` no longer change: main's
  pointer refusal already prints them that way.

## On main `363123e96`: a store after which the global's earlier value is used

A store after which an earlier value of the global is still used is now left to
upstream (chapter 03), because keeping it made `Merge` copy that earlier value
out where it is defined, above a pointer store or call the binary loads the
global after (riot O2 `0x8000b4c` printed `v1 = dat_20000200;` above
`disableIRQinterrupts();`, while the binary loads at `8000b5a`, after it).
Against the previous revision:

- 12 binaries (libedit, libselinux, libbsd O0, ip, ssh-keyscan,
  dpkg-statoverride O0, freertos, nuttx, riot, dexter PE, mirai, chpasswd):
  1 of 5,642 functions changes, riot `0x8000b4c`, which prints main's text
  again. Against main, 29 of 5,642 change: in-class splits with the store where
  the binary makes it (dpkg `0x71ad`, ip `0x72570`, freertos `0x2684`, nuttx
  `0x8001da4`), local renumbering, equivalent reorders of stores to distinct
  globals, and freertos `0x22d4`, which stores `dat_200004e4` at the binary's
  `230c` and, on the path that falls through, again after the `if`/`else` with
  nothing in between (equivalent).
- The 42 binaries above: 4 of 15,936 change. tar O2-noinline `0xdfd0` and
  `0x30600` and rsyslogd O2 `0x29fc0` print main's text again. tar O2
  `0x308a0` now stores `dat_82c70` after `dat_82c78`, as the binary does
  (`30af1`, `30afc`), and loads the earlier `dat_82c70` just before that store
  as main does (the binary loads it at `30aa2`; only the `dat_82c78` store lies
  between); the previous revision stored `dat_82c70` first. Against main,
  89 of 15,936 change, all among the functions classified above.
