Step 1 of the approved ARM entry-recall sequence ([PROPOSAL] #239), which measured
**2,026 of 2,061 live recall misses (98.3%) as function-entry granularity on embedded ARM
Cortex-M** and called this step SMALL and independent.

Full write-up: [`docs/features/cortexmvectors/analysis.md`](docs/features/cortexmvectors/analysis.md).

## The bug

`entry/mod.rs::cortexm_vector_table` (entry-discovery oracle 6) confirms a Cortex-M hardware
vector table only when it starts a section the loader maps executable, `word[0]` is in
`0x2000_0000..=0x3FFF_FFFF`, and `word[1] == e_entry`. All three over-constrain real
firmware, and between them they silently no-op the four biggest contributors to the gap:

| project | table section | `word[0]` | `word[1]` vs `e_entry` | detected today |
|---|---|---|---|---|
| chibios / libopencm3 / freertos / riot-os | `AX`, or `WA` in an `RWE` load (PR #224) | SRAM | equal | **yes** |
| cleanflight | `.isr_vector` `A`-only in a **read-only** `PT_LOAD` | `0x1000fff0` (CCM RAM) | equal | **no** |
| betaflight | same, linked into RAM at `0x20000000` | `0x1000fff0` | equal | **no** |
| crazyflie | `.isr_vector` @ `0x08004000` | `0x2001ffff` | `0x0801a811` vs `0x080041c0` | **no** |
| nuttx | `.text` @ `0x08000000` | `0x20001908` | `0x08000a95` vs `0x08000189` | **no** |

The table is *data the CPU reads* — what must be executable is what its slots point at,
which the harvest already checks. STM32F4/`-M7` parts put the initial stack in CCM/TCM at
`0x1000_0000`. And `e_entry` is the ELF's start symbol, which a bare-metal link script is
free to point somewhere other than the reset vector. PR #224 made the first argument and
stopped one step short.

A fourth defect surfaced while measuring: betaflight's table is linked *above* the flash it
points at, so the harvest's "stop once the scan reaches the start of code" rule fires on the
second slot and the table looks one word long.

## The fix — `--option cortexmvectors on|off` (default-OFF)

A candidate is confirmed when it starts **any allocated section**, `word[0]` lies anywhere in
`0x1000_0000..=0x3FFF_FFFF`, and its slots from `word[1]` yield **at least three Thumb handler
pointers** (odd, in an executable section). The run of handlers replaces the `e_entry`
equality; it is counted by the same harvest loop the oracle then seeds from, over accepted
*slots* rather than distinct addresses (a bare-metal table aims most vectors at one shared
`Default_Handler`). The start-of-code stop is additionally conditioned on the lowest handler
lying at or above the table base.

**Monotonic by construction:** the widened scan runs only where the shipped signature found
nothing, so an image that already resolved a table resolves the same section with the same
harvest. The option can add entries; it cannot remove one.

New module `decompiler/crates/kuna-analysis/src/analyzers/entry/kuna_cortexmvectors.rs`,
carrying a standalone `CortexMVectorsPass` (`id() == "cortexmvectors"`) stashed at load and
gated at commit by `engine.rs::analysis_pass_enabled` — the `eh_frame_full` /
`funcstart_patterns` shape.

## Acceptance measurement

Re-measured on this branch against the **stripped** binaries decbench feeds, with ground
truth from the matching `compiled/` binary's DWARF. 16 binaries x 3 optimisation levels,
41,378 ground-truth function addresses. Per-binary tables:
[`analysis.md` section 5](docs/features/cortexmvectors/analysis.md), raw diff in
[`measurements.txt`](docs/features/cortexmvectors/measurements.txt).

### Default decbench surface (`kuna functions`, Listing on)

| binary | entries before -> after | recall before -> after | new | ground truth | splits a real body | outside any body | lost |
|---|---|---|---|---|---|---|---|
| betaflight_STM32F405.elf (O0 / O2) | 5798 -> 5804 | 3877 -> 3882 | 6 | 5 | 0 | 1 | 0 |
| betaflight_STM32F405.elf (O2-noinline) | 6389 -> 6394 | 4513 -> 4517 | 5 | 4 | 0 | 1 | 0 |
| cleanflight_DALRCF405.elf (O0 / O2) | 2840 -> 2846 | 2060 -> 2065 | 6 | 5 | 0 | 1 | 0 |
| cleanflight_DALRCF405.elf (O2-noinline) | 3118 -> 3123 | 2386 -> 2390 | 5 | 4 | 0 | 1 | 0 |
| crazyflie cf2/firmware.elf (O0) | 2790 -> 2796 | 2474 -> 2478 | 6 | 4 | 0 | 2 | 0 |
| crazyflie cf2/firmware.elf (O2) | 2093 -> 2109 | 1461 -> 1474 | 16 | 13 | 0 | 3 | 0 |
| crazyflie cf2/firmware.elf (O2-noinline) | 2914 -> 2931 | 2243 -> 2257 | 17 | 14 | 0 | 3 | 0 |
| nuttx (O0 / O2 / O2-noinline) | +2 each | +2 each | 2 | 2 | 0 | 0 | 0 |
| chibios, libopencm3 (8 binaries), freertos, riot-os | unchanged | unchanged | 0 | — | — | — | 0 |

**Totals: recall 38,238 -> 38,334 (+96). 118 new entries, 96 (81.4%) ground-truth DWARF
function addresses, 0 splitting a real body, 22 outside any DWARF body. Zero entries lost,
on any binary, at any optimisation level.**

The 22 "outside any DWARF body" are vector slots pointing at debug-info-free
compiler-runtime/startup code (crazyflie's shared `Default_Handler` at `0x801a890`) — real
code the hardware enters, not error.

### Without the Listing tier (`--option listing off`: `--mode fast`, the WASM and Ghidra front-ends, single-function `kuna decompile`)

| binary | entries before -> after | recall before -> after |
|---|---|---|
| O2 betaflight_STM32F405.elf | 1 -> 57 | 1 -> 56 |
| O2 cleanflight_DALRCF405.elf | 491 -> 538 | 464 -> 510 |
| O2 crazyflie cf2.elf | 331 -> 350 | 305 -> 321 |
| O2 nuttx | 245 -> 248 | 239 -> 242 |

**Totals: recall 8,885 -> 9,278 (+393). 415 new entries, 393 (94.7%) ground truth, 0 splits,
0 lost.** betaflight goes from **one** recovered function to 57: this is the surface where
the whole-image Thumb region paint, which only a confirmed table unlocks, is load-bearing.

### Strict no-op elsewhere

Whole-binary `kuna decompile-all --json` is **byte-identical** (sha256) off vs on for x86-64
(`gzip`), ARM Linux (`u-boot`), and the four Cortex-M images the shipped signature already
detects (libopencm3 `msc.elf`, chibios `ch.elf`, freertos `RTOSDemo.out`, riot-os
`hello-world.elf`).

## A measured design decision worth reading

The widened table's facts can reach three consumers. All three were built and measured:

| wiring | ground-truth entries recovered | precision | splits | entries lost | GT lost |
|---|---|---|---|---|---|
| **entry facts + context paints only (shipped)** | **+96** | **81.4%** | **0** | **0** | **0** |
| + mirror the region paint into the walk's `ContextPainter` | +86 | 73.8% | 8 | 64 | 10 |
| + seed the Listing walk as well | +44 | 74.2% | 8 | 120 | 54 |

Handing the walk more seeds *costs* recall. Traced on crazyflie: the vector points at
`0x8010bd4`, an ISR stub `ldr r0,=i2c1Dev ; b i2cdrvDmaIsrHandler`. `listing/walk.rs` treats
an unconditional `B` as same-function flow, so the walk absorbs `i2cdrvDmaIsrHandler`
(`0x8010458`); that address is then no longer `is_undefined`, so
`code_pointer_table_seeds` rejects the code pointer to it and the AIF gap scan no longer sees
a gap. Fourteen ground-truth ISR handlers disappear that way on one binary. That is exactly
the tail-call split proposal step 3 owns — so this PR keeps the pass purely additive, which
also makes "never removes an entry" a property of the wiring rather than of a heuristic.

## Tests — a Rust two-pass gate, not a `tests/stages/*.xml`

The oracle keys entirely on **ELF structure**: `sh_flags`, `PT_LOAD` flags, `e_entry`, and
the bytes at a section's base. The datatest/stages harness embeds raw `<bytechunk>`s with
explicit `<symbol>` definitions and never constructs an `ObjectLoadImage`, so none of those
facts exist on that path and the option is unobservable there. Every sibling analysis-tier
discovery flag (`funcstart_patterns`, `aif`, `eh_frame_full`, `i386_pie_plt`) is gated the
same way. The two-pass case is therefore
`decompiler/crates/kuna-console/tests/verify_cortexmvectors.rs`, over a new 357-byte
hand-assembled fixture (`cortexm_ccm_vectors_le32`, with its committed generator and a
fixtures-README row) that fails all three shipped predicates at once:

* **pass 1, option off — the bug:** the four vector handlers are in no symbol table, and even
  the one function `e_entry` finds produces **no C at all** (nothing paints `TMode=1`, so its
  Thumb halfwords are decoded as A32);
* **pass 2, option on — the fix:** all five register and each decompiles to its real constant
  (`sub_8008004` -> `return 7;`). An A32 read of those bytes does not produce that, so the
  arithmetic is the Thumb-paint proof.

Consequently `kuna-base/src/xml.rs`' corpus file count and `docs/baseline-stages.json` are
untouched. Seven unit tests in the new module cover each real-world shape, the `.data`
false-positive floor, the two-vs-three-handler threshold, the non-ARM no-op, and
shipped-match monotonicity.

## Speed

Median of 3 whole-binary runs, off vs on: `kuna functions` cleanflight −3.33%, nuttx +2.88%,
msc.elf −1.81%; `kuna decompile-all` cleanflight −6.91%, nuttx +0.79%, msc.elf +8.87%. The
scan is O(sections x <=1024 slots) and unmeasurable; the deltas are the extra decompile work
for the extra functions against run-to-run noise — msc.elf is the byte-identical no-op case,
so its +8.87% on a 0.7-second run bounds the measurement error.
`scripts.pipeline.timeit` on the fixture: off 92.41 ms, on 85.13 ms, −7.88%, within the 5%
budget.

## Default decision

**Default-OFF, no DIV row.** The option discovers more functions, so it changes emitted C by
construction; default output is byte-identical. The `docs/baseline.json` ablation is
structurally clean (the XML datatest path never builds an `ObjectLoadImage`, so oracle 6
cannot fire there) but that is evidence of non-interference, not of quality. A default flip
belongs with a corpus sweep across more ARM firmware than decbench carries — the same
decision proposal step 2 defers.

## Gates

```
GATE1  make test         datatests: 675/675 assertions passed   ->  PARITY OK
GATE2  make test-stages  datatests: 356/356 assertions passed   ->  PARITY OK
GATE3  make rust-test    full cargo workspace suite             ->  green
GATE4  make check-spec   check-spec OK (lenient mode) / OK (strict mode)
       kuna catalog --check   catalog OK: documents exactly the registered kuna options
```

🤖 Generated with [Claude Code](https://claude.com/claude-code)
