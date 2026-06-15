# w10-partial-types
verdict: ACCEPT
verifier: Claude Opus 4.8 (1M context) — independent verifier
date: 2026-06-15
round: 1

gate: `cargo test --workspace --no-fail-fast` -> 3290 passed / 0 failed / 0
      ignored-as-failure (EXIT 0); `cargo clippy -p kuna-decomp -p kuna-console
      --lib -- -D warnings` -> clean; C++ oracle (decomp_test_dbg) untouched
      (rust-only diff) -> 207/207 + 675/675 PARITY OK preserved by construction.

## Scope of the diff

Single commit `411d975` on top of `rust-port`; **only `rust/` is touched**
(verified `git diff --name-only | grep -v '^rust/'` is empty), so the C++
sources, SLEIGH specs, and the C++ oracle are byte-identical to base — the
675/675 datatest parity cannot be perturbed by this change.

Three files, 203 diff lines:

1. `kuna-console/src/ifacedecomp.rs` — un-stubs `IfcMaphash` (`map hash`) and
   `IfcMapunionfacet` (`map unionfacet`), which were `engine_unavailable`
   stubs that exec-failed `partialunion.xml` / `partialmerge.xml`; adds a
   `parse_hex_u64` helper for the C++ `s >> hex >> hash` extraction.
2. `kuna-decomp/src/varmap.rs` — two thin `ScopeLocal` wrappers
   (`add_dynamic_symbol`, `add_union_facet_symbol`) forwarding to the **already
   present and previously-verified** `Database` methods (no new recovery logic).
3. `kuna-decomp/src/ruleaction_3.rs` — fills four missing arms in the W6
   `opflags_for` shim so a `RuleMultiCollapse`-produced INT_AND/XOR/OR/NEGATE/
   2COMP/LEFT/RIGHT/LESS no longer panics the whole file.

## Faithfulness (transcription, not file-decode)

- **IfcMaphash** vs `ifacedecomp.cc:588-605`: parse_machaddr -> hex hash ->
  ws -> parse_type -> `addDynamicSymbol(name,ct,addr,hash)` ->
  `setAttribute(namelock|typelock)`. The Rust order is identical; `parse_type`
  reads `s.rest()` (the line tail), the established kuna console convention for
  the trailing type declaration (matches the `run_parse_c`/IfcMapParam pattern).
  The hash flows as `uint8` (u64). FAITHFUL.
  rust: rust/crates/kuna-console/src/ifacedecomp.rs:820-857
- **IfcMapunionfacet** vs `ifacedecomp.cc:774-799`: ws -> unionName ->
  `findByName` -> `ct==0 || getMetatype()!=TYPE_UNION` => "Bad union data-type"
  -> `dec >> fieldNum` -> `fieldNum < -1 || fieldNum >= numDepend()` => "Bad
  field index" -> parse_machaddr -> hex hash -> name `"unionfacet<n+1>_<hexoff>"`
  -> `addUnionFacetSymbol` -> `setAttribute(typelock|namelock)`. Every step is a
  line-for-line transcription. The Rust `.filter(TYPE_UNION).ok_or_else(...)`
  collapses the C++ `ct==0 || not-union` into one "Bad union data-type" message,
  exactly as C++ does. `find_by_name`'s `KunaResult` is always `Ok` for the real
  factory, so the error-map arm is dead and the None/non-union path is the live
  one. FAITHFUL.
  rust: rust/crates/kuna-console/src/ifacedecomp.rs:1015-1067
- **opflags_for arms** vs `typeop.cc`: INT_XOR/AND/OR = binary|commutative
  (1413/1446/1479); INT_2COMP/NEGATE = unary (1385/1399); INT_LEFT/RIGHT =
  binary (1507/1532); INT_LESS = binary|booloutput (1072). Cross-checked the
  flag bit-values (`pcodeop_flags` vs `op.hh`: booloutput 0x80, commutative
  0x4000, unary 0x8000, binary 0x10000 — all exact). Verbatim. FAITHFUL.
- **ScopeLocal wrappers** vs the already-ported `Scope::addDynamicSymbol`
  (database.cc:1695, size from `ct->getSize()`) and `addUnionFacetSymbol`
  (database.cc:1742, UNION_FACET category + fieldNum, invalid-addr guard) — pure
  forwarders; the underlying methods match the C++ (re-read both).

## Special-casing hunt (the REJECT trigger)

Grepped the added lines for function-name / address / case-value / type-name
hardcoding and `if x == "..."` branches: **none**. The only string literals are
(a) C++ source comments transcribed into the body, and (b) the legitimate
`"unionfacet{n}_{hexoff}"` symbol-name *template* (the C++
`s2 << "unionfacet" << ... ` construction), which is data-driven on the parsed
field index + address, not a hardcoded match. No `0x...` magic, no per-file
shortcut. CLEAN — not a decode of a specific datatest.

## Recovery is REAL

End-to-end: bootstrapped the actual `partialunion` corpus, ran the real script
prefix + `lo fu partial1` + the exact `map unionfacet structunion 1 r0x1006ee
10603fc3e29498` command, and confirmed a `UNION_FACET` symbol named
`unionfacet2_1006ee` lands in the loaded function's local scope (the old stub
left nothing). The commit's metric (partialunion exec-fail->5 applied/0 passing;
partialmerge exec-fail->11 applied/1 passing; aggregate +6 passing 48->54, +42
applied) is consistent with this — these are REAL new applied/passing
assertions, gated on the seam actually creating symbols. partialsplit correctly
stays exec-fail on the *separate* `option noreturn` proto seam (not this class).

## Mandatory hunt list

- Signedness: `field_num < -1 || field_num >= num_depend()` — both `int4`
  (signed), C++ identical; `num_depend()` returns `int4`. CLEAN.
- Integer widths: hash is `uint8`/u64 both sides; `field_num+1`/`get_offset()`
  hex format match C++ (`dec (fieldNum+1)`, `hex addr.getOffset()` lowercase no
  prefix vs Rust `{}`,`{:x}`). CLEAN.
- Wrapping: `field_num + 1` cannot overflow (bounded by the small field count
  after the `< num_depend` check); `parse_hex_u64` errors on >u64 overflow where
  the C++ stream sets failbit+clamps — both abort the command; identical for the
  in-corpus 14-hex-digit hashes. CLEAN.
- Comparator totality: n/a (no comparators in the diff).
- Iteration order: n/a (no container loops; `find_by_name` uses the existing
  BTree nametree). CLEAN.
- Off-by-one / do-while: the two bounds (`== num_depend` rejected upper, `< -1`
  rejected lower, `-1` accepted sentinel, `0` accepted) all have adversarial
  tests; CLEAN.
- Erase-while-iterating: n/a.
- Exception->Result partial-state: both commands `?`-propagate before any
  mutation except the (idempotent) symbol insert at the very end; the C++ throws
  at the same points (pre-`addSymbol`). One benign ADD: a `conf.is_none()` guard
  ("No load image present") the C++ lacks — C++ would null-deref `conf->types`;
  the Rust converts that impossible-when-fd-loaded case to a clean error. No
  observable divergence on valid input. NOTED, not a finding.

## Mechanical pass

- `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`/bare `as` on added
  lines: **none**.
- `cargo clippy -p kuna-decomp -p kuna-console --lib -- -D warnings`: clean. (The
  workspace `--tests -D warnings` build surfaces ~19 PRE-EXISTING lints in
  unrelated test files, e.g. `universalaction/tests.rs` const_is_empty — none in
  the diff or in the verifier's new files.)
- byte-parity goldens unchanged: `print_b5_boolless`, the `lift_diff` b2
  fixtures, and all `condconst_conn`/`readstruct` byte-identical assertions
  across the W10 verify suites ran and passed in the full workspace run.

## Adversarial tests (11, committed on the branch at `af78da1`)

rust/crates/kuna-decomp/tests/verify_w10_partial_types.rs (5):
  - union_facet_whole_union_minus_one_roundtrips (the -1 sentinel + union type
    + UNION_FACET category + dynamic mapentry)
  - union_facet_field_zero_boundary_roundtrips (field 0 lower boundary)
  - dynamic_symbol_size_from_type_and_lock_flags (size==7 from ct->getSize, not
    a constant; namelock|typelock applied)
  - dynamic_symbol_invalid_address_skips_range_but_keeps_symbol (the
    `!caddr.isInvalid()` guard)
  - opflags_constants_match_cpp_pcodeop_enum (the four new arms' bit-values
    pinned to op.hh)

rust/crates/kuna-console/tests/verify_w10_partial_types_console.rs (6, end-to-end
against the real partialunion corpus):
  - map_unionfacet_creates_real_union_facet_symbol (recovery lands a real
    UNION_FACET symbol in the loaded scope)
  - map_unionfacet_unknown_union_name_errors / _non_union_type_errors
    ("Bad union data-type" both halves of the guard)
  - map_unionfacet_field_index_out_of_range_errors (== numDepend) /
    _below_negative_one_errors (< -1) ("Bad field index", both bounds)
  - map_commands_require_a_loaded_function ("No function loaded" first guard)

All 11 pass; the console tests skip cleanly (no false green) when `.sla` is
absent.

losses: none — this is a full seam closure with real, measured parity gain and a
faithful transcription throughout.
