# structmerge -- every changed function, classified

`decompile-all` over ten stripped binaries (decbench `full_run_address_2026-09-11`),
`--option structmerge off` against `--option structmerge siblings`, with
`structsynth` at its default `locals`. Classified by
[`hunks.py`](hunks.py), in order: a skeleton delta (a control-flow keyword or a
call token moved), a declaration-count delta, `record name only` (the two
functions are identical once every `struct_N` number and every `vN`/`aN` is
spelled the same way, which is a parameter whose record gained members), `field
accesses only`, and `read` for anything else.

| binary            | functions | changed | record name only | field accesses only | read |
|-------------------|---:|---:|---:|---:|---:|
| coreutils-fmt-O2  | 151 | 0 | 0 | 0 | 0 |
| coreutils-ls-O0   | 589 | 7 | 7 | 0 | 0 |
| coreutils-ls-O2   | 404 | 3 | 3 | 0 | 0 |
| coreutils-sort-O0 | 479 | 0 | 0 | 0 | 0 |
| coreutils-du-O2   | 320 | 22 | 22 | 0 | 0 |
| findutils-find-O2 | 658 | 19 | 19 | 0 | 0 |
| grep-grep-O2      | 449 | 6 | 5 | 1 | 0 |
| tar-tar-O2        | 1125 | 23 | 23 | 0 | 0 |
| dash-dash-O2      | 335 | 0 | 0 | 0 | 0 |
| kmod-kmod-O2      | 394 | 12 | 12 | 0 | 0 |
| **total** | 4904 | 92 | 91 | 1 | 0 |

No skeleton delta, no declaration-count delta and nothing unclassified in these
4,904 functions. 91 of the 92 changed functions are a record rename: the
parameter's `struct_N` number moves and its record gains members, and the body is
otherwise identical.

Those counts are this set of ten binaries, not a general rate: how many bodies
change depends on how much filler the merge fills in, and on a binary with more
synthesized records the respelling class is larger. `e2fsprogs` `e2fsck` -O2
(1,421 functions, outside this table) is the worked example -- 56 functions
change, 54 of them record-name-only, 2 with a respelled body, one of those with a
declaration-count delta:

```
$ kuna decompile-all .../O2/e2fsprogs/stripped/e2fsck --option structmerge off > off.c
$ kuna decompile-all .../O2/e2fsprogs/stripped/e2fsck --option structmerge siblings > on.c
$ python3 docs/features/structmerge/hunks.py e2fsck-O2
| e2fsck-O2 | 1421 | 56 | 54 | 1 | 1 |
all classes: {'record name only': 54, 'declaration-count delta': 1, 'field accesses only': 1}
```

`dict_insert` is the declaration-count one. Its record's `char field_0x8[...]`
filler is replaced by real members, so the accesses are respelled off the member
that now covers them -- every one at the same address:

```
-  *(unsigned long *)&a1->field_0x8[0x18] = a2;        // 0x8 + 0x18
+  *(unsigned long *)&a1->field_0x10[0x10] = a2;       // 0x10 + 0x10
-  *(struct_47 **)&a1->field_0x8[8] = v7;              // 0x8 + 8
+  *(struct_140 **)a1->field_0x10 = v6;                // 0x10, array decay
```

and its declarations go 7 -> 6, because `int8 v2` and `int8 *v3` become one
`int8 *v2` once both hold the same typed member. `ext2fs_file_read` is the other:
`ext2fs_inline_data_get(v1,a0->field_0x10,a0->field_0x14,...)` becomes
`...,&a0->field_0x14,...` -- offset 0x14 is `char field_0x14[4]` filler on the off
arm, which decays to its address, and `unsigned int field_0x14` once the merge
claims it, which needs the `&`. Same address, and a field the merge could only
have added.

The one respelling is `grep` -O2 `sub_7110`, whose record had `undefined1`
filler at offset 0x20 and gains a `long` member there:

```
-  char *v1;
+  long *v1;
-        v1 = a0->field_0x20;
+        v1 = &a0->field_0x20;
```

The same address either way -- an array member's name decays to its address, a
scalar member's needs the `&` -- with the pointer now typed by the member it
points at. Taking a field's address is not a dereference, so it is not an
unclaimed access and the veto has nothing to say about it; this is the
documented consequence of the layout having no holes.

## The eleventh binary: a reader can come out worse

`e2fsck` -O0 (1,908 functions) holds a class none of the ten above do, and one
none of the tables here would show: the value can leave a function WORSE typed
than `off` leaves it.

```
| e2fsck-O0 | 1908 | 185 | 175 | 9 | 1 |
all classes: {'record name only': 175, 'field accesses only': 9, 'declaration-count delta': 1}
```

175 of the 185 are a record rename, as everywhere else. Of the nine that respell
a body, four change a variable's type rather than its record's number. Over the
binary's 18,080 exported variables (`--json variables[]`, which is what decbench
scores) the value changes 265: 254 a record renamed, 1 gained, 3 lost, 7 other.

```
$ kuna decompile-all .../O0/e2fsprogs/stripped/e2fsck --option structmerge off --json
$ kuna decompile-all .../O0/e2fsprogs/stripped/e2fsck --option structmerge siblings --json

sub_724c2  void *a0                              -> struct_81 *a0          (gained: 5 claims)
sub_73069  struct_87 *a0                         -> void *a0               (lost)
sub_2390b  struct_35 *a0, struct_35 *local_30    -> void *                 (lost)
sub_4618d  char *a2, char *a4                    -> unsigned long a2, a4   (lost)
```

`sub_4618d` is `reconfigure_bool`, and it is the only one of the four decbench
can price: type_match 0.6667 -> 0.3333, the binary's aggregate 201.21 -> 200.88
over an unchanged 12 perfect functions (`typesweep --project e2fsprogs --opt O0
--opt O2`). The other three are functions with no matched ground-truth variable,
so the metric never sees them; they are a quality loss all the same.

The cause is the convergence sweep, not the union. The union is minted under a
fresh name and supersedes the thinner record it contains, so every function that
named the thinner record decompiles again -- and the containment bounds that keep
a reader of two fields off a record of five put the union out of its reach, so
the redo measures something else and can settle lower than the answer it
replaces. `sub_724c2` and `sub_73069` read the same e2fs file handle: `off`
gives the two-field reader `struct_87` and the five-field reader nothing, and
`siblings` gives the five-field reader the union and leaves the two-field reader
with `void *`. With the sweep disabled the whole binary is byte-identical under
both values, which localizes the class exactly:

```
$ for m in off siblings; do kuna decompile-all .../O0/e2fsprogs/stripped/e2fsck \
    --option structmerge $m --option protoorder lock --json > lock-$m.json; done
$ # sub_73069, sub_2390b, sub_4618d, sub_724c2 identical in both
```

A union the factory declines to complete is not this class: `lookup_or_mint`
falls through to the reader's own claims, exactly as `off` mints them
(`KUNA_STRUCTMERGE_TRACE=1` prints the fall-through). That path does not fire
here -- 27,282 traced candidates on this binary, 41 merged, 0 mint failures --
so it is not the explanation for the three lost records.
