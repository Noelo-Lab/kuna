# Rust port loss ledger

**Append-only.** Every deliberate departure of the Rust port from the C++
oracle gets a numbered entry here at the moment the decision is made (porter or
verifier appends; nobody edits or deletes an existing entry — supersede with a
new entry that references the old id). Verifiers cite entries by id in
`ACCEPT-WITH-LOSSES` verdicts.

Entry schema (every field required):

```
## LOSS-NNN: <one-line title>
- date: YYYY-MM-DD
- kind: deferral | exclusion | dependency-substitution | paradigm-degradation
- cpp-anchor: <file(:line) in decompiler/cpp/ that embodies the C++ behavior>
- rust-anchor: <file/crate in rust/ that embodies the departure, or 'none yet'>
- surface: <what user-visible or test-visible surface is affected>
- why: <why the departure was taken>
- restoration criteria: <what would have to be true/built to close the loss>
```

---

## LOSS-001: SLEIGH compiler stays C++
- date: 2026-06-10
- kind: deferral (permanent-by-design candidate)
- cpp-anchor: decompiler/cpp/slgh_compile.cc, slghparse.cc, slghscan.cc (upstream Makefile `SLACOMP` group; grammars slghparse.y/slghscan.l)
- rust-anchor: none yet
- surface: the `.slaspec -> .sla` build step (`make specs`); Rust binaries consume `.sla` only
- why: the compiler is build-time tooling, not part of any oracle binary (`decomp_dbg`/`decomp_test_dbg` link `CORE+DECCORE+SLEIGH+EXTRA`, never `SLACOMP`); porting ~11.5k LOC of bison/flex-generated compiler buys zero parity
- restoration criteria: a pure-Rust toolchain requirement materializes; port slgh_compile plus hand-written grammar replacements, gated on byte-identical `.sla` output for all vendored `.slaspec` modules

## LOSS-002: ghidra_* IPC client unported
- date: 2026-06-10
- kind: exclusion
- cpp-anchor: decompiler/cpp/ghidra_arch.cc, ghidra_context.cc, ghidra_process.cc, ghidra_translate.cc, inject_ghidra.cc, comment_ghidra.cc, cpool_ghidra.cc, database_ghidra.cc, loadimage_ghidra.cc, string_ghidra.cc, typegrp_ghidra.cc, signature_ghidra.cc (upstream Makefile `GHIDRA` group)
- rust-anchor: none yet
- surface: the Ghidra-plugin decompile server (`ghidra_dbg`/`ghidra_opt` binaries)
- why: not linked into any oracle binary kuna tests; kuna's harness drives the console binaries only
- restoration criteria: a Ghidra-plugin binding for the Rust engine is wanted; port the IPC marshalling against a live Ghidra and validate with Ghidra's Java-side protocol tests

## LOSS-003: Band-B coupling forces one kuna-decomp crate
- date: 2026-06-10
- kind: paradigm-degradation
- cpp-anchor: decompiler/cpp/ruleaction.cc / coreaction.cc cross-stage reads (S3 rules read/write S5 type facts etc.; see docs/stage-mapping.md straddler notes and docs/stage-model.md Band B)
- rust-anchor: rust/kuna-decomp (none yet)
- surface: crate graph: stages S2-S9 share one crate instead of one-crate-per-stage; stage modularity is expressed via the stage registry (`kuna_stages` port), not crate boundaries
- why: Band B is a genuine fixed-point coupling — slicing it into crates would force either circular dependencies or a trait soup that falsifies the oracle's structure
- restoration criteria: Band B interfaces narrowed to explicit traits with no loss of parity; then split kuna-decomp along the S3/S4/S5/S6 seams and have `cargo` enforce what the stage registry currently only documents

## LOSS-004: zlib -> flate2
- date: 2026-06-10
- kind: dependency-substitution
- cpp-anchor: decompiler/cpp/compression.cc:29-53 (zlib deflate/inflate streams)
- rust-anchor: none yet (rust/kuna-base, W1 item w1-base-util)
- surface: compressed `.sla` payload read (and write, for cache regeneration)
- why: flate2 is the standard Rust DEFLATE implementation; binding C zlib would add an FFI dependency for an identical wire format
- restoration criteria: not needed if round-trip stays byte-identical; gate: decompress-then-recompress every shipped `.sla` and compare payload bytes against the C++ path

## LOSS-005: libbfd -> object crate (W11)
- date: 2026-06-10
- kind: dependency-substitution
- cpp-anchor: decompiler/cpp/bfd_arch.cc, loadimage_bfd.cc (BfdArchitecture/LoadImageBfd)
- rust-anchor: none yet (rust/kuna-decomp, W11 item w11-loader-object)
- surface: real-ELF loading in the console (`load file`), i.e. `kuna.decompile` against actual binaries; datatests are unaffected (XML loadimage)
- why: libbfd is a GPL C dependency that already needs `tools/fetch_bfd.sh` gymnastics; the `object` crate is the Rust ecosystem standard and covers the formats kuna exercises
- restoration criteria: not needed if M4 holds: identical bytes loaded, identical symbol/entry resolution, identical decompilation on the real-ELF smoke corpus vs the libbfd-linked C++ oracle

## LOSS-006: bison/flex grammars -> hand-written parsers
- date: 2026-06-10
- kind: paradigm-degradation
- cpp-anchor: decompiler/cpp/xml.y -> xml.cc; grammar.y -> grammar.cc (`parse line` C declarations); pcodeparse.y -> pcodeparse.cc (pcode injection snippets). (slghparse.y/slghscan.l fall under LOSS-001; ruleparse.y is conditionally compiled, CPUI_RULECOMPILE.)
- rust-anchor: none yet (rust/kuna-base xml; rust/kuna-sleigh pcodeparse; rust/kuna-console grammar)
- surface: XML datatest/save-file parsing; console `parse line` declarations; `<pcode>` compilable injections in .cspec/.pspec
- why: no bison/flex in the Rust toolchain, and transliterating generated parser tables would be unmaintainable and unreviewable; hand-written recursive-descent parsers replace the generated automata
- restoration criteria: per parser, a grammar-conformance corpus extracted from the oracle (all vendored specs, all datatests, error cases) with identical accept/reject behavior and identical error text wherever the harness or datatests can observe it

## LOSS-007: std::regex -> regex crate in the datatest runner
- date: 2026-06-10
- kind: dependency-substitution
- cpp-anchor: decompiler/cpp/testfunction.hh:39 + testfunction.cc:31-75 (FunctionTestProperty, ECMAScript std::regex over output lines)
- rust-anchor: none yet (rust/kuna-harness, W9 item w9-harness-runner)
- surface: matching of the 675 datatest assertion patterns against decompiler output
- why: std::regex (ECMAScript dialect) has no Rust equivalent; the regex crate differs in dialect (no backreferences/lookaround)
- restoration criteria: corpus proof — every pattern in all 83 datatest files (and tests/stages) shown to produce the same match decision on the oracle's captured output under both engines; any pattern relying on unsupported syntax is flagged here and rewritten upstream-compatibly

## LOSS-008: goldens B3 boundary anchored at `break start paramdouble`, not heritage completion
- date: 2026-06-10
- kind: pinned-by-fiat
- cpp-anchor: decompiler/cpp/coreaction.cc (universalAction mainloop: ActionParamDouble immediately follows ActionHeritage); decompiler/cpp/action.cc (break_action only fires after an active transformation — ActionHeritage registers no change count, so `break action heritage` provably never triggers; verified empirically)
- rust-anchor: kuna/goldens.py (B3 capture)
- surface: differential-testing boundary only; no product behavior
- why: the intended anchor "first heritage completion" has no directly breakable event; `break start paramdouble` is the next action start in the same mainloop iteration, which is exactly first-heritage-completion (snapshots show fresh SSA def links)
- restoration criteria: none needed for parity; the Rust engine must replicate the paramdouble registration point so inv2 replay lines up. If the mainloop ordering ever changes, re-derive the anchor.

## LOSS-009: `print tree varnode` snapshots normalized (heap-address scrub + line-block sort)
- date: 2026-06-10
- kind: pinned-by-fiat
- cpp-anchor: decompiler/cpp printing of varnode trees: `(internal=0x...)` per varnode; AddrSpace*-valued constants (LOAD/STORE space pointers) print and SORT by raw heap pointer
- rust-anchor: kuna/goldens.py (_normalize for B3/B4)
- surface: differential harness only — B3/B4 varnode-tree comparison is multiset-style per line block (op/block ORDER stays exact via print raw / print tree block)
- why: process heap addresses are nondeterministic across runs/engines; scrubbing (regex `internal=`, 12-hex 0x[5-7]... constants -> `#(spaceptr)`) + per-block sorting makes the snapshot deterministic (proven: cpp-vs-cpp 131/131 identical). Slight diff-discrimination loss: a genuine 48-bit constant matching the scrub pattern would be masked.
- restoration criteria: a deterministic C++-side dump printing space NAMES instead of pointers (natural home: the `golden` console command in kuna_goldengen.cc); then drop the normalization.

## LOSS-010: miniz_oxide deflate output bytes differ from C zlib (refines LOSS-004)
- date: 2026-06-10
- kind: dependency-substitution
- cpp-anchor: decompiler/cpp/compression.cc:46-57 (Compress::deflate over C zlib)
- rust-anchor: rust/crates/kuna-base/src/compression.rs (Compress::deflate over flate2/miniz_oxide)
- surface: bytes WRITTEN by the Rust deflate path (.sla regeneration via FormatEncode/CompressBuffer, future W2+ callers). Demonstrated at verification (reviews/w1-base-util.md): level -1 (the only level used in-tree) on a 5000-byte test payload produces 3902 bytes vs zlib's 3916, diverging from byte 2 onward, and the per-call deflate avail_out return traces differ. Reads are unaffected: the chunked inflate return-value trace on zlib-produced bytes is call-for-call identical. Byte-identical cases pinned by test: empty stream, stored-block levels 1/9 on incompressible data; level 0 differs only in header (0x08 0x1D vs 0x78 0x01, both valid). Interop holds in both directions (zlib decodes miniz output and vice versa, verified with a C++ harness).
- why: miniz_oxide's match finder/lazy matching makes different (equally valid) encoder choices than zlib at compressed levels; the wire format is identical
- restoration criteria: LOSS-004's "decompress-then-recompress byte-identical" gate cannot hold against C++-written .sla bytes; replace it with content equality — decompress(Rust-compressed .sla) == decompress(C++-compressed .sla) for every shipped .sla (canary: verify_compression_default_level_stream_interop_loss010). Byte-identity would require binding C zlib via FFI, superseding LOSS-004.

## LOSS-011: filemanage POSIX emulation via std::fs (dot entries, UTF-8 names, getcwd cap)
- date: 2026-06-10
- kind: dependency-substitution
- cpp-anchor: decompiler/cpp/filemanage.cc:177-206 (matchListDir over readdir), 245-279 (directoryList), 92-102 (addCurrentDir getcwd[256])
- rust-anchor: rust/crates/kuna-base/src/filemanage.rs (std::fs::read_dir / std::env::current_dir)
- surface: (1) match_list_dir with allowdot=true can never return the "." / ".." entries that C++ readdir yields (std::fs::read_dir omits them) — observable only on the public API; every in-tree caller passes allowdot=false, where C++ filters them anyway. Pinned by verify_filemanage_fs_layouts_match_cpp_oracle (Rust result == C++ result minus exactly those two entries). (2) Directory entries with non-UTF-8 names are skipped (C++ strings carry raw bytes); vendored spec trees are ASCII. (3) add_current_dir works for cwd longer than 255 bytes where C++ getcwd(256) silently fails (and discoverGhidraRoot would then hit UB on pathlist[0]; the Rust port panics on a genuinely failed current_dir instead).
- why: std::fs is the idiomatic, safe directory API; reproducing readdir's dot entries or raw-byte names would require unix-only OsStr plumbing with no in-tree consumer
- restoration criteria: a caller materializes that needs allowdot=true dot entries or non-UTF-8 spec paths; then reimplement over std::os::unix::fs with OsString surfaces and re-pin against the C++ oracle

## LOSS-012: space/address Translate- and JoinRecord-dependent paths deferred
- date: 2026-06-11
- kind: deferral
- cpp-anchor: decompiler/cpp/space.cc:255-298 (AddrSpace::read register branch), space.cc:454-650 (JoinSpace overlapJoin/encodeAttributes/decodeAttributes/printRaw/read), address.cc:191-194 (renormalize join), address.cc:236-260 + 316-352 (register-name Range/decode branches), fspec.cc:2124-2169 (FspecSpace encodeAttributes/printRaw payloads), op.cc:41-59 (IopSpace::printRaw), translate.cc:139-233 + 659-937 (SpacebaseSpace, JoinRecord machinery: findAddJoin/findJoin/renormalizeJoinAddress/constructJoinAddress)
- rust-anchor: rust/crates/kuna-base/src/space.rs (AddrSpaceKind::Join/Fspec/Iop arms; module docs "Deferred"), rust/crates/kuna-base/src/address.rs (renormalize, decode_varnode_attributes ATTRIB_NAME, Range::from_properties is_register)
- surface: join-space addresses (split variables), register-name console/stream syntax ("eax", <register name="...">), FspecSpace/IopSpace debug printing and fspec address encoding, SpacebaseSpace (stack) semantics beyond what the base AddrSpace carries. Methods with real C++ behavior return explicit Err("kuna rust port: ... not yet ported") — except AddrSpace::read, where a register name silently parses through the C++ catch branch (identical to C++ behavior for an *unknown* register; known registers need Translate). The w1 golden gate substitutes a base AddrSpace (IPTR_SPACEBASE, "stack", index 8) for the C++ SpacebaseSpace; the Address comparators consult only identity/index, so the substitution is comparator-equivalent.
- why: JoinRecord/VarnodeData/Translate/FuncCallSpecs/PcodeOp are W2+ items (pcoderaw is kuna-num; Translate is the sleigh wave); porting stubs of those types now would freeze their design prematurely
- restoration criteria: the sleigh-wave Translate port lands JoinRecord + getRegister; replace every "not yet ported" arm with the transcribed C++ body and re-pin against C++ oracle vectors that exercise join encode/decode round-trips and register-name reads

## LOSS-013: marshal readOpcode/writeOpcode deferred to the kuna-num opcode wave
- date: 2026-06-11
- kind: deferral
- cpp-anchor: decompiler/cpp/marshal.cc:429-456 (XmlDecode::readOpcode x2, `get_opcode` + "Bad encoded OpCode"), 583-598 (XmlEncode::writeOpcode), 1046-1062 (PackedDecode::readOpcode x2, `val < 0 || val >= CPUI_MAX` range check), 1224-1229 (PackedEncode::writeOpcode); marshal.hh:273-287, 370-374 (virtual declarations)
- rust-anchor: rust/crates/kuna-base/src/marshal.rs:27-32 (module docs), 696-697 and 763 (trait-surface notes) — the six opcode methods are absent from the Rust Decoder/Encoder traits
- surface: opcode-valued attribute encode/decode (XML: opcode mnemonic strings; packed: positive signed-integer opcode values). No W1 caller exists; first callers arrive with pcode op decode (W2+)
- why: OpCode/get_opcode live in opcodes.cc/hh, owned by item w1-num-pcode-semantics in crate kuna-num, which depends on kuna-base — porting the methods into the kuna-base traits would invert the crate dependency (recorded by the w1-base-marshal verifier so the surface cannot be silently dropped; the porter's deferral was previously tracked only in module doc comments)
- restoration criteria: w1-num-pcode-semantics (or the first wave that decodes pcode ops) ports the six marshal.cc opcode methods as kuna-num extension traits or free functions over Decoder/Encoder, pinned against the C++ oracle (XML mnemonic round-trip including the "Bad encoded OpCode" error path; packed CPUI_MAX range check); its verifier checks this entry off

## LOSS-014: INT_SDIV/INT_SREM overflow cell returns EvaluationError where C++ SIGFPEs (UB-2 decision)
- date: 2026-06-11
- kind: paradigm-degradation
- cpp-anchor: decompiler/cpp/opbehavior.cc:508-518 (OpBehaviorIntSdiv::evaluateBinary), 530-540 (OpBehaviorIntSrem::evaluateBinary) — host 64-bit signed division; `0x8000000000000000 / 0xffffffffffffffff` at sizein 8 raises SIGFPE (#DE) and kills the process (docs/rust-port/upstream-bugs.md UB-2)
- rust-anchor: rust/crates/kuna-num/src/opbehavior.rs (OpBehaviorIntSdiv/OpBehaviorIntSrem: explicit `num == i64::MIN && denom == -1` guard returning `KunaError::Evaluation("Signed division overflow")`)
- surface: emulation of the single INT64_MIN / -1 (and % -1) cell per op: the C++ oracle process traps; the Rust engine returns an evaluation error like the divide-by-0 path. Pinned by the 4 TRAP rows of tests/golden/vectors/opbehavior.csv (TRAP == error asserted, no panic) and kuna-num tests `test_opbehavior_ub2_sdiv_srem_error_not_panic`. This entry records the W2-decision upstream-bugs.md UB-2 called for ("decide at W2 (opbehavior port) and record a DIV/LOSS entry").
- why: a process-killing hardware trap is not a behavior a memory-safe port can or should reproduce; the EvaluationError path is the same recovery route the adjacent divide-by-0 check already takes, so emulator callers see a uniform failure mode
- restoration criteria: none (the C++ behavior is a crash); if upstream later guards the cell, re-pin the TRAP rows to upstream's chosen result

## LOSS-015: spaceid constant varnodes carry the AddrSpace manager index, not a heap pointer
- date: 2026-06-11
- kind: paradigm-degradation
- cpp-anchor: decompiler/cpp/pcoderaw.cc:111-116 (PcodeOpRaw::decode `<spaceid>`: `offset = (uintb)(uintp)decoder.readSpace(ATTRIB_NAME)`), pcoderaw.hh:100-103 (getSpaceFromConst: `(AddrSpace *)(uintp)offset`)
- rust-anchor: rust/crates/kuna-num/src/pcoderaw.rs (PcodeOpRaw::decode stores `spc.get_index() as u64`; `VarnodeData::get_space_from_const(&self, manage: &AddrSpaceManager)` resolves it back — signature gains the manager the C++ pointer implied)
- surface: the numeric `offset` of constant-space varnodes that encode an address space (LOAD/STORE spaceid inputs): nondeterministic heap address in C++, deterministic small space index in Rust. Visible wherever such offsets print or sort (the B3/B4 differential dumps LOSS-009 already scrubs+sorts on the C++ side; the harness normalization must map BOTH representations to the same token, e.g. the space name, before W2+ differential runs compare engines). Round-trip pinned by kuna-num test `test_pcoderaw_decode_xml` (recovered space identity).
- why: a Rust u64 cannot round-trip an `Rc<AddrSpace>` identity; the manager index is the stable, deterministic equivalent (and is what LOSS-009's restoration criteria already proposes the C++ side print)
- restoration criteria: none for value equality (representations intentionally differ); closed for comparison purposes when the differential harness normalizes spaceid constants by space NAME on both engines (LOSS-009 restoration)

## LOSS-016: XML element nesting depth uncapped (bison YYMAXDEPTH not emulated)
- date: 2026-06-10
- kind: paradigm-degradation
- cpp-anchor: decompiler/cpp/xml.cc:971 (YYMAXDEPTH 10000), 2042-2043 (yyexhaustedlab: yyerror("memory exhausted"), yyparse returns 2)
- rust-anchor: rust/crates/kuna-base/src/xml.rs (hand-written parser per LOSS-006: iterative `parse_element`, no depth cap; non-recursive worklist `Drop for Element`)
- surface: accept/reject behavior for element nesting deeper than the bison parse-stack cap: the C++ oracle accepts the plain nesting probe to depth 4997 and rejects depth >= 4998 with "memory exhausted" (`xml_parse` returns 2; the exact boundary is a function of the LALR stack's high-water mark, so it can shift by a few levels with surrounding symbols); the Rust port accepts any depth (`xml_parse` returns only 0/1). No in-tree XML (datatests, save files, specs) nests deeper than ~10.
- why: the cap is an artifact of the bison automaton's heap-allocated, doubling parse stack that LOSS-006 deliberately replaced; emulating the boundary exactly would mean simulating the generated automaton's per-shift stack depth (including transient symbol counts), re-importing precisely the generated-table complexity the hand parser exists to avoid. The hazard that made the divergence dangerous — the recursive DOM `Drop` aborting the process on input the oracle rejects cleanly (review w1-base-xml F1) — is repaired by the worklist `Drop`, so arbitrarily deep documents now parse AND tear down safely.
- restoration criteria: a depth guard in `Parser::parse_element` that reproduces the bison boundary (reject the deepest open element the oracle rejects with error text "memory exhausted" and `xml_parse` return code 2), validated by boundary probes (depths 4996-4999, with/without prolog and attributes) against the C++ oracle harness described in `docs/rust-port/reviews/w1-base-xml.md`.

## LOSS-017: context.{hh,cc} parser-state machinery deferred to the sleigh decode-engine item
- date: 2026-06-11
- kind: deferral
- cpp-anchor: decompiler/cpp/context.hh:61-292 (ConstructState, ContextSet, ParserContext, ParserWalker, ParserWalkerChange, inline deallocateState/allocateOperand/pushOperand), context.cc:22-323 (everything except the Token/FixedHandle declarations: ConstructState ctors, ParserContext ctor/initialize/getN2addr/getInstructionBytes/getInstructionBits/getContextBytes/getContextBits/addCommit/applyCommits/expandState, ParserWalker::setOutOfBandState, ParserWalkerChange::calcCurrentLength)
- rust-anchor: rust/crates/kuna-sleigh/src/context.rs (module docs; only `Token` and `FixedHandle` are ported)
- surface: instruction-parse state for SLEIGH decode — none of it is callable yet (no Rust `Sleigh` engine exists); the gap becomes user-visible the moment item w2-sleigh-core ports sleigh.{hh,cc}, whose `Sleigh::resolve`/`PcodeCacher` are built directly on ParserContext/ParserWalkerChange
- why: the deferred types are built around `Constructor`/`TripleSymbol`/`OperandSymbol` (slghsymbol, item w2-sleigh-symbol) and `Translate` (item w2-sleigh-translate), none ported when this item landed; porting them now would force placeholder types into modules owned by later items. Accountability gap this entry exists to close: checklist item w2-sleigh-core's `modules` list does NOT include context.{hh,cc}, so without this entry the deferred ~70% of the item's C++ scope would escape every verifier's in-scope diff
- restoration criteria: w2-sleigh-core (or a dedicated follow-up) ports the deferred context.{hh,cc} machinery with context.cc/context.hh added to that item's verified scope (hunt-list review of getInstructionBits/getContextBytes shift arithmetic, MAX_DEPTH/breadcrumb bounds, applyCommits commit ordering); its verifier checks this entry off

## LOSS-018: ContextCache re-fetches on hit instead of caching the C++ raw blob pointer (and drops getDatabase)
- date: 2026-06-11
- kind: paradigm-degradation
- cpp-anchor: decompiler/cpp/globalcontext.hh:317-331 (`ContextDatabase *database` member, `mutable const uintm *context` cached pointer, `getDatabase()`), globalcontext.cc:568-577 (getContext serves the cached pointer while the address stays inside [first,last])
- rust-anchor: rust/crates/kuna-sleigh/src/globalcontext.rs (`ContextCache`: no owned database — every method takes `&dyn ContextDatabase`/`&mut dyn ContextDatabase`; cache hits re-fetch the blob via the single-lookup `ContextDatabase::get_context`)
- surface: context blobs served by `ContextCache::get_context` after the database is mutated WITHOUT going through the cache's set_context methods while a cached range is live: if such a mutation inserts a new split point inside the cached [first,last], C++ keeps serving the stale pre-split blob until the next miss; the Rust port serves the fresh post-split blob (pinned by canary test `verify_w2ctx_cache_direct_mutation_divergence_canary`). All flows that mutate through the cache API invalidate identically in both engines (transcribed three-test invalidation), and in-place paints reaching into the cached range from below are seen identically. `getDatabase()` has no Rust equivalent — future callers (e.g. C++ ParserContext's ctor `ccache->getDatabase()->getContextSize()`) must receive the database explicitly
- why: safe Rust cannot store a `*const u32` into a `BTreeMap`-owned blob alongside a `&mut` database handle; caching the bounds and re-fetching by single lookup preserves the cache's purpose (skipping the bounds query) without unsafe. The porter documented the divergence in module docs and claimed it "reported as a loss", but never appended the entry — added here by the w2-sleigh-context verifier
- restoration criteria: when the W2 sleigh engine + console goldens land, demonstrate either (a) no in-tree flow mutates the ContextDatabase directly while a Sleigh ContextCache holds a live range (divergence unreachable -> close), or (b) the differential harness shows a golden diff, in which case replicate the C++ staleness by snapshotting the blob words into the cache at miss time

## LOSS-019: join-record table lives on the JoinSpace (shared via copy_spaces), not per-manager
- date: 2026-06-11
- kind: paradigm-degradation
- cpp-anchor: decompiler/cpp/translate.hh:233-235 (per-AddrSpaceManager members `joinallocate`/`splitset`/`splitlist`), translate.cc:659-703 (findAddJoin allocates from the manager's own counter)
- rust-anchor: rust/crates/kuna-base/src/space.rs (`JoinState` inside `AddrSpaceKind::Join`; `AddrSpaceManager::find_add_join` reaches it through the registered join space)
- surface: join-record identity and allocation offsets when two managers share one join space: after `copy_spaces` both Rust managers would see ONE allocation counter and record set, where each C++ manager starts its own empty table. Unobservable in-tree: the only copySpaces caller (architecture.cc:637, restoreFromSpec) runs BEFORE the JoinSpace is inserted, and the join space is created only on the Architecture manager — so exactly one manager ever owns a join table (verified by grep of the vendored tree). The porter documented the equivalence argument in module docs; this entry makes it ledger-visible (recorded by the w2-sleigh-translate verifier).
- why: the JoinSpace virtuals (overlapJoin/encodeAttributes/decodeAttributes/printRaw/read) need the join table without the C++ `manage` back-pointer; storing it in the join space's kind behind a RefCell avoids a manager<->space reference cycle without unsafe
- restoration criteria: if a future flow gives two managers join spaces with distinct record sets (or the architecture wave ports a second record-creating manager), move `JoinState` back onto `AddrSpaceManager` behind a RefCell and pass the manager into the JoinSpace virtuals explicitly (the decode paths already carry it); re-pin with the w2 verify tests

## LOSS-020: AddrSpaceManager::decode_spaces un-callable with a decoder over the same manager
- date: 2026-06-11
- kind: paradigm-degradation
- cpp-anchor: decompiler/cpp/translate.cc:285-302 (decodeSpaces inserts spaces while the same manager serves the Decoder's readSpace resolution, e.g. `<space_base contain="...">`)
- rust-anchor: rust/crates/kuna-base/src/space.rs (`AddrSpaceManager::decode_spaces`; aliasing caveat in the method docs)
- surface: the `<spaces>` spec-parse entry point: Rust callers cannot pass a decoder borrowing `self` (the method needs `&mut self` while the W1 `Decoder` design holds `&AddrSpaceManager`), so the architecture bootstrap must drive the identical loop body stepwise — a fresh decoder per child element around each `decode_space`/`insert_space` pair, as the w2-sleigh-translate tests demonstrate. Calling the method with a decoder over a DIFFERENT manager compiles but resolves space references against the wrong table.
- why: Rust aliasing rules; reproducing the C++ call shape would need interior mutability across the whole manager or a redesigned Decoder manager-access seam, both bigger than this item
- restoration criteria: the architecture wave (first real caller) revisits Decoder's manager access (per-call manager parameter or RefCell'd space table) so the C++ call shape works; gate on parity of `<spaces>` parsing for all vendored spec inputs, then drop the stepwise pattern

## LOSS-021: slghpattern/slghpatexpress compiler-side classes unported (TokenPattern, PatternEquation, Token plumbing)
- date: 2026-06-11
- kind: deferral (permanent-by-design candidate, extends LOSS-001)
- cpp-anchor: decompiler/cpp/slghpatexpress.hh:23-55 (TokenPattern), :339-487 (OperandResolve + PatternEquation hierarchy: OperandEquation, UnconstrainedEquation, ValExpressEquation and its six comparison subclasses, EquationAnd/Or/Cat, ellipsis equations); slghpatexpress.cc:21-455 (resolveTokens, buildSingle/buildBigBlock/buildLittleBlock, TokenPattern bodies), :1162-1680 (advance_combo/buildPattern statics, all PatternEquation bodies); the genPattern/genMinPattern virtuals throughout; OperandValue::getSubValue (slghpatexpress.cc:793-798) and isConstructorRelative/getName (slghpatexpress.cc:800-812)
- rust-anchor: rust/crates/kuna-sleigh/src/slghpatexpress.rs (module docs "What is NOT ported"; `OperandValue::get_sub_value` returns a Sleigh error; index/table_id/ct_id accessors exposed for the deferred isConstructorRelative/getName)
- surface: none at runtime. All consumers of the unported classes sit under the .slaspec compiler path (SleighCompile::buildPatterns -> Constructor::buildPattern -> genPattern/genMinPattern/ContextOp::validate; slgh_compile.cc:2122-2130, slghsymbol.cc:1832-1849,1887,2452), which LOSS-001 already keeps in C++ (`sleigh_opt`). OperandValue::getSubValue is reachable only from the unported equation genPattern bodies; isConstructorRelative/getName only from ContextOp::validate (compile-time)
- why: same rationale as LOSS-001 — the compiler is build-time tooling outside every oracle binary; porting pattern-construction machinery buys zero parity. Accountability gap this entry exists to close: these classes live in modules that ARE in w2-sleigh-pattern's verified scope (unlike slgh_compile.cc/slghparse.cc, which no port item claims), so without this entry the cut would be invisible to the checklist. Recorded by the w2-sleigh-pattern verifier (review F1)
- restoration criteria: only if LOSS-001's restoration ever happens (a pure-Rust .slaspec compiler); port these classes as part of that effort, gated on byte-identical .sla output. If instead a later wave needs isConstructorRelative/getName at runtime (it should not — verify via grep at w2-sleigh-symbol review), implement them on the symbol-table side through the exposed accessors and check this entry off for that sliver

## LOSS-022: slghsymbol compiler-side classes and table maintenance unported (Macro/Label/Section/Bitrange, buildPattern, DecisionNode::split, purge/renumber/replaceSymbol, getVarnode templates)
- date: 2026-06-11
- kind: deferral (extends LOSS-001/LOSS-021; the Label and getVarnode slivers are wave-deferrals, not permanent)
- cpp-anchor: decompiler/cpp/slghsymbol.hh:120-132 (SectionSymbol), :266-278 (BitrangeSymbol), :607-635 (MacroSymbol, LabelSymbol); slghsymbol.cc:123-143 (replaceSymbol), :281-369 (purge/renumber), :1730-1891 (orderOperands/buildPattern), :1974-2026 (buildDecisionTree/SubtableSymbol::buildPattern), :2028-2342 (DecisionProperties + DecisionNode split/orderPatterns/chooseOptimalField/getScore/getNumFixed/getMaximumLength/consistentValues/addConstructorPair); the getVarnode() virtuals returning VarnodeTpl (slghsymbol.cc:460-467,736-740,1012-1029,1152-1159,1217-1224,1282-1289,1338-1345,1370-1377); OperandSymbol compile-side flag setters
- rust-anchor: rust/crates/kuna-sleigh/src/slghsymbol.rs (module docs "What is NOT ported"; `SymbolType` keeps the Macro/Section/Bitrange/Label/Dummy discriminants so getType comparisons transcribe)
- surface: none in the W2 gate. Consumers split three ways: (a) slgh_compile.cc/slghscan.cc (Section/Bitrange/Macro creation, buildPattern/split/purge/renumber/replaceSymbol) — the LOSS-001 compiler cut; (b) pcodecompile.cc getVarnode() template builders — in w2-sleigh-semantics' claimed scope; (c) pcodeparse.cc:230,3249 LabelSymbol (+ the SymbolTree scope machinery it reuses) — pcodeparse IS linked into the oracle runtime (p-code snippet parsing for injections) and is w2-sleigh-pcodeparse's claimed scope
- why: same accountability rationale as LOSS-021 — these classes live in modules inside w2-sleigh-symbol's verified scope, so the cut must be ledgered even though every consumer is either compiler-side (LOSS-001) or a later checklist item. Recorded by the w2-sleigh-symbol verifier (review F1). Note: LOSS-021's isConstructorRelative/getName sliver is CLOSED by this item (SymbolTable::operand_value_is_constructor_relative / operand_value_name, slghsymbol.rs)
- restoration criteria: (a) LabelSymbol + scope reuse must be added when w2-sleigh-pcodeparse is ported (extend the SymbolKind enum; gate on pcodeparse parity); (b) the getVarnode() virtuals must be added with w2-sleigh-semantics once VarnodeTpl exists; (c) Macro/Section/Bitrange and the pattern-build/table-maintenance machinery only if LOSS-001's pure-Rust compiler restoration ever happens

## LOSS-023: EmulatePcodeOp (emulateutil) deferred to the kuna-decomp IR wave
- date: 2026-06-11
- kind: deferral
- cpp-anchor: decompiler/cpp/emulateutil.hh:43-99 (class EmulatePcodeOp) / emulateutil.cc:22-148 (its method bodies: getLoadImageValue, executeUnary/Binary/Load/Store, executeCbranch with the booleanflip bit, executeMultiequal's in-edge scan over FlowBlock, executeIndirect-as-copy, executeSegmentOp via glb->userops, the CpoolRef/New no-ops)
- rust-anchor: rust/crates/kuna-sleigh/src/emulateutil.rs (module docs "Scope note"; no Rust counterpart yet)
- surface: none at runtime today. EmulatePcodeOp emulates over syntax-tree PcodeOp/Varnode/FlowBlock objects plus Architecture::userops, none of which exist before the kuna-decomp IR wave; its only in-tree subclass is EmulateFunction (jumptable.hh:110, jump-table recovery), itself in the unported decomp wave. EmulateSnippet — the other class in the same file, consumed by ExecutablePcode (pcodeinject.hh:163) — IS ported, including its Architecture* substitution (loader + space manager slices); the Architecture-bound getArch() accessor is dropped with the class
- why: porting it now would invent the kuna-decomp IR API ahead of ADR 0001's arena decisions (every method takes Funcdata-resident objects). Accountability: the class sits inside w2-sleigh-emulate's verified module scope, so the cut must be ledgered (the LOSS-021/022 precedent). Recorded by the w2-sleigh-emulate verifier (review F1)
- restoration criteria: when kuna-decomp lands op.rs/varnode.rs/block.rs and the userop table, port EmulatePcodeOp against them and re-verify with EmulateFunction (jump-table recovery) as the consumer; check this entry off in that item's review

## LOSS-024: breakpoint setEmulate back-pointer replaced by an invocation-time emulator parameter (break-table re-entry panics)
- date: 2026-06-11
- kind: paradigm-degradation
- cpp-anchor: decompiler/cpp/emulate.hh:48 (BreakTable::setEmulate), :87/:125-129 (BreakCallBack::setEmulate + the stored `Emulate *emulate`), emulate.cc:25-50 (register* wiring), :54-68 (BreakTableCallBack::setEmulate map walks), :337 (constructor `breaktable->setEmulate(this)`), :428/:454 (callback invocation while the emulator is live on the stack)
- rust-anchor: rust/crates/kuna-sleigh/src/emulate.rs (BreakCallBack/BreakTable trait methods take `&mut dyn EmulateMemory`; EmulatePcodeCache::execute_callother and execute_instruction hold the `Rc<RefCell<dyn BreakTable>>` borrow across the callback)
- surface: none in the oracle binaries — no in-tree callback re-enters its break table, and the upstream callback pattern (emulate.hh's PutsCallBack example: memory-state access + setExecuteAddress) behaves identically. Divergences for out-of-tree client code: (a) a callback that drives the passed-in emulator far enough to consult the break table again (executeInstruction, or a CALLOTHER) is defined behavior in C++ but panics on the RefCell double-borrow in Rust; (b) doPcodeOpBreak/doAddressBreak can no longer be invoked without an emulator in hand; (c) a BreakCallBack can no longer stash the emulator pointer for use outside an invocation
- why: Rust cannot store the C++ mutable back-pointer while the emulator itself is `&mut` on the call stack; threading the emulator into the callback at invocation time preserves every observable in-tree behavior without unsafe. Recorded by the w2-sleigh-emulate verifier (review F2)
- restoration criteria: if a future consumer legitimately needs re-entrant break tables, split BreakTable into lookup (returning the callback Rc under a short borrow) + invocation (table borrow released), and pin the recursion with a test; until then the panic is the guard

## LOSS-025: print_decimal readback overflows to inf instead of C++11 num_get ERANGE-stores-MAX (testfloatemu double_decimal_precision DBL_MAX row ignored)
- date: 2026-06-11
- kind: divergence (library bug in already-verified w1-num-float-multiprec scope, surfaced by the w2-harness-floatemu port)
- cpp-anchor: decompiler/cpp/float.cc:465-473 (printDecimal's `istringstream >> float/double` roundtrip readback; C++11 [facet.num.get.virtuals] stores +-numeric_limits::max() and sets failbit on ERANGE overflow — probe-verified on this host's libstdc++: "1.79769313486232e+308" reads back == DBL_MAX), decompiler/unittests/testfloatemu.cc:186-187 (the failing oracle assertion: printDecimal(0x7fefffffffffffff,false) == "1.79769313486232e+308")
- rust-anchor: rust/crates/kuna-num/src/float.rs (print_decimal: `s.parse::<f32/f64>().unwrap_or(0.0)` — Rust parse overflows to +-inf, so the precision-15 roundtrip never equals DBL_MAX and the ladder falls through to decimal_max_precision, returning "1.7976931348623157e+308"); rust/crates/kuna-num/tests/testfloatemu.rs (double_decimal_precision is #[ignore]d citing this entry); divergence pinned by rust/crates/kuna-num/tests/verify_w2_harness_floatemu.rs verify_print_decimal_dbl_max_loss_trace
- surface: print_decimal output differs from the C++ oracle only for values whose minimal-digit rendering at some ladder precision parses out of the host-float range — i.e. the immediate neighborhood of +-DBL_MAX (and +-FLT_MAX on the size<=4 path, e.g. "3.402824e+38".parse::<f32>() == inf where num_get stores FLT_MAX). One oracle assertion (testfloatemu double_decimal_precision f1 row) is not reproduced; the other five rows of that TEST pass (pinned by verify_double_decimal_precision_non_overflow_rows). The in-code comment at the float.rs readback ("strtof/strtod semantics ... same as str::parse") is wrong at exactly this boundary. Note the w1-num-float-multiprec review's printDecimal sweeps (its F6) did not include a MAX-boundary cell, which is how this escaped
- why: not a deliberate cut — a faithful-port gap in the w1 library item discovered by this harness item; the harness port itself is assertion-for-assertion faithful and may not silently re-pin the oracle string, so the TEST is #[ignore]d against this ledger entry instead
- restoration criteria: in float.rs print_decimal, emulate the C++11 num_get overflow semantics in the readback (on overflow of the parsed magnitude, substitute +-f64::MAX / +-f32::MAX before the roundtrip comparison), then un-ignore double_decimal_precision in testfloatemu.rs, re-run the w2-harness-floatemu gate (31/31), and drop/retighten verify_print_decimal_dbl_max_loss_trace's either-or assertion to the oracle string

## LOSS-026: getVarnode() template virtuals stay unported at w2-sleigh-semantics; createBitRange takes a caller-resolved varnode (refines LOSS-022(b))
- date: 2026-06-11
- kind: deferral (bookkeeping refinement of LOSS-022 restoration criteria (b))
- cpp-anchor: decompiler/cpp/slghsymbol.hh getVarnode() virtuals (VarnodeSymbol/OperandSymbol/Start/End/Next2/FlowDest/FlowRef/SpecificSymbol); decompiler/cpp/pcodecompile.cc:676-755 (createBitRange's only uses of `sym` are getVarnode()/getName()/getLocation(sym)); decompiler/cpp/pcodeparse.cc:2365,2369-2373,2397,2421,2453 (the runtime getVarnode() call sites)
- rust-anchor: rust/crates/kuna-sleigh/src/pcodecompile.rs (`PcodeCompile::create_bit_range(vn: VarnodeTpl, sym_name: &[u8], ...)` — the SpecificSymbol parameter is replaced by its resolved varnode + name; module docs record the reshaping); the getVarnode() builders have no Rust anchor yet
- surface: none at this item's gate (createBitRange has no in-tree caller until pcodeparse). LOSS-022's restoration criteria (b) said the getVarnode() virtuals "must be added with w2-sleigh-semantics once VarnodeTpl exists" — this entry records that they were NOT: the w2-sleigh-semantics port instead moved varnode resolution to the caller, so the whole getVarnode obligation (both createBitRange's argument and pcodeparse's four direct grammar uses) lands on w2-sleigh-pcodeparse. Counterweight: LOSS-022(a)'s LabelSymbol sliver is partially closed EARLY by this item (LabelSymbol now lives in rust/crates/kuna-sleigh/src/pcodecompile.rs with Cell-based isplaced/refcount; the SymbolTree scope-reuse half remains with pcodeparse)
- why: the symbol wave deferred getVarnode (LOSS-022); implementing it at this item would have meant designing the VarnodeTpl-construction surface of seven symbol kinds with no consumer to validate against. Taking the resolved varnode keeps pcodecompile.cc's behavior byte-equivalent while leaving the symbol-side design to the item that exercises it. Recorded by the w2-sleigh-semantics verifier (review F1)
- restoration criteria: w2-sleigh-pcodeparse implements the getVarnode() equivalents (on the symbol table or as a resolver seam) for VarnodeSymbol/OperandSymbol/Start/End/Next2/FlowDest/FlowRef, wires them into its grammar actions and into create_bit_range's call site, and pins them against the C++ template shapes (space/offset/size ConstTpl triples per symbol kind); then LOSS-022(b) and this entry close together

## LOSS-027: SleighBuilder::appendCrossBuild cannot lazily disassemble an uncached CROSSBUILD target (Hexagon CROSSBUILD parse fails)
- date: 2026-06-11
- kind: paradigm-degradation
- cpp-anchor: decompiler/cpp/sleigh.cc:405-439 (appendCrossBuild), :423 `discache->getParserContext(newaddr)` (DisassemblyCache::getParserContext lazily disassembles ANY address on a cache miss, sleigh.cc:443-503); the cross context is fetched on demand, not pre-resolved
- rust-anchor: rust/crates/kuna-sleigh/src/sleigh.rs:1381-1421 (append_cross_build), :1400-1404 `find_context(&newaddr).ok_or_else(|| ..."Could not obtain cached crossbuild instruction")` over the pre-resolved `contexts: &[ResolvedCtx]` slice; one_instruction (sleigh.rs:1953-1992) pre-resolves only the MAIN instruction (contexts[0]) and its DELAY-SLOT addresses — never CROSSBUILD targets, since their addresses are only known mid-`build` from `bld->getIn(0)` after the walker is positioned
- surface: a `crossbuild <addr>,<section>` directive whose target address is not already in the pre-resolved set (i.e. not the main instruction or a delay-slot byte) returns `LowlevelError("Could not obtain cached crossbuild instruction")` instead of disassembling the target on demand and weaving in its named p-code section. The only in-tree spec that emits the `crossbuild` directive is Hexagon (specs/Ghidra/Processors/Hexagon/data/languages/hexagon.{sinc,slaspec}); none of the 16 golden_lift languages use it, so the w2-sleigh-core gate is green. The failure mode is a loud error, not silent wrong p-code. (Verified: `crossbuild` directive count is 0 across MIPS/x86/6502/V850/RISCV/Atmel/Toy/SuperH4 and nonzero only for Hexagon.)
- why: the Rust engine resolves a fixed set of ParserContexts up front (an arena, no lazy heap-allocated DisassemblyCache with a hashed re-disassembly path); pre-resolving crossbuild targets is impossible because their addresses are computed during the build walk that consumes them. A faithful lazy-disassembly path needs the engine to call back into obtain_context from inside append_cross_build, which the current borrow structure (`contexts: &[..]` held immutably across the build) does not permit.
- restoration criteria: give append_cross_build an obtain_context-on-miss path (DisassemblyCache::getParserContext equivalent) — e.g. an owned, growable context arena the builder can extend mid-walk, or an interior-mutable cache keyed by Address — then validate with a Hexagon CROSSBUILD fixture (build the Hexagon .sla, lift a packet that crosses sub-instruction sections) against the C++ oracle; the w2-sleigh-core verifier (or a Hexagon-language follow-up) checks this entry off

## LOSS-028: PcodeCacher::resolveRelatives label id kept at full u64 instead of the C++ uint4 truncation
- date: 2026-06-11
- kind: divergence
- cpp-anchor: decompiler/cpp/sleigh.cc:126 `uint4 id = ptr->offset;` — the 64-bit relative-branch varnode offset is truncated to 32 bits BEFORE the `(id >= labels.size())||(labels[id] == 0xbadbeef)` bounds check and the `labels[id]` index (sleigh.cc:127,130)
- rust-anchor: rust/crates/kuna-sleigh/src/sleigh.rs:962 `let id = self.pool[ptr].offset;` (full u64), used directly at :964 `id >= self.labels.len() as u64 || self.labels[id as usize] == 0x0badbeef` and :968 `self.labels[id as usize]`
- surface: none from any real spec. The label id is a `set_label`-assigned sequential index (sleigh.rs:1373 set_label / add_label grows `labels` to id+1), bounded by the small per-instruction label count, so `pool[ptr].offset` never exceeds u32 range in practice and the two computations coincide. They diverge only for a crafted/corrupt `.sla` whose relative-branch template offset exceeds 2^32: C++ would mask to the low 32 bits (which could spuriously pass the bounds check and index a wrong label), while the Rust keeps the full value (always failing the `id >= labels.len()` check -> "Reference to non-existant sleigh label"). The Rust behavior is the safer one; it is recorded only because it is a confirmed departure from the literal C++ truncation.
- why: the port read the offset as its natural u64 rather than transcribing the C++ `uint4` narrowing; left unfixed at the round-2 repair because it is unreachable from any valid spec and the Rust outcome is strictly safer (error vs. potential mis-index).
- restoration criteria: to match the C++ bit-for-bit, narrow with `let id = self.pool[ptr].offset as u32;` (cast: C++ `uint4 id = ptr->offset`) and index with `id as usize`, then add a unit test feeding a label-ref varnode whose offset has nonzero high 32 bits and asserting the C++ truncate-then-check outcome; the verifier checks this entry off

## LOSS-029: five pure VarnodeBank tree-query methods unported at w3-ir-varnode (beginLoc(AddrSpace*)/beginLoc(Address)/endLoc(AddrSpace*)/endLoc(Address) + overlapLoc), no SEAM marker
- date: 2026-06-11
- kind: deferral (scope incompleteness; pure container queries droppable now)
- cpp-anchor: decompiler/cpp/varnode.cc:1579 beginLoc(AddrSpace*), :1590 endLoc(AddrSpace*), :1601 beginLoc(const Address&), :1612 endLoc(const Address&), :1810 overlapLoc(iter, vector<...>&bounds); declared decompiler/cpp/varnode.hh:398-401,408. All five read ONLY loc_tree (manager getNextSpaceInOrder / searchvn template) — no PcodeOp, Datatype, HighVariable, or Cover dependency, so none is blocked by a W3/W6/W7 seam.
- rust-anchor: rust/crates/kuna-decomp/src/varnode.rs — NO corresponding `iter_loc_space`/`iter_loc_addr`/`overlap_loc` exist; the diff ports beginLoc/endLoc only for the (void), (s,addr), (s,addr,fl), (s,addr,pc,uniq) overloads and the def-tree family. The four space/address overloads and overlapLoc are silently absent (no SEAM(...) comment, unlike the genuinely-deferred op/type/symbol/cover surfaces which are all annotated).
- surface: none at this item's gate. The sibling W3 files op.rs/block.rs/funcdata.rs are still 1-line pending stubs, so there is no in-tree caller of these queries yet; `cargo test -p kuna-decomp` is green. The gap becomes load-bearing the moment heritage/funcdata iterates varnodes by address space (funcdata's beginLoc(spaceid) walks, e.g. spacebase/alias analysis) or calls overlapLoc (used by Heritage::guard* / overlap grouping). Until then it is invisible.
- why: the port scoped itself to the (size, addr, flag, pc/uniq) probe family and the def-tree family and did not transcribe the AddrSpace-span / single-Address / overlapLoc probes. These need only the already-ported loc_tree plus `AddrSpaceManager::getNextSpaceInOrder` (beginLoc(AddrSpace*) advances searchvn.loc to the next space in order); endLoc(Address) also handles the `addr.getOffset()==space->getHighest()` wrap by bumping to the next space. All buildable on the existing LocProbe machinery.
- restoration criteria: a later W3 item (the first consumer — funcdata_varnode or heritage) adds `iter_loc_space`/`iter_loc_addr` as LocProbe pairs mirroring varnode.cc:1579-1622 (incl. the getNextSpaceInOrder span bound and the getHighest()-wrap in endLoc(Address)) and `overlap_loc` mirroring varnode.cc:1810-1839 (the `endLoc(size,addr,written)`-stepped scan that returns the unioned flag word + the bounds vector), pins them against the C++ via the existing build_manager fixture, and then this entry closes; OR the w3-ir-varnode item is re-opened to add them with a SEAM marker if a consumer needs them sooner.

## LOSS-030: op.cc/op.hh methods needing the not-yet-ported block-graph (W3-block) or type/emulation (W6) subsystems are seam-deferred at w3-ir-op
- date: 2026-06-11
- kind: deferral (cross-wave seam; methods are annotated SEAM(...) in op.rs, not silently dropped)
- cpp-anchor: decompiler/cpp/op.cc — PcodeOp::{collapse :473, executeSimple :502, collapseConstantSymbol :527, encode :412, printDebug :399, nextOp :346, previousOp :367, target :383, compareOrder :808, isMoveable :201, setCopyImmed :128, hasCopyImmed :139}; PcodeOpBank::fallthru :1140; PieceNode::{isLeaf :831, findRoot :854, gatherPieces :895}. (IopSpace::printRaw :41 is NOT part of this entry — it is restored under LOSS-012 and lives at op.rs:1318+.)
- rust-anchor: rust/crates/kuna-decomp/src/op.rs — each deferred method is either absent or returns a seam-`Err`/seam-stub carrying a `// SEAM(W3-block)` or `// SEAM(W6)` comment (e.g. the block-graph note at op.rs:52, the TypeOp/OpBehavior note at op.rs:40-43, parent/basic-iter seams at op.rs:211,571,866-900). No `todo!`/`unimplemented!`; the seam markers grep-locate them.
- surface: none at the w3-ir-op gate (`cd rust && cargo test -p kuna-decomp` green). Each deferred method genuinely requires a subsystem this wave does not yet have: the block-graph family (nextOp/previousOp/target/compareOrder/isMoveable/setCopyImmed/hasCopyImmed/fallthru, and PieceNode's loneDescend/compareOrder walks) needs BlockBasic/FlowBlock (W3-block); the value family (collapse/executeSimple/collapseConstantSymbol/encode/printDebug) needs the W6 OpBehavior/TypeFactory/Encoder seam. The cut becomes load-bearing only when those later items wire in their first consumer.
- why: porting these now would require inventing the W3-block FlowBlock graph API and the W6 OpBehavior/TypeFactory/Encoder surfaces ahead of their own checklist items, with no consumer to validate against — exactly the LOSS-023/LOSS-029 situation (an in-scope method whose only consumers are later waves must still be ledgered). This entry exists to close the accountability gap the round-1 w3-ir-op review (finding F3) flagged: the methods sit inside op.cc/op.hh, which IS w3-ir-op's verified module scope, so the deferral must be visible in the ledger even though every consumer is a later item. Recorded by the w3-ir-op verifier (round 2).
- restoration criteria: each method is ported as its subsystem lands — the block-graph family with w3-block (FlowBlock::findCommonBlock, BlockBasic op iterators, basiciter/insertiter walks), the value family with W6 (OpBehavior::evaluate{Unary,Binary,Ternary}, TypeFactory symbol propagation, Encoder); each is pinned against its C++ body and the relevant sliver of this entry checked off. PieceNode (isLeaf/findRoot/gatherPieces) restores with the CONCAT-tree consumer (w6/typeprop) that needs it.

## LOSS-031: userop/pcodeinject/inject_sleigh dataflow/display/emit/debug methods seam-deferred at w3-ir-userop-inject
- date: 2026-06-11
- kind: deferral (cross-wave seam; covered by umbrella SEAM module-doc comments + the InjectEngine trait, not silently dropped — but no per-method markers, hence this ledger entry per the LOSS-029/030 precedent)
- cpp-anchor: decompiler/cpp/userop.cc — the data-type/display/dataflow virtuals: UserPcodeOp::extractAnnotationSize :38, VolatileReadOp::{getOperatorName :122, getOutputLocal :129, extractAnnotationSize :144}, VolatileWriteOp::{getOperatorName :153, getInputLocal :160, extractAnnotationSize :175}, DatatypeUserOp::{getOutputLocal :71, getInputLocal :77}, InternalStringOp::getOutputLocal :362, SegmentOp::{unify :190, execute :219}. decompiler/cpp/pcodeinject.cc — ExecutablePcode::{build :146, evaluate :192} (emulator wiring). decompiler/cpp/inject_sleigh.cc — InjectPayloadSleigh::{inject :48, checkParameterRestrictions :109, setupParameters :137}, ExecutablePcodeSleigh::inject :234, the entire InjectPayloadDynamic class (:282-341 + decodeEntry/inject), PcodeInjectLibrarySleigh::{forceDebugDynamic :357, parseInject wiring of inject() :373 (the compile half parse_inject IS ported), decodeDebug :465, getBehaviors :485}. (parseInject's compile path, allocateInject, registerInject, and ALL decode/registration/lookup are PORTED — this entry is only the emit/emulate/dataflow/debug remainder.)
- rust-anchor: rust/crates/kuna-decomp/src/userop.rs — the dataflow/display virtuals are entirely ABSENT (grep `fn get_operator_name|get_output_local|get_input_local|extract_annotation|fn unify|fn execute` = 0 hits); the module-doc umbrella SEAM at userop.rs:17-23 and the SEAM(W6) notes at :201,670,687,715 cover them as a class. rust/crates/kuna-decomp/src/pcodeinject.rs — `InjectEngine` trait (:464) names the emit/build/evaluate seam, no implementor exists. rust/crates/kuna-decomp/src/inject_sleigh.rs — no `InjectPayloadDynamic`/`inject`/`build`/`evaluate`/`decodeDebug`/`forceDebugDynamic`; the dynamic/debug path is noted at :434,599 and `decode_inject_debug_entry` (:600) ports only the inner name/type read.
- surface: none at this item's gate (`cd rust && cargo test -p kuna-decomp` green; 71 in-module + 6 verifier tests). The display/dataflow virtuals need the W6 TypeFactory + SymbolTable + a live PcodeOp/Varnode (the W3-op IR exists but these read `op->getOut()->getSize()` / `glb->symboltab` / `op->doesSpecialPropagation()`); SegmentOp::unify needs Funcdata.newConstant and SegmentOp::execute needs the inject library's evaluate() (the emulate seam); the emit family needs kuna-sleigh's private SleighBuilder/PcodeCacher/ParserWalkerChange/ParserContext mutators (also LOSS-027/028's neighborhood); InjectPayloadDynamic/decodeDebug are a debug-only path (`<injectdebug>`) absent from the spec/oracle corpus. Each becomes load-bearing only when its consumer wave (W6 typeprop / heritage / the action that fires CALLOTHER injection) wires in.
- why: porting any of these now would mean inventing the W6 type/symbol surface, the Funcdata constant-creation API, or making kuna-sleigh's emit internals public — all ahead of their own checklist items, with no consumer to validate against (the LOSS-023/029/030 situation: an in-scope method whose only consumers are later waves must still be ledgered). The decode/registration/lookup half — the genuine new behavior of this wave — IS fully ported and adversarially tested. Recorded by the w3-ir-userop-inject verifier (round 1) to close the accountability gap that the umbrella SEAM comments (no per-method markers) leave in the ledger.
- restoration criteria: the dataflow/display virtuals port with W6 (TypeFactory/SymbolTable + the typeprop/printlanguage consumer that calls getOutputLocal/getOperatorName); SegmentOp::unify/execute with the segment-recovery action (Funcdata.newConstant + the ExecutablePcode emulate seam, which also closes LOSS-023's EmulatePcodeOp dependency); the InjectPayload::inject emit family + ExecutablePcode::build/evaluate when kuna-sleigh exposes its SleighBuilder/PcodeCacher/ParserWalkerChange/ParserContext emit API (the InjectEngine trait gets a real implementor); InjectPayloadDynamic/decodeDebug/forceDebugDynamic with the debug-replay item (or droppable permanently if the `<injectdebug>` path is declared out of scope). Each is pinned against its C++ body and the relevant sliver checked off.

## LOSS-032: SegmentOp/JumpAssistOp decode child-loop adds an `else { break }` the C++ for(;;) lacks (hardens an upstream infinite-loop on a malformed third child)
- date: 2026-06-11
- kind: divergence (defensive hardening; safer than C++, unreachable from any valid spec)
- cpp-anchor: decompiler/cpp/userop.cc:256-275 (SegmentOp::decode child loop) and :312-339 (JumpAssistOp::decode child loop) — both are `for(;;){ subId=peekElement(); if(subId==0)break; if(==A){..} else if(==B){..} }` with NO final `else`. If a child element appears whose id matches none of the handled arms, neither `peekElement` advances nor any arm consumes it, so the loop re-peeks the same nonzero id forever — an infinite loop on malformed input.
- rust-anchor: rust/crates/kuna-decomp/src/userop.rs:531-535 (decode_segment) and :614-616 (decode_jump_assist) — the ported loops add a final `else { break; }`, so an unhandled child terminates the loop and is left for `close_element(elem_id)` to consume (or to surface as a decoder error there).
- surface: none from any real spec. Every in-tree `<segmentop>` child is `<constresolve>` or `<pcode>`, and every `<jumpassist>` child is `<case_pcode>/<addr_pcode>/<default_pcode>/<size_pcode>`; no vendored cspec/pspec emits a third child kind, so the divergent arm is never taken. The behaviors differ only on a crafted/corrupt element with an unexpected child: C++ hangs, Rust breaks cleanly. The Rust outcome is strictly safer (termination vs. hang), so it is recorded only as a confirmed literal departure (mirrors LOSS-028's "safer divergence" classification).
- why: the port transcribed the if/else-if chain with a terminating else rather than reproducing the C++ unbounded for(;;); leaving it as-is would import an upstream hang. Unfixed because the divergence is unreachable from valid input and the Rust behavior is the safe one.
- restoration criteria: to match the C++ literally (hang on malformed third child) is undesirable; instead the entry closes when either (a) a test feeds a `<segmentop>`/`<jumpassist>` with an unexpected child and asserts the Rust breaks-and-errors-on-close path (documenting the intended hardened behavior), or (b) the upstream loops are confirmed structurally incapable of seeing a third child kind (the marshaler only surfaces declared children), making the divergence vacuous. Either way the literal-fidelity gap is acknowledged, not re-introduced.
