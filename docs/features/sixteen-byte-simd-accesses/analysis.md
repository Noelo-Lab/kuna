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

Broad datatest, stage, CLI, corpus-differential, and full-workspace evidence is
recorded in `record.json` once run on the final head.
