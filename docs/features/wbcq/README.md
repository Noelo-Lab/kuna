# kuna-wbcq: the state machine of a step function, learned against its machine code

`kuna-wbcq` is a dev-only experiment. It takes a step function, `result step(ctx *, input)`,
and computes the smallest state machine that behaves like its machine code. It then
certifies that machine and checks it against the machine code.

The idea is WBCQ, from Fission RFC #1093 ("white-box coalgebraic bisimulation
quotienting"). The RFC was read for its idea only. Nothing from Fission is used: no crate,
no copied code, no build tool. The engine is built on kuna-verify's lifting, op semantics,
expressions and z3 driver.

Nothing here changes kuna's output. The release `kuna` binary does not link this crate.

## 1. Quick start

```sh
make binaries specs
(cd decompiler && cargo build --release -p kuna-wbcq)
W=decompiler/target/release/kuna-wbcq
T=decompiler/crates/kuna-wbcq/testdata

$W $T/auth_x86_64_O2.elf 0x$(llvm-nm $T/auth_x86_64_O2.elf | awk '$3=="auth_flat_step"{print $1}')
$W <elf> <0xentry> --mutate --initial 01000000b979379e --initial 02000000b979379e
```

It needs `z3` on `PATH`. The function must take the context pointer in the first argument
register and a 32-bit input in the second, and return a 32-bit result (System V x86-64 or
AAPCS64). The context is 8 bytes of zeros unless `--initial` gives it.

```
status   proved_by_path_closure
states   q0 access []          mode = 0
         q1 access [0x5a]      mode = 1
         q2 access [0x5a 0xa5] mode = 2
q0 -> q1  result 0   (in[7:0] == 0x5a)
q0 -> q0  result 0   !(in[7:0] == 0x5a)
q1 -> q2  result 1   (in[7:0] == 0xa5)
q1 -> q0  result 0   !(in[7:0] == 0xa5)
q2 -> q2  result 1   true
witnesses q0/q1 [0xa5]   q0/q2 [0x00]   q1/q2 [0x00]
control bytes 0-3 (mode), data bytes 4-7 (acc)
```

(`auth_step`, x86-64 O0: the JSON report, summarized.)

## 2. How it works

The four roles of a counterexample-guided loop, and the module that plays each:

| role | module | what it does |
|---|---|---|
| semantic check (the reference) | `machine.rs` | Runs one step of the machine code. Each instruction is lifted with kuna's SLEIGH translator (`kuna_verify::machine::lift`) and its p-code is executed (`kuna_verify::value::eval`). Every byte carries three things: its value; an expression over the input's four bytes (`kuna_verify::concolic::Expr`), when the input reaches it; and the set of sources it depends on (each context byte, the input). Anything outside the model stops the step with the reason: a call, a user op with an effect, memory outside the context, the frame and the image, an address or a jump computed from data, or a loop past the op budget. |
| candidate model | `learn.rs` | L# (Vaandrager et al., TACAS 2022). An observation tree of the words run so far, and a basis of states. Two nodes are apart only when one word, run on both, gives different results, so every basis state is a real state of the machine, with a concrete witness. |
| counterexample extraction | `engine.rs::explore` | Enumerates every path of one step from one concrete context. The input is symbolic; for each branch of each path found, z3 is asked for an input that keeps the branches before it and flips it. **Value branches** cover code with no branch (`setcc`, `cset`, `cmov`): the result and each control byte end a path with `value == v`. A value branch's query excludes every value seen after the same prefix, so a step with 2^32 results is never taken as complete. |
| refinement | `engine.rs::learn` | Three refinement moves. **New symbol:** a path, from a basis context or one symbol past it, that no alphabet symbol takes adds its input to the alphabet. **Counterexample:** a word on which the machine and the hypothesis disagree goes back to L# (Rivest–Schapire: run `access(q_i)·word[i..]` for each i). **Pins grow:** pins widen when the control bytes do. |
| certificate | `engine.rs::closure` | Path closure (below). |
| guards | `guard.rs` | A transition's guard is the disjunction of its paths' conditions. These conditions are evaluated on concrete inputs (to classify a random input) and printed. |
| mutation testing | `mutate.rs` | Plants single faults in the learned machine and counts which checks refuse them. |

### The certificate: path closure

**C** is the set of context bytes that a branch condition or the result depends on, closed
under the bytes that their next values depend on. The certificate holds if all four of
these hold:

1. From every state's context, path enumeration is complete: no path cap, no z3 `unknown`,
   and no branch on the input through an op with no expression.
2. On every path, the result and every byte of C either do not depend on the input, or are
   fixed by a value branch.
3. Every alphabet symbol's result is the hypothesis's result.
4. For every symbol, the next context agrees on C with the context of the state the
   hypothesis goes to.

Then every context that agrees with a state's context on C takes the same path as that
state on every input. It gets the same result and goes to the same state. By induction,
the hypothesis agrees with the machine on **every word of every length** from the initial
context. This is not a horizon-H bound.

The basis states are pairwise apart by concrete words, so no smaller machine exists: the
machine is minimal.

The proof rests on:
- the p-code semantics;
- the dependence tracking (an address or jump computed from data stops the step);
- z3's `unsat` answers. An `unknown` answer withholds the proof.

Random conformance runs after the certificate as a tripwire. A disagreement on a proved
machine is reported as a bug in the prototype, never silently.

### Statuses

| status | meaning |
|---|---|
| `proved_by_path_closure` | the certificate holds |
| `empirically_consistent: why` | no certificate, for the reason given; 512 random words of 8 steps (256 uniform, 256 drawn from the alphabet) agreed |
| `partial_frontier: why` | more than 256 paths in one step, or more than K_max (32) control valuations reachable |
| `unsupported: why` | the step leaves the model (a call, a global write, …) |

## 3. kuna-verify and WBCQ, component by component

| kuna-verify | role in WBCQ | taken as code | taken as design / algorithm | new here |
|---|---|---|---|---|
| `machine.rs`: lift one instruction, run its p-code | semantic check | `lift` | run the machine code itself, not a model of it | a step executor with persistent context, a fresh frame per step, and fail-closed memory |
| `value.rs`: op semantics | semantic check | `eval` | – | – |
| `concolic.rs`: byte expressions, branch recording, z3 | counterexample extraction | `Expr`, `op`, `byte`, `solve` (now public, with `unknown` kept apart from `unsat`) | "z3 picks inputs, the run decides" | full path enumeration per prefix instead of branch coverage; value branches; all-values exclusion |
| `state.rs`: lazily seeded random state | initial state | `Env` (lift cache, image) | many initial states, not one (§4.3) | concrete initial contexts given by the caller |
| `check.rs`: compare calls, memory, result | semantic check | – | compare per step | observations are results; the context is compared on its control bytes only |
| `mutate.rs`: plant faults, count catches | meta: the certificate's recall | – | the method | mutants of a learned machine (`retarget`, `result`, `guard`, `merge`) |
| `localize.rs`: first rule application that differs | refinement | – | bisection over a sequence | Rivest–Schapire over a word |
| Tier L (`step.rs`): one instruction vs Unicorn | trust in the reference | – | – | not applied; the certificate inherits any SLEIGH error |

**No counterpart in kuna-verify.** kuna-verify never proposes a candidate, so everything
that builds one is new:
- the observation tree and apartness (`learn.rs`);
- alphabet refinement from paths;
- the control/data split by dependence;
- symbolic guards;
- the path-closure certificate.

**What kuna-verify would gain.**
1. **Multi-call, state-dependent checks.** kuna-verify checks a step function one call at
   a time from a random state. A random 32-bit `mode` reaches `auth_step`'s IDLE or
   CHALLENGE code with probability about 2^-31 per run. A learned machine gives exactly
   the reachable states: each state's access word, and words built from its witnesses.
   Running kuna's IR and the machine code over such words, with the context carried
   between calls, would check the decompile in every reachable state class. This is a
   proposal; it was not built or measured here.
2. **A certificate for one class of functions.** For a step function inside this model,
   path closure over the decompiled IR (kuna-verify's `graph.rs` side) and over the machine
   code, with the same machine, would be an unbounded equivalence proof. kuna-verify's
   `AGREES(n)` is not a proof.

## 4. The experiment: RFC #1093's smallest example

The RFC's minimal test (its P0) is a three-state flattened authentication dispatcher that
should reduce to exactly 3 states with apartness witnesses. `testdata/auth.c` holds it in
two forms, both written from the RFC's prose:
- `auth_step`: the plain form;
- `auth_flat_step`: the same machine behind a flattened dispatcher, with artificial state
  numbers and the opaque predicate `(x*(x+1)) & 1` guarding a decoy that would clobber
  the mode.

`testdata/negatives.c` holds four step functions outside the model.

`build.sh` builds both files for {x86-64, AArch64} × {O0, O2} with Homebrew clang 22. The
flattening survives O2: the dispatcher compares and the opaque `(~x * x) & 1` test are in
the code. All numbers were measured on a 24 GB laptop shared with other builds.

### 4.1 The machine

| binary | function | status | states / transitions | membership queries | z3 queries (unknown) | learn time |
|---|---|---|---|---|---|---|
| x86-64 O0 | auth_step | proved | 3 / 5 | 19 | 42 (0) | 0.29 s |
| x86-64 O0 | auth_flat_step | proved | 3 / 5 | 19 | 68 (0) | 2.72 s |
| x86-64 O2 | auth_step | proved | 3 / 5 | 19 | 82 (0) | 0.21 s |
| x86-64 O2 | auth_flat_step | proved | 3 / 5 | 19 | 143 (0) | 0.73 s |
| AArch64 O0 | auth_step | proved | 3 / 5 | 19 | 42 (0) | 0.26 s |
| AArch64 O0 | auth_flat_step | proved | 3 / 5 | 19 | 68 (0) | 0.99 s |
| AArch64 O2 | auth_step | proved | 3 / 5 | 19 | 56 (0) | 0.19 s |
| AArch64 O2 | auth_flat_step | proved | 3 / 5 | 21 | 147 (0) | 0.52 s |

On all eight:
- **The same machine.** The witnesses are `[0xa5]` (q0/q1) and `[0x00]` (q0/q2, q1/q2),
  the ones the RFC names. The control bytes are 0–3 (`mode`) and the data bytes 4–7
  (`acc`), found by dependence alone.
- **Determinism.** Two runs printed byte-identical JSON.
- **Cost.** About 8,500 machine steps in all, conformance replay included. Loading the
  program takes 0.07 s.
- **Branchless O2 code.** x86-64 O2 `auth_step` and AArch64 O2 `auth_step` compute the
  next mode with `sete`/`cset` and no branch. Without value branches, the first run stopped
  at 1 state ("empirically consistent") and the second at "result depends on the input".
  Value branches fixed both.
- **The opaque predicate.** z3 refutes the decoy path: no enumerated path takes it, and no
  decoy block runs. But the predicate is still printed as a conjunct of every guard of the
  flattened form, e.g. `(bvand(bvmul(sext32(in), sext32(bvadd(in, 0x1)))[31:0], 0x1) == 0x0)
  && (in[7:0] == 0x5a)`. The guards are path conditions verbatim; simplifying them with
  the solver is not done.

### 4.2 Mutation testing of the certificate

There are 23 single-fault mutants per machine: 10 `retarget`, 5 `result`, 4 `guard` (one
input class moved to another transition) and 4 `merge`. The checks:

| check | refuses / kills, every binary |
|---|---|
| path closure | **23 / 23** |
| 256 alphabet-drawn words × 8 | **23 / 23** |
| 256 uniform 32-bit words × 8 (kuna-verify's random runs) | **10 / 23** |
| the unmutated machine | passes all three |

The uniform words miss every fault that shows only after a `0x5a` byte followed by an
`0xa5` byte (1 in 65,536 per pair). This is the multi-call gap of §3, measured.

### 4.3 Other initial contexts

| `mode` at the start (`acc` = 0x9e3779b9) | states |
|---|---|
| 0 | 3 |
| 1 | 3 (back to IDLE first) |
| 2, 3, 0xffffffff | 1 (AUTH_OK forever) |

All results are proved, on all eight builds. A minimal state count is a property of the
initial context as well as of the code.

### 4.4 Outside the model: fail closed

| function | x86-64 O2 | AArch64 O0 |
|---|---|---|
| `echo_step` (returns its input) | `partial_frontier`: more than 256 paths | same |
| `counter_step` (100 states) | `partial_frontier`: more than 32 control valuations | same |
| `global_step` (writes a global) | `unsupported`: a write outside the context | same |
| `call_step` (calls `helper`) | `partial_frontier`: more than 256 paths (O2 inlined `helper`; the result is `3·in + mode`) | `unsupported`: a call |

**These negatives found a soundness bug.** The first value-branch query excluded only the
value it had just seen. When z3 answered with a value already seen, enumeration looked
complete, and `echo_step` was certified as a 1-state machine. The random-conformance
tripwire then disagreed with the proof, and the engine reported a bug instead of a
result. The query now excludes every value seen after the same prefix. A certificate
needs an independent check to catch errors like this one.

## 5. The RFC's claims, after this experiment

"Confirmed" means: on this one machine, in eight builds of two forms, by this
implementation. It does not mean anything more.

| claim (RFC section) | verdict | evidence |
|---|---|---|
| The flattened 3-state dispatcher reduces to exactly 3 states with apartness witnesses (§5 P0, §9) | **confirmed** | 8/8; witnesses as the RFC states |
| 5 semantic transitions (§9) | **confirmed** | 8/8 |
| "11 blocks / 17 edges → 3 / 5" (§9) | **not measured** | no CFG baseline was computed here |
| The opaque predicate is removed by an `unsat` query (§9) | **half** | The decoy path is refuted, but the predicate stays in the printed guards. |
| Certification with a bounded horizon H = 4 (§7–§9) | **superseded for this class** | Path closure proves equivalence for every length, so no horizon is needed. It does not apply where the result or the control state is a non-finite function of the input (`echo_step`), and the bounded query was not built. |
| At most n SMT equivalence queries (§8) | **does not transfer** | 42–147 z3 queries for n = 3. They are path queries, not equivalence queries, so the RFC's count does not describe this design. |
| O(n²m + n log H) membership queries (§8) | **consistent, not tested** | 19–21 for n = 3. One machine says nothing about asymptotics. |
| Minimal state count (§7–§8) | **confirmed, relative to the initial context** | the apartness witnesses; 3 from mode 0/1, 1 from mode ≥ 2 |
| Control/data separation (§7) | **confirmed here** | bytes 0–3 vs 4–7 by dependence |
| SMT-backed certification is sound (§7 invariants) | **confirmed within the model, after one fix** | 23/23 mutants refused, control clean. One false proof (`echo_step`) was possible before the fix (§4.4). A SLEIGH spec error would be shared by the certificate; Tier L was not run. |
| Apartness soundness: no spurious state (§7) | **confirmed** | every witness is a concrete word on the machine code |
| Deterministic output (§8) | **confirmed** | 8/8 byte-identical |
| P1: under 10 s per function (§11) | **confirmed for these functions** | 0.19–2.72 s |
| H1: ≥ 50% complexity cut and ≥ 99% preservation on real code (§11) | **open; Fission's own measurement says no** | see §6 |
| Impact on obfuscated firmware and protocols (§13) | **open** | not measured |

## 6. Fission measured this already, and decided to pivot to AEQ

Fission built WBCQ itself (`crates/fission-quotient`), evaluated it, and wrote down a
decision. Its P3/P4 evaluation document (`docs/proposals/2026-11-23-wbcq-p3-evaluation.md`
in the Fission repository) reports three things:
- **On a written corpus,** every machine matched the native ground truth. These were 11
  protocol machines from published specifications, in 8 builds each, flattened and with
  opaque predicates.
- **On real code it did not reach.** After three model extensions (calls, pointer fields,
  globals), 87% of 186 step-shaped functions from real programs were still outside the
  model. 22 of the 24 machines it did build had a single state.
- **Its conclusion:** real code keeps its state in heap structures shared with callees,
  not in the arguments of a step function. So it would **pivot to AEQ** (algebraic laws of
  data-structure operations).

This was read as evidence; none of its code is used here.

What it implies for this attempt:

- **The mechanism is not the question.** Here, as in Fission, the mechanism is right
  wherever it answers. The open question is reach, and Fission's numbers answer it for a
  model that is *wider* than this one: it stubs calls and models pointer fields and
  globals, while this prototype refuses all three. Run on the same real-code sample, this
  engine would reach less, not more. Nothing here contradicts the pivot.
- **What remains worth having is narrow.**
  1. As a **checking component for kuna-verify**: the multi-call gap of §4.2 is real and
     measured. A learned machine's access words and witnesses are the inputs that close it
     for step-shaped functions.
  2. As a study of the **unbounded certificate**: path closure proves what a bounded
     horizon cannot, at no extra cost on this class.

  Neither is a decompiler output mode, and neither should be pitched as one.
- **AEQ.** Another kuna worktree has an AEQ design note in progress (`aeq-design.md` under
  `docs/features/diffverify/`, uncommitted). Several pieces here would carry over to it:
  the step executor (any operation on a context), the concolic path enumeration, and
  mutation testing of the certificate. L# and the path-closure certificate would not.

## 7. Limits

- It handles one 32-bit input, a context of at most 63 bytes, and a 32-bit result. The
  calling conventions are System V x86-64 and AAPCS64 only, and little-endian only.
- No calls, no globals, no pointers out of the context, no floating point. Each refuses.
- The frame is fresh at every step. A step function that keeps state in its own static
  storage is a global write, and refused.
- K_max = 32 states and 256 paths per step are hard limits.
- Guards are path conditions as z3 sees them, not simplified.
- The proof inherits the SLEIGH specs' and kuna-verify's op semantics' errors.

## 8. Files

| path | what |
|---|---|
| `decompiler/crates/kuna-wbcq/src/` | the engine (§2) |
| `decompiler/crates/kuna-wbcq/src/bin/kuna-wbcq.rs` | the CLI |
| `decompiler/crates/kuna-wbcq/testdata/` | `auth.c`, `negatives.c`, `build.sh`, the four ELFs |
| `decompiler/crates/kuna-wbcq/tests/auth.rs` | 4 positive builds must prove 3 states; 3 negatives must fail closed (skipped without z3) |
| `decompiler/crates/kuna-verify/src/concolic.rs` | `Solver::start` and `solve` public; `Answer::Unknown` apart from `Unsat` (the Worklist treats both as before) |
| `decompiler/crates/kuna-verify/src/machine.rs` | `lift` public |
