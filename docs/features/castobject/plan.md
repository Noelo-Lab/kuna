# castobject — plan

1. Census the `(T *)&x` casts on the castbench arm of main by what `x` is and why
   its declaration differs (`addrcensus.py`; analysis.md section 1).
2. Ship the one class a declaration fixes and whose uses agree: a stack
   out-parameter the callee declares `T *`, re-typed in the frame hints before
   the layout decision (`p6_variables/kuna_castobject.rs`), only when no reader
   wants the other sign, and checked again over the merged variable before the
   cast pass.
3. Prove value preservation with compiled round trips (gcc and clang, -O0 and
   -O2 builds of the printed C against the fixture binary's own output, over
   objects initialized from a parameter with top-bit values and opposing
   readers), a two-pass stage test, and the whole-corpus castbench diff.
4. Flip per the default-on procedure (default-on-evaluation.md).
5. Report the classes that belong elsewhere with their measured size: the libc
   aggregate members (`libctypes glibc`, `FILE` alone measured), the pointer
   elements (records), and the two pre-existing out-parameter wrong outputs.
