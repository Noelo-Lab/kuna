# Implementation-phase findings — `jumptable-augment` is mis-shaped; the real fix is noreturn-by-body

**Status:** the APPROVED step-1 design (`jumptable-augment`, see `proposal.md`) was taken
into implementation (Phase 3). Reproducing the gap at IR level revealed the design is built
on a premise that does **not** hold for this function. The actual, validated fix is a
different, cleaner single feature: **noreturn-by-body inference**. See `proposal-v2.md` for
the revised proposal. This document records the evidence.

## What step-1 assumed

> "Detect an equality cascade in the `default`/guard region of an already-recovered
> `BRANCHIND` switch and append its (value→target) pairs to the existing `JumpTable`."

This assumes the residual `-0x83`/`-0x82` comparisons (a) test the **switch variable** and
(b) are absent from the jump table. Both are false in kuna's actual IR.

## Ground truth (disassembly, `mv -O2` `main`, default region 0x2d41–0x2dc6)

```
2d38: cmp $0x54,%eax ; je 2dc0          (case 'T')
2d41: mov $0x1,%edi ; call usage         <-- the DEFAULT sink. usage() is NORETURN
2d4b: nop                                    (its body always reaches `call exit@plt`)
2d50: cmp $0xffffff7d,%eax ; jne 2db2     (eax==-0x83  -> version_etc(); exit  @2d57)
2da0: cmp $0x6e,%eax ; jne 2d41           (case 'n')
2db2: cmp $0xffffff7e,%eax ; jne 2d41     (eax==-0x82  -> usage(0)  @2db9 = --help)
```

This is GCC's **jump-table + residual comparison-cascade** idiom: the dense cases dispatch
through a `BRANCHIND` jump table; the sparse outliers (`-0x83`, `-0x82`) are tested by
explicit `cmp %eax` comparisons that all fall back to the default `usage(1)` at `0x2d41`.
The comparisons read `%eax` = the `getopt_long` return = the switch variable.

## kuna's IR (from `print raw`)

The switch **is** recovered: block 2 `switch EAX(0x2c9a:dc)`. But kuna treats `usage` as a
**returning** function, so it keeps the `usage(1)` call and the comparisons in the *same*
basic block (block 47, `0x2d41-0x2d55`):

```
0x2d46:125 : call fusage(#0x1)
0x2d46:eee : EAX(0x2d46:eee) = [create]          <-- usage's MODELED return value
0x2d50:12c : ZF = EAX(0x2d46:eee) == #0xffffff7d  <-- cascade tests the call output, NOT swvar
0x2db2:15f : ZF = EAX(0x2d46:eee) != #0xffffff7e
```

So **the residual cascade tests `EAX(0x2d46:eee)` — the post-call clobbered re-read of EAX
(usage's modeled return) — not the switch variable `EAX(0x2c9a:dc)`.** They are distinct SSA
varnodes across the call barrier. `canon_switch_var` peels only transparent ops
(COPY/CAST/ZEXT/SEXT/zero-`SUBPIECE`); it cannot bridge a call-output `[create]`. A detector
keyed on "cascade testing the switch variable" therefore **cannot fire** here. (step-1
premise (a) false.)

Moreover the `-0x83`/`-0x82` targets are **already native entries of the recovered jump
table** — they are not missing from the table; they are *shadowed at flow time* by the live
default/cascade block that exists only because `usage` is modeled as returning. (step-1
premise (b) false.)

## The real fix — proven by ablation

Marking `usage` no-return makes kuna re-split block 47 at the call, dead-code-prune the now
unreachable cascade, and surface the native jump-table cases:

```
$ decomp_dbg -s $SPECS
[decomp]> load file .../mv_-O2
[decomp]> load function main
[decomp]> option noreturn usage true     # <-- must be AFTER load (load rebuilds the arch)
[decomp]> decompile ; print C
...
  switch(v6) {
    ...
    case 0x80: ...
    case -0x83:
      version_etc(...); exit(0);
    case -0x82:
      usage(0);
  }
```

Result: **12 cases (was 10), 17 gotos (was 24)** — exactly the case-recovery win step-1
aimed for, via existing machinery. (An earlier ablation that placed the `option` line *before*
`load file` showed "no change" and was wrong — the load rebuilds the architecture/symbol
table and discards the pre-load option. Order matters.)

The catch: kuna cannot establish `usage`-noreturn automatically.
- `noreturn_known` (static list: exit/abort/…) does not include the program-local `usage`.
- `noreturn_disc` (discovered, evidence-tally: ≥3 call sites with no valid fall-through) is
  **defeated by GCC's layout** — the residual comparisons sit physically *after* every
  `usage` call site, so each call has a valid fall-through and the tally never fires.
- There is **no body-based inference** ("a callee all of whose terminating paths reach a
  known-noreturn call is itself noreturn") — which is exactly what angr does and what `usage`
  needs.

## Adversarial confirmation

An independent reviewer was tasked to *refute* the infeasibility of step-1 and instead
confirmed it, while correcting the bad ablation above. Verdict (verbatim summary in
`record.json` → `decisions`): *"the single-Action cascade-augment is infeasible/mis-shaped …
correctness is gated on usage-noreturn … The faithful path is noreturn-by-body inference for
`usage` … a P0/S1 analysis feature, not an S2 jump-table augment — a different, cleaner
single-feature candidate worth proposing in place of step 1."* It also flagged that a
storage-address heuristic (treat the post-`usage` cascade var as the switch var because both
live in EAX) is an unsafe semantic gamble (asserts EAX is unclobbered across an arbitrary
call) and not worth shipping even default-OFF.

## Conclusion

`jumptable-augment` does not close this gap. The case-recovery sub-gap (#3 in `proposal.md`)
is a **noreturn-by-body inference** gap. Revised proposal: `proposal-v2.md`.
