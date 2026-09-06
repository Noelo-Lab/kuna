//! (kuna) `cookiescramble` — a stack-cookie scramble of the stack pointer is
//! not a pointer escape.
//!
//! # The gap
//!
//! `AliasChecker::gatherAdditiveBase` (`varmap.cc:733`) walks the stack
//! pointer's def/use graph looking for the offsets at which a pointer into the
//! frame escapes.  It follows additive chains (`INT_ADD`, `PTRADD`, `PTRSUB`,
//! `COPY`) and treats **every other** use as evidence that the address the
//! Varnode holds has escaped, recording its frame offset in `alias`.
//! `hasLocalAlias` then answers "is anything at or above the shallowest such
//! offset aliased?", and `FuncCallSpecs::checkInputTrialUse` (`fspec.cc:5616`)
//! uses that answer to score a stack-passed argument trial *no-use* — which
//! replaces the CALL's input with a constant 0 and hands the argument's
//! computation to dead-code elimination.
//!
//! MSVC's `/GS` prologue mixes the stack pointer into the frame cookie:
//!
//! ```text
//!   sub  rsp,0x940
//!   mov  rax,qword ptr [__security_cookie]
//!   xor  rax,rsp                     ; <-- RSP used as a VALUE
//!   mov  qword ptr [rsp+0x930],rax
//! ```
//!
//! `xor rax,rsp` is not an additive use, so the raw stack pointer becomes an
//! escape site at the *bottom* of the frame.  That is the shallowest possible
//! offset, so `hasLocalAlias` answers yes for every stack location in the
//! function — including the outgoing-argument slots, which sit at the bottom by
//! construction.  Every stack-passed argument at every call site in a `/GS`
//! function is scored no-use and dropped.
//!
//! GCC/Clang's `-fstack-protector` reads the cookie from `%fs:0x28` and never
//! touches the stack pointer, which is why the defect is invisible on ELF
//! corpora and shows up on MSVC PEs.
//!
//! # The mechanism
//!
//! An `INT_XOR` of a stack-pointer-derived Varnode does not compute an address:
//! the result is a scrambled integer that no code dereferences.  With the
//! option on, that use no longer marks an escape site, so the alias boundary is
//! decided by the genuine address-forming uses — `lea`, indexed stores,
//! pointers handed to callees.
//!
//! The rule is not conditioned on the second operand.  Whether the cookie is
//! loaded from `__security_cookie` or has been folded to an immediate is a
//! property of the optimizer, not of the aliasing, and a rule that flipped on
//! it would be answering the wrong question.
//!
//! Nothing else about the gather changes.  The exempted use is not followed
//! into the additive chain either — upstream does not follow it, and
//! `gatherOffset` has no `INT_XOR` arm, so a scrambled pointer was never
//! tracked past the scramble in the first place.  The rule can only ever
//! *remove* an escape site that was never an address, never add one; the cost
//! is a deliberately masked pointer (`p ^ mask`, dereferenced after a second
//! `^ mask`), whose base no longer counts as escaped.  Flip the option off to
//! restore upstream's answer there.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;

use crate::p0_knowledge::options::on_or_off;
use kuna_num::opcodes::OpCode;

/// Marshaling element `<cookiescramble>` (kuna 4000+ range; 4148 = the previous
/// high-water mark).
pub const ELEM_COOKIESCRAMBLE: ElementId = ElementId::new("cookiescramble", 4149);

/// (kuna) A stack-pointer scramble is not a pointer escape: `cookiescramble on|off`.
pub struct OptionCookieScramble;

impl OptionCookieScramble {
    /// The option name.
    pub const NAME: &'static str = "cookiescramble";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::cookie_scramble`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Stack-cookie scramble alias exemption turned {prop}")))
    }
}

/// Does this non-additive use of a stack-pointer-derived Varnode record an
/// escape site for the local-alias gather?
///
/// The `gatherAdditiveBase` default arm (`varmap.cc:757`).  With the option on,
/// an `INT_XOR` — the `/GS` cookie scramble — answers `false`; every other use,
/// and the whole arm with the option off, answers `true` as upstream does.
pub fn is_escape_site(enabled: bool, code: OpCode) -> bool {
    !(enabled && code == OpCode::CPUI_INT_XOR)
}

#[cfg(test)]
#[path = "kuna_cookiescramble/tests.rs"]
mod tests;
