# structsynth `locals` -- every changed function, classified

`decompile-all` over 29 stripped binaries (decbench `full_run_address_2026-09-11`),
one build, `--option structsynth param` against the default `locals`.
Classified by `declcast.py` (signature, declarations and casts compared, locals
named by first use so a split variable does not renumber the rest) on top of
`canon.py` (each access rewritten to `<var>@<offset>`); `skeleton.py` compares
control flow and the call sequence.

* numbering: identical after collapsing `struct_N` numbers.
* clean respelling: bodies equal after canon, and every declaration change is a
  pointer that became a record defined once by one callee's result, with no new
  cast of it to another type and no record return of another value.
* read: anything else, each listed below.

| binary | functions | changed | numbering | clean respelling | read |
|---|---:|---:|---:|---:|---:|
| bzip2-bzip2-O2 | 114 | 9 | 6 | 3 | 0 |
| chibios-ch.elf-O0 | 772 | 1 | 0 | 1 | 0 |
| coreutils-cp-O2 | 409 | 1 | 0 | 1 | 0 |
| coreutils-du-O0 | 441 | 38 | 33 | 4 | 1 |
| coreutils-du-O2 | 320 | 41 | 36 | 4 | 1 |
| coreutils-fmt-O0 | 191 | 0 | 0 | 0 | 0 |
| coreutils-fmt-O2 | 151 | 0 | 0 | 0 | 0 |
| coreutils-ls-O0 | 589 | 2 | 0 | 1 | 1 |
| coreutils-ls-O2 | 404 | 1 | 0 | 0 | 1 |
| coreutils-sort-O0 | 479 | 8 | 7 | 0 | 1 |
| coreutils-sort-O2-noinline | 490 | 22 | 20 | 1 | 1 |
| coreutils-sort-O2 | 343 | 0 | 0 | 0 | 0 |
| cronie-crond-O2 | 180 | 4 | 3 | 0 | 1 |
| dash-dash-O2 | 335 | 15 | 10 | 4 | 1 |
| diffutils-diff-O2 | 398 | 25 | 22 | 2 | 1 |
| diffutils-diff3-O0 | 271 | 21 | 20 | 1 | 0 |
| diffutils-sdiff-O2 | 197 | 0 | 0 | 0 | 0 |
| findutils-find-O0 | 862 | 58 | 44 | 11 | 3 |
| findutils-find-O2 | 658 | 63 | 48 | 13 | 2 |
| freertos-RTOSDemo.out-O0 | 150 | 6 | 6 | 0 | 0 |
| grep-grep-O0 | 642 | 45 | 40 | 4 | 1 |
| grep-grep-O2 | 449 | 36 | 31 | 4 | 1 |
| gzip-gzip-O0 | 248 | 0 | 0 | 0 | 0 |
| gzip-gzip-O2 | 204 | 0 | 0 | 0 | 0 |
| kmod-kmod-O2 | 394 | 87 | 75 | 11 | 1 |
| libexpat-xmlwf-O2 | 126 | 0 | 0 | 0 | 0 |
| shadow-useradd-O2 | 427 | 3 | 1 | 2 | 0 |
| tar-tar-O0 | 1570 | 257 | 248 | 8 | 1 |
| tar-tar-O2 | 1125 | 129 | 119 | 8 | 2 |
| total | 12939 | 872 | 769 | 83 | 20 |

`skeleton.py`: 0 control-flow or call-sequence deltas in 12,939 functions.
No function's argument count changes. No function gains an element store into
a filler array. `origin/main` (2888a2a94) equals the `param` arm on 28 binaries;
on tar O0 it differs in one line, the `PTRSUB` printer fix (`0x2c4f3`).

## The 20 read

The record gets a variable of its own where `param` merged it with an unrelated
value of the same primitive pointer type in one register (one more local each;
`ls` `print_dir` two):

| function | what shared the variable |
|---|---|
| du O0 0xe932, ls O0 0x12f60, sort O0 0x117de, find O0 0x22182, grep O0 0x20ea7, tar O0 0x531ed, sort O2-noinline 0xd330 | gnulib `transfer_entries`: the `safe_hasher` bucket and the `allocate_entry` entry (separate `bucket`/`new_bucket` in the source) |
| du O2 0x6b70, diff O2 0x10680, grep O2 0x16d40, tar O2 0x40c90 | gnulib `add_exclude`: the 0x28 segment and the 0x10 pattern record |
| ls O2 0xd820 | `print_dir`: the `dev_ino` record, and the obstack cursor |
| kmod O2 0x17bd0, 0x7bf0 | a cursor (into an array; down a list) and the new `malloc`/`calloc` node |
| dash O2 0x4f40 | `aliascmd`: a table cursor and the new alias record |
| crond O2 0x5300 | the `find_user` result and the orphan-list head; the `load_user` result and a `calloc` orphan record |
| tar O2 0x28cf0 | the `hash_lookup` result and a list link loaded later |

Records whose name field holds a string literal (`find` predicates, true
records): find O0 0x17be3, 0xd2d0; find O2 0xad30, 0xb9b0.

A local typed from the record's field: kmod O2 0xf820 (`kmod_new`), `long v4`
becomes `void *v4` for the `hash_new` result stored in a `void *` field.

## Return types

27 functions' return types become the record they build: 24 are struct
pointers in DWARF; `BZ2_bzWriteOpen` and `sharefile_init` return a record behind
a `void *` handle; bzip2 0x3820 builds a `Cell` whose DWARF return type sits on
its abstract origin (`rettype.py`).
