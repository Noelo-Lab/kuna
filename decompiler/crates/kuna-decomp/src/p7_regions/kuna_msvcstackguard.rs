//! (kuna `msvcstackguard`) Strip the MSVC `/GS` frame-cookie boilerplate — the
//! sibling of [`kuna_stackguard`](crate::kuna_stackguard) for the Windows shape.
//!
//! # Why the glibc matcher cannot see this
//!
//! `stackguard` pins the canary by its **compare**: a `CBRANCH` whose boolean
//! compares two values that both derive from a LOAD of `<tls_base> + 0x28`, with
//! the `__stack_chk_fail` handler on the failure edge.  Every ingredient of that
//! is absent from an MSVC `/GS` function.  MSVC scrambles a global cookie with
//! the stack pointer at entry and hands the unscrambled value to a checker at
//! each exit:
//!
//! ```text
//!   prologue:  sub  rsp,0x48
//!              mov  rax,[__security_cookie]
//!              xor  rax,rsp                  ; cookie ^ SP
//!              mov  [rsp+0x38],rax
//!   epilogue:  mov  rcx,[rsp+0x38]
//!              xor  rcx,rsp                  ; (cookie ^ SP) ^ SP  ==  cookie
//!              call __security_check_cookie
//! ```
//!
//! The compare lives inside the callee, so the guarded function contains no
//! `CBRANCH` at all, no `fs:0x28` load and no failure edge — `stackguard on` and
//! `off` are byte-identical on a Windows PE, and the boilerplate survives in
//! every protected function.
//!
//! # The recognizer is the algebra, not the name
//!
//! A stripped PE has no `__security_check_cookie` symbol, so the callee name is
//! not a usable signal.  What cannot be stripped away is the arithmetic: the
//! value handed to the checker is `(K ^ SP) ^ SP`, and the two `SP` operands are
//! the *same* stack-pointer value.  [`cookie_cancel`] requires exactly that —
//! an `INT_XOR` feeding a `CALL` argument whose operands are (a) a
//! stack-pointer-derived Varnode at frame offset `d`, and (b) a value produced by
//! a second `INT_XOR` of a non-stack, non-constant operand with a
//! stack-pointer-derived Varnode at the *same* offset `d`.  Both offsets are
//! resolved to the function's entry stack pointer by [`stack_pointer_offset`],
//! which walks the `PTRSUB`/`PTRADD`/`INT_ADD` chain the stack-pointer
//! normalization leaves behind, so a `/Od` frame (`xor rcx,rsp` after
//! `sub rsp,N`) and an `/O2` frame agree.
//!
//! A cookie slot may be carried through many call guards or around a loop. The
//! value peel therefore has its own finite 256-link budget, and a
//! `MULTIEQUAL` is accepted by a cycle-aware fixed point only when every
//! non-backedge input proves the same scramble and at least one seed exists.
//!
//! The cost of a false positive is a deleted call and a deleted argument
//! computation, which is why the envelope is narrow and the option ships OFF.
//! The shape is not reachable by accident: `(K ^ SP) ^ SP` cancels to `K`, so
//! any code that computes it and passes it to a function is scrambling and
//! unscrambling with the stack pointer on purpose.  GCC/Clang's
//! `-fstack-protector` reads `%fs:0x28` and never touches the stack pointer, so
//! the pass is structurally inert on an ELF corpus — the same argument the
//! sibling `cookiescramble` alias exemption makes about the same instruction.
//!
//! Three further guards keep the edit honest:
//!
//! * the victim must be a direct `CPUI_CALL` (`__security_check_cookie` is
//!   statically linked into the image; it is never an indirect thunk call);
//! * the call's output must have no reader — the checker returns `void`, and
//!   destroying a read Varnode is not recoverable;
//! * **exactly one** of the call's arguments may be a cookie cancel.  A call
//!   that takes the pattern twice is not the one-argument checker.
//!
//! The CALL is load-bearing, and that is the scope: MSVC always emits the check
//! as a call to the statically linked `__security_check_cookie`, so a hand-rolled
//! variant that inlines the compare (`xor rcx,rsp; cmp rcx,[cookie]; jne fail`)
//! keeps its compare and is left alone.
//!
//! # The edit
//!
//! The recognizer runs after SSA, but call guarding has already replaced the
//! caller's pre-check return register with a `KILLEDBYCALL` INDIRECT by then.
//! On the first exact match this action records the call's instruction address
//! in the function's P0 override store and requests a pipeline restart.  During
//! replay, `Heritage::guard_calls` treats only ABI output storage at that exact
//! call as unaffected (only for an actual `KILLEDBYCALL`, and never across an
//! explicit effect override). The same non-destructive marker handoff also runs
//! when `calleeretpreserves` is on and the exact checker is declared and locked
//! `void`: it retains ABI output but leaves the checker and all cookie algebra
//! visible. No generic callee-body claim is made, so a failure tail containing
//! nested calls remains conservative. The normal SSA construction can then
//! retain every reaching caller definition.
//!
//! The stock pair `Funcdata::block_remove_internal` uses for a CALL inside a
//! deleted block, and the pair `cleanupcode` uses for a Rust drop call:
//! [`Funcdata::delete_call_specs`] to drop the `FuncCallSpecs` record, then
//! [`Funcdata::op_destroy`].  Nothing else is deleted by hand.  The epilogue
//! `INT_XOR` loses its last reader and dies in the following `ActionDeadCode`;
//! and the repeating `fullloop` re-runs `mainloop` over the reduced function
//! before P8 structures it.
//!
//! The prologue init is released the same way `stackguard` releases the glibc
//! canary init: [`collect_cookie_slots`] resolves the addrtied stack storage the
//! scrambled cookie was written to, and
//! [`release_canary_slots`](crate::kuna_stackguard::release_canary_slots)
//! clears its `addrforce` (plus `ScopeLocal::markNotMapped` on the slot) so the
//! store, the cookie read and the entry-side scramble die through the ordinary
//! consume fixpoint instead of surviving as `v5 = dat_140074040 ^ (unsigned
//! long long)v1;`.  A slot version still feeding a live reader is never removed:
//! the release is a liveness change, not a deletion.
//!
//! # The option
//!
//! `option msvcstackguard on|off`, default **off** — the same escape hatch
//! `stackguard off` gives a reader who wants to audit the protector itself,
//! kept separate so flipping one does not silently change the other.  The live
//! flag is
//! [`Architecture::strip_msvc_stack_guard`](crate::architecture::Architecture),
//! mirrored onto [`ArchContext`](crate::context::ArchContext).

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::kuna_stackguard::release_canary_slots;

/// How far the structural derivation recurses before giving up.
const WALK_DEPTH: int4 = 32;

/// How many value-preserving copies/INDIRECTs a long function may carry the
/// cookie through. One INDIRECT can be introduced at every call, so this bound
/// is intentionally larger than the algebra walk while remaining finite.
const PEEL_DEPTH: int4 = 256;

/// The storage of the stack space's base register (`getStackSpace()->
/// getSpacebase(0)`), or `None` when the architecture has no stack space.
fn stack_pointer_storage(data: &Funcdata) -> Option<(Rc<AddrSpace>, uintb, int4)> {
    let stackspc = data.get_arch().manage().get_stack_space().map(Rc::clone)?;
    let point = stackspc.get_spacebase(0).ok()?;
    let space = point.space.clone()?;
    Some((space, point.offset, point.size as int4))
}

/// Peel the value-preserving ops between a Varnode and the computation that
/// produced it: `COPY`, `CAST`, and the `INDIRECT` that carries a value across a
/// call.  Returns the deepest Varnode reached.
fn peel(mut vn: VarnodeId, data: &Funcdata) -> VarnodeId {
    for _ in 0..PEEL_DEPTH {
        let Some(v) = data.vbank().get(vn) else { return vn };
        if !v.is_written() {
            return vn;
        }
        let def = v.get_def().expect("peel: written vn def");
        let Some(dop) = data.obank().get(def) else { return vn };
        match dop.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INDIRECT => {
                vn = dop.get_in(0).expect("peel: in0");
            }
            _ => return vn,
        }
    }
    vn
}

/// Byte offset of `vn` from the function's entry stack pointer, when `vn`
/// provably holds a stack-pointer value.
///
/// The base case is a Varnode whose storage IS the stack-pointer register (the
/// `getSpacebase(0)` triple) or that carries the `spacebase` flag the
/// constructed spacebase input gets; from there the walk follows the additive
/// chain the stack-pointer normalization leaves behind (`PTRSUB` and `INT_ADD`
/// with a constant, `PTRADD` with a constant index).  Anything else — a
/// `MULTIEQUAL` the normalization could not resolve, an unrelated register —
/// answers `None`, and the caller declines.
fn stack_pointer_offset(
    vn: VarnodeId,
    depth: int4,
    data: &Funcdata,
    sp: &(Rc<AddrSpace>, uintb, int4),
) -> Option<i64> {
    if depth <= 0 {
        return None;
    }
    let vn = peel(vn, data);
    let v = data.vbank().get(vn)?;
    if v.is_constant() {
        return None;
    }
    let is_sp_storage = Rc::ptr_eq(v.get_space(), &sp.0)
        && v.get_offset() == sp.1
        && v.get_size() == sp.2;
    if is_sp_storage || v.is_spacebase() {
        if !v.is_written() {
            return Some(0); // the entry stack pointer itself
        }
    } else if !v.is_written() {
        return None;
    }
    let def = v.get_def()?;
    let dop = data.obank().get(def)?;
    let (code, nin) = (dop.code(), dop.num_input());
    match code {
        OpCode::CPUI_PTRSUB | OpCode::CPUI_INT_ADD => {
            let a = dop.get_in(0)?;
            let b = dop.get_in(1)?;
            let (base, konst) = if data.vbank().get(b)?.is_constant() {
                (a, b)
            } else if data.vbank().get(a)?.is_constant() {
                (b, a)
            } else {
                return None;
            };
            let k = sign_extend(data, konst)?;
            stack_pointer_offset(base, depth - 1, data, sp).map(|o| o.wrapping_add(k))
        }
        OpCode::CPUI_PTRADD if nin == 3 => {
            let base = dop.get_in(0)?;
            let idx = sign_extend(data, dop.get_in(1)?)?;
            let sz = sign_extend(data, dop.get_in(2)?)?;
            stack_pointer_offset(base, depth - 1, data, sp)
                .map(|o| o.wrapping_add(idx.wrapping_mul(sz)))
        }
        // A stack-pointer-storage Varnode written by anything else (a restored
        // spill, an unresolved MULTIEQUAL) is not a known offset.
        _ => None,
    }
}

/// The signed value of a constant Varnode, at its own size.
fn sign_extend(data: &Funcdata, vn: VarnodeId) -> Option<i64> {
    let v = data.vbank().get(vn)?;
    if !v.is_constant() {
        return None;
    }
    let size = v.get_size();
    if size <= 0 || size > 8 {
        return None;
    }
    let raw = v.get_offset();
    let shift = 64 - (size as u32) * 8;
    Some(((raw << shift) as i64) >> shift)
}

/// Is `vn` the entry-side scramble `K ^ SP`?
///
/// Answers the frame offset every scramble on the chain used, and pushes the
/// producing `INT_XOR` ops onto `inits` (more than one when the value reaches
/// the epilogue through a `MULTIEQUAL` join).  A `MULTIEQUAL` only qualifies
/// when EVERY input is a scramble at that same offset — one non-cookie input
/// and the whole match is declined.
///
/// `memo` caches completed answers. `active` distinguishes a loop back-edge
/// from a failed derivation: a loop-carried phi is valid when every non-cyclic
/// incoming value is the same scramble and at least one such seed exists.
#[derive(Clone, Copy)]
enum ScrambleWalk {
    Found(i64),
    Backedge,
    No,
}

fn cookie_scramble(
    vn: VarnodeId,
    depth: int4,
    data: &Funcdata,
    sp: &(Rc<AddrSpace>, uintb, int4),
    inits: &mut Vec<OpId>,
) -> Option<i64> {
    let mut memo: BTreeMap<VarnodeId, Option<i64>> = BTreeMap::new();
    let mut active: BTreeSet<VarnodeId> = BTreeSet::new();
    match cookie_scramble_walk(vn, depth, data, sp, &mut memo, &mut active, inits) {
        ScrambleWalk::Found(off) => Some(off),
        ScrambleWalk::Backedge | ScrambleWalk::No => None,
    }
}

fn cookie_scramble_walk(
    vn: VarnodeId,
    depth: int4,
    data: &Funcdata,
    sp: &(Rc<AddrSpace>, uintb, int4),
    memo: &mut BTreeMap<VarnodeId, Option<i64>>,
    active: &mut BTreeSet<VarnodeId>,
    inits: &mut Vec<OpId>,
) -> ScrambleWalk {
    if depth <= 0 {
        return ScrambleWalk::No;
    }
    let vn = peel(vn, data);
    if let Some(cached) = memo.get(&vn) {
        return cached.map(ScrambleWalk::Found).unwrap_or(ScrambleWalk::No);
    }
    if !active.insert(vn) {
        return ScrambleWalk::Backedge;
    }
    let answer = cookie_scramble_uncached(vn, depth, data, sp, memo, active, inits);
    active.remove(&vn);
    match answer {
        ScrambleWalk::Found(off) => {
            memo.insert(vn, Some(off));
        }
        ScrambleWalk::No => {
            memo.insert(vn, None);
        }
        ScrambleWalk::Backedge => {}
    }
    answer
}

/// The body of [`cookie_scramble`], with the memo/cycle bookkeeping lifted out.
fn cookie_scramble_uncached(
    vn: VarnodeId,
    depth: int4,
    data: &Funcdata,
    sp: &(Rc<AddrSpace>, uintb, int4),
    memo: &mut BTreeMap<VarnodeId, Option<i64>>,
    active: &mut BTreeSet<VarnodeId>,
    inits: &mut Vec<OpId>,
) -> ScrambleWalk {
    let Some(v) = data.vbank().get(vn) else { return ScrambleWalk::No };
    if !v.is_written() {
        return ScrambleWalk::No;
    }
    let Some(def) = v.get_def() else { return ScrambleWalk::No };
    let Some(dop) = data.obank().get(def) else { return ScrambleWalk::No };
    match dop.code() {
        OpCode::CPUI_INT_XOR => {
            let Some(a) = dop.get_in(0) else { return ScrambleWalk::No };
            let Some(b) = dop.get_in(1) else { return ScrambleWalk::No };
            let (cookie, off) = match stack_pointer_offset(b, WALK_DEPTH, data, sp) {
                Some(o) => (a, o),
                None => {
                    let Some(off) = stack_pointer_offset(a, WALK_DEPTH, data, sp) else {
                        return ScrambleWalk::No;
                    };
                    (b, off)
                }
            };
            // The cookie operand must be a real loaded value: a constant would
            // make the whole `(K ^ SP) ^ SP` fold to a constant, and a second
            // stack-pointer reference is not a cookie at all.
            let Some(c) = data.vbank().get(peel(cookie, data)) else {
                return ScrambleWalk::No;
            };
            if c.is_constant() {
                return ScrambleWalk::No;
            }
            if stack_pointer_offset(cookie, WALK_DEPTH, data, sp).is_some() {
                return ScrambleWalk::No;
            }
            inits.push(def);
            ScrambleWalk::Found(off)
        }
        OpCode::CPUI_MULTIEQUAL => {
            let n = dop.num_input();
            let mut off: Option<i64> = None;
            for i in 0..n {
                let Some(ini) = dop.get_in(i) else { return ScrambleWalk::No };
                match cookie_scramble_walk(
                    ini,
                    depth - 1,
                    data,
                    sp,
                    memo,
                    active,
                    inits,
                ) {
                    ScrambleWalk::Found(o) => match off {
                        None => off = Some(o),
                        Some(prev) if prev == o => {}
                        Some(_) => return ScrambleWalk::No,
                    },
                    ScrambleWalk::Backedge => {}
                    ScrambleWalk::No => return ScrambleWalk::No,
                }
            }
            off.map(ScrambleWalk::Found).unwrap_or(ScrambleWalk::No)
        }
        _ => ScrambleWalk::No,
    }
}

/// Is `vn` the epilogue's `(K ^ SP) ^ SP` — the value MSVC hands to
/// `__security_check_cookie`?
///
/// Both stack-pointer references must resolve to the SAME frame offset; that
/// equality is the whole recognizer, because it is what makes the two XORs
/// cancel.  Returns the entry-side scramble ops so their storage can be
/// released.
fn cookie_cancel(
    vn: VarnodeId,
    data: &Funcdata,
    sp: &(Rc<AddrSpace>, uintb, int4),
) -> Option<Vec<OpId>> {
    let vn = peel(vn, data);
    let v = data.vbank().get(vn)?;
    if !v.is_written() {
        return None;
    }
    let def = v.get_def()?;
    let dop = data.obank().get(def)?;
    if dop.code() != OpCode::CPUI_INT_XOR {
        return None;
    }
    let a = dop.get_in(0)?;
    let b = dop.get_in(1)?;
    let (saved, unscramble_off) = match stack_pointer_offset(b, WALK_DEPTH, data, sp) {
        Some(o) => (a, o),
        None => (b, stack_pointer_offset(a, WALK_DEPTH, data, sp)?),
    };
    let mut inits: Vec<OpId> = Vec::new();
    let scramble_off = cookie_scramble(saved, WALK_DEPTH, data, sp, &mut inits)?;
    if scramble_off != unscramble_off || inits.is_empty() {
        return None;
    }
    Some(inits)
}

/// The addrtied stack storage the scrambled cookie was written to.
///
/// A forward fixpoint from each entry-side scramble's output over the
/// value-preserving readers — `COPY`/`CAST` (the store into the frame slot),
/// `INDIRECT` (the slot carried across a call), and a `MULTIEQUAL` only when
/// EVERY input is already known to hold the scramble.  Every addrtied member of
/// the resulting set is the `/GS` cookie slot; nothing else can join it, so the
/// liveness release below cannot reach an unrelated local.
fn collect_cookie_slots(
    inits: &[OpId],
    data: &Funcdata,
    slots: &mut Vec<(Address, int4)>,
) {
    let mut set: BTreeSet<VarnodeId> = BTreeSet::new();
    let mut work: Vec<VarnodeId> = Vec::new();
    for &op in inits {
        if let Some(out) = data.obank().get(op).and_then(|o| o.get_out()) {
            if set.insert(out) {
                work.push(out);
            }
        }
    }
    while let Some(vn) = work.pop() {
        let readers: Vec<OpId> = match data.vbank().get(vn) {
            Some(v) => v.descend_iter().collect(),
            None => continue,
        };
        for r in readers {
            let Some(rop) = data.obank().get(r) else { continue };
            let joins = match rop.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_CAST => true,
                // INDIRECT's in(0) is the value; in(1) is the blocking op.
                OpCode::CPUI_INDIRECT => rop.get_in(0) == Some(vn),
                OpCode::CPUI_MULTIEQUAL => {
                    (0..rop.num_input()).all(|i| rop.get_in(i).is_some_and(|x| set.contains(&x)))
                }
                _ => false,
            };
            if !joins {
                continue;
            }
            let Some(out) = rop.get_out() else { continue };
            if set.insert(out) {
                work.push(out);
            }
        }
    }
    for vn in set {
        let Some(v) = data.vbank().get(vn) else { continue };
        if !v.is_addr_tied() {
            continue;
        }
        let key = (v.get_addr().clone(), v.get_size());
        if !slots.contains(&key) {
            slots.push(key);
        }
    }
}

/// Classify the exact direct, unread-output, one-cookie-cancel call this pass strips.
fn cookie_check_op(
    data: &Funcdata,
    op: OpId,
    sp: &(Rc<AddrSpace>, uintb, int4),
) -> Option<Vec<OpId>> {
    let o = data.obank().get(op)?;
    if o.code() != OpCode::CPUI_CALL || o.get_parent().is_none() {
        return None;
    }
    if let Some(out) = o.get_out() {
        if data.vbank().get(out).map(|v| !v.has_no_descend()).unwrap_or(true) {
            return None;
        }
    }
    let mut found: Option<Vec<OpId>> = None;
    for j in 1..o.num_input() {
        let arg = data.obank().get(op)?.get_in(j)?;
        let Some(inits) = cookie_cancel(arg, data, sp) else { continue };
        if found.is_some() {
            return None;
        }
        found = Some(inits);
    }
    found
}

/// The CALL op to strip, with the entry-side scramble ops feeding it.
fn call_has_locked_void_output(data: &Funcdata, op: OpId) -> bool {
    let Some(index) = data.get_call_specs_index(op) else { return false };
    let proto = data.get_call_specs(index).proto();
    proto.is_output_locked()
        && proto.get_output_type().map(|ty| ty.get_metatype())
            == Some(crate::dtype::type_metatype::TYPE_VOID)
}

fn cookie_check_calls(
    data: &Funcdata,
    sp: &(Rc<AddrSpace>, uintb, int4),
    require_locked_void: bool,
    skip_marked: bool,
) -> Vec<(OpId, Vec<OpId>)> {
    let mut found = Vec::new();
    for i in 0..data.num_calls() {
        let op = data.get_call_specs(i).get_op();
        if require_locked_void && !call_has_locked_void_output(data, op) {
            continue;
        }
        if skip_marked {
            let Some(callop) = data.obank().get(op) else { continue };
            let site = callop.get_addr();
            if data.get_override().is_msvc_cookie_call(site) {
                continue;
            }
        }
        if let Some(inits) = cookie_check_op(data, op, sp) {
            found.push((op, inits));
        }
    }
    found
}

/// (kuna) Strip the MSVC `/GS` frame-cookie check and its entry-side init
/// (option `msvcstackguard`, default off).
pub struct ActionStripMsvcStackGuard {
    base: ActionBase,
    /// Unit-test OR-override of the `strip_msvc_stack_guard` gate; the live gate
    /// rides the boundary Architecture, exactly as in `ActionStripStackGuard`.
    enabled: bool,
}

impl ActionStripMsvcStackGuard {
    /// Construct the action in the given group.
    pub fn new(enabled: bool, g: impl Into<String>) -> ActionStripMsvcStackGuard {
        ActionStripMsvcStackGuard { base: ActionBase::new(0, "stripmsvcstackguard", g), enabled }
    }
}

impl Action for ActionStripMsvcStackGuard {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionStripMsvcStackGuard {
            base: self.base.clone(),
            enabled: self.enabled,
        }))
    }

    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        let strip = self.enabled || data.get_arch().strip_msvc_stack_guard;
        // `calleeretpreserves` also consumes the exact cookie mark for a locked
        // void declaration. It preserves the caller's ABI return storage but
        // leaves the check and all cookie algebra intact.
        if !strip && !data.get_arch().callee_ret_preserves {
            return 0;
        }
        if data.num_calls() == 0 {
            return 0;
        }
        let Some(sp) = stack_pointer_storage(data) else { return 0 };
        let calls = cookie_check_calls(data, &sp, !strip, !strip);
        if !strip {
            if calls.is_empty() {
                return 0;
            }
            for (op, _) in calls {
                let site = data.obank().get(op).expect("cookie checker op").get_addr().clone();
                data.get_override_mut().insert_msvc_cookie_call(site);
            }
            data.set_restart_pending(true);
            return 0;
        }
        let Some((op, inits)) = calls.into_iter().next() else { return 0 };
        let site = data.obank().get(op).expect("cookie checker op").get_addr().clone();
        if !data.get_override().is_msvc_cookie_call(&site) {
            data.get_override_mut().insert_msvc_cookie_call(site);
            data.set_restart_pending(true);
            return 0;
        }
        if !strip {
            return 0;
        }
        let mut slots: Vec<(Address, int4)> = Vec::new();
        collect_cookie_slots(&inits, data, &mut slots);
        data.delete_call_specs(op);
        data.op_destroy(op);
        // The check is gone; release the entry-side scramble's frame slot so the
        // following ActionDeadCode collects the store, the cookie read and the
        // scramble itself instead of leaving a dead `v = cookie ^ SP`.
        release_canary_slots(data, &slots);
        self.base.count += 1;
        // One check per apply; the fullloop re-invokes and self-gates (this
        // call and its argument chain are gone on the next pass).
        1
    }
}

#[cfg(test)]
#[path = "kuna_msvcstackguard/tests.rs"]
mod tests;
