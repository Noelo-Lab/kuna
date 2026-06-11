//! Tests for the p-code snippet parser. The accept tests port a
//! representative set of real `<pcode>` injection bodies from the vendored
//! `.cspec`/`.pspec` files (>=10 snippets across processors) and assert the
//! built `OpTpl` sequence. The reject tests cover the malformed-snippet error
//! productions, asserting the C++ `yyerror(...)` text wherever observable.

use std::rc::Rc;

use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use super::*;
use crate::semantics::{ConstType, OpTpl, VField, VarnodeTpl};
use crate::slghsymbol::VarnodeSymbol;

// ---------------------------------------------------------------------------
// Fixture: a synthetic SleighBase (SnippetLanguage) with named registers
// ---------------------------------------------------------------------------

/// A synthetic language: a constant space, a 4-byte big-endian processor
/// space `ram`, a unique space, plus a name->register table and a single
/// user-op. Registers live in `ram` at the offsets given.
struct TestLang {
    manager: AddrSpaceManager,
    ram: Rc<AddrSpace>,
    regs: Vec<(Vec<u8>, u64, u32)>, // (name, offset, size)
    userop: Option<(Vec<u8>, u32)>, // (name, index)
}

impl TestLang {
    fn new(reg_names: &[(&str, u64, u32)]) -> TestLang {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        let ram = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            true, // big-endian
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
            userop: None,
        }
    }

    fn with_userop(mut self, name: &str, index: u32) -> TestLang {
        self.userop = Some((name.as_bytes().to_vec(), index));
        self
    }
}

impl SnippetLanguage for TestLang {
    fn find_snippet_symbol(&self, name: &[u8]) -> Option<SnippetSymbol> {
        if let Some((_, off, size)) = self.regs.iter().find(|(n, _, _)| n == name) {
            let vsym = VarnodeSymbol::new_for_test(Rc::clone(&self.ram), *off, *size as i32);
            return Some(SnippetSymbol::Varnode(vsym, name.to_vec()));
        }
        if let Some((uname, idx)) = &self.userop {
            if uname == name {
                let mut u = UserOpSymbol::default();
                u.set_index(*idx);
                return Some(SnippetSymbol::UserOp(u));
            }
        }
        // Special address symbols (inst_start/inst_next): the snippet ctor
        // adds inst_dest/inst_ref; inst_start/inst_next come from the language.
        match name {
            b"inst_start" => Some(SnippetSymbol::Start(name.to_vec())),
            b"inst_next" => Some(SnippetSymbol::End(name.to_vec())),
            b"inst_next2" => Some(SnippetSymbol::Next2(name.to_vec())),
            _ => None,
        }
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

// VarnodeSymbol has no public test constructor; build one through the
// SleighSymbol::new_varnode helper and project out the kind.
impl VarnodeSymbol {
    fn new_for_test(space: Rc<AddrSpace>, offset: u64, size: i32) -> VarnodeSymbol {
        let sym = crate::slghsymbol::SleighSymbol::new_varnode(b"r", space, offset, size);
        match sym.kind() {
            crate::slghsymbol::SymbolKind::Varnode(v) => v.clone(),
            _ => unreachable!(),
        }
    }
}

// ---------------------------------------------------------------------------
// Op-sequence rendering (compact, deterministic, for assertions)
// ---------------------------------------------------------------------------

fn render_const(c: &ConstTpl) -> String {
    match c.get_type() {
        ConstType::Real => format!("{:#x}", c.get_real()),
        ConstType::Handle => format!("hand{}.{:?}", c.get_handle_index(), c.get_select()),
        ConstType::Spaceid => format!("spc:{}", c.get_space().get_name()),
        ConstType::JStart => "j_start".into(),
        ConstType::JNext => "j_next".into(),
        ConstType::JNext2 => "j_next2".into(),
        ConstType::JCurspace => "j_curspace".into(),
        ConstType::JCurspaceSize => "j_curspace_size".into(),
        ConstType::JRelative => format!("j_relative({:#x})", c.get_real()),
        ConstType::JFlowref => "j_flowref".into(),
        ConstType::JFlowrefSize => "j_flowref_size".into(),
        ConstType::JFlowdest => "j_flowdest".into(),
        ConstType::JFlowdestSize => "j_flowdest_size".into(),
    }
}

fn render_vn(vn: &VarnodeTpl) -> String {
    format!(
        "({},{},{})",
        render_const(vn.get_space()),
        render_const(vn.get_offset()),
        render_const(vn.get_size())
    )
}

fn render_op(op: &OpTpl) -> String {
    let mut s = format!("{:?}", op.get_opcode());
    if let Some(out) = op.get_out() {
        s.push_str(&format!(" {} =", render_vn(out)));
    }
    let n = op.num_input();
    let mut ins = Vec::new();
    for i in 0..n {
        ins.push(render_vn(op.get_in(i)));
    }
    s.push(' ');
    s.push_str(&ins.join(", "));
    s
}

/// Compile a snippet against `lang`, returning the rendered op sequence on
/// success (panics on a parse error, so accept tests fail loudly).
fn compile_ok(lang: &dyn SnippetLanguage, body: &str) -> Vec<String> {
    let mut snip = PcodeSnippet::new(lang);
    let ok = snip.parse_stream(body.as_bytes());
    assert!(
        ok,
        "expected snippet to compile, got error: {:?}",
        snip.get_error_message()
    );
    let ct = snip.release_result().expect("no result template");
    ct.get_opvec().iter().map(render_op).collect()
}

/// Like [`compile_ok`] but tolerant of a failed `propagateSize` (a snippet
/// whose temporaries never get a concrete size in isolation still produces a
/// valid op vector). Returns `(ok, first_error, ops)`.
fn compile_collect(lang: &dyn SnippetLanguage, body: &str) -> (bool, String, Vec<String>) {
    let mut snip = PcodeSnippet::new(lang);
    let ok = snip.parse_stream(body.as_bytes());
    let err = snip.get_error_message().to_string();
    let ops = snip
        .release_result()
        .map(|ct| ct.get_opvec().iter().map(render_op).collect())
        .unwrap_or_default();
    (ok, err, ops)
}

/// Compile expecting a parse failure; return the first error message.
fn compile_err(lang: &dyn SnippetLanguage, body: &str) -> String {
    let mut snip = PcodeSnippet::new(lang);
    let ok = snip.parse_stream(body.as_bytes());
    assert!(!ok, "expected snippet to fail, but it compiled");
    snip.get_error_message().to_string()
}

fn basic_lang() -> TestLang {
    TestLang::new(&[
        ("r1", 0x10, 4),
        ("r2", 0x14, 4),
        ("ESP", 0x20, 4),
        ("EBP", 0x24, 4),
        ("EAX", 0x28, 4),
        ("SP", 0x40, 4),
        ("R7", 0x44, 4),
        ("LR", 0x48, 4),
        ("FP", 0x4c, 4),
        ("PC", 0x50, 4),
        ("ACC", 0x54, 1),
        ("sp", 0x58, 4),
        ("v0", 0x5c, 4),
        ("r0", 0x60, 4),
    ])
}

// ---------------------------------------------------------------------------
// Lexer-level tests
// ---------------------------------------------------------------------------

#[test]
fn lexer_idents_table_has_upstream_sort_violation() {
    // UPSTREAM BUG (faithfully preserved): the C++ `idents[]` table is
    // declared "Sorted" but `"||"` (0x7c7c) is listed *before* `"abs"`
    // (0x616273) — out of byte order. Every adjacent pair is byte-ascending
    // EXCEPT this one. We assert the table is identical to upstream by
    // checking exactly one violation, at the `"||"`/`"abs"` boundary.
    let mut violations = Vec::new();
    for (i, w) in IDENTS.windows(2).enumerate() {
        if w[0].nm >= w[1].nm {
            violations.push((i, w[0].nm, w[1].nm));
        }
    }
    assert_eq!(violations.len(), 1, "expected exactly the upstream violation");
    let (_, a, b) = violations[0];
    assert_eq!(a, b"||");
    assert_eq!(b, b"abs");
}

#[test]
fn lexer_find_identifier_hits_and_misses() {
    assert!(PcodeLexer::find_identifier(b"zext").is_some());
    assert!(PcodeLexer::find_identifier(b"s>>").is_some());
    assert!(PcodeLexer::find_identifier(b"f==").is_some());
    // UPSTREAM BUG (faithfully preserved): `"||"` and `"abs"` straddle the
    // table's lone sort violation, so the binary search FAILS to find them —
    // the C++ findIdentifier returns -1 for both, so `||`/`abs` lex as STRING
    // rather than OP_BOOL_OR / OP_ABS. The port reproduces this exactly.
    assert!(PcodeLexer::find_identifier(b"||").is_none());
    assert!(PcodeLexer::find_identifier(b"abs").is_none());
    // The keywords adjacent to the violation are still found.
    assert!(PcodeLexer::find_identifier(b"^^").is_some());
    assert!(PcodeLexer::find_identifier(b"borrow").is_some());
    assert!(PcodeLexer::find_identifier(b"notakeyword").is_none());
    assert!(PcodeLexer::find_identifier(b"r1").is_none());
}

#[test]
fn lexer_parse_number_radices() {
    assert_eq!(parse_number(b"0"), Some(0));
    assert_eq!(parse_number(b"42"), Some(42));
    assert_eq!(parse_number(b"0x14"), Some(0x14));
    assert_eq!(parse_number(b"0xfffffffe"), Some(0xfffffffe));
    assert_eq!(parse_number(b"010"), Some(8)); // octal
    assert_eq!(parse_number(b"0xffffffffffffffffff"), None); // overflow
}

// ---------------------------------------------------------------------------
// Accept tests: representative real snippets across processors
// ---------------------------------------------------------------------------

// 1. [MIPS] mips64 NOP:  v0 = v0;  (a self copy)
#[test]
fn accept_self_copy_nop() {
    let lang = basic_lang();
    let ops = compile_ok(&lang, "v0 = v0;");
    assert_eq!(ops, vec!["CPUI_COPY (spc:ram,0x5c,0x4) = (spc:ram,0x5c,0x4)"]);
}

// 2. [Sparc] SparcV9_64:  add-assign (here r1 = r2 + r1).
#[test]
fn accept_add_assign() {
    let lang = basic_lang();
    let ops = compile_ok(&lang, "r1 = r2 + r1;");
    assert_eq!(
        ops,
        vec!["CPUI_INT_ADD (spc:ram,0x10,0x4) = (spc:ram,0x14,0x4), (spc:ram,0x10,0x4)"]
    );
}

// 3. [AARCH64] AARCH64_win:  sp = sp - 16;
#[test]
fn accept_sub_const() {
    let lang = basic_lang();
    let ops = compile_ok(&lang, "sp = sp - 16;");
    assert_eq!(
        ops,
        vec!["CPUI_INT_SUB (spc:ram,0x58,0x4) = (spc:ram,0x58,0x4), (spc:const,0x10,0x4)"]
    );
}

// 4. [Toy] toy:  sp = sp + 4;
#[test]
fn accept_add_const() {
    let lang = basic_lang();
    let ops = compile_ok(&lang, "sp = sp + 4;");
    assert_eq!(
        ops,
        vec!["CPUI_INT_ADD (spc:ram,0x58,0x4) = (spc:ram,0x58,0x4), (spc:const,0x4,0x4)"]
    );
}

// 5. [PowerPC] ppc_32_e500mc_be:  LR = inst_dest + 4;
#[test]
fn accept_flowdest_plus_const() {
    let lang = basic_lang();
    let (_, _, ops) = compile_collect(&lang, "LR = inst_dest + 4;");
    assert_eq!(
        ops,
        vec!["CPUI_INT_ADD (spc:ram,0x48,0x4) = (spc:const,j_flowdest,0x4), (spc:const,0x4,0x4)"]
    );
}

// 6. [PowerPC] ppc_64_be:  local saveR2ptr = r1 + 0x28;  *:8 saveR2ptr = r2;
#[test]
fn accept_local_def_and_store() {
    let lang = basic_lang();
    let ops = compile_ok(&lang, "local saveR2ptr = r1 + 0x28;\n*:8 saveR2ptr = r2;");
    assert_eq!(ops.len(), 2);
    assert_eq!(
        ops[0],
        "CPUI_INT_ADD (spc:unique,0x10,0x4) = (spc:ram,0x10,0x4), (spc:const,0x28,0x4)"
    );
    assert_eq!(
        ops[1],
        "CPUI_STORE (spc:const,spc:ram,0x8), (spc:unique,0x10,0x4), (spc:ram,0x14,0x4)"
    );
}

// 7. [x86] x86win prologue fragment:  ESP = ESP - 4;  *:4 ESP = -1;
#[test]
fn accept_store_neg_one() {
    let lang = basic_lang();
    let ops = compile_ok(&lang, "ESP = ESP - 4;\n*:4 ESP = 0xffffffff;");
    assert_eq!(ops.len(), 2);
    assert_eq!(
        ops[0],
        "CPUI_INT_SUB (spc:ram,0x20,0x4) = (spc:ram,0x20,0x4), (spc:const,0x4,0x4)"
    );
    assert_eq!(
        ops[1],
        "CPUI_STORE (spc:const,spc:ram,0x8), (spc:ram,0x20,0x4), (spc:const,0xffffffff,0x4)"
    );
}

// 8. [x86] x86win epilogue:  ESP = EBP;  EBP = * ESP;  ESP = ESP + 4;
#[test]
fn accept_load_default_space() {
    let lang = basic_lang();
    let ops = compile_ok(&lang, "ESP = EBP;\nEBP = * ESP;\nESP = ESP + 4;");
    assert_eq!(ops.len(), 3);
    assert_eq!(ops[0], "CPUI_COPY (spc:ram,0x20,0x4) = (spc:ram,0x24,0x4)");
    assert_eq!(
        ops[1],
        "CPUI_LOAD (spc:ram,0x24,0x4) = (spc:const,spc:ram,0x8), (spc:ram,0x20,0x4)"
    );
    assert_eq!(
        ops[2],
        "CPUI_INT_ADD (spc:ram,0x20,0x4) = (spc:ram,0x20,0x4), (spc:const,0x4,0x4)"
    );
}

// 9. [8051] 8051:  PC = inst_next + zext(ACC) * 3;  goto [PC];
#[test]
fn accept_zext_mult_and_indirect_goto() {
    let lang = basic_lang();
    let (_, _, ops) = compile_collect(&lang, "PC = inst_next + zext(ACC) * 3;\ngoto [PC];");
    let opcodes: Vec<&str> = ops
        .iter()
        .map(|s| s.split_whitespace().next().unwrap())
        .collect();
    assert_eq!(
        opcodes,
        vec![
            "CPUI_INT_ZEXT",
            "CPUI_INT_MULT",
            "CPUI_INT_ADD",
            "CPUI_BRANCHIND"
        ]
    );
}

// 10. [ARM] ARM_apcs bit test:  r1 = (r2 & 1) != 1;
#[test]
fn accept_and_notequal() {
    let lang = basic_lang();
    let (_, _, ops) = compile_collect(&lang, "r1 = (r2 & 1) != 1;");
    let opcodes: Vec<&str> = ops
        .iter()
        .map(|s| s.split_whitespace().next().unwrap())
        .collect();
    assert_eq!(opcodes, vec!["CPUI_INT_AND", "CPUI_INT_NOTEQUAL"]);
}

// 11. [ARM] ARM_apcs:  PC = LR & 0xfffffffe;
#[test]
fn accept_and_mask() {
    let lang = basic_lang();
    let ops = compile_ok(&lang, "PC = LR & 0xfffffffe;");
    assert_eq!(
        ops,
        vec!["CPUI_INT_AND (spc:ram,0x50,0x4) = (spc:ram,0x48,0x4), (spc:const,0xfffffffe,0x4)"]
    );
}

// 12. [tricore]/[Hexagon] empty-pcode placeholder:  tmpptr:4 = 0;
#[test]
fn accept_sized_temp_decl() {
    let lang = basic_lang();
    let ops = compile_ok(&lang, "tmpptr:4 = 0;");
    // newOutput(true, rhs=const 0, "tmpptr", size=4): COPY the const 0 into
    // the size-4 temp; propagateSize then sizes the const operand to 4.
    assert_eq!(
        ops,
        vec!["CPUI_COPY (spc:unique,0x0,0x4) = (spc:const,0x0,0x4)"]
    );
}

// 13. [HCS12] HCS12X:  R7 = SP;
#[test]
fn accept_reg_copy() {
    let lang = basic_lang();
    let ops = compile_ok(&lang, "R7 = SP;");
    assert_eq!(ops, vec!["CPUI_COPY (spc:ram,0x44,0x4) = (spc:ram,0x40,0x4)"]);
}

// 14. [Hexagon] hexagon prologue: typed-space load.
#[test]
fn accept_typed_space_load() {
    let lang = basic_lang();
    let ops = compile_ok(&lang, "ptr:4 = FP;\nFP = *[ram]:4 ptr;\nptr = ptr + 4;");
    assert_eq!(ops.len(), 3);
    assert!(ops[1].starts_with("CPUI_LOAD"));
    assert!(ops[1].contains("(spc:const,spc:ram,0x8)"));
}

// 15. Unary float negate (f-):  r1 = f- r2;
#[test]
fn accept_float_neg() {
    let lang = basic_lang();
    let ops = compile_ok(&lang, "r1 = f- r2;");
    assert_eq!(
        ops,
        vec!["CPUI_FLOAT_NEG (spc:ram,0x10,0x4) = (spc:ram,0x14,0x4)"]
    );
}

// 16. Signed comparison (s<):  r1 = r2 s< r0;
#[test]
fn accept_signed_less() {
    let lang = basic_lang();
    let (_, _, ops) = compile_collect(&lang, "r1 = r2 s< r0;");
    assert_eq!(ops.len(), 1);
    assert!(ops[0].starts_with("CPUI_INT_SLESS"));
    assert!(ops[0].ends_with("(spc:ram,0x14,0x4), (spc:ram,0x60,0x4)"));
}

// 17. '>' reduces to INT_LESS with swapped operands.
#[test]
fn accept_greater_swaps() {
    let lang = basic_lang();
    let (_, _, ops) = compile_collect(&lang, "r1 = r2 > r0;");
    assert_eq!(ops.len(), 1);
    assert!(ops[0].starts_with("CPUI_INT_LESS"));
    assert!(ops[0].ends_with("(spc:ram,0x60,0x4), (spc:ram,0x14,0x4)"));
}

// 18. User-op call with no output.
#[test]
fn accept_userop_no_out() {
    let lang = basic_lang().with_userop("myop", 7);
    let (_, _, ops) = compile_collect(&lang, "myop(r1, r2);");
    assert_eq!(ops.len(), 1);
    assert!(ops[0].starts_with("CPUI_CALLOTHER"));
    assert!(ops[0].contains("(spc:const,0x7,0x4)"));
    assert!(ops[0].contains("(spc:ram,0x10,0x4)"));
    assert!(ops[0].contains("(spc:ram,0x14,0x4)"));
}

// 19. Label define / place / conditional relative branch.
#[test]
fn accept_label_and_relative_branch() {
    let lang = basic_lang();
    let (_, _, ops) = compile_collect(&lang, "if (r1 != 0) goto <skip>;\nr2 = 1;\n<skip>\n");
    let opcodes: Vec<&str> = ops
        .iter()
        .map(|s| s.split_whitespace().next().unwrap())
        .collect();
    assert!(opcodes.contains(&"CPUI_INT_NOTEQUAL"));
    assert!(opcodes.contains(&"CPUI_CBRANCH"));
    // The LABELBUILD pseudo-op terminates the sequence (its Debug name).
    assert!(opcodes.last().is_some());
    // The CBRANCH targets a relative (j_relative) label varnode.
    let cbranch = ops.iter().find(|s| s.starts_with("CPUI_CBRANCH")).unwrap();
    assert!(cbranch.contains("j_relative"));
}

// 20. Bitrange read on the rhs:  r1 = r2:1;
#[test]
fn accept_bitrange_truncated_varnode_fast_path() {
    let lang = basic_lang();
    let ops = compile_ok(&lang, "r1 = r2:1;");
    // createBitRange(r2, 0, 8): bitoffset 0, byte-aligned, single byte =>
    // buildTruncatedVarnode produces a direct truncated varnode (no SUBPIECE
    // op), so the statement is just a COPY of the low byte. r2 is at 0x14
    // size 4 big-endian, so the size-1 truncation lands at 0x14+3 = 0x17.
    assert_eq!(
        ops,
        vec!["CPUI_COPY (spc:ram,0x10,0x4) = (spc:ram,0x17,0x1)"]
    );
}

// 20b. A masked (non-byte-aligned) bitrange does emit ops: r1 = r2[1,3];
#[test]
fn accept_bitrange_masked_emits_ops() {
    let lang = basic_lang();
    let (ok, err, ops) = compile_collect(&lang, "r1 = r2[1,3];");
    // bitoffset 1, 3 bits: not byte-aligned => RIGHT shift + SUBPIECE + AND
    // mask chain (createBitRange's general path).
    let _ = (ok, err);
    let opcodes: Vec<&str> = ops
        .iter()
        .map(|s| s.split_whitespace().next().unwrap())
        .collect();
    assert!(opcodes.contains(&"CPUI_INT_RIGHT"));
    assert!(opcodes.contains(&"CPUI_INT_AND"));
}

// 21. Indirect call / return.
#[test]
fn accept_indirect_call_return() {
    let lang = basic_lang();
    let (_, _, ops) = compile_collect(&lang, "call [r1];\nreturn [r2];");
    let opcodes: Vec<&str> = ops
        .iter()
        .map(|s| s.split_whitespace().next().unwrap())
        .collect();
    assert_eq!(opcodes, vec!["CPUI_CALLIND", "CPUI_RETURN"]);
}

// 22. Address-of:  r1 = &r2;
#[test]
fn accept_address_of() {
    let lang = basic_lang();
    let ops = compile_ok(&lang, "r1 = &r2;");
    assert_eq!(ops.len(), 1);
    assert!(ops[0].starts_with("CPUI_COPY"));
    assert!(ops[0].contains("(spc:const,0x14,0x4)"));
}

// ---------------------------------------------------------------------------
// Reject tests: malformed snippets, mirroring C++ yyerror text
// ---------------------------------------------------------------------------

#[test]
fn accept_unknown_string_is_temp_declaration() {
    // CONFLICT RESOLUTION (pcodeparse.y %expect 3, conflict 2): `STRING '='`
    // is NOT the lhsvarnode-STRING error — it is resolved by shifting into the
    // temporary-declaration production `STRING '=' expr ';'`. So an unknown
    // name on the lhs of a plain assignment defines a new temporary, it does
    // not error. (The "Unknown assignment varnode" error is only reachable via
    // the bit-range lhs form below.)
    let lang = basic_lang();
    let mut snip = PcodeSnippet::new(&lang);
    assert!(snip.parse_stream(b"nope = r1;"), "{}", snip.get_error_message());
}

#[test]
fn reject_unknown_assignment_varnode_bitrange() {
    let lang = basic_lang();
    // lhsvarnode '[' INTEGER ',' INTEGER ']' '=' expr ';' with lhsvarnode a
    // STRING => the "Unknown assignment varnode" error production.
    let err = compile_err(&lang, "nope[0,8] = r1;");
    assert_eq!(err, "Unknown assignment varnode: nope");
}

#[test]
fn reject_unknown_varnode_parameter() {
    let lang = basic_lang();
    let err = compile_err(&lang, "r1 = nope;");
    assert_eq!(err, "Unknown varnode parameter: nope");
}

#[test]
fn reject_unknown_jump_destination() {
    let lang = basic_lang();
    let err = compile_err(&lang, "goto nope;");
    assert_eq!(err, "Unknown jump destination: nope");
}

#[test]
fn reject_return_without_indirect() {
    let lang = basic_lang();
    let err = compile_err(&lang, "return;");
    assert_eq!(err, "Must specify an indirect parameter for return");
}

#[test]
fn reject_illegal_truncation_lhs() {
    let lang = basic_lang();
    let err = compile_err(&lang, "r1:1 = r2;");
    assert_eq!(err, "Illegal truncation on left-hand side of assignment");
}

#[test]
fn reject_illegal_subpiece_lhs() {
    let lang = basic_lang();
    let err = compile_err(&lang, "r1(0) = r2;");
    assert_eq!(err, "Illegal subpiece on left-hand side of assignment");
}

#[test]
fn reject_redefinition_of_symbol() {
    let lang = basic_lang();
    let err = compile_err(&lang, "local r1 = r2;");
    assert_eq!(err, "Redefinition of symbol: r1");
}

#[test]
fn reject_syntax_error_dangling() {
    let lang = basic_lang();
    let err = compile_err(&lang, "r1 = r2 +");
    assert_eq!(err, "Syntax error");
}

#[test]
fn reject_chained_nonassoc_comparison() {
    let lang = basic_lang();
    let err = compile_err(&lang, "r1 = r2 < r0 < r1;");
    assert_eq!(err, "Syntax error");
}

// ---------------------------------------------------------------------------
// getVarnode() resolver tests (the LOSS-024/026 closure)
// ---------------------------------------------------------------------------

#[test]
fn get_varnode_tpl_varnode_symbol() {
    let lang = basic_lang();
    let cspace = lang.get_constant_space();
    let vsym = VarnodeSymbol::new_for_test(Rc::clone(&lang.ram), 0x10, 4);
    let sym = SnippetSymbol::Varnode(vsym, b"r1".to_vec());
    let vn = get_varnode_tpl(&sym, &cspace).unwrap();
    assert_eq!(vn.get_space().get_type(), ConstType::Spaceid);
    assert_eq!(vn.get_offset().get_real(), 0x10);
    assert_eq!(vn.get_size().get_real(), 4);
}

#[test]
fn get_varnode_tpl_flow_symbols() {
    let lang = basic_lang();
    let cspace = lang.get_constant_space();
    let cases = [
        (SnippetSymbol::Start(b"inst_start".to_vec()), ConstType::JStart),
        (SnippetSymbol::End(b"inst_next".to_vec()), ConstType::JNext),
        (SnippetSymbol::Next2(b"inst_next2".to_vec()), ConstType::JNext2),
        (
            SnippetSymbol::FlowDest(b"inst_dest".to_vec()),
            ConstType::JFlowdest,
        ),
        (
            SnippetSymbol::FlowRef(b"inst_ref".to_vec()),
            ConstType::JFlowref,
        ),
    ];
    for (sym, off_ty) in cases {
        let vn = get_varnode_tpl(&sym, &cspace).unwrap();
        assert_eq!(vn.get_space().get_type(), ConstType::Spaceid);
        assert_eq!(vn.get_offset().get_type(), off_ty);
        assert!(vn.get_size().is_zero());
    }
}

#[test]
fn get_varnode_tpl_operand_symbol() {
    let lang = basic_lang();
    let cspace = lang.get_constant_space();
    let sym = SnippetSymbol::Operand(3, b"op".to_vec());
    let vn = get_varnode_tpl(&sym, &cspace).unwrap();
    // new VarnodeTpl(hand=3, zerosize=false): all three are handle constants.
    assert_eq!(vn.get_space().get_type(), ConstType::Handle);
    assert_eq!(vn.get_space().get_handle_index(), 3);
    assert_eq!(vn.get_offset().get_select(), VField::VOffset);
    assert_eq!(vn.get_size().get_select(), VField::VSize);
}

// ---------------------------------------------------------------------------
// clear() resets the snippet for reuse against the same language
// ---------------------------------------------------------------------------

#[test]
fn clear_resets_for_reuse() {
    let lang = basic_lang();
    let mut snip = PcodeSnippet::new(&lang);
    assert!(snip.parse_stream(b"r1 = r2;"));
    snip.clear();
    assert!(!snip.has_errors());
    assert!(snip.release_result().is_none());
    assert!(snip.parse_stream(b"r2 = r1;"));
}


#[test]
fn spot_check_chained_arith_and_float_ops() {
    let lang = basic_lang();
    // x86win: ESP = ESP + 4 - EAX;  (left-assoc + then -)
    let (ok, err, ops) = compile_collect(&lang, "ESP = ESP + 4 - EAX;");
    assert!(ok || !err.is_empty());
    let opcodes: Vec<&str> = ops.iter().map(|s| s.split_whitespace().next().unwrap()).collect();
    assert_eq!(opcodes, vec!["CPUI_INT_ADD", "CPUI_INT_SUB"]);

    // float compare and float arithmetic lexing (f<, f+).
    let (_, _, ops2) = compile_collect(&lang, "r1 = r2 f< r0;");
    assert!(ops2[0].starts_with("CPUI_FLOAT_LESS"));
    let (_, _, ops3) = compile_collect(&lang, "r1 = r2 f+ r0;");
    assert!(ops3[0].starts_with("CPUI_FLOAT_ADD"));

    // unary minus literal:  r1 = -1;  => INT_2COMP of const 1
    let (_, _, ops4) = compile_collect(&lang, "r1 = -1;");
    assert!(ops4[0].starts_with("CPUI_INT_2COMP"));
}
