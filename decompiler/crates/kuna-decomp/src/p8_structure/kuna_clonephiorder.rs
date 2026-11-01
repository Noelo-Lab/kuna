//! (kuna `clonephiorder`) The order a cloned return block prints the stores its
//! phis absorbed.
//!
//! When copy propagation folds a store's value into a MULTIEQUAL on a global (the
//! phi reads the stored register directly and the store itself dies), the store's
//! position in its predecessor is gone.  Splitting the return block then turns the
//! clone's copy of that phi into the store, at the start of the clone.  Several
//! such stores print in the order the clone's phis are cloned, and nothing in the
//! IR records the order the binary made them in.
//!
//! With `option clonephiorder on` (the default), [`order_global_phis`] clones the
//! phis on globals after the others and in address order, which is how kuna
//! printed most of these stores before the clone kept its return guards'
//! `return_copy` flag.  Off clones the phis in block order, the reverse of the
//! order heritage placed them in, which is descending address order for phis one
//! heritage pass placed.  Only the clone is reordered; the edge the
//! original block keeps prints its absorbed stores in block order either way.
//! Neither order is read from the binary: a source that stores globals in
//! descending address order prints them reversed with the option on.

use kuna_base::error::KunaResult;

use crate::context::OpId;
use crate::funcdata::Funcdata;
use crate::options::on_or_off;
use kuna_num::opcodes::OpCode;

/// Sort the leading phis in `ops`, the op list of a block about to be cloned, so the
/// ones on globals come last, by address.  Only the clone takes the new order.
pub(crate) fn order_global_phis(data: &Funcdata, ops: &mut [OpId]) {
    let n = ops
        .iter()
        .take_while(|&&op| data.obank().get(op).is_some_and(|o| o.code() == OpCode::CPUI_MULTIEQUAL))
        .count();
    let global_addr = |op: OpId| {
        data.obank()
            .get(op)
            .and_then(|o| o.get_out())
            .and_then(|v| data.vbank().get(v))
            .filter(|v| v.is_persist() && v.is_addr_tied())
            .map(|v| v.get_addr().clone())
    };
    ops[..n].sort_by(|&a, &b| match (global_addr(a), global_addr(b)) {
        (None, None) => std::cmp::Ordering::Equal,
        (None, Some(_)) => std::cmp::Ordering::Less,
        (Some(_), None) => std::cmp::Ordering::Greater,
        (Some(x), Some(y)) => x.cmp(&y),
    });
}

/// The `clonephiorder on|off` option.
pub struct OptionClonePhiOrder;

impl OptionClonePhiOrder {
    pub const NAME: &'static str = "clonephiorder";

    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Address order for a cloned return block's global phis turned {prop}")))
    }
}
