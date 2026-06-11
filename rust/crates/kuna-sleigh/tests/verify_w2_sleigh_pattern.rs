//! Verifier adversarial tests for item `w2-sleigh-pattern` (round 1).
//!
//! Written by the independent verifier against the C++ oracle semantics of
//! `decompiler/cpp/slghpattern.{hh,cc}` / `slghpatexpress.{hh,cc}`.  Each
//! test targets a hunt-list spot the port could plausibly have gotten wrong
//! while still passing the porter's own tests; the expected values are
//! hand-derived C++ traces (anchors cited per test).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{Decoder, Encoder, PackedDecode, PackedEncode};
use kuna_base::space::{AddrSpaceManager, ConstantSpace};
use kuna_base::types::Wrap;

use kuna_sleigh::slghpatexpress::PatternExpressionContext;
use kuna_sleigh::slghpattern::{
    sla, ContextPattern, DisjointPattern, InstructionPattern, OrPattern, Pattern, PatternBlock,
};

// -- harness -----------------------------------------------------------------

/// Synthetic walker: instruction bytes packed big-endian with a bounds error
/// past the buffer (the C++ `ParserContext::getInstructionBytes` BadDataError
/// analog); context served from 32-bit words.
struct ByteWalker {
    instr: Vec<u8>,
}

impl ByteWalker {
    fn instr(bytes: &[u8]) -> ByteWalker {
        ByteWalker {
            instr: bytes.to_vec(),
        }
    }
}

impl PatternExpressionContext for ByteWalker {
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

    fn get_context_bytes(&self, _byteoff: i32, _numbytes: i32) -> KunaResult<u32> {
        Ok(0)
    }

    fn get_addr(&self) -> Address {
        panic!("verify_w2_sleigh_pattern never reads addresses")
    }

    fn get_naddr(&self) -> Address {
        panic!("verify_w2_sleigh_pattern never reads addresses")
    }

    fn get_n2addr(&self) -> KunaResult<Address> {
        Err(KunaError::lowlevel("inst_next2 not available"))
    }

    fn operand_value(&self, _index: i32, _table_id: u32, _ct_id: u32) -> KunaResult<i64> {
        Err(KunaError::sleigh("no operands in verify tests"))
    }
}

fn test_manager() -> AddrSpaceManager {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manager
}

/// instruction pattern requiring `byte[pos] == val` (same shape the C++
/// compiler emits through TokenPattern::buildSingle for an 8-bit field).
fn ip_byte(pos: i32, val: u8) -> InstructionPattern {
    InstructionPattern::new(PatternBlock::new(
        (pos / 4) * 4,
        0xff00_0000u32 >> ((pos % 4) * 8),
        (u32::from(val) << 24) >> ((pos % 4) * 8),
    ))
}

/// Two-word block: mask 0xFFFFFFFF_FFFFFFFF over bytes [0,8), value words
/// `w0`/`w1` (built the C++ way, by intersecting two single-word blocks).
fn two_word_block(w0: u32, w1: u32) -> PatternBlock {
    let a = PatternBlock::new(0, 0xffff_ffff, w0);
    let b = PatternBlock::new(4, 0xffff_ffff, w1);
    let res = a.intersect(&b);
    assert_eq!(res.get_length(), 8);
    res
}

// -- T1: the unsigned-division transcription in getMask/getValue --------------
//
// C++ slghpattern.cc:405-462: `startbit/(8*sizeof(uintm))` divides an int4 by
// a size_t, so a negative startbit converts to a huge 64-bit unsigned before
// the divide, and the quotient/remainder truncate back to int4.  The oracle
// values below are hand-computed from that exact arithmetic.
#[test]
fn verify_w2pat_getmask_getvalue_negative_startbit_unsigned_division() {
    let blk = two_word_block(0xaabb_ccdd, 0x1122_3344);

    // straddling positive read: word0<<8 | word1>>24
    assert_eq!(blk.get_value(8, 32), 0xbbcc_dd11);

    // startbit=-8: ustart=2^64-8 -> wordnum1=-1 (out of range, contributes
    // 0), shift=24, wordnum2=0 -> res = valvec[0] >> (32-24)
    assert_eq!(blk.get_value(-8, 32), 0x00aa_bbcc);

    // startbit=-16: shift=16, second word contributes valvec[0] >> 16,
    // then res >>= (32-32)=0
    assert_eq!(blk.get_value(-16, 32), 0x0000_aabb);

    // same path on the mask side, with a final res >>= (32-16)
    assert_eq!(blk.get_mask(-8, 16), 0x0000_00ff);

    // startbit=-40: wordnum1=-2, wordnum2=-1 -> both out of range -> 0
    assert_eq!(blk.get_value(-40, 32), 0);
    assert_eq!(blk.get_mask(-40, 32), 0);
}

// -- T2/T3: the OrPattern::doOr receiver-mutation quirk -----------------------
//
// C++ slghpattern.cc:931-956: OrPattern::doOr is declared const but, when
// sa < 0, shifts the elements of the RECEIVER's own orlist while the result
// keeps the unshifted clones; when sa > 0 it shifts EVERY element of the
// result (the receiver's clones included).  Delegation from
// InstructionPattern::doOr (slghpattern.cc:591-609) negates sa, so an
// outer-positive sa reaches the OrPattern as a negative one and mutates what
// the caller passed as `b`.

#[test]
fn verify_w2pat_or_door_delegated_negative_sa_mutates_or_operand() {
    // C++ trace: instr->doOr(or, 1) -> or->doOr(instr, -1):
    //   newlist = unshifted clones of or's elements
    //   sa<0    -> or's OWN elements shift by +1   (receiver mutation)
    //   then instr's clone is appended; sa>0 branch not taken
    let mut a = Pattern::Disjoint(DisjointPattern::Instruction(ip_byte(0, 0x33)));
    let mut b = Pattern::Or(OrPattern::new(
        DisjointPattern::Instruction(ip_byte(0, 0x11)),
        DisjointPattern::Instruction(ip_byte(0, 0x22)),
    ));
    let res = a.do_or(&mut b, 1);

    // result: all three requirements still at byte 0, UNSHIFTED
    assert_eq!(res.num_disjoint(), 3);
    assert!(res.is_match(&ByteWalker::instr(&[0x11, 0, 0, 0, 0])).unwrap());
    assert!(res.is_match(&ByteWalker::instr(&[0x22, 0, 0, 0, 0])).unwrap());
    assert!(res.is_match(&ByteWalker::instr(&[0x33, 0, 0, 0, 0])).unwrap());
    assert!(!res.is_match(&ByteWalker::instr(&[0, 0x11, 0, 0, 0])).unwrap());

    // the Or OPERAND was mutated through the delegation: its byte-0
    // requirements moved to byte 1
    assert!(b.is_match(&ByteWalker::instr(&[0, 0x11, 0, 0, 0])).unwrap());
    assert!(b.is_match(&ByteWalker::instr(&[0, 0x22, 0, 0, 0])).unwrap());
    assert!(!b.is_match(&ByteWalker::instr(&[0x11, 0, 0, 0, 0])).unwrap());

    // the instruction receiver is untouched
    assert!(a.is_match(&ByteWalker::instr(&[0x33, 0, 0, 0, 0])).unwrap());
}

#[test]
fn verify_w2pat_or_door_positive_sa_shifts_receiver_clones_too() {
    // C++ trace: or->doOr(instr, 1): newlist = or clones + instr clone, then
    // the sa>0 loop shifts EVERY newlist element by 1 -- the receiver's
    // clones included (only b "should" shift); the receiver itself is not
    // mutated.
    let mut a = Pattern::Or(OrPattern::new(
        DisjointPattern::Instruction(ip_byte(0, 0x11)),
        DisjointPattern::Instruction(ip_byte(0, 0x22)),
    ));
    let mut b = Pattern::Disjoint(DisjointPattern::Instruction(ip_byte(0, 0x33)));
    let res = a.do_or(&mut b, 1);

    assert_eq!(res.num_disjoint(), 3);
    // everything in the result moved to byte 1
    assert!(res.is_match(&ByteWalker::instr(&[0, 0x11, 0, 0, 0])).unwrap());
    assert!(res.is_match(&ByteWalker::instr(&[0, 0x22, 0, 0, 0])).unwrap());
    assert!(res.is_match(&ByteWalker::instr(&[0, 0x33, 0, 0, 0])).unwrap());
    assert!(!res.is_match(&ByteWalker::instr(&[0x11, 0, 0, 0, 0])).unwrap());
    assert!(!res.is_match(&ByteWalker::instr(&[0x33, 0, 0, 0, 0])).unwrap());

    // the receiver is NOT mutated this time
    assert!(a.is_match(&ByteWalker::instr(&[0x11, 0, 0, 0, 0])).unwrap());
    assert!(!a.is_match(&ByteWalker::instr(&[0, 0x11, 0, 0, 0])).unwrap());
}

// -- T4: normalize()'s unaligned slide-up through decode ----------------------
//
// C++ slghpattern.cc:156-227 + 505-521: decode ingests raw words then calls
// normalize(), which (a) strips leading zero words bumping offset by 4,
// (b) slides the remaining words up by the leading zero BYTES of word0
// (suboff), and (c) trims trailing zero bytes out of nonzerosize.  Oracle:
// off=0/nonzero=8/words [0,0][0x00FF1100,0x00AB0100] -> offset 5,
// nonzerosize 2 (mask bytes 5,6 = FF,11; value bytes AB,01), length 7.
#[test]
fn verify_w2pat_decode_normalize_slide_up_unaligned_mask() {
    let manager = test_manager();
    let mut buf = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf);
        enc.open_element(&sla::ELEM_PAT_BLOCK);
        enc.write_signed_integer(&sla::ATTRIB_OFF, 0);
        enc.write_signed_integer(&sla::ATTRIB_NONZERO, 8);
        enc.open_element(&sla::ELEM_MASK_WORD);
        enc.write_unsigned_integer(&sla::ATTRIB_MASK, 0);
        enc.write_unsigned_integer(&sla::ATTRIB_VAL, 0);
        enc.close_element(&sla::ELEM_MASK_WORD);
        enc.open_element(&sla::ELEM_MASK_WORD);
        enc.write_unsigned_integer(&sla::ATTRIB_MASK, 0x00ff_1100);
        enc.write_unsigned_integer(&sla::ATTRIB_VAL, 0x00ab_0100);
        enc.close_element(&sla::ELEM_MASK_WORD);
        enc.close_element(&sla::ELEM_PAT_BLOCK);
    }
    let mut dec = PackedDecode::new(&manager);
    dec.ingest_stream(&buf).unwrap();
    let blk = PatternBlock::decode(&mut dec).unwrap();

    assert_eq!(blk.get_length(), 7); // offset 5 + nonzerosize 2
    assert_eq!(blk.get_mask(40, 8), 0xff); // byte 5
    assert_eq!(blk.get_value(40, 8), 0xab);
    assert_eq!(blk.get_mask(48, 8), 0x11); // byte 6
    assert_eq!(blk.get_value(48, 8), 0x01);
    assert_eq!(blk.get_mask(32, 8), 0); // byte 4 fell out of the mask
    assert!(!blk.always_true());
    assert!(!blk.always_false());

    // re-encode of the NORMALIZED block decodes back identical (stable form)
    let mut buf2 = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf2);
        blk.encode(&mut enc);
    }
    let mut dec2 = PackedDecode::new(&manager);
    dec2.ingest_stream(&buf2).unwrap();
    let blk2 = PatternBlock::decode(&mut dec2).unwrap();
    assert!(blk2.identical(&blk));
    let mut buf3 = Vec::new();
    {
        let mut enc = PackedEncode::new(&mut buf3);
        blk2.encode(&mut enc);
    }
    assert_eq!(buf3, buf2);
}

// -- T5: multi-word intersect conflict / commonSubPattern ---------------------
//
// C++ slghpattern.cc:295-355: intersect detects an impossible pattern per
// 32-bit word (commonmask disagreement -> nonzerosize=-1), and
// commonSubPattern keeps a mask bit only where both masks have it AND the
// values agree.  Both checked in the SECOND word, where a word-indexing slip
// would go unnoticed by single-word tests.
#[test]
fn verify_w2pat_intersect_conflict_and_common_sub_in_second_word() {
    let a = two_word_block(0xaabb_ccdd, 0x1122_3344);

    // conflict only in word 1 (0x11... vs 0x22... under mask 0xFF000000)
    let clash = PatternBlock::new(4, 0xff00_0000, 0x2200_0000);
    assert!(a.intersect(&clash).always_false());
    // ...and the same constraint with the matching value intersects fine
    let agree = PatternBlock::new(4, 0xff00_0000, 0x1100_0000);
    let merged = a.intersect(&agree);
    assert!(!merged.always_false());
    assert_eq!(merged.get_value(32, 32), 0x1122_3344);

    // commonSubPattern: values differ only in bit 0x2 of word 1's low byte
    let b = two_word_block(0xaabb_ccdd, 0x1122_3346);
    let common = a.common_sub_pattern(&b);
    assert_eq!(common.get_length(), 8);
    assert_eq!(common.get_mask(0, 32), 0xffff_ffff);
    assert_eq!(common.get_value(0, 32), 0xaabb_ccdd);
    assert_eq!(common.get_mask(32, 32), 0xffff_fffd);
    assert_eq!(common.get_value(32, 32), 0x1122_3344);
}

// -- T6: cross-stream identical/specializes + the 4-byte over-read ------------
//
// C++ slghpattern.cc:48-103: DisjointPattern::identical/specializes treat an
// absent block (null getBlock) as satisfied only by an always-true
// counterpart -- so InstructionPattern(true) and ContextPattern(true) are
// `identical` even though they are different classes.  And
// PatternBlock::isInstructionMatch (slghpattern.cc:464-475) always requests
// sizeof(uintm)=4 bytes per mask word, so a 2-byte pattern errors on a
// 2-byte instruction buffer (the C++ BadDataError over-read quirk).
#[test]
fn verify_w2pat_identical_cross_type_specializes_asymmetry_and_overread() {
    let itrue = DisjointPattern::Instruction(InstructionPattern::new_always(true));
    let ctrue = DisjointPattern::Context(ContextPattern::new(PatternBlock::new_always(true)));
    assert!(itrue.identical(&ctrue));
    assert!(ctrue.identical(&itrue));
    assert!(itrue.specializes(&ctrue));
    assert!(ctrue.specializes(&itrue));
    // but a constrained context side breaks it in both directions
    let creq = DisjointPattern::Context(ContextPattern::new(PatternBlock::new(
        0,
        0xf000_0000,
        0x3000_0000,
    )));
    assert!(!itrue.identical(&creq));
    assert!(!creq.identical(&itrue));
    assert!(!itrue.specializes(&creq));

    // specializes asymmetry across lengths: 8-byte fine vs 1-byte coarse
    let fine = two_word_block(0xaabb_ccdd, 0x1122_3344);
    let coarse = PatternBlock::new(0, 0xff00_0000, 0xaa00_0000);
    assert!(fine.specializes(&coarse));
    assert!(!coarse.specializes(&fine));

    // over-read: a 2-byte pattern still fetches 4 instruction bytes
    let two_byte = PatternBlock::new(0, 0xffff_0000, 0xabcd_0000);
    assert_eq!(two_byte.get_length(), 2);
    assert!(two_byte
        .is_instruction_match(&ByteWalker::instr(&[0xab, 0xcd]))
        .is_err());
    assert!(two_byte
        .is_instruction_match(&ByteWalker::instr(&[0xab, 0xcd, 0, 0]))
        .unwrap());
}
