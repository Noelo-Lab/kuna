//! Port of `org/jetbrains/java/decompiler/modules/decompiler/AssertProcessor.java`.

use crate::assert_exprent::AssertExprent;
use crate::basic_block_statement::BasicBlockStatement;
use crate::class_wrapper::ClassWrapper;
use crate::classes_processor::ClassNodeType;
use crate::code_constants::ACC_INTERFACE;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent;
use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::if_statement::IfStatement;
use crate::interpreter_util::InterpreterUtil;
use crate::invocation_exprent::InvocationExprent;
use crate::jthrow;
use crate::root_statement::RootStatement;
use crate::rt::JList;
use crate::sequence_helper::SequenceHelper;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::var_type::VarType;

pub struct AssertProcessor;

struct AssertResult {
    exprent: Option<Exprent>,
    valid: bool,
    replace_parent: bool,
    negate: bool,
    assignment: bool,
}

impl AssertResult {
    fn new(exprent: Option<Exprent>, valid: bool) -> AssertResult {
        AssertResult {
            exprent,
            valid,
            replace_parent: false,
            negate: true,
            assignment: false,
        }
    }
}

impl AssertProcessor {
    #[allow(non_snake_case)]
    fn CLASS_ASSERTION_ERROR() -> VarType {
        thread_local! {
            static CLASS_ASSERTION_ERROR: VarType =
                VarType::new_value(CodeType::Object, 0, "java/lang/AssertionError");
        }
        CLASS_ASSERTION_ERROR.with(|t| t.clone())
    }

    pub fn build_assertions(root: RootStatement) -> bool {
        if !DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_ASSERTIONS) {
            return false;
        }

        let wrapper = Self::get_holding_class();
        let prop = wrapper.get_method_properties("<clinit>", "()V");

        if let Some(assert_field) = prop.and_then(|p| p.assert_field) {
            let key = InterpreterUtil::make_unique_key(
                &assert_field.get_name(),
                &assert_field.get_descriptor(),
            );

            let mut res = false;
            let qualified_name = wrapper.get_class_struct().qualified_name();
            if root.mt().get_name() == "<clinit>" {
                res |= Self::clean_interface_static_initializer(root.0, &qualified_name, &key);
            }

            res |= Self::replace_assertions(root.0, &qualified_name, &key);

            if res {
                SequenceHelper::condense_sequences(root.0);
                return true;
            }
        }

        false
    }

    pub fn get_holding_class() -> ClassWrapper {
        let wrapper =
            DecompilerContext::get_current_class_wrapper().expect("CURRENT_CLASS_WRAPPER");

        if wrapper.get_class_struct().has_modifier(ACC_INTERFACE) {
            let mut node = DecompilerContext::get_current_class_node().expect("CURRENT_CLASS_NODE");
            while node.with(|d| d.type_) != ClassNodeType::Root {
                node = node.with(|d| d.parent).expect("parent");
            }

            let nested = node.with(|d| d.nested.clone());
            for nd in nested.iter() {
                if let Some(nd_wrapper) = nd.get_wrapper() {
                    if nd_wrapper.get_class_struct().is_synthetic() {
                        let props = nd_wrapper.get_method_properties("<clinit>", "()V");

                        if props.is_some_and(|p| p.assert_field.is_some()) {
                            return nd_wrapper;
                        }
                    }
                }
            }
        }
        wrapper
    }

    fn clean_interface_static_initializer(stat: Statement, class_name: &str, key: &str) -> bool {
        let mut res = false;
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            res |= Self::clean_interface_static_initializer(st, class_name, key);
        }

        if let Some(if_st) = stat.try_as_if().filter(|i| i.get_ifstat().is_none()) {
            let condition = if_st.get_headexprent().get_condition();
            if let Some(func) = condition
                .try_as_function()
                .filter(|f| f.get_func_type() == FunctionType::Ne)
            {
                if let Some(field) = func.get_lst_operands().get(0).try_as_field() {
                    if class_name == field.get_classname()
                        && key
                            == InterpreterUtil::make_unique_key(
                                &field.get_name(),
                                &field.get_descriptor().descriptor_string,
                            )
                    {
                        if let Some(con) = func.get_lst_operands().get(1).try_as_const() {
                            if con.get_value().is_some() && con.get_int_value() == 0 {
                                stat.replace_with_empty();
                                res = true;
                            }
                        }
                    }
                }
            }
        }

        res
    }

    fn replace_assertions(statement: Statement, classname: &str, key: &str) -> bool {
        let mut res = false;

        let stats = statement.get_stats().borrow().to_vec();
        for st in stats {
            res |= Self::replace_assertions(st, classname, key);
        }

        let mut replaced = true;
        while replaced {
            replaced = false;

            let stats = statement.get_stats().borrow().to_vec();
            for st in stats {
                if st.is_if() && st.get_parent().is_some() {
                    if Self::replace_assertion(statement, st.as_if(), classname, key) {
                        replaced = true;
                        break;
                    }
                }
            }

            res |= replaced;
        }

        res
    }

    fn replace_assertion(parent: Statement, stat: IfStatement, classname: &str, key: &str) -> bool {
        let mut parent = parent;
        let mut throw_in_if = true;
        let mut ifstat = stat.get_ifstat();
        if ifstat.is_none() && stat.get_if_edge().unwrap().get_type() == StatEdge::TYPE_BREAK {
            ifstat = Some(stat.get_if_edge().unwrap().get_destination());
        }
        let mut throw_error = Self::is_assertion_error(ifstat);

        if throw_error.is_none() {
            let elsestat = stat.get_elsestat();
            throw_error = Self::is_assertion_error(elsestat);

            if throw_error.is_none() {
                return false;
            } else {
                throw_in_if = false;
                ifstat = elsestat;
            }
        }
        let throw_error = throw_error.unwrap();

        let res = Self::get_assertion_exprent(
            stat.0,
            stat.get_headexprent().get_condition().copy(),
            classname,
            key,
            throw_in_if,
        );
        if !res.valid {
            return false;
        }

        let mut lst_params: Vec<Option<Exprent>> = Vec::new();

        let mut ascond: Option<Exprent> = None;
        let mut retcond: Option<Exprent> = None;
        if let Some(res_exprent) = res.exprent {
            ascond = Some(if res.negate {
                FunctionExprent::new_single(
                    FunctionType::BoolNot,
                    res_exprent,
                    throw_error.get_bytecode().as_ref(),
                )
                .0
            } else {
                res_exprent
            });
            retcond = ascond;
        }

        lst_params.push(if retcond.is_none() { ascond } else { retcond });
        if !throw_error.get_lst_parameters().is_empty() {
            lst_params.push(Some(throw_error.get_lst_parameters().get(0)));
        }

        let asexpr = AssertExprent::new(lst_params);

        let mut newstat: Statement = BasicBlockStatement::create().0;
        let block = newstat;
        let exprs: JList<Exprent> = JList::new();
        exprs.add(asexpr.0);
        newstat.set_exprents(exprs);

        let first = stat.get_first().unwrap();

        let iftype = stat.iftype();
        if iftype == IfStatement::IFTYPE_IFELSE
            || (!res.assignment && first.get_exprents().is_some_and(|e| !e.is_empty()))
        {
            if let Some(if_edge) = stat.get_if_edge() {
                first.remove_successor(if_edge);
            }
            if let Some(else_edge) = stat.get_else_edge() {
                first.remove_successor(else_edge);
            }

            let mut lst_statements: Vec<Statement> = Vec::new();
            if first.get_exprents().is_some_and(|e| !e.is_empty()) {
                lst_statements.push(first);
            }

            lst_statements.push(newstat);
            if iftype == IfStatement::IFTYPE_IFELSE {
                if throw_in_if {
                    lst_statements.push(stat.get_elsestat().unwrap());
                } else {
                    lst_statements.push(stat.get_ifstat().unwrap());
                }
            }

            let sequence = SequenceStatement::new(lst_statements);
            sequence.set_all_parent();

            let seq_stats = sequence.get_stats().borrow().to_vec();
            for i in 0..seq_stats.len() - 1 {
                seq_stats[i].add_successor(StatEdge::new(
                    StatEdge::TYPE_REGULAR,
                    seq_stats[i],
                    seq_stats[i + 1],
                ));
            }

            if iftype == IfStatement::IFTYPE_IFELSE || !throw_in_if {
                let stmts = if throw_in_if {
                    stat.get_elsestat()
                } else {
                    stat.get_ifstat()
                };

                let lst_succs = stmts.unwrap().get_all_successor_edges();
                if !lst_succs.is_empty() {
                    let endedge = lst_succs[0];
                    if endedge.closure() == Some(stat.0) {
                        sequence.add_labeled_edge(endedge);
                    }
                }
            }

            newstat = sequence.0;
        }

        for edge in ifstat.unwrap().get_all_successor_edges() {
            edge.remove();
        }

        for edge in stat.get_all_successor_edges() {
            if edge.get_type() == StatEdge::TYPE_BREAK {
                edge.change_source(block);
            }
        }

        newstat
            .get_var_definitions()
            .add_all(stat.get_var_definitions().to_vec());

        if res.assignment {
            for edge in stat.get_all_successor_edges() {
                edge.remove();
            }

            stat.set_parent(None);

            parent.replace_with(newstat);
        } else {
            parent.replace_statement(stat.0, newstat);
        }

        if res.replace_parent {
            if !parent
                .get_parent()
                .unwrap()
                .get_basichead()
                .get_exprents()
                .unwrap()
                .is_empty()
            {
                let p_parent = parent.get_parent();
                parent = SequenceStatement::new(vec![
                    parent.get_parent().unwrap().get_basichead().0,
                    parent,
                ])
                .0;
                parent.set_parent(p_parent);
            }
            parent.get_parent().unwrap().replace_with(parent);
        }

        true
    }

    fn is_assertion_error(stat: Option<Statement>) -> Option<InvocationExprent> {
        let stat = stat?;
        let exprents = stat.get_exprents()?;
        if exprents.len() != 1 {
            return None;
        }

        let expr = exprents.get(0);

        if let Some(exexpr) = expr.try_as_exit() {
            if exexpr.get_exit_type() == exit_exprent::Type::Throw {
                if let Some(nexpr) = exexpr.get_value().and_then(|v| v.try_as_new()) {
                    if Self::CLASS_ASSERTION_ERROR().equals(&nexpr.get_new_type()) {
                        if let Some(constructor) = nexpr.get_constructor() {
                            return Some(constructor);
                        }
                    }
                }
            }
        }

        None
    }

    fn get_assertion_exprent(
        stat: Statement,
        exprent: Exprent,
        classname: &str,
        key: &str,
        throw_in_if: bool,
    ) -> AssertResult {
        let mut exprent = exprent;
        if let Some(fexpr) = exprent.try_as_function() {
            if !throw_in_if {
                if fexpr.get_func_type() == FunctionType::BoolNot {
                    exprent = fexpr.get_lst_operands().get(0);
                }
            } else if fexpr.get_func_type() == FunctionType::BoolNot {
                if let Some(func) = fexpr
                    .get_lst_operands()
                    .get(0)
                    .try_as_function()
                    .filter(|f| f.get_func_type() == FunctionType::BoolNot)
                {
                    if let Some(f) = func.get_lst_operands().get(0).try_as_function() {
                        exprent = f.0;
                    }
                }
            }

            let fexpr = exprent.as_function();
            if fexpr.get_func_type() == FunctionType::BooleanAnd {
                for i in 0..2 {
                    let param = fexpr.get_lst_operands().get(i);

                    if Self::is_assertion_field(param, classname, key) {
                        return AssertResult::new(Some(fexpr.get_lst_operands().get(1 - i)), true);
                    }
                }

                for i in 0..2 {
                    let param = fexpr.get_lst_operands().get(i);

                    let res = Self::get_assertion_exprent(stat, param, classname, key, throw_in_if);
                    if res.valid {
                        if res.exprent != Some(param) {
                            match res.exprent {
                                Some(e) => {
                                    fexpr.get_lst_operands().set(i, e);
                                }
                                None => jthrow!(
                                    NullPointerException;
                                    "AssertProcessor: null operand set into a BOOLEAN_AND"
                                ),
                            }
                        }

                        return AssertResult::new(Some(fexpr.0), true);
                    }
                }
            } else if Self::is_assertion_field(fexpr.0, classname, key) {
                return AssertResult::new(None, true);
            } else if fexpr.get_func_type() == FunctionType::Eq {
                let parent = Self::parent_skipping_sequences(stat);
                if let Some(parent_if) = parent.and_then(|p| p.try_as_if()) {
                    let head_exprents = stat.get_basichead().get_exprents().unwrap();
                    if head_exprents.is_empty() {
                        if Self::is_assertion_field(
                            parent_if.get_headexprent().get_condition(),
                            classname,
                            key,
                        ) {
                            let mut res = AssertResult::new(Some(fexpr.0), true);
                            res.replace_parent = true;
                            return res;
                        }
                    } else if head_exprents.len() == 1 {
                        if let Some(assign) = head_exprents.get(0).try_as_assignment() {
                            let right = assign.get_right();
                            if fexpr.get_lst_operands().get(0).equals(right) {
                                if let Some(con) = fexpr.get_lst_operands().get(1).try_as_const() {
                                    if con.get_value().is_some() && con.get_int_value() == 0 {
                                        let mut res = AssertResult::new(Some(assign.0), true);
                                        res.negate = false;
                                        res.assignment = true;
                                        return res;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        AssertResult::new(Some(exprent), false)
    }

    fn parent_skipping_sequences(stat: Statement) -> Option<Statement> {
        let mut stat = stat.get_parent();

        while let Some(s) = stat.filter(|s| s.is_sequence()) {
            stat = s.get_parent();
        }

        stat
    }

    fn is_assertion_field(exprent: Exprent, classname: &str, key: &str) -> bool {
        if let Some(mut fparam) = exprent.try_as_function() {
            if fparam.get_func_type() == FunctionType::BoolNot {
                if let Some(func) = fparam
                    .get_lst_operands()
                    .get(0)
                    .try_as_function()
                    .filter(|f| f.get_func_type() == FunctionType::BoolNot)
                {
                    if let Some(f) = func.get_lst_operands().get(0).try_as_function() {
                        fparam = f;
                    }
                }
            }

            if fparam.get_func_type() == FunctionType::Eq {
                if let Some(fdparam) = fparam.get_lst_operands().get(0).try_as_field() {
                    if let Some(con) = fparam.get_lst_operands().get(1).try_as_const() {
                        if con.get_value().is_some() && con.get_int_value() == 0 {
                            return classname == fdparam.get_classname()
                                && key
                                    == InterpreterUtil::make_unique_key(
                                        &fdparam.get_name(),
                                        &fdparam.get_descriptor().descriptor_string,
                                    );
                        }
                    }
                }
            }
        }

        false
    }
}
