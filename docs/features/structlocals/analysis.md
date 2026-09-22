# structsynth `locals`: records for call-returned pointers

## The gap

`structsynth param` measures a pointer parameter's own dereferences and
nothing else. A record a function gets from a call keeps the pointee the
lattice gave it, so the most common way C builds a record prints as raw
offsets:

```text
v1 = (unsigned long *)sub_18024(a0 << 8);          // sort O0 merge_tree_init
v1[3] = 0; ... *(unsigned int *)&v1[10] = 0; *(char *)((long)v1 + 0x54) = 0;
```

## What exists, measured before any design

Ground truth (`gtcensus.py`, DWARF of the eight layout builds `fmt`/`ls`/`sort`/`du`
at O0 and O2): 750 struct-pointer locals against 816 struct-pointer parameters.
417 live in a stack slot (`DW_OP_fbreg`, almost all O0), 312 in a location list
(registers, O2).

IR (a temporary diagnostic in the pass, removed): every base with a
constant-offset access, by what defines it, and whether it meets a parameter's
conditions (pointer already, not locked, no phi/index/integer use, 2+ offsets
incl. 0, not an array run):

| base class | O0 meets | O2 meets |
|---|---:|---:|
| parameter (shipped) | 110 | 229 |
| value a CALL returned | 37 | 17 |
| phi output (loop walk / merge) | 9 | 83 |
| loaded from an array element | 1 | 3 |
| global RAM (13 bases with 2+ offsets) | 0 | 0 |

All 54 call-returned candidates are real records: 24 are `hash_entry` pointers
from `safe_hasher`/`allocate_entry` (callee DWARF return type, 2/2 fields), the
other 30 are `xmalloc`/`malloc`/`calloc`/`xzalloc` results read by hand
(`merge_node`, `pending`, `tm_zone`, `userid`, `argv_iterator`, `mount_entry`,
`exclude_segment`, ...). Phi outputs are what the rules decline for a parameter
(`p = p->next`). Every global base reaches a phi or an INDIRECT (a global is
re-read after each call), and a global *record* is not a base at all: each of its
accesses is an absolute address of its own. So the class that pays is the
returned value; globals are left out.

## The scored surface is empty for this class

`type_match` scores `variables[]`: arguments, stack symbols and `framelayout`
slots. Pairing the ground truth the way the metric does: of 413 O0
struct-pointer locals, 217 pair with a `framelayout` slot typed `undefined8` --
the slot a returned pointer is stored to is copy-propagated away, and
`framelayout` keeps the slot from the first restructure pass, before type
recovery starts -- and 175 with live stack symbols, which are loop variables
(phis). At O2, 577 of 1,000 are unpaired (registers). The typesweep below
confirms it: nothing this value types is scored.

## Design

* `structsynth locals` (and `all` = `locals` + `nest`), values of the existing
  option. A base is a parameter or the output of a `CALL`/`CALLIND`
  (`is_call_return`); the evidence walk is unchanged (the peel already stops at
  a call's output) and every parameter condition applies.
* A callee declared to return a pointer to something (`char *`, `struct stat *`)
  declines; only `void *` or an unlocked return is measured.
* `joins_other_values`: the value, and every COPY/CAST of it, must reach no
  MULTIEQUAL or INDIRECT and must not sit in address-tied storage or in the
  function's return register where a phi joins that register. The first build
  without this rule declared `tar`'s `wordsplit_add_segm` and
  `parse_transform_expr` and `grep`'s `Pexecute` to return `struct_N *`: the
  record shared `rax` with the returned status/string, the return register is
  tied into one variable once merging starts (`mark_output_storage_addr_tied`),
  and a type-locked member decides a variable's type. 11 of 36 return-type
  changes were wrong before the rule, 0 of 11 after.
* A base whose every access stores a constant whose bytes read as text is a
  string buffer (`constant_text`): `tar` writes `"SCHILY.xattr."` into a fresh
  allocation as 8/4/1-byte stores, which a record would print as integer
  constants instead of `builtin_strncpy`. Zero stores are neutral.
* Printer fix, found by the hunks: `opPtrsub` reads the merged variable's pointer
  type where the member's own type cannot name a member. A global assigned a
  `struct_N *` and walked as `unsigned int *` printed `v4 = PTRSUB(dat_263f0,0);`
  (not C; one instance on `main`, tar O0, 10 more under the first `locals` build).

## Results (both arms of one build unless noted)

* **Census, final rule** (8 layout builds): 24 (O0) and 11 (O2) returned values
  accepted; 13 and 6 decline because the value shares a variable.
* **Layout vs DWARF** (`localscore.py`, 14 builds; call-defined locals declared a
  record pointer, joined at the call site): 40 -> 128 records; 105 struct
  pointers, 3 not (`grep` `realtrans`, an array of two pointers), 20 unjoined.
  New records' claimed fields 187/223 exact (0.839; parameters 0.862), misses are
  merged byte stores (`mode_change` `op`/`flag`) and a union measured at its first
  word. Parameter layout unchanged: P 0.8622 -> 0.8623, R 0.0666 both, nesting
  unchanged; 500 -> 548 structures in the headers.
* **typesweep** (444 slices, 10,748 functions, metric pinned 625e892): 1,353
  perfect in both arms, 0 improved, 0 worse; all 65,377 scored decisions the same
  type up to `struct_N` numbering; no function's variable count changes.
* **Hunks** (16 binaries, 8,682 functions incl. ARM32 chibios/freertos O0):
  693 change against `param`, 621 only in `struct_N` numbering, 61 verified by
  `canon.py` as the same statements respelled through the returned record, 11
  read (past-the-end `&v1[1]`, `NULL`/`4U`, one temporary holding `field - 0x20`,
  one zero-store pair merged into its 8-byte field, one 4-byte filler field
  `char field_0x24[4]` stored through its decayed address).
  `skeleton.py`: 0 control-flow or call-sequence deltas. Return types: 11 change,
  11 DWARF struct pointers (`rettype.py`).
* **Speed**: see `record.json` (`speed`).

## Review round: holes in "alone in its variable"

The first review found wrong types the numbering/respelling classifier could
not see, because `canon.py` drops declarations and strips casts:

* `kmod` O2 0x7ee0: a `calloc` record in the `rax` variable that also held a
  name from `strdup` and its length from `strlen` (`struct_38 *v11;`, `v11 =
  (unsigned long)strlen((char *)v11);`). The function returns `int` in `eax`; the
  old rule matched return storage by exact size on the first `RETURN` only.
* `dash` O2 0x12750 and a two-path `either()`: a record returned on one path,
  another call's result on the other, both typed by the one return type
  (`v2 = (struct_6 *)sub_12010();`). Kuna duplicates the return block, so there
  was no phi to decline on.
* `cp` O2 0xbe10: a `mempcpy` tail filled by loads from `"CuXXXXXX"` in
  `.rodata`; the text rule looked at constant stores only.

Fixes (`kuna_structsynth.rs`): return storage is matched by overlap on every
live `RETURN` (`storage_is_tied`); a record returned beside a different
non-zero value declines (`returned_beside_others`); bytes loaded from a
read-only literal, INDIRECT-clobbered or not, count as text up to 16 bytes
(`stored_text`). A declaration- and cast-aware classifier (`declcast.py`)
flagged all three witnesses on the reviewed outputs, and on a 29-binary sweep
it found one more class: an address formed just past the fields a function
reads (`sortlines`' `&node->lock`, `mountlist`'s `&me->me_next`, `kmod`'s
flexible `name[]`, `grep`'s `&buf[2]`) printed as `&v1[1]` and gave the mutex or
the list tail the record's type. `points_past` declines those bases.

The 8-byte store split into a 4-byte field and four filler bytes
(`v3->field_0x4[0] = 0; ...`) was a record type reaching a value that was never
measured, through the return type; with `returned_beside_others` no function in
the sweep gains such a store.

Re-measured on the fixed build (both arms of one build):

* **Hunks** (29 binaries, 12,939 functions: the 16 above plus gzip O0/O2,
  bzip2, useradd, diff3 O0, sdiff, cp, dash, crond, kmod, sort O2-noinline,
  xmlwf, find O0): 872 change against `param`; 769 only in `struct_N`
  numbering; 83 are the same statements respelled with clean declarations and
  casts; 20 flagged and read: 15 give the record a variable of its own where
  `param` merged it with an unrelated same-typed value in one register (one
  more local; `ls` `print_dir` two), 4 are `find` predicate records whose name
  field holds a string literal, 1 is a local typed `void *` from the record's
  field (`kmod_new`). 0 control-flow or call deltas, 0 arity changes, 0 filler
  element stores. Return types: 27 change, 24 DWARF struct pointers, 2 records
  behind a `void *` handle (`BZ2_bzWriteOpen`, `sharefile_init`), 1 a `Cell *`
  whose DWARF type sits on its abstract origin. `main` equals the `param` arm on
  28 binaries and differs on tar O0 by the PTRSUB printer fix alone.
* **Layout vs DWARF** (14 builds): 40 -> 116 call-defined record locals, 103
  struct pointers, 0 not (`grep`'s `realtrans` pair now declines: it forms `&realtrans[2]`), 13 unjoined;
  new records' claimed fields 173/203 exact (0.852). Parameter layouts unchanged.

* **After the rebase onto 656e6c701** (#710 `slotptr`): the hunk
  classification is identical; the typesweep is 1,472 perfect in both arms, and
  95 scored decisions now change type, each an -O0 slot `slotptr` types from the
  stored record, from `undefined8`/`long *`/`int8 *` to `struct_N *` where DWARF
  names a struct pointer. None replaces a correct pointer; the pinned metric
  scores them the same because it compares pointee names.

## Default

See `default-on-evaluation.md` and `record.json` (`default_decision`).
