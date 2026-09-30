# fieldtype — plan

1. Census the address-of family on the castbench arm of main by what the operand
   is (`census2.py`), every cast on a synthesized member with its declared field
   type (`census4.py` over `decompile-project` headers), and every synthesized
   field's accesses (`KUNA_FIELDTYPE_TRACE`, `ftanalyze.py`); analysis.md section 1.
2. Keep every access of a field's widest width in structsynth's evidence and let
   `p5_types/kuna_fieldtype.rs` declare a field the program uses as a pointer as
   the most specific pointer those uses carry, option `fieldtype on|off`; a type
   merged into a value is not evidence, and a field used as a number stays one
   (analysis.md section 2.1).
3. Prove value preservation with compiled round trips (gcc -O0, clang -O0,
   gcc -O2, clang -O2 builds of `fieldtype_x86_64.c`, the printed C compiled with
   gcc and clang, option off and on, against the binary's own output), a two-pass
   stage test with an index and a merged number as integer controls, typesweeps
   on the campaign slice and a disjoint one, and the whole-corpus hunk
   classification (`hunks.py`, `otherclass.py`, corpus-hunks.txt).
4. Fix what measuring it exposed: the convergence sweep dropping a callee's
   unrelated caller statements (tar `exclude_add_pattern_buffer`).
5. Flip per the default-on procedure: every criterion passes but the layout gate
   (fields-only precision .8713 -> .8693), so the option ships off
   (default-on-evaluation.md).
