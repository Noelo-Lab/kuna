# w3-ir-userop-inject
verdict: ACCEPT-WITH-LOSSES
verifier: opus-4.8 (independent)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp` -> ok (71 in-module + 13 w3-ir-op + 9 this-item verifier + 6 w3-ir-varnode = 99 passed; 0 failed)

## Scope

Branch `worktree-wf_434ccb01-75c-7` @ e09585c (item commit) over the 3 src + 3
test modules. C++ in scope at the recorded `cpp_blob_sha` — all six blob shas
verified IDENTICAL to the checklist (`git hash-object` match, no drift):
userop.{cc,hh}, pcodeinject.{cc,hh}, inject_sleigh.{cc,hh}. The branch's
`rust-port...` 3-dot diff also carries the already-merged w3-ir-op /
w3-ir-varnode work; only the last commit (e09585c) is this item and is what was
reviewed.

## Hunt list

- **Signedness (-Wno-sign-compare):** clean. The signed/unsigned C++ comparison
  sites (`slot>=0 && slot<inTypes.size()` in DatatypeUserOp::getInputLocal) live
  in the W6-deferred dataflow virtuals (not ported; LOSS-031). The ported
  `int4 ind`/`useroplist.size()` guards (registerOp userop.cc:516 -> userop.rs:968
  `while (self.useroplist.len() as int4) <= ind`) preserve the C++ signed-index
  growth semantics. `injectid` reverse-vector pads compare `len() as int4 <=
  injectid` matching the C++ `vector.size() <= injectid` exactly.
- **Integer widths:** clean. `int4`->i32 / `uint4`->u32 / `uintb`->u64 mappings
  consistent. The semantic narrowings ARE commented: `injid as uint4`
  (userop.rs:262, int4->uint4 bit-preserve, matches C++ int4 injid into uint4
  member), `sz as uint4` (:520, constresolve.size), `read_unsigned_integer()? as
  uint4` (pcodeinject.rs:281), `read_signed_integer()? as int4` (:319, :603).
- **Wrapping:** clean. `SnippetLayout::build` uses `uniq_reserve.wadd(0x20)`
  (pcodeinject.rs:695,705) for C++ `uniqReserve += 0x20` (uintb silent wrap).
  `order_parameters`'s `id += 1` is int4 (cannot wrap in practice, matches C++).
- **Comparator totality:** N/A — no custom comparators; all ordered containers
  are `BTreeMap<Vec<u8>, _>` / `BTreeMap<uint4, _>` with total Ord.
- **Iteration-order provenance:** clean, NO HashMap/HashSet. `name_lookup`
  iterates `useropmap` (BTreeMap byte-string order == C++ `std::map<string,..>`),
  but its only consumer is point lookups (`get_by_name`), so order is not
  output-observable. `initialize`'s `basicops.iter().enumerate()` is Vec-index
  order == C++ vector index. All `0..size_*()` loops are index order.
- **Off-by-one / do-while / reverse iterators:** clean. No do-while; the `for(;;)`
  decode loops are ported as `loop { ... break }` preserving at-least-once peek.
  One structural divergence found in the SegmentOp/JumpAssist child loops (added
  `else { break }`) — benign hardening, ledgered LOSS-032 (see findings F2).
- **Erase-while-iterating:** N/A — no list erasure during traversal in this item.
- **Exception -> Result partial-state parity:** clean. Failed decode_segment /
  decode_jump_assist leave a partially-mutated `s_op`/`op` that is DROPPED (never
  reaches register_op), identical to the C++ `unique_ptr` discard on throw in
  decodeSegmentOp/decodeJumpAssist. register_builtin errors before
  builtinmap.insert (matches C++ throw-before-assign). decodeInject's
  allocate->decode->register chain: a decode `?`-error leaves the allocated
  payload in `injection` (already pushed by allocate_inject) but unregistered in
  the name maps — identical to C++ where `allocateInject` push_backs before
  `decode` throws (the payload vector keeps the half-built object in both).
- **SeqNum/create_index allocation-order fidelity:** the relevant allocator here
  is `InjectPayload::orderParameters` (sequential index across inputs-then-outputs)
  and `allocateInject`'s `injectid = injection.size()` — both faithful and
  adversarially pinned (verifier tests order_parameters_*,
  decode_payload_params_indexes_inputs_first_despite_doc_order,
  manual_call_other_fixup_*).
- **Intrusive-list splice-order fidelity (ADR 0001):** N/A — this item has no
  intrusive lists; the C++ containers are plain vector/map.

## Mechanical pass

- port-audit: tool `port-audit` not present in this workspace; blob-sha drift
  checked manually (all six IDENTICAL — see Scope).
- grep diff: NO `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`. Two
  `.unwrap()` in src (userop.rs:881,943) are on `builtinmap.get(&i)` immediately
  after a `contains_key`/`insert` of the same key -> provably present. Bare `as`
  casts: the load-bearing semantic narrowings carry justifying comments; the
  remainder are container index/length bridges (`len() as int4`, `idx as usize`)
  reproducing C++ int4/size_t interplay (idiomatic; consistent with prior W3
  reviews).
- clippy: `cargo clippy -p kuna-decomp --lib` clean; the new test file clippy
  clean. (NOTE: `cargo clippy --all-targets` fails on a `doc_lazy_continuation`
  lint in tests/verify_w3_ir_op.rs:434 — that file belongs to the PRIOR item
  w3-ir-op, is NOT in this commit, and is outside this item's scope. The item
  gate `cargo test -p kuna-decomp` does not run clippy and passes.)

## Findings

- F1 (info, no action): the data-type/display/dataflow virtuals
  (getOperatorName/getOutputLocal/getInputLocal/extractAnnotationSize/
  SegmentOp::unify/execute), the emit family (InjectPayload::inject,
  ExecutablePcode::build/evaluate, setupParameters/checkParameterRestrictions),
  and the debug path (InjectPayloadDynamic/decodeDebug/forceDebugDynamic) are
  seam-deferred. They sit inside this item's verified module scope but every
  consumer is a later wave (W6 types / Funcdata / the kuna-sleigh emit API /
  the `<injectdebug>` debug path). Covered by umbrella SEAM module-doc comments +
  the `InjectEngine` trait, but lacked per-method ledger visibility -> recorded
  as **LOSS-031** (LOSS-029/030 accountability precedent). No surface at the gate.
       cpp: decompiler/cpp/userop.cc:38,71,77,122,129,144,153,160,175,190,219,362; inject_sleigh.cc:48,109,137,234,282-341,357,465,485; pcodeinject.cc:146,192
       rust: rust/crates/kuna-decomp/src/userop.rs:17-23,201,670; pcodeinject.rs:464; inject_sleigh.rs:434,599

- F2 (minor, accepted): SegmentOp::decode and JumpAssistOp::decode child loops
  add a final `else { break; }` the C++ `for(;;)` chains lack. On a malformed
  element with an unexpected third child kind the C++ infinite-loops (re-peeks
  the same id forever); the Rust breaks cleanly. Unreachable from any vendored
  spec (only the two/four declared child kinds ever appear). Strictly-safer
  divergence -> recorded as **LOSS-032** (LOSS-028 "safer divergence" precedent).
       cpp: decompiler/cpp/userop.cc:256-275, 312-339
       rust: rust/crates/kuna-decomp/src/userop.rs:531-535, 614-616

No blockers, no majors. The decode / registration / lookup / payload-factory /
parameter-ordering logic — the genuine new behavior of this wave — is a faithful
line-against-line port and is adversarially confirmed.

## Adversarial tests

Added rust/crates/kuna-decomp/tests/verify_w3_ir_userop_inject.rs (9 tests, all
pass; committed on branch as 9fb5be6):
- order_parameters_inputs_then_outputs_single_counter
- decode_payload_params_indexes_inputs_first_despite_doc_order
- get_op_hole_in_range_does_not_leak_builtin
- register_op_rejects_two_segmentops_for_same_space
- snippet_layout_empty_payload_keeps_initial_reserve
- snippet_layout_output_only_starts_at_first_slot
- get_payload_id_routes_by_type_and_default_is_script
- manual_call_other_fixup_pushes_inputs_then_output_in_order
- manual_call_other_fixup_empty_output_pushes_no_output

Two initially failed on verifier-side expectation errors (a miscounted
AddrSpace::new arg, and an expectation that manualCallOtherFixup binds the userop
name — it does NOT; both C++ and the port register the payload under the
InjectPayloadCallother default name "unknown"); corrected to assert the
C++-faithful behavior, after which all 9 pass. No port bug was found by the
adversarial pass — the failures confirmed the port matches C++ even on the
easy-to-get-wrong "unknown"-name and allocation-order edges.

## losses

LOSS-031 (in-scope dataflow/display/emit/debug methods seam-deferred),
LOSS-032 (SegmentOp/JumpAssist decode-loop `else break` hardening divergence).
Both appended to docs/rust-port/losses.md (main tree).
