//! **ehcatch** — a C++ `catch` handler is part of the function that owns it.
//!
//! ```text
//!   work(a0);            try {
//!   return 0;              work(a0);
//!                   -->  }
//!                        catch (int) {
//!                          v1 = __eh_exception();
//!                          v2 = *(unsigned int *)__cxa_begin_catch(v1);
//!                          __cxa_end_catch();
//!                          return v2;
//!                        }
//!                        return 0;
//! ```
//!
//! # The defect
//!
//! A landing pad is reached only by the unwinder: no instruction branches to
//! it, so the flow walk never decodes it and the function prints as its normal
//! path alone. The handler body, and the fact that one exists, are lost.
//!
//! # The fix
//!
//! The analysis tier decodes each function's language-specific data area
//! (`kuna_analysis::entry::kuna_ehtable`) and parks one [`EhCallSite`] per
//! call-site record on the architecture. When the flow walk meets a CALL or
//! CALLIND inside such a record, [`plant`] adds the exceptional edge directly
//! after the call, at the call's own address:
//!
//! ```text
//!   CALL work
//!   u = CALLOTHER __eh_catch(#site)
//!   CBRANCH <stub>, u
//!   BRANCH <join>
//!   stub: r_exception = CALLOTHER __eh_exception()
//!         r_selector  = CALLOTHER __eh_selector()
//!         BRANCH <landing pad>
//!   join: (falls through to the next instruction)
//! ```
//!
//! The stub defines the two registers the unwinder loads before it enters the
//! pad, so the pad never reads the call's return value through them. A guarded
//! call that does not return (`__cxa_throw`, `__cxa_rethrow`) continues only at
//! its pad, so there the halt after the call is replaced by the stub alone and
//! the exception op names the caught types: `__eh_exception(T)`. The
//! printer renders an `if` on `__eh_catch` whose body is the handler as
//! `try { call; } catch (T) { ... }` ([`try_statement`]); any other shape keeps
//! the plain `if (__eh_catch(T))`.
//!
//! Only the unwinder enters a landing pad, so the flow also ends the path of
//! a call whose fall-through address is one: that call does not return.
//!
//! # The option
//!
//! `option ehcatch off|catch|all` ([`EhCatchMode`]). `catch`, the default, adds
//! the edge only for a call site whose action chain holds a catch clause.
//! `all` also follows cleanup-only pads (destructor calls that end in
//! `_Unwind_Resume`), printed as `catch (...)`. `off` adds no edge and the
//! function header counts the catch pads that are not shown.
//!
//! # What it will not do
//!
//! * **Only where the two unwinder registers are known**: x86, x86-64, ARM32
//!   and AArch64.
//! * **Only a call that ends its instruction** (or is followed by its own
//!   no-return halt), so a delay-slot call keeps its lifting.
//! * **Not inside an inlined callee body.**

use std::rc::Rc;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::userop::{BUILTIN_EH_CATCH, BUILTIN_EH_EXCEPTION, BUILTIN_EH_SELECTOR};
use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_num::opcodes::OpCode;
use kuna_base::types::int4;

/// The printed spelling of a catch-all clause, and of a cleanup-only pad.
pub const CATCH_ALL: &str = "...";

/// One call-site record of a function's exception table: a call in
/// `[start, end)` that throws lands at `landing_pad`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EhCallSite {
    /// First address of the guarded range.
    pub start: u64,
    /// One past the last address of the guarded range.
    pub end: u64,
    /// The handler's entry.
    pub landing_pad: u64,
    /// The caught types in action-chain order, [`CATCH_ALL`] for `catch (...)`;
    /// empty for a cleanup-only pad.
    pub catches: Vec<String>,
}

impl EhCallSite {
    /// The text between the parentheses of the `catch` clause.
    pub fn clause(&self) -> String {
        if self.catches.is_empty() {
            CATCH_ALL.to_string()
        } else {
            self.catches.join(" | ")
        }
    }
}

/// When a landing pad joins its function: `ehcatch off|catch|all`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EhCatchMode {
    /// No exceptional edge; the header counts the hidden catch pads.
    Off,
    /// Only call sites with a catch clause.
    #[default]
    Catch,
    /// Cleanup-only pads too.
    All,
}

impl EhCatchMode {
    /// The option token for this mode.
    pub fn as_str(self) -> &'static str {
        match self {
            EhCatchMode::Off => "off",
            EhCatchMode::Catch => "catch",
            EhCatchMode::All => "all",
        }
    }

    /// Parse + validate `option ehcatch off|catch|all`.
    pub fn apply(p1: &str) -> KunaResult<(EhCatchMode, String)> {
        let mode = match p1 {
            "off" => EhCatchMode::Off,
            "catch" => EhCatchMode::Catch,
            "all" => EhCatchMode::All,
            other => {
                return Err(KunaError::parse(format!(
                    "Unknown ehcatch value: {other} (expected off|catch|all)"
                )))
            }
        };
        Ok((mode, format!("Exception landing-pad flow set to {p1}")))
    }
}

/// The index of the record guarding `addr`; `sites` is sorted by `start` and
/// its ranges are disjoint.
pub fn site_at(sites: &[EhCallSite], addr: u64) -> Option<usize> {
    let i = sites.partition_point(|s| s.start <= addr).checked_sub(1)?;
    (addr < sites[i].end).then_some(i)
}

/// The registers the unwinder loads with the exception object and the handler
/// selector before it enters a landing pad (`__builtin_eh_return_data_regno`
/// 0 and 1), by language id; `None` where the pair is not known.
pub fn unwinder_registers(archid: &str) -> Option<(&'static [u8], &'static [u8])> {
    let mut fields = archid.split(':');
    match (fields.next()?, fields.nth(1)?) {
        ("x86", "64") => Some((b"RAX", b"RDX")),
        ("x86", "32") => Some((b"EAX", b"EDX")),
        ("ARM", "32") => Some((b"r0", b"r1")),
        ("AARCH64", "64") => Some((b"x0", b"x1")),
        _ => None,
    }
}

/// What the flow needs to add one exceptional edge.
#[derive(Clone, Debug)]
pub struct EhEdge {
    /// The handler's entry.
    pub pad: Address,
    /// The record's index in the architecture's site list.
    pub site: usize,
    /// Storage of the exception-object register.
    pub exception: (Address, int4),
    /// Storage of the handler-selector register.
    pub selector: (Address, int4),
}

/// The ops [`plant`] added that the flow walk must still mark.
pub struct EhStub {
    /// The BRANCH that skips the stub on the normal path.
    pub resume: OpId,
    /// The stub's first op.
    pub stub: OpId,
    /// The BRANCH to the landing pad.
    pub enter: OpId,
    /// The normal path's join op.
    pub join: OpId,
}

fn callother(data: &mut Funcdata, site: &Address, builtin: u32, arg: Option<u64>) -> KunaResult<OpId> {
    let op = data.new_op(if arg.is_some() { 2 } else { 1 }, site.clone());
    data.op_set_opcode_code(op, OpCode::CPUI_CALLOTHER);
    let index = data.new_constant(4, builtin as u64);
    data.op_set_input(op, index, 0)?;
    if let Some(arg) = arg {
        let vn = data.new_constant(4, arg);
        data.op_set_input(op, vn, 1)?;
    }
    Ok(op)
}

fn relative(data: &mut Funcdata, from: OpId, to: OpId) -> KunaResult<()> {
    let time = |data: &Funcdata, op| data.obank().get(op).expect("ehcatch: stale op").get_time();
    let delta = time(data, to).wrapping_sub(time(data, from));
    let target = data.new_constant(4, delta as u64);
    data.op_set_input(from, target, 0)
}

/// The three ops that enter the landing pad: the two unwinder registers and
/// the BRANCH. The exception op carries the record index when `named`.
fn pad_entry(data: &mut Funcdata, site: &Address, edge: &EhEdge, named: bool) -> KunaResult<[OpId; 3]> {
    let exception = callother(data, site, BUILTIN_EH_EXCEPTION, named.then_some(edge.site as u64))?;
    data.new_varnode_out(edge.exception.1, &edge.exception.0, exception)?;
    let selector = callother(data, site, BUILTIN_EH_SELECTOR, None)?;
    data.new_varnode_out(edge.selector.1, &edge.selector.0, selector)?;
    let enter = data.new_op(1, site.clone());
    data.op_set_opcode_code(enter, OpCode::CPUI_BRANCH);
    let pad = data.new_code_ref(&edge.pad);
    data.op_set_input(enter, pad, 0)?;
    Ok([exception, selector, enter])
}

fn insert_after(data: &mut Funcdata, mut prev: OpId, ops: &[OpId]) {
    for &op in ops {
        data.op_dead_insert_after(op, prev);
        prev = op;
    }
}

/// Add the exceptional edge after the returning call `call`: the test, the
/// stub, and a join op so the instruction still ends in a fall-through.
pub fn plant(data: &mut Funcdata, call: OpId, edge: &EhEdge) -> KunaResult<EhStub> {
    let site = data.obank().get(call).expect("ehcatch: stale call").get_addr().clone();
    let catch = callother(data, &site, BUILTIN_EH_CATCH, Some(edge.site as u64))?;
    let flag = data.new_unique_out(1, catch)?;
    let test = data.new_op(2, site.clone());
    data.op_set_opcode_code(test, OpCode::CPUI_CBRANCH);
    data.op_set_input(test, flag, 1)?;
    let resume = data.new_op(1, site.clone());
    data.op_set_opcode_code(resume, OpCode::CPUI_BRANCH);
    let [stub, selector, enter] = pad_entry(data, &site, edge, false)?;
    let join = data.new_op(1, site);
    data.op_set_opcode_code(join, OpCode::CPUI_COPY);
    let zero = data.new_constant(1, 0);
    data.op_set_input(join, zero, 0)?;
    data.new_unique_out(1, join)?;
    relative(data, test, stub)?;
    relative(data, resume, join)?;
    insert_after(data, call, &[catch, test, resume, stub, selector, enter, join]);
    Ok(EhStub { resume, stub, enter, join })
}

/// Replace the halt after the no-return call `call` with the edge to its
/// landing pad, the only place control continues; returns the BRANCH.
pub fn plant_no_return(data: &mut Funcdata, call: OpId, halt: OpId, edge: &EhEdge) -> KunaResult<OpId> {
    let site = data.obank().get(call).expect("ehcatch: stale call").get_addr().clone();
    let ops = pad_entry(data, &site, edge, true)?;
    data.op_destroy_raw(halt)?;
    insert_after(data, call, &ops);
    Ok(ops[2])
}

/// The `__eh_catch` op that defines `vn`, looking through boolean negations;
/// the flag is true when an odd number of negations lies between.
fn catch_source(fd: &Funcdata, mut vn: VarnodeId) -> Option<(OpId, bool)> {
    let mut negated = false;
    for _ in 0..4 {
        let def = fd.vbank().get(vn)?.get_def()?;
        let op = fd.obank().get(def)?;
        match op.code() {
            OpCode::CPUI_BOOL_NEGATE => {
                negated = !negated;
                vn = op.get_in(0)?;
            }
            OpCode::CPUI_CAST | OpCode::CPUI_COPY => vn = op.get_in(0)?,
            OpCode::CPUI_CALLOTHER => {
                let index = fd.vbank().get(op.get_in(0)?)?.get_offset();
                return (index == BUILTIN_EH_CATCH as u64).then_some((def, negated));
            }
            _ => return None,
        }
    }
    None
}

/// Is `op` one of the three `ehcatch` user-ops? Each only reads unwinder
/// state, so dead-code removal deletes it once its result is unused.
pub(crate) fn is_pure(data: &Funcdata, op: OpId) -> bool {
    let Some(o) = data.obank().get(op).filter(|o| o.code() == OpCode::CPUI_CALLOTHER) else {
        return false;
    };
    let index = o.get_in(0).and_then(|v| data.vbank().get(v)).map(|v| v.get_offset());
    [BUILTIN_EH_CATCH, BUILTIN_EH_EXCEPTION, BUILTIN_EH_SELECTOR].iter().any(|&b| index == Some(b as u64))
}

/// The result type of the `ehcatch` user-op `index`: a `bool` flag, a `void *`
/// exception object, a signed selector.
pub(crate) fn output_type(data: &Funcdata, index: u64, size: int4) -> Option<Rc<Datatype>> {
    let types = data.get_arch().types()?;
    match u32::try_from(index).ok()? {
        BUILTIN_EH_CATCH => types.get_base(size, type_metatype::TYPE_BOOL).ok(),
        BUILTIN_EH_SELECTOR => types.get_base(size, type_metatype::TYPE_INT).ok(),
        BUILTIN_EH_EXCEPTION => {
            let word = data.get_arch().manage().get_default_data_space()?.get_word_size();
            types.get_type_pointer(size, types.get_type_void().ok()?, word).ok()
        }
        _ => None,
    }
}

/// The record index a `__eh_catch` op, or the `__eh_exception` op of a
/// no-return call, carries.
pub fn catch_site(fd: &Funcdata, op: OpId) -> Option<usize> {
    let o = fd.obank().get(op)?;
    let vn = fd.vbank().get(o.get_in(1)?)?;
    vn.is_constant().then(|| vn.get_offset() as usize)
}

/// The successor of basic block `bb` on the exceptional edge, when `bb` ends in
/// the test of a guarded call.
pub(crate) fn catch_successor(fd: &Funcdata, bb: BlockId) -> Option<BlockId> {
    let o = fd.obank().get(fd.bb_op_tail(bb)?)?;
    if o.code() != OpCode::CPUI_CBRANCH {
        return None;
    }
    let (_, negated) = catch_source(fd, o.get_in(1)?)?;
    let block = fd.bblocks_ref().block(bb);
    (block.size_out() == 2).then(|| block.get_out(if negated == o.is_boolean_flip() { 1 } else { 0 }))
}

/// A conditional branch that prints as `try { call; } catch (clause)`.
pub struct TryStatement {
    /// The op whose statement holds the guarded call: the call, or the op that
    /// prints it as part of its own expression.
    pub statement: OpId,
    /// The record index.
    pub site: usize,
}

/// Is `cbranch`, the branch ending a condition block, taken into its `if` body
/// exactly when the guarded call throws? The call is the nearest CALL or
/// CALLIND before the `__eh_catch` op in the same block, and its statement
/// must print in that block too.
pub fn try_statement(fd: &Funcdata, cbranch: OpId) -> Option<TryStatement> {
    let o = fd.obank().get(cbranch)?;
    if o.code() != OpCode::CPUI_CBRANCH {
        return None;
    }
    let (catch, negated) = catch_source(fd, o.get_in(1)?)?;
    if negated != o.is_boolean_flip() {
        return None;
    }
    let site = catch_site(fd, catch)?;
    let parent = fd.obank().get(catch)?.get_parent()?;
    let mut call = None;
    let mut cur = fd.bb_op_head(parent);
    while let Some(op) = cur {
        if op == catch {
            break;
        }
        if matches!(fd.obank().get(op)?.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
            call = Some(op);
        }
        cur = fd.bb_op_next(op);
    }
    let call = call?;
    if fd.obank().get(call)?.get_addr() != fd.obank().get(catch)?.get_addr() {
        return None;
    }
    let mut statement = call;
    for _ in 0..32 {
        let Some(out) = fd.obank().get(statement)?.get_out() else {
            break;
        };
        if !fd.vbank().get(out)?.is_implied() {
            break;
        }
        statement = fd.lone_descend(out)?;
    }
    (fd.obank().get(statement)?.get_parent() == Some(parent)).then_some(TryStatement { statement, site })
}

#[cfg(test)]
#[path = "kuna_ehcatch/tests.rs"]
mod tests;
