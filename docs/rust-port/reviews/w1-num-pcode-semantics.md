# w1-num-pcode-semantics

verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier agent (round 1)
date: 2026-06-11
branch: worktree-wf_cea42066-041-19 (item commit b24212f; verifier tests d31c640)
gate: `cd rust && cargo test -p kuna-num --test golden_opbehavior` -> ok (1 passed:
      all 20227 lines of tests/golden/vectors/opbehavior.csv regenerated
      byte-for-byte, TRAP/SPULL cells asserted to error without panicking).
      Also run: `cargo test -p kuna-num` -> 52 passed, 0 failed (incl. the 9
      verifier tests below); `cargo clippy -p kuna-num --all-targets -- -D
      warnings` -> clean.

## Scope and method

C++ in scope re-derived line-by-line: `decompiler/cpp/opcodes.cc/.hh` (75-entry
enum, name table, sorted-search table, get_opcode/get_opname/get_booleanflip),
`pcoderaw.cc/.hh` (VarnodeData comparators/decode/contains/isContiguous,
PcodeOpRaw incl. static decode), `opbehavior.cc/.hh` (base class + all
subclasses + registerInstructions), plus the marshal.cc readOpcode/writeOpcode
sextet this item was assigned by LOSS-013. All six `cpp_blob_sha` values in
checklist.json verified against HEAD (git hash-object): match.

Oracle work performed by the verifier (independent of the porter):

- Regenerated tests/golden/vectors/opbehavior.csv from the live C++ binary
  (`tools/rust-port/gen_vectors.py` driving decomp_dbg at C++ rev 87016be):
  byte-identical to the pinned file the gate replays. The gate is anchored to
  the real oracle, and the branch did not touch the vectors.
- Built a fresh C++ harness (committed at
  rust/crates/kuna-num/tests/data/verify_w1_pcode_oracle_harness.cc, output
  verify_w1_pcode_oracle.out) linked against the decomp_dbg objects, covering
  what the golden vectors do NOT: 5187 recoverInputUnary/recoverInputBinary
  cells (full size matrix, shift counts 0..64 and u64::MAX), 16 get_opcode
  lookups, get_booleanflip over the whole enum. The Rust port replays every
  row exactly (test `verify_recover_paths_match_cpp_oracle` et al.).
- Empirical UB-1 confirmation: the C++ oracle **segfaults inside
  `get_opcode("ZPULL")` / `get_opcode("SPULL")`** (the binary search probes
  the out-of-bounds sorted slot 74). upstream-bugs.md UB-1 only documents the
  get_opname OOB read; the lookup-side crash is additional evidence the
  regenerated Rust tables are the only viable transcription.

## hunt list

- signedness: clean. Every mixed `uintb` vs `int4` comparison
  (`in2 >= sizeout*8` family) is transcribed as `wmul(8) as i64 as u64` with a
  justifying comment (opbehavior.rs:696,715,722,743,760,767,788,814 region),
  reproducing C++'s int->u64 conversion; oracle-replayed including u64::MAX
  shift operands.
- integer widths: clean. uint4 truncations in SCARRY/SBORROW carry 0/1 values;
  packed readOpcode `(int4)` truncation reproduced and tested
  (`verify_packed_opcode_truncation_and_range`: 0x1_0000_0001 -> COPY,
  0x1_0000_004B -> rejected).
- wrapping: clean. All uintb-lineage arithmetic uses the `Wrap` helpers
  (ADR 0003); the full golden matrix + 5187-row oracle replay run under the
  dev profile (overflow checks on) without panicking — the ADR's wrap
  detector. Shift-count UB cells (PIECE sizeout<sizein -> count -8&63=56,
  recover paths hitting count 64) resolve by wrapping_shl/shr count masking,
  which matches the x86 oracle and is pinned by the golden rows/oracle rows.
- comparator totality: finding F3 (note). VarnodeData Ord is total; the
  distinct-space-same-index fall-through diverges from C++'s op< (which would
  declare such pairs equivalent) only in a state unreachable within one
  manager, and is documented at the site (pcoderaw.rs:120-123).
- iteration order: clean. No HashMap/HashSet/sort_unstable anywhere in the
  item (mechanical grep). Loops are over fixed tables, Vec insertion order
  (PcodeOpRaw::in_ mirrors the C++ vector), or the behavior table in index
  order; generator-loop order is pinned by the byte-identical golden output.
- off-by-one / do-while / reverse iterators: clean. get_opcode binary-search
  bounds (min=1, max=CPUI_MAX-1, inclusive) match; with the 75-slot table the
  Rust search's slot 74 is in bounds where C++'s is the UB-1 OOB probe.
  IntSright recover loop `while i <= sa` preserves `for(i=0;i<=sa;++i)`.
  No do-while or reverse iterators in scope.
- erase-while-iterating: n/a (none in scope).
- exception -> Result partial-state parity: finding F2 (minor) on
  VarnodeData::decode_from_attributes error-path field order; PcodeOpRaw::
  decode verified to leave outvar/invar untouched when readOpcode fails,
  matching the C++ throw point (test `verify_decode_bad_opcode_preserves_outvar`).

## mechanical

- grep todo!/unimplemented!/HashMap/HashSet/sort_unstable over the four item
  files: no hits. Bare `as` casts all carry `// cast:` justifications or are
  enum-discriminant array indexes.
- `cargo clippy -p kuna-num --all-targets -- -D warnings`: clean.
- `cargo run -p port-audit -- --item w1-num-pcode-semantics`: **not runnable —
  no port-audit crate exists in the workspace** (finding F6, process note).
  Substituted: manual cpp_blob_sha verification (all six match).

## findings

- F1 (note, UB-1 surface): the port's regenerated name/sort tables
  deliberately diverge from C++ exactly where C++ is broken: C++
  get_opname(ZPULL)=="EXTRACT", get_opname(SPULL) reads OOB (segfaults), and
  get_opcode("ZPULL"/"SPULL") segfaults (verifier-reproduced);
  get_opcode("EXTRACT")==71 and get_opcode("UNUSED1")==(OpCode)45 in C++
  resolve to None in Rust (OpCode cannot hold 45; the XML decode path then
  errors where C++ would propagate a nameless value into a null behavior slot
  — C++ UB shortly after). Sanctioned by docs/rust-port/upstream-bugs.md UB-1
  ("the Rust port does NOT replicate"), compile-time table-length assertion
  present as UB-1 requires, golden vectors pin the canonical names.
     cpp: decompiler/cpp/opcodes.cc:29-56,60-86
     rust: rust/crates/kuna-num/src/opcodes.rs:318-399
- F2 (minor, error-path partial state): C++ decodeFromAttributes assigns
  `space` BEFORE `space->decodeAttributes(decoder,size)`; the Rust assigns
  `self.space` after `decode_attributes` succeeds (and writes offset first).
  On a mid-decode error C++ leaves the new space set; Rust leaves space=None.
  Error-path only; no in-scope caller observes the partially decoded struct
  (both engines propagate the error). Not blocking; should be aligned if a
  later wave's recovery path starts reading the partial struct.
     cpp: decompiler/cpp/pcoderaw.cc:42-47
     rust: rust/crates/kuna-num/src/pcoderaw.rs:186-191
- F3 (note, comparator): VarnodeData Ord falls through to offset/size when two
  DISTINCT space objects share an index, where C++ op< returns false both ways
  (equivalent-but-unequal, itself a strict-weak-order/== incoherence). State
  unreachable within one AddrSpaceManager; documented at the site; totality
  and Ord/Eq consistency swept over reachable states by
  `verify_varnodedata_ord_total_and_consistent_with_eq`.
     cpp: decompiler/cpp/pcoderaw.hh:67-71
     rust: rust/crates/kuna-num/src/pcoderaw.rs:103-128
- F4 (note, LOSS-014): INT_SDIV/INT_SREM INT64_MIN / -1 returns
  KunaError::Evaluation("Signed division overflow") where the C++ process
  SIGFPEs (UB-2). The W2 decision upstream-bugs.md UB-2 called for is hereby
  recorded as LOSS-014 (verifier-appended). TRAP rows pinned as
  error-without-panic by the gate.
     cpp: decompiler/cpp/opbehavior.cc:508-518,530-540
     rust: rust/crates/kuna-num/src/opbehavior.rs:886-951
- F5 (note, LOSS-015): spaceid constant varnodes store the AddrSpace manager
  index in `offset` instead of the C++ heap pointer;
  `get_space_from_const` gains the manager parameter. Deterministic
  replacement for a value LOSS-009 already has to scrub; recorded as LOSS-015
  (verifier-appended) so the differential-harness wave normalizes both
  representations to the same token.
     cpp: decompiler/cpp/pcoderaw.cc:111-116, pcoderaw.hh:100-103
     rust: rust/crates/kuna-num/src/pcoderaw.rs:151-159,432-444
- F6 (note, process): verification.md's `port-audit` mechanical step is not
  executable — no such crate exists in rust/. Blob shas were verified
  manually. Separately, the branch's docs/rust-port/losses.md carries its own
  "LOSS-010: XML element nesting depth uncapped" while main's ledger already
  assigns LOSS-010 to the miniz/deflate entry (and continues through
  LOSS-013, now -015): the branch entry must be renumbered at merge or two
  different LOSS-010s will exist.
     cpp: n/a
     rust: docs/rust-port/losses.md (branch vs main divergence)
- F7 (note, deferral): the `name=` (register) arm of
  VarnodeData::decodeFromAttributes returns an explicit "requires Translate
  (sleigh wave)" error instead of `trans->getRegister(...)`. Same deferral
  class LOSS-012 records for address.cc's register-name branches (its why
  explicitly covers VarnodeData/pcoderaw in kuna-num); restoration lands with
  the sleigh-wave Translate port.
     cpp: decompiler/cpp/pcoderaw.cc:48-53
     rust: rust/crates/kuna-num/src/pcoderaw.rs:192-199
- F8 (note, LOSS-013 closure): the six marshal.cc readOpcode/writeOpcode
  methods deferred by LOSS-013 are now ported as the OpcodeDecoder/
  OpcodeEncoder extension traits, exactly as that entry's restoration criteria
  prescribed (XML mnemonic round-trip incl. "Bad encoded OpCode" both paths,
  packed CPUI_MAX range check + (int4) truncation) and verifier-pinned by
  `verify_xml_opcode_roundtrip_all_ops` / `verify_packed_opcode_truncation_and_range`.
  LOSS-013 can be treated as restored from this item forward.
     cpp: decompiler/cpp/marshal.cc:429-456,583-598,1046-1062,1224-1229
     rust: rust/crates/kuna-num/src/opcodes.rs:459-553

No blocker or major findings. registerInstructions transcribed exactly,
including the upstream quirk that PTRADD/PTRSUB register the plain base
behavior (not OpBehaviorPtradd/Ptrsub) — preserved and tested; slots 0/45
remain empty as in C++.

adversarial tests: rust/crates/kuna-num/tests/verify_w1_num_pcode_semantics.rs
(committed d31c640 with the C++ oracle harness + captured output under
tests/data/):
  - verify_recover_paths_match_cpp_oracle (5187-row C++ oracle replay)
  - verify_get_opcode_matches_cpp_oracle
  - verify_ub1_lookups_resolve_instead_of_crashing
  - verify_booleanflip_matches_cpp_oracle (full-enum sweep vs C++)
  - verify_varnodedata_ord_total_and_consistent_with_eq
  - verify_varnodedata_contains_wrap_semantics (size-0 u32 wrap, u64 end wrap)
  - verify_packed_opcode_truncation_and_range
  - verify_xml_opcode_roundtrip_all_ops
  - verify_decode_bad_opcode_preserves_outvar
All pass.

losses: LOSS-012 (register-name decode deferral, F7), LOSS-014
(verifier-appended: UB-2 SIGFPE -> EvaluationError, F4), LOSS-015
(verifier-appended: spaceid manager-index representation, F5). LOSS-013's
restoration criteria are satisfied by this item (F8).
