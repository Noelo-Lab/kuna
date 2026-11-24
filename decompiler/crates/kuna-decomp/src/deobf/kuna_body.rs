//! (kuna deobf) What a recovered body tells the stages after it.
//!
//! A deobfuscator acting at the front of decompilation
//! ([`kuna_deobf::front`](super::kuna_deobf::front)) hands the flow follower
//! a synthetic program in place of the function's bytes.  Such a program is
//! not compiler output, and the stages after the flow have to know where it
//! differs.  They read that from [`Funcdata::kuna_body`], in two parts:
//! [`BodyFacts`], which are true of the program (what its calls do to
//! storage the prototype model cannot speak for, which frame addresses its
//! calls receive), and [`BodyPolicy`], which says how a body of its kind has
//! to be analysed.  No stage asks which deobfuscator recovered the body; a
//! function lifted from its own bytes reads [`NATIVE`], with no facts and
//! every policy off.
//!
//! The functions here are the questions the later stages ask, each named for
//! its one caller.

use std::any::Any;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::{int4, uint4, uintb};
use kuna_num::opcodes::OpCode;
use kuna_sleigh::kuna_pcodeoverlay::PcodeOverlay;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::fspec::effect_type;
use crate::funcdata::Funcdata;

/// A function body recovered by a deobfuscator, for one function entry.
pub struct RecoveredBody {
    /// The deobfuscator that recovered it
    /// ([`Deobfuscator::name`](super::kuna_deobf::Deobfuscator::name)).
    pub by: &'static str,
    /// The synthetic program the flow follower lifts.
    pub overlay: Rc<PcodeOverlay>,
    pub facts: BodyFacts,
    /// Read-only string literals the program refers to by a constant pointer:
    /// (address, bytes including the terminator).
    pub strings: Vec<(u64, u32)>,
    /// What the function's header says of the recovery, in order.
    pub header: Vec<String>,
    /// The deobfuscator's own record of the recovery, for its audit.
    pub detail: Box<dyn Any>,
}

/// What is true of a recovered body.
#[derive(Default)]
pub struct BodyFacts {
    /// Register ranges every external call kills, over and above the
    /// prototype model's own effect list.
    pub call_kills: Vec<(uintb, int4)>,
    /// Register ranges every external call leaves untouched.
    pub call_preserves: Vec<(uintb, int4)>,
    /// The lowest stack offset, from the entry stack pointer, any external
    /// call is made at: every call overwrites what lies below it.
    pub call_floor: Option<i64>,
    /// Entry-stack offsets whose address an external call receives, whether
    /// or not the store that passed it survives in the body.
    pub escapes: Vec<i64>,
    /// Register ranges a COPY out of which moves no value of the program's.
    pub incidental_from: Vec<(u64, u8)>,
    /// Register ranges a COPY into which moves no value of the program's.
    pub incidental_to: Vec<(u64, u8)>,
    pub policy: BodyPolicy,
}

/// How the stages after the flow analyse a recovered body.
#[derive(Clone, Copy, Default)]
pub struct BodyPolicy {
    /// Every STORE that may reach a stack range guards it (heritage).
    pub guard_stack_stores: bool,
    /// A stack argument slot heritage split into halves is passed whole
    /// (heritage).
    pub join_stack_inputs: bool,
    /// A LOAD guard found through a spilled pointer stops only at a slot
    /// spilled to directly (heritage).
    pub direct_spills: bool,
    /// Call argument trials are judged by their data flow, without the
    /// local-alias test (call prototypes).
    pub inputs_by_dataflow: bool,
    /// The structure is rebuilt once dead code may be removed in every
    /// space (structuring).
    pub restructure_after_deadcode: bool,
    /// The rules for arithmetic residue apply (rule pool).
    pub residue_rules: bool,
}

/// The facts and policy of a function lifted from its own bytes.
pub static NATIVE: BodyFacts = BodyFacts {
    call_kills: Vec::new(),
    call_preserves: Vec::new(),
    call_floor: None,
    escapes: Vec::new(),
    incidental_from: Vec::new(),
    incidental_to: Vec::new(),
    policy: BodyPolicy {
        guard_stack_stores: false,
        join_stack_inputs: false,
        direct_spills: false,
        inputs_by_dataflow: false,
        restructure_after_deadcode: false,
        residue_rules: false,
    },
};

impl RecoveredBody {
    /// A body with nothing but its analysis policy.
    #[cfg(test)]
    pub fn with_policy(policy: BodyPolicy) -> Self {
        RecoveredBody {
            by: "test",
            overlay: Rc::new(PcodeOverlay::new(0)),
            facts: BodyFacts { policy, ..Default::default() },
            strings: Vec::new(),
            header: Vec::new(),
            detail: Box::new(()),
        }
    }
}

fn within(set: &[(uintb, int4)], addr: &Address, size: int4) -> bool {
    if set.is_empty() || addr.get_space().is_none_or(|spc| spc.get_name() != "register") {
        return false;
    }
    let lo = addr.get_offset();
    let hi = lo.saturating_add(size.max(0) as u64);
    set.iter().any(|&(o, s)| o <= lo && hi <= o.saturating_add(s.max(0) as u64))
}

fn stack_offset(addr: &Address) -> Option<i128> {
    let spc = addr.get_space().filter(|spc| spc.get_type() == spacetype::IPTR_SPACEBASE)?;
    let bits = 8 * spc.get_addr_size() as u32;
    let off = addr.get_offset();
    Some(if bits >= 64 { off as i64 as i128 } else { ((off << (64 - bits)) as i64 >> (64 - bits)) as i128 })
}

/// The effect every external call of the body has on `addr:size`, where the
/// prototype model states none: killed in a range the body names, untouched
/// in one it keeps, killed below the lowest stack pointer any call is made
/// with.  Consulted by `Heritage::guardCalls`.
pub fn call_effect(fd: &Funcdata, addr: &Address, size: int4) -> Option<uint4> {
    let facts = fd.kuna_body();
    if within(&facts.call_kills, addr, size) {
        return Some(effect_type::KILLEDBYCALL);
    }
    if within(&facts.call_preserves, addr, size) {
        return Some(effect_type::UNAFFECTED);
    }
    let below = size > 0 && facts.call_floor.zip(stack_offset(addr)).is_some_and(|(floor, off)| off + size as i128 <= floor as i128);
    below.then_some(effect_type::KILLEDBYCALL)
}

/// Does every STORE that may reach the stack range at `addr` guard it, as
/// upstream `Heritage::guardStores` does?  Consulted by `Heritage::guard`.
pub fn guards_every_store(fd: &Funcdata, addr: &Address) -> bool {
    fd.kuna_body().policy.guard_stack_stores && addr.get_space().is_some_and(|spc| spc.get_type() == spacetype::IPTR_SPACEBASE)
}

fn escape_offset(fd: &Funcdata, addr: &Address) -> Option<i128> {
    if fd.kuna_body().escapes.is_empty() {
        return None;
    }
    stack_offset(addr)
}

/// Is the stack range `addr:size` at or above the lowest local whose address
/// an external call receives (below the return address)?  An unmapped slot
/// there never gets `nolocalalias`.  Consulted by `syncVarnodesWithSymbols`.
pub fn frame_escaped(fd: &Funcdata, addr: &Address, size: int4) -> bool {
    let Some(&lo) = fd.kuna_body().escapes.iter().filter(|o| **o < 0).min() else { return false };
    let Some(off) = escape_offset(fd, addr) else { return false };
    off + size.max(1) as i128 > lo as i128 && off < 0
}

/// Does the stack range `addr:size` hold an address an external call
/// receives?  The local-alias mapping counts each such address as a pointer
/// into the frame, but marks a symbol that starts below the address
/// unaliased; that slot never gets `nolocalalias` either.  Consulted by
/// `syncVarnodesWithSymbols`.
pub fn holds_escape(fd: &Funcdata, addr: &Address, size: int4) -> bool {
    let Some(off) = escape_offset(fd, addr) else { return false };
    fd.kuna_body().escapes.iter().any(|&o| off <= o as i128 && (o as i128) < off + size.max(1) as i128)
}

/// Mark every COPY the body's facts call incidental, once the flow follower
/// has built the ops.  The parameter analysis then walks through the copy
/// into its source instead of taking it as a solid movement of a value into
/// an argument register.
pub fn mark_incidental_copies(fd: &mut Funcdata) {
    let Some(body) = fd.kuna_recovered().cloned() else { return };
    let (from_set, to_set) = (&body.facts.incidental_from, &body.facts.incidental_to);
    let within = |set: &[(u64, u8)], off: u64, size: i32| set.iter().any(|&(o, s)| off >= o && off + size as u64 <= o + s as u64);
    let copies: Vec<OpId> = fd.obank().iter_alive().filter(|&o| fd.obank().get(o).is_some_and(|op| op.code() == OpCode::CPUI_COPY)).collect();
    for op in copies {
        let reg = |v: Option<VarnodeId>| v.and_then(|v| fd.vbank().get(v)).filter(|v| v.get_space().get_name() == "register").map(|v| (v.get_offset(), v.get_size()));
        let from = reg(fd.obank().get(op).and_then(|o| o.get_in(0)));
        let to = reg(fd.obank().get(op).and_then(|o| o.get_out()));
        let incidental = from.is_some_and(|(off, size)| within(from_set, off, size)) || to.is_some_and(|(off, size)| within(to_set, off, size));
        if incidental {
            if let Some(o) = fd.obank_mut().get_mut(op) {
                o.set_additional_flag(crate::op::pcodeop_addlflags::incidental_copy);
            }
        }
    }
}

/// Must the main loop's structuring of the body be built again?  The first
/// structuring runs before memory is heritaged, while every block still
/// holds the stores the recovery left dead, so a loop head that tests one
/// comparison counts as complex and takes the `while (true)` overflow form.
/// The structure is rebuilt once, on the first pass at which dead code may be
/// removed in every space.  Consulted by `ActionBlockStructure`.
pub fn restructure_settled(fd: &Funcdata) -> bool {
    fd.kuna_body().policy.restructure_after_deadcode && !fd.is_high_on() && fd.get_heritage_pass() == fd.deadcode_settled_pass()
}

/// Record the blocks whose branch a structuring of the body negated, so the
/// next structuring can start from the emitted orientation.
pub fn record_structure_flips(fd: &mut Funcdata, flips: &[BlockId]) {
    if fd.kuna_body().policy.restructure_after_deadcode {
        fd.kuna_structure_flips_mut().extend_from_slice(flips);
    }
}

/// Negate again every branch the discarded structure of the body negated,
/// before the body is structured anew.  Each structuring then starts from the
/// emitted orientation, as a native function's only one does, rather than
/// from its predecessor's choices: a rule that takes the false edge first
/// would otherwise pick the other arm of a two-exit split on every rebuild.
pub fn unflip_structure(fd: &mut Funcdata) {
    let flips = std::mem::take(fd.kuna_structure_flips_mut());
    if flips.is_empty() {
        return;
    }
    let live: std::collections::BTreeSet<BlockId> = (0..fd.bblocks_get_size()).map(|i| fd.bblocks_get_block(i)).collect();
    for bb in flips {
        if !live.contains(&bb) || fd.bblocks_ref().block(bb).size_out() != 2 {
            continue;
        }
        let cbranch = fd.bb_op_tail(bb).and_then(|op| fd.obank().get(op)).is_some_and(|op| op.code() == OpCode::CPUI_CBRANCH);
        if cbranch {
            fd.block_basic_negate_lastop(bb);
        }
    }
}

#[cfg(test)]
mod tests;
