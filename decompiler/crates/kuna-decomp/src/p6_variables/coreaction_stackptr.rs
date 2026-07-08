//! `ActionStackPtrFlow` — stack-pointer flow analysis (C++ `coreaction.cc:40-512`).
//!
//! This is the realized body of [`ActionStackPtrFlow`](crate::coreaction_render)
//! (its `apply` previously a `// STUB(W8-funcdata)` stub).  It is the third link
//! of the stack-variable promotion chain: after `Funcdata::spacebase` marks the
//! stack-pointer Varnodes (link 1) and `RuleLoad/StoreVarnode` rewrites
//! stack-relative LOAD/STORE into spacebase accesses (link 2), this pass repairs
//! the two remaining stack-pointer flow anomalies the spacebase fold can leave:
//!
//!   * **clogs** (`checkClog`/`repair`/`adjustLoad`/`isStackRelative`,
//!     `coreaction.cc:303-495`) — an `INT_ADD` of a value that was itself
//!     `LOAD`ed from a stack-relative pointer (`RSP = RSP + *(RSP+k)`).  When the
//!     matching `STORE` of a constant is found, the `LOAD` is turned into a
//!     `COPY` so the stack pointer stays a tractable affine expression.
//!   * **extra-pop** (`StackSolver` + `analyzeExtraPop`, `coreaction.cc:40-340`)
//!     — a full linear solve over every stack-pointer reference to recover the
//!     net stack adjustment across sub-function calls, then rewrite the
//!     unresolved `INDIRECT` (and spurious `INT_ADD`) producers to the solved
//!     `INT_ADD` constant.
//!
//! Faithfulness: every numeric is `u64`/`int4` per the C++ widths, wrapping ops
//! match the unsigned C++ arithmetic, and the magic constants (`65535` unknown
//! marker, the `4` extra-pop guess, the `-1` multiply test) are the verbatim C++
//! literals.  The two surfaces the merged tree does not yet build — the
//! per-call `FuncCallSpecs` (`getCallSpecs`/`setEffectiveExtraPop`, a W6/W7 seam,
//! `numCalls()==0` here) and the IOP-space `getOpFromConst` decode (W3-varnode
//! seam) — are reached exactly where the C++ reaches them: with no call specs
//! the INDIRECT companion takes the C++ `fc==0` *guess* branch (`rhs=4`), and the
//! `setEffectiveExtraPop` write lands on an absent call spec (a no-op).  The
//! solver's INT_ADD rewrite of the solution runs regardless, so the in-scope
//! repair is real; only the cross-call extra-pop *propagation into call specs*
//! is gated behind the proto-recovery seam (tracked as a loss).
//!
//! The `lower_bound(vnlist, othervn, comparePointers)` index search is replaced
//! by a linear `position()` over the loc-ordered `vnlist`: in the C++ it is a
//! binary search used only to find the index of a member Varnode (every
//! `othervn` here is guaranteed to be in `vnlist` — it carries the spacebase
//! address), so the index recovered is identical regardless of the ordering key.

use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uintb};

use kuna_num::opcodes::OpCode;

use crate::funcdata::Funcdata;
use crate::context::{OpId, VarnodeId};

#[cfg(test)]
mod tests;

/// The C++ `65535` sentinel for "no solution yet" in [`StackSolver::soln`].
const UNKNOWN_SOLN: int4 = 65535;

/// One equation `var1 - var2 = rhs` (C++ `struct StackEqn`, `coreaction.cc:40`).
#[derive(Clone, Copy)]
struct StackEqn {
    /// Variable with `+1` coefficient.
    var1: int4,
    /// Variable with `-1` coefficient.
    var2: int4,
    /// Right-hand side.
    rhs: int4,
}

/// Solve for stack-pointer changes across unknown sub-functions (C++
/// `class StackSolver`, `coreaction.cc:48`).
struct StackSolver {
    /// Known equations (C++ `vector<StackEqn> eqs`).
    eqs: Vec<StackEqn>,
    /// Guessed equations for underdetermined systems (C++ `guess`).
    guess: Vec<StackEqn>,
    /// The indexed set of variables — one per spacebase reference (C++ `vnlist`).
    vnlist: Vec<VarnodeId>,
    /// Index of the companion input for an INDIRECT-produced variable (C++
    /// `companion`).
    companion: Vec<int4>,
    /// Collected solutions, parallel to `vnlist` (C++ `soln`).
    soln: Vec<int4>,
    /// Number of variables missing an equation (C++ `missedvariables`).
    missedvariables: int4,
}

impl StackSolver {
    fn new() -> StackSolver {
        StackSolver {
            eqs: Vec::new(),
            guess: Vec::new(),
            vnlist: Vec::new(),
            companion: Vec::new(),
            soln: Vec::new(),
            missedvariables: 0,
        }
    }

    /// C++ `StackEqn::compare`: order by `var1` (used as the `lower_bound` key).
    fn compare(a: &StackEqn, b: &StackEqn) -> std::cmp::Ordering {
        a.var1.cmp(&b.var1)
    }

    /// C++ `StackSolver::propagate` (`coreaction.cc:82`): given a solution for
    /// one variable, walk the (sorted) equations propagating new solutions.
    fn propagate(&mut self, varnum: int4, val: int4) {
        if self.soln[varnum as usize] != UNKNOWN_SOLN {
            return; // This variable already specified
        }
        self.soln[varnum as usize] = val;

        let mut workstack: Vec<int4> = Vec::with_capacity(self.soln.len());
        workstack.push(varnum);

        while let Some(cur) = workstack.pop() {
            // lower_bound(eqs, {var1=cur}, compare) — first eqn with var1>=cur.
            let mut top = self
                .eqs
                .partition_point(|e| e.var1 < cur);
            while top < self.eqs.len() && self.eqs[top].var1 == cur {
                let var2 = self.eqs[top].var2;
                let rhs = self.eqs[top].rhs;
                if self.soln[var2 as usize] == UNKNOWN_SOLN {
                    self.soln[var2 as usize] = self.soln[cur as usize] - rhs;
                    workstack.push(var2);
                }
                top += 1;
            }
        }
    }

    /// C++ `StackSolver::duplicate` (`coreaction.cc:111`): mirror every equation
    /// (swap vars, negate rhs) and stable-sort by `var1`.
    fn duplicate(&mut self) {
        let size = self.eqs.len();
        for i in 0..size {
            let e = self.eqs[i];
            self.eqs.push(StackEqn { var1: e.var2, var2: e.var1, rhs: -e.rhs });
        }
        self.eqs.sort_by(Self::compare); // stable_sort
    }

    /// C++ `StackSolver::solve` (`coreaction.cc:127`).
    fn solve(&mut self) {
        self.soln.clear();
        self.soln.resize(self.vnlist.len(), UNKNOWN_SOLN);
        self.duplicate();

        self.propagate(0, 0); // We know one variable (the input)
        let size = self.guess.len();
        let mut lastcount = size as int4 + 2;
        loop {
            let mut count: int4 = 0;
            for i in 0..size {
                let var1 = self.guess[i].var1;
                let var2 = self.guess[i].var2;
                let rhs = self.guess[i].rhs;
                let s1 = self.soln[var1 as usize];
                let s2 = self.soln[var2 as usize];
                if s1 != UNKNOWN_SOLN && s2 == UNKNOWN_SOLN {
                    self.propagate(var2, s1 - rhs);
                } else if s1 == UNKNOWN_SOLN && s2 != UNKNOWN_SOLN {
                    self.propagate(var1, s2 + rhs);
                } else if s1 == UNKNOWN_SOLN && s2 == UNKNOWN_SOLN {
                    count += 1;
                }
            }
            if count == lastcount {
                break;
            }
            lastcount = count;
            if count <= 0 {
                break;
            }
        }
    }

    /// Find the index of `othervn` within `vnlist` (C++
    /// `lower_bound(vnlist,othervn,comparePointers)`).  Linear member search —
    /// see the module note.
    fn index_of(&self, othervn: VarnodeId) -> Option<int4> {
        self.vnlist.iter().position(|&v| v == othervn).map(|i| i as int4)
    }

    /// C++ `StackSolver::build` (`coreaction.cc:162`): collect spacebase
    /// references as variables and derive the equations from their defs.
    fn build(&mut self, data: &Funcdata, id: &Rc<AddrSpace>, spcbase: int4) -> Result<(), ()> {
        let point = id.get_spacebase(spcbase).map_err(|_| ())?;
        let point_space = point.space.clone().ok_or(())?;
        let spacebase = Address::new(point_space, point.offset);
        let point_size = point.size as int4;

        let refs: Vec<VarnodeId> =
            data.vbank().iter_loc_size_addr(point_size, &spacebase).collect();
        for vn in refs {
            // C++ breaks at the first free spacebase reference.
            if data.vbank().get(vn).map(|v| v.is_free()).unwrap_or(true) {
                break;
            }
            self.vnlist.push(vn);
            self.companion.push(-1);
        }
        self.missedvariables = 0;
        if self.vnlist.is_empty() {
            return Ok(());
        }
        if !data.vbank().get(self.vnlist[0]).map(|v| v.is_input()).unwrap_or(false) {
            // C++ throw LowlevelError("Input value of stackpointer is not used")
            return Err(());
        }

        for i in 1..self.vnlist.len() {
            let vn = self.vnlist[i];
            let op = match data.vbank().get(vn).and_then(|v| v.get_def()) {
                Some(o) => o,
                None => {
                    self.missedvariables += 1;
                    continue;
                }
            };
            let opc = match data.obank().get(op) {
                Some(o) => o.code(),
                None => {
                    self.missedvariables += 1;
                    continue;
                }
            };
            match opc {
                OpCode::CPUI_INT_ADD => {
                    let mut othervn = data.obank().get(op).and_then(|o| o.get_in(0));
                    let mut constvn = data.obank().get(op).and_then(|o| o.get_in(1));
                    if Self::is_const(data, othervn) {
                        constvn = othervn;
                        othervn = data.obank().get(op).and_then(|o| o.get_in(1));
                    }
                    if !Self::is_const(data, constvn) {
                        self.missedvariables += 1;
                        continue;
                    }
                    if !Self::addr_is(data, othervn, &spacebase) {
                        self.missedvariables += 1;
                        continue;
                    }
                    let idx = match othervn.and_then(|o| self.index_of(o)) {
                        Some(x) => x,
                        None => {
                            self.missedvariables += 1;
                            continue;
                        }
                    };
                    let rhs = Self::const_off(data, constvn) as int4;
                    self.eqs.push(StackEqn { var1: i as int4, var2: idx, rhs });
                }
                OpCode::CPUI_COPY => {
                    let othervn = data.obank().get(op).and_then(|o| o.get_in(0));
                    if !Self::addr_is(data, othervn, &spacebase) {
                        self.missedvariables += 1;
                        continue;
                    }
                    let idx = match othervn.and_then(|o| self.index_of(o)) {
                        Some(x) => x,
                        None => {
                            self.missedvariables += 1;
                            continue;
                        }
                    };
                    self.eqs.push(StackEqn { var1: i as int4, var2: idx, rhs: 0 });
                }
                OpCode::CPUI_INDIRECT => {
                    let othervn = data.obank().get(op).and_then(|o| o.get_in(0));
                    if !Self::addr_is(data, othervn, &spacebase) {
                        self.missedvariables += 1;
                        continue;
                    }
                    let idx = match othervn.and_then(|o| self.index_of(o)) {
                        Some(x) => x,
                        None => {
                            self.missedvariables += 1;
                            continue;
                        }
                    };
                    self.companion[i] = idx;
                    // C++ coreaction.cc:221-235 — if the INDIRECT is due to a call
                    // (its in[1] is an IOP-space annotation) AND the call spec has a
                    // KNOWN extrapop (set by L0/L1's setEffectiveExtraPop in the
                    // de-indirect process), push the EXACT equation (rhs = extrapop)
                    // instead of the rhs=4 guess.  In the merged tree numCalls()==0
                    // so this falls through to the guess; in the `jumptable` clone
                    // the call specs ARE present, so this builds the exact INDIRECT
                    // equation for the switch index (the L1 net-safety coupling).
                    use crate::fspec::EXTRAPOP_UNKNOWN;
                    let iopvn = data.obank().get(op).and_then(|o| o.get_in(1));
                    let is_iop = iopvn
                        .and_then(|v| data.vbank().get(v))
                        .and_then(|v| v.get_addr().get_space())
                        .map(|s| s.get_type() == kuna_base::space::spacetype::IPTR_IOP)
                        .unwrap_or(false);
                    if is_iop {
                        // PcodeOp *iop = getOpFromConst(iopvn->getAddr());
                        let off = iopvn
                            .and_then(|v| data.vbank().get(v))
                            .map(|v| v.get_addr().get_offset());
                        if let Some(off) = off {
                            let iop = crate::funcdata_varnode::op_iop_decode(off);
                            // FuncCallSpecs *fc = data.getCallSpecs(iop);
                            if let Some(fc_idx) = data.get_call_specs_index(iop) {
                                let extrapop = data.get_call_specs(fc_idx).get_extra_pop();
                                if extrapop != EXTRAPOP_UNKNOWN {
                                    // eqn.rhs = fc->getExtraPop(); eqs.push_back(eqn);
                                    self.eqs.push(StackEqn {
                                        var1: i as int4,
                                        var2: idx,
                                        rhs: extrapop,
                                    });
                                    continue;
                                }
                            }
                        }
                    }
                    // eqn.rhs = 4; guess.push_back(eqn);  // Otherwise make a guess
                    self.guess.push(StackEqn { var1: i as int4, var2: idx, rhs: 4 });
                }
                OpCode::CPUI_MULTIEQUAL => {
                    let n = data.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
                    for j in 0..n {
                        let othervn = data.obank().get(op).and_then(|o| o.get_in(j));
                        if !Self::addr_is(data, othervn, &spacebase) {
                            self.missedvariables += 1;
                            continue;
                        }
                        let idx = match othervn.and_then(|o| self.index_of(o)) {
                            Some(x) => x,
                            None => {
                                self.missedvariables += 1;
                                continue;
                            }
                        };
                        self.eqs.push(StackEqn { var1: i as int4, var2: idx, rhs: 0 });
                    }
                }
                OpCode::CPUI_INT_AND => {
                    // Stack-pointer alignment — treat as a copy.
                    let mut othervn = data.obank().get(op).and_then(|o| o.get_in(0));
                    let mut constvn = data.obank().get(op).and_then(|o| o.get_in(1));
                    if Self::is_const(data, othervn) {
                        constvn = othervn;
                        othervn = data.obank().get(op).and_then(|o| o.get_in(1));
                    }
                    if !Self::is_const(data, constvn) {
                        self.missedvariables += 1;
                        continue;
                    }
                    if !Self::addr_is(data, othervn, &spacebase) {
                        self.missedvariables += 1;
                        continue;
                    }
                    let idx = match othervn.and_then(|o| self.index_of(o)) {
                        Some(x) => x,
                        None => {
                            self.missedvariables += 1;
                            continue;
                        }
                    };
                    self.eqs.push(StackEqn { var1: i as int4, var2: idx, rhs: 0 });
                }
                _ => {
                    self.missedvariables += 1;
                }
            }
        }
        Ok(())
    }

    fn is_const(data: &Funcdata, vn: Option<VarnodeId>) -> bool {
        vn.and_then(|v| data.vbank().get(v).map(|x| x.is_constant())).unwrap_or(false)
    }

    fn const_off(data: &Funcdata, vn: Option<VarnodeId>) -> uintb {
        vn.and_then(|v| data.vbank().get(v).map(|x| x.get_offset())).unwrap_or(0)
    }

    fn addr_is(data: &Funcdata, vn: Option<VarnodeId>, addr: &Address) -> bool {
        vn.and_then(|v| data.vbank().get(v).map(|x| addr_eq(x.get_addr(), addr)))
            .unwrap_or(false)
    }
}

/// Order-by-value address comparison (C++ `getAddr() != spacebase`).
fn addr_eq(a: &Address, b: &Address) -> bool {
    let sa = a.get_space().map(|s| s.get_index());
    let sb = b.get_space().map(|s| s.get_index());
    sa == sb && a.get_offset() == b.get_offset()
}

/// C++ `ActionStackPtrFlow::isStackRelative` (`coreaction.cc:344`): is `vn`
/// defined as `spcbasein + const` (or `== spcbasein`)?  Returns the constant.
fn is_stack_relative(
    data: &Funcdata,
    spcbasein: VarnodeId,
    vn: VarnodeId,
) -> Option<uintb> {
    if spcbasein == vn {
        return Some(0);
    }
    let v = data.vbank().get(vn)?;
    if !v.is_written() {
        return None;
    }
    let addop = v.get_def()?;
    let o = data.obank().get(addop)?;
    if o.code() != OpCode::CPUI_INT_ADD {
        return None;
    }
    if o.get_in(0) != Some(spcbasein) {
        return None;
    }
    let constvn = o.get_in(1)?;
    let cv = data.vbank().get(constvn)?;
    if !cv.is_constant() {
        return None;
    }
    Some(cv.get_offset())
}

/// C++ `ActionStackPtrFlow::adjustLoad` (`coreaction.cc:368`): convert the
/// matched `LOAD` into a `COPY` of the `STORE`d value.
fn adjust_load(data: &mut Funcdata, loadop: OpId, storeop: OpId) -> bool {
    let storedvn = match data.obank().get(storeop).and_then(|o| o.get_in(2)) {
        Some(v) => v,
        None => return false,
    };
    // vn = storeop->getIn(2); if const make a fresh const; if free bail.
    let vn = {
        let sv = match data.vbank().get(storedvn) {
            Some(v) => v,
            None => return false,
        };
        if sv.is_constant() {
            let sz = sv.get_size();
            let off = sv.get_offset();
            data.new_constant(sz, off)
        } else if sv.is_free() {
            return false;
        } else {
            storedvn
        }
    };
    data.op_remove_input(loadop, 1);
    data.op_set_opcode_code(loadop, OpCode::CPUI_COPY);
    if data.op_set_input(loadop, vn, 0).is_err() {
        return false;
    }
    true
}

/// C++ `ActionStackPtrFlow::repair` (`coreaction.cc:393`): trace back from a
/// stack-relative `LOAD` to a matching constant `STORE`; on a match convert the
/// `LOAD` to a `COPY`.  Returns 1 on a successful repair, else 0.
fn repair(
    data: &mut Funcdata,
    id: &Rc<AddrSpace>,
    spcbasein: VarnodeId,
    loadop: OpId,
    constz: uintb,
) -> int4 {
    let loadsize = match data.obank().get(loadop).and_then(|o| o.get_out()) {
        Some(ov) => data.vbank().get(ov).map(|v| v.get_size()).unwrap_or(0),
        None => return 0,
    };
    let mut curblock = match data.obank().get(loadop).and_then(|o| o.get_parent()) {
        Some(b) => b,
        None => return 0,
    };
    // Walk the basic-block op list backwards starting from loadop.
    let mut iter = loadop;
    loop {
        // C++: if (iter == begiter) { cross single-pred edge or stop }
        let begiter = data.bb_op_head(curblock);
        let at_begin = begiter == Some(iter);
        if at_begin {
            if data.bblocks_ref().block(curblock).size_in() != 1 {
                return 0; // can only trace back through a unique predecessor
            }
            curblock = data.bblocks_ref().block(curblock).get_in(0);
            iter = match data.bb_op_tail(curblock) {
                Some(t) => t,
                None => {
                    // Empty predecessor — its head==None; C++ would set iter to
                    // endOp() and immediately --, but with no ops we cannot
                    // proceed; treat as unsolvable.
                    return 0;
                }
            };
            // After crossing, `iter` points at the predecessor's LAST op; the
            // C++ sets begiter=beginOp() and iter=endOp() then loops to --iter.
            // We are already at the last op, so fall through to inspect it.
        } else {
            // --iter (previous op in the same block)
            iter = match data.obank().get(iter).and_then(|o| o.basic_neighbours().0) {
                Some(p) => p,
                None => return 0,
            };
        }
        let curop = iter;
        let (is_call, code) = match data.obank().get(curop) {
            Some(o) => (o.is_call(), o.code()),
            None => return 0,
        };
        if is_call {
            return 0; // don't trace aliasing through a call
        }
        if code == OpCode::CPUI_STORE {
            let ptrvn = match data.obank().get(curop).and_then(|o| o.get_in(1)) {
                Some(v) => v,
                None => return 0,
            };
            let datavn = match data.obank().get(curop).and_then(|o| o.get_in(2)) {
                Some(v) => v,
                None => return 0,
            };
            match is_stack_relative(data, spcbasein, ptrvn) {
                Some(constnew) => {
                    let datasize =
                        data.vbank().get(datavn).map(|v| v.get_size()).unwrap_or(0);
                    if constnew == constz && loadsize == datasize {
                        // matching store
                        if adjust_load(data, loadop, curop) {
                            return 1;
                        }
                        return 0;
                    } else if constnew <= constz.wrapping_add((loadsize - 1) as uintb)
                        && constnew.wrapping_add((datasize - 1) as uintb) >= constz
                    {
                        return 0; // overlapping store, can't resolve aliasing
                    }
                    // else: disjoint stack store — keep tracing.
                }
                None => return 0, // any other STORE: can't solve aliasing
            }
        } else {
            // If some op writes into the stack space, the stack is already
            // traced past here — too late.
            if let Some(outvn) = data.obank().get(curop).and_then(|o| o.get_out()) {
                let in_stack = data
                    .vbank()
                    .get(outvn)
                    .and_then(|v| v.get_addr().get_space().map(|s| s.get_index()))
                    == Some(id.get_index());
                if in_stack {
                    return 0;
                }
            }
        }
    }
}

/// C++ `ActionStackPtrFlow::checkClog` (`coreaction.cc:447`): find stack-pointer
/// clogs (an `INT_ADD` whose addend was `LOAD`ed off the stack) and repair them.
/// Returns the number of clogs repaired.
pub(crate) fn check_clog(data: &mut Funcdata, id: &Rc<AddrSpace>, spcbase: int4) -> int4 {
    let point = match id.get_spacebase(spcbase) {
        Ok(p) => p,
        Err(_) => return 0,
    };
    let point_space = match point.space.clone() {
        Some(s) => s,
        None => return 0,
    };
    let spacebase = Address::new(point_space, point.offset);
    let point_size = point.size as int4;
    let mut clogcount: int4 = 0;

    let refs: Vec<VarnodeId> =
        data.vbank().iter_loc_size_addr(point_size, &spacebase).collect();
    if refs.is_empty() {
        return 0;
    }
    let spcbasein = refs[0];
    if !data.vbank().get(spcbasein).map(|v| v.is_input()).unwrap_or(false) {
        return 0;
    }
    for &outvn in &refs[1..] {
        let addop = match data.vbank().get(outvn) {
            Some(v) if v.is_written() => match v.get_def() {
                Some(o) => o,
                None => continue,
            },
            _ => continue,
        };
        if data.obank().get(addop).map(|o| o.code()) != Some(OpCode::CPUI_INT_ADD) {
            continue;
        }
        let mut y = match data.obank().get(addop).and_then(|o| o.get_in(1)) {
            Some(v) => v,
            None => continue,
        };
        // y must be written (C++ `if (!y->isWritten()) continue;` — a constant
        // is not written; so is an input, and the C++ skips both here).
        if !data.vbank().get(y).map(|v| v.is_written()).unwrap_or(false) {
            continue;
        }
        let mut x = match data.obank().get(addop).and_then(|o| o.get_in(0)) {
            Some(v) => v,
            None => continue,
        };
        // If x is not stack relative, swap x and y and retry.
        if is_stack_relative(data, spcbasein, x).is_none() {
            x = y;
            y = match data.obank().get(addop).and_then(|o| o.get_in(0)) {
                Some(v) => v,
                None => continue,
            };
            if is_stack_relative(data, spcbasein, x).is_none() {
                continue;
            }
        }
        // loadop = y->getDef()
        let mut loadop = match data.vbank().get(y).and_then(|v| v.get_def()) {
            Some(o) => o,
            None => continue,
        };
        if data.obank().get(loadop).map(|o| o.code()) == Some(OpCode::CPUI_INT_MULT) {
            // multiply-by-(-1) unwrap.
            let constvn = match data.obank().get(loadop).and_then(|o| o.get_in(1)) {
                Some(v) => v,
                None => continue,
            };
            let cv = match data.vbank().get(constvn) {
                Some(v) if v.is_constant() => v,
                _ => continue,
            };
            let csz = cv.get_size();
            if cv.get_offset() != calc_mask(csz) {
                continue; // must multiply by -1
            }
            y = match data.obank().get(loadop).and_then(|o| o.get_in(0)) {
                Some(v) => v,
                None => continue,
            };
            if !data.vbank().get(y).map(|v| v.is_written()).unwrap_or(false) {
                continue;
            }
            loadop = match data.vbank().get(y).and_then(|v| v.get_def()) {
                Some(o) => o,
                None => continue,
            };
        }
        if data.obank().get(loadop).map(|o| o.code()) != Some(OpCode::CPUI_LOAD) {
            continue;
        }
        let ptrvn = match data.obank().get(loadop).and_then(|o| o.get_in(1)) {
            Some(v) => v,
            None => continue,
        };
        let constz = match is_stack_relative(data, spcbasein, ptrvn) {
            Some(c) => c,
            None => continue,
        };
        clogcount += repair(data, id, spcbasein, loadop, constz);
    }
    clogcount
}

/// C++ `ActionStackPtrFlow::analyzeExtraPop` (`coreaction.cc:276`): full linear
/// solve to recover stack-pointer change across undetermined sub-functions, then
/// rewrite the solved producers to `INT_ADD const`.  Returns the number of
/// producers rewritten (so the caller can signal `count`).
pub(crate) fn analyze_extra_pop(
    data: &mut Funcdata,
    stackspace: &Rc<AddrSpace>,
    spcbase: int4,
) -> int4 {
    use crate::fspec::EXTRAPOP_UNKNOWN;

    // C++ `ActionStackPtrFlow::analyzeExtraPop` (coreaction.cc:279-282):
    //   ProtoModel *myfp = data.getArch()->evalfp_called;
    //   if (myfp == 0) myfp = data.getArch()->defaultfp;
    //   if (myfp->getExtraPop() != extrapop_unknown) return;
    // The `evalfp_called ?: defaultfp` fallback is load-bearing: when no
    // evaluation model is set, the C++ reads `defaultfp`'s extrapop, so a known
    // default extrapop suppresses the solve (matching the byte oracle).
    let myfp_extrapop = data
        .get_arch()
        .eval_fp_called()
        .or_else(|| data.get_arch().default_fp())
        .map(|m| m.get_extra_pop());
    if let Some(ep) = myfp_extrapop {
        if ep != EXTRAPOP_UNKNOWN {
            return 0;
        }
    }

    let mut solver = StackSolver::new();
    // C++ wraps build() in try/catch and warns (`warningHeader`) on a malformed
    // stack frame; an Err here mirrors that LowlevelError -> early return.  The
    // cosmetic warning text is routed through the action context elsewhere and is
    // not load-bearing for the analysis result, so it is omitted here.
    if solver.build(data, stackspace, spcbase).is_err() {
        return 0;
    }
    if solver.vnlist.is_empty() {
        return 0;
    }
    solver.solve();

    let invn = solver.vnlist[0];
    let mut changed: int4 = 0;
    for i in 1..solver.vnlist.len() {
        let vn = solver.vnlist[i];
        let soln = solver.soln[i];
        if soln == UNKNOWN_SOLN {
            // C++ warns "Unable to track spacebase fully" once here; omitted
            // (cosmetic, see above) — the variable is simply left unsolved.
            continue;
        }
        let op = match data.vbank().get(vn).and_then(|v| v.get_def()) {
            Some(o) => o,
            None => continue,
        };
        // L1 (RSP keystone) — transcription of coreaction.cc:303-320.  When the
        // solved producer is the per-call INDIRECT (the spacebase op L0 inserts at
        // an `extrapop_unknown` call), propagate the recovered net stack change
        // (soln - companionSoln) back onto the call spec via setEffectiveExtraPop.
        // This is THE switchind gate (LOSS-148): the `jumptable` ActionDatabase
        // group runs ActionStackPtrFlow over the partial clone (which DOES carry
        // call specs), and StackSolver::build's INDIRECT-companion equation
        // (coreaction.cc:226-228) reads getExtraPop() to build the exact equation
        // for the switch index — so L0's INT_ADD/INDIRECT is cleanly solved
        // instead of leaking stack-relative residue into the switch index.
        if data.obank().get(op).map(|o| o.code()) == Some(OpCode::CPUI_INDIRECT) {
            // Varnode *iopvn = op->getIn(1);
            let iopvn = data.obank().get(op).and_then(|o| o.get_in(1));
            // if (iopvn->getSpace()->getType()==IPTR_IOP) {
            let is_iop = iopvn
                .and_then(|v| data.vbank().get(v))
                .and_then(|v| v.get_addr().get_space())
                .map(|s| s.get_type() == kuna_base::space::spacetype::IPTR_IOP)
                .unwrap_or(false);
            if is_iop {
                // PcodeOp *iop = PcodeOp::getOpFromConst(iopvn->getAddr());
                let iop_off = iopvn
                    .and_then(|v| data.vbank().get(v))
                    .map(|v| v.get_addr().get_offset());
                if let Some(off) = iop_off {
                    let iop = crate::funcdata_varnode::op_iop_decode(off);
                    // FuncCallSpecs *fc = data.getCallSpecs(iop);
                    // if (fc != 0) {
                    if let Some(fc_idx) = data.get_call_specs_index(iop) {
                        // int4 soln2 = 0;
                        // int4 comp = solver.getCompanion(i);
                        // if (comp >= 0) soln2 = solver.getSolution(comp);
                        let comp = solver.companion[i];
                        let soln2 = if comp >= 0 { solver.soln[comp as usize] } else { 0 };
                        // fc->setEffectiveExtraPop(soln-soln2);
                        data.get_call_specs_mut(fc_idx).set_effective_extra_pop(soln - soln2);
                    }
                }
            }
        }

        // paramlist = { invn, newConstant(sz, soln & calc_mask(sz)) };
        // opSetOpcode(op, INT_ADD); opSetAllInput(op, paramlist).
        let sz = data.vbank().get(invn).map(|v| v.get_size()).unwrap_or(0);
        let cmask = calc_mask(sz);
        let cval = (soln as uintb) & cmask;
        let constvn = data.new_constant(sz, cval);
        data.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
        if data.op_set_all_input(op, &[invn, constvn]).is_ok() {
            changed += 1;
        }
    }
    changed
}
