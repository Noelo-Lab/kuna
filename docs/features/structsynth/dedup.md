# structsynth — the layout ledger (subsumption dedup)

The second half of `structsynth`. The first version compared minted layouts for
*equality*, so two functions that read overlapping but unequal subsets of one
record got a name each. This is how the ledger decides instead, and what the
decision measures. `structsynth` is on by default since `6e4f6fa5` (#682), so
everything here is default output. Unless a section says otherwise, the tables
compare this branch with main at `75f832e3`, both with the option at its
default. The rules were developed against earlier mains (`e1139df9`,
`f1ec42a7`, `6e4f6fa5`); the arms that only exist for a rule that was later
changed keep the main they were measured on, and say so.

## The rule

A measured layout `L` is answered with a minted structure `S` when `S` is at
least as large and every field `L` claims exists in `S` at the same offset, at
the same width, with the same type. Filler is not a claim. Among the structures
that contain `L`, the smallest is reused.

`S` saying strictly more than `L` measured must pass four more tests:

- `S` claims at most **twice as many fields** as `L` and is at most **four
  times its size**.
- `L` claims at least **two fields one of which is a typed pointer, or three
  without one**. A typed pointer is one whose pointee is a type: `char *`,
  `long *`, `struct_N *`. `code *`, `void *` and `undefinedN *` are not.
- `L` is not a **table**: a layout whose every claim past offset 0 is a pointer
  without a pointee is answered only by its own shape.
- If `L` has no typed pointer, `S` is **the same size** as `L` and claims
  nothing past `L`'s last field. It can fill in the holes between the fields `L`
  measured, but not the bytes after them.
- `S` claims nothing inside the **alignment padding** `L`'s claims leave between
  two of them: from the end of one claim up to the next claim or the next
  multiple of that claim's width, whichever comes first.

And whatever `S` says, it answers only if it lays **exactly `L`'s own members**
(offsets, widths, types and filler) over every byte range `L` read or wrote
without claiming a field.

Two layouts of exactly the same shape always share a name, whatever those tests
say. [Why](#the-bounds-and-the-evidence-rules) is the measurement below.

When containment runs the other way (the new layout strictly contains a minted
one), the minted one cannot be widened. `find_add` refuses to redefine a held
name, and the completed structure is already shared by every function that took
a pointer to it. So the larger layout mints and the smaller is **superseded**:
still defined, and handed out again only to a layout no live structure answers
for. Supersession is derived from the minted set on every lookup, for the
entries that answer, and never stored.

`decompile-all` decompiles in address order, so the smaller layout at the lower
address mints first and its function's text still names the superseded structure.
`converge_synthesized_structs` handles that. After the eager batch it asks the
ledger which names were superseded and decompiles again exactly the results that
spell one. The lookup now answers each with the survivor wherever the survivor is
in reach. A redo that fails where the first pass succeeded, for example by
running out of the `--mode fast` watchdog budget, keeps the first body.

This is one sweep, not a fixpoint, and it does not always reach one name per
record, because the growth bounds are not transitive. Suppose `fb` claims four
fields, `fa` two of them and `fc` eight that include `fb`'s four. `fa` reuses
`fb`'s structure, `fc` supersedes it, and `fc` is out of `fa`'s reach, so `fa`
keeps the superseded structure. Apart from the unclaimed-bytes veto, the lookup
never mints a name that an existing structure already answers for, since that
name would be superseded at birth.
The sweep therefore mints nothing as long as a re-decided function measures
what it measured the first time. This shape is pinned by
`tests/cli/structsynth-sweep-mints-no-third-name.json`, which runs over the
fixture `structsynthchain_x86_64`.

## Bytes a reader touched without claiming them

The layout prune turns an access the header cannot spell at its own offset (an
unaligned word, a 16-byte copy, a 3-byte width) into filler, and the body then
spells that access through whichever member holds its first byte. Whether a
load may print after a store is decided by `ActionMarkImplied`, which compares
the two addresses (`isPossibleAlias`). It treats one base plus two different
constants as two different objects and never asks how wide either access is.
So how an unclaimed access is spelled decides whether it can be misordered.

A reviewer built the case, gcc -O2, stripped:

```c
struct T { int i0; char c4, c5, c6, c7, c8, c9, c10, c11; long pad; long l10; char *s; long l20; };
void full(struct T *p, unsigned v, char *s)      /* writes c7..c10, i0, l10, s, l20 */
unsigned long rd_c8(struct T *p, char *s, unsigned w) {
  unsigned v; memcpy(&v, &p->c7, 4); p->c8 = w >> 3;
  p->i0 = 5; p->l10 = 0; p->s = s; p->l20 = v; return v * 3u;
}
```

`rd_c8`'s own layout is `{0: uint, 8: char, 0x18, 0x20, 0x28}` with filler at
4..8, so the load prints as `*(uint4 *)&a0->field_0x4[3]`. The test cannot
separate that from `a0->field_0x8`, and the load keeps its own statement first.
`full`'s layout has the same size, contains every field `rd_c8` claims, and has
a `char` at 7. Without the rule it answered `rd_c8`, the load became
`*(uint4 *)&a0->field_0x7`, and the printed body read the word after overwriting
its second byte:

```
  a0->field_0x8 = (char)(a2 >> 3);
  ...
  a0->field_0x28 = (uint8)*(uint4 *)&a0->field_0x7;
  return *(uint4 *)&a0->field_0x7 * 3;
```

Compiled against the original, it returned `0xd5c06096` where the original
returns `0xd5c0ab96`. With a store at 9 or 10 the result is the same. A store at 7
is safe, because the two addresses are then identical.

The rule makes each unclaimed access print exactly as it does under the
reader's own layout: a structure answers only if the members it lays over those
bytes are the reader's own. Members elsewhere, and the structure's size, still
follow the rules above. A reader turned away mints its own shape even though a
live structure contains it, so that entry is superseded from the start and the
convergence sweep decides the reader again, to the same answer.
`tests/stages/structsynth-unclaimed-bytes.xml` pins the three readers (0 of 3
on the previous revision, 3 of 3 here and on main) and two ledger unit tests pin
the rule.

The address test itself is unchanged. It misorders the same reader with the
option off, on raw offsets (`*(uint4 *)((int8)a0 + 7)` after
`*(char *)&a0[2] = ...`), so that is an engine defect in its own right and is
not addressed here.

Measured over all 101 builds of the five sets below, on `f1ec42a7` with and
without the rule, it changes exactly one parameter: `rsyslogd` O2 `timeConvertToUTC`'s `struct syslogTime *`, a 4-byte
store at 7 that was answered with the same-size layout declaring byte fields at
7..10. It returns to its own 10-claim shape, losing 4 fields that were real
(held-out 3 precision 0.8409 → 0.8407 on that main). Its body is main's again.
The previous revision split that store into four byte stores, which wrote the
same bytes but was the one non-rename hunk in the whole-corpus sweep.

Since #682 a field read both as a float and as an integer is raw bytes
(`undefined1 field_0x<hex>[N]`) in the reader's own layout, which is filler to
the ledger. A container with a typed field there would print the float read as a
value conversion again, the defect #682 fixed, so those bytes count as unclaimed
too (`Evidence::unclaimed_ranges`).

## Records that differ only in a reader's padding

A reviewer measured the previous revision on 42 builds outside every earlier set
and found 10 of 43 absorptions given a different record. All ten were a
same-size container with a member where the reader's own layout has padding:

- `bash` O0 `execute_if_command` and `copy_if_command` (`if_com`:
  `{int flags; COMMAND *test, *true_case, *false_case}`) measure
  `{0: uint, 8, 0x10, 0x18}` and were answered with `for_com`'s layout, which
  has `int line` at 4. `copy_while_command` (`while_com`) the same.
- Six `it_init_*` readers of `ITEMLIST` measure `{0: uint, 0x10: long *}` and
  were answered with `case_com`'s `{0: uint, 4: uint, 8, 0x10: long *}`.
- dpkg O2-noinline `pkg_queue_init` (`pkg_queue`, 24 bytes, `{0, 8, 0x10: uint}`)
  was answered with `pkg_spec`'s layout, which has two flag bytes at 0x14 and
  0x15, in `pkg_queue`'s tail padding.

Records part ways in their first words as much as after their last one: every
`bash` command record begins with `int flags`, and whether offset 4 is `int line`
or padding is exactly what tells `if_com` from `for_com`. So a container may not
claim bytes the reader's claims leave as alignment padding, and a reader with no
typed pointer may not be given anything past its last field. A hole wider than
its padding is still room for a member the reader skipped (`{0: uint, 0x10: ptr}`
may still take a word at 8), and a typed reader may still be given members in its
tail padding: `fmt`'s `put_word` measures `{0: char *, 8: int}` of a `WORD` whose
`int space` sits at 0xc. `tests/stages/structsynth-shared-layout.xml` asserts
10 to 12 pin the three shapes (all three fail on the previous revision).

On the reviewer's builds and on 34 more chosen before any result under this rule
was seen, main (`75f832e3`) → previous revision → this revision:

| | main | previous revision | shipped |
|---|---:|---:|---:|
| reviewer, 24 builds: precision | 3933/4292 = 0.9164 | 3965/4324 = 0.9170 | 3960/4319 = 0.9169 |
| reviewer, 24 builds: absorbed / different record | | 15 / 0 | 11 / 0 |
| reviewer, 18 builds incl. `bash` O0, dpkg O2-noinline: precision | 4091/4527 = 0.9037 | 4129/4576 = 0.9023 | 4115/4551 = 0.9042 |
| reviewer, 18 builds: absorbed / different record | | 28 / 10 | 13 / 0 |
| fresh, 34 builds: precision | 4000/4517 = 0.8855 | 4094/4626 = 0.8850 | 4081/4610 = 0.8852 |
| fresh, 34 builds: absorbed / different record | | 67 / 4 | 62 / 3 |

The fresh set is `bash` O2-noinline, `man2html` O0, `mkbuiltins` O2,
`dpkg-divert` O0, `dpkg-query` O2-noinline, `dpkg-trigger` O0,
`dpkg-statoverride` O0, `rsyslogd` O2-noinline, `e2fsck` O2-noinline, `kmod`
O2-noinline, `dash` O2-noinline, `ssh-keysign` O2, `ssh-add` O0, `sftp`
O2-noinline, `gnutls-serv` O0, `ocsptool` O0, `rtmon` O0, `grep` O2, `gzip` O0,
`diff` O2, `diff3` O0, zlib `minigzip` O0, `setfacl` O0, `shutdown` O0,
`killall5` O0, `crontab` O2-noinline, `libbsd` O2-noinline, `bzip2` O0,
`ginstall` O2, `stty` O0, `cksum` O2-noinline, `useradd` O0, `usermod`
O2-noinline and `newusers` O2. The rule removed one of its four wrong records
(`useradd` O0 `new_grent`, a `group` reader whose padding at 0x14 a larger record
fills). The three left are all `e2fsck` O2-noinline and are untyped readers
filled in *between* their fields, which the padding rule does not reach:
`handle_htree` and `ext2fs_is_fast_symlink` measure `{0: ushort, 4: uint,
0x6c: uint}` of an `ext2_inode` and take the layout an `ext2_inode_large` reader
measured, which is the same record for its first 128 bytes, so the two added
fields are real; `ext2fs_new_dir_block` measures `{0, 0x20, 0x28}` of a
`struct_ext2_filsys` and takes a `utf8cursor` layout, 2 of 3 added fields real.
On the fresh set the rule is 0.0003 behind main (81 more true fields, 93 more
claimed); `e2fsck` O2-noinline alone adds 50 true fields of 58.

`bash` O2-noinline scores nothing on either build: its `decompile-project`
export panics in `kuna-sleigh` (`kuna_sharedbytes.rs`) on main as well, so
there is no header to read.

## Address arithmetic past a reader's end

A container larger than the reader's own object also changes how the reader's
addresses past its measured end are spelled. A reviewer built the case, gcc -O2,
stripped:

```c
struct Small { char *name; long len; };
struct Big { char *name; long len; int a; int pad; long c; };
long wbig(struct Big *p, long v);   /* writes name, len, a, c */
struct Small *rsmall(struct Small *p) { long l = p->len; if (*p->name) l += 2; p->len = l; return p + 1; }
```

`rsmall` measures `{0: char *, 8: long}`, 16 bytes, and `wbig`'s 32-byte layout
contains it. `p + 1` is 16 bytes on, which in the container is its `int` at
0x10:

```
main:    struct_0 * sub_12b0(struct_0 *a0)     ...  v1 = &a0[1];
branch:  unsigned int * sub_12b0(struct_0 *a0) ...  v1 = &a0->field_0x10;
```

The address and the value are the same; the return type is now a pointer to the
container's field instead of to the record. A call argument is re-spelled the
same way (`rsmall_arg`'s `&a0[1]` becomes `&a0->field_0x10`). This is inherent
to answering a reader with a larger record: the ledger sees the fields a reader
dereferences, not the arithmetic it does on its base. No function in the 177
builds changes this way (the only non-rename change is `ip` O0
`ll_remember_index`, below).

## Why agreement on a shared field is exact

The obvious relaxation is to let `undefined<N>` be a wildcard any `N`-byte type
may fill in — "the more informative type wins". It was implemented, measured and
rejected: it unifies records that only share a shape.

`du` O2 `sub_c090` writes two double bit patterns and a zero byte:

```
void sub_c090(struct_19 *a0) {
  a0->field_0x10 = 0;
  a0->field_0x0 = 0x3f80000000000000;
  a0->field_0x8 = 0x3fb4fdf43f4ccccd;
}
```

Its three fields are `{undefined8, undefined8, undefined1}`. Under the wildcard
rule that layout is "contained by" an unrelated record whose offset 0 is a
pointer and whose offset 8 is a `long`, and the body becomes
`a0->field_0x0 = (void *)0x3f80000000000000;`.

Both arms were measured before any bound existed, so the name counts are the
unbounded ones; the comparison is between the two *agreement* rules, holding
everything else fixed.

| variant (no bounds) | names over the 12-binary census | typesweep aggregate `type_match` | non-rename hunks |
|---|---:|---:|---:|
| main (exact-signature dedup) | 514 | 3035.55 | — |
| wildcard `undefined<N>` | 356 | 3034.13 | 23 |
| **exact agreement (kept)** | **388** | **3035.55** | **2** |

## The bounds and the evidence rules

Sharing one layout across functions gives every parameter it absorbs the
container's whole field list, so an absorbed parameter is declared with fields
its own evidence never saw. `dedup_heldout.py` measures that against DWARF with
this option's own instrument, `layoutscore.py`, which counts a claimed field as
true when the real record has a field at that offset with that size. It also
checks the record, which field precision cannot see (below).

Five sets of builds. The rules were drawn from the first four, so only the last
is a clean test of the final rule:

- **tuning**: `fmt`, `ls`, `sort`, `du` at O0 and O2. The growth bounds were
  chosen here.
- **held-out 1**: 17 builds picked independently: `cp`, `pr`, `od`, `tail`,
  `df`, `csplit` at O0 and O2, `kmod` O0/O2, `dash` O0/O2 and `gzip` O2. The
  two-integer-word shape was found here.
- **held-out 2**: 22 builds measured after the pointer rule was fixed: `mv`, `rm`,
  `stat`, `join`, `wc`, `split` at O0 and O2, `crond` O0/O2, `xmlwf` O0/O2, and
  `init`, `dpkg-divert`, `cmp`, `sdiff`, `setfacl`, `chage` at O2.
- **held-out 3**: 28 builds a reviewer picked, mostly outside coreutils
  (`rsyslogd`, `ip`, `dpkg`, `dpkg-query`, `e2fsck`, `sftp`, `ssh-keyscan` and
  others). The table rule and the integer rule were drawn from this set.
- **held-out 4**: 26 builds chosen before any result under those two rules was
  seen: `bash`, `certtool`, `gnutls-cli`, `ocsptool`, `passwd`, `groupadd`,
  `login`, `bootlogd`, `killall5`, `shutdown`, `ssh-agent`, `scp`, `ssh-add`,
  `dpkg-statoverride`, `dpkg-trigger`, zlib `example`, `cronnext`,
  `update-passwd`, `touch`, `expr`, `seq` at O2 and `grep`, `diff`, `ip`,
  `rsyslogd`, `e2fsck` at O0. `bash` O2's project export writes no header on
  either build, so it scores nothing.

Claimed-field precision (filler excluded), with recall in parentheses:

| dedup rule | tuning | held-out 1 | held-out 2 | held-out 3 | held-out 4 |
|---|---:|---:|---:|---:|---:|
| exact signature (main `75f832e3`; `e1139df9`, `f1ec42a7` and `6e4f6fa5` score the same) | 704/787 = 0.8945 (0.0746) | 1011/1081 = 0.9352 (0.0659) | 804/888 = 0.9054 (0.0482) | 2746/3271 = 0.8395 (0.0307) | 3455/3937 = 0.8776 (0.0233) |
| bounds + pointer rule + table rule (on `e1139df9`) | | | | 2818/3368 = 0.8367 (0.0315) | 3592/4123 = 0.8712 (0.0243) |
| + integer rule (on `f1ec42a7`) | 748/833 = 0.8980 (0.0792) | 1060/1134 = 0.9347 (0.0691) | 836/920 = 0.9087 (0.0501) | 2795/3324 = 0.8409 (0.0313) | 3555/4049 = 0.8780 (0.0240) |
| + unclaimed-bytes rule (on `f1ec42a7`) | 748/833 = 0.8980 (0.0792) | 1060/1134 = 0.9347 (0.0691) | 836/920 = 0.9087 (0.0501) | 2791/3320 = 0.8407 (0.0312) | 3555/4049 = 0.8780 (0.0240) |
| previous revision, on `6e4f6fa5` | 748/831 = 0.9001 (0.0792) | 1064/1138 = 0.9350 (0.0694) | 836/920 = 0.9087 (0.0501) | 2794/3321 = 0.8413 (0.0313) | 3549/4042 = 0.8780 (0.0240) |
| **+ padding rule (shipped), on `75f832e3`** | **748/831 = 0.9001 (0.0792)** | **1064/1138 = 0.9350 (0.0694)** | **836/920 = 0.9087 (0.0501)** | **2786/3312 = 0.8412 (0.0312)** | **3545/4037 = 0.8781 (0.0239)** |

The unclaimed-bytes row and the previous revision differ because #682 changed
the types the pass gives some fields (a sign-contested integer is
`undefined<N>`, a float read as an integer is raw bytes), and agreement on a
shared field is exact.

Pooled over the seven held-out sets (these four, the reviewer's 24 and 18
builds and the fresh 34 above; 169 builds), precision is 20040/22513 = 0.8902 on
main and 20387/22887 = 0.8908 shipped, with 347 more true fields. Recall is
higher on every set. The gain is small, and it is not uniform. Of the 177 builds,
53 gain precision and 6 lose it. The losses are `kmod` O2 (0.8507 → 0.8417),
`kmod` O2-noinline (0.8876 → 0.8791), `kmod` O0 (0.9246 → 0.9163), `grep` O2
(0.9212 → 0.9128), `ip` O0 (0.6699 → 0.6675) and `e2fsck` O0 (0.8931 → 0.8926).
The largest gains are `diff` O0 (0.8563 → 0.8629), `sort` O0 (0.8571 → 0.8636),
`sort` O2 (0.8750 → 0.8814) and `du` O0 (0.9206 → 0.9265).

The previous revision had neither the table rule nor the integer rule. A reviewer
measured it on held-out 3 against a `d3617d64` main: 0.8395 → 0.8294, with 7 of
the 46 checkable absorptions given a different record.

### The record each absorbed parameter gets

Field precision cannot see a parameter given the wrong *record* whose fields
happen to line up. `stat` and `cycle_check_state` agree field for field at
`{0, 8, 0x10, 0x18}`, so `layoutscore.py` scores that false unification as
correct. `dedup_heldout.py absorb` checks the record instead. For each
parameter answered by a strictly larger structure, it asks whether that
structure is one some parameter of the *same* DWARF record measured on main.

| shipped rule, on `75f832e3` | absorbed | same record | different record | uncheckable | added fields true |
|---|---:|---:|---:|---:|---:|
| tuning | 28 | 28 | 0 | 0 | 44/44 |
| held-out 1 | 35 | 35 | 0 | 0 | 53/57 |
| held-out 2 | 20 | 20 | 0 | 0 | 32/32 |
| held-out 3 | 27 | 25 | 0 | 2 | 40/41 |
| held-out 4 | 62 | 60 | 1 | 1 | 90/100 |
| reviewer, 24 builds | 11 | 11 | 0 | 0 | 27/27 |
| reviewer, 18 builds | 13 | 13 | 0 | 0 | 24/24 |
| fresh, 34 builds | 62 | 57 | 3 | 2 | 81/93 |
| **all 177 builds** | **258** | **249** | **4** | **5** | **391/418** |
| previous revision (no padding rule) on the first five, `6e4f6fa5` | 178 | 174 | 1 | 3 | |
| previous revision on the last three, `75f832e3` | 110 | 94 | 14 | 2 | |
| without the integer rule, held-out 3 / 4 (on `e1139df9`) | 57 / 108 | 39 / 80 | 0 / 8 | 18 / 20 | 72/97, 137/186 |

Four wrong records remain, all `e2fsck`, all untyped readers filled in between
their own fields by a same-size layout: O0 `ea_refcount_intr_next` (three
`uint64` words of a 32-byte `ea_refcount`, answered with a `dentry_info_args`
layout) and the three O2-noinline readers above. Two of those (`ext2_inode`
answered with `ext2_inode_large`) are one record's common initial sequence, and
their added fields are real. On `f1ec42a7` there were four more: `du` O2
`add_exclude` (`struct exclude *` answered with an `mbchar` layout) and three
`rsyslogd` O0 `hashtable` readers (answered with a `lookup_s` table). Under
#682's field types those containers no longer contain them.

Each rule answers one shape:

- **Two claims need a typed pointer, else three.** Two integer words are how
  distinct records begin: `stat` (`{dev, ino}`), `cycle_check_state`,
  `Src_to_dest` (`{ino, dev}`), a list node, a `timespec`, a hash entry. Without
  it `cp` O2 `cycle_check` declared its `struct stat *` as the state record.
- **A table is answered by its own shape.** `rsyslogd` O2's seven `*_if_s`
  interface records, 80 to 136 bytes, are a version word and then function
  pointers. `code *` names no record, so with the pointer rule alone all seven
  took the 144-byte layout of `statsobj_if_s`, and 1 of the 31 fields that added
  was real. `strgenQueryInterface` (`0x414e0`) and `statsobjQueryInterface`
  (`0x551d0`) now get two names, as on main.
- **No typed pointer, same size.** Records that begin alike part ways after the
  last field a reader touched. Every netlink reader in `ip` measures the same
  `nlmsghdr` words ahead of a payload of its own; on held-out 3 and 4 those 13
  absorptions added 34 fields, none of them real. dpkg's 24-byte `pkg_queue` was
  answered with a 40-byte command layout, 0 of 3 added fields real. The rule
  costs some right answers too: openssh's `sshbuf` readers measure 64 of its
  72 bytes and now keep their own name.
- **Nothing in the reader's padding.** See [above](#records-that-differ-only-in-a-readers-padding).
  Over all eight sets it removes 30 of the previous revision's absorptions and
  adds none: 11 were a different record and 19 the same one. On the first five
  sets it removes only same-record absorptions (6: 4 into interior padding, 2
  into an untyped reader's tail), which is why held-out 3 and 4 barely move
  (0.8413 → 0.8412 and 0.8780 → 0.8781).

## Distinct `struct_N` names per binary

`kuna decompile-all <bin> --option structsynth param --option structdefs on --json`,
counting the distinct synthesized names the run emits and the
`(function, structure)` pairs behind them.

| binary | main | branch | pairs main → branch |
|---|---:|---:|---:|
| fmt O0 | 4 | 4 | 4 → 4 |
| fmt O2 | 3 | 3 | 3 → 3 |
| ls O0 | 27 | 24 | 33 → 33 |
| ls O2 | 35 | 27 | 71 → 71 |
| sort O0 | 30 | 27 | 34 → 34 |
| sort O2 | 26 | 23 | 28 → 28 |
| du O0 | 31 | 27 | 34 → 34 |
| du O2 | 32 | 29 | 35 → 35 |
| find O0 | 55 | 50 | 68 → 68 |
| find O2 | 53 | 50 | 62 → 62 |
| tar O0 | 126 | 121 | 155 → 155 |
| tar O2 | 93 | 86 | 110 → 110 |
| **total** | **515** | **471** | **637 → 637** |

The same functions are retyped, and they name 8.5% fewer records. No pair is
lost. (On `6e4f6fa5` the same count was 516 → 472, and on `f1ec42a7` 514 → 467.)

## `fprate.py`, the type metric and the variable rows are blind to all of this

`fprate.py` only asks which parameters are retyped and what DWARF says they are.
The ledger changes neither; it only changes which record a retyped parameter is
given. The same holds for decbench's `type_match`, which compares pointee
spellings by name, so `struct_0 *` and `struct_1 *` score alike.

`scripts.decbench.typesweep --option structsynth param`, 444 slices
(coreutils grep gzip diffutils bzip2 findutils tar shadow x × O0/O2/O2-noinline),
`KUNA_BIN` pinned per build. With the option on by default both of the sweep's
arms are the default output, so the comparison is between the two builds:

| | main `75f832e3` | branch |
|---|---:|---:|
| `type_match` PERFECT | 986 | 986 |
| aggregate `type_match` | 3111.18 | 3111.18 |

Every slice's per-function scores, match details and variable counts (`values`,
`meta`, `nvars` in `rows.json`) are identical between the builds. The only
difference is the `vars_sig` hash of 74 slices, which covers the `struct_N`
names. That is expected by construction and is not evidence either way.

No variable or argument appears or disappears. Over all 177 builds above
(92,680 functions, 623,992 exported variable rows), no function's variable count
or argument count changes, and no row changes in anything but the `struct_N` it
names (2,119 functions have such a rename). One function's C changes beyond a
name: `ip` O0 `ll_remember_index` passes `&a0->field_0x6[10]` to a callee as
`&a0->field_0x8[8]`, the same address spelled through a same-size container's
filler. It is a call argument, not a dereference.

## Whole-corpus `decompile-all` before/after

Nine binaries, main build vs branch build, both arms, every changed line
classified (`docs/features/structsynth/dedup_hunks.py`).

| binary | default (`structsynth param`) | `--option structsynth off` |
|---|---|---|
| fmt O2 | identical | identical |
| ls O2 | 55 rename | identical |
| sort O2 | 16 rename | identical |
| du O2 | 19 rename | identical |
| find O2 | 22 rename | identical |
| tar O2 | 61 rename | identical |
| grep O2 | 42 rename | identical |
| diff O2 | 19 rename | identical |
| rsyslogd O2 | 2 rename | identical |

Main is `75f832e3`. The default run and an explicit `--option structsynth param`
give the same classification. `rename` is a line whose only change is which
`struct_N` it names; there is no other kind of hunk. No statement moved, and no declaration
appeared or vanished. The previous revision had one more hunk, in `rsyslogd` O2,
where `timeConvertToUTC`'s `syslogTime` parameter was answered with a same-size
layout that declares byte fields at 7..10, and the 4-byte store
`*(unsigned int *)a1->field_0x7 = 0x2b000006;` became four byte-field stores of
the same bytes. The unclaimed-bytes rule gives that parameter its own layout
back, and the body is main's again.

## `decompile-project` round trip

`kuna decompile-project <bin> --option structsynth param`, then
`gcc -fsyntax-only` on the export and `offsetcheck.py` on its header, on a main
build and the branch build.

| project | `struct_N` definitions in the `.h`, main → branch | `has no member named` |
|---|---:|---:|
| fmt O2 | 3 → 3 | 0 |
| ls O2 | 35 → 27 | 0 |
| sort O2 | 26 → 23 | 0 |
| du O2 | 32 → 29 | 0 |
| find O2 | 53 → 50 | 0 |
| tar O2 | 93 → 86 | 0 |
| grep O2 | 40 → 35 | 0 |
| diff O2 | 33 → 30 | 0 |
| rsyslogd O2 | 91 → 90 | 0 |

`offsetcheck.py`: **0 of 373** synthesized structures have a field the C compiler
moves (main exports 406). Each export's total `gcc` error count is the same as
main's (324 / 892 / 718 / 558 / 1375 / 4121 / 1019 / 1038 / 6483). These are the
export's existing prelude errors, not new ones. The header declares one
definition per name still in use: `build_header` drops a `struct_<digits>`
definition when nothing left in the document names it (no prototype, no body, no
exported variable row and no field of a definition that is kept). The rule is
syntactic, so it is also safe on the streaming export, where the convergence
sweep does not run.

The *order* of definitions in the exported `.h` is not stable from run to run,
and that is already true on main: two runs of the same main build on `ls` O2
give two different header orders. Comparisons here therefore read definitions
by name, never by position.

## Does the program actually converge?

`docs/features/structsynth/dedup_residual.py` re-reads the emitted definitions and
looks for a layout the ledger's own rule would still have merged, which is what
a convergence sweep that stopped too early leaves behind. It applies the shipped
rule, so a contained layout the rule refuses is not counted: keeping its own
name is the decision, not a residue.

| binary | emitted | still mergeable |
|---|---:|---:|
| fmt O0 | 4 | 0 |
| ls O2 | 27 | 0 |
| du O2 | 29 | 0 |
| tar O2 | 86 | 0 |
| cp O2 | 30 | 0 |
| kmod O0 | 51 | 0 |
| rsyslogd O2 | 90 | 0 |

A non-transitive chain like the `fb`/`fa`/`fc` fixture above would show up here
as a superseded structure that is still emitted. None of these binaries has one.
The previous revision's script counted one layout in `rsyslogd` O2,
`timeConvertToUTC`'s, whose container the unclaimed-bytes rule turns away (the
script cannot see the unclaimed store, which is not in the header). The same
container also has members in that reader's padding, which the script now
checks, so it is no longer counted; the emitted definitions are the same 90.

## The streaming export does not converge

`converge_synthesized_structs` is a property of the eager batch: it decompiles a
result again once the whole program has been seen. `decompile-project --stream`
has already written a body to the `.c` by the time a name it spells can be
superseded, so it stops wherever its write order left it. On `cp` O2 with
`--option structsynth param` the two surfaces disagree about how many records
the program has:

| | definitions in the `.h` | distinct `struct_N` in the `.c` |
|---|---:|---:|
| main, eager | 33 | 33 |
| main, `--stream` | 33 | 33 |
| branch, eager | 30 | 30 |
| branch, `--stream` | 31 | 31 |

Each surface is self-consistent: every name a body spells is defined in its
header. That is what the header prune guarantees and what `offsetcheck.py`
checks. The streaming export names one record that the eager one has merged.
On main the two agreed, because nothing merged at all.

`--jobs N` is a different case. Since `structsynth` went on by default, a
sharded run turns it off in its workers and says so on stderr, because per-worker
ledgers would give one name two layouts. A sharded run therefore neither mints nor
converges.

There is no `tests/cli` probe for the stream-vs-eager difference. That schema
runs one command and matches its stdout, and this claim compares the files of two
exports. The property the prune depends on is pinned instead: a unit test in
`kuna-console/src/project.rs` builds a header from records whose `code` is
`None`, which is what the streaming writer leaves behind, and checks that the
definition named by their prototype survives.

## Speed

Interleaved main/branch, min-of-15, `kuna decompile-all` wall clock, with both
binaries frozen and the order alternating from round to round. Main is
`6e4f6fa5`, where `structsynth` is on by default. The machine was shared with
other jobs.

| case | main | branch | delta |
|---|---:|---:|---:|
| `tar` O0, default (densest real ledger, 126 → 121 names) | 19.158 s | 18.671 s | −2.54% |
| `ls` O2, default | 13.383 s | 13.447 s | +0.48% |
| `rsyslogd` O2, default | 25.509 s | 24.709 s | −3.14% |
| `ls` O2, `--option structsynth off` | 12.984 s | 12.926 s | −0.44% |
| synthetic, 1000 distinct layouts, default | 2.349 s | 2.641 s | +12.44% |

Real binaries are inside the +5% budget. The synthetic binary is not. It is a
112 KB program whose 1000 functions each read a different random layout, about
eight times the densest real ledger measured. It was +43% before supersession
was limited to the structures that answer a lookup and a lookup was limited to
the sizes that can answer (from the measured size up to 16× it); neither changes
any output. What is left is one scan of the held names per lookup, plus the
convergence sweep re-deciding the functions that named a superseded structure.
The unclaimed-bytes check runs only for a candidate that already answers and a
reader that has an unclaimed access, and the previous revision measured +11.65%
on the same binary against `e1139df9`.
