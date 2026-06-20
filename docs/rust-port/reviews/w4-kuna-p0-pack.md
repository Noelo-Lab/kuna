# w4-kuna-p0-pack
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier subagent)
date: 2026-06-11
gate: `cd rust && cargo test -p kuna-decomp --test catalog_bytecompat && cargo test -p kuna-decomp` -> PASS (catalog_bytecompat 5/5; full kuna-decomp suite 350 lib + all integration tests pass, exit 0). NOTE: `cargo clippy -p kuna-decomp --all-targets -- -D warnings` FAILS, but exclusively on OTHER items' verifier test files (verify_w4_fw_options.rs, verify_w4_fw_architecture_r2.rs, verify_w3_*.rs) with `doc_lazy_continuation` doc-lint nits; `cargo clippy -p kuna-decomp --lib` (which compiles every in-scope source file + the build.rs-generated code) is CLEAN.

## Scope
C++ in scope: kuna_stages.{cc,hh}, kuna_assert.{cc,hh}, kuna_restartlog.{cc,hh}.
Rust diff (in-scope subset): src/kuna_stages.rs, src/kuna_assert.rs, src/kuna_restartlog.rs, build.rs, stages.toml, tests/catalog_bytecompat.rs (+ fixtures/stage_catalog.json), and the three `*/tests.rs` modules. (The diff also contains sibling W3/W4 items — action, architecture, options, database, capability — which are out of scope here.)

## Oracle strength
The catalog gate is a STRONG oracle: I regenerated the fixture from the MAIN tree's live `decomp_dbg` binary (`stage catalog`, no program loaded) and `diff`'d it against `tests/fixtures/stage_catalog.json` — IDENTICAL (24 lines). So the settable table (22 rows) and the entire JSON emitter (key order, framing, `, `/`: ` separators, pipe-split, escaping) are pinned to real C++ output, not a hand-tuned match.

For the three tables NOT covered by the catalog JSON, I parsed the C++ `groupTable`/`surfaceTable`/`substageTable` directly and compared every field against `stages.toml`: **0 mismatches** across group (39), surface (90), substage (40) — name/stage/substage/note for group+surface; name/stage/decision/assertion/strength/rewind/latent/exposure for substage. I also parsed `kunaLiveValue`'s 19 ternaries and compared field/true/false against the toml `live_*` keys: **0 mismatches**, with exactly the 3 reader-less options (loweredswitch/stackguard/namestyle) omitting live keys. The inverted-polarity ternaries (compareform = present_lessequal?original:canonical; returnpair = return_single?single:pair) are transcribed correctly.

## hunt list
- Signedness: clean. The only signed/unsigned interplay is `KunaStage::index() -> i32` (mirrors C++ `int4` enum), `RestartLog::seq: i32` (= C++ `int4 restartSeq`), and `KunaFuncKey { space_index: i32, offset: u64 }` (= C++ `int4 spaceindex` / `uintb offset`); signedness matches the C++ field types. Cap check `events.len() >= 32` (usize) == C++ `(int4)events.size() >= MAX_EVENTS_PER_FUNC`.
- Integer widths: clean. No arithmetic that could promote/overflow; the only width concern is `seq += 1` (i32, matches C++ `restartSeq++`, both UB/panic only at i32::MAX, practically unreachable).
- Wrapping: clean. No `uintb`-lineage arithmetic in scope (pure data tables + string building). `pos + bar + 1` in the values split is byte-offset arithmetic bounded by string length.
- Comparator totality: clean. `KunaFuncKey` derives `Ord` over (i32, u64) in declaration order = total, and matches C++ `operator<` (glb drops out — the log is owned per-Architecture). No float comparators.
- Iteration-order provenance: clean. GROUP/SUBSTAGE/SURFACE/SETTABLE iterate `[T; N]` static slices in file order; build.rs preserves stages.toml row order (BTreeMap keys the table NAME, but rows are a `Vec` pushed in order). `emit_catalog_json` walks `0..n` by index = C++ `for(i=0;i<kunaNumSettables();++i)`. `RestartLog::render` reads one bucket's `Vec` in insertion order = C++ vector order. No HashMap/HashSet in scope.
- Off-by-one / do-while / reverse iterators: clean, and adversarially tested. The highest-risk spot is `emit_settable_json`'s values split, a port of C++ `while(pos <= vals.size())` (INCLUSIVE) — a trailing/lone/empty `|` emits an extra empty `""` token. Re-derived by hand for `"a|"`, `""`, `"|"` and confirmed the Rust `loop{...match next{None=>break}}` produces the identical token sequence; pinned by a new test.
- Erase-while-iterating: n/a (no container mutated during traversal in scope).
- Exception -> Result partial-state parity: clean for the ported half; the C++ throw sites inside `IfcKunaAssert::execute` (the per-arm `IfaceParseError`/`IfaceExecutionError` + store mutations) are the W5-deferred half (see LOSS-049). `validate_assertion` reproduces the 4 pre-dispatch throws in C++ order (bad stage -> missing sub -> unknown sub -> wrong stage) as `KunaError::Parse`.
- Aliasing: clean. The C++ global `static std::map restartTable` + `static int4 restartSeq` become a `RestartLog` owned per-Architecture — a behavior-preserving Rust-aliasing accommodation (the arch-pointer dimension of the C++ key becomes the owner), documented in the module and standard for this codebase (cf. LOSS-019/024).

## findings
  - F1 (minor): bare `as usize` cast without a `// cast:` justification comment.
       cpp: decompiler/cpp/kuna_stages.cc:48 `STAGE_CODES[(int4)stage]`
       rust: rust/crates/kuna-decomp/src/kuna_stages.rs:103 `other.index() as usize`
       Provably safe: the `Infra` arm is matched separately (returns 10); the `other` arm is P0..S9 whose `index()` is 0..=9, always non-negative, so `as usize` cannot wrap. Protocol wants the comment; behavior is correct.
  - F2 (minor): bare `as u32` cast without a `// cast:` justification comment.
       cpp: decompiler/cpp/kuna_console.cc:184 `(unsigned char)c < 0x20`
       rust: rust/crates/kuna-decomp/src/kuna_stages.rs:362 `(ch as u32) < 0x20`
       `char as u32` is the standard Unicode-scalar idiom and is the correct port of the C++ byte test for the catalog's ASCII content (verified: multibyte chars >= 0x80 pass through both sides identically). Comment missing; behavior is correct (adversarially tested).

Neither finding is a blocker or major: both casts are correct and the byte-compat gate plus my multibyte test confirm no observable divergence.

## adversarial tests
Added rust/crates/kuna-decomp/tests/verify_w4_kuna_p0_pack.rs (6 tests, all green, committed to the branch as f1e2494):
  - w4_kuna_p0_pack_values_split_inclusive_loop_edges (trailing/lone/empty `|` -> empty token, the C++ `pos<=size` port)
  - w4_kuna_p0_pack_json_string_escaping_and_control_chars (`"`/`\\` escape, 0x09 & 0x01 -> single space, `\n` -> `\n`, U+00E9 multibyte passthrough)
  - w4_kuna_p0_pack_from_code_boundaries_and_byte_safety ('0'/':' bracketing '1'..='9', length 1/2/3, and a 2-byte multibyte input that must not panic on byte indexing)
  - w4_kuna_p0_pack_restartlog_cap_is_exactly_32_per_function (33rd dropped, per-function not global)
  - w4_kuna_p0_pack_restartlog_keys_do_not_collide_interleaved (interleaved inserts across two keys stay in separate buckets, insertion order)
  - w4_kuna_p0_pack_assertlog_hint_via_hard_annotation_gate (" (applied via HARD mechanism)" only when HINT requested + HARD applied; "-" label for NONE)

## losses
- LOSS-049 (appended to MAIN docs/rust-port/losses.md): kassert's live store-mutation (Override/FuncProto/Scope retype-rename/Symbol-isolate/OptionDatabase) and per-sub-stage console `<args>` parsing are W5-deferred; the Rust port carries the routing DECISION (`Dispatch`/`validate_assertion`) and the `kassert list` rendering (`AssertLog`), not the write. Established seam pattern (cf. LOSS-030..038); no oracle exercises the store-write path at this item's boundary.

Verdict ACCEPT-WITH-LOSSES: the in-scope port is a faithful transcription (all four tables verified exactly; the JSON/list/dump emitters are byte-for-byte; the lookup API, from_code, RestartLog, and AssertLog logic match the C++), with two minor missing-cast-comment nits and one cited deferral (LOSS-049). No blocker or major stands.
