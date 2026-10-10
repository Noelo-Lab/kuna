//! (kuna) `calleereadarg` — keep a call argument the caller also tests when the
//! callee's own body reads it.
//!
//! # The symptom
//!
//! ```text
//!   int t9(int *p) { int x = *p + 2; if (x) return ext(x) + 1; return 5; }
//!
//!   mov  (%rdi),%eax
//!   lea  0x2(%rax),%edi        ; x, computed straight into the argument register
//!   test %edi,%edi             ; ... and tested there
//!   je   L
//!   call ext
//! ```
//!
//! kuna printed `return ext() + 1;`. gcc's `int fwd(int *p) { int x = *p; if (x
//! > 5) return x; return getk(x); }` loses `getk`'s argument the same way: the
//! value it loads into `edi` is compared, returned on one path and tail-called
//! on the other.
//!
//! # Why the argument is lost
//!
//! `FuncCallSpecs::checkInputTrialUse` (`fspec.cc:5592`) activates a register
//! trial only when `Funcdata::onlyOpUse` (`funcdata_varnode.cc:1851`) finds the
//! value reaching the CALL and nothing else. A value the caller also branches
//! on, stores, dereferences or returns is refused, and with nothing active
//! behind it the argument list ends before it. On the caller's side that is the
//! right call: relaxing it would invent an argument at every `test; jz; call`.
//!
//! # The evidence
//!
//! The callee's body settles it. The bounded entry decode
//! [`crate::p4_calls::kuna_calleedeadarg`] takes for every direct call, cached
//! per callee, shows whether some path reads the register's LOW byte before
//! writing it, for a value that can reach something
//! ([`CalleeEntryDead::reads_low_byte`]); whatever the caller left there is then
//! what the callee receives. That read leaves out a wide read past a low-byte
//! write (`mov 0x8(%rbx),%cl; lea -0x4(%rcx),%edx`, clang -Os), a register
//! compared with itself (`sbb %ecx,%ecx`), a register stored by an instruction
//! that stores several (gcc's `push {r0, r1, r4, lr}` reserving stack with dead
//! registers), on ARM and AArch64 a register the callee only stores (callers
//! there mark no variadic call, and gcc bounds a variadic callee's save to the
//! `va_arg` it can reach: `str x2, [sp, #24]`), and every read of a walk that met
//! an undecodable instruction (a body decoded in the wrong instruction set). A refused general-purpose
//! register trial whose value the caller wrote (`AncestorRealistic` accepted it)
//! qualifies, at a direct `CALL` to a known entry with an unlocked, non-variadic
//! prototype, unless one of these says the read is not an argument:
//!
//! * the callee reads the LAST general-purpose argument register: a variadic
//!   register-save prologue reads every one through the last, and an LLVM `-Oz`
//!   prologue that reserves stack by pushing dead registers takes them downward
//!   from the first saved one, so it reaches the last argument register first;
//! * the caller set the call up as variadic (SysV's `xor %eax,%eax`): gcc saves
//!   only the registers `va_arg` can reach, so gnulib `open_safer(char const *,
//!   int, ...)` reads `rdx` alone;
//! * the calling function is or may be variadic itself (it reads `al` in its
//!   entry block other than to store it, as `test %al,%al` does and clang's
//!   alignment `push %rax` does not, or its own entry reads its last argument
//!   register): an argument supplied there can be the `va_list` (`cliPrintf`
//!   handing `cliPrintfva` its saved `r1`-`r3`), and once the save area escapes
//!   through it the function's own recovery takes every saved register as a
//!   parameter.
//!
//! Floating-point registers are left alone: a callee that only moves its
//! `double` (`lua_pushnumber` stores `xmm0`) is recovered with an integer
//! parameter, and handing it the caller's `floor()` result reads that result as
//! an integer, which un-floats `floor` itself. The caller's own incoming
//! register is `passthrough`'s.
//!
//! # The mechanism
//!
//! The trial is not re-scored: `ParamListStandard::forceInactiveChain` reads a
//! run of inactive trials as the end of a list, and a trial turned active there
//! can unblock a spurious later one (on Cortex-M, the constant a caller loads
//! for `msr basepri` and the slot its own `push` wrote) and drag it in with
//! every hole before it. [`capture`] records, in `buildInputFromTrials`, each
//! general-purpose register trial with the value the CALL carried there, and
//! [`extend_pending`] runs at the end of `ActionActiveParam`, after the sibling
//! and body rules and before `passthrough`. A final list whose general-purpose
//! registers are a leading run of the model's gains the entries after it up to
//! the last one the callee reads, each as wide as the callee reads it. An entry
//! in between is the callee's parameter too, since it reads a later one, but its
//! value must be one the caller wrote, or the caller's own incoming register
//! where the callee reads it: clang drops an argument its callee never reads, so
//! `g3(1, 0, x)` leaves `esi` holding the caller's own second parameter, which
//! is a leftover. No more than two such entries may stand in a row, the chain
//! upstream's positional rule tolerates.

use std::collections::HashSet;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::type_class;
use crate::fspec::FuncCallSpecs;
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;
use crate::p4_calls::kuna_calleedeadarg::CalleeEntryDead;

/// (kuna) Keep a refused argument the callee's body reads:
/// `calleereadarg on|off`.
pub struct OptionCalleeReadArg;

impl OptionCalleeReadArg {
    /// The option name.
    pub const NAME: &'static str = "calleereadarg";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_read_arg`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Callee-read argument reinstatement turned {prop}")))
    }
}

/// How many entries in a row may stand between the list and the register the
/// callee reads (upstream `fillinMap`'s `maxchain`).
const MAX_HOLES: usize = 2;

/// One model register entry of a captured call, with the trial standing at it.
#[derive(Clone, Debug)]
pub struct Slot {
    /// The trial's index in prototype order.
    pub index: int4,
    /// The trial's storage.
    pub addr: Address,
    /// The trial's size in bytes.
    pub size: int4,
    /// How many low bytes of it to pass: as many as the callee reads, or the
    /// whole trial for a register it does not read.
    pub width: int4,
    /// The value the CALL carried in the trial's slot, when it may be passed:
    /// the caller wrote it, or it is the caller's own incoming register and
    /// the callee reads it. clang drops an argument its callee never reads
    /// (`g3(1, 0, x)` leaves `esi` alone), so the caller's own incoming
    /// register there is a leftover, not an argument.
    pub vn: Option<VarnodeId>,
    /// Does the callee read this register as an argument, for a value the
    /// caller's scoring refused?
    pub reads: bool,
}

/// A call whose callee reads a register the caller's scoring refused, held
/// until every other rule has settled the call's argument list.
pub struct PendingReadArg {
    op: OpId,
    /// The model's general-purpose register entries in order, with the trial
    /// standing at each (`None`: no trial, a hole nothing can fill).
    row: Vec<Option<Slot>>,
    /// Every trial's storage with its index in prototype order, which places
    /// the arguments the list already has.
    order: Vec<(Address, int4)>,
}

/// Is `addr` in the `register` space the callee-body summary answers for?
fn is_register(addr: &Address) -> bool {
    addr.get_space().map(|s| s.get_name() == "register").unwrap_or(false)
}

/// Does the callee summary `live` show `[addr, addr+size)` consumed as an
/// argument: its low byte read on entry, with `last`, the final register of its
/// storage class, left unread?
pub(crate) fn reads_as_argument(
    live: &CalleeEntryDead,
    addr: &Address,
    size: int4,
    last: Option<&(Address, int4)>,
    stores: bool,
) -> bool {
    let Some((last_addr, last_size)) = last else { return false };
    live.reads_low_byte(addr, size, stores) && !live.proves_read(last_addr, *last_size)
}

/// Record the calls this rule must leave alone, before heritage folds the
/// writes that identify them away: each call the function sets up as variadic
/// (SysV's `xor %eax,%eax` vector-register count), and every call of a function
/// that is variadic itself (`test %al,%al` in its entry block).
///
/// In a variadic function an argument this rule supplies can be the
/// `va_list` (`luaG_runerror` hands `&argp` to `luaO_pushvfstring`), and once
/// the register-save area escapes through it the function's own recovery takes
/// every saved register as a parameter, fourteen of them on x86-64.
/// Called at the end of `ActionFuncLink`, next to `passthrough`'s own record,
/// which that option skips in a function that is variadic itself.
pub fn record_variadic_setups(data: &mut Funcdata) {
    if !data.get_arch().callee_read_arg {
        return;
    }
    let variadic = crate::p4_calls::kuna_passthrough::tests_the_vararg_count(data);
    let ops: Vec<OpId> = (0..data.num_calls())
        .filter(|&i| variadic || crate::p4_calls::kuna_passthrough::set_up_as_variadic(data, i))
        .map(|i| data.get_call_specs(i).get_op())
        .collect();
    data.kuna_set_readarg_vararg_calls(ops);
}

/// Record the register trials of a call about to be finalized, when the callee
/// reads one the caller's scoring refused.
///
/// Called from `build_input_from_trials` while the trials and the CALL's
/// pre-rewrite inputs are both intact. `None` whenever the rule cannot apply:
/// option off, locked or variadic prototype, a call the caller set up as
/// variadic or made from a function that is or may be variadic, not a live
/// direct CALL to a known entry, no body summary, or no refused register the
/// callee reads.
pub fn capture(fc: &FuncCallSpecs, data: &Funcdata) -> Option<PendingReadArg> {
    if !data.get_arch().callee_read_arg
        || fc.is_input_locked()
        || fc.is_dotdotdot()
        || !fc.proto().has_model()
    {
        return None;
    }
    let op = fc.get_op();
    let o = data.obank().get(op)?;
    if o.is_dead() || o.code() != OpCode::CPUI_CALL || data.kuna_readarg_vararg_calls().contains(&op) {
        return None;
    }
    let entry = fc.get_entry_address();
    if entry.is_invalid() {
        return None;
    }
    let live = data.kuna_callee_entry_dead(entry)?;
    let regs: Vec<_> = fc
        .proto()
        .model()
        .input()
        .get_entry()
        .iter()
        .filter(|e| e.get_type() == type_class::TYPECLASS_GENERAL && e.get_space().get_name() == "register")
        .collect();
    let last = regs
        .last()
        .map(|e| (Address::new(std::rc::Rc::clone(e.get_space()), e.get_base()), e.get_size()));
    let own = data.kuna_callee_entry_dead(data.get_address());
    if let (Some(own), Some((a, s))) = (own, last.as_ref()) {
        if own.proves_read(a, *s) {
            return None;
        }
    }
    // A read that only stores the register is evidence only where callers mark
    // their variadic calls: the convention returns in a register that carries
    // no argument (x86-64 `al`). AArch64 gcc bounds a variadic callee's save to
    // the one `va_arg` it can reach (`str x2, [sp, #24]`), and nothing at the
    // call says the register is optional.
    let stores = fc.proto().model().output().get_entry().iter().any(|e| {
        e.get_type() == type_class::TYPECLASS_GENERAL
            && e.get_space().get_name() == "register"
            && !fc.proto().possible_input_param(
                &Address::new(std::rc::Rc::clone(e.get_space()), e.get_base()),
                e.get_size(),
            )
    });
    let active = fc.active_input();
    let mut any = false;
    let mut row: Vec<Option<Slot>> = Vec::new();
    for e in &regs {
        let found = (0..active.get_num_trials()).find(|&i| {
            let t = active.get_trial(i);
            is_register(t.get_address()) && e.justified_contain(t.get_address(), t.get_size()) >= 0
        });
        let Some(i) = found else {
            row.push(None);
            continue;
        };
        let t = active.get_trial(i);
        let (addr, size) = (t.get_address().clone(), t.get_size());
        let callee_reads = live.reads_low_byte(&addr, size, stores);
        let vn = if t.is_definitely_not_used() || t.is_unref() || t.get_slot() < 1 {
            None
        } else {
            o.get_in(t.get_slot()).filter(|v| {
                data.vbank().get(*v).is_some_and(|x| {
                    x.get_size() >= size && (x.is_written() || x.is_constant() || callee_reads)
                })
            })
        };
        let width = if callee_reads { live.live_input_width(&addr, size).unwrap_or(size) } else { size };
        let refused =
            !t.is_used() && !t.is_active() && (t.has_ancestor_realistic() || t.has_ancestor_solid());
        let reads = refused && vn.is_some() && reads_as_argument(live, &addr, size, last.as_ref(), stores);
        any |= reads;
        row.push(Some(Slot { index: i, addr, size, width, vn, reads }));
    }
    let order = (0..active.get_num_trials())
        .map(|i| (active.get_trial(i).get_address().clone(), i))
        .collect();
    any.then_some(PendingReadArg { op, row, order })
}

/// Which slots of the general-purpose row extend the list, given the trial indices
/// the call's final list already uses: the entries after a leading run of used
/// ones, up to the last the callee reads, through at most [`MAX_HOLES`] entries
/// in a row it does not.
pub(crate) fn extension<'a>(row: &'a [Option<Slot>], used: &HashSet<int4>) -> Vec<&'a Slot> {
    let is_used = |e: &Option<Slot>| e.as_ref().is_some_and(|s| used.contains(&s.index));
    let k = row.iter().take_while(|e| is_used(e)).count();
    if row[k..].iter().any(is_used) {
        return Vec::new();
    }
    let mut run: Vec<&Slot> = Vec::new();
    let mut take = 0;
    let mut holes = 0;
    for e in &row[k..] {
        let Some(s) = e else { break };
        if s.vn.is_none() {
            break;
        }
        if s.reads {
            holes = 0;
            run.push(s);
            take = run.len();
        } else {
            holes += 1;
            if holes > MAX_HOLES {
                break;
            }
            run.push(s);
        }
    }
    run.truncate(take);
    run
}

/// Extend each captured call's final argument list with the registers its
/// callee reads, once every other rule has settled it. Returns how many calls
/// gained arguments.
pub fn extend_pending(data: &mut Funcdata, pending: &[PendingReadArg]) -> int4 {
    pending.iter().filter(|p| extend_one(data, p)).count() as int4
}

/// [`extend_pending`] for one call.
fn extend_one(data: &mut Funcdata, p: &PendingReadArg) -> bool {
    let Some(idx) = data.get_call_specs_index(p.op) else { return false };
    let current = data.get_call_specs(idx).final_input_storage().to_vec();
    let inputs: Vec<VarnodeId> = match data.obank().get(p.op) {
        Some(o) if !o.is_dead() && o.code() == OpCode::CPUI_CALL => {
            (0..o.num_input()).filter_map(|i| o.get_in(i)).collect()
        }
        _ => return false,
    };
    if inputs.len() != current.len() + 1 {
        return false;
    }
    let mut have: Vec<(int4, VarnodeId, (Address, int4))> = Vec::new();
    for (k, (a, s)) in current.iter().enumerate() {
        let Some(&(_, index)) = p.order.iter().find(|(x, _)| x == a) else { return false };
        have.push((index, inputs[k + 1], (a.clone(), *s)));
    }
    if have.windows(2).any(|w| w[0].0 >= w[1].0) {
        return false;
    }
    let used: HashSet<int4> = have.iter().map(|h| h.0).collect();
    let added: Vec<Slot> = extension(&p.row, &used).into_iter().cloned().collect();
    if added.is_empty() {
        return false;
    }
    for s in &added {
        let Some(vn) = s.vn else { return false };
        let Some(vsize) = data.vbank().get(vn).map(|v| v.get_size()) else { return false };
        let arg = if vsize > s.width {
            match crate::p4_calls::kuna_passthrough::truncate_before(data, vn, s.width, p.op) {
                Some(v) => v,
                None => return false,
            }
        } else {
            vn
        };
        let addr = if s.addr.is_big_endian() { &s.addr + (s.size - s.width) as i64 } else { s.addr.clone() };
        have.push((s.index, arg, (addr, s.width)));
    }
    have.sort_by_key(|h| h.0);
    let mut newparam = vec![inputs[0]];
    newparam.extend(have.iter().map(|h| h.1));
    if data.op_set_all_input(p.op, &newparam).is_err() {
        return false;
    }
    data.get_call_specs_mut(idx).set_final_input_storage(have.into_iter().map(|h| h.2).collect());
    true
}

#[cfg(test)]
#[path = "kuna_calleereadarg/tests.rs"]
mod tests;
