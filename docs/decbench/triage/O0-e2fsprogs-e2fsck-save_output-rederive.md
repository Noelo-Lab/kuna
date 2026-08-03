---
case_id: O0-e2fsprogs-e2fsck-save_output
pool: ida
group_id: e2fsprogs::save_output
status: feature-candidate
tier: M
margin: 56
fresh_verdict: the symptom still reproduces byte-for-byte on today's build (GED 29, identical to the round-1 fresh number; auto == reliable, and aggressive differs only in an unrelated string constant) -- but the round-1 MECHANISM is confirmed dead and the real anchor is a THIRD site neither the record nor its skeptic named
option_closing: null
feature_slug: paramcopyhoist
scope: small
confidence: medium
---

This record SUPERSEDES the mechanism half of
`docs/decbench/triage/O0-e2fsprogs-e2fsck-save_output.md`. That record's symptom,
GED numbers and option sweep all hold up; its root cause does not, and neither
does the obvious repair the skeptic's refutation implies.

## Verify-first: what today's build actually does

```
$ python3 -m scripts.decbench.triage --case O0-e2fsprogs-e2fsck-save_output --also ida,ghidra
| output       | loc | gotos | labels | ifs | loops |
| ida(stored)  |  84 |     0 |      0 |  16 |     3 |
| ghidra(stored)| 103 |    1 |      1 |  17 |     4 |
| kuna(stored) |  93 |     1 |      0 |  16 |     4 |
| kuna(fresh)  |  78 |     0 |      0 |  16 |     4 |
```

```
$ ~/.virtualenvs/decbench/bin/python -m scripts.decbench.rescore \
      --case O0-e2fsprogs-e2fsck-save_output
 "ged_recorded": 56.0, "ged_before": 56.0, "ged_after": 29.0, "ged_delta": -27.0
 "source_nodes": 46, "ged_approximated": false
```

The residual 29 is unchanged from round 1, so nothing that landed since (PR #228's
two restored Cover extensions included) touched this. The two sunk copies are
still there:

```c
  v11 = *(unsigned long *)(v4 + 0x28);
  v7 = a0;
  if ((a0) && (!*a0))
    v7 = NULL;
  v6 = a1;                       // <-- NOT in the entry block  (+1 CFG node)
  if ((a1) && (!*a1))
    v6 = NULL;
  v5 = a2;                       // <-- NOT in the entry block  (+1 CFG node)
  if ((a2) && (!*a2))
    v5 = NULL;
```

**Modes.** `e2fsck` is 908 KiB, i.e. ABOVE the 500 KiB aggressive threshold, so
`--mode auto` resolves to `reliable` here and the two are byte-identical (which is
why round 1 saw no mode effect). Forcing `--mode aggressive` changes exactly one
line and does not touch the copies:

```
$ diff mode-auto.c mode-aggressive.c
38c38
<     v2 = fdopen(v10,0xaba35);
---
>     v2 = fdopen(v10,"w");
```

**Option sweep.** `condfold on`, `condfold wide`, `iteexpr on`, `regionedgeorder on`,
`stackalias on` are all byte-identical to the default run (md5 verified). Nothing in
the 83-row catalog moves the copies. `option_closing: null` stands.

**The measured value of the fix**, re-measured today with the real GED (not the
recorded one), by hoisting only those two lines into the entry block:

```
source nodes 46 edges 72
base       nodes=53 edges=77 GED=29.0
hoisted    nodes=51 edges=77 GED=17.0
```

So -12 GED, confirming round 1's textual ablation with the actual metric.

## Source

`~/github/decbench/results/full_run/O0/e2fsprogs/compiled/logfile.i` -- three guards
that write **through** the parameter, so the source's entry block holds only frame
setup, and in the binary all three spills are likewise in the entry basic block
(`4eec2/4eec6/4eeca`, entry block ends at the `je` at `4eee2`).

## Analysis -- the mechanism, re-derived by instrumenting

### 1. The filed fix is a no-op (confirmed)

`trim_op_input_prep` (`p6_variables/funcdata_merge.rs:844`) returns `pc`; `pc` is
consumed by `allocate_copy_trim` -> `copy_trim_op(in_vn, addr, trim_op)`, i.e. it
becomes the new op's **Address** only. The physical placement is the next statement:

```rust
// p6_variables/merge.rs:1610  Merge::trimOpInput
fn trim_op_input(&mut self, ctx: &mut dyn MergeContext, op: OpId, slot: int4) -> KunaResult<()> {
    let (in_vn, pc, is_multiequal) = ctx.trim_op_input_prep(op, slot);
    let copy_op = self.allocate_copy_trim(ctx, in_vn, pc, op)?;
    ...
    if is_multiequal {
        ctx.op_insert_end_pred(copy_op, op, slot);   // <-- the placement
    } else {
        ctx.op_insert_before(copy_op, op);
    }
```

Changing `pc` moves nothing. The skeptic was right.

### 2. But `op_insert_end_pred` is not the deciding site either

`print raw` on the fresh build (`decomp_dbg`, stripped `e2fsck`, `load addr 0x4eeb6`):

```
Basic Block 0 0x0004eeba-0x0004eee2
0x0004eee2:8ce:	u0x10000269(0x0004eee2:8ce) = RDI(i)
0x0004eee2:32:	goto Block_1:0x0004eee4 if (ZF != 0) else Block_3:0x0004eef7
Basic Block 3 0x0004eef7-0x0004eefc
0x0004eef7:77c:	s0xff..ffc0 = u0x10000269(0x0004eee2:8ce) ? u0x10000269(0x0004eee2:8ce) ? s0xff..ffc0(0x0004eeef:46)
0x0004eefc:8cc:	u0x10000259(0x0004eefc:8cc) = RSI(i)
Basic Block 6 0x0004ef11-0x0004ef16
0x0004ef11:758:	s0xff..ffb8 = u0x10000259(0x0004eefc:8cc) ? u0x10000259(0x0004eefc:8cc) ? s0xff..ffb8(0x0004ef09:67)
0x0004ef16:8ca:	u0x10000249(0x0004ef16:8ca) = RDX(i)
```

**Read the phi inputs.** Slot 0 AND slot 1 of each MULTIEQUAL reference the *same*
unique. `trim_op_input` cannot produce that: it is called once per failing slot and
each call allocates a **fresh** COPY with a **fresh** unique, inserted at a
**different** predecessor (slot 0 -> BB0, slot 1 -> BB1). Two slots sharing one
unique can only come from `total_replace(outVn, domVn)` + `opDestroy` inside
`Merge::buildDominantCopy`.

So the real chain is three passes deep:

1. `merge_op` (`merge.rs:1415`) fails `merge_test_required` for the input-register
   high against the addr-tied stack-slot high, and calls `trim_op_input` for slot 0
   and again for slot 1.
2. `trim_op_input` puts one COPY at the tail of BB0 and one at the tail of BB1
   (a0's phi) / BB3 and BB4 (a1's phi) / BB6 and BB7 (a2's phi).
3. `process_copy_trims` -> `process_high_dominant_copy` (`merge.rs:2042`) groups the
   two by identical root varnode and calls `build_dominant_copy` ->
   `build_dominant_copy_impl` (`substrate/funcdata.rs:2945`):

```rust
let dom_bl = self.bblocks.find_common_block_set(&block_set);
...
let stop_addr = self.block_stop_addr(dom_bl);
let new_copy = self.new_op(1, stop_addr);
...
self.op_insert_end(new_copy, dom_bl);
```

**`find_common_block_set` over the two trim sites is what decides where the printed
`vN = aM;` lands.** For `a0` that is BB0 -- the entry block -- so parameter 1 renders
correctly. For `a1` it is BB3, for `a2` BB6. kuna is internally inconsistent about the
identical construct, and the discriminator is nothing but whether the guard's join
happens to be the entry block.

Second, independent witness (different binary, different register width) --
`O0/gzip/gzip` `sub_40fe`:

```
Basic Block 2 0x00004147-0x00004193
0x00004193:635:	u0x100000a4:4(0x00004193:635) = EDI(i)
Basic Block 4 0x00004199-0x000041b5
0x00004199:616:	s0xff..ffbc:4 = u0x100000a4:4(0x00004193:635) ? u0x100000a4:4(0x00004193:635) ? EAX(0x00004326:474)
```

Same signature: two phi slots, one unique, anchored at the common dominator of the
two trim sites (BB2), which is not the entry block (BB0).

### 3. Owning phase

**P6 (variable & storage model)** -- `Merge`/HighVariable, `p6_variables/`. Not P8:
every P7/P8 structurer option is byte-identical here (re-verified today).

### 4. This is a divergence, not a port bug

Ghidra's stored pane sinks `local_48 = param_2;` to exactly the same place. The port
is faithful; hoisting is a deliberate DIV, so it needs an option.

## Breadth

`kuna decompile-all --json` over four O0 binaries, counting **top-level** (brace depth 1)
`vN = aM;` statements that appear after the first top-level control-flow statement --
i.e. copy-shadows of an unmodified incoming parameter emitted outside the entry block:

| binary | fns | fns with a sunk top-level param copy | sunk | in entry |
|---|---|---|---|---|
| O0 e2fsprogs/e2fsck | 1991 | 101 (5.1%) | 173 | 240 |
| O0 coreutils/ls | 617 | 14 (2.3%) | 21 | 54 |
| O0 gzip/gzip | 263 | 8 (3.0%) | 10 | 34 |
| O0 bzip2/bzip2 | 160 | 4 (2.5%) | 4 | 5 |
| **total** | **3031** | **127 (4.2%)** | **208** | **333** |

So **~4 in 10 parameter copy-shadows kuna emits at top level are outside the entry
block.** At O2 the same scan over `O2/{bzip2,gzip}` gives 9/318 functions (2.8%) and
only 1 of 16 in the guard shape -- the pattern is O0-dominated, as round 1 guessed.

Classifying each sunk copy by what follows it (rough textual split, 143 classified):
**41 guard-cascade shape** (`vN = aM;` immediately followed by a guard on `aM` -- the
`save_output` shape, MULTIEQUAL trim), **75 call-adjacent**, 27 other.

The call-adjacent ones are a **different anchor site**, and this matters for scoping.
`e2fsck sub_1e497` instrumented:

```
Basic Block 5 0x0001e603-0x0001e642
0x0001e634:859:	u0x1000020c(0x0001e634:859) = RDI(i)
0x0001e634:6eb:	s0xff..ff98(0x0001e634:6eb) = u0x1000020c(0x0001e634:859) [] i0x0001e634:172(free)
```

That COPY feeds an **INDIRECT**, not a MULTIEQUAL: it comes from `merge_indirect`
(`merge.rs:1506`, `allocate_copy_trim(ctx, invn0, indaddr, indop)` + `op_insert_before`),
anchored at the call. Same user-visible symptom, second producer.

## Proposed fix

**Feature `paramcopyhoist` (P6, default-off), new module
`decompiler/crates/kuna-decomp/src/p6_variables/kuna_paramcopyhoist.rs`.**

One decision, one call site:

- Export `hoist_trim_anchor(fd, high, root_vn, default_bl) -> BlockId`. It returns
  `default_bl` unless `root_vn.is_input()` (`substrate/varnode.rs:777` -- a function
  input varnode is defined at entry and dominates every block); when it is, it walks
  the `immed_dom` chain from `default_bl` up toward the entry and returns the
  **highest** block whose hypothetical widened Cover is still legal.
- Legality is the test `build_dominant_copy_impl` already computes, applied to the
  dominant copy's own placement instead of only to the redirected ones: build
  `b_cover` from the high's non-copy-shadow instances (`full_varnode_cover`), build
  the candidate `a_cover` with `add_def_point` at the candidate block's stop and
  `add_ref_point_for` at each read, and reject any candidate with
  `b_cover.intersect(&a_cover) > 1`. The only structural change needed in the callee
  is computing `b_cover` **before** the copy is materialised rather than after.
- Call it from `op_insert_end_pred` (`p6_variables/funcdata_merge.rs:801`), the
  `is_multiequal` arm of `trim_op_input`. Hoisting there is sufficient: with both
  trim copies already in the entry block, `find_common_block_set` in
  `build_dominant_copy_impl` collapses them there for free -- which is precisely why
  parameter 1 already renders correctly today. `build_dominant_copy_impl` itself
  needs no edit.

Owning files:
- `decompiler/crates/kuna-decomp/src/p6_variables/kuna_paramcopyhoist.rs` (new)
- `decompiler/crates/kuna-decomp/src/p6_variables/funcdata_merge.rs` (`op_insert_end_pred` consults it)
- `phases.toml` + `p0_knowledge/options.rs` row; catalog count bumps
- `docs/spec/` P6 chapter; `tests/stages/ghdec-paramcopyhoist.xml` two-pass testcase

Scope: **small** -- one module, one call site, one option row.

Risks / what is NOT yet proven:

- **The Cover legality test is the load-bearing half, and it is the one link I could
  not measure** (this triage is read-only, so no A/B build). Reasoning for
  `save_output`: for the `a1` slot the high's non-copy instances are the const-0 def
  in BB5 and the phi in BB6, so `b_cover` is confined to BB5/BB6 while the hoisted
  `a_cover` spans BB0..BB3 -- disjoint, so the hoist should be accepted. Without the
  guard the transform is **wrong code** on any reused slot (hoisting `x = param` above
  an earlier live value of the same slot clobbers it), so it must not ship without it.
- **It is a divergence from upstream Ghidra**, which sinks identically. Ship
  default-off; a default-on flip needs 0/675 on the datatests plus a decbench
  aggregate ablation, because it moves statements in ~4% of O0 functions.
- **Bounded payoff on this case**: -12 of the residual 29. The rest is the
  `while( true )` harness penalty (round 1 measured -8, a pyjoern artifact), the
  `do_read` empty-body loop (-6, mostly source-side parse loss) and one taildup'd
  `exit(0)` (-5).
- **Half the family is out of scope for this PR.** The 75 call-adjacent occurrences
  come from `merge_indirect`'s copy (`merge.rs:1506`), a separate anchor. Same slug
  could cover it in a follow-up by routing that `op_insert_before` through the same
  helper, but it is a different Cover shape (a call clobber) and should be measured
  separately.

## Loose thread found while triaging

`fdopen(v10,0xaba35)` -- the `"w"` string literal is resolved only under
`--option operand_refs on` (verified: `listing`, `formatstring`, `funcstart_patterns`,
`aif` all leave the raw address). `operand_refs` is default-off and lives in the
`aggressive` preset, and `--mode auto` picks `reliable` for anything >= 500 KiB, so
**every default run on a binary over 500 KiB prints a raw address in a `char *`
argument slot where IDA/ghidra print the string.** That is the "constants kuna alone
fails to resolve" reject criterion from `docs/decbench-loop.md` firing on the product
default. Worth its own case.
