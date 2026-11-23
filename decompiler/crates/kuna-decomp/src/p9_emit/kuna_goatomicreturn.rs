//! One typed atomic addition whose new value is the entire Go return body.

use std::collections::HashSet;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::p2_lift::kuna_atomicxadd::Instruction;
use kuna_num::opcodes::OpCode;

pub(crate) struct Plan {
    pub atomic: OpId,
    pub store: OpId,
    pub sum: OpId,
    pub pointer: VarnodeId,
    pub pointer_def: Option<OpId>,
    pub delta: VarnodeId,
    pub width: i32,
}

fn signed(ty: &Datatype, width: i32) -> bool {
    ty.get_metatype() == type_metatype::TYPE_INT
        && ty.get_size() == width
        && !crate::kuna_typequal::is_volatile(ty)
}

fn constant(fd: &Funcdata, vn: VarnodeId) -> Option<u64> {
    let v = fd.vbank().get(vn)?;
    v.is_constant().then_some(v.get_offset())
}

fn parameter(fd: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = fd.vbank().get(vn) else {
        return false;
    };
    if !v.is_input() || v.is_volatile() || v.is_spacebase() {
        return false;
    }
    let proto = fd.get_func_proto();
    (0..proto.num_params())
        .filter_map(|i| proto.get_param(i))
        .any(|p| {
            p.is_type_locked()
                && !p.is_indirect_storage()
                && !p.is_hidden_return()
                && p.get_address() == *v.get_addr()
                && p.get_size() == v.get_size()
                && p.get_type()
                    .is_some_and(|ty| !crate::kuna_typequal::is_volatile(ty))
        })
}

fn parameter_type(fd: &Funcdata, vn: VarnodeId) -> Option<&Datatype> {
    let v = fd.vbank().get(vn)?;
    let proto = fd.get_func_proto();
    (0..proto.num_params())
        .filter_map(|i| proto.get_param(i))
        .find(|p| {
            p.is_type_locked() && p.get_address() == *v.get_addr() && p.get_size() == v.get_size()
        })?
        .get_type()
        .map(|ty| ty.as_ref())
}

fn transparent(
    fd: &Funcdata,
    mut vn: VarnodeId,
    width: i32,
    used: &mut HashSet<OpId>,
) -> Option<VarnodeId> {
    for _ in 0..8 {
        let v = fd.vbank().get(vn)?;
        if v.get_size() != width || v.is_volatile() {
            return None;
        }
        let Some(id) = v.get_def() else {
            return Some(vn);
        };
        let op = fd.obank().get(id)?;
        if !matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            return Some(vn);
        }
        if op.num_input() != 1 || op.get_out() != Some(vn) {
            return None;
        }
        used.insert(id);
        vn = op.get_in(0)?;
    }
    None
}

fn pointer(
    fd: &Funcdata,
    vn: VarnodeId,
    used: &mut HashSet<OpId>,
    depth: usize,
) -> Option<(VarnodeId, u64)> {
    if depth >= 4 {
        return None;
    }
    let v = fd.vbank().get(vn)?;
    if v.get_size() != 8 || v.is_volatile() {
        return None;
    }
    if parameter(fd, vn) {
        return (v.get_type().get_metatype() == type_metatype::TYPE_PTR
            && parameter_type(fd, vn)?.get_metatype() == type_metatype::TYPE_PTR)
            .then_some((vn, 0));
    }
    if depth != 0 && !v.is_implied() {
        return None;
    }
    let id = v.get_def()?;
    let op = fd.obank().get(id)?;
    let offset = match (op.code(), op.num_input()) {
        (OpCode::CPUI_COPY | OpCode::CPUI_CAST, 1) => 0,
        (OpCode::CPUI_PTRSUB, 2) => constant(fd, op.get_in(1)?)?,
        (OpCode::CPUI_PTRADD, 3) => {
            constant(fd, op.get_in(1)?)?.checked_mul(constant(fd, op.get_in(2)?)?)?
        }
        _ => return None,
    };
    let (base, displacement) = pointer(fd, op.get_in(0)?, used, depth + 1)?;
    used.insert(id);
    Some((base, displacement.checked_add(offset)?))
}

fn userop(fd: &Funcdata, arch: &Architecture, id: OpId, name: &[u8]) -> bool {
    let Some(op) = fd.obank().get(id) else {
        return false;
    };
    if op.code() != OpCode::CPUI_CALLOTHER || op.num_input() != 1 || op.get_out().is_some() {
        return false;
    }
    op.get_in(0)
        .and_then(|v| constant(fd, v))
        .and_then(|n| u32::try_from(n).ok())
        .and_then(|n| arch.userops.get_op(n))
        .is_some_and(|u| u.get_name() == name)
}

fn load(
    fd: &Funcdata,
    proof: &Instruction,
    vn: VarnodeId,
    used: &mut HashSet<OpId>,
    target: (VarnodeId, u64),
    space: u64,
) -> Option<OpId> {
    let v = fd.vbank().get(vn)?;
    let id = v.get_def()?;
    let op = fd.obank().get(id)?;
    if v.get_size() != proof.width
        || v.is_volatile()
        || !proof.loads.contains(&id)
        || op.code() != OpCode::CPUI_LOAD
        || op.num_input() != 2
        || constant(fd, op.get_in(0)?)? != space
        || pointer(fd, op.get_in(1)?, used, 0)? != target
    {
        return None;
    }
    used.insert(id);
    Some(id)
}

fn addition(
    fd: &Funcdata,
    vn: VarnodeId,
    used: &mut HashSet<OpId>,
) -> Option<(OpId, VarnodeId, VarnodeId)> {
    let id = fd.vbank().get(vn)?.get_def()?;
    let op = fd.obank().get(id)?;
    if op.code() != OpCode::CPUI_INT_ADD || op.num_input() != 2 {
        return None;
    }
    used.insert(id);
    Some((id, op.get_in(0)?, op.get_in(1)?))
}

pub(crate) fn plan(fd: &Funcdata, arch: &Architecture) -> Option<Plan> {
    if !arch.source_lang().is_go()
        || !arch.archid.starts_with("x86:LE:64:")
        || fd.kuna_atomic_xadds.len() != 1
        || fd.num_calls() != 0
        || fd.sblocks_get_size() == 0
        || fd.kuna_pipeline_failure().is_some()
        || fd.get_override().has_flow_override()
        || !fd.kuna_rejected_flow_overrides().is_empty()
        || !fd.kuna_rejected_callee_overrides().is_empty()
    {
        return None;
    }
    let proto = fd.get_func_proto();
    if !proto.has_store() || !proto.get_output().is_type_locked() {
        return None;
    }
    let proof = &fd.kuna_atomic_xadds[0];
    if !signed(proto.get_output_type()?, proof.width)
        || !userop(fd, arch, proof.lock, b"LOCK")
        || !userop(fd, arch, proof.unlock, b"UNLOCK")
    {
        return None;
    }
    let live: Vec<_> = fd.obank().iter_alive().take(33).collect();
    if live.len() > 32 {
        return None;
    }
    let block = fd.obank().get(proof.lock)?.get_parent()?;
    if live.iter().any(|id| {
        fd.obank()
            .get(*id)
            .is_none_or(|op| op.get_parent() != Some(block))
    }) {
        return None;
    }
    let returns: Vec<_> = live
        .iter()
        .copied()
        .filter(|id| {
            fd.obank()
                .get(*id)
                .is_some_and(|op| op.code() == OpCode::CPUI_RETURN)
        })
        .collect();
    if returns.len() != 1 {
        return None;
    }
    let ret = returns[0];
    let retop = fd.obank().get(ret)?;
    if retop.num_input() != 2 || retop.get_halt_type() != 0 || fd.bb_op_tail(block) != Some(ret) {
        return None;
    }
    let mut order = Vec::new();
    let mut cursor = fd.bb_op_head(block);
    while let Some(id) = cursor {
        order.push(id);
        cursor = fd.bb_op_next(id);
    }
    let position = |id| order.iter().position(|item| *item == id);
    if !(position(proof.lock)? < position(proof.store)?
        && position(proof.store)? < position(proof.unlock)?
        && position(proof.unlock)? < position(ret)?)
    {
        return None;
    }
    let store = fd.obank().get(proof.store)?;
    if store.code() != OpCode::CPUI_STORE || store.num_input() != 3 {
        return None;
    }
    let address = store.get_in(1)?;
    let ptr_type = fd.vn_type_read_facing(address, proof.store);
    if ptr_type.get_metatype() != type_metatype::TYPE_PTR
        || crate::kuna_typequal::is_volatile(&ptr_type)
        || !ptr_type
            .get_ptr_to()
            .is_some_and(|ty| signed(&ty, proof.width))
    {
        return None;
    }
    let space = constant(fd, store.get_in(0)?)?;
    if fd
        .get_arch()
        .manage()
        .get_space(i32::try_from(space).ok()?)?
        .get_name()
        != "ram"
    {
        return None;
    }
    let mut used = HashSet::from([proof.lock, proof.unlock, proof.store, ret]);
    let target = pointer(fd, address, &mut used, 0)?;
    let stored = transparent(fd, store.get_in(2)?, proof.width, &mut used)?;
    let (sum, left, right) = addition(fd, stored, &mut used)?;
    if !proof.members.contains(&sum) {
        return None;
    }
    let (loaded, delta) = if parameter(fd, right) {
        (left, right)
    } else if parameter(fd, left) {
        (right, left)
    } else {
        return None;
    };
    let stored_load = load(fd, proof, loaded, &mut used, target, space)?;
    if stored_load == proof.old_load
        || !signed(&fd.vn_type_read_facing(delta, sum), proof.width)
        || !signed(parameter_type(fd, delta)?, proof.width)
        || fd.vbank().get(delta)?.get_size() != proof.width
    {
        return None;
    }
    let returned = transparent(fd, retop.get_in(1)?, proof.width, &mut used)?;
    let (_, a, b) = addition(fd, returned, &mut used)?;
    let old = if a == delta {
        b
    } else if b == delta {
        a
    } else {
        return None;
    };
    if load(fd, proof, old, &mut used, target, space)? != proof.old_load
        || live.iter().any(|id| !used.contains(id))
    {
        return None;
    }
    Some(Plan {
        atomic: proof.lock,
        store: proof.store,
        sum,
        pointer: address,
        pointer_def: fd.vbank().get(address)?.get_def(),
        delta,
        width: proof.width,
    })
}

impl crate::printc::PrintC {
    pub(crate) fn emit_go_atomic_return(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        plan: &Plan,
    ) {
        use crate::prettyprint::{Emit, SyntaxHighlight};
        use crate::printc::{op_key, tokens};

        self.emit.tag_line();
        let markup = self.op_markup(fd, plan.atomic);
        let statement = self.emit.begin_statement(&markup);
        self.emit.print("return ", SyntaxHighlight::KeywordColor);
        self.push_go_call(
            if plan.width == 4 {
                "atomic.AddInt32"
            } else {
                "atomic.AddInt64"
            },
            plan.atomic,
        );
        self.push_op(&tokens::COMMA, Some(op_key(plan.atomic)));
        if let Some(def) = plan.pointer_def {
            self.op_push_ir(fd, arch, def, Some(plan.store));
        } else {
            self.push_vn_ir(fd, arch, plan.pointer, plan.store);
        }
        self.push_vn_ir(fd, arch, plan.delta, plan.sum);
        self.recurse();
        self.emit.end_statement(statement);
    }
}
