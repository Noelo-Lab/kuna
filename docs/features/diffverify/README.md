# diffverify — start here

`kuna-verify` answers one question about a decompiled function: **does kuna's final IR
compute what the machine code computes?** It runs both from the same random machine
state and compares what each does: the calls it makes and their arguments, the memory it
writes outside its own frame, and the value it returns. No source code is needed. When
they differ, it prints the input that shows it and what each side did.

It is a development tool. The crate is `decompiler/crates/kuna-verify`; the release `kuna`
binary does not link it, and nothing it does changes kuna's output.

## 1. Quick start

```sh
make binaries specs
(cd decompiler && cargo build --release -p kuna-verify)
K=decompiler/target/release/kuna-verify

$K check ./prog main parse_args          # some functions of a binary
$K check ./prog --all --jobs 10          # every function
$K corpus tests/datatests tests/stages   # every function the test corpus decompiles
make verify-corpus                       # the same against docs/baseline-verify.json
```

A difference looks like this (#794's fixture, with the option that brings the bug back):

```
ins16  DIFFERS  (run 1 of 256)
  contract u64 ins16(unsigned a, unsigned short b)  (ins16.c)
  input    a0=0x00007fff (r0)  a1=0x0000007f (r1)
  machine  r1:r0 = 0x000000008000007f
  kuna     int4 ins16(int4 a0, uint4 a1) = 0x8000007f; as u64 0xffffffff8000007f
```

## 2. What it checks, and what it does not

- **Checked:** the final `Funcdata`, after every rule and action, against the function's
  own SLEIGH p-code. Calls are compared one by one: where they are made from, their
  target, their arguments, and the memory a callee could read at that moment. At the end,
  the memory written outside the frame and the return value are compared. The return
  value is compared under the calling convention, and under the real return type when a
  C prototype is given (`--contract FILE.c`).
- **Not checked: the printed C.** A fault in the C printer (a cast, an operator's
  precedence, a type's spelling) is not in the IR, so `AGREES` cannot rule it out. That
  is Tier P, a separate axis (`proposal.md`).
- **Shared semantics.** Both sides use the same p-code op semantics (`value.rs`). A wrong
  op semantic, or a wrong SLEIGH spec, is the same on both sides and invisible here. Tier
  L checks the spec and the op semantics against Unicorn, one instruction at a time
  (`tier-l-results.md`).
- **What the machine side takes from kuna.** It runs the machine code, but it still has to
  know where each call's arguments live and whether a call returns. It does not take
  kuna's word that a call does not return. It stops there only on evidence of its own:
  - the callee's name is on Ghidra's no-return lists or on the checker's own list (for
    `error` and `error_at_line`, only with a nonzero status);
  - the compiler left only padding after the call;
  - an in-image callee, run for real, reaches such a call.

  An in-image callee that, run for real, returns without going through a stub proves
  kuna wrong. A run that ends on kuna's word alone is not counted (`noreturn-unverified`).
- **How much it catches** is measured by planting faults in the IR (`mutation.md`): 76%
  of reached mutants on the test corpus, 74–75% on `ls` and `find`.

## 3. Verdicts

| verdict | meaning |
|---|---|
| `AGREES(n)` | `n` runs agreed. Runs that trapped on both sides are not counted. |
| `DIFFERS` | one run gave a difference. The report says which run and what differed. |
| `DIVERGES(id)` | the difference is one kuna makes on purpose, a declared divergence (`src/divergence.rs`): `switchrunon-cut-row`, `synthesized-call`, `gowritebarrier`, `panic-check`, `ptrauth-trap`, `copy-overlap`, `divide-by-zero`, `nan-quieting`, `syscall-result`. Each one names the option that makes the trade, and the pattern the difference has to match. Any other difference under the same option is still `DIFFERS`. A declared call leaves out only itself and what follows it in that run: the calls and memory before it are still compared. |
| `UNKNOWN(reason)` | the check could not decide. `budget`: a side ran past `--budget` ops. `unsupported-op`: an op or user op the model does not run. `trap`: only one side trapped, or both trapped in every run. `harness`: the model failed. `no switch row`: a switch value no row of kuna's table takes. `prototype-suspect`: the difference rests on a callee prototype kuna recovered and may have wrong. `uninitialized-local`: kuna's result depends on a local nothing assigned. `guessed-callee`: a callee whose effect the model had to guess. `noreturn-unverified`: every run ended at a call only kuna says does not return (§2). `timeout`: no verdict within `--fn-seconds`. |
| `SKIPPED(reason)` | reserved for Tier S. |

Exit status: 1 when a function differs, 3 when a file could not be checked, 2 for a usage
error, and 0 otherwise. With `--baseline`, it is 1 only for a regression (§6).

### Reading a report

| line | says |
|---|---|
| `input` | the run's inputs, by the parameters kuna declares and where they live |
| `machine` / `kuna` | each side's return value. `kuna` repeats the prototype it states. |
| `contract` | the C prototype used for the return value, with `--contract` |
| `call N at A to T` | a call one side makes and the other does not, or makes from elsewhere or to another target |
| `call N to T, argument i (reg)` | an argument value that differs. With a recovered prototype it is a `PROTOTYPE_SUSPECT` note, not a difference. |
| `the callee reads R; kuna's call passes nothing there` | a callee run for real reads an argument kuna's C does not pass |
| `kuna passes N arguments, and the prototype it gives the callee takes M` | a call short of its callee's declared prototype |
| `memory before call N` / `memory at exit` | the first byte that differs, and how many do |
| `kuna's result depends on ... which its C never assigns` | rerunning kuna's side with another value there changes its result: the C reads something it never set |
| `note` | not a difference: runs left out (trapped on both sides, or ended at a call only kuna says does not return), a call the machine ran itself |
| `concolic` | with `--concolic`: z3 queries, the inputs found, and whether z3 ran |
| `trace` | with `--trace`: every call of the failing run on each side |
| `PROTOTYPE_SUSPECT(...)` | kuna's prototype for this function or a callee disagrees with what the run showed. This is not a verdict. |

`--json` prints the same as one object per function (`function`, `verdict`, `label`,
`run`, `lines`, `suspects`, `ms`, and `z3` with `--concolic`). A summary line follows.

## 4. Commands

```sh
kuna-verify check <binary> <function|0xaddr>... | --all  [--jobs N] [--fn-seconds S] [--contract FILE.c] [--option NAME VALUE]...
kuna-verify corpus <test.xml|dir>...  [--jobs N] [--file-seconds S] [--baseline FILE] [--save-baseline FILE]
kuna-verify localize <test.xml|binary> <function> [--pass N] [--option NAME VALUE]...
kuna-verify step <binary> --regs R,R.. [--flags F,F..] [--states N] [--per-form N] [--limit N] [--canonical]
kuna-verify mutate <test.xml|dir>... | <binary> <function|0xaddr>... | --all  [--per-function N] [--jobs N]
common: [--runs N (256)] [--seed S] [--budget OPS (200000)] [--concolic QUERIES (0)] [--json] [--trace] [--sleighpath DIR]
```

- **`check`** loads a binary with the options `kuna decompile` turns on (the Listing, and
  the discovery passes off x86-64), so the function checked is the one the CLI prints.
  `--option` passes kuna options through. With `--jobs`, the functions run in fixed chunks
  of 32, one worker process per chunk. A function with no verdict after `--fn-seconds` is
  `UNKNOWN(timeout)`.
- **`corpus`** checks every decompile a datatest script makes, under the options that
  script sets, one worker process per file. A worker still running after `--file-seconds`
  is killed and its file reported as an error.
- **`localize`** names the rule application after which a function starts to differ, by
  holding rules back one at a time (`c3-results.md`).
- **`step`** is Tier L's kuna side: one instruction's p-code from random register values,
  as JSON. `scripts/diffverify/tier_l.py` runs it against Unicorn.
- **`mutate`** plants faults in kuna's side and reports how many the check catches
  (`mutation.md`).
- **`--concolic N`** adds up to `N` z3 queries per function, for inputs that take a branch
  the random runs did not. z3 only picks inputs; the verdict still comes from running both
  sides (`c5-results.md`).

## 5. How a check runs

1. **Decompile.** `session.rs` drives the decompiler console in process and hands each
   finished `Funcdata` to the check.
2. **Compile kuna's side.** `program.rs` turns the `Funcdata`'s blocks and ops into a
   program with value slots. `calls.rs` records, for each call, what kuna says about it:
   its target, prototype, killed registers, whether it returns, and where its arguments
   live. Both sides use this record, so a call has one effect whichever side makes it.
3. **Make the state.** `state.rs` derives every register and memory byte from the run's
   seed, lazily. It knows the image's read-only data, code sections and stack, and what a
   call may write (memory an argument pointer reaches, globals).
4. **Run the machine side.** `machine.rs` lifts the function's own instructions one at a
   time with kuna's SLEIGH translator and executes the p-code (`value.rs`, `x87.rs`). A
   callee inside the image runs for real when the model can run it. kuna's side then
   replays that callee's effect. A callee outside the image, or one the model cannot run,
   gets a deterministic stub: it kills the registers the convention says it kills, with
   values derived from the run, and may change the memory it can reach.
5. **Run kuna's side.** `graph.rs` interprets the compiled program from the same state,
   with the same stubs and replays. Where kuna's `varargsave` folded a variadic
   function's register-save prologue, the printed `va_list` setup stands for
   `va_start`, so kuna's side starts by filling the save area as `va_start` does (the
   vector registers only when `al` is nonzero). At a call that receives the `va_list`,
   the save area compared is the 176 bytes `va_start` defines, not the padding kuna's
   local may take in.
6. **Compare.** `check.rs` compares the calls in order, the memory before each call, the
   memory at exit and the return value. At a call, the frame objects compared are the
   ones a callee can reach: an argument points to one, its address was stored outside
   the frame (`global = &x`), or it was stored in an object already reachable
   (`h.p = &x; f(&h)`). It declares divergences (`divergence.rs`), and
   returns `UNKNOWN` where a difference rests on a guess. It also reruns kuna's side with
   other values in anything kuna's C reads but never assigns.
7. **Report.** `report.rs` prints the verdict as text or JSON.

| module | role |
|---|---|
| `lib.rs`, `bin/kuna-verify.rs` | entry points, worker processes, the CLI |
| `session.rs` | drive the console; the load options `kuna decompile` uses |
| `program.rs` | compile a `Funcdata` for repeated runs |
| `calls.rs` | per-call facts shared by both sides; no-return sites |
| `state.rs` | the machine state, memory model, image and stack layout |
| `machine.rs` | the machine side: lift, run, callees run for real, stubs |
| `graph.rs` | kuna's side |
| `value.rs`, `x87.rs` | p-code op semantics, x87's 80-bit format |
| `check.rs` | runs, comparisons, verdicts |
| `abi.rs`, `contract.rs`, `storage.rs` | calling conventions, C contracts, storage names |
| `divergence.rs` | declared divergences |
| `concolic.rs` | expressions, branch recording, z3 |
| `localize.rs` | rule localization |
| `mutate.rs` | mutation testing |
| `step.rs` | Tier L's kuna side |
| `baseline.rs`, `report.rs` | corpus baseline, output |

## 6. In CI

`make verify-corpus` runs `corpus` over `tests/datatests` and `tests/stages` against
`docs/baseline-verify.json`. It runs as a non-gating step of the parity-gates job.

- **A regression** is a function that now differs where it did not, a new function that
  differs, or a file that can no longer be checked. It fails the step.
- **Lost coverage** is a function that agreed and no longer does, or one no longer
  checked. It is a warning.
- **Every other change** is listed, with the `AGREES` count before and after.

Rows are paired by function and occurrence, so a file whose function list changed is
still compared. Re-record the baseline with
`make verify-corpus VERIFY_ARGS='--save-baseline docs/baseline-verify.json'`.

## 7. As a check on pull requests

The check is deterministic: its verdict comes from running the machine code, not from a
model's judgement, so it holds every pull request to the same bar whichever agent wrote
it. It has done three things review did not:

- **Found bugs in merged code.** The decbench run found seven kuna bugs (#1916–#1922)
  and three errors in the vendored SLEIGH specs (`tier-l-results.md`), all in code that
  had passed its tests and review.
- **Confirmed fixes on its own.** The stage tests #1924 and #1927 added give `DIFFERS`
  in their pass with the fix turned off and `AGREES` in the default pass, without anyone
  reading the printed C.
- **Gates regressions.** `make verify-corpus` (§6) names every function a change makes
  differ.

It is not a replacement for review. What it does not cover:

- **Recall on real bugs is lower than on planted ones.** Of 22 past P3/P4 fixes put back
  one at a time, it catches 7, and 11 with a C contract (`c4-results.md` §6). Planted
  faults are caught at 74–76% (`mutation.md`).
- **The printed C.** Casts, precedence and readability are Tier P's (§2).
- **About 11% of functions get no verdict** (`UNKNOWN`): mostly the run budget,
  unsupported ops (SIMD, user ops) and traps.
- **A `DIFFERS` still needs triage.** Most false ones so far were the harness's
  (`triage.md` §2), so someone has to rule it out before blaming kuna.
- **What a reviewer looks at.** Option rules, the design of an edge case and code
  quality are outside what running the IR can see.

**Feedback an agent can act on.** A `DIFFERS` names an input, the call or byte that
differs and what each side did; `localize` then names the rule application after which
the function first differs (`c3-results.md`). That is a place to start debugging, not
only a verdict, which is what an agent fixing its own change needs.

**What is not claimed.** `AGREES` means the runs made agreed, not that the two are
equivalent on every input. The check covers semantic correctness of the decompile only:
not speed, memory safety or maintainability of the change.

**The experiment that would settle it.** The hypothesis is that the check lowers the
semantic regressions decompiler pull requests bring in, without a stronger model. With
the same model and the same reasoning budget in every arm:

| arm | how a change is written and checked |
|---|---|
| A | an agent writes the change and an agent reviews it |
| B | A, plus the existing tests (`make test`, `make test-stages`, `make rust-test`) |
| C | B, plus `kuna-verify`: the corpus baseline, the changed functions, `localize` on each `DIFFERS` |

Measured per arm: regressions that reach main, defects found before merge, fixes that
succeed, tokens spent, and fixes a maintainer has to add after merge. The tasks can be
replayed from merged pull requests that later needed a follow-up fix. It has not been
run yet.

## 8. When a function differs

1. **Read the report** (§3), and rerun with `--trace` to see every call.
2. **Rule out the harness** before naming a kuna bug (`triage.md`). The model has been
   wrong before, about RTOS context switches, libgcc's multi-level returns, no-return
   calls and joined return registers. `triage.md` lists them.
3. **Name the rule** with `localize` when a rule rather than an action is responsible.
4. **Reduce it** to a few lines of C where you can (`clang --target=...`). Check that
   `kuna decompile` prints the same wrong C, then file it or fix it.

## 9. The other documents

| file | what it holds |
|---|---|
| `proposal.md` | the design: tiers C, S, L and P, slices, principles, prior work |
| `analysis.md` | the analysis behind the proposal |
| `c1-results.md` | slice C1: register-only leaf functions |
| `c2-results.md` | slice C2: memory and calls |
| `c3-results.md` | slice C3: rule localization |
| `c4-results.md` | slice C4 on the test corpus: precision and recall against restored bugs |
| `c4-decbench-results.md` | slice C4 on the decbench binaries: harness defects found, every `DIFFERS` sorted by cause, the issues filed |
| `c5-results.md` | slice C5: concolic inputs |
| `tier-l-results.md` | Tier L: SLEIGH p-code against Unicorn, the spec errors found |
| `triage.md` | how to decide whether a difference is the spec's, the emulator's, the checker's or the harness's |
| `mutation.md` | mutation testing: what share of planted faults the check catches |
| `record.json` | the slices' measured numbers, for scripts |
