# w9-con-codedata
verdict: ACCEPT-WITH-LOSSES
verifier: Claude Opus 4.8 (1M context) [verifier subagent]
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-console` -> ok, 70 passed / 0 failed (5 verifier tests added)

## Scope
- C++: decompiler/cpp/codedata.cc, codedata.hh (blob SHAs MATCH checklist:
  cc 4a4235ca…, hh 29e676d7… — no drift).
- Rust diff: commit 2dd12da `rport/w9-con-codedata` — rust/crates/kuna-console/src/codedata.rs (+1858).
- Branch worktree-wf_a7f2562d-46c-12 (worktree present).

## Hunt list
- **Signedness** — clean. The only int4-vs-uintb comparisons are
  `pushTaintAddress`' `key+size-1 < addr` (codedata.cc:157) and `disassembleBlock`'s
  `wholesize < 10` (codedata.cc:347). Both reproduce the C++ exactly: the former via
  `key.get_offset().wrapping_add(cu.size as u64).wrapping_sub(1)` (int4→uintb
  sign-extend, then uintb compare, matching the C++ implicit promotion); the latter is a
  signed i32 compare on a value computed by `wrapping_sub(...) as int4`. Verified by
  `verify_w9_push_taint_address_containment_boundary`.
- **Integer widths** — clean. `int4`→`i64` for `Address + off` (`operator+(int8)`),
  `int4`→`u64` for the offset arithmetic, `uintb`→`int4` for the `dumpModelHits` diff
  cast. Each is the C++ implicit conversion, not a "fixed" variant.
- **Wrapping** — clean. All `uintb`-lineage arithmetic uses `wrapping_*`
  (codedata.rs:481,501,516,732,1075,1146) or routes through `&Address + i64` which wraps
  via `wrap_offset` (kuna-base, verified). The `dumpModelHits` `(int4)(off-endoff)` is
  `wrapping_sub(...) as int4` — exact C++ low-32-truncate-and-reinterpret.
- **Comparator totality** — clean. `AddrLink::cmp` (codedata.rs:390) is a→b lexicographic
  on the (total) `Address::cmp`; no floats. `targethits.sort_by(|a,b| a.funcstart.cmp(...))`
  matches `list<TargetHit>::sort` using `operator<` (funcstart only) and is stable
  (std::list::sort is stable; sort_by is stable). Verified by `addrlink_orders_on_a_then_b`.
- **Iteration-order provenance** — clean, per loop. Every container is a `BTreeMap`/
  `BTreeSet`/`Vec` mirroring the C++ `std::map`/`std::set`/`std::list`/`vector`; NO
  HashMap/HashSet anywhere (grep clean). `codeunit` (map key order), `fromto_crossref`/
  `tofrom_crossref` (AddrLink key order), `modelhits` (Range set order), `targethits`/
  `unlinkedstarts`/`taintlist` (insertion order), `targets` (Address key order) all
  preserved. The `mark_*`/`find_*` loops snapshot keys (`keys().cloned().collect()`)
  then re-fetch — BTree key order = C++ map order. Verified by
  `verify_w9_find_unlinked_flag_mask_classification` (dump order) and the dump tests.
- **Off-by-one / do-while / reverse iterators** — clean.
  * `runModel` `do { } while(moresections)` (codedata.cc:688) → `loop { … if !moresections break }`
    with the size==0 `continue` correctly routed to the `while(moresections)` check
    (codedata.rs:1144-1171). At-least-once preserved.
  * `findFunctionStart` `lower_bound` + reverse `--iter` (codedata.cc:594-600) →
    `range(..single(addr)).next_back()` loop — the `single(addr)` (b=invalid, smallest)
    boundary correctly EXCLUDES on-cut `a==addr` entries from the reverse walk. Verified
    by `verify_w9_find_function_start_lower_bound_reverse_walk` (this test initially
    encoded my own wrong oracle and FAILED, then re-derivation confirmed the PORT is
    correct and my expectation was wrong — see Findings note).
  * `pushTaintAddress` `upper_bound`+`--iter` → `range(..=addr).next_back()` (last ≤ addr). Match.
  * `dumpModelHits` `++iter; if(iter!=enditer)` lookahead → `peekable().peek()`. Match.
- **Erase-while-iterating** — clean. `processTaint`'s `tofrom_crossref` drain
  (`pushTaintAddress((*ftiter).first.b); ++ftiter; erase(diter)`, codedata.cc:188-193)
  → collect-then-(push+remove); `pushTaintAddress` does not mutate `tofrom_crossref`, so
  collect-first is order-equivalent. `disassembleBlock`'s `++iter; erase(iter2)` notcode
  size-1 repair (codedata.cc:303-305) → `range((Excluded(badkey),Unbounded)).next()`
  before `remove(badkey)` (codedata.rs:685-690). `clearCrossRefs`/`clearCodeUnits`/
  `findOffCut`'s `upper_bound(addrlink)` re-seek after `repairJump` all collect-then-erase
  or re-seek correctly. Verified by `verify_w9_process_taint_spreads_along_tofrom_and_erases`.
- **Exception → Result partial-state parity** — clean. The one throw site,
  `addTargetHit`'s "Found thunk without a feature mask" (codedata.cc:441), is
  `Err(IfaceError::execution(...))` propagated via `?`. C++ throws BEFORE
  `targethits.emplace_back`, so no partial mutation; the Rust `?` returns before the
  `push`. The caller (`findUnlinked` via `runModel`) lets it escape to `execute()` —
  matches C++ where the throw unwinds out of `runModel`. `DisassemblyEngine::disassemble`
  reproduces the C++ try/catch (BadData/DataUnavail → success=false; Unimpl → length)
  exactly; a 4th `Err(_)` arm maps unexpected errors to a failed decode rather than a
  panic (defensive; C++ would let an unlisted throw escape — unreachable from oneInstruction).

## Console-output / parse grammar (this wave's focus)
- `IfcCodeDataTarget` error text "Missing system call name" and output
  "Unable to find reference to call <token>\n" — byte-exact (endl→\n). The `s>>ws`/eof
  → IfaceParseError and `s>>token` (whitespace-delimited `read_token`) grammar matches.
  Pinned by `target_command_*` tests.
- `dumpModelHits` "0x<first> 0x<last>[ <dec-gap>]\n" multi-line sequential grammar with the
  signed-narrow gap — pinned exactly by `verify_w9_dump_model_hits_multiline_gap_grammar`
  (3-range, 1-range, empty).
- `dumpCrossRefs` "0x<a> -> 0x<b>[ call]\n", `dumpFunctionStarts` "0x<a>\n",
  `dumpUnlinked` "0x<off>\n", `dumpTargetHits` "<func|nostart> <addr> <name>\n"
  (hex, no 0x — matches C++ which omits the "0x" there) — pinned by the run_model tests.
- Command prefix/registration: `register_commands` registers the 8 commands in the C++
  `registerCommands` order with identical word-paths (init/target/run/dump{hits,crossrefs,
  starts,unlinked,targethits}). Pinned by `registration_creates_codedata_module_data`.

## Mechanical pass
- `cargo clippy -p kuna-console --tests -- -D warnings` — clean.
- grep todo!/unimplemented!/unreachable! — none. HashMap/HashSet — none. sort_unstable — none.
- bare `as` casts (13, codedata.rs:257,481,501,516,566,671,732,741,794,971,1023,1075,1146):
  each is a faithful C++ implicit conversion (int4→i64 for Address+off; int4→u64 / uintb
  cast for offset arithmetic; uintb→int4 for the dump diff). All carry an adjacent C++-anchor
  comment but NOT the literal `// cast:` token the protocol's mechanical pass prescribes —
  same minor-convention gap recorded for w9-con-grammar (LOSS-126). Recorded as F2 (minor),
  not a blocker; the cast SEMANTICS are all verified correct.
- port-audit: crate `port-audit` is absent from this workspace — not run (blob SHAs hand-verified instead).

## Findings
- F1 (none / informational): `verify_w9_find_function_start_lower_bound_reverse_walk`
  initially asserted `find_function_start(0x80) == 0x80`. Re-derivation against
  codedata.cc:594-599 showed the real `tofrom` key `pair(0x80,0x700)` is ≥ `single(0x80)`,
  so C++ `lower_bound` lands ON it and the reverse `--iter` walk SKIPS it → the call AT
  0x80 is not seen for addr==0x80; correct oracle is 0x10. The Rust `range(..single(addr))`
  reproduces this exactly. **No port bug** — my verifier oracle was wrong; corrected and
  the test now pins the boundary EXCLUSION for both sides.
- F2 (minor, convention): 13 bare `as` casts lack the `// cast:` justification token
  (cpp: codedata.cc, multiple; rust: codedata.rs:257,481,…,1146). Cast semantics verified
  correct; convention-only, mirrors LOSS-126.
- L1 (loss): BFD import-table lookup in `IfcCodeDataTarget` unported (GPL-3) — `target`
  always reports not-found. cpp: codedata.cc:732-754; rust: codedata.rs:1254-1266. → LOSS-128.
- L2 (loss): `codedata init`/`run` bodies stubbed to an execution error pending the
  `decompile` IfaceData/Architecture wiring (the analysis itself is fully ported and
  tested directly). cpp: codedata.cc:726-760; rust: codedata.rs:1224-1288. → LOSS-129.

## Adversarial tests added (committed 12e7cfb on the branch)
- verify_w9_find_function_start_lower_bound_reverse_walk
- verify_w9_process_taint_spreads_along_tofrom_and_erases
- verify_w9_push_taint_address_containment_boundary
- verify_w9_dump_model_hits_multiline_gap_grammar
- verify_w9_find_unlinked_flag_mask_classification

## losses
LOSS-128 (BFD import-table lookup unported), LOSS-129 (init/run engine seam).
Both are item-specific instances of the established LOSS-005 (libbfd) / LOSS-127
(ifacedecomp engine seam) patterns; every divergent path is unreachable through the
datatest/harness oracle (`codedata` appears in zero datatests — grep-verified) and the
ported analysis (`CodeDataAnalysis::*`) is byte-faithful end-to-end against a synthetic
Translate/LoadImage.

verdict: ACCEPT-WITH-LOSSES — the analysis transcription is faithful across the entire
hunt list (verified by re-derivation + 5 adversarial tests + the porter's 10); the only
divergences are two cited, unreachable, integration-deferred losses and one minor cast-
comment convention gap.
