# ptrentry — pointer-referenced ARM function entries

Step 2 of the approved ARM entry-recall sequence (`[PROPOSAL]` PR #239,
`docs/features/arm-entry-granularity/`), and the one that proposal called *the
prize*: **1,671 missed function entries, 56.8% of the whole ARM entry-recall gap**,
all of them reachable only through a code-pointer word that kuna's shipped scan
already finds and then throws away.

Every number below is measured on this branch against the **stripped** binaries
decbench feeds the decompilers, with ground truth taken from the matching
`compiled/` binary's DWARF (`DW_TAG_subprogram` `DW_AT_low_pc`, Thumb-LSB-masked),
using the same harness as step 1 (`docs/features/cortexmvectors/`). Corpus: the six
Cortex-M projects plus freertos and riot-os — 16 binaries x 3 optimisation levels,
48 images, 41,378 ground-truth function addresses. Raw per-binary tables:
`measurements.txt`.

---

## 1. Headline

| surface | ground truth | recall before | recall after | new entries | of which GT | **splits a real body** | outside any DWARF body | **entries lost** |
|---|---|---|---|---|---|---|---|---|
| `kuna functions` (Listing on) | 41,378 | 38,238 (92.4%) | **39,851 (96.3%)** | 1,694 | 1,613 (**95.2%**) | **0** | 81 | **0** |
| `--option listing off` | 41,378 | 8,885 | 8,885 | 0 | — | 0 | 0 | **0** |

**+1,613 recovered ground-truth functions at 95.2% precision, zero split bodies,
zero losses.** The proposal's acceptance bar was >= 80% ground truth with zero
regression.

The `listing off` surface (`--mode fast`, the WASM and Ghidra front-ends,
single-function `kuna decompile`) is structurally unaffected: `ptrentry` is a
Listing consumer — its whole precision model reads the code/data partition the
recursive-descent walk leaves behind — so with no Listing there is nothing to
consume. Zero new, zero lost, on all 48 images.

## 2. What the shipped scan already knows, and refuses to say

`aif/mod.rs::code_pointer_table_seeds` (ARM discovery Stage 3) scans every
allocated section at 4-byte alignment for a word whose value, Thumb bit cleared,
lands in an executable range. It **finds every one of these addresses.** It then
requires the target to

1. be in an undefined gap (`Listing::is_undefined`),
2. open with a stack-frame prologue (`is_thumb_function_prologue`:
   `PUSH` / `PUSH.W` / `SUB SP` / `SUB.W SP` / `VPUSH` / `STMDB SP!`), and
3. pass `check_valid_subroutine`, whose `MIN_SUBROUTINE_INSNS = 3` rejects a
   two-instruction routine (Ghidra's *"don't allow two-instruction routines"*).

Predicates 2 and 3 reject **1,632 of the 1,671** pointer-class misses, because the
population they are aimed at does not have those shapes:

- **93.3% of the missed entries have no canonical Thumb frame prologue.** A leaf
  callback establishes no frame at all.
- **41% are <= 8-byte leaves.** `movs r0,#0 ; bx lr`, `mov r0,r2 ; bx lr`, a bare
  `bx lr`. A one-instruction `bx lr` *is* a valid Cortex-M exception handler, and
  angr, Ghidra, IDA and Binary Ninja all recover them.

Predicate 1 turns out to reject almost nothing: the proposal established that the
gap is a *hole* (1,896 dropped vs 3 absorbed), so these addresses genuinely produce
no output at all.

Deleting predicates 2 and 3 is not shippable — the proposal measured the naive
relaxation at **560 new entries, 348 ground truth (62%), 117 (21%) splitting a real
function body**. decbench scores per ground-truth function, so those splits cost
nothing on the benchmark and everything in real use. The design work is entirely
the precision model.

## 3. The precision model: containment, not shape

A scratch build was instrumented to dump, for every code-pointer candidate on all
48 images, the features a precision model could key on: the referencing sites and
their sections, the target's prologue shape, the length and termination of its
speculative decode, table-run lengths, and the two containment facts below.
**3,462 candidates** pass `is_undefined` plus a terminating-routine decode. Joined
against DWARF:

| guard | candidates | ground truth | splits a real body | outside any body |
|---|---|---|---|---|
| validated candidates | 3,462 | 2,009 (58.0%) | **1,261** | 192 |
| + no site shares the target's discovered function | 2,184 | 1,997 (91.4%) | 8 | 179 |
| + no site overlaps a decoded instruction | 2,143 | 1,992 (**93.0%**) | **0** | 151 |

Two facts do the whole job:

1. **A referencing word in the same discovered function as the target.** That
   pairing *is* the `ldr pc,[pc,r]` switch table: the slots point into the body
   that holds them. Rejecting it removes **1,253 of the 1,261 splits** and costs 12
   ground-truth entries. (`Listing::function_containing` is entry-ordered — the
   greatest function entry <= the address — so this reads "the nearest preceding
   discovered function", which is exactly the granularity the tier has, and it errs
   conservative on a sparsely-discovered image, where more addresses collapse onto
   one entry.)
2. **A referencing word that overlaps a decoded instruction.** Then it is not a
   table slot at all — it is an instruction's operand bytes, read four-aligned.
   Rejecting it removes the remaining **8 splits** and costs 5 ground-truth
   entries.

This is the kuna form of the line Ghidra draws between `OperandReferenceAnalyzer`
(which calls `createFunctions()` on targets derived from **instruction operands**)
and `DataOperandReferenceAnalyzer`, which overrides `createFunctions` to a no-op
with the comment *"don't ever create a function from a data pointer"* — the guard
the proposal identified as the one worth importing. kuna cannot use Ghidra's
version directly: the Listing tier populates only `RefKind::Call`/`Code`
(control-flow) references and leaves the data kinds unpopulated by design, so
"which instruction loaded this word" does not exist. The containment pair recovers
the same discrimination from the code/data partition the walk already leaves
behind.

The effect of the body-claim dedup is visible in the headline: 2,143 accepted
candidates collapse to 1,694 committed entries, because an accepted routine's body
is claimed and the pointers into its interior are then skipped.

### 3.1 Table-run corroboration was measured and is dominated

The proposal's other named guard was **table-run corroboration**: accept a target
only if some referencing word is part of a run of >= 2 consecutive stride-4
code-pointer words (a vtable / fops struct / ISR table), keeping the frame prologue
as a tie-breaker for lone pointers. It was implemented, built and measured on the
whole corpus on top of the containment model:

| | new entries | ground truth | splits | outside any body | lost |
|---|---|---|---|---|---|
| shipped (containment only) | 1,694 | **1,613 (95.2%)** | 0 | 81 | 0 |
| + require a frame prologue or a run of >= 2 | 1,338 | 1,302 (97.3%) | 0 | 36 | 0 |

The run test **cannot separate switch tables**, because a switch table *is* a run —
which is why it removes no additional split. What it does remove is 311
ground-truth functions reached by a lone `ldr rN,=func` literal-pool constant,
which is an ordinary and correct way to take a function's address. The 2.1pp of
precision it buys is entirely in the "outside any DWARF body" bucket — addresses in
code the debug info does not describe (newlib, compiler-rt, vendor startup), which
is unverifiable rather than wrong. Trading 311 real functions for it is the wrong
side of this PR's purpose, so the shipped predicate does not include it. Both
configurations are in `measurements.txt` (blocks 1 and 4) so the choice can be
revisited on evidence.

## 4. Terminating-routine validity replaces the length floor

`MIN_SUBROUTINE_INSNS` is replaced, not deleted. `check_terminating_routine` is
`check_valid_subroutine` with the length floor removed and the escape policy kept
strict: follow the speculatively-decoded flow from the target; reject an
undecodable byte, a flow that leaves the executable image, or an escape into
another undecoded region; accept when the walk reaches a clean `RET` / computed
jump, or calls into already-discovered code, inside the `MAX_FOLLOW_INSNS` (4000)
bound. A single `bx lr` passes. A data word that happens to decode does not: it
almost always runs off into an undecodable byte or an out-of-image flow first.

On its own the termination requirement is worth 15 candidates out of 3,462 — it is
a floor, not a discriminator. What matters is that removing the *length* floor
costs nothing in precision once containment is in place, which is what makes the
852 pointer-referenced tiny leaves recoverable at all.

## 5. Wiring: two ways, and the additive one wins on every axis

Step 1 (`cortexmvectors`) found that handing the recursive-descent walk more seeds
*costs* recall, because `listing/walk.rs` treats an unconditional `B` as
same-function flow: seed a two-instruction ISR stub that tail-calls a shared
handler and the walk absorbs the handler, whose own entry then fails the
`is_undefined` guard in `code_pointer_table_seeds` and in the AIF gap scan. That is
proposal step 3's tail-call split, which has not shipped. The same trap applies
here, and it is far larger, because `ptrentry` produces 14x as many seeds:

| wiring | recall delta | new entries | of which GT | splits a real body | **entries LOST** | of which GT |
|---|---|---|---|---|---|---|
| **entry facts only (shipped)** | **+1,613** | 1,694 | 1,613 (**95.2%**) | **0** | **0** | **0** |
| re-seed the walk + rebuild the Listing | +1,626 | 2,758 | 1,641 (59.5%) | **974** | **734** | 15 |

Re-seeding buys 13 more ground-truth entries, splits 974 real function bodies and
drops 734 already-recovered entries. Proposal step 2 item 4 suggested the re-seed
("so their `BL` edges are followed... this is where the `called` sub-class partly
comes back for free"); measured, it is not free, and the containment model cannot
defend against it, because the damage is done inside the walk before `ptrentry`
looks at anything.

The shipped pass therefore emits its accepted targets as an additive `entries`
fact stream and nothing else. That is what makes **"this option never removes an
entry" a property of the wiring rather than of a heuristic** — the Listing is not
rebuilt, so `funcdisc_recursive`, `aif` and every other consumer see byte-identical
input. `verify_ptrentry.rs::ptrentry_only_adds_entries` pins it.

### 5.1 Consequence: AIF's un-re-walked gap starts stay out of scope

The proposal noted a related one-line-shaped omission — `passes.rs` emits
`run_aif`'s accepted gap starts straight to the commit stream without the re-walk
Stages 2 and 3 do — worth +39 entries over 9 binaries, and said to fold it into
this PR "if it falls out naturally". It does not: closing it *is* adding a walk
re-seed, which is the wiring measured above at -734 entries. It stays with proposal
step 3, behind the tail-call split that would make any re-seed safe.

## 6. Where the residual sits

After this PR corpus recall is 39,851 / 41,378 = **96.3%**, up from 92.4% (angr
measured 95.3% on the 36,360-address subset the proposal used). The 1,527 remaining
misses are the classes this PR does not own: the tailcall sub-class (proposal step
3), the `called` sub-class behind unresolved `TBB`/`TBH` switches (step 4), the
fall-through / after-terminator classes with no reference at all, and the
proposal's measured noise floor (105 alias/ICF surplus names, 309 misses no rival
recovers either).

The 81 new entries outside any DWARF body are not error in general: 72 of the 151
pre-dedup cases fall inside the address span DWARF does describe, and the rest are
concentrated in riot-os and chibios, whose debug info covers the application CU
only, not the RIOT/newlib core the vector table points at. They are code the
hardware genuinely enters.

## 7. Strict no-op elsewhere

Whole-binary `kuna decompile-all --json` output is **byte-identical** (sha256) with
the option off vs on for x86-64 (`gzip`), ARM Linux (`u-boot`), and the Cortex-M
images with no admissible candidate (libopencm3 `blink.elf`, `miniblink.elf`,
`tick_blink.elf`). The XML datatest path never builds an
`ObjectLoadImage` or a Listing, so the option cannot fire there at all: `make test`
is 675/675 **PARITY OK** with the flag registered.

## 8. Speed

Median of 5 whole-binary runs against a frozen release binary, off vs on:

| surface | binary | off | on | delta |
|---|---|---|---|---|
| `kuna functions` | cleanflight | 2417.8 ms | 2435.4 ms | +0.73% |
| `kuna functions` | nuttx | 1006.8 ms | 1010.9 ms | +0.40% |
| `kuna functions` | libopencm3 msc.elf | 144.3 ms | 137.9 ms | -4.41% |
| `kuna functions` | gzip (x86-64) | 341.9 ms | 332.9 ms | -2.62% |
| `kuna decompile-all` | nuttx | 9166.3 ms | 9529.1 ms | +3.96% |
| `kuna decompile-all` | libopencm3 msc.elf | 538.5 ms | 590.9 ms | +9.73% |

The scan itself is one 4-byte-strided pass over the allocated sections plus one
bounded speculative decode per surviving candidate, reusing the `GapDecoder` cache
Stages 2-3 already warmed. On the discovery surface (`kuna functions`) that is
under 1% on both large ARM images and inside run-to-run noise elsewhere — gzip is a
strict no-op (byte-identical output), so its -2.62% bounds the measurement error.

The `decompile-all` deltas are the extra decompile work for the extra functions,
and both are **sub-proportional** to it: nuttx gains 46 functions of 786 (+5.9%
work) for +3.96% wall clock, and msc.elf gains 8 of 67 (+11.9%) for +9.73% on a
half-second run. Discovering more code costs more time by construction; nothing
here is overhead.

## 9. Default decision

**Default-OFF.** The option discovers more functions, so it changes emitted C by
construction; a clean `docs/baseline.json` ablation is evidence of non-interference,
not of quality. Two things should happen before a flip: a human read of the
precision numbers above — this is the step of the sequence with a real
precision/recall trade-off — and a corpus sweep across more ARM firmware than
decbench carries. No DIV row: default output is byte-identical.

## 10. Reproduction

```bash
# entry recall, before vs after, against DWARF ground truth
kuna functions <stripped-arm-elf> --json                          # before
kuna functions <stripped-arm-elf> --json --option ptrentry on     # after
# compare both against DW_AT_low_pc of the matching compiled/ binary, and classify
# each new address as ground truth / inside another body / outside every body

# the strict no-op check
kuna decompile-all <x86-64-or-arm-linux-elf> --json | sha256sum
kuna decompile-all <same> --json --option ptrentry on | sha256sum
```
