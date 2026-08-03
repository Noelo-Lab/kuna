---
case_id: O0-crazyflie-cf2-brightnessEffect
pool: angr
group_id: crazyflie::brightnessEffect
status: needs-proposal
tier: M
margin: 68
fresh_verdict: reproduces — the source is 18 `?:` ternaries in 4 families and kuna renders 5 of them (aggressive) / 2 (reliable) as ternaries and the rest as if/else. One family is the high-breadth `itecondlist` bug filed from the mavlink case; the other three need CHAINED (`a ? x : (b ? y : z)`) and IDENTITY-ARM (`v = c ? A : v`) ternary recovery, and the identity arm has no arm op to render, so it does not fit one module.
option_closing: null
feature_slug: iteternarychain
scope: proposal
confidence: medium
---

## Verify-first

```
$ ~/.virtualenvs/decbench/bin/python -m scripts.decbench.rescore \
      --case O0-crazyflie-cf2-brightnessEffect --siblings
 "ged_recorded": 68.0, "ged_before": 68.0, "ged_after": 60.0, "ged_delta": -8.0,
 "ged_perfect_after": false, "ged_kuna_commit": "9623dc27"
 siblings: O0-crazyflie-firmware-brightnessEffect  before 60.0  after 60.0  (METRIC-DRIFT)
```

`before` reproduces `recorded` exactly on the mined case (the stored `kuna_cf2.c` is
run-era, `mtime Jul 15`), so there is no metric drift here and the -8 is a real code/mode
move. `cf2.elf` is 491,004 B — just **under** the 500 KiB threshold — so a no-flag run
today is `aggressive`, which is *not* what the benchmark measured:

```
$ kuna decompile-all .../O0/crazyflie/stripped/cf2.elf --json --addr 0x800d00d <mode>
--mode reliable  : loc 82  ternaries 2  ifs 17    <- the benchmark's option surface
--mode aggressive: loc 76  ternaries 5  ifs 14    <- today's product default (iteexpr on)
```

Neither closes it. Sweep of every default-off catalog option (`iteexpr`, `condfold on`,
`condfold wide`) on this function: 5 ternaries in all cases.

## Source (`ledring12.i`) — 18 ternaries, no `if/else` except the outer guard

```c
static void brightnessEffect(uint8_t buffer[][3], _Bool reset)
{
  static int gyroYid, gyroZid, gyroXid =- 1;
  static uint8_t brightness = 0;
  if (gyroXid < 0) { gyroXid = logGetVarId("gyro","x"); ... }
  else {
    int i;
    int gyroX = (int)logGetFloat(gyroXid); ... gyroZ ...
    gyroX = (gyroX>512) ? 512:(gyroX<-512) ? -512:gyroX;      /* x3  - chained + identity */
    gyroX = ((gyroX>=0)?1:-1) * gyroX / 2;                    /* x3  - plain diamond      */
    gyroX = ((gyroX<5) ? 0:gyroX);                            /* x3  - identity arm       */
    for (i=0; i < 12; i++) {
      buffer[i][0] = (uint8_t)(((gyroZ>255)?255:(gyroZ<0)?0:gyroZ));   /* x3 - chained  */
      ...
    }
    brightness++;
  }
}
```

angr (stored, GED 0) reproduces every one of them, including the identity arms:

```c
    v3 = (0x200 < v3 ? 0x200 : (v3 < 0xfffffe00 ? 0xfffffe00 : v3));
    v8 = (v3 < 0 ? 4294967295 : 1);
    v3 = v3 * v8 + (v3 * v8 >> 31) >> 1;
    if (v3 <= 4) v3 = 0;
    ...
    *((char *)(idx * 3 + a0)) = (0xff < v1 ? 255 : (v1 < 0 ? 0 : (char)v1));
```

## kuna (fresh, `aggressive`) — 5 of 18

```c
    if (0x201 <= v4) // branch-flip           <- (A) chained: else-arm is a 2-component if
      v4 = 0x200;
    else if (v4 < dat_800d220)
      v4 = dat_800d220;
    ... x3 ...
    v3 = (0 <= v4) ? 1 : -1; // branch-flip   <- (B) plain diamond, re-rolled
    v4 = (v4 * v3) / 2;
    if (0 <= v5) // branch-flip               <- (B) IDENTICAL diamond, NOT re-rolled
      v3 = 1;
    else {
      v3 = -1;
    }
    v5 = (v5 * v3) / 2;
    v3 = (0 <= v6) ? 1 : -1; // branch-flip   <- (B) re-rolled
    v6 = (v6 * v3) / 2;
    if (v4 <= 4)                              <- (C) identity arm: no else, v4 unchanged
      v4 = 0;
    ... x3 ...
    for (v7 = 0; v7 <= 0xb; v7 = v7 + 1) {
      if (0x100 <= v6) // branch-flip         <- (D) chained: else-arm is itself a ternary
        v2 = 0xff;
      else {
        v2 = (0 <= v6) ? (char)v6 : 0; // branch-flip
      }
      *(char *)(v7 * 3 + a0) = v2;
      ... x3 ...
```

## Analysis — four distinct sub-symptoms, only one is small

Owning phase for all four: **P8**,
`decompiler/crates/kuna-decomp/src/p8_structure/kuna_iteregion.rs`
(`match_ite_assignment` / `single_assign_arm` / `cond_cbranch`).

**(B) plain diamond, 2 of 3 re-rolled — the `itecondlist` bug.** Three byte-identical
diamonds; the middle one declines. This is the same defect filed in detail from
`O0-betaflight-betaflight_STM32F405-mavlinkSendRCChannelsAndRSSI` (the matcher's condition
component must be a single leaf, but the collapse structurer folds the previous `if` and the
next condition block into one multi-component `BlockList`). High breadth (952 declined
diamonds in 648 functions over five binaries), one-module fix, tracked there.

**(D) chained ternary, else-arm is itself a matched diamond.** `single_assign_arm` calls
`leaf_bblock`, which returns `None` for a `BlockIf`, so an arm that is itself an
`IteAssignMatch` writing the same destination declines. A recursive extension of
`single_assign_arm` (accept an arm that `match_ite_assignment` matches with a
storage-equal destination, render it as the nested `?:`) is contained to the same module.
Breadth is small, though — scanning whole-binary output for
`if (c) V = A; else { V = <ternary>; }`: **4 occurrences in `cf2.elf` (2790 fns), 7 in
`ls` (617 fns)**. It is 3 of this function's 4 families but not a campaign-scale lever on
its own.

**(A) + (C) identity arm.** `(g<5) ? 0 : g` and the tail of `(g>512)?512:(g<-512)?-512:g`
compile to a **2-component** `BlockIf` — one arm assigns, the fall-through leaves the
variable at its incoming value, and the join `MULTIEQUAL` takes the pre-diamond definition.
There is no second arm op, so `single_assign_arm` has nothing to hand the printer for the
`:` side: `emit_block_if_ite` renders both arms via `op_push_ir` on an arm op. Recovering
`v = c ? A : v` therefore needs a **printer** change (render the join `MULTIEQUAL`'s other
input as the false expression) on top of a matcher change — i.e. two modules and a new
value-safety argument (the false expression must be the *pre-diamond* SSA definition, not
the post-join HighVariable, or the ternary reads its own output).

That last point is why this is `needs-proposal` and not `feature-candidate`: (A)+(C) are 9
of this function's 18 ternaries and they do not fit one module.

## Metric-artifact check

Not an artifact. Source CFG is 30 nodes / 42 edges (real, not degenerate), not
approximated, and Joern parses every pane; `before` reproduces the recorded 68 exactly.
Today's 60 is real distance. ida scores 56 and ghidra 71 here, so kuna is already ahead of
its own ancestor — the remaining distance is genuinely angr's aggressive `?:` recovery.
(The `firmware.elf` sibling's stored artifact *does* drift, 68 recorded vs 60 recomputed —
same binary content, so that one is the 2026-07-28 decbench GED overhaul `e6e6b0f`.)

## Siblings

`O0-crazyflie-firmware-brightnessEffect` is the same function in a second binary (same
address `0x800d00d`, same margin 68) and shows the identical pane.

## Proposed fix

Stage it, do not do it in one PR:

1. **`itecondlist`** (small, filed under the mavlink case) — picks up (B) here and 952
   diamonds corpus-wide.
2. **`iteternarychain`** (small-to-medium, one module) — recursive `single_assign_arm`, for
   (D). Low breadth; ship it only bundled with 3, or as a readability follow-up.
3. **`iteidentityarm`** (`[PROPOSAL]`) — 2-component diamond whose fall-through leaves the
   destination unchanged, rendered `v = c ? A : v`. Needs the printer to source the false
   expression from the join `MULTIEQUAL`'s non-arm input; the value-safety argument (that
   input must be the pre-diamond definition) is the whole review. This is also the family
   angr recovers most aggressively, so it is where the remaining GED lives.
