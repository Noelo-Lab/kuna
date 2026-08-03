# subright — restore the stubbed `RuleSubRight`

Route B (decbench), case `novel-concat-subpiece-soup`, NOVEL pool. The triage record
that nominated it lives on `campaign/decbench-round2-triage` as
`docs/decbench/triage/novel-concat-subpiece-soup.md`; this file records what the
implementation worker independently re-derived and measured.

## The symptom

Every SUBPIECE with a **nonzero byte offset** reached the printer as a raw p-code
operator. `SUBnn` is an undeclared identifier — kuna emits no helper header that
defines it — so the emitted function is not compilable C.

openssh `ssh` (O2-noinline, stripped) `sshbuf_put_u64` @ `0x4cd60`, `--mode reliable`:

```c
  *v2 = SUB81(a1,7);          /* before */
  v2[1] = SUB81(a1,6);
  ...
  v2[7] = (char)a1;           /* the offset-0 one already rendered as a cast */
```

Upstream Ghidra on the same function:

```c
  *local_18 = (char)((ulong)param_2 >> 0x38);
  local_18[1] = (char)((ulong)param_2 >> 0x30);
  ...
```

and the source (`sshbuf-getput-basic.i`) is a plain shift:
`((u_char *)(p))[0] = (__v >> 56) & 0xff;`. So the construct is ordinary
arithmetic that kuna failed to reassemble, not honest bitfield code — and the
last line of kuna's own pane is the proof that the printer was never at fault:
the offset-0 SUBPIECE already renders as `(char)a1`.

## The owning phase and the decision

**P3, simplification quiescence, cleanup pool.**
`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_6.rs (RuleSubRight)`
(upstream `ruleaction.cc:7271`, pinned `GHIDRA_REV=cef869af`) is the rule that
normalizes a non-least-significant truncation into `sub(V,c) => sub(V>>c*8,0)`.

It was a **port stub**. Only the leading `doesSpecialPrinting` /
`isPieceStructured` half was transcribed; the body then read

```rust
        // Remainder transcribed for the next wave; unreachable at this merge base:
        //   PcodeOp *lone = outvn->loneDescend();  ... lump a lone right-shift ...
        // W6
        0
```

so the rewrite never ran. Nothing else was wrong: the rule is already registered
`rrow!("subright", "cleanup", ...)` at `infra/universalaction.rs:552`, which is
exactly where upstream schedules it (after `fullloop`, before `ActionAssignHigh`/
`ActionMergeRequired`), so the synthesized shift and its unique get Highs assigned
normally. Every primitive the tail needs already existed
(`Varnode::is_addr_tied`/`overlap`, `Funcdata::new_op`/`new_unique`/
`op_set_output`/`op_set_input`/`op_insert_before`/`op_unlink`/`new_constant`,
`get_arch().types().get_base`).

Confirmed by construction rather than by reading: with the rule disabled at
runtime (`option togglerule subright off`) today's build reproduces the exact
pre-fix rendering, and with it enabled it produces upstream's shape — see
`tests/stages/ghdec-subright.xml`, whose two passes are precisely that A/B.

## Not covered by any existing option

The triage swept all 83 catalog rows for subpiece/truncation/piece and found
nothing relevant; `--mode auto|reliable|aggressive|fast` and `option condfold on`
all leave the witness at 7 `SUB81`s. Reconfirmed here: the only runtime control
that changes it is `togglerule`, which names the rule itself.

## Breadth

Triage's corpus-wide count (matched per-function comparison over 788 binaries /
76,046 functions both kuna and ghidra recovered, nesting-aware balanced-argument
scanner):

| | SUBPIECE with offset != 0 |
|---|---|
| kuna | **5,919 in 2,394 functions (3.1%)** |
| ghidra | 18 in 13 functions (0.017%) |
| ida | 0 |

A **329x** ratio. Shape distribution over kuna's 7,839 occurrences: SUB41 2,640,
SUB84 2,568, SUB81 1,021, SUB87 496, SUB42 472, SUB21 377, rest < 200.

The before/after measured directly for this PR is in `record.json` and the PR
body: over 7 whole-binary `decompile-all --mode reliable` runs (x86-64 -O0/-O2 and
ARM Cortex-M, 14,720 functions common to both panes) nonzero-offset raw operators
go **1,912 -> 95** (95.0%), the functions carrying one go 733 -> 20, and the blast
radius is 728 functions (4.95%) with emitted lines +0.011%.

Every one of the 95 residuals is a case upstream's own guards decline: 42
address-tied global-array stores (round 1's separately-filed
`rodata-phantom-store`), 47 address-tied local partial-symbol writes (the P6
thread that renders `sym._2_2_` upstream), and 6 piece-structured field
extractions on the special-print branch. `subright` owns 1,817 of the 1,912.

## Not the CONCAT half of the case

The triage cluster also named CONCAT soup. That half is **disproven as a kuna
defect** — corpus-wide kuna 4,198 vs ghidra 3,700, and on the two rows that named
it (`cleanflight::ftoa`, `betaflight/cleanflight::applyLedFixedLayers`) ghidra
emits the same or more. This PR does not touch it.

## Why this ships without an option

See `plan.md` — short version: `SUBnn` is an undeclared identifier, so this is a
strict correctness fix, and it restores upstream Ghidra's own behavior from a
port stub rather than making a judgment call. The vendored-from-upstream datatest
corpus agrees: **0 of 675 assertions move.** It is therefore a **Convergence**
(`docs/history.md`), not a divergence, and earns no DIV row.
