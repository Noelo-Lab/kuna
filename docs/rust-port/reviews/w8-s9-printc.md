# w8-s9-printc
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8[1m] (independent verifier)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-decomp` -> PASS. lib 1424/1424 (incl. the 21 printc unit tests) + all integration test binaries green once the gitignored `.sla` artifacts are present (I copied the 148 built `.sla` from the main tree into the worktree; the one initial failure `corpus_bootstrap::ldefs_pspec_cspec_parse_matrix` was env-only — that test file is untouched by this branch and needs a built x86 `.sla`, identical to the env failure already noted in the w8-s9-printlanguage-cast and w8-s9-prettyprint verdicts). Plus 5 new verifier tests (`verify_w8_s9_printc.rs`) PASS. `cargo clippy -p kuna-decomp -- -D warnings` clean for the crate and for my new test file (two pre-existing clippy errors live in unrelated, untouched test files `verify_w3_ir_funcdata_varnode.rs` / `verify_w4_fw_architecture_r2.rs`).

## Scope / provenance
- cpp blob shas match the checklist `cpp_blob_sha` EXACTLY: `printc.cc` = c776ae0…, `printc.hh` = a1c716d… (verified against `git rev-parse HEAD:…`).
- This item's commit is `173f841` and touches ONLY `rust/crates/kuna-decomp/src/printc.rs` (+914) and `printc/tests.rs` (+475). The `cast.rs`/`printlanguage.rs`/`printjava.rs` deltas in the `rust-port...branch` diff belong to the already-ACCEPTed dependency `w8-s9-printlanguage-cast` (commit `2fb5840`, review on file) and are OUT OF SCOPE here.

## What is ported, and what is deferred
This is a **partial port by design**. Of printc.cc's 3,560 LOC, the porter transcribed only the self-contained DATA + constant/char/float formatters; the entire RPN/`Emit`-driven body (~printc.cc:144–3514: every `op*`/`emit*`/`push*`/`doc*`/`emitBlock*` method) is **seam-noted `// SEAM(W9-emit)` and deferred**. The deferral is structurally justified and consistent with the established precedent:
- the `Emit` low-level driver is `prettyprint.rs` — a **1-line stub** (separate item `w8-s9-prettyprint`, ACCEPTed-as-deferral, NOT a dependency of printc);
- the `PrintLanguage` RPN driver (`pushOp`/`pushAtom`/`recurse`/`emitOp`/`opBinary`/`opUnary`) was **explicitly deferred to W9** by the already-ACCEPTed `printlanguage.rs` (grep confirms no `fn push_op`/`fn op_binary`/`fn recurse` in printlanguage.rs).
Porting the bodies now would only produce `todo!()` against missing infrastructure. Ledgered as **LOSS-117**.

The ported half (re-derived against the C++ and independently re-verified):
- **Operator-token table** (`tokens::*`, printc.cc:24–78): all 51 `OpToken` statics compared field-by-field (print1/print2/stage/precedence/associative/type/spacing/bump) — EXACT. `TokenType` enum order matches C++ `OpToken::tokentype` (binary…hiddenfunction). Statics so `parentheses`'s `ptr::eq` identity holds.
- **Negate cross-links** (`token_negate`, printc.cc:130–135): the six comparison flip pairs (less_than↔greater_equal, less_equal↔greater_than, equal↔not_equal) — EXACT; all other tokens return `None` (C++ `negate` stays null). Realized as a ptr-identity lookup to sidestep self-referential statics.
- **Keyword/punctuation constants** (`keywords::*`, printc.cc:80–104) — EXACT, incl. `typePointerRelToken = "ADJ"`.
- **`PrintCOptions`** (printc.cc:1649–1664 `resetDefaultsPrintC`): every default matches incl. the kuna DIV-2 default-on `array_notation = true`; `brace_*` map onto `BraceStyle` whose discriminants match `Emit::brace_style` (same_line=0/next_line=1/skip_line=2). Setters mirror printc.hh:242–255. (Note: `setCStyleComments()`'s comment-delimiter side effect is part of the deferred body — acceptable.)
- **`print_char_hex_escape`** (printc.cc:1580–1591): width 2/4/8 by magnitude. Critically, Rust `{:0Wx}` on a SIGNED `i32` emits the two's-complement bit pattern (`ffffffff`), matching C++ `<< hex` — NOT a `-` sign. Verified by adversarial test against an oracle.
- **`print_unicode`** (printc.cc:1494–1538): the special-escape switch is case-for-case identical; the raw branch (`StringManager::writeUtf8`) is reproduced by `char::from_u32(onechar as u32)` + `push`. Confirmed reachability-safe: `unicode_needs_escape` (verified) pre-escapes negatives (`<0x20`), surrogates (`>=0xd7fc`), and `>=0x2fa20`, so the raw branch only ever sees valid scalar values — `char::from_u32` never returns `None` there, and the UTF-8 bytes match `writeUtf8` exactly (exhaustively checked 0..0x30000).
- **`format_integer_token`** + **`resolve_integer_format`** (printc.cc:1359–1434): sign-strip (`flip = val^mask; val = flip+1` with `wrapping_add`), hex/dec/oct/bin/char decision, FORCE_CHAR high-byte split (`sz==1 && val>=0x80` → hex escape), wide-char `L` prefix gate (`sz>1`), `U`/sizeSuffix suffixes — all match. octal/hex-of-zero edges (`"00"`/`"0x0"`) match the C++ stream behavior.
- **`format_float_token`** (printc.cc:1449–1492): INFINITY/NAN/FLOAT_UNKNOWN + sign, the `.0` fix-up keyed on bytes `.`/`e`, `force_scinote` bypass. The shortest-round-trip `printDecimal` is correctly LEFT to kuna-num `FloatFormat` (W6) — not reinvented.
- **`generic_function_name`/`generic_type_name`** (printc.cc:3516–3558): `func_<addr>` (kuna angr branch correctly factored to the caller) and the `unk*`/`BADSPACEBASE`/`BADTYPE` switch with early-returns — EXACT.
- **`op_emit_kind`** (printc.hh:289–351): all 50+ inline `op*` delegations cross-checked against the header — every Binary/Unary/Func/TypeCast token mapping matches; signed+unsigned variants (div/sdiv, less/sless) collapse to the same token as in C++; hand-written/no-op overrides → `Custom`.

## Mandatory hunt list
- Signedness: CLEAN. The one signed/`<<hex` hazard (`print_char_hex_escape` of a negative codepoint) is correct (Rust `LowerHex` == C++ two's-complement) — verified adversarially. `resolve_integer_format` FORCE_CHAR guard (`displayFormat != force_char`) suppresses sign handling exactly as C++.
- Integer widths: CLEAN. `int4`/`uintb`/`u32` mappings faithful; `val as int4` casts mirror C++ `(int4)val`; widths in `calc_mask(sz)` consumed correctly.
- Wrapping: CLEAN. `flip.wrapping_add(1)` matches C++ unsigned `flip+1`; verified at INT_MIN and all-ones boundaries for sz 1/2/4/8.
- Comparator totality: N/A — no comparators ported in this item (the only "order" is `token_negate`'s ptr-identity lookup, total by construction).
- Iteration-order provenance: N/A — `op_emit_kind`/`token_negate` are pure dispatch; no container iteration in the ported code. No HashMap/HashSet (grep clean).
- Off-by-one / do-while / reverse iterators: CLEAN. `format_float_token`'s looks-like-float scan is a faithful `.bytes().any(.)` of the C++ for-loop; no do-while idioms.
- Erase-while-iterating: N/A (no mutation-during-traversal in the ported code).
- Exception→Result partial-state: CLEAN. Only `generic_function_name` returns `KunaResult` (from `Address::print_raw`); no mid-mutation throw sites in the ported surface.

## Mechanical pass
- grep printc.rs: no `todo!`/`unimplemented!`/`panic!` (the lone hit is a doc comment), no `HashMap`/`HashSet`, no `sort_unstable`, no float `partial_cmp`/`unwrap`. Three bare `as` casts (`onechar as u32`, two `val as int4`) — each is a faithful mirror of a C++ `(int4)val`/bit-reinterpret and is now backed by an adversarial test; recommend (non-blocking) adding `// cast:` notes.
- `cargo clippy -p kuna-decomp -- -D warnings`: clean for the crate; my new test clippy-clean.
- `port-audit` tool not present in this tree (same as prior waves).

## Adversarial tests added (rust/crates/kuna-decomp/tests/verify_w8_s9_printc.rs, all PASS)
- `printc_w8_char_hex_escape_negative_matches_cpp_bit_pattern` — negative-codepoint `<<hex` bit pattern vs oracle; asserts no `-` sign, 8-digit width.
- `printc_w8_resolve_integer_twos_complement_wrap_boundaries` — sign/magnitude at signed minima & all-ones for sz 1/2/4/8 + FORCE_CHAR sign suppression, vs oracle.
- `printc_w8_print_unicode_raw_branch_matches_writeutf8_for_all_reachable` — EXHAUSTIVE 0..0x30000: every non-escaped codepoint's bytes equal a hand-coded `writeUtf8` oracle; surrogates proven escaped; `char::from_u32` never drops a reachable codepoint.
- `printc_w8_format_float_dot_zero_and_scinote_boundary` — `.0` fix-up keyed on `.`/`e`, `force_scinote` bypass, special classes.
- `printc_w8_format_integer_char_high_byte_and_wide_split` — `sz==1 && val>=0x80` hex-escape vs `sz>1` unicode path; wide `L` gate.

## Findings
- F1 (loss, not a blocker): the RPN/`Emit`-driven body of `PrintC` (~printc.cc:144–3514 — every `op*`/`emit*`/`push*`/`doc*`/`emitBlock*` method, the statement/declaration/block driver, `docFunction`, `printCharacterConstant`, `pushConstant`/`pushSymbol`/`pushType`/`pushPartialSymbol` dispatch) is deferred and seam-noted, not transcribed. Justified: the `Emit` driver (`prettyprint.rs` stub, separate item) and the `PrintLanguage` RPN driver (deferred to W9 by the accepted `printlanguage.rs`) do not exist in the merged tree; transcription would be `todo!()` against missing infra. Matches the precedent set by `w8-s9-printlanguage-cast` (RPN deferral) and LOSS-115 (PrintJava). Recorded as **LOSS-117**.
       cpp: decompiler/cpp/printc.cc:144-3514
       rust: rust/crates/kuna-decomp/src/printc.rs:884-910 (SEAM(W9-emit) block)
- F2 (nit, non-blocking): the three bare `as` casts in printc.rs (lines 564, 612, 614) lack the `// cast:` justification comment the protocol's mechanical pass requests. All three are correct (mirror C++ `(int4)val` / a bit-reinterpret for UTF-8) and verified; cosmetic only.

losses: LOSS-117 (PrintC RPN/Emit-driven method bodies deferred to W9).
