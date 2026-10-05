//! Keep existing variadic trials when late prototype resolution rebuilds fixed inputs.
use crate::context::VarnodeId;
use crate::fspec::{FuncCallSpecs, ParamTrial};
use crate::funcdata::Funcdata;

/// Preserve values and scoring outside the new fixed parameters. No trial is
/// invented, and definitely unused trials retain their exclusion flags.
pub(crate) fn pending_trials(data: &Funcdata, fc: &FuncCallSpecs) -> Vec<(ParamTrial, VarnodeId)> {
    if !fc.is_dotdotdot() {
        return Vec::new();
    }
    let Some(call) = data.obank().get(fc.get_op()) else {
        return Vec::new();
    };
    let fixed: Vec<_> = (0..fc.proto().num_params())
        .filter_map(|i| fc.proto().get_param(i))
        .collect();
    (0..fc.active_input().get_num_trials())
        .filter_map(|i| {
            let trial = fc.active_input().get_trial(i);
            if trial.is_unref()
                || trial.get_slot() <= 0
                || trial.get_slot() == fc.get_stack_placeholder_slot()
                || fixed.iter().any(|p| {
                    trial
                        .get_address()
                        .overlap(0, &p.get_address(), p.get_size())
                        >= 0
                        || p.get_address()
                            .overlap(0, trial.get_address(), trial.get_size())
                            >= 0
                })
            {
                return None;
            }
            call.get_in(trial.get_slot()).map(|vn| (trial.clone(), vn))
        })
        .collect()
}
