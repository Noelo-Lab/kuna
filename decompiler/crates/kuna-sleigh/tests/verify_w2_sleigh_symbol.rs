//! Verifier adversarial tests for item `w2-sleigh-symbol` (round 1).
//!
//! Written by the independent verifier against the C++ oracle semantics of
//! `decompiler/cpp/slghsymbol.{hh,cc}`.  Each test targets a hunt-list spot
//! the port could plausibly have gotten wrong while still passing the
//! porter's own tests; expected values are hand-derived C++ traces (anchors
//! cited per test).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{Decoder, Encoder, PackedDecode, PackedEncode};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace,
};
use kuna_base::types::Wrap;

use kuna_sleigh::context::FixedHandle;
use kuna_sleigh::slghpatexpress::{
    ConstantValue, OperandValue, PatternExpressionContext, TokenField,
};
use kuna_sleigh::slghpattern::InstructionPattern;
use kuna_sleigh::slghsymbol::{
    sla, ConstructTplHandle, Constructor, ConstructorRef, ContextChange, ContextOp,
    SleighBaseTrans, SymbolKind, SymbolTable, SymbolWalker, SymbolWalkerChange,
};

// -- harness -----------------------------------------------------------------

fn test_manager() -> AddrSpaceManager {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    let spc = AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        4,
        1,
        1,
        addrspace_flags::hasphysical,
        1,
        1,
    );
    manager.insert_space(Rc::new(spc)).unwrap();
    manager
}

fn ram(manager: &AddrSpaceManager) -> Rc<AddrSpace> {
    Rc::clone(manager.get_space_by_name("ram").unwrap())
}

struct TestTrans {
    cspace: Rc<AddrSpace>,
}

impl TestTrans {
    fn new(manager: &AddrSpaceManager) -> TestTrans {
        TestTrans {
            cspace: Rc::clone(manager.get_constant_space().unwrap()),
        }
    }
}

impl SleighBaseTrans for TestTrans {
    fn get_constant_space(&self) -> Rc<AddrSpace> {
        Rc::clone(&self.cspace)
    }

    fn decode_construct_tpl(
        &mut self,
        _decoder: &mut dyn Decoder,
    ) -> KunaResult<(i32, ConstructTplHandle)> {
        Err(KunaError::sleigh("no ConstructTpl in verifier tests"))
    }

    fn encode_construct_tpl(
        &self,
        _handle: ConstructTplHandle,
        _section_id: i32,
        _encoder: &mut dyn Encoder,
    ) -> KunaResult<()> {
        Err(KunaError::sleigh("no ConstructTpl in verifier tests"))
    }
}

/// Synthetic walker mirroring the documented ParserWalker semantics for
/// exactly the surface these tests touch.
struct VWalker {
    instr: Vec<u8>,
    addr: Option<Address>,
    path: Vec<i32>,
    /// operand path -> constructor at that point
    ctmap: Vec<(Vec<i32>, ConstructorRef)>,
    set_words: Vec<(i32, u32, u32)>,
    commits: Vec<(u32, i32, u32, bool)>,
}

impl VWalker {
    fn from_bytes(instr: &[u8]) -> VWalker {
        VWalker {
            instr: instr.to_vec(),
            addr: None,
            path: Vec::new(),
            ctmap: Vec::new(),
            set_words: Vec::new(),
            commits: Vec::new(),
        }
    }
}

impl PatternExpressionContext for VWalker {
    fn get_instruction_bytes(&self, byteoff: i32, numbytes: i32) -> KunaResult<u32> {
        if byteoff < 0 || (byteoff + numbytes) as usize > self.instr.len() {
            return Err(KunaError::bad_data(
                "Instruction is using more than 16 bytes",
            ));
        }
        let mut res: u32 = 0;
        for i in 0..numbytes {
            res = res.wshl(8);
            res |= u32::from(self.instr[(byteoff + i) as usize]);
        }
        Ok(res)
    }

    fn get_context_bytes(&self, _bytestart: i32, _size: i32) -> KunaResult<u32> {
        Ok(0)
    }

    fn get_addr(&self) -> Address {
        self.addr.clone().expect("verifier walker addr unset")
    }

    fn get_naddr(&self) -> Address {
        panic!("verify_w2_sleigh_symbol never reads naddr")
    }

    fn get_n2addr(&self) -> KunaResult<Address> {
        Err(KunaError::lowlevel("inst_next2 not available"))
    }

    fn operand_value(&self, _index: i32, _table_id: u32, _ct_id: u32) -> KunaResult<i64> {
        Err(KunaError::sleigh("no operand values in verifier tests"))
    }
}

impl SymbolWalker for VWalker {
    fn push_operand(&mut self, i: i32) -> KunaResult<()> {
        self.path.push(i);
        Ok(())
    }

    fn pop_operand(&mut self) -> KunaResult<()> {
        self.path.pop();
        Ok(())
    }

    fn get_constructor(&self) -> KunaResult<ConstructorRef> {
        for (p, ctref) in &self.ctmap {
            if *p == self.path {
                return Ok(*ctref);
            }
        }
        Err(KunaError::sleigh("no constructor mapped for operand path"))
    }

    fn get_fixed_handle(&self, _i: i32) -> KunaResult<FixedHandle> {
        Err(KunaError::sleigh("no fixed handles in verifier tests"))
    }

    fn get_const_space(&self) -> Rc<AddrSpace> {
        panic!("verify_w2_sleigh_symbol never reads const space")
    }

    fn get_cur_space(&self) -> Rc<AddrSpace> {
        panic!("verify_w2_sleigh_symbol never reads cur space")
    }

    fn get_dest_addr(&self) -> KunaResult<Address> {
        Err(KunaError::lowlevel("no flowdest in verifier tests"))
    }

    fn get_ref_addr(&self) -> KunaResult<Address> {
        Err(KunaError::lowlevel("no flowref in verifier tests"))
    }

    fn get_instruction_bits(&self, startbit: i32, size: i32) -> KunaResult<u32> {
        let mut res: u32 = 0;
        for i in 0..size {
            let bit = startbit + i;
            let byte = (bit / 8) as usize;
            if byte >= self.instr.len() {
                return Err(KunaError::bad_data("instruction bits past buffer"));
            }
            let b = (self.instr[byte] >> (7 - (bit % 8))) & 1;
            res = (res << 1) | u32::from(b);
        }
        Ok(res)
    }

    fn get_context_bits(&self, _startbit: i32, _size: i32) -> KunaResult<u32> {
        Ok(0)
    }
}

impl SymbolWalkerChange for VWalker {
    fn set_context_word(&mut self, i: i32, val: u32, mask: u32) {
        self.set_words.push((i, val, mask));
    }

    fn add_commit(&mut self, symbol_id: u32, num: i32, mask: u32, flow: bool) {
        self.commits.push((symbol_id, num, mask, flow));
    }
}

fn enc_head(enc: &mut PackedEncode, elem: &kuna_base::marshal::ElementId, name: &[u8], id: u64) {
    enc.open_element(elem);
    enc.write_string(&sla::ATTRIB_NAME, name);
    enc.write_unsigned_integer(&sla::ATTRIB_ID, id);
    enc.write_unsigned_integer(&sla::ATTRIB_SCOPE, 0);
    enc.close_element(elem);
}

/// One-scope symbol table stream built by `heads` + `contents`, then decoded.
fn decode_table(
    manager: &AddrSpaceManager,
    nsyms: i64,
    heads: impl FnOnce(&mut PackedEncode),
    contents: impl FnOnce(&mut PackedEncode),
) -> KunaResult<(SymbolTable, Vec<u8>)> {
    let mut buf = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf);
        enc.open_element(&sla::ELEM_SYMBOL_TABLE);
        enc.write_signed_integer(&sla::ATTRIB_SCOPESIZE, 1);
        enc.write_signed_integer(&sla::ATTRIB_SYMBOLSIZE, nsyms);
        enc.open_element(&sla::ELEM_SCOPE);
        enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
        enc.write_unsigned_integer(&sla::ATTRIB_PARENT, 0);
        enc.close_element(&sla::ELEM_SCOPE);
        heads(&mut enc);
        contents(&mut enc);
        enc.close_element(&sla::ELEM_SYMBOL_TABLE);
    }
    let mut table = SymbolTable::new();
    let mut trans = TestTrans::new(manager);
    let mut dec = PackedDecode::new(manager);
    dec.ingest_stream(&buf)?;
    table.decode(&mut dec, &mut trans)?;
    Ok((table, buf))
}

// -- T1: OperandSymbol::decode double-fetch attribute loop --------------------
//
// C++ slghsymbol.cc:1108-1127 fetches getNextAttributeId at the TOP of the
// loop body, so the first peeked attribute only feeds the loop condition.
// This is correct ONLY because the caller's find-by-id read of ATTRIB_ID
// (SymbolTable::decode, slghsymbol.cc:213) resets curPos to startPos
// (marshal.cc:932-939), making ATTRIB_ID itself the skipped attribute.  A
// port that "fixed" the loop shape, or a marshal that did not reset, would
// drop SUBSYM (the attribute physically after ID).  The porter's own tests
// never encode SUBSYM or CODE; this stream carries every optional attribute.
#[test]
fn verify_w2sym_operand_decode_all_attributes_and_byte_roundtrip() {
    let manager = test_manager();
    let (table, buf) = decode_table(
        &manager,
        3,
        |enc| {
            enc_head(enc, &sla::ELEM_SUBTABLE_SYM_HEAD, b"t", 0);
            enc_head(enc, &sla::ELEM_OPERAND_SYM_HEAD, b"op", 1);
            enc_head(enc, &sla::ELEM_VARNODE_SYM_HEAD, b"r0", 2);
        },
        |enc| {
            // subtable t: one minimal constructor, decision tree so the
            // re-encode emits the subtable (C++ skips "not fully formed")
            enc.open_element(&sla::ELEM_SUBTABLE_SYM);
            enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
            enc.write_signed_integer(&sla::ATTRIB_NUMCT, 1);
            enc.open_element(&sla::ELEM_CONSTRUCTOR);
            enc.write_unsigned_integer(&sla::ATTRIB_PARENT, 0);
            enc.write_signed_integer(&sla::ATTRIB_FIRST, -1);
            enc.write_signed_integer(&sla::ATTRIB_LENGTH, 1);
            enc.write_signed_integer(&sla::ATTRIB_SOURCE, 0);
            enc.write_signed_integer(&sla::ATTRIB_LINE, 1);
            enc.open_element(&sla::ELEM_PRINT);
            enc.write_string(&sla::ATTRIB_PIECE, b"nop");
            enc.close_element(&sla::ELEM_PRINT);
            enc.close_element(&sla::ELEM_CONSTRUCTOR);
            enc.open_element(&sla::ELEM_DECISION);
            enc.write_signed_integer(&sla::ATTRIB_NUMBER, 1);
            enc.write_bool(&sla::ATTRIB_CONTEXT, false);
            enc.write_signed_integer(&sla::ATTRIB_STARTBIT, 0);
            enc.write_signed_integer(&sla::ATTRIB_SIZE, 0);
            enc.open_element(&sla::ELEM_PAIR);
            enc.write_signed_integer(&sla::ATTRIB_ID, 0);
            InstructionPattern::new_always(true).encode(enc);
            enc.close_element(&sla::ELEM_PAIR);
            enc.close_element(&sla::ELEM_DECISION);
            enc.close_element(&sla::ELEM_SUBTABLE_SYM);
            // operand op: full attribute set, in C++ encode order
            // (slghsymbol.cc:1072-1089): ID, SUBSYM, OFF, BASE, MINLEN,
            // CODE, INDEX, then localexp
            enc.open_element(&sla::ELEM_OPERAND_SYM);
            enc.write_unsigned_integer(&sla::ATTRIB_ID, 1);
            enc.write_unsigned_integer(&sla::ATTRIB_SUBSYM, 2);
            enc.write_signed_integer(&sla::ATTRIB_OFF, 5);
            enc.write_signed_integer(&sla::ATTRIB_BASE, -1);
            enc.write_signed_integer(&sla::ATTRIB_MINLEN, 2);
            enc.write_bool(&sla::ATTRIB_CODE, true);
            enc.write_signed_integer(&sla::ATTRIB_INDEX, 3);
            OperandValue::new(3, 0, 0).encode(enc);
            enc.close_element(&sla::ELEM_OPERAND_SYM);
            // varnode r0 (the SUBSYM target)
            enc.open_element(&sla::ELEM_VARNODE_SYM);
            enc.write_unsigned_integer(&sla::ATTRIB_ID, 2);
            enc.write_space(&sla::ATTRIB_SPACE, &ram(&test_manager()));
            enc.write_unsigned_integer(&sla::ATTRIB_OFF, 0x20);
            enc.write_signed_integer(&sla::ATTRIB_SIZE, 4);
            enc.close_element(&sla::ELEM_VARNODE_SYM);
        },
    )
    .unwrap();
    let SymbolKind::Operand(op) = table.find_global_symbol(b"op").unwrap().kind() else {
        panic!("expected operand symbol");
    };
    // every attribute after the skipped ATTRIB_ID must have been picked up
    assert_eq!(op.get_defining_symbol(), Some(2), "SUBSYM lost by the attribute loop");
    assert_eq!(op.get_relative_offset(), 5, "OFF lost by the attribute loop");
    assert_eq!(op.get_offset_base(), -1);
    assert_eq!(op.get_minimum_length(), 2);
    assert!(op.is_code_address(), "CODE lost by the attribute loop");
    assert_eq!(op.get_index(), 3);
    // with a defining symbol there is no defexp (C++ leaves it null)
    assert!(op.get_defining_expression().is_none());
    let lexp = op.get_local_expression().unwrap();
    assert_eq!((lexp.index(), lexp.table_id(), lexp.ct_id()), (3, 0, 0));
    // re-encode of the whole table is byte-identical to the C++-shaped input
    let trans = TestTrans::new(&manager);
    let mut reenc = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut reenc);
        table.encode(&mut enc, &trans).unwrap();
    }
    assert_eq!(reenc, buf);
}

// -- T2: DecisionNode pair-id intb -> uintm truncation ------------------------
//
// C++ slghsymbol.cc:2398: `uintm id = decoder.readSignedInteger(ATTRIB_ID)`
// truncates the signed read to uint32, then compares against
// getNumConstructors() converted to uintm.  A pair id of -1 therefore
// becomes 0xFFFFFFFF and MUST be rejected ("Invalid constructor id").  A
// port that kept the comparison signed (`-1 >= 1` is false) would accept it
// and store a wild constructor id.
#[test]
fn verify_w2sym_decision_pair_id_truncation_boundaries() {
    let manager = test_manager();
    let build = |pair_id: i64| {
        decode_table(
            &manager,
            1,
            |enc| enc_head(enc, &sla::ELEM_SUBTABLE_SYM_HEAD, b"t", 0),
            |enc| {
                enc.open_element(&sla::ELEM_SUBTABLE_SYM);
                enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
                enc.write_signed_integer(&sla::ATTRIB_NUMCT, 1);
                enc.open_element(&sla::ELEM_CONSTRUCTOR);
                enc.write_unsigned_integer(&sla::ATTRIB_PARENT, 0);
                enc.write_signed_integer(&sla::ATTRIB_FIRST, -1);
                enc.write_signed_integer(&sla::ATTRIB_LENGTH, 1);
                enc.write_signed_integer(&sla::ATTRIB_SOURCE, 0);
                enc.write_signed_integer(&sla::ATTRIB_LINE, 1);
                enc.open_element(&sla::ELEM_PRINT);
                enc.write_string(&sla::ATTRIB_PIECE, b"nop");
                enc.close_element(&sla::ELEM_PRINT);
                enc.close_element(&sla::ELEM_CONSTRUCTOR);
                enc.open_element(&sla::ELEM_DECISION);
                enc.write_signed_integer(&sla::ATTRIB_NUMBER, 1);
                enc.write_bool(&sla::ATTRIB_CONTEXT, false);
                enc.write_signed_integer(&sla::ATTRIB_STARTBIT, 0);
                enc.write_signed_integer(&sla::ATTRIB_SIZE, 0);
                enc.open_element(&sla::ELEM_PAIR);
                enc.write_signed_integer(&sla::ATTRIB_ID, pair_id);
                InstructionPattern::new_always(true).encode(enc);
                enc.close_element(&sla::ELEM_PAIR);
                enc.close_element(&sla::ELEM_DECISION);
                enc.close_element(&sla::ELEM_SUBTABLE_SYM);
            },
        )
    };
    // -1 truncates to 0xFFFFFFFF >= 1: rejected
    let err = build(-1).expect_err("pair id -1 must be rejected after truncation");
    assert!(format!("{err}").contains("Invalid constructor id"));
    // id == numct: rejected (boundary)
    let err = build(1).expect_err("pair id == numct must be rejected");
    assert!(format!("{err}").contains("Invalid constructor id"));
    // id == numct-1: accepted, stored
    let (table, _) = build(0).unwrap();
    let SymbolKind::Subtable(sub) = table.find_global_symbol(b"t").unwrap().kind() else {
        panic!("expected subtable");
    };
    let tree = sub.get_decision_tree().unwrap();
    assert_eq!(tree.get_list().len(), 1);
    assert_eq!(tree.get_list()[0].1, 0);
}

// -- T3: flow-through printing (flowthruindex) --------------------------------
//
// C++ slghsymbol.cc:1521-1565 + 1723-1726: a constructor whose only print
// piece is an operand placeholder gets flowthruindex set at decode;
// printMnemonic/printBody then push that operand and delegate to the
// resolved sub-constructor (the dynamic_cast<SubtableSymbol*> test on the
// operand's defining symbol).  Mnemonic stops at the sub-constructor's
// firstwhitespace; body starts after it.
#[test]
fn verify_w2sym_flowthru_mnemonic_body_recursion() {
    let manager = test_manager();
    let (table, _) = decode_table(
        &manager,
        3,
        |enc| {
            enc_head(enc, &sla::ELEM_SUBTABLE_SYM_HEAD, b"instruction", 0);
            enc_head(enc, &sla::ELEM_SUBTABLE_SYM_HEAD, b"sub", 1);
            enc_head(enc, &sla::ELEM_OPERAND_SYM_HEAD, b"op0", 2);
        },
        |enc| {
            // instruction: one constructor, print pieces = [OPPRINT 0] only
            enc.open_element(&sla::ELEM_SUBTABLE_SYM);
            enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
            enc.write_signed_integer(&sla::ATTRIB_NUMCT, 1);
            enc.open_element(&sla::ELEM_CONSTRUCTOR);
            enc.write_unsigned_integer(&sla::ATTRIB_PARENT, 0);
            enc.write_signed_integer(&sla::ATTRIB_FIRST, -1);
            enc.write_signed_integer(&sla::ATTRIB_LENGTH, 1);
            enc.write_signed_integer(&sla::ATTRIB_SOURCE, 0);
            enc.write_signed_integer(&sla::ATTRIB_LINE, 1);
            enc.open_element(&sla::ELEM_OPER);
            enc.write_unsigned_integer(&sla::ATTRIB_ID, 2);
            enc.close_element(&sla::ELEM_OPER);
            enc.open_element(&sla::ELEM_OPPRINT);
            enc.write_signed_integer(&sla::ATTRIB_ID, 0);
            enc.close_element(&sla::ELEM_OPPRINT);
            enc.close_element(&sla::ELEM_CONSTRUCTOR);
            enc.close_element(&sla::ELEM_SUBTABLE_SYM);
            // sub: one constructor "mv" ' ' "x" with firstwhitespace 1
            enc.open_element(&sla::ELEM_SUBTABLE_SYM);
            enc.write_unsigned_integer(&sla::ATTRIB_ID, 1);
            enc.write_signed_integer(&sla::ATTRIB_NUMCT, 1);
            enc.open_element(&sla::ELEM_CONSTRUCTOR);
            enc.write_unsigned_integer(&sla::ATTRIB_PARENT, 1);
            enc.write_signed_integer(&sla::ATTRIB_FIRST, 1);
            enc.write_signed_integer(&sla::ATTRIB_LENGTH, 1);
            enc.write_signed_integer(&sla::ATTRIB_SOURCE, 0);
            enc.write_signed_integer(&sla::ATTRIB_LINE, 2);
            for piece in [&b"mv"[..], &b" "[..], &b"x"[..]] {
                enc.open_element(&sla::ELEM_PRINT);
                enc.write_string(&sla::ATTRIB_PIECE, piece);
                enc.close_element(&sla::ELEM_PRINT);
            }
            enc.close_element(&sla::ELEM_CONSTRUCTOR);
            enc.close_element(&sla::ELEM_SUBTABLE_SYM);
            // op0: defined by subtable "sub"
            enc.open_element(&sla::ELEM_OPERAND_SYM);
            enc.write_unsigned_integer(&sla::ATTRIB_ID, 2);
            enc.write_unsigned_integer(&sla::ATTRIB_SUBSYM, 1);
            enc.write_signed_integer(&sla::ATTRIB_OFF, 0);
            enc.write_signed_integer(&sla::ATTRIB_BASE, -1);
            enc.write_signed_integer(&sla::ATTRIB_MINLEN, 1);
            enc.write_signed_integer(&sla::ATTRIB_INDEX, 0);
            OperandValue::new(0, 0, 0).encode(enc);
            enc.close_element(&sla::ELEM_OPERAND_SYM);
        },
    )
    .unwrap();
    let top = table
        .get_constructor(ConstructorRef {
            table_id: 0,
            ct_id: 0,
        })
        .unwrap();
    // the single-OPPRINT piece sets the flow-through index at decode
    assert_eq!(top.get_flowthru_index(), 0);
    let mut walker = VWalker::from_bytes(&[0, 0, 0, 0]);
    walker.ctmap.push((
        vec![0],
        ConstructorRef {
            table_id: 1,
            ct_id: 0,
        },
    ));
    // mnemonic flows through to sub ct0's mnemonic (pieces before FIRST=1)
    let mut s = String::new();
    top.print_mnemonic(&mut s, &mut walker, &table).unwrap();
    assert_eq!(s, "mv");
    assert!(walker.path.is_empty(), "pushOperand/popOperand unbalanced");
    // body flows through to sub ct0's body (pieces after FIRST)
    let mut s = String::new();
    top.print_body(&mut s, &mut walker, &table).unwrap();
    assert_eq!(s, "x");
    assert!(walker.path.is_empty());
    // full print recurses through the operand into the sub-constructor
    let mut s = String::new();
    top.print(&mut s, &mut walker, &table).unwrap();
    assert_eq!(s, "mv x");
    assert!(walker.path.is_empty());
}

// -- T4: ContextOp::apply intb -> uintm truncation ----------------------------
//
// C++ slghsymbol.cc:2439-2445: `uintm val = patexp->getValue(walker)`
// truncates the 64-bit expression value to uint32 BEFORE the shift.  The
// hand trace for value -1, shift 28: (uintm)(-1) = 0xFFFFFFFF, << 28 =
// 0xF0000000.  For value 0x100000005, shift 0: truncates to 5.
#[test]
fn verify_w2sym_contextop_apply_value_truncation() {
    let manager = test_manager();
    let decode_op = |i: i64, shift: i64, mask: u64, val: i64| {
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            enc.open_element(&sla::ELEM_CONTEXT_OP);
            enc.write_signed_integer(&sla::ATTRIB_I, i);
            enc.write_signed_integer(&sla::ATTRIB_SHIFT, shift);
            enc.write_unsigned_integer(&sla::ATTRIB_MASK, mask);
            ConstantValue::new(val).encode(&mut enc);
            enc.close_element(&sla::ELEM_CONTEXT_OP);
        }
        let table = SymbolTable::new();
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&buf).unwrap();
        ContextOp::decode(&mut dec, &table).unwrap()
    };
    // value -1 truncates to 0xFFFFFFFF, then shifts within 32 bits
    let op = decode_op(2, 28, 0xf000_0000, -1);
    let mut walker = VWalker::from_bytes(&[]);
    ContextChange::Op(op).apply(&mut walker).unwrap();
    assert_eq!(walker.set_words, vec![(2, 0xf000_0000, 0xf000_0000)]);
    // bits above 32 are discarded by the truncation
    let op = decode_op(0, 0, 0xff, 0x1_0000_0005);
    let mut walker = VWalker::from_bytes(&[]);
    ContextChange::Op(op).apply(&mut walker).unwrap();
    assert_eq!(walker.set_words, vec![(0, 5, 0xff)]);
}

// -- T5: Constructor::addSyntax coalescing / firstwhitespace ------------------
//
// C++ slghsymbol.cc:1453-1482 hand trace:
//   addSyntax("   ")  all-space -> syntrim " ", firstwhitespace = 0 (empty
//                     printpiece), pushed                      -> [" "]
//   addSyntax("a")    back == " " -> pushed (not appended)     -> [" ", "a"]
//   addSyntax("b")    appended to back                         -> [" ", "ab"]
//   addSyntax(" x ")  has non-space -> kept verbatim, appended -> [" ", "ab x "]
//   addOperand        placeholder "\nA" (first operand)        -> [..., "\nA"]
//   addSyntax(" ")    back[0] == '\n' -> pushed                -> [..., " "]
//   addSyntax(" ")    back == " " && syntrim == " " -> dropped
//   removeTrailingSpace pops the single trailing " "
#[test]
fn verify_w2sym_add_syntax_coalescing_and_first_whitespace() {
    let mut ct = Constructor::new();
    ct.add_syntax(b"");
    assert!(ct.get_print_pieces().is_empty());
    assert_eq!(ct.get_first_whitespace(), -1);
    ct.add_syntax(b"   ");
    assert_eq!(ct.get_first_whitespace(), 0);
    ct.add_syntax(b"a");
    ct.add_syntax(b"b");
    ct.add_syntax(b" x ");
    ct.add_operand(7);
    ct.add_syntax(b" ");
    ct.add_syntax(b" ");
    assert_eq!(
        ct.get_print_pieces(),
        &[
            b" ".to_vec(),
            b"ab x ".to_vec(),
            b"\nA".to_vec(),
            b" ".to_vec(),
        ]
    );
    ct.remove_trailing_space();
    assert_eq!(
        ct.get_print_pieces(),
        &[b" ".to_vec(), b"ab x ".to_vec(), b"\nA".to_vec()]
    );
    // firstwhitespace was captured at the first all-space piece and is not
    // recomputed
    assert_eq!(ct.get_first_whitespace(), 0);
    assert_eq!(ct.get_num_operands(), 1);
    assert_eq!(ct.get_operand(0).unwrap(), 7);
}

// -- T6: NameSymbol truncated table + "_" rewrite on re-encode ----------------
//
// C++ slghsymbol.cc:643-655: checkTableFill leaves tableisfilled false when
// maxValue >= nametable.size() and rewrites "_" entries to "\t"; resolve
// (657-671) then rejects both an out-of-range index and a "\t" hole with
// BadDataError.  encode (681-697) writes a "\t" entry as a NAMETAB element
// with NO name attribute, so a decoded "_" does NOT round-trip to "_".
#[test]
fn verify_w2sym_name_symbol_truncated_table_and_tab_reencode() {
    let manager = test_manager();
    let name_sym_content = |enc: &mut PackedEncode, second_entry_name: Option<&[u8]>| {
        enc.open_element(&sla::ELEM_NAME_SYM);
        enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
        // 2-bit field: maxValue 3 >= table size 2 -> table not filled
        TokenField::new(1, false, false, 0, 1).encode(enc);
        enc.open_element(&sla::ELEM_NAMETAB);
        enc.write_string(&sla::ATTRIB_NAME, b"x");
        enc.close_element(&sla::ELEM_NAMETAB);
        enc.open_element(&sla::ELEM_NAMETAB);
        if let Some(nm) = second_entry_name {
            enc.write_string(&sla::ATTRIB_NAME, nm);
        }
        enc.close_element(&sla::ELEM_NAMETAB);
        enc.close_element(&sla::ELEM_NAME_SYM);
    };
    let (table, _) = decode_table(
        &manager,
        1,
        |enc| enc_head(enc, &sla::ELEM_NAME_SYM_HEAD, b"names", 0),
        |enc| name_sym_content(enc, Some(b"_")),
    )
    .unwrap();
    let sym = table.find_global_symbol(b"names").unwrap();
    let SymbolKind::Name(ns) = sym.kind() else {
        panic!("expected name symbol");
    };
    // "_" was rewritten to the TAB illegal-index marker
    assert_eq!(ns.get_name_table(), &[b"x".to_vec(), b"\t".to_vec()]);
    // index 2 is out of range (table not filled, so resolve checks)
    let mut walker = VWalker::from_bytes(&[0x02]);
    walker.addr = Some(Address::new(ram(&manager), 0x40));
    let err = sym.resolve(&walker).expect_err("out-of-range index");
    assert!(format!("{err}").contains("No corresponding entry in nametable"));
    // index 1 hits the TAB hole
    let mut walker = VWalker::from_bytes(&[0x01]);
    walker.addr = Some(Address::new(ram(&manager), 0x40));
    let err = sym.resolve(&walker).expect_err("TAB hole");
    assert!(format!("{err}").contains("No corresponding entry in nametable"));
    // index 0 resolves (the C++ null constructor) and prints
    let mut walker = VWalker::from_bytes(&[0x00]);
    walker.addr = Some(Address::new(ram(&manager), 0x40));
    assert_eq!(sym.resolve(&walker).unwrap(), None);
    let mut s = String::new();
    sym.print(&mut s, &mut walker, &table).unwrap();
    assert_eq!(s, "x");
    // re-encode: the TAB entry becomes an attribute-less NAMETAB, i.e. the
    // stream a C++ encode of the post-decode state would produce
    let trans = TestTrans::new(&manager);
    let mut reenc = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut reenc);
        table.encode(&mut enc, &trans).unwrap();
    }
    let mut expected = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut expected);
        enc.open_element(&sla::ELEM_SYMBOL_TABLE);
        enc.write_signed_integer(&sla::ATTRIB_SCOPESIZE, 1);
        enc.write_signed_integer(&sla::ATTRIB_SYMBOLSIZE, 1);
        enc.open_element(&sla::ELEM_SCOPE);
        enc.write_unsigned_integer(&sla::ATTRIB_ID, 0);
        enc.write_unsigned_integer(&sla::ATTRIB_PARENT, 0);
        enc.close_element(&sla::ELEM_SCOPE);
        enc_head(&mut enc, &sla::ELEM_NAME_SYM_HEAD, b"names", 0);
        name_sym_content(&mut enc, None);
        enc.close_element(&sla::ELEM_SYMBOL_TABLE);
    }
    assert_eq!(reenc, expected);
}
