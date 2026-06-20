# ADR 0001: IR storage — slotmap generational arenas owned by Funcdata

## Context

The C++ IR is a pointer web: `Varnode*`/`PcodeOp*`/`FlowBlock*` cross-reference
each other freely, each `PcodeOp` sits on up to three intrusive `std::list`s at
once (its basic-block position, the global alive/dead list, and the per-opcode
dispatch list in `ActionPool`), and `Varnode::descend` is a by-value list of
consumer ops. Rust cannot express this with references, and `Rc<RefCell<...>>`
would scatter borrow panics and aliasing hazards through 100k+ lines of ported
mutation code.

## Decision

`Funcdata` owns three slotmap generational arenas, keyed by newtypes
`VarnodeId`, `OpId`, `BlockId`. No `Rc<RefCell>` anywhere in the IR. The three
`std::list` memberships become intrusive prev/next **ID** links stored on the
node itself (block position, alive/dead, per-opcode), so splice/erase keep C++
iterator-stability semantics. `descend` is a `SmallVec<OpId>` with the C++
append/remove-by-value semantics transcribed (order matters: it is observable
through iteration). ALL graph mutation goes through `Funcdata` methods named
after the C++ API (`op_set_input`, `op_set_opcode`, `op_insert_before`, ...);
those methods are also the single place where per-stage artifact version
counters bump (ADR 0007).

## Consequences

- Generational keys make stale handles a caught error (panic per ADR 0004's
  internal-invariant rule), not a use-after-free.
- `Funcdata` snapshots are plain `Clone` — keys stay valid across the copy —
  which ADR 0007's rewind design relies on.
- Every C++ call site `vn->beginDescend()` style maps mechanically to
  `fd.descend(vn_id)`, keeping the port reviewable line-against-line.
- Cost: all mutation funnels through `&mut Funcdata`, so helper code holds IDs,
  not borrows; occasional two-phase (collect-then-mutate) rewrites are needed.
