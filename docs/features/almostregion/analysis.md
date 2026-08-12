# almostregion — identify inlined code from the structurer's virtualized edges

Stage 1 of a two-stage capability. Stage 1 (this PR) **identifies** code the
compiler inlined; stage 2 (`outline`, separate PR) **removes** a chosen region
and emits a call to a synthesized pseudofunction in its place. The two are split
because the second is destructive and the first is not, and because the region
selection can come from a human or an LLM instead of from this pass.

## The gap

kuna has no way to say which bytes of a function did not come from that
function. Every consumer of an optimized `-O2` decompile — a human reader, a
diffing tool, an LLM agent — sees one flat body with a callee's control flow
spliced into it, and nothing distinguishes the two.

## The premise, and the measurement that constrains it

SAILR (Basque et al., USENIX Security 2024) observes that a residual `goto`
marks where the compiler restructured control flow past what a source-level
`if`/`while` nest can express. kuna already exploits this three times
(`gotoreduce`, `taildup`, `crossjumprevert`), each undoing a specific
optimization. Inlining is another such restructuring.

decbench ships an oracle for this that had not been used: every project is built
at both `O2` and `O2-noinline` (`O2_NOINLINE = ["-O2","-fno-inline"]`,
`decbench/models/project.py:39`), and the `compiled/` copies carry full DWARF
including `DW_TAG_inlined_subroutine` — exact ground truth for which callee was
inlined, at which PC ranges, inside which caller.

Measured over **1,483 paired functions** (`coreutils/tr`, `coreutils/ls`,
`bzip2`, `gzip`, `tar`), with every goto-removal pass disabled so the raw
structurer output is visible:

| quantity | value |
|---|---|
| raw gotos, O2 vs O2-noinline | 3,513 vs 2,151 (**1.63x**) |
| gotos suppressed by kuna's own goto-removal passes | 650 of 3,513 (**19%**) |
| marginal gotos per 100 *added* (inlined) bytes | 0.826 |
| baseline goto density in the `-fno-inline` build | 0.514 |
| **inlined bytes vs ordinary bytes** | **1.61x as goto-dense** |
| goto *density*, functions with vs without inlining | 0.671 vs 0.699 (**no separation**) |

Two conclusions, both load-bearing for the design:

1. **"There is a goto here" is not an inlining detector.** The function-level
   association (functions with gotos average 6.2 inlined callees, functions
   without average 0.9) is almost entirely a size effect — inlining adds code,
   and the added code carries gotos at close to the ordinary rate. What is
   reported must therefore be the specific *structural* fact underneath the
   goto, not the goto.
2. **The signal must be read before kuna's own cleanup passes**, which consume
   19% of it.

## The mechanism

Virtualization is recorded on `sblocks` and never on `bblocks` — every
`set_goto_branch` call site is on the structurer's own graph, and
`KunaRegionIdentifier::build_from_block_graph` rebuilds from `bblocks` with no
edge-flag filter. P7 therefore already sees the fully un-virtualized CFG, so
"un-virtualize the edge and re-run region identification" is a no-op: it returns
exactly the regions P7 already declined to find.

The operation that carries the intended meaning is the inverse. For each
virtualized edge `e = (s,t)`: build a scratch region graph over `bblocks`,
**delete** `e`, and keep a `(head, exit)` pair only when it is a single-entry
region in `G - e` and **not** one in `G`. Both directions are asserted, and the
deleted edge must be **incident** to the region it exposed — without that,
deletion perturbs dominance at a distance and the pass reports regions with no
relationship to the goto (the first implementation did exactly this).

Head candidates are the goto target, then the dominator chain above the target
(the edge entered the body below its head — the cross-jump shape), then the
chain above the source (the edge gave the body a second exit).

## Validation against the oracle

On `coreutils/tr::parse_str` (O2 `0x4260`, 18 DWARF inline sites, 16 gotos at
O2 and 0 at `-fno-inline`) the pass reports three regions. Checked against the
DWARF records, the reported member blocks fall inside genuinely inlined code;
one region is a clean single-callee hit (`append_char_class`, `0x4838-0x48ea`),
one spans two adjacent inlined callees (`append_equiv_class` and
`append_char_class`), which is a real imprecision and is not claimed otherwise.

## Honest scope

This finds the **messy half** of inlining by construction. A callee that inlines
tidily leaves no goto at all: `openssh-portable/ssh::sshkey_drop_cert` at O2 has
three DWARF-confirmed inlined callees (`sshkey_type_is_cert`, `cert_free`,
`sshkey_type_plain`) and **zero** gotos even with every goto-removal pass
disabled. No goto means no virtualized edge means nothing for this pass to
probe.

A complete inline-identification story needs the signals this one cannot see.
Measured or identified while building this:

- **Interface narrowness** — |live-in| + |live-out| against region size. A real
  function keeps an ABI-shaped interface as it grows; an arbitrary slice does
  not. Measured against the same DWARF oracle: interface-per-instruction
  **0.242 for inlined ranges vs 0.478 for size-matched controls (2.0x)** — a
  cleaner separation than gotos, and inherently size-normalized. Alone it is
  precision 0.66 at recall 0.69 against a 0.53 prior, so it is a strong feature,
  not a standalone classifier. `p6_variables/cover.rs` already holds live ranges.
- **Stack-slot lifetime clustering** — an inlined callee's frame merges into the
  caller's, leaving a disjoint slot cluster confined to one region.
- **Windowed FID matching** — `kuna-analysis/src/analyzers/fid/` already hashes
  instruction streams and `clip_extent` is already factored out as a pure
  function; today `calculate_extent` clips to whole functions.
- **Repeated isomorphic regions** — 1,692 of 3,025 damaged functions have a
  callee inlined at least twice.
- **Inverting kuna's own passes** — `crossjumprevert`, `dedupitetail` and
  `taildup` fire exactly where the compiler duplicated code, which is
  multi-site-inlining evidence they currently discard.

## Corpus-wide opportunity

Across 16,487 O2 functions with DWARF and a GED score: **3,037 are worse at O2
than at O2-noinline, totalling 193,754 GED points**, 768 of them GED-perfect
without inlining. Control: 99.6% of damaged functions contain inlined code (mean
633 inlined bytes) versus 33.1% of unaffected ones (mean 22.5 bytes).

## Risk

Report-only. No p-code and no structured node is touched, so `off` is
byte-identical and `on` differs from `off` by comment lines alone — requirement 7
(whole-corpus sweep of changed functions) reduces to zero changed functions with
no spec supplied, and both parity gates confirm it. Cost is bounded before any
graph is built (`MAX_PROBES = 24`, `MAX_BLOCKS = 512`) because each probe
rebuilds two dominator trees and the per-function watchdog is live on
`decompile-all`.
