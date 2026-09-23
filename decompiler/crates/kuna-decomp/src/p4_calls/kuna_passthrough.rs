//! (kuna) `passthrough` — a register a function forwards untouched to a callee
//! that reads it is a parameter of that function too.
//!
//! # The symptom
//!
//! gzip -O2 `gzip_base_name` is `endbr64; jmp gzip_last_component` — it hands
//! its own `rdi` to the callee and returns what the callee returns. kuna recovers
//! the callee as `char *sub_dfd0(char *a0)` in the same run, yet prints
//!
//! ```text
//!   void sub_d290(void) { sub_dfd0(); } // tail-call
//! ```
//!
//! while DWARF says `char *gzip_base_name(char const *fname)`. coreutils df
//! `dir_name` (`call mdir_name; test rax,rax; ...`) and tar's `*_diag` helpers
//! are the same shape without the tail call.
//!
//! # Why the argument is lost
//!
//! A call site's argument trials come from `Heritage::guardCalls`
//! (`heritage.cc:1444`), which registers one for each HERITAGED storage range
//! the callee's model could take a parameter in. A range is heritaged only when
//! some op of the function reads or writes it, and a register the function only
//! forwards is named by no op at all: the call gets no trial for it, so no
//! argument, so the function's own input never has a reader and it gets no
//! parameter either. Where the register IS heritaged (the function also tests
//! it), `AncestorRealistic::execute` (`funcdata_varnode.cc:2159`) refuses the
//! trial anyway, because upstream expects "active movement into the parameter"
//! and an input Varnode sitting at the call slot is the absence of one.
//!
//! # The evidence
//!
//! Only the callee can say whether it wanted the register, and the statement
//! this pass reads is the one `argclobber` reads in the other direction: the
//! callee's own RECOVERED prototype, which [`crate::p4_calls::kuna_protoorder`]
//! states for every callee `decompile-all` decompiled before its callers. For a
//! register `R` at a direct, unlocked, non-variadic call ([`stated_width`]):
//!
//! * the stated prototype has a parameter starting at `R`, and the list is one
//!   `protoorder`'s locking mode would accept as an arity claim
//!   ([`crate::p4_calls::kuna_protoorder::RecoveredTypes::arity_sound`]): all in
//!   registers, the callee's body does not read the register the next parameter
//!   would arrive in (a variadic's register-save prologue does), the list does not
//!   end on the last argument register, and it sits where the convention puts
//!   those types;
//! * the callee's own body READS `R` before writing it on some path, for a value
//!   that reaches something ([`crate::p4_calls::kuna_calleedeadarg`]'s
//!   `proves_input`), so the parameter is not a recovery artifact of the callee;
//! * that read is for something other than a variadic tail
//!   ([`crate::p4_calls::kuna_varargtail`]): a callee whose parameter only
//!   reaches an argument slot the ABI lets a caller leave unset says nothing
//!   about what its callers put there -- openssh `xcalloc`'s third parameter is
//!   gcc's `push %rdx` alignment filler at a variadic call, gnulib `open_safer`'s
//!   is `open`'s `mode`;
//! * every EARLIER parameter the callee states is either forwarded here too or
//!   is one this function's own entry does not write before reading
//!   ([`no_hole_before`]): a claim the positional list can only reach across a
//!   register this function loads itself materializes that register as a
//!   parameter that is nothing -- the `overrec(unsigned long *a0,unsigned long
//!   a1,long a2)` of the `protoorder` fixture, whose `a1` occurs in no
//!   statement and whose one call site passes one argument;
//! * the caller did not set the call up as variadic (`set_up_as_variadic`:
//!   `xor %eax,%eax` before a SysV call), and is not variadic itself
//!   (`reads_the_vararg_count`: `test %al,%al` in its entry block);
//! * the value at the call is the function's own incoming `R` — the SSA input
//!   Varnode itself, never a value any op of the function wrote.
//!
//! The argument is as wide as the callee's body reads it, so a parameter the
//! callee recovered as `rdi` but reads as `edi` becomes the caller's `int`.
//!
//! Nothing is added for a callee that stated nothing: a single-function
//! `kuna decompile`, a narrowed or sharded `decompile-all`, an import,
//! `--option protoorder off`, and under `--option protoorder types` a callee
//! inside a recursive component, all leave every call as it was. Under the
//! default `cycles` a recursive callee states its list like any other, and what
//! it stated is read here on the same terms as any other callee's.
//!
//! # The mechanism
//!
//! The pass only ever adds to a list every other rule has settled. The scoring
//! of a heritaged register is upstream's, and a call left empty by it is often
//! finished by a later rule (`calleearityfwd` fills tar `sysinttostr`'s first
//! tail call from its sibling), which an argument supplied while scoring would
//! turn off.
//!
//! [`claim_untouched_registers`] runs at the end of `ActionFuncLink`, before the
//! first heritage: for a register no op of the function touches, it registers
//! the trial at each call no other call can precede ([`first_calls`]) whose
//! callee states it, and gives the CALL a read of it, exactly as the
//! locked-prototype branch of `funcLinkInput` does. Heritage links that read to
//! the function's input. The claim is recorded on the `Funcdata`; heritage
//! registers the now visible range at every other call as usual, and only the
//! function's own RETURN is kept off it ([`suppresses_return_trial`]).
//!
//! A trial on a claimed range, at any call, is never scored ([`claimed_range`]):
//! it is inactive, where the option-off run's hole-filling trial for the slot
//! starts. Left to `AncestorRealistic`, a register that reaches a call through
//! an earlier call is "killed by call", definitely not used, and `forceNoUse`
//! drops every argument after it (`noop(); twoarg(p,3)` at gcc -O2 lost the `3`).
//!
//! [`capture`] runs first in `buildInputFromTrials`: it keeps the unused trials
//! that stand on the function's own input and satisfy the evidence, and retires
//! the unused ones on claimed ranges so the sibling and body rescues see what
//! they see with the option off. [`extend_pending`] runs last in `ActionActiveParam`: a call's
//! final list, when it is a leading run of the stated one, gains the stated
//! registers that follow it, up to the first the function does not forward.
//! `ActionInputPrototype` then makes the function's input a parameter because
//! the call now reads it.
//!
//! # The tail call's result
//!
//! When every live RETURN is reached from a direct call with nothing in between,
//! every such callee states a non-`void` return in the same register, and the
//! function never touches that register (`stated_tail_return`), each RETURN gets
//! a read of it and the function's return trial (`claim_tail_return`).
//! Upstream's `ancestorOpUse` refuses an INDIRECT creation at a RETURN, so
//! [`returns_tail_result`] accepts the one planted at a claimed call, whose output
//! then takes the callee's recovered return type ([`tail_return_type`]).

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::fspec::FuncCallSpecs;
use crate::funcdata::Funcdata;
use crate::p0_knowledge::options::on_or_off;

/// Marshaling element `<passthrough>` (kuna 4000+ range).
pub const ELEM_PASSTHROUGH: ElementId = ElementId::new("passthrough", 4177);

/// `option passthrough on|off`.
pub struct OptionPassThrough;

impl OptionPassThrough {
    /// The option name.
    pub const NAME: &'static str = "passthrough";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::pass_through`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Forwarded-register parameter recovery turned {prop}")))
    }
}

/// A register range this pass made visible to heritage, and the sites whose
/// trial for it the pass owns.
#[derive(Clone, Debug)]
pub struct PassThroughClaim {
    /// The register storage.
    pub addr: Address,
    /// Its size in bytes.
    pub size: int4,
    /// The CALL ops that own an ARGUMENT trial for it.
    pub arg_owners: Vec<OpId>,
    /// The CALL ops that own a RETURN-VALUE trial for it: the tail calls whose
    /// result every RETURN of the function hands back.
    pub ret_owners: Vec<OpId>,
}

/// How many bytes of `[addr, addr+size)` the callee of `fc` takes as a
/// parameter, by its own recovered prototype and its own body.
///
/// The stated prototype must have a parameter starting at `addr`, and the
/// callee's body must read it; the answer is the narrowest of the range, that
/// parameter and what the body consumes, so a register the callee reads as
/// `edi` is passed as the `int` it is. `None` for a locked or variadic call spec
/// (its argument list is not recovered from trials), an indirect call, a call
/// the caller set up as variadic, a callee `protoorder` stated nothing for, a
/// stated list that is not an arity claim, a register the callee's body is not
/// seen reading, a register the calling function's own prototype could not
/// take as a parameter, and any call of a variadic function.
pub fn stated_width(data: &Funcdata, fc: &FuncCallSpecs, addr: &Address, size: int4) -> Option<int4> {
    if !data.get_arch().pass_through || size <= 0 || data.kuna_passthrough_variadic() {
        return None;
    }
    if fc.is_input_locked() || fc.is_dotdotdot() {
        return None;
    }
    let op = fc.get_op();
    let is_call = data.obank().get(op).map(|o| !o.is_dead() && o.code() == OpCode::CPUI_CALL).unwrap_or(false);
    if !is_call || !is_register(addr) || data.kuna_passthrough_vararg_calls().contains(&op) {
        return None;
    }
    let entry = fc.get_entry_address();
    let stated = data.kuna_protoorder_types(entry)?;
    if !stated.arity_sound || stated.vararg_tail.iter().any(|a| a == addr) {
        return None;
    }
    let stated_size = stated.inputs.iter().find(|(a, _, _)| a == addr).map(|(_, s, _)| *s)?;
    let facts = data.kuna_callee_entry_dead(entry)?;
    let size = size.min(stated_size);
    if !facts.proves_input(addr, size) {
        return None;
    }
    let width = facts.live_input_width(addr, size).unwrap_or(size);
    data.get_func_proto().possible_input_param(addr, width).then_some(width)
}

/// Does `[addr, addr+size)` hold nothing a call takes but something it gives
/// back -- the return register on a convention that passes no argument in it?
fn return_only(proto: &crate::fspec::FuncProto, addr: &Address, size: int4) -> bool {
    is_register(addr)
        && proto.characterize_as_output(addr, size) != crate::fspec::Containment::NoContainment
        && !proto.possible_input_param(addr, size)
}

/// Did the caller set call spec `idx` up as a variadic call?
///
/// A register that carries no argument but the return value is written, and
/// not read again except by the writing instruction's own ops -- its operands
/// and its flag computation -- between the call and the last call or block start
/// before it: `xor %eax,%eax` before a SysV variadic call is the
/// vector-register count in `al`,
/// and nothing else has a reason to write the register the call is about to
/// overwrite. A variadic callee's recovered list can stop on a tail
/// register its body saves (gnulib `open_safer(char const *,int,...)` saves only
/// `rdx`), so the caller's side is what says the tail is optional.
fn set_up_as_variadic(data: &Funcdata, idx: int4) -> bool {
    let fc = data.get_call_specs(idx);
    let proto = fc.proto();
    let mut cur = data.op_previous_op(fc.get_op());
    // The instruction whose ops have read the register so far. Walking back, an
    // instruction's flag computation is reached BEFORE the write it derives
    // from, and `xor %eax,%eax` -- the idiom gcc actually emits for the vector
    // count -- reads the register it zeroes twice over: once as its own operand,
    // once in `ZF = (EAX == 0)`. Those reads are the writing instruction's own,
    // and only a read by some OTHER instruction says the register was carrying
    // something the call is about to take.
    let mut read_at: Option<Address> = None;
    while let Some(op) = cur {
        let Some(o) = data.obank().get(op) else { return false };
        if matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER) {
            return false;
        }
        let reads_it = (0..o.num_input()).filter_map(|i| o.get_in(i)).any(|v| {
            data.vbank().get(v).map(|v| return_only(proto, v.get_addr(), v.get_size())).unwrap_or(false)
        });
        if reads_it {
            match &read_at {
                Some(a) if a != o.get_addr() => return false,
                Some(_) => {}
                None => read_at = Some(o.get_addr().clone()),
            }
        }
        let writes_it = o
            .get_out()
            .and_then(|v| data.vbank().get(v))
            .map(|v| return_only(proto, v.get_addr(), v.get_size()))
            .unwrap_or(false);
        if writes_it {
            return read_at.as_ref().is_none_or(|a| a == o.get_addr());
        }
        cur = data.op_previous_op(op);
    }
    false
}

/// Is the function itself variadic -- does its entry block read a register
/// that carries no argument but the return value before writing it or calling
/// anything (a SysV register-save prologue's `test %al,%al`)?
///
/// Asked before heritage, of the raw p-code. Such a function's own parameter
/// recovery reads every register the prologue saves, so an argument this pass
/// supplies is what tips the saved tail into its list: gnulib
/// `version_etc(FILE *,char const *,char const *,char const *,...)` would come
/// out with fourteen parameters.
fn reads_the_vararg_count(data: &Funcdata) -> bool {
    if data.bblocks_get_size() == 0 {
        return false;
    }
    let proto = data.get_func_proto();
    let mut written: Vec<(u64, int4)> = Vec::new();
    let mut cur = data.bb_op_head(data.bblocks_get_block(0));
    while let Some(op) = cur {
        let Some(o) = data.obank().get(op) else { return false };
        if matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER) {
            return false;
        }
        for i in 0..o.num_input() {
            let Some(v) = o.get_in(i).and_then(|v| data.vbank().get(v)) else { continue };
            if !return_only(proto, v.get_addr(), v.get_size()) {
                continue;
            }
            let (off, end) = (v.get_offset(), v.get_offset() + v.get_size() as u64);
            if !written.iter().any(|&(w, ws)| w <= off && end <= w + ws as u64) {
                return true;
            }
        }
        if let Some(v) = o.get_out().and_then(|v| data.vbank().get(v)) {
            if return_only(proto, v.get_addr(), v.get_size()) {
                written.push((v.get_offset(), v.get_size()));
            }
        }
        cur = o.basic_neighbours().1;
    }
    false
}

/// Is `addr` in a processor register space?
fn is_register(addr: &Address) -> bool {
    addr.get_space()
        .map(|s| s.get_type() == spacetype::IPTR_PROCESSOR)
        .unwrap_or(false)
}

/// Does any Varnode of the function share a byte with `[addr, addr+size)`?
fn touched(data: &Funcdata, addr: &Address, size: int4) -> bool {
    let off = addr.get_offset();
    let end = off.wrapping_add(size as u64);
    let lo = Address::new(
        std::rc::Rc::clone(addr.get_space().expect("register address has a space")),
        off.saturating_sub(64),
    );
    let hi = addr + size as i64;
    data.vbank().iter_loc_addr_range(&lo, &hi).any(|id| {
        data.vbank()
            .get(id)
            .map(|v| {
                let voff = v.get_offset();
                voff < end && off < voff.wrapping_add(v.get_size() as u64)
            })
            .unwrap_or(false)
    })
}

/// The calls no other call can execute before: the first CALL or CALLIND of a
/// block that every path from the entry reaches without passing one.
///
/// A register no op of the function touches reaches such a call as the
/// function's own input; at any other call it may be a clobber, or the return
/// value, of the call before it.
fn first_calls(data: &Funcdata) -> Vec<OpId> {
    let n = data.bblocks_get_size();
    let is_call = |op: OpId| {
        data.obank().get(op).map(|o| matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)).unwrap_or(false)
    };
    let first_call_in = |i: int4| {
        let mut cur = data.bb_op_head(data.bblocks_get_block(i));
        while let Some(op) = cur {
            if is_call(op) {
                return Some(op);
            }
            cur = data.obank().get(op).and_then(|o| o.basic_neighbours().1);
        }
        None
    };
    let index_of = |bl| data.bblocks_ref().block(bl).get_index();
    let calls: Vec<Option<OpId>> = (0..n).map(first_call_in).collect();
    let mut after_call = vec![false; n as usize];
    loop {
        let mut changed = false;
        for i in 0..n {
            let b = data.bblocks_ref().block(data.bblocks_get_block(i));
            let entered_after = (0..b.size_in()).any(|k| after_call.get(index_of(b.get_in(k)) as usize) == Some(&true));
            let out = entered_after || calls[i as usize].is_some();
            if out && !after_call[i as usize] {
                after_call[i as usize] = true;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    (0..n)
        .filter(|&i| {
            let b = data.bblocks_ref().block(data.bblocks_get_block(i));
            !(0..b.size_in()).any(|k| after_call.get(index_of(b.get_in(k)) as usize) == Some(&true))
        })
        .filter_map(|i| calls[i as usize])
        .collect()
}

/// Give each call whose callee states a register parameter the function never
/// touches a trial and a read of it, and record the claim.
///
/// Called at the end of `ActionFuncLink`, after every call's locked inputs are
/// in place, so "touched" is asked of the whole function once, and only at a
/// call no other call can precede ([`first_calls`]), where the read is the
/// function's own input. Inert with the option off.
pub fn claim_untouched_registers(data: &mut Funcdata) {
    if !data.get_arch().pass_through {
        return;
    }
    let variadic = reads_the_vararg_count(data);
    data.kuna_set_passthrough_variadic(variadic);
    if variadic {
        return;
    }
    let returned_call_result = stated_tail_return(data);
    let vararg: Vec<OpId> = (0..data.num_calls())
        .filter(|&i| set_up_as_variadic(data, i))
        .map(|i| data.get_call_specs(i).get_op())
        .collect();
    data.kuna_set_passthrough_vararg_calls(vararg);
    let first = first_calls(data);
    let mut claims: Vec<PassThroughClaim> = Vec::new();
    for idx in 0..data.num_calls() {
        if !data.get_call_specs(idx).is_input_active() || !first.contains(&data.get_call_specs(idx).get_op()) {
            continue;
        }
        let entry = data.get_call_specs(idx).get_entry_address().clone();
        let stated: Vec<(Address, int4)> = match data.kuna_protoorder_types(&entry) {
            Some(s) => s.inputs.iter().map(|(a, sz, _)| (a.clone(), *sz)).collect(),
            None => continue,
        };
        let op = data.get_call_specs(idx).get_op();
        for (pos, (addr, stated_size)) in stated.iter().cloned().enumerate() {
            let Some(size) = stated_width(data, data.get_call_specs(idx), &addr, stated_size) else { continue };
            if !no_hole_before(data, &stated, pos, &claims) {
                continue;
            }
            if data.get_call_specs(idx).active_input().which_trial(&addr, stated_size) >= 0 {
                continue;
            }
            let claimed = claims.iter().any(|c| c.addr == addr && c.size == size);
            if !claimed && touched(data, &addr, stated_size) {
                continue;
            }
            let nin = data.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
            let vn = data.new_varnode(size, &addr, None);
            if data.op_insert_input(op, vn, nin).is_err() {
                continue;
            }
            let ai = data.get_call_specs_mut(idx).get_active_input();
            ai.register_trial(&addr, size);
            let t = ai.get_num_trials() - 1;
            ai.get_trial_mut(t).set_slot(nin);
            match claims.iter_mut().find(|c| c.addr == addr && c.size == size) {
                Some(c) => c.arg_owners.push(op),
                None => claims.push(PassThroughClaim { addr, size, arg_owners: vec![op], ret_owners: Vec::new() }),
            }
        }
    }
    if let Some(ret) = returned_call_result {
        claim_tail_return(data, &mut claims, ret);
    }
    data.kuna_set_passthrough_claims(claims);
}

/// Can the claim of the register at `pos` in the callee's stated list be made
/// without materializing a parameter this function does not have?
///
/// Parameters are positional: a claim at the third stated register gives the
/// function a third parameter, and the first two come with it whether or not
/// anything put a value in them. Every earlier stated register must therefore be
/// one this function could be carrying -- already claimed here (so forwarded
/// untouched), or one its own entry does not WRITE before reading on every path
/// ([`crate::p4_calls::kuna_calleedeadarg::CalleeEntryDead::proves_dead`],
/// asked of this function's own body). A register the function loads for itself
/// before the call is not a parameter, and a claim reaching across it prints an
/// argument-less slot: the `protoorder` fixture's `overrec` sets `rsi` to 16 and
/// forwards `rdx`, and rendered `void overrec(unsigned long *a0,unsigned long
/// a1,long a2)` against the call site `overrec(v1)`.
///
/// The evidence is one-sided, as everywhere this walk is read: a body the walk
/// cannot settle keeps the claim.
fn no_hole_before(data: &Funcdata, stated: &[(Address, int4)], pos: usize, claims: &[PassThroughClaim]) -> bool {
    if pos == 0 {
        return true;
    }
    let Some(facts) = data.kuna_callee_entry_dead(data.get_address()) else { return true };
    stated[..pos]
        .iter()
        .all(|(a, sz)| overlaps_claim(claims, a, *sz) || !facts.proves_dead(a, *sz))
}

/// How many blocks [`producing_call`] walks back from a RETURN.
const RETURN_WALK_BLOCKS: usize = 4;

/// The call whose result a RETURN hands back when nothing in between writes the
/// return register: the last CALL before `ret` in its block, or in the chain of
/// single predecessors above it. `None` when a join, an indirect call or the
/// entry comes first.
fn producing_call(data: &Funcdata, ret: OpId) -> Option<OpId> {
    let mut cur = data.op_previous_op(ret);
    let mut bl = data.obank().get(ret)?.get_parent()?;
    for _ in 0..RETURN_WALK_BLOCKS {
        while let Some(op) = cur {
            match data.obank().get(op)?.code() {
                OpCode::CPUI_CALL => return Some(op),
                OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER => return None,
                _ => cur = data.op_previous_op(op),
            }
        }
        let b = data.bblocks_ref().block(bl);
        if b.size_in() != 1 {
            return None;
        }
        bl = b.get_in(0);
        cur = data.bb_op_tail(bl);
    }
    None
}

/// The stated return value of the call every live RETURN of the function hands
/// back, when there is one: the storage, size and type the callee's own
/// recovery gave it, the same for every such call.
///
/// Asked before any argument is claimed, so "untouched" means untouched by the
/// function's own code. `None` unless the function's own output is recovered
/// (not locked), every live RETURN is reached from a direct call with nothing
/// between, every one of those callees states a non-`void` return in the same
/// register, and no op of the function touches that register.
fn stated_tail_return(data: &Funcdata) -> Option<(Address, int4, Vec<OpId>)> {
    if data.get_func_proto().is_output_locked() || data.get_active_output().is_none() {
        return None;
    }
    let rets: Vec<OpId> = data
        .obank()
        .iter_code(OpCode::CPUI_RETURN)
        .filter(|&r| data.obank().get(r).map(|o| !o.is_dead() && o.get_halt_type() == 0).unwrap_or(false))
        .collect();
    if rets.is_empty() {
        return None;
    }
    let mut storage: Option<(Address, int4)> = None;
    let mut producers: Vec<OpId> = Vec::new();
    for &r in &rets {
        let call = producing_call(data, r)?;
        let idx = (0..data.num_calls()).find(|&i| data.get_call_specs(i).get_op() == call)?;
        let fc = data.get_call_specs(idx);
        if fc.proto().is_output_locked() || !fc.is_output_active() {
            return None;
        }
        let (addr, size, _) = data.kuna_protoorder_types(fc.get_entry_address())?.output.clone()?;
        match &storage {
            Some((a, s)) if *a != addr || *s != size => return None,
            _ => storage = Some((addr, size)),
        }
        if !producers.contains(&call) {
            producers.push(call);
        }
    }
    let (addr, size) = storage?;
    if !is_register(&addr) || touched(data, &addr, size) {
        return None;
    }
    let out = data.get_func_proto().characterize_as_output(&addr, size);
    if out == crate::fspec::Containment::NoContainment {
        return None;
    }
    Some((addr, size, producers))
}

/// Give every live RETURN a read of the return register its tail calls produce,
/// register the function's return trial for it, and record the claim.
///
/// The RETURN reads are the function's own return trial, so heritage's
/// `guardReturns` is kept off the range ([`suppresses_return_trial`]); the producing
/// calls keep their return-value trial, which is what hands the RETURN the
/// callee's result.
fn claim_tail_return(data: &mut Funcdata, claims: &mut Vec<PassThroughClaim>, ret: (Address, int4, Vec<OpId>)) {
    let (addr, size, producers) = ret;
    let rets: Vec<OpId> = data
        .obank()
        .iter_code(OpCode::CPUI_RETURN)
        .filter(|&r| data.obank().get(r).map(|o| !o.is_dead() && o.get_halt_type() == 0).unwrap_or(false))
        .collect();
    let nins: Vec<int4> = rets.iter().filter_map(|&r| data.obank().get(r).map(|o| o.num_input())).collect();
    if nins.len() != rets.len() || nins.iter().any(|&n| n != nins[0]) {
        return;
    }
    let slot = nins[0];
    for &r in &rets {
        let vn = data.new_varnode(size, &addr, None);
        if data.op_insert_input(r, vn, slot).is_err() {
            return;
        }
    }
    if let Some(active) = data.get_active_output_mut() {
        active.register_trial(&addr, size);
        let t = active.get_num_trials() - 1;
        active.get_trial_mut(t).set_slot(slot);
    }
    match claims.iter_mut().find(|c| c.addr == addr && c.size == size) {
        Some(c) => c.ret_owners.extend(producers),
        None => claims.push(PassThroughClaim { addr, size, arg_owners: Vec::new(), ret_owners: producers }),
    }
}

/// Is `vn`, read by a RETURN in `[addr, addr+size)`, the result of a tail call
/// this pass claimed the return value of?
///
/// The scoring arm of `ActionReturnRecovery`. Upstream's `ancestorOpUse`
/// refuses an INDIRECT creation outright ("an indication of an output trial"),
/// so a callee whose return value is still recovered from trials never reaches
/// a RETURN; a claimed tail call's creation is the callee's stated result.
pub fn returns_tail_result(data: &Funcdata, vn: VarnodeId, addr: &Address, size: int4) -> bool {
    if !data.get_arch().pass_through {
        return false;
    }
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.get_addr() != addr || v.get_size() != size {
        return false;
    }
    let Some(def) = v.get_def().and_then(|d| data.obank().get(d)) else { return false };
    if !def.is_indirect_creation() {
        return false;
    }
    let Some(iop) = def.get_in(1).and_then(|i| data.vbank().get(i)) else { return false };
    let call = OpId::from(slotmap::KeyData::from_ffi(iop.get_offset()));
    data.kuna_passthrough_claims()
        .iter()
        .any(|c| c.addr == *addr && c.size == size && c.ret_owners.contains(&call))
}

/// The type a tail call this pass claimed hands back: the callee's own
/// recovered return type, offered as the CALL output's local type.
pub fn tail_return_type(data: &Funcdata, op: OpId, size: int4) -> Option<std::rc::Rc<crate::dtype::Datatype>> {
    if !data.get_arch().pass_through {
        return None;
    }
    let claim = data.kuna_passthrough_claims().iter().find(|c| c.ret_owners.contains(&op))?;
    if claim.size != size {
        return None;
    }
    let fc = (0..data.num_calls()).map(|i| data.get_call_specs(i)).find(|fc| fc.get_op() == op)?;
    let (_, osize, ct) = data.kuna_protoorder_types(fc.get_entry_address())?.output.as_ref()?;
    (*osize == size && ct.get_size() == size).then(|| std::rc::Rc::clone(ct))
}

/// Must heritage leave `[addr, addr+size)` out of the function's own RETURN
/// trials?
///
/// A range this pass claimed is one the option-off run heritages only if a
/// later hole fill reads it, so the function's RETURN never registers it: a
/// claimed return register's RETURN trial is the pass's own
/// ([`claim_tail_return`]), and a claimed argument register gives the function
/// no return value it would not have had. Every CALL registers the range as
/// usual -- the trials are left unscored instead ([`claimed_range`]) -- so a
/// later hole fill needs no new read of a range dead-code removal has already
/// visited, and an earlier call keeps the return value a later read of the
/// range asks for (a Cortex-M `double` returned in r0:r1).
pub fn suppresses_return_trial(data: &Funcdata, addr: &Address, size: int4) -> bool {
    overlaps_claim(data.kuna_passthrough_claims(), addr, size)
}

/// Does `[addr, addr+size)` share a byte with any claimed range?
fn overlaps_claim(claims: &[PassThroughClaim], addr: &Address, size: int4) -> bool {
    let Some(sp) = addr.get_space() else { return false };
    let off = addr.get_offset();
    let end = off.wrapping_add(size.max(0) as u64);
    claims.iter().any(|c| {
        let same_space = c.addr.get_space().map(|s| s.get_index()) == Some(sp.get_index());
        let coff = c.addr.get_offset();
        same_space && coff < end && off < coff.wrapping_add(c.size as u64)
    })
}

/// Is the argument trial `[addr, addr+size)` on a range this pass claimed?
///
/// Such a trial, at any call, is never scored: `checkInputTrialUse` marks it
/// inactive, which is where the option-off run's trial for the slot starts --
/// an unreferenced one `fillinMap` adds only to fill a hole before a later
/// argument. Left to `AncestorRealistic`, a register that reaches the call
/// through an earlier call is "killed by call", definitely not used, and
/// `forceNoUse` drops every argument after it.
pub fn claimed_range(data: &Funcdata, addr: &Address, size: int4) -> bool {
    data.get_arch().pass_through && overlaps_claim(data.kuna_passthrough_claims(), addr, size)
}

/// The forwarded registers a finalized call may still take, captured before its
/// dropped trials' Varnodes are gone.
#[derive(Clone, Debug)]
pub struct PendingPassThrough {
    /// The CALL op.
    pub op: OpId,
    /// `(storage, width, the function's own incoming Varnode)` per register.
    pub candidates: Vec<(Address, int4, VarnodeId)>,
}

/// Record which of `fc`'s unused register trials stand on the function's own
/// incoming register with the callee's recovered prototype reading it, and
/// retire the unused trials on a range this pass claimed.
///
/// Called by `buildInputFromTrials` before anything else reads the trials. A
/// retired trial is invisible to the sibling and body rescues that run after it,
/// as it is with the option off, where the range was never heritaged and the
/// trial never existed.
pub fn capture(fc: &mut FuncCallSpecs, data: &Funcdata) -> Option<PendingPassThrough> {
    if !data.get_arch().pass_through {
        return None;
    }
    let op = fc.get_op();
    let mut candidates: Vec<(Address, int4, VarnodeId)> = Vec::new();
    let mut retire: Vec<int4> = Vec::new();
    for i in 0..fc.active_input().get_num_trials() {
        let t = fc.active_input().get_trial(i);
        if t.is_used() || t.is_definitely_not_used() || t.is_unref() || t.get_slot() < 1 {
            continue;
        }
        let (addr, size) = (t.get_address().clone(), t.get_size());
        if claimed_range(data, &addr, size) {
            retire.push(i);
        }
        let Some(vn) = data.obank().get(op).and_then(|o| o.get_in(t.get_slot())) else { continue };
        let own_input = data
            .vbank()
            .get(vn)
            .map(|v| v.is_input() && !v.is_written() && *v.get_addr() == addr && v.get_size() == size)
            .unwrap_or(false);
        if !own_input {
            continue;
        }
        if let Some(width) = stated_width(data, fc, &addr, size) {
            candidates.push((addr, width, vn));
        }
    }
    for i in retire {
        fc.get_active_input().get_trial_mut(i).mark_no_use();
    }
    (!candidates.is_empty()).then_some(PendingPassThrough { op, candidates })
}

/// Extend each captured call's final argument list with the forwarded registers
/// its callee's recovered prototype names next, once every other rule has
/// finalized it. Returns how many calls gained arguments.
///
/// Runs last in `ActionActiveParam::apply`, so it can only add: the list the
/// upstream scoring and the sibling and body rescues settled on is kept as it
/// is, and must be a leading run of the stated list. Registers are taken in the
/// stated order and the extension stops at the first one the function does not
/// forward untouched.
pub fn extend_pending(data: &mut Funcdata, pending: &[PendingPassThrough]) -> int4 {
    pending.iter().filter(|p| extend_one(data, p)).count() as int4
}

/// [`extend_pending`] for one call.
fn extend_one(data: &mut Funcdata, p: &PendingPassThrough) -> bool {
    let Some(idx) = data.get_call_specs_index(p.op) else { return false };
    let (entry, current) = {
        let fc = data.get_call_specs(idx);
        (fc.get_entry_address().clone(), fc.final_input_storage().to_vec())
    };
    let nin = match data.obank().get(p.op) {
        Some(o) if !o.is_dead() && o.code() == OpCode::CPUI_CALL => o.num_input(),
        _ => return false,
    };
    if nin != current.len() as int4 + 1 || current.iter().any(|(a, _)| !is_register(a)) {
        return false;
    }
    let Some(stated) = data.kuna_protoorder_types(&entry) else { return false };
    let stated: Vec<Address> = stated.inputs.iter().map(|(a, _, _)| a.clone()).collect();
    let have: Vec<Address> = current.iter().map(|(a, _)| a.clone()).collect();
    let offered: Vec<Address> = p.candidates.iter().map(|(a, _, _)| a.clone()).collect();
    let added: Vec<(Address, int4, VarnodeId)> =
        extension(&have, &stated, &offered).into_iter().map(|k| p.candidates[k].clone()).collect();
    if added.is_empty() {
        return false;
    }
    let mut newparam: Vec<VarnodeId> = (0..nin).filter_map(|i| data.obank().get(p.op).and_then(|o| o.get_in(i))).collect();
    if newparam.len() != nin as usize {
        return false;
    }
    let mut storage = current;
    for (addr, width, vn) in added {
        let Some(vsize) = data.vbank().get(vn).filter(|v| v.is_input() && *v.get_addr() == addr).map(|v| v.get_size())
        else {
            return false;
        };
        let arg = if width < vsize {
            match truncate_before(data, vn, width, p.op) {
                Some(v) => v,
                None => return false,
            }
        } else {
            vn
        };
        newparam.push(arg);
        storage.push((addr, width));
    }
    if data.op_set_all_input(p.op, &newparam).is_err() {
        return false;
    }
    data.get_call_specs_mut(idx).set_final_input_storage(storage);
    true
}

/// Which of `offered` extend the argument list `have` along the callee's stated
/// list, as indices into `offered`, in order.
///
/// `have` must be a leading run of `stated`; the registers `stated` names after
/// it are taken while each is offered, and the first that is not ends the
/// extension, because parameters are positional.
pub fn extension(have: &[Address], stated: &[Address], offered: &[Address]) -> Vec<usize> {
    if have.len() >= stated.len() || have.iter().zip(stated).any(|(a, s)| a != s) {
        return Vec::new();
    }
    stated[have.len()..].iter().map_while(|s| offered.iter().position(|a| a == s)).collect()
}

/// The low `width` bytes of `vn`, as a `SUBPIECE` inserted before `op`: what
/// `buildInputFromTrials` does for a trial narrower than its Varnode.
fn truncate_before(data: &mut Funcdata, vn: VarnodeId, width: int4, op: OpId) -> Option<VarnodeId> {
    let (vaddr, vsize) = data.vbank().get(vn).map(|v| (v.get_addr().clone(), v.get_size()))?;
    let big_endian = data.get_arch().manage().get_default_code_space().map(|s| s.is_big_endian()).unwrap_or(false);
    let outaddr = if big_endian { &vaddr + ((vsize - width) as i64) } else { vaddr };
    let opaddr = data.obank().get(op)?.get_addr().clone();
    let newop = data.new_op(2, opaddr);
    let out = data.new_varnode_out(width, &outaddr, newop).ok()?;
    data.op_set_opcode_code(newop, OpCode::CPUI_SUBPIECE);
    data.op_set_input(newop, vn, 0).ok()?;
    let zero = data.new_constant(1, 0);
    data.op_set_input(newop, zero, 1).ok()?;
    data.op_insert_before(newop, op);
    Some(out)
}

#[cfg(test)]
#[path = "kuna_passthrough/tests.rs"]
mod tests;
