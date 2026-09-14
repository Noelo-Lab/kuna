# Sixteen-byte SIMD declarator plan

1. Reproduce the seven malformed casts on the preserved exact dataset witness
   from authoritative main.
2. Replace the inverted one-level pending-pointer walk with a generic C
   declarator-precedence builder.
3. Flip the test that deliberately pinned the known divergence and add nested
   pointer/array/function controls.
4. Prove the exact witness changes only the seven malformed type spellings and
   run a strict C compiler over representative declarations.
5. Run the complete datatest, stage, CLI, workspace, and source-corpus gates;
   enumerate and syntax-check every broader declarator change.
6. Close only the `sixteen-byte-simd-accesses` registry row, preserving all
   unrelated sparse/index-only rows.
