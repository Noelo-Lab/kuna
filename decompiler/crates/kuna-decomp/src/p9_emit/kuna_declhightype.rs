//! (kuna `declhightype`) Declare a merged local at the data-type its own body
//! was cast against.
//!
//! Upstream declares a local from the *Symbol* it is mapped to
//! (`PrintC::emitVarDecl`, `printc.cc:1719` `sym->getType()`), and a Symbol's
//! type is the merged `HighVariable`'s type — the type of the member Varnode
//! `HighVariable::getTypeRepresentative` (`variable.cc:377-396`) picks as the
//! most specialized.  That same type is what `ActionSetCasts` queries through
//! `Varnode::getHighTypeReadFacing` when it decides whether a use needs a cast,
//! so declaration and body agree by construction.
//!
//! kuna's printer walks HighVariables rather than Symbols and takes the declared
//! type off its *declaration representative* — the first address-tied member, or
//! member zero.  For a high whose members all carry one type the two
//! representatives agree and nothing changes.  For a merged register high whose
//! members carry different types they are simply two different Varnodes, and the
//! declaration then states a type no use in the body was checked against:
//!
//! ```text
//!   unsigned long long *v9; // rax
//!   v9 = (char *)sub_1400778c4(&v14[0x58],6);
//!   v8 = (uint4)*v9;                            // MOVSX EAX,byte ptr [RAX]
//! ```
//!
//! The cast machinery saw a one-byte pointee, so it left `*v9` uncast; read
//! against the declaration, `*v9` is an eight-byte load — seven bytes past the
//! object, with the sign extension dropped.  The instruction is a one-byte signed
//! load, and every other token on the line already says so.
//!
//! This option declares the type representative's type instead, which is the
//! `sym->getType()` upstream would have written.  It moves no cast and no
//! statement: the body is untouched, and the declaration stops contradicting it.

use kuna_base::error::KunaResult;

use std::rc::Rc;

use crate::context::{HighVariableId, VarnodeId};
use crate::dtype::Datatype;
use crate::funcdata::Funcdata;
use crate::options::on_or_off;

/// (kuna) Toggle the merged-local declaration type: `declhightype on|off`.
///
/// "on" declares the HighVariable's own data-type (its type representative's);
/// "off" keeps the declaration representative's, which for a merged high is
/// whichever member happens to be address-tied or first.
#[derive(Debug, Clone, Copy, Default)]
pub struct OptionDeclHighType;

impl OptionDeclHighType {
    /// The option name.
    pub const NAME: &'static str = "declhightype";

    /// Parse + validate the `on`/`off` value and return the resolved flag plus the
    /// confirmation message (the caller flips `Architecture::decl_high_type`).
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Merged-local declaration type turned {prop}")))
    }
}

/// The member Varnode whose data-type the merged HighVariable carries.
///
/// The read-only transcription of C++ `HighVariable::getTypeRepresentative`
/// (`variable.cc:377-396`), which is what `HighVariable::getType` reads and
/// therefore what every cast `ActionSetCasts` inserted was decided against.  The
/// printer cannot call `getType` itself: that path runs `updateType` and needs a
/// mutable high plus a `HighContext`, neither of which the emitter holds.
pub fn type_representative(fd: &Funcdata, high: HighVariableId) -> Option<VarnodeId> {
    let h = fd.high_bank().get(high)?;
    let members = (0..h.num_instances()).map(|i| h.get_instance(i)).map(|vn| {
        fd.vbank().get(vn).map(|v| Member {
            vn,
            type_lock: v.is_type_lock(),
            ty: Rc::clone(v.get_type()),
        })
    });
    pick_representative(members)
}

/// One candidate member of a merged HighVariable: the two fields
/// `getTypeRepresentative` orders on, plus the Varnode it came from.
pub struct Member {
    /// The member Varnode.
    pub vn: VarnodeId,
    /// `Varnode::isTypeLock()`.
    pub type_lock: bool,
    /// `Varnode::getType()`.
    pub ty: Rc<Datatype>,
}

/// The ordering `HighVariable::getTypeRepresentative` applies: a type-locked
/// member beats an unlocked one, and among members that agree on the lock the
/// most specialized data-type (`Datatype::typeOrder`, formal) wins.  Member zero
/// is the seed, so a tie keeps the earlier member.
///
/// A member the caller could not resolve is skipped rather than treated as a
/// candidate, which is the one place this can differ from upstream — upstream's
/// instance list cannot hold a dangling Varnode.
pub fn pick_representative(members: impl Iterator<Item = Option<Member>>) -> Option<VarnodeId> {
    let mut rep: Option<Member> = None;
    for cand in members {
        let Some(cand) = cand else { continue };
        let Some(cur) = rep.as_ref() else {
            rep = Some(cand);
            continue;
        };
        let better = if cur.type_lock != cand.type_lock {
            cand.type_lock
        } else {
            cand.ty.type_order_formal(&cur.ty).unwrap_or(0) < 0
        };
        if better {
            rep = Some(cand);
        }
    }
    rep.map(|m| m.vn)
}

/// Does the declaration of `high` come from the high itself rather than from a
/// mapped `ScopeLocal` Symbol?
///
/// Upstream declares a mapped local at `sym->getType()` (`printc.cc:1719`), and
/// upstream's type representative IS that Symbol's Varnode, because
/// `Varnode::setSymbolProperties` type-locks the storage a Symbol claims.  kuna
/// does not always carry that lock, so on a mapped local the most specialized
/// member can be some transient the Symbol never described — a DWARF
/// `acl_entry_obj_tag *` re-declared as the `obj_prefix` struct one member was
/// briefly cast to.  A high the Symbol table describes therefore keeps the
/// declaration representative's type, which is the mapped storage's; only a high
/// with no Symbol of its own — the register and unique temporaries the naming
/// pass calls `vN` — takes the merged type.
pub fn declares_from_high(fd: &Funcdata, high: HighVariableId) -> bool {
    fd.high_bank().get(high).is_some_and(|h| h.kuna_symbol_type().is_none())
}

#[cfg(test)]
#[path = "kuna_declhightype/tests.rs"]
mod tests;
