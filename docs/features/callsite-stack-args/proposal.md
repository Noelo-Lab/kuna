# callsite-stack-args — proposal

**Size: SMALL.** One argument in one predicate, plus option scaffolding, one two-pass stage
test, and one spec paragraph. This is *not* infrastructure work. The investigation was large;
the change is not.

**Recommendation: approve, implement as ONE gated PR, default-ON.**

> **STATUS: implemented on this branch.** The proposal was approved and Increment 1 is in
> the diff. Everything below is the design as approved; where implementation changed a
> conclusion it says so inline (the speed measurement in §2.4 is the one that moved).

---

## 1. What is broken

`check_input_trial_use` decides whether a call's stack-parameter trial is a real argument. For
the "is this stack slot inside the caller's own frame?" test it passes the trial's
**callee-relative** address where upstream passes the argument Varnode's **caller-relative**
address. On any downward-growing stack the callee-relative offsets are always positive and the
caller's `localrange` is negative, so *every* stack-parameter trial at *every* unlocked call
site is scored `no-use` — and, being definitely-unused, has its CALL input replaced by constant
`0`, which hands the producing computation to dead-code elimination.

Consequences: call sites truncate at the register budget (x86-64: 6; i386: 0), and any
computation whose only consumer was a dropped argument is deleted — including whole loops
(`msg_b64enc`) and whole `if` cascades (`parse_dest_constraint`).

Evidence, instrumented mechanism, upstream diff, corpus numbers and the measured ablation are
in `analysis.md`. Headline numbers, all measured on a scratch build of the candidate fix:

- **675/675 datatest assertions unchanged** (PARITY OK); one stage assertion moves, and that
  movement is itself a correctness improvement.
- Speed: within noise on x86-64 and ARM, but **+6.2% to +8.2% on i386**, over the +5% budget (corrected numbers in `analysis.md` §6; the original "no detectable delta" was a bad measurement).
- Both originating decbench cases fully recovered; three minimal witnesses confirm the mechanism.
- 30-binary stratified corpus sweep, re-measured post-rebase with the shipped option:
  `calls_ge7_args` 7 -> 301 (43x), i386 empty-argument-list fraction 42.5% -> 2.5%,
  `short_calls` -35%, and **zero** failures, hangs or stderr differences attributable to the fix.

## 2. The design

### 2.1 Mechanism

`decompiler/crates/kuna-decomp/src/p4_calls/funcdata_callsite.rs`, `check_input_trial_use`,
the `is_spacebase` arm (line 116 at HEAD):

```rust
// before
} else if !data.get_func_proto().get_local_range().in_range(&trial_addr, 1) {
// after  (= fspec.cc:5618, `inRange(vn->getAddr(),1)`)
} else if !data.get_func_proto().get_local_range()
        .in_range(&Address::new(vn_space.clone(), vn_offset), 1) {
```

`vn_space`/`vn_offset` are already destructured a few lines above (lines 88-99) for the
`has_local_alias` call, which upstream also keys on the Varnode — so the correct value is
literally already in scope. The sibling `callee_pop` branch keeps `trial_addr`, matching
`fspec.cc:5621`. The stale comment at lines 110-112 ("the *callee's* local range") is corrected
to "the caller's".

Nothing else changes. No new module, no new pass, no new action, no registration-order change.

### 2.2 Owning module and spec chapter

- Code: `decompiler/crates/kuna-decomp/src/p4_calls/funcdata_callsite.rs` (P4, calls).
- Spec: `docs/spec/04-calls-and-prototypes.md`, section *"`ActionActiveParam` — does this
  argument exist?"*. That chapter **already describes the correct behaviour** ("outside the
  caller's local stack range"), so the spec edit is a short prose addition recording that stack
  arguments are recovered at unlocked call sites, what the caller-relative range test means, and
  the over-recovery caveat — not a rewrite.

### 2.3 Ordering constraints (why nothing needs rescheduling)

The pipeline dependency chain is already correct and the fix sits at its end:

1. `ActionFuncLink::func_link_input` (P4) creates the spacebase placeholder LOAD on every
   unlocked call (`fc.proto().get_spacebase()` is `Some`).
2. `ActionStackPtrFlow` (group `stackptrflow`) normalises the stack pointer so the placeholder
   LOAD's pointer becomes spacebase+constant.
3. `RuleLoadVarnode` (group `stackvars`) converts it to a COPY and calls
   `resolve_spacebase_relative`, setting `FuncCallSpecs::stackoffset`, then
   `abort_spacebase_relative` frees the placeholder slot.
4. The **stack space** heritage pass (delayed by the space's deadcode-delay, which is why it
   runs *after* step 3) reaches `Heritage::guard_calls`, which now has a resolved `stackoffset`,
   so `tryregister` is true, and registers the stack trials + appends the argument Varnodes.
5. `ActionActiveParam` -> `check_input_trial_use` scores them. **This is the only place the fix
   touches.**
6. `build_input_from_trials` rewrites the CALL's inputs, translating stack trials back by
   `off = stackoffset + off` and calling `scope_local_mark_not_mapped` on those ranges.

One scheduling subtlety worth flagging to the implementer (it is *not* changed by the fix, but
it constrains any future rework): `ParamActive::free_placeholder_slot` sets `maxpass = 0`, so
after the placeholder resolves there is exactly **one** further `ActionActiveParam` pass. The
stack trials from step 4 are scored in that single pass or never. This is upstream-faithful
(`fspec.cc:1995-2005`) and the observed traces confirm the trials do land in time, but it means
anything that delays stack-space heritage by one more mainloop iteration would silently
re-break the feature. The stage test in section 4 is the regression guard for that.

Interaction with dead-code elimination: the fix *removes* an aggressive DCE trigger (the
`is_definitely_not_used` -> `op_set_input(const 0)` dataflow-freeing) for accepted stack trials.
It never adds one. Interaction with P6 variable mapping: `scope_local_mark_not_mapped` now
un-maps the outgoing-argument stack ranges it should always have un-mapped, which is why the
`// stack - 0x40`-commented pseudo-locals visible under `--mode fast` in the ssh-add triage
disappear and become arguments.

### 2.4 Option or no option?

The repo rule cuts both ways and a human should decide; here is the case for each.

**No option (pure bug fix).** CLAUDE.md: *"a strict bug fix that only corrects wrong output
needs no flag."* This restores upstream Ghidra behaviour, matches kuna's own normative spec
chapter, and its output changes are strictly-more-correct wherever inspected (the SPARC
struct-return pointer, the mydoom loop, the ssh-add ternaries). 675/675 datatests are
unaffected.

**Gated (recommended).** The same rule's other half: *"Anything that can change emitted C ships
behind a named option"*, and *"when in doubt, gate it"*. This changes emitted C at a large
fraction of call sites in every stripped binary on every architecture — the broadest single
output change the project has shipped. kuna's product thesis is that decision points are
flippable and discoverable from `kuna catalog`; a change this wide with a known artifact class
(over-recovery on unprototyped `__cdecl` callees, section 3) should have an ablation handle that
an agent can find from `symptoms` alone.

Recommendation: **option `callsitestackargs`, `tier = core`, `change_kind = correctness-fix`,
default-ON.** No DIV row is required — DIV records defaults that *diverge* from upstream, and
this one *converges* on it — but `docs/history.md` carries an entry recording the mis-port and
its correction, since the whole-corpus output shift needs an explanation for anyone bisecting
later. (Shipped as a "Convergences" row rather than a DIV, for exactly that reason.)

**Open question for review — the speed budget.** The ablation half of the case is unambiguous
(0/675 assertions). The speed half is not: re-measured properly at implementation time, this
costs **+6.2% to +8.2% on i386**, over the +5% budget, while x86-64 and ARM are within noise
(`analysis.md` §6). `docs/improvement-pipeline.md` §4 would demote an over-budget flip to
default-OFF opt-in. That table governs *feature* flips, and demoting this one means shipping a
default that deletes live loops and emits code that does not implement the binary — so it ships
ON with the breach recorded rather than absorbed. If the budget is judged to bind, flipping
`default` in `phases.toml` plus the `architecture.rs` reset path is a two-line change.

## 3. Alternatives considered (and rejected)

| # | Alternative | Why rejected |
|---|---|---|
| A | **The predicate correction** (recommended) | — |
| B | Keep the trial `no-use` but stop *freeing* its dataflow (skip the `op_set_input(const 0)` for spacebase trials) | Fixes only the second-order symptom: the mydoom loop survives, but the argument lists stay empty and the outgoing-argument stores render as dead stack writes. Strictly worse output, *more* divergence from upstream, and more code than A. |
| C | Score the trial `inactive` instead of `no-use` when it fails the range test | Same objection as B, plus `inactive` has its own downstream meaning (a maybe-pass-through register input) that does not apply to a stack slot. |
| D | Register the trial at the caller-relative address in `guard_calls` so the existing check passes | Breaks `build_input_from_trials`' `off = stackoffset + off` translation, `characterize_as_input_param`, and the `callee_pop` branch — all of which correctly want the callee-relative address. The two-address split is upstream's design, not an accident. |
| E | Widen the cspec `localrange` to cover positive stack offsets | Makes the wrong check accidentally pass, at the cost of corrupting `localrange`'s real meaning (MSABI's `stack 8..39` shadow-space range is a *caller-frame* range and would become ambiguous). A spec-file hack papering over a code bug. |

## 4. Implementation plan (the one PR)

Per `docs/improvement-pipeline.md` section 3's checklist:

1. **The change** — the one-argument correction above, plus the corrected comment. Marked
   `// (kuna)`? No: it is a ported-core file being brought *back* into line with its anchor, so
   the anchor comment stays and no divergence marker is added.
2. **`phases.toml` `[[settable]]` row** — `callsitestackargs`, every field populated:
   `tier = "core"`, `change_kind = "correctness-fix"`,
   `source_decompiler = "ghidra-upstream"`,
   `inspiration = "decbench:O0-mydoom-mydoom-msg_b64enc + O2-noinline-openssh-portable-ssh-add-parse_dest_constraint"`,
   `symptoms` drawn from the observed output shapes: *"call rendered with exactly 6 arguments on
   x86-64"*, *"call rendered with an empty argument list on i386"*, *"a `for(;;)` loop whose exit
   variable is written through a pointer argument disappears"*, *"`?:` guards feeding a logging
   call are dead-code eliminated"*, *"outgoing-argument stack slots render as locals commented
   `// stack - 0xNN`"*.
3. **Register in** `src/p0_knowledge/options.rs` (`KUNA_OPTION_NAMES`).
4. **Bump the hard-coded catalog counts** — `src/p0_knowledge/kuna_phases/tests.rs`,
   `tests/catalog_bytecompat.rs` + its `tests/fixtures/phase_catalog.json` fixture, and the
   `tests/stages/kuna-catalog.xml` count assertions (the `ghidra-upstream` /
   `correctness-fix` buckets).
5. **Two-pass stage test** `tests/stages/ghdec-callsitestackargs.xml`: pass 1
   `option callsitestackargs off` asserts the bug (an 8-argument callee called with 6), pass 2
   `on` asserts the fix (the 7th/8th arguments present), over a real binary -> `decompile` path.
   The witness in `analysis.md` section 2 is the fixture; a `bytechunk` of `caller`+`callee` is
   acceptable if it reproduces the same construct. Add a second assertion pair over an i386
   `push`-form call so both argument-passing forms are covered.
6. **Adjudicate the one stage regression**: `tests/stages/gh6882-sparcstructret.xml` assertion #2
   changes from `return v1 \+ a0;` to `return v1\[0\] \+ a0;`, with a comment recording *why*
   (the struct-return buffer pointer is now passed, so the buffer types as `int4[2]`). This is a
   legitimate expectation change, not an absorbed regression — the PR must show the before/after
   C, as `analysis.md` section 6 does. Then re-record `docs/baseline-stages.json`.
   **`docs/baseline.json` is NOT re-pinned and does not move.**
7. **Regenerate** `docs/options.md` (`kuna catalog --markdown`).
8. **Spec** — `docs/spec/04-calls-and-prototypes.md` prose addition; `make check-spec`.

## 5. Ablation plan

1. Develop default-OFF; confirm default output is byte-identical to `main` on a sample of
   binaries (diff `decompile-all --json` output).
2. Flip the default ON in the architecture reset path + the `phases.toml` `default =`, rebuild,
   run `kuna test --all --baseline docs/baseline.json`. **Already measured on a scratch build:
   675/675, PARITY OK.** If any assertion moves in the real implementation, it is adjudicated
   assertion-by-assertion in the PR body; the baseline is not re-pinned under any circumstance.
3. Speed:
   `python3 -m scripts.pipeline.timeit --record docs/features/callsite-stack-args/record.json
   --option callsitestackargs --binary <ssh-add> --selector 0x9190 --repeat 5`, plus a
   whole-binary `decompile-all` wall-clock on the largest corpus binary. **Already measured on a
   scratch build: no detectable delta** (ssh-add 10.80 s -> 10.79 s median over 3;
   mydoom 2.03 s -> 1.98 s min over 3). Budget is +5%.
4. Robustness sweep: run `decompile-all` with the option ON over the decbench stripped corpus
   and diff the error/timeout set against OFF. Any crash, hang or `error` field that appears
   *only* with the option ON blocks default-ON. **Already run on a 30-binary stratified sample
   (x86-64 / ARM Cortex-M / PE-i386, all three optimisation levels, 300 s timeout): 72 runs,
   zero timeouts, all exit 0, stderr byte-identical between OFF and ON on every pair, and
   dump-level counters (function count, empty-body count, error markers) all delta-zero on five
   binaries re-checked with full stderr capture.** Re-run at implementation time against the
   real option.
5. decbench rescore of the two originating cases
   (`python3 -m scripts.decbench.rescore --record …`) to splice `ged_before`/`ged_after`.
6. Whole workspace: `make rust-test`, `make check-spec`, `kuna catalog --check`.

## 6. Risks

| Risk | Assessment |
|---|---|
| Emitted C changes at a large fraction of call sites in every binary | Real and intended. Bounded by the ablation: 0/675 datatest assertions, 1 stage assertion (itself a fix). |
| **Over-recovery** on unprototyped `__cdecl`/indirect callees — stale outgoing-argument slots from a previous call are accepted as arguments (mydoom: `CloseHandle` renders with 5 arguments) | The main quality regression risk. Upstream Ghidra behaves the same for a genuinely unprototyped callee; it escapes via PE IAT import prototypes, which kuna does not yet apply to indirect-call targets. Sequenced as increment 2. Trading 192 empty argument lists for a handful of over-long ones is the right trade, but the PR must show it, not hide it. |
| More live CALL inputs -> more heritage / merge / DCE work | **Measured: real on i386** (+6.2% to +8.2%), within noise on x86-64 and ARM. Proportional to the arguments actually recovered, not a constant factor. Over the +5% budget; see the open question in §2.4. |
| Local-variable maps change (`scope_local_mark_not_mapped` now un-maps outgoing-argument ranges) | This is the correct behaviour and the mechanism by which pseudo-locals become arguments. Watch for aggregate/array retyping of adjacent stack slots (the SPARC case shows one). |
| Non-x86 architectures under-tested | SPARC verified (improves). ARM Cortex-M covered by the 30-binary sweep (`chibios/ch.elf`: `short_calls` 628 -> 420 across three opt levels, no failures); MIPS, PPC, AArch64 covered only by the datatest corpus. The robustness sweep in section 5.4 is the gate and has already returned clean. |
| The `calls_ge7_args` gain is *not* confined to stripped binaries — it nearly doubles even with full DWARF (37 -> 73 on six binaries) | Not a risk so much as a scope correction: DWARF locks only the prototypes it describes, so the change is visible in debugged builds too. Anyone expecting "DWARF builds are unaffected" will be surprised; `analysis.md` section 5 states the measured split. |
| The `maxpass = 0` single-scoring-pass window (section 2.3) | Not changed by this fix, but it makes the feature fragile against future scheduling changes. The stage test is the guard. |

## 7. Sequencing

**Is there a contained first increment that fixes only the dead-code-elimination consequence?**
Technically yes (alternative B), but it should not be taken: it is *more* code than the real fix,
produces strictly worse output, and increases the divergence from upstream. The correct first
increment *is* the whole fix.

| # | Increment | Size | Depends on |
|---|---|---|---|
| **1** | **This PR** — the predicate correction, option `callsitestackargs`, stage test, spec, sparcstructret assertion update | **small** | — |
| 2 | PE/ELF import prototypes applied to indirect-call targets (`(*dat_411350)()` -> `ReadFile(...)`), closing the i386 over-recovery artifact | small-medium | 1 (the artifact only exists once arguments are recovered) |
| 3 | The two remaining `guard_calls` STUBs: `guardCallOverlappingInput` (partial / sub-register stack arguments, `Containment::ContainedBy`) and `try_output_overlap_guard` | medium | 1 |
| 4 | DWARF stack-symbol binding at incoming-parameter offsets (the mydoom triage's "secondary observation": non-stripped builds read parameters as unbound locals) | medium | independent — separate case, separate proposal |

Increments 2-4 are **not** part of this PR and each needs its own analysis. Increment 1 stands
alone and closes both originating decbench cases.

## 8. What a reviewer should check

1. That the upstream line really is `inRange(vn->getAddr(),1)` — `fspec.cc:5618` in the pinned
   `GHIDRA_REV`.
2. That the neighbouring `callee_pop` branch correctly keeps the *trial* address — `fspec.cc:5621`.
3. That `docs/spec/04-calls-and-prototypes.md` already says "caller's local stack range", i.e.
   the spec was right and the code was wrong.
4. The `gh6882-sparcstructret.xml` before/after C in `analysis.md` section 6 — and agree that the
   assertion-text change is an expectation correction, not a swallowed regression.
5. The over-recovery example (mydoom `CloseHandle`) — and agree the trade is worth taking now
   with increment 2 sequenced behind it.
