//! Opt-in stack-address evidence for an existing register input trial.
use crate::{funcdata::Funcdata, context::VarnodeId};
use super::kuna_spillargtrial::{frame_slot, stack_pointer_storage};

/// A same-width copy/displacement chain must reach a particular stack pointer.
/// Unknown inputs, phi merges, and values killed by another call are declined.
pub(crate) fn carries_stack_address(data: &Funcdata, vn: VarnodeId) -> bool {
    if !data.get_arch().stack_addr_arg_trial { return false; }
    let Some(sp) = stack_pointer_storage(data) else { return false; };
    let Some(v) = data.vbank().get(vn) else { return false; };
    if v.get_size() != sp.size as i32 || !v.is_written() { return false; }
    frame_slot(data, vn, &sp).is_some_and(|(base, _)| base != vn)
}

/// Preserve proven register pointers across the inactive-prefix length heuristic.
/// The existing ABI hole-filling loop supplies earlier register positions.
pub(crate) fn trial_is_protected(
    active: &crate::fspec::ParamActive,
    trial: &crate::fspec::ParamTrial,
    entries: &[crate::fspec::ParamEntry],
) -> bool {
    active.is_recover_subcall() && trial.is_active() && trial.has_stack_address()
        && trial.get_entry().is_some_and(|i| entries[i].is_exclusion())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use kuna_base::{address::Address, space::{AddrSpace, AddrSpaceManager, spacetype}};
    use crate::{dtype::type_class, fspec::{ParamActive, ParamEntry, ParamListKind, ParamListStandard}};

    #[test]
    fn only_a_proven_register_trial_survives_the_prefix_gap() {
        for (proof, dead, expected) in [(false,false,0),(true,false,4),(true,true,0)] {
            let mgr=AddrSpaceManager::new();
            let reg=Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR,"register",false,4,1,3,0,0,0));
            let mut model=ParamListStandard::new(ParamListKind::Standard);
            for group in 0..4 {
                let e=ParamEntry::seed(group,type_class::TYPECLASS_GENERAL,Rc::clone(&reg),group as u64*4,4,1,0,0,true,false,model.get_entry(),&mgr).unwrap();
                model.push_entry(e);
            }
            model.finish_decode();
            let mut active=ParamActive::new(true);
            active.register_trial(&Address::new(reg,12),4);
            let trial=active.get_trial_mut(0);
            trial.mark_active();
            if proof {trial.set_stack_address();}
            if dead {trial.mark_no_use();}
            model.fillin_map(&mut active,&mgr).unwrap();
            assert_eq!((0..active.get_num_trials()).filter(|&i|active.get_trial(i).is_used()).count(),expected);
        }
    }

    #[test]
    fn pointer_evidence_does_not_protect_a_stack_slot_or_own_input() {
        let mgr=AddrSpaceManager::new();
        let stack=Rc::new(AddrSpace::new(spacetype::IPTR_SPACEBASE,"stack",false,4,1,4,0,0,0));
        let entry=ParamEntry::seed(0,type_class::TYPECLASS_GENERAL,Rc::clone(&stack),0,128,1,4,0,true,false,&[],&mgr).unwrap();
        let mut active=ParamActive::new(true);
        active.register_trial(&Address::new(stack,0),4);
        let trial=active.get_trial_mut(0);trial.mark_active();trial.set_stack_address();trial.set_entry(Some(0),0);
        assert!(!trial_is_protected(&active,active.get_trial(0),&[entry]));
        let own=ParamActive::new(false);
        assert!(!trial_is_protected(&own,active.get_trial(0),&[]));
    }
}
