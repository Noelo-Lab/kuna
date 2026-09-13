# Sixteen-byte SIMD declarator analysis

The round-11 witness is the 16,384-byte PE
`crackme1.exe` (SHA-256
`16512154b0c419b2e4b064dbabd4932251ec5e0e11e14302d86821dca1ade1f0`),
function `sub_1400012f0`.  On authoritative main
`ccc82a37812a1bbfd6c2eac43f1fa6597d631efa`, one decompilation contains seven
casts spelled `*(char *[16])`.  Each cast is attached to a sixteen-byte lane;
the output declares the destination as `char v63[16]` and preserves the
`._0_16_` access width.

The cast emitter is not the source of the error.  `push_cast_type` correctly
concatenates the two halves returned by `CSpeller::declarator`.  The speller
builds a stack in outermost-to-base order, but consumed it in reverse.  Its
"pending pointer" rule therefore grouped an array of pointers and failed to
group a pointer to an array, swapping the two C types:

- `pointer(array(char, 16))` became `char *[16]` instead of `char (*)[16]`;
- `array(pointer(char), 16)` became `char (*)[16]` instead of `char *[16]`.

Simply reversing that loop is sufficient for those two shallow types but is
not a complete declarator algorithm.  For example, when another postfix is
already present, inserting the close parenthesis before the accumulated suffix
changes `(*x[2])[3]` into `(*x)[2][3]`.  The repair therefore represents the
identifier-surrounding declarator as a front and back.  Modifiers are consumed
outermost-to-base: a pointer prefixes the complete current declarator; an array
or function postfix groups the complete current declarator only when an
ungrouped pointer is pending, then appends its suffix.

This is an unconditional correctness repair.  An option would preserve invalid
C and the wrong type relationship, so there is no useful compatibility policy
to expose and no phase or XML identifier to allocate.  The Rust speller is
independent and does not use this C declarator builder.

Focused evidence before the broad gates:

- exact witness: seven invalid casts before, zero after; seven corrected
  `*(char (*)[16])` casts after; the other 623 output lines are byte-identical;
- the former divergence unit now asserts the correct mirror spellings;
- nested three-modifier Datatype tests cover both grouping directions;
- a table covers pointer/array/function postfix orderings, including alternating
  three- and four-level declarators;
- the representative declarations compile with
  `cc -std=c11 -Wall -Wextra -pedantic-errors -fsyntax-only`.

The implementation-head broad gates preserve 675/675 datatest assertions, 824/824 stage
assertions, and 158/158 CLI probes. Both spec checks and catalog consistency
pass. A deterministic before/after sweep covered 253 binaries and 2,949
functions: every in-repo executable analysis fixture, 20 binaries from each of
DecBench O0/O2/O2-noinline, and the exact witness. Of 253 outputs, 247 were
byte-identical. The six changed outputs contain only three kinds of correction:

- 15 cast/declaration occurrences gain required pointer-to-array grouping (the
  seven witness occurrences appear twice because the exact bytes also exist as
  an in-repo fixture, plus one `structreturn_x86_64` parameter declaration);
- two known outer arrays of pointers lose the wrong grouping;
- one nested array changes `[16][2]` to `[2][16]`, placing the outer dimension
  next to the identifier as C requires.

Four exported variable type strings move with those same corrections. No
function inventory, address, body outside the declarator tokens, variable
identity/storage, or run metadata changes. Every corrected shape compiles under
strict C11. Exact unit assertions pin all four exported strings, and the existing
`structreturn_x86_64` end-to-end control now pins the corrected `passthru`
signature. `record.json` enumerates the affected functions and type strings.

The complete release workspace was started after the pinned gates. A first run
showed only missing generated `.sla` failures; after supplying all 148 generated
specs, the rerun was stopped by the captain at the disk-safety threshold before
completion. It is therefore deliberately not recorded as green; hosted full CI
remains a merge gate.

## Declaration positions

The first repair corrected every place that receives both declarator halves
(casts, parameters, members, typedefs, exported type strings) but not the three
places that spelled a declared type in front of a name only: local
declarations, the element type of an array local, and the function return
type. They took `CSpeller::type_name`, which returned the declarator front. For
a pointer to an array that front changed from the wrong but balanced `char *`
to `char (*`, so a whole-binary sweep printed `char (*v1;` and
`char (* sub_140024330(char (*a0)[32],...)` in 1,128 functions of 98 binaries
(2,967 unbalanced lines). The original differential used `--limit 25` per
binary and never reached those functions.

`TypeSpeller::type_name` now returns the `(front, back)` pair; the back is
empty except for a C pointer whose declarator needs a suffix. A local prints
`<front><name>[ [count]]<back>`, so `char (*pair [2])[16];` keeps the count
inside the group, and the prototype prints the back after the parameter list,
`char (* get_row(int i))[16]`. The rendered-line declaration dedup keys on the
back as well. The Rust speller returns an empty back and its output is
byte-identical.

Evidence on the final head:

- every pointer/array chain up to five modifiers, spelled as a declaration,
  abstract type, struct member, local and return type, compiles under gcc and
  clang with a `__builtin_types_compatible_p` assertion against a typedef-built
  reference (1,413 assertions); a console-parser round trip of the same shapes
  to four modifiers is a unit test;
- `ptrarraydecl_x86_64` (DWARF-typed return types, a pointer-to-array local, an
  array of them, and an array-of-pointers control) is an end-to-end test; three
  of its four cases fail on the first repair and the control passes on both;
- a base/final sweep of 637 binaries and 186,030 functions changed 42,472 lines
  in 2,224 functions: 39,505 move only declarator punctuation with identical
  array dimensions, 2,660 local declarations and 307 prototypes gain the
  pointed-to array suffix, and no other line changed. All 2,819 exported type
  string changes reorder declarator punctuation with identical dimensions.
