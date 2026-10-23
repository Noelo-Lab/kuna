//! (kuna `protoorder` trial storage) Which direct calls the P4 bootstrap may retype.
//!
//! A single-function drive has no callee-first order behind it, so a call's
//! parameter types are whatever the caller itself wrote into registers.  The
//! only fact a callee's own recovery can add there is what heritage destroyed:
//! a compiler-spec `<prefersplit>` register is replaced at heritage pass 0 by
//! its scalar pieces, so a value the callee reads as one scalar arrives at the
//! caller as half a whole register.
//!
//! `ActionActiveParam` already decides, per call site, which of those argument
//! ranges are real parameters: its input trials are the CALL op's own
//! argument storage, scored by data flow.  An active trial whose range is a
//! preferred-split register (or one of its pieces) is therefore the exact
//! witness for this bootstrap — no block/dominator reasoning and no width
//! audit.  A value merged through a `MULTIEQUAL` (`if (c) q0=x; else q0=y;`)
//! is one trial on that storage like any other, which is the case a
//! dominance-shaped gate misses.

use kuna_base::address::Address;
use kuna_base::types::int4;

use crate::funcdata::Funcdata;
use crate::p5_types::prefersplit::PreferSplitRecord;

/// Is `addr:size` the whole preferred-split register of `record`, or a scalar
/// argument living inside one of the two pieces heritage pass 0 replaced it
/// with?
///
/// The record names the full container (`q0`, 16 bytes) and the byte offset of
/// the second piece (`8`).  A scalar argument in the register can be whole (16
/// bytes), a piece (8), or narrower than a piece: a single-precision `float`
/// arrives in the low 4 bytes of `d0` in little-endian and the high 4 in
/// big-endian.  The range must therefore stay inside ONE piece; an access that
/// crosses the split is not a scalar the split creates.
pub fn is_split_storage(records: &[PreferSplitRecord], addr: &Address, size: int4) -> bool {
    let Some(space) = addr.get_space() else { return false };
    records.iter().any(|record| {
        let Some(rec_space) = record.storage.space.as_ref() else { return false };
        if rec_space.get_index() != space.get_index() {
            return false;
        }
        let start = record.storage.offset;
        let total = record.storage.size as int4;
        let split = record.splitoffset;
        if size <= 0 || size > total {
            return false;
        }
        let off = addr.get_offset();
        let Some(rel) = off.checked_sub(start) else { return false };
        if rel > (total - size) as u64 {
            return false;
        }
        let rel = rel as int4;
        let end = rel + size;
        // The whole container is the value heritage splits, so it hits too.
        size == total || end <= split || (rel >= split && end <= total)
    })
}

/// The direct callee a call's active input trials show receiving preferred-split
/// storage, or `None`.
///
/// The trial list is the caller's own recovery, so this sees exactly the
/// arguments the call takes; `is_active` is the per-trial verdict
/// `check_input_trial_use` reached (a write reached the call with no
/// intervening read).  Declared and already-stated callees are excluded here so
/// the drive does not pause for a recovery that cannot park anything.
pub fn split_trial_callee(data: &Funcdata, call_index: int4) -> Option<Address> {
    let records = &data.get_arch().split_records;
    if records.is_empty() {
        return None;
    }
    let fc = data.get_call_specs(call_index);
    let entry = fc.get_entry_address();
    if entry.is_invalid() || entry == data.get_address() {
        return None;
    }
    if data.kuna_protoorder_types(entry).is_some() {
        return None;
    }
    let active = fc.active_input();
    let hit = (0..active.get_num_trials()).any(|i| {
        let trial = active.get_trial(i);
        trial.is_active()
            && !trial.is_unref()
            && is_split_storage(records, trial.get_address(), trial.get_size())
    });
    hit.then(|| entry.clone())
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace};
    use kuna_num::pcoderaw::VarnodeData;

    use super::*;

    fn manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "register",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m
    }

    fn records(manage: &AddrSpaceManager) -> Vec<PreferSplitRecord> {
        let reg = Rc::clone(manage.get_space_by_name("register").unwrap());
        vec![PreferSplitRecord {
            storage: VarnodeData { space: Some(reg), size: 16, offset: 0 },
            splitoffset: 8,
        }]
    }

    #[test]
    fn whole_register_and_each_piece_hit() {
        let manage = manager();
        let recs = records(&manage);
        let reg = Rc::clone(manage.get_space_by_name("register").unwrap());
        assert!(is_split_storage(&recs, &Address::new(Rc::clone(&reg), 0), 16));
        assert!(is_split_storage(&recs, &Address::new(Rc::clone(&reg), 0), 8));
        assert!(is_split_storage(&recs, &Address::new(Rc::clone(&reg), 8), 8));
    }

    #[test]
    fn a_single_precision_scalar_inside_either_piece_hits() {
        let manage = manager();
        let recs = records(&manage);
        let reg = Rc::clone(manage.get_space_by_name("register").unwrap());
        // Little-endian `s0` is the low word of `d0`; big-endian is the high one.
        assert!(is_split_storage(&recs, &Address::new(Rc::clone(&reg), 0), 4));
        assert!(is_split_storage(&recs, &Address::new(Rc::clone(&reg), 4), 4));
        assert!(is_split_storage(&recs, &Address::new(Rc::clone(&reg), 8), 4));
        assert!(is_split_storage(&recs, &Address::new(Rc::clone(&reg), 12), 4));
    }

    #[test]
    fn integer_and_unrelated_storage_miss() {
        let manage = manager();
        let recs = records(&manage);
        let reg = Rc::clone(manage.get_space_by_name("register").unwrap());
        // The next argument register is not part of the split container.
        assert!(!is_split_storage(&recs, &Address::new(Rc::clone(&reg), 0x10), 8));
        // An access that crosses the split is not a scalar the split creates.
        assert!(!is_split_storage(&recs, &Address::new(Rc::clone(&reg), 4), 8));
        // A register window that runs past the container.
        assert!(!is_split_storage(&recs, &Address::new(Rc::clone(&reg), 12), 8));
        assert!(!is_split_storage(&recs, &Address::new(Rc::clone(&reg), 0), 20));
    }
}
