# outline — excise a region into a synthesized pseudofunction

Stage 2 of inline identification. Stage 1 (`almostregion`,
`docs/features/almostregion/`) **finds** candidate inlined regions; this
**acts** on one.

## Why the two are separate options

Acting is destructive and finding is not — but the deciding reason is that the
choice of what to outline does not have to come from stage 1. The region is
supplied as the option value:

```
--option outline 0x401000:0x40100a-0x401018
         ^fn entry ^head    ^exit
```

so a human reading the code, or an LLM agent reading stage 1's report, drives the
transform directly. The function entry is part of the grammar because an option
value is global to the run: it is applied once and every function is then
decompiled under it.

**The region is re-derived, never trusted.** The value names only a head and an
exit; the member set is recomputed from the live `bblocks` with the same
single-entry predicate stage 1 uses (`kuna_check_region`), so both stages agree
on what a region is and a stale or hand-written pair cannot excise something that
is not a region. This follows from stage 1's finding that a region is a block
*set* — an inlined callee is laid out in several disjoint pieces, so an address
interval could not have named it.

## Result on the witness

```c
// off (default)                      // --option outline 0x401000:0x40100a-0x401018
if (a1) {                             if (a1)
  v1 = a0 + 7;                          a0 = outlined_0x40100a(a0);
  if (0x33 <= v1)                     return a0 + 100;
    v1 = a0 + 4;
  a0 = v1 * 3;
}
return a0 + 100;
```

Three blocks excised; the live-in `EAX` recovered as the argument, the live-out
`EAX` as the result.

## The three liveness corrections

There is no pre-SSA liveness API, so the pass computes its own. The skeleton is
easy; these three are what actually made it work, and each was found by the
transform silently declining:

1. **Direction.** Counting *any* read outside the region as a live-out flags the
   entry block's own `test`/`je` flag pair, which happens *before* the region.
   That rejected every region. Liveness is computed over the region's
   **continuation** — blocks reachable from the exit without re-entering.
2. **Sub-register aliasing.** An x86 32-bit write zero-extends, so SLEIGH writes
   both `EAX` (4 bytes) and `RAX` (8 bytes) at the same offset. Keyed on
   `(space, offset, size)` that is two live-outs for one register, and v1 declines
   above one. Overlapping storage in a space is merged, keeping the widest.
3. **Branch operands.** Slot 0 of a control-transfer op is its *destination*,
   carried as a varnode in the code space. It is an address, not a value; counting
   it makes every conditional look like it reads memory (`ram+0x401015:1` showed up
   as a live-in).

Liveness remains an over-approximation in the safe direction — over-counting can
only cause extra declines, never a wrong excision.

## Two primitives that do not do what their names suggest

Both cost real debugging time and are recorded in `docs/spec/08-structuring.md`
§8.6 so the next edit of this kind does not repeat them:

- **`Funcdata::node_split_block_edge`** reads as "insert a block on this edge" and
  is not. It `copy_basic_range`s the target's p-code into the new block and gives
  it the target's successors, because it exists for tail duplication. Used here it
  produced a clone of the region head whose out-edges pointed back into the region,
  and heritage failed with `non-root block has no idom`. The correct sequence is
  that function minus the copy: `new_block_basic` + `switch_edge` + `add_edge`, and
  then **no** `remove_branch`/`push_branch` — with the entry edge moved the region
  is unreachable and the sweep collects it, matching `kuna_stackguard.rs`.
- **A hand-built `FuncCallSpecs` has no prototype store.** Every later
  `FuncProto::store` query panics on the null. Decoder-produced specs get theirs
  from `ActionDefaultParams`; a synthesized one must call
  `proto_mut().attach_internal_store(void_ty)`.

## v1 restrictions

Declines, leaving the function untouched, unless: the head/exit really bound a
single-entry region over the live CFG; exactly one edge enters the head from
outside; at most one value is live out; and the region contains no call and no
`STORE`. Excising a region with an unfound live-out silently produces C that
computes the wrong value — the worst failure available here — so v1 refuses rather
than guesses.

## What it does not do

It does not emit a **body** for the pseudofunction. There is no seam:
`PrintC::doc_function_full` is one `Funcdata` per document. The call is emitted and
named; the bytes remain in the binary and can be decompiled separately at the head
address. Nor does it choose regions — that is stage 1's job, or the operator's.

## Risk

`destructive = true`, default `off`, and inert for any function the supplied
region does not name. Standing requirement 7 (whole-corpus sweep of changed
functions) reduces to zero changed functions with no region supplied; both parity
gates confirm it.
