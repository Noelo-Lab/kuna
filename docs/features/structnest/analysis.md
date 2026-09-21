# structsynth `nest`: a record pointer loaded from a field

## The gap

`structsynth param` synthesizes a record over a pointer parameter's own
dereferences and stops there. A field that holds a pointer keeps the scalar its
loads carried, so every read through it is raw offset arithmetic again, and the
synthesized layout never contains a struct pointer of its own:

```text
struct struct_0 { long *field_0x0; long field_0x8; int field_0x10; char field_0x14[4]; };
long sub_12f0(struct_0 *a0) {
  long *v1 = a0->field_0x0;
  return (long)(int)v1[1] + *v1 + (long)*(int *)((long)v1 + 0xc) + a0->field_0x8 + ...;
}
```

`results.md` D.8 ranked this third among the remaining levers: struct layout
recall 0.093 and nesting F1 0 on every build.

## What "nesting F1 is 0" actually was

Half of it was the instrument. `structscore.header_layouts` recorded a member's
pointee only when that struct had been defined *earlier* in the header, so a
pointer to a struct defined later, or to the struct being parsed, scored as "not
a struct pointer". Fixed (two-pass over the header's definitions; two new
selftest cases, 35/35). With the fix, `param` already has 68 struct-pointer
members on the 14 builds below, 62 of them real: `protoorder` carries a callee's
`struct_N *` parameter type back into the caller's field type.

## The opportunity, measured before any design

A temporary diagnostic in the pass, over 12 binaries (ls/sort/du O0+O2, find O2,
tar O0+O2, grep O2, diff O0+O2), tallied every pointer-width field of an
accepted parameter whose loaded values are themselves dereferenced: 274 fields.

| class | fields |
|---|---:|
| a load reaches a phi (list walk) or an index (array) | 153 |
| clean, loaded values already `TYPE_PTR`, 2+ offsets incl. 0 | 10 |
| clean, same but loaded values typed as integers | 11 |
| clean, 1 offset or no offset 0 | 97 |
| of all clean: claims a subset of the enclosing record's | 25 (most single-slot) |

The single-slot "subset" rows are exactly the false self-reference trap:
`_obstack_newchunk` reads a chunk's `prev` at 8 where the obstack itself keeps an
unsigned word at 8.

## Design

* Every `LOAD` of one pointer-sized field is a value of the field's type, so
  their accesses are one base's evidence, merged by `Evidence::absorb` exactly as
  `Evidence::record` merges one base (widest wins, a sign contest stays
  contested, a float beside anything is raw bytes, negative evidence ORs).
* The merged evidence is held to the parameter's own conditions
  (`accepts_record`): already `TYPE_PTR`, not type-locked, no phi, index or
  integer use, two offsets including 0, inside `MAX_FIELD_OFFSET`, not an array
  run. Pruned and made dense exactly as a parameter's layout.
* Same record: the loaded record's claims are a subset of the enclosing record's
  (offset, width and type spelling) and its unclaimed bytes are laid out alike;
  checked after the standalone conditions, so one coincident word never counts.
* A self-pointing structure is minted around its incomplete shell (a completed
  structure is a fresh `Rc` and cannot point at itself). Its self field is keyed
  `SELF` in the ledger so two self-pointing layouts of one shape dedupe.
* The shell has no members, which is DWARF's documented "one level less" loss
  (`n->next->val` prints raw). For a synthesized record the shell is resolved: a
  LOAD/STORE value type that is a pointer to a synthesized record's own shell
  takes the pointer to the completed record, at the propagation edge and at the
  load/store cast tokens. Without it grep's `treenext` printed
  `*(long **)((long)v1 + 8)`; with it, `v1->field_0x8`.
* Depth: two records below the parameter. Only the immediately enclosing record
  is compared for self-reference.

## Results (both arms of the same build unless noted)

* **Nested fields vs DWARF** (`nestcensus.py`, 10 builds): `param` 67 struct-pointer
  members (61 real struct pointers, 6 on a non-pointer DWARF field), `nest` 69
  (+2, both real, inner field precision 5/5). The self-referential kwset
  `treenext`/`treedelta` is not in the census: their O2 DWARF subprograms carry
  parameters through `DW_AT_abstract_origin`, which the campaign instrument does
  not follow.
* **Layout** (`nestscore.py`, 14 builds, fields only): precision 0.8630 -> 0.8636,
  recall 0.0698 -> 0.0699; nesting 62/68 -> 63/70 (P 0.9118 -> 0.9000,
  F1 0.0181 -> 0.0184); structs in headers 508 -> 511.
* **TRex Fig.6** (grep O2, tar O2, ls O2, du O0): identical in both arms
  (1.6362, 1.663, 1.7331, 4.0488). The score reads `variables[]`, which nesting
  reaches only through register locals, which are not exported.
* **typesweep** (444 slices, 10,748 functions): 1,349 perfect in both arms,
  aggregate 3,659.62 in both, 0 improved, 0 worse; 69 functions' variables
  differ, all by `struct_N` numbering, all scored identically.
* **Whole-corpus hunks** (`hunks.py`, 15 binaries, 7,819 functions, three ARM32
  firmwares): 93 change; 86 only in `struct_N` numbering (a nested record is
  minted before the record holding it); 7 are reads respelled through the nested
  record (grep O2 `treenext`/`treedelta`, tar O2 `dumpdir_next`, `comma`,
  `re_compile_fastmap_iter` and two dumpdir callers). Two side effects inside
  those 7, both of them results of the new pointee type: tar `0x1f3f0` declares
  one more local (a `uint8` count and an `int8` word no longer share a
  variable), and grep `treedelta` holds a recomputed byte address in a temporary
  (same two accesses, same order). No control flow moves. ARM32: 0 changes (no
  pointer field there is loaded and dereferenced at 2+ offsets).
* **Round trip**: 487 synthesized structs over 10 exported projects compile with
  every `offsetof` exact; `gcc -fsyntax-only` error counts over four whole
  projects are identical in both arms, with no "has no member" error in either;
  the recursive definition compiles standalone.

## Default

Not flipped; `nest` is opt-in. Every default-flip criterion passes (see
`default-on-evaluation.md`), and serial `decompile-all` is 1.7% to 5.0% faster
under `nest`. But a sharded run is 63% slower on tar O2 (`--jobs 8`: 15.9 s ->
26.0 s), because a nested or self field has no portable recipe and the pool
names every structure-bearing function in one ordered worker. The accuracy gain
is too small to pay for that. The fix is a request-relative recipe in `shard.rs`.

## The measured next lever (not shipped)

The binding constraint is "already a pointer": most loads of a pointer field are
dereferenced only at non-zero offsets, and the lattice types them `long`. Two
relaxations were measured with temporary toggles over the same 10 builds:

| variant | nested fields beyond `param` | each a DWARF struct pointer | inner field precision (new) |
|---|---:|---:|---:|
| shipped (strict) | 2 | 2/2 | 5/5 |
| no offset-0 condition for a nested record | 5 | 5/5 | 17/21 |
| pointer-ness from the dereferences | 17 | 17/17 | 42/44 |
| both | 33 (1 self) | 33/33 | 106/134 |

A follow-up value that takes pointer-ness from use for a *loaded field value*
(not a parameter) is the lever; it is not in this PR because pointer-ness is the
one fact `structsynth` does not invent, and the campaign brief keeps that rule
for nested records.
