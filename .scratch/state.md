# F4 elfmain — state

- slug: elfmain / branch: feat/elfmain / worktree /home/mahaloz/kwt/elfmain
- PR: https://github.com/Noelo-Lab/kuna/pull/649 (OPEN, full-ci label). DO NOT MERGE.

## Round 2 (adversarial review returned CHANGES) — what is being done
1. BLOCKER (3rd-arg truncation): FIXED in b5578239 — the parked prototype is now
   `int main(int argc, char **argv, char **envp)`. Both witnesses verified:
   armlibcmain_le32 @0x103dc prints `__printf_chk(2,"...",envp,0)` (was an
   undefined `v4`), and a freshly built 3-arg main walks `envp` as a real param.
2. Header + spec + phases.toml + analysis.md: the false "still shows the third
   register" claim replaced with the real reason all three are declared. DONE.
3. verify_aif.rs assertion re-pointed to `(**(code **)` + `0x3df0`. DONE.
4. record.json: hunk classes re-classified (expression re-anchoring class added,
   "no statement moved" claim dropped); oracle denominator 612/725 with the 33
   refusal rows excluded. DONE (metric.measured still PENDING).
5. Real-binary sweep (8 stripped decbench binaries) published under
   docs/features/elfmain/sweep-real/. DONE.
6. typesweep running -> .scratch/typesweep-elfmain (fills metric.measured).
7. TODO: rebuild after phases.toml edit, regen docs/options.md, counters --check,
   re-record baseline-stages, ALL GATES, PR body trim, push.

## Gates (STALE — from round 1, must be re-run)
