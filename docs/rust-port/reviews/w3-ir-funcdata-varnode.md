# w3-ir-funcdata-varnode
verdict: ACCEPT-WITH-LOSSES
verifier: claude (independent verifier subagent)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> ok (122 in-tree + verifier suites, all green; clippy `-p kuna-decomp -D warnings` clean)

## Scope

C++ in scope: `decompiler/cpp/funcdata_varnode.cc` (blob `7be1bf1b61a3b9252d5d7de1295728ac0ea9e522`, confirmed
matches the checklist `cpp_blob_sha` via `git rev-parse HEAD:decompiler/cpp/funcdata_varnode.cc`).
Rust under review: `rust/crates/kuna-decomp/src/funcdata_varnode.rs` (the only item file in the
diff `rust-port...worktree-wf_bb220272-745-8` for this id; the branch stacks sibling items
`block.rs`/`funcdata.rs`/`funcdata_block.rs`, which are out of this item's scope, and the
bank/op helpers it leans on live in the pre-existing `varnode.rs`/`op.rs`).

The `funcdata_varnode.cc` file has ~50 methods; this item ports the **IR-surface-portable subset**
and defers the rest (W4 symbol/scope/fspec, W6 types, W7 highvar/merge, funcdata_op op-graph, and
the AncestorRealistic param-trial machinery). The cut is honest: deferred methods are **not**
transcribed (no wrong-value stubs), and the module doc precisely names the two missing public
surfaces (`Funcdata::banks_mut()`, `Varnode::set_flags`). The deferral is recorded as **LOSS-036**.

## Hunt list

- **Signedness** — clean. `int4` sizes/offsets, `int4` slots, `int4` overlap returns are all
  carried as `int4` (i32); the one signed/unsigned interaction is `vsize as i64` widening for the
  `Address + i64` probe (lossless, sign-preserving), matching the C++ `int4`→`int8` promotion.
  No `int4`-vs-`size()` loop-index comparisons in the ported subset.
- **Integer widths** — clean. `newVarnodeIop`/`newVarnodeSpace`/`newVarnodeCallSpecs` map
  `sizeof(ptr)`→8 (documented). The iop encode stores a full `u64` slotmap key with no truncation
  (`Address::new` stores the offset raw, matching the C++ `Address(AddrSpace*,uintb)` ctor).
- **Wrapping** — clean. The probe `&vaddr + (vsize as i64)` routes through `Address::operator+`,
  which wraps within the space (`wrap_offset(offset.wadd(off))`) exactly as C++; `last_input_before`
  then compares against the same wrapped probe, so the two stay consistent.
- **Comparator totality** — N/A in this file; the def-tree ordering it relies on
  (`VarnodeCompareDefLoc`, input<written<free) is `varnode.rs`'s and total (verified there). The
  ported finders only *consume* that order.
- **Iteration-order provenance** — every loop accounted for. `descend_snapshot`/`total_replace`/
  `destroy_varnode` iterate the `descend` SmallVec in push order == C++ `list<PcodeOp*>` order.
  `last_input_before` iterates `iter_def_flag(input)`, a `BTreeMap::range` over the **whole** input
  class in def-tree sorted order (begin..end_def_flag(input) covers all inputs), so the early
  `break` on the first input `addr >= probe` is sound. No `HashMap`/`HashSet` anywhere.
- **Off-by-one / do-while / reverse iterators** — the C++ `beginDef(input, addr); --iter` step-back
  is reproduced by `last_input_before` (greatest input strictly below the probe), with the
  `iter != beginDef()` guard reproduced as "no input below probe -> None". Re-derived for the
  empty-input, probe-at-first-input, same-address-multiple-sizes, and overlap-error cases (V1).
- **Erase-while-iterating** — `total_replace` (`op = *iter++; opSetInput(...)`) is ported as
  snapshot-then-iterate. Re-derived to be order-equivalent: for a double-reader op the descend list
  holds the op twice, `getSlot` returns the FIRST still-matching slot and `eraseDescend` removes the
  FIRST matching entry, so the two iterations repoint slot 0 then slot 1 — confirmed by V2 against
  the C++ oracle. `vn`'s descend list only shrinks during the loop, so the snapshot is exact.
- **Exception -> Result partial-state parity** — `find_input_overlap` returns `Err` (the
  `LowlevelError("Overlapping input varnodes")`) **before** any mutation, so there is no partial
  state to match (the C++ throws at the same pre-mutation point). `destroy_varnode`/`delete_varnode`
  propagate the bank's "integrated/deleting" errors with no intervening mutation past the C++ catch
  points.
- **Aliasing** — the methods that need simultaneous `&mut vbank` + `&mut obank` (the xref-callback
  tail of `setInputVarnode`/`newVarnodeOut`/`newUniqueOut`) are correctly *deferred*, not faked.
  The ported `op_set_input`/`destroy_varnode`/`total_replace` decompose into strictly sequential
  single-arena borrows, matching the C++ effect order.
- **SeqNum/create_index allocation-order fidelity (ADR 0001)** — preserved. Every factory goes
  straight to the bank's single `create_index++` path in C++ statement order (`new_constant`,
  `new_unique`, the three `new_varnode` overloads, `new_varnode_iop/space/call_specs`); verified by
  the in-tree `new_unique_advances_uniq_offset_and_create_index` (0 then 1) and re-confirmed by
  reading `VarnodeBank::create`/`create_unique`.
- **Intrusive-list splice-order fidelity (ADR 0001)** — descend list manipulation
  (`add_descend` push_back, `erase_descend` first-match removal, `destroy_descend` clear) is faithful
  to `varnode.cc:316/330`; the `op_set_input` "addDescend BEFORE setInput" ordering matches
  `funcdata_op.cc:123-124`.

## Mechanical pass

- `cargo run -p port-audit -- --item w3-ir-funcdata-varnode` -> tool **not present** in the
  workspace (the verification.md reference is aspirational). Blob-sha drift checked manually:
  scope blob matches the recorded sha (above).
- grep `funcdata_varnode.rs` for `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/
  `panic!`/`unreachable!` -> **none**.
- bare `as` casts -> one (`vsize as i64`, funcdata_varnode.rs:360). Faithful i32→i64 widening for
  `Address + i64`; lossless and sign-preserving. **Minor (non-blocking):** lacks a `// cast:`
  justification comment that the protocol requests. Recorded as F1, not a divergence.
- `cargo clippy -p kuna-decomp -- -D warnings` -> clean.

## Findings

- F1 (minor / nit): bare `as` cast without a `// cast:` comment.
     cpp: decompiler/cpp/funcdata_varnode.cc:350 (`vn->getAddr()+vn->getSize()`, int4→int8)
     rust: rust/crates/kuna-decomp/src/funcdata_varnode.rs:360 (`&vaddr + (vsize as i64)`)
     Behavior is correct and matches C++; only the documentation convention is unmet. Non-blocking.

No blockers or majors. Re-derivation found the ported logic faithful on every hunt-list axis,
including the two fragile spots (the `--iter` step-back and the double-reader `totalReplace`).

## Adversarial tests

Added `rust/crates/kuna-decomp/tests/verify_w3_ir_funcdata_varnode.rs` (9 tests, committed to the
branch as `94c95ba`), each re-derived from the C++ oracle:
  - V1 `find_input_overlap_*` (5): `--iter` step-back picks the immediately-preceding input;
    no-inputs-but-written -> None; probe at/below first input start -> None; overlapping-not-identical
    -> Err; same-address-multiple-sizes picks the largest.
  - V2 `total_replace_double_reader_repoints_both_slots_in_order` + `_empty_descend_is_noop`: the
    one-op-two-slot descend/getSlot/eraseDescend interplay and the empty-list no-op.
  - V3 `destroy_varnode_written_clears_def_output_and_readers` + `_input_no_def_unhooks_and_frees`:
    the def/descend teardown for written and input varnodes.
All 9 pass; full `cargo test -p kuna-decomp` remains green.

## Losses

- LOSS-036 (appended this review): the deferred remainder of `funcdata_varnode.cc` — (a) the
  xref-callback tail / op-creation methods needing `funcdata_op` + a `banks_mut` split accessor
  (the same gap LOSS-035 records from the funcdata_op side), (b) the W4 symbol/scope/fspec family,
  (c) the W7 HighVariable/nzmask family, (d) the W4 param-trial / AncestorRealistic family, plus
  `newCodeRef` (blocked only by `Varnode::set_flags` privacy). The ported subset is the verified
  delivered scope; no ported method depends on a deferred one's non-trivial result.

verdict: **ACCEPT-WITH-LOSSES** (cites LOSS-036). The delivered varnode life-cycle subset is a
faithful, total, allocation-order-preserving port; the cut to later waves is honest and ledgered.
