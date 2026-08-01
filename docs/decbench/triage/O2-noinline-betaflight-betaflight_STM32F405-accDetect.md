---
case_id: O2-noinline-betaflight-betaflight_STM32F405-accDetect
pool: novel
status: feature-candidate
tier: N
margin: 1
fresh_verdict: kuna still wins the case (GED 10 vs ghidra 11) and its 19 gotos match ghidra exactly, but every call reachable only through the jump table keeps a spurious `setISAMode(1);` statement that ghidra/ida do not emit — 10 of them in this function, reproduced on today's build in both `--mode reliable` and `--mode aggressive`.
option_closing: null
feature_slug: jumptable-callother-inject
scope: small
confidence: high
---

## Side-by-side

Source (the real shape — one `switch` with `FALLTHROUGH` chains and one `goto retry`):

```c
retry:
    switch (accHardwareToUse) {
    case ACC_DEFAULT:
        ;__attribute__ ((fallthrough));
    case ACC_MPU6050:
        if (mpu6050AccDetect(dev)) { accHardware = ACC_MPU6050; break; }
        ;__attribute__ ((fallthrough));
    case ACC_MPU6000:
        if (mpu6000SpiAccDetect(dev)) { accHardware = ACC_MPU6000; break; }
        ;__attribute__ ((fallthrough));
    ...
```

ghidra (stored) — jump-table dispatch, no userop residue:

```c
  do {
    switch(param_2) {
    case 0:
    case 3:
      iVar1 = FUN_08060c08(param_1);
      if (iVar1 != 0) { cVar2 = '\x03'; goto LAB_08041eea; }
      break;
    ...
    }
    iVar1 = FUN_08062120(param_1);
    if (iVar1 != 0) { cVar2 = '\a'; goto LAB_08041eea; }
switchD_08041e80_caseD_8:
    iVar1 = FUN_0806220c(param_1);
    if (iVar1 != 0) {
      switch(*(undefined1 *)(param_1 + 0x14)) {
      case 6: cVar2 = '\t'; break;
      ...
      default: cVar2 = '\b';
      }
```

kuna (fresh, today's build) — identical control flow, plus a `setISAMode(1);` in front of
every table-reachable call:

```c
  do {
    switch(a1) {
      case 0:
      case 3:
        setISAMode(1);                      /* <-- spurious */
        if (sub_8060c08(a0)) { v1 = '\x03'; goto label_8041eea; }
        break;
      ...
    }
    setISAMode(1);                          /* <-- spurious */
    if (sub_8062120(a0)) { v1 = '\a'; goto label_8041eea; }
label_8041e00:
    setISAMode(1);                          /* <-- spurious */
    if (sub_806220c(a0)) {
      if (4 <= (uint4)*(uint1 *)(a0 + 0x14) - 6) // branch-flip
        v1 = '\b';
      else {
        setISAMode(1);                      /* <-- spurious, in the tbb dispatch block */
        switch(*(uint1 *)(a0 + 0x14)) { case 6: ... case 9: v1 = '\f'; }
      }
```

kuna (stored, 2026-07-27) has the same 10 `setISAMode(1)` statements and the same 19 gotos —
only the print-normalisation wave (DIV-34..39: truthy conditions, brace elision, inline
`// branch-flip`) differs. Structure is unchanged.

A control function in the same binary proves the residue is not "all ARM Thumb calls":
`accInitFilters` (no jump table) calls `pt2FilterGain`/`pt2FilterInit` and emits **zero**
`setISAMode`; `gyroDetect` (has a recovered jump table, but its table targets are pure
constant assignments and all its `bl`s are in pre-table fall-thru flow) also emits zero.
`blackboxUpdate` (jump table first, whole body table-reachable) emits **56**.

## Source

`~/github/decbench/results/full_run/O2-noinline/betaflight/compiled/acceleration_init.i:9198`,
`accDetect(accDev_t *dev, accelerationSensor_e accHardwareToUse)` — a `retry:` label plus one
switch whose ~12 case groups fall through into each other, each `if (xxxAccDetect(dev)) {
accHardware = X; break; }`, with two nested switches on `dev->mpuDetectionResult.sensor` and
a `default: case ACC_NONE:` arm, then:

```c
    if (accHardware == ACC_NONE && accHardwareToUse != ACC_DEFAULT && accHardwareToUse != ACC_NONE) {
        accHardwareToUse = ACC_DEFAULT;
        goto retry;
    }
    if (accHardware == ACC_NONE) return 0;
    detectedSensors[SENSOR_INDEX_ACC] = accHardware;
    sensorsSet(SENSOR_ACC);
    return 1;
```

Source CFG: 38 nodes / 64 edges, so the GED is exact (not approximated).

## Analysis

**The miner hint is wrong.** "19 gotos vs a switch in source, likely a P8 structuring gap" does
not survive contact with the panes: ghidra emits **exactly the same 19 gotos / 9 labels / 2
switches / 18 cases**, and IDA's prettier 10-goto rendering (case labels welded onto the
fall-through chain blocks) scores *worse* (GED 17 vs kuna 10, ghidra 11). Recovering the
source's `case X:`-attached fall-through chain would be a large P8 feature that the metric does
not even reward here; it is not this case's finding. `condfold on` and `condfold wide` both
leave the goto count at 19, and `--mode aggressive` changes nothing (the stripped image is
537 KiB, so `--mode auto` already resolves to `reliable` for this binary — the benchmark's
mode and today's product default coincide).

**The one structural symptom:** 10 spurious `setISAMode(1);` statements in the emitted C,
one in front of every call site (and one in the inner `tbb` dispatch block) that is reachable
only through the recovered jump table. No other decompiler emits them, and kuna itself does not
emit them for the same instruction in the same binary when the block is reached by ordinary
fall-thru.

**Root cause (P2 lift/flow).** Thumb-2 `bl` and `tbb` lower through
`SetThumbMode(1)` -> `setISAMode(TB)`
(`specs/Ghidra/Processors/ARM/data/languages/ARMTHUMBinstructions.sinc:1484,4916`), so each one
raises a `CALLOTHER`. `ARM.cspec:307` supplies the standard NOP eraser:

```xml
<callotherfixup targetop="setISAMode">
  <pcode incidentalcopy="true"><input name="newTB"/><body><![CDATA[ r0 = r0; ]]></body></pcode>
</callotherfixup>
```

kuna does parse and register it (`infra/architecture.rs:2382 init_userops_and_fixups`), and
`p2_lift/flow.rs:1193-1203` (`xref_control_flow`) queues every injected-userop `CALLOTHER` onto
`injectlist`. But the drain runs **once**:

`decompiler/crates/kuna-decomp/src/p2_lift/flow.rs:1735`
```rust
    pub fn generate_ops(&mut self) -> KunaResult<()> {
        self.clear_properties();
        self.addrlist.push(self.data.get_address().clone());
        while !self.addrlist.is_empty() {
            self.fallthru()?;                 // pre-jumptable flow
        }
        if self.has_inject() {
            self.inject_pcode()?;             // ... the ONLY call site of inject_pcode
        }
```

and `generate_ops_with_jumptables` (`flow.rs:1765`) then discovers the table targets and drains
`addrlist` again **without** re-running the injection:

```rust
        self.generate_ops()?;
        while !self.tablelist.is_empty() {
            let new_tables = self.recover_jump_tables(run_pipeline)?;
            ...
                while !self.addrlist.is_empty() {
                    self.fallthru()?;         // pushes NEW CALLOTHERs onto injectlist
                }
            // no `if self.has_inject() { self.inject_pcode()?; }` here
        }
```

`inject_pcode` ends with `self.injectlist.clear()` (`flow.rs:2472`), so everything queued after
the first drain is silently discarded, and the raw `CALLOTHER` survives to P9. The p-code dump
confirms the survivors, all at table-reachable addresses:

```
0x08041dee:158:  setISAMode(#0x1:1)      (bl mpu6050AccDetect, block only reachable via the table)
0x08041e30:135:  setISAMode(#0x1:1)      (tbb [pc,r3])
...  10 total
```

Ghidra wins here purely because it applies the same declared fixup to table-discovered flow too.

Secondary (not claimed, worth a look in the same PR): the leftover `CALLOTHER` sits *inside*
the inner `tbb` dispatch block, and that is also the switch whose `default:` kuna failed to fold
in — it emits a hoisted `if (4 <= x - 6) v1 = '\b'; else switch (...) {4 cases, no default}`
where ghidra emits one 5-case switch with `default:`. Causality is unproven; the guard fold-in
may be an independent P2 jump-table issue.

## Proposed fix

**Mechanism.** In `decompiler/crates/kuna-decomp/src/p2_lift/flow.rs`,
`generate_ops_with_jumptables`, drain the inject list after each table-driven fall-thru round —
the same `if self.has_inject() { self.inject_pcode()?; }` that `generate_ops` already runs — so
a registered `<callotherfixup>` is applied uniformly regardless of how the block was
discovered. One call site; `inject_pcode` is already re-entrant (it re-reads `injectlist.len()`
and clears at the end).

**Classification.** Strict bug fix, not a judgment call: the compiler spec declares the userop
must be replaced by a NOP, kuna already honours that everywhere else in the same function, and
the current behaviour is internally inconsistent. Per `docs/agents.md` this needs no option
flag. It does need a `tests/stages/` case (an ARM Thumb function whose jump-table targets
contain calls) and a spec-prose update in the P2 chapter.

**Risk / blast radius.** Only architectures whose cspec carries a `<callotherfixup>` can change:
ARM, MIPS, Dalvik, JVM, tricore, Xtensa (x86 has none), and only in functions with a recovered
jump table whose table-reachable code contains an injected userop. Corpus exposure is the 14
ARM/MIPS datatest+stage files (`tests/datatests/switchreturn.xml` is the one that combines a
switch with those targets). Expect 0/675 movement, but it must be measured. Removing a
side-effecting `CALLOTHER` can unblock further simplification, so a small amount of downstream
output change on ARM/MIPS switch functions is possible and should be diffed before/after on
fixtures.
