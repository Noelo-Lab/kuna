# Ghidra-beats-kuna: gap analysis & port roadmap

kuna is a line-faithful Rust port of Ghidra's decompiler + analyzer tier, so a case
where **Ghidra scores a perfect GED 0 and kuna scores worse** means kuna has *diverged* —
an unported or mis-ported pass, or an analyzer Ghidra runs that kuna does not. This is
the highest-signal pool. We mined it from the decbench `full_run`
(`decbench improvements … -b ghidra -t kuna -m ged --perfect-only`): **152 cases / 77
dedup (project,function) groups**, then verify-first triaged each with *current* kuna
(one agent per case: re-decompile, classify, root-cause against Ghidra source).

## Triage outcome (77 groups)

| Status | Count | Meaning |
|---|---|---|
| already-fixed | 28 | current kuna matches Ghidra (the Jun-29 benchmark ran kuna 0.1.0, before F1 Listing-default / F2 noreturn_error / returndup / iteregion) |
| genuine-gap | 26 | still diverges; a real port target |
| covered-by-option | 20 | an existing knob (or a runtime-choice flip) already matches Ghidra |
| metric-artifact | 2 | Ghidra's stored output is truncated/degenerate — a decbench scoring artifact |
| load-failure | 1 | loader/decoder gap (PE) |

## The clusters (by Ghidra mechanism)

### 1. Non-returning-function overrun — **44 cases, the dominant cluster** → FIXED here

Ghidra's **"Non-Returning Functions - Discovered"** analyzer
(`FindNoReturnFunctionsAnalyzer.targetOnlyCallsNoReturn`). kuna's `noreturn_propagate`
concluded a wrapper no-return only when its *last* instruction was a terminal call to an
already-no-return callee — a tail-call subset that missed **mid-body** no-return calls
(openssh `cleanup_exit`), **dead/unreachable returns** (`sshpkt_fatal`), and
**switch-of-no-return** (`sshpkt_vfatal`). A caller then decoded the cold path as live
code and ballooned.

**Ported as `noreturn_reach` (DIV-19, default-on)** — a CFG-reachability walk iterated to
a call-graph fixpoint. Result: `ssh_tun_confirm` 222 LOC → 18 (Ghidra 24),
`sshd_hostkey_sign` 153 → 37 (Ghidra 50) — kuna now *tighter* than Ghidra. This also
subsumes the 8 "covered-by-option: noreturn_disc" cases (the agents' proposed knob is
exactly this rule).

### 2. Loop structuring — **~7 genuine-gaps** → ROOT-CAUSED (deferred, see below)

Symptom: on a `-O2` guarded tail-tested loop kuna emits `if (c) { do { body } while (c); }`
(peeled guard + do-while) where Ghidra emits a single top-tested `while (c) { body }`. Both
are correct; the margins are small (2–13). Families: `loop-while-recovery`,
`loop-while-reconstruction`, `loop-derotation`, `for-loop-recovery`, `while-guard-folding`,
`goto-structuring`. Representative: bash `dequote_list` (0x6f410).

**Root cause (traced to the bottom, dequote_list).** Ghidra un-rotates the loop with
`ActionNodeJoin`/`ConditionalJoin`: it joins the guard block and the latch block (both
2-out, both branching to {exit, loop-top}) once `findDups` proves their `!= 0` tests
functionally equal, then `ruleBlockWhileDo` forms the `while`. **kuna ports all of this
line-for-line** (`s8_structure/blockaction.rs`: `ConditionalJoin::{match_blocks,find_dups}`,
the `ActionNodeJoin` driver, `functional_equality_level`; and `isDoNothing`/`hasOnlyMarkers`/
`hasNoImmediateCopy`/`unblockedMulti` in `funcdata_block.rs`) — verified by tracing: the
driver never even *tries* `match(guard, latch)`.

The reason is one block upstream. GCC's rotated loop has the guard `jmp` into the loop top as
its **own empty forwarding block** (a lone `CPUI_BRANCH`, @0x6f430) sitting between the guard
and the loop top. So the guard branches to `{exit, jmpblock}` and the latch to `{exit,
loop-top}` — different targets, no join. Ghidra removes that empty block (`ActionDoNothing` /
`ActionLateDoNothing` → `isDoNothing`), after which the guard branches directly to the loop
top and the join fires. **kuna keeps the block**: tracing `ActionDoNothing` shows it is
correctly *delayed* (`unblockedMulti=true`, `hasNoImmediateCopy=false` — there is an immediate
copy), but by `ActionLateDoNothing` time it is **no longer `isDoNothing`** (`hasOnlyMarkers`
is false). The difference: **kuna materializes the loop's phi-init (`v = a0`) as a real
`CPUI_COPY` op *inside* the pre-header block** — and a `COPY` is not a marker — so the block
never becomes empty-enough to remove before late-do-nothing runs, whereas Ghidra keeps the
phi-init as a `MULTIEQUAL` edge input (the block stays a lone branch and is removed). By the
time the copy is trimmed (post-merge), the do-nothing sweeps have already passed.

**Fix options (deferred — high effort/risk for a cosmetic, both-correct gain):**
1. *Core-heritage*: don't materialize the loop pre-header phi-init as a `COPY` op (keep it a
   `MULTIEQUAL` edge input like Ghidra). Correct but **high-risk** — phi/COPY materialization
   is pipeline-wide and would need full 675-datatest re-validation.
2. *Post-structuring guard-fold pass* (recommended, ~iteregion-sized, default-off/gated):
   after structuring, rewrite a `BlockCondition{ true: BlockDoWhile }` whose guard test and
   loop-continuation test are the same predicate on the loop-head `MULTIEQUAL` into a single
   `BlockWhileDo`, dropping the guard. Operates on the structured tree, so it sidesteps the
   do-nothing/copy-trim timing entirely.

Deferred after landing the 44-case `noreturn_reach` win; option 2 is the scoped follow-up.

### 3. Merge / copy-propagation quality — **~6 genuine-gaps** → ROOT-CAUSED (deferred)

The `merge.cc` phase Ghidra runs that kuna under-applies:
`Merge::mergeAddrTied` / `ActionMergeAdjacent` / `ActionHideShadow` (coreaction.cc:4976 →
`Merge::hideShadows`). Ghidra coalesces a return-value / size MULTIEQUAL with its
address-tied stack-var / parameter / pass-through input into one HighVariable, so the
unchanged branch's copy becomes an identity `v = v` that is elided → a single-armed in-place
`if`. kuna keeps them distinct, emitting the residual `v = v;` self-copy / a two-armed `if`.
Families: `phi-copy-merge` (×2), `merge-copy-residue`, `highvar-merge`, `copy-shadow-merge`,
`global-store-heritage`. A couple also touch `RuleStoreVarnode` (STORE-through-const-pointer
→ COPY to the mapped global, ruleaction.cc:4340) and `SubvariableFlow` return narrowing.

**Root cause (traced, tar `make_directory` @0x1e190).** Same shape as cluster (2): the
merge passes ARE ported and registered (`s6_variables/merge.rs`:
`merge_multi_entry`/`merge_adjacent`/`mark_internal_copies`/`merge_test_speculative`,
`merge_opcode(MULTIEQUAL)`; `universalaction.rs` wires `ActionMergeMultiEntry`/`MergeCopy`/
`MergeAdjacent`) — but **`merge_test_speculative` (the cover/interference test) declines the
coalesce Ghidra accepts** (coalescing a MULTIEQUAL output with a compatible input whose cover
reaches the phi on both in-edges). So a phi-output register var stays a distinct
`v5 = phi(v1, v1+1)` where Ghidra reuses `sVar1`.

**Secondary defect = a real correctness bug (invalid C).** The un-merged phi output `v5` is
**emitted but never declared** (used 3× in `make_directory`, no declaration → won't compile).
Traced: `v5`'s high *is* reached by `emit_local_var_decls` (`printc.rs`) and *is* named, but
is skipped by the **`is_param` filter** — because the failed merge left `v5` with an instance
overlapping a parameter register, so the containment query mis-classifies it as a signature
parameter (declared in the signature, not the body). So invalid C is stacked on top of the
cosmetic diff, both rooted in the same declined coalesce.

**Fix options (deferred — deep/risky for the margins; the invalid-C sub-bug is the valuable
part):**
1. *Core-merge*: align `merge_test_speculative`'s cover/interference + candidate ordering with
   Ghidra's `Merge::mergeAdjacent` for the MULTIEQUAL-output ↔ input case. Correct root fix but
   **high-risk** (merge phase is pipeline-wide; 675-datatest revalidation).
2. *Emission backstop for the invalid-C bug* (targeted, byte-identical on the 675 datatests
   since none has an undeclared var): make `emit_local_var_decls` never skip a body-referenced
   high whose storage is not *actually* the signature parameter (tighten `is_param` to "is the
   declared parameter", or add a post-emit declaration backstop for any referenced-but-undeclared
   high). Fixes the invalid C without touching the merge phase. Recommended as the scoped
   follow-up — it removes a genuine invalid-C output regardless of the cosmetic coalesce.

### 4. Runtime choices — **already flippable, no fix needed** (20 covered-by-option)

kuna intentionally diverges *cleaner* than Ghidra; the benchmark rewards matching Ghidra.
All already agent-flippable (see `docs/decbench/runtime-choices.md`):
- **`taildup` off** (4) — Ghidra keeps a shared-exit goto; kuna's SAILR tail-duplication removes it.
- **`stackguard` off** (3) — Ghidra keeps the `-fstack-protector` canary epilogue; kuna strips it.
- **`branchflip` off** (3) — branch-polarity / negated-guard rendering.
- **`regionstructure`** (2) — includes one genuine correctness residual (the `ruleBlockWhileDo`
  `is_complex()` / `setOverflowSyntax` whiledo case, `overwrite_database`).

### 5. Singletons / out-of-scope

`var-naming`, `type-recovery` (SubvariableFlow return narrowing, ApplyDataArchive WinAPI
prototypes), `pe-import-recovery` (PE IAT devirtualization — multiformat-loader effort),
and 2 metric-artifacts (degenerate source CFG, already handled by decbench #6).

## Roadmap

1. ✅ **`noreturn_reach`** (this PR) — the 44-case overrun cluster.
2. **Loop-structuring port** — `ruleBlockWhileDo` de-rotation + `ActionNodeJoin` guard-join (~7 cases).
3. **Merge/copy-propagation port** — `Merge::mergeAddrTied` / `ActionMergeAdjacent` / `ActionHideShadow` (~6 cases).
4. `overwrite_database` whiledo `is_complex` correctness residual (small).
5. Runtime choices stay flippable (no fix); PE / type-archive are separate efforts.

Per-case records: `docs/decbench/triage-ghidra/` (the 14 hand-picked) + the full 77-case
verify-first pass (scratch records, folded into this summary).
