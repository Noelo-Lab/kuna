//! Follow canonical PowerPC PC reads into a body owned by the current function.
//!
//! `bcl 20,31,target` is the ISA's PC-read form. Branching to the next
//! instruction cannot be a call. Skipping one to three literal words is a PC read
//! only when the code at `target` goes on to read LR (`mflr`) before returning
//! through LR, overwriting it or calling, and then loads one of the skipped words
//! through the register the `mflr` wrote before that register is overwritten or
//! the path returns, calls or traps. The path must also never move that register
//! back into LR or return through LR before restoring it, since either resumes at
//! the skipped words. A forward `bcl 20,31` to a helper that never loads those
//! words, or that returns into them, keeps its call semantics.
use crate::{context::OpId, funcdata::Funcdata};
use kuna_base::address::Address;

/// The longest inline literal a PC read branches over, in bytes after the `bcl`.
const MAX_SKIP: u64 = 12;
/// How many instructions the LR read may be scheduled after the target.
const LR_READ_WINDOW: usize = 32;
/// How many instructions after the LR read are searched for the literal load.
const LITERAL_LOAD_WINDOW: usize = 32;
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
        && (dest.get_offset() == site.get_offset() + 4
            || loads_literal(fd, dest.get_addr(), (start, end), dest.get_offset() - site.get_offset() - 4))
}

/// The fall-through path from `target` reads LR before anything could consume or
/// replace it, then loads one of the `skip` bytes after the `bcl` through the
/// register that read.
fn loads_literal(fd: &Funcdata, target: &Address, body: (u64, u64), skip: u64) -> bool {
    let mut path = FallThrough { fd, pc: Some(target.clone()), body };
    for word in path.by_ref().take(LR_READ_WINDOW) {
        if word & LR_MOVE_MASK == MFLR {
            return reads_skipped_word(path.take(LITERAL_LOAD_WINDOW), (word >> 21) & 31, skip);
        }
        if leaves_path(word) || word & LR_MOVE_MASK == MTLR {
            return false;
        }
    }
    false
}

/// After `mflr reg`: a load through `reg` reads a skipped word before `reg` may be
/// overwritten or the path leaves, and LR is neither set from `reg` nor returned
/// through before another register restores it.
fn reads_skipped_word(path: impl Iterator<Item = u32>, reg: u32, skip: u64) -> bool {
    let (mut loaded, mut restored) = (false, false);
    for word in path {
        if word & LR_MOVE_MASK == MTLR {
            if (word >> 21) & 31 == reg {
                return false;
            }
            restored = true;
        } else if !loaded && is_load_from(word, reg, skip) {
            loaded = true;
        } else if leaves_path(word) {
            return loaded && (restored || !(word >> 26 == 19 && (word >> 1) & 0x3ff == 16));
        } else if !loaded && writes_gpr(word, reg) {
            return false;
        }
        if loaded && restored {
            return true;
        }
    }
    loaded
}

/// A return, call, CTR/TAR jump, absolute branch, system call or trap.
fn leaves_path(word: u32) -> bool {
    let (op, xo) = (word >> 26, (word >> 1) & 0x3ff);
    matches!(op, 3 | 17)
        || op == 18 && word & 3 != 0
        || op == 16 && word & 1 != 0
        || op == 19 && matches!(xo, 16 | 528 | 560)
        || op == 31 && xo == 4
}

/// A D-form load based on `reg` whose displacement lies within `skip` bytes.
fn is_load_from(word: u32, reg: u32, skip: u64) -> bool {
    matches!(word >> 26, 32..=35 | 40..=43 | 48..=51)
        && reg != 0
        && (word >> 16) & 31 == reg
        && u64::try_from(word as u16 as i16).is_ok_and(|disp| disp < skip)
}

/// Whether `word` may write general register `reg`; unlisted forms are assumed to
/// write both register fields.
fn writes_gpr(word: u32, reg: u32) -> bool {
    let (op, rt, ra, xo) = (word >> 26, (word >> 21) & 31, (word >> 16) & 31, (word >> 1) & 0x3ff);
    match op {
        3 | 10 | 11 | 16..=19 | 36 | 38 | 44 | 47 | 48 | 50 | 52 | 54 | 59 | 63 => false,
        37 | 39 | 45 | 49 | 51 | 53 | 55 => ra == reg,
        7 | 8 | 12..=15 | 32 | 34 | 40 | 42 => rt == reg,
        46 => rt <= reg,
        31 if matches!(xo, 0 | 4 | 32 | 150 | 151 | 215 | 407 | 661..=663 | 725 | 727 | 918 | 983) => false,
        31 if matches!(xo, 183 | 247 | 439 | 695 | 759) => ra == reg,
        31 if matches!(xo, 533 | 597) => true,
        _ => rt == reg || ra == reg,
    }
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
