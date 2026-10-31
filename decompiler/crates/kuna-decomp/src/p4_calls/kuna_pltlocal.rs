//! (kuna) `pltlocal` — a call through a PLT stub the image binds to its own
//! definition is a call to that definition.
//!
//! A shared library calls its own default-visibility functions through
//! `name@plt`. The loader names the stub after the relocation's symbol and
//! registers it as an import (`kuna-analysis` `loader::elf_plt`), so a call's
//! entry address is the stub. Every callee fact the engine keys by entry address
//! (the `protoorder` statement, the bounded callee-body probes, the declared
//! prototype, the callee-first order) is about the stub, whose body is one jump
//! through the GOT and says nothing. `bl timer_helper@plt` forwarding the
//! caller's own x0..x3 to a helper that reads all four then renders
//! `timer_helper();`, while the `-Bsymbolic` build of the same code renders
//! `timer_helper(a0,a1,a2,a3)`.
//!
//! The loader records, per stub, the function the slot's dynamic relocation
//! binds it to (`loader::kuna_pltlocal`, joined by relocation symbol, never by
//! name). With the option on, flow builds the call spec of a direct call to
//! such a stub at the definition's entry: the printed name and the flow effects
//! (no-return, inline, injection) stay the stub's. A stub that declares a
//! prototype of its own (a library signature seeded on the import, an
//! `--assert prototype` at the stub) keeps the call where it was: the
//! declaration already states what the call passes. The definition's body
//! answers what it reads and returns, never what it leaves alone: a compiler
//! emitting `bl name@plt` cannot keep a value in a call-clobbered register
//! across it (the lazy resolver runs first, and another definition may win), so
//! `calleepreserves` and `calleeretpreserves` take no proof across a stub.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;

use crate::infra::architecture::Architecture;
use crate::p0_knowledge::options::on_or_off;

/// `option pltlocal on|off`.
pub struct OptionPltLocal;

impl OptionPltLocal {
    /// The option name.
    pub const NAME: &'static str = "pltlocal";

    /// Resolve the flag and its confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Calls through a PLT stub bound to the image's own definition turned {prop}")))
    }
}

/// The definition the PLT stub at `entry` is bound to, in `entry`'s space, when
/// the option is on, the stub declares no prototype of its own, and the symbol
/// table knows a function there.
pub fn definition(arch: &Architecture, entry: &Address) -> Option<Address> {
    if !arch.plt_local_calls || arch.plt_local.is_empty() {
        return None;
    }
    let target = target_of(&arch.plt_local, entry.get_offset())?;
    if arch.symboltab.function_proto_pieces_across_scopes(entry).is_some() {
        return None;
    }
    let space = entry.get_space()?;
    let def = Address::new(std::rc::Rc::clone(space), target);
    arch.symboltab.function_display_name_across_scopes(&def).map(|_| def)
}

/// The definition `stub` is bound to in `table` (ascending by stub).
pub fn target_of(table: &[(u64, u64)], stub: u64) -> Option<u64> {
    table.binary_search_by_key(&stub, |&(s, _)| s).ok().map(|i| table[i].1)
}

#[cfg(test)]
mod tests;
