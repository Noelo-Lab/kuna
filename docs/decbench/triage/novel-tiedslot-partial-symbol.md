---
case_id: novel-tiedslot-partial-symbol
pool: novel
group_id: cluster (cleanflight/betaflight::applyLedFixedLayers @ 0x8030a88 / 0x8052b04, cleanflight::ftoa @ 0x800f04c, siblings sub_8030aee, sub_8030cbc, sub_8052b6e, sub_8052d50)
status: needs-proposal
tier: N
margin: 0
fresh_verdict: reproduces exactly on HEAD e38ffc31 — one 4-byte stack slot that the source writes field-by-field is emitted as SIX aliasing locals (`v12`/`v13`/`v14` all `// stack - 0x24`, `v15`/`v16` `// stack - 0x22`, `v17` `// stack - 0x21`) instead of Ghidra's single `undefined4 local_24` with `local_24._2_2_` partial writes; the proximate cause is instrumented and proved (a SKIPPED forced `Merge::mergeAddrTied` at that slot, 37 addr-tied varnodes, already a documented kuna divergence at `p6_variables/merge.rs:1640-1657`), but the FIX is not — the intersection that blocks the merge is a full-block cover overlap that upstream's `eliminateIntersect` structurally cannot snip, so "complete the port" is a direction, not a mechanism
option_closing: null
feature_slug: tiedslotunify
scope: proposal
confidence: high
---

## Verdict in one line

The census's surviving `concat`/`subpiece` thread is **one P6 defect**: when
`Merge::mergeAddrTied`'s **forced** merge of a stack slot is skipped, that slot's
sub-width accesses lose the mapped Symbol and become **separate aliasing locals**, and the
refinement `SUBPIECE`/`PIECE` that would have been hidden become **raw `SUBnn`/`CONCATnn`
statements**. The symptom, the chain and the blast radius are measured; the repair is not
identified, so this is `needs-proposal`, not `feature-candidate`.

## Reproduction (today's build, pasted)

`make binaries` at HEAD `e38ffc31`. The 346,852-byte binary is under 500 KiB, so a no-flag
run is `--mode auto` == `--mode aggressive`.

```
$ export SLEIGHHOME=/home/mahaloz/github/kuna/specs
$ ./decompiler/target/release/kuna decompile \
    ~/github/decbench/results/full_run/O2-noinline/cleanflight/stripped/cleanflight_DALRCF405.elf \
    --addr 0x8030a88

void sub_8030a88(void)
{
  uint2 v1;
  uint4 v10; // r4
  int4 v11; // r7
  unsigned int v12; // stack - 0x24      <-- ONE 4-byte slot ...
  unsigned short v13; // stack - 0x24    <-- ... six locals
  undefined3 v14; // stack - 0x24
  char v15; // stack - 0x22
  unsigned short v16; // stack - 0x22
  char v17; // stack - 0x21
  uint4 v18; // stack - 0x20
  uint4 v19; // stack - 0x1c
  ...
          if (0x5dc <= dat_200079b8) { // branch-flip
            v9 = v12;
            v3 = sub_800e454(dat_200079b8,0x5dc,2000,v12 & 0xffff,v18 & 0xffff);
            v16 = SUB42(v12,2);
            v12 = CONCAT22(v16,v3);
            v15 = SUB41(v9,2);
            v2 = sub_800e454(dat_200079b8,0x5dc,2000,v15,v18 >> 0x10 & 0xff);
            v17 = SUB41(v12,3);
            v13 = (unsigned short)v12;
            v14 = CONCAT12(v2,v13);
            v12 = CONCAT13(v17,v14);
            v2 = sub_800e454(dat_200079b8,0x5dc,2000,v17,v18 >> 0x18);
            v14 = (undefined3)v12;
            v12 = CONCAT13(v2,v14);
```

Ghidra (stored pane, same function, same binary) — **one** declaration, and every partial
access rendered as a field of it:

```c
  undefined4 local_24;
  uint local_20;
  undefined4 local_1c;
  ...
        else {
          uVar4 = FUN_0800e454(iVar8,0x5dc,2000,local_24 & 0xffff,local_20 & 0xffff);
          uVar11 = local_24;
          local_24 = CONCAT22(local_24._2_2_,uVar4);
          uVar7 = local_24;
          local_24._2_1_ = SUB41(uVar11,2);
          uVar3 = local_24._2_1_;
          local_24 = uVar7;
          uVar3 = FUN_0800e454(*DAT_08030d10,0x5dc,2000,uVar3,local_20 >> 0x10 & 0xff);
          local_24._0_3_ = CONCAT12(uVar3,(undefined2)local_24);
          uVar3 = FUN_0800e454(*piVar2,0x5dc,2000,local_24._3_1_,local_20 >> 0x18);
          local_24 = CONCAT13(uVar3,(undefined3)local_24);
```

Counted with the census scanner:

| pane | raw nonzero-offset `SUBnn` | `CONCAT` | `sym._o_n_` refs | decls for the slot |
|---|---:|---:|---:|---:|
| ghidra (stored) | **1** | **7** | **9** | **1** |
| kuna (HEAD) | 6 | 9 | 0 | **6** |

`--option dedupvardecls off` exposes the underlying multiplicity — **nine** HighVariables
are whole 4-byte covers of that one slot:

```
  unsigned int v12; // stack - 0x24
  uint4 v12; // stack - 0x24
  uint4 v12; // stack - 0x24
  uint4 v12; // stack - 0x24
  unsigned int v12; // stack - 0x24
  unsigned int v12; // stack - 0x24
  uint4 v12; // stack - 0x24
  unsigned int v12; // stack - 0x24
  unsigned int v12; // stack - 0x24
  unsigned short v13; // stack - 0x24
  undefined3 v14; // stack - 0x24
```

`decompile-all --mode reliable` (the benchmark's own option surface) emits the identical
declaration block, so this is not a mode artifact.

## Source

`~/github/decbench/results/full_run/O2-noinline/cleanflight/compiled/ledstrip.i`

```c
typedef struct hsvColor_s { uint16_t h; uint8_t s; uint8_t v; } hsvColor_t;  /* 4 bytes */

static void applyLedFixedLayers(void)
{
    for (int ledIndex = 0; ledIndex < ledCounts.count; ledIndex++) {
        ...
        hsvColor_t color = *getSC(LED_SCOLOR_BACKGROUND);      /* the stack - 0x24 slot */
        ...
                if (auxInput < centerPWM) {
                    color.h = scaleRange(auxInput, 1000, centerPWM, previousColor.h, color.h);
                    color.s = scaleRange(auxInput, 1000, centerPWM, previousColor.s, color.s);
                    color.v = scaleRange(auxInput, 1000, centerPWM, previousColor.v, color.v);
                } else { ... }
        color.h = (color.h + hOffset) % (359 + 1);
        setLedHsv(ledIndex, &color);
    }
}
```

**One** local, written at 2-byte and 1-byte granularity. The stripped binary carries no
struct type, so *some* CONCAT/SUBPIECE reassembly is honest and Ghidra emits it too — but
"one variable" is not a judgement call, it is what the source says, and Ghidra recovers it.
kuna recovers six.

## Root cause — instrumented, in four steps

All four measured with temporary env-gated `eprintln!` instrumentation on
`p6_variables/{merge.rs, varmap.rs, coreaction_cleanup.rs}` and `p9_emit/printc.rs`, run
through `decomp_dbg` directly (the `kuna decompile` wrapper pipes stderr). **The
instrumentation was reverted; the tree carries no `.rs` change from this triage, and the
restored build reproduces the pane above byte-identically (md5 `1533cc66…`).**

**1. The stack layout is CORRECT.** `ScopeLocal::restructure` creates exactly one Symbol
per slot — the same layout Ghidra has:

```
[varmap] ENTRY addr=0xffffffdc size=4 type=xunknown4     <-- the `color` slot, ONE entry
[varmap] ENTRY addr=0xffffffe0 size=4 type=uint4
[varmap] ENTRY addr=0xffffffe4 size=4 type=xunknown4
```

So this is **not** a P6 stack-layout (`RangeHint`/`MapState`) defect, and not `subright`.

**2. The forced addr-tied merge at that ONE slot is skipped.** Every other slot merges:

```
[mergeAddrTied] sub n=28 addr=0xffffffe0 size=4 -> OK
[mergeAddrTied] sub n=28 addr=0xffffffe4 size=4 -> OK
[mergeAddrTied] sub n=37 addr=0xffffffdc size=4 -> SKIPPED(intersect)
[mergeRangeMust] FAIL at vn=VarnodeId(1826v3) off=0xffffffdc sz=4 written=true input=false
                 defop=Some(CPUI_INDIRECT)
[isect] blk=0 vn=VarnodeId(570v5) off=0xffffffdc sz=4 def=None
             | vn2=VarnodeId(1826v3) off=0xffffffdc sz=4 def=Some(CPUI_INDIRECT)
```

This is the **already-documented kuna divergence**, `p6_variables/merge.rs:1640-1657`:
`merge_range_must`'s error is swallowed instead of aborting the loop, and the comment
names this exact outcome —

> "…so `mark_internal_copies` can hide no partial-preserve SUBPIECE and the function
> renders `SUB84(x,4)`/`CONCAT44(…)` reconstruction soup instead of Ghidra's inline
> `x._4_4_` member access. … The residual conflicting slot is the follow-up (match
> Ghidra's `eliminateIntersect` on the INPUT-vs-INDIRECT reused-slot intersection)."

**3. The un-unified slot then costs the partial accesses their Symbol.**
`ActionNameVars`/`linkSymbols` (`p6_variables/coreaction_cleanup.rs:2567-2613`) treats a
**narrower addr-tied** representative as a storage *conflict* when another HighVariable
sits at the entry's exact storage in a different `VariableGroup` — which is exactly what
nine un-unified whole covers produce:

```
[name] addr=0xffffffdc size=2 tied=true ... entry_size=4 narrower=true reuse=false conflict=true  entry_name=v12
[name] addr=0xffffffdc size=3 tied=true ... entry_size=4 narrower=true reuse=false conflict=true  entry_name=v12
[name] addr=0xffffffde size=1 tied=true ... entry_size=4 narrower=true reuse=false conflict=true  entry_name=v12
[name] addr=0xffffffde size=2 tied=true ... entry_size=4 narrower=true reuse=false conflict=true  entry_name=v12
[name] addr=0xffffffdf size=1 tied=true ... entry_size=4 narrower=true reuse=false conflict=true  entry_name=v12
[name] addr=0xffffffe6 size=1 tied=true ... entry_size=4 narrower=true reuse=false conflict=FALSE entry_name=v19
[name] addr=0xffffffe7 size=1 tied=true ... entry_size=4 narrower=true reuse=false conflict=FALSE entry_name=v19
```

The last two lines are the control: the **same** narrower-addr-tied shape at the slot that
*did* merge takes `conflict=false`, keeps the Symbol, and renders `v19._2_1_` /
`v19._3_1_` in the body — Ghidra's exact form. The printer path is fine; the highs at
`-0x24` simply carry no symbol at all (`[decl] name=v13 symoff=-1 symtype=None
rep=off0xffffffdc/sz2`), so `PrintC` falls to the unnamed-location arm and mints `v13`.

**4. Counterfactual, to close the chain.** Forcing `conflict = false` (temporary
`KUNA_EXP_NOCONFLICT` gate) collapses the six declarations to one and converts every
aliasing local into a partial-symbol reference:

```c
  unsigned int v12; // stack - 0x24      <-- the only decl for the slot now
  ...
            v12._2_2_ = SUB42(v12,2);
            v12 = CONCAT22(v12._2_2_,v3);
            v12._2_1_ = SUB41(v9,2);
            v2 = sub_800e454(dat_200079b8,0x5dc,2000,v12._2_1_,v13 >> 0x10 & 0xff);
            v12._3_1_ = SUB41(v12,3);
            v12._0_3_ = CONCAT12(v2,(unsigned short)v12);
            v12 = CONCAT13(v12._3_1_,(undefined3)v12);
```

Measured effect of that counterfactual alone: declarations for the slot **6 → 1**,
`v12._o_n_` references **0 → 20**, raw `SUBnn` **6 → 6**, `CONCAT` **9 → 9**. That is the
important split: **the aliasing-locals half of the symptom is downstream of naming; the
raw-operator half is downstream of the merge itself**, because
`Merge::mark_internal_copies` only hides a `PIECE`/`SUBPIECE` when
`piece_group(out) == piece_group(in)` (`p6_variables/merge.rs:2168`/`:2218`), and two
`VariableGroup`s at one slot can never satisfy that.

## Why this is `needs-proposal` and not `feature-candidate`

The proximate cause is proved, but the repair is not, and the campaign's own rule is that
a diagnosis is a hypothesis until it is instrumented:

- **The intersection is structural, not a missed guard.** `eliminate_intersect` only
  examines varnodes whose **def block** equals a block in the read's single-read cover
  (`BlockVarnode::find_front`). Instrumented: for the input varnode `570v5` the single-read
  cover is `[0, 1]` and `blocksort` holds exactly one entry with index 0 — `570v5` itself.
  The varnode it actually intersects (`1826v3`, an `INDIRECT` defined in block 27) is
  **never examined**, because the two covers overlap block 0 *in full* rather than one
  stepping over the other's def. No `continue`-guard in the ported `merge.cc:489-572` body
  is responsible; the only bails that fire anywhere in this function are
  `!addrforce` (14) and `op != indirect_effect_op` (21), none on this pair. So "finish the
  `eliminateIntersect` port" — the follow-up the merge.rs comment names — is **not**
  supported by the evidence.
- That pushes the real question upstream of P6: why kuna's IR has a **function-input**
  varnode at a slot the loop unconditionally writes before reading, whose cover spans the
  same block as the loop-back `INDIRECT`. That is P3 heritage/cover shape, and answering it
  is a separate investigation. A change there is a Band-B fixed-point change, not a module.
- The narrow alternative that *is* demonstrated — teach the `linkSymbols` conflict scan
  that a different-group high resolving to the **same SymbolEntry/name** is not a genuine
  storage conflict — fits one module (`p6_variables/coreaction_cleanup.rs`) and is measured
  above, but it buys only half the symptom and it re-opens the LOSS-234 `zeroprop` guard it
  was written for. It is a candidate *inside* the proposal, not a standalone feature.

## Breadth (corpus-wide, measured on today's build)

Instrumented `kuna decompile-all --mode reliable` over the census's 7-binary reliable set
(`ssh`, `ip`, `e2fsck`, `gzip`, `bzip2`, `betaflight_STM32F405.elf`,
`cleanflight_DALRCF405.elf`):

| binary | functions | fns with ≥1 skipped forced merge | skip events |
|---|---:|---:|---:|
| O0 gzip | 262 | 2 | 2 |
| O2ni bzip2 | 153 | 3 | 4 |
| O2ni iproute2 ip | 1,912 | 28 | 39 |
| O2ni e2fsprogs e2fsck | 1,909 | 53 | 65 |
| O2ni openssh ssh | 1,853 | 78 | 108 |
| O2ni cleanflight | 3,114 | 150 | 225 |
| O2ni betaflight | 6,387 | 345 | 526 |
| **total** | **15,590** | **659 (4.23%)** | **969** |

The skip is **not** ARM-specific (x86-64 `ssh` 4.2%, `e2fsck` 2.8%); what is ARM-heavy is
the *visible* damage, because Cortex-M firmware writes packed 4-byte structs field by
field. Two independent damage proxies on the same run:

- **Aliasing locals**: 46 functions emit two or more locals carrying the same
  `// stack ± 0xNN` storage comment. **37 of the 46 (80%)** are in the skip set.
- **Residual raw nonzero-offset `SUBnn`**: the total on this set is **85**, reproducing the
  census exactly. **42 of the 85 (49%), in 14 functions, are in the skip set** — including
  every ARM function the census's "class B addr-tied LOCAL partial-symbol write" bucket
  named except the two `sub_8069d2x` global-store cases:

```
  sub_801853c 2   sub_802ab10 1   sub_8052b04 6   sub_8052b6e 5   sub_8052d50 5
  sub_800f04c 2   sub_8030a88 6   sub_8030aee 5   sub_8030cbc 5
  e2fsck sub_71860 1   ip sub_11290 1   ip sub_78dc0 1   ssh sub_387f0 1   ssh sub_68960 1
```

So the census's arithmetic holds up: this one thread owns roughly **half** of what is left
of the `subpiece` wart, and essentially all of its ARM Cortex-M mass.

## Siblings — all reproduce

| case | pane |
|---|---|
| O0 cleanflight `applyLedFixedLayers` @ 0x80301d4 | 6 locals at the `- 0x1c` slot (`v15`..`v20`), same shape |
| O2ni betaflight `sub_8052b04` @ 0x8052b04 | 6 locals at `- 0x24` (`v12`,`v14`,`v15`,`v16`,`v17`,`v18`) |
| O2ni betaflight `sub_8052b6e` / `sub_8052d50` | **ten** locals at `stack + 0xc` alone |
| O2ni cleanflight `sub_8030aee` / `sub_8030cbc` | same, 5 raw `SUBnn` each |
| O0/O2ni cleanflight `ftoa` @ 0x800f04c | 6 locals at the `- 0x28` slot |

`ftoa` is the case where the rendering becomes **semantically wrong**, not just verbose.
kuna:

```c
  unsigned int v2; // stack - 0x28
  char v5; // stack - 0x28
  unsigned short v6; // stack - 0x28
  undefined3 v7; // stack - 0x28
  undefined3 v8; // stack - 0x27
  ...
  v2 = 0;
  ...
  v2 = (0 <= (int4)v4) ? CONCAT31(v8,0x20) : CONCAT31(v8,0x2d); // branch-flip
```

`v8` is **read and never assigned**. Ghidra emits `local_28 = CONCAT31(local_28._1_3_,0x20)`
— a read of the upper three bytes of a slot the same function set to `0` two lines earlier.
Splitting the slot into independent C objects turns a defined partial read into a read of an
uninitialized local, so the exported `.c` is not merely unreadable, it is wrong.

## Option / mode sweep (all negative)

`kuna decompile … --addr 0x8030a88 --option <X>`, diffed against the no-flag pane:

| option | diff lines | decls at `- 0x24` | raw `SUBnn` |
|---|---:|---:|---:|
| `condfold on` / `condfold wide` | 0 | 3 | 6 |
| `cortexmvectors on`, `ptrentry on`, `paramcopyhoist on`, `dwarf_lines on` | 0 | 3 | 6 |
| `stackalias off`, `iteexpr off`, `subright off` | 0 | 3 | 6 |
| `realtypes off` | 32 | 3 | 6 |
| `dedupvardecls off` | 9 | **11** (worse) | 6 |

`--mode auto`, `--mode aggressive` and `decompile-all --mode reliable` all show the same
declaration block. `option_closing: null`.

## Metric-artifact check — clean

Source CFG **26 nodes / 40 edges** (exact GED, not the >60-node approximation, not
degenerate). kuna GED **25**, tied best with ghidra 25 (angr 42, ida 41, binja 32). Joern
parsed every pane. This is a NOVEL-pool readability/correctness defect with **GED value 0** —
no basic block moves. Rank it by correctness, not by the metric.

## Owning phase

**P6 — Variable & Storage Model** (`p6_variables/`), per `docs/phases.md`: HighVariables,
merge, stack layout. Every step of the chain is in that folder
(`merge.rs::merge_addr_tied` → `coreaction_cleanup.rs::linkSymbols` →
`merge.rs::mark_internal_copies`); P9 only renders the result. The *origin* of the blocking
cover intersection is plausibly P3 (heritage/`INDIRECT` placement, the spurious slot input),
which is the Band-B feedback edge the proposal has to settle.

## What a proposal has to answer

1. Why kuna materializes a function-**input** varnode at a stack slot the loop always
   writes before reading, and whether that input is what makes the entry-block cover
   overlap the loop-carried `INDIRECT` (compare against `- 0x20` / `- 0x1c`, which have
   inputs too and merge fine).
2. Whether `unify_address`/`eliminate_intersect` can be made to resolve a **full-block**
   cover overlap at all, or whether upstream simply never produces one here.
3. Whether the narrow `linkSymbols` conflict discriminator ("same SymbolEntry ⇒ not a
   conflict") is safe against LOSS-234 `zeroprop`, and whether shipping only that half —
   one declaration per slot, partial-symbol writes, but the raw operators still printed —
   is worth landing on its own.
4. The blast radius: 659 functions on the 7-binary set would change merge behaviour. Per
   the standing rule, a whole-corpus before/after diff of **every changed function** is the
   gate, not the witness; and the datatest corpus is the parity gate (`ActionMergeCopy` /
   `mergeAddrTied` sit under every fixture).

## Secondary observations (separate cases, recorded so they are not lost)

1. **`v18` / `v19` are read but never assigned in `applyLedFixedLayers`.** Ghidra emits
   `local_20 = *(uint *)(iVar8 + (uVar6 + 0x20) * 4);` and `local_1c = …`; kuna emits
   `v9 = *(uint4 *)(v7 + (v4 + 0x20) * 4);` for both — the store lands on a register high
   and the stack locals are then read undefined. The `- 0x20` / `- 0x1c` slots **merge
   cleanly** (`n=28 OK`), so this is *not* the mechanism above. It refines secondary
   observation #1 of `O2-noinline-betaflight-betaflight_STM32F405-applyLedFixedLayers.md`
   (filed there as a dropped call argument; the LHS is wrong, which is a stronger claim).
2. **`kuna decompile --addr` collapses on ≥500 KiB ARM binaries.** On the 537,492-byte
   `betaflight_STM32F405.elf` (`--mode auto` ⇒ `reliable`),
   `kuna decompile … --addr 0x8052b04` returns a 9-line stub reading `&Stack00000000`,
   while `decompile-all --mode reliable` on the same binary/address returns the full
   145-line body. The same happens on cleanflight under an explicit `--mode reliable`
   (4 lines vs 139). Per-function `--addr` decompilation is losing the discovery context
   that whole-binary loading provides; any triage that quotes a single-address reliable-mode
   pane on an embedded ARM target is quoting an artifact.
