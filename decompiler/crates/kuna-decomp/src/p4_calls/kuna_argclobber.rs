//! (kuna) `argclobber` — drop a trailing register argument that a previous
//! call's clobber put there.
//!
//! # The symptom
//!
//! coreutils `fmt` (-O2, stripped) calls one static function three times and
//! kuna renders three different arities, the longest of which reads a variable
//! nothing assigns:
//!
//! ```text
//!   unsigned long v9;  // rdx        <- never assigned
//!   unsigned long v11; // rdx        <- never assigned
//!   ...
//!     v10 = v9;
//!   ...
//!     v10 = v11;
//!   ...
//!     v12 = sub_3700(stdin,"-",v10); // three arguments
//!     v3  = sub_3700(stdin,v6);      // two
//!     v12 &= sub_3700(v5);           // one
//! ```
//!
//! DWARF says `fmt(FILE *f, char const *name)` — two parameters. The third
//! argument is `rdx`, and `rdx` at that call is a three-input MULTIEQUAL: on two
//! paths a previous `call` destroyed the register, on the third a dead `idiv`
//! remainder landed in it. The caller never placed a value there for this call.
//!
//! # Why it is admitted
//!
//! `FuncCallSpecs::checkInputTrialUse` (`fspec.cc:5592`) runs
//! `AncestorRealistic` over the register trial. `Heritage::guardCalls` plants an
//! INDIRECT *creation* for every killed-by-call register that is also a possible
//! output location of the callee's model — `rdx` is one, because x86-64 returns
//! a 16-byte value in `rax:rdx` — and `AncestorRealistic::enterNode`
//! (`funcdata_varnode.cc:2043`) answers `pop_success` for exactly that op,
//! recording `ParamTrial::setIndCreateFormed` on the way. The trial is realistic,
//! `onlyOpUse` says the value is used nowhere else, and `fillinMap` marks every
//! active trial used.
//!
//! Upstream reads that bit back on the RETURN side only:
//! `ParamListStandardOut::fillinMap` (`fspec.cc:1721`) scores a trial formed by a
//! remainder or an indirect creation no-use when its `ParamEntry` is not first in
//! its class. `ParamListStandard::fillinMap` — the *input* list — has no such
//! test, so the same evidence that disqualifies a return value is ignored for an
//! argument. Stock Ghidra emits the identical call, naming the phantom
//! `extraout_RDX`; this option is the other answer to the same evidence.
//!
//! # The rule
//!
//! At `buildInputFromTrials`, after the additive sibling rules have had their
//! turn, drop the LAST used argument when all of:
//!
//! * it is a **register** trial carrying `isIndCreateFormed`;
//! * its defining op is an indirect creation, or a MULTIEQUAL one of whose
//!   **immediate** inputs is one. A clobber that reaches the call only through a
//!   further join is left alone: tar's `str_days(pc, buffer, n)` is called with
//!   `n` live from a dominating block and a clobber merging in behind a second
//!   phi, and that argument is real;
//! * every one of those indirect creations is a creation of **this same
//!   register** ([`clobber_of_this_register_reaches`]);
//! * **every other input of that join is a division by-product**
//!   ([`is_division_byproduct`]) — the value an `idiv` leaves in the remainder
//!   register while the caller goes on to use only the quotient. Nothing else
//!   qualifies: a join input the caller wrote is an argument on the path that
//!   wrote it, and one clobber among the inputs does not change that;
//! * the callee's own **recovered prototype** is exactly the argument list this
//!   call is left with -- no parameter in those register bytes, and the
//!   parameters it does have account, storage for storage, for every argument
//!   that survives ([`callee_prototype_is_the_argument_list`]). This is the
//!   clause that carries the rule, and it is the reason the option can be a
//!   default at all;
//! * the callee's own body does not **read** those register bytes before writing
//!   them;
//! * it is **trailing** -- no used trial follows it -- so the argument list keeps
//!   its positional shape and no later argument moves;
//! * at least one used argument remains, since an empty list is
//!   [`kuna_calleearityfwd`](crate::p4_calls::kuna_calleearityfwd)'s failure
//!   shape, not this one's;
//! * no already-final call to the same callee entry in this function passed an
//!   argument in that storage. This is the subtractive twin of
//!   [`kuna_calleearity`](crate::p4_calls::kuna_calleearity)'s witness rule and
//!   it is what orders this pass against the additive family: a slot a sibling
//!   really used outranks the clobber evidence here.
//!
//! The drop is `markNoUse`, so every later member of the family reads it as
//! definitely-not-used and none of them puts the argument back
//! (`calleearitylive`'s `capture_partial_call` skips such a trial explicitly).
//!
//! # The three clauses that are not about the clobber's shape
//!
//! **Which register was clobbered.** On every ABI the first return register is
//! also killed-by-call and a possible output location, so until the output seam
//! resolves it a preceding call's RESULT is an indirect creation exactly like a
//! clobber is; when the caller moves that result into an argument register, copy
//! propagation puts the creation into the argument's join at its own address.
//! u-boot -O2 `sub_60827fa4` is that shape: `r0` holds
//! `ofnode_read_u32_default(dev->node,"bus-width",1)`, three `cmp`s test it, and
//! a pair of predicated `mov r3,r0` carry it into
//!
//! ```text
//!   printf("%s %s: Invalid \"bus-width\" value %u!\n",dev,name,width);
//! ```
//!
//! where it is the value `%u` prints. Requiring the creation to be at the
//! trial's own address is what declines it, and the statement is simple: a
//! register the caller wrote is a register the caller is passing, whatever the
//! value in it came from.
//!
//! **The other inputs of the join.** The same statement settles a join. One
//! clobber among a phi's inputs says nothing about the other paths, so every
//! other input has to be positively classified as a value nobody placed either,
//! and the division by-product is the only class that qualifies — upstream
//! scores a trial formed by a remainder exactly like one formed by an indirect
//! creation, on the return side (`ParamListStandardOut::fillinMap`,
//! `fspec.cc:1721`). Without this clause the rule deletes an argument the
//! caller demonstrably writes: a caller that forwards its own second parameter
//! (`mov %r12,%rdx`) on the non-clobber path loses both the argument and the
//! parameter, and `void caller(long,unsigned long)` becomes `void
//! caller(long)`. That shape is
//! `tests/stages/kuna-argclobber-forward.xml`.
//!
//! **The callee.** That still leaves a register the caller never wrote, and
//! nothing on the caller's side can say whether the callee wanted it. Only the
//! callee can, and the statement that settles it is the callee's own **recovered
//! prototype**: the parameter list kuna gets from decompiling that function,
//! which [`crate::p4_calls::kuna_protoorder`] parks for every callee it
//! decompiles before its callers. A prototype with a parameter overlapping the
//! register says the callee takes an argument there, and the drop is declined
//! whatever the caller's side looks like.
//!
//! Requiring that prototype to EXIST carries part of the clause, because
//! `protoorder` states nothing for a callee inside a recursive component
//! (`Decline::Scc` — "callees first" has no meaning in a cycle), one with no
//! recovered body such as a PLT import, one whose recovery produced no
//! parameters at all, and one whose prototype is already declared (that case
//! input-locks the call spec, which this rule declines at its first line). A
//! callee that was never decompiled in this run states nothing either, so a
//! single-function `kuna decompile`, a narrowed `decompile-all`, a `--jobs N`
//! run and `--option protoorder off` all leave every call site alone.
//!
//! It is NOT true that `protoorder` declines a variadic callee, and this rule
//! must not be read as resting on that. `protoorder`'s variadic guards —
//! `Decline::UnderRecovered` and `Decline::RegisterFileFull` — live in the
//! branch that parks a prototype in the symbol table (`lock`), and its default
//! `types` mode, the one this rule reads, returns before them;
//! `Decline::Variadic` itself is only ever set from a DECLARED `...`, so on a
//! stripped image it cannot fire at all. A stripped SysV variadic is STATED:
//! `long vlog(int,long,...)` built with `gcc -O2` and stripped states three
//! parameters, and `KUNA_PROTOORDER_TRACE=1` prints `state sub_11d0 params=3`.
//! What declines the drop there is the list itself — a register-save prologue
//! reads every argument register there is, so the recovered list CARRIES the
//! register the drop would take, and the first half of the clause refuses it.
//! Where a variadic recovers short instead, the callee-body veto below and the
//! surviving-argument accounting above are what answer for it.
//!
//! The callee's BODY is still read, as a veto only:
//! [`crate::p4_calls::kuna_calleedeadarg`] decodes each callee once per image for
//! the opposite claim, and a bounded entry walk that positively sees those
//! register bytes read before they are written declines the drop too
//! (`proves_input`). It cannot admit one.
//!
//! # What it cannot know
//!
//! The evidence is a recovery, not a fact. A callee whose own parameter list
//! kuna under-recovers — it misses a parameter the callee really reads — states
//! a prototype that admits the drop, and the argument goes. That is the residual
//! hole, and it is the same hole every callee-derived statement has. The
//! surviving-argument accounting bounds it: the under-recovery has to be exactly
//! one slot deep, at exactly the register the clobber wrote, with every other
//! argument still accounted for.
//!
//! What the clause removes is the class the bounded entry probe could not see at
//! all: a read past a jump table, a read beyond the probe's instruction budget, a
//! read inside an import, and a callee that really returns a 16-byte value in
//! `rax:rdx` and forwards the high half onward. Each of those is a callee whose
//! recovered prototype carries the parameter, so each is declined.
//! `docs/features/argclobber/` holds the five programs and the corpus
//! measurement that bounds the rule.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::fspec::FuncCallSpecs;
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;

/// Marshaling element `<argclobber>` (kuna 4000+ range).
pub const ELEM_ARGCLOBBER: ElementId = ElementId::new("argclobber", 4173);

/// `option argclobber on|off` (default on).
pub struct OptionArgClobber;

impl OptionArgClobber {
    /// The option name.
    pub const NAME: &'static str = "argclobber";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::arg_clobber`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Trailing clobber-argument drop turned {prop}")))
    }
}

/// Depth bound for [`is_division_byproduct`]'s walk.  The `fmt` witness needs
/// three hops (a mask, a zero extension, a `SUBPIECE` of the low half) and no
/// plausible width adjustment needs more.
const BYPRODUCT_WALK_DEPTH: u32 = 6;

/// Is `vn` the by-product half of a division — the value an `idiv` leaves in
/// the remainder register because the instruction writes it, not because
/// anybody wanted it there?
///
/// This is the only non-clobber shape a join input may have.  Upstream scores
/// it as exactly the same kind of evidence on the RETURN side:
/// `ParamListStandardOut::fillinMap` (`fspec.cc:1721`) marks a trial no-use
/// when `isRemFormed() || isIndCreateFormed()` and its `ParamEntry` is not
/// first in its class, and `AncestorRealistic` sets `RemFormed` for precisely
/// this chain (`funcdata_varnode.cc`, the `CPUI_SUBPIECE` arm).
///
/// The walk follows only the width adjustments a compiler puts between the
/// `INT_REM` and the register.  A join, a load, a call result or any other
/// arithmetic is a value the caller computed, and declines.
fn is_division_byproduct(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    if depth == 0 {
        return false;
    }
    let Some(vv) = data.vbank().get(vn) else { return false };
    let Some(def) = vv.get_def() else { return false };
    let Some(op) = data.obank().get(def) else { return false };
    match op.code() {
        OpCode::CPUI_INT_REM | OpCode::CPUI_INT_SREM => true,
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => op
            .get_in(0)
            .map(|i| is_division_byproduct(data, i, depth - 1))
            .unwrap_or(false),
        OpCode::CPUI_SUBPIECE => {
            let low = op
                .get_in(1)
                .and_then(|v| data.vbank().get(v))
                .map(|v| v.is_constant() && v.get_offset() == 0)
                .unwrap_or(false);
            low && op
                .get_in(0)
                .map(|i| is_division_byproduct(data, i, depth - 1))
                .unwrap_or(false)
        }
        OpCode::CPUI_INT_AND => {
            let masked = op
                .get_in(1)
                .and_then(|v| data.vbank().get(v))
                .map(|v| v.is_constant())
                .unwrap_or(false);
            masked
                && op
                    .get_in(0)
                    .map(|i| is_division_byproduct(data, i, depth - 1))
                    .unwrap_or(false)
        }
        _ => false,
    }
}

/// Is `vn` a value nothing on the caller's side put there for this call?
///
/// Three halves.
///
/// *Shape*: `vn` is defined by an indirect creation, or by a MULTIEQUAL, and at
/// least one of the join's **immediate** inputs is one.  A clobber that reaches
/// the call only through a further join is left alone: tar's
/// `str_days(pc, buffer, n)` is called with `n` live from a dominating block
/// and a clobber merging in behind a second phi, and that argument is real.
///
/// *Storage*: every indirect creation among those inputs is a creation of
/// `addr` itself — the register the argument is passed in.  This is the half
/// that separates a clobber from a **return value**.  On every ABI the first
/// return register is also killed-by-call and a possible output location, so
/// until the output seam resolves it a preceding call's result is an indirect
/// creation too; when the caller then moves it into an argument register, copy
/// propagation puts that creation straight into the argument's join, at its own
/// address rather than the argument's.  u-boot `sub_60827fa4` is that shape —
/// `r0` holds `ofnode_read_u32_default(node,"bus-width",1)`, three `cmp`s test
/// it, and a pair of predicated `mov r3,r0` carry it into `printf`'s fourth
/// slot, where it is the value `%u` prints.  A register the caller wrote is a
/// register the caller is passing, whatever the value in it came from.
///
/// *Every other input*: [`is_division_byproduct`].  A join input that is not a
/// clobber has to be positively classified as one the caller did not mean to
/// write either, and the division by-product is the only such class.  Without
/// this half one clobber input condemns the whole join, and a value the caller
/// demonstrably wrote on another path — a constant, its own parameter — is
/// deleted along with it.
fn clobber_of_this_register_reaches(data: &Funcdata, vn: VarnodeId, addr: &Address) -> bool {
    let is_creation_here = |v: VarnodeId| -> Option<bool> {
        let vv = data.vbank().get(v)?;
        let d = data.obank().get(vv.get_def()?)?;
        if !d.is_indirect_creation() {
            return Some(false);
        }
        Some(vv.get_addr() == addr)
    };
    let Some(def) = data.vbank().get(vn).and_then(|v| v.get_def()) else { return false };
    let Some(op) = data.obank().get(def) else { return false };
    if op.is_indirect_creation() {
        return is_creation_here(vn) == Some(true);
    }
    if op.code() != OpCode::CPUI_MULTIEQUAL {
        return false;
    }
    let n = op.num_input();
    let mut saw_clobber = false;
    for i in 0..n {
        let Some(iv) = data.obank().get(def).and_then(|o| o.get_in(i)) else { return false };
        let Some(vv) = data.vbank().get(iv) else { return false };
        let creation = vv
            .get_def()
            .and_then(|d| data.obank().get(d))
            .map(|d| d.is_indirect_creation())
            .unwrap_or(false);
        if !creation {
            if !is_division_byproduct(data, iv, BYPRODUCT_WALK_DEPTH) {
                return false;
            }
            continue;
        }
        if vv.get_addr() != addr {
            return false;
        }
        saw_clobber = true;
    }
    saw_clobber
}

/// Is the callee's own RECOVERED prototype exactly the argument list this call
/// is left with once `[addr, addr+size)` is dropped?
///
/// Two things are asked of it, and both have to hold.  The register bytes the
/// drop would remove must be free of any recovered parameter -- the callee does
/// not take an argument there.  And the parameters it DOES have must be, storage
/// for storage, the arguments that survive: every recovered parameter covered by
/// a surviving argument and every surviving argument covered by a recovered
/// parameter.  A recovery that is SHORT of what the call passes is not a
/// statement that the tail is unwanted, it is a statement that the recovery did
/// not reach the tail, and those two read identically from the caller's side.
///
/// That second half is what answers for a forwarding thunk.  `mov (%rdi),%rax;
/// jmp *%rax` never names the registers it passes through, so its own recovery
/// finds one parameter while its real callee consumes three; with only the
/// free-bytes test the drop is admitted and a forwarded argument is deleted
/// (`docs/features/argclobber/ce-forward-thunk.s`).  Requiring the recovered
/// list to ACCOUNT for every surviving argument declines it: one recovered
/// parameter cannot be the two arguments that would remain.
///
/// `false` is also the answer when `protoorder` parked nothing for this entry.
/// It states a prototype only for a function it decompiled before this caller,
/// and refuses for a callee inside a recursive component, one with no recovered
/// body such as a PLT import, one that recovered no parameters at all, and one
/// whose prototype is already declared.  "Nothing parked" and "cannot tell" are
/// the same answer here.
fn callee_prototype_is_the_argument_list(
    data: &Funcdata,
    entry: &Address,
    addr: &Address,
    size: int4,
    surviving: &[(Address, int4)],
) -> bool {
    let Some(stated) = data.kuna_protoorder_types(entry) else { return false };
    if size <= 0 {
        return false;
    }
    if stated.inputs.len() != surviving.len() {
        return false;
    }
    for (paddr, psize, _) in &stated.inputs {
        // A parameter whose storage cannot be compared is one this cannot rule
        // out, and an argument is not deleted on "cannot tell".
        if *psize <= 0 || paddr.get_space().is_none() {
            return false;
        }
        if storage_overlaps(paddr, *psize, addr, size) {
            return false;
        }
        if !surviving.iter().any(|(a, sz)| storage_overlaps(paddr, *psize, a, *sz)) {
            return false;
        }
    }
    for (a, sz) in surviving {
        if *sz <= 0 || a.get_space().is_none() {
            return false;
        }
        if !stated.inputs.iter().any(|(p, ps, _)| storage_overlaps(p, *ps, a, *sz)) {
            return false;
        }
    }
    true
}

/// Do two storages share a byte?  Different spaces never do.
fn storage_overlaps(a: &Address, asize: int4, b: &Address, bsize: int4) -> bool {
    let (Some(aspace), Some(bspace)) = (a.get_space(), b.get_space()) else {
        return false;
    };
    if aspace.get_index() != bspace.get_index() {
        return false;
    }
    let (alo, blo) = (a.get_offset(), b.get_offset());
    let (ahi, bhi) = (alo.wrapping_add(asize as u64), blo.wrapping_add(bsize as u64));
    alo < bhi && blo < ahi
}

/// Score the trailing argument of this call no-use when a previous call's
/// clobber is what put a value in its register.
///
/// Called from [`build_input_from_trials`](crate::funcdata_callsite::build_input_from_trials)
/// after `kuna_calleearity::unify_with_sibling_call`, so a slot the sibling rule
/// just promoted is visible here and declined. Inert with the option off.
pub fn drop_clobber_tail_arg(fc: &mut FuncCallSpecs, data: &mut Funcdata) {
    if !data.get_arch().arg_clobber {
        return;
    }
    // A locked or variadic prototype is not recovered from trials at all, and a
    // variadic call site legitimately differs in arity from its siblings.
    if fc.is_input_locked() || fc.is_dotdotdot() {
        return;
    }
    let op = fc.get_op();
    let live_call = data
        .obank()
        .get(op)
        .map(|o| !o.is_dead() && o.code() == OpCode::CPUI_CALL)
        .unwrap_or(false);
    if !live_call {
        return;
    }

    let num_trials = fc.active_input().get_num_trials();
    let mut last_used: Option<int4> = None;
    let mut used_count = 0;
    for i in 0..num_trials {
        if fc.active_input().get_trial(i).is_used() {
            used_count += 1;
            last_used = Some(i);
        }
    }
    let Some(idx) = last_used else { return };
    if used_count < 2 {
        return;
    }

    let (addr, slot, is_ind_create) = {
        let t = fc.active_input().get_trial(idx);
        (t.get_address().clone(), t.get_slot(), t.is_ind_create_formed())
    };
    if !is_ind_create {
        return;
    }
    let is_register = addr
        .get_space()
        .map(|s| s.get_type() != spacetype::IPTR_SPACEBASE)
        .unwrap_or(false);
    if !is_register {
        return;
    }
    // The argument Varnode has to be the one this call reads, so the clobber can
    // be looked at; a synthesized unreferenced trial has none.
    if slot < 0 {
        return;
    }
    let Some(vn) = data.obank().get(op).and_then(|o| o.get_in(slot)) else {
        return;
    };
    if !clobber_of_this_register_reaches(data, vn, &addr) {
        return;
    }
    // The callee's own RECOVERED prototype is the evidence, and it has to exist
    // and has to BE the list this call is left with: a callee that stated
    // nothing about itself, or stated less than the call still passes, cannot be
    // read as saying this register is free.
    let entry = fc.get_entry_address().clone();
    let trial_size = fc.active_input().get_trial(idx).get_size();
    let mut surviving: Vec<(Address, int4)> = Vec::new();
    for i in 0..num_trials {
        if i == idx {
            continue;
        }
        let t = fc.active_input().get_trial(i);
        if t.is_used() {
            surviving.push((t.get_address().clone(), t.get_size()));
        }
    }
    if !callee_prototype_is_the_argument_list(data, &entry, &addr, trial_size, &surviving) {
        return;
    }
    // The callee's body is read as a veto on top of that: a bounded entry walk
    // that positively sees these bytes READ before they are written is looking at
    // a parameter the recovery missed, however the value got into the register.
    if data
        .kuna_callee_entry_dead(&entry)
        .map(|d| d.proves_input(&addr, trial_size))
        .unwrap_or(false)
    {
        return;
    }
    // A sibling call to the same callee that really passed an argument here
    // outranks the clobber evidence.
    let witness = crate::p4_calls::kuna_calleearity::best_witness_for(&entry, op, data);
    if witness.iter().any(|(a, _)| *a == addr) {
        return;
    }
    fc.get_active_input().get_trial_mut(idx).mark_no_use();
}
