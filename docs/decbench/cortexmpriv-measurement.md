# `cortexmpriv`: the Cortex-M privilege-guard fold, measured (2026-08-29)

The evidence behind DIV-99. `cortexmpriv` registers a synthesized `<callotherfixup>` that
makes the vendored ARM user op `isCurrentModePrivileged` return the constant `1`, so the
privilege guard `ARMTHUMBinstructions.sinc` wraps around every VERSION_7M MRS/MSR folds
away. It ships default-OFF and ON in the `aggressive` preset.

Every number here is from one kuna binary (`bb0b9c38`) invoked exactly as decbench's
`kuna_raw` backend invokes it -- `kuna decompile-all <stripped> --json --max-fn-seconds 120`
with no `--mode`, so `auto` resolves to `aggressive` under 500 KiB.

**Arm naming.** `optsweep`'s *baseline* arm is the shipped defaults, which on this branch
means the feature is ON; its *variant* arm is `--option cortexmpriv off`, which is main
HEAD. Everything below is restated in FEATURE terms, so `optsweep`'s `off perfect` column
is the feature's wins.

## 1. Corpus-scale, bidirectional

`scripts.decbench.optsweep --option cortexmpriv off --opt O0 --opt O2 --opt O2-noinline`
over the eight Cortex-M projects: **90 slices, 31,044 functions scored in both arms.**

| | |
|---|---|
| move ONTO GED-perfect | **93** |
| move OFF GED-perfect | **0** |
| improved / worsened | 336 / 98 |
| aggregate GED | 236,133 (off) → 230,129 (on), **−6,004** |
| GED removed / added | −7,033 / +1,029 (6.8 : 1) |
| McNemar z | 9.64 |

Per project (Δ aggregate GED, feature wins): nuttx −2,801, chibios −1,495, crazyflie −1,300,
cleanflight −256, riot-os −56. Two projects are flat and both are informative: `libopencm3`
is Cortex-M but has no MRS/MSR in any scored function, and `betaflight` is 533 KB, so `auto`
picks `reliable`, the preset never reaches it, and its two arms are identical — a control
inside the sweep.

Harness controls, all clean: 30,600 unchanged bodies with **0** scoring differently; 0
slug-only diffs; the baseline arm agrees with the tree's published per-function verdict on
**30,612 / 31,044 (98.6%)**, the residual being kuna-build drift since the tree was scored.

## 2. DecBench sample set (250 functions): +2

Only 49 of the 250 sit on a guard-bearing binary, and all 49 were re-scored in both arms
through decbench's own `GEDMetric` (the baseline arm reproduces every published value
exactly). Four change:

| case | off | on | |
|---|---:|---:|---|
| `O0-riot-os-hello-world-thread_sleep` | 24 | **0** | flip |
| `O0-nuttx-nuttx-nxsched_process_scheduler` | 26 | **0** | flip |
| `O2-noinline-freertos-RTOSDemo-xFirstTimerHandler` | 84 | 14 | improvement |
| `O0-nuttx-nuttx-nxsig_find_pendingsignal` | 18 | 9 | improvement |

Neither flip was perfect on any metric before, so both are genuine `overall` gains. Nothing
regresses. Two of the 49 are unscorable in BOTH arms identically (a `CMSIS_DAP::main` kuna
never discovers, and a `__swrite` with no source CFG), so neither can move; the other 201
sample-set functions live on a binary with no MRS/MSR and are unchanged by construction.

## 3. Collateral: zero

`decompile-all --json` over the 26 guard-bearing decbench binaries, `cortexmpriv off` vs the
shipped default. **46,747 function bodies; discovery identical on every binary (same function
set, same error count); 868 changed bodies, of which 868 contain the guard.** Not one
function without an MRS/MSR changes.

| binary | fns off | fns on | err off | err on | changed | guard-bearing |
|---|---:|---:|---:|---:|---:|---:|
| O0 betaflight/betaflight_STM32F405.elf | 5797 | 5797 | 2 | 2 | 0 | 0 |
| O0 chibios/ch.elf | 772 | 772 | 0 | 0 | 81 | 81 |
| O0 cleanflight/cleanflight_DALRCF405.elf | 2559 | 2559 | 0 | 0 | 10 | 10 |
| O0 crazyflie/CMSIS_DAP.axf | 318 | 318 | 0 | 0 | 24 | 24 |
| O0 crazyflie/cf2.elf | 2681 | 2681 | 0 | 0 | 24 | 24 |
| O0 crazyflie/firmware.elf | 2681 | 2681 | 0 | 0 | 24 | 24 |
| O0 freertos/RTOSDemo.out | 150 | 150 | 0 | 0 | 15 | 15 |
| O0 libopencm3/usart_irq_console.elf | 93 | 93 | 0 | 0 | 1 | 1 |
| O0 nuttx/nuttx | 930 | 930 | 0 | 0 | 104 | 104 |
| O0 riot-os/hello-world.elf | 129 | 129 | 0 | 0 | 17 | 17 |
| O2-noinline betaflight/betaflight_STM32F405.elf | 6388 | 6388 | 1 | 1 | 0 | 0 |
| O2-noinline chibios/ch.elf | 720 | 720 | 0 | 0 | 77 | 77 |
| O2-noinline cleanflight/cleanflight_DALRCF405.elf | 2825 | 2825 | 0 | 0 | 19 | 19 |
| O2-noinline crazyflie/cf2.elf | 2839 | 2839 | 0 | 0 | 21 | 21 |
| O2-noinline crazyflie/firmware.elf | 2839 | 2839 | 0 | 0 | 21 | 21 |
| O2-noinline freertos/RTOSDemo.out | 154 | 154 | 0 | 0 | 15 | 15 |
| O2-noinline nuttx/nuttx | 954 | 954 | 0 | 0 | 105 | 105 |
| O2-noinline riot-os/hello-world.elf | 171 | 171 | 0 | 0 | 17 | 17 |
| O2 betaflight/betaflight_STM32F405.elf | 5797 | 5797 | 2 | 2 | 0 | 0 |
| O2 chibios/ch.elf | 398 | 398 | 0 | 0 | 102 | 102 |
| O2 cleanflight/cleanflight_DALRCF405.elf | 2559 | 2559 | 0 | 0 | 10 | 10 |
| O2 crazyflie/cf2.elf | 1973 | 1973 | 0 | 0 | 21 | 21 |
| O2 crazyflie/firmware.elf | 1973 | 1973 | 0 | 0 | 21 | 21 |
| O2 freertos/RTOSDemo.out | 126 | 126 | 0 | 0 | 16 | 16 |
| O2 nuttx/nuttx | 792 | 792 | 0 | 0 | 106 | 106 |
| O2 riot-os/hello-world.elf | 129 | 129 | 0 | 0 | 17 | 17 |

Two further controls:

- **`cortexmpriv off` on this branch is byte-identical to unmodified HEAD** on all 46,747
  bodies, with identical discovery. Registering the payload is therefore provably inert (it
  shifts no inject tempbase and has no other side effect); only consuming it changes output.
- Ten non-guard binaries picked at random from the sample set (4,148 bodies), one of them
  ARM with no MRS/MSR: **0 changed lines** against HEAD at the shipped default.

The synthesized body compiles at architecture bootstrap on all 15 ARM language variants
(LE and BE, v4 through v8-m) and is a measured no-op on AArch64 and x86-64.

## 4. What the number is NOT

**98 of the 434 changed functions score WORSE on GED.** That is a property of the metric,
not lost recovery: GED counts blocks and edges, not correctness, so deleting phantom blocks
widens the node deficit wherever kuna's CFG is already smaller than the source's. The split
is exactly that shape — the 336 wins have a **median source CFG of 4 nodes** (the guard
blocks were pure surplus), the 98 losses a **median of 11** (the body was already
node-deficient).

NuttX `arm_hardfault` is the worked example. Source CFG: 7 nodes. With the option on:

```c
unsigned int sub_80015b8(unsigned int a0,unsigned int a1) {
  getBasePriority();
  setBasePriority(0x80);
  sub_80024ac(0,0,dat_8001610,a1);
  return 0;
}                                            // GED 26
```

with it off, the same four statements wrapped in two tests that appear in no source:

```c
unsigned int sub_80015b8(unsigned int a0,unsigned int a1) {
  if ((bool)isCurrentModePrivileged()) getBasePriority();
  if ((bool)isCurrentModePrivileged()) setBasePriority(0x80);
  sub_80024ac(0,0,dat_8001610,a1);
  return 0;
}                                            // GED 8
```

The `on` body is the `off` body minus the guards; nothing is lost in either direction. The
93-to-perfect / 0-off-perfect column is the part that moves a scoreboard, and it is
one-directional.

## 5. Speed

Interleaved, alternating arms, min-of-N **child CPU time** (the box was running several
other agents' builds throughout, which is why wall clock alone is not usable and why min-of-N
matters: on `ch.elf` the same A/B read **+7.2% at min-of-7, −0.6% at min-of-15 and +0.2% at
min-of-21**, so anything under ~15 reps on this box is noise):

| binary | fns | reps | CPU off | CPU on | Δ |
|---|---:|---:|---:|---:|---:|
| O0 riot-os/hello-world.elf | 129 | 31 | 0.822 s | 0.812 s | −1.3% |
| O0 freertos/RTOSDemo.out | 150 | 31 | 0.987 s | 0.935 s | −5.3% |
| O0 chibios/ch.elf | 772 | 21 | 3.653 s | 3.661 s | **+0.2%** |
| O0 nuttx/nuttx | 930 | 21 | 8.347 s | 8.290 s | **−0.7%** |
| O0 cleanflight/cleanflight_DALRCF405.elf | 2559 | 9 | 27.438 s | 25.303 s | −7.8% |
| O2-noinline crazyflie/cf2.elf | 2839 | 15 | 31.105 s | 30.497 s | −2.0% |
| *null control* — cf2.elf, the SAME arm twice | 2839 | 9 | 32.570 s | 31.003 s | *−4.8%* |

The null control is the point: the identical harness, comparing `--option cortexmpriv off`
against itself, reads **−4.81%**. Nothing inside ±5% at min-of-9 on this box is signal, and
`cf2.elf` demonstrates it directly — **+9.8% at min-of-9, −2.0% at min-of-15**. The two
min-of-21 rows are the trustworthy ones and both are flat (−0.7%, +0.2%).

Folding the guard removes blocks, so if anything the sign should be favourable, and five of
the six rows are negative. The honest claim is **flat**: no measurable cost, and no speedup
worth quoting either. Off ARM the payload is never registered, so the cost there is
structurally zero.
