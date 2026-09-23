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
