---
case_id: novel-concat-subpiece-soup
pool: novel
group_id: cluster (betaflight/cleanflight::applyLedFixedLayers, cleanflight::ftoa, betaflight/cleanflight::hsvToRgb24, e2fsprogs::parse_int_node, e2fsprogs::ask_yn, iproute2::netns_add, openssh-portable::mux_master_process_close_fwd)
status: feature-candidate
tier: N
margin: 0
fresh_verdict: the cluster splits in two — CONCAT is INHERITED (kuna 4198 vs ghidra 3700 over 76046 matched functions, and kuna is *better* on the two rows that named it), while SUBPIECE-with-nonzero-offset is a kuna-specific 329x regression (kuna 5919 in 2394 functions vs ghidra 18 in 13) caused by `RuleSubRight::apply_op` being a stub that returns 0 before the `sub(V,c) => sub(V>>c*8,0)` rewrite
option_closing: null
feature_slug: subright
scope: small
confidence: high
---

## Verdict in one line

`CONCAT` soup is **not a kuna defect** — Ghidra prints the same or more of it, and the
source really is byte manipulation. `SUBPIECE` soup **is** a kuna defect, and it is one
stubbed rule: kuna emits `SUB81(a1,7)` on 2,394 corpus functions where upstream Ghidra
emits `(char)(a1 >> 0x38)` on 13.

## The witness (fresh, today's build)

`openssh-portable/ssh` `sshbuf_put_u64` @ `0x4cd60`, stripped binary, no flags and
`--mode reliable` identical:

```
$ export SLEIGHHOME=/home/mahaloz/github/kuna/specs
$ ./decompiler/target/release/kuna decompile \
    ~/github/decbench/results/full_run/O2-noinline/openssh-portable/stripped/ssh \
    --addr 0x4cd60 --mode reliable

unsigned long sub_4cd60(unsigned long a0,unsigned long a1) // early-return
{
  unsigned long v1; // rax
  char *v2; // stack - 0x18
  int8 v3; // fs_offset
  unsigned long v4; // stack - 0x10

  v4 = *(unsigned long *)(v3 + 0x28);
  v1 = sub_3ea00(a0,8,&v2);
  if ((int4)v1 <= -1)
    return v1;
  *v2 = SUB81(a1,7);
  v2[1] = SUB81(a1,6);
  v2[2] = SUB81(a1,5);
  v2[3] = SUB81(a1,4);
  v2[4] = SUB81(a1,3);
  v2[5] = SUB81(a1,2);
  v2[6] = SUB81(a1,1);
  v2[7] = (char)a1;          <-- the offset-0 one DOES render as a cast
  return 0;
}
```

Ghidra (stored pane, same function, same binary):

```c
    *local_18 = (char)((ulong)param_2 >> 0x38);
    local_18[1] = (char)((ulong)param_2 >> 0x30);
    local_18[2] = (char)((ulong)param_2 >> 0x28);
    local_18[3] = (char)((ulong)param_2 >> 0x20);
    local_18[4] = (char)((ulong)param_2 >> 0x18);
    local_18[5] = (char)((ulong)param_2 >> 0x10);
    local_18[6] = (char)((ulong)param_2 >> 8);
    local_18[7] = (char)param_2;
```

IDA: `HIBYTE(a2)` / `BYTE6(a2)` / … (its own macro family).

The last line of kuna's own output is the proof: the *offset-0* SUBPIECE already renders
as `(char)a1`. Nothing is wrong with the printer. The only thing missing is the
normalization that turns a nonzero-offset SUBPIECE into `shift + offset-0 SUBPIECE`.

### The source (the thing "correct" means)

`~/github/decbench/results/full_run/O2-noinline/openssh-portable/compiled/sshbuf-getput-basic.i`:

```c
sshbuf_put_u64(struct sshbuf *buf, u_int64_t val)
{
 ...
 do { const u_int64_t __v = (val);
      ((u_char *)(p))[0] = (__v >> 56) & 0xff;
      ((u_char *)(p))[1] = (__v >> 48) & 0xff;
      ... } while (0);
```

The source is a **shift**. Ghidra's `(char)(param_2 >> 0x38)` is literally the source
line. kuna's `SUB81(a1,7)` is a p-code operator standing in for ordinary 64-bit
arithmetic. This answers the track's question 3 for the SUBPIECE half: **not honest
bitfield code — ordinary shift/mask arithmetic kuna failed to reassemble.**

Same answer for `hsvToRgb24`
(`.../O0/cleanflight/compiled/colorconversion.i`), which is plain 32-bit integer math:

```c
    base = ((255 - sat) * val) >> 8;
    switch (hue / 60) {
        case 0: r.rgb.g = (((val - base) * hue) / 60) + base;
```

kuna (fresh) vs ghidra (stored) on the same two expressions:

| | kuna today | ghidra |
|---|---|---|
| `((255-sat)*val) >> 8` | `v2 = SUB41(v6,1);` | `bVar2 = (byte)((uint)iVar7 >> 8);` |
| `hue / 60` | `SUB84((uint8)dat_80164e8 * (uint8)v3,4) >> 5` | `(uint)((ulonglong)DAT_080164e8 * (ulonglong)uVar4 >> 0x25)` |

`0x25 == 37 == 32 + 5` — that is upstream `RuleSubRight` lumping the lone descendant
right-shift into the synthesized shift. Neither decompiler recovers `/ 60` (a separate
`RuleDivOpt`/ARM-UMULL gap); the delta kuna owns is only the SUBPIECE lowering.

## Root cause (instrumented, not read)

IR dump for the witness, after full decompilation
(`decomp_dbg` → `load function sshbuf_put_u64` → `decompile` → `print raw`):

```
0x0004cd98:2ce:	DL(0x0004cd98:2ce)  = SUB81(RSI(i),#0x7:4)
0x0004cda5:2cf:	DL(0x0004cda5:2cf)  = SUB81(RSI(i),#0x6:4)
0x0004cdb3:2d0:	DL(0x0004cdb3:2d0)  = SUB81(RSI(i),#0x5:4)
...
0x0004cdeb:2df:	BL(0x0004cdeb:2df)  = SUB81(RSI(i),#0x0:4)
```

The SUBPIECEs survive to the final IR with their nonzero constant offsets, and their
input `RSI` is a distinct register from the outputs `DL`/`BL` (so upstream's
`outvn->overlap(*a) == c` bail cannot fire).

`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_6.rs:912-947` —
`RuleSubRight::apply_op` — is a **stub**:

```rust
        let a = in_vn(data, op, 0);
        let in0_piece = data.vn_type_read_facing(a, op).is_piece_structured();
        if in0_piece {
            data.op_mark_special_print(op); // Print this as a field extraction
            return 0;
        }
        // Remainder transcribed for the next wave; unreachable at this merge base:
        //   PcodeOp *lone = outvn->loneDescend();  ... lump a lone right-shift ...
        // W6
        0
```

Only the special-print half of the rule was ported. Upstream
(`ruleaction.cc:7271`, `GHIDRA_REV=cef869af`) continues past that point with ~45 lines:
`c = in(1)->getOffset()`; bail if 0; bail if `outvn`/`a` are both addr-tied and
`overlap == c`; lump a lone `INT_RIGHT`/`INT_SRIGHT` descendant with a constant shift
into `d` when `outvn->getSize() + c == a->getSize()`; synthesize the shift op before the
SUBPIECE; rewrite the SUBPIECE's offset input to 0.

The rule is already **registered and scheduled correctly** —
`infra/universalaction.rs:552`, `rrow!("subright", "cleanup", ...)` in the cleanup pool,
which sits after `fullloop` and before `ActionAssignHigh`/`ActionMergeRequired`, exactly
where upstream puts it. So the new INT_RIGHT and its unique get Highs assigned normally.

## Breadth (corpus-wide)

Matched per-function comparison, kuna vs ghidra, over **788 binaries / 76,046 functions
that both tools recovered** in `~/github/decbench/results/full_run`
(scanner is nesting-aware; args parsed with a balanced scan, so `CONCAT71(SUB87(x,1),y)`
counts as one of each):

| | SUBPIECE, offset != 0 | CONCAT |
|---|---|---|
| **kuna** | **5,919 in 2,394 fns (3.1%)** | 4,198 |
| **ghidra** | **18 in 13 fns (0.017%)** | 3,700 |
| **ida** | 0 | 0 (uses BYTEn/LOWORD macros) |

Ratio on SUBPIECE: **329x**. On CONCAT: **1.13x**.

Stored-pane sweep validated against fresh `kuna decompile-all` on four binaries — the
metric is stable across the DIV-34..39 print wave, so the corpus number is trustworthy:

| binary (matched fns) | kuna fresh | kuna stored | ghidra | ida |
|---|---|---|---|---|
| O0/cleanflight ARM (1506) | 203 SUB / 63 CONCAT | 196 / 62 | **2** / 72 | 0 / 0 |
| O2ni/iproute2 ip (615) | 25 / 122 | 25 / 122 | **0** / 102 | 0 / 0 |
| O2ni/e2fsprogs e2fsck (367) | 12 / 35 | 12 / 35 | **0** / 33 | 0 / 0 |
| O2ni/openssh ssh (1250) | 60 / 41 | 60 / 41 | **0** / 19 | 0 / 0 |

Top affected binaries (kuna SUB / ghidra SUB): betaflight O2ni 236/2, cleanflight O2ni
234/2, betaflight O0 230/2, crazyflie O0 154/0, openssh ssh-keyscan O0 115/0.
Shape distribution over all kuna panes (7,839 occurrences): SUB41 2640, SUB84 2568,
SUB81 1021, SUB87 496, SUB42 472, SUB21 377, rest < 200.

**Not double-counting `rodata-phantom-store`.** Only 102 of those 7,839 (1.3%) have a
global-array-indexed first argument — the `s_822f0[1] = SUB81(s_822f0[0],1);` family in
`iproute2/ip` that round 1 already filed as `rodata-phantom-store`. Upstream's
addr-tied/`overlap == c` guard would bail on those too; upstream just never *creates*
them. So `subright` and `rodata-phantom-store` are disjoint fixes, and `subright` owns
≥98% of the symptom.

## What is DISPROVEN

1. **CONCAT is not a kuna regression.** On the two rows that named it:
   - `cleanflight::ftoa` (`concatx10`): ghidra's own pane is `local_28 = CONCAT31(...)`,
     `CONCAT11(0x30,(undefined1)local_28)`, `CONCAT12`, `CONCAT13`, `SUB41(uVar1,3)` —
     the same soup. The `.i` source is `sprintf`-style byte poking into a 4-byte char
     buffer, so some CONCAT is honest.
   - `betaflight/cleanflight::applyLedFixedLayers` (`concatx8`): ghidra emits
     `CONCAT22`/`CONCAT12`/`CONCAT13` in the same arms. Matched CONCAT counts on the
     whole cleanflight binary: **kuna 63 vs ghidra 72 — kuna is better.**
   The corpus-wide +13% CONCAT excess is a thin tail with no single mechanism (worst
   single function is +7 on `ssh` 0x1e830, `CONCAT44(dat_4,vN)` — an unresolved
   high-half global, a different bug). **Not a campaign item.**

2. **`openssh-portable::mux_master_process_close_fwd` is already clean.** novel.md lists
   `concatx1,subpiecex1`; today's build emits **zero** of either in that function. That
   row's wart component is stale.

3. **`e2fsprogs::parse_int_node` is inherited and identical.** kuna 1 CONCAT, ghidra 1
   CONCAT, same expression (`CONCAT71(SUB87(v7,1), v4 != 0)` vs
   `CONCAT71((int7)((ulong)extraout_RDX >> 8), iVar4 != 0)`). Only the inner truncation
   differs — the same `subright` gap. `ask_yn` is kuna 1 / ghidra 0 (one `CONCAT26`).
   Both rows are **goto-dominated** (`gotox11` / `gotox8`); their concat/subpiece
   components are noise and should not have clustered them here.

4. **PR #242 verified fixed on `applyLedFixedLayers`.** The stored pane declares the
   0x24 stack slot twice under the same name (`unsigned int v12;` **and** `uint4 v12;`).
   Today's build emits only `unsigned int v12; // stack - 0x24`. The remaining
   CONCAT/SUB in that function is the inherited kind above plus `SUB42(v12,2)` /
   `SUB41(v12,3)` where ghidra prints `local_24._2_2_` / `local_24._3_1_` — that residue
   is the P6 partial-symbol/merge thread, not `subright` (kuna does render `v20._2_1_`
   elsewhere in the same function, so the printer path exists).

## Option / mode sweep (all negative)

```
$ for m in auto reliable aggressive fast; do kuna decompile ssh --addr 0x4cd60 --mode $m | grep -c SUB81; done
7
7
7
7
$ kuna decompile ssh --addr 0x4cd60 --option condfold on | grep -c SUB81
7
```

None of the 83 catalog rows controls SUBPIECE truncation lowering (grepped
`kuna catalog --json` for subpiece/truncat/piece — hits are `returnpair`,
`switchmultipred`, `unrolledguard`, `callsitestackargs`, `iteregion`, `iteexpr`,
`noreturn_*`, none relevant). `option_closing: null`.

## Proposed fix

- **Owning phase: P3** (Definition Web — the simplification rule pools). The symptom is
  visible at P9 but the decision is a P3 rule, per `docs/phases.md`'s own caveat.
- **One module, one function**: complete the port of
  `decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_6.rs`
  `RuleSubRight::apply_op` — the ~45-line tail after the existing `is_piece_structured`
  early return. Every primitive it needs already exists in kuna:
  `Varnode::is_addr_tied` / `Varnode::overlap` (`substrate/varnode.rs:813,1345`),
  `Funcdata::new_op` / `new_unique` / `op_set_output` / `op_set_input` /
  `op_insert_before` / `new_constant` / `op_unlink` / `op_set_opcode`
  (`substrate/funcdata_op.rs`, `funcdata_varnode.rs`), and
  `data.get_arch().types().get_base(size, TYPE_UINT|TYPE_INT)` (pattern already used at
  `p3_dataflow/ruleaction_8.rs:1773`). No new registration, no schedule change — the
  rule is already in the cleanup pool at the upstream position.
- **Option**: `subright` (on = lower `SUB(V,c)` to `(T)(V >> 8c)`, off = keep the raw
  operator). This is a faithful-port completion whose *only* effect is emitted C, so per
  AGENTS.md's "when in doubt, gate it" it should ship as a settable row in `phases.toml`
  + `p0_knowledge/options.rs`, with the default-ON flip carried by the 0/675 ablation.
- **Stage test**: `tests/stages/ghdec-subright.xml` (no angr analog) — two-pass, an
  8-byte big-endian store loop; option off = `SUB81(x,7)`, default = `(char)(x >> 0x38)`.
- **Correctness value**: `SUB81` / `SUB84` are undeclared identifiers — kuna emits no
  helper header for them (grepped `kuna-cli`/`kuna-console`), so 5,919 call sites across
  2,394 functions make the exported `.c` non-compilable. Same tier as round 1's
  `spacebase-unnamed-location` and `realtypes-pointee-size`.
- **GED value: 0.** No basic-block count changes. Rank this by correctness, not metric.
- **Risks to measure**: (a) the datatest corpus — the rewrite runs on every function, so
  a 0/675 ablation is the gate and the likeliest source of churn is cast/type rendering
  on the new `INT_RIGHT` unique; (b) `ActionCopyMarker` interaction — the addr-tied
  bail must be ported verbatim or stack-piece writes that currently render as
  `sym._n_m_` would regress to shift chains; (c) the lone-descend lumping destroys the
  descendant op, so `op_unlink` ordering must match upstream; (d) speed — one extra op
  per lowered SUBPIECE, ~6k corpus-wide, measure with `scripts.pipeline.timeit`.
