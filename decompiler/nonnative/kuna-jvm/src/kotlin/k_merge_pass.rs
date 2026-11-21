//! Port of `org/vineflower/kotlin/pass/KMergePass.java`. `KMergePass extends
//! MergeHelper` only to reach its statics: call `MergeHelper::…` for those.

use crate::assignment_exprent::AssignmentExprent;
use crate::do_statement::{DoStatement, Type as LoopType};
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::merge_helper::MergeHelper;
use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::sequence_helper::SequenceHelper;
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement};

pub struct KMergePass;

/// Java `source.getBytecodeRange(target.bytecode)`: `target`'s own offset
/// set grows by `source`'s range, and nothing happens while it is `null`.
fn measure_into(source: Exprent, target: Exprent) {
    let mut values = target.get_bytecode();
    if values.is_some() {
        source.get_bytecode_range(values.as_mut());
        target.set_bytecode(values);
    }
}

impl Pass for KMergePass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        let root = ctx.get_root().expect("NullPointerException: root");

        while KMergePass::enhance_loops_rec(root.0) {}
        SequenceHelper::condense_sequences(root.0);

        false
    }
}

impl KMergePass {
    fn enhance_loops_rec(stat: Statement) -> bool {
        let mut res = false;

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            if st.get_exprents().is_none() {
                res |= KMergePass::enhance_loops_rec(st);
            }
        }

        if let Some(do_stat) = stat.try_as_do() {
            res |= KMergePass::enhance_loop(do_stat);
        }

        res
    }

    fn enhance_loop(stat: DoStatement) -> bool {
        let oldloop = stat.get_looptype();

        match oldloop {
            LoopType::Infinite => {
                if MergeHelper::match_while(stat) && !KMergePass::match_for_each(stat) {
                    MergeHelper::match_for(stat);
                }
            }
            LoopType::While => {
                if !KMergePass::match_for_each(stat) {
                    MergeHelper::match_for(stat);
                }
            }
            _ => {}
        }

        stat.get_looptype() != oldloop
    }

    /// Hides `MergeHelper.matchForEach`.
    pub fn match_for_each(stat: DoStatement) -> bool {
        let mut first_do_exprent: Option<AssignmentExprent> = None;
        let mut init_exprents: [Option<AssignmentExprent>; 3] = [None; 3];
        let mut pre_data: Option<Statement> = None;
        let mut last_exprent: Option<Exprent> = None;

        let mut current = stat.0;
        loop {
            let Some(parent) = current.get_parent() else {
                break;
            };

            if parent.is_sequence() {
                if Some(current) == parent.get_first() {
                    current = parent;
                } else {
                    let pd =
                        current.get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Backward)[0];
                    pre_data = MergeHelper::get_last_direct_data(pd);
                    if let Some(pd) = pre_data {
                        let exprents = pd.get_exprents().expect("NullPointerException: exprents");
                        if !exprents.is_empty() {
                            let size = exprents.len();
                            for (x, slot) in init_exprents.iter_mut().enumerate() {
                                if size > x {
                                    let exprent = exprents.get(size - 1 - x);
                                    if let Some(assignment) = exprent.try_as_assignment() {
                                        *slot = Some(assignment);
                                    }
                                }
                            }
                        }
                    }
                    break;
                }
            } else {
                break;
            }
        }

        let first_data = MergeHelper::get_first_direct_data(
            stat.get_first().expect("NullPointerException: first"),
        );
        if let Some(fd) = first_data {
            if let Some(assignment) = fd
                .get_exprents()
                .expect("NullPointerException: exprents")
                .get(0)
                .try_as_assignment()
            {
                first_do_exprent = Some(assignment);
            }
        }
        let last_data = MergeHelper::get_last_direct_data(
            stat.get_first().expect("NullPointerException: first"),
        );
        if let Some(ld) = last_data {
            let exprents = ld.get_exprents().expect("NullPointerException: exprents");
            if !exprents.is_empty() {
                last_exprent = Some(exprents.get(exprents.len() - 1));
            }
        }

        if stat.get_looptype() == LoopType::While {
            if let (Some(init0), Some(first_do_exprent)) = (init_exprents[0], first_do_exprent) {
                if MergeHelper::is_iterator_call(init0.get_right()) {
                    let invc = MergeHelper::get_uncast(init0.get_right()).as_invocation();
                    if invc.get_classname().contains("java/util/stream") {
                        return false;
                    }

                    let condition = || {
                        stat.get_condition_exprent()
                            .expect("NullPointerException: conditionExprent")
                    };

                    if !MergeHelper::is_has_next_call(MergeHelper::drill_nots(condition())) {
                        return false;
                    }

                    let ass = first_do_exprent;
                    if (!MergeHelper::is_next_call(ass.get_right())
                        && !MergeHelper::is_next_unboxing(ass.get_right()))
                        || !ass.get_left().is_var()
                    {
                        return false;
                    }

                    let mut next = MergeHelper::get_uncast(ass.get_right()).as_invocation();
                    if MergeHelper::is_next_unboxing(next.0) {
                        next = MergeHelper::get_uncast(
                            next.get_instance().expect("NullPointerException: instance"),
                        )
                        .as_invocation();
                    }
                    let hnext = MergeHelper::get_uncast(MergeHelper::drill_nots(condition()))
                        .as_invocation();
                    let next_instance = next.get_instance();
                    let hnext_instance = hnext.get_instance();
                    if !next_instance.is_some_and(|i| i.is_var())
                        || !hnext_instance.is_some_and(|i| i.is_var())
                        || init0.get_left().as_var().is_var_referenced(
                            stat.0,
                            &[
                                next_instance.expect("instance").as_var(),
                                hnext_instance.expect("instance").as_var(),
                            ],
                        )
                    {
                        return false;
                    }

                    let mut right = init0.get_right();
                    if let Some(f_right) = right.try_as_function() {
                        if f_right.get_func_type() == FunctionType::Cast {
                            right = f_right.get_lst_operands().get(0);
                        }

                        if right.is_invocation() {
                            return false;
                        }
                    }

                    if MergeHelper::is_var_used_before(ass.get_left().as_var(), stat.0) {
                        return false;
                    }

                    let holder = right.as_invocation();

                    let Some(holder_instance) = holder.get_instance() else {
                        return false;
                    };

                    measure_into(init0.0, holder_instance);
                    measure_into(holder.0, holder_instance);
                    measure_into(first_do_exprent.0, ass.get_left());
                    measure_into(ass.get_right(), ass.get_left());
                    if let Some(inc) = stat.get_inc_exprent() {
                        measure_into(inc, holder_instance);
                    }
                    if let Some(init) = stat.get_init_exprent() {
                        measure_into(init, ass.get_left());
                    }

                    stat.set_looptype(LoopType::ForEach);
                    stat.set_init_exprent(ass.get_left());
                    stat.set_inc_exprent(holder_instance);
                    let pre_data = pre_data.expect("NullPointerException: preData");
                    pre_data
                        .get_exprents()
                        .expect("NullPointerException: exprents")
                        .remove_item(&init0.0);
                    first_data
                        .expect("NullPointerException: firstData")
                        .get_exprents()
                        .expect("NullPointerException: exprents")
                        .remove_item(&first_do_exprent.0);

                    if let Some(init1) = init_exprents[1] {
                        if let (Some(copy), Some(inc)) =
                            (init1.get_left().try_as_var(), holder_instance.try_as_var())
                        {
                            if copy.get_index() == inc.get_index()
                                && copy.get_version() == inc.get_version()
                                && !inc.is_var_referenced(stat.get_top_parent().0, &[copy])
                                && !MergeHelper::is_next_call(init1.get_right())
                            {
                                pre_data
                                    .get_exprents()
                                    .expect("NullPointerException: exprents")
                                    .remove_item(&init1.0);
                                measure_into(init1.0, init1.get_right());
                                measure_into(
                                    stat.get_inc_exprent()
                                        .expect("NullPointerException: incExprent"),
                                    init1.get_right(),
                                );
                                stat.set_inc_exprent(init1.get_right());
                            }
                        }
                    }

                    if let Some(type_res) = ass.0.check_expr_type_bounds() {
                        if !type_res.get_lower_bounds().is_empty() {
                            let bound_type = type_res.get_lower_bounds()[0].type_.clone();
                            let var = ass.get_left().as_var();
                            var.set_bound_type(Some(bound_type));
                        }
                    }

                    return true;
                } else if let Some(init1) = init_exprents[1] {
                    let Some(arr) = first_do_exprent.get_right().try_as_array() else {
                        return false;
                    };
                    if !first_do_exprent.get_left().is_var() {
                        return false;
                    }

                    let Some(func_inc) = last_exprent.and_then(|e| e.try_as_function()) else {
                        return false;
                    };
                    let last_exprent = func_inc.0;

                    if !init1.get_right().is_const() {
                        return false;
                    }
                    let Some(func_right) = init0.get_right().try_as_function() else {
                        return false;
                    };
                    if !stat
                        .get_condition_exprent()
                        .is_some_and(|c| c.is_function())
                    {
                        return false;
                    }

                    let inc_type = func_inc.get_func_type();

                    if func_right.get_func_type() != FunctionType::ArrayLength
                        || (inc_type != FunctionType::Ppi && inc_type != FunctionType::Ipp)
                    {
                        return false;
                    }
                    let Some(index) = arr.get_index().try_as_var() else {
                        return false;
                    };
                    let Some(array) = arr.get_array().try_as_var() else {
                        return false;
                    };

                    let count_expr = func_inc.get_lst_operands().get(0);

                    if let Some(counter) = count_expr.try_as_var() {
                        if counter.get_index() != index.get_index()
                            || counter.get_version() != index.get_version()
                        {
                            return false;
                        }

                        if counter.is_var_referenced(
                            stat.get_first().expect("NullPointerException: first"),
                            &[index],
                        ) {
                            return false;
                        }
                    }

                    if MergeHelper::is_var_used_before(first_do_exprent.get_left().as_var(), stat.0)
                    {
                        return false;
                    }

                    let length_of = || func_right.get_lst_operands().get(0);
                    length_of().add_bytecode_offsets(init0.get_bytecode().as_ref());
                    length_of().add_bytecode_offsets(init1.get_bytecode().as_ref());
                    length_of().add_bytecode_offsets(last_exprent.get_bytecode().as_ref());
                    first_do_exprent
                        .get_left()
                        .add_bytecode_offsets(first_do_exprent.get_bytecode().as_ref());
                    first_do_exprent
                        .get_left()
                        .add_bytecode_offsets(init0.get_bytecode().as_ref());

                    stat.set_looptype(LoopType::ForEach);
                    stat.set_init_exprent(first_do_exprent.get_left());
                    stat.set_inc_exprent(length_of());
                    let pre_data = pre_data.expect("NullPointerException: preData");
                    let pre_exprents = || {
                        pre_data
                            .get_exprents()
                            .expect("NullPointerException: exprents")
                    };
                    pre_exprents().remove_item(&init1.0);
                    pre_exprents().remove_item(&init0.0);
                    first_data
                        .expect("NullPointerException: firstData")
                        .get_exprents()
                        .expect("NullPointerException: exprents")
                        .remove_item(&first_do_exprent.0);
                    last_data
                        .expect("NullPointerException: lastData")
                        .get_exprents()
                        .expect("NullPointerException: exprents")
                        .remove_item(&last_exprent);

                    if let Some(init2) = init_exprents[2] {
                        if let Some(copy) = init2.get_left().try_as_var() {
                            if copy.get_index() == array.get_index()
                                && copy.get_version() == array.get_version()
                            {
                                pre_exprents().remove_item(&init2.0);
                                init2
                                    .get_right()
                                    .add_bytecode_offsets(init2.get_bytecode().as_ref());
                                init2.get_right().add_bytecode_offsets(
                                    stat.get_inc_exprent()
                                        .expect("NullPointerException: incExprent")
                                        .get_bytecode()
                                        .as_ref(),
                                );
                                stat.set_inc_exprent(init2.get_right());
                            }
                        }
                    }

                    if let Some(type_res) = first_do_exprent.0.check_expr_type_bounds() {
                        if !type_res.get_lower_bounds().is_empty() {
                            let bound_type = type_res.get_lower_bounds()[0].type_.clone();
                            let var = first_do_exprent.get_left().as_var();
                            var.set_bound_type(Some(bound_type));
                        }
                    }

                    return true;
                }
            }
        }

        false
    }
}
