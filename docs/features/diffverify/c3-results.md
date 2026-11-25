# diffverify — slice C3 results (rule localization)

C3 names the rule application after which a decompiled function stops computing what
the binary computes (proposal, Slices).

```sh
decompiler/target/release/kuna-verify localize tests/stages/kuna-floatret.xml logish
```
```
logish (decompile 2)  differs; of the 24 rules that applied, it agrees without these, in the order their last needed application ran:
  step   134  selectcse  application 1 of 1 at 0x40001c (agrees with 0, differs with 1)
  step   155  subcommute  application 2 of 2 at 0x40001c (agrees with 1, differs with 2)
  step   180  subvar_shift  application 3 of 4 at 0x40001c (agrees with 2, differs with 3)
  step   200  ignorenan  application 1 of 1 at 0x400018 (agrees with 0, differs with 1)  <- latest
```

`--pass N` picks the `N`-th decompile of the function in the script. By default
`localize` picks the first one that differs. An object file works too, with
`--option NAME VALUE`.

## 1. Method

The proposal suggested reverting fixes. That needs a fix to revert, so it does not
work for a bug nobody has fixed. C3 instead reruns the decompile that differs with
rules held back. A per-thread cap in the rule pool does this
(`kuna-decomp/src/infra/kuna_rulecap.rs`, spec §0.6):
1. Count every rule application in the full decompile, by rule, in order.
2. Rerun once per rule that applied, with that rule alone held back. Every other rule
   and every action runs, so each rerun is a complete decompile.
3. A rule without which the function agrees is a culprit. A binary search over how
   many times it may apply finds `k`: with `k - 1` applications it agrees, with `k` it
   differs. The `k`-th application is named by the op address it rewrote.
4. Both ends of the pair are measured. The agreeing end is confirmed over every run;
   the search itself checks only up to 32 runs past the full decompile's failing run.
5. Culprits are listed in the order their `k`-th application ran in the full decompile.

The binary search assumes that once an application makes the function differ, the
later ones do not undo it. A rewrite that a later application cancels breaks that, and
can hide an earlier application that already differed. So `k` is a candidate: the
agreeing end with `k - 1` is checked over every run and the differing end with `k` is a
concrete counterexample, but neither shows that `k` is the first or the only cause.

Holding back every rule at once does not work as a baseline. Without the folding
rules, stack addresses are never resolved and actions see a different program. So
the first design, a cap on all applications, was dropped.

Two kinds of culprit come out:
- **Enablers.** `propagatecopy`, `collapseconstants` and `sub2add` prepare the IR that
  a later step gets wrong, so without them the wrong step never fires.
- **The rule that is wrong**, which is usually the one marked `latest`.

A capped decompile gets a 5-second budget: with a rule held back, some actions
never settle. Running past that budget, or a verdict other than `AGREES` or
`DIFFERS`, is reported as inconclusive for that rule.

## 2. The corpus

`localize` was run on all 113 `DIFFERS` in `docs/baseline-verify.json`: 199 s wall
time on 12 parallel jobs.

| outcome | functions |
|---|---:|
| names one or more rules | 15 |
| differs without any one rule: an action, or rules together | 98 |

The latest culprit in each of the 15:

| latest culprit | functions | what §4 / §6 of `c4-results.md` says it is |
|---|---|---|
| `ignorenan` | `armfloat-preset` `scale_err` (2 passes), `floatret` `logish`, `prefersplit` `ccw_mixed` | the ARM/AArch64 NaN compares, the kuna defect of c4 §6c |
| `storevarnode` | `stackviews` `obj_global_reuse` (0x4011ec), `declaring-double-score-return` `refined_score` | the store kuna drops (c4 §2; #1834) and the callee-clobber control |
| `loadvarnode` | `mipsgot-weak` `same` (2 passes) | the load of an undefined weak symbol folded from the image: the harness limit of c4 §2 |
| `propagatecopy` | `narrowext` `ci`, `cu`, `csc`; `indexedstoreguard` `cb_m8`, `-a64` `m8`; `calleearitycut` `caller3`; `calleearityscratch` `boundary_const` | off passes and known limitations the tests pin |

Each matches the triage in `c4-results.md` that was done by hand.

The 98 that name no rule are mostly the call defects:
- arguments a caller passes straight through, dropped by prototype recovery;
- call outputs never connected;
- killed registers.

Those are actions in P4, not rules. The rest are harness limits and off passes whose
cause is an action.

`gh8777-flagcompare-sborrow` names no rule, although `RuleSborrowGe` is the bug
there. On PIC24 the W registers are memory-mapped, so the first difference the check
sees is the register file in RAM. That difference is there with or without the rule.

## 3. Cost

- **The hook, unarmed.** The pool reads one thread-local flag per rule attempt. A/B
  of `decompile-all --jobs 1` over `dwarf_namespaces_rust_x86_64` (549 functions,
  five runs each, alternating) showed no measurable cost: median user time 29.7 s
  without the hook, 26.9 s with it, on a machine under load average 60.
- **Localizing.** About 1 + (rules applied) + Σ log2(applications of each culprit)
  decompiles. A script is cut after the decompile being localized. Typical cases take
  1–10 s.
