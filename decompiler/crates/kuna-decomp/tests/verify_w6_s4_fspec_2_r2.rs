//! Round-2 adversarial verification tests for item `w6-s4-fspec-2`
//! (`decompiler/cpp/fspec.cc` ~2268-4928: ProtoModel / ScoreProtoModel /
//! ProtoModelMerged / ProtoParameter-ProtoStore / FuncProto core).
//!
//! Written by the INDEPENDENT verifier in round 2, AFTER the F1 repair
//! (commit 7af2e13: the ParamUnassignedError-only catch is now matched by
//! `Err(KunaError::ParamUnassigned { .. })`, with `Err(e) => return Err(e)` for
//! everything else, and `unassigned_err` builds the `ParamUnassigned` variant).
//!
//! These target the spots re-derived as most fragile for this round:
//!
//!   - **F1 must not OVER-tighten.**  The repair must still RECOVER a genuine
//!     `ParamUnassignedError` (the catch must fire when the error really is
//!     ParamUnassigned), not just stop swallowing the non-ParamUnassigned case.
//!     A true input-side ParamUnassigned (more intypes than the model has
//!     entries -> fspec.cc:814) must set `error_inputparam` and return Ok; a
//!     true output-side ParamUnassigned under `ignore_output_error=true`
//!     (fspec.cc:2441) must still fall back to a void return.
//!   - **doScore penaltyfinal saturation** (fspec.cc:2762): a hole at slot index
//!     four-or-more uses `penaltyfinal == 3`, not the penalty[] table (whose only
//!     valid indices are 0..3).  This is the `nextfree < 4` boundary; an
//!     off-by-one here would index past `penalty[4]`.
//!   - **resolveExtraPop wrap** (fspec.cc:3991-3992): `(int4)addr.getOffset() +
//!     getSize()` then `(cur+3)&0x0ffffffc`.  C++ wraps; the port's `cur+3` is a
//!     plain i32 add.  A spacebase param with a near-i32::MAX offset probes
//!     whether that add panics in debug (F2 reachability).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager};
use kuna_base::types::{int4, uint4};

use kuna_decomp::dtype::{type_class, type_metatype, Datatype, TypeFactory};
use kuna_decomp::fspec::{
    parameter_pieces_flags, FuncProto, ParamEntry, ParameterPieces, ProtoModel, PrototypePieces,
    ScoreProtoModel,
};

// --- shared builders (mirror the porter's harness) --------------------------

fn reg_space_le() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        4,
        1,
        3,
        0,
        0,
        0,
    ))
}

/// A spacebase (stack) space, so `resolveExtraPop` treats its params as on the
/// stack (`getType() == IPTR_SPACEBASE`).
fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_SPACEBASE,
        "stack",
        false,
        8,
        1,
        4,
        0,
        0,
        0,
    ))
}

fn addr(spc: &Rc<AddrSpace>, off: u64) -> Address {
    Address::new(Rc::clone(spc), off)
}

fn excl_entry(
    grp: int4,
    space: &Rc<AddrSpace>,
    base: u64,
    size: int4,
    prev: &[ParamEntry],
    mgr: &AddrSpaceManager,
) -> ParamEntry {
    ParamEntry::seed(
        grp,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(space),
        base,
        size,
        1,
        0,
        0,
        true,
        false,
        prev,
        mgr,
    )
    .expect("seed exclusion entry")
}

fn int4_type() -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(4, 4, type_metatype::TYPE_INT))
}
fn int8_type() -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(8, 8, type_metatype::TYPE_INT))
}
fn void_type() -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(0, 1, type_metatype::TYPE_VOID))
}

/// Standard model with ONE input entry (group 0 at 0x10) and ONE 4-byte output
/// entry at 0x10.  A second int input has no entry -> input-side
/// ParamUnassignedError (fspec.cc:814).
fn one_in_one_out_model(mgr: &AddrSpaceManager, reg: &Rc<AddrSpace>) -> ProtoModel {
    let mut model = ProtoModel::new(mgr);
    model.build_param_list("standard").unwrap();
    model.set_name("__cdecl");
    {
        let input = model.input_mut();
        input.push_entry(excl_entry(0, reg, 0x10, 4, &[], mgr));
        input.finish_decode();
    }
    {
        let output = model.output_mut();
        output.push_entry(excl_entry(0, reg, 0x10, 4, &[], mgr));
        output.finish_decode();
    }
    model
}

/// Minimal TypeFactory answering only `get_type_void`.
struct VoidTypeFactory;
macro_rules! unreached {
    () => {
        panic!("TypeFactory method should not be reached in this test")
    };
}
#[allow(unused_variables)]
impl TypeFactory for VoidTypeFactory {
    fn get_size_of_int(&self) -> int4 {
        unreached!()
    }
    fn get_size_of_long(&self) -> int4 {
        unreached!()
    }
    fn get_size_of_char(&self) -> int4 {
        unreached!()
    }
    fn get_size_of_wchar(&self) -> int4 {
        unreached!()
    }
    fn get_size_of_pointer(&self) -> int4 {
        unreached!()
    }
    fn get_size_of_alt_pointer(&self) -> int4 {
        unreached!()
    }
    fn get_alignment(&self, _size: uint4) -> KunaResult<int4> {
        unreached!()
    }
    fn get_primitive_align_size(&self, _size: uint4) -> KunaResult<int4> {
        unreached!()
    }
    fn get_type_void(&self) -> KunaResult<Rc<Datatype>> {
        Ok(void_type())
    }
    fn get_base_no_char(&self, _s: int4, _m: type_metatype) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_base(&self, _s: int4, _m: type_metatype) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_base_named(&self, _s: int4, _m: type_metatype, _n: &str) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_char(&self, _s: int4) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_code(&self) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_pointer_strip_array(
        &self,
        _s: int4,
        _pt: Rc<Datatype>,
        _ws: uint4,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_pointer(&self, _s: int4, _pt: Rc<Datatype>, _ws: uint4) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_pointer_named(
        &self,
        _s: int4,
        _pt: Rc<Datatype>,
        _ws: uint4,
        _n: &str,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn resize_pointer(&self, _ptr: Rc<Datatype>, _new_size: int4) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_pointer_rel(
        &self,
        _parent_ptr: Rc<Datatype>,
        _ptr_to: Rc<Datatype>,
        _off: int4,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_pointer_rel_full(
        &self,
        _sz: int4,
        _parent: Rc<Datatype>,
        _ptr_to: Rc<Datatype>,
        _ws: int4,
        _off: int4,
        _nm: &str,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_pointer_with_space(
        &self,
        _ptr_to: Rc<Datatype>,
        _spc: Rc<AddrSpace>,
        _nm: &str,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_array(&self, _as_: int4, _ao: Rc<Datatype>) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_struct(&self, _n: &str) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_partial_struct(
        &self,
        _contain: Rc<Datatype>,
        _off: int4,
        _sz: int4,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_union(&self, _n: &str) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_partial_union(
        &self,
        _contain: Rc<Datatype>,
        _off: int4,
        _sz: int4,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_partial_enum(
        &self,
        _contain: Rc<Datatype>,
        _off: int4,
        _sz: int4,
    ) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_enum(&self, _n: &str) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_type_spacebase(&self, _id: Rc<AddrSpace>, _addr: &Address) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn resize_integer(&self, _ct: Rc<Datatype>, _new_size: int4) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
    fn get_exact_piece(
        &self,
        _ct: Rc<Datatype>,
        _offset: int4,
        _size: int4,
    ) -> KunaResult<Option<Rc<Datatype>>> {
        unreached!()
    }
    fn find_by_name(&self, _n: &str) -> KunaResult<Option<Rc<Datatype>>> {
        unreached!()
    }
    fn concretize(&self, _ct: Rc<Datatype>) -> KunaResult<Rc<Datatype>> {
        unreached!()
    }
}

// ===========================================================================
// F1 (post-repair): the ParamUnassigned-only catch must still RECOVER a
// genuine ParamUnassignedError — the repair must not have over-tightened it.
// ===========================================================================

/// `updateAllTypes` (fspec.cc:4199, `ignoreOutputError=false`).  Two int inputs
/// but the model has only ONE input entry: the second input cannot be assigned,
/// so `ParamListStandard::assignMap` throws `ParamUnassignedError` (fspec.cc:814)
/// — a GENUINE ParamUnassigned, NOT the STUB Lowlevel.  C++ `catch
/// (ParamUnassignedError&)` sets `error_inputparam`; the repaired port matches
/// `Err(KunaError::ParamUnassigned { .. })` and must do the same (Ok + flag),
/// proving the discriminator fires for the real ParamUnassigned case.
#[test]
fn update_all_types_genuine_param_unassigned_sets_flag_w6s4f1_r2() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = Rc::new(one_in_one_out_model(&mgr, &reg));
    let tf = VoidTypeFactory;

    let mut fp = FuncProto::new();
    fp.set_internal(Rc::clone(&model), void_type());

    let proto = PrototypePieces {
        name: "g".to_string(),
        outtype: Some(int4_type()),
        // two int inputs, but only one input entry exists -> 2nd is unassignable
        intypes: vec![int4_type(), int4_type()],
        innames: vec!["a".into(), "b".into()],
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
    };
    let r = fp.update_all_types(&proto, &tf, &mgr);
    assert!(
        r.is_ok(),
        "a genuine ParamUnassignedError must be RECOVERED (Ok), not propagated (got {:?})",
        r
    );
    assert!(
        fp.has_input_errors(),
        "the recovered ParamUnassignedError must set error_inputparam (C++ fspec.cc:4226)"
    );
}

/// `assignParameterStorage(..., ignoreOutputError=true)` (fspec.cc:2437).  The
/// output type is too big for the 4-byte output register; with no model rules
/// the StandardOut output assign reaches the hidden-return fallback.  In the
/// port that fallback is the STUB(W4) Lowlevel error — which, after the F1
/// repair, must PROPAGATE (it is NOT ParamUnassigned).  This is the
/// complementary direction of the F1 fix: the catch fires ONLY for
/// ParamUnassigned, and a non-ParamUnassigned error from the output assign
/// escapes even on the ignore-output path.  (Mirrors the porter's
/// `..._propagates_nonparamunassigned` test from an independent build.)
#[test]
fn assign_parameter_storage_ignore_output_only_catches_param_unassigned_w6s4f1_r2() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let model = one_in_one_out_model(&mgr, &reg);
    let tf = VoidTypeFactory;

    let proto = PrototypePieces {
        name: "f".to_string(),
        outtype: Some(int8_type()), // too big for the 4-byte out reg -> hidden-ret seam
        intypes: vec![],
        innames: vec![],
        first_var_arg_slot: -1,
        output_storage: None,
        input_storage: Vec::new(),
    };
    let mut res: Vec<ParameterPieces> = Vec::new();
    let r = model.assign_parameter_storage(&proto, &mut res, true, &tf, &mgr);
    // The STUB error is a Lowlevel, NOT ParamUnassigned -> must escape the
    // ParamUnassigned-only catch even with ignore_output_error = true.
    assert!(
        matches!(r, Err(KunaError::Lowlevel { .. })),
        "ignore_output_error path must still PROPAGATE a non-ParamUnassigned \
         (Lowlevel) error from the output assign (got {:?})",
        r
    );
    assert!(
        !matches!(r, Err(KunaError::ParamUnassigned { .. })),
        "the STUB error must not be misclassified as ParamUnassigned"
    );
}

// ===========================================================================
// doScore: the penaltyfinal saturation boundary (nextfree < 4).
// ===========================================================================

/// `doScore` (fspec.cc:2758-2778): the penalty table has exactly 4 entries
/// (16,10,7,5); a hole at slot index >= 4 must use `penaltyfinal == 3`, NOT an
/// out-of-bounds `penalty[4]`.  Build a model with FIVE single-slot input
/// entries (slots 0..4) and present ONLY the slot-5... no: present only the
/// slot-4 parameter so slots 0,1,2,3 (table) AND no slot >=4 contribute.  Then
/// a SECOND scenario presents only a slot-5 param so slots 0..4 are holes:
/// 16+10+7+5 (table) + 3 (penaltyfinal for slot 4) == 41.  This exercises the
/// `nextfree < 4 ? penalty[nextfree] : penaltyfinal` branch and the index bound.
#[test]
fn do_score_penaltyfinal_used_past_slot_index_4_w6s4_r2() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();

    // Build a 6-entry input model: groups 0..5 at 0x10,0x20,...,0x60.
    let mut model = ProtoModel::new(&mgr);
    model.build_param_list("standard").unwrap();
    model.set_name("__cdecl");
    {
        let input = model.input_mut();
        input.push_entry(excl_entry(0, &reg, 0x10, 4, &[], &mgr));
        let e = excl_entry(1, &reg, 0x20, 4, input.get_entry(), &mgr);
        input.push_entry(e);
        let e = excl_entry(2, &reg, 0x30, 4, input.get_entry(), &mgr);
        input.push_entry(e);
        let e = excl_entry(3, &reg, 0x40, 4, input.get_entry(), &mgr);
        input.push_entry(e);
        let e = excl_entry(4, &reg, 0x50, 4, input.get_entry(), &mgr);
        input.push_entry(e);
        let e = excl_entry(5, &reg, 0x60, 4, input.get_entry(), &mgr);
        input.push_entry(e);
        input.finish_decode();
    }
    {
        let output = model.output_mut();
        output.push_entry(excl_entry(0, &reg, 0x10, 4, &[], &mgr));
        output.finish_decode();
    }

    // Present ONLY the slot-5 parameter (0x60): slots 0,1,2,3 use the table
    // (16+10+7+5 = 38), slot 4 is past the table -> penaltyfinal (3).  Total 41.
    let mut score = ScoreProtoModel::new(true, 1);
    score.add_parameter(&model, &addr(&reg, 0x60), 4);
    score.do_score();
    assert_eq!(score.get_num_mismatch(), 0, "0x60 is a possible param (slot 5)");
    assert_eq!(
        score.get_score(),
        16 + 10 + 7 + 5 + 3,
        "holes 0..3 use penalty[], hole at slot 4 uses penaltyfinal=3 (nextfree<4 boundary)"
    );
}

// ===========================================================================
// resolveExtraPop wrap probe (F2): `(cur+3)` is a plain i32 add in the port.
// ===========================================================================

/// `resolveExtraPop` (fspec.cc:3991-3992): for a stack (IPTR_SPACEBASE) param,
/// `cur = (int4)addr.getOffset() + getSize()`, then `cur = (cur+3)&0x0ffffffc`.
/// C++ wraps the `+3` silently.  The port's first add is `wrapping_add`, but the
/// `cur + 3` is a plain i32 add.  This drives `cur` to exactly `i32::MAX` (so
/// `cur + 3` would overflow) to determine whether the port panics where C++
/// wraps (F2 reachability).
///
/// We choose offset so that `(offset as i32).wrapping_add(4) == i32::MAX`:
///   offset as i32 = i32::MAX - 4 = 0x7fff_fffb.
/// Then `cur + 3` = i32::MAX.wrapping... in C++; in the port a plain add of
/// i32::MAX + 3 overflows and panics in debug.  If this test does NOT panic the
/// add is safe for this input; if it panics, F2 is reachable.
#[test]
fn resolve_extra_pop_near_i32_max_offset_w6s4f2_r2() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let stack = stack_space();

    let mut fp = FuncProto::new();
    // resolveExtraPop only reads locked input params; install an internal store
    // via a trivial model (the model itself is not consulted here).
    let model = Rc::new(one_in_one_out_model(&mgr, &reg));
    fp.set_internal(Rc::clone(&model), void_type());
    let off: u64 = 0x7fff_fffb; // (as i32) + 4 == i32::MAX
    let piece = ParameterPieces {
        addr: addr(&stack, off),
        type_: Some(int4_type()),
        flags: parameter_pieces_flags::TYPELOCK,
    };
    fp.set_param(0, "p", &piece);
    fp.set_input_lock(true);
    assert!(fp.is_input_locked(), "input must be locked for resolveExtraPop");
    assert!(!fp.is_dotdotdot());

    // C++ computes `cur = (cur+3)&0x0ffffffc` with SILENT WRAP and never panics.
    // The port's `cur + 3` is a plain i32 add: in a DEBUG build it panics with
    // "attempt to add with overflow" exactly here (fspec.rs:5074).  In a RELEASE
    // build (overflow checks off) it wraps and matches C++.  This test PINS the
    // debug-build divergence (LOSS-086): we assert that `resolve_extra_pop`
    // panics on this crafted near-i32::MAX stack offset, which is the observable
    // symptom of the missing `wrapping_add`.  If a future fix replaces `cur + 3`
    // with `cur.wrapping_add(3)`, this test will need to be updated to assert the
    // wrapped result instead (the loss would then be closed).
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {})); // silence the expected-overflow panic
    let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        fp.resolve_extra_pop();
    }))
    .is_err();
    std::panic::set_hook(prev_hook);
    assert!(
        panicked,
        "DEBUG build: resolveExtraPop's non-wrapping `cur + 3` overflows on a \
         near-i32::MAX stack offset where C++ wraps (LOSS-086, F2).  If this no \
         longer panics the missing wrapping_add was fixed -- update this pin."
    );
}

/// Control: a normal small stack offset resolves extrapop deterministically and
/// matches the C++ alignment math `(off+size+3)&0x0ffffffc`.  off=0x14 (20),
/// size=4 -> cur=24 -> (24+3)&0xffffffc = 24 -> max(4,24)=24.
#[test]
fn resolve_extra_pop_aligns_to_four_bytes_w6s4_r2() {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space_le();
    let stack = stack_space();

    let mut fp = FuncProto::new();
    let model = Rc::new(one_in_one_out_model(&mgr, &reg));
    fp.set_internal(Rc::clone(&model), void_type());
    let piece = ParameterPieces {
        addr: addr(&stack, 0x14), // 20
        type_: Some(int4_type()), // size 4
        flags: parameter_pieces_flags::TYPELOCK,
    };
    fp.set_param(0, "p", &piece);
    fp.set_input_lock(true);
    fp.resolve_extra_pop();
    // cur = 20 + 4 = 24; (24+3)&0x0ffffffc = 27 & 0x0ffffffc = 24.
    assert_eq!(fp.get_extra_pop(), 24, "(0x14+4+3) & 0x0ffffffc == 24");
}
