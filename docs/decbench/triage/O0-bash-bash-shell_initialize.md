---
case_id: O0-bash-bash-shell_initialize
pool: ida
group_id: bash::shell_initialize
status: feature-candidate
tier: L
margin: 105
fresh_verdict: reproduces on today's build in BOTH modes (no-flag/auto and --mode reliable are byte-identical here); the function has three *structurally identical* `x = a||b||c||d;` call arguments and `iteboolean` re-rolls exactly ONE of them, leaving two explicit `if (...) x=1; else {x=0;}` diamonds where IDA and the source have one statement
option_closing: null
feature_slug: itecondlist
scope: small
confidence: high
---

## Side-by-side

Only the residual region is shown; the rest of the pane matches IDA
statement-for-statement (fresh kuna 6 `if`s vs IDA 4; the delta is exactly the two
un-re-rolled diamonds).

**IDA (stored)** — three identical boolean assignments:

```c
  v2 = privileged_mode || restricted || is_restricted || dword_172330;
  initialize_shell_variables(shell_environment, v2);
  ...
  v3 = privileged_mode || restricted || is_restricted || dword_172330;
  initialize_shell_options(v3);
  v4 = privileged_mode || restricted || is_restricted || dword_172330;
  initialize_bashopts(v4);
```

**kuna (fresh, today's build, no flags)** — one re-rolled, two not:

```c
  v1 = shell_is_restricted(shell_name);
  if (((privileged_mode) || (restricted)) || ((v1 || (dat_172330))))   // NOT re-rolled
    v4 = 1;
  else {
    v4 = 0;
  }
  initialize_shell_variables(shell_environment,v4);
  initialize_job_control(jobs_m_flag);
  initialize_bash_input();
  initialize_flags();
  v2 = (((privileged_mode) || (restricted)) || ((v1 || (dat_172330))));   // re-rolled
  initialize_shell_options(v2);
  if (((privileged_mode) || (restricted)) || ((v1 || (dat_172330))))   // NOT re-rolled
    v2 = 1;
  else {
    v2 = 0;
  }
  initialize_bashopts(v2);
```

Header says it plainly: `// warn: iteboolean: re-rolled 1 0/1 select diamond(s)`.

`--mode reliable` (the option surface the benchmark measured) produces the identical
text, so this is a live code gap, not a mode artifact.

**Option sweep** (all re-run on the stripped binary at `0x34684`): `condfold on`,
`condfold wide`, `iteexpr on`, `iteregion off`, `branchflip off`, `gotoreduce off`,
`taildup off`, `dedupitetail off`, `crossjumprevert off`, `ifelseflatten off`,
`truthycond off`, `braceelide off`, `returndup off`, `earlyreturn off`,
`foldcallret off`, `condexeplace off`, `loopbreak_recovery off` — **none** changes the
count. `regionstructure off` changes *which* diamond is re-rolled (2 instead of 1, and
a different two), which is the first clue.

## Source

`~/github/decbench/results/full_run/O0/bash/compiled/shell.i`:

```c
  should_be_restricted = shell_is_restricted (shell_name);
  initialize_shell_variables (shell_environment,
       privileged_mode||restricted||should_be_restricted||running_setuid);
  initialize_job_control (jobs_m_flag);
  initialize_bash_input ();
  initialize_flags ();
  initialize_shell_options (privileged_mode||restricted||should_be_restricted||running_setuid);
  initialize_bashopts     (privileged_mode||restricted||should_be_restricted||running_setuid);
```

Three occurrences of the identical 4-clause `||` chain, all in argument position.
The correct shape is one boolean expression per call — which is what `iteboolean`
(PR #241) exists to produce and what it produces for exactly one of the three.

## Analysis

### Symptom (one, named)

**In a run of consecutive assignment diamonds, the P8 diamond matcher declines every
other one.** The declined diamonds stay as `if (c) x = 1; else { x = 0; }` — 3 extra
CFG blocks and 4 extra edges each against both the source and IDA.

### Reduction: a 3-line reproducer

`gcc -O0 -shared -fPIC`, decompiled with today's `kuna decompile`:

```c
int a,b,c,d;
void g(void); void f1(int);void f2(int);void f3(int);
void t(void){ g(); f1(a||b||c||d); g(); f2(a||b||c||d); g(); f3(a||b||c||d); }
```

```
void t(void) // warn: iteboolean: re-rolled 2 0/1 select diamond(s) ...
  v1 = (((*dat_3ff0) || (*dat_3fc0)) || ((*dat_3fd8 || (*dat_3fe8))));
  f1(v1);
  g();
  if (((*dat_3ff0) || (*dat_3fc0)) || ((*dat_3fd8 || (*dat_3fe8))))   <-- declined
    v1 = 1;
  else { v1 = 0; }
  f2(v1);
  g();
  v1 = (((*dat_3ff0) || (*dat_3fc0)) || ((*dat_3fd8 || (*dat_3fe8))));
  f3(v1);
```

Scaling N (same generator): re-rolled = 1, 1, 2, 2, 3, 3 for N = 1…6 — **strict
even-index decline**. The raw CFG (`decomp_dbg` → `print raw`) is a perfectly
symmetric chain of three identical diamonds (blocks 4/5→6, 10/11→12, 16/17→18), so
nothing distinguishes them before structuring.

### The defect is NOT in `iteboolean` — it is in the shared matcher

`iteregion` (P8, default-on, DIV-17, a *different* pass with a *different* mark)
shows the identical alternation on a plain ternary chain:

```c
int a,p,q; void g(void); void f1(int);void f2(int);void f3(int);
void t(void){ int x; g(); if(a) x=p; else x=q; f1(x);
                     g(); if(a) x=p; else x=q; f2(x);
                     g(); if(a) x=p; else x=q; f3(x); }
```
```
void t(void) // ternary x2
  v1 = (*dat_3fe8) ? *dat_3fc8 : *dat_3ff0;
  ...
  if (*dat_3fe8)          <-- declined
    v1 = *dat_3fc8;
  else { v1 = *dat_3ff0; }
  ...
  v1 = (*dat_3fe8) ? *dat_3fc8 : *dat_3ff0;
```

Both passes share `match_*`'s preconditions and the leaf helpers in
`p8_structure/kuna_iteregion.rs` (`leaf_bblock`, `printed_ops`, `same_storage`), so
the failing predicate is in the shared half, not in `iteboolean`'s `BlockCondition`
requirement. It is also independent of the structurer: `--option regionstructure off`
alternates too (it just alternates on a different phase).

### The trigger, isolated

Three one-diamond probes, identical except for what precedes the diamond:

| probe | preceding code | ternary fires? |
|---|---|---|
| `v_pre_str` | `g(); h();` (straight-line, same basic block) | **yes** |
| `v_pre_loop` | `while(b) g();` | **yes** |
| `v_pre_if` | `if(a) g();` | **no** |
| `v_pre_if2` | `if(a) g(); h();` | **no** |
| `v_pre_ifelse` | `if(a) g(); else h();` | **no** |
| `v_tight` | a preceding *diamond*, nothing between | **no** (2nd declines) |

So: **an immediately preceding structured `if` statement suppresses the rewrite of
the very next diamond.** A chain of diamonds is exactly that situation repeated,
which produces the parity.

That is the signature of `leaf_bblock` (`kuna_iteregion.rs:232`) declining a
multi-component list:

```rust
BlockType::Ls | BlockType::Graph => {
    if blk.get_size() != 1 { return None; }   // <-- declines a 2+ component list
    leaf_bblock(data, blk.get_block(0))
}
```

When the preceding `if` collapses first, the block-concatenation rule folds it
together with the following condition block into a `BlockList` of two components,
and that list becomes component 0 of the next `BlockIf`. `iteregion`'s `cond_cbranch`
and `iteboolean`'s `cond_terminal_cbranch` both bottom out in `leaf_bblock`, get
`None`, and decline — even though the printer renders that same list perfectly well
(the leading component is printed as a statement and the trailing block's CBRANCH
becomes the `if` header; you can see it in `v_tight`, where diamond 1's ternary is
printed on the line *above* diamond 2's `if`).

Two honest caveats, both worth re-deriving at fix time rather than trusting:

1. I could not print the `sblocks` tree to read the node types directly —
   **`print tree block` and `structure blocks` in `decomp_dbg` are unported stubs**
   (`engine integration not yet ported: Funcdata::printBlockTree`). That is itself a
   tooling gap for P8 work. The list-shape conclusion is inferred from the
   preceding-statement probe table plus the printed nesting, not read off the tree.
2. The exact parity in longer chains (why diamond 3 recovers) depends on collapse
   bookkeeping I did not instrument. It does not change the symptom or the owning
   predicate.

### Owning phase

**P8** — Structured AST & Goto Quality (`decompiler/crates/kuna-decomp/src/p8_structure/`).

### Breadth

`kuna decompile-all --json` over four O0 corpus binaries (2,444 emitted functions),
counting residual `if (c) V = A; else { V = B; }` diamonds:

| binary | fns | residual 0/1 diamonds | in fns | residual any-arm diamonds | in fns | `iteboolean` successes |
|---|---|---|---|---|---|---|
| coreutils `ls` | 617 | 3 | 3 | 34 | 14 | 26 |
| diffutils `diff` | 532 | 32 | 14 | 62 | 23 | 24 |
| gzip `gzip` | 263 | 0 | 0 | 0 | 0 | 2 |
| openssh `ssh-add` | 1032 | 6 | 6 | 16 | 15 | 1 |
| **total** | **2444** | **41** | **23** | **112** | **52** | **53** |

41 residual 0/1 diamonds against 53 successful re-rolls: **kuna leaves behind roughly
as many as it fixes.** The wider `any-arm` count (112 in 52 functions) is the
`iteregion` half of the same predicate.

### Siblings

`O2-noinline-bash-bash-shell_initialize` (margin 72) does **not** show this: at -O2
gcc computes the boolean without a materialization diamond and kuna prints
`initialize_shell_options(((privileged_mode || restricted) || dat_1409d4) || v1);`
directly. The defect is an -O0 shape, which is where it matters most (the corpus is
one third -O0 and -O0 materializes booleans everywhere).

## Proposed fix

**Slug `itecondlist`, P8, one module** —
`decompiler/crates/kuna-decomp/src/p8_structure/kuna_iteregion.rs`.

Mechanism: in the **condition position only**, let the matcher descend a
multi-component `Ls`/`Graph` to its **last** component and take that as the condition
leaf. Concretely, add a `cond_leaf_bblock` next to `leaf_bblock` that on
`BlockType::Ls | BlockType::Graph` recurses into `get_block(size-1)` instead of
bailing, and call it from `cond_cbranch` (`kuna_iteregion.rs:260`) and
`cond_terminal_cbranch` (`kuna_iteboolean.rs:265`). The **arms** keep the strict
`leaf_bblock` (an arm must remain a single statement — that is what makes the
rewrite sound). `iteboolean`'s `BlockType::Condition` requirement then applies to
that same tail node.

Why this is safe: the printer already emits the leading components of the list as
ordinary statements *before* the `if` header — the shape is visible in `v_tight`
above — so folding only the tail changes nothing about what precedes it. It is the
same tolerance `cond_cbranch` already documents for leading statements *inside* one
basic block ("Leading statements are allowed … the printer emits them normally
before the ternary"); this is that same rule spelled across a block boundary.

Guards to keep: `f_unstructured_targ` on the descended leaf (a labelled goto target
must still decline), and `printed_ops(...).last()` must still be the CBRANCH.

Gating: `iteregion`/`iteboolean` are already option-gated and default-on, and this is
a widening of an existing matcher, not new behaviour — but it *does* change emitted C,
so it needs its own named option per the standing rules, plus a two-pass
`tests/stages/` testcase built from the `v_tight` reproducer (option off = the second
diamond stays an `if/else`, default = both fold), and the usual datatest ablation.

Risk: the shipped `iteregion`/`iteboolean` ablations are 0/675; widening the match set
can move that, so the ablation has to be re-measured before any default-on claim.
