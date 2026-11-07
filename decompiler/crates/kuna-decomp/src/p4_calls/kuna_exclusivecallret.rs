//! P4 mutually exclusive call and return uses in trial scoring — the
//! `exclusivecallret` decision point.
//!
//! `Funcdata::only_op_use` (C++ `Funcdata::onlyOpUse`, `funcdata_varnode.cc:1851`)
//! rejects a trial whose value has another CALL or RETURN use. GCC compiles
//! `return a(x) ? b(x) : x` with a tail call to `b`, so the same scalar is the
//! tail call's argument on one arm and the function's return on the other. Each
//! use vetoes the other's trial although no execution reaches both, and the
//! function is recovered `void` or the tail call loses its argument.
//!
//! The rule skips a CALL/CALLIND use while scoring a RETURN trial, and a RETURN
//! use while scoring a call's input trial, when the two ops sit in distinct
//! blocks, neither reaches the other and the RETURN is a normal return. The use
//! must read the value as data, not as a call target or return address. An
//! input trial also requires that the value was produced in a block both uses
//! follow through single-entry, single-exit chains, so it was prepared for the
//! branch rather than merely still live.
//!
//! The rule is limited to the shape `calloverlap auto` recovers: a four-byte
//! trial in the low word of a named eight-byte register on a truncated 32-bit
//! address target, such as an e500 GPR. Reachability is
//! [`exclusivearguse`](crate::p4_calls::kuna_exclusivearguse)'s bounded walk,
//! which declines once its bound is exceeded.

use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p4_calls::kuna_exclusivearguse::{def_block, reaches, MAX_VISIT};

/// Whether the trial matched at `opmatch` has the narrow scalar shape the rule
/// covers. Independent of the use being scored, so it is computed once per walk.
pub(crate) fn applies(data: &Funcdata, opmatch: OpId, trial: &crate::fspec::ParamTrial) -> bool {
    if !data.get_arch().exclusive_call_ret
        || trial.get_size() != 4
        || !data.get_func_proto().has_model()
        || !data.obank().get(opmatch).is_some_and(|o| {
            matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_RETURN)
        })
    {
        return false;
    }
    let addr = trial.get_address();
    let Some(space) = addr.get_space() else {
        return false;
    };
    let wide = addr - if space.is_big_endian() { 4 } else { 0 };
    crate::p3_dataflow::kuna_calloverlap::auto_register(
        data.get_arch().manage(),
        data.get_func_proto(),
        &wide,
        8,
        true,
    )
}

/// Whether `op`'s read of `reader` is a call or return use on a path that cannot
/// co-execute with the trial matched at `opmatch`, whose root value is `value`.
pub(crate) fn skips_use(
    data: &Funcdata,
    opmatch: OpId,
    op: OpId,
    value: VarnodeId,
    reader: VarnodeId,
) -> bool {
    let (Some(m), Some(o)) = (data.obank().get(opmatch), data.obank().get(op)) else {
        return false;
    };
    if o.get_slot(reader) <= 0 {
        return false;
    }
    let (ret, call) = match (m.code(), o.code()) {
        (OpCode::CPUI_RETURN, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) => (m, o),
        (OpCode::CPUI_CALL | OpCode::CPUI_CALLIND, OpCode::CPUI_RETURN) => (o, m),
        _ => return false,
    };
    if ret.get_halt_type() != 0 {
        return false;
    }
    let (Some(r), Some(c)) = (ret.get_parent(), call.get_parent()) else {
        return false;
    };
    disjoint(data, r, c)
        && (m.code() == OpCode::CPUI_RETURN
            || def_block(data, value).is_some_and(|d| shared_producer(data, d, r, c)))
}

/// Distinct blocks neither of which reaches the other.
fn disjoint(data: &Funcdata, a: BlockId, b: BlockId) -> bool {
    a != b && !reaches(data, a, b) && !reaches(data, b, a)
}

/// `producer` precedes both `a` and `b` through chains of single-entry,
/// single-exit blocks.
fn shared_producer(data: &Funcdata, producer: BlockId, a: BlockId, b: BlockId) -> bool {
    let graph = data.bblocks_ref();
    let follows = |mut bl: BlockId| {
        for _ in 0..MAX_VISIT {
            let block = graph.block(bl);
            if block.size_in() != 1 {
                return false;
            }
            let prev = block.get_in(0);
            if prev == producer {
                return true;
            }
            if graph.block(prev).size_out() != 1 {
                return false;
            }
            bl = prev;
        }
        false
    };
    a != b && follows(a) && follows(b)
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};

    use super::*;
    use crate::context::ArchContext;

    fn build_fd() -> Funcdata {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            true,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        let glb = Rc::new(ArchContext::new(m));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        Funcdata::new("f", "f", glb, Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap()
    }

    #[test]
    fn call_return_arms_must_be_disjoint_and_share_the_producer() {
        let mut fd = build_fd();
        let root = fd.bblocks_root_pub();
        let entry = fd.bblocks_mut().new_block_basic(root);
        let head = fd.bblocks_mut().new_block_basic(root);
        let left = fd.bblocks_mut().new_block_basic(root);
        let right = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(entry, head);
        fd.bblocks_mut().add_edge(head, left);
        fd.bblocks_mut().add_edge(head, right);
        let end = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().add_edge(left, end);
        assert!(disjoint(&fd, end, right));
        assert!(disjoint(&fd, right, end));
        assert!(shared_producer(&fd, head, left, right));
        assert!(shared_producer(&fd, head, end, right));
        assert!(!shared_producer(&fd, entry, left, right));
        assert!(!disjoint(&fd, left, left));
        assert!(!disjoint(&fd, head, right));
        fd.bblocks_mut().add_edge(left, right);
        assert!(!disjoint(&fd, left, right));
        assert!(!disjoint(&fd, right, left));
    }
}
