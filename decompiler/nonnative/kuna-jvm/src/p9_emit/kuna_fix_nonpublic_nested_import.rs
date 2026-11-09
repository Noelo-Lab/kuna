//! kuna fix `nonpublic_nested_import` (`kuna_fixes`): a single-type import must
//! name an accessible type (JLS 7.5.1), and a nested class that is not public
//! is accessible from another package only inside the body of a subclass of
//! its enclosing class, never from an import. `ImportCollector.getShortName`
//! imports every nested class outside the input by its dotted name, so a
//! protected nested class of a library type (`Weight.DefaultScorerSupplier`
//! used inside a `Weight` subclass) becomes an import javac rejects. With the
//! fix, such a class is named through its enclosing class, whose name is
//! shortened (and imported) as usual: `Weight.DefaultScorerSupplier`.

use crate::classes_processor::ClassNode;
use crate::code_constants::ACC_PUBLIC;
use crate::decompiler_context::DecompilerContext;
use crate::import_collector::ImportCollector;
use crate::struct_class::StructClass;
use crate::struct_general_attribute::StructGeneralAttribute;

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
