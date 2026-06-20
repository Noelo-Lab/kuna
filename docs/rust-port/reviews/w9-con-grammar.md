# w9-con-grammar
verdict: ACCEPT-WITH-LOSSES
verifier: claude-opus-4-8 (independent verifier subagent)
date: 2026-06-13
gate: `cd rust && cargo test -p kuna-console` -> PASS (51 lib tests, 0 failed; includes the 7 round-2 verifier adversarial tests, commit `23d7849`). `cargo clippy -p kuna-console --tests -- -D warnings` -> clean.

## Round history

- Round 1: **REJECT** (F1 major + F2 minor). F1 = `parse_c_integer` full-string-parsed
  the C-declaration NUMBER token and saturated to `i64::MAX` on any invalid trailing
  char, where C++ `grammar.cc:1796-1804` `istringstream >> intb` (base flags unset)
  does a strtoll-style leading-prefix scan. Recorded as **LOSS-124**.
- Round 2 (this verdict): the porter's repair commit `752184b` reworked
  `parse_c_integer` into a faithful leading-prefix scanner (exactly the LOSS-124
  restoration criteria). **F1 is verified RESOLVED** against a fresh C++ oracle (a
  4000-token differential corpus + all edge cases). F2 (the bare-`as`-cast convention
  finding) persists as a documented **minor** loss and does not block ACCEPT.

## Scope

C++ in scope: `decompiler/cpp/grammar.cc` (+ `grammar.hh`, `grammar.y`) — the
bison-generated C-declaration parser. Per **LOSS-006** the generated LALR automaton is
not transliterated; the port hand-writes the *semantics* (lexer FSM, the reduce-action
builder methods, the AST, and a recursive-descent parser over `grammar.y`). Rust:
`rust/crates/kuna-console/src/grammar.rs` (+ `grammar/tests.rs`). The interface-command
parsers (`parse_machaddr`/`parse_varnode`/`parse_op`/`parse_toseparator`) and the
`parse_C` typedef/extern dispatcher are out of grammar.y scope (separate console
concerns); `parse_type`/`parse_protopieces` are the in-scope entry points and are ported.

## Hunt list

- **Signedness** — clean. `bufstart`/`bufend`/`colno`/`lineno` kept as `int4` (i32)
  exactly as C++. The `(self.bufend - self.bufstart)` / `bufend-1` / `bufend-2` index
  arithmetic is i32 and each site is guarded so it stays non-negative (`bufend>1` before
  `bufend-2`; token-start sets `bufstart=bufend-1` with `bufend>=1`). `count > 1`
  qualifier counts are signed in both. Verified no-panic by adversarial v6.
- **Integer widths** — clean (was F1, now resolved). `parse_c_integer` (grammar.rs:244)
  now mirrors the C++11 `num_get` facet: optional ws/sign, auto-base, longest valid-digit
  run, per-sign signed saturation. `(int4)*num` array-size / bitfield truncation
  (`num as int4`, `n as int4`) is faithful low-32-bit two's-complement reinterpretation
  (verified end-to-end by v4: `int4 a[4294967296]`->int4 0->invalid;
  `int4 b[4294967297]`->int4 1->valid). `intb=i64`, `uintb=u64`, matching C++.
- **Wrapping** — clean. The only integer arithmetic on `uintb`-lineage values is the
  `checked_mul`/`checked_add`/`checked_sub` accumulation in `parse_c_integer` with
  explicit `unwrap_or(i64::MAX/MIN)` saturation — proven to match the C++ overflow
  behavior in BOTH directions by the oracle (positive->i64::MAX, negative->i64::MIN,
  exact i64::MIN/MAX representable). The final `val as uintb` re-tag matches `(uintb)val`.
- **Comparator totality** — n/a (no ported `operator<`/sort; the only map is the
  lookup-only `keywords` BTreeMap).
- **Iteration-order provenance** — clean. `keywords` is a `BTreeMap` (C++
  `map<string,uint4>`, ordered; lookup-only in `lookup_identifier`/`convert_flag`/
  `add_func_specifier`, order immaterial). The pointer-qualifier vector is built
  left-to-right then `reverse()`d (grammar.rs:1856) to reproduce grammar.y's
  right-recursive `pointer` production order; pointer COUNT (the only output-affecting
  part — `modType` ignores PointerModifier flags) is preserved (v7). No `HashMap`/`HashSet`.
- **Off-by-one / do-while / reverse iterators** — clean. `getNextToken`'s
  `do{}while(tok==0)` -> `loop{...; if tok!=0 break}` keeps at-least-once. `buildType`'s
  `iter=mods.end(); while(iter!=mods.begin()){--iter;...}` -> `mods.iter().rev()` faithful.
  `getPrototype`'s "apply mods except the leading function mod" (`--iter` once then
  reverse-walk to begin) -> `mods.iter().skip(1).rev()` faithful (re-derived: both apply
  `mods[last..=1]` outermost-first, excluding `mods[0]`). `firstVarArgSlot =
  dotdotdot ? intypes.size() : -1` faithful (grammar.rs:977 vs grammar.cc:2375).
- **Erase-while-iterating** — n/a (the C++ `*_alloc` arena lists are pure cleanup; the
  Rust by-value AST drops them, no traversal-mutation).
- **Exception -> Result partial-state parity** — clean. `parse_stream`'s
  "lasterror set => Ok(false) parse-failure; lasterror empty => Err thrown-exception"
  discriminator faithfully splits C++ `yyparse()!=0` (caught, re-thrown as ParseError by
  the entry point) from a `throw` inside an action that `runParse` never catches (the
  struct/enum construction SEAMs). `setError`-then-continue (`convert_flag`/
  `add_type_specifier`/`add_func_specifier` record the error and keep going; the next
  `lex()` latches BADTOKEN via `if !lasterror.is_empty()` at grammar.rs:1282) is preserved.

## Findings

- **F1 (major) — RESOLVED in round 2.** `parse_c_integer` prefix-parse divergence
  (LOSS-124). The repair (commit `752184b`) replaced the full-string `from_str_radix`
  with a leading-prefix scanner and corrected the false doc-comment. Verified against a
  standalone `g++ -std=c++11` `istringstream + unsetf(dec|hex|oct) >> intb` oracle:
  a 4000-token random differential corpus (`w9_oracle_corpus.txt`) PLUS every hand-picked
  edge agree with the port — `1z`->1, `123abc`->123, `08`->0, `5_0`->5, `007_0`->7,
  `0x`/`0X`/`0xx`->0, `0x10g5`->16, `-0x1`->-1, `-0xFFFFFFFFFFFFFFFF`->i64::MIN,
  `-010`->-8, both-direction overflow saturation, exact i64::MIN/MAX, leading-ws skip.
  The `int4 a[1z]` end-to-end case now builds a valid `int4[1]`. LOSS-124 RESOLUTION
  NOTE appended to `losses.md`.
       cpp: decompiler/cpp/grammar.cc:1796-1804
       rust: rust/crates/kuna-console/src/grammar.rs:244-312

- **F2 (minor, persists from round 1): ~18 bare `as` casts lack the `// cast:`
  justification the mechanical pass requires.** The repair added `// cast:` notes inside
  `parse_c_integer` but the rest of grammar.rs still has uncommented bare casts:
  `bytes[0] as uintb` (172), `other as uintb` (184), `(bytes[i] as char)` in the ws-skip
  prologue (250), the `as usize` buffer indices (376, 539, 550, 583, 629, 630, 661, 670,
  680, 1240), `b':' as uint4` (530), `other as u8` (1317), `pieces.intypes.len() as int4`
  (977), and the `(int4)`-truncation casts `num as int4` (1427) / `n as int4` (1679).
  **Each is individually faithful** to the C++ (`(uint4)buffer[bufstart]`, `(int4)*num`,
  index reinterpretations on values the lexer bounded to ASCII / non-negative), so this is
  a convention finding, **not a correctness one**, and does not block ACCEPT. Restoration:
  add `// cast:` notes (or a module-level waiver) on a follow-up.
       cpp: decompiler/cpp/grammar.cc:2061,2600 (and index expressions)
       rust: rust/crates/kuna-console/src/grammar.rs (172,184,250,530,539,977,1317,1427,1679,...)

## Non-findings (re-checked round 2, faithful)

- char-constant escape decode (`set_with`, including `len==1`/escape-table/default
  `ptr[1]`; high bytes unreachable since `moveState` rejects >=127 -> signed/unsigned
  `char` cast immaterial), the lexer `number`-state `0x` guard (only lowercase `x` at the
  exact `0x` position is a hex indicator; `X`/mid-number `x` handled as plain/`1x`->BADTOKEN),
  `establishToken`'s `buffer[bufstart..bufend-1]` slice, the EOF "simulate a trailing
  space" final-token resolution (+1 buffer slot makes the C++ over-allocation provably
  safe), `f(void)`->empty paramlist, varargs null-marker pop + `new_func` unwrap (no
  interior null reaches it), the struct/union/enum/nested-func-pointer construction SEAMs
  (correct C++-faithful explain text, lasterror-empty -> propagate as Err), `set_error`
  location formatting (`writeLocation`/`writeTokenLocation` verbatim), `lookupIdentifier`
  keyword classification, the `parse_type`/`parse_protopieces` entry-point error-string +
  ordering parity (`Did not parse a datatype`/`Parsed multiple declarations`/
  `Parsed type is invalid`), `is_valid` storage/qualifier-count throw order + strings,
  the `syntax_error` "Syntax error" default-only-if-empty.

## Mechanical pass

- `cargo test -p kuna-console`: 51 passed, 0 failed.
- `cargo clippy -p kuna-console --tests -- -D warnings`: clean.
- grep `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable` in grammar.rs: none.
- grep `partial_cmp`/float-unwrap: none.
- `wrapping_`/`checked_`: only the `parse_c_integer` saturating accumulator (verified).
- bare `as` casts: present without `// cast:` outside `parse_c_integer` (F2, minor).
- (port-audit not in this worktree's workspace; blob-sha drift not machine-checkable here.)

## Adversarial tests (committed on branch, `23d7849`)

In `rust/crates/kuna-console/src/grammar/tests.rs`, tagged `w9_con_grammar_v*`, with the
oracle corpus `grammar/w9_oracle_corpus.txt`:
- `w9_con_grammar_v1_sign_applies_to_nondecimal_base`
- `w9_con_grammar_v2_positive_overflow_saturates_no_wrap`
- `w9_con_grammar_v3_whitespace_and_bare_prefixes`
- `w9_con_grammar_v4_array_size_int4_truncation_zero_rejected` (end-to-end)
- `w9_con_grammar_v5_differential_against_cpp_oracle` (4000-token g++ differential)
- `w9_con_grammar_v6_lexer_boundary_no_panic` (Line too long / Incomplete token / comment edge)
- `w9_con_grammar_v7_pointer_array_modifier_order` (end-to-end pointer/array binding)

The round-1 porter also flipped the four former `should_panic` tests
(`w9_con_grammar_parse_int_{letter_suffix,trailing_alpha,bad_octal_digit,underscore}`) to
assert the oracle prefix values, and updated `w9_con_grammar_array_size_letter_suffix` to
expect a valid `int4[1]` — all pass.

## losses

- **LOSS-124** (F1): **RESOLVED** — resolution note appended to `losses.md` (not edited).
- **F2** (bare `as` casts outside `parse_c_integer`): minor convention loss, recorded as
  **LOSS-126** in `losses.md`. No correctness impact; ACCEPT-WITH-LOSSES on this item.
  ACCEPT stands.
