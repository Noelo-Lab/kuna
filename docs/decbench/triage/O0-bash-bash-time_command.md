---
case_id: O0-bash-bash-time_command
pool: ida
status: feature-candidate
tier: M
margin: 55
fresh_verdict: today's build already went 55 -> 26 GED (print-normalization only, CFG shape unchanged); the whole residual gap is two `if (c) x=1; else x=0;` boolean-materialization diamonds, and hand-rerolling them takes GED to 6 (28 nodes, exactly the source's)
option_closing: null
feature_slug: iteboolean
scope: small
confidence: high
---

## Side-by-side

Everything below is the *same* region of the function: the two `-O0` boolean
assignments at the top of the body. The rest of the function matches IDA
statement-for-statement, so this is the whole structural delta (IDA 8 `if`s,
kuna 10 `if`s; nothing else differs).

**ida (stored) — GED 0, isomorphic to the source CFG**

```c
  v18 = subshell_environment;
  v5 = a1 && (*(int *)(a1 + 4) & 0x100) != 0;
  v20 = v5;
  v6 = !a1
    || *(int *)a1 == 4 && !*(long long *)(*(long long *)(a1 + 24) + 8LL) && !*(long long *)(*(long long *)(a1 + 24) + 16LL);
  v21 = v6;
  if ( posixly_correct && v21 )
```

**kuna (stored, benchmark run) — GED 55**

```c
  v26 = dat_173a54;
  if ((a0 != (int4 *)0x0) && (v2 = a0[1] & 0x100, v2 != 0)) {
    v2 = 1;
  }
  else {
    v2 = 0;
  }
  v28 = v2;
  if ((a0 != (int4 *)0x0) && (((*a0 != 4 || (*(int8 *)(*(int8 *)&a0[6] + 8) != 0)) || (*(int8 *)(*(int8 *)&a0[6] + 0x10) != 0)))) {
    v2 = 0;
  }
  else {
    v2 = 1;
  }
```

**kuna (fresh, today's build, `--mode reliable` == what `auto` picks for this 1.5 MB binary) — GED 26**

```c
  v26 = subshell_environment;
  if ((a0) && (v2 = a0[1] & 0x100, v2))
    v2 = 1;
  else {
    v2 = 0;
  }
  v28 = v2;
  if ((a0) && (((*a0 != 4 || (*(int8 *)(*(int8 *)&a0[6] + 8))) || (*(int8 *)(*(int8 *)&a0[6] + 0x10)))))
    v2 = 0;
  else {
    v2 = 1;
  }
```

Same control flow as the stored block — only rendering changed (truthy
conditions, NULL printing, brace elision, symbol naming). The `-29` GED is
therefore a *metric* response to DIV-34..39 print normalization, not a
structural fix: pyjoern's block CFG counts `x != 0` chains and braced
single-statement bodies as extra nodes (41n/58e stored vs 34n/51e fresh, with
an identical `if` skeleton).

**hypothetical: the same fresh output with only those two diamonds re-rolled — GED 6, 28n/43e**

```c
  v2 = (a0) && (v2 = a0[1] & 0x100, v2);
  v28 = v2;
  v2 = !((a0) && (((*a0 != 4 || (*(int8 *)(*(int8 *)&a0[6] + 8))) || (*(int8 *)(*(int8 *)&a0[6] + 0x10)))));
```

Measured with the benchmark's own machinery (`decbench.metrics.ged.GEDMetric`
on pyjoern CFGs, every pane parsed identically):

| pane | nodes/edges | GED |
|---|---|---|
| source (`execute_cmd.i`) | 28 / 44 | — |
| ida (stored) | 28 / 44 | **0** (isomorphism) |
| angr (stored) | 36 / 54 | 34 |
| ghidra (stored) | 43 / 62 | 59 |
| kuna (stored) | 41 / 58 | 55 (reproduces the recorded value) |
| kuna (fresh, today) | 34 / 51 | 26 |
| kuna + hand boolean re-roll | 28 / 43 | 6 |

`scripts.decbench.rescore` agrees: `before 55, after 26, delta -29` at
commit `895b1f1b`.

## Source

`~/github/decbench/results/full_run/O0/bash/compiled/execute_cmd.i`, `time_command`:

```c
  old_subshell = subshell_environment;
  posix_time = command && (command->flags & 0x100);

  nullcmd = (command == 0) || (command->type == cm_simple && command->value.Simple->words == 0 && command->value.Simple->redirects == 0);
  if (posixly_correct && nullcmd)
```

gcc `-O0` materializes each of these as a jump-based 0/1 select whose constant
arms are reached from *several* predecessors (`objdump`, `0x4ea9c`-`0x4eac5`):

```
 4ea9c: cmpq $0x0,-0x3c8(%rbp) ; je 4eac0     <- false arm, edge 1
 4eaa6: mov 0x4(%rax),%eax ; and $0x100,%eax
 4eab5: test %eax,%eax ; je 4eac0             <- false arm, edge 2
 4eab9: mov $0x1,%eax ; jmp 4eac5
 4eac0: mov $0x0,%eax                         <- 2 in-edges
 4eac5: mov %eax,-0x3ac(%rbp)                 <- the MULTIEQUAL
```

The second idiom (`nullcmd`, `0x4eacb`-`0x4eb17`) is the same shape with a
3-way `||`: its `mov $0x0` arm has three in-edges.

## Analysis

**Structural symptom (one).** kuna renders each `-O0` short-circuit boolean
assignment as an explicit two-arm `if`/`else` constant diamond plus a separate
store — `if ((a0) && (...)) v2 = 1; else v2 = 0; v28 = v2;` — where the source
and Hex-Rays have a single boolean-valued assignment. Two of them add 6 CFG
blocks / 8 edges to a 28-node function; they are *the entire* remaining
difference from IDA's pane (removing them by hand takes kuna from 34n/51e to
28n/43e, i.e. exactly the source's node count).

**Root cause.** Nothing in kuna can turn a *structured* short-circuit condition
plus constant 0/1 arms back into a boolean expression:

- P3 `RuleConditionalMove`
  (`decompiler/crates/kuna-decomp/src/p3_dataflow/ruleaction_7.rs:1882`, port of
  Ghidra `ruleaction.cc:9292`) is the rule that folds `MULTIEQUAL(0,1)` into
  `zext(cond)`/`BOOL_AND`/`BOOL_OR`, and it *does* fire on the simple case. It
  requires each MULTIEQUAL input block to be either the CBRANCH root itself or a
  **single-predecessor** pass-through (`if block_size_in(inblock) != 1 { return 0 }`).
  A short-circuit chain gives the constant arm 2+ predecessors, so it bails.
  Confirmed on a purpose-built probe (gcc -O0, today's build):
  `int simple(int a,int b){int x=(a>b);return x;}` -> `return a1 < a0;` (folded),
  while `int shortcircuit(int *p){int x = p && (p[1]&0x100); return x;}` ->
  `if (!a0) return 0; if (!(*(uint4 *)(a0+4) & 0x100)) return 0; return 1;` (not folded).
  This bail is *correct at the IR level*: hoisting the guarded operand out of the
  branch would evaluate `command->flags` when `command == 0`. The short-circuit
  must stay in the CFG; only C's own `&&`/`||` can express it.
- P8 `iteregion` (`p8_structure/kuna_iteregion.rs`) is the pass that does collapse
  a two-arm assignment diamond after structuring, but `cond_cbranch()` /
  `leaf_bblock()` require the condition component to reduce to a single
  `Basic`/`Copy` leaf ending in the CBRANCH. Here the condition component is the
  `BlockType::Condition` produced by the short-circuit fold, so
  `match_ite_assignment` declines.

**Owning phase: P8 structure** (rendering seam in P9) — the IR is right and must
stay as it is; the decision that is missing is a structured-tree one.

**Why IDA wins.** Hex-Rays re-rolls the 0/1 materialization into a boolean
expression at the microcode level regardless of how many predecessors the
constant arms have, so its pane keeps the source's `x = a && b;` shape and hits
the metric's isomorphism fast path (GED 0). ghidra (59) and angr (34) both
expand the idiom too — kuna is not an outlier, it is the same class of miss.

**Not the answer:** swept `condfold on`, `condfold wide`, `iteexpr on`,
`returndup on`, `regionedgeorder on` and `--mode aggressive` — all produce
byte-identical output to the default run on this function. No existing option
closes it.

**Secondary observations (no GED impact, not the symptom):**
- kuna calls `sub_4e553(stderr,v33,v14,v13,v17,v16)` with 6 arguments; the callee
  (`print_formatted_time`) takes 9 — the three stack-passed arguments are missing.
  That is a P4 prototype/parameter-recovery miss on a prototype-less static callee;
  IDA recovers all 9.
- The residual GED 6 in the hypothetical is one missing edge at the tail: kuna
  flips the source's `if (code) siglongjmp(...); return rv;` into
  `if (!v31) return v27; siglongjmp(...);`, so the trailing no-return call has no
  fall-through successor while the source's has one. Not worth a feature.

## Proposed fix

**Mechanism (small, one module + one printer branch).** Extend `iteregion`'s
matcher with a boolean-assignment case, behind a new option (`iteboolean`,
default-off until an ablation earns the flip — same judgment-call category as
`iteregion`/DIV-17, since the source may genuinely have written the `if/else`):

1. In `match_ite_assignment` (`p8_structure/kuna_iteregion.rs`), accept a
   condition component of `BlockType::Condition` (the folded `&&`/`||` chain), not
   only a single-CBRANCH leaf. The condition is already printed as a C expression
   inside the `if (...)` today, so it is renderable as an r-value unchanged.
2. Add the specialization: when the two arms are the constants `1` and `0` written
   to the same storage, mark for `dest = cond;` (arms `0`/`1` -> `dest = !cond;`)
   instead of `dest = cond ? 1 : 0;`. With `branchflip off` the second idiom's
   condition already prints as `(!a0) || ((*a0 == 4 && !x) && !y)` — literally the
   source's `nullcmd` expression — so the re-roll should run before/with the
   existing negation choice rather than fight it.
3. Printer: reuse `PrintC::emit_block_if`'s existing `kuna_iteregion` path in
   `p9_emit/printc.rs`; this is the same print-only marking, no p-code mutation.

**Owning files.** `decompiler/crates/kuna-decomp/src/p8_structure/kuna_iteregion.rs`,
`src/p9_emit/printc.rs`, `phases.toml` + `p0_knowledge/options.rs` (new settable
row + the hard-coded catalog counts), `docs/spec/08-structuring.md` (the
`iteregion` section) and `docs/spec/09-emission.md`, plus a two-pass
`tests/stages/` case.

**Risks.** (a) Faithfulness: an explicit `if (c) x = 1; else x = 0;` in the source
compiles to the same object code, so the rewrite can diverge from source — this is
why it stays optional. (b) Side effects riding on the condition: the folded
condition may contain assignments (`v2 = a0[1] & 0x100, v2`); moving it into an
r-value keeps evaluation order and C's short-circuit semantics, but the matcher
must still decline when an arm is labelled / a goto target, exactly as today.
(c) The destination temp (`v2`) is reused as the arm register; emitting
`v2 = (a0) && (v2 = ..., v2);` is legal but ugly — a follow-up could fold the temp
into the subsequent `v28 = v2;` store (IDA emits the same two-step form, so this is
not required for the win).

**Expected benchmark delta.** GED 26 -> ~6 on this case (measured on the hand-rerolled
pane). The idiom is ubiquitous in `-O0` C, so the same pass should move many other
O0 cases; that breadth is the argument for doing it, and the reason to measure a
full-corpus ablation before proposing the default flip.
