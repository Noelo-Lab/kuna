//! Follow canonical PowerPC PC reads into a body owned by the current function.
//!
//! `bcl 20,31,target` is the ISA's PC-read form. Branching to the next
//! instruction cannot be a call. Skipping one to three literal words is a PC read
//! only when the code at `target` goes on to read LR (`mflr`) before returning
//! through LR, overwriting it or calling, and then restores LR (`mtlr`) before
//! any return through it. A forward `bcl 20,31` to a helper that returns through
//! LR, before or after reading it, keeps its call semantics.
use crate::{context::OpId, funcdata::Funcdata};
use kuna_base::address::Address;

/// The longest inline literal a PC read branches over, in bytes after the `bcl`.
const MAX_SKIP: u64 = 12;
/// How many instructions the LR read may be scheduled after the target.
const LR_READ_WINDOW: usize = 32;
/// How many instructions after the LR read are searched for an LR return.
const LR_RESTORE_WINDOW: usize = 64;
const MFLR: u32 = 0x7c08_02a6;
const MTLR: u32 = 0x7c08_03a6;
const LR_MOVE_MASK: u32 = 0xfc1f_ffff;

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

/// The fall-through path from `target` reads LR before anything could consume or
/// replace it, and does not return through LR before restoring it.
fn reads_lr(fd: &Funcdata, target: &Address, body: (u64, u64)) -> bool {
    let mut path = FallThrough { fd, pc: Some(target.clone()), body };
    for word in path.by_ref().take(LR_READ_WINDOW) {
        if word & LR_MOVE_MASK == MFLR {
            return path
                .take(LR_RESTORE_WINDOW)
                .find_map(|word| {
                    if word & LR_MOVE_MASK == MTLR {
                        Some(true)
                    } else if word >> 26 == 19 && (word >> 1) & 0x3ff == 16 {
                        Some(false)
                    } else {
                        None
                    }
                })
                .unwrap_or(true);
        }
        let op = word >> 26;
        let xo = (word >> 1) & 0x3ff;
        if matches!(op, 3 | 17)
            || op == 18 && word & 3 != 0
            || op == 16 && word & 1 != 0
            || op == 19 && matches!(xo, 16 | 528 | 560)
            || op == 31 && xo == 4
            || word & LR_MOVE_MASK == MTLR
        {
            return false;
        }
    }
    false
}

/// Instruction words on the fall-through path inside `body`, following direct
/// relative branches and ending at an absolute or CTR/TAR jump.
struct FallThrough<'a> {
    fd: &'a Funcdata,
    pc: Option<Address>,
    body: (u64, u64),
}

impl Iterator for FallThrough<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let pc = self.pc.take()?;
        if pc.get_offset() < self.body.0 || pc.get_offset() >= self.body.1 {
            return None;
        }
        let word = word_at(self.fd, &pc)?;
        let (op, xo, link) = (word >> 26, (word >> 1) & 0x3ff, word & 1 != 0);
        self.pc = if op == 18 && word & 3 == 0 {
            Some(&pc + i64::from(((word & 0x03ff_fffc) << 6) as i32 >> 6))
        } else if !link && (op == 18 || op == 19 && matches!(xo, 528 | 560)) {
            None
        } else {
            Some(&pc + 4)
        };
        Some(word)
    }
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
