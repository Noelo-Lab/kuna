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
   - `EQUIVALENT(≤k)`: proved for inputs that iterate each loop at most k times;
   - `UNKNOWN(reason)`, including `UNKNOWN(budget)`;
   - `SKIPPED(reason)`.

   Only `EQUIVALENT` is a proof. `AGREES` and `EQUIVALENT(≤k)` are evidence, and the
   report says which one it is.
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
- Pointer arguments point into a mapped scratch region with the same contents **and the
  same addresses** on both sides. A pointer that differs between the sides makes every
  address-dependent value differ, which is a harness false positive, not a bug. All of
  D-Helix's false positives had this cause (see Prior work).
- Globals read their real initial values from the load image, the same image on both
  sides, not zero.

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
- **Only fired rules are candidates.** The search space is the k rules that applied to
  this function, not the whole rule table, so a single culprit costs about log2 k
  re-decompiles of one function.
- **A candidate must not add a difference.** Disabling rules can break output on its own;
  D-Helix saw new errors once more than two Ghidra rules were off. A subset that removes
  the original difference but makes any other observable differ is rejected.
- **"No rule set removes it" is a result.** D-Helix could not fix 83.7% of its Ghidra
  findings by toggling rules, because the defect was in printing or core code rather than
  a rule. The report says so, which tells the reader to look outside the rule table.
- **Past culprits are tried first.** Rule sets that explained earlier differences are
  kept with a hit count and tried before the search. D-Helix's version of this sped its
  tuner up 44–278×.

**Sees.** The body-rewrite class (~15 of 56 P3/P4 fixes) and the prototype/ABI class
(~29), per `analysis.md` §2.

**Does not see.**
- Lift bugs: the machine side shares the SLEIGH lift (Tier L).
- Type, printing, P6 merge and P8/P9 bugs (Tier P).

## Tier S — prove with z3 (second)

For functions that are `AGREES`, encode the same two sides and the same observables as SMT
bit-vectors and arrays:
- calls become uninterpreted functions of (callee, arguments, memory);
- CALLOTHER becomes an uninterpreted function;
- loops are unrolled up to a bound k on both sides;
- floating point stays `UNKNOWN` until a later slice.

z3 runs as a subprocess speaking SMT-LIB 2. That keeps libz3 out of the cargo build, lets
CI `apt install z3`, and lets any verdict be cross-checked with cvc5 or bitwuzla.

**Budget.** Every proof runs under a deterministic budget instead of a structural
restriction:
- z3's `rlimit` (its resource counter, stable across runs for one z3 version);
- caps on unrolling depth and on the number of queries per function;
- wall-clock time only as a safety net, never as the measure.

A proof that exhausts the budget is `UNKNOWN(budget)`. The z3 version is pinned, so the
same function spends the same budget on every machine.

**The machine side's encoding.** kuna encodes it from the same SLEIGH p-code that Tier C
runs, not through angr or binsec:
- The `Funcdata` is p-code too, so one p-code-to-SMT encoder serves both sides. A second
  lifter would need glue between two register, memory and calling-convention models, and
  every glue bug would surface as a false `DIFFERS`.
- A `sat` model then replays exactly in Tier C. A model that does not replay is an encoder
  bug, not a disagreement between two lifters.
- It adds no toolchain and covers every architecture kuna has a SLEIGH spec for.

Independence from SLEIGH is Tier L's job, per instruction, where a lift bug is easier to
localize than inside a whole-function proof.

- **Outcomes.**
  - `unsat` on a function without loops, or whose loops the bound covers completely,
    upgrades it to `EQUIVALENT`.
  - `unsat` with a loop the bound does not cover gives `EQUIVALENT(≤k)`: equivalent for
    every input that iterates at most k times. It is reported as bounded, never as a
    proof.
  - `sat` must replay in Tier C. A model that does not replay is an encoder bug and is
    reported as one.
- **Budget as a regression signal.** The budget each function spends is recorded in the
  baseline. A function that proved within budget and now exhausts it, or whose spend grows
  past a tolerance, is flagged even though no value changed: the decompiled output got
  harder to reason about, which is itself a regression.
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
already hits that wall (`docs/decbench-loop.md`, Trap 1). D-Helix measured the wall on
Ghidra: clang compiled 24.9% of decompiled functions as printed, and its repair loop
raised that to 72.4%. Its repairs gave undefined variables default types and values,
which introduced differences of their own. A repair here therefore stays out of
values the function reads, and decompiler pseudo-operations are linked from a helper
library rather than stubbed.

## Slices

| slice | scope | done when |
|---|---|---|
| C1 | Tier C, leaf functions, integer only; ABI projection of returns and parameters; declared-divergence list | reverting #794 gives `DIFFERS` on its fixture; verdict table over `tests/datatests` + `tests/stages`, with per-function and total time |
| C2 | calls by record/replay; rule bisection | bisection names `subvar_zext` on reverted #794; reverting #801 gives `DIFFERS`; bisection cost per finding |
| C3 | corpus run (decbench binaries); precision and recall | precision: triage of every `DIFFERS` into bug / declared divergence / harness bug. Recall: of the ~44 visible P3/P4 fixes that have a fixture, how many give `DIFFERS` when reverted. Published table with both |
| S1 | Tier S for loop-free `AGREES` functions, under the budget | `EQUIVALENT` and `UNKNOWN(budget)` counts on the same corpus; every `sat` replays |
| S2 | bounded loops; budget baseline and budget-regression flag | `EQUIVALENT(≤k)` counts; a deliberately bloating rule change is flagged by budget alone |
| L1 | Tier L for x86-64, AArch64, ARM | per-arch mismatch report with skip list |
| P | separate proposal | — |

**CI.** The tool runs as a non-gating job that publishes the verdict table. It becomes a
gate only once the `DIFFERS` set is empty or every entry is a declared divergence, with a
baseline file like `docs/baseline.json`.

## Cost

Nothing has been measured yet; C1 measures it. What the design expects, and why:

- **Tier C** costs one decompile, which a check needs anyway, plus n concrete runs of
  one function (256 by default), each capped by a step budget. There is no solver and
  no compiler in the loop. The target is that a Tier C pass over `tests/datatests` and
  `tests/stages` fits in a CI job measured in minutes.
- **Localization** runs only on `DIFFERS`, at about log2 k re-decompiles of one function
  for a single culprit (k = the rules that fired on it).
- **Tier S** is bounded per function by the budget in its section, so its total is
  bounded by the corpus size.
- **For scale:** D-Helix gave symbolic execution two minutes per function on a 112-core
  server with 1 TB of memory, left out binaries over 30 MiB because they took days, and
  let its tuner run up to two weeks on one function. Tier C and fired-rule bisection are
  meant to avoid both costs; C1 and C2 report whether they do.

kuna-verify is a development tool. It is never on the decompile path, so its cost never
reaches a user's decompile.

## Prior work

**D-Helix** (Zou et al., USENIX Security 2024,
<https://www.usenix.org/system/files/usenixsecurity24-zou.pdf>) is the closest prior
design.
- It recompiles each decompiled function, symbolically executes the result and the
  lifted P-code, and asks z3 whether the return values can differ.
- A tuner toggles rules to find the culprit. It found 17 new bugs in Ghidra and angr,
  and the tuner fixed 7 automatically by toggling rules.
- One of them is the same rule family as #794: `RuleIntLessEqual` together with
  `RuleSubvarSext` turned `x < 128` into `x < -128`.

This proposal keeps its core idea (the lift is ground truth, and rule toggling localizes)
and differs where D-Helix reported its losses:

| | D-Helix | this proposal |
|---|---|---|
| decompiled side | recompiled C; 27.6% of Ghidra functions lost to compile failures | the final `Funcdata`, run directly (Tier C); recompiling is Tier P only |
| observables | the return value | return value, stores outside the frame, the ordered calls with their arguments (#801, #893) |
| prototype | read through the recovered prototype; all of its false negatives came from that | machine side read by the ABI, kuna's side through its claimed prototype; undeclared inputs read poison |
| calls | modeled as returning the sum of the arguments' low bytes | recorded and replayed concretely; uninterpreted functions in Tier S |
| loops | after a timeout, each branch at most twice, then a dummy return | step budget, `UNKNOWN` beyond it; Tier S reports `EQUIVALENT(≤k)` as bounded |
| globals | zero | the load image's initial values |
| culprit search | combinations of 1, then 2, of all 151 rules; up to two weeks per function | delta debugging over the rules that fired on the function |
| lifter | trusted | checked separately (Tier L) |

**MeanDiff** compares lifters per instruction with symbolic formulas, which is the shape
of Tier L. **Alive2** checks LLVM optimizations with SMT and gives inputs `poison` and
`undef` values, the same idea as Tier C's poison input.

## Related work in this repository

No earlier issue or PR proposes a standing semantic check. These overlap in part:

- **#1534 (Pushan, open).** It brings a symbolic p-code emulator over a hash-consed
  expression arena into `p2_lift/`. That emulator decides guards by case-splitting boolean
  atoms and has no SMT solver, so it cannot prove two functions equivalent, and Tier S
  still needs z3. If #1534 lands, its expression arena is a candidate for Tier S's
  encoding side, before the SMT-LIB printer.
- **#1641 / #1699 (open).** Phase hooks. Tier C does not need them; they would localize
  a `DIFFERS` to P2, Band B or P6 (open question 1).
- **#317 (merged).** The ghidra-sim harness diffs kuna's **text** against the Ghidra GUI
  path. It checks fidelity to Ghidra, not what the code computes.
- **Per-PR compile-and-compare oracles** (#677, #773, #942, #974, #1009, #1154 and others).
  Each compiles the printed C for its own fixture and compares it with the source. Tier P
  generalizes them; Tier C covers much of the same ground without a compiler.
- **DecBench `byte_match`.** It recompiles printed C and scores byte similarity. A high
  score does not show that the function computes the same values.

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
5. What tolerance on budget growth counts as a regression, given that `rlimit` is
   deterministic for one z3 version but shifts when z3 is upgraded?
