//! (kuna `callerreads`) A function decompiled alone returns what its callers
//! read after the call (P4).
//!
//! `int f(int a) { gi = a * 3; return gi; }` and `void f(int a) { gi = a * 3; }`
//! both compile to `lea (%rdi,%rdi,2),%eax; mov %eax,gi(%rip); ret`. Upstream's
//! `onlyOpUse` refuses a return value the function also stores, so the function
//! alone reads as `void`. `decompile-all` settles it from the callers it
//! decompiles (`kuna_voidret`); a single-function decompile had no caller, and
//! `kuna decompile ./rv f` printed `void f(int a0)` while `kuna decompile ./rv
//! main` printed `printf("%d\n",f(2))`.
//!
//! The Listing files where each direct call returns to
//! (`Architecture::kuna_call_returns`). For a function recovered `void`, [`read`]
//! decodes each of its callers from that address, the way `calleedeadarg`
//! decodes a callee from its entry, and asks whether a path reads the low byte
//! of one of the model's return registers before writing it. The ABI leaves that
//! register clobbered by the call, so a read there is the call's result. The
//! decompile step then forces the return the way `decompile-all` does
//! ([`force`]), and keeps the redo only when it returns a value.

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::funcdata::Funcdata;
use crate::infra::architecture::Architecture;

/// The most call sites of one function [`read`] decodes.
pub const MAX_CALL_SITES: usize = 32;

/// The return storage the callers of `data`'s function read after their calls
/// to it, when that function was recovered `void` and has no declared or locked
/// output: the widest read of the first model output register a caller reads,
/// when every caller that reads one reads the same register.
pub fn read(arch: &Architecture, data: &Funcdata) -> Option<(Address, int4)> {
    if !arch.caller_reads || arch.kuna_call_returns.is_empty() {
        return None;
    }
    let entry = data.get_address();
    let code = entry.get_space().filter(|s| s.get_type() == spacetype::IPTR_PROCESSOR)?;
    let proto = data.get_func_proto();
    if !proto.has_model()
        || !proto.has_store()
        || proto.is_output_locked()
        || arch.symboltab.function_proto_pieces_across_scopes(entry).is_some()
        || proto
            .get_output_type()
            .is_some_and(|t| t.get_metatype() != crate::dtype::type_metatype::TYPE_VOID)
    {
        return None;
    }
    let returns = data
        .obank()
        .iter_code(OpCode::CPUI_RETURN)
        .any(|r| data.obank().get(r).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0));
    if !returns {
        return None;
    }
    let reg = arch.manage().get_space_by_name("register")?;
    let outputs: Vec<(Address, int4)> = proto
        .model()
        .output()
        .get_entry()
        .iter()
        .filter(|e| e.get_space().get_index() == reg.get_index() && e.get_size() > 0)
        .map(|e| (Address::new(std::rc::Rc::clone(e.get_space()), e.get_base()), e.get_size()))
        .collect();
    if outputs.is_empty() {
        return None;
    }
    let off = entry.get_offset();
    let lo = arch.kuna_call_returns.partition_point(|&(callee, _)| callee < off);
    let mut found: Option<(usize, int4)> = None;
    let sites = arch.kuna_call_returns[lo..].iter().take_while(|&&(callee, _)| callee == off);
    for &(_, back) in sites.take(MAX_CALL_SITES) {
        let at = Address::new(std::rc::Rc::clone(code), back);
        let probe = crate::p4_calls::kuna_calleedeadarg::probe_entry(arch.translate(), &at, reg.get_index(), false);
        let read = outputs.iter().enumerate().find_map(|(k, (addr, size))| Some((k, probe.low_read_width(addr, *size)?)));
        let Some(read) = read else { continue };
        match &mut found {
            None => found = Some(read),
            Some((k, _)) if *k != read.0 => return None,
            Some((_, width)) => *width = (*width).max(read.1),
        }
    }
    let (k, width) = found?;
    let (addr, size) = &outputs[k];
    let low = if addr.is_big_endian() { addr + i64::from(size - width) } else { addr.clone() };
    Some((low, width))
}

/// Have the next decompile of the function at `entry` force its return to
/// `storage`, as `kuna_voidret` does for a function whose callers read it.
pub fn force(arch: &mut Architecture, entry: &Address, storage: (Address, int4)) {
    if let Some(space) = entry.get_space() {
        arch.kuna_voidret.forced.insert((space.get_index(), entry.get_offset()), storage);
    }
}

/// Undo [`force`].
pub fn release(arch: &mut Architecture, entry: &Address) {
    if let Some(space) = entry.get_space() {
        arch.kuna_voidret.forced.remove(&(space.get_index(), entry.get_offset()));
    }
}

/// Does the finished function return a value?
pub fn returns_value(data: &Funcdata) -> bool {
    data.get_func_proto()
        .get_output_type()
        .is_some_and(|t| t.get_metatype() != crate::dtype::type_metatype::TYPE_VOID)
}
