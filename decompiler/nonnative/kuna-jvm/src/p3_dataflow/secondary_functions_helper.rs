//! Port of `org/jetbrains/java/decompiler/modules/decompiler/SecondaryFunctionsHelper.java`.
//!
//! kuna fix `compound_assignment_shape` (`kuna_fix_compound_assignment_shape`):
//! a field or array element assignment is made compound by the bytecode's
//! shape, not by Vineflower's side-effect heuristic.

use crate::assignment_exprent::AssignmentExprent;
use crate::code_type::CodeType;
use crate::concatenation_helper::ConcatenationHelper;
use crate::const_exprent::{ConstExprent, ConstValue};
use crate::counter_container::CounterContainer;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::{Exprent, ExprentType};
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::if_statement::IfStatement;
use crate::kuna_fixes::{self, KunaFix};
use crate::p3_dataflow::kuna_fix_compound_assignment_shape;
use crate::rt::JHashMap;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::type_family::TypeFamily;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

pub struct SecondaryFunctionsHelper;

const DEFAULT_OPTIONS: IdentifySecondaryOptions = IdentifySecondaryOptions {
    force_ternary_simplification: false,
};

thread_local! {
    static FUNCSNOT: JHashMap<FunctionType, FunctionType> = {
        let mut map = JHashMap::new();
        map.put(FunctionType::Eq, FunctionType::Ne);
        map.put(FunctionType::Ne, FunctionType::Eq);
        map.put(FunctionType::Lt, FunctionType::Ge);
        map.put(FunctionType::Ge, FunctionType::Lt);
        map.put(FunctionType::Gt, FunctionType::Le);
        map.put(FunctionType::Le, FunctionType::Gt);
        map.put(FunctionType::BooleanAnd, FunctionType::BooleanOr);
        map.put(FunctionType::BooleanOr, FunctionType::BooleanAnd);
        map
    };

    static MAP_NUM_COMPARISONS: JHashMap<FunctionType, [Option<FunctionType>; 3]> = {
        let mut map = JHashMap::new();
        map.put(
            FunctionType::Eq,
            [Some(FunctionType::Lt), Some(FunctionType::Eq), Some(FunctionType::Gt)],
        );
        map.put(
            FunctionType::Ne,
            [Some(FunctionType::Ge), Some(FunctionType::Ne), Some(FunctionType::Le)],
        );
        map.put(FunctionType::Gt, [Some(FunctionType::Ge), Some(FunctionType::Gt), None]);
        map.put(FunctionType::Ge, [None, Some(FunctionType::Ge), Some(FunctionType::Gt)]);
        map.put(FunctionType::Lt, [None, Some(FunctionType::Lt), Some(FunctionType::Le)]);
        map.put(FunctionType::Le, [Some(FunctionType::Lt), Some(FunctionType::Le), None]);
        map
    };
}

/// `funcsnot.get(type)`, which every caller dereferences.
#[track_caller]
fn funcsnot(type_: FunctionType) -> FunctionType {
    FUNCSNOT
        .with(|m| m.get_cloned(&type_))
        .expect("NullPointerException: funcsnot")
}

/// `fexpr.getSimpleCastType() != null && !fexpr.doesCast()`.
fn is_implicit_cast(exprent: Exprent) -> Option<FunctionExprent> {
    exprent
        .try_as_function()
        .filter(|f| f.get_simple_cast_type().is_some() && !f.does_cast())
}

#[derive(Clone, Copy, Debug, Default)]
pub struct IdentifySecondaryOptions {
    force_ternary_simplification: bool,
}

impl IdentifySecondaryOptions {
    pub fn new() -> IdentifySecondaryOptions {
        IdentifySecondaryOptions {
            force_ternary_simplification: false,
        }
    }

    pub fn force_ternary_simplification(mut self) -> IdentifySecondaryOptions {
        self.force_ternary_simplification = true;
        self
    }
}

impl SecondaryFunctionsHelper {
    pub fn identify_secondary_functions(stat: Statement, var_proc: VarProcessor) -> bool {
        Self::identify_secondary_functions_options(stat, var_proc, DEFAULT_OPTIONS)
    }

    /// Java: identifySecondaryFunctions(Statement, VarProcessor, IdentifySecondaryOptions)
    pub fn identify_secondary_functions_options(
        stat: Statement,
        var_proc: VarProcessor,
        options: IdentifySecondaryOptions,
    ) -> bool {
        if stat.get_exprents().is_none() {
            if let Some(ifelsestat) = stat.try_as_if() {
                let ifstat = ifelsestat.get_ifstat();

                if ifelsestat.iftype() == IfStatement::IFTYPE_IFELSE
                    && ifstat
                        .expect("NullPointerException: ifstat")
                        .get_exprents()
                        .is_some_and(|e| e.is_empty())
                    && (ifstat.unwrap().get_all_successor_edges().is_empty()
                        || !ifstat.unwrap().get_first_successor().explicit())
                {
                    let ifstat = ifstat.unwrap();

                    ifelsestat
                        .get_stats()
                        .borrow_mut()
                        .remove_with_key(&ifstat.id());

                    ifelsestat.d_mut(|d| d.iftype = IfStatement::IFTYPE_IF);
                    ifelsestat.set_ifstat(ifelsestat.get_elsestat());
                    ifelsestat.set_elsestat(None);

                    if ifelsestat.get_all_successor_edges().is_empty()
                        && !ifstat.get_all_successor_edges().is_empty()
                    {
                        let endedge = ifstat.get_first_successor();

                        ifstat.remove_successor(endedge);
                        endedge.set_source(ifelsestat.0);
                        if endedge.closure().is_some() {
                            ifelsestat
                                .get_parent()
                                .expect("NullPointerException: parent")
                                .add_labeled_edge(endedge);
                        }
                        ifelsestat.add_successor(endedge);
                    }

                    let first = ifelsestat.get_first().expect("NullPointerException: first");
                    if let Some(if_edge) = ifelsestat.get_if_edge() {
                        first.remove_successor(if_edge);
                    }

                    ifelsestat.set_if_edge(ifelsestat.get_else_edge());
                    ifelsestat.set_else_edge(None);

                    ifelsestat.set_negated(!ifelsestat.is_negated());
                    let negated = ifelsestat.get_headexprent().copy().as_if().negate_if();
                    ifelsestat.get_headexprent_list().set(0, negated.0);

                    return true;
                } else if ifelsestat.iftype() == IfStatement::IFTYPE_IF
                    && ifstat.is_some_and(|ifstat| {
                        ifstat.get_exprents().is_some_and(|e| e.is_empty())
                            && (ifstat.has_any_successor()
                                && ifstat.get_first_successor().get_type()
                                    == StatEdge::TYPE_FINALLYEXIT)
                    })
                {
                }
            }
        }

        let mut ret = false;
        let mut replaced = true;
        while replaced {
            replaced = false;

            let stats = stat.get_stats().borrow().to_vec();
            for st in stats {
                if Self::identify_secondary_functions_options(st, var_proc, options) {
                    ret = true;
                    replaced = true;
                    break;
                }
            }

            let exprents: Vec<Exprent> = match stat.get_exprents() {
                None => stat.get_stat_exprents(),
                Some(exprents) => exprents.to_vec(),
            };

            for (i, &expr) in exprents.iter().enumerate() {
                let retexpr =
                    Self::identify_secondary_functions_exprent(stat, expr, true, var_proc, options);
                if let Some(retexpr) = retexpr {
                    match stat.get_exprents() {
                        None => stat.replace_exprent(expr, retexpr),
                        Some(exprents) => {
                            exprents.set(i, retexpr);
                        }
                    }
                    ret = true;
                    replaced = true;
                    break;
                }
            }
        }

        ret
    }

    /// Java: identifySecondaryFunctions(Statement, Exprent, boolean, VarProcessor, IdentifySecondaryOptions)
    fn identify_secondary_functions_exprent(
        stat: Statement,
        exprent: Exprent,
        statement_level: bool,
        var_proc: VarProcessor,
        options: IdentifySecondaryOptions,
    ) -> Option<Exprent> {
        use FunctionType::*;

        if let Some(fexpr) = exprent.try_as_function() {
            match fexpr.get_func_type() {
                BoolNot => {
                    let retparam = Self::propagate_bool_not(fexpr.0);

                    if retparam.is_some() {
                        return retparam;
                    }
                }
                Eq | Ne | Gt | Ge | Lt | Le => {
                    let mut expr1 = fexpr.get_lst_operands().get(0);
                    let mut expr2 = fexpr.get_lst_operands().get(1);

                    if expr1.is_const() {
                        expr2 = expr1;
                        expr1 = fexpr.get_lst_operands().get(1);
                    }

                    if let (Some(funcexpr), Some(cexpr)) =
                        (expr1.try_as_function(), expr2.try_as_const())
                    {
                        let functype = funcexpr.get_func_type();
                        if functype == Lcmp
                            || functype == Fcmpg
                            || functype == Fcmpl
                            || functype == Dcmpg
                            || functype == Dcmpl
                        {
                            let mut desttype: Option<FunctionType> = None;

                            let destcons =
                                MAP_NUM_COMPARISONS.with(|m| m.get_cloned(&fexpr.get_func_type()));
                            if let Some(destcons) = destcons {
                                let index = cexpr.get_int_value().wrapping_add(1);
                                if (0..=2).contains(&index) {
                                    if let Some(destcon) = destcons[index as usize] {
                                        desttype = Some(destcon);
                                    }
                                }
                            }

                            if let Some(desttype) = desttype {
                                if functype != Lcmp {
                                    let one_for_nan = functype == Dcmpl || functype == Fcmpl;
                                    let true_for_one = desttype == Lt || desttype == Le;
                                    let true_for_nan = one_for_nan == true_for_one;
                                    if true_for_nan {
                                        let operands = vec![
                                            FunctionExprent::new(
                                                funcsnot(desttype),
                                                funcexpr.get_lst_operands(),
                                                funcexpr.get_bytecode().as_ref(),
                                            )
                                            .0,
                                        ];
                                        return Some(
                                            FunctionExprent::new(
                                                BoolNot,
                                                operands,
                                                funcexpr.get_bytecode().as_ref(),
                                            )
                                            .0,
                                        );
                                    }
                                }
                                return Some(
                                    FunctionExprent::new(
                                        desttype,
                                        funcexpr.get_lst_operands(),
                                        funcexpr.get_bytecode().as_ref(),
                                    )
                                    .0,
                                );
                            }
                        }
                    }
                }
                Ppi => {
                    if statement_level {
                        let func = FunctionExprent::new(
                            Ipp,
                            fexpr.get_lst_operands(),
                            fexpr.get_bytecode().as_ref(),
                        );
                        func.set_implicit_type(Some(fexpr.0.get_expr_type()));
                        return Some(func.0);
                    }
                }
                Mmi => {
                    if statement_level {
                        let func = FunctionExprent::new(
                            Imm,
                            fexpr.get_lst_operands(),
                            fexpr.get_bytecode().as_ref(),
                        );
                        func.set_implicit_type(Some(fexpr.0.get_expr_type()));
                        return Some(func.0);
                    }
                }
                _ => {}
            }
        }

        let mut ret: Option<Exprent> = None;

        let mut replaced = true;
        while replaced {
            replaced = false;

            for expr in exprent.get_all_exprents() {
                let retexpr = Self::identify_secondary_functions_exprent(
                    stat, expr, false, var_proc, options,
                );
                if let Some(retexpr) = retexpr {
                    exprent.replace_exprent(expr, retexpr);
                    retexpr.add_bytecode_offsets(expr.get_bytecode().as_ref());
                    replaced = true;
                    ret = Some(exprent);
                    break;
                }
            }
        }

        match exprent.get_type() {
            ExprentType::Function => {
                let fexpr = exprent.as_function();
                let lst_operands = fexpr.get_lst_operands();

                match fexpr.get_func_type() {
                    Xor => {
                        for i in 0..2 {
                            let operand = lst_operands.get(i);
                            let operandtype = operand.get_expr_type();

                            if operand.is_const() && operandtype.type_ != CodeType::Boolean {
                                let cexpr = operand.as_const();
                                let value = cexpr.get_value().expect("NullPointerException: value");
                                let val: i64 = if operandtype.type_ == CodeType::Long {
                                    value.as_long()
                                } else {
                                    value.as_int() as i64
                                };

                                if val == -1 {
                                    let lst_bit_not_operand = vec![lst_operands.get(1 - i)];
                                    return Some(
                                        FunctionExprent::new(
                                            BitNot,
                                            lst_bit_not_operand,
                                            fexpr.get_bytecode().as_ref(),
                                        )
                                        .0,
                                    );
                                }
                            }
                        }
                    }
                    Eq | Ne => {
                        if lst_operands.get(0).get_expr_type().type_ == CodeType::Boolean
                            && lst_operands.get(1).get_expr_type().type_ == CodeType::Boolean
                        {
                            for i in 0..2 {
                                if lst_operands.get(i).is_const() {
                                    let cexpr = lst_operands.get(i).as_const();
                                    let val = cexpr.get_int_value();

                                    if (fexpr.get_func_type() == Eq && val == 1)
                                        || (fexpr.get_func_type() == Ne && val == 0)
                                    {
                                        return Some(lst_operands.get(1 - i));
                                    } else {
                                        let lst_not_operand = vec![lst_operands.get(1 - i)];
                                        return Some(
                                            FunctionExprent::new(
                                                BoolNot,
                                                lst_not_operand,
                                                fexpr.get_bytecode().as_ref(),
                                            )
                                            .0,
                                        );
                                    }
                                }
                            }
                        }
                    }
                    BoolNot => {
                        if lst_operands.get(0).is_const() {
                            let val = lst_operands.get(0).as_const().get_int_value();
                            if val == 0 {
                                return Some(
                                    ConstExprent::new(
                                        VarType::VARTYPE_BOOLEAN(),
                                        Some(ConstValue::Int(1)),
                                        fexpr.get_bytecode().as_ref(),
                                    )
                                    .0,
                                );
                            } else {
                                return Some(
                                    ConstExprent::new(
                                        VarType::VARTYPE_BOOLEAN(),
                                        Some(ConstValue::Int(0)),
                                        fexpr.get_bytecode().as_ref(),
                                    )
                                    .0,
                                );
                            }
                        }
                    }
                    Ternary => {
                        let expr0 = lst_operands.get(0);
                        let expr1 = lst_operands.get(1);
                        let expr2 = lst_operands.get(2);

                        if let (Some(cexpr1), Some(cexpr2)) =
                            (expr1.try_as_const(), expr2.try_as_const())
                        {
                            if cexpr1.get_expr_type().type_ == CodeType::Boolean
                                && cexpr2.get_expr_type().type_ == CodeType::Boolean
                            {
                                if cexpr1.get_int_value() == 0 && cexpr2.get_int_value() != 0 {
                                    return Some(
                                        FunctionExprent::new_single(
                                            BoolNot,
                                            lst_operands.get(0),
                                            fexpr.get_bytecode().as_ref(),
                                        )
                                        .0,
                                    );
                                } else if cexpr1.get_int_value() != 0 && cexpr2.get_int_value() == 0
                                {
                                    return Some(lst_operands.get(0));
                                }
                            }
                        }

                        if DecompilerContext::get_option(
                            IFernflowerPreferences::TERNARY_CONSTANT_SIMPLIFICATION,
                        ) || options.force_ternary_simplification
                        {
                            if expr1.is_const() && expr1.get_expr_type().type_ == CodeType::Boolean
                            {
                                let cexpr1 = expr1.as_const();
                                let val = cexpr1.get_int_value() != 0;

                                if val {
                                    return Some(
                                        FunctionExprent::new(
                                            BooleanOr,
                                            vec![lst_operands.get(0), lst_operands.get(2)],
                                            fexpr.get_bytecode().as_ref(),
                                        )
                                        .0,
                                    );
                                } else {
                                    let fnot = FunctionExprent::new_single(
                                        BoolNot,
                                        lst_operands.get(0),
                                        fexpr.get_bytecode().as_ref(),
                                    );
                                    return Some(
                                        FunctionExprent::new(
                                            BooleanAnd,
                                            vec![fnot.0, lst_operands.get(2)],
                                            fexpr.get_bytecode().as_ref(),
                                        )
                                        .0,
                                    );
                                }
                            } else if expr2.is_const()
                                && expr2.get_expr_type().type_ == CodeType::Boolean
                            {
                                let cexpr2 = expr2.as_const();
                                let val = cexpr2.get_int_value() != 0;

                                if val {
                                    let fnot = FunctionExprent::new_single(
                                        BoolNot,
                                        lst_operands.get(0),
                                        fexpr.get_bytecode().as_ref(),
                                    );
                                    return Some(
                                        FunctionExprent::new(
                                            BooleanOr,
                                            vec![fnot.0, lst_operands.get(1)],
                                            fexpr.get_bytecode().as_ref(),
                                        )
                                        .0,
                                    );
                                } else {
                                    return Some(
                                        FunctionExprent::new(
                                            BooleanAnd,
                                            vec![lst_operands.get(0), lst_operands.get(1)],
                                            fexpr.get_bytecode().as_ref(),
                                        )
                                        .0,
                                    );
                                }
                            } else if expr1.get_expr_type().type_ == CodeType::Boolean
                                && expr1.equals(expr0)
                            {
                                return Some(
                                    FunctionExprent::new(
                                        BooleanOr,
                                        vec![lst_operands.get(0), lst_operands.get(2)],
                                        fexpr.get_bytecode().as_ref(),
                                    )
                                    .0,
                                );
                            } else if expr2.get_expr_type().type_ == CodeType::Boolean
                                && expr2.equals(expr0)
                            {
                                return Some(
                                    FunctionExprent::new(
                                        BooleanAnd,
                                        vec![lst_operands.get(0), lst_operands.get(1)],
                                        fexpr.get_bytecode().as_ref(),
                                    )
                                    .0,
                                );
                            }
                        }
                    }
                    Lcmp | Fcmpl | Fcmpg | Dcmpl | Dcmpg => {
                        let var = DecompilerContext::get_counter_container()
                            .get_counter_and_increment(CounterContainer::VAR_COUNTER);
                        let type_ = lst_operands.get(0).get_expr_type();

                        let iff = FunctionExprent::new(
                            Ternary,
                            vec![
                                FunctionExprent::new(
                                    Lt,
                                    vec![
                                        VarExprent::new(var, Some(type_.clone()), Some(var_proc)).0,
                                        ConstExprent::get_zero_constant(type_.type_).0,
                                    ],
                                    None,
                                )
                                .0,
                                ConstExprent::new(
                                    VarType::VARTYPE_INT(),
                                    Some(ConstValue::Int(-1)),
                                    None,
                                )
                                .0,
                                ConstExprent::new(
                                    VarType::VARTYPE_INT(),
                                    Some(ConstValue::Int(1)),
                                    None,
                                )
                                .0,
                            ],
                            None,
                        );

                        let head = FunctionExprent::new(
                            Eq,
                            vec![
                                AssignmentExprent::new(
                                    VarExprent::new(var, Some(type_.clone()), Some(var_proc)).0,
                                    FunctionExprent::new(
                                        Sub,
                                        vec![lst_operands.get(0), lst_operands.get(1)],
                                        None,
                                    )
                                    .0,
                                    None,
                                )
                                .0,
                                ConstExprent::get_zero_constant(type_.type_).0,
                            ],
                            None,
                        );

                        var_proc.set_var_type(VarVersionPair::new(var, 0), &type_);

                        return Some(
                            FunctionExprent::new(
                                Ternary,
                                vec![
                                    head.0,
                                    ConstExprent::new(
                                        VarType::VARTYPE_INT(),
                                        Some(ConstValue::Int(0)),
                                        None,
                                    )
                                    .0,
                                    iff.0,
                                ],
                                fexpr.get_bytecode().as_ref(),
                            )
                            .0,
                        );
                    }
                    I2b | I2c | I2s | I2l | I2f | I2d | L2f | L2d | F2d => {
                        if matches!(fexpr.get_func_type(), I2b | I2c | I2s) {
                            if let Some(inner_function) = lst_operands.get(0).try_as_function() {
                                let cast_type = inner_function.get_func_type().cast_type();
                                if cast_type.is_some_and(|t| t.ptr_eq(&VarType::VARTYPE_INT())) {
                                    inner_function.set_needs_cast(false);
                                    return ret;
                                }
                            }
                        }

                        let expr_type = lst_operands.get(0).get_expr_type();
                        let cast_type = fexpr
                            .get_simple_cast_type()
                            .expect("NullPointerException: castType");

                        if cast_type.type_family == TypeFamily::Integer {
                            if cast_type.higher_in_lattice_than(&expr_type) {
                                fexpr.set_needs_cast(false);
                                return ret;
                            }
                        } else if cast_type.type_family.is_greater(expr_type.type_family) {
                            fexpr.set_needs_cast(false);
                            return ret;
                        }
                    }
                    Add | Sub | Mul | Div => {
                        let left = lst_operands.get(0);
                        let left_implicit_cast = is_implicit_cast(left).is_some();
                        let right = lst_operands.get(1);
                        let right_implicit_cast = is_implicit_cast(right).is_some();

                        if left_implicit_cast
                            && right_implicit_cast
                            && right.get_expr_type().ptr_eq(&left.get_expr_type())
                        {
                            left.as_function().set_needs_cast(true);
                        }
                    }
                    Shl | Shr | Ushr => {
                        let op = lst_operands.get(0);
                        if let Some(op) = is_implicit_cast(op) {
                            op.set_needs_cast(true);
                        }
                    }
                    _ => {}
                }
            }
            ExprentType::Assignment => {
                let asexpr = exprent.as_assignment();

                if asexpr.get_cond_type().is_some() {
                    return ret;
                }

                if kuna_fixes::enabled(KunaFix::CompoundAssignmentShape) {
                    if let Some(made) = kuna_fix_compound_assignment_shape::compound(asexpr) {
                        return if made { Some(exprent) } else { ret };
                    }
                }

                let mut right = asexpr.get_right();
                let left = asexpr.get_left();

                if let Some(mut func) = right.try_as_function() {
                    if Self::is_simple(left, None) {
                        let mut midlayer: Option<VarType> = None;
                        if func.get_func_type().cast_type().is_some() {
                            right = func.get_lst_operands().get(0);
                            midlayer = func.get_simple_cast_type();
                            match right.try_as_function() {
                                Some(inner) => func = inner,
                                None => return ret,
                            }
                        }

                        let lst_func_operands = func.get_lst_operands();

                        let mut cond: Option<Exprent> = None;

                        let func_type = func.get_func_type();
                        let mut fall_through = false;
                        if matches!(func_type, Add | And | Or | Xor) {
                            if left.equals(lst_func_operands.get(1)) {
                                cond = Some(lst_func_operands.get(0));
                            } else {
                                fall_through = true;
                            }
                        }
                        if fall_through
                            || matches!(func_type, Sub | Mul | Div | Rem | Shl | Shr | Ushr)
                        {
                            if left.equals(lst_func_operands.get(0)) {
                                cond = Some(lst_func_operands.get(1));
                            }
                        }

                        if let Some(cond) = cond {
                            if (midlayer.is_none()
                                || midlayer.as_ref().unwrap().equals(&cond.get_expr_type()))
                                && Self::is_simple(left, Some(cond))
                            {
                                asexpr.set_right(cond);
                                asexpr.set_cond_type(Some(func.get_func_type()));
                                ret = Some(exprent);
                            }
                        }
                    }
                }
            }
            ExprentType::Invocation => {
                let invocation_expr = exprent.as_invocation();
                if invocation_expr.is_boxing_call() {
                    let param = invocation_expr.get_lst_parameters().get(0);
                    if let Some(param) = is_implicit_cast(param) {
                        param.set_needs_cast(true);
                    }
                }

                if !statement_level {
                    let retexpr =
                        ConcatenationHelper::contract_string_concat(stat.get_top_parent(), exprent);
                    if !exprent.equals(retexpr) {
                        return Some(retexpr);
                    }
                } else {
                    let invocation_exprent = exprent.as_invocation();
                    if invocation_exprent.is_boxing_call() {
                        invocation_exprent.force_boxing(true);
                    }
                }
            }
            _ => {}
        }

        ret
    }

    fn is_simple(exprent: Exprent, other: Option<Exprent>) -> bool {
        match exprent.get_type() {
            ExprentType::Var | ExprentType::Const => true,
            ExprentType::Array => {
                let array = exprent.as_array();
                Self::is_super_simple(array.get_index()) && Self::is_super_simple(array.get_array())
            }
            ExprentType::Field => match other {
                None => true,
                Some(other) => (other.get_exprent_use() & Exprent::SIDE_EFFECTS_FREE) != 0,
            },
            _ => false,
        }
    }

    fn is_super_simple(exprent: Exprent) -> bool {
        exprent.get_type() == ExprentType::Var || exprent.get_type() == ExprentType::Const
    }

    fn has_pattern(exprent: Exprent) -> bool {
        for ex in exprent.get_all_exprents_rec_self(false, true) {
            if ex.is_pattern()
                || ex.try_as_function().is_some_and(|func| {
                    func.get_func_type() == FunctionType::Instanceof
                        && func.get_lst_operands().len() > 2
                })
            {
                return true;
            }
        }

        false
    }

    pub fn propagate_bool_not(exprent: Exprent) -> Option<Exprent> {
        use FunctionType::*;

        if let Some(fexpr) = exprent.try_as_function() {
            if fexpr.get_func_type() == BoolNot {
                let param = fexpr.get_lst_operands().get(0);

                if let Some(fparam) = param.try_as_function() {
                    let ftype = fparam.get_func_type();
                    if Self::has_pattern(fparam.0) && ftype != BoolNot {
                        return None;
                    }
                    let mut can_simplify = false;
                    match ftype {
                        BoolNot => {
                            let newexpr = fparam.get_lst_operands().get(0);
                            let retexpr = Self::propagate_bool_not(newexpr);
                            return Some(retexpr.unwrap_or(newexpr));
                        }
                        Ternary => {
                            let fex1 = FunctionExprent::new_single(
                                BoolNot,
                                fparam.get_lst_operands().get(1),
                                None,
                            );
                            let fex2 = FunctionExprent::new_single(
                                BoolNot,
                                fparam.get_lst_operands().get(2),
                                None,
                            );

                            let ex1 = Self::propagate_bool_not(fex1.0);
                            let ex2 = Self::propagate_bool_not(fex2.0);

                            fparam.get_lst_operands().set(1, ex1.unwrap_or(fex1.0));
                            fparam.get_lst_operands().set(2, ex2.unwrap_or(fex2.0));

                            return Some(fparam.0);
                        }
                        BooleanAnd | BooleanOr | Eq | Ne | Lt | Ge | Gt | Le => {
                            if matches!(ftype, BooleanAnd | BooleanOr) {
                                let operands = fparam.get_lst_operands();
                                for i in 0..operands.len() {
                                    let newparam = FunctionExprent::new_single(
                                        BoolNot,
                                        operands.get(i),
                                        operands.get(i).get_bytecode().as_ref(),
                                    );

                                    let retparam = Self::propagate_bool_not(newparam.0);
                                    operands.set(i, retparam.unwrap_or(newparam.0));
                                }
                            }
                            if matches!(ftype, BooleanAnd | BooleanOr | Eq | Ne) {
                                can_simplify = true;
                            }
                            if !can_simplify {
                                let operands = fparam.get_lst_operands();
                                let left = operands.get(0).get_expr_type();
                                let right = operands.get(1).get_expr_type();
                                let common_supertype = VarType::join(&left, &right);
                                if let Some(common_supertype) = common_supertype {
                                    can_simplify = common_supertype.type_ != CodeType::Float
                                        && common_supertype.type_ != CodeType::Double;
                                }
                            }
                            if can_simplify {
                                fparam.set_func_type(funcsnot(ftype));
                                return Some(fparam.0);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        None
    }

    pub fn update_assignments(stat: Statement) -> bool {
        let mut res = false;

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            res |= Self::update_assignments(st);
        }

        let exprents: Vec<Exprent> = match stat.get_exprents() {
            None => stat.get_stat_exprents(),
            Some(exprents) => exprents.to_vec(),
        };

        for exprent in exprents {
            if let Some(assignment) = exprent.try_as_assignment() {
                let params = exprent.get_all_exprents();

                let lhs = params[0];
                let rhs = params[1];

                if assignment.get_cond_type().is_none() && lhs.is_var() && rhs.is_function() {
                    let lhs_var = lhs.as_var();
                    let rhs_func = rhs.as_function();

                    let func_params = rhs.get_all_exprents();

                    if rhs_func.get_func_type().is_arithmetic_binary_operation()
                        && func_params[0].is_var()
                    {
                        let lhs_var_func = func_params[0].as_var();

                        if lhs_var.get_index() == lhs_var_func.get_index() {
                            assignment.set_cond_type(Some(rhs_func.get_func_type()));
                            assignment.set_right(func_params[1]);

                            res = true;
                        }
                    }
                }
            }
        }

        res
    }
}
