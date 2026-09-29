# Test fixtures

## `list_action_decompile_snapshot.txt`

Regression snapshot of kuna's `decompile` action tree, rendered by
`SchedNode::list_action_dump` with the default decompile groups. It began as a
Ghidra `list action` capture, but has since been updated to include kuna passes
such as `structsynth`, `constspaceload` and `callpush`. It is not an independent
C++ oracle.

`universalaction_listing.rs` and `verify_w8x_allowlist.rs` compare the listing
verbatim, including numbering and blank lines. They also require an empty
`UNPORTED_ALLOWLIST`; neither test strips missing passes or renumbers the dump.
Separate assertions check pass presence, adjacency and other root filters.

Change the snapshot only for an intentional schedule change. Review each changed
pass, group, flag and registration position against `universal_sched`, then run
the schedule tests and both parity suites. A matching snapshot protects the
registration layout; it does not establish semantic equivalence to Ghidra.
