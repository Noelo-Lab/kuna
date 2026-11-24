//! (kuna deobf) The deobfuscation plane's stage hooks.
//!
//! A deobfuscator is not tied to one phase.  A protector that replaces a
//! function's code (a VM, a packer, a dispatcher loop) has to be undone
//! before anything is lifted, because every later stage takes the flow graph
//! for the program's; an obfuscation that only respells values (mixed
//! boolean-arithmetic, opaque predicates) is cheapest to undo on SSA, where
//! constants are already folded.  So the pipeline calls the plane at fixed
//! points, and each [`Deobfuscator`] implements the ones it needs:
//!
//! * [`front`], before the flow follower: a deobfuscator may recover a body
//!   ([`RecoveredBody`]), a synthetic p-code program the translator's overlay
//!   serves in place of the function's bytes.
//! * [`after_flow`], once the plain flow is known: the same, for a
//!   deobfuscator whose detection reads the function's own ops.  The flow is
//!   then followed again from the recovered body.
//! * the rule pool (`infra/universalaction.rs`): rules a deobfuscator
//!   contributes, each gated by the body's [`BodyPolicy`].
//! * [`audit`], once the function is decompiled: what the result still holds
//!   that its reader must be told.
//!
//! Between the front and everything after it stands one contract: the stages
//! read the body's facts and policy from [`Funcdata::kuna_body`] and never
//! ask which deobfuscator produced it.  Adding a deobfuscator is a module, a
//! row in [`DEOBFUSCATORS`], and whatever it states about the bodies it
//! recovers.  A stage with no hook yet gets one as a method here with an
//! empty default and one call from that stage.
//!
//! Recovered bodies are cached by function entry, since a restart follows the
//! flow again.  An option that changes what a deobfuscator recovers clears
//! [`Architecture::deobf_bodies`].

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_sleigh::kuna_pcodeoverlay::PcodeOverlay;

use super::kuna_body::RecoveredBody;
use crate::architecture::Architecture;
use crate::funcdata::Funcdata;

/// The instruction bound of a flow followed from a recovered body, whose
/// length is the protected program's and not its stub's.
pub const RECOVERED_MAX_INSTRUCTIONS: u32 = 4_000_000;

pub trait Deobfuscator: Sync {
    /// The option that switches it, which also heads its header lines.
    fn name(&self) -> &'static str;

    /// The body to lift in place of the function's bytes, when it recovers one.
    fn front(&self, _arch: &mut Architecture, _fd: &mut Funcdata) -> Option<RecoveredBody> {
        None
    }

    /// The same, decided from the function's plain flow.
    fn after_flow(&self, _arch: &mut Architecture, _fd: &Funcdata) -> Option<RecoveredBody> {
        None
    }

    /// Report what the decompiled function still holds of a body it recovered.
    fn audit(&self, _arch: &Architecture, _fd: &mut Funcdata, _body: &RecoveredBody) {}
}

/// Every deobfuscator, in the order the front asks them.
pub static DEOBFUSCATORS: &[&dyn Deobfuscator] = &[&super::kuna_pushan::Pushan];

/// Decide what the flow follower lifts for `fd`: the body a deobfuscator
/// recovers for it, the first that does, or its own bytes.  Installs the
/// body's overlay (clearing a stale one otherwise) and hands `fd` its facts.
pub fn front(arch: &mut Architecture, fd: &mut Funcdata) -> Option<Rc<RecoveredBody>> {
    let entry = fd.get_address().get_offset();
    let body = arch.deobf_bodies.get(&entry).cloned().or_else(|| {
        let body = Rc::new(DEOBFUSCATORS.iter().find_map(|d| d.front(arch, fd))?);
        arch.deobf_bodies.insert(entry, Rc::clone(&body));
        Some(body)
    });
    install(arch, body.as_ref().map(|b| Rc::clone(&b.overlay)));
    if let Some(body) = &body {
        adopt(arch, body, fd);
    }
    body
}

/// Whether a deobfuscator recovered a body from `fd`'s plain flow, which the
/// caller then follows again.  A front-end whose translator is not SLEIGH
/// (Ghidra's) cannot serve an overlay and is never asked.
pub fn after_flow(arch: &mut Architecture, fd: &Funcdata) -> bool {
    let entry = fd.get_address().get_offset();
    if fd.kuna_recovered().is_some() || arch.translate().as_sleigh().is_none() || arch.deobf_bodies.contains_key(&entry) {
        return false;
    }
    let Some(body) = DEOBFUSCATORS.iter().find_map(|d| d.after_flow(arch, fd)) else { return false };
    arch.deobf_bodies.insert(entry, Rc::new(body));
    true
}

/// Let the deobfuscator that recovered `fd`'s body report on the result.
pub fn audit(arch: &Architecture, fd: &mut Funcdata) {
    if fd.kuna_recovered().is_none() {
        return;
    }
    let Some(body) = arch.deobf_bodies.get(&fd.get_address().get_offset()).cloned() else { return };
    for d in DEOBFUSCATORS.iter().filter(|d| d.name() == body.by) {
        d.audit(arch, fd, &body);
    }
}

/// Point `fd` at its recovered body: the flow follower lifts the overlay
/// instead of a byte range, and the later stages read the body's facts.  The
/// body's string literals are planted and, when that moved the symbol table,
/// the function takes a fresh view of it: the one it was built with predates
/// them.
fn adopt(arch: &mut Architecture, body: &Rc<RecoveredBody>, fd: &mut Funcdata) {
    if plant_strings(arch, &body.strings) {
        fd.kuna_set_arch(arch.build_arch_handle_for_entry(Some(fd.get_address())));
    }
    fd.kuna_set_size(0);
    fd.kuna_set_body(Some(Rc::clone(body)));
}

/// Point the translator at `overlay`. Through the shared handle: taking the
/// translator mutably resets per-image state other passes keep.
fn install(arch: &mut Architecture, overlay: Option<Rc<PcodeOverlay>>) {
    if let Some(s) = arch.translate().as_sleigh() {
        if overlay.is_some() || s.pcode_overlay().is_some() {
            s.set_pcode_overlay(overlay);
        }
    }
}

/// Define a typelocked `char[len]` at each literal a recovered body refers
/// to, so its constant pointers print as strings: a read-only string symbol
/// is otherwise found only through an instruction operand, which in a
/// protected image exists only inside the bytecode. True when one was planted.
fn plant_strings(arch: &mut Architecture, strings: &[(u64, u32)]) -> bool {
    let (Some(global), Some(space)) = (arch.symboltab.get_global_scope(), arch.manage().get_default_data_space().cloned()) else {
        return false;
    };
    let mut planted = false;
    for &(off, len) in strings {
        let addr = Address::new(Rc::clone(&space), off);
        let occupied = arch.symboltab.find_function(global, &addr).is_some()
            || arch.is_lazy_entry(off)
            || arch.symboltab.find_container(global, &addr, 1, &Address::new_invalid()).is_some();
        if occupied || !arch.is_global_read_only(&addr, len as int4) {
            continue;
        }
        let types = arch.types();
        let Ok(arr) = types.get_type_char(types.get_size_of_char()).and_then(|ch| types.get_type_array(len as i32, ch)) else {
            continue;
        };
        if let Ok((sid, _)) = arch.symboltab.add_symbol_mapped(global, &format!("s_{off:x}"), arr, &addr, &Address::new_invalid()) {
            arch.symboltab.set_attribute(sid, crate::varnode::varnode_flags::typelock);
            planted = true;
        }
    }
    planted
}
