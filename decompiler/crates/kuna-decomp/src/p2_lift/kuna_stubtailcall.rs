//! (kuna `stubtailcall`) Flow a loader-proven lazy-binding stub as a tail call.
//!
//! A MIPS `.MIPS.stubs` entry loads the lazy resolver, saves the caller's return
//! address in t7, calls the resolver and passes the dynamic-symbol index in the
//! delay slot. The resolver binds the import and jumps to it with that return
//! address restored, so control never comes back to the stub. Decoded as an
//! ordinary call, flow falls through into the next stub, or past the end of the
//! section after the last one. The object loader validates each
//! stub (`kuna_mipsstubs`); with the option on (the default), flow keeps the
//! resolver call and ends the stub with a return after it. An explicit
//! per-function flow override always wins.

use kuna_base::address::Address;
use kuna_base::types::uint4;

use crate::context::ArchContext;
use crate::overrides::flow_type;

/// Whether any loader-proven stub transfer can apply to this run.
pub fn active(arch: &ArchContext) -> bool {
    arch.stub_tail_call && !arch.loader_transfer_sites.is_empty()
}

/// The flow override for the instruction at `addr`: `explicit` when one is
/// set, otherwise a call-and-return at a loader-proven stub transfer.
pub fn flow_override(arch: &ArchContext, addr: &Address, explicit: uint4) -> uint4 {
    if explicit == flow_type::NONE && arch.loader_transfer_sites.contains(addr) {
        flow_type::CALL_RETURN
    } else {
        explicit
    }
}
