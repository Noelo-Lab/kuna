# itecondlist — kuna re-rolls only every other diamond in a chain

Campaign case: `docs/decbench/triage/O0-bash-bash-shell_initialize.md` (IDA pool,
margin 105, tier L).

## The symptom, named

In a **run** of structurally identical two-arm assignment diamonds, kuna's own
`iteregion` / `iteboolean` passes re-roll only **ceil(N/2) of N**. The rest print
as `if (c) V = A; else { V = B; }` — three extra CFG blocks and four extra edges
each against a source that wrote one `?:` or one boolean assignment.

The tell is that *which* diamonds are missed depends on the **neighbouring**
structure, not on the diamond itself: an immediately preceding structured `if`
suppresses the very next one.

### The driving case

`O0-bash-bash-shell_initialize` (`bash` at `0x34684`). The source has three
occurrences of one identical 4-clause `||` chain, all in argument position:

```c
  initialize_shell_variables (shell_environment,
       privileged_mode||restricted||should_be_restricted||running_setuid);
  ...
  initialize_shell_options (privileged_mode||restricted||should_be_restricted||running_setuid);
  initialize_bashopts     (privileged_mode||restricted||should_be_restricted||running_setuid);
```

kuna before, 1 of 3 re-rolled:

```c
  v1 = shell_is_restricted(shell_name);
  if (((privileged_mode) || (restricted)) || ((v1 || (dat_172330))))
    v4 = 1;
  else {
    v4 = 0;
  }
  initialize_shell_variables(shell_environment,v4);
  initialize_job_control(jobs_m_flag);
  initialize_bash_input();
  initialize_flags();
  v2 = (((privileged_mode) || (restricted)) || ((v1 || (dat_172330))));
  initialize_shell_options(v2);
  if (((privileged_mode) || (restricted)) || ((v1 || (dat_172330))))
    v2 = 1;
  else {
    v2 = 0;
  }
  initialize_bashopts(v2);
```

kuna after, 3 of 3, source-identical:

```c
  v1 = shell_is_restricted(shell_name);
  v4 = (((privileged_mode) || (restricted)) || ((v1 || (dat_172330))));
  initialize_shell_variables(shell_environment,v4);
  initialize_job_control(jobs_m_flag);
  initialize_bash_input();
  initialize_flags();
  v2 = (((privileged_mode) || (restricted)) || ((v1 || (dat_172330))));
  initialize_shell_options(v2);
  v2 = (((privileged_mode) || (restricted)) || ((v1 || (dat_172330))));
  initialize_bashopts(v2);
```

### The N-sweep

Generator (`gcc -O0 -c`), N chained identical diamonds in one function:

```c
int a,p,q; void g(void); void f1(int); ... void fN(int);
void repron(void){ int x;
  g(); if(a) x=p; else x=q; f1(x);
  ...  /* N times */
}
```

| N | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|
| ternaries, stock | 1 | 1 | 2 | 2 | 3 | 3 | 4 | 4 |
| ternaries, fixed | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |

Exactly `ceil(N/2)` becomes exactly `N`.

## Root cause — confirmed by instrumentation, not inferred

`iteregion` and `iteboolean` reach the diamond's condition component through the
same helper, `kuna_iteregion.rs (leaf_bblock)`, which descends a `BlockList` of
**one** component but bails on a list of two or more:

```rust
BlockType::Ls | BlockType::Graph => {
    if blk.get_size() != 1 { return None; }
    leaf_bblock(data, blk.get_block(0))
}
```

The collapse structurer concatenates a just-collapsed predecessor together with
the following condition block into a two-component `BlockList`, and that list
becomes component 0 of the *next* `BlockIf`. A chain of diamonds alternates
between the two shapes.

Instrumenting `match_ite_assignment` on the N = 3 reproducer prints, per matched
`BlockIf`:

```
node=BlockId(14v1) cond_ty=Copy         cbr=true    armT=true armE=true   <- folds
node=BlockId(16v1) cond_ty=Ls cond_sz=2 cbr=false   armT=true armE=true   <- declines
node=BlockId(12v1) cond_ty=Copy         cbr=true    armT=true armE=true   <- folds
  (all six arms: ops=[CPUI_COPY] dest=stack:-0xc:4)
```

Both arms of the declined diamond pass every test; the only failing predicate is
the condition-leaf descent. That is the whole defect.

## Two corrections to the original filing

1. **`mavlinkSendRCChannelsAndRSSI` is not this case.** On that pane only 2 of 6
   misses are the condition-list shape; the other 4 fail `same_storage` — the two
   arms write *different* raw storage (a register vs `unique:0x139f00` /
   `stack:0xffffffd4`) even though they render as one variable. That is a
   separate defect, and that pane also needs `iteexpr` (default-off) to show any
   ternary at all, so this fix delivers nothing on its benchmark surface. Left
   alone deliberately.

   The same `same_storage` wall is what makes the obvious reproducer shapes fail:
   instrumented, `if (c) x = p; else x = q;` over *register*-resident values
   reports `dest=unique:0x10000092:4` / `dest=unique:0x10000096:4` — two distinct
   post-SSA instances of one HighVariable. Only diamonds whose arms write the
   same *raw* storage (a stack slot) reach the condition predicate at all.

2. **Breadth is ~59 folds, not the 952-diamond census.** The census counted every
   declined diamond; the condition-list shape is only ~5% of them (see below).

## Owning phase

**P8** — Structured AST & goto quality
(`decompiler/crates/kuna-decomp/src/p8_structure/`), spec chapter
`docs/spec/08-structuring.md` section 8.3.

## Why the fix is sound

The rewrite is print-only and the printer already renders this exact shape.
`PrintC::emit_block_if` emits the condition component **twice** — once under
`NO_BRANCH` (its statements) and once under `ONLY_BRANCH` (its branch condition)
— and `PrintC::emit_block_ls` emits every component in order under `NO_BRANCH`
but **only the last** under `ONLY_BRANCH`. The leading components therefore
already print exactly once, as ordinary statements ahead of the `if` header, and
`emit_block_if_ite` / `emit_block_if_bool` perform the same two emissions on the
same block. Folding only the tail cannot disturb, duplicate or drop anything that
precedes it.

The descent is restricted to `BlockList`. `BlockGraph` is deliberately excluded:
`PrintC::emit_block_graph` ignores `ONLY_BRANCH` and emits every component, so
descending one would make the emitters print the whole graph twice — a real bug
the `Ls`-only restriction avoids.

## Measured breadth (stock vs fixed, whole-binary `decompile-all --mode reliable`)

| binary | functions | folded diamonds gained | ternaries | iteboolean re-rolls | changed lines |
|---|---|---|---|---|---|
| coreutils `ls` (O0) | 613 | 5 | 21 -> 24 | 23 -> 24 | 36 |
| iproute2 `ip` (O0) | 1,962 | 3 | 95 -> 97 | 2 -> 2 | 24 |
| `bash` (O0) | 3,278 | 17 | 124 -> 135 | 99 -> 108 | 138 |
| betaflight STM32F405 (O2-noinline, ARM) | 6,388 | 34 | 141 -> 178 | 0 -> 0 | 268 |
| **total** | **12,241** | **59** | **381 -> 434** | **124 -> 134** | **466** |

Roughly 5% of the residual declined diamonds in the corpus are this shape; the
rest are the `same_storage` defect and the arm-shape declines `iteexpr` covers.

## Regression sweep

Every hunk of all four A/B diffs was audited mechanically (`audit.py` in this
bundle): each must be either a function-header comment whose ternary / re-roll
count went **up**, or a removed `if (C) V = A; else { V = B; }` block replaced by
exactly one added line `V = (C) ? A : B;` / `V = (C);` / `V = !(C);` with the
*same* destination, *same* condition and *same* arm expressions in the same
order.

Result over 12,241 functions: **52 header-only hunks and 59 folded diamonds, 0
real problems.** Two hunks the audit could not classify automatically, both
hand-checked:

- `bash _rl_init_terminal_io`:
  `v4 = !((v4 != 1) || (v4 = tgetflag(0x147219), v4 != 1));` — the arm-extraction
  regex trips over the assignment *inside* the condition. The transform is
  correct: the condition is rendered by the same `ONLY_BRANCH` renderer, so the
  comma-expression side effect and its ordering survive verbatim. Writing the
  destination inside its own RHS is the shipped `iteboolean` (DIV-51) contract —
  its module docstring carries exactly this shape — not something this change
  introduces.
- `bash command_word_completion_function`: header only. Two accumulated
  `iteboolean` warning lines ("re-rolled 1", "re-rolled 2") collapse into one
  ("re-rolled 2") because both structuring rounds now find the same count. The
  body is byte-identical.

Error/diagnostic line counts are unchanged in every binary (ls 24 -> 24,
ip 290 -> 290, bash 633 -> 633, betaflight 1 -> 1): no function starts or stops
producing output.

## Rebase note (DIV-55 -> DIV-56)

This work was rebased onto PR #252 (`1b6d4db4`, DIV-55, `returncopysplit`), which
was dispatched at the same time and merged first. DIV numbers are claimed on
**merge**, so this row renumbered to **DIV-56**; the two rows are kept separate
(#252 declines a datatype copy split into a read-only output, a different
mechanism entirely).

Both PRs also moved the `decompiler/crates/kuna-base/src/xml.rs` corpus count
187 -> 188, so the rebase resolves it to **189**, and `docs/baseline-stages.json`
was **re-recorded** on the merged base rather than hand-merged (the auto-merge
left a stale `375` in `data_footer`; the correct figure is 381 = 369 + 6 for
#252 + 6 for this PR).

Everything above was **re-measured on the rebased tree**, not carried forward.
#252 changes emitted C (it removes phantom `.rodata` store statements), so the
OFF arm of all four sweep binaries was re-generated and diffed against the
pre-rebase OFF arm: **0 lines differ in every one of them** (#252's witnesses are
`O2-noinline` `ip`/`sshd`; this sweep's `ip` is `O0`). The breadth table, the
53 changed functions and the mechanical audit therefore reproduce exactly, and
the ablation is 0/675 again with the option default-ON. GED on the driving case
re-measures at **46 -> 24** on the rebased commit. Only the timings moved, as
expected from a shared box: `timeit` 356.33 -> 354.28 ms (-0.58%), interleaved
`ls` 7,193 -> 7,220 ms (+0.38%), gzip 2,881 -> 2,873 ms (-0.28%).
