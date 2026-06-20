//! Verifier adversarial tests for item `w2-sleigh-semantics`
//! (semantics.{hh,cc} + pcodecompile.{hh,cc}).
//!
//! Targets chosen from the hunt list:
//! - the port's index-based re-engineering of the C++ pointer-aliased
//!   `force_size`/`propagateSize` machinery (multi-pass convergence),
//! - the `v_offset_plus` constant-handle shift at the 64-bit boundary,
//! - the asymmetric symsize/insize guards between `assignBitRange` and
//!   `createBitRange` (checks gated by `insize > 0` only in the latter),
//! - `adjustTruncation`'s int4 sign-extension into the uintb encode,
//! - `PcodeBuilder::build`'s exception partial state (labelbase NOT
//!   restored on error, as in C++),
//! - `fillinBuild`'s partial check-vector mutation on duplicate BUILD.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};
use kuna_num::opcodes::OpCode;
use kuna_sleigh::context::FixedHandle;
use kuna_sleigh::pcodecompile::{
    propagate_size, ExprTree, Location, PcodeCompile, PcodeCompileSymbol,
};
use kuna_sleigh::semantics::{
    ConstTpl, ConstType, ConstructTpl, OpTpl, PcodeBuilder, VField, VarnodeTpl, BUILD,
    LABELBUILD,
};
use kuna_sleigh::slghpatexpress::PatternExpressionContext;
use kuna_sleigh::slghsymbol::{ConstructorRef, SymbolWalker};

// -- fixtures ---------------------------------------------------------------

fn test_manager(big_endian: bool) -> AddrSpaceManager {
    let mut manager = AddrSpaceManager::new();
    manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    let ram = AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        big_endian,
        4,
        1,
        1,
        addrspace_flags::hasphysical,
        1,
        1,
    );
    manager.insert_space(Rc::new(ram)).unwrap();
    manager
        .insert_space(Rc::new(UniqueSpace::new(2, 0, big_endian)))
        .unwrap();
    manager
}

fn cspace(manager: &AddrSpaceManager) -> Rc<AddrSpace> {
    Rc::clone(manager.get_constant_space().unwrap())
}

fn ram(manager: &AddrSpaceManager) -> Rc<AddrSpace> {
    Rc::clone(manager.get_space_by_name("ram").unwrap())
}

fn uniq(manager: &AddrSpaceManager) -> Rc<AddrSpace> {
    Rc::clone(manager.get_space_by_name("unique").unwrap())
}

fn ram_vn(manager: &AddrSpaceManager, off: u64, size: u64) -> VarnodeTpl {
    VarnodeTpl::new(
        ConstTpl::new_space(ram(manager)),
        ConstTpl::new_real(ConstType::Real, off),
        ConstTpl::new_real(ConstType::Real, size),
    )
}

fn uniq_vn(manager: &AddrSpaceManager, off: u64, size: u64) -> VarnodeTpl {
    VarnodeTpl::new(
        ConstTpl::new_space(uniq(manager)),
        ConstTpl::new_real(ConstType::Real, off),
        ConstTpl::new_real(ConstType::Real, size),
    )
}

struct VerifyWalker {
    addr: Address,
    naddr: Address,
    const_space: Rc<AddrSpace>,
    cur_space: Rc<AddrSpace>,
    handles: Vec<FixedHandle>,
}

impl VerifyWalker {
    fn new(manager: &AddrSpaceManager) -> VerifyWalker {
        VerifyWalker {
            addr: Address::new(ram(manager), 0x1000),
            naddr: Address::new(ram(manager), 0x1004),
            const_space: cspace(manager),
            cur_space: ram(manager),
            handles: Vec::new(),
        }
    }
}

impl PatternExpressionContext for VerifyWalker {
    fn get_instruction_bytes(&self, _byteoff: i32, _numbytes: i32) -> KunaResult<u32> {
        Err(KunaError::sleigh("not in scope"))
    }
    fn get_context_bytes(&self, _byteoff: i32, _numbytes: i32) -> KunaResult<u32> {
        Err(KunaError::sleigh("not in scope"))
    }
    fn get_addr(&self) -> Address {
        self.addr.clone()
    }
    fn get_naddr(&self) -> Address {
        self.naddr.clone()
    }
    fn get_n2addr(&self) -> KunaResult<Address> {
        Err(KunaError::lowlevel("no next2"))
    }
    fn operand_value(&self, _index: i32, _table_id: u32, _ct_id: u32) -> KunaResult<i64> {
        Err(KunaError::sleigh("not in scope"))
    }
}

impl SymbolWalker for VerifyWalker {
    fn push_operand(&mut self, _i: i32) -> KunaResult<()> {
        Ok(())
    }
    fn pop_operand(&mut self) -> KunaResult<()> {
        Ok(())
    }
    fn get_constructor(&self) -> KunaResult<ConstructorRef> {
        Err(KunaError::sleigh("not in scope"))
    }
    fn get_fixed_handle(&self, i: i32) -> KunaResult<FixedHandle> {
        self.handles
            .get(i as usize)
            .cloned()
            .ok_or_else(|| KunaError::sleigh("no handle"))
    }
    fn get_const_space(&self) -> Rc<AddrSpace> {
        Rc::clone(&self.const_space)
    }
    fn get_cur_space(&self) -> Rc<AddrSpace> {
        Rc::clone(&self.cur_space)
    }
    fn get_dest_addr(&self) -> KunaResult<Address> {
        Err(KunaError::lowlevel("no flowdest"))
    }
    fn get_ref_addr(&self) -> KunaResult<Address> {
        Err(KunaError::lowlevel("no flowref"))
    }
    fn get_instruction_bits(&self, _startbit: i32, _size: i32) -> KunaResult<u32> {
        Err(KunaError::sleigh("not in scope"))
    }
    fn get_context_bits(&self, _startbit: i32, _size: i32) -> KunaResult<u32> {
        Err(KunaError::sleigh("not in scope"))
    }
}

struct VerifyCompile {
    defaultspace: Option<Rc<AddrSpace>>,
    constantspace: Option<Rc<AddrSpace>>,
    uniqspace: Option<Rc<AddrSpace>>,
    local_labelcount: u32,
    enforce_local_key: bool,
    tempbase: u32,
    errors: Vec<String>,
}

impl VerifyCompile {
    fn new(manager: &AddrSpaceManager) -> VerifyCompile {
        VerifyCompile {
            defaultspace: Some(ram(manager)),
            constantspace: Some(cspace(manager)),
            uniqspace: Some(uniq(manager)),
            local_labelcount: 0,
            enforce_local_key: false,
            tempbase: 0x1000,
            errors: Vec::new(),
        }
    }
}

impl PcodeCompile for VerifyCompile {
    fn get_default_space(&self) -> Option<Rc<AddrSpace>> {
        self.defaultspace.clone()
    }
    fn set_default_space(&mut self, spc: Rc<AddrSpace>) {
        self.defaultspace = Some(spc);
    }
    fn get_constant_space(&self) -> Option<Rc<AddrSpace>> {
        self.constantspace.clone()
    }
    fn set_constant_space(&mut self, spc: Rc<AddrSpace>) {
        self.constantspace = Some(spc);
    }
    fn get_unique_space(&self) -> Option<Rc<AddrSpace>> {
        self.uniqspace.clone()
    }
    fn set_unique_space(&mut self, spc: Rc<AddrSpace>) {
        self.uniqspace = Some(spc);
    }
    fn is_enforce_local_key(&self) -> bool {
        self.enforce_local_key
    }
    fn set_enforce_local_key(&mut self, val: bool) {
        self.enforce_local_key = val;
    }
    fn local_label_count(&self) -> u32 {
        self.local_labelcount
    }
    fn set_local_label_count(&mut self, val: u32) {
        self.local_labelcount = val;
    }
    fn allocate_temp(&mut self) -> u32 {
        let res = self.tempbase;
        self.tempbase += 16;
        res
    }
    fn add_symbol(&mut self, _sym: PcodeCompileSymbol) {}
    fn get_location(&self, _symbol_name: &[u8]) -> Option<Location> {
        None
    }
    fn report_error(&mut self, _loc: Option<&Location>, msg: &str) {
        self.errors.push(msg.to_string());
    }
    fn report_warning(&mut self, _loc: Option<&Location>, _msg: &str) {}
}

// -- T1: v_offset_plus constant-handle shift boundaries ----------------------

/// C++ semantics.cc:160-167: for a constant handle, `fix` returns
/// `val >> 8*(value_real>>16)`.  At byte count 7 (shift 56) the C++ shift is
/// defined; at byte count 8 (shift 64) C++ is UB which x86 resolves by
/// masking the count mod 64 (shift 0) — the port's `wshr` pins exactly that
/// resolution.
#[test]
fn verify_w2sem_offset_plus_const_shift_boundary() {
    let manager = test_manager(false);
    let mut walker = VerifyWalker::new(&manager);
    walker.handles.push(FixedHandle {
        space: Some(cspace(&manager)),
        size: 8,
        offset_space: None,
        offset_offset: 0xdead_beef_1234_5678,
        offset_size: 0,
        temp_space: None,
        temp_offset: 0,
    });

    // defined region: shift by 7 bytes = 56 bits
    let plus7 = ConstTpl::new_handle_plus(0, VField::VOffsetPlus, 0x0007_0000);
    assert_eq!(plus7.fix(&walker).unwrap(), 0xdead_beef_1234_5678u64 >> 56);

    // C++-UB region (shift 64): the port resolves as x86 hardware does
    // (count mod 64 -> shift 0, value unchanged)
    let plus8 = ConstTpl::new_handle_plus(0, VField::VOffsetPlus, 0x0008_0000);
    assert_eq!(plus8.fix(&walker).unwrap(), 0xdead_beef_1234_5678u64);

    // non-constant handle still adds only the low 16 bits of value_real
    let mut walker2 = VerifyWalker::new(&manager);
    walker2.handles.push(FixedHandle {
        space: Some(ram(&manager)),
        size: 8,
        offset_space: None,
        offset_offset: 0x100,
        offset_size: 0,
        temp_space: None,
        temp_offset: 0,
    });
    let plus = ConstTpl::new_handle_plus(0, VField::VOffsetPlus, 0x0007_0003);
    assert_eq!(plus.fix(&walker2).unwrap(), 0x103);
}

// -- T2: propagateSize multi-pass convergence --------------------------------

/// C++ pcodecompile.cc:265-293: the propagation loop must converge over
/// MULTIPLE passes — op0 is unresolvable on the first pass and only becomes
/// resolvable after op1's fillinZero propagates a size into op0's output
/// through the shared local temp.  This stresses the port's index-based
/// re-engineering of the C++ pointer-aliased force_size propagation.
#[test]
fn verify_w2sem_propagate_size_multipass_convergence() {
    let manager = test_manager(false);

    // op0: tmpA(0) = COPY(tmpB(0))      -- both zero size, unresolvable
    // op1: ram:8:4 = COPY(tmpA(0))      -- forces tmpA = 4 (incl. op0's out)
    // second pass: op0's out is now 4 -> matchSize fills tmpB = 4
    let mut op0 = OpTpl::new(OpCode::CPUI_COPY);
    op0.add_input(uniq_vn(&manager, 0x20, 0)); // tmpB
    op0.set_output(uniq_vn(&manager, 0x10, 0)); // tmpA
    let mut op1 = OpTpl::new(OpCode::CPUI_COPY);
    op1.add_input(uniq_vn(&manager, 0x10, 0)); // tmpA
    op1.set_output(ram_vn(&manager, 8, 4));

    let mut ct = ConstructTpl::new();
    assert!(ct.add_op(op0));
    assert!(ct.add_op(op1));
    assert!(propagate_size(&mut ct).unwrap());

    let ops = ct.get_opvec();
    // every instance of both temps got size 4
    assert_eq!(ops[0].get_out().unwrap().get_size().get_real(), 4);
    assert_eq!(ops[0].get_in(0).get_size().get_real(), 4);
    assert_eq!(ops[1].get_in(0).get_size().get_real(), 4);

    // and an unresolvable chain (no sized anchor anywhere) returns false
    // while still terminating
    let mut op2 = OpTpl::new(OpCode::CPUI_COPY);
    op2.add_input(uniq_vn(&manager, 0x40, 0));
    op2.set_output(uniq_vn(&manager, 0x30, 0));
    let mut op3 = OpTpl::new(OpCode::CPUI_COPY);
    op3.add_input(uniq_vn(&manager, 0x30, 0));
    op3.set_output(uniq_vn(&manager, 0x50, 0));
    let mut ct2 = ConstructTpl::new();
    assert!(ct2.add_op(op2));
    assert!(ct2.add_op(op3));
    assert!(!propagate_size(&mut ct2).unwrap());
}

// -- T3: assignBitRange vs createBitRange zero-symsize asymmetry -------------

/// C++ pcodecompile.cc:624-635 vs 712-724: in `assignBitRange` the
/// bad/superfluous checks run whenever the size type is real (symsize == 0
/// makes `bitoffset >= symsize*8` trivially true -> "Assigned bitrange is
/// bad"); in `createBitRange` ALL the checks sit inside `if (insize > 0)`,
/// so a zero-size varnode silently takes the truncation path instead.
#[test]
fn verify_w2sem_bitrange_zero_size_guard_asymmetry() {
    let manager = test_manager(false);
    let mut comp = VerifyCompile::new(&manager);

    // assignBitRange to a zero-size real varnode: error + rhs passthrough
    let vn = ram_vn(&manager, 0x100, 0);
    let rhs = ExprTree::new(ram_vn(&manager, 0x200, 1));
    let ops = comp.assign_bit_range(vn, 8, 8, rhs).unwrap();
    assert_eq!(comp.errors, vec!["Assigned bitrange is bad".to_string()]);
    // the rhs had no ops of its own; the error path passes them through
    assert!(ops.is_empty());

    // createBitRange on the same zero-size varnode: NO error; fullsz == 0
    // defeats buildTruncatedVarnode (returns None, not Err), insize == 0
    // skips the checks, and the result is SUBPIECE(vn, 1) sized 1
    let mut comp2 = VerifyCompile::new(&manager);
    let vn2 = ram_vn(&manager, 0x100, 0);
    let res = comp2.create_bit_range(vn2, b"reg", 8, 8).unwrap();
    assert!(comp2.errors.is_empty());
    assert_eq!(res.get_out().unwrap().get_size().get_real(), 1);
    let res_ops = ExprTree::to_vector(res);
    let opcodes: Vec<OpCode> = res_ops.iter().map(|op| op.get_opcode()).collect();
    assert_eq!(opcodes, vec![OpCode::CPUI_SUBPIECE]);
    // SUBPIECE's byte-shift constant is the truncshift (bitoffset/8 == 1)
    assert_eq!(res_ops[0].get_in(1).get_offset().get_real(), 1);
}

// -- T4: adjustTruncation int4 sign extension --------------------------------

/// C++ semantics.cc:497-521: `byteoffset = (int4)offset.getReal()` can go
/// negative; `uintb val = byteoffset` then SIGN-EXTENDS (defined C++
/// behavior), and the endian arm or-in uses the same conversion.  Pin the
/// exact encodes for byteoffset == -1.
#[test]
fn verify_w2sem_adjust_truncation_sign_extension() {
    // offset real u64::MAX -> (int4) == -1; numbytes 2; sz 8
    // bounds: 2 + (-1) = 1 > 8 is false -> proceeds
    let mk = || {
        VarnodeTpl::new(
            ConstTpl::new_real(ConstType::Real, 0),
            ConstTpl::new_handle_plus(1, VField::VOffsetPlus, u64::MAX),
            ConstTpl::new_real(ConstType::Real, 2),
        )
    };

    // little endian: val = (u64)(-1) << 16 | (u64)(-1) = all ones
    let mut le = mk();
    assert!(le.adjust_truncation(8, false));
    assert_eq!(le.get_offset().get_type(), ConstType::Handle);
    assert_eq!(le.get_offset().get_select(), VField::VOffsetPlus);
    assert_eq!(le.get_offset().get_handle_index(), 1);
    assert_eq!(le.get_offset().get_real(), u64::MAX);

    // big endian: val = (u64)(-1) << 16 | (u64)(8 - (2 + -1))
    //           = 0xFFFF_FFFF_FFFF_0000 | 7
    let mut be = mk();
    assert!(be.adjust_truncation(8, true));
    assert_eq!(be.get_offset().get_real(), 0xFFFF_FFFF_FFFF_0007);

    // in-bounds failure still returns false without touching the offset
    let mut oob = VarnodeTpl::new(
        ConstTpl::new_real(ConstType::Real, 0),
        ConstTpl::new_handle_plus(1, VField::VOffsetPlus, 7),
        ConstTpl::new_real(ConstType::Real, 4),
    );
    assert!(!oob.adjust_truncation(8, false));
    assert_eq!(oob.get_offset().get_select(), VField::VOffsetPlus);
    assert_eq!(oob.get_offset().get_real(), 7);
}

// -- T5: PcodeBuilder error partial state -------------------------------------

/// C++ semantics.cc:925-960: `PcodeBuilder::build` restores `labelbase`
/// only on the fall-through path; a throw from dump/appendBuild/... leaves
/// labelbase at the new base and labelcount grown (no RAII).  The port's
/// `?`-propagation must preserve exactly that partial state.
#[test]
fn verify_w2sem_pcodebuilder_error_partial_state() {
    struct FailingBuilder {
        labelbase: u32,
        labelcount: u32,
    }
    impl PcodeBuilder for FailingBuilder {
        fn get_label_base(&self) -> u32 {
            self.labelbase
        }
        fn set_label_base(&mut self, val: u32) {
            self.labelbase = val;
        }
        fn get_label_count(&self) -> u32 {
            self.labelcount
        }
        fn set_label_count(&mut self, val: u32) {
            self.labelcount = val;
        }
        fn dump(&mut self, _op: &OpTpl) -> KunaResult<()> {
            Err(KunaError::lowlevel("dump fails"))
        }
        fn append_build(&mut self, _bld: &OpTpl, _secnum: i32) -> KunaResult<()> {
            Ok(())
        }
        fn delay_slot(&mut self, _op: &OpTpl) -> KunaResult<()> {
            Ok(())
        }
        fn set_label(&mut self, _op: &OpTpl) -> KunaResult<()> {
            Ok(())
        }
        fn append_cross_build(&mut self, _bld: &OpTpl, _secnum: i32) -> KunaResult<()> {
            Ok(())
        }
    }

    let manager = test_manager(false);
    let mut ct = ConstructTpl::new();
    // one label, then a plain op that hits the failing dump
    let mut lab = OpTpl::new(LABELBUILD);
    lab.add_input(VarnodeTpl::new(
        ConstTpl::new_space(cspace(&manager)),
        ConstTpl::new_real(ConstType::Real, 0),
        ConstTpl::new_real(ConstType::Real, 4),
    ));
    assert!(ct.add_op(lab));
    let mut cp = OpTpl::new(OpCode::CPUI_COPY);
    cp.add_input(ram_vn(&manager, 0, 4));
    cp.set_output(ram_vn(&manager, 4, 4));
    assert!(ct.add_op(cp));

    let mut builder = FailingBuilder {
        labelbase: 3,
        labelcount: 3,
    };
    assert!(builder.build(Some(&ct), -1).is_err());
    // C++ partial state: labelbase moved to the entry labelcount (3) and
    // NOT restored; labelcount grew by the template's labels (1)
    assert_eq!(builder.get_label_base(), 3);
    assert_eq!(builder.get_label_count(), 4);

    // distinguishable case: start with labelbase != labelcount
    let mut builder2 = FailingBuilder {
        labelbase: 1,
        labelcount: 5,
    };
    assert!(builder2.build(Some(&ct), -1).is_err());
    assert_eq!(builder2.get_label_base(), 5); // new base, not restored to 1
    assert_eq!(builder2.get_label_count(), 6);
}

// -- T6: fillinBuild partial check-vector state --------------------------------

/// C++ semantics.cc:763-791: on a duplicate BUILD the scan returns the
/// stored check value immediately, leaving the marks made so far (partial
/// mutation visible to the caller, who owns the vector).
#[test]
fn verify_w2sem_fillin_build_partial_marks_on_duplicate() {
    let manager = test_manager(false);
    let build_op = |index: u64| {
        let mut op = OpTpl::new(BUILD);
        op.add_input(VarnodeTpl::new(
            ConstTpl::new_space(cspace(&manager)),
            ConstTpl::new_real(ConstType::Real, index),
            ConstTpl::new_real(ConstType::Real, 4),
        ));
        op
    };
    let mut ct = ConstructTpl::new();
    assert!(ct.add_op(build_op(0)));
    assert!(ct.add_op(build_op(1)));
    assert!(ct.add_op(build_op(1))); // duplicate
    let mut check = vec![0i32; 3];
    assert_eq!(ct.fillin_build(&mut check, &cspace(&manager)), 1);
    // marks made before the duplicate stay; index 2 untouched; no BUILD
    // ops were prepended (the second loop never ran)
    assert_eq!(check, vec![1, 1, 0]);
    assert_eq!(ct.get_opvec().len(), 3);
}
