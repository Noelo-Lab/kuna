//! Port of `org/jetbrains/java/decompiler/modules/decompiler/PPandMMHelper.java`.
//!
//! kuna fix `compound_assignment_shape` (`kuna_fix_compound_assignment_shape`):
//! `x.f = x.f + 1` that loads `x` twice is not written `++x.f`.
//! kuna option `jvm_source_locals` (`kuna_source_increments`): `inlinePPIandMMIIf`
//! keeps an increment the source wrote as a statement before the `if`.

use std::collections::VecDeque;

use crate::direct_edge_type::DirectEdgeType;
use crate::direct_graph::DirectGraph;
use crate::direct_node::DirectNode;
use crate::exprent::Exprent;
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::if_statement::IfStatement;
use crate::p3_dataflow::kuna_source_increments;
use crate::p3_dataflow::kuna_fix_compound_assignment_shape;
use crate::root_statement::RootStatement;
use crate::rt::{JHashSet, JList};
use crate::sequence_helper::SequenceHelper;
use crate::statement::Statement;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;
use crate::var_version_pair::VarVersionPair;

/// Folds `x = x + 1` into `++x`; one instance per pass, never stored.
pub struct PPandMMHelper {
    exprent_replaced: bool,
    var_proc: VarProcessor,
    dgraph: Option<DirectGraph>,
}

/// Java `==` on two `Integer`s boxed by `Integer.valueOf`: only the cached
/// values -128..=127 share an object.
fn boxed_integer_identity_eq(a: Option<i32>, b: Option<i32>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => a == b && (-128..=127).contains(&a),
        _ => false,
    }
}

impl PPandMMHelper {
    pub fn new(var_proc: VarProcessor) -> PPandMMHelper {
        PPandMMHelper {
            exprent_replaced: false,
            var_proc,
            dgraph: None,
        }
    }

    pub fn find_p_pand_mm(&mut self, root: RootStatement) -> bool {
        let mut flatthelper = FlattenStatementsHelper::new();
        let dgraph = flatthelper.build_direct_graph(root);
        self.dgraph = Some(dgraph);

        let mut stack: VecDeque<DirectNode> = VecDeque::new();
        stack.push_back(dgraph.first().expect("NullPointerException"));

        let mut set_visited: JHashSet<DirectNode> = JHashSet::new();

        let mut res = false;

        while let Some(node) = stack.pop_front() {
            if set_visited.contains(&node) {
                continue;
            }
            set_visited.add(node);

            res |= self.process_exprent_list(&node.exprents());

            for suc in node.get_successors(DirectEdgeType::Regular).iter() {
                stack.push_back(suc.get_destination());
            }
        }

        res
    }

    fn process_exprent_list(&mut self, lst: &JList<Exprent>) -> bool {
        let mut result = false;

        let mut i: i32 = 0;
        while i < lst.len() as i32 {
            let exprent = lst.get(i as usize);
            self.exprent_replaced = false;

            let retexpr = self.process_exprent_recursive(exprent);
            if let Some(retexpr) = retexpr {
                lst.set(i as usize, retexpr);

                result = true;
                i -= 1;
            }

            result |= self.exprent_replaced;
            i += 1;
        }

        result
    }

    fn process_exprent_recursive(&mut self, exprent: Exprent) -> Option<Exprent> {
        let mut replaced = true;
        while replaced {
            replaced = false;

            for expr in exprent.get_all_exprents() {
                let retexpr = self.process_exprent_recursive(expr);
                if let Some(retexpr) = retexpr {
                    exprent.replace_exprent(expr, retexpr);
                    retexpr.add_bytecode_offsets(expr.get_bytecode().as_ref());
                    replaced = true;
                    self.exprent_replaced = true;
                    break;
                }
            }
        }

        if let Some(as_) = exprent.try_as_assignment() {
            if let Some(mut func) = as_.get_right().try_as_function() {
                let midlayer = func.get_func_type().cast_type();
                if midlayer.is_some() {
                    if let Some(inner) = func.get_lst_operands().get(0).try_as_function() {
                        func = inner;
                    } else {
                        return None;
                    }
                }

                if func.get_func_type() == FunctionType::Add
                    || func.get_func_type() == FunctionType::Sub
                {
                    let mut econd = func.get_lst_operands().get(0);
                    let mut econst = func.get_lst_operands().get(1);

                    if !econst.is_const()
                        && econd.is_const()
                        && func.get_func_type() == FunctionType::Add
                    {
                        econd = econst;
                        econst = func.get_lst_operands().get(0);
                    }

                    if econst.is_const() && econst.as_const().has_value_one() {
                        let left = as_.get_left();

                        let condtype = left.get_expr_type();
                        if self.exprs_equal(Some(left), Some(econd))
                            && midlayer.as_ref().is_none_or(|m| m.equals(&condtype))
                            && !kuna_fix_compound_assignment_shape::loads_twice(left, econd)
                        {
                            let ret = FunctionExprent::new_single(
                                if func.get_func_type() == FunctionType::Add {
                                    FunctionType::Ppi
                                } else {
                                    FunctionType::Mmi
                                },
                                econd,
                                func.get_bytecode().as_ref(),
                            );
                            ret.set_implicit_type(Some(condtype));

                            self.exprent_replaced = true;

                            if !left.equals(econd) {
                                self.update_versions(
                                    self.dgraph.expect("NullPointerException"),
                                    VarVersionPair::from_var(left.as_var()),
                                    VarVersionPair::from_var(econd.as_var()),
                                );
                            }

                            return Some(ret.0);
                        }
                    }
                }
            }
        }

        None
    }

    fn exprs_equal(&self, e1: Option<Exprent>, e2: Option<Exprent>) -> bool {
        if e1 == e2 {
            return true;
        }
        let (Some(e1), Some(e2)) = (e1, e2) else {
            return false;
        };
        if e1.is_var() {
            return self.vars_equal(e1, e2);
        }
        e1.equals(e2)
    }

    fn vars_equal(&self, e1: Exprent, e2: Exprent) -> bool {
        let Some(v1) = e1.try_as_var() else {
            return false;
        };
        let Some(v2) = e2.try_as_var() else {
            return false;
        };
        boxed_integer_identity_eq(
            self.var_proc.get_var_original_index(v1.get_index()),
            self.var_proc.get_var_original_index(v2.get_index()),
        )
    }

    fn update_versions(
        &self,
        graph: DirectGraph,
        old_vvp: VarVersionPair,
        new_vvp: VarVersionPair,
    ) {
        graph.iterate_exprents(|exprent: Exprent| {
            let mut lst = exprent.get_all_exprents_rec(true);
            lst.push(exprent);

            for expr in lst {
                if let Some(var) = expr.try_as_var() {
                    if var.get_index() == old_vvp.var && var.get_version() == old_vvp.version {
                        var.set_index(new_vvp.var);
                        var.set_version(new_vvp.version);
                    }
                }
            }

            0
        });
    }

    pub fn inline_pp_iand_mmi_if(stat: RootStatement) -> bool {
        let res = Self::inline_pp_iand_mmi_if_rec(stat.0);

        if res {
            SequenceHelper::condense_sequences(stat.0);
        }

        res
    }

    fn inline_pp_iand_mmi_if_rec(stat: Statement) -> bool {
        let mut res = false;
        let sub_stats = stat.get_stats().borrow().to_vec();
        for st in sub_stats {
            res |= Self::inline_pp_iand_mmi_if_rec(st);
        }

        let Some(exprents) = stat.get_exprents() else {
            return res;
        };
        if exprents.is_empty() {
            return res;
        }

        let Some(destination) = Self::find_if_successor(stat) else {
            return res;
        };

        let expr = exprents.get(exprents.len() - 1);
        let Some(func) = expr.try_as_function() else {
            return res;
        };

        if func.get_func_type() != FunctionType::Ppi && func.get_func_type() != FunctionType::Mmi {
            return res;
        }

        let inner = func.get_lst_operands().get(0);
        let Some(inner_var) = inner.try_as_var() else {
            return res;
        };

        let if_expr = destination.get_headexprent().get_condition();
        let Some(mut if_func) = if_expr.try_as_function() else {
            return res;
        };

        while if_func.get_func_type() == FunctionType::BoolNot {
            let inner_func = if_func.get_lst_operands().get(0);

            if let Some(inner_func) = inner_func.try_as_function() {
                if_func = inner_func;
            } else {
                break;
            }
        }

        let mut found = false;
        let mut old: Option<VarExprent> = None;
        for ex in if_func.get_all_exprents_rec(true) {
            if let Some(var) = ex.try_as_var() {
                if var.get_index() == inner_var.get_index() {
                    if found {
                        return false;
                    }

                    old = Some(var);
                    found = true;
                }
            } else if let Some(func_ex) = ex.try_as_function() {
                if func_ex.get_func_type() == FunctionType::BooleanAnd
                    || func_ex.get_func_type() == FunctionType::BooleanOr
                {
                    return false;
                }
            }
        }

        if found && !kuna_source_increments::own_statement(expr, inner_var, if_expr) {
            let old = old.expect("old");
            let mut stack: Vec<Exprent> = Vec::new();
            stack.push(if_func.0);

            while let Some(exprent) = stack.pop() {
                for ex in exprent.get_all_exprents() {
                    if ex == old.0 {
                        exprent.replace_exprent(old.0, expr);
                        expr.add_bytecode_offsets(old.get_bytecode().as_ref());

                        stack.clear();
                        break;
                    }

                    stack.push(ex);
                }
            }

            exprents.remove_item(&expr);
            res = true;

            destination.set_has_ppmm(true);
        }

        res
    }

    fn find_if_successor(stat: Statement) -> Option<IfStatement> {
        if let Some(parent) = stat.get_parent() {
            if let Some(if_parent) = parent.try_as_if() {
                if parent.get_first() == Some(stat) {
                    return Some(if_parent);
                }
            }
        }

        None
    }
}
