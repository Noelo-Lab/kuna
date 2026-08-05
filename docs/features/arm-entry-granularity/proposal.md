# arm-entry-granularity — proposal

**Size: LARGE.** Two new discovery mechanisms in the analysis tier, each needing its own
precision work, plus two small independent repairs. Not one predicate, not one flag.

**Recommendation: approve as a 4-PR sequence, in the order below. Ship every step
default-OFF first; earn the default flips one at a time on measured precision.**

The evidence is in `analysis.md`; the raw tables are in `rivals-vs-kuna.txt`. Headlines:

- The gap is **2,941 addresses / 1,838 decbench-measured** on the ARM corpus. The code is
  **not decompiled at all** (1,896 dropped vs 3 absorbed) — the recall doc's
  "absorbed into the preceding function" framing does not survive measurement.
- **57% is one sub-class**: entries reachable only through a code-pointer word, which
  `code_pointer_table_seeds` *already finds* and then rejects on two precision predicates.
- **No existing option closes any of it** — `aggressive`/`addrtable`/`fast_funcdisc`/
  `eh_frame_full`/`operand_refs` each recover exactly 0.
- A scratch build with the predicates relaxed reaches **99.2% entry recall (from 91.1%)** on
  9 binaries — at 53-62% precision, which is the whole of the remaining work.

---

## The sequence

### PR 1 — `cortexmvectors`: widen the Cortex-M vector-table signature (SMALL, do first)

`entry/mod.rs::cortexm_vector_table` demands (a) the table's section is `SHF_EXECINSTR` or in
an executable `PT_LOAD`, (b) `word[0]` in `0x2000_0000..=0x3FFF_FFFF`, (c) `word[1] == e_entry`.
Measured against the corpus, **all four of the biggest miss contributors fail it**:
cleanflight and betaflight because their `.isr_vector` is an `A`-only section whose stack word
is `0x1000fff0` (STM32F4 CCM RAM, and betaflight's table is linked at `0x20000000` for runtime
relocation); crazyflie and nuttx because their reset word legitimately differs from `e_entry`.

Proposed signature, in the same "confirm, then harvest" shape:

- allow any **allocated** section (the table is data the CPU reads; what must be executable is
  what its slots point at, which `harvest_vector_words` already checks — the existing comment
  at `entry/mod.rs:1098` already argues this, it just did not go far enough);
- accept `word[0]` in any plausible SRAM window — add `0x1000_0000..=0x1FFF_FFFF` (CCM/TCM),
  keep the existing `0x2000_0000..=0x3FFF_FFFF`;
- replace `word[1] == e_entry` with `word[1]` is odd and in an executable section, and require
  **N >= 3 consecutive conforming slots** so a coincidental two-word match cannot arm it.

Worth on its own: it re-arms the Thumb region paint and the handler seeds on four projects.
Independent of everything below. Gate: `cortexmvectors`, default-OFF until the corpus sweep
shows no regression, then a DIV row.

### PR 2 — `ptrentry`: pointer-referenced entries, with a real precision model (LARGE, the prize)

`aif/mod.rs::code_pointer_table_seeds` already collects every 4-byte-aligned Thumb code pointer
in every allocated section. Today it accepts a target only if it is in an undefined gap, opens
with a stack-frame prologue, and disassembles into a >2-instruction valid subroutine. The
second and third reject **1,632 of the 1,671** pointer-class misses; they are why
`devnull_read` (`movs r0,#0; bx lr`) and `NMI_Handler` (`bx lr`) are dropped.

Relaxing them naively gets 62% precision (348 of 560 new entries are ground truth, 117 split a
real function). That is not shippable. The design work *is* the precision model, and the
measurement says where it must come from — corroboration, not shape:

1. **Table-run corroboration.** A pointer that is part of a run of >= 2 consecutive
   code-pointer words at the same stride is a vtable / fops struct / ISR table; a lone word
   that happens to look like a code pointer is usually a switch-table entry or a constant.
   The 21% mid-body splits are dominated by `ldr pc,[pc,r]`-style switch tables, whose entries
   point *into* a function — a run test plus "reject a target whose containing kuna function
   also contains the pointer word" should remove most of them.
2. **Terminating-routine validity, not length.** Replace `MIN_SUBROUTINE_INSNS = 3` on this
   path with "reaches a clean RET / computed jump within N instructions, no undecodable byte,
   no out-of-image flow" — the same walk, without the length floor. A 1-instruction `bx lr`
   IS a valid Cortex-M handler.
3. **Keep the frame-prologue signal as a tie-breaker, not a gate**: accept prologue-shaped
   targets on a single pointer, require the run for prologue-less ones.
4. **Re-seed the recursive-descent walk** with the accepted targets (as Stages 2 and 3 already
   do), so their `BL` edges are followed. This is where the `called` sub-class partly comes
   back for free.

Ship as `--option ptrentry on|off`, default-OFF, tier `analysis-enablement`, wired the same way
`funcstart_patterns`/`aif` are (commit-time gate + the `decompile-all` non-x86-64 injection).
The default flip is a separate decision, taken on a corpus precision sweep, not on this PR.

Acceptance target for the implementation: **>= 80% of new entries are ground truth and
zero regression in already-recovered entries**, measured with the same harness as
`rivals-vs-kuna.txt` §8.

### PR 3 — `tailcallentry`: split at unconditional-branch targets (MEDIUM, needs a containment predicate)

`walk.rs` makes a new function only at CALL targets; an unconditional `B` is same-function
flow. 513 misses (226 measured) are reachable only that way.

The naive rule ("target of an unconditional `B`, and the preceding instruction ends the flow")
measures 39% precision — it splits rotated loop heads. Tightening "preceding instruction must
RETURN" gets 54%. Neither is enough. A shippable version needs the containment fact the walk
does not currently keep: **track each walked function's claimed instruction set, and split only
when the branch source and the target belong to different claimed sets** (angr's `CFGFast`
job model in miniature). That is a real change to `WalkState`, hence its own PR after PR 2.

Ship as `--option tailcallentry on|off`, default-OFF.

### PR 4 — Listing-tier `TBB`/`TBH` switch resolution (SEPARATE GAP, sequence last)

The `called` sub-class (297 misses, 146 measured) is not an entry-discovery failure at all:
these addresses have a direct `BL` from a *discovered* caller, but the call site sits past an
unresolved Thumb table branch, so the walk never decodes it. Worked example in `analysis.md`
§4. Resolving `TBB`/`TBH` (and `ldr pc,[pc,r,lsl#2]`) in the Listing walk is the fix; it also
improves the Listing's coverage for every other consumer. Treat as its own investigation —
the engine already has jump-table machinery (P2 + feedback) that the analysis tier does not
reuse.

## What is explicitly NOT proposed

- **Re-walking AIF's accepted gap starts** as a standalone change. It is a one-line-shaped
  omission (`passes.rs:535` emits `run_aif`'s entries without a re-walk, unlike Stages 2/3)
  but it is worth **+39 entries over 9 binaries** and is subsumed by PR 2's re-seed. Fold it
  into PR 2; do not spend a PR on it.
- **Chasing the last 10-14%.** 105 of the missed names are alias/ICF surplus on an address
  that is already counted, and 309 misses were recovered by no rival at all. Those are the
  floor.
- **Porting Ghidra's analyzer wholesale.** kuna already out-recalls Ghidra by ~3,800 entries on
  this corpus. The useful import is the *shape* of Ghidra's precision guard — create functions
  from **instruction operands** (`OperandReferenceAnalyzer`, *Subroutine References*), never
  from a raw data-pointer scan (`DataOperandReferenceAnalyzer` overrides `createFunctions` to
  nothing, explicitly) — not its analyzer list.

## Risks

- **Over-discovery is not free even when it does not split.** Extra `sub_` entries cost
  decompile time on `decompile-all` and can confuse `decompile-project` output. Every step
  must carry a speed measurement; PR 2 adds a second Listing rebuild.
- **The benchmark is insensitive to precision.** decbench scores per ground-truth function, so
  a false start costs nothing on GED and everything on real use. Precision must be gated on the
  `splits-a-real-body` metric in `rivals-vs-kuna.txt` §8, not on GED.
- **Every recovered entry is new emitted C**, so each step needs the full four gates plus a
  two-pass `tests/stages/` case (option off = the function is absent; default/on = it is
  emitted). Suggested first testcase: `nuttx` O2 `devnull_read` @ `0x8003aa0` — 4 bytes, one
  pointer reference, currently zero output.

## Open questions for the reviewer

1. Should `ptrentry` be ARM-gated (like Stages 2/3) or run on every architecture? The
   mechanism is arch-neutral; only the Thumb-LSB pointer test is ARM-specific. Recommend
   ARM-first, generalise on evidence.
2. Is the `decompile-all` non-x86-64 auto-injection (DIV-20's pattern) the right default
   surface for `ptrentry`, or should it wait for an explicit `--mode aggressive`?
3. PR 4 (`TBB`/`TBH`) overlaps the "jump-table post-typing refinement" row already tracked in
   `docs/missing-ghidra-analyses.md` §7 as an engine-tier item. Confirm whether the
   analysis-tier Listing should get its own resolver or consume the engine's.
