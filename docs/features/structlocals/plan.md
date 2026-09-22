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
3b. Declines the review found (the respelling classifier dropped declarations
   and casts, so a declaration- and cast-aware one, `declcast.py`, replaces it):
   return storage matched by overlap on every RETURN; a record returned beside a
   different value; bytes loaded from a read-only literal are text; an address
   formed past the last access means the record is wider than the function shows.
4. Printer: `opPtrsub` falls back to the merged variable's pointer type rather
   than the functional `PTRSUB(...)` render.
5. Evidence: layout vs DWARF at the call site (`localscore.py`), 444-slice
   typesweep, 29-binary hunks with `declcast.py`/`skeleton.py`/`rettype.py`,
   interleaved min-of-15 speed, `--jobs` equality.
6. Default: `locals` only if the default-flip procedure passes.
