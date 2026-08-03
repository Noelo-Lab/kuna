## What

kuna re-rolls only **every other** ternary in a chain. In a run of N structurally
identical two-arm assignment diamonds, `iteregion`/`iteboolean` recover exactly
`ceil(N/2)` of them; the rest print as `if (c) V = A; else { V = B; }`. Which ones
are missed depends on the *neighbouring* structure, not on the diamond itself.

`option itecondlist on|off` (default **ON**, DIV-56) closes it: in the **condition
position only**, the two matchers now descend a multi-component `BlockList` to its
**last** component before applying their usual leaf test.

Analysis: [`docs/features/itecondlist/analysis.md`](docs/features/itecondlist/analysis.md).
Design + the option decision: [`plan.md`](docs/features/itecondlist/plan.md).

## The driving case — `decbench:O0-bash-bash-shell_initialize` (IDA pool, margin 105)

The source has three occurrences of one identical 4-clause `||` chain, all in
argument position. Before, 1 of 3 re-rolled:

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

After, 3 of 3, statement-for-statement with the source:

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

**GED 46 -> 24 (-48%)**, `~/.virtualenvs/decbench/bin/python -m scripts.decbench.rescore`.
The `O2-noinline` sibling was already GED 0 and is unmoved (at -O2 gcc computes the
boolean without a materialization diamond).

## The N-sweep

`gcc -O0 -c` on N chained identical diamonds in one function:

| N | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|
| ternaries, before | 1 | 1 | 2 | 2 | 3 | 3 | 4 | 4 |
| ternaries, after  | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |

Exactly `ceil(N/2)` becomes exactly `N`.

## Mechanism, and why it is sound

`iteregion` and `iteboolean` both reach the condition component through
`kuna_iteregion.rs (leaf_bblock)`, which descends a `BlockList` of **one**
component but bails on a list of two or more. The collapse structurer concatenates
a just-collapsed predecessor together with the following condition block into a
two-component `BlockList`, and that list becomes component 0 of the *next*
`BlockIf` — so a chain alternates between the two shapes.

Confirmed by instrumenting `match_ite_assignment` on the N = 3 reproducer:

```
node=BlockId(14v1) cond_ty=Copy         cbr=true    armT=true armE=true   <- folds
node=BlockId(16v1) cond_ty=Ls cond_sz=2 cbr=false   armT=true armE=true   <- declines
node=BlockId(12v1) cond_ty=Copy         cbr=true    armT=true armE=true   <- folds
  (all six arms: ops=[CPUI_COPY] dest=stack:-0xc:4)
```

Both arms of the declined diamond pass every other test; the only failing
predicate is the condition-leaf descent.

The rewrite is **print-only**, and the printer already renders this shape:
`PrintC::emit_block_if` emits the condition component twice — once under
`NO_BRANCH` (its statements) and once under `ONLY_BRANCH` (its branch condition) —
and `PrintC::emit_block_ls` emits every component under `NO_BRANCH` but **only the
last** under `ONLY_BRANCH`. The leading components therefore already print exactly
once, ahead of the `if` header, and `emit_block_if_ite`/`emit_block_if_bool` do the
same two emissions on the same block. The arms keep the strict single-statement
match, the labelled-goto-target guard still applies to the descended leaf, and the
match struct keeps the **outer** list as its `cond_block`.

`BlockGraph` is deliberately **excluded** from the descent: `emit_block_graph`
ignores `ONLY_BRANCH` and emits every component, so descending one would make the
emitters print the whole graph twice.

The `iteboolean` descent sits at the `BlockType::Condition` gate, **not** inside
`cond_terminal_cbranch` — that function's leaf arm is only ever reached for the
individual clauses of an already-identified chain, so a descent there can never
fire on the outer list.

## New option, not an unflagged widening

`iteexpr` is the standing precedent: a broadening of the *same* matcher shipped as
its own option (it widens `iteregion`'s **arm** match; this widens the
**condition** match). The new flag is also what buys a two-pass stage test that
pins the *exact* `ceil(N/2)` defect — turning `iteregion` off removes **all**
ternaries, so it cannot express "every other one is missing".
`tests/stages/ghdec-itecondlist.xml` folds 2 of 3 with the option off and 3 of 3
with it on, over the same bytes.

It is a strictly narrower match than the two options it widens, so
`option itecondlist off` can only ever fold **fewer** diamonds, never different
ones, and is byte-identical to the pre-DIV-56 render.

## Breadth + regression sweep (12,241 functions)

`decompile-all --mode reliable`, option off vs on:

| binary | functions | changed | diamonds gained | ternaries | iteboolean |
|---|---|---|---|---|---|
| coreutils `ls` (O0) | 613 | 3 | +5 | 21 -> 24 | 23 -> 24 |
| iproute2 `ip` (O0) | 1,962 | 3 | +3 | 95 -> 97 | 2 -> 2 |
| `bash` (O0) | 3,278 | 15 | +17 | 124 -> 135 | 99 -> 108 |
| betaflight STM32F405 (O2-noinline, ARM) | 6,388 | 32 | +34 | 141 -> 178 | 0 -> 0 |
| **total** | **12,241** | **53 (0.43%)** | **+59** | **381 -> 434** | **124 -> 134** |

That is ~5% of the residual declined diamonds in the corpus; the rest are the
separate `same_storage` defect and the arm-shape declines `iteexpr` covers.

Every diff hunk was audited **mechanically**
([`audit.py`](docs/features/itecondlist/audit.py)): each must be either a header
comment whose ternary/re-roll count went *up*, or a removed
`if (C) V = A; else { V = B; }` replaced by exactly one added
`V = (C) ? A : B;` / `V = (C);` / `V = !(C);` with the **same** destination,
**same** condition and **same** arm expressions in order.

**52 header-only hunks, 59 folded diamonds, 0 problems.** The two hunks the audit
could not classify automatically were hand-checked and are benign (a `bash`
comma-expression condition the arm regex cannot parse; and one function whose two
accumulated `iteboolean` warning lines collapse into one with a byte-identical
body). Error/diagnostic line counts are unchanged in every binary — no function
starts or stops producing output.

## Ablation, speed, gates

**Ablation with the option default-ON: 0/675 datatest assertions change.** No
re-pin of `docs/baseline.json`, and no per-test opt-out was needed anywhere.

Speed (`scripts.pipeline.timeit`, target function, 7 samples):
**356.33 -> 354.28 ms, -0.58%** — inside the +5% budget. Interleaved whole-binary
medians of 5 pairs (`decompile-all --mode reliable`): coreutils `ls`
7,193 -> 7,220 ms (+0.38%), gzip 2,881 -> 2,873 ms (-0.28%). The predicate is one
extra pointer chase per candidate `BlockIf` and mutates no p-code.

| Gate | Result |
|---|---|
| `make test` | `datatests: 675/675 assertions passed` — **PARITY OK** |
| `make test-stages` | `datatests: 381/381 assertions passed` — **PARITY OK** |
| `make rust-test` | `passed=4424 failed=0 ignored=37`, exit 0 |
| `make check-spec` | `check-spec OK (lenient mode)` / `check-spec OK (strict mode)` |
| `kuna catalog --check` | `catalog OK: documents exactly the registered kuna options` |

## Not fixed here (deliberately)

The `same_storage` decline — two arms writing different *raw* storage (a register
vs a `unique`, or a `unique` vs a stack slot) that render as one variable. That is
a second, larger defect; it is what blocks
`O2-noinline-betaflight-...-mavlinkSendRCChannelsAndRSSI` (4 of its 6 misses) and
most register-resident diamonds. One PR, one feature.

## Rebase note (DIV-55 -> DIV-56)

Rebased onto #252 (`1b6d4db4`, DIV-55, `returncopysplit`), which was dispatched
concurrently and merged first — DIV numbers are claimed on **merge**, so this row
is **DIV-56**. Both rows are kept; they are unrelated mechanisms.

Shared counters resolved on the merged base: `xml.rs` corpus count **189** (both
PRs moved 187 -> 188), and `docs/baseline-stages.json` **re-recorded** rather than
hand-merged (381 = 369 + 6 for #252 + 6 for this PR; the auto-merge had left a
stale `375` in `data_footer`). The catalog fixture was re-captured on the rebased
tree and is byte-identical — #252 adds no option, so settables stay at 86.

**Every number above was re-measured on the rebased tree, not carried forward.**
#252 changes emitted C, so the OFF arm of all four sweep binaries was regenerated
and diffed against the pre-rebase OFF arm: **0 lines differ in any of them**
(#252's witnesses are `O2-noinline` `ip`/`sshd`; this sweep's `ip` is `O0`). The
breadth table, the 53 changed functions, the N-sweep and the 0-problem mechanical
audit all reproduce exactly; the ablation is 0/675 again with the option
default-ON; GED re-measures at **46 -> 24** on the rebased commit. Only the
timings moved, as expected on a shared box.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
