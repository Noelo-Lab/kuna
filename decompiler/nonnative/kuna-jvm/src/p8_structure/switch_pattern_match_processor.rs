//! Port of `org/jetbrains/java/decompiler/modules/decompiler/SwitchPatternMatchProcessor.java`.

use crate::assignment_exprent::AssignmentExprent;
use crate::code_constants::{CONSTANT_CLASS, CONSTANT_INTEGER, CONSTANT_STRING};
use crate::code_type::CodeType;
use crate::const_exprent::{ConstExprent, ConstValue};
use crate::counter_container::CounterContainer;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::field_descriptor::FieldDescriptor;
use crate::field_exprent::FieldExprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::if_pattern_match_processor::IfPatternMatchProcessor;
use crate::if_statement::IfStatement;
use crate::invocation_exprent::InvocationType;
use crate::jthrow;
use crate::pair::Pair;
use crate::pattern_exprent::PatternExprent;
use crate::pooled_constant::PooledConstant;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JList, JavaEq};
use crate::sequence_helper::SequenceHelper;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::switch_head_exprent::SwitchHeadExprent;
use crate::switch_helper::SwitchHelper;
use crate::switch_statement::SwitchStatement;
use crate::type_family::TypeFamily;
use crate::validation_helper::ValidationHelper;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;

pub struct SwitchPatternMatchProcessor;

impl SwitchPatternMatchProcessor {
    pub fn process_pattern_matching(root: Statement) -> bool {
        let ret = Self::process_pattern_matching_rec(root, root);

        if ret {
            SequenceHelper::condense_sequences(root);
        }

        ret
    }

    fn process_pattern_matching_rec(stat: Statement, root: Statement) -> bool {
        ValidationHelper::validate_statement(root.as_root());

        let mut ret = false;
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            ret |= Self::process_pattern_matching_rec(st, root);
            ValidationHelper::validate_statement(root.as_root());
        }

        if let Some(switch_stat) = stat.try_as_switch() {
            ret |= Self::process_statement(switch_stat, root);
            ValidationHelper::validate_statement(root.as_root());
        }

        ret
    }

    fn process_statement(stat: SwitchStatement, root: Statement) -> bool {
        if stat.is_phantom() {
            return false;
        }

        let head = stat.get_headexprent().as_switch_head();

        let switch_pattern_match = Self::is_switch_pattern_match(head);

        if !switch_pattern_match {
            return false;
        }

        let value = head.get_value().as_invocation();
        let orig_params = value.get_lst_parameters();
        let mut real_selector = orig_params.get(0);
        let mut guarded = true;
        let is_enum_switch = value.get_name() == "enumSwitch";
        let mut _null_case = false;
        let mut references: Vec<Pair<Statement, Exprent>> = Vec::new();
        if let Some(var) = orig_params.get(1).try_as_var() {
            SwitchHelper::find_exprents(
                root,
                Some,
                |e| var.is_var_referenced_exp(e, &[]),
                false,
                |st, expr| references.push(Pair::of(st, expr)),
            );
            if references.len() == 1 {
                let reference = references[0];
                if reference.b.is_assignment() {
                    reference
                        .a
                        .get_exprents()
                        .expect("exprents")
                        .remove_item(&reference.b);
                    guarded = false;
                }
            }
        }

        let mut guards: JHashMap<Vec<Option<Exprent>>, Exprent> = JHashMap::with_capacity(0);
        if guarded {
            guards = JHashMap::with_capacity(references.len());
            let mut can_eliminate = true;
            let initial_use = references[0];
            let initial_const = initial_use
                .b
                .try_as_assignment()
                .and_then(|a| a.get_right().try_as_const());
            if let Some(const_exprent) = initial_const {
                if const_exprent.get_const_type().type_family == TypeFamily::Integer
                    && const_exprent.get_int_value() == 0
                {
                    references.remove(0);
                } else {
                    return false;
                }
            } else {
                return false;
            }
            for reference in &references {
                can_eliminate &= Self::eliminate_guard_ref(stat, &mut guards, reference, true);
            }
            if !can_eliminate {
                return false;
            }
            initial_use
                .a
                .get_exprents()
                .expect("exprents")
                .remove_item(&initial_use.b);
            for reference in &references {
                Self::eliminate_guard_ref(stat, &mut guards, reference, false);
            }
        }

        let mut default_case: i32 = -1;

        let mut i = 0usize;
        while i < stat.get_case_statements().len() {
            if stat.get_case_values().get(i).contains(&None) {
                default_case = i as i32;
            }
            let mut case_stat = stat.get_case_statements().get(i);

            let all_cases = stat.get_case_values().get(i);
            let Some(case_expr) = all_cases.get(0) else {
                i += 1;
                continue;
            };

            let guard_key = all_cases.to_vec();
            if guards.contains_key(&guard_key) {
                while stat.get_case_guards().len() <= i {
                    stat.get_case_guards().add(None);
                }
                stat.get_case_guards()
                    .set(i, guards.get(&guard_key).copied());
            }
            if let Some(const_case) = case_expr.try_as_const() {
                let case_value = const_case.get_int_value();

                if case_value == -1 {
                    _null_case = true;
                    all_cases.remove_item(&Some(case_expr));
                    let null_const = ConstExprent::new(VarType::VARTYPE_NULL(), None, None);
                    if all_cases.contains(&None) {
                        let index = all_cases.index_of(&None) as usize;
                        all_cases.add_at(index, Some(null_const.0));
                    } else {
                        all_cases.add(Some(null_const.0));
                    }
                }
            }

            if case_stat.is_sequence() {
                let old_stat = case_stat;
                case_stat = old_stat.get_stats().borrow().get(0);
                let old_count = old_stat.get_stats().borrow().len();
                if old_count == 1 {
                    old_stat.replace_with(case_stat);
                }
            }

            let case_guards = stat.get_case_guards();
            let pattern =
                if case_guards.len() > i && case_guards.get(i).is_some_and(|g| g.is_pattern()) {
                    case_guards.set(i, None).map(|g| g.as_pattern())
                } else {
                    Self::identify_switch_record_pattern_match(
                        stat,
                        stat.get_case_statements().get(i),
                        false,
                    )
                };

            if let Some(pattern) = pattern {
                let operands: Vec<Exprent> = vec![
                    real_selector,
                    ConstExprent::new(pattern.get_expr_type(), None, None).0,
                    pattern.0,
                ];

                let function = FunctionExprent::new(FunctionType::Instanceof, operands, None);
                all_cases.set(0, Some(function.0));
            } else {
                let case_stat_block = case_stat.get_basichead();
                let block_exprents = case_stat_block.get_exprents().expect("exprents");
                if !block_exprents.is_empty() {
                    let expr = block_exprents.get(0);
                    if let Some(assign) = expr.try_as_assignment() {
                        if let Some(var) = assign.get_left().try_as_var() {
                            if Self::is_pattern_matching_cast_assignment(head, assign) {
                                let mut operands: Vec<Exprent> = Vec::new();
                                if let Some(check) = assign.get_right().try_as_var() {
                                    if let Some(const_expr) = case_expr.try_as_const() {
                                        let bootstrap_arguments =
                                            value.get_bootstrap_arguments().expect("bsm args");
                                        let index = if const_expr.get_int_value() == -1 {
                                            bootstrap_arguments.len() - 1
                                        } else {
                                            const_expr.get_int_value() as usize
                                        };
                                        if let Some(PooledConstant::Primitive(primitive)) =
                                            &bootstrap_arguments[index]
                                        {
                                            if primitive.type_ == CONSTANT_CLASS {
                                                operands.push(check.0);
                                                operands.push(
                                                    ConstExprent::new(
                                                        VarType::VARTYPE_CLASS(),
                                                        primitive
                                                            .value
                                                            .as_ref()
                                                            .map(ConstValue::from),
                                                        None,
                                                    )
                                                    .0,
                                                );
                                                operands.push(var.0);
                                                if all_cases.contains(&None) {
                                                    stat.set_default_edge(None);
                                                    let index = all_cases.index_of(&None) as usize;
                                                    all_cases.remove_at(index);
                                                    stat.get_case_edges().get(i).remove_at(index);
                                                }
                                            }
                                        }
                                    }
                                } else if let Some(cast) = assign.get_right().try_as_function() {
                                    operands.push(cast.get_lst_operands().get(0));
                                    operands.push(cast.get_lst_operands().get(1));
                                    operands.push(var.0);
                                }

                                if !operands.is_empty() {
                                    let func = FunctionExprent::new(
                                        FunctionType::Instanceof,
                                        operands,
                                        None,
                                    );

                                    case_stat_block
                                        .get_exprents()
                                        .expect("exprents")
                                        .remove_at(0);

                                    all_cases.set(0, Some(func.0));
                                }
                            }
                        }
                    }
                }
            }

            i += 1;
        }

        if default_case != -1 {
            let default_case = default_case as usize;
            let case_values = stat.get_case_values();
            let moved = case_values.remove_at(default_case);
            case_values.add(moved);
            let case_statements = stat.get_case_statements();
            let moved = case_statements.remove_at(default_case);
            case_statements.add(moved);
            let case_edges = stat.get_case_edges();
            let moved = case_edges.remove_at(default_case);
            case_edges.add(moved);
            let case_guards = stat.get_case_guards();
            if default_case < case_guards.len() {
                if case_guards.get(default_case).is_some() {
                    while case_guards.len() < stat.get_case_statements().len() {
                        case_guards.add(None);
                    }
                    let moved = case_guards.remove_at(default_case);
                    case_guards.add(moved);
                } else {
                    case_guards.remove_at(default_case);
                }
            }
        }

        let bootstrap_arguments = value.get_bootstrap_arguments().expect("bsm args");
        for (i, bsa) in bootstrap_arguments.iter().enumerate() {
            let i = i as i32;
            let mut replace_index = i;
            for case_value_set in stat.get_case_values().iter() {
                if let Some(const_expr) = case_value_set.get(0).and_then(|e| e.try_as_const()) {
                    if let Some(ConstValue::Int(v)) = const_expr.get_value() {
                        if v == i {
                            replace_index = index_of_list(&stat.get_case_values(), &case_value_set);
                        }
                    }
                }
            }
            if let Some(PooledConstant::Primitive(p)) = bsa {
                let mut new_value: Option<Exprent> = None;
                match p.type_ {
                    CONSTANT_INTEGER => {
                        let boxed = p.value.as_ref().expect("NullPointerException: value");
                        new_value = Some(
                            ConstExprent::new_int(ConstValue::from(boxed).as_int(), false, None).0,
                        );
                    }
                    CONSTANT_STRING => {
                        if is_enum_switch {
                            let type_name = real_selector.get_expr_type().value_str().to_string();
                            let name = p
                                .value
                                .as_ref()
                                .expect("NullPointerException: value")
                                .to_java_string();
                            new_value = Some(
                                FieldExprent::new_full(
                                    name,
                                    type_name.clone(),
                                    true,
                                    None,
                                    FieldDescriptor::parse_descriptor(&format!("L{type_name};")),
                                    None,
                                    false,
                                    false,
                                )
                                .0,
                            );
                        } else {
                            new_value = Some(
                                ConstExprent::new(
                                    VarType::VARTYPE_STRING(),
                                    p.value.as_ref().map(ConstValue::from),
                                    None,
                                )
                                .0,
                            );
                        }

                        replace_index = Self::get_index(i, &stat.get_case_values());
                    }
                    CONSTANT_CLASS => {
                        let replace_values = stat.get_case_values().get(replace_index as usize);
                        if replace_values
                            .iter()
                            .all(|x| x.is_some_and(|x| x.is_const()))
                        {
                            let cast_type =
                                VarType::new_value(CodeType::Object, 0, p.get_string_opt());
                            let mut operands: Vec<Exprent> = Vec::new();
                            operands.push(real_selector);
                            operands.push(ConstExprent::new(cast_type.clone(), None, None).0);
                            let index = DecompilerContext::get_counter_container()
                                .get_counter_and_increment(CounterContainer::VAR_COUNTER);
                            operands.push(
                                VarExprent::new(
                                    index,
                                    Some(cast_type),
                                    Some(DecompilerContext::get_var_processor()),
                                )
                                .0,
                            );
                            new_value = Some(
                                FunctionExprent::new(FunctionType::Instanceof, operands, None).0,
                            );
                        }
                    }
                    _ => ValidationHelper::assert_true(false, "unexpected case"),
                }
                if let Some(nvx) = new_value {
                    let ix = i;
                    let list = stat.get_case_values().get(replace_index as usize);
                    let replaced: Vec<Option<Exprent>> = list
                        .to_vec()
                        .into_iter()
                        .map(|u| match u {
                            Some(c)
                                if c.is_const()
                                    && c.get_expr_type().type_family == TypeFamily::Integer
                                    && c.as_const().get_int_value() == ix =>
                            {
                                Some(nvx)
                            }
                            other => other,
                        })
                        .collect();
                    list.set_all(replaced);
                }
            }
        }

        let mut replace_with = stat.0;
        let mut parent = stat.get_parent();
        if let Some(seq) = parent.filter(|p| p.is_sequence()) {
            let seq_stats = seq.get_stats().borrow().to_vec();
            if seq_stats.len() == 2 && seq_stats[0] == stat.0 && seq_stats[1].is_basic_block() {
                let block_exprents = seq_stats[1].get_exprents().expect("exprents");
                if block_exprents.len() == 1 {
                    if let Some(exit) = block_exprents.get(0).try_as_exit() {
                        if exit.get_value().is_some() {
                            replace_with = seq;
                            parent = seq.get_parent();
                        }
                    }
                }
            }
        }
        if guarded {
            if let Some(do_parent) = parent.filter(|p| p.is_do()) {
                do_parent.replace_with(replace_with);
                for edge in stat.get_predecessor_edges(StatEdge::TYPE_CONTINUE) {
                    edge.change_type(StatEdge::TYPE_BREAK);
                }
            }
        }

        let old_selector = real_selector;
        let mut basic_head: Option<JList<Exprent>> = stat.get_basichead().get_exprents();
        if basic_head
            .as_ref()
            .expect("NullPointerException: exprents")
            .is_empty()
        {
            let edges = replace_with.get_predecessor_edges(StatEdge::TYPE_REGULAR);
            if edges.len() == 1 && edges[0].get_source().is_basic_block() {
                basic_head = edges[0].get_source().get_exprents();
            }
        }
        if let (Some(var), Some(basic_head)) = (real_selector.try_as_var(), basic_head.as_ref()) {
            if !basic_head.is_empty() {
                if let Some(assignment) = basic_head.get(basic_head.len() - 1).try_as_assignment() {
                    if let Some(assigned) = assignment.get_left().try_as_var() {
                        if var.equals(assigned.0) {
                            let mut whitelist: Vec<VarExprent> = vec![assigned];
                            for exp in stat.get_case_values().iter().flat_map(|l| l.iter()) {
                                let Some(func) = exp.and_then(|e| e.try_as_function()) else {
                                    continue;
                                };
                                if func.get_func_type() == FunctionType::Instanceof {
                                    if let Some(checked) =
                                        func.get_lst_operands().get(0).try_as_var()
                                    {
                                        if checked.equals(var.0) {
                                            whitelist.push(func.get_lst_operands().get(0).as_var());
                                        }
                                    }
                                }
                            }
                            if !var.is_var_referenced(root, &whitelist) {
                                real_selector = assignment.get_right();
                                basic_head.remove_at(basic_head.len() - 1);
                            }
                        }
                    }
                }
            }
        }

        if old_selector != real_selector {
            let final_selector = real_selector;
            for list in stat.get_case_values().iter() {
                for exp in list.iter().flatten() {
                    if let Some(func) = exp.try_as_function() {
                        if func.get_func_type() == FunctionType::Instanceof
                            && func.get_lst_operands().get(0).equals(old_selector)
                        {
                            func.get_lst_operands().set(0, final_selector);
                        }
                    }
                }
            }
        }

        head.set_value(real_selector);

        true
    }

    fn get_index(check: i32, values: &JList<JList<Option<Exprent>>>) -> i32 {
        for i in 0..values.len() {
            let list = values.get(i);

            for exp in list.iter() {
                if let Some(cons) = exp.and_then(|e| e.try_as_const()) {
                    if let Some(ConstValue::Int(val)) = cons.get_value() {
                        if val == check {
                            return i as i32;
                        }
                    }
                }
            }
        }

        jthrow!(IllegalStateException; "Unexpected case in switch pattern matching")
    }

    /// `guards` is Java's `Map<List<Exprent>, Exprent>`, keyed by the content of a case-value
    /// list (`List.equals`/`hashCode`), held here as a snapshot of that list.
    fn eliminate_guard_ref(
        stat: SwitchStatement,
        guards: &mut JHashMap<Vec<Option<Exprent>>, Exprent>,
        reference: &Pair<Statement, Exprent>,
        simulate: bool,
    ) -> bool {
        if reference.b.is_assignment() {
            let assign_stat = reference.a;

            let mut i = 0usize;
            while i < stat.get_case_statements().len() {
                if stat
                    .get_case_statements()
                    .get(i)
                    .contains_statement(assign_stat)
                {
                    let pattern_stat = stat.get_case_statements().get(i);
                    let pattern =
                        Self::identify_switch_record_pattern_match(stat, pattern_stat, simulate);
                    if let Some(pattern) = pattern {
                        if !simulate {
                            guards.put(stat.get_case_values().get(i).to_vec(), pattern.0);
                        }
                        return true;
                    }
                    break;
                }
                i += 1;
            }

            if assign_stat.get_all_predecessor_edges().len() > 1 {
                return false;
            }
            let mut parent = assign_stat.get_parent();
            let mut invert = true;
            if let Some(seq) = parent.filter(|p| p.is_sequence()) {
                let seq_stats = seq.get_stats().borrow().to_vec();
                if seq_stats.len() == 2 && seq_stats[1] == assign_stat {
                    parent = Some(seq_stats[0]);
                    invert = false;
                }
            }
            let guard_layout = assign_stat.is_basic_block()
                && assign_stat.get_exprents().expect("exprents").len() == 1
                && parent.and_then(|p| p.try_as_if()).is_some_and(|if_parent| {
                    if_parent.iftype() == IfStatement::IFTYPE_IF && if_parent.get_ifstat().is_some()
                })
                && parent
                    .and_then(|p| p.get_parent())
                    .is_some_and(|pp| pp.is_sequence())
                && parent
                    .and_then(|p| p.get_parent())
                    .and_then(|pp| pp.get_parent())
                    == Some(stat.0);
            if guard_layout {
                let continue_edge = assign_stat.get_successor_edges(StatEdge::TYPE_CONTINUE)[0];
                let next = continue_edge.get_destination();
                if Some(next) == stat.get_parent() {
                    let guard_if = parent.expect("parent").as_if();
                    let mut guard_exprent = guard_if.get_headexprent().get_condition();
                    let case_statements = stat.get_case_statements();
                    for i in 0..case_statements.len() {
                        if case_statements.get(i).contains_statement(reference.a) {
                            if simulate {
                                return true;
                            }
                            let guard_head = guard_if.get_stats().borrow().get(0);
                            let guard_exprs = guard_head.get_exprents().expect("exprents");
                            let carry_exprs: Vec<Exprent> = if !guard_exprs.is_empty() {
                                vec![guard_exprs.get(0)]
                            } else {
                                Vec::new()
                            };

                            continue_edge.remove();

                            let guard_parent = guard_if.get_parent().expect("parent");
                            guard_parent.get_stats().borrow_mut().remove_at(0);

                            let next_stat;

                            if invert {
                                let bytecode = guard_exprent.get_bytecode();
                                guard_exprent = FunctionExprent::new_single(
                                    FunctionType::BoolNot,
                                    guard_exprent,
                                    bytecode.as_ref(),
                                )
                                .0;
                                next_stat = guard_parent.get_stats().borrow().get(0);
                            } else {
                                next_stat = guard_if.get_ifstat().expect("ifstat");
                                guard_parent.get_stats().borrow_mut().remove_at(0);
                                guard_parent.get_stats().borrow_mut().add(next_stat);
                            }

                            guards.put(stat.get_case_values().get(i).to_vec(), guard_exprent);
                            guard_parent.set_first(next_stat);
                            for edge in next_stat.get_all_predecessor_edges() {
                                edge.remove();
                            }
                            next_stat
                                .get_basichead()
                                .get_exprents()
                                .expect("exprents")
                                .add_all_at(0, carry_exprs);
                            return true;
                        }
                    }
                }
            } else if parent == Some(stat.0) {
                let case_statements = stat.get_case_statements();
                for i in 0..case_statements.len() {
                    if case_statements.get(i).contains_statement(reference.a) {
                        if simulate {
                            return true;
                        }
                        guards.put(
                            stat.get_case_values().get(i).to_vec(),
                            ConstExprent::new_int(0, true, None).0,
                        );
                        let replaced = reference.a.replace_with_empty();
                        replaced.get_first_successor().remove();
                        let switch_parent = stat.get_parent().expect("parent");
                        let label_edges = switch_parent.get_label_edges().borrow().to_vec();

                        let mut multiple_successors = false;
                        let mut target: Option<Statement> = None;
                        for edge in label_edges {
                            if edge.get_type() != StatEdge::TYPE_BREAK {
                                continue;
                            }

                            match target {
                                None => target = Some(edge.get_destination()),
                                Some(t) if edge.get_destination() != t => {
                                    multiple_successors = true;
                                    break;
                                }
                                Some(_) => {}
                            }
                        }

                        if let Some(target) = target.filter(|_| !multiple_successors) {
                            replaced.add_successor(StatEdge::new_closure(
                                StatEdge::TYPE_BREAK,
                                replaced.0,
                                target,
                                stat.get_parent(),
                            ));
                        }
                        return true;
                    }
                }
            }
        }
        false
    }

    fn identify_switch_record_pattern_match(
        stat: SwitchStatement,
        statement: Statement,
        simulate: bool,
    ) -> Option<PatternExprent> {
        let head = statement.get_basichead();
        let exprents = head.get_exprents()?;
        if exprents.is_empty() {
            return None;
        }

        let first = exprents.get(0);

        let assignment = first.try_as_assignment()?;

        let assigned = assignment.get_left().try_as_var()?;
        let pattern = IfPatternMatchProcessor::identify_record_pattern_match(
            stat.0,
            statement,
            assigned.0,
            &assigned.get_expr_type(),
            simulate,
        );
        if pattern.is_some() && !simulate {
            head.get_exprents().unwrap().remove_at(0);
        }
        pattern
    }

    fn is_switch_pattern_match(head: SwitchHeadExprent) -> bool {
        let value = head.get_value();

        if let Some(invoc) = value.try_as_invocation() {
            return invoc.get_invocation_type() == InvocationType::Dynamic
                && (invoc.get_name() == "typeSwitch" || invoc.get_name() == "enumSwitch");
        }

        false
    }

    pub fn has_pattern_match(root: RootStatement) -> bool {
        root.mt().get_bytecode_version().has_switch_pattern_match()
            && DecompilerContext::get_option(IFernflowerPreferences::PATTERN_MATCHING)
    }

    fn is_pattern_matching_cast_assignment(
        switch_head: SwitchHeadExprent,
        assignment: AssignmentExprent,
    ) -> bool {
        let right = assignment.get_right();
        if let Some(switch_head_ref) = right.try_as_var() {
            if switch_head.contains_var(switch_head_ref.get_var_version_pair()) {
                return true;
            }
        } else if let Some(function_exprent) = right.try_as_function() {
            if function_exprent.get_func_type() != FunctionType::Cast {
                return false;
            }

            let lst_operands = function_exprent.get_lst_operands();
            if lst_operands.len() < 2 {
                return false;
            }
            let Some(switch_head_ref) = lst_operands.get(0).try_as_var() else {
                return false;
            };
            if !lst_operands.get(1).is_const() {
                return false;
            }
            if !switch_head.contains_var(switch_head_ref.get_var_version_pair()) {
                return false;
            }
            return true;
        }
        false
    }
}

/// `values.indexOf(list)`: the first case-value list equal by content (`List.equals`).
fn index_of_list(values: &JList<JList<Option<Exprent>>>, list: &JList<Option<Exprent>>) -> i32 {
    let wanted = list.to_vec();
    for (i, candidate) in values.iter().enumerate() {
        if wanted.java_equals(&candidate.to_vec()) {
            return i as i32;
        }
    }
    -1
}
