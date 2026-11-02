//! (kuna) `gottailcall` — a computed jump whose one destination is a function
//! is a tail call when the call loses nothing the followed table would show.
//!
//! A position-independent tail call loads its target from a GOT slot and jumps
//! through it: `lw $t9,%call16(f)($gp); jr $t9` on MIPS, `jmp *f@GOTPCREL(%rip)`
//! in an x86-64 `-fno-plt` build. Jump-table recovery reads the slot and yields
//! a table with one entry, `f`. `tailcalljump on` makes that a tail call only
//! where the callee's prototype is stated or the destination lies outside a
//! declared extent; otherwise the flow follows the table, and the function
//! prints `switch(0x...) { case 0x...: <f's body> }`, or, when the slot names
//! an extern with no bytes, `switch(0x...) { case 0x...: halt_missing(); }`
//! with the call's arguments gone.
//!
//! This module adds two cases where the tail call loses nothing:
//!
//! * the destination has no bytes: following the table can only reach
//!   `halt_missing()`, and the call keeps the arguments the caller set;
//! * the destination is a function a callee-first run decompiled earlier and
//!   [`callable`] filed: it takes and returns nothing, or it stated its
//!   recovered prototype for its callers (`protoorder`), from which
//!   `passthrough` gives the call the registers the caller forwards and the
//!   caller the callee's return value.
//!
//! The reference walk behind decompile-all's callee-first plan does not see
//! these jumps (a MIPS `$gp` slot is not a constant it reads), so the callee is
//! often planned after its caller. [`record`] files such a caller beside the
//! callee it copied in, and [`due`] names it for a redo once that callee is
//! filed, so the redo takes the tail call with the callee's prototype in hand.
//!
//! Whether passthrough hands the call the callee's arguments and return also
//! depends on the caller (clang's `push %rax` alignment slot reads as a SysV
//! vector count, and passthrough then claims nothing in that function; a
//! second tail callee may state no return). So [`record`] checks each tail
//! call it took against the decompile it produced, and a call that lost the
//! callee's return value or a stated argument is refused for that caller,
//! which is decompiled again with the copied-in body.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uint4, uintb};
use kuna_num::opcodes::OpCode;

use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::infra::architecture::Architecture;
use crate::p0_knowledge::options::on_or_off;

/// `option gottailcall on|off`.
pub struct OptionGotTailCall;

impl OptionGotTailCall {
    /// The option name.
    pub const NAME: &'static str = "gottailcall";

    /// Resolve the flag and its confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Tail call through a one-destination jump turned {prop}")))
    }
}

type Key = (int4, uintb);

fn key(entry: &Address) -> Option<Key> {
    Some((entry.get_space()?.get_index(), entry.get_offset()))
}

/// What a call to a filed function rests on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Callee {
    /// Its protoorder statement supplies what it takes and returns.
    pub stated: bool,
    /// It returns a value.
    pub returns: bool,
}

/// What a callee-first run knows about the functions it decompiled.
#[derive(Debug, Default, Clone)]
pub struct Ledger {
    /// Set by the callee-first driver for the length of its run.
    pub recording: bool,
    /// Functions a tail call to loses nothing.
    pub callable: BTreeMap<Key, Callee>,
    /// Per callee, the callers that followed a one-destination table into it
    /// before it was filed.
    pub waiting: BTreeMap<Key, BTreeSet<Key>>,
    /// `(caller, callee)` tail calls [`callable`] allowed in a decompile
    /// [`record`] has not yet checked.
    pub taken: RefCell<BTreeSet<(Key, Key)>>,
    /// `(caller, callee)` tail calls that lost what the copied-in body shows.
    pub refused: BTreeSet<(Key, Key)>,
    /// The functions the run's plan decompiles.
    pub planned: BTreeSet<Key>,
    /// The planned functions it has decompiled so far.
    pub seen: BTreeSet<Key>,
}

/// Does a tail call from `data` to `dest` lose nothing, by what this run
/// recorded? A callee that returns a value hands it back only where every
/// return of the caller is a tail call: passthrough takes no tail call's
/// result beside a return of the caller's own.
pub fn callable(arch: &Architecture, data: &Funcdata, dest: &Address) -> bool {
    let ledger = &arch.kuna_gottailcall;
    if !ledger.recording {
        return false;
    }
    let (Some(k), Some(own)) = (key(dest), key(data.get_address())) else { return false };
    let ok = match ledger.callable.get(&k) {
        Some(c) => {
            !ledger.refused.contains(&(own, k))
                && (!c.stated || arch.kuna_protoorder_types.contains_key(&k))
                && !(c.returns && returns_on_its_own(data))
        }
        None => false,
    };
    if ok {
        ledger.taken.borrow_mut().insert((own, k));
    }
    ok
}

/// Has the flow so far reached a return that is no tail call: a RETURN with
/// no call at its instruction?
fn returns_on_its_own(data: &Funcdata) -> bool {
    let obank = data.obank();
    obank.iter_code(OpCode::CPUI_RETURN).any(|r| {
        let Some(o) = obank.get(r) else { return false };
        o.get_halt_type() == 0
            && !obank.iter_at(o.get_addr()).any(|(_, id)| {
                obank.get(id).is_some_and(|c| matches!(c.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND))
            })
    })
}

/// File what the function entered at `entry` lets a tail call to it say, and
/// each callee it followed a one-destination table into while that callee was
/// not yet filed.
///
/// Run after `protoorder` parked the function's statement. A declared
/// prototype is `tailcalljump`'s to read, and a variadic or model-less
/// recovery states nothing a call could carry.
pub fn record(arch: &mut Architecture, entry: &Address, data: &Funcdata) {
    if !arch.got_tail_call || !arch.kuna_gottailcall.recording {
        return;
    }
    let Some(own) = key(entry) else { return };
    let taken: Vec<Key> = {
        let mut t = arch.kuna_gottailcall.taken.borrow_mut();
        let mine: Vec<Key> = t.iter().filter(|(c, _)| *c == own).map(|(_, k)| *k).collect();
        t.retain(|(c, _)| *c != own);
        mine
    };
    let lost: Vec<Key> = taken.into_iter().filter(|k| loses(arch, data, *k)).collect();
    for k in &lost {
        arch.kuna_gottailcall.refused.insert((own, *k));
        arch.kuna_gottailcall.waiting.entry(*k).or_default().insert(own);
    }
    match callable_by_recovery(arch, entry, data).filter(|_| lost.is_empty()) {
        Some(stated) => arch.kuna_gottailcall.callable.insert(own, stated),
        None => arch.kuna_gottailcall.callable.remove(&own),
    };
    for i in 0..data.num_jump_tables() {
        let Some(dest) = crate::kuna_tailcalljump::kuna_sole_table_destination(data.get_jump_table(i))
        else {
            continue;
        };
        let Some(k) = key(&dest).filter(|k| *k != own) else { continue };
        if !arch.kuna_gottailcall.callable.contains_key(&k) {
            arch.kuna_gottailcall.waiting.entry(k).or_default().insert(own);
        }
    }
}

/// Did the tail call to `callee` lose what the copied-in body would show: the
/// callee returns a value and the caller came out returning nothing, or a call
/// to it passes fewer arguments than its statement names?
fn loses(arch: &Architecture, data: &Funcdata, callee: Key) -> bool {
    let Some(c) = arch.kuna_gottailcall.callable.get(&callee) else { return false };
    let own_void = data.get_func_proto().get_output_type().is_none_or(|t| t.get_metatype() == type_metatype::TYPE_VOID);
    if c.returns && own_void {
        return true;
    }
    let wanted = arch.kuna_protoorder_types.get(&callee).map_or(0, |s| s.inputs.len() as int4);
    (0..data.num_calls()).any(|i| {
        let fc = data.get_call_specs(i);
        key(fc.get_entry_address()) == Some(callee)
            && data.obank().get(fc.get_op()).is_some_and(|o| o.num_input() - 1 < wanted)
    })
}

/// `Some(stated)` when a tail call to this function loses nothing: it takes
/// and returns nothing (`false`), or its statement fixes what it takes and
/// what it returns (`true`).
fn callable_by_recovery(arch: &mut Architecture, entry: &Address, data: &Funcdata) -> Option<Callee> {
    if arch.symboltab.function_proto_pieces_across_scopes(entry).is_some() {
        return None;
    }
    let proto = data.get_func_proto();
    if !proto.has_store() || !proto.has_model() || proto.is_model_unknown() || proto.is_dotdotdot() {
        return None;
    }
    let returns = proto.get_output_type().is_some_and(|t| t.get_metatype() != type_metatype::TYPE_VOID);
    if proto.num_params() == 0 && !returns {
        return Some(Callee { stated: false, returns });
    }
    let stated = Rc::clone(arch.kuna_protoorder_types.get(&key(entry)?)?);
    if (returns && stated.output.is_none()) || !takes_every_input(arch, entry, &stated) {
        return None;
    }
    Some(Callee { stated: true, returns })
}

/// Does the callee's body take every parameter its statement names, as
/// passthrough asks before it forwards one: each is a register its entry
/// probe reads (any read for a list that is an arity claim, the low byte
/// otherwise) or hands on to a callee that takes it. A parameter passthrough
/// would not forward is one the copied-in body still shows.
fn takes_every_input(
    arch: &mut Architecture,
    entry: &Address,
    stated: &crate::p4_calls::kuna_protoorder::RecoveredTypes,
) -> bool {
    use crate::p4_calls::kuna_entryreads::overlaps;
    if stated.inputs.iter().any(|(a, _, _)| !crate::p4_calls::kuna_calleearitybody::is_register(a)) {
        return false;
    }
    if stated.inputs.is_empty() {
        return true;
    }
    let Some(reg_idx) = arch.manage().get_space_by_name("register").map(|s| s.get_index()) else { return false };
    let facts = crate::p4_calls::kuna_calleedeadarg::probe_cached(arch, entry, reg_idx).filter(|f| f.is_complete());
    stated.inputs.iter().all(|(a, s, _)| {
        overlaps(&stated.passes, a, *s)
            || match &facts {
                Some(f) if stated.arity_sound => f.proves_input(a, *s),
                Some(f) => f.reads_low_byte(a, *s, false, true),
                None => overlaps(&stated.reads, a, *s),
            }
    })
}

/// Drop the function's recovered-tail-call warnings that a new flow of it in
/// a callee-first run does not raise again (`pending`). A redo can refuse a
/// tail call the first decompile took, and the stale `// tail-call` would
/// print on the switch the redo follows.
pub(crate) fn clear_stale_warnings(arch: &mut Architecture, entry: &Address, pending: &[(uint4, Address, String)]) {
    if !arch.kuna_gottailcall.recording {
        return;
    }
    arch.commentdb.retain_comments(|c| {
        &c.func_addr != entry
            || c.tp != crate::architecture::comment_type::warning
            || !c.text.contains(": recovered tail call -> introduced call to ")
            || pending.iter().any(|(tp, ad, txt)| *tp == c.tp && *ad == c.addr && *txt == c.text)
    });
}

/// The callers to decompile again: each one that followed a table into a
/// callee that is now filed, once every callee it copied in has been
/// decompiled, and each one refused a tail call at once.
pub fn due(arch: &mut Architecture) -> BTreeSet<Key> {
    arch.kuna_gottailcall.take_due()
}

impl Ledger {
    /// [`due`] over the ledger, taking each caller it names off every waiting
    /// list. Waiting for the last planned callee redoes a caller that copied
    /// in many callees (a `switch` of PIC tail calls) once, not once per
    /// callee.
    pub fn take_due(&mut self) -> BTreeSet<Key> {
        if !self.recording || self.waiting.is_empty() {
            return BTreeSet::new();
        }
        let mut awaits: BTreeMap<Key, Vec<Key>> = BTreeMap::new();
        for (k, callers) in &self.waiting {
            for c in callers {
                awaits.entry(*c).or_default().push(*k);
            }
        }
        let filed = |k: &Key| self.callable.contains_key(k);
        let settled = |k: &Key| self.seen.contains(k) || !self.planned.contains(k);
        let out: BTreeSet<Key> = awaits
            .into_iter()
            .filter(|(c, ks)| {
                ks.iter().any(|k| filed(k) && self.refused.contains(&(*c, *k)))
                    || (ks.iter().any(filed) && ks.iter().all(settled))
            })
            .map(|(c, _)| c)
            .collect();
        for callers in self.waiting.values_mut() {
            callers.retain(|c| !out.contains(c));
        }
        self.waiting.retain(|_, callers| !callers.is_empty());
        out
    }
}

#[cfg(test)]
#[path = "kuna_gottailcall/tests.rs"]
mod tests;
