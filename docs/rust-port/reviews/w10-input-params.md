# w10-input-params
verdict: ACCEPT
verifier: opus-4.8 (independent)
date: 2026-06-15
branch: rport/w10-input-params (5325ec4; verifier tests committed at 06b9200)
gate: cargo test --workspace (debug) -> 165 `test result: ok` blocks, 0 failed, 0 panicked
      rust datatest suite (decomp_test_dbg over decompiler/datatests) -> 182/668 passing (== baseline)
      C++ oracle decomp_test_dbg -> 675/675 PARITY OK (untouched; diff is rust-only)
      cargo clippy -p kuna-decomp --lib -> clean (Finished, 0 warnings)

## Scope of the change

Single source change: `Architecture::decode_pentry_list`
(`rust/crates/kuna-decomp/src/architecture.rs:1308-1390`) now dispatches on a
`<group>` child of an `<input>`/`<output>` cspec element and parses each nested
`<pentry>` with `grouped == true` at a shared `basegroup`; `decode_pentry` gains
a `grouped` parameter threaded into `ParamEntry::seed`. This is a transcription
of C++ `ParamListStandard::decode` (`fspec.cc:1453`) + `parseGroup`
(`fspec.cc:1264`). `fspec.rs` is UNCHANGED by this item; only `architecture.rs`
plus one test file are touched.

Effect: register inputs declared inside a cspec `<pentry>` group (x86-64
System V / `__fastcall` integer + XMM-float slots) were previously dropped
because the `<group>` wrapper was silently skipped; the input ParamList held
only the stack-overflow entry, `possibleInputParam(reg)` was false, no trial
registered, and the recovery emitted `(void)` protos with raw register reads.
After the fix those slots are present and recover as parameters.

## Faithfulness — line-by-line against the C++

- `<pentry>`/`<group>` dispatch loop: matches fspec.cc:1479-1491
  (ELEM_PENTRY -> parsePentry grouped=false; ELEM_GROUP -> parseGroup).
- `basegroup = numgroup` capture; every grouped pentry parsed at `basegroup`
  with `grouped=true`: matches fspec.cc:1267-1272.
- numgroup advancement `maxgroup = getAllGroups().back()+1; if (maxgroup>numgroup)
  numgroup=maxgroup`: matches fspec.cc:1251-1253, replicated in BOTH the pentry
  arm and the group arm. (The `.unwrap_or(...)` fallback is provably dead:
  `group_set` is `vec![grp]` at construction and is only ever replaced by a
  non-empty set — resolve_join errors if empty, resolve_overlap replaces only
  when non-empty — so `get_all_groups()` is never empty after `seed`, matching
  C++'s unconditional `.back()`.)
- IPTR_JOIN rejection inside `<group>`: matches fspec.cc:1274-1275.
- `orderWithinGroup(previous1,cur)` then `(previous2,cur)`, with the
  `previous2=previous1; previous1=cur` shift: matches fspec.cc:1276-1282 exactly
  (index form). `order_within_group` body itself matches fspec.cc:585-597.
- `grouped` -> `IS_GROUPED` flag in `seed`: matches fspec.cc parsePentry/decode
  grouped handling (the `lastClass`/resource-section consumer in `push_entry`
  reads `isGrouped()`).

The two `numgroup` counters (the local one in `decode_pentry_list` feeding the
running group id during decode, and `ParamListStandard::numgroup` advanced by
`push_entry` feeding `finish_decode`'s `resource_start.push`) use the identical
update rule over the identical entries in the identical order, so they converge
to the same value — faithful to C++'s single member `numgroup` serving both
roles.

## Hunt list
- Signedness: clean. numgroup/basegroup/maxgroup are int4 == C++ int4; the only
  comparisons are int4 vs int4 (`maxgroup > numgroup`). No signed/unsigned mix.
- Integer widths: clean. group ids are tiny; `last()+1` cannot wrap (i32).
- Wrapping: clean. No uintb arithmetic in the diff.
- Comparator totality: n/a (no new comparator; order_within_group unchanged).
- Iteration order: clean. Both loops iterate `Element::get_children()` in
  document order == C++ decoder's sequential `peekElement` consumption. No
  HashMap/HashSet in the diff.
- Off-by-one / do-while: clean. C++ `while(peekElement()!=0)` over group children
  == Rust `for gchild in child.get_children()` filtering non-`<pentry>`. The
  `continue` on non-pentry is marginally more lenient than C++ (which would throw
  on a non-pentry element id inside `<group>`), but valid cspecs contain only
  `<pentry>` inside `<group>`, so behavior is identical for every real input;
  it guards stray whitespace/text nodes. Not a divergence in practice.
- Erase-while-iterating: n/a (no erase).
- Exception->Result partial-state: faithful. The two `Err` returns (IPTR_JOIN in
  group; order_within_group violation) propagate via `?` exactly where C++
  throws; the partially-built `pentries` is discarded the same way C++ unwinds
  past the partially-built `entry` member on a decode throw (the model is not
  registered on failure in either path).

## No special-casing
grep of the SOURCE-side diff for function-name/address/constant/register/type
hardcoding: NONE. The dispatch is purely on XML element name (`"pentry"`/
`"group"`) and the generic `IPTR_JOIN` space-type. Names like `modulo2`/`a0`/
`RCX`/`mod2` appear ONLY in the test file's assertions (verifier/oracle
territory), never in `architecture.rs`.

## Mechanical pass
- grep source diff for `todo!`/`unimplemented!`/`HashMap`/`HashSet`/
  `sort_unstable`/`unwrap()`/`expect(`: NONE.
- bare `as` casts in source diff: NONE (the two `as` hits are inside doc
  comments, not casts).
- clippy -p kuna-decomp --lib: clean.

## Real-parity evidence (vs C++ oracle B5)
modulo2 (UNLOCKED proto — recovery is model-driven):
  baseline rust-port: `uint8 mod2(void) { v1 = ZEXT(SUB(RCX,0) % 2); ... }`
                      (raw RCX/ECX reads in all four modN bodies)
  branch:             `uint8 mod2(xunknown8) { v1 = ZEXT(SUB(a0,0) % 2); ... }`
                      (input bound to recovered param `a0`, no raw register read)
  oracle:             `int4 mod2(int4 a0) { return a0 % 2; }`
  => the recovery half is REAL and matches the oracle's param identity (`a0`).

mixfloatint (multi-slot int+XMM-float group):
  baseline: `xunknown4 dldlll(int4,int4)`            (arity 2, truncated)
  branch:   `xunknown4 dldlll(xunknown4,int4,float8,int4,int4,int4)` (arity 6)
  oracle:   `float8 dldlll(float8 a0,int4 a1,float8 a2,int4 a3,int4 a4,int4 a5)`
  => the full 6-arg arity (incl. the float slot) is recovered, matching the
     oracle; the body now references a1..a5 instead of EDX/XMM/R9D.

## No regression
- Rust datatest suite: 182/668 passing on BOTH baseline (rust-port @ c229df1)
  and branch; per-test Success/FAIL lists are byte-identical (diff empty) — no
  assertion flipped in either direction.
- Whole-corpus body sweep (all 83 datatests, branch binary vs baseline binary,
  KUNA_DUMP bodies): exactly TWO bodies changed — `modulo2` and `mixfloatint`,
  both moving TOWARD the oracle, both already-failing the same assertions before
  and after (no passing test regressed).
- boolless / readstruct / condconst_conn: bodies byte-identical to baseline (the
  one apparent boolless "diff" was solely the DUMP header echoing the `-path`
  arg; bodies match byte-for-byte).
- verify_w10_typed_access parity fence: 8/8 pass.
- C++ oracle: 675/675 PARITY OK, untouched (diff is rust-only).

## Divergence (out of scope, not a new loss)
The recovered params are NOT yet fully cleaned/typed: `xunknown`/`uint8` instead
of `int4`/`float8`, and the `ZEXT`/`SUB(a0,0)`/`XMM0_Db` high-half residue
survives (modulo2/mixfloatint datatest assertions, which require the fully
cleaned `return a0 % N`, still FAIL — which is why the suite count is unchanged).
This is the downstream type-inference / cast / sized-stack plane
(ActionSetCasts + type recovery), an explicitly separate, pre-existing seam
family (LOSS-154 / LOSS-169 / LOSS-172). It is a CONTINUATION of work this item
does not own — it neither weakens any prior parity nor diverges from the C++
algorithm this item ports. No new LOSS attributable to w10-input-params.

## Adversarial tests added
rust/crates/kuna-decomp/tests/verify_w10_input_params.rs (committed 06b9200):
  - modulo2_register_input_recovers_as_param_not_void
       (teeth: FAILS on baseline — baseline emits `modN(void)` + raw RCX/ECX)
  - modulo2_recovered_param_name_matches_oracle
       (faithfulness: Rust param identity `a0` agrees with the C++ oracle)
  - mixfloatint_multislot_group_recovers_full_arity
       (teeth: FAILS on baseline — baseline `dldlll(int4,int4)` arity 2 vs 6)
  - unrelated_models_keep_exact_signature
       (no-regression fence: boolless/nanops/promote_compare exact signatures +
        run-to-run determinism)
All 4 pass on the branch; the two modulo2/mixfloatint tests demonstrably fail
on the rust-port baseline (verified against captured baseline dumps).

findings: none (no correctness finding; the type-plane residue is out-of-scope
          continuation, not a defect in this item's transcription).
losses: none (ACCEPT, not ACCEPT-WITH-LOSSES — no parity weakened, no divergence
        from the ported C++ algorithm introduced by this item).
