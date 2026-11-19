# [PROPOSAL] diffverify — differential semantic checking with z3, QEMU and kuna

Status: **proposal**. Design only; nothing is implemented before approval.
Evidence, coverage estimates and the hard part: `analysis.md` in this folder.

## What this makes possible

A command that says whether kuna changed what a function computes, and on which input:

```
$ kuna-verify check ins16.o ins16          # planned output
ins16  DIFFERS  (run 3 of 256)
  input    a0=0x00007fff a1=0x0000   (r0, r1)
  machine  r1:r0 = 0x0000000080000000
  kuna     int  0x80000000 -> r1:r0 = 0xffffffff80000000
  rules    bisected to: subvar_zext
```

Run over the corpus, it lists the functions where kuna's output provably computes
something else than the binary. It also gives the automated improvement loops a gate: a
candidate change that makes a function newly `DIFFERS` is rejected before review.

## Principles

1. **No home-grown solver or emulator.** z3 proves, QEMU (through Unicorn where
   `qemu-user` is unavailable) checks the lift, and kuna's existing emulators execute.
   kuna owns only the encoding.
2. **Dev-only.** A new workspace crate, `kuna-verify`, next to `kuna-harness`. The release
   binary does not link it and its output is byte-identical. Emitted C does not change,
   so there is no option row or DIV entry.
3. **Concrete before symbolic.** A difference found by running both sides is a replayable
   fact and needs no solver. Proof is a second step, only for functions that already
   agree.
4. **Unknown is not equivalent.** Verdicts:
   - `DIFFERS`: a concrete input that disagrees;
   - `AGREES(n)`: n runs, no difference;
   - `EQUIVALENT`: proved;
   - `UNKNOWN(reason)`;
   - `SKIPPED(reason)`.

   Only `EQUIVALENT` is a proof. `AGREES` is evidence, and the report says which one it
   is.
5. **Declared divergence.** Defaults that knowingly trade fidelity for readability are
   listed, keyed by option name. #794's x86-64 `RDX:RAX` trim is one example. A
   difference they explain is reported as `DIVERGES(option)`, not `DIFFERS`. The list
   is part of the first slice, not an afterthought.

## Tier C — run both sides (first)

**Machine side.**
- Run the function's own machine code from its entry with `EmulatePcodeCache` (SLEIGH
  p-code over the load image) until it returns.
- Inputs: a full random ABI state, with seeded edge values (0, -1, sign boundaries, shift
  counts ≥ width).
- Pointer arguments point into a mapped scratch region of the same contents on both
  sides.

**Decompiled side.**
- Run the final `Funcdata` (after the action tree, before printing) with a graph
  interpreter built on `EmulatePcodeOp`.
- The interpreter drives its own control flow, and resolves MULTIEQUAL by the incoming
  edge.
- Declared parameters get the values the ABI places in their storage. Any other input the
  body reads gets a poison value, which catches dropped parameters (#875).

**Observables.**
- The return value: the decompiled side's value is converted through its claimed C type
  to ABI storage, which catches #794.
- Stores outside the function's own frame.
- The ordered call sequence, each call with its callee and arguments.

**Calls.**
- The machine side runs a stub at each call. The stub records the call, returns a
  recorded random value in the return registers, and clobbers caller-saved registers.
- The decompiled side replays the same values call by call. INDIRECT is a copy unless the
  recorded effect wrote that storage.
- A different call sequence is itself `DIFFERS`.

**Needs no phase hooks.** The machine side reads the binary and the decompiled side reads
the finished `Funcdata`. Loops are fine: execution is bounded by a step budget, and a run
that exceeds it is `UNKNOWN`.

**Localization.** On `DIFFERS`, re-decompile with subsets of the fired rules
(`count_apply > 0`) disabled, by delta debugging over `disable_rule`, and report the
smallest set that removes the difference. Once #1699 lands, per-phase snapshots further
narrow the difference to P2, Band B or P6.

**Sees.** The body-rewrite class (~15 of 56 P3/P4 fixes) and the prototype/ABI class
(~29), per `analysis.md` §2.

**Does not see.**
- Lift bugs: the machine side shares the SLEIGH lift (Tier L).
- Type, printing, P6 merge and P8/P9 bugs (Tier P).

## Tier S — prove with z3 (second)

For functions that are `AGREES` and loop-free (or bounded by unrolling), encode the same
two sides and the same observables as SMT bit-vectors and arrays:
- calls become uninterpreted functions of (callee, arguments, memory);
- CALLOTHER becomes an uninterpreted function;
- floating point stays `UNKNOWN` until a later slice.

z3 runs as a subprocess speaking SMT-LIB 2. That keeps libz3 out of the cargo build, lets
CI `apt install z3`, and lets any verdict be cross-checked with cvc5 or bitwuzla.

- **Outcomes.**
  - `unsat` upgrades the function to `EQUIVALENT`.
  - `sat` must replay in Tier C. A model that does not replay is an encoder bug and is
    reported as one.
- **Order.** This tier reuses Tier C's observable definitions, so it is built after they
  have settled on the corpus.

## Tier L — the lift against the machine (QEMU/Unicorn)

**Compares** one instruction's effect on random state:
- kuna side: SLEIGH p-code in `EmulatePcodeCache`;
- machine side: Unicorn, which is QEMU's TCG as a library and runs on macOS, unlike
  `qemu-user`.

**Inputs.**
- Instructions come from the corpus binaries, deduplicated by mnemonic and operand form.

**Compares.** Registers, flags and touched memory.

**Driver.**
- The kuna side is a small Rust binary that emits JSON.
- The comparison is a Python driver, as `verify_arm.py` already does with Unicorn. This
  keeps Unicorn's C build out of the cargo workspace.

**Known noise.** Architecturally undefined flags and privileged instructions go into a
per-architecture skip list, not silently dropped.

This tier makes Tier C's machine side trustworthy.

## Tier P — printed C against the binary (later, separate proposal)

1. Compile one function's printed C with a generated header for kuna's
   `undefinedN`/`intN` types.
2. Link it into a harness next to the original.
3. Run both under `qemu-user` in the Linux devcontainer.

It is the only tier that sees type, P6, P8 and P9 bugs. It is also the tier most limited
by printed C that does not compile, and by cross toolchains. decbench's `byte_match`
already hits that wall (`docs/decbench-loop.md`, Trap 1).

## Slices

| slice | scope | done when |
|---|---|---|
| C1 | Tier C, leaf functions, integer only; ABI projection of returns and parameters; declared-divergence list | reverting #794 gives `DIFFERS` on its fixture; verdict table over `tests/datatests` + `tests/stages` |
| C2 | calls by record/replay; rule bisection | bisection names `subvar_zext` on reverted #794; reverting #801 gives `DIFFERS` |
| C3 | corpus run (decbench binaries), triage of every `DIFFERS` into bug / declared divergence / harness bug | published verdict table |
| S1 | Tier S for loop-free `AGREES` functions | `EQUIVALENT` counts on the same corpus; every `sat` replays |
| L1 | Tier L for x86-64, AArch64, ARM | per-arch mismatch report with skip list |
| P | separate proposal | — |

**CI.** The tool runs as a non-gating job that publishes the verdict table. It becomes a
gate only once the `DIFFERS` set is empty or every entry is a declared divergence, with a
baseline file like `docs/baseline.json`.

## Non-goals

- Proving kuna correct in general. The check is per function, against kuna's own lift
  (and, through Tier L, against the machine).
- Changing emitted C, adding options, or shipping z3 or QEMU with the release binary.
- Floating point and self-modifying code in the first slices.

## Open questions

1. Is the final `Funcdata` the right decompiled side, or should C1 already require
   #1699's Band B snapshot, which is closer to where most bugs are made?
2. How should the machine side's initial stack be modeled? One option is a fixed
   return-address sentinel, with the caller's frame as scratch memory that is identical on
   both sides.
3. Who owns the declared-divergence list: the option's `phases.toml` row, or a file in
   `kuna-verify`?
4. Does the improvement pipeline treat a new `DIFFERS` as a hard reject immediately, or as
   a reviewer signal first?
