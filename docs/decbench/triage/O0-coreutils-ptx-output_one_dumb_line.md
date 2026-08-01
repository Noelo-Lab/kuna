---
case_id: O0-coreutils-ptx-output_one_dumb_line
pool: angr
status: feature-candidate
tier: M
margin: 98
fresh_verdict: today's build (auto/aggressive AND reliable) still emits 16 `if`s / 0 ternaries — the six source `cond ? tsl : 0` selects are all still explicit if/else diamonds; nothing about this case is fixed
option_closing: null
feature_slug: iteregion-merged-dest
scope: small
confidence: high
---

## Side-by-side

Trimmed to the one region that carries the margin (the `tail.start < tail.end`
branch). Everything else is structurally identical across all four panes.

**angr (stored, GED 0)** — folds both selects into `?:`, so the region is one `if/else`:

```c
    if (g_414840 < *((long long *)&g_414848))
    {
        sub_4041da(g_414840, *((long long *)&g_414848));
        if (g_414850)
            fputs_unlocked(g_414028, stdout);
        sub_4041a6(g_414820 - g_414020 - g_414868 + g_414860 - (!g_414870 ? 0 : g_414838)
                   - *((long long *)&g_414848) + g_414840 - (!g_414850 ? 0 : g_414838));
    }
    else
    {
        sub_4041a6(g_414820 - g_414020 - (g_414868 - g_414860) - (!g_414870 ? 0 : g_414838));
    }
```

**ghidra (stored, GED 80)** — no ternary either, but hoists one arm so each select
costs a 2-block `if` (no `else`), and keeps **two** variables:

```c
    lVar1 = DAT_00114838;
    if (DAT_00114870 == '\0') {
      lVar1 = 0;
    }
    lVar2 = DAT_00114838;
    if (DAT_00114850 == '\0') {
      lVar2 = 0;
    }
    FUN_001041a6(((((DAT_00114820 - DAT_00114020) - (DAT_00114868 - DAT_00114860)) - lVar1) -
                 (DAT_00114848 - DAT_00114840)) - lVar2);
```

**kuna (stored, 2026-07-27 benchmark, GED 98)** and **kuna (fresh, today)** —
structurally identical to each other; only the DIV-34..39 print normalization
differs (truthy conditions, brace elision, `// branch-flip` inline warnings):

```c
    if (dat_14870) // branch-flip
      v1 = dat_14838;
    else {
      v1 = 0;
    }
    if (dat_14850) // branch-flip
      v1 = dat_14838;
    else {
      v1 = 0;
    }
    sub_41a6(((((dat_14820 - dat_14020) - (dat_14868 - dat_14860)) - v1) - (dat_14848 - dat_14840)) - v1);
```

Structural metrics (from the triage dump): angr 10 `if`s + 6 ternaries; ghidra 16
`if`s + 0; kuna stored 16 + 0; kuna fresh 16 + 0. `10 + 6 == 16` — the entire
`if`-count delta versus angr is the six unfolded selects.

## Source

`~/github/decbench/results/full_run/O0/coreutils/compiled/ptx.i:11806`,
`output_one_dumb_line`. Six ternaries, no explicit `if/else` around any of them:

```c
      print_spaces (half_line_width - gap_size
                    - (before.end - before.start)
                    - (before_truncation ? truncation_string_length : 0)
                    - (tail.end - tail.start)
                    - (tail_truncation ? truncation_string_length : 0));
    }
  else
    print_spaces (half_line_width - gap_size
                  - (before.end - before.start)
                  - (before_truncation ? truncation_string_length : 0));
```

plus the matching `(keyafter_truncation ? ... : 0)` / `(head_truncation ? ... : 0)`
pair in the `head.start < head.end` region. Source CFG is 22 nodes / 31 edges — a
real, non-degenerate control-flow graph, and `source_exact` is false but
`approximated` is false too, so the GED numbers are meaningful. angr's 0 is a
genuine structural match, not a truncation artifact (its body is complete and 78
LOC against kuna's 79).

## Analysis

**Symptom (structural, output-shaped).** Every source `cond ? truncation_string_length : 0`
renders as a full three-component assignment diamond
`if (dat_14870) v1 = dat_14838; else { v1 = 0; }` instead of a `?:`. Six of them,
each adding a condition node, two arm nodes and their edges where the source has
none — that is 6 of kuna's 16 `if`s and the whole 98-vs-0 GED margin.

**The miner hint is only 18% right.** `bucket: kuna-specific` and "kuna far worse
than ghidra" overstate it: ghidra 80, binja 66, ida 65 all lose badly to angr's 0
on the same six selects. 80 of the 98 points is a family-wide ternary-recovery
gap; only the ~18-point kuna-vs-ghidra residue is kuna-specific (ghidra hoists one
arm and pays a 2-block `if`, kuna pays a 3-block `if/else`). Fixing the ternary
fold closes all 98 at once, so that is the lever worth pulling.

**Root cause — one predicate in P8.** kuna already ships the pass for exactly this
shape: `iteregion` (default-ON, DIV-17) plus its `iteexpr` extension. Neither
fires here, and `--option iteexpr on` does not change a single line
(verified). The declining predicate is
`decompiler/crates/kuna-decomp/src/p8_structure/kuna_iteregion.rs::same_storage`,
which requires the two arms' assignment destinations to be the **same raw storage**
(space name + offset + size). The raw p-code shows they are not:

```
Basic Block 12 0x000058ac-0x000058ac
0x000058ac:248:	RAX(0x000058ac:248) = #0x0                       <- else arm: register RAX
Basic Block 13 0x000058a3-0x000058aa
0x000058aa:854:	u0x1000023f(0x000058aa:854) = r0x00014838(...)   <- true arm: a UNIQUE
Basic Block 14 0x000058b1-0x000058d1
0x000058b1:3c2:	RAX(0x000058b1:3c2) = u0x1000023f(...) ? RAX(0x000058ac:248)
```

The P6 merge relocated one arm's write into a copy-shadow `unique` and unified it
with the other arm through the `MULTIEQUAL` — the two writes **are the same
HighVariable** (both print as `v1`, and the function declares exactly one
`int8 v1;`, with `dedupvardecls off` too), but they are not the same *address*.
`same_storage` asks the address question, so it answers "different variable" for a
diamond that is one variable. Every other gate in `match_ite_assignment` passes:
3-component `BlockIf`, no goto target, condition leaf ends in the `CBRANCH`, and
each arm is a single printed op — a plain `CPUI_COPY`, so this is missed by
default-ON `iteregion` itself, not only by `iteexpr`.

**Minimal reproducer, today's build** (`/tmp/.../scratchpad/t3.c`, gcc -O0 -no-pie):

```c
void g3(void) { sink(A - B - (D - E) - (f1 ? t1 : 0) - (C - F) - (f2 ? t2 : 0)); }
```

```c
void g3(void)
{
  int8 v1;
  if (f1) v1 = t1; else { v1 = 0; }
  if (f2) v1 = t2; else { v1 = 0; }
  sink(((((A - B) - (D - E)) - v1) - (C - F)) - v1);
}
```

Same p-code shape: true arm into `u0x10000051`, else arm into `RAX`, joined by a
`MULTIEQUAL`. Not ptx-specific — a default `decompile-all` of ptx leaves 27 such
unfolded same-variable diamonds across 10 of 259 functions while producing 0
ternaries (11 with `iteexpr on`, none of them in this family).

**Spin-off defect — FIXED (DIV-47).** The spin-off below is closed; the analysis in it
is superseded. The two selects were *not* cover-legal to merge: `ActionMarkImplied`
dropped `Merge::markImplied`'s operand cover-dirtying (`merge.cc:1595-1605`) and a
Varnode `coverdirty` never reached its HighVariable (`Varnode::setFlags`,
`varnode.cc:377-378`), so `Cover::rebuild`'s forward walk through implied consumers
never re-ran and the first select's live range never reached the point where the
inlined expression is printed. With both halves restored the function emits two
variables and subtracts the right values. The *structural* gap in this record (six
selects rendering as diamonds instead of `?:`) is untouched and still open.

**Spin-off defect, different phase, NOT the same root cause.** The reproducer above
also shows that kuna's C is *semantically wrong*: `v1` is written twice and both
reads are folded into one expression printed after the second write, so with
`f1=1, t1=100, f2=0` kuna's C subtracts 0 where the binary subtracts 100. ghidra
(`lVar1`/`lVar2`), ida (`v0`/`v2`) and angr all keep the two values apart. The IR
covers are disjoint (`RAX` select read at `0x58b1`, `RCX` select defined at
`0x58dc`), so the P6 speculative merge is cover-legal; what is not legal is that
the first read is an *implied* varnode whose printed position is dragged past the
second write. Folding the diamonds to ternaries does **not** fix it. This is a P6
merge/implied-cover issue (`p6_variables/merge.rs`, `p6_variables/cover.rs`
`Cover::rebuild`'s implied recursion, `coreaction_cleanup.rs::check_implied_cover`)
and deserves its own triage/proposal — it is a correctness bug, invisible to GED.

## Proposed fix

**Mechanism.** In `kuna_iteregion.rs`, replace `same_storage(data, dest_true, dest_else)`
with a HighVariable-identity test — the two arm outputs must resolve to the same
`HighVariable` (`Varnode::get_high()`), falling back to the current storage
comparison when either varnode has no high. That is the question the pass actually
means to ask ("do both arms assign the one variable the printer will name?"), and
it is what makes the copy-shadow/`unique` form match. One function, one module,
inside a pass that is already print-only (no p-code is mutated), so the blast
radius is a rendering change only.

**Owning files.** `decompiler/crates/kuna-decomp/src/p8_structure/kuna_iteregion.rs`
(`same_storage`, and its doc comment / the module header's "same storage" prose);
spec chapter owning `p8_structure/`; a `tests/stages/` two-pass testcase built on
the `g3` reproducer shape (option off = the diamond, default = the ternary).

**Risks.** (1) This broadens a **default-ON** option (`iteregion`), so it changes
default emitted C wherever a merged-destination diamond exists — 27 sites in ptx
alone. Expect datatest movement; if the 675 assertions move, the widening belongs
behind its own flag (or under `iteexpr`, which is default-off and aggressive-only)
rather than under `iteregion`. Either way it needs a DIV row. (2) The ternary form
is only right when the source wrote a ternary — the standard `iteregion` caveat,
already documented in the option's `use_when`. (3) It does not touch, and must not
be claimed to fix, the `v1`-aliasing correctness bug above.

**Expected payoff.** 6 `if`s -> 6 ternaries in this function, giving 10 `if`s / 6
ternaries — angr's exact shape, i.e. the full 98-point margin, and it also closes
the ~18-point kuna-vs-ghidra residue without needing ghidra's arm-hoisting.
