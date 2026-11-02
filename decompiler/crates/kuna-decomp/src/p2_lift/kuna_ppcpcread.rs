//! Follow canonical PowerPC PC reads into a body owned by the current function.
//!
//! `bcl 20,31,target` is the ISA's PC-read form. Branching to the next
//! instruction cannot be a call. Skipping one to three literal words is a PC read
//! only when the code at `target` goes on to read LR (`mflr`) before returning
//! through LR, overwriting it or calling; a forward `bcl 20,31` to a helper that
//! returns through LR keeps its call semantics.
use crate::{context::OpId, funcdata::Funcdata};
use kuna_base::address::Address;

/// The longest inline literal a PC read branches over, in bytes after the `bcl`.
const MAX_SKIP: u64 = 12;
/// How many instructions the LR read may be scheduled after the target.
const LR_READ_WINDOW: usize = 32;

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
        || dest.get_offset() - site.get_offset() > 4 + MAX_SKIP
        || dest.get_offset() >= end
        || dest.get_space().get_index() != site.get_space().unwrap().get_index()
        || known(dest.get_addr())
    {
        return false;
    }
    word_at(fd, site).is_some_and(|w| w & 0xffff_0003 == 0x429f_0001)
        && (dest.get_offset() == site.get_offset() + 4 || reads_lr(fd, dest.get_addr(), (start, end)))
}

/// The fall-through path from `target`, following direct unconditional branches
/// inside `body`, reads LR before anything could consume or replace it.
fn reads_lr(fd: &Funcdata, target: &Address, body: (u64, u64)) -> bool {
    let mut pc = target.clone();
    for _ in 0..LR_READ_WINDOW {
        if pc.get_offset() < body.0 || pc.get_offset() >= body.1 {
            return false;
        }
        let Some(word) = word_at(fd, &pc) else {
            return false;
        };
        let op = word >> 26;
        let xo = (word >> 1) & 0x3ff;
        let link = word & 1 != 0;
        if word & 0xfc1f_ffff == 0x7c08_02a6 {
            return true;
        }
        if op == 18 && word & 3 == 0 {
            pc = &pc + i64::from(((word & 0x03ff_fffc) << 6) as i32 >> 6);
            continue;
        }
        if matches!(op, 3 | 17 | 18)
            || op == 16 && link
            || op == 19 && matches!(xo, 16 | 528 | 560)
            || op == 31 && xo == 4
            || word & 0xfc1f_ffff == 0x7c08_03a6
        {
            return false;
        }
        pc = &pc + 4;
    }
    false
}

fn word_at(fd: &Funcdata, at: &Address) -> Option<u32> {
    let mut bytes = [0; 4];
    fd.get_arch().loader_fill(&mut bytes, at).ok()?;
    Some(if at.get_space()?.is_big_endian() {
        u32::from_be_bytes(bytes)
    } else {
        u32::from_le_bytes(bytes)
    })
}
