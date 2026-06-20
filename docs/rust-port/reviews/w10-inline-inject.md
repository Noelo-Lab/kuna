# w10-inline-inject
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier, round 1)
date: 2026-06-15
gate: `cd rust && cargo test --workspace` -> all suites green, 0 failed (incl. the new
      verify_w10_inline_inject.rs 9/9); `cargo clippy -p kuna-decomp --lib` /
      `-p kuna-console --lib` clean; C++ oracle `python -m kuna.run_tests --all
      --baseline docs/baseline.json` -> 207/207 unit + 675/675 datatest, PARITY OK.

## Scope

Closes the inline/injection/override exec-failure seam. The Rust diff (rust/ only;
zero C++/specs change) ports, against the in-scope C++:

- `Funcdata::overrideFlow` (funcdata_op.cc:969-1021) -> funcdata_op.rs `override_flow`.
- the flow.cc flow-override processing (flow.cc:43,433-434,492-493) -> flow.rs
  `FlowInfo::new` + `process_instruction` reading `data.get_override()` not the env seam.
- `Override::stringToType`/`insertFlowOverride`/`getFlowOverride`/`hasFlowOverride`/
  `insertProtoOverride` USAGE (override.cc/.hh — the store itself was ported in
  w4-p0-override; this diff only consumes it) via the new `Funcdata::localoverride`.
- the per-function inline/noreturn/inject-id properties (`OptionInline`/`OptionNoReturn`/
  `IfcFixupApply` setters + `FlowInfo::queryCall`-time readers) parked on the
  FunctionSymbol kind (database.rs), `query_global_function` (architecture.rs).
- cspec `<callfixup>` decode/registration: `Architecture::decode_call_fixups`
  (architecture.cc:1290-1291 parseCompilerConfig CALLFIXUP arm) feeding the
  w3-ported `decodeInject`->`registerInject`->`registerCallFixup` chain so
  `getPayloadId(CALLFIXUP_TYPE,name)` resolves for `fixup apply`.
- the console commands `IfcProtooverride`/`IfcFlowOverride`/`IfcFixupApply`
  (ifacedecomp.cc:1840-1866,1932-1953,2969-2999) made real (were `engine_unavailable`).

## Mandatory hunt list

- Signedness: clean. `injectid < 0` is `int4` vs literal-0 (C++ `if (injectid<0)`);
  `for i in 0..fd.num_calls()` is `int4` loop over a non-negative `int4` count
  (matches C++ `int4 i; i<numCalls()`). No int4-vs-uint compare introduced.
- Integer widths: clean. flow types are `uint4` small enums (0..=4, the override.hh
  enum); `injectid`/`SymbolId` widths match (`int4`); no implicit-promotion sites.
- Wrapping: clean. The only arithmetic added is `new_constant(1, 0)` (literal 0) and
  `addr.clone()`; no `uintb`-lineage op. (The `0x57AB12CD<<32` in funcdata.rs:290 is
  pre-existing, not in this diff.)
- Comparator totality: N/A — no new `Ord`/comparator; the override store reuses the
  w4-ported `BTreeMap<Address,_>`.
- Iteration-order provenance: clean. (1) `override_flow` walks `iter_at(addr)` =
  `optree.range([SeqNum(addr,0), SeqNum(addr,uintm::MAX)])` in SeqNum order ==
  C++ `beginOp(addr)..endOp(addr)` PcodeOpTree (map<SeqNum,_>) order; collected into a
  Vec first only to release the `&self` borrow, then walked in the same order.
  (2) `decode_call_fixups` filters `root.get_children()` (XML document order) for
  "callfixup" and decodes in that order; each `decode_inject` allocates a fresh id
  (`injection.len()`), so ids ascend in document order — injectids are name-resolved
  opaque handles (no raw-id dependency anywhere in the corpus), so the separation from
  the C++ interleaved-with-callother allocation is benign.
- Off-by-one / do-while / reverse iter: clean. No do-while ported here; the
  `findPrimaryBranch` walk and the override arms are straight-line.
- Erase-while-iterating: N/A — no erasure in any ported body.
- Exception->Result partial-state: clean and faithful. `override_flow` returns the C++
  `LowlevelError` messages verbatim ("Could not apply flowoverride", "Do not currently
  support CBRANCH overrides", "Do not currently support complex overrides") at the same
  points, leaving the op graph in the same partial state (the errors fire BEFORE any
  rewrite in the relevant arm; the CALL_RETURN insert happens only after the rewrite,
  same as C++). `IfcFixupApply` error messages ("No load image present"/"Missing fixup
  name"/"Missing function name"/"Unknown fixup: "/"Unknown function name: ") match.

## Mechanical pass

- grep diff for `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` in the 7
  touched files: zero hits.
- grep diff for bare numeric `as` casts in added lines: zero (only `as_mut`/`as_ref`/
  `as_bytes`/`as_payload`).
- special-casing grep (function-name/address/value/type-name hardcoding): clean. Every
  string literal in the diff is a C++-faithful XML element/attribute name or keyword
  ("callfixup" == ELEM_CALLFIXUP pcodeinject.cc:29; "nocode" == ifacedecomp.cc:678;
  "negative"/"c-language"/"processor_spec" are pre-existing; "func" is the C++
  `FuncProto::printRaw` literal). No binary/function/address is decoded by a hardcoded
  fact. Confirmed by the adversarial name-keyed-lookup test (unknown name -> -1).
- port-audit: not present in this workspace (`package port-audit not found`) — skipped.
- clippy `-p kuna-decomp --lib` and `-p kuna-console --lib`: clean.

## Recovery is REAL (matches the C++)

- `override_flow`: all four arms transcribed statement-for-statement vs
  funcdata_op.cc:976-1020; `find_primary_branch(&ops, findbranch, findcall, findreturn)`
  argument order matches the C++ `findPrimaryBranch(iter,enditer,...)` exactly for
  BRANCH(false,true,true)/CALL(true,false,true)/CALL_RETURN(true,true,true)/
  RETURN(true,true,false). Verified by re-derivation + 6 new override_flow tests.
- call-fixup registration is genuine name->id: `decode_call_fixups` -> `decode_inject`
  -> `register_inject` -> `register_call_fixup(name,id)` populates `call_fixup_map`,
  which `get_payload_id(CALLFIXUP_TYPE,name)` reads (pcodeinject.cc:285-309 faithful).
  The `guard_dispatch_icall` callfixup exists in x86-64-win.cspec:228 and is registered
  by this path. Verified by the name-keyed adversarial test.
- the flow-override re-seed: `build_and_follow_flow_with_override` inserts the stashed
  `(addr,type)` onto the fresh Funcdata's `localoverride` BEFORE `FlowInfo::new`, which
  reads `data.get_override().has_flow_override()`; `process_instruction` then reads
  `data.get_override().get_flow_override(curaddr)` and calls `data.override_flow(...)` —
  the exact flow.cc:43,434,493 cadence.

## Byte-parity / prior-function regression check

- boolless full-byte-parity, condconst_conn forms-andand byte-identical, readstruct
  symbol-naming: all green in the workspace run (`w10_boolless_full_byte_parity_*`,
  `w10_eob_condconst_conn_forms_andand_byte_identical`,
  `verify_w10_symbol_naming_readstruct_*`). No prior byte-parity function broken.
- C++ oracle 675/675 PARITY OK, untouched (diff is rust/ only).

## Findings

  - F1 (low / cosmetic — LOSS-164): `IfcProtooverride::execute` prints a trailing
    "Successfully added override\n" that the C++ does NOT emit (C++ ends silently at
    `dcp->fd->clear()`; only `IfcFlowOverride` prints that line), and omits the C++
    `dcp->fd->clear()`. Zero gate/corpus impact: the Rust datatest harness compares the
    `print C` text only, not the command-echo stream, so no `stringmatch` is affected;
    the only datatest using `override proto` (injectoverride.xml) is a pre-existing known
    loss (the install is W4-qlst-seamed — `Funcdata::num_calls` reads `qlst` which
    `FlowInfo::setup_call_specs` does not populate, LOSS-037 — so the command throws
    "No call is made at this address" before reaching the success line) and the
    inject-body/applyPrototype consume is seamed (LOSS-031). The `clear()` omission is
    moot in the kuna console rebuild model.
       cpp: decompiler/cpp/ifacedecomp.cc:1840-1866 (no success line; ends at clear())
       rust: rust/crates/kuna-console/src/ifacedecomp.rs (IfcProtooverride::execute —
             the extra status.out + missing clear)

## Notes on declared losses (pre-existing, not regressions)

- The SLEIGH inject-body compile (`parseInject`) is deferred (LOSS-031); `register_inject`
  registers the name but does not compile the body. This is exactly the C++ ALL-
  decode/registration-lookup-ported boundary recorded in LOSS-031, and is all
  `getPayloadId`/`setInjectId` need. The `<callfixup>` requires a `<pcode>` subtag
  (InjectPayloadCallfixup::decode throws otherwise) — faithful, w3-ported.
- The qlst/FuncCallSpecs population (W4) and `Override::applyPrototype` consume are
  seamed (LOSS-037), so `override prototype`/`override flow callreturn`/`fixup apply`
  cannot complete end-to-end on this branch; the new code is faithful up to those W4
  seams. injectoverride.xml / inline.xml / multiret.xml remain C++-oracle-only.

## Adversarial tests added (committed on rport/w10-inline-inject @ 64e35c2)

rust/crates/kuna-decomp/tests/verify_w10_inline_inject.rs (9 tests, all green):
  - w10_override_flow_return_rewrites_branchind (BRANCHIND -> RETURN arm)
  - w10_override_flow_return_rewrites_callind (CALLIND -> RETURN arm)
  - w10_override_flow_return_on_call_is_complex_error ("complex overrides" + op unchanged)
  - w10_override_flow_call_on_cbranch_is_cbranch_error ("CBRANCH overrides" + unchanged)
  - w10_override_flow_call_return_branchind_becomes_callind_plus_return (combined arm +
    exactly-one inserted RETURN)
  - w10_string_to_type_exhaustive_and_unknown_is_none (4 real + 8 unknown->NONE; the
    no-special-casing guard; distinct 1..=4 enum)
  - w10_override_store_flow_roundtrip_and_miss (insert/get/has + exact-address miss)
  - w10_funcdata_override_reseed_visible_through_getoverride (the console rebuild re-seed)
  - w10_callfixup_getpayloadid_is_name_keyed_no_special_casing (name->id, unknown->-1,
    namespace no-leak, duplicate-name hard error — proves no address/value hardcode)

losses: LOSS-164 (the F1 cosmetic divergence; appended to docs/rust-port/losses.md).
