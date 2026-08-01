---
case_id: O0-bash-bash-rl_vi_redo
pool: angr
status: covered-by-option
tier: M
margin: 40
fresh_verdict: not fixed — today's build still scores GED 40 (43 nodes/65 edges vs the source's 33/50), identical in `--mode auto` and `--mode reliable`; `--option branchflip off` makes kuna's CFG *isomorphic* to the source (GED 0, angr parity)
option_closing: branchflip (set to `off`)
feature_slug: branchflip-leaf-only
scope: small
confidence: high
---

## Side-by-side

All three panes below are the same 6-arm dispatch cascade. angr and kuna have
**identical** structural metrics (0 gotos, 0 labels, 12 `if`s) — the difference
is entirely the *guard form*.

**angr (stored) — GED 0, isomorphic to the source**

```c
    if (_rl_vi_last_command == 105 && g_57bd80 && g_57bd80->field_0)
    {
        sub_4f9abd(a0);
        if (rl_point > 0)
            sub_4fa13d();
    }
    else if (_rl_vi_last_command == 82 && g_57bd80 && g_57bd80->field_0)
    { ... }
    else if (_rl_vi_last_command == 73 && g_57bd80 && g_57bd80->field_0)
    { ... }
    ...
    else
    {
        v1 = _rl_dispatch(_rl_vi_last_command, *((long long *)&_rl_keymap));
    }
```

**kuna (fresh, today's default `--mode auto`; `--mode reliable` is byte-identical) — GED 40**

```c
  if (((_rl_vi_last_command != 0x69) || (!dat_17bd80)) || (!*dat_17bd80)) {
    if (((_rl_vi_last_command != 0x52) || (!dat_17bd80)) || (!*dat_17bd80)) {
      if (((_rl_vi_last_command != 0x49) || (!dat_17bd80)) || (!*dat_17bd80)) {
        if (((_rl_vi_last_command != 0x61) || (!dat_17bd80)) || (!*dat_17bd80)) {
          if (((_rl_vi_last_command != 0x41) || (!dat_17bd80)) || (!*dat_17bd80)) {
            if ((_rl_vi_last_command != 0x2e) || (_rl_keymap != vi_movement_keymap))
              v1 = _rl_dispatch(_rl_vi_last_command,_rl_keymap);
            else { rl_ding(); v1 = 0; }
          }
          else { rl_end_of_line(1,0x41); sub_f9abd(a0); if (1 <= rl_point) sub_fa13d(); }
        }
        ...
```

Every source arm `X == K && p && *p` has been De Morgan'd into
`X != K || !p || !*p` **and the arms swapped**, so the cascade continues in the
*then* branch and nests six deep instead of chaining as `else if`.

**kuna, same build, `--option branchflip off` — GED 0, isomorphic**

```c
  if (((_rl_vi_last_command == 0x69) && (dat_17bd80)) && (*dat_17bd80)) {
    sub_f9abd(a0);
    if (1 <= rl_point)
      sub_fa13d();
  }
  else if (((_rl_vi_last_command == 0x52) && (dat_17bd80)) && (*dat_17bd80)) {
    sub_f9a5b(a0);
    if (1 <= rl_point)
      sub_fa13d();
  }
  ...
  else if ((_rl_vi_last_command == 0x2e) && (_rl_keymap == vi_movement_keymap)) {
    rl_ding();
    v1 = 0;
  }
  else {
    v1 = _rl_dispatch(_rl_vi_last_command,_rl_keymap);
  }
```

For reference IDA scores 20, worse than angr and better than kuna, and it gets
there by planting `LABEL_27` + 4 `goto`s into the shared `_rl_vi_backup()` tail —
a genuinely worse shape than either. With `branchflip off` kuna beats both.

## Source

`~/github/decbench/results/full_run/O0/bash/compiled/vi_mode.i:7284` (bash
readline `vi_mode.c`, `rl_vi_redo`):

```c
  if (_rl_vi_last_command == 'i' && vi_insert_buffer && *vi_insert_buffer)
    {
      _rl_vi_stuff_insert (count);
      if (rl_point > 0)
        _rl_vi_backup ();
    }
  else if (_rl_vi_last_command == 'R' && vi_insert_buffer && *vi_insert_buffer)
    { ... }
  else if (_rl_vi_last_command == 'I' && vi_insert_buffer && *vi_insert_buffer)
    { rl_beg_of_line (1, 'I'); ... }
  else if (_rl_vi_last_command == 'a' && vi_insert_buffer && *vi_insert_buffer)
    { _rl_vi_append_forward ('a'); ... }
  else if (_rl_vi_last_command == 'A' && vi_insert_buffer && *vi_insert_buffer)
    { rl_end_of_line (1, 'A'); ... }
  else if (_rl_vi_last_command == '.' && _rl_keymap == vi_movement_keymap)
    { rl_ding (); r = 0; }
  else
    r = _rl_dispatch (_rl_vi_last_command, _rl_keymap);
```

Source CFG: 33 nodes / 50 edges. Not degenerate, not approximated — the GED is
an exact edit distance and the metric is telling the truth here.

## Analysis

**Verification method.** decbench's GED was replicated locally
(`decbench.utils.cfg.extract_cfgs_from_source` + `decbench.metrics.ged._is_isomorphic`
+ `decbench.metrics.vj_ged`, run under `~/.virtualenvs/decbench`). It reproduces
all three recorded values exactly — angr 0 (isomorphic), ida 20, kuna 40 — so
every number below is the real metric, not an estimate.

| pane | decompiled CFG | GED |
|---|---|---|
| source | 33n / 50e | — |
| angr (stored) | 33n / 50e | 0 (isomorphic) |
| ida (stored) | 29n / 46e | 20 |
| kuna (stored, 2026-07-27) | 43n / 65e | 40 |
| **kuna (fresh, today, `--mode auto`)** | **43n / 65e** | **40** |
| kuna (fresh, `--mode reliable`) | identical text | 40 |
| **kuna (fresh, `--option branchflip off`)** | **33n / 50e** | **0 (isomorphic)** |

**The one structural symptom.** Every arm of a 6-way `else if` cascade whose
guard is a conjunction led by an equality (`_rl_vi_last_command == 'i' &&
vi_insert_buffer && *vi_insert_buffer`) is emitted as its De Morgan complement
with the arms swapped — `((cmd != 0x69) || (!p)) || (!*p)` with the *rest of the
cascade* in the then-arm — turning the source's flat 7-arm chain into a
six-deep right-nested if/else ladder.

The nesting itself is metric-neutral; the *condition form* is the whole 40.
Two hand-built controls settle it: rewriting kuna's output with positive `&&`
guards and swapped arms scores **0** whether it is written as a flat `else if`
chain **or** kept as literal nested `else { if ... }` blocks
(both 33n/50e, isomorphic). The cost is per-operand: joern gives a bare truthy
identifier (`vi_insert_buffer`) no CFG node, while its negation (`!dat_17bd80`)
is an operation and gets one — and losing that node also loses an edge merge.
That is +2 nodes / +3 edges per arm × the 5 pointer arms = the exact +10 nodes /
+15 edges observed.

**Root cause: kuna's own `branchflip` option over-fires on short-circuit guards.**
`--option branchflip off` on the same binary, same build, restores the positive
conjunction and takes the case to GED 0. The mechanism, read in
`decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs`:

* `Funcdata::block_if_flip_negated_guard` (~line 3588) takes a 3-component
  `BlockIf`, resolves its guard to a `SplitPoint` via `get_split_point`, and
  flips whenever `split_flip_in_place_test` returns `1` (the
  "negated / equality-to-zero" class that `preferComplement`, which only takes
  `0`, declines).
* `SplitPoint` has **two** variants: `Basic(bb)` — one leaf `CBRANCH` — and
  `Condition(scond)` — a short-circuit `BlockCondition` built by
  `CollapseStructure::rule_block_or` (`p8_structure/region_structurer.rs::try_block_or`).
  `block_if_flip_negated_guard` accepts both.
* For a `Condition`, `split_flip_in_place_test` returns `subtest1` — the class
  of the **leftmost leaf only** (`op_flip_in_place_test`:
  `INT_EQUAL → 1`, `INT_NOTEQUAL → 0`, `substrate/funcdata_op.rs:2101`).
  So a whole conjunction is judged "a negated guard" purely because its first
  comparison is an `==`.
* `split_flip_in_place_execute` on a `Condition` then calls
  `flip_condition_opcode` on every nested `BlockCondition` and recurses into
  every leaf — i.e. a full De Morgan — and `swap_blocks(sif,1,2)` swaps the arms.

The heuristic is a leaf-level statement about one comparison's polarity; applying
it to a compound short-circuit condition is a category error. `A == K && p && *p`
is *already* the source's positive form; inverting it is a strict divergence.

Direct corroboration that the `Condition` path was never the intended target:
`split_point_addr` returns `None` for `SplitPoint::Condition`, so these flips
emit **no** `branchflip:` warning — the `// branch-flip` marker that makes leaf
flips observable is silently absent on exactly the cases that regress. The spec
(`docs/spec/08-structuring.md:363-385`) likewise describes `branchflip` only as
`if (x == 0) {A} else {B}` → `if (x != 0) {B} else {A}`, never mentioning
`BlockCondition`.

**Owning phase: P8 (structure), `readability-rewrites` subphase** — the
`ActionBranchFlip` scheduled at
`decompiler/crates/kuna-decomp/src/infra/universalaction.rs:740`.

**Minimal reproducer** (this is a general shape, not bash-specific). Compiled
`-O0 -shared -fPIC`, decompiled with today's binary:

```c
int w(int a) { int r = 0; if (a == 3 && buf) { f1(a); r = 1; } else { r = 7; } f3(); return r; }
int y(int a) { int r = 0; if (a != 3 && buf) { f1(a); r = 1; } else { r = 7; } f3(); return r; }
```

* `y` (leading `!=`, leaf class `0`): kuna emits `if ((a0 != 3) && (*dat_3ff0)) {...} else {...}` — correct.
* `w` (leading `==`, leaf class `1`): kuna emits `if ((a0 != 3) || (!*dat_3ff0)) { v1 = 7; } else { f1(3); v1 = 1; }` — De Morgan'd, arms swapped, **no** `// branch-flip` marker.
* `w` with `--option branchflip off`: `if ((a0 == 3) && (*dat_3ff0)) { f1(3); v1 = 1; } else { v1 = 7; }` — the source's form.

**Why the reference wins.** angr's SAILR structurer does no such polarity
rewrite on a compound short-circuit condition; it keeps the conjunction and
orders the cascade positively, which is exactly the source. kuna without
`branchflip` matches it node-for-node.

**Breadth (measured, `man2html` O0, 78 functions, on vs off):** 18 functions
change; 17 of them are the *intended* leaf flips, 1 is a `Condition`-guard De
Morgan. Aggregate disjunctive-guard sites actually go **down** with branchflip on
(29 vs 32) — so this is a targeted over-fire on `Condition` split points, not a
wholesale inversion, and **a blanket default-off is the wrong response**: it
would throw away the DIV-14 leaf-flip win (the `z` control:
`if (!a0) {A} else {B}` → `if (a0) { // branch-flip B} else {A}`).

## Proposed fix

Narrow `branchflip` to leaf guards — decline `SplitPoint::Condition`.

* **Mechanism.** In
  `decompiler/crates/kuna-decomp/src/substrate/funcdata_block.rs::block_if_flip_negated_guard`,
  after `get_split_point`, return `Ok(false)` for `SplitPoint::Condition(_)`.
  One guard clause; `prefer_complement` (upstream Ghidra, class `0`) is untouched
  and keeps its existing `Condition` handling. Equivalent framing: only flip a
  guard for which `split_point_addr` can name an address — the pass's own logging
  contract already draws exactly this line.
* **Owning files.**
  `substrate/funcdata_block.rs` (the guard clause);
  `p8_structure/blockaction.rs (ActionBranchFlip)` (no change expected);
  `decompiler/crates/kuna-decomp/phases.toml` (branchflip row ~2250: `summary` /
  `use_when` / `symptoms` must say leaf-guard-only, and add the
  "negated-disjunction ladder instead of an `else if` cascade" symptom);
  `docs/spec/08-structuring.md` §8.1 (~line 363, state the `Condition` exclusion
  and why); `docs/history.md` (DIV row — this changes a default-ON option's
  emitted C); `tests/stages/` two-pass testcase built on the `w`/`y` reproducer
  above (option off = today's ladder, default = the conjunction).
* **Risks.** Functions where the source genuinely used a *disjunction* led by an
  `==` (`if (x == K || ...)`) currently get flipped into a conjunction and would
  stop being flipped; the man2html sample says the aggregate leans the other way
  (site count 29→32 with branchflip off), so the change needs a decbench
  aggregate GED re-measure before it lands, not just this one case. No datatest
  risk is expected beyond the files DIV-14 already pinned with per-test opt-outs,
  but `make test` / `make test-stages` must confirm.
* **Scope: small.** One guard clause plus the standing docs/test obligations —
  not a proposal-sized feature. It is a scope bug in an existing kuna option, not
  a missing capability.
