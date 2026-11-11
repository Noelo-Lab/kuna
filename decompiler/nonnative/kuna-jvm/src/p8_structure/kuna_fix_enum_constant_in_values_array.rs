//! kuna fix `enum_constant_in_values_array` (`kuna_fixes`): Kotlin 1.3's JVM
//! backend builds an enum's constants inside the `$VALUES` array
//! (`anewarray; dup; iconst_i; new E; .. invokespecial; dup; putstatic F;
//! aastore`), so the static initializer's first statement is
//! `$VALUES = new E[]{F0 = new E("F0", 0, ..), ..}`. `InitializerProcessor`
//! looks for `F = new E(..)` statements only; it moves the whole array into
//! the hidden `$VALUES` field, every constant prints without its constructor
//! arguments and the "Failed to inline enum fields" note goes with the
//! emptied, hidden `<clinit>`. With the fix such an array, when its elements
//! construct the class's enum constants in declaration order with their own
//! names and ordinals, gives each constant its `new E(..)` as initializer and
//! keeps a read of the constant in the array.

use crate::class_wrapper::ClassWrapper;
use crate::code_constants::{ACC_ENUM, ACC_STATIC};
use crate::const_exprent::ConstValue;
use crate::exprent::Exprent;
use crate::field_exprent::FieldExprent;
use crate::interpreter_util::InterpreterUtil;
use crate::new_exprent::NewExprent;
use crate::rt::{JHashSet, JList};
use crate::struct_class::StructClass;

/// Moves the constants built in a leading `$VALUES = new E[]{F = new E(..), ..}`
/// to their fields' initializers; `independent(field, value, whitelist, index)`
/// is `InitializerProcessor`'s independence test for a static initializer.
pub fn extract(
    wrapper: ClassWrapper,
    cl: StructClass,
    first_exprents: &JList<Exprent>,
    whitelist: &mut JHashSet<String>,
    multi_assign: &[String],
    independent: impl Fn(FieldExprent, Exprent, &JHashSet<String>, i32) -> bool,
) {
    let Some(elements) = values_array(cl, first_exprents) else {
        return;
    };
    let fields = cl.get_fields();
    let constants: Vec<_> = fields
        .iter()
        .filter(|fd| fd.has_modifier(ACC_ENUM) && fd.has_modifier(ACC_STATIC))
        .collect();
    if constants.len() != elements.len() {
        return;
    }
    let initializers = wrapper.get_static_field_initializers();
    let mut entries: Vec<(FieldExprent, NewExprent, String)> = Vec::new();
    for (ordinal, (element, fd)) in elements.iter().zip(&constants).enumerate() {
        let Some(assign) = element.try_as_assignment() else {
            return;
        };
        let Some(field) = assign.get_left().try_as_field() else {
            return;
        };
        let Some(value) = assign.get_right().try_as_new() else {
            return;
        };
        let name = fd.get_name();
        let descriptor = fd.get_descriptor();
        if !field.is_static()
            || field.get_instance().is_some()
            || field.get_classname() != cl.qualified_name()
            || field.get_name() != name
            || field.get_descriptor().descriptor_string != descriptor
            || multi_assign.contains(&name)
            || !constructs(value, &name, ordinal)
        {
            return;
        }
        let key = InterpreterUtil::make_unique_key(&name, &descriptor);
        if initializers.borrow().contains_key(&key) {
            return;
        }
        let index = fields.get_index_by_key(&key) as i32;
        if !independent(field, value.0, whitelist, index) {
            return;
        }
        entries.push((field, value, key));
    }
    for (i, (field, value, key)) in entries.into_iter().enumerate() {
        initializers.borrow_mut().add_with_key(value.0, key.clone());
        whitelist.add(key);
        elements.set(i, field.0);
    }
}

/// The elements of a first statement `S = new C[]{..}` storing into a static
/// field of `cl` whose type is an array of `cl`.
fn values_array(cl: StructClass, first_exprents: &JList<Exprent>) -> Option<JList<Exprent>> {
    let assign = first_exprents.first()?.try_as_assignment()?;
    let field = assign.get_left().try_as_field()?;
    let array = assign.get_right().try_as_new()?;
    let own = format!("[L{};", cl.qualified_name());
    let descriptor = field.get_descriptor().descriptor_string.clone();
    if !field.is_static()
        || field.get_classname() != cl.qualified_name()
        || descriptor != own
        || !cl.has_field(&field.get_name(), &descriptor)
    {
        return None;
    }
    let elements = array.get_lst_array_elements();
    (!elements.is_empty()).then_some(elements)
}

/// Whether `value` is a constructor call whose first two arguments are the
/// constant's name and ordinal.
fn constructs(value: NewExprent, name: &str, ordinal: usize) -> bool {
    let Some(constructor) = value.get_constructor() else {
        return false;
    };
    let params = constructor.get_lst_parameters();
    if params.len() < 2 {
        return false;
    }
    let named = params
        .get(0)
        .try_as_const()
        .and_then(|c| c.get_value())
        .is_some_and(|v| matches!(v, ConstValue::String(s) if s == name));
    let numbered = params
        .get(1)
        .try_as_const()
        .and_then(|c| c.get_value())
        .is_some_and(|v| matches!(v, ConstValue::Int(i) if i as usize == ordinal && i >= 0));
    named && numbered
}
