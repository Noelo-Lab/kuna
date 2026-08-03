---
case_id: O0-betaflight-betaflight_STM32F405-mavlinkSendRCChannelsAndRSSI
pool: angr
group_id: betaflight::mavlinkSendRCChannelsAndRSSI
status: feature-candidate
tier: M
margin: 66
fresh_verdict: half the margin is already paid — today's build recovers all 13 call arguments and the 8 assignment diamonds the run-era output had deleted (GED 66 -> 24, measured). The whole residual 24 is that the source's EIGHT identical `?:` ternaries render as `if/else`: kuna's own `iteregion`/`iteexpr` matcher re-rolls only 2 of the 8, and which 2 depends on the *neighbouring* structure, not on the diamond.
option_closing: null
feature_slug: itecondlist
scope: small
confidence: high
---

## Verify-first

```
$ ~/.virtualenvs/decbench/bin/python -m scripts.decbench.rescore \
      --case O0-betaflight-betaflight_STM32F405-mavlinkSendRCChannelsAndRSSI --siblings
 "ged_recorded": 66.0, "ged_before": 66.0, "ged_after": 24.0, "ged_delta": -42.0,
 "ged_kuna_commit": "9623dc27"
 siblings: O2-noinline-...-mavlinkSendRCChannelsAndRSSI  before 24.0  after 24.0
```

`before` reproduces `recorded` exactly, so there is **no metric drift on this case** — the
42-point move is a kuna code change (the round-1 `callsitestackargs` fix; the run-era pane
below had every stack-passed argument deleted).

`betaflight_STM32F405.elf` is 533,396 B, i.e. just over the 500 KiB `auto` threshold, so a
no-flag run today is **`reliable`** — the same option surface the benchmark measured.
`--mode aggressive` (which carries `iteexpr on`) recovers 2 of the 8 ternaries; nothing
else in the catalog moves it.

## Side-by-side

Source (`mavlink.i`) — eight identical ternaries, one per RC channel:

```c
void mavlinkSendRCChannelsAndRSSI(void)
{
    uint16_t msgLength;
    mavlink_msg_rc_channels_raw_pack(0, 200, &mavMsg, millis(), 0,
        (rxRuntimeState.channelCount >= 1) ? rcData[0] : 0,
        (rxRuntimeState.channelCount >= 2) ? rcData[1] : 0,
        ... (8 in total) ...
        scaleRange(getRssi(), 0, 1023, 0, 254));
    msgLength = mavlink_msg_to_send_buffer(mavBuffer, &mavMsg);
    mavlinkSerialWrite(mavBuffer, msgLength);
}
```

angr (stored, GED 0) — all eight as ternaries, inline in the call:

```c
    sub_805c3c7(0, 200, &g_20013b98, v1, 0, v2,
        (g_2000fbd2 <= 1 ? 0 : g_2000fc3c & 0xffff),
        (g_2000fbd2 <= 2 ? 0 : g_2000fc40 & 0xffff), ... , v3);
```

kuna (stored, run-time, GED 66) — the arguments and every diamond are **gone**:

```c
void mavlinkSendRCChannelsAndRSSI(void)
{
  unsigned int v1; // r0
  v1 = sub_8023ef8();
  sub_8016f30(sub_803e1c0(),0,0x3ff);
  sub_805c3c6(0,200,dat_805c9c4,v1);
  sub_805c650(dat_805c9c0,sub_805c3a0(dat_805c9c0,dat_805c9c4));
  return;
}
```

kuna (fresh, `--mode aggressive`, GED 24) — arguments recovered; 2 of 8 diamonds re-rolled:

```c
  v2 = sub_8023ef8();
  v1 = *(uint1 *)(dat_805c9b8 + 2);
  if (v1) // branch-flip
    v11 = (uint4)(0.0 < *dat_805c9bc) * (int4)*dat_805c9bc & 0xffff;
  else {
    v11 = 0;
  }
  if (2 <= v1) // branch-flip
    v4 = (uint4)(0.0 < dat_805c9bc[1]) * (int4)dat_805c9bc[1] & 0xffff;
  else {
    v4 = 0;
  }
  v5 = (3 <= v1) ? (uint4)(0.0 < dat_805c9bc[2]) * (int4)dat_805c9bc[2] & 0xffff : 0;   <- re-rolled
  if (4 <= v1) ... else { v6 = 0; }
  v7 = (5 <= v1) ? ... : 0;                                                            <- re-rolled
  if (6 <= v1) ... else { v8 = 0; }
  if (7 <= v1) ... else { v9 = 0; }
  if (8 <= v1) ... else { v10 = 0; }
  v3 = sub_803e1c0();
  sub_805c3c6(0,200,dat_805c9c4,v2,0,v11,v4,v5,v6,v7,v8,v9,v10,sub_8016f30(v3,0,0x3ff,0,0xfe));
```

## The symptom, isolated

Eight structurally identical diamonds; two are re-rolled and six are not. That is an
internal inconsistency, not a judgement call. Minimal x86-64 repro (`gcc -O0 -c`):

```c
extern int sinkn(int,int,int,int,int,int,int,int);
extern int cnt;
int repron(void){ return sinkn((cnt>=1)?7:0, (cnt>=2)?8:0, ... (cnt>=8)?14:0); }
```

```
$ kuna decompile rep8.o repron              # default-ON iteregion, plain COPY arms
void repron(void) // ternary x4
  v6 = (8 <= dat_402000) ? 0xe : 0;         <- re-rolled
  if (7 <= dat_402000) v5 = 0xd; else { v5 = 0; }
  v7 = (6 <= dat_402000) ? 0xc : 0;         <- re-rolled
  if (5 <= dat_402000) v8 = 0xb; else { v8 = 0; }
  ...                                        (exactly every other one)
```

Sweeping N = 1..8 diamonds gives `ternaries = ceil(N/2)` every time:

```
N=1 ternaries=1 ifs=0     N=5 ternaries=3 ifs=2
N=2 ternaries=1 ifs=1     N=6 ternaries=3 ifs=3
N=3 ternaries=2 ifs=1     N=7 ternaries=4 ifs=3
N=4 ternaries=2 ifs=2     N=8 ternaries=4 ifs=4
```

It is not the diamond. `print raw` shows all eight p-code diamonds are byte-for-byte the
same shape — condition block ending in `CBRANCH`, each arm a single `COPY` of a constant
**to the same register**, joined by a `MULTIEQUAL`:

```
Basic Block 3  0x00400021:fe: R8(...) = R8(0x00400013:13) ? R8(0x0040001b:ae)   <- MULTIEQUAL
               0x0040002a:22: u0x00025f00:1 = #0x7:4 <= r0x00402000:4(i)
               0x0040002a:23: goto Block_5 if (...) else Block_4
Basic Block 4  0x00400033:ad: RDI(...) = #0x0            [ goto Block_6 ]
Basic Block 5  0x0040002c:24: RDI(...) = #0xd             goto Block_6
Basic Block 6  0x00400038:f9: RDI(...) = RDI(...) ? RDI(...)
```

(the failing `RDI` diamond and the matching `R8` one differ in nothing), and the P7 region
tree is a clean right-leaning chain of 3-node regions, one per diamond. Nor is it any other
pass — ablating `regionlooprefine`, `gotoreduce`, `ifelseflatten`, `crossjumprevert`,
`taildup`, `dedupitetail`, `iteboolean`, `returndup`, `earlyreturn`, `branchflip`,
`loopbreak_recovery`, `condfold on|wide` all leave 4/8 (`regionstructure off` gives 5/8 —
a *different* structurer, a different split, still not 8).

And it is positional, not content-based:

```
# diamonds separated by opaque calls -> still every other one (2 of 4)
# a plain `if (cnt==5) barrier();` placed BEFORE the chain -> the parity flips:
#   the first diamond now fails and the remaining three all re-roll (3 of 4)
```

## Analysis — root cause (P8)

Everything the matcher tests *about the diamond* is identical across all eight, so the
declining predicate has to be the one that depends on the diamond's **neighbours**: the
condition component.

`p8_structure/kuna_iteregion.rs::match_ite_assignment` requires the `BlockIf`'s component 0
to be a single leaf:

```rust
let cbranch = cond_cbranch(data, cond_block)?;   // -> leaf_bblock(data, id)?
...
pub(crate) fn leaf_bblock(data: &Funcdata, id: BlockId) -> Option<BlockId> {
    match blk.get_type() {
        BlockType::Basic => Some(id),
        BlockType::Copy  => { ...; blk.get_copy() }
        BlockType::Ls | BlockType::Graph => {
            if blk.get_size() != 1 { return None; }     // <-- declines a real list
            leaf_bblock(data, blk.get_block(0))
        }
        _ => None,
    }
}
```

In a chain of diamonds the collapse structurer folds the preceding structured `if` and the
next diamond's condition basic block into **one multi-component `BlockList`** before
`structureIf` runs, so the next `BlockIf`'s component 0 is a 2-component `Ls` and
`leaf_bblock` returns `None`. That is the only predicate in the matcher whose value can
change when a neighbour changes, and it is exactly what makes the miss alternate and makes
the parity flip when an unrelated `if` is prepended.

The **printer already handles this shape** — both `emit_block_if` and the ternary path
`emit_block_if_ite` emit `emit_block(cond_block)` once under `NO_BRANCH` (leading
statements) and once under `ONLY_BRANCH` (the condition), and `emit_block_ls` under
`ONLY_BRANCH` emits *only the last component*:

```rust
fn emit_block_ls(...) {
    if self.context.is_set(modifiers::ONLY_BRANCH) {
        if let Some(&last) = list.last() { self.emit_block(fd, arch, last); }
        return;
    }
```

So the render is already correct for a list condition; only the *matcher* refuses. The
sibling pass `kuna_iteboolean.rs` already generalises past `leaf_bblock` in exactly this
spirit (`cond_terminal_cbranch` descends a `BlockType::Condition` tree to its terminal
`CBRANCH`) — it just never learned about `Ls`.

Owning phase: **P8** (structured AST & goto quality), module
`decompiler/crates/kuna-decomp/src/p8_structure/kuna_iteregion.rs`.

**Instrumentation gap found on the way**: kuna cannot dump its own P8 structured tree —
`print tree block`, `print C xml` and `structure blocks` are all
`Err(engine_unavailable(...))` stubs (`kuna-console/src/ifacedecomp.rs:2416, 2134, 3576`).
The list shape above is therefore established by elimination + the printer's own handling
of it, not by reading the tree. **The implementer's first step should be a throwaway
`eprintln!` of `blk.get_block(0)`'s type/size in `match_ite_assignment` to confirm `Ls(2)`
before writing the fix** — and un-stubbing `print tree block` is worth its own small PR.

## Breadth

`if (c) V = A; else { V = B; }` with the same `V` — a diamond kuna already knows how to
render as a ternary and declined — counted over whole-binary `decompile-all` on each
binary's shipped default mode (the regex only catches the braceelided shape, so these are
lower bounds):

| binary | arch / default mode | fns | missed diamonds | in fns | ternaries recovered |
|---|---|---|---|---|---|
| `betaflight_STM32F405.elf` | ARM / reliable | 5795 | **615** | 403 | 312 |
| `crazyflie cf2.elf` | ARM / aggressive | 2790 | **113** | 90 | 116 |
| `iproute2 ip` | x86-64 / reliable | 1962 | **112** | 84 | 172 |
| `coreutils ls` | x86-64 / aggressive | 617 | **81** | 45 | 77 |
| `coreutils sort` | x86-64 / aggressive | 506 | **31** | 26 | 22 |
| **total** | | 11670 | **952** | **648** | 699 |

kuna declines ~58% of the diamonds it already knows how to render, in **648 functions
across five binaries and two architectures**. The defect is on the **default-ON `iteregion`
path** (the 8-diamond repro above uses plain constant `COPY` arms with `iteexpr` off), so
the fix improves shipped default output in every mode, and it hits `iteboolean` too —
`O0` bash `shell_initialize` re-rolls 1 of 3 identical 0/1 diamonds on today's build.

## Proposed fix

`itecondlist` — in `kuna_iteregion.rs::cond_cbranch`, when the condition component is a
multi-component `Ls`/`Graph`, descend to its **last** component and take the `CBRANCH` from
there (the earlier components keep printing as ordinary statements ahead of the ternary,
which `emit_block_if_ite`'s `NO_BRANCH` pass already does). Optionally lift the same
descent into `kuna_iteboolean.rs::cond_terminal_cbranch` so all three passes agree.

- Files: `p8_structure/kuna_iteregion.rs` (one function), optionally
  `p8_structure/kuna_iteboolean.rs` (one call site). Printer unchanged.
- Risks: a `Ls` component that is not a plain statement run (a loop, a labelled goto
  target) must still decline — keep the existing per-component leaf checks on the last
  element and reject when the list's non-final components contain an unstructured target.
- Gating: it broadens the existing `iteregion` (default-ON) and `iteexpr` (default-off,
  in `aggressive`) matches, so it changes emitted C; per the standing rules it needs its
  own named option or an explicit DIV row for widening a default-ON transform, plus a
  two-pass `tests/stages/ghangr-itecondlist.xml`.
- Expected GED: this case 24 -> low single digits (angr's ternary-for-ternary pane is the
  0). Sibling `O2-noinline-...-mavlinkSendRCChannelsAndRSSI` is at 24 too.
