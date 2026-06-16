# w10-union-value
verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier (Opus 4.8)
date: 2026-06-16
gate: `decomp_test_dbg datatests` (Rust harness, full corpus) -> branch 294 passing / 672 applied; `cargo test --workspace` -> all green (0 failed); `cargo clippy -p kuna-decomp -p kuna-console --lib -D warnings` -> clean; C++ oracle `decomp_test_dbg` -> 207/207 unit + 675/675 datatest PARITY OK (untouched).

## Scope of the actual change

The branch is a single commit (`00f20b1`) on merge-base `dfe337b`. The production
diff is **26 lines across 3 files** — it widens three type-resolution dispatchers
to match `DatatypeKind::PointerRel` alongside `DatatypeKind::Pointer`:

- `rust/crates/kuna-decomp/src/dtype.rs:3126` — `Datatype::find_compatible_resolve`
  (also swaps `as_plain_pointer().map(...)` -> `get_ptr_to()` so the `ct` ptrto
  read works for a PointerRel argument).
- `rust/crates/kuna-decomp/src/funcdata_facing.rs:72` — `Funcdata::find_resolve_facing`.
- `rust/crates/kuna-decomp/src/funcdata_resolveflow.rs:55` — `Funcdata::resolve_in_flow`.

This is faithful to C++: `TypePointerRel : public TypePointer` (type.hh:724) does
**not** override `resolveInFlow`/`findResolve`/`findCompatibleResolve`, so a
relative pointer dispatches the inherited `TypePointer` body. The `PointerRel`
match arm binds the same `ptrto` member the `Pointer` arm does (the relative
pointed-to / field type), exactly matching the inherited `this->ptrto` and
`((TypePointer*)ct)->ptrto` member reads (type.cc:1314-1354). The
`as_plain_pointer()->get_ptr_to()` accessor swap is behavior-preserving for the
pre-existing Pointer path (both yield `ptrto`) and correctly extends to PointerRel.

The named `pushPartialSymbol`/`pushImpliedField` member walk and
`TypeUnion::resolveTruncation` from the task scope are **NOT implemented** — the
commit declares them seam-remaining. Hence PARTIAL / ACCEPT-WITH-LOSSES.

## hunt list
- Signedness: clean (N/A — no comparisons in the diff; pure match-arm widening).
- Integer widths: clean (N/A — no arithmetic; `int4` result of `find_compatible_resolve` unchanged).
- Wrapping: clean (N/A — no arithmetic).
- Comparator totality: clean (N/A — no comparator touched).
- Iteration-order provenance: clean (N/A — no container iteration added; no HashMap/HashSet).
- Off-by-one / do-while / reverse iterators: clean (N/A — no loop).
- Erase-while-iterating: clean (N/A — no mutation/erase).
- Exception -> Result partial-state parity: clean — the only `?`-propagation is the
  pre-existing `pointer_invariant_err` on a non-pointer `get_ptr_to`, unreachable for
  the matched arms (which are pointers by construction).

## Mechanical pass
- grep of added lines: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/bare `as`.
- No hardcoding: zero field-name / type-name / offset / function-name / address literals in the diff.
- Reserved files: NOT touched — dtype `getTypeCode`/`setPrototype` absent from the diff; console grammar untouched.
- `cargo clippy -p kuna-decomp -p kuna-console --lib -D warnings`: clean.

## Parity (re-run, REAL)
- Union datatests, branch worktree vs its merge-base `dfe337b` (both 672 applied):
  base 289 passing -> branch 294 passing, **+5** = Union #20 `(ptr->value).myfloat`,
  #21 `(ptr->value).myint`, #23 `(ptr->value).myint + 0x1b`, #30 `*(s->value).ptr`,
  #31 `(s->value).a`. The C++ oracle B5 passes these exact 5 (and all 38 union
  assertions); the branch's renders match byte-for-byte (the datatest stringmatches
  are `min=1 max=1`).
- Full corpus passing-SET diff branch vs merge-base: regressions EMPTY
  (`comm -23` empty), gains exactly the 5 above. NO regression.
- Whole-suite: 289 -> 294 (>= 289 met).
- The 290->294 / 675->672-applied delta seen against *current* rust-port is the
  concurrent indproto wave the branch does not yet carry (`Indirect prototype #1`
  is FAIL on both sides at the branch's merge-base; applied is 672 on both) — NOT
  a regression from this change. Re-verified at the merge-base.
- Seam-remaining renders stay FAIL on the branch (Union #4 `ptr->arr[index]`,
  #10 `glob1.c`, #22/#25/#29 cast/SUBPIECE forms) — confirms no special-casing /
  no shortcut to fake a render.
- `cargo test --workspace`: green (0 failed). C++ oracle 207/207 + 675/675 PARITY OK,
  Rust-only diff (decompiler/cpp, specs, datatests, baseline.json all untouched).

findings:
  - F1 (informational, no defect): this item delivers only the pointer-to-union
    value-member render via the inherited-dispatch fix; the task-named
    `pushPartialSymbol`/`pushImpliedField`/`TypeUnion::resolveTruncation` are
    seam-remaining (symbol-side `glob1.c`/`val.c` + SUBPIECE forms still FAIL).
    Tracked as LOSS-196. Not a correctness defect — the transcription is complete
    for its scope.
       cpp: decompiler/cpp/type.cc:1314-1354 (delivered); printc.cc pushPartialSymbol + type.cc:2541-2556 resolveTruncation (seam-remaining)
       rust: rust/crates/kuna-decomp/src/dtype.rs:3126, funcdata_facing.rs:72, funcdata_resolveflow.rs:55

adversarial tests: rust/crates/kuna-decomp/tests/verify_w10_union_value.rs (6 tests; all pass on the branch, mutation-checked — reading `stripped` instead of the relative `ptrto` fails 4 of them):
  - w10_union_value_compat_resolve_reads_ct_relative_ptrto_not_stripped
  - w10_union_value_compat_resolve_ptrto_is_relative_field_not_stripped (the concrete index-0-vs-(-1) mutation discriminator)
  - w10_union_value_compat_resolve_pointerrel_receiver_matches_plain_pointer
  - w10_union_value_compat_resolve_non_pointer_ct_returns_minus_one (TYPE_PTR gate)
  - w10_union_value_compat_resolve_cross_kind_symmetry
  - w10_union_value_union_carries_needs_resolution_flag (the dispatcher-union-arm precondition)

  Note: 3 of these (the PointerRel-parity asserts) FAIL on the main tree pre-merge
  (rust-port lacks this branch's fix) and PASS once the branch merges — they target
  the exact behavior this item changes.

losses: LOSS-196 (the symbol-side union value-members + SUBPIECE/resolveTruncation forms; see docs/rust-port/losses.md).
