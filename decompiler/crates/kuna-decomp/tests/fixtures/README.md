# Test fixtures

## `list_action_decompile_snapshot.txt`

Captured kuna `decompile` action tree used as bootstrap metadata by
`scripts/observatory/schedule.py` when building the pass atlas. It began as a
Ghidra `list action` capture and was later updated with kuna passes; it is not an
independent C++ oracle or a regression contract for registration layout.

Retain this input while the atlas consumes it. Refresh from the real
`universal_sched(...).list_action_dump(...)` API with the default decompile
groups from `build_default_groups`; the console's `list action` is not yet wired.
Do not edit indices or invent pass positions. Runtime tests should exercise
behavior and meaningful filters, not pin the capture's size or pass adjacency.
