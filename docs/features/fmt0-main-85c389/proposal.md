# [PROPOSAL] angr-fmt0-main-85c389: structure irreducible (multi-entry) loops

**Status:** draft proposal — awaiting human go/no-go. **Do not implement before approval.**

- **Opportunity:** `test_decompiling_fmt0_main :: main`
- **Binary:** `/home/mahaloz/github/angr-dev/binaries/tests/x86_64/fmt_0` (GNU coreutils `fmt`), `main` @ `0x4019b0`, x86_64
- **Reference:** angr 9.2.213
- **Proposed option:** `irreducibleloops` (change_kind: `structure-recovery`)
- **Scope:** **large** (scope-decider verdict in `record.json`)

## The problem

kuna emits **syntactically invalid C** for `fmt`'s `main` because `main` contains an
**irreducible (multi-entry) loop**: the `-WIDTH` first-arg special case `goto`s into the
`getopt_long` option-parsing loop body. kuna's Ghidra collapse-based structurer resolves
irreducibility only by emitting gotos, and additionally produces a while-block whose
condition carries side-effecting multi-statements that the S9 printer renders as incomplete
`while (...` lines. angr recovers a clean, valid `while(true){ ...; if(v==-1) break; ... }`.

Full evidence: [`analysis.md`](analysis.md), side-by-side: [`angr-vs-kuna.txt`](angr-vs-kuna.txt).

## angr reference

- `angr/analyses/decompiler/region_identifier.py` — `RegionIdentifier` (already ported,
  read-only, as kuna `kuna_regionid`/`kuna_regiongraph` under `s7_regions/`).
- `angr/analyses/decompiler/structuring/` — the SAILR/Phoenix condition-based structurer
  (`PhoenixStructurer`) with `_refine_cyclic` / goto-and-loop refinement that handles
  multi-entry loops by node duplication / condition recovery. **Not yet ported.**

## Why this is not a single Action/Rule

`loweredswitch` works by **manufacturing a missing S2 `JumpTable`** that the existing
structurer already consumes. Here the structurer *itself* is wrong on irreducible loops —
there is no artifact to pre-build. Closing the gap requires new structuring infrastructure
across S7 (region structuring), S8 (`blockaction.rs`), and S9 (`printc.rs`), > 3 anchor
files and > 1 new module. The decider verdict (verbatim in `record.json`):

> "The gap is a failure of the S7/S8 structurer+printer on an irreducible (multi-entry)
> loop, not a missing consumable artifact … Both options need new pass infrastructure and
> touch S7 structuring/S8/S9 emit far beyond a single gated early-return …"

## Proposed multi-step implementation plan

1. **Promote `kuna_regiongraph`/`kuna_regionid` from read-only to emit-capable.** Add a
   SAILR/Phoenix-style condition-based structuring + goto/loop-refinement layer on top of the
   existing `RegionIdentifier` so it yields a *structured* tree (proper loops with
   break/continue, recovered if-conditions, labeled gotos only for residual irreducible
   edges) — not just a region dump.
2. **Lower the structured region tree into kuna's `sblocks` (`BlockGraph`)** — or into a
   sibling structured representation consumable by a new printer path.
3. **Add an S9 printer path** (`printc.rs::emitBlock*`) for the new node kinds so `print C`
   walks it and emits valid C (proper `while`/`break`, labeled gotos for true irreducibility).
4. **Gate the whole alternate path behind `option irreducibleloops on`, default OFF.** Only
   functions whose region tree is detected cyclic **and** multi-entry route through the new
   path; everything else keeps the collapse structurer and **byte-identical** default output.
5. **Testcase + speed.** Add `tests/stages/ghangr-fmt0-main-85c389.xml` (off = current
   invalid rendering / fix = valid `while(true)` loop) and record the per-function decompile
   wall-time off vs on for `main`.

## Speed / risk assessment

- **Default path unchanged** (gated OFF → byte-identical; parity preserved). Low parity risk.
- **Routed path** adds the region-structuring + condition-recovery layer per eligible
  function; cost is bounded to detected irreducible functions but must be measured (the
  pipeline speed budget is +5% default; if over, ship default-OFF opt-in).
- **Engineering risk: high.** This is essentially porting angr's structuring backend (or a
  reduced irreducible-loop subset of it) and a new printer path — multi-PR, the largest of the
  angr-port surfaces. Recommend scoping a **minimal first cut**: handle only single-extra-entry
  loops via node duplication of the entry block (the `fmt` shape), behind the option, before
  attempting the general SAILR structurer.

## Recommendation

Approve as a multi-PR effort, or de-scope to "duplicate the single extra loop-entry block so
the collapse structurer sees a reducible loop" as a first, smaller PR (still > 1 anchor file
in S8, hence still proposal-gated). Either way: human go/no-go before any implementation
worker is spent.
