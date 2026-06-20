# w10-float-cluster
verdict: ACCEPT
verifier: independent verifier (Opus 4.8, 1M)
date: 2026-06-16
round: 1
diff base: `rust-port...rport/w10-float-cluster` (three-dot merge-base `b5277eb`)
gate: `cargo build --workspace` -> green; `cargo test --workspace` -> 184/184 suites ok, 0 failed; `cargo clippy --lib --workspace` -> clean (0 warnings); C++ oracle `KUNA_ENGINE=cpp python -m kuna.run_tests --all --baseline docs/baseline.json` -> 207/207 unit + 675/675 datatest, PARITY OK (untouched — diff is Rust-only)

## Scope

Branch ports the read-only-global constant fold + the float-constant render +
the float-lane naming seam. Two commits:

- `0629eaf` ActionVarnodeProps/fillinReadOnly: ports the readonly/volatile half
  of `ActionVarnodeProps::apply` (coreaction.cc:1298) and the three `Funcdata`
  replace primitives — `fillin_read_only` (funcdata_varnode.cc:653),
  `total_replace_constant` (1517), `replace_volatile` (733). Wires the engine
  bootstrap `fillinReadOnlyFromLoader` (architecture.cc:1375) + `readonlypropagate`
  carry into the per-function seam.
- `76d2e7d` float-lane naming + float render: `linkSymbol`/`handleSymbolConflict`
  (funcdata_varnode.cc:1177/1018) modeled in `name_local_highs_angr`; `PrintC`
  float-constant arm -> `push_float` (printc.cc:1448) via `push_float_ir`;
  `setDefaultFloatFormats` gate (translate.cc) in `Architecture::init`.

Diff touches ONLY `rust/` (9 files; `git diff --name-only | grep -v '^rust/'`
empty). C++ tree, specs, and `docs/baseline.json` untouched.

## Faithfulness (line-for-line against the cited C++)

CONFIRMED faithful — read each C++ source and matched it against the Rust:

- `fillin_read_only` (funcdata_varnode.cc:653-727): the written/marker-warning
  branch, the `getSize() > sizeof(uintb)` precision guard, the `loadFill` ->
  `DataUnavailError`/`clearFlags(readonly)` catch, the big/little-endian byte
  accumulation (`res<<=8; res|=bytes[i]` order preserved), the typelock
  pass-through (`updateType(locktype,true,true)`), and the descend loop with the
  INDIRECT->COPY marker rewrite (`opRemoveInput(op,1)`+`opSetOpcode(COPY)`, the
  `outvn->getAddr()==vn->getAddr()` self-indirect skip) are all present and in
  C++ order. The constant value is assembled FROM the loaded bytes, never a
  literal.
- The bytes are REAL: `Architecture::loader_fill` -> `LoadImageXml::load_fill`
  (loadimage_xml.rs:352) reads from the loaded `chunk` map (the actual image
  bytes), not a stub. The engine bootstrap collects `loader.get_readonly(...)`
  ranges and `symboltab.set_property_range(readonly, first, last_open)` — an
  exact mirror of `fillinReadOnlyFromLoader` (architecture.cc:1375-1387).
- `total_replace_constant` (1517): the single-shared-COPY-for-marker logic, the
  written (`opInsertAfter`) vs block-0 (`opInsertBegin`) entry cases, the
  non-marker `newConstant` path — order-exact.
- `ActionVarnodeProps::apply` (coreaction.cc:1298-1363): the full
  beginLoc..endLoc walk, the AutoLiveHold LOAD-through-const/readonly-pointer
  skip (incl. the one-COPY indirection), the `hasActionProperty` readonly-fold /
  volatile-replace gate on `cachereadonly`, and the NZmask-disjoint const-fold
  with the COPY-of-0 anti-recursion guard. The C++ `continue` in the LOAD branch
  is modeled with a `skip` flag that gates exactly the `clearAutoLiveHold`+count;
  semantically equivalent (both fall through to the next vn with no clear).
- `push_float_ir` (printc.rs:3786) vs `PrintC::push_float` (printc.cc:1448):
  `FLOAT_UNKNOWN` for no format, signed INFINITY/NAN sentinels, the
  `printDecimal` decimal path with `force_scinote` honoured — exact. The `.0`
  fix-up + looksLikeFloat detection live in the pre-existing `format_float_token`
  (not in this diff), reused correctly. The constant arm switch on
  `TYPE_FLOAT` metatype mirrors `pushConstant` (printc.cc:1860).
- `compareName`/`getNameRepresentative` (variable.cc:456/492) preference chain is
  the real one; `query_container_for_link` uses the real `find_container`
  (database.rs:2128) = C++ `queryProperties(addr,1)`->`findContainer`; the
  `handleSymbolConflict` reuse predicate (`isInput||isAddrTied||isPersist||
  isConstant`) is faithful.

## Special-casing audit

NONE. The folded float constants come from loaded read-only bytes, NOT literals
in `src/`:
- `grep` of the src-only diff (non-comment) for `1.1234567812345`/`1.12345678`/
  `9.8765432198765`/`0x3ff1f9*`/`0x4023c0*`/`0x3f80*`/`0x3ff0*` -> EMPTY. Every
  occurrence of those values is in doc comments or in the adversarial test file
  (as expected-output / adversarial-input), never on the runtime path.
- `grep` for `name()=="..."`/`get_offset()==0x...`/`"main"`/`"prec`/address-
  equality branching -> EMPTY. No function-name, address, or constant-value
  hardcoding anywhere in src.
- `grep` for `=="`/`.contains("`/`hardcod`/`magic`/`TODO`/`HACK`/`special-case`/
  `0x100080`/`0x100088` in non-comment src -> EMPTY.

## New passes are REAL parity (re-measured)

Measured the rust-engine datatest suite (`KUNA_ENGINE=rust run_tests
--datatests`) with the float-cluster `decomp_test_dbg` vs a freshly-built base
(`b5277eb`) `decomp_test_dbg` swapped in at the resolved path:

- base (`b5277eb`): **242/672** passing.
- float-cluster: **263/672** passing. (+21, matches the commit claim.)
- Failing-SET diff (base vs branch): **21 `<` lines (newly passing), 0 `>` lines
  (regressions).** The newly-passing assertions are exactly: Floating-point cast
  #1-#4 (4), Float print #1-#14 (14), Long double #1/#13 (2), Relative base #1
  (1) = 21. These are genuine — the floatcast `v1 = 1.1234567812345` /
  `v2 = 1.12345678` constants (#3/#4) now match the C++ oracle B5, and the float
  literals are decoded from the loaded encoding.

## No-regression (whole suite)

- **Whole-suite passing >= 242: 263 >= 242. Every previously-passing assertion
  still passes** (failing-SET diff has ZERO `>` lines, so the base passing set is
  a strict subset of the branch passing set).
- boolless / condconst_conn: BYTE-IDENTICAL between base and branch (per-name
  harness output diff empty). `readstruct` is a test name inside
  `nestedoffset.xml`; absent from both failure sets => passing in both, no
  regression.
- `cargo test --workspace`: 184 suites ok, 0 failed; 5 `w10fc_*` adversarial
  tests pass. No `#[ignore]`/`should_panic` added (no fabricated passes).
- `cargo clippy --lib --workspace`: clean.
- C++ oracle: 207/207 unit + 675/675 datatest, PARITY OK; `git diff -- decompiler/`
  empty.
- B0 list-action order: `universalaction.rs`/`coreaction.rs` UNCHANGED by the
  diff; `universalaction_listing` test passes.

## Modeling note (not a blocker, not a loss)

`name_local_highs_angr` gates the `handleSymbolConflict` conflicting-HighVariable
scan behind a `size_mismatch = v_size != entry_size` check (coreaction_cleanup.rs)
— a documented approximation of the C++ outcome. C++ `handleSymbolConflict`
(funcdata_varnode.cc:1018) would also detect a same-width conflict if a different
high occupied the exact slot; the porter argues equal-width siblings are already
merged in faithful C++, so the gate cannot drop a reachable conflict. This is a
structural simplification gated on storage WIDTH, NOT on any value/name/address,
so it is not special-casing; and it produces zero measured regression and +21
correct gains with byte-identical previously-passing output. Faithful-enough,
clearly marked, justified. Recorded here for the record; no new loss entry.

## Adversarial tests (committed, 76d2e7d)

`rust/crates/kuna-decomp/tests/verify_w10_float_cluster.rs` — 5 tests:

- `w10fc_format_float_token_covers_all_classes` — every class incl. signed
  INFINITY/NAN, FLOAT_UNKNOWN, and the `.0` / force-scinote fix-up.
- `w10fc_float8_constant_decoded_from_encoding_not_hardcoded` — the anti-special-
  casing core: `1.1234567812345`/`1.12345678` are DECODED from the raw IEEE-754
  encodings, and a DIFFERENT encoding (`9.8765432198765`) renders a DIFFERENT
  literal (proves value tracks bits).
- `w10fc_float8_decode_roundtrips_encoding` — `host->encoding` reproduces the
  bits (guards against a lossy by-value shortcut).
- `w10fc_float8_special_classes_render_sentinels` — inf/nan encodings -> sentinel
  tokens, not stray decimals.
- `w10fc_default_float_formats_decode_both_sizes` — `FloatFormat::new(4/8)`
  resolve+classify (the `setDefaultFloatFormats` gate that lets push_float emit a
  literal instead of FLOAT_UNKNOWN).

All 5 pass; >=3 requirement met.

findings: none (no correctness findings).

losses: none.
