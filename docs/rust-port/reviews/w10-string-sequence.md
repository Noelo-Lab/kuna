# Heap-string sequence (w10-string-sequence) — ACCEPT, +7

Wave (2026-06-17) integrated at rust-port (merge of `rport/w10-string-sequence` @ `5362478`,
base 074f199/441). Verdict: **ACCEPT**. Resolves the heapstring LOSS (heap half).

## What landed (faithful transcription of constseq.cc:486-1029 + the string infra)

- `HeapSequence` (STORE driver) + `RuleStringStore::apply_op` (`constseq.rs`): findBasePointer/
  findDuplicateBases/findInitialStores/collectStoreOps/buildStringCopy/gatherIndirectPairs/
  deduplicatePairs/removeStoreOps/transform. Restored the faithful `select_string_copy_function`
  charType-identity selection.
- `Funcdata::get_internal_string` (`funcdata_varnode.rs`, port of funcdata_varnode.cc:1434):
  registers into a persistent `StringManager` and builds the `BUILTIN_STRINGDATA` CALLOTHER.
- Persistent string manager + seam threading (`architecture.rs`
  `string_manager: Rc<RefCell<StringManagerUnicode>>`, `register_string_builtins`,
  `decode_data_organization`; `seams.rs` `internal_strings`).
- Printer `op_callother_ir` (functional/annotation/no_operator/display_string) +
  `print_character_constant` rewired to the persistent manager (`printc.rs`, port of printc.cc:693).
- **Load-bearing cast fix:** `output_type_local` returns the STRINGDATA op's own locked
  char-pointer output type (`coreaction_infertypes.rs`, mirrors `InternalStringOp::getOutputLocal`
  userop.cc:362) — without it ActionSetCasts reassigned the STRINGDATA output to a fresh UNKNOWN
  unique and broke the literal render.
- `<data_organization>` size setters (`dtype.rs`) so x86-64 gcc `wchar_size=4` picks
  `builtin_memcpy(L"...")` over `wcsncpy`.

## Effect

441 → 448 on its base; +7 on the integrated 442 tree → **449**. Gained: Heap string #1-7 (full
parity 7/7).

## Gate (re-run at integration onto 442)

- `cargo test --workspace` (debug): 0 failures.
- datatests: `[675, 449]`; passing-set diff vs base 442: **regressed-set EMPTY** (gained Heap
  string #1-7). printc.rs auto-merged with the longdouble stride change.
- switch cluster 8/16/3 unchanged; oracle **PARITY OK**; C++ tree byte-untouched.

## Remaining: stackstring (14) — separate next-locus

The COPY-into-array `StringSequence` driver (`constseq.cc:188-483`) + `RuleStringCopy::apply_op`
wiring. Prerequisites: the address-only `beginLoc(addr)/endLoc(addr)` loc-set overload for
`collectCopyOps`, `getScopeLocal()->queryContainer` (already at database.rs:2618), and
`constructTypedPointer`'s `constructSpacebaseInput`/`constructConstSpacebase` +
`getTypePointerStripArray` PTRSUB/PTRADD builder. NOTE: the stack COPYs are dead-code-eliminated
(cf. LOSS-229), so RuleStringCopy must fire before that removal.

## No special-casing

General string-sequence machinery driven by the STORE/charType structure; no string/address
hardcoding.
