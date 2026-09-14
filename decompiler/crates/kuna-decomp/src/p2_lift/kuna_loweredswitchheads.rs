//! (kuna `loweredswitchheads`) Look for a lowered-switch cascade behind every head
//! on the switch variable, not only the first.
//!
//! [`crate::kuna_loweredswitch`] picks the most-compared variable and recovers a
//! cascade from a head: a compare on that variable that no other compare on it
//! continues into. Any unrelated test of the same variable is a head as well, and
//! upstream tried only the first head in block order. MSVC `/O2` inlines an
//! `and_chain(argc, ...)` and a 15-case `cascade_switch(argc)` into `main`; the
//! inlined `TEST EBX,EBX; JLE` comes first, recovers no case, and the
//! binary-search tree behind it stayed a goto ladder into the shared join.
//!
//! With the option on, detection tries the heads in block order and records the
//! first cascade that recovers. A cascade behind a head after the first is kept
//! only when [`crate::p2_lift::kuna_loweredswitchexact::record_is_exact`] shows the
//! switch the install builds routes every value and keeps every statement as the
//! compare tree does, and its install runs the raw p-code check, whether or not
//! `loweredswitchexact` is on, and when no case
//! body is also entered from outside the compare tree (from another case's body,
//! or from code after the switch): the structurer prints such a switch with a
//! `goto` to a label it never declares.

use std::collections::BTreeSet;

use kuna_base::marshal::ElementId;

use crate::context::BlockId;
use crate::funcdata::Funcdata;
use crate::p2_lift::kuna_loweredswitch::KunaLoweredSwitchRecord;
use crate::p2_lift::kuna_loweredswitchexact::{block_at_start, removed_blocks};

/// Marshaling element `<loweredswitchheads>` (kuna 4000+ range).
pub const ELEM_LOWEREDSWITCHHEADS: ElementId = ElementId::new("loweredswitchheads", 4172);

/// The cascade heads to try, in block order: every compare on the switch
/// variable in `spine` that no other one continues into, or only the first of
/// them when `every_head` is off.
pub(crate) fn cascade_heads(spine: &BTreeSet<BlockId>, is_child: &BTreeSet<BlockId>, every_head: bool) -> Vec<BlockId> {
    let heads = spine.iter().copied().filter(|bl| !is_child.contains(bl));
    if every_head {
        heads.collect()
    } else {
        heads.take(1).collect()
    }
}

/// True when the body of a case of `rec`, or its default, is also entered from
/// somewhere other than the compare tree at `head`: from another case's body or
/// from code outside the switch.
pub(crate) fn case_body_entered_elsewhere(data: &Funcdata, head: BlockId, rec: &KunaLoweredSwitchRecord) -> bool {
    let mut targets: Vec<BlockId> = Vec::new();
    let mut bodies: BTreeSet<BlockId> = BTreeSet::new();
    for addr in rec.case_targets.iter().chain(std::iter::once(&rec.default_target)) {
        let b = match block_at_start(data, addr) {
            Some(b) => b,
            None => return true,
        };
        targets.push(b);
        bodies.insert(b);
    }
    let tree = removed_blocks(data, head, &targets);
    bodies.iter().any(|&body| {
        let blk = data.bblocks_ref().block(body);
        (0..blk.size_in()).map(|i| blk.get_in(i)).any(|p| p != head && !tree.contains(&p))
    })
}

#[cfg(test)]
#[path = "kuna_loweredswitchheads/tests.rs"]
mod tests;
