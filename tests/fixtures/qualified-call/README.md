# Indirect callback result with partial-unit DWARF

Original GCC-built x86-64 code compiled without optimization calls `initialize`
through a local function pointer and branches on its integer result. DWZ moves
shared declarations into a partial unit. Such an incomplete parked declaration
must not replace the callee's code-type prototype with a locked void return.

Rebuild with `python3 build.py --cc gcc --dwz dwz`. The build verifies a DWARF
partial unit and the original executable's exit code. `provenance.json` seals
source and binary hashes. CI consumes the retained ELF and needs neither GCC
nor DWZ.
