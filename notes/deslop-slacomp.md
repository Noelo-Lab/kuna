# SLEIGH compiler cleanup

Five compiler modules suppressed all dead-code warnings. A forced-warning build
identified three unused private items: an empty `macrotable` field, the
`symbol_varnode_size` helper and the `pcode_unported` panic stub. Removing these
also permits removing the blanket suppressions. The compiler builds without
warnings, and its public API and executed compilation paths are unchanged.

Module documentation now describes the completed compiler instead of unfinished
porting milestones. The end-to-end tests describe their actual fixture source:
the checkout's built `.sla` files can come from kuna, so comparing against them
alone is not an independent C++ oracle. All 44 listed source fixtures are
vendored; missing sources now fail instead of silently skipping the comparison.

The independent compiler-cleanup snapshot passed all four gates: 675 upstream assertions, 1,467 stage
assertions, 7,697 workspace tests with 38 existing ignores, and spec checks.
Catalog checks, the 41 compiler tests and public documentation generation also
passed. No regression expectations changed. The combined PR snapshot is checked
separately, with its results recorded in `notes/deslop.md`.

For an independent comparison, Ghidra's `sleigh_opt` was built from the pinned
revision `cef869af04c4740a71ad31a55704045b1b0d1644`. Both compilers compiled all
44 selected specs into temporary files. Their decompressed element streams
matched byte-for-byte for every spec. Neither the installed `.sla` files nor
the vendored sources were modified.
