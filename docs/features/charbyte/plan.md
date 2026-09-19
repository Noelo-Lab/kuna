# charbyte — plan

## Option

`charbyte on|off`, P5 `type-propagation`, tier `core`, change_kind
`type-inference`, shipped **on** (see record.json `default_on`).

## Mechanism (`p5_types/kuna_charbyte.rs`)

1. `run_infer_types` calls `begin`, which arms a per-thread record when the
   option is on.
2. `propagate_type_edge` calls `note_edge` (one line) before its `type_order`
   test. On a `LOAD` edge it records the byte when the pointer carries `char *`
   and the byte holds `uint1` only because of the zero-extension: pointer-to-byte
   (`char` offered to a `uint1` byte) or byte-to-pointer (`uint1 *` offered over
   `char *`), whichever the DFS order reaches first. "Only because of the
   zero-extension" = an `INT_ZEXT` reads the byte and no other reader's
   `input_type_local` is `TYPE_UINT`. Two kinds of byte are never recorded
   (`is_candidate`): one that reaches a call or user-op argument, a return or
   a switch through copies, joins and one-byte arithmetic
   (`read_without_cast`), and a counter whose own sum is stored back through
   the address it was loaded from (`is_counter`).
3. After the first propagation, `take_noted` ends the record. If it is
   non-empty, `build_localtypes` runs again, `seed_char` replaces those bytes'
   `uint1` seed with `char`, and `propagate_all` runs again. No edge rule
   changes, so everything downstream is a propagation upstream's rules produce
   from a different seed.
4. `keep_or_restore` compares the second propagation with a snapshot of the
   first and keeps it only if every Varnode's type is unchanged or the same
   type with `uint1` read as `char` (through pointers and arrays), no byte
   that moved to `char` is read without a cast, is a counter or is loaded
   through a pointer that did not become `char *` (this catches a sibling byte
   read through the retyped pointer), and no pointer that moved copies into or
   out of one that did not; otherwise the snapshot is restored (analysis.md
   §3b, §5).

Rejected: overriding the edge (install `char` against `type_order`, refuse the
`uint1 *` push). It leaves Varnodes the byte already typed `uint1` behind, which
prints `char v1; ... v1 == 0xe9` - wrong C (analysis.md §3).

## Tests

* `tests/stages/kuna-charbyte.xml`, two passes (off, on), 21 assertions. The
  review round added four controls that keep `uint1` in both passes:
  `argbyte` (the byte passed to `logit` as a one-byte argument), `sibling`
  (`a0[1]` passed to `logit` while `a0[0]` qualifies), `counter` (gcc -O0
  `c = *p; *p = c + 1`) and `highcase` (a switch over the byte with a
  `case 0x80:` label). The original 13:
  `isdash` (the witness: `v1 == 0x2d` -> `v1 == '-'`, widened copy as
  `(uint4)(uint1)v1`), `isdashu` (an `uint1 *` table byte stays `uint1`),
  `highbit` (an unsigned-compared byte stays `uint1`), `ishigh` (the `0xe9`
  constant becomes `'\xe9'` with the byte), `dashlen` (a pointer that is `char *`
  only through `strlen`: off `uint1 *a0` + `strlen((char *)a0)`, on `char *a0`).
* Unit tests (`kuna_charbyte/tests.rs`): the lattice fact (`uint1` outranks
  `char`), the `char`/`uint1` predicates, one-shot record semantics.
* Whole-corpus before/after on 8 binaries, then 34, with every hunk
  classified and every unchanged line that reads a retyped byte audited; a
  compile-and-run oracle over all 256 byte values for every value-bearing form
  (analysis.md §4-5, `oracle.c`).

## Default flip (the campaign's procedure, re-measured on 6e4f6fa5 + this branch)

(a) `make test` with the default on: 675/675, PARITY OK, no assertion moved.
(b) `make test-stages`: 1177/1177, PARITY OK; the only new keys are the 21
    charbyte assertions (pass 1 of the stage test says `option charbyte off`).
(c) `make test-cli`: 189/189.
(d) 444-slice typesweep, on vs off: 0 improved, 0 worse, perfect 986 -> 986
    (record.json `benchmark`; 19 variables move from `unsigned char *` to
    `char *`, all with GT `char *`, and 21 sort -O0 `struct_N` names renumber).
(e) interleaved timing, min-of-15: worst minimum +1.17% (bash -O2; fmt +0.23%,
    ls -0.71%, sort -0.41%); record.json `speed`.
(f) 34-binary hunk classification with the unchanged-line audit: analysis.md
    §4-5.
(g) `modes.rs`: nothing to do; a default-on option is outside the aggressive
    preset's default-off scope.
