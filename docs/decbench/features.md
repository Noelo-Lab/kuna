# decbench campaign — the feature menu (round 2, 2026-08-03)

Output of `mine` → `triage` → cluster, then `fix`. Round 1's menu (2026-08-01, 16 cases
triaged against the 2026-07-27 run) is superseded by what actually shipped; this file
records the outcomes. Nine round-2 investigation tracks, one agent each, verify-first on
today's build, every finding handed to a second agent briefed to refute it — **8 live
candidates, 1 already-fixed** (`docs/decbench/triage/`, PR #247).

**Nine PRs merged**: seven of those eight candidates (only `catalog-upstream-options` is
still open), plus steps 1 and 2 of the ARM entry-recall sequence approved as
[PROPOSAL] #239.

Two columns matter and they are not the same column:

- **GED value** — does fixing it move the benchmark? GED is a CFG edit distance, so it is
  blind to types, argument lists, declarations, operator spelling and anything else that
  does not change basic-block count.
- **Correctness value** — does the emitted C stop being wrong? Several items below score
  **zero** on the benchmark and are still the most important things in this table, because
  kuna already wins those panes and emits code that does not compile or does not compute
  the right value.

Ranking is by correctness first. A campaign that optimizes only the metric it can see will
happily ship a decompiler that scores well and lies. Round 2 is the evidence: of the **nine
merged items** below, only **three move GED on a mined case** (#250, #253, #254). Four are
correctness fixes the metric structurally cannot see (#249, #251, #252, #257) — three of
them emitting C that does not compile — and two are recall (#248, #255), which GED cannot
rank at all, because a function nobody produced has no score to improve.

**A triage record's *symptom* is evidence; its *diagnosis* is a hypothesis.** Round 1 filed
five mechanisms and **two were right**. Round 2's refuters **overturned the filed diagnosis
on three of eight** (`returncopysplit`, `peimportcall`, `itecondlist`) — and in all eight
the *symptom* stood exactly as filed. Two of the refuters rebuilt kuna with the proposed
fix rather than arguing about it. Treat every "root cause" line as a starting point to
disprove; see `docs/decbench-loop.md` → *A symptom is evidence; a diagnosis is a
hypothesis* for the two method rules this cost.

## What shipped

| PR | slug | phase | what was wrong | measured effect |
|---|---|---|---|---|
| **#248** | `cortexmvectors` (option, default-OFF) | P1 | The ARM Cortex-M vector-table signature was too narrow on three axes at once — it demanded an executable section, `word[0]` in `0x2000_0000..0x3FFF_FFFF`, and `word[1] == e_entry` — so it silently no-op'd on the four projects contributing most of the ARM recall gap. | Recall **+96** ground-truth entries (118 new, **81.4% precision**, 0 splitting a real body, **0 lost** at any opt level). With `--option listing off` (the `fast`/WASM/Ghidra surface) **+393** (415 new, **94.7%**, 0 lost) — O2 betaflight goes **1 → 57** functions, because the whole-image Thumb paint only a confirmed table unlocks is load-bearing there. Byte-identical on x86-64, ARM Linux and the four images the shipped signature already detected. |
| **#249** | `funcptralign` (no option — port fix) | P3 | `RuleFuncPtrEncoding` was fully ported and registered, but its **only input** was a hard-coded `fn funcptr_align(_) -> int4 { 0 }` stub and the rule's first two lines bail on `align == 0`. **It could never fire on any architecture.** Every ARM/Thumb indirect call emitted an invented ISA-mode bit-clear the program does not perform, which also blocked the function-pointer type from back-propagating onto the LOAD. | crazyflie `cf2.elf`: masks **264 → 15**, `(code *)` **255 → 46**, `(code **)` **0 → 205**; nuttx 459 → 40 / 435 → 42 / 0 → 397. Class measured corpus-wide at **4,027 masked indirect calls in 2,496 functions across 9 ARM projects**. x86-64 / i386 controls **byte-identical** (their cspecs declare no `<funcptr>`); AARCH64 `align=4` verified against stock Ghidra 12.1 headless. Ablation 0/675. Speed: crazyflie `cf2.elf` `decompile-all` **−12.99%** — one op leaves the graph at every indirect call. |
| **#250** | `paramcopyhoist` (option, default-OFF) | P6 | `Merge::trimOpInput` anchors a parameter's copy-shadow at the tail of the phi's incoming predecessor, so for every parameter after the first the `vN = aM;` sinks *below* the previous guard and splits the source's single entry block. kuna is internally inconsistent about the identical construct. | GED **29 → 17** on `e2fsck save_output`, and the refuter separately measured deleting the copies outright — literally what the source does — at 17.0 as well, so the hoist is **GED-optimal** for this construct. Sunk copies **200 → 108** over 3,031 functions; 116 functions change, all 116 diffed, **0 semantic regressions**. Default-OFF: the datatest ablation is clean but the stage corpus moves 1 assertion and the aggregate decbench ablation is unrun. |
| **#251** | `subright` (no option — port fix) | P3 | `RuleSubRight::apply_op` was a **port stub**: only the leading special-print half was transcribed, so upstream's `sub(V,c) => sub(V>>c*8,0)` never ran and **every nonzero-offset SUBPIECE reached the printer as a raw p-code operator**. `SUB41`/`SUB81`/`SUB84` are undeclared identifiers — those bodies were not compilable C. | Raw `SUBnn` **1,912 → 95 (95.0% removed)**, functions carrying one **733 → 20**, over 7 whole-binary runs / **14,720 functions**; every one of the 95 residuals is a case upstream's own guards decline. Ablation **0/675** — exactly what "restoring upstream behavior" predicts. Measured cost: **2 functions in 14,720** gain a dead partial-register shift. Triage measured the class at kuna 5,919 sites vs ghidra 18 — a **329×** ratio. |
| **#252** | `returncopysplit` (no option — DIV-55) | P3 | `SplitDatatype` split a 16-byte read-only copy into per-byte COPYs whose write-backs printed as **stores into a `.rodata` literal the binary never performs** — 16 of them on a source line that reads `return -1;`. The filed mechanism (`is_return_copy()`) was **refuted by instrumentation**: that predicate never reaches a successful split. | Phantom statements **112 → 0**. Whole-binary before/after over 12 binaries / **15,386 functions**: exactly **3 functions change**, and they are the 3 the triage found. No real store lost — every surviving global assignment lands in a writable section, and all those counts are unchanged. |
| **#253** | `itecondlist` (option, default-ON, DIV-56) | P8 | In a run of N structurally identical two-arm diamonds, `iteregion`/`iteboolean` recovered exactly **ceil(N/2)**. Both reach the condition through one helper that descends a `BlockList` of *one* component and bails on two or more; the collapse structurer concatenates a just-collapsed predecessor with the following condition block, so a chain alternates between the two shapes. Which diamonds were missed depended on the *neighbours*, not the diamond. | N-sweep **ceil(N/2) → N** for N=1..8. GED **46 → 24 (−48%)** on bash `shell_initialize`. Over **12,241 functions**: 53 change (0.43%), +59 diamonds folded; a mechanical per-hunk audit found **0 problems**. Ablation 0/675 with the flip, no re-pin, no per-test opt-out. |
| **#254** | `peimportcall` (option, **default-ON, DIV-57**, `tier = transform` despite being P1) | P1/P2 | A PE reaches its imports through an IAT slot, which is *data*, so `call [slot]` lifts to a `CALLIND` through a global. The only pass that resolves that, `ActionDeindirect`, gates on `isPersist() && isExternalRef()` — and **`Varnode::externref` was set nowhere in the workspace**. Every Windows API call stayed an unnamed `(*dat_4112c4)(0)` with no name, no prototype and, the damaging part, **no no-return flow effect**, so a function ending in `ExitThread` swallowed the whole of the next one. | **927 unnamed Win32 call sites → 8.** GED **37 → 0** on `mydoom mmsender_th` and **39 → 0** on `sync_visual_th` — both perfect. Three dexter functions stop overrunning; the largest is `Uninstall` **152 → 19 lines**. No function gained or lost. Ablation 0/675. |
| **#255** | `ptrentry` (option, default-OFF) | P1 | Step 2 of the ARM entry-recall sequence: a function entry reachable only through a pointer-table/code-pointer reference was not admitted, because the naive relaxation splits real bodies (the proposal measured it at 62% ground truth with 21% mid-body splits). The shipped guard is **containment** — no referencing word may share the target's discovered function, nor overlap a decoded instruction. | Cortex-M recall **38,238 (92.4%) → 39,851 (96.3%)** on the decbench surface: **+1,694 new entries, 1,613 ground truth (95.2%)**, **0 splitting a real body, 0 lost**, against a proposal bar of ≥80% GT and zero losses. `make test` **PARITY OK 675/675**, no DIV row (default-OFF). |
| **#257** | `undefname` (no option — DIV-58) | P6 | The `$$undefNNNNNNNN` placeholder reached emitted C — an identifier containing `$$`, and worse, a **second** identifier for a stack Symbol already declared under its `vN` name, so the body read a variable nothing ever wrote. kuna binds the name *per HighVariable* in a cached `String` where upstream reads `getDisplayName()` live; and `Database::assign_default_names`, the port of upstream's catch-all rename, **had zero call sites in the tree**. | Occurrences **2,142 → 0** over **116 stripped binaries / 90,808 functions**: binaries emitting one **73 → 0**, functions **371 → 0**, Symbol tables carrying one **93 → 0**, function count identical in both arms. Blast radius over 12 whole binaries / 18,590 functions: **162 changed, 0 of them without a placeholder in the before text**; all four controls byte-identical. GED-neutral by construction (8.0 → 8.0) — a NOVEL-pool item, ranked by correctness. |

## The finding that outlives its PRs: registered passes that cannot run

Three of the items above are the same class of defect, and they failed in **three distinct
ways**. A pass was ported, registered in the schedule, and **could not affect output** —
each one invisible to every existing test, because a pass that never fires breaks nothing.
Grep for all three shapes; they do not look alike in the source:

| shape | instance | what to grep for | found by |
|---|---|---|---|
| **Stub body** — the transform half was never transcribed, so the rule returns 0 before doing anything | `RuleSubRight::apply_op` | a registered `Rule`/`Action` whose body is a bare `0` / early `return` under a `STUB`/wave marker comment | #251 |
| **Live body, permanently-zero input** — the code is complete, but a helper feeding it is hard-coded to a value the first guard bails on | `RuleFuncPtrEncoding` (`fn funcptr_align(_) -> int4 { 0 }`, and the rule's line 2 is `if align == 0 { return 0 }`) | stub helpers returning a constant, then whoever reads them; the real value had been parsed into the field since W4 | #249 |
| **Live body, gate bit with no writer** — a flag is defined and read, and nothing in the workspace ever sets it | `ActionDeindirect`'s external-ref arm (`varnode_flags::externref`) | flag/property constants with readers and **zero writers** across the whole workspace | #254 |

The third shape had a **second gap in the same chain**, which is why it is worth naming
separately: even once `externref` was painted, `GlobalEntry::func_no_return` never copied
the callee's no-return flag into the symbol snapshot, so the resolved import would have got
its name and prototype but still no flow effect — and the flow effect is the part that was
eating the next function. One dead gate can hide another.

Each had large breadth once revived (5,919 / 4,027 / 927 sites). **"Which registered passes
cannot fire?" is now a first-class mining route** alongside the GED pools, and it needs no
benchmark at all.

> **Why #257's count is 2,142 and not 2,143.** #254 landed underneath that branch and moved
> its before-arm by one. The agent caught it by **re-measuring on the rebased tree instead
> of carrying the number forward** — on a number small enough that nobody would have
> questioned it. That is standing requirement 9 of `docs/improvement-pipeline.md` earning
> its place, and it is the same failure mode as the silent counter auto-merges below.

## Did not ship — still open

- **`catalog-upstream-options`** (P0, feature-candidate,
  `triage/audit-uncatalogued-options.md`). Round 1 filed `readonly` as a one-off; it is a
  family. **All 38 upstream-inherited `ArchOption`s are absent from `kuna catalog`** — 38
  of the 121 real options the audit counted — and at least 8 of them measurably change
  emitted C via `--option` on today's build. `kuna catalog --check` is green because it
  enforces bidirectional equality between the catalog and `KUNA_OPTION_NAMES`; the
  contract simply does not cover the other half. They are therefore invisible to any agent
  sweeping options by symptom, which is the failure that matters most here: **the LLM
  control surface is the product**.

## What round 2 says about the campaign

Round 1's reading was that kuna's *structural* recovery is competitive and the remaining
defects are in what the metric cannot see. Round 2 confirms it and sharpens it.

- **Six of the nine merged items have no GED value**, and four of those were emitting
  invalid or semantically wrong C. GED found none of them; the NOVEL and recall pools and a
  correctness read of kuna's own panes did.
- **The biggest wins came from reading kuna's own code, not a rival's.** The three dead
  passes were found by asking "why does kuna print this?", not by a margin ranking. No
  rival pane was needed for any of them.
- **The ARM recall sequence is where the raw function count moves.** Steps 1 (#248), 2
  (#255) and 3 (#259) are merged, as is the unplanned step 5 `poolentry` (#278); only step 4
  (TBB/TBH) is not started, and it needs re-scoping since its estimate predates all four.
  All four shipped default-OFF and were inert on the default path until **DIV-93** put them
  in the `aggressive` preset, which is what finally moved the measured number: entry recall
  88.63% → 93.31% over the 110 non-x86-64 twins, with mid-body false entries falling. The
  original sequence table and the three predictions the proposal got wrong live as a comment
  on **PR #239** — read it there rather than duplicating it here.
- **The refuters keep earning their keep, and their brief needed widening.** They overturn
  roughly a third of filed diagnoses. But "the mechanism is not a no-op" is not "the
  mechanism is correct" — on `paramcopyhoist` a design that provably reached the printer
  still deleted an assignment. Both rules that follow are now standing requirements in
  `docs/decbench-loop.md` and `docs/improvement-pipeline.md`.

Rank by correctness, use GED as a filter, keep mining the NOVEL and recall pools — and add
"which registered passes cannot fire?" to the list of questions the pools cannot answer.
