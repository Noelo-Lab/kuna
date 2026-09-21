# structsynth `locals` plan

1. Measure first: DWARF struct-pointer locals vs parameters on the eight layout
   builds, every IR base with a constant-offset access by its defining op
   (temporary diagnostic, removed), and the scored surface through the metric's
   own pairing.
2. `locals` and `all` as values of `structsynth`: a CALL/CALLIND output is a base
   under every parameter condition; globals left out (measured: none passes).
3. Declines found by reading hunks: a declared non-void pointer return; a value
   that shares a variable (phi through copies, tied storage, the phi-joined
   return register); a buffer filled with text constants.
4. Printer: `opPtrsub` falls back to the merged variable's pointer type rather
   than the functional `PTRSUB(...)` render.
5. Evidence: layout vs DWARF at the call site (`localscore.py`), 444-slice
   typesweep, 16-binary hunks with `canon.py`/`skeleton.py`/`rettype.py`,
   interleaved min-of-15 speed, `--jobs` equality.
6. Default: `locals` only if the default-flip procedure passes.
