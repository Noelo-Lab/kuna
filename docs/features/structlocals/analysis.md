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
  682 change against `param`, 610 only in `struct_N` numbering, 61 verified by
  `canon.py` as the same statements respelled through the returned record, 11
  read (past-the-end `&v1[1]`, `NULL`/`4U`, one temporary holding `field - 0x20`,
  one zero-store pair merged into its 8-byte field, one float field as raw bytes).
  `skeleton.py`: 0 control-flow or call-sequence deltas. Return types: 11 change,
  11 DWARF struct pointers (`rettype.py`).
* **Speed**: see `record.json` (`speed`).

## Default

See `default-on-evaluation.md` and `record.json` (`default_decision`).
