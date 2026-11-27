//! Fix family `kt_constructors` (Kotlin output): constructors and their calls:
//! a data class's property parameters, the delegation call of a constructor
//! without metadata, the constructor a synthetic accessor forwards to, and an
//! inner class constructed by its simple name.

use std::rc::Rc;

use crate::class_wrapper::ClassWrapper;
use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::{ACC_STATIC, OPC_ALOAD, OPC_ILOAD, OPC_INVOKESPECIAL, OPC_RETURN};
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::fixes::kt_default_args::kt_default_args_call;
use crate::fixes::kt_delegation_calls;
use crate::interpreter_util::InterpreterUtil;
use crate::invocation_exprent::InvocationExprent;
use crate::k_element::KElement;
use crate::k_parameter::KParameter;
use crate::k_property::KProperty;
use crate::key::Key;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_proto::MemberKind;
use crate::method_descriptor::MethodDescriptor;
use crate::method_wrapper::MethodWrapper;
use crate::new_exprent::NewExprent;
use crate::rt::{try_catch, JHashSet, JList};
use crate::statements::Statements;
use crate::struct_class::StructClass;
use crate::struct_field::StructField;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;

const MARKER: &str = "Lkotlin/jvm/internal/DefaultConstructorMarker;";

/// Per parameter of the primary constructor, the field of the property it
/// declares.
type Plan = Rc<Vec<Option<StructField>>>;

#[allow(non_snake_case)]
fn PLAN() -> Key<Plan> {
    Key::of("kuna.kt_data_class_property_parameters.plan")
}

#[allow(non_snake_case)]
fn DECLARED() -> Key<bool> {
    Key::of("kuna.kt_data_class_property_parameters.declared")
}

/// `KotlinWriter`'s set of the properties already written, holding those
/// the constructor of `cl` declares, so the class body leaves them out.
pub fn visited(cl: StructClass) -> JHashSet<KProperty> {
    let mut visited = JHashSet::new();
    for field in cl.get_fields().to_vec() {
        if KtAttrs::has(field, &DECLARED()) {
            if let Some(property) = property(field) {
                visited.add((*property).clone());
            }
        }
    }
    visited
}

/// The fields of the properties the primary constructor `method` of the data
/// class `cl` declares, per parameter; the first call takes their stores out
/// of the constructor body (`first`, its first block's exprents).
pub fn plan(
    cl: StructClass,
    class_flags: i32,
    method: StructMethod,
    parameters: &[KParameter],
    method_wrapper: MethodWrapper,
    wrapper: ClassWrapper,
    first: impl FnOnce() -> JList<Exprent>,
) -> Plan {
    if let Some(plan) = KtAttrs::get(method, &PLAN()) {
        return plan;
    }
    let mut fields: Vec<Option<StructField>> = vec![None; parameters.len()];
    if Flags::IS_DATA.get(class_flags)
        && method_wrapper.with(|d| d.root.is_some() && d.decompile_error.is_none())
    {
        let exprents = first();
        let mut taken = Vec::new();
        collect(cl, method, parameters, &exprents, wrapper, &mut fields, &mut taken);
        for &index in taken.iter().rev() {
            exprents.remove_at(index);
        }
        for field in fields.iter().flatten() {
            KtAttrs::put(*field, &DECLARED(), true);
        }
        hide_synthesized(cl, wrapper);
    }
    let plan = Rc::new(fields);
    KtAttrs::put(method, &PLAN(), plan.clone());
    plan
}

/// Hides the members kotlinc synthesizes for a data class (`componentN`,
/// `copy`, and `equals`, `hashCode`, `toString` unless the class declares
/// them), which Kotlin rejects as declarations ("conflicting overloads").
fn hide_synthesized(cl: StructClass, wrapper: ClassWrapper) {
    for method in cl.get_methods().to_vec() {
        let Some(KElement::Function(function)) = KtAttrs::get(method, &KElement::KEY()) else {
            continue;
        };
        if function.class_struct == Some(cl)
            && Flags::MEMBER_KIND.get(function.flags) == Some(MemberKind::Synthesized)
        {
            let key = InterpreterUtil::make_unique_key(&method.get_name(), &method.get_descriptor());
            wrapper.get_hidden_members().borrow_mut().add(key);
        }
    }
}

/// The leading stores of parameters into their properties' fields.
fn collect(
    cl: StructClass,
    method: StructMethod,
    parameters: &[KParameter],
    exprents: &JList<Exprent>,
    wrapper: ClassWrapper,
    fields: &mut [Option<StructField>],
    taken: &mut Vec<usize>,
) {
    let jvm_params = method.method_descriptor().params.clone();
    let Some(lead) = jvm_params.len().checked_sub(parameters.len()) else {
        return;
    };
    let slots: Vec<i32> = (0..parameters.len())
        .map(|i| 1 + jvm_params[..lead + i].iter().map(|p| p.stack_size).sum::<i32>())
        .collect();
    let start = match exprents.len() {
        0 => return,
        _ => usize::from(
            exprents
                .get(0)
                .try_as_invocation()
                .is_some_and(|invocation| invocation.get_name() == "<init>"),
        ),
    };
    let mut next = 0;
    for index in start..exprents.len() {
        let Some((param, field)) = store(cl, exprents.get(index), &slots) else {
            break;
        };
        if param < next || !declarable(field, &parameters[param], cl, wrapper) {
            break;
        }
        fields[param] = Some(field);
        taken.push(index);
        next = param + 1;
    }
}

/// `this.f = p` of the parameter at one of `slots` into a field of `cl`.
fn store(cl: StructClass, exprent: Exprent, slots: &[i32]) -> Option<(usize, StructField)> {
    let assignment = exprent.try_as_assignment()?;
    if assignment.get_cond_type().is_some() {
        return None;
    }
    let left = assignment.get_left().try_as_field()?;
    if left.is_static() || left.get_classname() != cl.qualified_name() {
        return None;
    }
    let instance = left.get_instance()?.try_as_var()?;
    if instance.get_index() != 0 {
        return None;
    }
    let value = assignment.get_right().try_as_var()?;
    let index = value.get_index();
    let original = value
        .get_processor()
        .and_then(|p| p.get_var_original_index(index))
        .unwrap_or(index);
    let param = slots.iter().position(|&slot| slot == original)?;
    let field = cl.get_field(&left.get_name(), &left.get_descriptor().descriptor_string)?;
    Some((param, field))
}

/// Whether the property of `field` can be written as `parameter`.
fn declarable(
    field: StructField,
    parameter: &KParameter,
    cl: StructClass,
    wrapper: ClassWrapper,
) -> bool {
    let Some(KElement::Property(property)) = KtAttrs::get(field, &KElement::KEY()) else {
        return false;
    };
    if property.class_struct != cl
        || property.name != parameter.name
        || property.extension.is_some()
        || parameter.vararg_type.is_some()
        || Flags::IS_DELEGATED.get(property.flags)
        || Flags::IS_LATEINIT.get(property.flags)
        || Flags::IS_CONST.get(property.flags)
    {
        return false;
    }
    let custom = |accessor: &Option<crate::k_property_accessor::KPropertyAccessor>| {
        accessor
            .as_ref()
            .is_some_and(|a| Flags::IS_NOT_DEFAULT.get(a.flags))
    };
    if custom(&property.getter) || custom(&property.setter) {
        return false;
    }
    header(&property, wrapper, 0).is_some()
}

/// The property as a constructor parameter: its annotations, modifiers,
/// `val`/`var`, name and type on one line, as the class body writes them.
pub fn header(property: &KProperty, wrapper: ClassWrapper, indent: i32) -> Option<String> {
    let text = property.stringify(wrapper, indent).convert_to_string_and_allow_data_discard();
    let lines: Vec<&str> = text.lines().map(str::trim).filter(|l| !l.is_empty()).collect();
    let (last, annotations) = lines.split_last()?;
    if annotations.iter().any(|l| !l.starts_with('@'))
        || last.contains(" = ")
        || last.contains('{')
        || !(last.contains("val ") || last.contains("var "))
    {
        return None;
    }
    let mut out = String::new();
    for annotation in annotations {
        out.push_str(annotation);
        out.push(' ');
    }
    out.push_str(last);
    Some(out)
}

/// The property of `field`.
pub fn property(field: StructField) -> Option<Rc<KProperty>> {
    match KtAttrs::get(field, &KElement::KEY()) {
        Some(KElement::Property(property)) => Some(property),
        _ => None,
    }
}

/// How a constructor delegates.
#[derive(Clone, Copy)]
pub enum Delegation {
    /// The superclass or `this` call that is the body's first statement.
    Call(InvocationExprent),
    /// None written: the empty `super()` that `hideEmptySuper` removed.
    Implicit,
}

/// The delegation of the constructor `method` of `wrapper`'s class, when it
/// can be written as a Kotlin constructor's (not in a file class, whose
/// members are top-level declarations).
pub fn delegation(wrapper: ClassWrapper, method: MethodWrapper) -> Option<Delegation> {
    if matches!(
        KtAttrs::get(wrapper.get_class_struct(), &KElement::KEY()),
        Some(KElement::File(_) | KElement::MultifileFacade(_))
    ) {
        return None;
    }
    let (root, failed) = method.with(|d| (d.root, d.decompile_error.is_some()));
    let root = root.filter(|_| !failed)?;
    if kt_delegation_calls::hidden_call(method).is_some() {
        return Some(Delegation::Implicit);
    }
    let exprents = root.get_basichead().get_exprents()?;
    if exprents.is_empty() {
        return None;
    }
    let call = exprents.get(0).try_as_invocation()?;
    Statements::is_invocation_init_constructor(call, method, wrapper, true)
        .then_some(Delegation::Call(call))
}

/// Puts the delegation call back as the body's first statement when dropped.
pub struct Restore(Option<(JList<Exprent>, InvocationExprent)>);

impl Drop for Restore {
    fn drop(&mut self) {
        if let Some((exprents, call)) = self.0.take() {
            exprents.add_at(0, call.0);
        }
    }
}

/// Writes ` : super(..)` or ` : this(..)` for `delegation` and takes the
/// call out of `method`'s body until the returned guard is dropped.
pub fn write_delegation(
    method: MethodWrapper,
    delegation: Delegation,
    buffer: &mut TextBuffer,
    indent: i32,
) -> Restore {
    let Delegation::Call(call) = delegation else {
        return Restore(None);
    };
    kt_default_args_call::resugar_delegation(call);
    let mt = method.with(|d| d.method_struct);
    buffer.append(" : ");
    buffer.append_buffer_mapped(
        &call.0.to_java(indent + 1),
        Some(&mt.get_class_qualified_name()),
        Some(&InterpreterUtil::make_unique_key(
            &mt.get_name(),
            &mt.get_descriptor(),
        )),
    );
    let exprents = method
        .with(|d| d.root)
        .and_then(|root| root.get_basichead().get_exprents())
        .expect("delegation without a body");
    exprents.remove_at(0);
    Restore(Some((exprents, call)))
}

/// Whether the constructor, written with `delegation`, may be hidden as the
/// class's default constructor: only with no explicit delegation, in a class
/// whose superclass is `Object`.
pub fn may_hide(wrapper: ClassWrapper, delegation: Delegation) -> bool {
    matches!(delegation, Delegation::Implicit)
        && wrapper
            .get_class_struct()
            .super_class()
            .is_none_or(|s| &*s.get_string() == "java/lang/Object")
}

/// The constructor `mt` only forwards to when it is such an accessor.
pub fn real_constructor(cl: StructClass, mt: StructMethod) -> Option<StructMethod> {
    let desc = mt.get_descriptor();
    let target_desc = desc
        .strip_suffix(&format!("{MARKER})V"))
        .map(|head| format!("{head})V"))?;
    if mt.get_name() != "<init>" || !mt.is_synthetic() {
        return None;
    }
    let target = cl.get_method("<init>", &target_desc)?;
    let descriptor = mt.method_descriptor();
    let params = &descriptor.params;
    let mut slots = vec![0];
    let mut slot = 1;
    for param in &params[..params.len() - 1] {
        slots.push(slot);
        slot += param.stack_size;
    }

    let expanded = mt.get_instruction_sequence().is_some();
    if !expanded && try_catch(|| mt.expand_data(cl)).is_err() {
        return None;
    }
    let forwards = forwards_to(cl, mt, &slots, &target_desc);
    if !expanded {
        mt.release_resources();
    }
    forwards.then_some(target)
}

/// Whether `mt`'s code loads `slots` in order, calls this class's
/// `<init>target_desc` and returns.
fn forwards_to(cl: StructClass, mt: StructMethod, slots: &[i32], target_desc: &str) -> bool {
    let Some(seq) = mt.get_instruction_sequence() else {
        return false;
    };
    if seq.length() as usize != slots.len() + 2 {
        return false;
    }
    for (i, &slot) in slots.iter().enumerate() {
        let instr = seq.get_instr(i as i32);
        if !(OPC_ILOAD..=OPC_ALOAD).contains(&instr.opcode()) || instr.operand(0) != slot {
            return false;
        }
    }
    let call = seq.get_instr(slots.len() as i32);
    if call.opcode() != OPC_INVOKESPECIAL
        || seq.get_instr(slots.len() as i32 + 1).opcode() != OPC_RETURN
    {
        return false;
    }
    cl.get_pool()
        .get_link_constant(call.operand(0))
        .is_some_and(|link| {
            link.classname.as_deref() == Some(cl.qualified_name().as_str())
                && link.elementname == "<init>"
                && link.descriptor == target_desc
        })
}

/// Rewrites the constructor call of `new` when it goes through an accessor.
pub fn resugar_accessor_call(new: NewExprent) {
    if new.is_anonymous() || new.is_lambda() || new.is_enum_const() {
        return;
    }
    let Some(invocation) = new.get_constructor() else {
        return;
    };
    let descriptor = invocation.get_string_descriptor();
    if !descriptor.ends_with(&format!("{MARKER})V")) {
        return;
    }
    let args = invocation.get_lst_parameters();
    let n = args.len();
    if n == 0
        || invocation.get_descriptor().params.len() != n
        || !args.get(n - 1).try_as_const().is_some_and(|c| c.is_null())
    {
        return;
    }
    let Some(cl) = DecompilerContext::get_struct_context().get_class(&invocation.get_classname())
    else {
        return;
    };
    let Some(accessor) = cl.get_method("<init>", &descriptor) else {
        return;
    };
    let Some(target) = real_constructor(cl, accessor) else {
        return;
    };
    let target_descriptor = target.get_descriptor();
    args.remove_at(n - 1);
    invocation.set_descriptor(MethodDescriptor::parse_descriptor(&target_descriptor));
    invocation.set_string_descriptor(target_descriptor);
}

/// The class's simple name when the constructor call of class `classname`
/// with `params` builds an `inner` member class of the current class on its
/// `this`.
pub fn implicit_receiver_name(classname: &str, params: &[Exprent]) -> Option<String> {
    let node: ClassNode = DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .get(&classname.to_string())
        .copied()?;
    let (type_, access) = node.with(|d| (d.type_, d.access));
    if type_ != ClassNodeType::Member || (access & ACC_STATIC) != 0 {
        return None;
    }
    let var = params.first()?.try_as_var()?;
    let current = DecompilerContext::get_current_class_node()?;
    if node.with(|d| d.parent) != Some(current) {
        return None;
    }
    let current_class = current.with(|d| d.class_struct);
    let this_classname = var
        .get_processor()?
        .get_this_vars()
        .borrow()
        .get_cloned(&var.get_var_version_pair());
    if this_classname.as_deref() != Some(current_class.qualified_name().as_str()) {
        return None;
    }
    node.with(|d| d.simple_name.clone())
}
