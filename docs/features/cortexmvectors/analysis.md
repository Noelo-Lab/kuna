# cortexmvectors — widening the ARM Cortex-M vector-table signature

Step 1 of the approved ARM entry-recall sequence (`[PROPOSAL]` PR #239,
`docs/features/arm-entry-granularity/`). That proposal measured **2,026 of 2,061 live
recall misses (98.3%) as function-entry granularity on embedded ARM Cortex-M** and broke
the fix into four PRs; this is the one it called SMALL and independent.

Every number below is re-measured on this branch against the **stripped** binaries decbench
feeds the decompilers, with ground truth taken from the matching `compiled/` binary's DWARF
(`DW_TAG_subprogram` `DW_AT_low_pc`, Thumb-LSB-masked). Corpus: the six Cortex-M projects
plus freertos and riot-os — 16 binaries x 3 optimisation levels, 41,378 ground-truth
function addresses.

---

## 1. What the shipped signature demands, and what real firmware does

`entry/mod.rs::cortexm_vector_table` (oracle 6) confirms a hardware vector table when

1. it starts a section the **loader maps executable** (`SHF_EXECINSTR`, or — since PR #224 —
   any section inside a `PF_X` `PT_LOAD`),
2. `word[0]` is in the architectural SRAM block `0x2000_0000..=0x3FFF_FFFF`, and
3. `word[1] == e_entry`.

Measured section-by-section over the corpus:

| project | table section | flags | in a `PF_X` load? | `word[0]` | `word[1]` vs `e_entry` | detected today |
|---|---|---|---|---|---|---|
| chibios | `.vectors` @ `0x08000000` | `AX` | yes | `0x20000400` ok | `0x080002b9` == `0x80002b9` ok | **yes** |
| libopencm3 | `.text` @ `0x08000000` | `AX` | yes | `0x20030000` ok | `0x08001035` == `0x8001035` ok | **yes** |
| freertos / riot-os | `.isr_vector` in an `RWE` load | `WA` | yes (PR #224) | ok | ok | **yes** |
| cleanflight | `.isr_vector` @ `0x08000000` | `A` | **no** (`PF_R` load) | `0x1000fff0` FAIL | `0x08009809` ok | **no** |
| betaflight | `.isr_vector` @ `0x20000000` | `A` | **no** (`PF_R` load) | `0x1000fff0` FAIL | `0x0800bfa9` ok | **no** |
| crazyflie | `.isr_vector` @ `0x08004000` | `A` | yes | `0x2001ffff` ok | `0x0801a811` vs `0x080041c0` FAIL | **no** |
| nuttx | `.text` @ `0x08000000` | `AX` | yes | `0x20001908` ok | `0x08000a95` vs `0x08000189` FAIL | **no** |

All three predicates are category errors on real firmware:

* **The executable gate.** The table is *data the CPU reads*; what has to be executable is
  what its slots point at, which the harvest already checks. PR #224 made the same argument
  and stopped one step short — it widened to "inside a `PF_X` `PT_LOAD`", which still misses
  the far more common layout where the linker puts `.isr_vector` in its own **read-only**
  load ahead of the code (cleanflight, betaflight).
* **The SRAM window.** STM32F4 (CCM) and `-M7` (TCM) parts place the initial stack at
  `0x1000_0000`. `0x1000fff0` is a perfectly ordinary Cortex-M reset SP.
* **`word[1] == e_entry`.** `e_entry` is the *ELF*'s start symbol. A bare-metal link script
  is free to point it elsewhere: nuttx points it at `__start`, crazyflie at the `.text` base.

## 2. A fourth defect, found while measuring

Even with the signature widened, **betaflight still recovered nothing**. Its table is linked
into RAM at `0x2000_0000` (for runtime relocation) while its handlers live in flash at
`0x0800_xxxx`. `harvest_vector_words`' "stop once the scan address reaches the lowest
handler — that is the start of code" rule then fires on the *second* slot
(`0x20000008 >= 0x0800bfa8`) and the table looks one word long, so it never reaches the
three-handler floor.

That stop only means anything when the code follows the table in the same address region.
The widened harvest therefore additionally requires the lowest handler to lie at or above the
table's own base before the stop can apply. Unit test:
`ram_relocated_table_below_its_handlers_still_harvests`.

## 3. The widened signature

A candidate is confirmed when

1. it starts **any allocated section** (`SHF_ALLOC`; executable or not),
2. `word[0]` is in `0x1000_0000..=0x3FFF_FFFF` (SRAM + CCM/TCM), and
3. the slots from `word[1]` on yield at least **three Thumb handler pointers** (odd, inside
   an executable section).

Requirement 3 is what replaces `e_entry`. It is counted by the *same* harvest loop the
oracle then seeds from, over accepted **slots** rather than distinct addresses — a
bare-metal table aims most of its vectors at one shared `Default_Handler` (nuttx's first
fourteen slots resolve to two addresses, chibios' first eleven to three), so counting
addresses would reject the very tables this exists to find.

Three is enough on its own. cleanflight's `.text` opens with `0x2b011c4b`, which *is* inside
the SRAM window by accident; its next word is even, so the run is length 0 and it is
rejected. Every `.data` in the corpus fails at slot 1 or 2 the same way. Unit test:
`sram_first_word_alone_is_not_a_table`.

**Monotonic by construction:** the widened scan runs only where the shipped signature found
nothing, so an image that already resolved a table resolves the same section with the same
harvest. Unit test: `shipped_match_wins_over_the_widened_scan`.

## 4. Wiring — three variants measured, the conservative one wins

The widened table's facts can reach three consumers. All three combinations were built and
measured on the full decbench ARM surface (`kuna functions`, which auto-injects
`listing`+`funcstart_patterns`+`aif` on non-x86-64):

| wiring | ground-truth entries recovered | new entries | of which ground truth | splits a real body | entries LOST | of which ground truth |
|---|---|---|---|---|---|---|
| entry facts + context paints only | **+96** | 118 | 96 (**81.4%**) | **0** | **0** | **0** |
| + mirror the region paint into the walk's `ContextPainter` | +86 | 130 | 96 (73.8%) | 8 | 64 | 10 |
| + seed the walk (`passes::listing_seeds`) as well | +44 | 132 | 98 (74.2%) | 8 | 120 | 54 |

Handing the Listing walk more seeds *costs* recall. The mechanism, traced on crazyflie: the
vector points at `0x8010bd4`, a two-instruction ISR stub `ldr r0,=i2c1Dev ; b
i2cdrvDmaIsrHandler`. `listing/walk.rs` treats an unconditional `B` as same-function flow,
so the walk absorbs `i2cdrvDmaIsrHandler` (`0x8010458`) into the stub; that address is then
no longer `is_undefined`, so `aif/mod.rs::code_pointer_table_seeds` rejects the code pointer
to it and the AIF gap scan no longer sees a gap there. Fourteen ground-truth ISR handlers
disappear that way on one binary. This is exactly the tail-call split that proposal step 3
owns; until it lands, the conservative wiring is strictly better on every axis, so the pass
emits entry facts and context paints and nothing else.

The same reasoning fixed a second wiring bug found on the way: reading the flag inside
`EntryDiscoveryPass::run` silently did nothing, because load-time passes run *before*
`--option` is applied. The widening therefore ships as its own `CortexMVectorsPass` with the
stash-at-load / gate-at-commit shape (`engine.rs::analysis_pass_enabled`) that
`eh_frame_full` and `funcstart_patterns` use.

## 5. Acceptance measurement

### 5.1 The default decbench surface (`kuna functions`, Listing on)

| binary | entries before | after | recall before | after | new | GT | splits | outside any body | lost |
|---|---|---|---|---|---|---|---|---|---|
| O0 betaflight_STM32F405.elf | 5798 | 5804 | 3877 | 3882 | 6 | 5 | 0 | 1 | 0 |
| O0 cleanflight_DALRCF405.elf | 2840 | 2846 | 2060 | 2065 | 6 | 5 | 0 | 1 | 0 |
| O0 crazyflie cf2.elf | 2790 | 2796 | 2474 | 2478 | 6 | 4 | 0 | 2 | 0 |
| O0 crazyflie firmware.elf | 2790 | 2796 | 2474 | 2478 | 6 | 4 | 0 | 2 | 0 |
| O0 nuttx | 924 | 926 | 910 | 912 | 2 | 2 | 0 | 0 | 0 |
| O2 betaflight_STM32F405.elf | 5798 | 5804 | 3877 | 3882 | 6 | 5 | 0 | 1 | 0 |
| O2 cleanflight_DALRCF405.elf | 2840 | 2846 | 2060 | 2065 | 6 | 5 | 0 | 1 | 0 |
| O2 crazyflie cf2.elf | 2093 | 2109 | 1461 | 1474 | 16 | 13 | 0 | 3 | 0 |
| O2 crazyflie firmware.elf | 2093 | 2109 | 1461 | 1474 | 16 | 13 | 0 | 3 | 0 |
| O2 nuttx | 740 | 742 | 668 | 670 | 2 | 2 | 0 | 0 | 0 |
| O2-noinline betaflight_STM32F405.elf | 6389 | 6394 | 4513 | 4517 | 5 | 4 | 0 | 1 | 0 |
| O2-noinline cleanflight_DALRCF405.elf | 3118 | 3123 | 2386 | 2390 | 5 | 4 | 0 | 1 | 0 |
| O2-noinline crazyflie cf2.elf | 2914 | 2931 | 2243 | 2257 | 17 | 14 | 0 | 3 | 0 |
| O2-noinline crazyflie firmware.elf | 2914 | 2931 | 2243 | 2257 | 17 | 14 | 0 | 3 | 0 |
| O2-noinline nuttx | 890 | 892 | 822 | 824 | 2 | 2 | 0 | 0 | 0 |
| chibios / libopencm3 (8) / freertos / riot-os, all 3 levels | — | unchanged | — | unchanged | 0 | — | — | — | 0 |

**Totals: recall 38,238 -> 38,334 (+96) of 41,378. 118 new entries, 96 (81.4%) ground-truth
DWARF function addresses, 0 splitting a real body, 22 outside any DWARF body. Zero entries
lost, on any binary, at any optimisation level.**

The 22 "outside any DWARF body" are vector slots pointing at debug-info-free
compiler-runtime/startup code (crazyflie's shared `Default_Handler` at `0x801a890`,
betaflight's `0x806fd24`) — real code the hardware genuinely enters, not error.

### 5.2 Without the Listing tier (`--option listing off`)

The surface used by `--mode fast`, the WASM and Ghidra front-ends, and single-function
`kuna decompile`. Here the entry oracles are the *only* discovery source, so the widening
carries the whole weight:

| binary | entries before | after | recall before | after |
|---|---|---|---|---|
| O2 cleanflight_DALRCF405.elf | 491 | 538 | 464 | 510 |
| O2 crazyflie cf2.elf | 331 | 350 | 305 | 321 |
| O2 nuttx | 245 | 248 | 239 | 242 |
| O2 betaflight_STM32F405.elf | 1 | 57 | 1 | 56 |

**Totals: recall 8,885 -> 9,278 (+393). 415 new entries, 393 (94.7%) ground truth, 0 splits,
0 lost.** betaflight goes from **one** recovered function to 57 — this is the surface where
the whole-image Thumb region paint, which only a confirmed table unlocks, is load-bearing.

### 5.3 Strict no-op elsewhere

Whole-binary `kuna decompile-all --json` output is **byte-identical** (sha256) off vs on for
x86-64 (`gzip`), ARM Linux (`u-boot`), and the four Cortex-M images the shipped signature
already detects (libopencm3 `msc.elf`, chibios `ch.elf`, freertos `RTOSDemo.out`, riot-os
`hello-world.elf`).

## 6. Speed

Median of 3 whole-binary runs, off vs on:

| surface | binary | off | on | delta |
|---|---|---|---|---|
| `kuna functions` | cleanflight | 3602.6 ms | 3482.7 ms | -3.33% |
| `kuna functions` | nuttx | 1180.4 ms | 1214.4 ms | +2.88% |
| `kuna functions` | libopencm3 msc.elf | 145.8 ms | 143.2 ms | -1.81% |
| `kuna decompile-all` | cleanflight | 45428.0 ms | 42288.2 ms | -6.91% |
| `kuna decompile-all` | nuttx | 13115.2 ms | 13218.6 ms | +0.79% |
| `kuna decompile-all` | libopencm3 msc.elf | 669.4 ms | 728.8 ms | +8.87% |

The scan itself is O(sections x <=1024 slots) and unmeasurable; the deltas are the extra
decompile work for the extra functions, against run-to-run noise. msc.elf is the
shipped-signature no-op case (byte-identical output, section 5.3), so its +8.87% on a
0.7-second run is pure noise and bounds the measurement error.
`scripts.pipeline.timeit` on the fixture: off 92.41 ms, on 85.13 ms, -7.88%, within the
5% budget.

## 7. Default decision

**Default-OFF.** The option discovers more functions, so it changes emitted C by
construction. The `docs/baseline.json` ablation is structurally clean — the XML datatest
path never builds an `ObjectLoadImage`, so oracle 6 cannot fire there; measured 675/675
PARITY OK — but that is evidence of non-interference, not of quality. A default flip belongs
with a corpus sweep across more ARM firmware than decbench carries, and is the same decision
proposal step 2 defers. No DIV row: default output is byte-identical.

## 8. Reproduction

```bash
# entry recall, before vs after, against DWARF ground truth
kuna functions <stripped-cortexm-elf> --json                              # before
kuna functions <stripped-cortexm-elf> --json --option cortexmvectors on   # after
# compare both against DW_AT_low_pc of the matching compiled/ binary

# the strict no-op check
kuna decompile-all <x86-64-or-already-detected-cortexm-elf> --json | sha256sum
kuna decompile-all <same> --json --option cortexmvectors on | sha256sum
```
