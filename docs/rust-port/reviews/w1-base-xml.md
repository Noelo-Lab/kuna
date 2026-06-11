# w1-base-xml

## Round 1

verdict: REJECT
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-10
branch: worktree-wf_cea42066-041-3 (worktree `.claude/worktrees/wf_cea42066-041-3`, reviewed at 7fb7a38; adversarial tests committed as ae13736)
cpp blob shas: both in-scope files match the checklist entry (`git hash-object` verified: xml.cc 6831da5a, xml.hh 72533012; xml.y read as the grammar source behind the generated xml.cc)

gate: `cd rust && cargo test -p kuna-base` -> **FAILS**: the verifier test
`verify_xml_deep_nesting_beyond_oracle_envelope_f1` overflows the stack and
SIGABRTs the test binary (finding F1, below). With that single test skipped
(`cargo test -p kuna-base -- --skip beyond_oracle_envelope_f1`) everything
else is green: 61 lib tests + 6 foundation verify tests + 4 xml verify tests
(485/485 differential cases byte-identical to the C++ oracle).
clippy: `cargo clippy -p kuna-base --tests -- -D warnings` -> clean.
port-audit: `cargo run -p port-audit -- --item w1-base-xml` still not
runnable — no `port-audit` crate exists (carried note from the
w1-base-foundation review, F6 here); blob SHAs checked manually.

## Method

Re-derived the scanner (`XmlScan`, xml.y:222-449 / xml.cc:2103-2307) and the
grammar (xml.y:141-219) semantics from source, including the LALR mechanics
the hand parser must reproduce (which mode-arming reductions are default
reductions, where lookahead is fetched, the `%expect 8` shift-preferred
conflicts, bison's `token <= 0 => $end` normalization, YYERROR_VERBOSE=0
plain "syntax error"), and diffed against the Rust
(`rust/crates/kuna-base/src/xml.rs`, +2189 lines).

Then built an **independent C++ oracle harness** from the vendored xml.cc
(g++ -std=c++11 -fsanitize=address,undefined; out of tree at
/tmp/kuna-xml-verify, reproducible from the description in
`tests/verify_w1_base_xml.rs`) emitting a canonical per-input trace: escaped
input bytes, full SAX callback sequence, `xml_parse` return code, and DOM
walk / `xml_tree` error text. Pinned a 485-case corpus — 125 handpicked
hunt-list cases (high bytes under signed-char, the 0xFF==EOF-sentinel
collision, NUL-as-EOF, char-ref i32 wrapping, trailing-Misc quirks,
PI/DTD/DOCTYPE position-dependent error texts, mode-arming order) plus 360
LCG-fuzz cases (verifier-chosen Knuth MMIX constants, seed
0xBADC0FFEE0DDF00D, fragment assembly + byte mutations) — and the
`xml_escape` output for every byte 1..=255. **All 485 traces and the escape
table are byte-identical** between the vendored C++ and the Rust port
(committed: `tests/data/w1_xml_oracle.out`).

Depth probing (the one divergence found): C++ accepts element nesting to
depth 4997 (`RET 0`) and rejects >= 4998 with "memory exhausted" (`RET 2`,
bison YYMAXDEPTH 10000, xml.cc:971,2042-2043) — cleanly, at any depth. The
Rust parser is iterative and accepts unboundedly (depth 200000 verified);
teardown is NOT iterative — see F1.

## Hunt list

- signedness: **clean.** The C++ reads bytes into a signed `char` then
  `int4` (xml.cc:141-156); the port reproduces signed-char semantics
  explicitly (`i32::from(b as i8)`, xml.rs:620) so bytes >= 0x80 are
  negative, fail `isChar` in comments/CDATA, and byte 0xFF collides with the
  -1 EOF sentinel — all pinned by oracle cases (`<a>\xff</a>`,
  `<a><!--\xc3\xa9--></a>`, `\xff<a/>` etc., byte-identical).
- integer widths: **clean.** Token codes/lengths int4->i32; `intb`->i64
  (`a_v_i`), `uintb`->u64 (`a_v_u`); `Attributes::getLength` keeps the
  C++ size_t->int4 narrowing shape. Char-ref conversion accumulates in i32
  exactly like the C++ `int4` (see wrapping).
- wrapping: **clean, and C++-side UB documented.** `convertCharRef`
  (xml.cc:2337-2360) overflows signed int for large references — UBSan
  confirms (xml.cc:2356-2357, signed overflow on `<a>&#4294967296;</a>`);
  g++ x86-64 wraps in practice and the port transcribes that with
  `wmul/wadd` per ADR 0003 — oracle-pinned (`&#99999999999999999999;`,
  `&#2147483648;`, `&#xffffffff;` all byte-identical). Recorded as UB-3 in
  `docs/rust-port/upstream-bugs.md`.
- comparator totality: **clean.** No comparators; `tagmap` is
  std::map->BTreeMap<String,_> (same key order).
- iteration-order provenance: **clean.** Attribute lists and child lists are
  C++ vectors -> Rust Vec (insertion order); the TreeHandler restructure
  (children linked at endElement instead of startElement) provably preserves
  child order because a parent acquires no other children while one child is
  open — DOM walks oracle-pinned across all 485 cases. No HashMap/HashSet
  (grep + workspace clippy deny).
- off-by-one / do-while / reverse iterators: **clean.** The 4-byte lookahead
  ring (`(pos+i)&3`), the synthetic `'\n'` at EOF, `]]>`/`--` lookahead
  fall-throughs, scanCharRef's "x"-only backtrack (`lvalue.len()==1`), and
  the exactly-one-trailing-Misc document rule are all transcribed and
  oracle-pinned (e.g. `<a/><!--c-->` rejected, `<a><![CDATA[a]]b]]></a>`
  content `a]]b`).
- erase-while-iterating: **n/a.** Nothing in scope erases during traversal.
- exception -> Result partial-state parity: **clean with documented
  departures.** Mid-parse SAX callbacks before an error fire identically
  (oracle traces include events-then-setError sequences); single setError
  per failed parse on both sides; `xml_tree` discards the partial DOM on
  both sides; `DocumentStorage::parse_document` push-after-success vs C++'s
  inert null slot (xml.cc:2444-2450) is invisible to every accessor;
  `getAttributeValue` unknown-attribute error text matches
  ("Unknown attribute: <nm>").
- LALR fidelity of the hand parser (LOSS-006-specific hunt): **clean.**
  Mode-arming reductions verified to be bison default reductions
  (single-item states), so the hand parser's arm-then-fetch order consumes
  the byte stream identically; the `<?x` shift-preference, PI-vs-xmldecl
  initial-position split, leading-`<!DOCTYPE` plain-syntax-error quirk,
  unchecked ETag names, and `$end` normalization all oracle-pinned.

## Findings

- F1 (major): unbounded-depth parse + recursive DOM `Drop` aborts the
  process on input the C++ oracle rejects cleanly.
     cpp: decompiler/cpp/xml.cc:971 (YYMAXDEPTH 10000), 2042-2043
     (yyexhaustedlab -> yyerror("memory exhausted"), return 2), 2417-2424
     (Element::~Element recursion — safe in C++ because the depth cap keeps
     it < ~5000)
     rust: rust/crates/kuna-base/src/xml.rs:1200-1205 (iterative
     parse_element, no depth cap), 245-256 + default recursive `Drop` of
     `Element`/`children: Vec<Rc<Element>>`, 52-55 (module-doc claim "deep
     documents cannot overflow the Rust call stack" — false for teardown)
     claim/observed: C++ fails CLEANLY at every depth (<=4997 parses,
     >=4998 "memory exhausted", never crashes). Rust parses any depth, then
     the recursive Drop overflows the stack and SIGABRTs: depth 6000 on a
     2 MiB cargo-test thread, depth 30000 on an 8 MiB main thread; depths
     4998..~5500 are silently *accepted* where C++ errors (also see F2).
     Within the oracle-accepted envelope (<=4997) the Rust port is safe
     (verified at 4990/4997 on a 2 MiB thread,
     `verify_xml_deep_nesting_oracle_accepted_envelope`).
     repro: `cd rust && cargo test -p kuna-base --test verify_w1_base_xml
     verify_xml_deep_nesting_beyond_oracle_envelope_f1` -> SIGABRT (stack
     overflow).
     work order: make teardown non-recursive (e.g. `impl Drop for Element`
     draining `children` into a worklist via `Rc::try_unwrap`) **or**
     emulate the bison depth cap with the exact "memory exhausted" error.
     The committed test accepts either outcome; if unbounded depth is kept,
     append the accept/reject divergence to losses.md (F2) and fix the
     module-doc claim.
- F2 (minor): accept/reject divergence beyond the bison stack cap is
  undocumented.
     cpp: decompiler/cpp/xml.cc:971,2042-2043 (depth >= 4998 -> "memory
     exhausted", xml_parse returns 2)
     rust: rust/crates/kuna-base/src/xml.rs:52-55,1381-1394 (no cap;
     xml_parse returns only 0/1, so even a cap-emulating repair should
     decide the return-code question)
     claim/observed: depth-4998+ documents parse successfully in Rust
     (verified to 200000) but are errors in C++. Pathological input only —
     no in-tree XML is deeper than ~10 — but it is a test-visible
     divergence from defined oracle behavior and per protocol must either
     be repaired (cap emulation) or cited as a losses.md entry. The module
     doc mentions the bison stack but does not record the divergence.
     repro: depth-6000 nested doc -> C++ harness `P RET 2`/"memory
     exhausted"; Rust `xml_tree` -> Ok.
- F3 (minor): bare `as` casts in ported logic without per-site `// cast:`
  justification comments.
     cpp: decompiler/cpp/xml.cc:2114-2307 (`*lvalue += getxmlchar()` int4
     -> char truncation sites)
     rust: rust/crates/kuna-base/src/xml.rs:714,723,738,752,774,793,803,
     808,827,832 (`self.getxmlchar() as u8`; only the first site at 694
     carries the justifying comment), plus size casts at 144,320,874,876
     (`len() as i32`)
     claim/observed: the casts are all correct (scanner loop guards make
     the value a byte; the i32 length casts mirror C++ int4 narrowing) but
     the protocol requires each bare `as` to carry a justification comment.
     Same class as foundation F1; fix opportunistically in the F1 repair
     pass.
- F4 (note): `xml_escape` boundary departs from C-string semantics,
  documented in code (xml.rs:1420-1422): C++ stops at the first NUL byte
  (xml.cc:2492-2509, `while(*str!='\0')`), Rust escapes the whole slice. A
  value with an embedded NUL (constructible via `&#0;`) would render
  differently through `a_v`. Invisible to all current callers; watch when
  later waves port writers that pass parsed content back out.
- F5 (note): C++ `a_v_u`/`a_v_i` mutate persistent ostream state
  (`<< hex`/`<< dec` stick to the stream, xml.hh:352-367); the Rust
  Vec<u8>-based writers are stateless per call. Identical output for the
  helpers themselves (oracle-equivalent formatting verified in module
  tests), but later-wave ports of C++ code that relies on a *leftover*
  stream base after calling these helpers must port that flag explicitly.
- F6 (note): `cargo run -p port-audit -- --item w1-base-xml` still not
  executable — no port-audit crate in the workspace (carried from
  w1-base-foundation F4). Blob SHAs verified manually.
- F7 (note): the C++ signed-overflow UB in `convertCharRef` found while
  harness-building is now recorded as UB-3 in
  `docs/rust-port/upstream-bugs.md` (the port's ADR-0003 wrapping is the
  correct pin of g++ x86-64 behavior and is oracle-verified).

## Adversarial tests

Committed in the worktree as ae13736
(`rust/crates/kuna-base/tests/verify_w1_base_xml.rs` +
`tests/data/w1_xml_oracle.out`, oracle output from the verifier's own C++
harness over the vendored xml.cc, ASan/UBSan-clean apart from the UB-3
signed-overflow diagnostics):

- verify_xml_escape_all_bytes_matches_cpp_oracle — xml_escape over bytes
  1..=255 vs the C++ output (signed `*str < '?'` fast path is behaviorally
  inert). PASSES.
- verify_xml_handpicked_traces_match_cpp_oracle — 125 hunt-list cases:
  SAX sequences, error texts, RET codes, DOM walks byte-identical. PASSES.
- verify_xml_fuzz_traces_match_cpp_oracle — 360 LCG-fuzz documents
  (valid + mutated), byte-identical. PASSES.
- verify_xml_deep_nesting_oracle_accepted_envelope — depths 4990/4997
  (the C++-accepted limit) parse, walk, and drop safely on a 2 MiB test
  thread. PASSES.
- verify_xml_deep_nesting_beyond_oracle_envelope_f1 — depth 6000: encodes
  the post-repair contract (Ok + safe teardown, or exactly
  "memory exhausted"); currently SIGABRTs the test binary = the F1
  evidence and the round-2 gate.

losses: LOSS-006 covers the hand-written-parser substitution itself (its
"identical accept/reject behavior and identical error text" criterion is
met on all 485 differential cases); the depth-cap divergence (F2) is NOT
yet covered by any loss entry — repair or append one in round 2.

## Round 2

verdict: ACCEPT-WITH-LOSSES
verifier: independent verifier agent (Claude, not the porter)
date: 2026-06-11
branch: worktree-wf_cea42066-041-3 (reviewed at repair commit 7bc5a58;
round-2 adversarial tests committed as 526b01d)
cpp blob shas: unchanged from round 1 (re-verified: xml.cc 6831da5a,
xml.hh 72533012, xml.y 222e9d49)

gate: `cd rust && cargo test -p kuna-base` -> **PASSES**: 61 lib + 6
foundation verify + 5 xml verify (round 1, incl. the F1 gate test
`verify_xml_deep_nesting_beyond_oracle_envelope_f1`, now green; the 1
"ignored" is the round-1 verifier's `dump_corpus` utility, by design) + 5
xml verify (round 2). 485/485 oracle differential traces still
byte-identical.
clippy: `cargo clippy -p kuna-base --all-targets -- -D warnings` -> clean.
port-audit: still no `port-audit` crate (carried note, was F6); blob SHAs
checked manually.

### Repair verification (against round-1 work orders)

- F1 (major) — **repaired and verified.** `impl Drop for Element`
  (xml.rs:348-364) replaces the default recursive drop glue with an
  explicit worklist: `mem::take(&mut self.children)`, pop, and for each
  child either `Rc::try_unwrap` succeeds (sole owner: grandchildren are
  appended to the worklist, so the unwrapped element drops with empty
  `children` — its own Drop is O(1)) or fails (shared: the handle drop
  only decrements the strong count; the surviving owner tears down later
  by the same mechanism). Re-derivation: recursion depth is constant on
  every path; no `Weak` exists in the crate so `try_unwrap` succeeding
  iff strong==1 is the full ownership story; no parent back-pointers, so
  no Rc cycles/leaks. The round-1 gate test passes (depth 6000 on a 2 MiB
  test thread), and the round-2 adversarial battery (below) holds it to
  200000 deep, 120000-element wide-deep combs, shared-handle teardown in
  both extreme orders, and the parse-error discard path.
- F2 (minor) — **repaired.** The accept/reject divergence beyond the
  bison cap is now LOSS-010 in `docs/rust-port/losses.md` (correct
  anchors xml.cc:971, 2042-2043; honest surface statement including the
  return-code difference and the boundary's high-water-mark sensitivity;
  concrete restoration criteria). The boundary claims re-confirmed this
  round against the round-1 C++ oracle harness: depth 4997 -> RET 0 +
  full tree walk, depth 4998 -> `P RET 2` / "memory exhausted". The
  false module-doc claim is rewritten (xml.rs:52-61) and now states the
  divergence and cites LOSS-010.
- F3 (minor) — **repaired.** All round-1-flagged sites now carry
  justifications: `// cast:` comments on the scanner `getxmlchar() as u8`
  sites and the size_t->int4 narrowings in `get_length`/
  `get_num_attributes`/`print_content`. See R2-F1 (note) for the
  remaining cosmetic gap.

### Findings (round 2)

- R2-F1 (note): a few `as` casts still lack the literal `// cast:` prefix,
  though all are justified or trivially lossless.
     cpp: decompiler/cpp/xml.cc:2114-2307 (scanner), 2425-2431 (attribute
     index accessors take int4)
     rust: rust/crates/kuna-base/src/xml.rs:726 (comment reads
     "// (char) low byte, as C++ string +=" without the `cast:` tag —
     the pre-repair original site), 937/1195/1333 (prose justification
     on the preceding lines), 95 (`u8 as i32`, lossless widening),
     162/167/172/333/338 (`i as usize` index casts mirroring C++ int4
     vector indexing; negative -> Rust panic vs C++ UB), 280
     (`start as usize` slice bounds in add_content, same class)
     claim/observed: protocol-letter gap only; every site is semantically
     correct and most were implicitly passed in round 1. Cosmetic — fold
     into any future touch of the file.
- R2-F2 (note, carried from round-1 F6): `cargo run -p port-audit --
  --item w1-base-xml` still not executable — no port-audit crate in the
  workspace. Blob SHAs verified manually both rounds.
- Round-1 notes F4 (xml_escape NUL boundary), F5 (a_v_u/a_v_i stream-state
  statelessness), F7 (UB-3 recorded) stand unchanged as watch-items for
  later waves; none is load-bearing for this item.

### Hunt list (round 2 delta)

All round-1 entries stand (the repair touched no scanner/parser logic —
`git show 7bc5a58` is Drop + comments + docs only; 485/485 oracle traces
re-confirmed via the gate). Round-2-specific hunts on the repair:

- copy-vs-reference aliasing of the Drop steal: **clean** — try_unwrap
  moves only when sole-owned; shared-subtree integrity proven by test
  (survivor keeps its full 40000-deep subtree after the Document drops).
- iteration/teardown order of the worklist (pop-back + append vs C++
  forward recursion): **clean** — Element drop has no side effects beyond
  deallocation, so order is unobservable; child-list *content* unchanged.
- partial-state parity of the error path: **clean** — failed parses
  discard a 100000-deep partial DOM through the Err return without
  overflow, error text "syntax error" as in C++ (YYERROR_VERBOSE=0).

### Adversarial tests (round 2)

Committed in the worktree as 526b01d
(`rust/crates/kuna-base/tests/verify_w1_base_xml_r2.rs`), all passing:

- verify_xml_r2_shared_subtree_survives_parent_drop_then_drops_safely —
  Rc kept 10000 levels into a 50000-deep chain: Document drop must free
  the levels above without stealing from the shared survivor (subtree
  intact, strong_count back to 1), then the survivor's own 40000-deep
  teardown must not overflow.
- verify_xml_r2_tagmap_shared_handles_both_drop_orders — tagmap-style
  shared handle at every level of a 20000-deep chain, BTreeMap key order
  arranged shallow-first and deepest-first (the two extreme teardown
  schedules: 20000 O(1) root drops vs one whole-chain worklist drain).
- verify_xml_r2_wide_deep_comb_parse_and_drop — 30000 spine levels x 3
  leaf children (120000 elements through the worklist), full structural
  walk then drop.
- verify_xml_r2_depth_200k_and_loss010_boundary — the rewritten
  module-doc claim at full strength (depth 200000 parse+walk+drop on a
  2 MiB test thread) plus the LOSS-010 pin: C++-rejected depth 4998
  parses Ok in Rust.
- verify_xml_r2_error_path_discards_deep_partial_dom_safely — truncated
  document fails with "syntax error" after building a 100000-deep closed
  subtree; the internal discard must be as stack-safe as the success path.

losses: LOSS-006 (hand-written parser replacing the bison grammar;
criterion met on all 485 differential cases) + LOSS-010 (nesting depth
uncapped — accept/reject divergence beyond the bison YYMAXDEPTH cap,
pathological input only, no in-tree XML deeper than ~10). Both cited
entries exist in `docs/rust-port/losses.md` on the branch. No blocker or
major finding stands -> ACCEPT-WITH-LOSSES.
