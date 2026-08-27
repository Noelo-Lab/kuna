# decbench campaign -- the feature menu (round 4, 2026-08-10)

The O0-vs-IDA round. 64 cases mined from the `--base ida` pool where **IDA's GED is 0 and
kuna's is not, at -O0**; one triage agent per case, then a clustering pass, then **three
adversarial refuters per cluster** on the top six clusters (lenses: *would this produce
wrong output?*, *is it already fixed or a metric artifact?*, *is the mechanism wrong?*).

Every number in this file marked **[V]** was re-derived by the synthesis step directly from
`~/github/decbench/results/full_run/ged_new.json` and the stored panes, not copied from an
agent record. Everything else is attributed.

Measured against HEAD `82dd39a7`. The stored kuna column is the pinned build `9623dc27`.

---

## 1. Census headline

**Of the 64 mined cases, 52 are real kuna structural defects that reproduce on today's
build. 12 are not.** The campaign's standing "roughly half are scoring artifacts" prior
**does not hold for this slice** -- the O0-vs-IDA gap is overwhelmingly real, and it is far
more concentrated than any previous round.

| verdict | cases | note |
|---|---:|---|
| real defect, reproduces on HEAD | **52** | 81% |
| metric artifact | 5 | `beeperInit`, `timerChConfigIC` (pyjoern gives a `?:` its own diamond only when the arms are non-trivial); `argv_next`, `get_crl_revocation_date`, `get_crl_this_update_date` (Joern does not split a short-circuit whose operand is a bare identifier) |
| closed at synthesis, no feature | 5 | the remaining `iteregion`/ternary cases -- the ablation is net-**positive** and the population is de-enriched (see Killed K4) |
| already fixed before the round opened | 2 | `mmsender_th`, `sync_visual_th` -- PR #254 `peimportcall`; `sync_visual_th` already scores 0.0 in the current tree |

Triage-record statuses over the same 64 (the raw filing, before synthesis reclassified
anything): 35 feature-candidate, 17 covered-by-option, 5 metric-artifact, 5 needs-proposal,
2 already-fixed. **[V]** -- counted from the 64 untracked records in `docs/decbench/triage/`.

Two structural facts about the 52 that matter more than the count:

**(a) The gap is concentrated in one rule and one pass.** Roughly 31 of the 64 cases (48%)
involve `returndup` or its interaction with condition folding; a further 10 trace to a single
60-line structuring rule (`rule_block_if_no_exit`). Four print-tree signatures, near-disjoint
(max pairwise overlap 14 functions), cover **1,124 of the 3,067 addressable O0 functions
(36.6%)** carrying **9,250 of 22,730 GED (40.7%)**.

**(b) The deficit those clusters chase is smaller than the clusters.** **[V]** Of the 32,077
O0 functions scored by both tools: ida holds 14,722 perfect, kuna 14,593 -- **a net deficit
of 129 functions**, being 3,067 ida-only wins against 2,938 kuna-only wins. (The
`round4-grounding.md` figure of 274 is the same thing plus 261 ida-perfect functions kuna
produced nothing scoreable for; that residue belongs to the recall workstream, not here.)

> Every single cluster in this menu is 2-4x larger than the entire net deficit. That is the
> round's most important operational fact: **standing requirement 8 is not hygiene here, it
> is the whole measurement.** A feature that flips 400 losses into wins and 400 wins into
> losses moves the scoreboard by zero, and nothing in the mining pipeline would show it.

**Eight of the 52 are cases where kuna's pane is semantically equal-or-better than IDA's
GED-0 pane and still scores worse.** IDA types `xcalloc`/`xvasprintf`/`dangerous_locale` as
`double`, discards the allocation result and emits `return 0.0`; kuna computes the right
value. IDA wins `initbuf` by *not* knowing the callee is no-return. IDA synthesises an `else`
the source does not have on the three gnutls date functions. **IDA scoring 0 means its CFG
matched the source, never that its body is right** -- on those eight the target is the
source, not IDA's pane.

### The refuters' hit rate, and what it implies for the nine unrefuted clusters

Six clusters got the three-lens adversarial pass. **Zero symptoms fell. Four of six filed
root causes were materially overturned, and six of six had a load-bearing claim refuted.**
That continues the campaign's most reliable finding exactly (round 1: 2 of 5 mechanisms
right; round 2: 3 of 8 overturned).

**Nine clusters in this menu were never refuted.** Given a 4-in-6 overturn rate they should
be read as *unvalidated*, and they are marked as such below. Do not let an unrefuted
cluster's confidence field outrank a refuted one's.

---

## 2. Two method corrections, both verified here

These change how every number in this round and the next must be read.

**2.1 The scored kuna column is `aggressive`, not `reliable`. `docs/decbench-loop.md` is
wrong about this.** **[V]** `decbench/decompilers/raw/kuna_raw.py:199-214` builds
`kuna decompile-all <bin> --json --max-fn-seconds 120`, appends `--mode` **only** if
`DECBENCH_KUNA_MODE` is set, and appends `--option` only from `config.extra_options`, which
defaults empty. There is no injected `option listing on` anywhere in the command builder. So
the benchmark ran `--mode auto` = **aggressive for 768 of 803 binaries**. Consequences:
an aggressive-only regression such as `regionedgeorder` is *inside* the published number,
and any triage that assumes `reliable` mis-attributes it. Fix the loop doc.

**2.2 `returndup`'s default-ON evidence never saw O0.** **[V]**
`docs/decbench/returndup-regression-triage.md` has exactly two ablation rows -- O2 (22,549)
and O2-noinline (30,313), totalling the published 52,862 -- and no O0 row anywhere in the
file. `docs/options.md:511` repeats "+417 GED-perfect, -7,756 aggregate GED" without the
scope caveat. returndup is default-ON globally (not merely via the preset), so the O0 third
of the corpus has been carrying an unmeasured flip since DIV-54.

Secondary, from the batch14 pass: `docs/decbench/cases-ida.json` is dated 2026-08-03 while
the tree's kuna artifacts were re-decompiled 2026-08-08 and re-scored 2026-08-09. At O0 only
11 of 3,073 rows moved (6 to perfect), so this round's slice is sound -- but every future
triage agent should look the case key up in the current `function_results.json` first.

---

## 3. The surviving menu, ranked

### Rank 1 -- `returndup-o0-ablation` (measurement, zero code) -- **DO THIS FIRST**

| | |
|---|---|
| **symptom** | `returndup` imposes the per-predecessor early-return shape on a shared compiled epilogue; at -O0 that shape frequently is not what the source wrote. |
| **root cause** | *Re-specified by the refuters.* The filed cause ("gcc -O0 does no cross-jumping, so a shared epilogue means the source shared it") is **false** -- see Killed K10. The real separator is **in-edge provenance**: the split recovers the source when each in-edge is a distinct source `return`, and fabricates returns when the in-edges are control-flow merges (short-circuit false edges, switch-`break` joins, if/else joins, loop-exit edges). At -O0 the merge population dominates; at -O2 it does not. |
| **phase / files** | P8. `p8_structure/kuna_returndup.rs`; `substrate/funcdata_block.rs` (`return_split_is_splittable` :2882, `returndup_is_const_ret` :3010, `returndup_apply` :3080); extend `docs/decbench/returndup-regression-triage.md`, do not replace it. |
| **witnesses** | 13, incl. `gz_init`, `inflateValidate`, `ConvertStatusLite`, `USBD_StdDevReq` (x2 projects), `nextArg`, `nud_state_a2n`, `check_name_arg`, `dangerous_locale`, `detect_data_type`. |
| **coverage** | Governs ~31 of 64 cases. **[V]** 10,360 of 34,108 O0 panes carry the banner (30.4%); 1,066 of 3,067 addressable (34.8%). Gates **11 of the 17 covered-by-option cases** **[V]** (4 `returndup`, 3 `returndup off`, 1 `returndup (OFF)`, 1 `returndup off + condfold wide`, 1 `returndup off (20->2)`, 1 `iteregion off + returndup off`). |
| **scope** | proposal (measurement only). No option, no code. |
| **gating** | n/a for the sweep. Any resulting flip is a default/preset change and needs a DIV row. |
| **GED available** | Pilot only: 3 binaries / 644 scored O0 functions, aggregate a wash (-25, -0.45%) but **GED-perfect 243 ON -> 285 OFF, +42 (+17.3%)**, 60 perfects created / 18 destroyed = 3.3:1 for OFF, mirroring O2's 3.2:1 for ON. Three binaries against DIV-54's 52,862 justifies the sweep, not a flip. |
| **residual risk NOT eliminated** | **(i) The coverage stat has no control and is close to inert.** **[V]** 2,938 O0 functions are kuna-perfect/ida-not, and **41.3% of those carry the same banner** -- a *higher* rate than the 34.8% in the losing set. Banner mass is nearly a uniform property of O0 panes; it is not a loss signature. **(ii) There is no O0 discriminator in the engine.** kuna cannot observe the optimisation level, so the only levers are a global default flip or a preset edit, both of which hit O2 where +417 perfect was measured. An O0-only win is not shippable as measured. **(iii) The OFF arm de-structures loops** in 13 of 10,216 functions (e.g. `ip rtnl_rttable_a2n` hoists a post-loop tail *into* a `while(true)`), and it can render an 8-deep nested else-if that GED scores as perfect (`ip nud_state_a2n`) -- a readability regression the metric rewards. **(iv) Both arms emit control-flow-unfaithful C on real O0 functions and GED sees none of it** (see Rank 8). |
| **change to the plan** | **Drop the `condfold wide` third arm** -- measured inert by two independent refuters (byte-identical to plain `returndup off` on `dangerous_locale` and `detect_data_type`). If a third arm is wanted, use **`returndup off + iteregion off`**, which the largest-margin witness (`USB_OTG_EPDeactivate`, margin 27) actually needs. Score **both** populations (3,067 losses and 2,938 wins), split by architecture and by whether the split de-structures a loop, and report O0 and O2 on the same instrument. |

---

### Rank 2 -- `ifnoexit` (NEW: `noreturnguard` + `loopcondhoist` are ONE site)

This is the synthesis step's own finding and it does not appear in any single agent record.
The two largest clusters in the round were filed independently, with different symptoms, in
different owning files -- and the refuters' mechanism verdicts land them on **the same 60
lines**: `CollapseStructure::rule_block_if_no_exit` (`p8_structure/blockaction.rs:2466`) and
its deferred scan (`:2915-2929`), whose only caller is `:2920`. **[V]** `grep` finds
`rule_block_if_no_exit` exactly 3 times in the workspace, all in `blockaction.rs`.

- **`noreturnguard`** is the rule choosing the wrong **arm** (the `for i in 0..2` tie-break).
- **`loopcondhoist`** is the deferred scan choosing the wrong **block** (first-match in RPO
  puts a loop head ahead of its body; folding the head to `if (!C) return X;` drops it to
  `size_out()==1` and makes `rule_block_while_do` at `:2515` structurally unreachable).

They interact: changing the scan order changes which blocks are still 2-out when the arm
tie-break runs. **One instrumentation pass settles both**, and three separate refuters asked
for exactly it.

| | |
|---|---|
| **symptom A (`noreturnguard`)** | A guard clause whose arm is a no-return call is INVERTED: the condition is negated, the whole body is re-parented one level deeper inside the `if`, and the fatal call becomes the function's trailing fall-through. Source, IDA and Ghidra all write `if (bad) fatal(); <flat body>; return K;`. |
| **symptom B (`loopcondhoist`)** | A loop's head test is left inside the loop as a guarded exit: `while( true ) { if (!C) <leave>; BODY; }` where the source and IDA write `while (C) { BODY }`. |
| **phase / files** | P8. `p8_structure/blockaction.rs` (`rule_block_if_no_exit` :2466-2508, the deferred scan :2915-2929, `rule_block_while_do` :2515), plus one new `p8_structure/kuna_ifnoexit.rs` for the predicates and option gates. |
| **witnesses** | A: `xcalloc`, `xvasprintf`, `validate_archname`, `validate_pkgname`, `validate_trigname`, `control_list` (has its own built-in negative control -- its second guard is already correct in the same function), `dh_info` (free oracle: the same source built `-fno-stack-protector` produces the correct flat guards on the same build). B: `read_char`, `walk_tree_visited`, + `ptx search_table`, `du mount_point_in_fts_cycle`. |
| **coverage** | **[V] B is the strongest population signal in the round: 1,513 O0 panes carry the `while( true ) {` + immediate-guard signature, 1,395 are scored, and kuna is PERFECT ON ZERO of them.** 432 addressable / 2,778 GED. A: **[V]** with a loose signature 1,646 panes / 406 addressable / 3,454 GED, perfect on 173 of 1,616 (10.7% against a 45.5% base rate); the tighter filed signature gives 1,280 / 353 / 2,912, perfect on 44 of 1,256. Combined and de-overlapped: **~780 addressable functions / ~5,700 GED, 25% of the entire addressable pool.** |
| **scope** | small per predicate -- but **instrument before writing either.** |
| **gating** | Two default-off options (`noreturnguard`, `loopcondhoist`) + DIV rows. Note default-OFF measures **zero** on this benchmark, because neither `reliable` nor `aggressive` picks up a newly-added off-by-default flag; the corpus ablation has to argue the flip. |
| **GED available** | A: measured closes `xcalloc` 6->0, `validate_archname` 8->0, `validate_trigname` 8->0. Honest recoverable band **~1,100-1,600**, not 2,912: only 33 of 345 have Ghidra also at 0 (238 pts, demonstrably reachable), 236 sit at GED<=8 (1,374 pts, plausible), and 109 carry 1,490 pts at mean 13.7 where kuna already beats Ghidra -- one guard flip cannot close those. B: a hoist removes 1 node + 2 edges and ~5 GED; 322 of 432 sit at exactly GED 5, so ~210 functions reach 0 and **~1,234 GED** is recoverable, +0.65pp on the O0 perfect rate. |
| **residual risk NOT eliminated -- A** | **(i) The filed predicate ("prefer the arm whose component ends in an artificial no-return halt") is refuted.** ~82-84% of the 1,217-panes signature set are single-test guards where the halt is on out(1) by construction, so the predicate fires on ALL of them and flips the ones that are already right. Two named victims verified fresh on HEAD in both modes: `coreutils/make-prime-list xalloc` @0x1906 and `coreutils/tail xlseek` @0x4804 -- the latter re-parents a 20-line `whence` cascade one level deeper and demotes `return v1` to the trailing statement, i.e. manufactures the exact symptom on a function that is perfect today. A third, `pinky short_pinky`, is a genuine trailing-fatal source shape. **(ii) The filed root cause predicts the OPPOSITE of the observed output on two witnesses.** `objdump` of `scp xcalloc` @0x24093 (`jne`) puts the fatal block on out(0), so a first-match on out(0) would produce the CORRECT shape; kuna produces the inverse, and still does with 15 P8/P9 options individually off. The mechanism refuter's gdb trace reconciles it -- `collapse_all` runs twice per function and `take_pending_flips` -> `block_basic_negate_lastop` -> `swap_edges` re-orients the CFG between runs -- which means the pre-state orientation at the rule is *not* the disassembly orientation and the one-line diagnosis is under-specified. **(iii) The owning-files list is wrong: there is no twin in `region_structurer.rs`** **[V]**; `regionstructure off` is byte-identical on all 8 witnesses and the region structurer reports `ok=false` on 10 of 10 invocations there. **(iv) "Ghidra never hits the tie" is false** -- 7,326 of 29,649 stored O0 Ghidra panes carry the no-return warning, Ghidra marks all four `dh_info` fatal callees, and Ghidra itself inverts on 32 of 343 signature functions. Ghidra reaches the flat shape *with* the fact in hand, so this is a lost coin flip, not a price for being right. **(v)** `blockaction/tests.rs` has zero tests naming `if_no_exit`. |
| **residual risk NOT eliminated -- B** | **(i) The filed root cause (`cb.size_out() != 1` declining at `region_structurer.rs:1788`) is refuted by minimal pair.** Probes `p7` (a source-level `break` in the body -- exactly the second out-edge the diagnosis blames) hoists CORRECTLY; `p20`, identical but with one call moved out of the break arm, does not. Relaxing that guard is a **no-op on both witnesses**, and `regionstructure off` is byte-identical, so the live site is `blockaction.rs`. **(ii) "Ship form A first, it is trivially safe" is backwards and dangerous.** Form A (clause is a bare `break;`) is **9 of 432 addressable / 62 GED (2.1%)**, it closes neither witness (read_char is form B, walk_tree_visited is form C), and it is produced by a *different* path -- the `is_complex(head)` overflow-syntax branch, where `rule_block_while_do` already SUCCEEDED. Promoting those heads emits wrong C: `mksyntax main` @0x19f7 plus 91 measured siblings become `while (v1 != -1) { ... }`, reading `v1` before `getopt` assigns it and dropping the call out of the loop. **(iii) Form C (217 instances / 1,181 GED, and 197 of the ~210 functions that reach 0) relocates a `return`,** so the follower predicate must be an in-code assertion: the loop's *immediate structural follower*, in-degree exactly 1. A matcher keyed on "a `return X` reachable after the loop" makes `libacl getfacl get_list` free the list it just built and return NULL (297 of 1,561 instances have a non-return follower; 128 of those have a call). **(iv) The hoist alone regresses form B/C by 1 goto + 1 label** (`BlockWhileDo::scopeBreak` rebinds `curloopexit`), so the follower move cannot be deferred to a second PR. **(v)** 530 of the 1,395 scored signature panes are ones kuna already WINS; the pass fires on all of them. **(vi)** `bzip2 mainGtU` (the largest cited margin, 30) is a non-member -- its source is a genuine `do/while` and the hoist moves *away* from it while GED improves 30->25. Drop it from the witness list. |
| **the one experiment that unblocks both** | Add a decision trace to `rule_block_if_no_exit` (reuse the `KUNA_RS_DEBUG` pattern already in `region_structurer.rs`) printing, per candidate: `bl`'s type, both arms' `size_in`/`size_out`/`is_decision_out`, whether the arm's exit leaf carries `pcodeop_flags::noreturn` (0x1000000 -- set by `op_mark_halt`, and distinct from `badinstruction`/`unimplemented`/`missing`/`halt`, which share the identical `return(#0x1:4)` shape), whether the candidate is a loop head, and the chosen `i`. Run it on `scp xcalloc`, `[ posixtest`, `control_list`, `od read_char`, `getfacl walk_tree_visited`, and probes `p7`/`p20`/`p23`. That single trace decides whether A is an arm predicate or a scan-order predicate, whether B is the same, and which of the two must land first. |

---

### Rank 3 -- `boolreturn` (contiguous trailing boolean-return fold)

| | |
|---|---|
| **symptom** | A source function that is ONE boolean `return <short-circuit expr>;` comes back as a cascade of N constant-return guards plus a trailing constant return. Witnesses run 2 to 13 guards. |
| **root cause** | *Overturned.* The filed cause ("kuna has no boolean-RETURN re-roller; `iteboolean` declines because a RETURN of a constant is not a COPY") is **false** -- with `--option returndup off` the arms ARE COPYs and `iteboolean` fires, logging `re-rolled 1 0/1 select diamond(s)` and emitting the single boolean expression on `find is_octal_char`, `basenc isbase2`, `dir needs_quoting`, `grep is_device_mode`, `ip validate_dump`/`validate_secy_dump` and others. This is a **collision between two default-ON features** (DIV-54 `returndup` shatters the {0,1} diamond DIV-51 `iteboolean` needs), not a missing capability. It is a two-stage shatter: upstream option-less `ActionReturnSplit` produces the `if (chain) return 1; return 0;` wrapper (the 11-12 plateau every non-IDA tool sits on), then `returndup` re-splits *its own output* once per short-circuit exit. |
| **phase / files** | P8. New `p8_structure/kuna_boolreturn.rs`; siblings `kuna_iteboolean.rs`; `substrate/funcdata_block.rs:2913`/:3010/:3080; `infra/universalaction.rs` returnsplit group :675/:682/:689. |
| **witnesses** | 8 of the 12 filed hold: `is_colored`, `should_suppress_fork`, `invalid_name`, `validate_dump`, `validate_secy_dump`, `ip_tunnel_parm_match`, `dangerous_locale`, `default_key_compare`. **Drop 4**: `gzvprintf` (non-constant terminal return, its gap is `||`-merging of `return -2` guards -- a condfold case), `check_name_arg` and `DCD_HandleUSBSuspend_ISR` (all-`return 1` chains around a BODY; IDA wins by inverting the chain, a different feature; ghidra scores 0 on one and binja 0 on the other), `abandon_move` (1-guard contiguous tail only). |
| **coverage** | 1,193 of 34,108 O0 panes (3.50%); 291 of 3,067 addressable (9.5%) / 2,847 GED; 2.7x enrichment; perfect on only 85 of 1,116 scored. A stricter independent recount gives 157 cases / 120 distinct functions / 1,832 GED. **The safe (contiguous) subset is 443 of 772 trailing regions -- 57%.** |
| **scope** | small, one module. |
| **gating** | New default-off option. |
| **GED available** | Six witnesses measured to **0** with decbench's own evaluator on hand-edited panes: `is_colored` 14->0, `should_suppress_fork` 16->0, `invalid_name` 16->0, `dangerous_locale` 18->0 (source form), `validate_secy_dump` 30->0, `ip_tunnel_parm_match` 18->0. Strongest generalisation in the round: for each of 157 strict-pool cases, the number of boolean leaves the fold produces was compared to the source's term count from its CFG signature (a source that is one boolean return with T leaves parses to exactly `2T-1` nodes / `3T-3` edges) -- **157/157 exact match, delta histogram {0: 157}**. |
| **residual risk NOT eliminated** | **(i) The filed safety gate is refuted.** "Fire only when the pre-split CFG had exactly one constant-0 and one constant-1 return block" cannot distinguish `return a && b && c;` from the C flag idiom `bool ok=false; if (a&&b&&c) { <work>; ok=true; } return ok;` -- at -O0 both compile to a phi with one 0- and one 1-materialisation. `crazyflie sensorsFindBiasValue` @0x80226e4 is the counterexample and the gate ADMITS it; folding it either hoists 7 statements (2 calls, 4 stores) onto a path the binary skips -- the flight controller writes an unvalidated gyro bias -- or drops them. 236 of 679 both-{0,1} regions (35%) are interleaved, 164 with a call or store. **(ii) The design must instead gate on CONTIGUITY**, which makes the fold exact by C's own short-circuit semantics and needs no purity analysis. **(iii) A hard minimum width of 2 is mandatory and is not in the filed design** -- 21 single-guard panes have a non-boolean condition, and `betaflight ADC_GetFlagStatus` folds to `return a1 & *a0;`, returning 0x10 where the source returns `SET == 1`. **(iv) 19 currently-GED-0 O0 functions carry an all-{0,1} trailing guard run and IDA scores 6-25 on every one of them** (bash `exportable_function_name`, bzip2 `hasSuffix`, crazyflie `memTest`/`kveCheck`, 5x gnutls-cli-debug `test_*`, ssh `compare_forward`, newusers `want_subuids`/`want_subgids`, ...) -- IDA loses them *because* IDA folds. They must be named in the sweep and none may regress. **(v)** Negating a guard by flipping the comparison operator is unsound on floats; `op_flip_in_place_test` (`substrate/funcdata_op.rs:2101`) already refuses FLOAT_LESS/FLOAT_LESSEQUAL, and a hand-rolled negation in a new module reintroduces the bug. **(vi)** `is_colored`, the lead witness, additionally needs `condfold wide` -- `rule_block_or`'s `is_complex` gate refuses the chain -- so returndup work alone does not close it. |
| **design call** | Of the two filed designs, take **(b)**: fold the POST-split contiguous trailing run backwards in the print tree (`if (C) return 0;` => `!C && R`, `if (C) return 1;` => `C || R`), leaving `returndup` alone. It is one module and independent of Rank 1's outcome. Design (a) (decline the split at all three peel sites + extend `iteboolean`) is now partly *demonstrated* by the mechanism refuter -- `returndup off` already produces the fold on 6 functions -- but it emits WRONG C today: `ip validate_dump` comes back as `int8 * sub_7bbb2(...)` with a boolean assigned to a pointer object. If Rank 1 flips returndup at O0, design (a) becomes free and (b) becomes redundant; that dependency is another reason to run Rank 1 first. |

---

### Rank 4 -- `returndup-merge-site-gates` -- BLOCKED on Rank 1

| | |
|---|---|
| **symptom** | `returndup` splits a shared epilogue where the split provably buys nothing -- no goto removed, the merged form already fully structured. |
| **root cause** | One pass, five candidate predicates in `returndup_apply` / `return_split_is_splittable` / `returndup_is_const_ret`. **Two of the five are dead** (see Killed K2, K3). The three that survive: **(ii) merge-site shape** (both arms of ONE if end in the identical bare return, so the split destroys the else), **(iii) tail guard** (the split predecessor is the last top-level component and the "guarded" body is the bulk of the function), **(v) or-chain** (the predecessors are the operand blocks of a short-circuit chain, so the split permanently blocks `rule_block_or`). |
| **phase / files** | P8. `substrate/funcdata_block.rs:2882`/:3010/:3045; `p8_structure/kuna_returndup.rs`; for (ii) also `kuna_dedupitetail.rs:355` + `infra/universalaction.rs:762`/:773. |
| **witnesses** | 9. Cleanest: `setDynaFileCacheSize` and `usb_standard_set_configuration` (tail guard -- `returndup off` matches the source exactly), `USBD_DataInStage` (merge site, margin 13, `returndup off` is structurally identical to both the ida-0 and ghidra-0 panes). |
| **coverage** | 688 of 5,172 O0 functions have their exit count go 1 -> N (a strict lower bound for (ii); cleanflight alone contributes 486); 420 lose at least one `else`; 1,525 of 17,000 loose upper bound for (iii). |
| **scope** | proposal. |
| **gating** | Options + DIV rows. **The unconditional carve-out for sub-shape (iv) is deleted** -- see Killed K2. |
| **GED available** | ~53 points across the nine witnesses if the ablation comes back positive, ~46 of it in the six that converge on an already-0 Ghidra pane. |
| **residual risk NOT eliminated** | **(i)** Two of the five predicates sit close to axes DIV-54 already refuted ("all returns identical" +4,615; "contains a switch" +299, both net wins at O2), so any gate keyed on the return-value SET is disproven and must be re-specified per-merge-site. **(ii)** angr ships the mirror pass (`ReturnDeduplicator`) and still scores 13 on `USBD_DataInStage` -- porting it verbatim is provably insufficient; the mechanism is angr's `dup_all = dup_count >= len(in_edges) - 2 > 0` re-admission, not the dedup rewrite. **(iii)** Sub-shape (ii)'s stated mechanism does not match its own headline witness -- `USBD_DataInStage`'s merge site is the function's unique 5-predecessor exit whose predecessors are three `&&`-chain operand blocks plus two from the far side of a top-level `if`, not "both arms of ONE if". (ii) and (v) are conflated. **(iv)** The DIV-54 const-ret gate is bypassed on **46% of firings** (32% void via the `num_input() < 2 => return true` early-out at `funcdata_block.rs:3027` **[V]**, 14% variable-return via depth-8 transitive phi/COPY resolution), so every "const-ret"-keyed coverage figure in DIV-54 and in this record needs re-deriving. **(v)** 12.6% of "returndup fired / pane changed" counts are banner-only diffs with identical bodies. **(vi)** Suppression is not free: it introduces a NEW goto in 5 of 1,309 changed functions (`coreutils cp abandon_move` 0 -> 1 goto + label + a v1 diamond; `cleanflight sub_802bb38` 0 -> 2). Every gate needs per-site goto-neutrality, not a shape match. |

---

### Rank 5 -- `aggressive-preset-regionedgeorder` (one line) -- *unrefuted*

| | |
|---|---|
| **symptom** | kuna virtualizes a worse edge into a goto than its own alternative ordering would, and `taildup` then duplicates the return tail into the resulting arm. `--mode reliable` closes the witness completely (GED 11 -> 0); the no-flag default does not. |
| **root cause** | `regionedgeorder` (`p8_structure/region_structurer.rs::order_virtualizable_edges_sailr`) violates its own contract ("only changes WHICH goto is chosen when already forced to virtualize; byte-identical on reducible code"): with every post-structuring goto remover disabled, the OFF arm still emits 0 gotos on the witness and the ON arm still emits 1, so the ordering CREATES virtualizations. It reaches users only because `AGGRESSIVE_OVERRIDES` turns it on **[V]** (`p0_knowledge/modes.rs:135`) and `auto` picks aggressive for 768 of 803 binaries. |
| **phase / files** | P8. `p0_knowledge/modes.rs:135` (one line), or the ordering itself. |
| **witness** | `O0-crazyflie-cf2-dwEnableClock`. |
| **coverage** | A/B on 5 binaries / 7,196 functions on today's build: 146 differ; gotos ON 1,670 / OFF 1,635; labels ON 698 / OFF 664; ON has MORE gotos in 32 functions and fewer in 1 (and that one is a goto-heavy O2-style function). At O2 on x86 it is nearly inert; one-sidedly harmful on ARM Cortex-M. Of the 146, 12 are addressable; 7 re-scored with decbench's own GED -- the ON arm reproduces the recorded score EXACTLY on all seven, OFF closes 3 to zero (`dwEnableClock` 11->0, `prvReadMessageFromBuffer` 9->0, `pulseProcessorV1ProcessPulse` 9->0). |
| **scope / gating** | small. **Strict preset change, no new option** -- drop `("regionedgeorder","on")` from `AGGRESSIVE_OVERRIDES`; the option stays available per-run. DIV row + `docs/modes.md` prose. |
| **GED available** | 29 measured across three functions; low hundreds across the ARM Cortex-M projects. |
| **residual risk** | **Unrefuted.** Five binaries establish the O0 sign, not the delta -- publishing the flip needs a full-corpus GED re-measure. This is also the concrete payoff of method correction 2.1: the regression is inside the published number and was invisible to anyone assuming `reliable`. |

---

### Rank 6 -- `tailcallentry` default-flip (free, code exists) -- **SHIPPED (DIV-93)**

> **Resolved.** DIV-93 put `tailcallentry` into `AGGRESSIVE_OVERRIDES` together with the other
> three ARM entry options, which is exactly the "default-flip evaluation only" this row scoped.
> The sweep this row asked for was run over all 110 non-x86-64 decbench twins: `tailcallentry`
> alone adds 649 entries of which 645 are ground truth (99.4%), losing none; all four together
> take entry recall 88.63% -> 93.31% while mid-body false entries fall. The row below is the
> original round-4 filing, kept as the record.

| | |
|---|---|
| **symptom** | Function-boundary overrun at a tail call on a stripped Cortex-M image: the tail-jump target is reachable only through the jump, so the Listing walk never creates an entry and the caller absorbs its body. |
| **root cause** | Not a defect -- `tailcallentry` (P1, PR #259, step 3 of approved [PROPOSAL] #239) was written for exactly this and ships default-OFF, and `aggressive` does not carry it. |
| **phase / files** | P1. `p0_knowledge/modes.rs` (preset) or the option default in `phases.toml` + `p0_knowledge/options.rs`. No new code. |
| **witness** | `O0-riot-os-hello-world-reset_handler_default` (all three opt-level siblings, 8 GED each, verified CLOSED on today's build with the flag). |
| **coverage** | riot-os hello-world 113 -> 124 functions (+11 entries, 12 existing change, 31 of 124 end in a recovered tail call). **coreutils sort (x86-64): 479 -> 479, ZERO functions changed** -- the blast radius is precisely the embedded/stripped targets it was written for. |
| **scope / gating** | small. Existing option, default-flip evaluation only: 0/675 datatest assertions, speed budget, DIV row. |
| **GED available** | 24 across the three group cases; the real value is in the separate 1,552-function recall pool. |
| **residual risk** | **Unrefuted.** Arguably kuna+`tailcallentry` is BETTER than IDA here (IDA renders `JUMPOUT(0x800051C)`; kuna emits `sub_800051c(); // tail-call`). Before anyone writes a structurer feature, sweep the rest of the O0 embedded-ARM pool with this flag on -- it may close more for free. |

---

### Rank 7 -- `spinwhile` -- **DEMOTED, not free**

Filed as "the do-this-first pick on effort, semantics-preserving by construction, the only
cluster with no correctness axis to argue about." Two of three refuters returned
*survives: false*, and the synthesis step verified their central measurement.

| | |
|---|---|
| **symptom** | An empty-body spin-wait renders bottom-tested: `do { } while (cond);` where the source and IDA write `while (cond);`. |
| **root cause** | `PrintC::emit_block_do_while` **[V]** (`p9_emit/printc.rs:4130`) unconditionally prints `do {`, the body under NO_BRANCH, then `} while (cond);`, never checking whether the body emits a statement. Site confirmed and uncontested. |
| **coverage** | **[V] verified by the synthesis step, and the filed figures are right**: 212 of 34,108 O0 panes carry an empty `do { }`; **69 addressable / 860 GED** (the refuters' stricter 45/586 recount used a tighter regex). Highest enrichment of any signature in the round (3.6x). |
| **GED available** | **Much less than 860.** Measured with the real evaluator, neither witness reaches 0: `i2c_er_handler` 18 -> 5 (the residual is condition folding) and `SetSysClock` 26 -> 10 (its third spin loop carries a live statement and the predicate must not touch it; only fixing all three reaches 0). |
| **residual risk NOT eliminated** | **(i) The payoff is a pyjoern parser artifact, not a structural recovery.** `while (c) ;`, `while (c) { }` and `while (c) { ; }` all parse to **1 node / 0 edges** -- the loop vanishes from the CFG -- while `do { } while (c);` parses to 3 nodes / 3 edges. GED here rewards erasure. **(ii) It hands back verified wins.** **[V]** 8 currently-GED-0 O0 kuna panes carry the signature and **IDA scores 10.0 on every one**: `console_putc` in all six libopencm3 binaries, `lcd_dma_init`, `nuttx clock_get_sched_ticks`. I read `usart_console`'s panes: kuna emits `do { } while (!(*dat_8000220 & 0x80));` (GED 0.0), IDA emits `while ( (MEMORY[0x40011000] & 0x80) == 0 ) ;` (GED 10.0), and the source is a **genuine** `do { reg = USART_SR(...); } while (...)`. The rewrite converts kuna's winning, more-faithful pane into IDA's losing one. **(iii) No predicate can discriminate**: at -O0 `while(c);` and `do{}while(c);` compile to byte-identical machine code, and kuna reaches the empty body by copy-propagating the source's real body statement into the condition. **(iv) The prescribed predicate deletes loop bodies.** "Guard on PRINTABLE statements" -- the emitter's only definition of that (`emit_basic_block_ops`, `printc.rs:4223-4305`) counts zero for a body whose sole content is `if (c) break;` or `if (call(...) != -1) goto L;`, because the CBRANCH is `is_branch()`, the call's result is `is_implied()`, and `break`/`goto` are `BlockGoto` sblocks carrying no op. 41 such sites / 34 functions, verified fresh: `libopencm3 console_getc` becomes an unconditional infinite UART spin (firmware hang), `tar sys_wait_for_child` loses its `waitpid()` call to an infinite errno loop, `betaflight micros` loses the inner sysTick consistency spin. That 42 sibling bodies stay safe is luck -- `CPUI_RETURN` carries `pf::returns` and not `pf::branch`. |
| **verdict** | Net at O0 is roughly **+69 addressable functions / partial credit** against **-8 verified perfect-today functions at -10 each**, i.e. positive by function count but far smaller than filed, on a Joern-quirk payoff. If it ships: default-OFF, predicate = "the NO_BRANCH body emit produced no output" (never an op count), reject any compound body (`BlockIf`/`BlockList`/`BlockCondition`/`BlockGoto`/nested loop) outright, and the record must name all 8 regressions. Strike "no correctness axis", "semantics-preserving by construction", "the sweep is a text sweep", and "all three pre-test spellings score 0". |

---

### Rank 8 -- two strict correctness bugs in default-ON structuring passes -- *unrefuted*

Ranked on correctness, not coverage. Both would be caught by the same class of tree-level
invariant, and building that invariant is worth more than either fix.

**8a. `branchflip-armswap-loses-arm-siblings`** -- SEMANTIC DEFECT. The default-on
`branchflip` arm swap hoists the second of two sibling `if` blocks out of an else arm, so it
executes on the then-path too: kuna calls `IOInit(rxIO)`/`IOConfigGPIOAF(rxIO)` on a hardware
path the machine code never takes (`cleanflight serialUART`, verified against the Thumb
disassembly at 800bab2/800bada/800bb06-800bb22). Site: `Funcdata::block_if_flip_negated_guard`
(`substrate/funcdata_block.rs:3693`) does `split_flip_in_place_execute` + `swap_blocks(sif,1,2)`;
after the swap the printer takes the else-if collapse path (`p9_emit/printc.rs:3508`), which
only fires when the else component IS a single BlockIf -- so an arm that was a BlockList of
two BlockIfs lost its tail child. **Mechanism not instrumented**: three x86-64 reproducers of
the same source shape did NOT corrupt. The ablation is unambiguous -- 14 other structuring
options swept individually keep the escape; only `branchflip off` removes it, and the off pane
matches ida and ghidra exactly. Strict bug fix, no flag (`branchflip` stays default-ON).
**The deliverable that matters most is the guard rail**: the flip is a pure arm swap, so a
debug assertion that the multiset of leaf blocks reachable under the BlockIf is unchanged
across `block_if_flip_negated_guard` would have caught this and would police the whole family.
An automated audit found 128 of 792 branch-flipped functions on that binary containing a
statement that becomes strictly shallower -- but that is a candidate set, not a defect count
(3 sampled gave 2 legitimate re-nestings and 1 disagreement where branchflip-OFF was the
hoister). A sound audit needs a tree-level checker, not text.
**Note the adjacent, independently-found instance**: `ip netns_add` @0x281bb reads `argv[1]`
on the create path on today's default (source `ipnetns.c:829` has it inside the `else`;
`2821d: jg 282e5` jumps past it), and `branchflip off` re-nests it. A guard-weight detector
flags 25 such functions on `ip`'s default arm. Same mechanism, same fix, second witness.

**8b. `regionstructure-exitless-loop`** -- one spurious goto + label on an EXITLESS `while(1)`
loop: the region structurer has no successor to hand the region, elects one from the region's
own members (the if-body, inside the natural loop), and forces `if (!v1) break;` plus
`goto label_...;` back in. Confirmed by construction: `while(1){ if(g()==0) h(1); s(2000); }`
reproduces; adding any real exit removes it. Ghidra scores 0 and kuna's `regionstructure off`
pane is byte-identical to Ghidra's. Strict bug fix. Measured A/B over dexter.dll + cronie
crond (418 functions): 3 functions where `regionstructure` ADDS gotos, 1 matching the
signature. Low frequency on desktop ELF, but exitless `while(1)` dispatch loops are the
standard shape of firmware main loops and RTOS tasks -- **re-run the same A/B on one Cortex-M
firmware; that measurement is the go/no-go.**

---

### Below the line

Real, but each is either unrefuted-and-small, blocked on something else, or measured to move
nothing shipped.

| # | slug | phase | one-line | coverage | scope | gate | why below the line |
|---|---|---|---|---|---|---|---|
| 9 | `switchfoldedindex` + swallowed-warning | P2 | An O0 state-machine dispatch loses a whole switch AND its loop to `(*(code *)(...))()` because the switch variable folds to a constant; 179 GED on one function. | 10 non-PLT functions in 54,591 (0.02%) -- rare but catastrophic; fully diagnosed with a controlled reproducer isolating the trigger to one bit. | small | option `switchfoldedindex`, 4 siblings to copy | Ghidra fails identically (170 vs 179), so the kuna-vs-ghidra split is noise. **Ship the sub-fix now, standalone, unblocked**: `stage_jump_table` maps `Err(e)` to `FailNormal` and drops the text where C++ `FlowInfo` calls `warningHeader(err.explain())` -- a failed table is indistinguishable from a genuine computed call in the pane. Few lines, no option, makes every future jump-table triage cheaper. |
| 10 | `callretpassthrough` | P4 | Prototype recovered as `void` when EVERY return's value is the direct output of a call, so `return f(x);` renders as two statements. | 21 of 785 functions (2.7%) on one binary vs DWARF ground truth; loose probe 66 of 793. | small | new default-off option | GED-blind by construction (the campaign's own calibration says GED cannot see types); its real value is `type_match` and readability. Blast radius is the whole corpus -- a prototype flip changes call sites everywhere. Least measured cluster here: one binary, one 4-function reproducer. |
| 11 | `condfold-reachability` | P8 | Three gates decline a multi-clause `||`/`&&` fold; **gate (a) is the round's only outright code bug** -- `printed_shape` (`kuna_condfold.rs:411`) tests `Varnode::is_implied()`, but `ActionMarkImplied` runs strictly AFTER every structuring pass that calls condfold, so no varnode is ever implied and the "printed-width budget" degenerates into counting raw p-code ops, ~4x over. | Directly measured: 123 declines on `libopencm3 cdcacm` and 122 on `coreutils cat` attributed to "over the printed-width budget". | small | rides the existing `condfold` option | **`condfold` is default-OFF and is one of the six options `aggressive` does not carry, so none of this moves the shipped default or the benchmark column** until a separately measured default flip. Widening the pass's only bound on rendered condition width makes that flip harder to justify, not easier. Compose after Rank 2A: on `dispatch_protocol_error`/`ssh_connection_hash` the guard swap gives the order and the fold gives the condition. |
| 12 | `loopcontinue` | P8 | **[V] kuna has emitted `continue;` zero times in 34,108 O0 panes** (ghidra 0; ida 1,833, binja 4,001, angr 1,388). The machinery exists -- `refine_loop_edges` (`region_structurer.rs:2295`) sets `f_continue_goto`, and the printer path (`printc.rs:4207`) is wired -- but `match_cyclic_schemas` tries `try_fold_loop` first for every head and returns on first success, so it is never consulted. | ~1,046 O0 functions (3.1%) across two renderings. No test in either corpus asserts a `continue;`. | proposal | needs a settled site first | The record's own ablation argues against its own diagnosis (`regionstructure off` is byte-identical on the witness while changing 2 of 147 in the same binary). **One instrumented experiment converts this to a ranked feature**: route a >1-latch head through `refine_loop_edges` BEFORE `try_fold_loop`; if `nextmatch` moves, the fix has a home in one module. Same "dead ported pass" pattern this repo has mined productively before. |
| 13 | `returndup-declines-on-variable-returns` | P8 | The mirror of Rank 4: the gates are const-only, so a mixed or all-variable return phi falls between them and neither can reach it. | Smallest real cluster: 37 of 1,665 for one variant, 5 honest residual instances for another, 20 of 2,111 for the third (and reading them shows the parameter test is not a discriminator). | small | three separate options | Points the gate in the OPPOSITE direction from Rank 4, loosening exactly the predicate DIV-18 tightened at a measured ~976 GED-perfect cost. If anyone takes it, take `earlyretself` alone -- its certificate ("the value returned on the peeled edge is the very varnode the guard tests") cannot re-open DIV-18, and it is the only one of the three with a witness measured to 0. Do NOT bundle with Rank 4. |

---

## 4. Killed

Not an apology. Nine of the twelve entries below were killed by a measurement that also
tells the next round something.

| # | what died | killing evidence |
|---|---|---|
| K1 | **`spinwhile` as "the free win"** ("semantics-preserving by construction", "no correctness axis", "the sweep is a text sweep") | **[V]** 8 currently-GED-0 O0 kuna panes carry the signature and IDA scores **10.0** on all 8 (`console_putc` x6 libopencm3, `lcd_dma_init`, `clock_get_sched_ticks`); the source for `console_putc` is a genuine `do { reg = USART_SR(...); } while (...)`, so the rewrite converts kuna's winning, more-faithful pane into IDA's losing shape. Separately, the prescribed "printable statements" predicate empties 41 bodies containing `if (c) break;` / `if (call()) goto L;`, hanging firmware. Feature demoted, not deleted (Rank 7). |
| K2 | **`returndupsideeffect`'s unconditional correctness carve-out** ("ship regardless of the ablation, it is a memory-visibility question") | Refuted independently by all three lenses. `node_split_block_edge` (`funcdata_block.rs:2742`) reroutes exactly ONE in-edge and a RETURN block has no out-edges, so clone and original are on mutually exclusive paths -- the cloned `COPY -> global` executes exactly once, as before. Volatile accesses lower to CALLOTHER user-ops (`p2_lift/userop.rs:101`/:103) and are rejected by the whitelist's `_ => return false`, so a genuine MMIO write can never reach a split block. 4 duplicated global stores in 151 changed panes (2.6%), all ordinary .bss on x86. It is a readability gate like the other four and must wait for the ablation. |
| K3 | **`returndupuniform` (per-edge uniform value)** | Refuted by DIV-54's own flagship win. `coreutils factor` is a **void** function emitting `// return-dupe x4` -- every duplicated return is a bare `return;`, so it is per-edge uniform by construction -- and `returndup off` reverts it to the merged form recorded at GED 12. `bash expr_unwind` is also void + uniform and is a 0->6 **loss**. Same axis, opposite signs. What actually separates them is whether the shared epilogue block emits a visible statement. |
| K4 | **`iteregion` / ternary re-roll as a feature (7 cases)** | Three independent measurements. Enrichment is **negative** (792 of 34,108 O0 panes carry a `// ternary` banner = 2.32%, but only 56 of 3,067 addressable = 1.83%, i.e. 0.79x). The ablation is **net-positive** (O0 zlib example, 150 scored: GED sum 860 ON vs 931 OFF, perfect 64 ON vs 61 OFF; of the 9 that move, 7 get WORSE with it off). And two of the seven are pure metric granularity -- pyjoern gives a `?:` its own diamond only when the arms are non-trivial, so GED systematically rewards de-sugaring a source ternary, and on `timerChConfigIC`/`beeperInit` kuna is CLOSER to the source than IDA and is charged 13 and 11 for it. Close all seven. |
| K5 | **`truthycond` / bare-identifier short-circuit collapse (3 cases) as a kuna change** | A decbench/pyjoern defect: Joern does not split a short-circuit whose operand is a bare IDENTIFIER, while every other expression form does. Minimal pair on kuna's own pane: `&& (v2)` -> 3n/3e GED 8; `&& (v2 != 0)` -> 5n/6e GED 0; same operand on the LEFT -> GED 0. kuna is not an outlier (kuna 1.24%, ida 1.52%, angr 1.53%; only ghidra escapes at 0.07%, by always printing the explicit comparison). **Route to decbench** -- one rewrite in `sanitize_decompiled_c` applied to every decompiler, then re-score. It should land before more "kuna-specific, structure looks identical" cases are mined, because this quirk manufactures exactly that bucket. |
| K6 | **"returndup fires" as a single cluster** | The banner's enrichment is **1.15x** (10,360/34,108 panes = 30.4% vs 1,066/3,067 addressable = 34.8%) -- and **[V]** 41.3% of the 2,938 kuna-perfect/ida-not functions carry it too, a *higher* rate than the losing set. The banner is not a loss signature; the sub-shapes are, and they want five predicates and two different verdicts. A single "fix returndup" PR would have been the worst possible outcome of the round. |
| K7 | **`loopcondhoist`'s filed root cause** (`cb.size_out() != 1` at `region_structurer.rs:1788`) | Minimal pair: probe `p7` has exactly the second out-edge the diagnosis blames (a source-level `break` in the body) and hoists CORRECTLY; `p20`, identical but with one call moved out of the break arm, does not. Relaxing the guard is a no-op on both witnesses, and `regionstructure off` is byte-identical -- the live site is `blockaction.rs`, not the region structurer. Replaced with a smaller and safer diagnosis (Rank 2). |
| K8 | **`loopcondhoist` form A as "trivially safe, ship first"** | Form A is 9 of 432 addressable / 62 GED (**2.1%**), closes neither witness, and comes from a different code path (the `is_complex(head)` overflow-syntax branch, where the while-do rule already SUCCEEDED). Promoting those heads emits wrong C on `mksyntax main` @0x19f7 and 91 measured siblings: `while (v1 != -1)` reads `v1` before `getopt` assigns it and drops the call out of the loop. The designated safe first step was the one form that silently produces wrong output. |
| K9 | **`boolreturn`'s filed root cause and its safety gate** | Root cause: `iteboolean` is reachable and DOES re-roll `return` diamonds -- `--option returndup off` makes it fire and log `re-rolled 1 0/1 select diamond(s)` on 8+ functions. This is a DIV-51/DIV-54 pass collision, not a missing capability. Gate: the one-const-0/one-const-1 pre-split test admits the C flag idiom (`crazyflie sensorsFindBiasValue`), whose fold hoists 2 calls and 4 stores onto a path the binary skips. Replaced by a contiguity requirement + minimum width 2. |
| K10 | **`returndup-o0`'s filed root cause** ("gcc -O0 does no cross-jumping, so a shared epilogue means the source shared it") | Disassembly refutes it directly: `int f(int x,int y){if(x)return 1;if(y)return 2;return 3;}` at gcc -O0 emits three `mov $K,%eax; jmp .L` blocks and ONE shared `pop %rbp; ret` of in-degree 3 -- a shared epilogue with N in-edges is the DEFAULT -O0 encoding of N distinct source returns. Corpus witness: `iproute2 ip do_iptunnel` has 8 source returns, all merged by -O0, and returndup ON reproduces all 8 while OFF produces a 7-deep nested cascade. The batch0 probe that "proved the -O0 half" measured the value-materialisation blocks, not the epilogue returndup splits. |
| K11 | **The `returndup off + condfold wide` third arm** | Measured inert by two independent lenses: byte-identical to plain `returndup off` on `dangerous_locale` and `detect_data_type`; `condfold wide` alone leaves `detect_data_type` byte-identical to baseline. The `||`/`&&` folding that returns in the OFF arm comes from baseline `rule_block_or`, not condfold. |
| K12 | **batch7's "gate the shared `return_split_is_splittable`/`node_split` seam, not one pass"** | False on 7 of 9 witnesses: `returndup off` and `returndup off + earlyreturn off + switchreturn off` are byte-identical on 7, and on the other 2 plain `returndup off` is the BEST configuration. Worse, the seam's other consumer (upstream `ActionReturnSplit`) is the only one that removes gotos -- measured: returndup removes 0 gotos in 132 body-changed panes and adds 4 -- so a shared-seam gate would narrow the wrong consumer. The safe cut is returndup-local. |
| K13 | **`noreturnguard`'s owning-file list and its "prefer the halt arm" predicate** | **[V]** There is no twin in `region_structurer.rs` -- `rule_block_if_no_exit` appears exactly 3 times in the workspace, all in `blockaction.rs`, and `regionstructure off` is byte-identical on all 8 witnesses. The predicate fires on ~84% single-test panes that are already correct and inverts two verified-correct functions (`tail xlseek`, `make-prime-list xalloc`). Also killed: "Ghidra never hits the tie" (7,326 of 29,649 Ghidra O0 panes carry the no-return warning; Ghidra marks all four `dh_info` fatal callees and still emits the flat shape) and the 2,912-GED claim (honest band 1,100-1,600). Feature survives, re-specified. |
| K14 | **`loop-noreturn-exit-demote` (`initbuf`) as a standalone feature, and as a Rank-2 free rider** | 4 of 34,108 O0 panes carry the signature, exactly 1 addressable. IDA's GED-0 pane wins by NOT knowing the callee is no-return, and Joern's source CFG is equally no-return-blind, so IDA's naive rendering is isomorphic to the source by accident; Ghidra knows the fact too and pays a goto. Real kuna-specific ground is 12 -> 6, not 12 -> 0. And it is not a free rider: `initbuf`'s `if (*a0) {...}` has one terminal arm and one that loops back, so no tie exists and the tie-break cannot touch it. |
| K15 | **`peimportcall` / PE boundary as a cluster** | Already fixed by PR #254 before the round opened: 497 of 501 unnamed IAT-slot call sites bind, all 5 previously-overrunning mydoom functions fixed, and `sync_visual_th` **already scores 0.0 in the current tree** (its recorded 39 is an Aug-3 pool value against an Aug-9 score). One residual worth its own case: `LOCK(); UNLOCK();` with the atomic read-modify-write missing between them (3 empty pairs of 5 on mydoom.exe) -- a wrong-value defect that adds no CFG node and is invisible to GED entirely. |
| K16 | **Grouping by project** (the 22-case embedded-ARM cluster) and **"needs-proposal" as a bucket** | Project is not a root cause -- every mechanism reproduces on x86-64 ELF except the spin-wait idiom, which is a hardware-polling pattern, not an architecture property. "needs-proposal" is a confidence statement about the filing agent: its five members are Rank 3 + `condflat`, Rank 2A composed with #11, K14, Rank 4 and #12 -- grouping on it would have hidden four tractable items inside one intimidating one. |
| K17 | **Merging the duplicate slugs** | Six agents filed four names (`boolretfold`, `boolreturn`, `booleanreturn`, `boolexprreturn`) for one symptom; three filed `returntail`, `noreturnguard`, `noretguard` for one line; two filed `spinwhile` and `emptyloop-toptest` for one rewrite. Recorded so the menu is not read as 20 features. |
| K18 | **`condflat` -- DEFERRED, not killed** | Printing `&&`/`||` chains left-associated instead of as the balanced tree `rule_block_or`'s pairwise fusion produces. Zero semantic risk, P9-only, measured at 6 GED on `default_key_compare` at IDENTICAL node and edge counts (VJ GED charges the degree profile). One witness, one measurement; 1,520 O0 panes carry a nested 4+-operand condition but that subpopulation is confounded. **Needs one independent second measurement; if it holds it is the cheapest zero-risk item in the whole backlog.** |

---

## 5. What to do next round

**Build the bidirectional sweep harness, and make it the campaign's default instrument.**

Not another mining pass. The round's own arithmetic says the signal is the bottleneck:
**[V]** the net O0 deficit among functions both tools scored is **129 functions**, while every
cluster in this menu is 300-430 functions wide. Nothing in this file can be decided by the
witnesses it names, and the mining pipeline produces only one direction of evidence -- it
mines losses and never prices the wins the same change would destroy.

Concretely, one tool that takes an option flip or a patched binary and reports, over the whole
O0 slice (or any named slice):

1. **functions moved to perfect** and **functions moved off perfect**, by name, both directions;
2. the summed GED delta split by those two populations;
3. per-project and per-architecture breakdown;
4. the changed-function diff set for the standing-requirement-8 audit, pre-classified by
   shape (goto delta, `while(true)` delta, callee/string multiset delta after comment
   stripping, "terminates in arm A but falls into a non-return statement in arm B").

Every decisive finding this round came from a refuter hand-rolling a fragment of that tool,
and each fragment killed or re-specified a cluster: `console_putc` 0 -> 10 (K1), `factor`
(K3), `mksyntax main` (K8), `sensorsFindBiasValue` (K9), `do_iptunnel` (K10), the
2,938-function control (K6), the 19 currently-perfect `boolreturn` functions. Three of those
were found only because one agent thought to look at the mirror population. That should not
depend on which agent is assigned.

It also converts standing requirement 8 from prose into a gate, and it is the precondition
for Rank 1 -- the returndup O0 ablation is exactly this tool's first job, and running it any
other way produces a number that cannot be defended.

**Second, cheap, and unblocks the two largest clusters at once:** the
`rule_block_if_no_exit` decision trace specified in Rank 2. One `KUNA_RS_DEBUG`-style print,
run on seven named functions and three probes, settles whether the round's #1 code item is an
arm predicate, a scan-order predicate, or both -- and three separate refuters, working
independently, all asked for it.

**Third, free:** fix `docs/decbench-loop.md`'s claim that the benchmark ran
`option listing on` = `--mode reliable` (method correction 2.1), and add the missing O0 row
caveat to `returndup`'s entry in `docs/options.md` (2.2). Both mislead every future triage
agent on their first read.
