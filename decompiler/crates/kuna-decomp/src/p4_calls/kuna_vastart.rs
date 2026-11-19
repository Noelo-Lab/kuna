//! (kuna `vastart`) The x86-64 SysV `va_start` prologue prints as
//! `va_start(ap, last)`.
//!
//! A variadic function that materializes a `va_list` spills every argument
//! register to a 176-byte register save area (the XMM half under `if (al)`)
//! and initializes the four `__va_list_tag` fields: `gp_offset` (8 per named
//! integer argument), `fp_offset` (48 plus 16 per named float argument),
//! `overflow_arg_area` (the caller's stack arguments) and `reg_save_area`.
//! Without this the sequence prints as a dozen register copies and four field
//! stores.
//!
//! [`ActionVaStart`] finds the two offset constants (two 4-byte writes or one
//! 8-byte write at `X`) beside the pointer writes at `X+8` and `X+16` in one
//! block, where the `X+16` value is the address of a stack range. It inserts a
//! `va_start(&X, last)` CALLOTHER before the first of those writes and turns
//! each write into an indirect creation of that CALLOTHER, so the `va_list`
//! keeps its later readers. `last` is the input that carries the last named
//! argument: the parameter at index `gp_offset/8 + (fp_offset-48)/16 - 1`,
//! or with no prototype yet and no named float argument, the integer argument
//! register `gp_offset/8 - 1`.
//! The register save area loses its address-forced liveness and its local
//! mapping (as the stack-guard strip does for the canary slot), so dead-code
//! removes the spills. An unlocked prototype is marked variadic.

use kuna_base::address::Address;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// Bytes of the x86-64 SysV register save area (6 GPRs, 8 XMM registers).
const SAVE_AREA: u64 = 176;

/// A stack write `(offset, size, op)`.
type Write = (u64, int4, OpId);

fn stack_writes(data: &Funcdata) -> Vec<Write> {
    let Some(stack) = data.get_arch().manage().get_stack_space() else { return Vec::new() };
    let mut out = Vec::new();
    let ops: Vec<OpId> = data.obank().iter_alive().collect();
    for op in ops {
        let Some(o) = data.obank().get(op) else { continue };
        if o.get_parent().is_none() || o.code() == OpCode::CPUI_MULTIEQUAL {
            continue;
        }
        let Some(v) = o.get_out().and_then(|v| data.vbank().get(v)) else { continue };
        if v.get_addr().get_space().is_some_and(|s| s.get_index() == stack.get_index()) {
            out.push((v.get_offset(), v.get_size(), op));
        }
    }
    out
}

/// The constant a COPY writes.
fn copied(data: &Funcdata, op: OpId) -> Option<u64> {
    let o = data.obank().get(op)?;
    if o.code() != OpCode::CPUI_COPY {
        return None;
    }
    let v = data.vbank().get(o.get_in(0)?)?;
    v.is_constant().then(|| v.get_offset())
}

/// `(spacebase, offset, op)` when `vn` is a stack address, `PTRSUB(sp, offset)`
/// or `INT_ADD(sp, offset)`, through COPYs.
fn stack_address(data: &Funcdata, mut vn: VarnodeId) -> Option<(VarnodeId, u64, OpCode)> {
    for _ in 0..4 {
        let d = data.obank().get(data.vbank().get(vn)?.get_def()?)?;
        match d.code() {
            OpCode::CPUI_COPY => vn = d.get_in(0)?,
            code @ (OpCode::CPUI_PTRSUB | OpCode::CPUI_INT_ADD) => {
                let sb = d.get_in(0)?;
                let off = data.vbank().get(d.get_in(1)?)?;
                return (data.vbank().get(sb)?.is_spacebase() && off.is_constant()).then(|| (sb, off.get_offset(), code));
            }
            _ => return None,
        }
    }
    None
}

/// The value `op` puts in its stack output: a COPY's input, or the input of an
/// INDIRECT whose input is other storage (a store folded into the call's
/// effect on the slot).
fn written(data: &Funcdata, op: OpId) -> Option<VarnodeId> {
    let o = data.obank().get(op)?;
    let input = o.get_in(0)?;
    match o.code() {
        OpCode::CPUI_COPY => Some(input),
        OpCode::CPUI_INDIRECT => {
            let (i, out) = (data.vbank().get(input)?, data.vbank().get(o.get_out()?)?);
            (i.get_addr() != out.get_addr() && !i.is_constant()).then_some(input)
        }
        _ => None,
    }
}

struct VaStart {
    list: u64,
    /// The COPYs of the two offsets.
    offsets: Vec<OpId>,
    /// The writes of `overflow_arg_area` and `reg_save_area`.
    pointers: Vec<OpId>,
    spacebase: VarnodeId,
    address_op: OpCode,
    save_area: u64,
    named: int4,
    gp: int4,
}

fn find(data: &Funcdata) -> Option<VaStart> {
    let writes = stack_writes(data);
    let at = |off: u64, size: int4| writes.iter().filter(move |w| w.0 == off && w.1 == size).map(|w| w.2);
    let gp_ok = |gp: u64| gp % 8 == 0 && gp <= 48;
    let fp_ok = |fp: u64| (48..=SAVE_AREA).contains(&fp) && (fp - 48) % 16 == 0;
    for &(x, size, op) in &writes {
        let (offsets, gp, fp) = match (size, copied(data, op)) {
            (8, Some(v)) if gp_ok(v & 0xffff_ffff) && fp_ok(v >> 32) => (vec![op], v & 0xffff_ffff, v >> 32),
            (4, Some(gp)) if gp_ok(gp) => {
                let Some((fop, fp)) = at(x.wrapping_add(4), 4).find_map(|f| copied(data, f).filter(|&fp| fp_ok(fp)).map(|fp| (f, fp))) else {
                    continue;
                };
                (vec![op, fop], gp, fp)
            }
            _ => continue,
        };
        let block = data.obank().get(op)?.get_parent()?;
        let same = |o: &OpId| data.obank().get(*o).and_then(|p| p.get_parent()) == Some(block) && written(data, *o).is_some();
        let Some(overflow) = at(x.wrapping_add(8), 8).find(same) else { continue };
        let Some((save, spacebase, save_area, address_op)) = at(x.wrapping_add(16), 8).filter(same).find_map(|s| {
            stack_address(data, written(data, s)?).map(|(sb, off, code)| (s, sb, off, code))
        }) else {
            continue;
        };
        let named = (gp / 8 + (fp - 48) / 16) as int4;
        return Some(VaStart { list: x, offsets, pointers: vec![overflow, save], spacebase, address_op, save_area, named, gp: (gp / 8) as int4 });
    }
    None
}

/// The input that carries the last named argument: the prototype's parameter
/// `named - 1` when it lists that many, else (no named float argument) the
/// SysV integer argument register `gp / 8 - 1` (a 4- or 8-byte input).
pub(crate) fn last_named(data: &Funcdata, named: int4, gp: int4) -> Option<VarnodeId> {
    let proto = data.get_func_proto();
    if named == 0 {
        return None;
    }
    if proto.has_store() && proto.num_params() >= named {
        let p = proto.get_param(named - 1)?;
        return data.vbank().find_input(p.get_size(), &p.get_address());
    }
    if named != gp {
        return None;
    }
    let name = ["RDI", "RSI", "RDX", "RCX", "R8", "R9"].get(usize::try_from(gp - 1).ok()?)?;
    let st = data.get_arch().manage().register_lookup()?.probe_register(name)?;
    let addr = Address::new(st.space.clone()?, st.offset);
    [8, 4].into_iter().find_map(|size| data.vbank().find_input(size, &addr))
}

fn rewrite(data: &mut Funcdata, m: &VaStart) -> kuna_base::error::KunaResult<()> {
    use kuna_base::error::KunaError;
    let block = data.obank().get(m.offsets[0]).and_then(|o| o.get_parent()).ok_or_else(|| KunaError::lowlevel("vastart: stale op"))?;
    let first = data.bb_ops(block).into_iter().find(|o| m.offsets.contains(o)).ok_or_else(|| KunaError::lowlevel("vastart: no first"))?;
    let at = data.obank().get(first).map(|o| o.get_addr().clone()).ok_or_else(|| KunaError::lowlevel("vastart: stale op"))?;
    let sb_size = data.vbank().get(m.spacebase).map_or(8, |v| v.get_size());
    let ptr = data.new_op(2, at.clone());
    data.op_set_opcode_code(ptr, m.address_op);
    data.op_set_input(ptr, m.spacebase, 0)?;
    let off = data.new_constant(sb_size, m.list as uintb);
    data.op_set_input(ptr, off, 1)?;
    let ap = data.new_unique_out(sb_size, ptr)?;
    data.op_insert_before(ptr, first);
    let last = last_named(data, m.named, m.gp);
    let call = data.new_op(if last.is_some() { 3 } else { 2 }, at);
    data.op_set_opcode_code(call, OpCode::CPUI_CALLOTHER);
    let id = data.new_constant(4, crate::userop::BUILTIN_VA_START as uintb);
    data.op_set_input(call, id, 0)?;
    data.op_set_input(call, ap, 1)?;
    if let Some(last) = last {
        data.op_set_input(call, last, 2)?;
    }
    data.op_insert_after(call, ptr);
    for &op in &m.offsets {
        let size = data.obank().get(op).and_then(|o| o.get_out()).and_then(|v| data.vbank().get(v)).map_or(8, |v| v.get_size());
        data.op_uninsert(op);
        let zero = data.new_constant(size, 0);
        let iop = data.new_varnode_iop(call);
        data.op_set_all_input(op, &[zero, iop])?;
        data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_INDIRECT));
        if let Some(o) = data.obank_mut().get_mut(op) {
            o.set_flag(crate::op::pcodeop_flags::indirect_creation);
        }
        data.op_insert_before(op, call);
    }
    for &op in &m.pointers {
        let Some(out) = data.obank().get(op).and_then(|o| o.get_out()) else { continue };
        let (addr, size) = data.vbank().get(out).map(|v| (v.get_addr().clone(), v.get_size())).ok_or_else(|| KunaError::lowlevel("vastart: stale out"))?;
        let created = data.new_indirect_creation(call, &addr, size, false);
        let value = data.obank().get(created).and_then(|o| o.get_out()).ok_or_else(|| KunaError::lowlevel("vastart: no creation"))?;
        data.op_set_input(op, value, 0)?;
    }
    if let Some(stack) = data.get_arch().manage().get_stack_space().cloned() {
        let end = m.save_area.wrapping_add(SAVE_AREA);
        let spills: Vec<VarnodeId> = data
            .vbank()
            .iter_loc()
            .filter(|&v| {
                data.vbank().get(v).is_some_and(|n| {
                    n.get_addr().get_space().is_some_and(|s| s.get_index() == stack.get_index())
                        && (m.save_area..end).contains(&n.get_offset())
                })
            })
            .collect();
        for vn in spills {
            if let Some(v) = data.vbank_mut().get_mut(vn) {
                if v.is_addr_tied() && v.is_addr_force() {
                    v.clear_addr_force();
                }
            }
        }
        data.scope_local_mark_not_mapped(&stack, m.save_area, SAVE_AREA as int4, false);
    }
    if !data.get_func_proto().is_input_locked() {
        data.get_func_proto_mut().set_dotdotdot(true);
    }
    Ok(())
}

/// Print the `va_start` prologue as one call (see the module docs).
pub struct ActionVaStart {
    base: ActionBase,
}

impl ActionVaStart {
    pub fn boxed(g: &str) -> Box<dyn Action> {
        Box::new(ActionVaStart { base: ActionBase::new(0, "vastart", g) })
    }
}

impl Action for ActionVaStart {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist.contains(self.get_group()).then(|| Box::new(ActionVaStart { base: self.base.clone() }) as Box<dyn Action>)
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().va_start {
            return 0;
        }
        let Some(m) = find(data) else { return 0 };
        if rewrite(data, &m).is_ok() {
            self.base.count += 1;
            return 1;
        }
        0
    }
}
