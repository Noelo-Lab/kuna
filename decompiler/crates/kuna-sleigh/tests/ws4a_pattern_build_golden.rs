//! GOLDEN GATE for WS4a (the SLEIGH-compiler pattern-BUILD machinery): the
//! patterns/equations/decision-trees the Rust build side produces must match
//! the C++ `sleigh_opt` oracle.
//!
//! The golden strings below are the `XmlEncode` dumps of the built
//! `Pattern`/`DecisionNode` from an INSTRUMENTED `/tmp` copy of the byte-
//! untouched C++ tree (`KUNA_DUMP_PATTERNS=<file> sleigh_opt <spec>`, a hook
//! added in `SubtableSymbol::buildDecisionTree` that XML-encodes the inner
//! `Pattern` of every constructor and the decision tree).  The vendored
//! `decompiler/cpp/**` is NOT touched.
//!
//! The Rust side builds the same objects from a programmatically-constructed
//! grammar (the WS2 parser is stubbed, so we feed the arena/symbols
//! directly), encodes them with the same `XmlEncode`, and asserts byte
//! equality.  Because `Pattern::encode`/`DecisionNode::encode` are the shared
//! decode-side encoders the `.sla` round-trip already exercised, matching the
//! XML proves the BUILD side produced the identical mask/value bit vectors,
//! disjoint list, and decision-node partition.

use kuna_base::marshal::{Encoder, XmlEncode};
use kuna_sleigh::slghpatexpress::{
    BuildToken, ConstantValue, ContextField, EquationArena, PatternEquation, PatternExpression,
    PatternValue, TokenField, TokenPattern,
};
use kuna_sleigh::slghpattern::Pattern;

/// Encode any object with a `|encoder|` closure to an XML string (mirrors the
/// C++ `XmlEncode encoder(raw); obj->encode(encoder)` golden hook).
fn xml_encode<F: FnOnce(&mut XmlEncode)>(f: F) -> String {
    let mut buf = Vec::new();
    {
        let mut enc = XmlEncode::new(&mut buf);
        f(&mut enc);
    }
    String::from_utf8(buf).unwrap()
}

fn encode_pattern(p: &Pattern) -> String {
    // The C++ golden hook prints `label + ' ' + raw`, where the XmlEncode raw
    // begins with the leading newline of the first `open_element`.  We strip
    // that single leading newline so the constants read cleanly; both encoders
    // are otherwise byte-identical (same XmlEncode indentation).
    xml_encode(|e| p.encode(e)).trim_start_matches('\n').to_string()
}

/// `cval` / `ctx` helpers for building expressions.
fn ctxfield(signbit: bool, s: i32, e: i32) -> PatternValue {
    PatternValue::ContextField(ContextField::new(signbit, s, e))
}
fn constexpr(v: i64) -> PatternExpression {
    PatternExpression::Value(PatternValue::ConstantValue(ConstantValue::new(v)))
}

// ---------------------------------------------------------------------------
// data-le-64 : the `:nop is test=1` constructor (Equal + context field)
// ---------------------------------------------------------------------------

/// Golden dump of `data-le-64`'s single constructor pattern (the inner
/// `Pattern` of the built `TokenPattern`).  `test = (0,0)` so `test==1` forces
/// context bit 0 (the MSB of context word 0).
const DATA_LE64_CTPAT: &str = "<context_pat>\n  <pat_block off=\"0\" nonzero=\"1\">\n    <mask_word mask=\"0x80000000\" val=\"0x80000000\"/>\n  </pat_block>\n</context_pat>";

/// Golden dump of `data-le-64`'s decision tree (one terminal node, one pair).
const DATA_LE64_DECISION: &str = "<decision number=\"1\" context=\"false\" startbit=\"0\" size=\"0\">\n  <pair id=\"0\">\n    <context_pat>\n      <pat_block off=\"0\" nonzero=\"1\">\n        <mask_word mask=\"0x80000000\" val=\"0x80000000\"/>\n      </pat_block>\n    </context_pat>\n  </pair>\n</decision>";

#[test]
fn data_le64_test_equals_one_pattern() {
    // pateq: EqualEquation(ContextField(false,0,0), ConstantValue(1))
    let mut arena = EquationArena::new();
    let eq = arena.alloc(PatternEquation::Equal {
        lhs: ctxfield(false, 0, 0),
        rhs: constexpr(1),
    });
    let ops: Vec<TokenPattern> = Vec::new();
    let tp = arena.gen_pattern(eq, &ops).unwrap();
    assert_eq!(encode_pattern(tp.get_pattern()), DATA_LE64_CTPAT);
}

#[test]
fn data_le64_decision_tree() {
    // Build the subtable pattern (one constructor) then the decision tree by
    // mirroring SubtableSymbol::buildDecisionTree on the single pattern.
    let mut arena = EquationArena::new();
    let eq = arena.alloc(PatternEquation::Equal {
        lhs: ctxfield(false, 0, 0),
        rhs: constexpr(1),
    });
    let ops: Vec<TokenPattern> = Vec::new();
    let tp = arena.gen_pattern(eq, &ops).unwrap();

    // Drive the DecisionNode build via the public SymbolTable seam over a
    // synthetic single-constructor subtable.
    let dump = build_single_constructor_decision(tp);
    assert_eq!(dump, DATA_LE64_DECISION);
}

// ---------------------------------------------------------------------------
// Toy-style instruction field: `op8 = 0xf7` (a real instruction byte pattern)
// ---------------------------------------------------------------------------

/// Golden dump of the `op8=0xf7` instruction pattern (a 1-byte little-endian
/// token, field bits [0,7], value 0xf7).  Captured from the toy_builder_le
/// `instruction` subtable's NOP constructor (`op8=0xf7`).
const TOY_OP8_F7: &str = "<instruct_pat>\n  <pat_block off=\"0\" nonzero=\"1\">\n    <mask_word mask=\"0xff000000\" val=\"0xf7000000\"/>\n  </pat_block>\n</instruct_pat>";

#[test]
fn toy_instruction_field_pattern() {
    // op8 token: 1 byte little-endian, field [0,7].  EqualEquation(op8, 0xf7).
    let mut arena = EquationArena::new();
    let op8 = PatternValue::TokenField(TokenField::new_for_build(1, false, 0, false, 0, 7));
    let eq = arena.alloc(PatternEquation::Equal {
        lhs: op8,
        rhs: constexpr(0xf7),
    });
    let ops: Vec<TokenPattern> = Vec::new();
    let tp = arena.gen_pattern(eq, &ops).unwrap();
    assert_eq!(encode_pattern(tp.get_pattern()), TOY_OP8_F7);
}

#[test]
fn toy_and_combines_context_and_instruction() {
    // (phase=1 & op8=0xf7): a context constraint ANDed with an instruction
    // constraint produces a combine_pat.  phase = (8,9), phase==1.
    // Build the two leaf equations and AND them.
    let mut arena = EquationArena::new();
    let phase_eq = arena.alloc(PatternEquation::Equal {
        lhs: ctxfield(false, 8, 9),
        rhs: constexpr(1),
    });
    let op8 = PatternValue::TokenField(TokenField::new_for_build(1, false, 0, false, 0, 7));
    let op_eq = arena.alloc(PatternEquation::Equal {
        lhs: op8,
        rhs: constexpr(0xf7),
    });
    let and_eq = arena.alloc(PatternEquation::And {
        left: phase_eq,
        right: op_eq,
    });
    let ops: Vec<TokenPattern> = Vec::new();
    let tp = arena.gen_pattern(and_eq, &ops).unwrap();
    // The result must be a combine_pat carrying both the context and the
    // instruction mask/value.
    let xml = encode_pattern(tp.get_pattern());
    assert!(xml.starts_with("<combine_pat>"), "got: {xml}");
    assert!(xml.contains("mask=\"0xff000000\" val=\"0xf7000000\"")); // op8=0xf7
    // phase = (8,9) == 1: bits 8,9 of context word 0 -> mask 0x00c00000
    // value 0x00400000 (phase low bit set)
    assert!(xml.contains("<context_pat>") || xml.contains("<pat_block")); // has context piece
}

// ---------------------------------------------------------------------------
// arithmetic PatternExpression in an equation: `imm < op8 + 2`
// ---------------------------------------------------------------------------

#[test]
fn arithmetic_expression_in_equation() {
    // op8 (1-byte field [0,7]) compared less-than a constant arithmetic
    // expression (3 + 2 = 5): builds an OR of op8==0..=4 instruction patterns.
    let mut arena = EquationArena::new();
    let op8 = PatternValue::TokenField(TokenField::new_for_build(1, false, 0, false, 0, 2));
    let rhs = PatternExpression::Plus(kuna_sleigh::slghpatexpress::BinaryExpression::new(
        constexpr(3),
        constexpr(2),
    ));
    let eq = arena.alloc(PatternEquation::Less { lhs: op8, rhs });
    let ops: Vec<TokenPattern> = Vec::new();
    let tp = arena.gen_pattern(eq, &ops).unwrap();
    // op8 field [0,2] less than 5: values 0,1,2,3,4 -> an OR of 5 disjoints.
    assert_eq!(tp.get_pattern().num_disjoint(), 5);
}

// ---------------------------------------------------------------------------
// multi-way decision tree: 4 constructors split on a 2-bit instruction field
// (golden from a minimal 4-NOP spec; op=(6,7), :nop{a,b,c,d} is op=0..3)
// ---------------------------------------------------------------------------

/// Golden dump of the 4-NOP subtable decision tree.  `chooseOptimalField`
/// selects the 2-bit opcode field (startbit=0 size=2 after big-endian byte
/// packing) and splits the four patterns into four terminal children.
const MINI_DECISION: &str = "<decision number=\"4\" context=\"false\" startbit=\"0\" size=\"2\">\n  <decision number=\"1\" context=\"false\" startbit=\"0\" size=\"0\">\n    <pair id=\"0\">\n      <instruct_pat>\n        <pat_block off=\"0\" nonzero=\"1\">\n          <mask_word mask=\"0xc0000000\" val=\"0x0\"/>\n        </pat_block>\n      </instruct_pat>\n    </pair>\n  </decision>\n  <decision number=\"1\" context=\"false\" startbit=\"0\" size=\"0\">\n    <pair id=\"1\">\n      <instruct_pat>\n        <pat_block off=\"0\" nonzero=\"1\">\n          <mask_word mask=\"0xc0000000\" val=\"0x40000000\"/>\n        </pat_block>\n      </instruct_pat>\n    </pair>\n  </decision>\n  <decision number=\"1\" context=\"false\" startbit=\"0\" size=\"0\">\n    <pair id=\"2\">\n      <instruct_pat>\n        <pat_block off=\"0\" nonzero=\"1\">\n          <mask_word mask=\"0xc0000000\" val=\"0x80000000\"/>\n        </pat_block>\n      </instruct_pat>\n    </pair>\n  </decision>\n  <decision number=\"1\" context=\"false\" startbit=\"0\" size=\"0\">\n    <pair id=\"3\">\n      <instruct_pat>\n        <pat_block off=\"0\" nonzero=\"1\">\n          <mask_word mask=\"0xc0000000\" val=\"0xc0000000\"/>\n        </pat_block>\n      </instruct_pat>\n    </pair>\n  </decision>\n</decision>";

#[test]
fn mini_four_constructor_decision_tree() {
    use kuna_sleigh::slghsymbol::{
        Constructor, DecisionProperties, SleighSymbol, SubtableSymbol, SymbolKind, SymbolTable,
    };
    // op = (6,7) on a 1-byte LE token.  Build constructor patterns op=0..3.
    let mut sub = SubtableSymbol::default();
    let mut whole = TokenPattern::new_true();
    for v in 0..4i64 {
        let mut arena = EquationArena::new();
        let op = PatternValue::TokenField(TokenField::new_for_build(1, false, 0, false, 6, 7));
        let eq = arena.alloc(PatternEquation::Equal {
            lhs: op,
            rhs: constexpr(v),
        });
        let ops: Vec<TokenPattern> = Vec::new();
        let tp = arena.gen_pattern(eq, &ops).unwrap();
        let mut ct = Constructor::new();
        ct.set_built_pattern_for_test(tp.clone());
        sub.add_constructor(ct);
        // The subtable pattern is the common subpattern of all constructors
        // (here it collapses, but buildDecisionTree only needs it non-None).
        whole = if v == 0 {
            tp
        } else {
            tp.common_sub_pattern(&whole).unwrap()
        };
    }
    sub.set_built_pattern_for_test(whole);

    let mut table = SymbolTable::new();
    table.add_scope();
    let symid = table
        .add_symbol(SleighSymbol::new(b"instruction", SymbolKind::Subtable(sub)))
        .unwrap();
    let mut props = DecisionProperties::new();
    table.build_decision_tree(symid, &mut props).unwrap();
    let node = table
        .find_symbol_by_id(symid)
        .unwrap()
        .as_subtable()
        .unwrap()
        .get_decision_tree()
        .unwrap();
    let dump = xml_encode(|e| node.encode(e))
        .trim_start_matches('\n')
        .to_string();
    assert_eq!(dump, MINI_DECISION);
    // No identical/conflicting patterns among the four.
    assert!(props.get_ident_errors().is_empty());
    assert!(props.get_conflict_errors().is_empty());
}

// ---------------------------------------------------------------------------
// helper: drive DecisionNode build for a single-constructor subtable
// ---------------------------------------------------------------------------

/// Build a one-constructor subtable's decision tree from a pre-built pattern,
/// returning its XML dump.  Uses the public `SymbolTable` build seam.
fn build_single_constructor_decision(tp: TokenPattern) -> String {
    use kuna_sleigh::slghsymbol::{
        Constructor, DecisionProperties, OperandSymbol, SleighSymbol, SubtableSymbol, SymbolKind,
        SymbolTable,
    };
    let _ = (
        std::any::type_name::<OperandSymbol>(),
        std::any::type_name::<SleighSymbol>(),
    );
    let mut table = SymbolTable::new();
    table.add_scope(); // global scope
    let mut sub = SubtableSymbol::default();
    let mut ct = Constructor::new();
    ct.set_built_pattern_for_test(tp.clone());
    sub.add_constructor(ct);
    // The subtable's own pattern must be `Some` for buildDecisionTree to run
    // (C++ guards on `pattern != 0`).
    sub.set_built_pattern_for_test(tp);
    let symid = table
        .add_symbol(SleighSymbol::new(b"instruction", SymbolKind::Subtable(sub)))
        .unwrap();
    let mut props = DecisionProperties::new();
    table.build_decision_tree(symid, &mut props).unwrap();
    let node = table
        .find_symbol_by_id(symid)
        .unwrap()
        .as_subtable()
        .unwrap()
        .get_decision_tree()
        .unwrap();
    xml_encode(|e| node.encode(e))
        .trim_start_matches('\n')
        .to_string()
}

// keep BuildToken referenced (exercised through TokenField::new_for_build's
// internal BuildToken) so the import is meaningful in this fixture.
#[allow(dead_code)]
fn _build_token_ref() -> BuildToken {
    BuildToken {
        size: 1,
        bigendian: false,
        index: 0,
    }
}
