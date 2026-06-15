# w10-const-prop-phi
verdict: ACCEPT
verifier: claude-opus-4-8[1m] (independent verifier)
date: 2026-06-15
round: 1

gate:
  - `cargo test --workspace` (debug) -> GREEN (149 test-result lines, 0 failed; +1
    over the base for the verifier-added `verify_w10_const_prop_phi` binary).
  - `cargo test -p kuna-decomp --test verify_w10_const_prop_phi` -> 6/6 PASS
    (3 `restrictedByConditional` unit tests over real dominator trees + 3 e2e
    real-recovery tests over `condconst.xml`).
  - Byte-parity suites still green: `print_b5_boolless`, `verify_w10_emptyblock_orform`
    (`w10_eob_condconst_conn_forms_andand_byte_identical`, boolless byte-identical),
    `verify_w10_struct_corpus` (readstruct), `verify_w10_dominant_copy`,
    `verify_w10_implied_vars_adversarial`, `verify_w10_stackslot_ssa` -> all PASS.
  - C++ oracle `decomp_test_dbg` -> **207/207 unit + 675/675 datatest = PARITY OK**,
    untouched (the branch only adds Rust + a pre-existing committed C++ anchor).
  - `cargo clippy -p kuna-decomp --lib -- -D warnings` -> CLEAN (fresh, touch-rebuilt).
    The new test file: `cargo clippy --test verify_w10_const_prop_phi -- -D warnings`
    -> CLEAN. (The `--tests`-wide clippy has pre-existing W3-W6 test-file lints, none
    referencing this item's files.)

## What the item is — and the REAL-parity proof

Ports `ActionConditionalConst` (`coreaction.cc:4292-4780`) out of the stubbed
no-op `apply` (which returned `count=0` and changed nothing) into the standalone
`crate::condconst` module: `apply`/`findConstCompare`/`propagateConstant`/
`pushConstant`/`handlePhiNodes`/`placeCopy`/`placeMultipleConstants`/
`collectReachable`/`flowToAlternatePath`/`flowTogether`/`testAlternatePath`, plus
the `FlowBlock::restrictedByConditional` (`block.cc:405`) and
`Funcdata::numHeritagePasses` helpers.

REAL recovery, measured on a fair fresh build of the merge-base binary against the
SAME compiled specs:

| condconst.xml assertion | base (rust-port) | this branch |
|---|---|---|
| #1 `*ptr = b;` (phi recovery) | **FAIL** | **PASS** |
| #3 `ptr[2] = 10;` (phi recovery) | **FAIL** | **PASS** |
| #5 `ptr[4] = 0x10;` (phi recovery) | **FAIL** | **PASS** |
| #2/#4/#10/#11/#12 | PASS | PASS (unregressed) |
| #6-#9 (`glob*` global symbol map) | FAIL | FAIL (pre-existing unported seam) |

The three newly-passing assertions are EXACTLY the named
constant-propagation-through-phi recoveries. The harness applies the stringmatch
regexes against the engine's rendered C, so a pass means the constant COPY was
materialized in the data-flow (`handlePhiNodes`/`placeCopy`), not in the printer.
No assertion that passed on the base regressed. The #6-#9 failures pre-date this
branch (global `map addr` symbol rendering, unrelated to const-prop) and are
identical on base and branch.

## Special-casing audit (priority #2) — CLEAN

`grep -nE '"..."|0x[0-9a-f]{4,}|condconst|glob|iStack|ptr\[|getName|symbol'` over
`condconst.rs` and the diff of `block.rs`/`funcdata.rs`: the ONLY string literals
are `expect()` panic messages; zero hardcoded function names, addresses, magic
values, type names, or symbol-name comparisons. Every branch keys on opcode,
block/edge geometry, dominator relations, and `isConstant()`/`isWritten()` — exactly
as the C++. The recovery is decode-the-algorithm, not decode-one-file.

## Hunt list

- **Signedness**: clean. `int4` slot/input loops (`0..num_input`, `get_slot`),
  `int4 const_edge ∈ {0,1}`, `int4` `results`/`marks` arrays — all match the C++
  `int4` operands; no signed/unsigned cross-compare introduced. `count: int4`
  threaded as the return delta and added to `self.base.count` (the C++ mutates the
  class member and `apply` returns 0; the Rust `Action::perform` reads `base.count`
  identically — action.rs:473-543 mirrors action.cc:300-362).
- **Integer widths**: clean. `value: uintb`=`u64`. `get_size() as usize > size_of::<uintb>()`
  (condconst.rs:365,378) mirrors C++ `getSize() > sizeof(uintb)` (int4 promoted to
  size_t; size non-negative). No `int4*int4->intb` promotion in scope.
- **Wrapping**: clean. No `uintb`-lineage arithmetic in `condconst.rs`; `outval`
  comes from `execute_simple` -> the shared `OpBehavior::evaluate_*` engine (same as
  `RuleCollapseConstants`), which carries its own wrapping. `count += 1` is a small
  `int4` counter bounded by the op set — cannot wrap.
- **Comparator totality**: clean. `PcodeOpNode::less` (op `SeqNum::getTime()`, then
  slot) is a total order; `collect_reachable`'s `sort_by` calls it both directions
  to derive `Ordering`, and `binary_search_edge` uses the SAME `less` consistently —
  faithful to C++ `sort` + `binary_search` over `operator<` (expression.hh:40-48).
- **Iteration-order provenance**: clean. No `HashMap`/`HashSet`. Every loop is over a
  `Vec`/slice (insertion order), a `0..num_input`/`0..size_in` index range, or
  `descend_iter` (a `SmallVec` preserving the C++ `Varnode::descend` `list`
  insertion order, addDescend = push_back — varnode.cc:338). The `phi_node_edges`
  sort in `collect_reachable` reorders the `&mut [PcodeOpNode]` in place; `results`/
  `marks` stay index-aligned to the SORTED list on both sides (handlePhiNodes
  creates `results` BEFORE the sort and indexes by the post-sort `i`, matching C++
  coreaction.cc:4529-4537).
- **Off-by-one / do-while / reverse iterators**: clean, and explicitly tested.
  `restricted_by_conditional`'s inner `while walk != Some(this_id)` faithfully ports
  the C++ `while (inBlock != this)` including the `seenCond` multiple-direct-edge
  guard; the self-loop-header unit test pins the `inBlock == this` zero-iteration
  exit, and the diamond/idom-mismatch tests pin the sibling-walk `return false`.
- **Erase-while-iterating**: clean. C++ `propagateConstant` walks `varVn`'s live
  descend list, pre-advancing the iterator off the current op (which `opSetInput`
  may erase) and skipping consecutive duplicates for multi-slot reads. The Rust
  snapshots the descend list into a `Vec` and skips duplicates
  (`while di < len && descend[di] == op`). Faithful because NO path in
  `propagateConstant` creates a NEW descendant of `varVn` (placeCopy/RETURN-COPY
  read `constVn`, not `varVn`; pushConstant outputs a fresh constant) — so the
  snapshot visits exactly the C++ set in the C++ order. Verified by reading every
  mutation site.
- **Exception -> Result partial-state parity**: clean, one documented benign
  divergence. `execute_simple` returns `None` on eval error == C++ `evalError`
  return. `num_heritage_passes` returns `Err` for a non-heritaged space (==C++
  throw); `apply` maps it via `unwrap_or(0)` -> `numPasses<=0` ->
  `use_multiequal=false`. The C++ throw can only fire if the stack space was never
  registered for heritage, which never happens at this late pipeline stage (the
  guard's real job is the `pass-delay <= 0` not-yet-heritaged case, which both sides
  treat as `use_multiequal=false`). Same observable outcome; see F2.

## Findings

  - F1 (low / advisory): five bare `as` casts in `condconst.rs` (lines 365, 374,
    378, 382, 685) lack the `// cast:` justification comment the protocol prefers.
    All are provably safe: `get_size() as usize` (non-negative size vs `size_of`),
    `i as usize` (`int4` loop index 0..num_input<=3 into `[u64;3]`, matching C++
    `uintb in[3]`), `const_edge as usize` (`int4` ∈ {0,1} into `[bool;2]`). No
    semantic effect. Consistent with sibling W10 `condexe.rs` (5 un-commented sized
    casts) which was already ACCEPTed — convention is advisory, not a REJECT gate.
       cpp: decompiler/cpp/coreaction.cc:4493 / 4496-4499 / 4745
       rust: rust/crates/kuna-decomp/src/condconst.rs:365,374,378,382,685
  - F2 (low / documented): `restricted_by_conditional` adds `None => break` to the
    idom walk where C++ `inBlock = inBlock->getImmedDom()` would null-deref if the
    chain reached the root without hitting `this`. This only differs from C++ in a
    case that crashes C++; for valid `this`-dominated input both return identically.
    Benign defensive hardening.
       cpp: decompiler/cpp/block.cc:419-422
       rust: rust/crates/kuna-decomp/src/block.rs:1300-1308

## Adversarial tests added (committed to the branch, 57f44ab)

`rust/crates/kuna-decomp/tests/verify_w10_const_prop_phi.rs`:
  - `w10cpp_restricted_diamond_merge_is_not_restricted` — sibling-reachable merge
    => false; arms (sizeIn==1) => true.
  - `w10cpp_restricted_self_loop_header_is_restricted` — back-edge == `this`,
    zero-iteration walk => true (do-while at-least-once / `inBlock==this`).
  - `w10cpp_restricted_idom_mismatch_short_circuits_false` — early `idom != cond`
    false; AND the conservative `between -> mid(==cond)` walk-hits-cond => false
    (pins that the port reproduces, not "fixes", the conservative C++ result).
  - `w10cpp_phi_recovery_constants_are_propagated` — #1/#3/#5 PASS (real recovery).
  - `w10cpp_loop_block_copy_is_declined_div3` — #12 absent (GH-9203/DIV-3 loop
    decline) AND #1 still recovers (decline is loop-shape-scoped, not blanket).
  - `w10cpp_non_phi_paths_unregressed_and_no_over_propagation` — #2/#4/#10/#11 stay
    green (no over-propagation; `condconst_conn`'s no-replace `v1 = x;` survives).

losses: none (clean ACCEPT; F1/F2 are advisory, no observable divergence).
