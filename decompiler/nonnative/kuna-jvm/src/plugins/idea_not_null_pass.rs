//! Port of `org/vineflower/ideanotnull/IdeaNotNullPass.java`: removes the
//! `$$$reportNull$$$0` checks IntelliJ IDEA compiles in for `@NotNull`
//! parameters and return values.

use crate::basic_block::BasicBlock;
use crate::basic_block_statement::BasicBlockStatement;
use crate::code_constants::ACC_STATIC;
use crate::code_type::CodeType;
use crate::counter_container::CounterContainer;
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::idea_not_null_options::IdeaNotNullOptions;
use crate::if_statement::IfStatement;
use crate::method_descriptor::MethodDescriptor;
use crate::pass::Pass;
use crate::pass_context::PassContext;
use crate::root_statement::RootStatement;
use crate::rt::JList;
use crate::sequence_helper::SequenceHelper;
use crate::statement::Statement;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_method::StructMethod;
use crate::validation_helper::ValidationHelper;

const NOT_NULL: &str = "org/jetbrains/annotations/NotNull";
const REPORT_NULL: &str = "$$$reportNull$$$0";

#[derive(Clone, Copy, Debug, Default)]
pub struct IdeaNotNullPass;

impl Pass for IdeaNotNullPass {
    fn run(&self, ctx: &mut PassContext) -> bool {
        if !DecompilerContext::get_option(IdeaNotNullOptions::IDEA_NOT_NULL_ANNOTATION) {
            return false;
        }

        IdeaNotNullPass::remove_hardcoded_checks(
            ctx.get_root().expect("NullPointerException: ctx.getRoot()"),
            ctx.get_method(),
        )
    }
}

/// `stat.getFirst()`, then down through nested sequences.
fn first_non_sequence(stat: Statement) -> Option<Statement> {
    let mut st = stat.get_first();
    while let Some(s) = st {
        if !s.is_sequence() {
            break;
        }
        st = s.get_first();
    }
    st
}

fn exprents(stat: Statement) -> JList<Exprent> {
    stat.get_exprents()
        .expect("NullPointerException: getExprents()")
}

impl IdeaNotNullPass {
    pub fn new() -> IdeaNotNullPass {
        IdeaNotNullPass
    }

    pub fn remove_hardcoded_checks(root: RootStatement, mt: StructMethod) -> bool {
        let mut checks_removed = false;

        while IdeaNotNullPass::find_and_remove_parameter_check(root.0, mt) {
            checks_removed = true;

            ValidationHelper::validate_statement(root);
        }

        while IdeaNotNullPass::find_and_remove_return_check(root.0, mt) {
            checks_removed = true;

            ValidationHelper::validate_statement(root);
        }

        checks_removed
    }

    fn find_and_remove_parameter_check(stat: Statement, mt: StructMethod) -> bool {
        let st = first_non_sequence(stat);

        if let Some(ifstat) = st.and_then(|s| s.try_as_if()) {
            let ifbranch = ifstat.get_ifstat();

            let if_condition = ifstat.get_headexprent().get_condition();

            let mut is_notnull_check = false;

            if let Some(ifbranch) = ifbranch.filter(|ifbranch| {
                if_condition
                    .try_as_function()
                    .is_some_and(|f| f.get_func_type() == FunctionType::Eq)
                    && ifbranch.is_basic_block()
                    && exprents(*ifbranch).len() == 1
                    && exprents(*ifbranch).get(0).is_invocation()
            }) {
                let func = if_condition.as_function();
                let first_param = func.get_lst_operands().get(0);
                let second_param = func.get_lst_operands().get(1);

                if second_param.is_const() && second_param.get_expr_type().type_ == CodeType::Null {
                    if let Some(var) = first_param.try_as_var().filter(|_| {
                        exprents(ifbranch).get(0).as_invocation().get_name() == REPORT_NULL
                    }) {
                        let thisvar = !mt.has_modifier(ACC_STATIC);

                        let md = MethodDescriptor::parse_descriptor(&mt.get_descriptor());

                        let param_annotations = mt.get_attribute(
                            StructGeneralAttribute::ATTRIBUTE_RUNTIME_INVISIBLE_PARAMETER_ANNOTATIONS,
                        );
                        if let Some(param_annotations) = param_annotations {
                            let param_annotations_lists = param_annotations.get_param_annotations();
                            let method_param_number = md.params.len() as i32;

                            let mut index: i32 = if thisvar { 1 } else { 0 };
                            for i in 0..method_param_number {
                                if index == var.get_index() {
                                    if param_annotations_lists.len() as i32
                                        >= method_param_number - i
                                    {
                                        let shift = method_param_number
                                            - param_annotations_lists.len() as i32;

                                        let annotations =
                                            &param_annotations_lists[(i - shift) as usize];

                                        for ann in annotations {
                                            if ann.get_class_name() == NOT_NULL {
                                                is_notnull_check = true;
                                                break;
                                            }
                                        }
                                    }

                                    break;
                                }

                                index += md.params[i as usize].stack_size;
                            }
                        }
                    }
                }
            }

            if !is_notnull_check {
                return false;
            }

            IdeaNotNullPass::remove_parameter_check(stat);

            return true;
        }

        false
    }

    fn remove_parameter_check(stat: Statement) {
        let ifstat = first_non_sequence(stat)
            .expect("NullPointerException: statement")
            .as_if();

        if ifstat.get_elsestat().is_none() {
            let ifedge = ifstat.get_if_edge();

            let ifbranch = ifstat.get_ifstat().expect("NullPointerException: ifstat");

            if let Some(ifedge) = ifedge {
                ifstat
                    .get_first()
                    .expect("NullPointerException: first")
                    .remove_successor(ifedge);
            }

            ifstat
                .get_stats()
                .borrow_mut()
                .remove_with_key(&ifbranch.id());

            if ifbranch.has_any_successor() {
                ifbranch.remove_successor(ifbranch.get_first_successor());
            }

            let block = BasicBlockStatement::new(BasicBlock::new(
                DecompilerContext::get_counter_container()
                    .get_counter_and_increment(CounterContainer::STATEMENT_COUNTER),
            ));
            block.set_exprents(JList::new());

            let parent = ifstat.get_parent().expect("NullPointerException: parent");
            parent.replace_statement(ifstat.0, block.0);
            ifstat
                .get_parent()
                .expect("NullPointerException: parent")
                .set_all_parent();

            SequenceHelper::condense_sequences(stat.get_top_parent().0);
        }
    }

    fn find_and_remove_return_check(stat: Statement, mt: StructMethod) -> bool {
        let mut is_notnull_check = false;

        let attr =
            mt.get_attribute(StructGeneralAttribute::ATTRIBUTE_RUNTIME_INVISIBLE_ANNOTATIONS);
        if let Some(attr) = attr {
            let annotations = attr.get_annotations();

            for ann in annotations {
                if ann.get_class_name() == NOT_NULL {
                    is_notnull_check = true;
                    break;
                }
            }
        }

        is_notnull_check && IdeaNotNullPass::remove_return_check(stat, mt)
    }

    #[allow(clippy::only_used_in_recursion)]
    fn remove_return_check(stat: Statement, mt: StructMethod) -> bool {
        let parent = stat.get_parent();

        if parent.is_some_and(|p| p.is_if()) && stat.is_basic_block() && exprents(stat).len() == 1 {
            let exprent = exprents(stat).get(0);
            if let Some(exit_exprent) = exprent.try_as_exit() {
                if exit_exprent.get_exit_type() == ExitType::Return {
                    let exprent_value = exit_exprent.get_value();

                    let ifparent = parent.unwrap().as_if();
                    let if_condition = ifparent.get_headexprent().get_condition();

                    if ifparent.get_elsestat() == Some(stat)
                        && if_condition
                            .try_as_function()
                            .is_some_and(|f| f.get_func_type() == FunctionType::Eq)
                    {
                        let func = if_condition.as_function();
                        let first_param = func.get_lst_operands().get(0);
                        let second_param = func.get_lst_operands().get(1);

                        let ifedge = ifparent.get_if_edge();
                        let elseedge = ifparent.get_else_edge();

                        let ifbranch = ifparent.get_ifstat();
                        let elsebranch = ifparent.get_elsestat();

                        if second_param.is_const()
                            && second_param.get_expr_type().type_ == CodeType::Null
                            && exprent_value.is_some_and(|v| first_param.equals(v))
                        {
                            if let Some(ifbranch) = ifbranch.filter(|ifbranch| {
                                ifbranch.is_basic_block()
                                    && exprents(*ifbranch).len() == 1
                                    && exprents(*ifbranch).get(0).is_exit()
                            }) {
                                let elsebranch =
                                    elsebranch.expect("NullPointerException: elsestat");
                                let first =
                                    ifparent.get_first().expect("NullPointerException: first");

                                if let Some(ifedge) = ifedge {
                                    first.remove_successor(ifedge);
                                }
                                if let Some(elseedge) = elseedge {
                                    first.remove_successor(elseedge);
                                }

                                ifparent
                                    .get_stats()
                                    .borrow_mut()
                                    .remove_with_key(&ifbranch.id());
                                ifparent
                                    .get_stats()
                                    .borrow_mut()
                                    .remove_with_key(&elsebranch.id());

                                if ifbranch.has_any_successor() {
                                    ifbranch.remove_successor(ifbranch.get_first_successor());
                                }

                                let first_exprents = exprents(first);
                                if !first_exprents.is_empty() {
                                    exprents(elsebranch).add_all_at(0, first_exprents.to_vec());
                                }

                                ifparent
                                    .get_parent()
                                    .expect("NullPointerException: parent")
                                    .replace_statement(ifparent.0, elsebranch);
                                ifparent
                                    .get_parent()
                                    .expect("NullPointerException: parent")
                                    .set_all_parent();

                                return true;
                            }
                        }
                    }
                }
            }
        } else if parent.is_some_and(|p| p.is_sequence())
            && stat.is_basic_block()
            && exprents(stat).len() == 1
        {
            let exprent = exprents(stat).get(0);
            if let Some(exit_exprent) = exprent.try_as_exit() {
                if exit_exprent.get_exit_type() == ExitType::Return {
                    let exprent_value = exit_exprent.get_value();

                    let sequence = parent.unwrap().as_sequence();
                    let sequence_stats_number = sequence.get_stats().borrow().size();

                    if sequence_stats_number > 1
                        && sequence.get_stats().borrow().get_last() == stat
                        && sequence
                            .get_stats()
                            .borrow()
                            .get(sequence_stats_number - 2)
                            .is_if()
                    {
                        let ifstat: IfStatement = sequence
                            .get_stats()
                            .borrow()
                            .get(sequence_stats_number - 2)
                            .as_if();
                        let if_condition = ifstat.get_headexprent().get_condition();

                        if ifstat.iftype() == IfStatement::IFTYPE_IF
                            && if_condition
                                .try_as_function()
                                .is_some_and(|f| f.get_func_type() == FunctionType::Eq)
                        {
                            let func = if_condition.as_function();
                            let first_param = func.get_lst_operands().get(0);
                            let second_param = func.get_lst_operands().get(1);

                            let ifbranch = ifstat.get_ifstat();

                            if second_param.is_const()
                                && second_param.get_expr_type().type_ == CodeType::Null
                                && exprent_value.is_some_and(|v| first_param.equals(v))
                            {
                                if let Some(ifbranch) = ifbranch.filter(|ifbranch| {
                                    ifbranch.is_basic_block()
                                        && exprents(*ifbranch).len() == 1
                                        && exprents(*ifbranch).get(0).is_invocation()
                                        && exprents(*ifbranch).get(0).as_invocation().get_name()
                                            == REPORT_NULL
                                }) {
                                    ifstat.get_first_successor().remove();

                                    let first_exprents = exprents(
                                        ifstat.get_first().expect("NullPointerException: first"),
                                    );
                                    if !first_exprents.is_empty() {
                                        exprents(stat).add_all_at(0, first_exprents.to_vec());
                                    }

                                    for edge in ifstat.get_all_predecessor_edges() {
                                        edge.change_destination(stat);
                                    }

                                    ifbranch.get_first_successor().remove();

                                    sequence
                                        .get_stats()
                                        .borrow_mut()
                                        .remove_with_key(&ifstat.id());
                                    let first = sequence.get_stats().borrow().get(0);
                                    sequence.set_first(first);

                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            if IdeaNotNullPass::remove_return_check(st, mt) {
                return true;
            }
        }

        false
    }
}
