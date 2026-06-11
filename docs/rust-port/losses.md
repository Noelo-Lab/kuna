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
