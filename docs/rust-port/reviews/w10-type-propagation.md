# w10-type-propagation
verdict: ACCEPT
verifier: verifier-agent (Opus 4.8 1M) — Round 2
date: 2026-06-14
gate: `cd rust && cargo test --workspace` -> ALL GREEN (3218 pass / 0 fail, incl. the porter's 6 `w10_` + the verifier's 3 `w10_r2_` adversarial tests); `cargo clippy -p kuna-decomp -- -D warnings` -> clean (lib gate, the protocol gate); C++ oracle `python -m kuna.run_tests --all --baseline docs/baseline.json` -> 207/207 unit + 675/675 datatest, **PARITY OK** (branch touches only `rust/`; `decompiler/`, `specs/`, `baseline.json` unchanged — verified by `--name-only`).

## Round-2 disposition
Round 1 was a REJECT on a single finding, **F1**: `propagate_add_in2_out`'s
do-while overwrote `parent`/`parent_off` *unconditionally* from each
`down_chain` return, so a scalar/enum tail iteration (which returns `par=None`)
cleared a struct/array container set by an earlier iteration — weakening the
array/struct member-pointer override on its central edge.

The fix commit (`b737262`) is **a faithful correction**:
`coreaction_infertypes.rs:974` now guards the write —
`if par.is_some() { parent = par; parent_off = par_off; }` — mirroring the C++
`TypePointer::downChain` (type.cc:1245-1248) which writes its `par`/`parOff`
reference parameters **only on the `isArray || meta==TYPE_STRUCT` arm** and never
resets them, so the C++ caller (typeop.cc:1228-1238) retains the *last*
STRUCT/ARRAY container across the do-while. I confirmed this directly against
both C++ sites. The corrected behavior is **strictly more specific** along the
affected edge (it now produces the `getTypePointerRel` TYPE_PTRREL the C++
produces, instead of a bare pointer/decline). No regression: the corpus tally is
byte-identical to the `rust-port` base (see below). **F1 is resolved.**

F2 (the missing `// cast:` comments) was **partially** addressed: the fix added
justifications to `idx as i32`, `offset as i64`, `parent_off as int4`, and
`sz as uintb`. A handful of bare `as` in the new production code still lack the
comment (`num_spaces() as u64`, `wordsz as u32`, `spc.get_word_size() as int4`,
the composite byte-offset `? as int8`, `v.get_offset() as int4`). These remain
low/non-blocking — each is individually faithful to a C++ implicit conversion
(verified) — but the protocol asks for the comment, so F2 is left open as a
documentation nit, not a correctness finding.

## Independent re-audit (this round, not relying on the porter's notes)
All 11 overrides were re-checked line-for-line against typeop.cc / type.cc,
independent of Round 1:
- `propagateToPointer` (typeop.cc:187) / `propagateFromPointer` (207): FAITHFUL
  — PTR->unknown*, PARTIALSTRUCT->getComponentForPtr, else getTypePointer; the
  size-mismatch relptr `getExactPiece`+isEnumType / plain `getTypePartialEnum`
  arms transcribed exactly.
- `getComponentForPtr` (type.cc:2784): FAITHFUL line-for-line (ARRAY + offset%alignSize==0 -> element, else stripped). `offset` and `getAlignSize()` both `int4` in C++ and Rust -> no signedness divergence in the modulo.
- `floatSignManipulation` (typeop.cc:154): FAITHFUL (`mask>>1` ABS / `mask^(mask>>1)` NEG / CPUI_MAX).
- IntXor/IntAnd/IntOr (typeop.cc:1424/1457/1490): FAITHFUL — enum-or-float(sign-manip) guard (IntOr enum-only); spacebase arm = `getTypePointer(altSize, getBase(1,UNKNOWN), defaultDataSpace.wordSize)`, matched by `spacebase_pointer`.
- LOAD/STORE (typeop.cc:488/559): FAITHFUL — slot-0 / spacebase guards, LOAD inslot==-1 / STORE inslot==2 -> toPointer else fromPointer; word-size from the constant space operand (LOSS-015 index model).
- Piece/Subpiece (typeop.cc:2076/2163) + computeByteOffsetForComposite (2106): FAITHFUL — the near/far pointer-resize arms and the getSubType composite walk. The C++ `while(alttype!=0 && (byteOff!=0 || size!=outsize))` pre-test loop is reproduced exactly by the Rust `while let Some` + top-of-loop break on `byte_off==0 && size==out_size`. `get_sub_type` returns `Ok((None,..))` (never `Err`) for the metatypes in play, so `.ok()?` is equivalent to the C++ null return — no exception/Result-parity divergence. (The UNION/PARTIALUNION `resolveTruncation` pre-step at 2178 is omitted — documented W8 union seam, common path identical.)
- Ptradd/Ptrsub/Segment/New (typeop.cc:2270/2368/2433/2503): FAITHFUL — slot guards (PTRSUB correctly omits the PTRADD slot-2 guard), PTR-metatype guard, inslot==-1 declines up, SEGMENT resizePointer(outvn->getSize()), NEW cpoolref-def guard.
- `propagateAddPointer` (typeop.cc:1270): FAITHFUL — PTRADD/PTRSUB/INT_ADD classification, INT_MULT offset detection, `mult==calc_mask(-1)` and `mult % sz` guards, `(off*mult)&calc_mask` via `wrapping_mul`.
- `propagateAcrossCompare` relptr-struct arm (typeop.cc:974-984): FAITHFUL — `isPointerRel() && !outvn const` -> parent STRUCT && byteOffset>=0 -> bare pointer; else identity.
- `propagateAddIn2Out` (typeop.cc:1217) including the do-while: FAITHFUL **after the F1 fix** — including the post-loop `if (parent!=0)` getTypePointerRel wrap and the spacebase-demote tail.

The 5 ancillary edits (`fspec/tests.rs`, `modelrules/tests.rs`, 3 `verify_w6_s4_*.rs`) are mechanical `get_type_partial_enum` trait-stub additions (the branch adds the method to the `TypeFactory` trait); all `unreached!()`/`panic!()`, mirroring the sibling partial-struct/-union stubs. Necessary, faithful, no concern.

## NO SPECIAL-CASING
CLEAN. The production additions (`coreaction_infertypes.rs`, `dtype.rs`) contain
**zero** string literals (verified by a literal-extraction pass over the diff)
and **zero** function-name / address / type-name hardcoding. Every string in the
diff is a test fixture (`"a"`, `"twofield"`, `"sa"`) or a C++-comment quote. The
inferred types come from genuine propagation through the real `TypeFactoryImpl`
machinery (`down_chain`, `get_type_pointer{,_rel,_strip_array}`,
`get_sub_type`, `get_component_for_ptr`), reachable in production via
`run_infer_types -> propagate_type -> propagate_add_in2_out` (lines 273/534/547/831),
exercised end-to-end green by `print_b5_boolless.rs:831`.

## Types are REAL / datatest parity
The pre-existing corpus tally (`verify_w10_struct_corpus.rs`, **on the
`rust-port` base** — NOT introduced by this branch) is **byte-identical** before
and after the branch: 675 assertions, 0 bootstrap skips, **3 PASS-positive /
32 PASS-negative / 630 FAIL-positive / 10 FAIL-negative** on both. So the branch
makes **no new datatest claim and causes no datatest regression** — consistent
with the checklist's "No new datatest expectations claimed." The 3 positive
content passes belong to the prior pspec/refinement items, not this one. The
"real types" evidence for this item is the unit level: the adversarial tests
build genuine struct/array/pointer types and exercise real `down_chain` +
`get_type_pointer_rel` to produce real TYPE_PTRREL / TYPE_PTR outputs (not
stubs). Execution-failure count: unchanged (no new exec failures; full workspace
green).

## hunt list
- **Signedness**: clean. `inslot`/`outslot` int4 vs small literals; `1 - slot`, slot in {0,1}; `mult % (sz as uintb)` reproduces the C++ int4->uintb promotion (sz is the non-negative align size; for hypothetical negative sz, i32->u64 sign-extends identically to C++).
- **Integer widths**: clean. `get_offset()`=uintb, `calc_mask`=u64; `address_to_byte_int` int8; `parent_off as int4` narrows int8->int4 exactly as `getTypePointerRel(int4 off)`; struct-field modulo is int4%int4 both sides.
- **Wrapping**: clean. `const_off.wrapping_mul(mult)` = C++ uintb*uintb; `down_chain` sign-extend/`%`/`+=` are signed intb math matching type.cc:1228-1235.
- **Comparator totality**: n/a (no comparator added).
- **Iteration-order provenance**: clean. **No HashMap/HashSet** in the production diff (grep NONE). The only loops are the single-pointer `down_chain` descent (deterministic, one container per level) and the deterministic getSubType composite walk.
- **Off-by-one / do-while**: clean. The two do-whiles (the `downChain` walk and the Piece getSubType walk) preserve at-least-once / pre-test semantics respectively — re-derived this round. **The F1 fix corrects the per-iteration `parent` retention to match the C++ ref-write; verified at both C++ sites and reproduced by the verifier's strip-array test.**
- **Erase-while-iterating**: n/a (no container mutation during traversal).
- **Exception->Result partial-state**: faithful. All `?`-decline points return `None` like the C++ null edges; `get_sub_type` never `Err`s for the in-play metatypes, so no spurious early-return vs C++.
- **Bare `as` casts**: F2 — several now carry `// cast:`; a few in the new production code still lack it. Each verified faithful; low/non-blocking.

## findings
- F1 (medium, Round 1 REJECT trigger): **RESOLVED** in `b737262`.
       cpp: decompiler/cpp/typeop.cc:1228-1238 + decompiler/cpp/type.cc:1245-1248
       rust: rust/crates/kuna-decomp/src/coreaction_infertypes.rs:974 (`if par.is_some()` guard)
- F2 (low, non-blocking — open): a few bare `as` casts in the new production code still lack the `// cast:` justification (`num_spaces() as u64`, `wordsz as u32`, `spc.get_word_size() as int4`, composite `? as int8`, `v.get_offset() as int4`). Each is faithful to a C++ implicit conversion; documentation nit only.
       rust: rust/crates/kuna-decomp/src/coreaction_infertypes.rs:310,335,447,614,651
- OBS (test quality, non-blocking): the porter's `propagate_*` unit tests and the verifier's `w10_r2_downchain_*` tests **replay** the `propagate_add_in2_out` loop body inline rather than driving the production function (which needs a full `Funcdata`). I confirmed empirically that reverting the production guard leaves these replay tests passing — so they validate the *fixed logic* but would not, by themselves, catch a regression in the production *call site*. Production correctness here rests on direct inspection (line 974 carries the guard) + the C++ oracle parity + the end-to-end `print_b5_boolless` exercise — all green. Not a faithfulness defect; noted for future hardening (an end-to-end PTRSUB-into-struct fixture would close the gap).

## out-of-scope observations (NOT W10 findings; for the owning item)
- `get_type_partial_enum_impl` (dtype.rs:4646, **pre-existing W6 type-3 code, byte-identical on the base** — this branch adds only the one-line trait dispatch) builds the partial enum with `metatype=TYPE_PARTIALENUM` and sets neither `enumtype` nor `has_stripped`, where C++ `TypePartialEnum` overrides `metatype=TYPE_UINT` and sets those flags (type.cc:2683-2691). This W10 item's *call* of `getTypePartialEnum` is faithful (C++ returns it directly); the flag/metatype divergence belongs to **w6-s5-type-3**, not this diff.
- The Round-1-noted clippy `--all-targets` errors (`kuna_regionid.rs:2179`, `universalaction/tests.rs:50`) are confirmed **pre-existing on the `rust-port` base** and in files this branch does not touch. The protocol lib gate (`-p kuna-decomp -- -D warnings`) is clean.

## adversarial tests
Verifier Round-2 tests, committed to the worktree
(`rust/crates/kuna-decomp/src/coreaction_infertypes.rs`, module
`propagate_type_tests`, prefix `w10_r2_`), commit `4176cc2`:
- `w10_r2_downchain_struct_with_array_field_retains_struct_across_strip` — a STRUCT with an inline int4-array field; asserts the array is stripped to a pointer-to-element on descent (getTypePointerStripArray ran) and the STRUCT container is retained across the scalar-element tail (the F1 fix). Discovering *why* my initial naive premise (expecting a separate ARRAY `par`) was wrong gave independent confirmation that the strip-array descent matches C++ exactly.
- `w10_r2_downchain_pure_scalar_leaves_no_parent` — a container-free scalar chain leaves `parent=None`, so the post-loop getTypePointerRel wrap is skipped (no spurious TYPE_PTRREL).
- `w10_r2_component_for_ptr_misaligned_offset_returns_stripped_not_element` — pins the `offset%alignSize==0` gate of getComponentForPtr (misaligned -> stripped; aligned -> element).
All 3 pass on the fixed branch.

losses: none. F1 was a fixable unfaithfulness (now fixed), not an accepted loss.
F2 is a documentation nit, not a behavioral divergence. No `losses.md` entry
required.
