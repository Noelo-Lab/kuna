# [PROPOSAL] Cyclic loop-refinement structuring (`loop_refine_structure`)

**Status:** draft proposal — awaiting human go/no-go. **Do NOT implement until approved.**

## The problem

On `tr_O2.o::parse_str` (x86_64, the angr `test_decompiling_tr_O2_parse_str` opportunity) kuna
produces correct but goto-heavy C: **37 gotos / 21 labels** where angr's SAILR/Phoenix
structurer produces **11 gotos / 8 labels** for the same function — a clean nested
`while(true){ if/else; switch }`. Same `switch` (16 cases), same overall logic; the gap is
purely structuring quality. See `analysis.md` and `angr-vs-kuna.txt`.

The dominant excess is **`goto label_400af2` ×12** — every `switch`-case body and several
branches jump to a single loop-tail/continuation block. angr hoists this continuation merge
into the loop body so the cases fall through to the latch (no `goto`). The remaining ~25 gotos
are shared-successor merges resolved by angr's condition-region structuring + tail duplication.

## Why this is not a single option-gated Action (the proposal gate)

kuna's existing structuring passes were measured on this exact target and **none change the
output** (goto count stays 37 with each, or all three, enabled):

| option | what it does | why it doesn't fire here |
|---|---|---|
| `gotoreduce` | tail-dup `if(cond) goto T` for small single-successor tails | acyclic tail dup only; the merge is the loop latch |
| `loopbreak_recovery` | loop-exit `goto` → `break` | exit edges only, not continuation merges |
| `regionstructure` | `region_structurer`: acyclic sequence + ITE schemas + edge-virtualization | explicitly **acyclic** — no cyclic loop refinement |

The fix is **cyclic loop refinement** — a new structuring pass type over `KunaRegionIdentifier`
loop regions. That is, by the Hard-Rule-7 criteria: (a) a new pass *type*/infrastructure, not
one Action/Rule; (b) it touches S7/S8 structuring beyond a single gated early-return. **LARGE.**

The decider subagent independently returned `scope: large` (verbatim in `record.json`).

## angr reference

angr SAILR/Phoenix **`LoopRefinement`** + `RegionIdentifier` cyclic structuring
(DreamStructurer-style condition regions over loops): it identifies the loop region, refines
the loop body so continuation-merge predecessors fall through to the latch (synthesizing
`continue`/fall-through), and applies condition-based refinement to collapse shared-successor
gotos before any edge is virtualized. kuna already has the **acyclic** half
(`region_structurer`, the W7 `KunaRegionIdentifier` port); this proposal adds the **cyclic**
half on top of it.

## Proposed multi-step implementation plan

Option name: **`loop_refine_structure`** (default-OFF; gated P0 assertion). Increments:

1. **Loop-region continuation analysis (read-only).** Over `KunaRegionIdentifier`'s loop
   regions, identify the single continuation-merge block (the latch's dominant predecessor
   merge) and the set of predecessor edges that `goto` it. Emit observability
   (`region` console sub-queries), no output change. *(~1 increment.)*
2. **Continuation hoisting / `continue` synthesis.** Restructure the loop body so those
   predecessors fall through to the latch (or emit `continue`) instead of `goto <latch>`,
   gated by `loop_refine_structure on`. Target: eliminate the 12× `goto label_400af2`.
   *(~1–2 increments; the riskiest — must preserve SSA/region invariants.)*
3. **Condition-region refinement for shared-successor merges.** Port the SAILR condition-based
   refinement that collapses the remaining ~13 single-use shared-successor gotos into nested
   `if/else` before virtualization. *(~1–2 increments.)*
4. **Default decision + ablation.** Run the full `kuna test --all` ablation; ship default-OFF
   opt-in unless 0/675 upstream assertions change *and* the speed gate passes. Add a stage test
   (`tests/stages/ghangr-tr-o2-parse-str-7b9159.xml`) asserting goto-count reduction / the
   `while(true)` form.

Estimated **3–5 increments / multiple PRs** — hence the proposal gate.

## Speed / risk assessment

- **Risk: high.** Cyclic structuring surgery over loop regions risks SSA/phi and region-tree
  invariant breakage; it can change output for *many* functions, so the ablation must be clean
  before any default-ON. Strictly default-OFF opt-in until proven.
- **Speed:** a region-tree walk + local restructuring is roughly linear in region count; the
  budget concern is the extra structuring pass on every function when ON. Must be measured per
  increment (Hard Rule 6); keep OFF by default regardless until measured within budget.
- **Blast radius:** S7/S8 only; no loader/lift changes. Same family as the parked
  `gotoreduce`-tail-duplication and irreducible-loop proposals (see `docs/PROGRESS.md`).

## Recommendation

File this as a draft `[PROPOSAL]` PR and park the opportunity. On human approval, dispatch an
implementation worker per increment (start with increment 1, the read-only analysis) on this
branch.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
