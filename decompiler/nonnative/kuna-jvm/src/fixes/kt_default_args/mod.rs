//! Fix family `kt_default_args` (Kotlin output): default arguments: the
//! `$default` bridge a function's defaults are read from (in its class or
//! `DefaultImpls`), the defaults written with the function's own names and
//! without the casts and temporaries kotlinc adds, and calls through the
//! bridges written as the source calls (`kt_default_args_call`).

pub(crate) mod kt_default_args_call;

use std::cell::RefCell;

use crate::class_wrapper::ClassWrapper;
use crate::code_constants::{ACC_INTERFACE, ACC_STATIC, ACC_SYNTHETIC};
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::fixes::kt_extensions;
use crate::fixes::kt_util::INTRINSICS;
use crate::function_exprent::FunctionType;
use crate::k_element::KElement;
use crate::k_parameter::KParameter;
use crate::kotlin_writer::KotlinWriter;
use crate::kt_attrs::KtAttrs;
use crate::method_wrapper::MethodWrapper;
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;
use crate::var_type::VarType;

const DEFAULT_IMPLS: &str = "$DefaultImpls";

/// A function's default-arguments method.
#[derive(Clone)]
pub struct Bridge {
    /// The class holding it, when that is not the function's class.
    pub holder: Option<String>,
    pub name: String,
    pub descriptor: String,
}

/// The default-arguments method of `method`, a function of `cl` with
/// `value_parameters` value parameters, when the class or, for an
/// interface, its `DefaultImpls` has it as a static synthetic method.
pub fn locate(cl: StructClass, method: StructMethod, value_parameters: usize) -> Option<Bridge> {
    let name = format!("{}$default", method.get_name());
    let own = method.get_descriptor();
    let (params, ret) = own.strip_prefix('(')?.split_once(')')?;
    let mut text = String::from("(");
    if !method.has_modifier(ACC_STATIC) {
        text.push_str(&format!("L{};", cl.qualified_name()));
    }
    text.push_str(params);
    text.push_str(&"I".repeat(value_parameters / 32 + 1));
    text.push_str("Ljava/lang/Object;)");
    text.push_str(ret);

    let is_bridge = |m: StructMethod| m.has_modifier(ACC_STATIC) && m.has_modifier(ACC_SYNTHETIC);
    if cl.get_method(&name, &text).is_some_and(is_bridge) {
        return Some(Bridge {
            holder: None,
            name,
            descriptor: text,
        });
    }
    if !cl.has_modifier(ACC_INTERFACE) || method.has_modifier(ACC_STATIC) {
        return None;
    }
    let holder = format!("{}{}", cl.qualified_name(), DEFAULT_IMPLS);
    let impls = DecompilerContext::get_struct_context().get_class(&holder)?;
    if !impls.get_method(&name, &text).is_some_and(is_bridge) {
        return None;
    }
    Some(Bridge {
        holder: Some(holder),
        name,
        descriptor: text,
    })
}

impl Bridge {
    /// The method, decompiled, when the class written is `writing`.
    pub fn wrapper(&self, writing: ClassWrapper) -> Option<MethodWrapper> {
        let wrapper = match &self.holder {
            None => writing,
            Some(holder) => DecompilerContext::get_class_processor()
                .get_map_root_classes()
                .borrow()
                .get(holder)
                .copied()?
                .get_wrapper()?,
        };
        wrapper.get_method_wrapper(&self.name, &self.descriptor)
    }

    /// The method's class-file record.
    pub fn method(&self, cl: StructClass) -> Option<StructMethod> {
        match &self.holder {
            None => cl.get_method(&self.name, &self.descriptor),
            Some(holder) => DecompilerContext::get_struct_context()
                .get_class(holder)?
                .get_method(&self.name, &self.descriptor),
        }
    }
}

struct Names {
    varproc: VarProcessor,
    slots: Vec<(i32, String)>,
}

thread_local! {
    static NAMES: RefCell<Vec<Names>> = const { RefCell::new(Vec::new()) };
}

/// While alive, the defaults of a function are being written.
pub struct NamesScope;

impl Drop for NamesScope {
    fn drop(&mut self) {
        NAMES.with(|n| {
            n.borrow_mut().pop();
        });
    }
}

/// Enters the defaults of `method` of `cl` (its value parameters
/// `parameters`, an extension when `extension`), read from `bridge`.
pub fn enter_defaults(
    bridge: MethodWrapper,
    cl: StructClass,
    method: StructMethod,
    parameters: &[KParameter],
    extension: bool,
    suspend: bool,
) -> Option<NamesScope> {
    let is_static = method.has_modifier(ACC_STATIC);
    let descriptor = method.method_descriptor();
    let jvm = &descriptor.params;
    let value_end = jvm.len().checked_sub(usize::from(suspend))?;
    let value_start = value_end.checked_sub(parameters.len())?;
    let receiver = if extension {
        Some(value_start.checked_sub(1)?)
    } else {
        None
    };

    let mut slots: Vec<(i32, String)> = Vec::new();
    if !is_static {
        let instance = if extension {
            format!("this@{}", kt_extensions::class_label(cl)?)
        } else {
            "this".to_string()
        };
        slots.push((0, instance));
    }
    let mut slot = i32::from(!is_static);
    for (i, param) in jvm.iter().enumerate() {
        if Some(i) == receiver {
            slots.push((slot, "this".to_string()));
        } else if (value_start..value_end).contains(&i) {
            let name = &parameters[i - value_start].name;
            slots.push((slot, KotlinWriter::to_valid_kotlin_identifier(name)));
        }
        slot += param.stack_size;
    }

    let varproc = bridge.with(|d| d.varproc);
    NAMES.with(|n| n.borrow_mut().push(Names { varproc, slots }));
    Some(NamesScope)
}

/// The name `var` is written with in the defaults being written.
pub fn name_in_defaults(var: VarExprent) -> Option<String> {
    let processor = var.get_processor()?;
    NAMES.with(|n| {
        let n = n.borrow();
        let names = n.last()?;
        if names.varproc != processor {
            return None;
        }
        let index = var.get_index();
        let slot = processor.get_var_original_index(index).unwrap_or(index);
        names
            .slots
            .iter()
            .find(|(s, _)| *s == slot)
            .map(|(_, name)| name.clone())
    })
}

const NULL_CHECKS: &[&str] = &[
    "checkNotNull",
    "checkNotNullExpressionValue",
    "checkExpressionValueIsNotNull",
];

/// The default the branch statements `exprents` assign to a parameter, whose
/// slots are those below `first_mask_slot`.
pub fn default_value(exprents: &[Exprent], first_mask_slot: i32) -> Option<Exprent> {
    let (last, before) = exprents.split_last()?;
    let assignment = last.try_as_assignment()?;
    let param = assignment.get_left().try_as_var()?;
    if param.get_index() < 0 || param.get_index() >= first_mask_slot {
        return None;
    }
    let right = assignment.get_right();
    let Some((first, checks)) = before.split_first() else {
        return Some(right);
    };

    let temp = right.try_as_var()?.get_index();
    if temp < first_mask_slot {
        return None;
    }
    let init = first.try_as_assignment()?;
    if init.get_left().try_as_var()?.get_index() != temp {
        return None;
    }
    let value = init.get_right();
    let checks_temp = |check: &Exprent| {
        check.try_as_invocation().is_some_and(|call| {
            call.get_classname() == INTRINSICS
                && NULL_CHECKS.contains(&call.get_name().as_str())
                && call
                    .get_lst_parameters()
                    .to_vec()
                    .first()
                    .is_some_and(|&arg| {
                        arg.try_as_var().is_some_and(|arg| arg.get_index() == temp)
                            || arg.equals(value)
                    })
        })
    };
    if !checks.iter().all(checks_temp) {
        return None;
    }
    Some(value)
}

/// `default` (a copy, whose operands are the Kotlin plugin's) without its
/// cast to the type of the parameter the branch statements `exprents`
/// (ending with the parameter's assignment) assign, a parameter of the
/// static default-arguments method `bridge`.
pub fn uncast(default: Exprent, exprents: &[Exprent], bridge: MethodWrapper) -> Exprent {
    let operand = || {
        let cast = default.try_as_function()?;
        if cast.get_func_type() != FunctionType::Cast {
            return None;
        }
        let operands = cast.get_lst_operands();
        if writes_class_body(operands.get(0)) {
            return None;
        }
        let target = operands.get(1).try_as_const()?.get_const_type();
        if target.type_ != CodeType::Object && target.array_dim == 0 {
            return None;
        }
        let param = exprents
            .last()?
            .try_as_assignment()?
            .get_left()
            .try_as_var()?;
        let processor = param.get_processor()?;
        let index = param.get_index();
        let slot = processor.get_var_original_index(index).unwrap_or(index);
        param_type(bridge, slot)?
            .equals(&target)
            .then(|| operands.get(0))
    };
    operand().unwrap_or(default)
}

/// The type of the parameter at `slot` of the static method `method`.
fn param_type(method: MethodWrapper, slot: i32) -> Option<VarType> {
    let mut at = 0;
    for param in method.desc().params.iter() {
        if at == slot {
            return Some(param.clone());
        }
        at += param.stack_size;
    }
    None
}

/// Whether `expr` creates an instance of an anonymous class or a lambda,
/// or reads a singleton lambda.
fn writes_class_body(expr: Exprent) -> bool {
    expr.try_as_new()
        .is_some_and(|new| new.is_anonymous() || new.is_lambda())
        || expr.try_as_field().is_some_and(|field| {
            field.is_static()
                && DecompilerContext::get_struct_context()
                    .get_class(&field.get_classname())
                    .and_then(|cl| KtAttrs::get(cl, &KElement::KEY()))
                    .is_some_and(|e| matches!(e, KElement::Function(_)))
        })
        || expr.get_all_exprents().into_iter().any(writes_class_body)
}
