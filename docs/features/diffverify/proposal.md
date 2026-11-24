# [PROPOSAL] diffverify — differential semantic checking with z3, QEMU and kuna

Status: **proposal**. Slices C1, C2, C3 and C4 (on the corpus kuna has) are implemented in
`decompiler/crates/kuna-verify`. Their measurements and the design changes they forced
are in `c1-results.md`, `c2-results.md`, `c3-results.md` and `c4-results.md`.
Evidence, coverage estimates and the hard part: `analysis.md` in this folder.

## What this makes possible

A command that says whether kuna changed what a function computes, and on which input:

```
$ kuna-verify check ins16.o ins16 --contract ins16.c     # planned output
ins16  DIFFERS  (run 3 of 256)
  contract u64 ins16(unsigned a, unsigned short b)       (ins16.c)
  input    a=0x00007fff b=0x0000                         (r0, r1)
  machine  r1:r0 = 0x0000000080000000
  kuna     int ins16(int a0, unsigned int a1) = 0x80000000; as u64 0xffffffff80000000
  rules    candidate culprits: subvar_zext
```

The verdict on a prototype needs a contract: the function's real type, from the fixture's
source here. Without one, `int` in `r0` and a `u64` in `r1:r0` are both consistent with
the machine, so the check compares only the storage both readings share, and that agrees.

Run over the corpus, it lists the functions where kuna's output provably computes
something else than the binary. It also gives the automated improvement loops a gate: a
candidate change that makes a function newly `DIFFERS` is rejected before review.

## Principles

1. **No home-grown solver or machine emulator.** z3 proves and finds inputs (C5), QEMU
   (through Unicorn where `qemu-user` is unavailable) checks the lift, and kuna's SLEIGH
   translator and op semantics run the machine code. kuna owns the encoding, the small
   symbolic executor that builds it (C5), and an interpreter for its own `Funcdata`
   (Tier C).
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
   - `EQUIVALENT(≤k)`: proved for the inputs on which both sides finish within k
     iterations of each loop (Tier S);
   - `UNKNOWN(reason)`, including `UNKNOWN(budget)` and `UNKNOWN(unsupported-op)`;
   - `SKIPPED(reason)`.

   Only `EQUIVALENT` is a proof. `AGREES` and `EQUIVALENT(≤k)` are evidence, and the
   report says which one it is. Separately from the verdict, `PROTOTYPE_SUSPECT(detail)`
   records a prototype claim the binary alone cannot settle (Tier C, Contracts).
5. **Declared divergence.** Defaults that knowingly trade fidelity for readability are
   listed. An entry is a predicate, not an option name alone: the option, the
   architecture, the observable it changes, and the pattern of the difference. #794's
   x86-64 `RDX:RAX` trim is one entry: option `returnpair`, x86-64, the `RDX` half of a
   16-byte return, machine `RDX = 0`. A difference that matches an entry is
   `DIVERGES(entry)`; a different difference under the same option is still `DIFFERS`.
   C1 found a second entry, `switchrunon-cut-row` (`c1-results.md` §3).
   Each entry counts its matches, so a sudden rise is visible. The list is part of the
   first slice, not an afterthought.

## Tier C — run both sides (first)

**Machine side.**
- Run the function's own machine code from its entry until it returns. Each instruction
  is lifted by the program's SLEIGH translator, and its ops are evaluated with the
  `OpBehavior` table `EmulatePcodeCache` uses (`c1-results.md` §6 says why not the engine
  itself).
- Inputs: a full random machine state, with seeded edge values (0, -1, sign boundaries,
  shift counts ≥ width). Both sides start from this same raw state: the same register,
  stack and memory values.
- Argument registers keep the extension the ABI documents for the claimed parameters
  (`narrowext abi`): a caller cannot pass a 32-bit `int` in a 64-bit RISC-V register
  without sign-extending it, so no input does.
- Pointer arguments point into a mapped scratch region with the same contents **and the
  same addresses** on both sides. A pointer that differs between the sides makes every
  address-dependent value differ, which is a harness false positive, not a bug. All of
  D-Helix's false positives had this cause (see Prior work).
- Globals read their real initial values from the load image, the same image on both
  sides, not zero.

**Decompiled side.**
- Run the final `Funcdata` (after the action tree, before printing) with a new graph
  interpreter. It is new work, not a reuse: `EmulateFunction`
  (`p2_lift/kuna_emulatefunction.rs`) is a jump-table evaluator whose `execute_store` does
  nothing, whose calls fall through, and whose branches are errors. The interpreter
  reuses only its per-op arithmetic, and adds the CFG walk, MULTIEQUAL by incoming edge,
  memory, call effects and register aliasing.
- An op the interpreter does not support ends the run as `UNKNOWN(unsupported-op)`. It is
  never approximated.
- Every input varnode reads its storage from the same raw state the machine side starts
  from, declared in the prototype or not.

**Contracts.** A prototype claim is checked only against an independent statement of the
function's type, because the binary alone cannot settle it:
- **Physical ABI (always).** Compare the bytes the calling convention defines for the
  claimed return type, and only those. A claimed `int` on 32-bit ARM is `r0`; whatever
  the machine leaves in `r1` is not compared.
- **Typed contract (when one exists).** From a fixture's source, from DWARF in the
  binary, or later from how callers in the same binary use the result. kuna's value is
  converted to the contract's type by C's rules and compared with the machine's value
  read at the contract's width. This is what makes #794 a `DIFFERS`: the contract is
  `u64` in `r1:r0`, and kuna's `int 0x80000000` reads as `0xffffffff80000000`.
- **Without a contract**, a claim the machine's behavior makes doubtful is reported as
  `PROTOTYPE_SUSPECT`, not `DIFFERS`. Examples: the body reads a register no declared
  parameter covers (#875's shape), or the machine writes the upper return register on
  every path to the return. A custom ABI or a platform register produces the same shape
  legitimately.

**Observables.**
- The return value, under the contracts above.
- Stores outside the function's own frame.
- The ordered call sequence, each call with its callee and arguments.

**Calls.** As built in C2 (`c2-results.md` §1):
- Both sides run the same stub at each call. It records the call with kuna's arguments
  and the memory the callee could read, kills the registers the model lists as
  killed-by-call and every output register with values fixed by the run and the call's
  index, and pops `extrapop`.
- It may write the image's writable data and 4 KiB past every pointer passed to a call
  so far, lazily, with bytes fixed by the call's index. That is no more than kuna itself
  treats as changed by a call, so a value kuna keeps across one stays equal unless kuna
  is wrong to keep it. Writing all memory outside the stack is too much: it rewrites the
  stack canary, which kuna rightly keeps.
  Since C4, a pointer into the frame reaches only the local object kuna's symbol gives
  it. That object is compared at the call, as the bytes outside the stack are
  (`c4-results.md` §5).
- Without the memory writes, a rule that reuses a value loaded before the call as if the
  call could not change it would agree with the machine and be missed.
- INDIRECT is moved after the op it guards. Its output is the call's value for a killed
  register, what memory holds for a stack or memory location, and its input otherwise.
- Calls are compared in order on the call instruction's address, and a computed target
  when both sides compute it. A different call sequence is itself `DIFFERS`. An argument
  is `DIFFERS` against a declared callee prototype and `PROTOTYPE_SUSPECT` against a
  recovered one.
- Where kuna has no call at an instruction that calls code in the image (inlined,
  injected or dropped), the machine runs the callee for real, so a deliberate removal
  agrees and a wrong one shows in what the callee does.
- Since C4 (`c4-results.md` §4), a call or tail call to code in the image runs for real
  on the machine side, and kuna's side replays the effect it recorded. Such callees share
  a step pool of their own, so they never spend the caller's budget. A callee the model
  cannot run, or one that exhausts the pool, gets the stub.

**Needs no phase hooks.** The machine side reads the binary and the decompiled side reads
the finished `Funcdata`. Loops are fine: execution is bounded by a step budget, and a run
that exceeds it is `UNKNOWN`.

**Localization.** On `DIFFERS`, re-decompile with subsets of the fired rules
(`count_apply > 0`) disabled, by delta debugging over `disable_rule`, and report the
subset it ends on as **candidate culprits**. That subset is 1-minimal (re-enabling any
one of its rules brings the difference back), not the smallest possible. Disabling a
rule changes which other rules fire and in what order, so the search is not monotone, and
the report says it is a candidate set. Once phase hooks land (#1641), per-phase snapshots
further narrow the difference to P2, Band B or P6.
- **Only fired rules are candidates.** The search space is the k rules that applied to
  this function, not the whole rule table. When one rule alone is the cause and disabling
  others does not mask it, that takes about log2 k re-decompiles of one function. Other
  cases cost more, up to quadratic in k for delta debugging.
- **A candidate must not add a difference.** Disabling rules can break output on its own;
  D-Helix saw new errors once more than two Ghidra rules were off. A subset that removes
  the original difference but makes any other observable differ is rejected.
- **"No rule set removes it" is a result.** D-Helix could not fix 83.7% of its Ghidra
  findings by toggling rules, because the defect was in printing or core code rather than
  a rule. The report says so, which tells the reader to look outside the rule table.
- **Past culprits are tried first.** Rule sets that explained earlier differences are
  kept with a hit count and tried before the search. D-Helix's version of this sped its
  tuner up 44–278×.

**Inputs that reach rare paths (slice C5).** Random inputs rarely take a branch that
depends on one value, such as `x == 0xDEADBEEF`, so a difference behind it stays hidden
after 256 runs. A small symbolic executor in `kuna-verify` closes that gap concolically:
- Both sides run concretely as before, and each register value also carries an expression
  over the inputs. Every branch taken, on either side, records its condition.
- Negating one recorded condition and asking z3 for a model gives an input that takes the
  other side of that branch. Tier C runs again on it.
- Verdicts still come only from concrete runs. A wrong expression costs a wasted input,
  never a false verdict, and every `DIFFERS` stays replayable.
- Expressions cover integer ops. An op without one makes its value concrete for the rest
  of that path. Paths per function and z3 queries per function are capped.
- The expression form is the one Tier S encodes, so S1 builds on it. If #1534's expression
  arena lands, it is the candidate representation.
- It never decides equivalence by rewriting two expressions to the same shape. Those
  rewrites would be the same kind as kuna's rules, and could share their bugs.

**Sees.** The body-rewrite class (~15 of 56 P3/P4 fixes) directly. The prototype/ABI
class (~29) as `DIFFERS` only where a typed contract exists (fixtures with source, DWARF),
and otherwise at most as `PROTOTYPE_SUSPECT`. Per `analysis.md` §2.

**Does not see.**
- Lift bugs: the machine side shares the SLEIGH lift (Tier L).
- Type, printing, P6 merge and P8/P9 bugs (Tier P).

## Tier S — prove with z3 (second)

For functions that are `AGREES`, encode the same two sides and the same observables as SMT
bit-vectors and arrays:
- a call is an uninterpreted function from (callee, argument registers, memory) to
  (return registers, memory). The memory it returns may differ from its input anywhere
  the call can reach, the same set Tier C's stub writes, so a value kept across a call
  is not assumed unchanged;
- CALLOTHER is an uninterpreted function in the same way;
- loops are unrolled up to a bound k on both sides;
- floating point stays `UNKNOWN` until a later slice.

**Entry condition: static, over every path.** Tier C's runs sample paths, so a function
can be `AGREES(256)` while an unsupported op sits on a path no run took (a branch on
`x == 0xDEADBEEF`). Admission to Tier S therefore does not rest on those runs. Before
encoding, every op reachable from the entry in **both** CFGs is checked:
- the machine side's lifted instructions;
- every op of the `Funcdata`.

Each op must have complete SMT semantics, and every call's reachable memory must be
bounded. One that does not makes the function `SKIPPED(unencodable: <op or call>)`, and
it is never proved.

Checking every op of a CFG only covers the paths that CFG has, so both CFGs must also be
complete. On both sides, every reachable control-flow target must be resolved or
conservatively over-approximated:
- a `BRANCHIND` or `CALLIND`;
- a jump table, whose entries count only when its index bound is proved by its guard;
- a call kuna treats as no-return, which counts only when the callee is proved not to
  return.

Where completeness cannot be established, the function is `SKIPPED(incomplete-cfg)`. An incomplete memory or call model can therefore never yield
`EQUIVALENT`, whichever paths the concrete runs happened to cover.

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
- A `sat` model then replays in Tier C on the same lift, so a failed replay cannot be a
  disagreement between two lifters (Outcomes says what it can be).
- It adds no toolchain and covers every architecture kuna has a SLEIGH spec for.

Independence from SLEIGH is Tier L's job, per instruction, where a lift bug is easier to
localize than inside a whole-function proof.

- **Outcomes.**
  - `unsat` on a function without loops, or whose loops the bound covers completely,
    upgrades it to `EQUIVALENT`.
  - `unsat` with a loop the bound does not cover gives `EQUIVALENT(≤k)`. Each side's
    encoding carries an explicit flag for "this path exceeded the bound", and the claim
    is equivalence on the inputs for which **both** sides finish within it. It is
    reported as bounded, never as a proof.
  - An input on which one side finishes within the bound and the other does not is not
    excluded: it is a difference candidate and is replayed in Tier C with a larger step
    budget.
  - `sat` must replay in Tier C. The model's choices for each call (return values and
    memory writes) become the stubs' recorded effects. A model that does not replay is
    reported as either an encoder bug, or `UNREPLAYABLE(call-effect)` when it relies on a
    call effect the stub cannot perform. The two are never merged.
- **Budget as a regression signal.** The budget each function spends is recorded in the
  baseline. A function that proved within budget and now exhausts it, or whose spend grows
  past a tolerance, is flagged even though no value changed: the decompiled output got
  harder to reason about, which is itself a regression.
- **Order.** This tier reuses Tier C's observable definitions, so it is built after they
  have settled on the corpus.

## Tier L — the lift against the machine (QEMU/Unicorn)

**Compares** one instruction's effect on random state:
- kuna side: the SLEIGH p-code and op semantics Tier C's machine side runs;
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

Until this tier exists, `AGREES` says the final IR computes what the machine code
computes, not that the printed C does: a printer fault (a cast, an operator's precedence,
a type's spelling) is not in the IR. Mutation recall (`mutation.md`) measures IR faults
only, and the printed C is a separate axis.

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
| C1 | Tier C on register-only leaf functions: no calls and no memory access at all, except loads from read-only image data; same raw state on both sides; physical-ABI and typed-contract return checks; `PROTOTYPE_SUSPECT`; `UNKNOWN(unsupported-op)`; declared-divergence predicates | reverting #794 gives `DIFFERS` against its fixture's source contract; verdict table over `tests/datatests` + `tests/stages`, with per-function and total time. **Done** (`c1-results.md`) |
| C2 | memory and call effects: stack, pointer-argument and global memory; call stubs that write reachable memory, globals included | reverting #801 gives `DIFFERS`; a rule that keeps a pre-call load across a call that writes it gives `DIFFERS`. **Done** (`c2-results.md`; both by a fault injected in a local build) |
| C3 | rule localization over the C1/C2 verdicts | candidate culprits include `subvar_zext` on reverted #794; cost per finding. **Done** (`c3-results.md`: by holding back one rule at a time rather than reverting; 15 of 113 corpus `DIFFERS` named down to a rule application, the rest are action-level) |
| C4 | corpus run (decbench binaries); precision and recall | precision: triage of every `DIFFERS` into bug / declared divergence / harness bug. Recall: of the ~44 visible P3/P4 fixes that have a fixture, how many give `DIFFERS` when reverted. Published table with both. **Precision and recall done on `tests/datatests` + `tests/stages`** (`c4-results.md`: 7 of 22 restored bugs caught without a contract, 11 with one). **decbench done** (`c4-decbench-results.md`: 28,233 functions of 66 binaries, 80% `AGREES`; 243 `DIFFERS` triaged by class, five new kuna bugs) |
| C5 | concolic inputs: expressions over the inputs on both sides, branch conditions negated through z3, capped paths and queries | a fixture whose difference appears only on one 32-bit value gives `DIFFERS`, where 256 random runs give `AGREES`; new paths reached per function on the corpus **Done** (`c5-results.md`: `--concolic N`; a value-specific fault caught that 256 random runs miss; on decbench, concolic runs in 49 of 250 sample functions and 105 of 405 `ls` functions) |
| S1 | Tier S for loop-free `AGREES` functions that pass the static entry condition, under the budget | `EQUIVALENT` and `UNKNOWN(budget)` counts on the same corpus; every `sat` replays or is classified |
| S2 | bounded loops; budget baseline and budget-regression flag | `EQUIVALENT(≤k)` counts; a deliberately bloating rule change is flagged by budget alone |
| L1 | Tier L for x86-64, AArch64, ARM | per-arch mismatch report with skip list. **Done** (`tier-l-results.md`: 8,805 instruction forms against Unicorn; x86-64 differs only in the `AF` the spec does not compute; three SLEIGH spec errors on AArch64 and Thumb) |
| M | mutation testing of the check: one semantic fault planted at a time in kuna's side of a function that agrees | recall per fault kind, the faults reached counted apart. **Done** (`mutation.md`: 77% of reached mutants caught on the corpus, 79% on `ls` and `find`; three gaps it showed in the check are fixed) |
| P | separate proposal | — |

C5 comes after C2 and before S1: most of what C1 skips needs C2's memory and call model
first, and S1 reuses C5's expressions.

**CI.** The tool runs as a non-gating job that publishes the verdict table. It becomes a
gate only once the `DIFFERS` set is empty or every entry is a declared divergence, with a
baseline file like `docs/baseline.json`.

That job exists: `make verify-corpus`, and a `continue-on-error` step of the parity-gates
job in `.github/workflows/tests.yml`. Each function's verdict class over
`tests/datatests` + `tests/stages` is pinned in `docs/baseline-verify.json`:
- `AGREES`, `DIFFERS`, `DIVERGES(id)` or `UNKNOWN(reason)`;
- no run count and no address, so the file changes only when a verdict does.

A function that now differs, or a file that can no longer be checked, is an error
annotation, and `make verify-corpus` exits 1. Every other change is a notice. The step
uploads the verdict table and its own recording of the baseline. Re-record:
`make verify-corpus VERIFY_ARGS='--save-baseline docs/baseline-verify.json'`.

Each file is checked in a worker process of its own. A process that checked
`kuna-ppc32picimports-abi` or `-reversed-save` before `kuna-ppc32picimports` decompiles
`check` differently there: as `__stack_chk_fail();` instead of `sub_1800(); return 7;`.
So verdicts that depended on how files were spread over workers now do not.

## Cost

C1 measured Tier C (`c1-results.md` §5): 3.6 s of checking for 3,979 functions over both
corpora, and 20.6 s of wall time including every decompile, on 14 worker processes.
What the design expects for the rest, and why:

- **Tier C** costs one decompile, which a check needs anyway, plus n concrete runs of
  one function (256 by default), each capped by a step budget. There is no solver and
  no compiler in the loop. The target is that a Tier C pass over `tests/datatests` and
  `tests/stages` fits in a CI job measured in minutes.
- **Localization** runs only on `DIFFERS`, at about log2 k re-decompiles of one function
  when a single rule is the cause (k = the rules that fired on it), more otherwise.
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
| prototype | read through the recovered prototype; all of its false negatives came from that | checked against a typed contract (source, DWARF) where one exists; otherwise only the physical ABI, with doubtful claims reported as `PROTOTYPE_SUSPECT` |
| calls | modeled as returning the sum of the arguments' low bytes | recorded and replayed concretely; uninterpreted functions in Tier S |
| loops | after a timeout, each branch at most twice, then a dummy return | step budget, `UNKNOWN` beyond it; Tier S reports `EQUIVALENT(≤k)` as bounded |
| globals | zero | the load image's initial values |
| culprit search | combinations of 1, then 2, of all 151 rules; up to two weeks per function | delta debugging over the rules that fired on the function |
| lifter | trusted | checked separately (Tier L) |

**MeanDiff** compares lifters per instruction with symbolic formulas, which is the shape
of Tier L.

## Related work in this repository

No earlier issue or PR proposes a standing semantic check. These overlap in part:

- **#1534 (Pushan, open).** It brings a symbolic p-code emulator over a hash-consed
  expression arena into `p2_lift/`. That emulator decides guards by case-splitting boolean
  atoms and has no SMT solver, so it cannot prove two functions equivalent, and Tier S
  still needs z3. If #1534 lands, its expression arena is a candidate for Tier S's
  encoding side, before the SMT-LIB printer.
- **#1641 (open issue) / #1699 (draft, closed unmerged).** Phase hooks. Tier C does not
  need them; they would localize a `DIFFERS` to P2, Band B or P6 (open question 1).
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
   a Band B snapshot from phase hooks (#1641), which is closer to where most bugs are
   made?
2. How should the machine side's initial stack be modeled? One option is a fixed
   return-address sentinel, with the caller's frame as scratch memory that is identical on
   both sides.
3. Who owns the declared-divergence list: the option's `phases.toml` row, or a file in
   `kuna-verify`?
4. Does the improvement pipeline treat a new `DIFFERS` as a hard reject immediately, or as
   a reviewer signal first?
5. What tolerance on budget growth counts as a regression, given that `rlimit` is
   deterministic for one z3 version but shifts when z3 is upgraded?
