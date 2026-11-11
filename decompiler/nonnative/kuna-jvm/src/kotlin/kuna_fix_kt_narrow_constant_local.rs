//! kuna fix `kt_narrow_constant_local`: in Kotlin output, a local that only
//! constants are stored into is typed `Int` where its uses take an `Int`.
//!
//! Vineflower types a local by the lower bound of what is stored into it,
//! and a constant's lower bound is the smallest type that holds it: a local
//! holding only `20`, `10` and `30` is a `byte`, one holding `256` a
//! `short`. Java widens such a local implicitly wherever an `int` is taken;
//! Kotlin does not, so the Kotlin output declares `var v: Byte` where the
//! source's local (or kotlinc's own temporary: a `when` value, the count of
//! an inlined `repeat(3)`, the size of an inlined `Array(256) { .. }`) was an
//! `Int`, and every use needs a conversion (`return v.toInt()`) or is
//! rejected (`repeat(v)`, `arrayOfNulls(v)`). With the fix, after the last
//! stack-variable pass, a local without a LocalVariableTable entry whose
//! every store is an integer constant (or an `if` expression choosing
//! between them) and whose type is `Byte` or `Short` is typed `Int` (its
//! constants too) when every read of it takes an `Int`:
//! a return from a function returning `Int`, an `Int` argument, a store into
//! an `Int` field, element or local, an operand of arithmetic, a comparison
//! or a conversion, an array index or size, a `when` subject. A read
//! anywhere else (a `Byte` argument, a branch of an `if` expression, a
//! string template) keeps Vineflower's type. The bytecode of either
//! declaration is the same: `bipush`, `istore`, `iload`.

use std::collections::BTreeMap;

use crate::code_constants::ACC_STATIC;
use crate::code_type::CodeType;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::{Exprent, ExprentType};
use crate::function_exprent::FunctionType;
use crate::method_descriptor::MethodDescriptor;
use crate::root_statement::RootStatement;
use crate::statement::Statement;
use crate::struct_method::StructMethod;
use crate::type_family::TypeFamily;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

#[derive(Default)]
struct Local {
    vars: Vec<Exprent>,
    constants: Vec<Exprent>,
    retype: bool,
}

/// Types the constant-only narrow locals under `root` whose uses take an `Int`.
pub fn retype(root: RootStatement, method: StructMethod, md: &MethodDescriptor, processor: VarProcessor) {
    let first_local = md.params.iter().map(|p| p.stack_size).sum::<i32>()
        + i32::from(!method.has_modifier(ACC_STATIC));
    let mut locals: BTreeMap<i32, Local> = BTreeMap::new();
    let mut tops = Vec::new();
    collect(root.0, &mut tops);
    for top in tops {
        visit(top, None, md, first_local, &mut locals);
    }
    for local in locals.values().filter(|l| l.retype) {
        for var in &local.vars {
            let var = var.as_var();
            var.set_var_type(VarType::VARTYPE_INT());
            processor.set_var_type(VarVersionPair::from_var(var), &VarType::VARTYPE_INT());
        }
        for constant in &local.constants {
            constant.as_const().set_const_type(VarType::VARTYPE_INT());
        }
    }
}

fn collect(stat: Statement, out: &mut Vec<Exprent>) {
    out.extend(stat.get_var_definitions().to_vec());
    out.extend(stat.get_stat_exprents());
    if let Some(exprents) = stat.get_exprents() {
        out.extend(exprents.to_vec());
    }
    let stats = stat.get_stats().borrow().to_vec();
    for child in stats {
        collect(child, out);
    }
}

fn narrow(t: &VarType) -> bool {
    t.array_dim == 0
        && matches!(t.type_, CodeType::Byte | CodeType::Short | CodeType::Bytechar | CodeType::Shortchar)
}

fn int(t: &VarType) -> bool {
    t.array_dim == 0 && t.type_ == CodeType::Int
}

/// Records `e`, read or stored in `parent`, and what is under it.
fn visit(
    e: Exprent,
    parent: Option<Exprent>,
    md: &MethodDescriptor,
    first_local: i32,
    locals: &mut BTreeMap<i32, Local>,
) {
    if let Some(var) = e.try_as_var() {
        if var.get_index() >= first_local {
            let candidate = var.get_lvt().is_none() && narrow(&var.get_var_type());
            let local = locals.entry(var.get_index()).or_insert_with(|| Local {
                retype: true,
                ..Local::default()
            });
            local.vars.push(e);
            local.retype &= candidate && takes_int(e, parent, md, local);
        }
        return;
    }
    for child in e.get_all_exprents() {
        visit(child, Some(e), md, first_local, locals);
    }
}

/// Whether `value` is an integer constant or an `if` expression choosing
/// between such values; collects the constants.
fn constants_of(value: Exprent, out: &mut Vec<Exprent>) -> bool {
    if let Some(c) = value.try_as_const() {
        out.push(value);
        return c.get_const_type().type_family == TypeFamily::Integer;
    }
    let Some(func) = value.try_as_function().filter(|f| f.get_func_type() == FunctionType::Ternary) else {
        return false;
    };
    let ops = func.get_lst_operands();
    constants_of(ops.get(1), out) && constants_of(ops.get(2), out)
}

/// Whether the occurrence `var` of a local in `parent` stores an integer
/// constant (recorded in `local`) or reads it where an `Int` is taken.
fn takes_int(var: Exprent, parent: Option<Exprent>, md: &MethodDescriptor, local: &mut Local) -> bool {
    let Some(parent) = parent else {
        return false;
    };
    match parent.get_type() {
        ExprentType::Assignment => {
            let a = parent.as_assignment();
            if a.get_left() == var {
                let mut constants = Vec::new();
                let constant = a.get_cond_type().is_none() && constants_of(a.get_right(), &mut constants);
                local.constants.extend(constants);
                constant
            } else {
                a.get_cond_type().is_none() && int(&a.get_left().get_expr_type())
            }
        }
        ExprentType::Exit => {
            let exit = parent.as_exit();
            exit.get_exit_type() == ExitType::Return && int(&md.ret)
        }
        ExprentType::Invocation => {
            let inv = parent.as_invocation();
            let params = inv.get_lst_parameters().to_vec();
            let descriptor = inv.get_descriptor();
            params
                .iter()
                .position(|p| *p == var)
                .and_then(|i| descriptor.params.get(i))
                .is_some_and(int)
        }
        ExprentType::Array => parent.as_array().get_index() == var,
        ExprentType::New => {
            let new = parent.as_new();
            new.get_lst_dims().to_vec().contains(&var)
                || (new.get_lst_array_elements().to_vec().contains(&var)
                    && int(&new.get_new_type().decrease_array_dim()))
        }
        ExprentType::SwitchHead => true,
        ExprentType::Function => !matches!(
            parent.as_function().get_func_type(),
            FunctionType::Ternary
                | FunctionType::StrConcat
                | FunctionType::Cast
                | FunctionType::Instanceof
                | FunctionType::Imm
                | FunctionType::Mmi
                | FunctionType::Ipp
                | FunctionType::Ppi
                | FunctionType::Other
        ),
        _ => false,
    }
}
