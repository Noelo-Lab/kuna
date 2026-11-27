//! Fix family `class_writing`: `compact_record_constructor` makes a canonical
//! constructor compact only when it ends with each component's store from its
//! parameter; `enum_body_separator` writes the `;` an enum's members need when
//! no constant precedes them; `enum_constant_in_values_array` takes the
//! constants Kotlin 1.3 builds inside `$VALUES` as their fields' initializers;
//! `external_inherited_member_type` reads the member classes of supertypes
//! outside the input from `InnerClasses` and qualifies an ambiguous name;
//! `nonpublic_nested_import` names a non-public nested library class through
//! its enclosing class; `override_inaccessible_method` writes `@Override`
//! only over a supertype method the class can override;
//! `sealed_private_permits` leaves out a `permits` clause that names a private
//! member class.

use crate::class_wrapper::ClassWrapper;
use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::{ACC_ENUM, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC, ACC_STATIC};
use crate::const_exprent::ConstValue;
use crate::decompiler_context::DecompilerContext;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::Exprent;
use crate::field_exprent::FieldExprent;
use crate::import_collector::ImportCollector;
use crate::interpreter_util::InterpreterUtil;
use crate::method_descriptor::MethodDescriptor;
use crate::new_exprent::NewExprent;
use crate::p3_dataflow::kuna_source_increments;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, JList};
use crate::struct_class::StructClass;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_inner_classes_attribute::Entry;
use crate::struct_method::StructMethod;
use crate::struct_record_component::StructRecordComponent;
use crate::text_buffer::TextBuffer;

/// Whether the one non-throwing exit of `root` ends with an assignment of
/// each component's field from a parameter, in component order.
pub fn assigns_components_last(root: RootStatement, components: &[StructRecordComponent]) -> bool {
    if components.is_empty() {
        return false;
    }
    let mut found = false;
    for edge in root.get_dummy_exit().get_all_predecessor_edges() {
        let exit = edge.get_source();
        if !exit.is_basic_block() {
            continue;
        }
        let Some(exprents) = exit.get_exprents().filter(|e| !e.is_empty()) else {
            continue;
        };
        let last = exprents.get(exprents.len() - 1);
        if last
            .try_as_exit()
            .is_some_and(|e| e.get_exit_type() == ExitType::Throw)
        {
            continue;
        }
        if found || exprents.len() < components.len() {
            return false;
        }
        let offset = exprents.len() - components.len();
        let mut last_index = 0;
        for (i, component) in components.iter().enumerate() {
            let Some(index) = component_store(exprents.get(offset + i), component) else {
                return false;
            };
            if index <= last_index {
                return false;
            }
            last_index = index;
        }
        found = true;
    }
    found
}

/// Whether the field stores that end the one non-throwing exit of `root`
/// (`count` of them) start on another line of the LineNumberTable of `mt` than
/// its first instruction: javac gives a compact constructor's implicit stores
/// the constructor's own line, and the source's `this.c = c;` statements a
/// line each.
pub fn stores_on_own_line(root: RootStatement, mt: StructMethod, count: usize) -> bool {
    let Some(table) = mt.get_attribute(StructGeneralAttribute::ATTRIBUTE_LINE_NUMBER_TABLE) else {
        return false;
    };
    for edge in root.get_dummy_exit().get_all_predecessor_edges() {
        let exit = edge.get_source();
        let Some(exprents) = exit.get_exprents().filter(|e| e.len() >= count && count > 0) else {
            continue;
        };
        let first = exprents.get(exprents.len() - count);
        if !is_this_field_store(first) {
            continue;
        }
        let Some(start) = kuna_source_increments::range(first).map(|r| r.next_set_bit(0)) else {
            continue;
        };
        let (header, line) = (table.find_line_number(0), table.find_line_number(start));
        return header >= 0 && line >= 0 && line != header;
    }
    false
}

/// The variable index of `this.<component> = <variable>`.
fn component_store(e: Exprent, component: &StructRecordComponent) -> Option<i32> {
    let assignment = e.try_as_assignment()?;
    let field = assignment.get_left().try_as_field()?;
    if field.get_name() != component.get_name() || !is_this(field.get_instance()) {
        return None;
    }
    Some(assignment.get_right().try_as_var()?.get_index())
}

/// Whether `e` is an assignment to a field of `this`.
pub fn is_this_field_store(e: Exprent) -> bool {
    e.try_as_assignment()
        .and_then(|a| a.get_left().try_as_field())
        .is_some_and(|f| is_this(f.get_instance()))
}

fn is_this(instance: Option<Exprent>) -> bool {
    instance
        .and_then(|i| i.try_as_var())
        .is_some_and(|v| v.get_index() == 0)
}

/// The `;` a class body written by `ClassWriter::write_class` still owes its members.
pub struct EnumBodySeparator {
    pending: bool,
    indent: i32,
}

impl EnumBodySeparator {
    /// Owed when `node` is declared with the `enum` keyword
    /// (the test of `write_class_definition`); `indent` is the class's.
    pub fn new(node: ClassNode, indent: i32) -> EnumBodySeparator {
        let (type_, access, cl) = node.with(|d| (d.type_, d.access, d.class_struct));
        let flags = if type_ == ClassNodeType::Root {
            cl.get_access_flags()
        } else {
            access
        };
        let pending = type_ != ClassNodeType::Anonymous
            && (flags & ACC_ENUM) != 0;
        EnumBodySeparator {
            pending,
            indent: indent + 1,
        }
    }

    /// A constant was written: the `;` after the constants is Vineflower's.
    pub fn constant_written(&mut self) {
        self.pending = false;
    }

    /// Called before each member: writes the owed `;` line before the first
    /// one and returns true then.
    pub fn before_member(&mut self, buffer: &mut TextBuffer) -> bool {
        if !self.pending {
            return false;
        }
        self.pending = false;
        buffer
            .append_indent(self.indent)
            .append(";")
            .append_line_separator();
        true
    }
}

/// Moves the constants built in a leading `$VALUES = new E[]{F = new E(..), ..}`
/// to their fields' initializers; `independent(field, value, whitelist, index)`
/// is `InitializerProcessor`'s independence test for a static initializer.
pub fn extract_values_array_constants(
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

/// Whether the member class of `entry` is inherited by a class of package
/// `package` (internal name prefix, `a/b/`): it is not private, and a
/// package-private one only within its package (JLS 8.5).
pub fn inherited_by(entry: &Entry, package: &str) -> bool {
    if (entry.access_flags & ACC_PRIVATE) != 0 {
        return false;
    }
    if (entry.access_flags & (ACC_PUBLIC | ACC_PROTECTED)) != 0 {
        return true;
    }
    let enclosing = entry.enclosing_name.as_deref().unwrap_or("");
    let own = enclosing.rfind('/').map_or("", |i| &enclosing[..=i]);
    own == package
}

/// Adds to `names` the member classes of each supertype in `external` (outside
/// the universe) found in `entries`, and marks a simple name with two different
/// inherited member classes (`inherited`, from the supertypes in the
/// universe, and these) as ambiguous: it maps to no class.
pub fn add_external_members(
    names: &mut JHashMap<String, String>,
    inherited: &mut Vec<(String, String)>,
    external: &[String],
    entries: &[Entry],
    package: &str,
) {
    for entry in entries {
        let (Some(enclosing), Some(simple)) = (&entry.enclosing_name, &entry.simple_name) else {
            continue;
        };
        if external.iter().any(|e| e == enclosing) && inherited_by(entry, package) {
            let inner = entry.inner_name.replace(['/', '$'], ".");
            if !inherited.iter().any(|(s, i)| s == simple && *i == inner) {
                inherited.push((simple.clone(), inner.clone()));
                names.put(simple.clone(), inner);
            }
        }
    }
    for (simple, inner) in inherited.iter() {
        if inherited.iter().any(|(s, i)| s == simple && i != inner) {
            names.put(simple.clone(), String::new());
        }
    }
}

/// The member-class entry of `class`'s `InnerClasses` for the class whose
/// dotted name is `dotted`: its internal name, enclosing class's internal
/// name, simple name and access flags.
fn entry_in(class: StructClass, dotted: &str) -> Option<(String, String, String, i32)> {
    let attribute = class.get_attribute(StructGeneralAttribute::ATTRIBUTE_INNER_CLASSES)?;
    attribute.get_entries().into_iter().find_map(|entry| {
        if entry.outer_name_idx == 0 || entry.inner_name.replace(['/', '$'], ".") != dotted {
            return None;
        }
        Some((entry.inner_name, entry.enclosing_name?, entry.simple_name?, entry.access_flags))
    })
}

/// The member-class entry for the class named `dotted`: from the class itself
/// when it is loaded, else from the classes being written (javac records every
/// nested class a class file refers to in that file's `InnerClasses`, and the
/// class being written or one enclosing it is the one that refers to it).
fn member_entry(dotted: &str) -> Option<(String, String, String, i32)> {
    let context = DecompilerContext::get_struct_context();
    let mut cut = dotted.len();
    while let Some(dot) = dotted[..cut].rfind('.') {
        let internal = format!("{}${}", dotted[..dot].replace('.', "/"), dotted[dot + 1..].replace('.', "$"));
        if let Some(class) = context.get_class(&internal) {
            if let Some(entry) = entry_in(class, dotted) {
                return Some(entry);
            }
        }
        cut = dot;
    }
    let mut node: Option<ClassNode> = DecompilerContext::get_current_class_node();
    while let Some(current) = node {
        if let Some(entry) = entry_in(current.with(|d| d.class_struct), dotted) {
            return Some(entry);
        }
        node = current.with(|d| d.parent);
    }
    None
}

fn package(internal_name: &str) -> &str {
    internal_name.rfind('/').map_or("", |i| &internal_name[..i])
}

/// The name to write for `full_name` (a class outside the input, `$` or `.`
/// separating nested names) when it is a member class in another package than
/// `collector`'s class that is not public or has an enclosing class that is
/// not: the enclosing class's name (shortened the same way), a dot and the
/// simple name. `None` when the class may be imported.
pub fn qualified_name(collector: ImportCollector, full_name: &str, imported: bool) -> Option<String> {
    let dotted = full_name.replace('$', ".");
    let (_, outer, simple, _) = member_entry(&dotted)?;
    let current_package = collector.with(|d| d.current_package_slash.clone());
    if package(&outer) == current_package.trim_end_matches('/') {
        return None;
    }
    let mut level = Some(dotted);
    loop {
        let (_, enclosing, _, flags) = member_entry(level.as_deref()?)?;
        if flags & ACC_PUBLIC == 0 {
            break;
        }
        level = Some(enclosing.replace(['/', '$'], "."));
    }
    let outer_name = collector.get_short_name_imported(&outer.replace('/', "."), imported)?;
    Some(format!("{outer_name}.{simple}"))
}

fn overridable(method: StructMethod, declaring: StructClass, origin_package: &str) -> bool {
    !method.has_modifier(ACC_PRIVATE)
        && (method.has_modifier(ACC_PUBLIC)
            || method.has_modifier(ACC_PROTECTED)
            || package(&declaring.qualified_name()) == origin_package)
}

/// Whether a supertype of `cl` declares a method that a method `name` with
/// descriptor `md` declared in `cl` overrides.
pub fn overrides(cl: StructClass, name: &str, md: &MethodDescriptor) -> bool {
    let qualified_name = cl.qualified_name();
    search(Some(cl), name, md, false, package(&qualified_name))
}

fn search(
    cl: Option<StructClass>,
    name: &str,
    md: &MethodDescriptor,
    search_here: bool,
    origin_package: &str,
) -> bool {
    let Some(cl) = cl else {
        return false;
    };
    if search_here
        && cl.get_methods().to_vec().into_iter().any(|method| {
            md.equals(&MethodDescriptor::parse_descriptor(&method.get_descriptor()))
                && name == method.get_name()
                && !method.has_modifier(ACC_STATIC)
                && overridable(method, cl, origin_package)
        })
    {
        return true;
    }
    let context = DecompilerContext::get_struct_context();
    if let Some(super_class) = cl.super_class() {
        if search(context.get_class(&super_class.get_string()), name, md, true, origin_package) {
            return true;
        }
    }
    cl.get_interface_names()
        .into_iter()
        .any(|iface| search(context.get_class(&iface), name, md, true, origin_package))
}

/// Whether `node`'s `permits` clause, naming `permitted`, is left out.
pub fn omit_permits(node: ClassNode, permitted: &[String]) -> bool {
    if node.with(|d| d.type_) != ClassNodeType::Root {
        return false;
    }
    let roots = DecompilerContext::get_class_processor().get_map_root_classes();
    let mut private = false;
    for name in permitted {
        let Some(mut current) = roots.borrow().get(name).copied() else {
            return false;
        };
        loop {
            if current == node {
                break;
            }
            if current.with(|d| d.type_) != ClassNodeType::Member {
                return false;
            }
            private |= current.with(|d| d.access) & ACC_PRIVATE != 0;
            match current.with(|d| d.parent) {
                Some(parent) => current = parent,
                None => return false,
            }
        }
    }
    private
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_the_separator_once_and_only_when_owed() {
        let mut buffer = TextBuffer::with_format("\n", "   ", 160);
        let mut sep = EnumBodySeparator {
            pending: true,
            indent: 1,
        };
        assert!(sep.before_member(&mut buffer));
        assert!(!sep.before_member(&mut buffer));
        assert!(buffer.content_equals("   ;\n"));

        let mut buffer = TextBuffer::with_format("\n", "   ", 160);
        let mut sep = EnumBodySeparator {
            pending: true,
            indent: 1,
        };
        sep.constant_written();
        assert!(!sep.before_member(&mut buffer));
        assert!(buffer.content_equals(""));
    }
}
