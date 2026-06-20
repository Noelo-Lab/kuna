//! Verifier adversarial tests for `w2-sleigh-pcodeparse` (round 1).
//!
//! These target the fragile spots the hunt list flagged for this item:
//!   * number-literal parsing vs the C++ `istringstream >> uintb`
//!     (octal partial-parse — the C++ stream reads a valid octal prefix and
//!     STOPS at the first non-octal digit, succeeding; `from_str_radix`
//!     requires every digit valid),
//!   * the `%nonassoc` comparison level (chained comparisons must be rejected,
//!     a comparison followed by a different-precedence operator accepted),
//!   * left-associativity of the additive level,
//!   * the leading-`0` / `0x`-empty number boundaries.
//!
//! The fixture mirrors the in-crate `TestLang` using only the public API.

use std::rc::Rc;

use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_num::opcodes::OpCode;
use kuna_sleigh::pcodeparse::{PcodeSnippet, SnippetLanguage, SnippetSymbol};
use kuna_sleigh::semantics::{ConstType, OpTpl, VarnodeTpl};
use kuna_sleigh::slghsymbol::{SleighSymbol, SymbolKind, VarnodeSymbol};

struct TestLang {
    manager: AddrSpaceManager,
    ram: Rc<AddrSpace>,
    regs: Vec<(Vec<u8>, u64, u32)>,
}

impl TestLang {
    fn new(reg_names: &[(&str, u64, u32)]) -> TestLang {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        let ram = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            true,
            4,
            1,
            1,
            addrspace_flags::hasphysical,
            1,
            1,
        ));
        manager.insert_space(Rc::clone(&ram)).unwrap();
        manager
            .insert_space(Rc::new(UniqueSpace::new(2, 0, true)))
            .unwrap();
        let regs = reg_names
            .iter()
            .map(|(n, o, s)| (n.as_bytes().to_vec(), *o, *s))
            .collect();
        TestLang {
            manager,
            ram,
            regs,
        }
    }
}

fn varnode_sym(space: Rc<AddrSpace>, offset: u64, size: i32) -> VarnodeSymbol {
    let sym = SleighSymbol::new_varnode(b"r", space, offset, size);
    match sym.kind() {
        SymbolKind::Varnode(v) => v.clone(),
        _ => unreachable!(),
    }
}

impl SnippetLanguage for TestLang {
    fn find_snippet_symbol(&self, name: &[u8]) -> Option<SnippetSymbol> {
        if let Some((_, off, size)) = self.regs.iter().find(|(n, _, _)| n == name) {
            let vsym = varnode_sym(Rc::clone(&self.ram), *off, *size as i32);
            return Some(SnippetSymbol::Varnode(vsym, name.to_vec()));
        }
        None
    }
    fn get_default_code_space(&self) -> Rc<AddrSpace> {
        Rc::clone(&self.ram)
    }
    fn get_constant_space(&self) -> Rc<AddrSpace> {
        Rc::clone(self.manager.get_constant_space().unwrap())
    }
    fn get_unique_space(&self) -> Rc<AddrSpace> {
        Rc::clone(self.manager.get_space_by_name("unique").unwrap())
    }
    fn num_spaces(&self) -> i32 {
        self.manager.num_spaces()
    }
    fn get_space(&self, i: i32) -> Option<Rc<AddrSpace>> {
        self.manager.get_space(i).cloned()
    }
}

fn lang() -> TestLang {
    TestLang::new(&[("r1", 0x10, 4), ("r2", 0x14, 4), ("r0", 0x60, 4)])
}

/// Compile, returning (ok, first_error, ops). Tolerant of propagateSize
/// failure (a bare temp may have no concrete size in isolation) — the op
/// vector is still produced from the result template.
fn compile(lang: &dyn SnippetLanguage, body: &str) -> (bool, String, Vec<OpTpl>) {
    let mut snip = PcodeSnippet::new(lang);
    let ok = snip.parse_stream(body.as_bytes());
    let err = snip.get_error_message().to_string();
    let ops = snip
        .release_result()
        .map(|ct| ct.get_opvec().to_vec())
        .unwrap_or_default();
    (ok, err, ops)
}

/// The single const operand of a single-op COPY-style statement.
fn first_in_const(ops: &[OpTpl], idx: i32) -> (ConstType, u64) {
    let op = &ops[0];
    let vn: &VarnodeTpl = op.get_in(idx);
    let off = vn.get_offset();
    (off.get_type(), off.get_real())
}

// ---------------------------------------------------------------------------
// 1. Number-literal parsing vs C++ `istringstream >> uintb`.
//
//    The C++ lexer puts a leading-`0` token containing `8`/`9` into the
//    `decstring` state (isDec accepts 0-9), then parses it with the octal
//    base (unsetf + leading 0). The C++ stream reads the valid octal prefix
//    and STOPS at the first non-octal digit, succeeding with a partial value:
//        "08"   -> 0   (reads "0", stops at "8")
//        "0789" -> 7   (reads "07", stops at "8")
//    The Rust port's `from_str_radix(_, 8)` requires EVERY digit valid and
//    fails the whole token => BADINTEGER (value 0 AND a spurious
//    "Parsed integer is too big (overflow)" error). This asserts the C++
//    oracle; it FAILS against the current port, documenting the divergence.
// ---------------------------------------------------------------------------

#[test]
fn verify_w2pp_octal_partial_parse_zero_eight() {
    // C++ oracle: `08` is INTEGER 0, no error, statement is a clean COPY.
    let (ok, err, ops) = compile(&lang(), "r1 = 08;");
    assert!(ok, "C++ accepts `08` as INTEGER(0); got error {err:?}");
    assert_eq!(err, "", "C++ reports no error for `08`");
    assert_eq!(ops.len(), 1);
    let (ty, val) = first_in_const(&ops, 0);
    assert_eq!(ty, ConstType::Real);
    assert_eq!(val, 0, "C++ octal `08` -> 0 (stream stops at '8')");
}

#[test]
fn verify_w2pp_octal_partial_parse_seven() {
    // C++ oracle: `0789` -> octal reads `07` = 7, stops at `8`. Value 7.
    let (ok, err, ops) = compile(&lang(), "r1 = 0789;");
    assert!(ok, "C++ accepts `0789` as INTEGER(7); got error {err:?}");
    assert_eq!(err, "");
    assert_eq!(ops.len(), 1);
    let (ty, val) = first_in_const(&ops, 0);
    assert_eq!(ty, ConstType::Real);
    assert_eq!(val, 7, "C++ octal `0789` -> 7 (stream stops at '8')");
}

// ---------------------------------------------------------------------------
// 2. Number boundaries that SHOULD match the C++ oracle (regression guard for
//    the same parse_number path): lone `0`, `00`, valid octal `010`, the
//    empty-hex `0x` failure, and a real overflow.
// ---------------------------------------------------------------------------

#[test]
fn verify_w2pp_number_boundaries_match_oracle() {
    // lone `0` -> 0 (C++ octal base, value 0)
    let (ok, _e, ops) = compile(&lang(), "r1 = 0;");
    assert!(ok);
    assert_eq!(first_in_const(&ops, 0), (ConstType::Real, 0));

    // `00` -> 0
    let (ok, _e, ops) = compile(&lang(), "r1 = 00;");
    assert!(ok);
    assert_eq!(first_in_const(&ops, 0), (ConstType::Real, 0));

    // valid octal `010` -> 8
    let (ok, _e, ops) = compile(&lang(), "r1 = 010;");
    assert!(ok);
    assert_eq!(first_in_const(&ops, 0), (ConstType::Real, 8));

    // hex `0xfe` -> 0xfe
    let (ok, _e, ops) = compile(&lang(), "r1 = 0xfe;");
    assert!(ok);
    assert_eq!(first_in_const(&ops, 0), (ConstType::Real, 0xfe));

    // real overflow -> BADINTEGER; integervarnode BADINTEGER yields const 0
    // and records the overflow error but the parse continues (jumpdest/
    // integervarnode BADINTEGER does NOT YYERROR). So `ok` is true here, but
    // the overflow error string is recorded.
    let (_ok, err, _ops) = compile(&lang(), "r1 = 0xffffffffffffffffff;");
    assert_eq!(err, "Parsed integer is too big (overflow)");
}

// ---------------------------------------------------------------------------
// 3. `%nonassoc` comparison level: `a < b s< c` chains two PREC_CMP operators
//    and must be a syntax error (bison rejects chained nonassoc). But a
//    comparison adjacent to a *different*-precedence operator is fine.
// ---------------------------------------------------------------------------

#[test]
fn verify_w2pp_nonassoc_chain_rejected_mixed_accepted() {
    // chained nonassoc comparisons: rejected.
    let (ok, err, _ops) = compile(&lang(), "r1 = r2 < r0 s< r1;");
    assert!(!ok, "chained nonassoc `<` then `s<` must be a syntax error");
    assert_eq!(err, "Syntax error");

    // `a < b == c`: `==` (PREC_EQ) is looser than `<` (PREC_CMP); accepted as
    // `(a < b) == c`. Not a nonassoc chain.
    let (ok, err, ops) = compile(&lang(), "r1 = r2 < r0 == r1;");
    assert!(ok, "`< ... == ...` is not a nonassoc chain; got {err:?}");
    // two ops: INT_LESS then INT_EQUAL.
    assert_eq!(ops.len(), 2);
    assert_eq!(ops[0].get_opcode(), OpCode::CPUI_INT_LESS);
    assert_eq!(ops[1].get_opcode(), OpCode::CPUI_INT_EQUAL);
}

// ---------------------------------------------------------------------------
// 4. Left-associativity of the additive level: `a - b - c` => `(a - b) - c`,
//    i.e. two ops, the second SUB consuming the first SUB's temp result.
//    A right-associative parse would build `a - (b - c)` (different op order).
// ---------------------------------------------------------------------------

#[test]
fn verify_w2pp_additive_left_associative() {
    let (ok, err, ops) = compile(&lang(), "r1 = r2 - r0 - r1;");
    assert!(ok, "{err:?}");
    assert_eq!(ops.len(), 2, "two SUB ops for a left-assoc chain");
    assert_eq!(ops[0].get_opcode(), OpCode::CPUI_INT_SUB);
    assert_eq!(ops[1].get_opcode(), OpCode::CPUI_INT_SUB);
    // Left-assoc: op0 computes (r2 - r0); op1 subtracts r1 from op0's output.
    // op0's two inputs are the registers r2 (0x14) and r0 (0x60); op1's
    // second input is r1 (0x10) and its first input is op0's temp (unique).
    let op0_in0 = ops[0].get_in(0).get_offset().get_real();
    let op0_in1 = ops[0].get_in(1).get_offset().get_real();
    assert_eq!((op0_in0, op0_in1), (0x14, 0x60), "op0 = r2 - r0");
    // op1's last input is r1 (the outer right operand), confirming left nesting.
    let op1_in1 = ops[1].get_in(1);
    assert_eq!(op1_in1.get_offset().get_real(), 0x10, "op1 right operand = r1");
}

// ===========================================================================
// ROUND 2 (verifier) — regression guard for the F1 repair (parse_number
// partial-parse). Every expected value below was taken from a standalone
// `istringstream >> uintb` + `unsetf(dec|hex|oct)` oracle program, the exact
// C++ construct in pcodeparse.y's getNextToken trailer.
// ===========================================================================

// Multiple leading zeros then a non-octal digit: the stream consumes the run
// of valid octal `0`s and stops at the bad digit, succeeding with value 0.
//   oracle: "008" -> 0, "0008" -> 0, "09" -> 0
#[test]
fn verify_w2pp_r2_multi_leading_zero_then_bad_octal() {
    for (src, want) in [("r1 = 008;", 0u64), ("r1 = 0008;", 0u64), ("r1 = 09;", 0u64)] {
        let (ok, err, ops) = compile(&lang(), src);
        assert!(ok, "C++ accepts {src:?} as a partial octal INTEGER; got {err:?}");
        assert_eq!(err, "", "{src:?} must produce no error (stream does not fail)");
        assert_eq!(ops.len(), 1, "{src:?} -> single COPY");
        assert_eq!(
            first_in_const(&ops, 0),
            (ConstType::Real, want),
            "{src:?} octal partial-parse value"
        );
    }
}

// Octal partial-parse that consumes more than one valid digit before the bad
// one. These ARE reachable as single decstring tokens (isDec admits 8/9).
//   oracle: "0178" -> 15, "0789012" -> 7
#[test]
fn verify_w2pp_r2_octal_multidigit_prefix() {
    let cases = [
        ("r1 = 0178;", 15u64),    // 0,1,7 valid octal, stop at 8 => 017 == 15
        ("r1 = 0789012;", 7u64),  // 0,7 valid, stop at 8 => 07 == 7
    ];
    for (src, want) in cases {
        let (ok, err, ops) = compile(&lang(), src);
        assert!(ok, "C++ accepts {src:?}; got {err:?}");
        assert_eq!(err, "", "{src:?} must not record an error");
        assert_eq!(ops.len(), 1);
        assert_eq!(
            first_in_const(&ops, 0),
            (ConstType::Real, want),
            "{src:?} value vs istringstream oracle"
        );
    }
}

// LEXER guard for the hex trigger: the C++ moveState `case '0'` enters the
// hexstring state ONLY on a lowercase `x` (`lookahead1 == 'x'`); a capital
// `X` is NOT a hex trigger and is NOT a decimal digit, so `0X1A` lexes as the
// decstring token `0` followed by the identifier `X1A`. With `X1A` unbound,
// the statement is a syntax error — i.e. capital-`0X` is NOT a hex literal.
// (This pins the lexer trigger so a future "fix" that accepts `0X` like the
// raw istringstream would be caught as a DIVERGENCE from the C++ lexer.)
#[test]
fn verify_w2pp_r2_capital_x_is_not_hex_trigger() {
    let (ok, err, _ops) = compile(&lang(), "r1 = 0X1A;");
    assert!(
        !ok,
        "C++ lexer does not treat capital `0X` as hex; `0 X1A` is a syntax error"
    );
    assert_eq!(err, "Syntax error");

    // Sanity: the lowercase form IS the hex literal and parses to 0x1A.
    let (ok, err, ops) = compile(&lang(), "r1 = 0x1A;");
    assert!(ok, "lowercase 0x1A is a hex literal; got {err:?}");
    assert_eq!(first_in_const(&ops, 0), (ConstType::Real, 0x1A));
}

// Boundary: the empty-`0x` token fails (BADINTEGER => const 0 + recorded
// error), and the u64-max octal/hex literals parse exactly at the boundary
// while one-past-max overflows to BADINTEGER.
//   oracle: "0x" -> BADINTEGER; "01777777777777777777777" -> 2^64-1;
//           "0xffffffffffffffff" -> 2^64-1; "0x10000000000000000" -> BADINTEGER
#[test]
fn verify_w2pp_r2_empty_hex_and_overflow_boundary() {
    // empty `0x`: lexer emits the `0x` hexstring token with zero hex digits;
    // istringstream sets failbit => BADINTEGER. integervarnode BADINTEGER does
    // NOT YYERROR, so the parse continues (ok) but const is 0 and the error is
    // recorded.
    let (_ok, err, ops) = compile(&lang(), "r1 = 0x;");
    assert_eq!(
        err, "Parsed integer is too big (overflow)",
        "empty `0x` is BADINTEGER, mirrored to the overflow error path"
    );
    assert_eq!(first_in_const(&ops, 0), (ConstType::Real, 0));

    // octal at exactly u64 max: 0o1777777777777777777777 == 0xFFFFFFFFFFFFFFFF.
    let (ok, err, ops) = compile(&lang(), "r1 = 01777777777777777777777;");
    assert!(ok, "octal u64-max literal must parse; got {err:?}");
    assert_eq!(err, "");
    assert_eq!(
        first_in_const(&ops, 0),
        (ConstType::Real, u64::MAX),
        "octal max == 2^64-1"
    );

    // hex at exactly u64 max.
    let (ok, _e, ops) = compile(&lang(), "r1 = 0xffffffffffffffff;");
    assert!(ok);
    assert_eq!(first_in_const(&ops, 0), (ConstType::Real, u64::MAX));

    // one past u64 max in hex: checked_mul overflow => None => BADINTEGER.
    let (_ok, err, _ops) = compile(&lang(), "r1 = 0x10000000000000000;");
    assert_eq!(
        err, "Parsed integer is too big (overflow)",
        "hex 2^64 overflows the u64 accumulator => BADINTEGER"
    );
}
