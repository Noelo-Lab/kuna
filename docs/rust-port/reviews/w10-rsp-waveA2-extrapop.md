# RSP Wave A′ (L0 ActionExtraPopSetup) — REJECTED by rigorous finalize gate

Wave wj7m7wd7f (2026-06-17). The L0 un-stub (faithful verbatim transcription of
coreaction.cc:1452-1482, no special-casing, RuleAddMultCollapse untouched) was rigorously
gated and **FAILED — do not land**.

## The +7/340 was MISLEADING (the fence tests caught what the datatest oracle masked)

- Datatest-assertion regressed-set: **EMPTY** (340 = 333 + 7; `comm` diff empty). The +7
  came from **concatsplit #2-7 + For-loop-thru-special #2 — UNRELATED to switchind.**
- But `cargo test --workspace`: **8 NEW regressions** (all green on baseline rust-port HEAD,
  fail on the worktree) — the loose datatest stringmatch oracle masks real output-quality
  degradation that the committed fence tests catch:
  - `verify_w10_input_params::unrelated_models_keep_exact_signature` — nanops gains a spurious
    2nd param: `void nanops(float8)` → `void nanops(float8,float8)`.
  - 4× `verify_w10_jts_chain` (switchind label recovery broken).
  - 2× `verify_w10_spacebase_render` (mapped-local &name + forloop1 dirty-rsp).
  - `verify_w10_struct_corpus::...forloop_varused...` — lost the 64-bit (RSP) lift.

## switchind is STILL 13/16, not 16/16 (CORRECTION-2 disproven)

KUNA_DUMP B4: `get_value_byref();  switch(0x100058)` — the FORBIDDEN state. Under the bare
console it degrades further to `switch((int8)dat_1000b8 + 0x1000b8)` with raw-address case
labels. **The `(RSP-8)+8` fold does NOT recover the stack slot** because the inserted
`INT_ADD` (ActionExtraPopSetup `getSpacebase(0)`) **targets the register-space spacebase
(offset 32), not the stack spacebase** — so RuleAddMultCollapse never sees a `(stackptr-8)+8`
term to fold. CORRECTION-2's claim ("L0 + the existing fold → switchind 16/16") is FALSE.

## CORRECTION-3 (the real conclusion): L0 is genuinely COUPLED — defer

The original deferred-stub comment was right: L0 ActionExtraPopSetup inserts the spacebase
INT_ADD but the downstream pass that consumes/cleans it (propagateSpacebaseRef + the
spacebase-store ActionDeadCode) is still stubbed, so the per-call op SURVIVES and corrupts
stack-pointer flow → switchind regresses + 7 other fence failures. **L0 cannot land
incrementally.** Three design passes + two port attempts have each been partially wrong;
RSP is a deep, coupled keystone, not a "minimal atomic landing."

Two concrete leads for the eventual full coupled effort (a dedicated multi-wave / separate
session, NOT another incremental probe):
1. **Why does `getSpacebase(0)` / the ExtraPopSetup spacebase varnode resolve to the
   register space (offset 32) instead of the stack spacebase** for these jumptable functions?
   If the op targeted the stack spacebase, RuleAddMultCollapse might fold it. This is the
   most specific, tractable sub-bug.
2. The full coupled set = L0 + propagateSpacebaseRef-consumes-the-op + spacebase-store
   ActionDeadCode + the register-vs-stack spacebase fix, landed ATOMICALLY (the deferred-stub
   comment's prediction). Plus L4/L5 from the plan for the render.

The faithful L0 transcription is shelved (WIP patch `w10-rsp-waveA-L0L1-wip.patch` +
checkpoint f34b411). The +7 (concatsplit/forloop) is a genuine clean gain that a DIFFERENT,
narrower change (not touching ActionExtraPopSetup) could capture — worth a separate probe.

**RSP keystone: DEFERRED to a dedicated coupled effort. Focus shifts to the tractable
clusters (struct-return, f0-flag-v2, spacebase-cast) which are landing.**
