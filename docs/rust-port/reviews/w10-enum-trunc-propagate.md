# Review — rport/w10-enum-trunc-propagate (Round 1)

**Verdict: ACCEPT**

Wave: `TypeOpSubpiece::propagateType` enum/equate through-truncation.
Branch HEAD: `7642dc4`. Base: `rust-port` @ `9fdbef0`.
Diff scope: `rust/crates/kuna-decomp/src/dtype.rs` + `rust/crates/kuna-decomp/src/coreaction_infertypes.rs` (test-only). No other files.

## What the change is

The single production change is 3 lines in `TypeFactoryImpl::get_type_partial_enum_impl`
(dtype.rs:5297-5301):

```rust
let mut tpe = Datatype::new_with_align(sz, -1, type_metatype::TYPE_PARTIALENUM);
tpe.metatype = type_metatype::TYPE_UINT;   // + NEW
tpe.flags |= flags::enumtype;              // + NEW
tpe.flags |= contain.inherit_for_partial();
tpe.flags |= flags::has_stripped;          // + NEW
tpe.kind = DatatypeKind::PartialEnum { stripped: strip, parent: contain, offset: off };
```

The baseline `get_type_partial_enum_impl` was an **unfaithful** port of the C++ ctor
chain: it left the partial enum with `metatype == TYPE_PARTIALENUM` and neither the
`enumtype` nor the `has_stripped` flag. As a result `is_enum_type()` returned `false`
for a truncated enum, so the truncated-through-pointer enum read
(`propagate_from_pointer`, coreaction_infertypes.rs:454/458) silently dropped the enum
type and the AND-mask constant rendered as a raw hex literal instead of by flag name.

## (1) Faithfulness — byte-faithful

`TypePartialEnum(par,off,sz,strip)` (type.cc:2683-2691) chains
`TypeEnum(sz, TYPE_PARTIALENUM)` (type.hh:548-549):
```cpp
TypeEnum(int4 s,type_metatype m) : TypeBase(s,m) {
  flags |= enumtype; metatype = (m==TYPE_ENUM_INT) ? TYPE_INT : TYPE_UINT; }   // TYPE_UINT
```
then the partial body adds `flags |= par->inheritForPartial()` and `flags |= has_stripped`.

- `metatype = TYPE_UINT` override — matches (m==TYPE_PARTIALENUM != TYPE_ENUM_INT → TYPE_UINT).
- `flags |= enumtype` — matches (`enumtype = 4`, type.hh:174 == dtype.rs:431).
- `flags |= has_stripped` — matches (`has_stripped = 0x100`, type.hh:180 == dtype.rs:443).
- `inheritForPartial()` — already present on baseline; unchanged.
- `submeta` correctly LEFT at `SUB_UINT_PARTIALENUM` (`new_with_align` seeds it from
  `base2sub(TYPE_PARTIALENUM)`; only `metatype` is overridden, never `submeta`).

Dedup safety: the metatype override to TYPE_UINT does NOT collide with a plain TYPE_UINT
base or a real enum in the intern tree. Ordering (`TreeKey::cmp` → `compare_dependency`)
dispatches on `DatatypeKind`; a `PartialEnum` routes to `compare_dependency_partial`,
which compares `submeta` (SUB_UINT_PARTIALENUM, distinct), then parent identity, offset,
size — a byte-faithful transcription of `TypePartialEnum::compareDependency` (type.cc:2731).

The `get_exact_piece_impl` guard `last_type.is_enum_type() && !last_type.has_stripped()`
(dtype.rs:5514) and the `propagate_from_pointer` enum arms (coreaction_infertypes.rs:454,
458) are byte-faithful to `getExactPiece` (type.cc:4606-4607). The fix lives in the
shared factory method, so it serves BOTH the SUBPIECE `getSubType` path and the
pointer-rel `getExactPiece` path that enum.xml #3/#4 (ptrenumlow/ptrenumhigh) drive.

## (2) No special-casing — clean

The 3 production lines apply to EVERY partial enum, keyed on nothing (no flag value, no
function name, no type name, no constant). Grep of the production diff for
flag/value/name keying finds only the generic `flags::enumtype` / `flags::has_stripped`
datatype flag-BIT constants. All `"flags"` / `FLAG_` / `flagfield` / `hash_name("flags")`
occurrences are inside test-fixture construction.

## (3) Real parity — passes

- C++ oracle (the B5 reference): enum.xml + convert.xml = **22/22**; full suite
  **675/675 PARITY OK** (`kuna.run_tests --all --baseline`). No C++/spec/Python file in
  the diff, so the oracle is definitionally untouched.
- The Rust truncated-enum propagation path now recognises the partial as an enum — the
  exact behaviour enum.xml "Enum Reading #3/#4" require.

## (4) No regression — clean

- `cargo test --workspace` (independently run, branch): **3643 passed / 0 failed** / 37
  ignored. Baseline (rust-port @ 9fdbef0): 3641 / 0. Net **+2** tests.
- Regressed set: **EMPTY** (`comm -23 baseline branch` yields nothing). The only set
  delta is the two new dtype.rs tests; every baseline-passing test still passes.
- boolless + condconst + convert: byte-identical (corpus `Expect` outcomes unchanged;
  `lift_diff` 3/3 pass). Switch guards (`verify_w10_callarg_piece_switchhide_guard`,
  `verify_w10_spacebase_typing_switch_guard`) pass — switch datatests byte-identical.
- Reserved files: NONE touched (subflow.rs, flow.rs, coreaction_cleanup.rs, dynamic.rs,
  printc.rs all untouched). Diff edits only dtype.rs + coreaction_infertypes.rs.
- `cargo clippy --workspace --lib`: **0 warnings / 0 errors**.
- B0 unchanged (change only fires for truncated-enum reads, absent from all B0 cases).

## Adversarial tests — 3 committed, all proven to catch the bug

1. `dtype::tests::factory_partial_enum_is_recognised_as_enum` (5 truncation geometries).
2. `dtype::tests::factory_get_exact_piece_enum_member_yields_partial_enum`
   (low/high slices of an enum struct member).
3. `coreaction_infertypes::propagate_type_tests::w10_from_pointer_enum_size_mismatch_yields_partial_enum`
   (strengthened: was asserting the wrong `TYPE_PARTIALENUM` metatype).

Verified adversarial: reverting ONLY the 3 production lines makes all 3 tests FAIL
(`is_enum_type()` / `TYPE_UINT` / `has_stripped()` assertions panic); restoring them
makes all 3 pass.

## Note (non-blocking)

The wave brief named `coreaction_infertypes.rs`/`typeop.rs` as the expected edit
targets, but the production fix correctly lands in `dtype.rs` (the C++ change site is
type.cc → dtype.rs). `dtype.rs` is not a reserved file; this is a benign deviation from
the brief's filename expectation, not a violation. The brief's named C++ anchor
(`TypeOpSubpiece::propagateType`) reaches the same `getTypePartialEnum` factory the fix
corrects, so the focus is satisfied.

## Losses

None. Clean faithful fix; empty regressed set; no undisclosed render deltas.
