# itecondlist — design

## Scope: small

One predicate, in one new module, read by the two existing matchers. No new pass,
no new Action, no structuring-engine edit.

## The option question — new flag, not an unflagged widening

Decided in favour of a **new named option** on three grounds:

1. **Precedent.** `iteexpr` is the standing example of a broadening of the *same*
   matcher shipped as its own option: it widens `iteregion`'s **arm** match
   (single `COPY` to any pure value op). This widens the **condition** match. The
   two are exactly parallel, and the catalog already reads that way.
2. **The standing rule.** "Anything that can change emitted C ships behind a
   named option." This changes emitted C on ~5% of declined diamonds.
3. **The test.** A new flag is what buys the two-pass stage test that pins the
   *exact* `ceil(N/2)` defect. Reusing a pre-existing flag cannot: turning
   `iteregion` off removes **all** ternaries, so pass 1 would assert "no ternaries
   at all", not "every other one missing". `tests/stages/ghdec-itecondlist.xml`
   pins 2-of-3 with the option off and 3-of-3 with it on, over the same bytes.

The alternative framing — "a coverage bug inside the already-gated
`iteregion`/`iteboolean`" — is defensible, and the change *is* a strict widening
(with the option off, output is byte-identical to the shipped behaviour). But the
flag costs one catalog row and buys a sharper test plus a per-run escape hatch,
so it wins.

## Mechanism

`p8_structure/kuna_itecondlist.rs::cond_list_tail(data, id) -> BlockId`

- `option itecondlist off` (or the arch flag clear): return `id` unchanged, so
  every existing decision is bit-identical.
- On: while the block is a `BlockType::Ls` with at least one component, descend to
  its **last** component (bounded at 16 levels).

Two call sites, both in the **condition** position only:

- `kuna_iteregion.rs (cond_cbranch)` — `leaf_bblock(data, cond_list_tail(data, id))`.
- `kuna_iteboolean.rs (match_ite_boolean)` — the descent sits at the
  `BlockType::Condition` gate, and the descended block is what
  `cond_terminal_cbranch` is called on. It must **not** go inside
  `cond_terminal_cbranch`: that function's leaf arm is only ever reached for the
  individual clauses of an already-identified `BlockCondition` chain, so a descent
  there can never fire on the outer list.

`IteAssignMatch::cond_block` / `IteBoolMatch::cond_block` keep the **outer** list,
so the printer still emits the leading components as ordinary statements.

## Why `Ls` and not `Graph`

`PrintC::emit_block_ls` honours `ONLY_BRANCH` by emitting only `list.last()`;
`PrintC::emit_block_graph` ignores it and emits every component. Descending a
`BlockGraph` would therefore make `emit_block_if_ite` print the whole graph twice
(once for the `NO_BRANCH` statements pass, once for the `ONLY_BRANCH` condition
pass). `leaf_bblock`'s existing size-1 `Graph` case is unaffected.

## What is deliberately NOT fixed here

The `same_storage` decline (two arms writing different raw storage — a register
vs a `unique`, or a `unique` vs a stack slot — that render as one variable). That
is a second, larger defect; it is what blocks the mavlink pane and most
register-resident diamonds. One PR, one feature.
