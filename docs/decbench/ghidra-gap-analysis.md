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

### 2. Loop structuring — **~7 genuine-gaps** → next port

`CollapseStructure::ruleBlockWhileDo` (blockaction.cc) fed by
`ActionNodeJoin`/`ConditionalJoin`: Ghidra de-rotates a `-O2` guarded tail-tested loop
(the compiler-peeled guard/latch pair) into a single **top-tested `while`** by joining the
duplicated condition (`functionalEqualityLevel` + `nodeJoinCreateBlock`), then
`BlockWhileDo::finalTransform` / `analyze_for_loops` recovers the `for` init/increment.
kuna leaves the rotated do-while + a residual guard goto. Families: `loop-while-recovery`,
`loop-while-reconstruction`, `loop-derotation`, `for-loop-recovery`, `while-guard-folding`,
`goto-structuring` (×2). Cases: e.g. iproute2 / bzip2 `-O2` loops.

### 3. Merge / copy-propagation quality — **~6 genuine-gaps** → port after (2)

The `merge.cc` phase Ghidra runs that kuna under-applies:
`Merge::mergeAddrTied` / `ActionMergeAdjacent` / `ActionHideShadow` (coreaction.cc:4976 →
`Merge::hideShadows`). Ghidra coalesces a return-value MULTIEQUAL with its address-tied
stack-var / parameter input, eliding the unchanged branch's **self-copy** into a
single-armed in-place `if`, and consolidates copy-shadow varnodes of a common ancestor
into one HighVariable. kuna emits the residual `v = v;` self-copy / a two-armed `if`.
Families: `phi-copy-merge` (×2), `merge-copy-residue`, `highvar-merge`, `copy-shadow-merge`,
`global-store-heritage`. A couple also touch `RuleStoreVarnode` (STORE-through-const-pointer
→ COPY to the mapped global, ruleaction.cc:4340) and `SubvariableFlow` return narrowing.

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
