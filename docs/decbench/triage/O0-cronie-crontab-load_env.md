---
case_id: O0-cronie-crontab-load_env
pool: ida
status: feature-candidate
tier: L
margin: 177
fresh_verdict: still broken — the 7-case `switch (state)` at 0x9910 resolves to a 0- or 1-entry jump table on today's default, deleting the whole state-machine loop and 3 of 4 case bodies
option_closing: null
feature_slug: switchstagedindex
scope: proposal
confidence: high
---

## Side-by-side

The dispatch (identical in all three cronie binaries):

```
    98cd:  mov    DWORD PTR [rbp-0x28],0x0        ; state = NAMEI (0)  <-- only reachable def
    98e0:  jmp    9a8d                            ; -> loop header
    98e5:  cmp    DWORD PTR [rbp-0x28],0x6        ; range guard
    98e9:  ja     9a88                            ;   -> default: abort()
    98ef:  mov    eax,DWORD PTR [rbp-0x28]
    98fa:  lea    rax,[rip+0x39bb]        # d2bc  ; table base
    9901:  mov    eax,DWORD PTR [rdx+rax*1]
    9904:  cdqe
    9906:  lea    rdx,[rip+0x39af]        # d2bc
    990d:  add    rax,rdx
    9910:  notrack jmp rax                        ; BRANCHIND
    ...
    9a8d:  cmp    DWORD PTR [rbp-0x28],0x7        ; while (state != ERROR && *c)
    9a9c:  jne    98e5                            ; back edge into the guard
```

Table at `.rodata:0xd2bc`, 7 valid entries (`d2bc + sext(int32)`):
`0->9913  1->993f  2->99ed  3->9a54  4->9913  5->993f  6->9a54`.

### ida (stored) — GED 0, recovers all 7 cases

```c
  while ( v8 != 7 && *v10 )
  {
    switch ( v8 )
    {
      case 0: case 4:  if ( *v10 == 39 || *v10 == 34 ) { v3 = v10++; v7 = *v3; }
                       ++v8;  goto LABEL_11;
      case 1: case 5:  LABEL_11: ...  v4 = v10++; v5 = (char *)v11++; *v5 = *v4;  break;
      case 2:          if ( *v10 == 61 ) { ++v8; v7 = 0; ... } ...  ++v10; continue;
      case 3: case 6:  if ( isspace ) ++v10; else ++v8; continue;
      default:         abort();
    }
  }
```

### kuna (stored, benchmark run) — GED 177

```c
  if (*a0 == '\0') { ... tail ... }
  v2 = (int8)dat_d2bc;
                    /* WARNING: Treating indirect jump as call */
  return (*(code *)(v2 + 0xd2bc))();
```

### kuna (fresh, today's build)

`kuna decompile-all <stripped> --addr 0x983d` (the exact call decbench scores, `--mode auto` = `aggressive`) — **unchanged**, and note the constant-folded `state` leaking into the printf:

```c
  if (*a0) {
    v2 = (int8)dat_d2bc;
    return (*(code *)(v2 + 0xd2bc))(); // jump-as-call
  }
  if (dat_503d0 & 8)
    printf("load_env, not an env var, state = %d\n",0);   // <-- state folded to 0
```

`kuna decompile <bin> load_env` (single-function path, `auto`/`aggressive`; `--mode reliable`
gives the fully-collapsed form above) — best case today is a **one-entry** table whose case
label is the raw target address:

```c
label_9a8d:
  if ((state != ERROR) && (*c)) {
    switch(state) {                 // jump-as-call
      case 0x9913:                  // <-- the *target address*, not a case value
        goto label_9913;
      default:
        abort();
    }
  }
```

Case bodies `993f` (states 1/5), `99ed` (state 2) and `9a54` (states 3/6) are absent from
the output in every mode.

## Source

`~/github/decbench/results/full_run/O0/cronie/compiled/env.i`, `load_env`:

```c
 val = str = envstr;
 state = NAMEI;                 /* == 0 */
 quotechar = '\0';
 c = envstr;
 while (state != ERROR && *c) {
  switch (state) {
  case NAMEI: case VALUEI:  if (*c=='\'' || *c=='"') quotechar = *c++;  state++;
  case NAME:  case VALUE:   ... *str++ = *c++; break;
  case EQ1:                 if (*c=='=') { state++; ... } else if (!isspace(*c)) state = ERROR;
                            c++; break;
  case EQ2:   case FINI:    if (isspace(*c)) c++; else state++; break;
  default:                  abort();
  }
 }
```

`enum env_state { NAMEI, NAME, EQ1, EQ2, VALUEI, VALUE, FINI, ERROR }` — 7 dispatched
states + `default: abort()`, exactly the 7-entry table. **`state` is written *only* before
the loop (the constant `NAMEI`) and inside the switch case bodies.** That is the whole
story of this case.

## Analysis

**Structural symptom (one).** The 7-case `switch (state)` dispatch at `0x9910` is not
recovered: kuna emits either `return (*(code *)(v2 + 0xd2bc))(); // jump-as-call` or a
degenerate one-entry `switch(state) { case 0x9913: goto label_9913; }` whose case label is
the folded *target address*. The consequence is a whole-function structural loss — the
`while` loop and 3 of the 4 distinct case bodies (~100 lines, `0x993f`/`0x99ed`/`0x9a54`)
never appear, and `state` constant-folds to `0` in the surviving tail
(`printf("... state = %d\n", 0)`).

**Root cause.** Jump-table recovery runs once, on a *partial* flow in which the case
bodies have not been decoded yet (they are reachable only through the very table being
recovered). In that partial, the only reaching definition of the index slot `[rbp-0x28]`
is the constant `0` stored at `0x98cd`. `stackptrflow`/`stackvars`/`analysis` are in the
`"jumptable"` action group (`infra/action.rs:1649`), so the index constant-propagates —
which also folds away the `cmp state,6; ja default` guard and the table `LOAD`. `JumpBasic`
then melds down to a *constant* BRANCHIND input, so the model's index range is the folded
target itself: `JumpTable::recoverAddresses` yields 0 entries (→ `Err` →
`FlowInfo::truncateIndirectJump` → `CALLIND` + `"Treating indirect jump as call"`) or 1
entry labelled with the target address.

kuna's escape hatch for exactly this situation — upstream's multistage recovery — is
stubbed out:

- `p2_lift/jumptable.rs:5196` `JumpTable::check_for_multistage` returns
  `Err("JumpTable::checkForMultistage: Override::queryMultistageJumptable is W4")` and is
  **never called** anywhere in the tree.
- `p2_lift/flow.rs:2662` `FlowInfo::recover_jump_tables` carries
  `STUB(W4): the `notreached` re-queue ... needs the multistage outer loop; here every
  BRANCHIND in the list is attempted once`, and on a partial result calls
  `mark_complete()` instead of re-queuing (`flow.rs:2687`).

**Owning phase: P2 (lift/flow — jump-table / switch recovery).**

**Controlled proof on today's build** (`/tmp/.../scratchpad/jt/{t,u}.c`, `gcc -O0 -g`,
`kuna decompile ... --mode reliable`). Four variants of the *same* loop+switch source,
differing only in where the index comes from:

| variant | index seed | result |
|---|---|---|
| `sw_param` | parameter, no loop | switch, 7 cases |
| `sw_loop_param` | parameter, in the loop | switch, 7 cases |
| `sw_loop_global` | writable global, in the loop | switch, 7 cases |
| `sw_loop_const` | **constant 0**, written only by case bodies (= `load_env`) | **`jump-as-call`, loop deleted** |
| `sw_const_noloop` | **constant 2**, no loop at all | **`jump-as-call`** |
| `sw_loop_two_defs` | constant 0 **plus** one reachable non-case def (`st = 3`) | switch, **4 of 7 cases** |

`sw_const_noloop` proves the loop is irrelevant — a constant-folded index alone breaks
recovery. `sw_loop_two_defs` proves the recovered table size tracks the *reaching-definition
value set* (`{0,3}` → indices 0..3), not the guard bound (7).

**Why the reference wins.** IDA (and angr/binja/phoenix, GED 15/13/5) bound the table from
the CFG guard `cmp state,6; ja` before any value analysis, so the undecoded case bodies do
not matter. Ghidra loses the same way kuna does (GED 170, `"Could not recover jumptable at
0x00109910. Too many branches"`), so this is an inherited upstream limitation, not a kuna
regression — but it is the entire 177-point margin, and it is *not* a metric artifact:
source CFG is 56 nodes / 82 edges, `approximated: false`, and IDA's 126-line pane is a
faithful rendering of the source state machine.

**Options swept, none close it** (all on the real binary, single-option and combined):
`switchguardbound`, `switchmodbound`, `switchmultipred`, `switchsharedcase`,
`unrolledguard`, `stackalias`, `regionedgeorder`, `returndup`, `iteexpr`, `condfold wide`,
plus the analysis tier (`addrtable`, `operand_refs`, `fid`, `aif`, `eh_frame_full`,
`funcstart_patterns`, `fast_funcdisc`, `formatstring`) and `--mode aggressive`. The four
`switch*bound` options are *structurally* unable to help: they all hang off the
`jrange.get_size() > maxtablesize` branch in `recover_model_basic`
(`jumptable.rs:2536-2578`), and here the range is 0/1, not too large. Worse, their guard
scan (`scan_guard_tree`) looks for a CBRANCH that constant propagation has already deleted.
(`formatstring on` alone flips the single-function run from 0-entry to 1-entry — an
incidental dataflow side effect, not a fix.)

## Proposed fix

New P2 option, suggested slug **`switchstagedindex`** (`p2_lift/kuna_switchstagedindex.rs`,
canonical `kuna_loweredswitch.rs` template). Two candidate mechanisms:

**(a) Un-fold the index in the recovery partial (smaller, provably sufficient here).**
In `Funcdata::stage_jump_table` (`substrate/funcdata_block.rs:2364`), before running the
`"jumptable"` action set on the partial clone: if the BRANCHIND's parent is reachable from a
block with a not-yet-decoded successor (i.e. this BRANCHIND is the only thing standing
between the flow and more code) and the dispatch index's storage has exactly one reaching
def in the partial, replace that def with an `INDIRECT`/unknown so it cannot constant-fold.
`analyzeGuards` then sees the surviving `cmp state,6; ja` and bounds `[0,7)` — which is
precisely the `sw_loop_param`/`sw_loop_global` path that already works today.

**(b) Port upstream's multistage recovery (faithful, larger).**
Mark a table whose index melded to a constant as `partial`, keep its entries, and re-queue
the BRANCHIND in `FlowInfo::recover_jump_tables`' `notreached` list instead of
`mark_complete()`; after the new case-body flow is decoded, re-run
`recover_jump_table_flow` on a fresh partial. `JumpTable::recover_multistage`
(`jumptable.rs:5061`) and `recover_count` already exist; the missing pieces are the
re-queue, a stage cap (upstream `recoverystage`), and a non-override entry into
`check_for_multistage`.

**Owning files.** `decompiler/crates/kuna-decomp/src/p2_lift/jumptable.rs`
(`recover_model_basic`, `recover_addresses`, `check_for_multistage`),
`src/p2_lift/flow.rs` (`recover_jump_tables`), `src/substrate/funcdata_block.rs`
(`stage_jump_table`), plus `phases.toml` + `p0_knowledge/options.rs` + the catalog counts.

**Risks.** (a) over-bounds a dispatch whose index is *genuinely* constant when a stale
range guard survives — gate it on an actual dominating range CBRANCH on the index storage.
(b) risks restart non-termination and a whole-corpus speed regression (an extra partial
pipeline per unresolved BRANCHIND). Either way this is a `--mode aggressive`-tier opt-in
first, with a measured speed delta and a two-pass `tests/stages/` case built from the
`sw_loop_const` micro-reproducer above.

**Scope: proposal.** It changes flow-stage control over jump-table recovery, so it needs a
draft `[PROPOSAL]` PR before implementation. Payoff: the 3 sibling cases in this group
(`crontab`/`crond`/`cronnext` `load_env`, all margin 177) plus every O0 hand-written
lexer/parser state machine, which is the canonical shape for this bug.
