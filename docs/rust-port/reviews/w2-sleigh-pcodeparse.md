# w2-sleigh-pcodeparse
verdict: ACCEPT
verifier: opus-4.8 (independent)
date: 2026-06-11
(round 1 REJECT superseded by round 2 ACCEPT below; F1 repaired)

## Round 1

gate: `cd rust && cargo test -p kuna-sleigh` ->
  - 200 in-crate unit tests PASS (the porter's tests, incl. 41 pcodeparse tests);
    all other verify_* integration suites PASS.
  - NEW verifier suite `verify_w2_sleigh_pcodeparse` (5 tests): 3 pass, **2 FAIL**
    (`verify_w2pp_octal_partial_parse_zero_eight`, `verify_w2pp_octal_partial_parse_seven`)
    — these failures are the concrete divergence trace for F1 below.

scope: the bison-generated `pcodeparse.cc` LALR tables are NOT the source of truth;
the real C++ is `pcodeparse.y` (grammar + hand-written lexer trailer, lines 226-806)
and `pcodeparse.hh`. The port replaces the bison parser with a hand-written
recursive-descent + precedence-climbing parser (LOSS-006) and transcribes the
`PcodeLexer` state machine character-for-character. Review re-derived the grammar
against the recursive-descent translation.

hunt list:
- Signedness: clean. The only signed/unsigned interplay is `findIdentifier`'s
  `int4 low/high` binary search; the port keeps `i32 low/high` so
  `targ = (low+high)/2` arithmetic and the `low <= high` (do/while) termination
  match. `(uint4)*$N` grammar casts are reproduced as `as u32` width truncations
  (see F2).
- Integer widths: clean except F1. `*$3 * 8` (uintb mul then `(uint4)` truncate)
  -> `n.wmul(8) as u32` (correct: wrapping mul then truncate, line 2156).
  `sizeof(uintm)` = 4 (uintm = uint32) -> literal `4` (lines 1786). `(uint4)*$N`
  offset/size casts -> `as u32` everywhere. The number LEXER value path diverges
  (F1).
- Wrapping: clean. `allocateTemp`'s `tempbase += 16` -> `.wadd(16)` (line 1103);
  `*$3 * 8` -> `.wmul(8)` (line 2156). No other arithmetic on uintb-lineage values.
- Comparator totality: clean. The `SymbolTree` order (`a->getName() < b->getName()`,
  std::string `<` = unsigned-byte memcmp) is reproduced by `BTreeMap<Vec<u8>>`
  (unsigned-byte lexicographic). Total order on both sides.
- Iteration-order provenance: clean. The only ported container is the `SymbolTree`
  (`BTreeMap<Vec<u8>, SnippetLocal>`). It is only iterated in `clear()` (delete all
  non-space) and at drop — neither is order-sensitive for output. Lookups
  (`tree.find`) are order-independent. `paramlist` preserves source order via
  push_back. No HashMap/HashSet.
- Off-by-one / do-while / reverse iterators: clean. `findIdentifier`'s C++ do/while
  (`do {...} while(low<=high)`) is ported as `loop { ...; if low>high break }`,
  preserving the at-least-once semantics (line 510). `%nonassoc` chaining verified
  (verify_w2pp_nonassoc_chain_rejected_mixed_accepted PASSES); additive
  left-associativity verified (verify_w2pp_additive_left_associative PASSES).
- Erase-while-iterating: clean. `clear()` uses `BTreeMap::retain` (collect-equivalent),
  matching the C++ erase-non-space loop; no index-rewind hazard.
- Exception -> Result partial-state parity: clean. The BADINTEGER quirk is faithfully
  reproduced: jumpdest/integervarnode BADINTEGER call yyerror (records firsterror,
  bumps errorcount) but do NOT YYERROR — the port sets `failed=true` via yyerror then
  immediately `failed=false` (lines 1770, 2229), so the parse continues and
  parse_stream returns true with an error recorded. YYERROR paths (Unknown jump
  destination, etc.) keep the FIRST error in firsterror because report_error only sets
  it when errorcount==0; the later "Syntax error" does not overwrite. Verified by the
  porter's reject_* tests (all PASS).

findings:
  - F1 (MAJOR): number-literal parsing diverges from the C++ `istringstream >> uintb`
    for leading-zero (octal-base) tokens containing a non-octal digit (8 or 9). The
    C++ stream reads the longest valid octal prefix and STOPS at the first invalid
    digit, SUCCEEDING with a partial value; `from_str_radix(_, 8)` requires every digit
    valid and fails the whole token.
       C++ oracle (verified with a standalone `istringstream` program):
         "08"   -> INTEGER 0   (reads "0", stops at "8"; no failbit)
         "0789" -> INTEGER 7   (reads "07" = 7, stops at "8")
         "019"  -> INTEGER 1   ; "0178" -> 15 ; etc.
       Rust port: each of these -> `from_str_radix` Err -> `None` -> BADINTEGER, which
       (a) yields const 0 (wrong value) and (b) records the spurious error
       "Parsed integer is too big (overflow)" (errorcount != 0). Because the lexer's
       `decstring` state accepts any `isDec` digit (0-9), `08`/`0789` ARE single tokens,
       so this is reachable from any snippet. A `08` appearing where INTEGER-vs-
       BADINTEGER changes a size/jumpdest also flips control. The decimal and hex paths
       do NOT diverge (those token states only admit valid digits; only overflow fails).
         cpp:  decompiler/cpp/pcodeparse.cc trailer (pcodeparse.y:587-595,
               `istringstream s1; s1.unsetf(ios::dec|ios::hex|ios::oct); s1 >> curnum;`)
         rust: rust/crates/kuna-sleigh/src/pcodeparse.rs:823-839 (`parse_number`,
               `u64::from_str_radix(digits, 8)`)
       trace: rust/crates/kuna-sleigh/tests/verify_w2_sleigh_pcodeparse.rs
               verify_w2pp_octal_partial_parse_zero_eight (FAILS),
               verify_w2pp_octal_partial_parse_seven (FAILS).
       fix: emulate the stream partial-parse — consume the longest prefix valid in the
       detected base; succeed (value = parsed prefix) if >=1 digit was consumed; fail
       (BADINTEGER) only on a u64 overflow of the consumed prefix or zero digits
       consumed (the `0x`-with-no-digits case, which C++ also fails). Note the prefix
       must be parsed with wrapping/checked semantics that reject only true overflow.

  - F2 (MINOR / note, not blocking): the `as u32` / `as i32` / `as usize` casts in the
    diff are all faithful reproductions of explicit C++ `(uint4)` grammar casts or the
    `int4` binary-search indices, and are semantically correct, but none carry the
    `// cast:` justification comment the verification protocol's mechanical pass calls
    for. Recommend annotating them when F1 is addressed. Not a correctness issue.

mechanical pass:
  - grep: no `todo!`/`unimplemented!`/`HashMap`/`HashSet`/`sort_unstable`. `panic!`/
    `expect`/`unreachable!` are all on internal invariants unreachable from parser input
    (e.g. `specificsymbol_varnode` only called on already-matched symbol tokens; the
    `addSymbol` non-varnode panic only fires on an internal-API misuse, never on snippet
    text). Bare `as` casts: see F2 (correct, uncommented).
  - `cargo clippy -p kuna-sleigh -- -D warnings`: clean.

adversarial tests (committed on the port branch, f8d89c9):
  rust/crates/kuna-sleigh/tests/verify_w2_sleigh_pcodeparse.rs
  - verify_w2pp_octal_partial_parse_zero_eight   (FAILS — F1)
  - verify_w2pp_octal_partial_parse_seven        (FAILS — F1)
  - verify_w2pp_number_boundaries_match_oracle    (passes: lone 0, 00, octal 010,
    hex 0xfe, real overflow recorded as overflow error)
  - verify_w2pp_nonassoc_chain_rejected_mixed_accepted (passes: `< ... s<` rejected,
    `< ... ==` accepted as `(a<b)==c`)
  - verify_w2pp_additive_left_associative         (passes: `a-b-c` => `(a-b)-c`)

losses: none cited. F1 is a faithful-port bug with a well-defined C++ oracle and a small
fix, not a deliberate divergence, so it is a work order (REJECT), not a losses.md entry.

work order for the next attempt:
  1. Rewrite `parse_number` to emulate the C++ stream partial-parse (F1). The detected
     base is correct (0x->hex, leading-0->octal, else decimal); the bug is requiring all
     digits valid. Stop at the first digit invalid in the base and return the prefix
     value; only overflow or zero-consumed-digits -> None (BADINTEGER).
  2. (optional) add `// cast:` comments to the truncating casts (F2).
  Keep the two failing verify_w2pp_octal_* tests as the regression oracle.

## Round 2

verdict: ACCEPT
date: 2026-06-11
port commits since round 1: f8d89c9 (r1 verifier tests, already present),
  ded4b49 ("repair fixes"). The repair is surgical: it touches ONLY
  `parse_number` (+41/-10 lines in src/pcodeparse.rs); no other parser logic
  changed.

gate: `cd rust && cargo test -p kuna-sleigh` -> 255 passed, 0 failed across all
  suites (200 in-crate unit tests + integration suites). The verifier suite
  `verify_w2_sleigh_pcodeparse` now has 9 tests (5 round-1 + 4 round-2), all
  PASS — including the two octal tests that FAILED in round 1
  (`verify_w2pp_octal_partial_parse_zero_eight`,
  `verify_w2pp_octal_partial_parse_seven`). F1 is resolved.

F1 (was MAJOR) — RESOLVED. `parse_number` now emulates the C++
  `istringstream >> uintb` with `unsetf(dec|hex|oct)` partial-parse: detect base
  from the prefix, consume the longest run of digits valid in that base, STOP at
  the first invalid digit while SUCCEEDING with the consumed prefix's value;
  `None` (=> BADINTEGER) only on zero consumed digits (empty `0x`) or a u64
  overflow of the accumulated prefix (checked_mul/checked_add).
    rust: rust/crates/kuna-sleigh/src/pcodeparse.rs:828-864 (parse_number).
  Independently re-derived the C++ oracle with a standalone program
  (`istringstream s1; s1.unsetf(ios::dec|ios::hex|ios::oct); s1 >> curnum;`) and
  cross-checked the port value-for-value:
    "0"->0  "00"->0  "08"->0  "0789"->7  "019"->1  "0178"->15  "010"->8
    "008"->0  "0008"->0  "09"->0  "0789012"->7
    "0x10"->16  "0xfe"->254  "0xffffffffffffffff"->2^64-1  "0xABCDEF"->0xABCDEF
    "0x"->BADINTEGER  "0xg"->BADINTEGER (zero hex digits)
    "0x10000000000000000"->BADINTEGER  "18446744073709551616"->BADINTEGER
    "01777777777777777777777"->2^64-1  octal/dec overflow ->BADINTEGER
  Every case matches. The overflow path (C++ stream clamps to max + sets failbit
  => BADINTEGER) is reproduced by `checked_*? => None`. Faithful.

  Reachability note (re-derived from the lexer, not assumed): the C++ `moveState`
  `case '0'` enters the hexstring state ONLY on lowercase `x`
  (`if (lookahead1=='x')`, pcodeparse.y:475-479); capital `X` is neither a hex
  trigger nor a decimal digit, so `0X1A` lexes as decstring `0` + identifier
  `X1A` and (with X1A unbound) is a SYNTAX ERROR — NOT a hex literal. The port
  reproduces this exactly (verify_w2pp_r2_capital_x_is_not_hex_trigger PASSES:
  `r1 = 0X1A;` -> "Syntax error", `r1 = 0x1A;` -> 0x1A). parse_number's
  capital-`0X` handling is therefore dead-but-harmless code; the observable
  behavior matches C++. The reachable partial-parse path is octal-only
  (`08`/`0789`/... via isDec admitting 8/9), and it is now correct.

F2 (MINOR, unchanged, NON-BLOCKING): the bare `as` casts in pcodeparse.rs
  (binary-search `targ as usize`, the digit `(b - b'0') as u64` widenings, the
  `(uint4)` grammar-cast reproductions `size/off/nbits as u32`, `wmul(8) as u32`)
  are all semantically faithful to explicit C++ `(uint4)`/`int4` casts but still
  carry no `// cast:` justification comment. The repair did not annotate them.
  This is a convention gap (the protocol's mechanical pass wants the comment),
  not a correctness divergence; it does not rise to blocker/major and does not
  block ACCEPT. Recommend annotating in a follow-up cleanup.

hunt list (re-checked against the repaired tree; round-1 conclusions hold, the
  repair only touched parse_number):
  - Signedness: clean. findIdentifier i32 low/high reproduces int4; binary-search
    arithmetic and `low <= high` (do/while) termination match.
  - Integer widths: clean. parse_number now accumulates in u64 (= uintb) with
    checked arithmetic; `(uint4)` casts reproduced as `as u32`. No truncation
    regressions.
  - Wrapping: clean. parse_number uses checked_mul/checked_add (overflow => fail,
    matching the stream failbit) — NOT wrapping, which is correct here (the C++
    stream does not wrap on overflow, it fails). `*$3 * 8` is `wmul(8)` elsewhere
    (unchanged). A wrapping_* here would be the wrong choice; the port chose
    checked, matching the oracle.
  - Comparator totality: clean (BTreeMap<Vec<u8>> = unsigned-byte lex; unchanged).
  - Iteration-order provenance: clean (only the SymbolTree BTreeMap; no
    HashMap/HashSet; unchanged by the repair).
  - Off-by-one / do-while: clean. findIdentifier do/while -> loop+break verified
    (pcodeparse.y:282-291 vs rust:504-515); parse_number's prefix loop terminates
    on first invalid digit / end of slice, no off-by-one.
  - Erase-while-iterating: clean (clear() retain; unchanged).
  - Exception -> Result partial-state: clean. The BADINTEGER-does-not-YYERROR
    quirk is intact (integervarnode/jumpdest BADINTEGER records the overflow error
    and continues; parse returns true). The repair routes empty-`0x` and overflow
    to the same BADINTEGER path, so the partial-state semantics are unchanged.

mechanical pass:
  - grep (src/pcodeparse.rs): no todo!/unimplemented!/HashMap/HashSet/
    sort_unstable. Bare `as` casts: F2 (correct, uncommented).
  - cargo clippy -p kuna-sleigh --all-targets -- -D warnings: pcodeparse.rs is
    CLEAN (no finding references it). 3 clippy errors exist in the crate but are
    in translate.rs:1202/1437 and slghsymbol.rs:3964 — files NOT in this item's
    diff (other ported items in the shared crate). Out of scope for
    w2-sleigh-pcodeparse; flagged for those items' verifiers, not a blocker here.

adversarial tests (round 2, committed on port branch 5011cc4;
  rust/crates/kuna-sleigh/tests/verify_w2_sleigh_pcodeparse.rs):
  - verify_w2pp_r2_multi_leading_zero_then_bad_octal (008/0008/09 -> 0)  PASS
  - verify_w2pp_r2_octal_multidigit_prefix (0178 -> 15, 0789012 -> 7)    PASS
  - verify_w2pp_r2_empty_hex_and_overflow_boundary (0x => BADINTEGER;
    octal/hex u64-max == 2^64-1; 2^64 hex => BADINTEGER)                 PASS
  - verify_w2pp_r2_capital_x_is_not_hex_trigger (0X1A => syntax error;
    0x1A => 0x1A)                                                        PASS
  All 9 pcodeparse verify tests PASS.

losses: none new. The bison->recursive-descent replacement is already recorded
  as LOSS-006 (covers pcodeparse). F1 was a faithful-port bug, now fixed — not a
  divergence. The capital-`0X` behavior matches the C++ lexer exactly, so it is
  not a loss.
