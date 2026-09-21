# `structsynth nest` as the default: evaluation

Measured on the rebased base `2e8e407be` (main after #698/#699) unless noted.
Both arms are the same build: `--option structsynth param` (the current default)
against `--option structsynth nest`. The flip was built and gated as its own
change, then reverted. Decision: **not flipped**. `nest` ships opt-in.

| criterion | result |
|---|---|
| (a) `make test` with `nest` as the default | 675/675, PARITY OK, no assertion moved |
| (b) `make test-stages` with `nest` as the default | 1272/1272, PARITY OK; the only new keys are `STRUCTSYNTH-NEST #1`-`#8` (pass 1 pins `param`) |
| (c) `make test-cli` with `nest` as the default | 214/214 |
| (d) 444-slice typesweep, nest vs param | 1,349 -> 1,349 perfect, aggregate 3,659.62 both, 0 improved, 0 worse; 69 functions differ in variables (struct numbering), all score identically. Same on `97315551d`. |
| (e) serial speed, interleaved min-of-15 | nest vs param: fmt O2 -1.96%, ls O2 -5.03%, sort O2 -3.00%, bash O2 -1.67% (bash 89,706 -> 88,208 ms). Within budget. |
| (f) whole-corpus hunks, 15 binaries / 7,819 functions | 93 change: 86 `struct_N` numbering only, 7 reads respelled through the nested record; 0 control-flow deltas; 0 arity changes; +1 declared local in one function; ARM32 unchanged |
| (g) `p0_knowledge/modes.rs` | `structsynth` is in no mode preset; nothing to keep coherent |
| sharded speed, `decompile-all --jobs 8 --option protoorder off`, tar O2, min of 3 | **param 15.9 s, nest 26.0 s (+63%)** |

## Why not

The last row. A request whose field points at a synthesized structure (a nested
record) or at its own structure (`SELF`) gets no `TypeRecipe`, because the
worker's number for that structure means nothing to another process. When such a
request mints, `jobs.rs` sends every function with a synthesized structure back
through one ordered worker (`--jobs-synth serial`). Under `param` tar O2 renames 100
of its 106 structure-bearing functions in place and redoes 10 in parallel. Under
`nest` all 106 run serially after the first pool. The output is byte-identical to
the serial run in both cases (checked on grep O2 with `--jobs 4`), but the
sharded surface loses most of its parallelism on any binary where nesting fires.
The accuracy gain behind the flip is small: 2 more nested fields against DWARF on
10 builds, and reads respelled in 7 functions over 15 binaries. The flip does not
pay for that cost under the user's "no large speed hit" rule, even though
criteria (a) to (g) all pass literally.

## What would make it a default candidate

A recipe for a nested field that names the *request* which minted the inner
structure, not its number: the replay already answers that earlier request
first, in the same function, so it can put the serial name into the outer
request's key before deciding it. Add a `SELF` recipe that a forced worker
mints around the shell (`ledger::mint`). Then only the requests that really
changed need a forced redo, as under `param`. Re-measure the sharded row after
that change, and flip if it is back within budget.
