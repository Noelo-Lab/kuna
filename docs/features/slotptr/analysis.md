# slotptr — analysis

## 1. The symptom

At -O0 every C local has its own stack slot. kuna's `variables[]` reports those
slots, but mostly as eight bytes of unknown type, while the C body declares the
value that lived there as a pointer:

```
$ kuna decompile-all O0/coreutils/stripped/pinky --addr 0x3075 --json
  "code": "... v1 = getpwnam(v5); ... v2 = strchr(*(char **)&v1->field_0x18,0x2c); ..."
  {"name": "local_1f0", "type": "undefined8", "stack_offset": -496, ...}   <- pw
  {"name": "local_1e8", "type": "undefined8", "stack_offset": -488, ...}   <- comma
  {"name": "local_1e0", "type": "undefined8", "stack_offset": -480, ...}   <- full name
```

The ground truth is `struct passwd *pw; char *comma; char *display_name;`.

Across the campaign's 444 decbench slices this is the largest single piece of
the `char *` gap to Binary Ninja: 805 of the 1,625 ground-truth `char *`
variables binja gets right and kuna missed are such slots
(`docs/features/charptr/next-levers.md` on `feat/charptr-on`).

## 2. Why the type is lost

`restructure_varnode` rebuilds the frame from the live stack Varnodes on every
`mainloop` pass. A -O0 spill goes through three stages:

1. first iteration: the `STORE [rbp-0x1e8], rax` is still a pointer-relative
   store; `oppool2`'s `RuleStoreVarnode` turns it into `stack@-0x1f0 = COPY rax`;
2. second iteration: heritage links the slot; `restructure_varnode` sees the
   Varnode and records `local_1f0` in the `framelayout` union, typed with
   whatever the Varnode carries so far (nothing committed: `undefined8`); then
   `oppool1`'s copy propagation replaces every read of the slot with `rax`;
3. third iteration: dead-code removal deletes the COPY. The slot is in no later
   layout.

The value itself (`rax` = the `getpwnam` call's output) survives to the end and
gets `passwd *` from the libc signature. The only thing lost is which slot it
was stored into, and `framelayout` reads its slot types from stage 2.

An earlier attempt read the slot's symbol from a later pass and changed
nothing, because by then the symbol is gone. The link has to be captured while
the COPY exists and resolved when the types are final.

## 3. Why only pointers

A width-only `undefined8` is kuna's honest statement that it never settled the
slot's scalar type. Replacing it with a scalar would assert something the
recovery did not establish, and it is also exactly where the type_match metric
stops crediting the width (`undefinedN` is accepted against a same-width scalar
ground truth; a committed scalar only against itself). A pointer is different:
the C body already claims it for the same value, and the width-only spelling
never matches a pointer ground truth anyway. So the option only ever turns
`undefinedN` into a pointer.

## 4. What the option cannot fix

The slot gets the type the C body gives the stored value. Where the body's type
is wrong, the slot's is wrong the same way. All 9 ground-truth variables the
sweep loses are of this kind:

- a length or count computed from two pointers that kuna prints as pointer
  arithmetic (`&v7[-(int8)v3]`, `&v3[0xfffffffffffffffe - (long)a0]`) and
  therefore types `char *` / `unsigned char *` / `long *`: diff
  `scan_char_literal` `digits`, diff `find_and_hash_each_line` `length`, tar
  `xheader_string_end` `p` and `size`, tar `expand_pax_option` `__len`, split -O2
  `main` `to_write`, grep -O2 `main` `newkeycc`;
- a register variable that kuna merges across an `lseek` result and an `errno`
  pointer, which the body declares `int *`: shred `dorewind` `offset`, tail
  `tail_lines` `end_pos`.

Both are bugs in the C body, visible there already; fixing them is separate
work and would fix these slots too.
