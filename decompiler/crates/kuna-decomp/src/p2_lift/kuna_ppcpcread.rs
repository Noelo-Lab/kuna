//! Follow canonical PowerPC PC reads into a body owned by the current function.
use crate::{context::OpId, funcdata::Funcdata};
use kuna_base::address::Address;

pub(crate) fn is_pc_capture(
    fd: &Funcdata,
    op: OpId,
    bodies: &[(u64, u64)],
    known: impl FnOnce(&Address) -> bool,
) -> bool {
    let Some(o) = fd.obank().get(op) else {
        return false;
    };
    let Some(dest) = o.get_in(0).and_then(|v| fd.vbank().get(v)) else {
        return false;
    };
    let site = o.get_addr();
    let entry = fd.get_address().get_offset();
    let n = bodies.partition_point(|&(start, _)| start <= entry);
    let Some(&(start, end)) = n.checked_sub(1).and_then(|i| bodies.get(i)) else {
        return false;
    };
    if entry != start
        || site.get_offset() < start
        || site.get_offset() >= end
        || dest.get_offset() <= site.get_offset()
        || dest.get_offset() >= end
        || dest.get_space().get_index() != site.get_space().unwrap().get_index()
        || known(dest.get_addr())
    {
        return false;
    }
    let mut bytes = [0; 4];
    if fd.get_arch().loader_fill(&mut bytes, site).is_err() {
        return false;
    }
    let word = if site.get_space().unwrap().is_big_endian() {
        u32::from_be_bytes(bytes)
    } else {
        u32::from_le_bytes(bytes)
    };
    word & 0xffff_0003 == 0x429f_0001
}
