# Normalize barrier forwarding paths before SSA

Gate on Go source language, `gojumpchain on`, and live `gowritebarrier`.
Find a conditional whose forwarding paths lead to a single-entry block with
one recognized write-barrier call and the same join reached by its other arm.
Allow another forwarding path between the call block and the join.

Delete only non-entry, one-in/one-out blocks containing a sole unconditional
branch. Recheck each block before using the existing block-removal primitive.
Do not create duplicate edges or collapse self-loops or two-block cycles.
Leave all actual calls, assignments and memory accesses to their existing passes.

Validate the off/on distinction, a non-Go control, a forwarding path with an
observable write, and an infinite jump cycle. Sweep every function in optimized
and unoptimized upstream fixtures; inspect every changed hunk and any changed
non-runtime callee count. Measure interleaved off/on CLI timings and run all
repository gates before committing.
