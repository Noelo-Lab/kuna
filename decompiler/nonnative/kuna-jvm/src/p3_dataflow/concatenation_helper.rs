//! Port of `org/jetbrains/java/decompiler/modules/decompiler/ConcatenationHelper.java`.

use crate::code_constants::CONSTANT_STRING;
use crate::code_type::CodeType;
use crate::const_exprent::{ConstExprent, ConstValue};
use crate::exprent::{Exprent, ExprentType};
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::invocation_exprent::InvocationExprent;
use crate::new_exprent::NewExprent;
use crate::pooled_constant::PooledConstant;
use crate::root_statement::RootStatement;
use crate::rt::BitSet;
use crate::statement::Statement;
use crate::var_type::VarType;

pub struct ConcatenationHelper;

const BUILDER_CLASS: &str = "java/lang/StringBuilder";
const BUFFER_CLASS: &str = "java/lang/StringBuffer";
const STRING_CLASS: &str = "java/lang/String";

const TAG_ARG: u16 = 0x0001;
const TAG_ARG_S: &str = "\u{1}";
const TAG_CONST: u16 = 0x0002;

thread_local! {
    static BUILDER_TYPE: VarType = VarType::new_value(CodeType::Object, 0, "java/lang/StringBuilder");
    static BUFFER_TYPE: VarType = VarType::new_value(CodeType::Object, 0, "java/lang/StringBuffer");
}

fn empty_string_const(bytecode: Option<&BitSet>) -> Exprent {
    ConstExprent::new(
        VarType::VARTYPE_STRING(),
        Some(ConstValue::String(String::new())),
        bytecode,
    )
    .0
}

impl ConcatenationHelper {
    #[allow(non_snake_case)]
    fn BUILDER_TYPE() -> VarType {
        BUILDER_TYPE.with(VarType::clone)
    }

    #[allow(non_snake_case)]
    fn BUFFER_TYPE() -> VarType {
        BUFFER_TYPE.with(VarType::clone)
    }

    pub fn simplify_string_concat(stat: Statement) {
        let sub_stats = stat.get_stats().borrow().to_vec();
        for s in sub_stats {
            Self::simplify_string_concat(s);
        }

        if stat.get_exprents().is_some() {
            let mut i = 0;
            while i < stat.get_exprents().unwrap().len() {
                let ret = Self::simplify_string_concat_exprent(
                    stat.get_top_parent(),
                    stat.get_exprents().unwrap().get(i),
                );
                if let Some(ret) = ret {
                    stat.get_exprents().unwrap().set(i, ret);
                }
                i += 1;
            }
        }
    }

    /// Java: simplifyStringConcat(RootStatement, Exprent)
    fn simplify_string_concat_exprent(root: RootStatement, exprent: Exprent) -> Option<Exprent> {
        for cexp in exprent.get_all_exprents() {
            if let Some(ret) = Self::simplify_string_concat_exprent(root, cexp) {
                exprent.replace_exprent(cexp, ret);
                ret.add_bytecode_offsets(cexp.get_bytecode().as_ref());
            }
        }

        if exprent.is_invocation() {
            let ret = Self::contract_string_concat(root, exprent);
            if !exprent.equals(ret) {
                return Some(ret);
            }
        }

        None
    }

    pub fn contract_string_concat(root: RootStatement, expr: Exprent) -> Exprent {
        let mut expr_tmp: Option<Exprent> = None;
        let mut cltype: Option<VarType> = None;

        if let Some(iex) = expr.try_as_invocation() {
            let name = iex.get_name();
            if name == "toString" {
                let classname = iex.get_classname();
                if BUILDER_CLASS == classname {
                    cltype = Some(Self::BUILDER_TYPE());
                } else if BUFFER_CLASS == classname {
                    cltype = Some(Self::BUFFER_TYPE());
                }
                if cltype.is_some() {
                    expr_tmp = iex.get_instance();
                }
            } else if name == "makeConcatWithConstants" || name == "makeConcat" {
                let bootstrap_arguments = iex.get_bootstrap_arguments();
                let mut parameters =
                    Self::extract_parameters(root, bootstrap_arguments.as_deref(), iex);

                let mut add_empty_string = true;
                let mut index = 0;
                while index < parameters.len() && index < 2 {
                    if parameters[index]
                        .get_expr_type()
                        .equals(&VarType::VARTYPE_STRING())
                    {
                        add_empty_string = false;
                        break;
                    }
                    index += 1;
                }

                if add_empty_string || parameters.len() == 1 {
                    let index = if parameters.len() == 1 { 1 } else { 0 };

                    parameters.insert(index, empty_string_const(expr.get_bytecode().as_ref()));
                }

                if parameters.len() >= 2 {
                    return Self::create_concat_exprent(&parameters, expr.get_bytecode().as_ref());
                }
            }
        }

        if expr_tmp.is_none() {
            return expr;
        }
        let cltype = cltype.expect("cltype is set with exprTmp");

        let mut lst_operands: Vec<Exprent> = Vec::new();

        loop {
            let mut found = 0;

            let current = expr_tmp.expect("NullPointerException: exprTmp");
            match current.get_type() {
                ExprentType::Invocation => {
                    let iex = current.as_invocation();
                    if Self::is_append_concat(iex, &cltype) {
                        lst_operands.insert(0, iex.get_lst_parameters().get(0));
                        expr_tmp = iex.get_instance();
                        found = 1;
                    }
                }
                ExprentType::New => {
                    let nex = current.as_new();
                    if Self::is_new_concat(nex, &cltype) {
                        let constructor = nex
                            .get_constructor()
                            .expect("NullPointerException: constructor");
                        let descriptor = constructor.get_descriptor();
                        if descriptor.params.len() == 1 {
                            lst_operands.insert(0, constructor.get_lst_parameters().get(0));
                        }
                        found = 2;
                    }
                }
                _ => {}
            }

            if found == 0 {
                return expr;
            } else if found == 2 {
                break;
            }
        }

        let mut first2str: i32 = 0;
        let mut index = 0;
        while index < lst_operands.len() && index < 2 {
            if lst_operands[index]
                .get_expr_type()
                .equals(&VarType::VARTYPE_STRING())
            {
                first2str |= index as i32 + 1;
            }
            index += 1;
        }

        if first2str == 0 {
            lst_operands.insert(0, empty_string_const(expr.get_bytecode().as_ref()));
        }

        for i in 0..lst_operands.len() {
            let rep = Self::remove_string_value_of(lst_operands[i]);

            let mut ok = i > 1;
            if !ok {
                let isstr = rep.get_expr_type().equals(&VarType::VARTYPE_STRING());
                ok = isstr || first2str != i as i32 + 1;

                if i == 0 {
                    first2str &= 2;
                }
            }

            if ok {
                lst_operands[i] = rep;
            }
        }
        Self::create_concat_exprent(&lst_operands, expr.get_bytecode().as_ref())
    }

    fn create_concat_exprent(lst_operands: &[Exprent], bytecode: Option<&BitSet>) -> Exprent {
        let mut func = lst_operands[0];

        for &oper in &lst_operands[1..] {
            let tmp = FunctionExprent::new(FunctionType::StrConcat, vec![func, oper], bytecode);
            tmp.set_implicit_type(Some(
                if VarType::VARTYPE_STRING().equals(&func.get_expr_type()) {
                    oper.get_expr_type()
                } else {
                    func.get_expr_type()
                },
            ));
            func = tmp.0;
        }

        func
    }

    fn extract_parameters(
        root: RootStatement,
        bootstrap_arguments: Option<&[Option<PooledConstant>]>,
        expr: InvocationExprent,
    ) -> Vec<Exprent> {
        let parameters = expr.get_lst_parameters();

        for i in 0..parameters.len() {
            let x = parameters.get(i);
            parameters.set(i, Self::remove_string_value_of(x));
        }

        if let Some(bootstrap_arguments) = bootstrap_arguments {
            let mut recipe: Option<String> = None;
            if !bootstrap_arguments.is_empty()
                && Self::pooled(&bootstrap_arguments[0]).type_() == CONSTANT_STRING
            {
                let constant = Self::pooled(&bootstrap_arguments[0]);
                if constant.type_() == CONSTANT_STRING {
                    recipe = constant
                        .as_primitive()
                        .get_string_opt()
                        .map(|s| s.to_string());
                }
            } else if bootstrap_arguments.is_empty() {
                recipe = Some(TAG_ARG_S.repeat(parameters.len()));
            }

            if let Some(recipe) = recipe {
                let mut res: Vec<Exprent> = Vec::new();
                let mut acc = String::new();
                let mut parameter_id = 0usize;
                for c in recipe.chars() {
                    let unit = c as u32;
                    if unit == TAG_CONST as u32 || unit == TAG_ARG as u32 {
                        if !acc.is_empty() {
                            res.push(
                                ConstExprent::new(
                                    VarType::VARTYPE_STRING(),
                                    Some(ConstValue::String(acc.clone())),
                                    expr.get_bytecode().as_ref(),
                                )
                                .0,
                            );
                            acc.clear();
                        }

                        if unit == TAG_ARG as u32 {
                            if parameter_id >= parameters.len() {
                                root.add_comment(
                                    "$VF: Could not fully resugar string concatentation!",
                                );
                                continue;
                            }
                            res.push(parameters.get(parameter_id));
                            parameter_id += 1;
                        }
                    } else {
                        acc.push(c);
                    }
                }

                if !acc.is_empty() {
                    res.push(
                        ConstExprent::new(
                            VarType::VARTYPE_STRING(),
                            Some(ConstValue::String(acc)),
                            expr.get_bytecode().as_ref(),
                        )
                        .0,
                    );
                }

                return res;
            }
        }

        parameters.to_vec()
    }

    #[track_caller]
    fn pooled(constant: &Option<PooledConstant>) -> &PooledConstant {
        constant
            .as_ref()
            .expect("NullPointerException: bootstrap argument")
    }

    fn is_append_concat(expr: InvocationExprent, cltype: &VarType) -> bool {
        if expr.get_name() == "append" {
            let md = expr.get_descriptor();
            if md.ret.equals(cltype) && md.params.len() == 1 {
                let param = &md.params[0];
                match param.type_ {
                    CodeType::Object => {
                        if param.equals(&VarType::VARTYPE_STRING())
                            || param.equals(&VarType::VARTYPE_OBJECT())
                        {
                            return true;
                        }
                    }
                    CodeType::Boolean
                    | CodeType::Char
                    | CodeType::Double
                    | CodeType::Float
                    | CodeType::Int
                    | CodeType::Long => return true,
                    _ => {}
                }
            }
        }

        false
    }

    fn is_new_concat(expr: NewExprent, cltype: &VarType) -> bool {
        if expr.get_new_type().equals(cltype) {
            let descriptor = expr
                .get_constructor()
                .expect("NullPointerException: constructor")
                .get_descriptor();
            let params = &descriptor.params;
            return params.is_empty()
                || params.len() == 1 && params[0].equals(&VarType::VARTYPE_STRING());
        }

        false
    }

    fn remove_string_value_of(exprent: Exprent) -> Exprent {
        if let Some(iex) = exprent.try_as_invocation() {
            if iex.get_name() == "valueOf" && STRING_CLASS == iex.get_classname() {
                let md = iex.get_descriptor();
                if md.params.len() == 1 {
                    let param = &md.params[0];
                    match param.type_ {
                        CodeType::Object => {
                            if param.equals(&VarType::VARTYPE_OBJECT()) {
                                return iex.get_lst_parameters().get(0);
                            }
                        }
                        CodeType::Boolean
                        | CodeType::Char
                        | CodeType::Double
                        | CodeType::Float
                        | CodeType::Int
                        | CodeType::Long => return iex.get_lst_parameters().get(0),
                        _ => {}
                    }
                }
            }
        }

        exprent
    }
}
