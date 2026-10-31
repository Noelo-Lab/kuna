# Raw p-code control-flow listing

A linear raw p-code listing decodes bytes after unconditional jumps, including
inline data, and does not identify branch joins. `raw-pcode --follow` will decode
from the entry and direct intra-window successors, preserving each lift's raw
operations and emitting basic blocks with explicit outgoing edges. Calls fall
through rather than entering callees. The function inventory or an explicit
byte range bounds the walk; an instruction limit bounds work.

The output must describe evidence and incompleteness. A return has no successor;
a direct conditional branch has a target and fallthrough; an indirect branch,
out-of-window edge, undecodable instruction, unsupported p-code-relative flow,
or exhausted budget is reported with its address and makes the graph incomplete.
A delay-slot lift covers the entire consumed span. No inferred jump-table edges
or claim of general reachability is made. Linear mode remains unchanged.

Tests use an x86 jump over inline bytes with a conditional join, an indirect
branch and a bounded decode; existing ARM/Thumb and MIPS tests protect mode and
delay-slot behaviour. Validate the JSON and text output and both parity corpora.

On this macOS host, five release CLI invocations of `fauxware main --count 5
--json` had median wall times of 49.1 ms for linear and 46.3 ms for `--follow`
(-5.7%). The modes visit different paths, so this small, load-dominated sample
is a command cost check, not a benchmark of equal work.
