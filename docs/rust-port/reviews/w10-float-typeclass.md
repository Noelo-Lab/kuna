# w10-float-typeclass
verdict: ACCEPT-WITH-LOSSES
verifier: independent (Opus 4.8, round 1)
date: 2026-06-16
gate: `cd rust && cargo test --workspace` -> 3603 passed / 0 failed; `cargo clippy -p kuna-decomp --lib -- -D warnings` -> clean

## Headline

The branch (ad08215) does NOT port new "float-register typeclass resolution" or the
"float10 model" as the title literally reads — those ALREADY EXIST on rust-port and are
correctly UNTOUCHED:
- the `splitFloat` pentry typeclass SEPARATION (open a new resource section on a
  storage-class change) lives at `fspec.rs:3260-3268` (`push_entry`) and is not in this diff;
- the float10 `TypeFloat` pre-exists in `dtype.rs`.

What the branch ACTUALLY delivers is the missing hook the float10/return parameter model
needs: wiring the (already-ported, earlier-item) `ModelRule`/`AssignAction` machinery
into `ParamListStandard`. The diff is 5 rust files, none of which is the typeclass/float10
code. This is a title/scope situation (the same shape as w10-proto-cluster), booked as
**LOSS-209**. It is NOT a REJECT condition: the content present is faithful, special-case
free, real-parity, and zero-regression (verified below), so the correct verdict is
ACCEPT-WITH-LOSSES (faithful PARTIAL relative to the literal title).

## What was verified faithful (the ModelRule wiring)

Statement-checked against the in-tree C++:
- `assign_address` (fspec.rs:2406-2430) is a faithful transcription of
  `ParamListStandard::assignAddress` (fspec.cc:778-785): iterate `model_rules` in order,
  return the first non-`fail` response, else fall through to
  `metatype2typeclass(dt) -> assign_address_fallback`. `pos: int4` (i32) == C++ `int4 pos`.
- cspec `<rule>` decode (architecture.rs:1723-1754): for each `<rule>` child of
  `<input>`/`<output>` (after the pentries are pushed, before `finish_decode`'s
  resourceStart/calcDelay/populateResolver, == C++ fspec.cc:1479-1506 ordering), decode via
  an `XmlDecode` rooted on the element through a `with_base_ids()+register_ids()` registry
  and `push_model_rule`. Children are a document-ordered `Vec` (xml.rs `add_child` push),
  so rules decode in C++ `modelRules.emplace_back()` insertion order.
- `push_pointermax_rule` (fspec.rs:3315-3334) == C++ fspec.cc:1507-1512:
  `SizeRestrictedFilter(pointermax+1,0)` + `ConvertToPointer{space=spacebase}`, appended at
  the END of `model_rules` (so a preceding rule wins). `pointermax` is parsed as `int4`
  (signed, == `readSignedInteger`), default 0/absent => no rule, `pmax>0` guard faithful.
- `register_ids` (modelrules.rs:148-196): registers the 15 modelrules elements + 16
  modelrules attributes NOT already in the base id set. VERIFIED COMPLETE: every `ELEM_*`
  and `ATTRIB_*` that `ModelRule::decode`/`AssignAction`/`DatatypeFilter`/`QualifierFilter`
  consult is registered (base ids `with_base_ids()` ∪ `register_ids`); `comm -23` of
  consulted-vs-registered is empty for both elements and attributes.
- `ParamListStandardOut::decode` delegates to the base decode (fspec.cc:1781), so output
  lists honor rules+pointermax too; the Rust handles both via the `is_input` branch. The
  output `initialize()` fillin-map wiring stays a documented separate SEAM
  (`use_fillin_fallback` stays true) — deferred, not regressed.

hunt list:
- Signedness: clean — `pos:int4`(i32), `pointermax` parsed `int4`, `pmax>0` guard; all
  faithful to the C++ `int4` types. No signed/unsigned compare introduced.
- Integer widths: clean — `pointermax+1` is `int4+1` == C++ `pointermax+1` (`SizeRestrictedFilter`
  takes `int4 min`); unreachable-overflow edge (`pointermax==i32::MAX`) is equally UB in C++
  (no guard either side), so faithful.
- Wrapping: clean — no `uintb` arithmetic in the diff (the rule loop and decode are pointer/
  index walks; the wrapping arithmetic lives in already-verified modelrules.rs/fspec).
- Comparator totality: n/a — no comparator ported in this diff.
- Iteration-order provenance: clean — `model_rules` is a `Vec` walked in index order
  (== C++ `list<ModelRule>` insertion order); the `<rule>` children are a document-ordered
  `Vec`. NO HashMap/HashSet anywhere in the diff.
- Off-by-one / do-while / reverse iterators: clean — the rule loop is a forward
  first-match `for ... { if !=fail return }` reproducing the C++ for-loop+early-return exactly
  (pinned by adversarial test A).
- Erase-while-iterating: n/a — no erase; rules are appended, never removed mid-walk.
- Exception -> Result partial-state parity: the one divergence — C++ `decode` THROWS if a
  `<pentry>`/`<group>` follows a `<rule>` (fspec.cc:1500); Rust filter-collects rules
  position-independently and never raises this malformed-cspec error (F2). Unobservable on
  any shipped/conformant cspec.

findings:
  - F1 (scope, MAJOR — booked as LOSS-209): the branch title names the float-register
    typeclass + float10 model, but those pre-exist on rust-port and are untouched; the
    branch delivers the ModelRule wiring. Faithful, real-parity, but a title/scope mismatch.
       cpp: decompiler/cpp/fspec.cc:774-785, 1453-1512 (assignAddress + decode — the ACTUAL
            content); typeclass/float10 (the TITLE) live elsewhere and are not in this diff.
       rust: rust/crates/kuna-decomp/src/fspec.rs:2406-2430, 3305-3340;
             architecture.rs:1723-1766; modelrules.rs:148-196.
  - F2 (minor, error-path omission — folded into LOSS-209): the C++ "rules must come after
    pentry/group" throw is not reproduced; Rust silently accepts out-of-order children.
    Unobservable on well-formed cspecs (rules still decode in document order).
       cpp: decompiler/cpp/fspec.cc:1492-1501 (LowlevelError throw)
       rust: rust/crates/kuna-decomp/src/architecture.rs:1735-1740 (position-independent filter)
  - F3 (cosmetic): `clippy -p kuna-decomp --lib --tests -- -D warnings` flags
    `unused variable: mgr` at the BRANCH's own test `push_pointermax_rule_appends_convert_to_pointer`
    (fspec/tests.rs:2197 — `mgr` is shadowed by `three_reg_model()`'s own manager). Pre-existing
    in the branch (NOT in the verifier's added tests); `clippy --lib` (the protocol requirement)
    is clean and `cargo test` compiles fine. Style nit, no correctness impact.

## Mechanical + parity evidence

- NO SPECIAL-CASING: the ONLY string literals in production added lines are `"rule"` and
  `"pointermax"` (cspec element/attribute names == C++ `ELEM_RULE`/`ATTRIB_POINTERMAX`).
  ZERO function-name / register-name / address / magic-value hardcoding in
  architecture.rs/fspec.rs/modelrules.rs added hunks.
- Forbidden constructs: NO `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/
  `partial_cmp`/bare `as` in the diff.
- RESERVED FILES: none touched — diff is exactly architecture.rs, fspec.rs, fspec/tests.rs,
  modelrules.rs, modelrules/tests.rs. funcdata_merge.rs / merge.rs / variable.rs /
  dynamic.rs / database.rs / varmap.rs are UNTOUCHED.
- `cargo test --workspace`: 3603 passed / 0 failed (3600 pre-existing + the verifier's 3
  adversarial tests). `cargo clippy -p kuna-decomp --lib -- -D warnings`: clean.
- Datatest passing SET (rust `decomp_test_dbg`, repo specs+datatests, `datatests`):
  parent a4bcd0f (branch HEAD^) = 675 applied / **327** passing; branch ad08215 = 675
  applied / **331** passing. `comm -23` of sorted Success sets: **ZERO regressions**.
  `comm -13`: the +4 are EXACTLY `Return Value Input Register #1..#4` (retspecial.xml,
  `void returnbig(mystruct *rethidden,int4 num)` recovered via the hidden_return ModelRule).
  Monotonic 327 ⊂ 331 >= 327.
- REAL PARITY vs C++ oracle B5: the C++ oracle (decomp_test_dbg, repo specs+datatests)
  prints `Success -- Return Value Input Register #1..#7` and the kuna baseline
  (docs/baseline.json) lists all 7 as passing; the 4 newly-passing rust assertions are
  byte-identical UPSTREAM stringmatches (`rethidden->a = 10;` etc.), so the rust output
  matches the same expected strings the C++ oracle matches. NOT fabricated. #5-#7 (the
  caller-side `read_returnbig` struct-return reconstruction) remain a rust-side downstream
  gap, correctly NOT claimed.
- mixfloatint / longdouble: NOT claimed as rust passes (correctly). `Mixed float/int` = 0
  passing on BOTH parent and branch; `Long double` = 4 passing on BOTH (unchanged — no
  regression, no false claim). They stay blocked DOWNSTREAM of fspec exactly as stated.
- C++ oracle 207/207 + 675/675 PARITY OK: structurally untouched — the diff changes ONLY
  `rust/` (no decompiler/cpp, specs/, or Makefile). baseline.json data_footer [675,675],
  unit_footer [207,207], returncode 0.
- B0 / universalAction listing: unchanged — no action registration/ordering touched; the
  rule loop is inert for rule-less models (empty `model_rules`), which is why all
  B-fixtures (boolless / condconst / readstruct etc.) stayed byte-identical (covered by the
  zero-regression passing-set diff and the 0-failure workspace suite).

adversarial tests (committed on the branch, 76000d3, marked w10_float_typeclass):
  - w10_float_typeclass_failing_rule_does_not_short_circuit_chain — a `[size>=20 filter, true
    filter]` two-rule chain: an int4 fails rule 1's size gate, so the loop continues and rule
    2 (stack) wins over BOTH the first `fail` AND the metatype register fallback. Pins the
    continue-on-fail loop boundary (C++ fspec.cc:778-784).
  - w10_float_typeclass_pointermax_rule_appends_after_existing_rules — a preceding goto_stack
    rule still wins for a float10 that also exceeds pointermax(8); the float10 keeps its full
    10-byte size (the appended ConvertToPointer never fires). Pins the END-append ordering
    (C++ fspec.cc:1507-1512).
  - w10_float_typeclass_register_ids_decodes_hidden_return_strategy — the multi-element
    `<hidden_return strategy="normalparam">` rule (the +4 retspecial driver) decodes through
    the production `with_base_ids()+register_ids()` registry and yields `hiddenret_ptrparam`
    (vs the default-strategy `hiddenret_specialreg`), proving `<hidden_return>` + `strategy`
    resolve in production registration.

losses: LOSS-209 (the title's float-register-typeclass + float10 model pre-exist and are
untouched; the faithful ModelRule wiring that IS delivered is accepted with zero regression
and real +4 parity; the unreproduced malformed-cspec "rules must come last" throw (F2) is
the minor residual, restored when a conformance test for it lands). F3 (unused `mgr` lint in
the branch's own test) is a cosmetic nit folded into LOSS-209's restoration note.
