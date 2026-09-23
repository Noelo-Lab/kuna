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

No skeleton delta, no declaration-count delta and nothing unclassified in 4,904
functions. 91 of the 92 changed functions are a record rename: the parameter's
`struct_N` number moves and its record gains members, and the body is otherwise
identical.

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
