# arm-entry-granularity — what the 2,026-function recall gap actually is

`docs/decbench/recall-measurement.md` closed with one headline: **2,026 of the 2,061 live
recall misses (98.3%) are function-entry granularity on embedded ARM** — "kuna emits a body
for code *containing* the DWARF `low_pc` but does not start a function *at* it", median 24
bytes from the containing kuna entry.

That number is right. The mechanism sentence is wrong, and the difference decides the fix.

All numbers below are re-measured on this build (HEAD `36641d35`) against the `stripped/`
binaries decbench actually feeds the decompilers, with ground truth taken from the
`compiled/` binary's DWARF. The full tables are in `rivals-vs-kuna.txt`.

---

## 1. First, the framing correction: kuna is winning this metric

Entry recall over the whole ARM Cortex-M corpus (36,360 DWARF function addresses,
8 projects x 3 optimisation levels):

| | recovered | recall |
|---|---|---|
| angr | 34,295 | 95.3% |
| **kuna** | **33,326** | **91.7%** |
| ghidra | 29,530 | 81.2% |
| binja | 29,364 | 80.8% |
| ida | 17,632 | 76.1% |

kuna is second of five and roughly 3,800 entries ahead of Ghidra. So "Ghidra and IDA recover
these, name the analyzer and port it" is not the right frame: Ghidra finds only 1,859 of the
2,941 addresses kuna misses, and misses several thousand kuna finds. The remaining gap is a
*tail against angr*, not a deficit against Ghidra.

## 2. The mechanism claim, tested: it is not absorption, it is a hole

For every missed address, is it inside the ground-truth body of the nearest lower kuna entry?

| | count |
|---|---|
| **dropped** — past the end of the nearest kuna function | **1,896** |
| unknown extent — nearest kuna entry is a kuna-invented `sub_` | 1,040 |
| **absorbed** — strictly inside a kuna body | **3** |

Three. The 24-byte median distance is real, but it measures "there happens to be another kuna
function nearby", not containment. Two witnesses, both from the recall doc's own example list:

`nuttx` O2 `devnull_read` @ `0x8003aa0` (4 bytes: `movs r0,#0; bx lr`). The nearest kuna entry
is `0x8003a8c`, whose real body ends at `0x8003a9e`:

```c
=== 0x8003a8c ===                       // the whole of kuna's output for this region
void sub_8003a8c(void) {
  sub_8004b24(); sub_8003acc();
  sub_800db20(dat_8003b30,dat_8003b2c,0x1b6,0); // tail-call
  return;
}
=== 0x8003aa8 ===                       // devnull_poll — the NEXT kuna entry
```

`devnull_read` and `devnull_write` are not absorbed into anything. Their eight bytes are
simply **not decompiled**. Same for `libopencm3` O2 `cdcacm`: kuna emits `0x80016f4` and then
`0x8001740`, and the whole of `0x80016b4`-`0x80016f4` — including
`_usbd_standard_request_device` — produces no output at all.

**Consequence for the campaign:** this is not a boundary/split problem that "only" costs a
name. It is missing output, and it is worth exactly what the recall pool says it is.

## 3. The sub-classes, counted

2,941 missed addresses (the address-level pool; decbench counts names, see §5):

| sub-class | n | share | measured by decbench | what reaches the entry |
|---|---|---|---|---|
| **pointer** | 1,671 | 56.8% | 1,402 | only a code-pointer word (vtable / fops struct / ISR table / literal pool) |
| **tailcall** | 513 | 17.4% | 226 | only an unconditional `B` |
| **called** | 297 | 10.1% | 146 | a direct `BL` — and kuna still has no entry |
| **fallthrough** | 194 | 6.6% | 43 | only fall-through from the previous instruction |
| **unreferenced** | 146 | 5.0% | 19 | nothing in the image |
| **after-terminator** | 118 | 4.0% | 2 | nothing; previous instruction ends the flow |
| condbranch | 2 | — | — | a conditional branch from outside |

Two shape facts that decide the design:

- **93.3% of the missed entries have no canonical Thumb frame prologue** (2,744 of 2,941).
  Every existing kuna ARM discovery stage requires one.
- **41% are tiny leaves** — ground-truth size <= 8 bytes (1,205; 852 of them pointer-referenced).
  `movs r0,#0; bx lr`, `mov r0,r2; bx lr`, a bare `bx lr`. Every existing stage rejects a
  routine of <= 2 instructions.

## 4. Owning pass per sub-class, and why the evidence was rejected

Discovery on a stripped Cortex-M image is
`decompiler/crates/kuna-analysis/src/passes.rs::run_listing_consumers`, four stages feeding
one recursive-descent walk (`src/listing/walk.rs`):

| stage | code | what it seeds |
|---|---|---|
| 1 | `entry/mod.rs::full_pattern_starts` | Ghidra `<patternpairs>` epilogue-paired prologues |
| 2 | `aif/mod.rs::raw_thumb_prologue_seeds` | raw `PUSH {..,lr}` / `PUSH.W {..,lr}` |
| 3 | `aif/mod.rs::code_pointer_table_seeds` | code pointers whose target has a frame prologue |
| 4 | `aif/mod.rs::run_aif` | fingerprint-matched gap starts |

**pointer (56.8%) — evidence present, rejected by two predicates.** Stage 3 already scans every
allocated section for Thumb code pointers and already finds these words. It then rejects them:
`is_thumb_function_prologue` (the target must open with `PUSH`/`SUB SP`/`VPUSH`/`STMDB SP!`)
and `check_valid_subroutine`'s `MIN_SUBROUTINE_INSNS = 3`. A 2-instruction leaf callback fails
both. `devnull_read`'s pointer is in `.text`; `NMI_Handler`'s is at `.isr_vector+0x8`. Both are
found and both are thrown away. The third guard, `listing.is_undefined(target)` ("never split a
discovered function"), turns out to reject almost nothing here — see §2.

**tailcall (17.4%) — the walk has no notion of a tail call.** `walk.rs` treats a CALL target as
a new function and every other flow target as a same-function successor (lines 155-166). An
unconditional `B` to a different function is therefore intra-function flow. In practice the
branch source is usually itself inside the dark region, so nothing is emitted at all rather
than being merged.

**called (10.1%) — a second-order effect of unresolved switches.** These have a direct `BL`
*and* a discovered caller (273 of 297). Worked example: betaflight O0
`blackboxLogInflightAdjustmentEvent` @ `0x8031688` has 46 direct `BL`s, all from
`applyStepAdjustment` @ `0x80316b4` — whose third instruction is `tbh [pc, r3, lsl #1]`. The
walk records an indirect branch with no static successor (`walk.rs` module doc: "Indirect
targets contribute NO static successor"), so every case body past the switch, and every `BL`
in them, is never decoded. Ghidra, angr and Binary Ninja all recover this entry. The owning gap
here is **jump-table resolution in the Listing tier**, not entry discovery.

**fallthrough / after-terminator (10.6%)** — a static placed immediately after its neighbour,
with no reference at all. Only a byte-pattern or a gap scan can find these; both are already
tried and both reject on the prologue.

**unreferenced (5.0%)** — 137 of 146 are found by some rival, 72 share an address with another
DWARF name. Mostly compiler-runtime aliases (`__floatundidf`/`__aeabi_ul2d`).

### Two incidental defects found on the way

1. **The Cortex-M vector-table oracle silently no-ops on the four biggest contributors.**
   `entry/mod.rs::cortexm_vector_table` requires the table's section to be `SHF_EXECINSTR` (or
   inside an executable `PT_LOAD`), word[0] in `0x2000_0000..0x3FFF_FFFF`, and
   `word[1] == e_entry`. Measured:

   | project | `.isr_vector` | detected |
   |---|---|---|
   | chibios / riot-os / libopencm3 | inside `.text` / exec `.vectors` | yes |
   | cleanflight | `A`-only section, SP word `0x1000fff0` (CCM RAM) | **no** |
   | betaflight | `A`-only section at `0x20000000` (RAM-relocated table) | **no** |
   | crazyflie | `A`-only section, reset word `0x08025239` != `e_entry` `0x080041c0` | **no** |
   | nuttx | table at `.text+0`, `word[1] 0x0800122f` != `e_entry` `0x8000189` | **no** |

   So `NMI_Handler` and friends have to be rediscovered by Stage 3, which then rejects them for
   having no prologue. Widening the signature is small and independent of the main proposal.

2. **AIF's accepted gap starts are never re-walked.** Stages 2 and 3 re-seed the walk and
   rebuild the Listing; `run_aif` (`passes.rs:535`) emits its entries straight to the commit
   stream, so their `BL` edges are never followed. Measured worth on its own: +39 entries over
   9 binaries (+0.4pp). Real, tiny, and subsumed once the other seeds trigger a re-walk.

## 5. How much is ground-truth noise

Small, and not where the recall doc guessed.

- **Alias / ICF surplus: 105 names, 3.4% of the name-based pool.** decbench counts names;
  105 missed addresses carry more than one DWARF name (`__floatundidf`/`__aeabi_ul2d`,
  `__gtdf2`/`__gedf2`, ...). Recovering one address scores all of them, so this inflates the
  pool but is not unrecoverable.
- The `NMI_Handler` / `PendSV_Handler` / `DebugMon_Handler` example in the recall doc **is not
  the noise it was labelled**. In cleanflight they sit at `0x803b060`, `0x803b062`, `0x803b064`
  — three *distinct* two-byte functions, each a real `bx lr` stub, each pointed at by its own
  vector-table slot, and all three recovered by Ghidra, IDA, angr and Binary Ninja. They are
  ordinary members of the pointer sub-class.
- **309 misses (10.5%) no rival found either** — 176 tailcall, 104 pointer. This is the
  genuinely hard residue; 102 of them are alias/ICF cases.

Honest total that is *not* worth chasing: **~105 alias names plus, at most, the 309 nobody
recovers** — call it 10-14% of the pool. The other ~86% is recoverable code.

## 6. What Ghidra and IDA actually do

The pointer sub-class in Ghidra is recovered by a **pair**: `ArmAnalyzer` (Processors/ARM,
constant propagation) resolves `LDR Rx,[pc,#k]` literal-pool loads, sets `TMode` from the
pointer's LSB and disassembles the target; then `OperandReferenceAnalyzer` ("Reference",
default-on, option *Subroutine References* default true) calls `createFunctions()` on those
disassembly targets (`OperandReferenceAnalyzer.java:508,614`). Notably its data-side sibling
`DataOperandReferenceAnalyzer` overrides `createFunctions` to a no-op with the comment
*"don't ever create a function from a data pointer"* — Ghidra creates these functions from
**instruction operands**, not from a raw data scan, which is exactly the precision guard kuna
would need.

`docs/missing-ghidra-analyses.md` §6 currently classifies the whole `OperandReferenceAnalyzer`
family as *"out-of-scope-at-tier ... function creation -> §4 (`s1-entry-disc`)"*. The function
creation half is **not** subsumed by `s1-entry-disc`, and kuna's ported `operand_refs` only
plants string-data facts. That row needs correcting; this document is the evidence.

For the tailcall sub-class the closest Ghidra machinery is `CreateThunkFunctionCmd` /
"Create Thunk Functions"; angr's `CFGFast` gets it from its job-based traversal, where a
`b` to an address outside the current function's block set starts a new function.

## 7. Does anything already shipped close it? No.

`--mode aggressive`, `--option addrtable on`, `fast_funcdisc on`, `eh_frame_full on` and
`operand_refs on` each recover **exactly zero** additional entries, on every binary tried
(§7 of `rivals-vs-kuna.txt`). `aggressive` resolves to the same
`listing`+`funcstart_patterns`+`aif` set the `decompile-all` driver already injects on ARM,
so it is the baseline, not an improvement. The earlier "aggressive recovers zero of
betaflight's residual 277" finding replicates and generalises. This is a **new-mechanism**
question, not a default-flip question.

## 8. What a fix is worth (measured, not estimated)

A throw-away env-gated scratch build (reverted; not in this diff) with the Stage-3 guards
removed (`p`) and a first-cut tail-call rule (`t`), both re-seeding the walk:

| | ground truth | base | `p` | `t` | `apt` |
|---|---|---|---|---|---|
| 9 binaries | 9,802 | 8,926 (91.1%) | 9,520 (97.1%) | 9,092 (92.8%) | **9,722 (99.2%)** |

**91% of the residual gap is reachable by relaxing predicates that already run.** On
`msc.elf` the combined arm reaches 63/63.

The cost is precision, and it is real. Of the new entries (7 binaries):

| arm | new | ground truth | splits a real body | outside any DWARF body |
|---|---|---|---|---|
| `p` | 560 | 348 (62%) | 117 (21%) | 95 (17%) |
| `t` | 384 | 149 (39%) | **213 (55%)** | 22 (6%) |
| `tT` (stricter) | 222 | 120 (54%) | 84 (38%) | 18 (8%) |

`p` at 62% precision unrefined is a workable starting point (the "outside any DWARF body" 17%
is largely real newlib/compiler-rt code with no debug info, not error). `t` at 39-54% is not
shippable as written: without a containment predicate it splits loop heads.

Projected benchmark value, honestly bounded: 1,838 of the 2,941 misses are decbench-measured.
At the `p`-arm's measured recall on the pointer sub-class this is worth roughly **1,100-1,400
measured functions**, plus ~1,135 more u-boot functions of the same class the moment decbench
PR #58 lifts the `.text` filter — but only if the precision work lands with it.

## 9. Reproduction

The tables every number here comes from are checked in as `rivals-vs-kuna.txt`. The one-line
reproduction of the core claim:

```bash
kuna functions <stripped-arm-elf> --json \
    --option listing on --option funcstart_patterns on --option aif on
# compare the addresses against DW_AT_low_pc of the matching compiled/ binary
```
