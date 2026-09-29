# conststr — plan

1. Census every `(T *)<const>` on the castbench shared set by what it addresses, with
   `globalref`'s refusal and IDA's print at the same address (`census.py`, `join.py`,
   `dpairs.py`, `strcensus.py`, `trace_run.py`).
2. Fix the sound classes behind `conststr on|off` (P9): the tail-merged `""`
   `emptystrconst` declines (the bytes before the NUL are a string's characters) and a
   byte string the UTF-8 check rejects (a byte-exact literal); both only for a character
   pointee, a read-only address inside a program-data section, C output, and not over a
   pointer-table word.
3. Leave non-addresses, records read by member, unknown pointees beside a typed direct read
   (C would need the cast: see analysis.md), and signed/width conflicts alone.
4. Tests: `tests/stages/kuna-conststr.xml` (two passes, gcc -O2 fixture with declared
   callees, four controls), the `decompile_all_cli.rs` round trip over gcc/clang x O0/O2
   builds with the option on and off, compiled by gcc and clang and run against the
   binary's own output, unit tests for the literal, the tail rule and the pointer word.
   `kuna-globalref.xml` and the `globalref` round trip pin `conststr off`: their `w_glyph`
   witness is a GB18030 string `conststr` prints as a literal.
5. Measure: castbench full both directions, whole-corpus hunk classification, 444-slice
   typesweep, speed (interleaved min-of-15), then the default-flip procedure.
