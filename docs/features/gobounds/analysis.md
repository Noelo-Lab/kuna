# Evidence

Reference: angr `6afa0c79a83d9f677a46b0275cdb63bf16c1979e`,
`tests/analyses/decompiler/test_go_builtins.py` and
`angr/go/optimization_passes/check_remover.py`.

The Go compiler emits an out-of-range branch to `runtime.panicIndex` (older
versions) or the `runtime.panicBounds` dispatcher (newer versions). Kuna already
knows Go's ABI and many no-return names, but its `securitycheck` matcher accepts
only Rust helpers. This leaves one extra branch at each checked access.

A source-oriented view can omit these compiler checks, just as Kuna's existing
Rust securitycheck option does. This does not preserve panic behavior and is
therefore explicitly marked destructive and reversible with `gobounds off`.
Unlike an unrestricted name-based deletion, the matcher rejects observable
stores and other calls in the failure arm. It also retains an unconditional
panic wrapper and a conditional whose both arms panic.

## Breadth and speed

The checked-in diffs cover every function in angr's `builtins` ELF fixtures:
Go 1.22.5: 1,996 functions, 249 changed, 501 compiler-check calls removed;
Go 1.27.1: 2,539 functions, 317 changed, 686 compiler-check calls removed.
Both sides completed with zero decompilation errors. Every changed function
lost at least one compiler-check call. Counts of all other recognized direct
callees were identical, including explicit panic and assertion helpers.
The companion hunk inventories classify the CFG changes and their downstream
signature/type, dead-spill, and local-renumbering effects.

A final 21-pair interleaved CLI measurement (alternating off/on and on/off,
after two warmups) measured 1052.00 ms off versus 1045.88 ms on,
-0.58%, within the 5% budget. Raw samples are in record.json. This includes
image loading; concurrent activity on the shared host makes individual samples
noisy, so no speedup is claimed. Both complete off/on corpus outputs remained
byte-identical after integrating main at `e27d885452316848c5a8ad17ba25169c5caa3d2a`.
