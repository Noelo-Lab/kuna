//! (kuna `--assert callee`, console `override indirect`) State where an
//! indirect call goes.
//!
//! A reader who knows the destination of `call *(%rax)` states it as
//! `callee [<func>::]<callsite> <name|address>`. [`plant`] checks the fact
//! against the image and parks it for the engine
//! (`kuna_decomp::kuna_calleeoverride`), which rebuilds the call as a direct
//! call so the callee's name and prototype apply to it.
//!
//! The call site must decode to an indirect call or an indirect jump; a direct
//! call, any other instruction and a destination that is not a function are
//! refused.

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::{PcodeEmit, Translate};

use crate::engine::{ConsoleProgram, EntryLookupError, EntrySelector};

/// The p-code control transfers of one decoded instruction.
#[derive(Default)]
struct Transfers {
    direct_call: bool,
    indirect_call: bool,
    indirect_jump: bool,
}

impl PcodeEmit for Transfers {
    fn dump(&mut self, _addr: &Address, opc: OpCode, _out: Option<&VarnodeData>, _vars: &[VarnodeData]) {
        match opc {
            OpCode::CPUI_CALL => self.direct_call = true,
            OpCode::CPUI_CALLIND => self.indirect_call = true,
            OpCode::CPUI_BRANCHIND => self.indirect_jump = true,
            _ => {}
        }
    }
}

/// Accept `site` when it decodes to an indirect call or an indirect jump.
fn check_site(trans: &dyn Translate, site: &Address) -> Result<(), String> {
    let mut seen = Transfers::default();
    let decoded = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        trans.one_instruction(&mut seen, site)
    }));
    if !matches!(decoded, Ok(Ok(len)) if len > 0) {
        return Err(format!("no instruction decodes at {:#x}", site.get_offset()));
    }
    if seen.indirect_call || (seen.indirect_jump && !seen.direct_call) {
        Ok(())
    } else if seen.direct_call {
        Err(format!(
            "the instruction at {:#x} is a direct call; its destination is already known",
            site.get_offset()
        ))
    } else {
        Err(format!("the instruction at {:#x} is not a call", site.get_offset()))
    }
}

/// The entry of the function `token` names: a function name, or a `0x` address
/// that is a function entry.
fn callee_entry(prog: &ConsoleProgram, token: &str) -> Result<Address, String> {
    match EntrySelector::parse(token) {
        EntrySelector::Numeric(vma) => prog
            .find_entry_at(vma)
            .map(|entry| entry.addr)
            .ok_or_else(|| format!("{vma:#x} is not a function entry")),
        selector => match prog.resolve_entry(&selector) {
            Ok(entry) => Ok(entry.addr),
            Err(EntryLookupError::NotFound { .. }) => Err(format!("{token:?} does not name a function")),
            Err(other) => Err(other.to_string()),
        },
    }
}

/// Park the fact that the indirect call at `site`, in the function entered at
/// `entry`, goes to the function `callee` names.
pub fn plant(
    prog: &mut ConsoleProgram,
    entry: &Address,
    site: &Address,
    callee: &str,
) -> Result<(), String> {
    crate::jump_targets::require_unstated(prog, site)?;
    check_site(prog.arch().translate(), site)?;
    let target = callee_entry(prog, callee)?;
    prog.arch_mut()
        .kuna_callee_overrides
        .entry(entry.clone())
        .or_default()
        .insert(site.clone(), target);
    Ok(())
}
