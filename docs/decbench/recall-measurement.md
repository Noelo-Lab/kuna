# Where the missing functions actually went (2026-08-01)

The recall pool — functions kuna produced **no output for** while a rival did — is the largest
single item in the campaign, so it got a measurement rather than an estimate. This file records
what that measurement found, including the parts that contradict what the campaign believed.

## Method

A function counts as *measured* if at least one non-LLM rival (angr / binja / dewolf / ghidra /
ida / phoenix / r2dec) has `decompiled: true` in the run's `function_results.json`, and *missing*
if kuna does not. Matching is by **address**, Thumb-bit tolerant — never by name, because
decbench relabels each function's own `sub_<addr>` placeholder to its DWARF name post-hoc.

`kuna decompile-all --json --mode reliable --max-fn-seconds 120` over **191 binaries** (all 126
carrying a recorded miss, plus 65 controls). Every run exited 0; the longest was 116 s.

Two methodological points that changed the answer:

- **The stored `kuna_*.c` artifacts are 12-16 days older than the fixes being measured**
  (2026-07-15/16, betaflight O2-noinline 07-23). A raw before/after therefore conflates the fix
  wave with two weeks of unrelated main drift. The sweep was re-run against a **pre-wave build**
  to separate them, and against the individual PR branches to attribute per-PR.
- **PE targets record RVAs in the rival artifacts while kuna reports ImageBase+RVA**
  (mydoom / minipig / x0r-usb `+0x400000`, dexter `+0x69940000`). Uncorrected, this fabricates
  **181 fake gaps**.

## The pool is 7,354, not 6,540

The earlier 6,540 figure could not be reproduced under any rival-subset or filter variant. The
counts agree exactly on the small projects (coreutils 17, openssh 16, bash 9, nuttx 87) and run
5-15% higher on the embedded ones (u-boot 2,978 vs 2,553; betaflight 2,577 vs 2,464). **7,354 is
the number to use.**

## Where it went

| bucket | count | share |
|---|---|---|
| Genuinely recovered by the 2026-08-01 fix wave | **63** | 0.9% |
| Already fixed before the wave — the stored artifact was simply stale | 1,994 | 27.1% |
| decbench-side `.text`-filter artifact — never kuna's fault | 3,233 | 44.0% |
| **Still missing, real kuna gap** | **2,061** | 28.0% |
| name unresolvable to an address | 3 | — |

So the wave's *benchmark-visible* recall gain is small, and the campaign should say so. Two
honest caveats in the other direction:

- **#224 recovered 256 freertos functions decbench structurally cannot score** (5 → 261 of 265
  covered), because the `.text` filter discards that project wholesale. The wave's true
  capability gain is roughly **319 measured + 176 unmeasured** functions.
- Both fix PRs **under-claimed**. #227 reported 19 recovered, measured **50**; #234 reported 6,
  measured **13**.

## The 44% that was never kuna's fault

u-boot's `.text` is 936 bytes — the real code lives in `.text_rest` (0x76b40). freertos is built
`-ffunction-sections` with 183 sections, `.text` covering only `0x78`-`0xbef`. decbench's raw
backends keep only functions inside `.text`, so **every** raw decompiler scores zero on those
rows; only dewolf and r2dec, which skip the filter, supplied their measured sets.

decbench PR #58 fixes this. When it lands, **2,094 of the 3,233 flip to covered instantly** — and
**1,135 u-boot functions become a newly visible real gap** (954 of them sit inside a function
kuna already enumerates, 179 nearby, 2 absent). That is a gap the artifact is currently hiding,
not one the fix creates.

Separately measured, past the filter: kuna covers **261 of freertos's 265** measured functions
and **1,833 of u-boot's 2,968**.

## The largest remaining real gap is entry granularity, not decompilation

**2,026 of the 2,061 live misses (98.3%) are function-entry granularity on embedded ARM.** kuna
produces a body for code *containing* the DWARF `low_pc` but does not start a function *at* it.
Median distance to the containing kuna entry is **24 bytes** (p25 10 B, p75 58 B; 74% within
64 B). Distribution: betaflight 761, crazyflie 522, cleanflight 352, libopencm3 136, chibios 115,
nuttx 87, riot-os 33, x0r-usb 15, mydoom 9, u-boot 4.

Some is ground-truth noise — `NMI_Handler` / `PendSV_Handler` / `DebugMon_Handler` are three
DWARF symbols on one `b .` loop. Most is real: small statics reached only by fall-through or
tail-call get absorbed into the preceding function (`_usbd_standard_request_device`,
`devnull_read` / `devnull_write`, `__ch_delegate_fn0/1`, `SCSI_Inquiry`).

The remaining 35: 24 error records, 8 nearby-but-not-contained, 3 absent.

> **Corrected 2026-08-01 by the follow-up investigation
> ([`docs/features/arm-entry-granularity/`](../features/arm-entry-granularity/analysis.md)).**
> The *count* holds — an independent address-based re-measurement on this build reproduces the
> per-project distribution (libopencm3 136 and nuttx 87 exactly). The *mechanism* and the
> *noise* claim above do not:
>
> - It is **not absorption**. Of 2,941 missed addresses, **1,896 sit past the end of the nearest
>   kuna function and exactly 3 are strictly inside a kuna body**. The 24-byte median measures
>   proximity, not containment. `devnull_read`/`devnull_write` and
>   `_usbd_standard_request_device` are not merged into a neighbour — their bytes produce **no
>   output at all**. This is missing decompilation, not a naming/boundary problem.
> - The `NMI_Handler` / `PendSV_Handler` / `DebugMon_Handler` example is **not noise**: in
>   cleanflight they are three *distinct* two-byte `bx lr` stubs at `0x803b060`/`0x803b062`/
>   `0x803b064`, each with its own vector-table slot, and Ghidra, IDA, angr and Binary Ninja all
>   recover all three. Real ground-truth noise is **3.4%** — 105 alias/ICF names on addresses
>   already counted once.
> - The dominant sub-class is not fall-through/tail-call but **code-pointer-referenced entries
>   (57%, 1,402 of them decbench-measured)** that `aif::code_pointer_table_seeds` already finds
>   and rejects on its frame-prologue and >2-instruction guards.

## Remaining error records

240 pre-wave → **137** now, across the 191-binary sweep. The generic
`un-ported seam (LOSS-131): panic with non-string payload` class went **109 → 0**, giving 101
functions bodies (63 of them decbench-measured).

| class | pre | now | where |
|---|---|---|---|
| `Unable to load N bytes at r0x…` | 127 | 125 | crazyflie 106, cleanflight 12, betaflight 5, rsyslog 1, u-boot 1 |
| `rename_recurse: set_input_varnode (empty stack): "Overlapping input varnodes"` | 0* | 8 | O2 x86-64 only |
| `SLEIGH exceeded maximum parse depth` | 4 | 4 | crazyflie `0x800b740`/`0x800b750` |
| generic panic | 109 | 0 | — |

\* not new — those addresses were already failing under the generic message; DIV-45 merely made
them nameable.

**The `rename_recurse` class is the highest-value remaining item.** A sweep of 213 additional O2
binaries found **48 hit it, exactly one function each**, and where a rival names the address it is
**`rpl_fcntl`** (gnulib's `fcntl` replacement). It is `-O2`-only — the same binaries at O0 and
O2-noinline are clean. Benchmark cost is only 2 functions (decbench narrows to project-source
functions, and gnulib is usually out of scope), but it is a hard no-output failure in ~22% of
optimized x86-64 binaries.

`Unable to load` is a wild computed-branch target — crazyflie `sub_80043b8` asks for 512 bytes at
`r0x0643fc14` while the image spans `0x08004000`-`0x0804db00`. 22 of the 125 are measured.

## Regression check

Pre-wave → post-wave over 191 binaries: **130,815 → 131,307 non-error bodies (+492)**. Exactly
**3 addresses disappear**, all freertos, all absorbed into a neighbour that the new vector-table
seeding now starts *earlier*. **Zero measured functions lost anywhere.**

Isolating the emission-only PRs (#226/#228/#229/#231/#232/#235) against the #227 branch over the
49 affected binaries: **54,613 → 54,629 bodies, 0 lost** — so "these change output but not which
functions are produced" is verified, not assumed.
