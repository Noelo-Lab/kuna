//! (kuna `calleevote`) A callee parameter takes the type every caller passes.
//!
//! `protoorder` carries what a callee's own recovery found out to its callers.
//! This is the other direction. A whole-binary run decompiles every caller of a
//! function, so after the first pass it knows the type each caller gave the
//! value it passes in every argument slot. Where the callee typed a parameter
//! only as `void *` or an integer, and every call to it passes the same
//! committed pointer -- a named record, a synthesized `struct_N *` the ledger
//! shares, a `char *` -- the callee is decompiled once more with that type as
//! one more vote for its input.
//!
//! "Every call" is a claim about the whole program, so it is checked against
//! the call graph rather than against what the decompiles happened to see: the
//! recorded call sites must be exactly the direct calls and tail jumps the
//! cross-reference walk finds, and a function whose address is taken in code
//! or stored in the image has callers nobody can list and states nothing. The
//! walk reads one instruction at a time, which sees every address taken in
//! code only on x86-64, so on any other architecture nothing is stated.
//!
//! The same closed caller set lets `structsynth` read a lone field: a pointer
//! parameter that the function reads at exactly one constant offset other than
//! zero is a record field, where a function reached through a pointer takes the
//! generic `void *` its contract declares (`fields`).

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::{int4, uintb};

use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::infra::architecture::Architecture;

/// What the complete caller set of a function may decide about it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CalleeVoteMode {
    /// Nothing flows from callers to a callee.
    #[default]
    Off,
    /// A parameter every caller passes the same committed pointer to takes it.
    Types,
    /// `types`, and a lone constant-offset field of a closed function's pointer
    /// parameter is a record field.
    Fields,
}

impl CalleeVoteMode {
    /// The option token.
    pub fn as_str(self) -> &'static str {
        match self {
            CalleeVoteMode::Off => "off",
            CalleeVoteMode::Types => "types",
            CalleeVoteMode::Fields => "fields",
        }
    }

    /// Does a whole-binary run record and decide anything?
    pub fn is_on(self) -> bool {
        self != CalleeVoteMode::Off
    }

    /// Does a closed function's lone field make a record?
    pub fn fields(self) -> bool {
        self == CalleeVoteMode::Fields
    }
}

/// (kuna) Parse `option calleevote off|types|fields`.
pub struct OptionCalleeVote;

impl OptionCalleeVote {
    /// The option name.
    pub const NAME: &'static str = "calleevote";

    /// Parse + validate the value.
    pub fn apply(&self, p1: &str) -> kuna_base::error::KunaResult<(CalleeVoteMode, String)> {
        let mode = match p1 {
            "off" => CalleeVoteMode::Off,
            "types" => CalleeVoteMode::Types,
            "fields" => CalleeVoteMode::Fields,
            other => {
                return Err(kuna_base::error::KunaError::parse(format!(
                    "Unknown calleevote value: {other} (expected off|types|fields)"
                )))
            }
        };
        Ok((mode, format!("Caller-to-callee parameter typing set to {}", mode.as_str())))
    }
}

/// A value in one storage location with the type its function gave it.
#[derive(Debug, Clone)]
pub struct Typed {
    pub addr: Address,
    pub size: int4,
    pub ct: Rc<Datatype>,
    /// A pointer to a pointer that is the address of one of the caller's own
    /// frame objects (`&v4`), or, for a statement, passed so by some caller.
    pub frame: bool,
}

/// One direct call: the caller, the call instruction, and what each argument
/// slot carried (`None` where the storage or the type could not be read).
#[derive(Debug, Clone)]
pub struct CallSite {
    pub caller: uintb,
    pub at: uintb,
    pub args: Vec<Option<Typed>>,
}

/// The types every caller agreed on, for the callee's own inputs.
#[derive(Debug)]
pub struct CallerTypes {
    pub inputs: Vec<Typed>,
}

impl CallerTypes {
    /// The statement for an input in exactly this storage.
    pub fn at(&self, addr: &Address, size: int4) -> Option<&Typed> {
        self.inputs.iter().find(|t| t.size == size && &t.addr == addr)
    }
}

/// The run's record: what each function's own recovery gave its parameters,
/// what every call passed, and what was decided.
#[derive(Debug, Default)]
pub struct Ledger {
    /// The whole-binary driver is recording the decompiles it runs.
    pub recording: bool,
    /// Callee key -> the direct calls recorded to it.
    pub sites: HashMap<(int4, uintb), Vec<CallSite>>,
    /// Function key -> the parameters its first decompile recovered; absent
    /// for a declared, variadic or failed function, which is never voted on.
    pub own: HashMap<(int4, uintb), Vec<Typed>>,
    /// Function key -> the types its callers stated.
    pub stated: HashMap<(int4, uintb), Rc<CallerTypes>>,
    /// Functions whose callers are all known direct calls.
    pub closed: HashSet<(int4, uintb)>,
}

impl Ledger {
    /// Drop what the function at `key` recorded: its parameters, the calls it
    /// makes and what its callers stated about it. Its callees then have fewer
    /// recorded calls than the graph lists and state nothing, which is the
    /// answer for a body the driver discarded, and a later decompile of it
    /// takes no vote.
    pub fn forget(&mut self, key: (int4, uintb)) {
        self.own.remove(&key);
        self.stated.remove(&key);
        for sites in self.sites.values_mut() {
            sites.retain(|s| s.caller != key.1);
        }
    }
}

impl Ledger {
    /// [`forget_statements_naming`] over the ledger.
    pub fn forget_naming(&mut self, names: &[String]) {
        if names.is_empty() {
            return;
        }
        self.stated
            .retain(|_, stated| !stated.inputs.iter().any(|t| crate::kuna_protoorder::names_type(&t.ct, names)));
    }
}

/// Is `ct` a pointer to a pointer?
fn points_at_a_pointer(ct: &Datatype) -> bool {
    ct.get_ptr_to().is_some_and(|p| p.get_metatype() == type_metatype::TYPE_PTR)
}

fn key_of(a: &Address) -> Option<(int4, uintb)> {
    Some((a.get_space()?.get_index(), a.get_offset()))
}

/// Is `ct` a pointer a caller commits to: a named record or union, a
/// character, or a pointer to a character pointer?
pub fn committed(ct: &Datatype) -> bool {
    if ct.get_metatype() != type_metatype::TYPE_PTR {
        return false;
    }
    let Some(pt) = ct.get_ptr_to() else { return false };
    match pt.get_metatype() {
        type_metatype::TYPE_STRUCT | type_metatype::TYPE_UNION => {
            !pt.get_name().is_empty() && !pt.is_incomplete()
        }
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT => pt.get_size() == 1 && pt.is_char_print(),
        type_metatype::TYPE_PTR => pt.get_ptr_to().is_some_and(|c| c.get_size() == 1 && c.is_char_print()),
        _ => false,
    }
}

/// Does `ct` say no more about a pointer-width value than that it is one: a
/// pointer to nothing (`void *`, `undefined1 *`), a plain integer of that
/// width, or a record synthesized from one field read, which the callers'
/// record replaces?
pub fn uncommitted(ct: &Datatype, ptr_size: int4) -> bool {
    match ct.get_metatype() {
        type_metatype::TYPE_PTR => {
            ct.get_ptr_to().is_some_and(|p| {
                matches!(p.get_metatype(), type_metatype::TYPE_VOID | type_metatype::TYPE_UNKNOWN)
            }) || crate::kuna_structsynth::points_at_lone_record(ct)
        }
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => {
            ct.get_size() == ptr_size
        }
        _ => false,
    }
}

/// The same type: one factory entry, or the same name and shape all the way
/// down its pointer chain. A layout comparison alone would equate two records
/// that merely have the same size.
fn same_type(a: &Datatype, b: &Datatype) -> bool {
    if std::ptr::eq(a, b) {
        return true;
    }
    match (a.get_ptr_to(), b.get_ptr_to()) {
        (Some(pa), Some(pb)) => a.get_size() == b.get_size() && same_type(&pa, &pb),
        (None, None) => a.get_name() == b.get_name() && a.compare(b, 10).is_ok_and(|c| c == 0),
        _ => false,
    }
}

/// Is `ct`, the type the function input `vn` carries, the one its callers
/// stated for it?
pub fn took_stated_type(data: &Funcdata, vn: VarnodeId, ct: &Datatype) -> bool {
    let Some(stated) = data.kuna_calleevote_inputs() else { return false };
    let Some(v) = data.vbank().get(vn) else { return false };
    v.is_input() && stated.at(v.get_addr(), v.get_size()).is_some_and(|t| same_type(&t.ct, ct))
}

/// Record a finished decompile: every direct call it makes, replacing what an
/// earlier decompile of the same function recorded, and, the first time, the
/// function's own recovered parameters. A redo's parameters carry the votes it
/// took, so the first decompile's stay what every round decides against: each
/// statement then covers every parameter the callers agree on, not only the
/// ones the last redo left untyped.
pub fn record(arch: &mut Architecture, entry: &Address, fd: &mut Funcdata) {
    let Some(me) = key_of(entry) else { return };
    let own = if arch.kuna_calleevote.own.contains_key(&me) { None } else { own_params(arch, entry, fd) };
    let mut calls: Vec<(Address, CallSite)> = Vec::new();
    for i in 0..fd.num_calls() {
        let (callee, op, storage) = {
            let fc = fd.get_call_specs(i);
            (fc.get_entry_address().clone(), fc.get_op(), fc.final_input_storage().to_vec())
        };
        if callee.is_invalid() {
            continue;
        }
        let Some(o) = fd.obank().get(op) else { continue };
        if o.code() != OpCode::CPUI_CALL {
            continue;
        }
        let at = o.get_addr().get_offset();
        let ins: Vec<Option<VarnodeId>> = (1..o.num_input()).map(|s| o.get_in(s)).collect();
        let mut args = Vec::with_capacity(ins.len());
        for (s, vn) in ins.into_iter().enumerate() {
            let (Some(vn), Some((addr, size))) = (vn, storage.get(s).cloned()) else {
                args.push(None);
                continue;
            };
            let ct = fd.high_get_type(vn).or_else(|| fd.vbank().get(vn).map(|v| Rc::clone(v.get_type())));
            args.push(ct.map(|ct| {
                let frame = points_at_a_pointer(&ct) && crate::kuna_protoorder::addresses_a_frame_object(fd, vn);
                Typed { addr, size, ct, frame }
            }));
        }
        calls.push((callee, CallSite { caller: me.1, at, args }));
    }
    let ledger = &mut arch.kuna_calleevote;
    for sites in ledger.sites.values_mut() {
        sites.retain(|s| s.caller != me.1);
    }
    for (callee, site) in calls {
        if let Some(k) = key_of(&callee) {
            ledger.sites.entry(k).or_default().push(site);
        }
    }
    if let Some(own) = own {
        ledger.own.insert(me, own);
    }
}

/// The function's own recovered parameters, or `None` when nothing may be
/// stated about them: a declared prototype outranks every caller, a variadic's
/// slots are not its parameters, and a hidden or `this` parameter moves the
/// slot numbering.
fn own_params(arch: &Architecture, entry: &Address, fd: &Funcdata) -> Option<Vec<Typed>> {
    if arch.symboltab.function_proto_pieces_across_scopes(entry).is_some() {
        return None;
    }
    let proto = fd.get_func_proto();
    if !proto.has_store() || proto.is_dotdotdot() || proto.is_input_locked() {
        return None;
    }
    let mut out = Vec::new();
    for i in 0..proto.num_params() {
        let p = proto.get_param(i)?;
        if p.is_hidden_return() || p.is_indirect_storage() || p.is_this_pointer() || p.is_type_locked() {
            return None;
        }
        let addr = p.get_address();
        if addr.is_invalid() || p.get_size() <= 0 {
            return None;
        }
        out.push(Typed { addr, size: p.get_size(), ct: Rc::clone(p.get_type()?), frame: false });
    }
    Some(out)
}

/// Decide what the callers of every recorded function state about it.
///
/// `expected` answers, for a function, the addresses of every direct call and
/// tail jump to it from another function, or `None` when its address escapes.
/// A function whose recorded calls are not exactly those states nothing, and
/// neither does a parameter some caller passes anything but the one committed
/// pointer the others pass. A first statement that only repeats the types the
/// function already has (a one-field record its callers pass back to it) is
/// not made, since decompiling it again would change nothing. Returns the
/// functions whose statement is new or changed.
pub fn decide(
    arch: &mut Architecture,
    expected: &dyn Fn((int4, uintb)) -> Option<Vec<uintb>>,
) -> Vec<(int4, uintb)> {
    let ptr_size = arch.types().get_size_of_pointer();
    let decided = decide_ledger(&arch.kuna_calleevote, ptr_size, expected);
    let changed = decided.iter().map(|(k, _)| *k).collect();
    for (key, stated) in decided {
        arch.kuna_calleevote.stated.insert(key, Rc::new(stated));
    }
    changed
}

/// [`decide`] over a ledger: the statements that are new or changed.
pub fn decide_ledger(
    ledger: &Ledger,
    ptr_size: int4,
    expected: &dyn Fn((int4, uintb)) -> Option<Vec<uintb>>,
) -> Vec<((int4, uintb), CallerTypes)> {
    let mut decided: Vec<((int4, uintb), CallerTypes)> = Vec::new();
    let mut keys: Vec<&(int4, uintb)> = ledger.own.keys().collect();
    keys.sort_unstable();
    for key in keys {
        let own = &ledger.own[key];
        if !own.iter().any(|p| uncommitted(&p.ct, ptr_size)) {
            continue;
        }
        let trace = trace();
        let Some(mut want) = expected(*key) else {
            if trace { eprintln!("[calleevote] 0x{:x} escapes", key.1); }
            continue;
        };
        let Some(sites) = ledger.sites.get(key) else {
            if trace { eprintln!("[calleevote] 0x{:x} no recorded calls (want {})", key.1, want.len()); }
            continue;
        };
        let sites: Vec<&CallSite> = sites.iter().filter(|s| s.caller != key.1).collect();
        let mut got: Vec<uintb> = sites.iter().map(|s| s.at).collect();
        want.sort_unstable();
        want.dedup();
        got.sort_unstable();
        if got.is_empty() || got != want {
            if trace { eprintln!("[calleevote] 0x{:x} calls differ: recorded {:x?} graph {:x?}", key.1, got, want); }
            continue;
        }
        let mut inputs: Vec<Typed> = Vec::new();
        for (j, p) in own.iter().enumerate() {
            if !uncommitted(&p.ct, ptr_size) {
                continue;
            }
            let mut agreed: Option<Rc<Datatype>> = None;
            let mut frame = false;
            let all = sites.iter().all(|s| {
                let Some(Some(a)) = s.args.get(j) else { return false };
                if a.addr != p.addr || a.size != p.size || !committed(&a.ct) {
                    return false;
                }
                frame |= a.frame;
                match &agreed {
                    None => {
                        agreed = Some(Rc::clone(&a.ct));
                        true
                    }
                    Some(t) => same_type(t, &a.ct),
                }
            });
            if trace {
                let seen: Vec<String> = sites.iter().map(|s| match s.args.get(j) {
                    Some(Some(a)) => format!("{}@{:x}:{}", spell(&a.ct), a.addr.get_offset(), a.size),
                    Some(None) => "unread".into(),
                    None => "absent".into(),
                }).collect();
                eprintln!("[calleevote] 0x{:x} param {j} ({}@{:x}:{}) callers {:?} -> {}", key.1, spell(&p.ct), p.addr.get_offset(), p.size, seen, all && agreed.is_some());
            }
            if let (true, Some(ct)) = (all, agreed) {
                inputs.push(Typed { addr: p.addr.clone(), size: p.size, ct, frame });
            }
        }
        if inputs.is_empty() {
            continue;
        }
        let noop = inputs.iter().all(|t| {
            own.iter().any(|p| p.addr == t.addr && p.size == t.size && same_type(&p.ct, &t.ct))
        });
        if noop && !ledger.stated.contains_key(key) {
            continue;
        }
        let unchanged = ledger.stated.get(key).is_some_and(|old| {
            old.inputs.len() == inputs.len()
                && old.inputs.iter().zip(&inputs).all(|(a, b)| {
                    a.addr == b.addr && a.frame == b.frame && same_type(&a.ct, &b.ct)
                })
        });
        if !unchanged {
            decided.push((*key, CallerTypes { inputs }));
        }
    }
    decided
}

/// Forget every statement that names one of `names`, through any depth of
/// pointer: the `structsynth` convergence sweep's superseded structures. The
/// sweep decompiles a function naming one again so that it takes the
/// survivor, and a statement made before the survivor existed would give it
/// the superseded one back.
pub fn forget_statements_naming(arch: &mut Architecture, names: &[String]) {
    arch.kuna_calleevote.forget_naming(names);
}

/// A type's name for the trace, pointers spelled through.
fn spell(ct: &Datatype) -> String {
    match ct.get_ptr_to() {
        Some(p) => format!("{} *", spell(&p)),
        None => ct.get_name().to_string(),
    }
}

/// Is the decision trace on (`KUNA_CALLEEVOTE_TRACE=1`)?
pub fn trace() -> bool {
    std::env::var("KUNA_CALLEEVOTE_TRACE").is_ok_and(|v| v != "0" && !v.is_empty())
}

/// Copy onto `data` what its callers stated and whether its callers are all
/// known, before the pipeline runs. It applies after recording stops too: the
/// batch's convergence sweep decompiles a function again and must not drop
/// the vote its body was printed with.
pub fn seed(arch: &Architecture, data: &mut Funcdata) {
    if !arch.calleevote.is_on() {
        return;
    }
    let Some(key) = key_of(data.get_address()) else { return };
    let ledger = &arch.kuna_calleevote;
    if let Some(stated) = ledger.stated.get(&key) {
        data.kuna_set_calleevote_inputs(Some(Rc::clone(stated)));
    }
    if arch.calleevote.fields() && ledger.closed.contains(&key) {
        data.kuna_set_calleevote_closed(true);
    }
}

/// The type every caller passes for the function input `vn`, offered to
/// `Varnode::getLocalType`'s fold in place of `ct`, or `None`.
///
/// Only where the fold so far says no more than "a pointer-width value" and the
/// callee's own uses of the value do not contradict the pointer: the same
/// refusals `protoorder` applies to a callee's vote at a call site, a loaded or
/// stored member that is not one of the record's own included. A pointer to a
/// pointer some caller passes as the address of its own frame object types
/// that one object, so a callee that reaches past it (`cfg_free(&cfg)` reading
/// the members after a `char *` first one) is not taking the object's type.
pub fn input_vote(data: &Funcdata, vn: VarnodeId, ct: &Rc<Datatype>) -> Option<Rc<Datatype>> {
    let stated = data.kuna_calleevote_inputs()?;
    let v = data.vbank().get(vn)?;
    if !v.is_input() || v.is_type_lock() {
        return None;
    }
    let want = stated.at(v.get_addr(), v.get_size())?;
    let ptr_size = data.get_arch().types().map(|t| t.get_size_of_pointer()).unwrap_or(8);
    if !uncommitted(ct, ptr_size) || crate::kuna_protoorder::input_refuses(data, vn, &want.ct) {
        return None;
    }
    if want.frame && crate::kuna_protoorder::reaches_past_the_pointee(data, vn, &want.ct) {
        return None;
    }
    Some(Rc::clone(&want.ct))
}

#[cfg(test)]
mod tests;
