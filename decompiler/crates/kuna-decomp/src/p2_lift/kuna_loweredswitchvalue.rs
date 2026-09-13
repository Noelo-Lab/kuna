//! (kuna `loweredswitchvalue`) Prove that a re-rolled lowered switch dispatches
//! on the value its comparison cascade compared.
//!
//! [`crate::kuna_loweredswitch`] detects a cascade on the simplified SSA graph,
//! where every compare reads one value, and records that value's storage. The
//! install re-lifts raw p-code on the restart and has a new `BRANCHIND` read
//! storage at the cascade head, and that storage need not hold the value any
//! more: a parameter copied into another register before its own register is
//! reused as a call argument dispatches the switch on the argument.
//!
//! Detection names the compared value in a form that survives the restart
//! ([`ValueName`]). The install reads what the head's own compare instruction
//! reads ([`head_compare_operand`]) in preference to the recorded storage. On the
//! restarted run [`verify_installed_switch_values`] names the `BRANCHIND` input
//! the same way; a switch whose input is not the recorded value is installed once
//! more over the recorded storage and then withdrawn, and the cascade renders as
//! `option loweredswitch off` renders it.

use std::collections::BTreeSet;


use kuna_base::address::Address;
use kuna_base::marshal::ElementId;
use kuna_base::space::spacetype;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::p2_lift::kuna_loweredswitch::SharedLoweredSwitchStore;

/// Marshaling element `<loweredswitchvalue>` (kuna 4000+ range).
pub const ELEM_LOWEREDSWITCHVALUE: ElementId = ElementId::new("loweredswitchvalue", 4170);

/// How a named value is defined.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueDef {
    /// The value its storage holds on entry to the function.
    Input,
    /// The effect of the call or store at the address: its output or `INDIRECT`.
    Effect,
    /// The output of an op of this opcode.
    Op(OpCode),
}

/// A restart-stable name for an SSA value: how and where it is defined, and the
/// storage it is written to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueName {
    pub def: ValueDef,
    pub at: Address,
    pub space: int4,
    pub offset: uintb,
    pub size: int4,
    pub big_endian: bool,
}

impl ValueName {
    fn low_offset(&self, size: int4) -> uintb {
        if self.big_endian {
            self.offset.wrapping_add((self.size - size) as uintb)
        } else {
            self.offset
        }
    }
}

/// What a varnode is, as far as the value check can tell.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Observed {
    Constant,
    /// Every byte of the varnode is a low byte of the named value.
    Named(ValueName),
    /// Every byte of the varnode is a byte of this phi's output, which has no
    /// restart-stable name of its own.
    Merge(VarnodeId),
    Opaque,
}

/// Where a recorded switch stands.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ValueState {
    #[default]
    Unchecked,
    Verified,
    Withdrawn,
}

/// The value-check state a lowered-switch record carries across restarts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValueCheck {
    /// The restart-stable name of the value every compare in the cascade read,
    /// when it has one. `None` for a value SSA construction defines (a phi) or one
    /// with no unique definition: such a switch is installed best-effort, reading
    /// the head compare's operand, and never verified or withdrawn.
    pub name: Option<ValueName>,
    pub state: ValueState,
    /// The next install reads the recorded storage even when the head compare
    /// reads something else.
    pub read_home: bool,
    /// The current install reads what the head compare reads.
    pub read_head_operand: bool,
    /// Which read (`read_head_operand`) the verification was made over; an
    /// install that reads anything else is checked again.
    pub verified_read: Option<bool>,
}

impl ValueCheck {
    /// A fresh check for the cascade compared over `swvar`.
    pub fn for_value(data: &Funcdata, swvar: VarnodeId) -> ValueCheck {
        let size = data.vbank().get(swvar).map(|v| v.get_size()).unwrap_or(0);
        let name = match observe(data, swvar) {
            Observed::Named(n) if name_is_unique(data, &n, size) => Some(n),
            _ => None,
        };
        ValueCheck { name, ..ValueCheck::default() }
    }

    /// Whether the next install reads the recorded storage: after a fallback, and
    /// always for a record without a name, which nothing could verify.
    pub fn reads_home(&self) -> bool {
        self.read_home || self.name.is_none()
    }

    /// Whether the value check lets this record be installed: a switch is
    /// installed unless a verification has withdrawn it. A named switch that
    /// reads the head operand and fails verification retries over the home
    /// storage before it can be withdrawn, so the install itself is only ever
    /// blocked for a switch the restart proved dispatches on the wrong value.
    pub fn allows_install(&self) -> bool {
        self.state != ValueState::Withdrawn
    }
}

/// Name `vn` after peeling the copies, casts, extensions and low truncations
/// between it and its defining op.
pub(crate) fn observe(data: &Funcdata, vn: VarnodeId) -> Observed {
    let size = match data.vbank().get(vn) {
        Some(v) => v.get_size(),
        None => return Observed::Opaque,
    };
    let mut cur = vn;
    let mut exact = size;
    let mut steps = 0;
    loop {
        let v = match data.vbank().get(cur) {
            Some(v) => v,
            None => return Observed::Opaque,
        };
        if v.is_constant() {
            return Observed::Constant;
        }
        if !v.is_written() {
            break;
        }
        let op = match v.get_def().and_then(|d| data.obank().get(d)) {
            Some(op) => op,
            None => return Observed::Opaque,
        };
        let input = match op.get_in(0).and_then(|i| data.vbank().get(i).map(|iv| (i, iv.get_size()))) {
            Some(i) => i,
            None => break,
        };
        let zero = |slot: int4| {
            op.get_in(slot).and_then(|c| data.vbank().get(c)).is_some_and(|c| c.is_constant() && c.get_offset() == 0)
        };
        let next = match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST if input.1 == v.get_size() => input.0,
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR
                if op.num_input() == 2 && input.1 == v.get_size() && zero(1) =>
            {
                input.0
            }
            OpCode::CPUI_SUBPIECE
                if input.1 >= v.get_size()
                    && op
                        .get_in(1)
                        .and_then(|c| data.vbank().get(c))
                        .is_some_and(|c| c.is_constant() && c.get_offset() == 0) =>
            {
                input.0
            }
            OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT if input.1 < v.get_size() => {
                exact = exact.min(input.1);
                input.0
            }
            _ => break,
        };
        steps += 1;
        if steps > 16 {
            return Observed::Opaque;
        }
        cur = next;
    }
    let v = match data.vbank().get(cur) {
        Some(v) => v,
        None => return Observed::Opaque,
    };
    let space = match v.get_addr().get_space() {
        Some(s) if matches!(s.get_type(), spacetype::IPTR_PROCESSOR | spacetype::IPTR_SPACEBASE) => s,
        _ => return Observed::Opaque,
    };
    if exact < size {
        return Observed::Opaque;
    }
    let (def, at) = if v.is_input() {
        (ValueDef::Input, Address::new_invalid())
    } else if let Some(op) = v.get_def().and_then(|d| data.obank().get(d)) {
        let def = match op.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_INDIRECT => ValueDef::Effect,
            // A phi is placed by SSA construction, not by an instruction, so its
            // identity does not survive the restart's re-heritage: a value merged
            // from copies of a spilled argument is a MULTIEQUAL on the detected
            // graph and the raw argument read on the restart.  Leaving it unnamed
            // keeps such a switch (it reads the head operand and is never
            // withdrawn) rather than mistaking a re-heritaged read for a different
            // value.
            OpCode::CPUI_MULTIEQUAL => return Observed::Merge(cur),
            code => ValueDef::Op(code),
        };
        (def, op.get_addr().clone())
    } else {
        return Observed::Opaque;
    };
    Observed::Named(ValueName {
        def,
        at,
        space: space.get_index(),
        offset: v.get_offset(),
        size: v.get_size(),
        big_endian: space.is_big_endian(),
    })
}

/// True when the low `size` bytes of `a` and of `b` are one value.
pub(crate) fn same_low_bytes(a: &ValueName, b: &ValueName, size: int4) -> bool {
    a.def == b.def
        && a.at == b.at
        && a.space == b.space
        && a.big_endian == b.big_endian
        && size > 0
        && a.size >= size
        && b.size >= size
        && a.low_offset(size) == b.low_offset(size)
}

/// True when `name` is an input, or exactly one live op of its kind at its
/// address writes storage overlapping its low `size` bytes.
pub(crate) fn name_is_unique(data: &Funcdata, name: &ValueName, size: int4) -> bool {
    if name.def == ValueDef::Input {
        return true;
    }
    if size <= 0 || name.size < size {
        return false;
    }
    let lo = name.low_offset(size);
    let hi = lo.wrapping_add(size as uintb);
    let mut count = 0;
    for (_, id) in data.obank().iter_at(&name.at) {
        let op = match data.obank().get(id) {
            Some(op) if !op.is_dead() => op,
            _ => continue,
        };
        let kind = match op.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_INDIRECT => ValueDef::Effect,
            code => ValueDef::Op(code),
        };
        if kind != name.def {
            continue;
        }
        let out = match op.get_out().and_then(|o| data.vbank().get(o)) {
            Some(out) => out,
            None => continue,
        };
        if out.get_addr().get_space().map(|s| s.get_index()) != Some(name.space) {
            continue;
        }
        let olo = out.get_offset();
        let ohi = olo.wrapping_add(out.get_size() as uintb);
        if olo < hi && lo < ohi {
            count += 1;
        }
    }
    count == 1
}

/// How the `BRANCHIND` input of a switch over a `size`-byte value relates to the
/// value `recorded` names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum InputVerdict {
    /// The input is the recorded value.
    Matches,
    /// The input is a different stable value, or a constant: the switch
    /// dispatches on the wrong thing and must not be kept.
    Differs,
    /// The input cannot be named stably (a phi, or a value with no unique
    /// definition): it is not shown to differ, so the switch is left as it is.
    Unknown,
}

pub(crate) fn classify_input(data: &Funcdata, input: VarnodeId, recorded: &ValueName, size: int4) -> InputVerdict {
    if data.vbank().get(input).map(|v| v.get_size()) != Some(size) {
        return InputVerdict::Unknown;
    }
    let mut seen = BTreeSet::new();
    classify_value(data, input, recorded, size, &mut seen).unwrap_or(InputVerdict::Unknown)
}

/// Classify `vn` against `recorded`; a phi is classified through its inputs.
/// `None` is a phi already being classified on this path, which adds nothing.
fn classify_value(
    data: &Funcdata,
    vn: VarnodeId,
    recorded: &ValueName,
    size: int4,
    seen: &mut BTreeSet<VarnodeId>,
) -> Option<InputVerdict> {
    match observe(data, vn) {
        Observed::Named(n) if same_low_bytes(recorded, &n, size) && name_is_unique(data, &n, size) => {
            Some(InputVerdict::Matches)
        }
        // A different uniquely-defined value, or a folded constant, is a wrong
        // dispatch.  A non-unique name is not proof of difference.
        Observed::Named(n) if name_is_unique(data, &n, size) => Some(InputVerdict::Differs),
        Observed::Constant => Some(InputVerdict::Differs),
        // The switch reads whichever input arrives: one provably different input
        // is a wrong dispatch on that path, and only inputs that all match prove
        // the value.
        Observed::Merge(phi) => {
            if !seen.insert(phi) {
                return None;
            }
            if seen.len() > 64 {
                return Some(InputVerdict::Unknown);
            }
            let inputs: Vec<VarnodeId> = match data.vbank().get(phi).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d)) {
                Some(op) => (0..op.num_input()).filter_map(|i| op.get_in(i)).collect(),
                None => return Some(InputVerdict::Unknown),
            };
            let mut verdict = None;
            for i in inputs {
                match classify_value(data, i, recorded, size, seen) {
                    Some(InputVerdict::Differs) => return Some(InputVerdict::Differs),
                    Some(InputVerdict::Unknown) => verdict = Some(InputVerdict::Unknown),
                    Some(InputVerdict::Matches) if verdict.is_none() => verdict = Some(InputVerdict::Matches),
                    _ => {}
                }
            }
            verdict.or(Some(InputVerdict::Unknown))
        }
        _ => Some(InputVerdict::Unknown),
    }
}

/// True when `input` is shown to be exactly the value `recorded` names.
pub(crate) fn input_is_recorded_value(data: &Funcdata, input: VarnodeId, recorded: &ValueName, size: int4) -> bool {
    classify_input(data, input, recorded, size) == InputVerdict::Matches
}

/// What the synthesized `BRANCHIND` reads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HeadOperand {
    /// A fresh read of this storage at the end of the head block.
    Storage(Address),
    /// The output of a load the head compare reads.
    Value(VarnodeId),
}

/// The operand the compare at the end of `head` reads, from raw p-code.
///
/// Walks the condition of `cbranch` back through the head block, and through its
/// predecessor when that is its only one. Raw reads are not linked to their
/// writes, so a read is matched to the last op before it on that path writing
/// the same location. Flags, temporaries and one-byte locations are followed to
/// their writer, which must be integer or boolean arithmetic. The operand is a
/// `size`-byte location in `home`'s space that nothing from the reading op up to
/// the branch writes, or the output of a `size`-byte load; every path must reach
/// the same operand.
pub(crate) fn head_compare_operand(
    data: &Funcdata,
    head: BlockId,
    cbranch: OpId,
    home: &Address,
    size: int4,
) -> Option<HeadOperand> {
    let home_space = home.get_space()?.get_index();
    let mut ops = Vec::new();
    let block = data.bblocks_ref().block(head);
    if block.size_in() == 1 && block.get_in(0) != head {
        ops = data.bb_ops(block.get_in(0));
    }
    ops.extend(data.bb_ops(head));
    let cbpos = ops.iter().position(|&o| o == cbranch)?;
    let cond = data.obank().get(cbranch)?.get_in(1)?;
    let mut found: Option<HeadOperand> = None;
    let mut seen: BTreeSet<OpId> = BTreeSet::new();
    let mut work: Vec<(VarnodeId, usize)> = vec![(cond, cbpos)];
    while let Some((vn, pos)) = work.pop() {
        if seen.len() > 64 {
            return None;
        }
        let v = data.vbank().get(vn)?;
        if v.is_constant() {
            continue;
        }
        let (operand, writer) = if v.is_written() {
            let d = v.get_def()?;
            let dpos = ops[..pos].iter().position(|&o| o == d)?;
            if data.obank().get(d)?.code() == OpCode::CPUI_LOAD && v.get_size() == size {
                (Some(HeadOperand::Value(vn)), None)
            } else {
                (None, Some((d, dpos)))
            }
        } else {
            let space = v.get_addr().get_space()?.get_index();
            let (off, sz) = (v.get_offset(), v.get_size() as uintb);
            let overlap = |o: OpId| -> Option<bool> {
                let out = data.vbank().get(data.obank().get(o)?.get_out()?)?;
                if out.get_addr().get_space()?.get_index() != space {
                    return None;
                }
                let (ooff, osz) = (out.get_offset(), out.get_size() as uintb);
                if ooff >= off.wrapping_add(sz) || off >= ooff.wrapping_add(osz) {
                    return None;
                }
                Some(ooff == off && osz == sz)
            };
            let prior = ops[..pos].iter().enumerate().rev().find_map(|(p, &o)| overlap(o).map(|exact| (o, p, exact)));
            let candidate = space == home_space && v.get_size() == size;
            if let Some((o, p, _)) = prior {
                if v.get_size() > size {
                    if let Some(inner) = extended_low_bytes(data, o, space, off, v.get_size()) {
                        if seen.insert(o) {
                            work.push((inner, p));
                        }
                        continue;
                    }
                }
            }
            match prior {
                Some((o, _, true)) if data.obank().get(o)?.code() == OpCode::CPUI_LOAD => {
                    if v.get_size() != size {
                        return None;
                    }
                    (Some(HeadOperand::Value(data.obank().get(o)?.get_out()?)), None)
                }
                Some((o, p, true)) if flag_op(data.obank().get(o)?.code()) || !candidate || v.get_size() == 1 => {
                    (None, Some((o, p)))
                }
                _ if candidate => {
                    if ops[pos..cbpos].iter().any(|&o| overlap(o).is_some()) {
                        return None;
                    }
                    (Some(HeadOperand::Storage(Address::new(v.get_addr().get_space()?.clone(), off))), None)
                }
                _ => return None,
            }
        };
        if let Some(op) = operand {
            match &found {
                Some(f) if *f != op => return None,
                _ => found = Some(op),
            }
            continue;
        }
        let (d, dpos) = writer?;
        if !seen.insert(d) {
            continue;
        }
        let op = data.obank().get(d)?;
        if !slice_op(op.code()) {
            return None;
        }
        let out = data.vbank().get(op.get_out()?)?;
        let temp = out.get_addr().get_space().map(|s| s.get_type() == spacetype::IPTR_INTERNAL);
        if temp != Some(true) && out.get_size() != 1 {
            return None;
        }
        for slot in 0..op.num_input() {
            work.push((op.get_in(slot)?, dpos));
        }
    }
    found
}

/// The input of `op` when `op` zero- or sign-extends it into storage whose low
/// bytes are the `sz`-byte location at `off` in space `space`, so a compare of
/// that location compares the extended value.
fn extended_low_bytes(data: &Funcdata, op: OpId, space: int4, off: uintb, sz: int4) -> Option<VarnodeId> {
    let o = data.obank().get(op)?;
    if !matches!(o.code(), OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT) {
        return None;
    }
    let out = data.vbank().get(o.get_out()?)?;
    let out_space = out.get_addr().get_space()?;
    if out_space.get_index() != space {
        return None;
    }
    let (ooff, osz) = (out.get_offset(), out.get_size() as uintb);
    let low = if out_space.is_big_endian() { ooff.wrapping_add(osz).wrapping_sub(sz as uintb) } else { ooff };
    if off != low || osz < sz as uintb {
        return None;
    }
    let input = o.get_in(0)?;
    if data.vbank().get(input)?.get_size() > sz {
        return None;
    }
    Some(input)
}

fn flag_op(oc: OpCode) -> bool {
    matches!(
        oc,
        OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_CARRY
            | OpCode::CPUI_INT_SCARRY
            | OpCode::CPUI_INT_SBORROW
            | OpCode::CPUI_BOOL_NEGATE
            | OpCode::CPUI_BOOL_AND
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_BOOL_XOR
    )
}

fn slice_op(oc: OpCode) -> bool {
    flag_op(oc)
        || matches!(
            oc,
            OpCode::CPUI_COPY
                | OpCode::CPUI_INT_ADD
                | OpCode::CPUI_INT_SUB
                | OpCode::CPUI_INT_AND
                | OpCode::CPUI_INT_OR
                | OpCode::CPUI_INT_XOR
                | OpCode::CPUI_INT_NEGATE
                | OpCode::CPUI_INT_2COMP
                | OpCode::CPUI_INT_LEFT
                | OpCode::CPUI_INT_RIGHT
                | OpCode::CPUI_INT_SRIGHT
                | OpCode::CPUI_INT_ZEXT
                | OpCode::CPUI_INT_SEXT
                | OpCode::CPUI_SUBPIECE
                | OpCode::CPUI_POPCOUNT
        )
}

/// Check each installed lowered switch of `data` that carries a recorded name
/// against the value it named.
///
/// A switch dispatching on the recorded value is verified. Any other input, a
/// constant included, requests a restart: a switch that read what the head
/// compare reads is installed next over the recorded storage, and one that
/// already read the recorded storage is withdrawn and renders as
/// `option loweredswitch off` renders it. An unnamed switch (a phi-defined or
/// non-unique value) is left alone: it dispatches on the head operand best-effort
/// and is never withdrawn, exactly the guarantee `loweredswitch` gave before.
pub(crate) fn verify_installed_switch_values(data: &mut Funcdata, store: &SharedLoweredSwitchStore) {
    if !data.get_arch().lowered_switch_value_check {
        return;
    }
    let pending: Vec<(usize, Address, ValueName, int4)> = store
        .borrow()
        .records(data)
        .iter()
        .enumerate()
        .filter(|(_, r)| r.value.state != ValueState::Verified)
        .filter_map(|(i, r)| r.value.name.clone().map(|n| (i, r.branch_addr.clone(), n, r.var_size)))
        .collect();
    for (idx, branch_addr, name, size) in pending {
        let branchind = (0..data.num_jump_tables())
            .map(|i| data.get_jump_table(i))
            .filter(|jt| jt.kuna_lowered_var().is_some() && *jt.get_op_address() == branch_addr)
            .filter_map(|jt| jt.get_indirect_op())
            .find(|&op| data.obank().get(op).is_some_and(|o| o.code() == OpCode::CPUI_BRANCHIND));
        let branchind = match branchind {
            Some(op) => op,
            None => continue,
        };
        let input = data.obank().get(branchind).and_then(|o| o.get_in(0));
        let verdict = match input {
            Some(x) => classify_input(data, x, &name, size),
            None => InputVerdict::Differs,
        };
        let mut store = store.borrow_mut();
        let rec = match store.record_mut(data, idx) {
            Some(r) => r,
            None => continue,
        };
        match verdict {
            InputVerdict::Matches => {
                rec.value.state = ValueState::Verified;
                rec.value.verified_read = Some(rec.value.read_head_operand);
                rec.value.read_home = !rec.value.read_head_operand;
                continue;
            }
            // A phi or non-unique input is not proof of a wrong dispatch: leave
            // the switch as `loweredswitch` alone would, reading the head operand.
            InputVerdict::Unknown => continue,
            // A provably different value: fall back to the home storage (what
            // `loweredswitch` alone reads), and only when that also disagrees
            // withdraw the switch.  Detection can look again before the restart
            // it requested; a fallback already pending is not a second failure.
            InputVerdict::Differs => {
                if !rec.value.read_head_operand {
                    rec.value.state = ValueState::Withdrawn;
                } else if rec.value.read_home {
                    continue;
                } else {
                    rec.value.read_home = true;
                }
            }
        }
        drop(store);
        data.set_restart_pending(true);
    }
}

#[cfg(test)]
#[path = "kuna_loweredswitchvalue/tests.rs"]
mod tests;
