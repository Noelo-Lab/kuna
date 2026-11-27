//! Fix `constant_local_int`: in a method without a LocalVariableTable, a JVM
//! slot whose every value is an integer constant holds an `int`: its locals
//! are typed `int`, or the `byte`, `short` or `char` that a read requires
//! (an argument, a store, a return of that type), instead of the narrowest
//! type the constants fit. A slot read where the fix cannot judge, where
//! reads require different types, or that an increment (`n++`) writes keeps
//! Vineflower's types.

use crate::code_constants::ACC_STATIC;
use crate::code_type::CodeType;
use crate::direct_graph::DirectGraph;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::method_descriptor::MethodDescriptor;
use crate::rt::JHashMap;
use crate::struct_method::StructMethod;
use crate::var_type::VarType;
use crate::var_type_processor::VarTypeProcessor;
use crate::var_version_pair::VarVersionPair;
use crate::var_versions_processor::VarVersionsProcessor;

/// Retypes in `types` the locals of `method` (its body `graph`) whose slot
/// only constants define; `previous` maps the graph's variables to their
/// slots after the first pass.
pub fn widen(
    types: VarTypeProcessor,
    graph: DirectGraph,
    method: StructMethod,
    previous: Option<VarVersionsProcessor>,
) {
    if method.get_local_variable_attr().is_some() {
        return;
    }
    let original = previous.map(|p| p.get_map_original_var_indices());
    let slot_of = |index: i32| {
        original
            .as_ref()
            .and_then(|m| m.borrow().get_cloned(&index))
            .map_or(index, |p| p.var)
    };
    let md = MethodDescriptor::parse_descriptor(&method.get_descriptor());
    let params: i32 = md.params.iter().map(|p| p.stack_size).sum::<i32>()
        + i32::from(!method.has_modifier(ACC_STATIC));
    let mut constant: JHashMap<i32, bool> = JHashMap::new();
    let mut versions: JHashMap<VarVersionPair, ()> = JHashMap::new();
    let mut uses: JHashMap<i32, Use> = JHashMap::new();
    graph.iterate_exprents(|exprent: Exprent| {
        let mut lst = exprent.get_all_exprents_rec(true);
        lst.push(exprent);
        for parent in lst {
            if let Some(assign) = parent.try_as_assignment() {
                if let Some(var) = assign.get_left().try_as_var() {
                    let slot = slot_of(var.get_index());
                    let all = constant.get_cloned(&slot).unwrap_or(slot >= params)
                        && assign.get_right().is_const();
                    constant.put(slot, all);
                    versions.put(VarVersionPair::from_var(var), ());
                }
            }
            if let Some(var) = stepped(parent) {
                constant.put(slot_of(var), false);
            }
            for (i, child) in parent.get_all_exprents().into_iter().enumerate() {
                let Some(var) = child.try_as_var() else {
                    continue;
                };
                if i == 0 && parent.is_assignment() {
                    continue;
                }
                let slot = slot_of(var.get_index());
                let merged = match (uses.get_cloned(&slot), use_of(parent, i, &md.ret)) {
                    (None, u) | (Some(Use::Int), u) => u,
                    (Some(u), Use::Int) => u,
                    (Some(Use::Narrow(a)), Use::Narrow(b)) if a.equals(&b) => Use::Narrow(a),
                    _ => Use::Unknown,
                };
                uses.put(slot, merged);
            }
        }
        0
    });
    let lower = types.get_lower_bounds();
    let mut retyped: Vec<(VarVersionPair, VarType)> = Vec::new();
    for pair in versions.key_vec() {
        let slot = slot_of(pair.var);
        if !constant.get_cloned(&slot).unwrap_or(false) {
            continue;
        }
        let target = match uses.get_cloned(&slot) {
            None | Some(Use::Int) => VarType::VARTYPE_INT(),
            Some(Use::Narrow(t)) => t,
            Some(Use::Unknown) => continue,
        };
        let Some(current) = lower.borrow().get_cloned(&pair) else {
            continue;
        };
        if current.array_dim != 0
            || !matches!(
                current.type_,
                CodeType::Byte
                    | CodeType::Short
                    | CodeType::Char
                    | CodeType::Bytechar
                    | CodeType::Shortchar
            )
            || !target.higher_equal_in_lattice_than(&current)
        {
            constant.put(slot, false);
            continue;
        }
        retyped.push((pair, target));
    }
    for (pair, target) in retyped {
        if constant.get_cloned(&slot_of(pair.var)).unwrap_or(false) {
            lower.borrow_mut().put(pair, target);
        }
    }
}

/// The local an increment or decrement (`n++`, `--n`) writes, if `exprent`
/// is one: a value that is not a constant.
fn stepped(exprent: Exprent) -> Option<i32> {
    let func = exprent.try_as_function()?;
    matches!(
        func.get_func_type(),
        FunctionType::Imm | FunctionType::Mmi | FunctionType::Ipp | FunctionType::Ppi
    )
    .then(|| func.get_lst_operands().first()?.try_as_var().map(|v| v.get_index()))?
}

/// What a read of a local asks of its type.
#[derive(Clone)]
enum Use {
    /// Any type that widens to `int`.
    Int,
    /// This `byte`, `short` or `char` type, which an `int` does not convert to.
    Narrow(VarType),
    /// A context this fix does not judge.
    Unknown,
}

/// What the read of a local that is child `index` of `parent` asks of its
/// type, in a method returning `ret`.
fn use_of(parent: Exprent, index: usize, ret: &VarType) -> Use {
    let target = |t: &VarType| {
        if t.array_dim == 0 && matches!(t.type_, CodeType::Byte | CodeType::Short | CodeType::Char) {
            Use::Narrow(t.clone())
        } else if t.array_dim == 0
            && matches!(
                t.type_,
                CodeType::Int | CodeType::Long | CodeType::Float | CodeType::Double
            )
        {
            Use::Int
        } else {
            Use::Unknown
        }
    };
    if let Some(assign) = parent.try_as_assignment() {
        if assign.get_left().is_var() {
            return Use::Unknown;
        }
        if assign.get_cond_type().is_some() {
            return Use::Int;
        }
        return target(&assign.get_left().get_expr_type());
    }
    if let Some(inv) = parent.try_as_invocation() {
        let args = inv.get_lst_parameters();
        let descriptor = inv.get_descriptor();
        let offset = parent.get_all_exprents().len() - args.len();
        return match index.checked_sub(offset).and_then(|i| descriptor.params.get(i)) {
            Some(p) if args.len() == descriptor.params.len() => target(p),
            _ => Use::Unknown,
        };
    }
    if let Some(func) = parent.try_as_function() {
        return match func.get_func_type() {
            FunctionType::Ternary | FunctionType::Instanceof | FunctionType::StrConcat => Use::Unknown,
            _ => Use::Int,
        };
    }
    if let Some(exit) = parent.try_as_exit() {
        return if exit.get_exit_type() == ExitType::Return {
            target(ret)
        } else {
            Use::Unknown
        };
    }
    if let Some(array) = parent.try_as_array() {
        return if index == 1 && array.get_index().is_var() { Use::Int } else { Use::Unknown };
    }
    if let Some(new) = parent.try_as_new() {
        let dims = new.get_lst_dims().len();
        return if index < dims {
            Use::Int
        } else {
            target(&new.get_new_type().decrease_array_dim())
        };
    }
    if parent.is_if() || parent.is_switch_head() {
        return Use::Int;
    }
    Use::Unknown
}
