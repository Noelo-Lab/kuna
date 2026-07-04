---
case_id: O2-noinline-dpkg-dpkg-spawn_shell
group_id: dpkg::spawn_shell
status: feature-candidate
tier: L
margin: 186
bucket: kuna-specific
fresh_reproduces: true
fresh_ged: null
option_closing: null
root_cause_family: noreturn-propagation-stripped
feature_slug: noreturn-returning-fixpoint
scope: small
angr_ref: angr.analyses.cfg.cfg_base.CFGBase._determine_function_returning (via _iteratively_analyze_function_features)
kuna_stage: S1 external-refinement (kuna-analysis s1_noreturn_propagate, listing consumer) -> S2 flow artificial-halt
---

## Side-by-side

angr (stored) — complete, 32 loc, matches dpkg source `spawn_shell` exactly:

```c
long long spawn_shell(char *a0, char *a1)
{
    ...
    v1 = dcgettext(NULL, "Useful environment variables:\n", 5);
    fputs(v1, stderr);
    fputs(" - DPKG_SHELL_REASON\n", stderr);
    fputs(" - DPKG_CONFFILE_OLD\n", stderr);
    fputs(" - DPKG_CONFFILE_NEW\n", stderr);
    v2 = dcgettext(NULL, "Type 'exit' when you're done.\n", 5);
    fputs(v2, stderr);
    v9 = sub_42c7d0(v3, v4, v5, v6, v7, v8);
    if (!v9)
    {
        setenv("DPKG_SHELL_REASON", "conffile-prompt", 1);
        setenv("DPKG_CONFFILE_OLD", a0, 1);
        setenv("DPKG_CONFFILE_NEW", a1, 1);
        sub_41e380(0, dcgettext(NULL, "conffile shell", 5)); /* do not return */
    }
    return sub_42c840(v9, dcgettext(NULL, "conffile shell", 5), 4);
}
```

kuna (stored == fresh, byte-identical; 147 loc / 6 gotos / 21 ifs / 2 loops — trimmed to the seam, ~110 lines of the swallowed neighbor cut):

```c
char * spawn_shell(unsigned long a0,unsigned long a1,unsigned long a2,uint4 a3,uint8 a4)
{
  ...
  fputs((char *)dcgettext(0,"Useful environment variables:\n",5),v1);
  fputs(" - DPKG_SHELL_REASON\n",dat_511c0);
  fputs(" - DPKG_CONFFILE_OLD\n",dat_511c0);
  fputs(" - DPKG_CONFFILE_NEW\n",dat_511c0);
  fputs((char *)dcgettext(0,"Type \'exit\' when you\'re done.\n",5),Unique10000691);
  v2 = sub_2c7d0();
  if (v2 != 0) {
    v4 = dcgettext(0,"conffile shell",5);
                    /* WARNING: tailcalljump: recovered tail call -> introduced call to 0x0002c840 */
    return (char *)sub_2c840(v2,v4,4);
  }
  setenv("DPKG_SHELL_REASON","conffile-prompt",1);
  setenv("DPKG_CONFFILE_OLD",a0,1);
  setenv("DPKG_CONFFILE_NEW",a1,1);
  v5 = (char *)dcgettext(0,"conffile shell",5);
  v9 = (char *)0x0;
  sub_1e380();                      /* <-- command_shell, noreturn-by-body; kuna keeps going */
  tcflush(0,0);                     /* <-- from here on: the NEXT function (the conffile   */
  fputs("\n",dat_511c0);            /*     prompt loop at 0xcc50), swallowed wholesale:     */
  v4 = 5;                           /*     do{}while(true), 6 gotos, 4 labels, getchar loop, */
  if (strcmp(v9,v5) != 0) goto label_cec0;  /* Y/I/N/O/D/Z prompt, ~110 more lines          */
  __fprintf_chk(dat_511c0,1,dcgettext(0,"Configuration file \'%s\'\n",5),v9);
  do {
    ...  /* [~110 lines of promptconfaction body trimmed] */
  } while( true );
}
```

kuna (fresh with `--option listing on --option noreturn_disc on`, i.e. the full existing noreturn machinery incl. default-on noreturn_propagate now armed): 146 loc / 5 gotos — still swallows the neighbor; neither pass concludes 0x1e380.

## Analysis

Symptom: kuna decompiles past the call to `sub_1e380` (dpkg's `command_shell`, which execs a shell or dies in `ohshite`) and swallows the entire next function — the ~110-line conffile prompt loop that begins at 0xcc50 — inflating spawn_shell from 32 source-shaped lines to 147 (GED 186 via the >60-node |dNodes|+|dEdges| approximation). Fresh kuna (current main, post-Jun-29 SAILR PRs) is byte-identical to the stored run: not fixed.

Ground truth at the byte level (stripped binary):
- spawn_shell's last real instruction is `call 0x1e380` at 0xcc48; 0xcc4d is 3-byte nop padding; the next function starts at 0xcc50. The binary's .eh_frame even records spawn_shell's exact extent (FDE `pc=0xcb30..0xcc4d`) and an FDE start at 0xcc50 — entry_disc (default-on) consumes the FDE *starts*, yet S2 flow happily falls through the padding into 0xcc50 because nothing plants an artificial halt after the call and nothing clamps flow at a discovered function boundary.
- `0x1e380` (command_shell) is noreturn-by-body: `execlp` → on failure `call 0x23e50` (ohshite) → `je`-target/getenv retry path → backward `jmp 0x1e39f`. No `ret` anywhere; every path loops or dies. `0x23e50` (ohshite) has the same looping cold-path shape, bottoming out (via `run_error_handler`) in `longjmp`/`exit` — both on the vendored known-noreturn list.

Why angr wins: angr's CFGFast runs a path-based interprocedural returning fixpoint (`CFGBase._analyze_function_features` → `_determine_function_returning`, iterated over callgraph predecessors by `_iteratively_analyze_function_features`, same code in the venv that ran the benchmark): a function with no return endpoints and no transfer out to a returning function gets `returning = False`, with no evidence threshold and no shape requirement — looping noreturn functions like command_shell/ohshite are concluded naturally, so the decompiler emits `/* do not return */` and clips the flow. ida agrees (GED 0, independent engine) and ghidra is close (6), so the score is genuine, not an artifact; angr's stored block is the complete source function, not truncated.

Why the existing kuna options don't cover it (all verified against the bytes):
- `noreturn_known` / `noreturn_extern` / `noreturn_externmatch` (default-on): name-list seams; `sub_1e380` is an internal stripped function with no name. Never applicable.
- `noreturn_disc` (off, tried): Ghidra's evidence rule needs >=3 call sites with invalid fall-through. `0x1e380` has exactly 3 call sites (0xcc48, 0x1306c, 0x272d5) and ALL yield zero evidence: each fall-through is nop padding (or the caller's own `__stack_chk_fail` tail) that the recursive-descent Listing decoded as a valid instruction, and the `fall == next-function-entry` check is defeated by the 3–6 byte padding gap at every site. 0/3 < 3.
- `noreturn_propagate` (default-on but Listing-gated; armed in the sweep): its rule 1 requires the *last-by-address* real instruction to be a call/jmp to a terminal callee — a straight-line `die()`-wrapper shape. command_shell's last instruction is the backward `jmp 0x1e39f` (retry loop), and ohshite's is the backward `jmp 0x23f08`, so the rule fails at every level of dpkg's error chain even though rules 2–4 (no ret, no indirect jmp, body-contained targets) all hold.
- Option sweep with `listing on` + `noreturn_disc on` (+ propagate active): 147→146 loc, 6→5 gotos — some unrelated conclusion elsewhere in the tail, gap intact.

Siblings: none (group has a single case).

## Proposed fix

Mechanism (small, one option-gated pass change per pipeline rule 1): extend `s1_noreturn_propagate` (`decompiler/crates/kuna-analysis/src/s1_noreturn_propagate/mod.rs`) with the angr-faithful *reachable-CFG no-returning-path* rule, replacing/augmenting the "last real instruction is a terminal call" anchor: BFS the Listing instructions from the function entry following static branches and fall-throughs; treat a call whose static callee is in the terminal set as a path terminator (no fall-through successor); disqualify on any `ret` reached, any computed/indirect jump, or any escape outside the body span that is not a terminal callee. Conclude noreturn when the frontier exhausts. This is exactly angr's `_determine_function_returning` restricted to the Listing tier, keeps the existing conservative guards (rules 2–3 become emergent), converges over the same caller-worklist fixpoint, and emits the same `NoReturnFact` through the proven commit arm (`engine.rs` → `set_function_no_return` → flow artificial-halt). command_shell then concludes in sweep 2 (after ohshite/run_error_handler conclude from the longjmp/exit seeds), and spawn_shell clips at 0xcc4d.

Owning files: `kuna-analysis/src/s1_noreturn_propagate/mod.rs` (the rule), `kuna-decomp/stages.toml` + catalog count tests per the adding-option recipe if gated as a new value (suggest reusing the existing `noreturn_propagate` gate — it is the same pass, strictly more complete; a `looppaths` sub-toggle only if reviewers want the old shape preserved).

Risks / default-on viability: the rule itself is sound (a function with no reachable returning path cannot return) — the risk is Listing quality (an undiscovered jump target would be invisible; the indirect-jump disqualifier already guards the computed case). The real default-on blocker is upstream of the rule: the whole family is inert without `option listing on` (decode cost on every load), so the benchmark's default run only moves if `listing` (or a stripped-binary-only auto-arm) passes the 0/675 datatest ablation and the <=5% speed budget — measure that separately. Cheaper complementary guard worth filing alongside: entry_disc already parses .eh_frame FDEs whose *extents* (`0xcb30..0xcc4d` here) are discarded — clamping S2 flow fall-through at the next discovered FDE start/extent would stop ALL boundary overruns on eh_frame-bearing stripped binaries without the Listing tier, at the cost of a truncation instead of a semantic `/* do not return */`.
