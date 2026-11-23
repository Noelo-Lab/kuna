//! AArch64 barrier, hint and memory-tagging user ops printed as the ACLE
//! intrinsics of `<arm_acle.h>`.
//!
//! The SLEIGH names of these ops (`DataMemoryBarrier(2,3)`) describe the
//! instruction; the source that produced it called `__dmb(11)`. The rewrite is
//! keyed on the user-op name and never changes what is called, only how the
//! call is spelled: the barrier domain and type operands recombine into the
//! 4-bit `CRm` the intrinsic takes, and a tag load or store prints as
//! `__arm_mte_get_tag` / `__arm_mte_set_tag` only in the one shape those
//! intrinsics have, the pointer naming its own granule.

use kuna_base::address::Address;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

/// One argument of a rewritten call.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Arg {
    /// An operand of the user op, printed as it would be under the SLEIGH name.
    Vn(VarnodeId),
    /// An immediate the intrinsic takes, printed in decimal.
    Dec(u64),
}

/// The intrinsic a user op prints as.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Intrinsic {
    pub name: &'static str,
    pub args: Vec<Arg>,
}

/// The ACLE spelling of the CALLOTHER `op` whose user op is named `name`.
pub(crate) fn intrinsic(fd: &Funcdata, arch: &Architecture, op: OpId, name: &[u8]) -> Option<Intrinsic> {
    if !arch.acle_intrinsics || !arch.archid.starts_with("AARCH64:") {
        return None;
    }
    let o = fd.obank().get(op)?;
    let operands: Vec<VarnodeId> = (1..o.num_input()).filter_map(|i| o.get_in(i)).collect();
    let constant = |vn: VarnodeId| fd.vbank().get(vn).filter(|v| v.is_constant()).map(|v| v.get_offset());
    let (name, args) = match (name, operands.as_slice()) {
        (b"DataMemoryBarrier", &[domain, types]) => ("__dmb", vec![Arg::Dec(crm(constant(domain)?, constant(types)?)?)]),
        (b"DataSynchronizationBarrier", &[domain, types, nxs]) if constant(nxs)? == 0 => {
            ("__dsb", vec![Arg::Dec(crm(constant(domain)?, constant(types)?)?)])
        }
        (b"InstructionSynchronizationBarrier", &[]) => ("__isb", vec![Arg::Dec(isb_option(arch, o.get_addr())?)]),
        (b"WaitForInterrupt", &[]) => ("__wfi", Vec::new()),
        (b"WaitForEvent", &[]) => ("__wfe", Vec::new()),
        (b"Yield", &[]) => ("__yield", Vec::new()),
        (b"SendEvent", &[]) => ("__sev", Vec::new()),
        (b"SendEventLocally", &[]) => ("__sevl", Vec::new()),
        (b"RandomizePtrTag_Exclude", &[ptr, mask]) => ("__arm_mte_create_random_tag", vec![Arg::Vn(ptr), Arg::Vn(mask)]),
        (b"AddPtrTag", &[ptr, offset]) => ("__arm_mte_increment_tag", vec![Arg::Vn(ptr), Arg::Dec(constant(offset)?)]),
        (b"ExcludePtrTag", &[ptr, excluded]) => ("__arm_mte_exclude_tag", vec![Arg::Vn(ptr), Arg::Vn(excluded)]),
        (b"LoadPtrTag", &[ptr, granule]) if same_value(fd, ptr, granule, 2) => ("__arm_mte_get_tag", vec![Arg::Vn(ptr)]),
        (b"StorePtrTag", &[granule, ptr]) if same_value(fd, ptr, granule, 2) => ("__arm_mte_set_tag", vec![Arg::Vn(ptr)]),
        _ => return None,
    };
    Some(Intrinsic { name, args })
}

/// Are `a` and `b` one varnode, equal constants, or the results of the same
/// address arithmetic over inputs that are, to `depth` levels? Two reads of a frame
/// address are separate varnodes computing one value.
fn same_value(fd: &Funcdata, a: VarnodeId, b: VarnodeId, depth: u32) -> bool {
    if a == b {
        return true;
    }
    let (Some(va), Some(vb)) = (fd.vbank().get(a), fd.vbank().get(b)) else { return false };
    if va.is_constant() || vb.is_constant() {
        return va.is_constant() && vb.is_constant() && va.get_offset() == vb.get_offset() && va.get_size() == vb.get_size();
    }
    if depth == 0 {
        return false;
    }
    let (Some(da), Some(db)) = (va.get_def().and_then(|d| fd.obank().get(d)), vb.get_def().and_then(|d| fd.obank().get(d))) else {
        return false;
    };
    let address_arithmetic = matches!(
        da.code(),
        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB | OpCode::CPUI_PTRADD
    );
    address_arithmetic
        && da.code() == db.code()
        && da.num_input() == db.num_input()
        && (0..da.num_input()).all(|i| match (da.get_in(i), db.get_in(i)) {
            (Some(x), Some(y)) => same_value(fd, x, y, depth - 1),
            _ => false,
        })
}

/// The `CRm` field a barrier's two-bit domain and type operands were split from.
fn crm(domain: u64, types: u64) -> Option<u64> {
    (domain < 4 && types < 4).then_some(domain << 2 | types)
}

/// The `CRm` field of the `isb` at `addr`, which its user op does not carry.
fn isb_option(arch: &Architecture, addr: &Address) -> Option<u64> {
    let mut bytes = [0u8; 4];
    let loader_rc = arch.translate().loader_rc();
    loader_rc.try_borrow_mut().ok()?.load_fill(&mut bytes, addr).ok()?;
    isb_crm(u32::from_le_bytes(bytes))
}

/// The `CRm` field of `word` when it encodes `isb`.
fn isb_crm(word: u32) -> Option<u64> {
    (word & 0xffff_f0ff == 0xd503_30df).then_some(u64::from(word >> 8 & 0xf))
}

#[cfg(test)]
#[path = "kuna_acleintrinsics/tests.rs"]
mod tests;
