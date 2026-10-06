//! (kuna `calltargettype`, P5) An indirect call's target takes the type of the
//! call.
//!
//! Type recovery types the target of an indirect call `code *`, a pointer to a
//! function of unknown signature, and C has no spelling for that: kuna prints
//! it `void *`, so every call through one is a call of a `void *`, which no C
//! compiler accepts.
//!
//! ```text
//!   (**(void **)&tbl[a0 * 8UL])();
//!   void *v1;  v1 = *(void **)&tbl[(a0 & 0xffffffff) * 8];  return (*v1)(a1,7) + 1;
//! ```
//!
//! The call itself states a signature: what it returns is the type of its
//! output (`void` when it has none) and what it takes is the type of each
//! argument it passes. Once every variable has its final type, this pass builds
//! that function-pointer type for each indirect call and gives it to the value
//! the call is made through, so the call reads as a call of a function pointer
//! whose parameters are exactly the arguments passed:
//!
//! ```text
//!   (**(void (**)(void))&tbl[a0 * 8UL])();
//!   int (*v1)(unsigned int,unsigned long);  ...  return (*v1)(a1,7) + 1;
//! ```
//!
//! The type must also be one C would pass the same way. Laid out under the
//! default model, every parameter has to sit in the storage the call passes
//! that argument in and the result where the call leaves it: a `float` that
//! recovery holds as an `unsigned int` in `xmm0` or `s0` would otherwise be
//! passed in `edi` or `r0`, and a `double`'s bits in `rdi` would be passed in
//! `xmm0`, so a call like that is left alone. So is one after which a value it
//! overwrites is still read -- a result in `xmm0` that its recovered output
//! does not hold.
//!
//! Three kinds of target are retyped:
//!
//! * a value of another type, which today is cast to `void *` at the call: the
//!   cast is to the call's own type instead;
//! * an expression (`*(void **)p`, `((void **)a0)[9]`): its value and, through
//!   the loads and pointer arithmetic it is computed by, the pointers it is read
//!   through, so the cast already printed on the way names the function pointer;
//! * a local variable that holds the target, when nothing it exchanges values
//!   with fixes another function type.
//!
//! Everything else keeps the spelling it has. A variable that feeds calls of
//! different signatures has no single type, so it is left alone rather than
//! guessed. A parameter, a global, a variable whose address is taken, a
//! type-locked variable and a returned one are seen by other functions, which
//! this pass cannot reconcile; one that is also dereferenced or offset is a data
//! pointer too. A variable that receives a function's address is left alone:
//! the function is declared with its own recovered prototype, which C requires
//! to match. So is one handed to a parameter declared as a pointer to code or to
//! a library callback slot (`qsort`'s comparator, `signal`'s handler), whose
//! header type is its own, and one stored through a pointer or copied into a
//! variable that already has a function-pointer type other than the call's,
//! and one compared with a function's address. An argument printed as an
//! address computed from a base (`&v4`, `&a1[2]`) has the base's C type rather
//! than its own, so the call's type takes `void *` there.
//!
//! Gated by [`ArchContext::call_target_type`](crate::context::ArchContext)
//! (option `calltargettype on|off`), which is set only together with `ctypes`:
//! without C's own spelling the target prints the upstream `code *`, and nothing
//! here runs.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{HighVariableId, OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::fspec::{FuncProto, ProtoParameter, PrototypePieces};
use crate::funcdata::Funcdata;

/// (kuna) `ActionCallTargetType` -- type each indirect call's target from the
/// call (option `calltargettype`).
pub struct ActionCallTargetType {
    base: ActionBase,
}

impl ActionCallTargetType {
    /// Construct the action in the given group.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionCallTargetType { base: ActionBase::new(0, "calltargettype", g) })
    }
}

impl Action for ActionCallTargetType {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionCallTargetType { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        data.kuna_clear_call_target_types();
        if data.get_arch().call_target_type {
            retype_call_targets(data);
        }
        0
    }
}

/// Is `ty` a pointer C cannot call and kuna does not cast at a call: a pointer
/// to the prototype-less `code` or to `void`.
fn uncallable_pointer(ty: &Datatype) -> bool {
    if ty.get_metatype() != type_metatype::TYPE_PTR {
        return false;
    }
    ty.get_ptr_to().is_some_and(|to| match to.get_metatype() {
        type_metatype::TYPE_VOID => true,
        type_metatype::TYPE_CODE => to.get_code_prototype().is_none(),
        _ => false,
    })
}

/// Is `ty` a pointer to a function with a prototype.
fn function_pointer(ty: &Datatype) -> bool {
    ty.get_metatype() == type_metatype::TYPE_PTR
        && ty
            .get_ptr_to()
            .is_some_and(|to| to.get_metatype() == type_metatype::TYPE_CODE && to.get_code_prototype().is_some())
}

/// Is `ty` a pointer to code, with or without a prototype.
fn code_pointer(ty: &Datatype) -> bool {
    ty.get_metatype() == type_metatype::TYPE_PTR
        && ty.get_ptr_to().is_some_and(|to| to.get_metatype() == type_metatype::TYPE_CODE)
}

fn same_type(a: &Rc<Datatype>, b: &Rc<Datatype>) -> bool {
    Rc::ptr_eq(a, b) || a.compare(b, 10).is_ok_and(|c| c == 0)
}

/// Can `ty` be a parameter or return type of a `size`-byte value as printed.
fn spellable(ty: &Datatype, size: i32) -> bool {
    if ty.get_size() != size || ty.needs_resolution() || ty.is_variable_length() {
        return false;
    }
    let scalar = match ty.get_metatype() {
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => Some(&[1, 2, 4, 8][..]),
        type_metatype::TYPE_FLOAT => Some(&[4, 8, 10, 16][..]),
        _ => None,
    };
    if scalar.is_some_and(|sizes| !sizes.contains(&size)) {
        return false;
    }
    !matches!(
        ty.get_metatype(),
        type_metatype::TYPE_VOID
            | type_metatype::TYPE_CODE
            | type_metatype::TYPE_SPACEBASE
            | type_metatype::TYPE_ARRAY
            | type_metatype::TYPE_PARTIALSTRUCT
            | type_metatype::TYPE_PARTIALUNION
            | type_metatype::TYPE_PARTIALENUM
    )
}

/// Is `vn` printed inline as an address computed from a base -- `&v4`,
/// `&a1[2]` -- whose C type is the base's, not necessarily `vn`'s own.
fn prints_as_address(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).filter(|v| v.is_implied() && v.is_written()).and_then(|v| v.get_def()).is_some_and(
        |def| {
            data.obank()
                .get(def)
                .is_some_and(|o| matches!(o.code(), OpCode::CPUI_PTRSUB | OpCode::CPUI_PTRADD))
        },
    )
}

/// The pointer to a function returning the type of `op`'s output (`void` when it
/// has none) and taking the types of its arguments, or `None` when one of them
/// has no C spelling.
fn call_type(data: &mut Funcdata, tlst: &dyn TypeFactory, op: OpId) -> Option<Rc<Datatype>> {
    let (out, args, target) = {
        let o = data.obank().get(op)?;
        let args: Vec<VarnodeId> = (1..o.num_input()).filter_map(|i| o.get_in(i)).collect();
        (o.get_out(), args, o.get_in(0)?)
    };
    let wordsize = data.obank().get(op)?.get_addr().get_space().map(|s| s.get_word_size()).unwrap_or(1);
    let mut pieces = PrototypePieces { first_var_arg_slot: -1, ..Default::default() };
    pieces.outtype = Some(match out {
        Some(out) => {
            let size = data.vbank().get(out)?.get_size();
            let ty = data.vn_high_type_def_facing(out);
            if !spellable(&ty, size) {
                return None;
            }
            ty
        }
        None => tlst.get_type_void().ok()?,
    });
    for vn in args {
        let size = data.vbank().get(vn)?.get_size();
        let ty = if prints_as_address(data, vn) {
            let void = tlst.get_type_void().ok()?;
            tlst.get_type_pointer(size, void, wordsize).ok()?
        } else {
            data.vn_high_type_read_facing(vn, op)
        };
        if !spellable(&ty, size) {
            return None;
        }
        pieces.intypes.push(ty);
        pieces.innames.push(String::new());
    }
    let nargs = pieces.intypes.len();
    if clobber_is_read(data, op) {
        return None;
    }
    let code = tlst.get_type_code_proto(&pieces).ok()?;
    if !passes_like_the_call(data, op, code.get_code_prototype()?, nargs, out) {
        return None;
    }
    let size = data.vbank().get(target)?.get_size();
    tlst.get_type_pointer(size, code, wordsize).ok()
}

/// Is a value the call `op` overwrites read afterwards?  The call then hands
/// back something its recovered output does not hold (a `double` left in
/// `xmm0` that the caller stores), and a call through the type would not.
fn clobber_is_read(data: &Funcdata, op: OpId) -> bool {
    let mut cur = data.op_previous_op(op);
    while let Some(prev) = cur {
        let Some(o) = data.obank().get(prev).filter(|o| o.code() == OpCode::CPUI_INDIRECT) else { break };
        let ours = o
            .get_in(1)
            .and_then(|iop| data.vbank().get(iop))
            .is_some_and(|iop| crate::funcdata_varnode::op_iop_decode(iop.get_addr().get_offset()) == op);
        let read = o.get_out().and_then(|v| data.vbank().get(v)).is_some_and(|v| !v.has_no_descend());
        if ours && o.is_indirect_creation() && read {
            return true;
        }
        cur = data.op_previous_op(prev);
    }
    false
}

/// Does `built`, the prototype the types lay out under the default model, pass
/// every argument and return the result in the storage the call `op` uses?  A
/// value in the other register class -- a `float` held as an `unsigned int` in
/// `xmm0` or `s0`, a `double`'s bits in `rdi` -- or a record returned through
/// memory compiles to a call that passes or reads something else.
fn passes_like_the_call(data: &Funcdata, op: OpId, built: &FuncProto, nargs: usize, out: Option<VarnodeId>) -> bool {
    let Some(fc) = data.get_call_specs_index(op).map(|i| data.get_call_specs(i)) else {
        return false;
    };
    let storage = fc.final_input_storage();
    if built.num_params() as usize != nargs || storage.len() != nargs {
        return false;
    }
    let plain = |p: &dyn ProtoParameter| !p.is_hidden_return() && !p.is_indirect_storage();
    let within = |p: &dyn ProtoParameter, addr: &Address, size: int4| {
        let space = |a: &Address| a.get_space().map(|s| s.get_index());
        plain(p) && space(&p.get_address()).is_some() && space(&p.get_address()) == space(addr) && {
            let (lo, hi) = (addr.get_offset(), addr.get_offset() + size as u64);
            let at = p.get_address().get_offset();
            lo <= at && at + p.get_size() as u64 <= hi
        }
    };
    let params = storage
        .iter()
        .enumerate()
        .all(|(i, (addr, size))| built.get_param(i as int4).is_some_and(|p| within(p, addr, *size)));
    let result = match out.and_then(|v| data.vbank().get(v)) {
        Some(v) => within(built.get_output(), v.get_addr(), v.get_size()),
        None => true,
    };
    params && result
}

/// The indirect calls a high is the target of, with the type each one states.
type Group = Vec<(OpId, Rc<Datatype>)>;

/// Type every indirect call's target from the call (see the module docs).
pub fn retype_call_targets(data: &mut Funcdata) {
    let Some(tlst) = data.get_arch().types_rc() else { return };
    let tlst: &dyn TypeFactory = &*tlst;
    let mut groups: BTreeMap<HighVariableId, Group> = BTreeMap::new();
    let mut casts: Vec<(OpId, Rc<Datatype>)> = Vec::new();
    let ops: Vec<OpId> = (0..data.num_calls())
        .filter(|&i| {
            let fc = data.get_call_specs(i);
            !fc.is_input_locked() && !fc.is_dotdotdot()
        })
        .map(|i| data.get_call_specs(i).get_op())
        .collect();
    for op in ops {
        let Some(target) = data
            .obank()
            .get(op)
            .filter(|o| o.code() == OpCode::CPUI_CALLIND && !o.is_dead())
            .and_then(|o| o.get_in(0))
        else {
            continue;
        };
        let Some(high) = data.vbank().get(target).filter(|v| !v.is_constant()).and_then(|v| v.get_high()) else {
            continue;
        };
        let current = data.vn_high_type_read_facing(target, op);
        if function_pointer(&current) {
            continue;
        }
        let Some(ty) = call_type(data, tlst, op) else { continue };
        if uncallable_pointer(&current) {
            groups.entry(high).or_default().push((op, ty));
        } else {
            casts.push((op, ty));
        }
    }
    for (op, ty) in casts {
        data.kuna_set_call_target_type(op, ty);
    }
    let mut chosen: BTreeMap<HighVariableId, Rc<Datatype>> = BTreeMap::new();
    for (high, group) in &groups {
        let ty = &group[0].1;
        if group.iter().any(|(_, t)| !same_type(t, ty)) {
            continue;
        }
        let ops: BTreeSet<OpId> = group.iter().map(|(op, _)| *op).collect();
        if retypable(data, *high, ty, &ops) {
            chosen.insert(*high, Rc::clone(ty));
        }
    }
    let linked_disagree: Vec<HighVariableId> = chosen
        .iter()
        .filter(|(high, ty)| {
            linked_highs(data, **high)
                .iter()
                .any(|other| chosen.get(other).is_some_and(|t| !same_type(t, ty)))
        })
        .map(|(high, _)| *high)
        .collect();
    for high in linked_disagree {
        chosen.remove(&high);
    }
    for (high, ty) in chosen {
        retype_high(data, tlst, high, &ty);
    }
}

fn instances(data: &Funcdata, high: HighVariableId) -> Vec<VarnodeId> {
    data.high_bank()
        .get(high)
        .map(|h| (0..h.num_instances()).map(|i| h.get_instance(i)).collect())
        .unwrap_or_default()
}

fn high_of(data: &Funcdata, vn: VarnodeId) -> Option<HighVariableId> {
    data.vbank().get(vn).and_then(|v| v.get_high())
}

/// The highs a value of `high` is copied to or from.
fn linked_highs(data: &Funcdata, high: HighVariableId) -> BTreeSet<HighVariableId> {
    let mut out = BTreeSet::new();
    for vn in instances(data, high) {
        let Some(v) = data.vbank().get(vn) else { continue };
        let mut ops: Vec<OpId> = v.descend_iter().collect();
        ops.extend(v.get_def().filter(|_| v.is_written()));
        for op in ops {
            let Some(o) = data.obank().get(op) else { continue };
            if !matches!(o.code(), OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT) {
                continue;
            }
            let ends = o.get_out().into_iter().chain((0..o.num_input()).filter_map(|i| o.get_in(i)));
            for end in ends {
                if let Some(h) = high_of(data, end).filter(|h| *h != high) {
                    out.insert(h);
                }
            }
        }
    }
    out
}

/// May `high`, the target of exactly the calls `ops`, take the call type `ty`.
fn retypable(data: &mut Funcdata, high: HighVariableId, ty: &Rc<Datatype>, ops: &BTreeSet<OpId>) -> bool {
    for vn in instances(data, high) {
        let Some(v) = data.vbank().get(vn) else { return false };
        if v.is_input() || v.is_persist() || v.is_addr_tied() || v.is_type_lock() || v.is_constant() {
            return false;
        }
        let def = v.get_def().filter(|_| v.is_written());
        let uses: Vec<OpId> = v.descend_iter().collect();
        if def.is_some_and(|def| !def_agrees(data, def, high, ty)) {
            return false;
        }
        for op in uses {
            if !use_agrees(data, op, vn, high, ty, ops) {
                return false;
            }
        }
    }
    true
}

/// Is `vn` an address formed from constants alone -- a function's or a global's
/// address, which prints as the object's name.
fn constant_address(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(def) = data.vbank().get(vn).filter(|v| v.is_written()).and_then(|v| v.get_def()) else {
        return false;
    };
    data.obank().get(def).is_some_and(|o| {
        matches!(o.code(), OpCode::CPUI_PTRSUB | OpCode::CPUI_INT_ADD | OpCode::CPUI_COPY)
            && (0..o.num_input())
                .filter_map(|i| o.get_in(i))
                .all(|i| data.vbank().get(i).is_some_and(|v| v.is_constant()))
    })
}

/// Does the op writing a member of `high` hand it a value C accepts into `ty`.
fn def_agrees(data: &mut Funcdata, def: OpId, high: HighVariableId, ty: &Rc<Datatype>) -> bool {
    let Some(o) = data.obank().get(def) else { return false };
    match o.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => {
            let ins: Vec<VarnodeId> = (0..o.num_input()).filter_map(|i| o.get_in(i)).collect();
            let indirect = o.code() == OpCode::CPUI_INDIRECT;
            for (slot, vn) in ins.into_iter().enumerate() {
                if indirect && slot == 1 {
                    continue;
                }
                let Some(v) = data.vbank().get(vn) else { return false };
                if v.is_constant() {
                    if v.get_offset() != 0 {
                        return false;
                    }
                    continue;
                }
                if high_of(data, vn) == Some(high) {
                    continue;
                }
                if constant_address(data, vn) {
                    return false;
                }
                let other = data.vn_high_type_read_facing(vn, def);
                if function_pointer(&other) && !same_type(&other, ty) {
                    return false;
                }
            }
            true
        }
        OpCode::CPUI_PTRSUB | OpCode::CPUI_INT_ADD => !o.get_out().is_some_and(|out| constant_address(data, out)),
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
            let Some(i) = data.get_call_specs_index(def) else { return true };
            let proto = data.get_call_specs(i).proto();
            !(proto.is_output_locked()
                && proto.get_output_type().is_some_and(|t| function_pointer(t) && !same_type(t, ty)))
        }
        _ => true,
    }
}

/// Does reading member `vn` of `high` at `op` accept a value of type `ty`.
fn use_agrees(
    data: &mut Funcdata,
    op: OpId,
    vn: VarnodeId,
    high: HighVariableId,
    ty: &Rc<Datatype>,
    ops: &BTreeSet<OpId>,
) -> bool {
    let Some(o) = data.obank().get(op) else { return false };
    let code = o.code();
    let slot = o.get_slot(vn);
    let out = o.get_out();
    match code {
        OpCode::CPUI_RETURN => false,
        OpCode::CPUI_LOAD | OpCode::CPUI_STORE if slot == 1 => false,
        OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB | OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => false,
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
            let Some(other) = o.get_in(1 - slot) else { return false };
            let constant = data.vbank().get(other).is_some_and(|v| v.is_constant() && v.get_offset() != 0);
            !constant && !constant_address(data, other)
        }
        OpCode::CPUI_CALLIND if slot == 0 => ops.contains(&op),
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
            let declared = crate::coreaction_infertypes::declared_input_type_local(data, op, slot);
            if code_pointer(&declared) && !same_type(&declared, ty) {
                return false;
            }
            let callee = data.get_call_specs_index(op).map(|i| data.get_call_specs(i).get_name().to_string());
            let param = usize::try_from(slot - 1).ok();
            !callee.zip(param).is_some_and(|(name, i)| crate::kuna_callbacktype::is_callback_slot(&name, i))
        }
        OpCode::CPUI_STORE if slot == 2 => {
            let Some(ptr) = o.get_in(1) else { return false };
            let ptr_ty = data.vn_high_type_read_facing(ptr, op);
            !ptr_ty.get_ptr_to().is_some_and(|to| function_pointer(&to) && !same_type(&to, ty))
        }
        OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => {
            let Some(out) = out else { return true };
            if high_of(data, out) == Some(high) {
                return true;
            }
            let other = data.vn_high_type_def_facing(out);
            !(function_pointer(&other) && !same_type(&other, ty))
        }
        _ => true,
    }
}

/// Give every member of `high` the type `ty`, and carry it back through the
/// loads and pointer arithmetic that compute a member inline.
fn retype_high(data: &mut Funcdata, tlst: &dyn TypeFactory, high: HighVariableId, ty: &Rc<Datatype>) {
    for vn in instances(data, high) {
        let old = Rc::clone(data.vbank().get(vn).map(|v| v.get_type()).expect("member"));
        data.vn_update_type(vn, Rc::clone(ty));
        carry_back(data, tlst, vn, old, Rc::clone(ty));
    }
    if let Some(&first) = instances(data, high).first() {
        let _ = data.high_get_type(first);
    }
}

/// `vn`'s type went from `old` to `new`; retype the implied pointer expression
/// it is loaded through, level by level, while that expression has the type
/// that pointed at `old`.
fn carry_back(data: &mut Funcdata, tlst: &dyn TypeFactory, vn: VarnodeId, old: Rc<Datatype>, new: Rc<Datatype>) {
    let (mut vn, mut old, mut new) = (vn, old, new);
    let mut loaded = false;
    for _ in 0..16 {
        let Some(def) = data.vbank().get(vn).filter(|v| v.is_written()).and_then(|v| v.get_def()) else { return };
        let Some(o) = data.obank().get(def) else { return };
        let next = match o.code() {
            OpCode::CPUI_LOAD => o.get_in(1),
            OpCode::CPUI_COPY | OpCode::CPUI_PTRADD if loaded => o.get_in(0),
            OpCode::CPUI_PTRSUB if loaded => o
                .get_in(1)
                .and_then(|c| data.vbank().get(c))
                .filter(|c| c.is_constant() && c.get_offset() == 0)
                .and(o.get_in(0)),
            _ => None,
        };
        let is_load = o.code() == OpCode::CPUI_LOAD;
        let Some(next) = next else { return };
        let Some(nv) = data.vbank().get(next) else { return };
        if !nv.is_implied() || nv.is_type_lock() || data.lone_descend(next) != Some(def) {
            return;
        }
        let next_ty = Rc::clone(nv.get_type());
        let (want_old, next_new) = if is_load {
            let Some(to) = next_ty.get_ptr_to().filter(|_| next_ty.get_metatype() == type_metatype::TYPE_PTR) else {
                return;
            };
            if !same_type(&to, &old) {
                return;
            }
            let ws = next_ty.get_word_size().unwrap_or(1);
            let Ok(p) = tlst.get_type_pointer(nv.get_size(), Rc::clone(&new), ws) else { return };
            (next_ty, p)
        } else {
            if !same_type(&next_ty, &old) {
                return;
            }
            (next_ty, Rc::clone(&new))
        };
        data.vn_update_type(next, Rc::clone(&next_new));
        vn = next;
        old = want_old;
        new = next_new;
        loaded = true;
    }
}


#[cfg(test)]
mod tests;
