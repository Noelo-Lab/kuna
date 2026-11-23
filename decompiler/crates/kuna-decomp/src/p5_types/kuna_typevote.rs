//! (kuna `typevote`) A variable the type passes leave `undefinedN` takes the
//! type its uses vote for.
//!
//! Type propagation leaves a variable `undefinedN` when no typed operation
//! reaches it, and the merge passes add copies after type inference that
//! start untyped, so `undefined4 v4;` printed even where `v4` is shifted into
//! an instruction word. [`ActionTypeVote`] runs once every variable is final,
//! before casts. Every op that writes or reads an instance of a variable
//! whose type is `undefinedN` votes: signed compares, divides, remainders,
//! arithmetic shifts, sign extensions and signed overflow vote signed; the
//! unsigned forms and zero extension vote unsigned; a copy from or to an
//! integer-typed value votes that value's signedness; other integer
//! arithmetic, bitwise operations, shifts and equality tests vote "numeric",
//! which only weighs against float; float operations vote float. Signed or
//! unsigned votes give `int`/`uint` of the variable's size (signed on a tie);
//! float votes at 4 or 8 bytes give `float`/`double`. A variable with no
//! signed, unsigned or winning float vote stays undefined, so arithmetic that
//! does not say a signedness claims none.
//!
//! The prototype is not changed: a value that reaches a RETURN is skipped,
//! and a parameter takes only an unsigned vote, since an undefined parameter
//! already prints as unsigned in the signature. Type-locked variables are
//! skipped. A piece of a union also prints as `undefinedN`, but its type
//! carries the union's field reads and is kept.

use std::rc::Rc;

use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

#[derive(Default)]
struct Votes {
    signed: i32,
    unsigned: i32,
    numeric: i32,
    float: i32,
}

impl Votes {
    fn integer(&mut self, ct: &Datatype, weight: i32) {
        match ct.get_metatype() {
            type_metatype::TYPE_INT => self.signed += weight,
            type_metatype::TYPE_UINT => self.unsigned += weight,
            _ => {}
        }
    }

    /// The vote of `op` on its output (`slot == None`) or input `slot`.
    fn op(&mut self, data: &Funcdata, op: OpId, slot: Option<i32>) {
        use OpCode::*;
        let Some(o) = data.obank().get(op) else { return };
        match o.code() {
            CPUI_INT_SLESS | CPUI_INT_SLESSEQUAL | CPUI_INT_SDIV | CPUI_INT_SREM | CPUI_INT_SCARRY | CPUI_INT_SBORROW => self.signed += 2,
            CPUI_INT_SEXT if slot == Some(0) => self.signed += 2,
            CPUI_INT_SRIGHT if slot != Some(1) => self.signed += 2,
            CPUI_INT_LESS | CPUI_INT_LESSEQUAL | CPUI_INT_DIV | CPUI_INT_REM | CPUI_INT_CARRY => self.unsigned += 2,
            CPUI_INT_ZEXT if slot == Some(0) => self.unsigned += 1,
            CPUI_INT_RIGHT if slot != Some(1) => self.unsigned += 1,
            CPUI_INT_ADD | CPUI_INT_SUB | CPUI_INT_MULT | CPUI_INT_2COMP | CPUI_INT_LEFT | CPUI_INT_AND | CPUI_INT_OR
            | CPUI_INT_XOR | CPUI_INT_NEGATE | CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL | CPUI_INT_ZEXT | CPUI_INT_SEXT
            | CPUI_INT_RIGHT | CPUI_INT_SRIGHT => self.numeric += 1,
            CPUI_FLOAT_ADD | CPUI_FLOAT_SUB | CPUI_FLOAT_MULT | CPUI_FLOAT_DIV | CPUI_FLOAT_LESS | CPUI_FLOAT_LESSEQUAL
            | CPUI_FLOAT_EQUAL | CPUI_FLOAT_NOTEQUAL | CPUI_FLOAT_NEG | CPUI_FLOAT_ABS | CPUI_FLOAT_SQRT
            | CPUI_FLOAT_FLOAT2FLOAT | CPUI_FLOAT_TRUNC => self.float += 2,
            CPUI_COPY => {
                let other = if slot.is_none() { o.get_in(0) } else { o.get_out() };
                if let Some(t) = other.and_then(|v| data.vbank().get(v)) {
                    self.integer(t.get_type(), 3);
                }
            }
            _ => {}
        }
    }
}

/// Does `vn` flow into a RETURN, directly or through copies, joins into a
/// register pair, extensions and truncations?
fn reaches_return(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    data.descend_snapshot(vn).into_iter().any(|d| match data.obank().get(d).map(|o| (o.code(), o.get_out())) {
        Some((OpCode::CPUI_RETURN, _)) => true,
        Some((OpCode::CPUI_COPY | OpCode::CPUI_PIECE | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_SUBPIECE, Some(out))) if depth > 0 => {
            reaches_return(data, out, depth - 1)
        }
        _ => false,
    })
}

/// The voted type for the variable made of `instances`, all of `size` bytes.
fn vote(data: &Funcdata, instances: &[VarnodeId], size: i32) -> Option<Rc<Datatype>> {
    let mut v = Votes::default();
    for &vn in instances {
        let n = data.vbank().get(vn)?;
        if let Some(def) = n.get_def() {
            v.op(data, def, None);
        }
        for d in data.descend_snapshot(vn) {
            let slot = data.obank().get(d).and_then(|o| (0..o.num_input()).find(|&i| o.get_in(i) == Some(vn)));
            v.op(data, d, slot);
        }
    }
    let types = data.get_arch().types()?;
    let ints = v.signed + v.unsigned + v.numeric;
    if v.float > 0 && v.float >= ints && matches!(size, 4 | 8) {
        return types.get_base(size, type_metatype::TYPE_FLOAT).ok();
    }
    if v.signed == 0 && v.unsigned == 0 {
        return None;
    }
    let meta = if v.unsigned > v.signed { type_metatype::TYPE_UINT } else { type_metatype::TYPE_INT };
    types.get_base(size, meta).ok()
}

/// Type each leftover undefined variable by its uses (see the module docs).
pub struct ActionTypeVote {
    base: ActionBase,
}

impl ActionTypeVote {
    pub fn boxed(g: &str) -> Box<dyn Action> {
        Box::new(ActionTypeVote { base: ActionBase::new(0, "typevote", g) })
    }
}

impl Action for ActionTypeVote {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist.contains(self.get_group()).then(|| Box::new(ActionTypeVote { base: self.base.clone() }) as Box<dyn Action>)
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().type_vote {
            return 0;
        }
        let mut seen = std::collections::BTreeSet::new();
        let all: Vec<VarnodeId> = data.vbank().iter_loc().collect();
        for vn in all {
            let Some(h) = data.vbank().get(vn).and_then(|v| v.get_high()) else { continue };
            if !seen.insert(h) {
                continue;
            }
            let Some(ht) = data.high_get_type(vn) else { continue };
            if ht.get_metatype() != type_metatype::TYPE_UNKNOWN || !matches!(ht.get_size(), 1 | 2 | 4 | 8) {
                continue;
            }
            let Some(hv) = data.high_bank().get(h) else { continue };
            let instances: Vec<VarnodeId> = (0..hv.num_instances()).map(|i| hv.get_instance(i)).collect();
            if instances.iter().any(|&i| data.vbank().get(i).is_none_or(|n| n.is_type_lock() || n.is_constant() || n.get_size() != ht.get_size())) {
                continue;
            }
            let input = instances.iter().any(|&i| data.vbank().get(i).is_some_and(|n| n.is_input()));
            if instances.iter().any(|&i| reaches_return(data, i, 3)) {
                continue;
            }
            let Some(ct) = vote(data, &instances, ht.get_size()) else { continue };
            if input && ct.get_metatype() != type_metatype::TYPE_UINT {
                continue;
            }
            for i in instances {
                if let Some(n) = data.vbank_mut().get_mut(i) {
                    n.update_type(Rc::clone(&ct));
                }
            }
            if let Some(hh) = data.high_bank_mut().get_mut(h) {
                hh.type_dirty();
            }
            self.base.count += 1;
        }
        0
    }
}
