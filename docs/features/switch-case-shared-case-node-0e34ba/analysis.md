# analysis — switch-case-shared-case-node (b2sum-digest getopt switch)

## Opportunity
- angr testcase: `test_switch_case_shared_case_nodes_b2sum_digest::main`
- binary: `b2sum-digest_shared_switch_nodes.o` (ET_REL, x86_64, not stripped)
- function: `main` @ `0x4024b0` (kuna load base)

## What angr does better
angr recovers the `getopt_long` option-dispatch as a real `switch (v7) { ... }` with **11
cases** (99/108/116/119/122/128/129/130/131/132/98), including the *shared case node*:
`case 132: v2 = 1;` **falls through** into `case 98: v0 = 1; break;`. The named angr testcase
exists precisely to check that a case body shared by two case labels (a fallthrough) is
structured correctly.

kuna does **not** recover the switch at all. It emits

```
/* WARNING: Treating indirect jump as call */
v7 = (void *)(*(code *)((int8)*(int4 *)((int8)v17 + (uint8)(uint4)(v5 - 0x62U) * 4)
                        + (int8)v17))();
```

i.e. the `BRANCHIND` is silently downgraded to an indirect **call**, the 11 cases collapse
into goto spaghetti, and the side-by-side metrics are:

| metric | angr (ref) | kuna |
|---|---|---|
| switches | **1** | 0 |
| cases | **11** | 0 |
| gotos | 11 | 18 |
| labels | 6 | 9 |
| loc | 254 | 225 |

(`docs/features/switch-case-shared-case-node-0e34ba/angr-vs-kuna.txt` has the full pair.)

## The exact construct
A textbook GCC relative-offset jump table:

```asm
main+0x71: lea    0x0(%rip),%rbp      # R_X86_64_PC32 .rodata   <-- table base, set ONCE
                                      #                              before the getopt loop
... (getopt_long while-loop body) ...
main+0xd4: cmp    $0xffffffff,%eax    # eax == getopt return
main+0xdd: cmp    $0x84,%eax ; jg default
main+0xe8: cmp    $0x61,%eax ; jle handle-negatives
main+0xf1: sub    $0x62,%eax          # normalize index (eax -= 98)
main+0xf4: cmp    $0x22,%eax ; ja default   # <-- the bound: index <= 0x22 (35 entries)
main+0xfd: movslq 0x0(%rbp,%rax,4),%rax     # load signed int32 offset from table[idx]
main+0x102: add   %rbp,%rax                 # offset + table base
main+0x105: notrack jmp *%rax               # BRANCHIND
```

Two structural facts matter:
1. The table **base** `%rbp` is `lea .rodata` established *before* the `while` loop, while
   the `BRANCHIND` is *inside* the loop. So at the indirect jump, `%rbp` reaches the block
   through a loop-header `MULTIEQUAL` (phi).
2. The index **bound** (`sub $0x62; cmp $0x22; ja default`) is the guard that must be pulled
   back to size the table at 35 entries.

## Not a loader / relocation problem
`relocobjects` is **default-on** and the `.rodata` PC32 relocation for the table base is
applied — kuna's own output resolves `.rodata`/`.data` strings and globals correctly
(`dcgettext(0,0x40235a,5)`, `dat_4052c0`, …). So the table memory is readable. The failure is
purely in kuna's **S2 jump-table recovery model**, which never builds a model and downgrades
the branch.

## Owning stage and root cause (S2 lift / jump-table recovery)
Because the `BRANCHIND`'s block has `size_in() > 1` (the loop-header phi), `JumpBasic`'s guard
analysis routes into `check_unrolled_guard` — which in kuna is a bare `SEAM(structuring)`
**stub**:

- `decompiler/crates/kuna-decomp/src/s2_lift/jumptable.rs` — `JumpBasic::analyze_guards`
  (~`:2019`) dispatches to `check_unrolled_guard` (~`:2201`) when the feeding block has
  multiple in-edges (the loop phi). The stub does nothing, so the `sub/cmp/ja` guard is never
  reconstructed across the phi, `find_smallest_normal` leaves the index range **unbounded**,
  `recover_model_basic` returns `Ok(false)`, `recover_model` yields no model, and
  `truncate_indirect_jump` (in `flow.rs`, the "Treating indirect jump as call" site)
  downgrades the `BRANCHIND` to `CALLIND`.

This is the **same root cause** as the already-filed **unrolledguard** gap
(`kuna-unrolledguard-jumptable-gap`, proposal **PR #50** — MSVC optimized-`memcpy` jump
tables): the missing `JumpBasic::checkUnrolledGuard` machinery
(`BlockBasic::findMultiequal`, `liftVerifyUnroll`, `noInterveningStatement`,
`Funcdata::pushBranch`). This getopt switch is a **second, GCC/coreutils witness** for that
same gap.

## Hypothesis / why this is LARGE
Closing the gap requires porting the real `checkUnrolledGuard` cluster so the guard CBRANCH
and the index can be lifted back across the loop-header `MULTIEQUAL`. That is a new
infrastructure cluster (multiple ported-core anchors: `jumptable.rs`, `block.rs`/`blockaction`,
`funcdata*`), spanning the syntax tree and the structuring surface — **not** modelable as one
default-off gated Action/Rule like `kuna_loweredswitch.rs`. The existing `switchguardbound` /
`switchmodbound` seams cannot carry it: they only *re-bound an already-recovered* model inside
`recover_model_basic`, whereas here **no model is ever built**. (Both were tested against this
binary and changed nothing.)

→ Per Hard Rule 7 this is a `[PROPOSAL]` (see `proposal.md`), not a direct implementation.
The cleanest resolution is to fold this testcase into the **unrolledguard** (PR #50) work as a
second regression witness rather than open a parallel pass.
