---
case_id: O2-noinline-dpkg-dpkg-pkg_infodb_remove_file
group_id: dpkg::pkg_infodb_remove_file
status: feature-candidate
tier: M
margin: 88
bucket: kuna-specific
fresh_reproduces: true
fresh_ged: null
option_closing: null
root_cause_family: noreturn-propagation-stripped
feature_slug: noreturn-reach-propagate
scope: small
angr_ref: angr.analyses.cfg.cfg_base.CFGBase._determine_function_returning (via _analyze_function_features callgraph fixpoint)
kuna_stage: kuna-analysis s1_noreturn_propagate (Listing consumer, analysis tier)
---
## Side-by-side

angr (stored) — matches the true source exactly (dpkg `pkg_infodb_remove_file` is a 5-line function):

```c
// Function: pkg_infodb_remove_file @ 0x19010
int pkg_infodb_remove_file(unsigned long a0)
{
    if (unlink(a0))
        sub_423e50(dcgettext(NULL, "unable to delete control info file '%.250s'", 5), a0); /* do not return */
    return sub_421820(2, "removal_bulk info unlinked %s", a0);
}
```

kuna (stored == fresh, byte-identical; trimmed to the first ~40 of 86 lines — the rest is more of the swallowed next function, ending in `execlp("dpkg-split",...)` + `__stack_chk_fail()`):

```c
// Function: pkg_infodb_remove_file @ 0x19010
char * pkg_infodb_remove_file(void *a0)
{
  ...21 locals elided...
  if (unlink() == 0) {
                    /* WARNING: tailcalljump: recovered tail call -> introduced call to 0x00021820 */
    v2 = (char *)sub_21820(2,"removal_bulk info unlinked %s",a0);
    return v2;
  }
  v3 = (int8 *)dcgettext(0,"unable to delete control info file \'%.250s\'",5);
  sub_23e50();                     /* <-- ohshite; real function ends here */
  v23 = *(void *)(v20 + 0x28);     /* <-- from here on: the NEXT function (deb reassembly code) */
  if (dat_51528 == 0) {
    v7 = sub_1e470("reassemble.deb");
  }
  if ((unlink(v20) != 0) && (v4 = (int4 *)__errno_location(), *v4 != 2)) {
    v12 = (char *)dcgettext(0,"error ensuring \'%.250s\' doesn\'t exist",5);
    sub_23e50(v12,v20);
    ...
  }
  sub_242c0(sub_9af0,0xffffffff,1,v20);
  v1 = sub_2c7d0();
  if (v1 != 0) {
    v1 = sub_2c840(v1,"dpkg-split",8);
    ...
      goto label_191bf;
    ...
  }
label_191bf:
  execlp("dpkg-split","dpkg-split",0x3ce36,dat_51528,*v18,0);
  ...
  __stack_chk_fail();
}
```

kuna fresh with `--option listing on --option noreturn_disc on`: partial only — `sub_23d80` (dpkg `ohshit`) gets discovered no-return (the `goto label_191bf` disappears, the swallowed tail restructures), but `sub_23e50` (`ohshite`) is still treated as returning, so the boundary overrun and ~80 lines of foreign code remain. GED gap not closed.

Metrics: angr 7 loc / 1 if / 0 goto; kuna (stored and fresh, defaults) 86 loc / 9 ifs / 1 loop / 1 goto.

## Analysis

**Symptom.** kuna decompiles past the call to `sub_23e50` — dpkg's internal `ohshite()` error routine (stripped, so unnamed) — and swallows the entire next function (deb-reassembly code: "reassemble.deb", `execlp("dpkg-split",...)`). The real function is 5 source lines; kuna emits 86. angr marks the same call "do not return" and stops, matching the source CFG exactly (GED 0). ida=0, phoenix=0, ghidra=6 (ghidra's discovered-no-return analyzer catches it, with minor residual diffs) — kuna-specific, not inherited. Fresh kuna (current main, post-Jun-29 SAILR PRs) reproduces byte-identically: not fixed.

**Root cause.** `ohshite` is `DPKG_ATTR_NORET`: it formats the errno message in a retry loop and calls `run_error_handler()` (0x23c60, which ends in `call exit@plt` / `__longjmp_chk@plt`). Disassembly of 0x23e50 shows the shape that defeats every existing kuna knob: the `call run_error_handler` sits **mid-body** at 0x23f08; the code after it (0x23f0d–0x23f59, `strerror`/`__asprintf_chk` message-building) is reachable via a conditional `jg 23f0d` and **jumps back** to 0x23f08 (`jle 23f08`, `jmp 23f08`). The function contains **no `ret` at all** — every path ends at that one noreturn call — but its *last-by-address* real instruction is `jmp 23f08`, not a call.

**Why angr wins.** angr's `CFGBase._analyze_function_features` / `_determine_function_returning` (pinned venv cfg_base.py:1067/1177) computes `Function.returning` by **CFG reachability over the callgraph fixpoint**: `exit`/`longjmp` SimProcedures are noreturn → `run_error_handler` has no returning endpoint → noreturn → `ohshite` has no return site and its only call successor is non-returning → `returning=False` → the fall-through edge after `call ohshite` in `pkg_infodb_remove_file` is dropped. A reachability formulation is insensitive to where in the body the terminal call sits.

**Why existing kuna options don't cover it** (verified by running them):
- `noreturn_extern`/`noreturn_externmatch`/`noreturn_known` — name-list based; `sub_23e50` is an internal, stripped function with no name.
- `noreturn_propagate` (default-on, listing-gated) — Rule 1 in `decompiler/crates/kuna-analysis/src/s1_noreturn_propagate/mod.rs` requires the **last real instruction** to be a CALL/tail-JMP to an already-terminal callee. Works for `ohshit`/`ohshitv` (their last instruction is literally `call run_error_handler`) — confirmed: with `listing on` the `sub_23d80` goto disappears — but `ohshite`'s last instruction is a backward `jmp`, so Rule 1 fails.
- `noreturn_disc` (>=3 call-site evidence tally, Ghidra FindNoReturnFunctionsAnalyzer analog) — of the 125 `call ohshite` sites in the binary, only **1** has a fall-through landing on another function's entry (GCC lays cold blocks after noreturn calls, so the other 124 fall-throughs decode as valid instructions of the same/adjacent function). 1 < 3, so the tally never fires.
- Additionally, the whole listing-consumer family is a no-op in a default run: `listing` is default-OFF, so the benchmark run had **zero** internal-noreturn discovery.

**Siblings.** None (siblings list empty). Same family as the coreutils `xalloc_die` case (noreturn-propagation-stripped), but a strictly harder shape: there the wrapper's last instruction is the terminal call; here it is not.

## Proposed fix

**Mechanism (small, one option-gated pass extension per pipeline rule 1).** Generalize `s1_noreturn_propagate` from the "last-real-instruction" pattern to angr's reachability formulation, as a new sub-rule (option `noreturn_reach`, or a Rule-1b inside `noreturn_propagate`): over the Listing body of each candidate, build the intra-function flow, **cut the fall-through edge of every CALL whose target is in the terminal set**, then conclude no-return when (a) no `RETURN` instruction is reachable from entry, (b) every remaining sink is a call/jump to a terminal callee, (c) no computed jump and no flow escaping the body. Iterate in the same existing fixpoint (chain here: `exit@plt` → `run_error_handler` → `ohshite`). Emits the existing `NoReturnFact` → `set_function_no_return` commit seam — no new S7 work.

**Owning files.** `decompiler/crates/kuna-analysis/src/s1_noreturn_propagate/mod.rs` (the reachability rule; ~all of the needed body/flow data is already collected for Rules 2–4), `decompiler/crates/kuna-decomp/stages.toml` (new settable entry) + the catalog count tests per the adding-option recipe.

**Risks.** A reachability rule is more aggressive than last-instruction matching: a function that loops forever by design (event loop) with no ret would be classified noreturn — technically true but changes callers; mitigate by keeping condition (b) (every sink must be a terminal *call*, an infinite loop with no terminal call sink does not qualify). Padding/data-in-body mis-decode could fake reachability — same exposure the current Rules 2–4 already have.

**Default-on viability.** The pass itself is cheap and parity-safe (real-ELF Listing path only, so all 675 datatests byte-identical — same argument as `noreturn_propagate` DIV-14). The real gate is that it only fires under `listing on`, which is default-off for decode cost; flipping `listing` default-on is a separate speed-policy decision (<=5% budget must be measured on decompile-all). Recommend: ship `noreturn_reach` default-on but listing-gated (identical posture to `noreturn_propagate`), and evaluate `listing`-by-default separately — without that, benchmark defaults still miss this case.
