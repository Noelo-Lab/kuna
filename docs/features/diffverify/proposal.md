# [PROPOSAL] diffverify — differential semantic checking with z3, QEMU and kuna

Status: **proposal** — design only, no implementation before approval.
Evidence and motivation: `analysis.md` in this folder.

## What this makes possible

A command that says whether kuna changed what a function computes, and if so, on which
input:

```
$ kuna-verify equiv ins16.o ins16
ins16  COUNTEREXAMPLE  stage=bandB
  input   r0=0x00007fff r1=0x00000000
  lifted  return r1:r0 = 0x0000000080000000
  final   return (int) = 0x80000000 -> ABI r1:r0 = 0xffffffff80000000
  rules   bisected to: subvar_zext
```

Run over the corpus, the same command turns "a user noticed a wrong value" into a list
of functions the engine provably changed. It also gives the automated improvement loops
a gate: a candidate rule that makes any function stop being equivalent is rejected
before review.

Today, each such fix brings its own one-off oracle (84 test files compile printed C, 5
run it under `qemu-user`). None of them is ever run outside its fixture.

## Principles

1. **No home-grown solver or emulator.** z3 decides, QEMU (via Unicorn on hosts without
   `qemu-user`) executes, and kuna's own `EmulatePcodeCache` replays. kuna owns only the
   encoding.
2. **Dev-only.** Everything lives in a new workspace crate, `kuna-verify`, next to
   `kuna-harness`. The release `kuna` binary does not link it, gains no dependency, and
   prints byte-identical C. Emitted C does not change, so there is no option row and no
   DIV entry.
3. **Unknown is not equivalent.** Every function gets exactly one verdict:
   `EQUIVALENT`, `COUNTEREXAMPLE`, `UNKNOWN(reason)` or `SKIPPED(reason)`. Timeouts,
   unsupported ops and loops beyond the bound are `UNKNOWN`. Only `EQUIVALENT` counts as
   a pass.
4. **A counterexample must replay.** Every SAT model is replayed concretely: kuna's p-code
   emulator runs it on the before-graph and the after-graph, and (layer 1 and 3) QEMU runs
   it on the machine. A model that does not reproduce is reported as an encoder bug, not
   as a decompiler bug.
5. **Talk to the solver in SMT-LIB 2 text**, over a subprocess pipe. This avoids building
   libz3 in the cargo workspace, lets CI `apt install z3`, and lets any verdict be
   cross-checked with cvc5 or bitwuzla by changing one path.

## The three layers

Each layer has a different oracle and catches a different class of bug. They ship as
separate slices in this order: 2, then 1, then 3.

### Layer 2 — translation validation of the engine (z3) — first

**Compares** one function's p-code right after P2 (lifted, pre-heritage) against its
p-code after Band B (P3–P5 settled), and later after P6.

**Encoding.**
- Inputs: every register and stack slot read before it is written, plus memory as an SMT
  array.
- Body: an acyclic CFG is encoded with path predicates. MULTIEQUAL becomes an `ite` over
  the incoming edge predicates.
- Outputs, the observables:
  - the final memory array;
  - the ordered sequence of call sites, each with its arguments and the memory at the
    call;
  - the return value.
- The after-graph is read through the prototype the decompiler claims (output storage,
  size, signedness) and mapped back to ABI registers before comparison. This is what
  makes #794 visible (`analysis.md` §2).
- Calls are uninterpreted functions of (callee, arguments, memory), returning fresh
  values and a fresh memory. In slice 2a, a function with calls is `SKIPPED`.
- CALLOTHER is an uninterpreted function. Floating-point ops are `UNKNOWN` until a later
  slice uses SMT FP.

**Bisection.** On a counterexample, re-run the decompile with subsets of the rules that
fired (`count_apply > 0`) disabled, using delta debugging over `disable_rule`. The report
names the smallest set that restores equivalence.

**Snapshot points.** Uses #1699's `on_phase` callbacks if that lands. Otherwise it adds
the same two observation leaves with an empty-registry fast path. #1699 shows that path
is byte-identical and no slower on `decompile-all /usr/bin/ls`.

**Catches.** P3/P4/P5 rewrites that change values: the 31 + 24 tagged fixes in
`analysis.md`.

**Does not catch.**
- Bugs introduced in P2 itself: both sides share the lift. Layer 1 covers those.
- P6 merge, P8 structuring and P9 printing bugs: they are not visible in p-code. Layer 3
  covers those.

### Layer 1 — the lift against the machine (QEMU/Unicorn)

**Compares** the effect of one instruction on random machine state:
- kuna side: SLEIGH p-code run in `EmulatePcodeCache`;
- machine side: Unicorn (QEMU's TCG as a library; runs on macOS, unlike `qemu-user`).

**Inputs.**
- Instruction bytes come from the corpus binaries' own instruction streams, deduplicated
  by mnemonic and operand form.
- States are random, with seeded edge values (0, -1, sign boundaries, shift counts ≥
  width).

**Compares.** All architectural registers and flags, plus touched memory.

**Driver.** A small Rust binary in `kuna-verify` emits per-instruction JSON from the
kuna side. The comparison is a Python driver, the way `verify_arm.py` already uses
Unicorn. That keeps Unicorn's C build out of the cargo workspace.

**Known noise.** Some flags are architecturally undefined, and some instructions are
privileged or system-level. These are recorded per architecture in a skip list. They are
not silently dropped.

**Catches.** SLEIGH spec and lift bugs, which are part of the 32 tagged P2 fixes; flow
and jump-table bugs are not in scope.

### Layer 3 — printed C against the binary (compiler + QEMU) — later, separate proposal

1. Compile the printed C for one function with a generated header for kuna's
   `undefinedN`/`intN` types.
2. Link it into a harness that also calls the original function.
3. Run both on the same random arguments under `qemu-user` inside the Linux devcontainer
   (`docs/devcontainer.md`), and compare return values and touched memory.

The decbench corpus ships sources and binaries, so it provides functions with a known
ground truth.

This is the only layer that sees P6/P8/P9 bugs. It is also the layer most limited by
printed C that does not compile, and by cross toolchains: decbench's `byte_match` already
hits that wall (`docs/decbench-loop.md`, Trap 1). Its own proposal will follow after
layer 2 has run on the corpus.

## Slices

| slice | scope | done when |
|---|---|---|
| 2a | layer 2: leaf, loop-free, integer-only functions; P2 vs Band B; SMT-LIB pipe to z3; concrete replay | verdicts on `tests/datatests` + `tests/stages` fixtures; #794's fixture reverted gives `COUNTEREXAMPLE` |
| 2b | calls as uninterpreted functions; prototype-projected returns; rule bisection | bisection names `subvar_zext` on the reverted #794 fixture |
| 2c | loops by bounded unrolling (k configurable, beyond-bound paths → `UNKNOWN`); P6 snapshot | corpus report with per-verdict counts |
| 1  | layer 1 for x86-64, AArch64, ARM; JSON emitter + Unicorn driver | per-arch mismatch report with a skip list |
| 3  | separate proposal | — |

**CI.** `kuna-verify` runs as a non-gating job that publishes a verdict table. It becomes
a gate only after the `COUNTEREXAMPLE` baseline is empty or explained, using a baseline
file like `docs/baseline.json`.

## Non-goals

- Proving kuna correct. This validates one function per run against its own lift.
- Changing emitted C, adding options, or shipping z3/QEMU with the release binary.
- Floating-point equivalence and self-modifying code in the first slices.

## Open questions

1. Should layer 2 depend on #1699 landing, or carry its own two observation leaves?
2. How should the P2 side model unknown stack contents? One option is a symbolic array
   with the return-address slot pinned.
3. Should the verdict table key on `tests/datatests` names, or on (binary, address) pairs
   from decbench?
4. Does the improvement pipeline consume `COUNTEREXAMPLE` as a hard reject immediately,
   or as a reviewer signal first?
