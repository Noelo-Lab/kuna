//! (kuna) A function decompiled alone returns what its callers
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
pub fn read(
    arch: &Architecture,
    data: &Funcdata,
    raw_bound: &dyn Fn(u64, &str) -> Option<int4>,
) -> Option<(Address, int4)> {
    let raw = &arch.kuna_raw_thumb_call_returns;
    if (arch.kuna_call_returns.is_empty() && raw.is_empty())
        || (!raw.is_empty() && !arch.protoorder.is_on())
    {
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
    let off = entry.get_offset();
    let object = &arch.kuna_call_returns;
    let raw_start = raw.partition_point(|&(callee, _, _)| callee < off);
    let object_start = object.partition_point(|&(callee, _)| callee < off);
    let raw_sites = raw[raw_start..].iter().take_while(|&&(callee, _, _)| callee == off);
    let object_sites = object[object_start..].iter().take_while(|&&(callee, _)| callee == off);
    let sites = raw_sites.map(|&(_, back, caller)| (back, Some(caller)))
        .chain(object_sites.map(|&(_, back)| (back, None)));
    let mut sites = sites.take(MAX_CALL_SITES).peekable();
    if sites.peek().is_none() {
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
    let thumb_r0 = if !raw.is_empty() {
        Some(arch.translate().get_register("r0").ok()?)
    } else {
        None
    };
    let outputs: Vec<(Address, int4)> = proto
        .model()
        .output()
        .get_entry()
        .iter()
        .filter(|e| e.get_space().get_index() == reg.get_index() && e.get_size() > 0)
        .filter(|e| thumb_r0.as_ref().is_none_or(|r0| {
            e.get_base() == r0.offset && e.get_size() == r0.size as int4
        }))
        .map(|e| (Address::new(std::rc::Rc::clone(e.get_space()), e.get_base()), e.get_size()))
        .collect();
    if outputs.is_empty() {
        return None;
    }
    let mut found: Option<(usize, int4)> = None;
    for (back, caller) in sites {
        let at = Address::new(std::rc::Rc::clone(code), back);
        let span = match caller {
            None => None,
            Some(caller) => {
                let owner = Address::new(std::rc::Rc::clone(code), caller);
                let name = arch.symboltab.find_function_across_scopes(&owner)
                    .map(|(id, _)| arch.symboltab.symbol(id).get_display_name());
                let Some(size) = raw_bound(caller, name.unwrap_or("")) else { continue };
                if size > 0 {
                    let Some(end) = caller.checked_add(size as u64) else { continue };
                    if back >= end {
                        continue;
                    }
                    Some((caller, end))
                } else {
                    None
                }
            }
        };
        let probe = match span {
            Some(span) => super::kuna_calleedeadarg::probe_entry_bounded(arch.translate(), &at, reg.get_index(), span),
            None => super::kuna_calleedeadarg::probe_entry(arch.translate(), &at, reg.get_index(), false),
        };
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
    if !raw.is_empty() && (width != *size || !complete_returns(data)) {
        return None;
    }
    let low = if addr.is_big_endian() { addr + i64::from(size - width) } else { addr.clone() };
    Some((low, width))
}

/// Have the next decompile of the function at `entry` force its return to
/// `storage`, as `kuna_voidret` does for a function whose callers read it.
pub fn force(arch: &mut Architecture, entry: &Address, storage: (Address, int4)) {
    if let Some(space) = entry.get_space() {
        let k = (space.get_index(), entry.get_offset());
        arch.kuna_voidret.forced.insert(k, storage);
        arch.kuna_voidret.unsettled.insert(k);
    }
}

/// Undo [`force`].
pub fn release(arch: &mut Architecture, entry: &Address) {
    if let Some(space) = entry.get_space() {
        let k = (space.get_index(), entry.get_offset());
        arch.kuna_voidret.forced.remove(&k);
        arch.kuna_voidret.unsettled.insert(k);
    }
}

/// Does the finished function return a value?
pub fn returns_value(data: &Funcdata) -> bool {
    data.get_func_proto()
        .get_output_type()
        .is_some_and(|t| t.get_metatype() != crate::dtype::type_metatype::TYPE_VOID)
}

/// Reject flow that was cut short or whose target remains unknown. Such a
/// path is not evidence that all the function's returns were analyzed.
fn complete_returns(data: &Funcdata) -> bool {
    use crate::op::pcodeop_flags;

    if data.has_bad_data()
        || data.has_unimplemented()
        || !data.kuna_rejected_flow_overrides().is_empty()
        || !data.kuna_rejected_callee_overrides().is_empty()
    {
        return false;
    }
    if data.call_specs().iter().any(|call| {
        call.is_jump_fallback()
            && call.get_entry_address().is_invalid()
            && data.obank().get(call.get_op()).is_some_and(|op| !op.is_dead())
    }) {
        return false;
    }
    !data.obank().iter_alive().any(|id| data.obank().get(id).is_some_and(|op| {
        op.get_halt_type()
            & (pcodeop_flags::badinstruction | pcodeop_flags::unimplemented | pcodeop_flags::missing) != 0
            || (op.code() == OpCode::CPUI_BRANCHIND && data.find_jump_table(id).is_none())
    }))
}

/// A recovered result must be present at every reachable ordinary return.
pub(crate) fn has_complete_value(data: &Funcdata) -> bool {
    let mut returns = data.obank().iter_code(OpCode::CPUI_RETURN).filter_map(|id| {
        data.obank().get(id).filter(|op| !op.is_dead() && op.get_halt_type() == 0)
    });
    returns.next().is_some_and(|op| op.num_input() == 2)
        && returns.all(|op| op.num_input() == 2)
        && complete_returns(data)
}

/// Select possible ordinary call results to type before the forced redo, not
/// proof that a result is returned. The redo must establish the actual SSA
/// value at each return, including any writes hidden by the first void run.
pub(crate) fn forwarded_callees(data: &Funcdata) -> Vec<&crate::fspec::FuncCallSpecs> {
    let mut entries = Vec::new();
    for ret in data.obank().iter_code(OpCode::CPUI_RETURN) {
        let Some(r) = data.obank().get(ret).filter(|r| !r.is_dead() && r.get_halt_type() == 0) else { continue };
        let Some(call) = super::kuna_passthrough::producing_call(data, ret) else { continue };
        if data.obank().get(call).is_none_or(|c| c.get_addr() == r.get_addr()) {
            continue;
        }
        let Some(i) = data.get_call_specs_index(call) else { continue };
        let fc = data.get_call_specs(i);
        if !fc.proto().is_output_locked() {
            entries.push(fc);
        }
    }
    entries
}
