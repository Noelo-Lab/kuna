//! kuna fix `kt_facade_call`: a top-level function or property is called by
//! its name, with the import of its package-level declaration, not through
//! the JVM class that holds it.
//!
//! kotlinc compiles a file's top-level declarations into a file facade
//! (`UtilKt`, or the `@file:JvmName`), and the files of a
//! `@JvmMultifileClass` into parts behind one facade (`StringsKt` delegates
//! to `StringsKt__StringsKt` ..), and calls them with `invokestatic` on the
//! facade. Vineflower writes such a call as Java does, `Util.format(..)`,
//! and reads a `@JvmField` or `const` as `Util.EMPTY_RESPONSE`: Kotlin has no
//! name for a facade class, so neither resolves. A top-level property's
//! accessor call it writes by the property's bare name without its import,
//! which can name a local variable of that name instead. Vineflower takes
//! the callee's metadata from the facade's own method only, which for a
//! multifile facade has none, so those calls lose every Kotlin form.
//!
//! With the fix the callee of a call through a multifile facade is the part
//! method of that name and descriptor (from the library on the class path
//! when the facade is not in the input; a name an inline value class
//! mangled is left alone), and a callee of a file facade, a multifile
//! facade or a part is top-level: it is written by its Kotlin name (a
//! property by its name, assigned for a setter), and its package's
//! declaration of that name is imported, which is not written for the
//! current package and for Kotlin's default imports. The name is written
//! bare only where it can mean nothing else: no other import of the file
//! holds that name, no member of a class whose members are in scope there
//! (the class written, its outer classes, their companions and supertypes)
//! and no local variable of their methods' local variable tables has it,
//! and, for a callee of a default import, the file's package declares no
//! class or top-level declaration of that name; else the call is qualified
//! with the package (`okio.buffer`). A call passing a parameter its method's
//! header declares with an inline value class type keeps Vineflower's form
//! (`value_class_argument`): Kotlin may pass it to another overload. An
//! extension is the fix
//! `kt_extension_call`'s. A callee without metadata (its class neither in
//! the input nor on the class path) keeps Vineflower's form.

use std::rc::Rc;

use crate::classes_processor::ClassNode;
use crate::code_constants::{ACC_STATIC, ACC_SYNTHETIC};
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::k_element::KElement;
use crate::k_function::KFunction;
use crate::k_property::KProperty;
use crate::k_type::KType;
use crate::kotlin::kuna_fix_kt_extension_call;
use crate::kotlin::kuna_fix_kt_unescaped_name;
use crate::kotlin_chooser::KotlinChooser;
use crate::kotlin_import_collector::KotlinImportCollector;
use crate::key::Key;
use crate::kt_attrs::KtAttrs;
use crate::kt_flags::Flags;
use crate::kt_metadata::KotlinMetadataAnnotation;
use crate::kuna_fix_kt_explicit_type_arguments;
use crate::kuna_fixes::{self, KunaFix};
use crate::struct_class::StructClass;
use crate::struct_method::StructMethod;
use crate::text_buffer::TextBuffer;
use crate::var_version_pair::VarVersionPair;

/// The method of a multifile facade's part that a call of the facade `cl`'s
/// `name` and `descriptor` reaches, when it carries Kotlin metadata.
pub fn part_method(cl: StructClass, name: &str, descriptor: &str) -> Option<StructMethod> {
    KotlinChooser::parse_metadata_for(cl);
    let Some(KElement::MultifileFacade(facade)) = KtAttrs::get(cl, &KElement::KEY()) else {
        return None;
    };
    let context = DecompilerContext::get_struct_context();
    facade
        .files
        .iter()
        .filter_map(|part| context.get_class(part))
        .find_map(|part| {
            KotlinChooser::parse_metadata_for(part);
            part.get_method(name, descriptor)
                .filter(|m| KtAttrs::has(*m, &KElement::KEY()))
        })
}

/// The Kotlin package of the top-level declarations of `classname`, when it
/// is a file facade, a multifile facade or a part: the metadata's package
/// name (`@JvmPackageName`), else the class's package.
pub fn top_level_package(classname: &str) -> Option<String> {
    let cl = DecompilerContext::get_struct_context().get_class(classname)?;
    KotlinChooser::parse_metadata_for(cl);
    match KtAttrs::get(cl, &KElement::KEY()) {
        Some(KElement::File(_)) | Some(KElement::MultifileFacade(_)) => {}
        _ => return None,
    }
    let package_name = KotlinMetadataAnnotation::find(cl).and_then(|anno| {
        let index = anno.index_of("pn");
        (index >= 0).then(|| KotlinMetadataAnnotation::string_value(anno.get_par_value(index)))
    });
    Some(package_name.unwrap_or_else(|| {
        classname
            .rfind('/')
            .map_or(String::new(), |i| classname[..i].replace('/', "."))
    }))
}

/// Whether Kotlin imports `package`'s declarations into every file.
pub fn default_import(package: &str) -> bool {
    package == "kotlin"
        || package == "java.lang"
        || package.strip_prefix("kotlin.").is_some_and(|rest| {
            KotlinImportCollector::AUTO_KOTLIN_IMPORTS.contains(&rest)
        })
}

/// The package of the file being written, dotted.
fn current_package() -> String {
    DecompilerContext::get_import_collector().with(|d| d.current_package_point.clone())
}

/// Whether the file's import map lets `name` stand for `package`'s
/// declaration: no other package holds the simple name (which this then
/// claims), and, for a default import, the file's package declares nothing
/// of that name.
pub fn claim_name(package: &str, name: &str) -> bool {
    let collector = DecompilerContext::get_import_collector();
    let key = name.to_string();
    if let Some(held) = collector.with(|d| d.map_simple_names.get_cloned(&key)) {
        return held == package;
    }
    let current = current_package();
    if package != current && default_import(package) && package_declares(&current, name) {
        return false;
    }
    collector.with_mut(|d| {
        if !d.write_locked {
            d.map_simple_names.put(key, package.to_string());
        }
    });
    true
}

/// Whether `package` (of the input) declares a class or a top-level
/// function or property named `name`.
fn package_declares(package: &str, name: &str) -> bool {
    let slash = package.replace('.', "/");
    let prefix = if slash.is_empty() {
        String::new()
    } else {
        format!("{slash}/")
    };
    let context = DecompilerContext::get_struct_context();
    if context.get_class(&format!("{prefix}{name}")).is_some() {
        return true;
    }
    let Some(root) = root_class() else {
        return true;
    };
    let key: Key<Rc<Vec<String>>> = Key::of(PACKAGE_NAMES);
    let names = KtAttrs::get(root, &key).unwrap_or_else(|| {
        let roots: Vec<ClassNode> = DecompilerContext::get_class_processor()
            .get_map_root_classes()
            .borrow()
            .values()
            .copied()
            .collect();
        let mut names = Vec::new();
        for node in roots {
            let cl = node.with(|d| d.class_struct);
            let in_package = cl
                .qualified_name()
                .strip_prefix(&prefix)
                .is_some_and(|rest| !rest.contains('/'));
            if in_package {
                names.extend(top_level_names(cl));
            }
        }
        let names = Rc::new(names);
        KtAttrs::put(root, &key, names.clone());
        names
    });
    names.iter().any(|n| n == name)
}

const PACKAGE_NAMES: &str = "kuna.kt_facade_call.package_names";

/// The top-level class of the file being written.
fn root_class() -> Option<StructClass> {
    let mut node = DecompilerContext::get_current_class_node()?;
    while let Some(parent) = node.with(|d| d.parent) {
        node = parent;
    }
    Some(node.with(|d| d.class_struct))
}

/// The file facades and multifile parts of the input in `package`.
pub fn package_facades(package: &str) -> Vec<StructClass> {
    let prefix = match package.replace('.', "/") {
        slash if slash.is_empty() => slash,
        slash => format!("{slash}/"),
    };
    let roots: Vec<ClassNode> = DecompilerContext::get_class_processor()
        .get_map_root_classes()
        .borrow()
        .values()
        .copied()
        .collect();
    roots
        .into_iter()
        .map(|node| node.with(|d| d.class_struct))
        .filter(|cl| {
            cl.qualified_name()
                .strip_prefix(&prefix)
                .is_some_and(|rest| !rest.contains('/'))
                && {
                    KotlinChooser::parse_metadata_for(*cl);
                    matches!(KtAttrs::get(*cl, &KElement::KEY()), Some(KElement::File(_)))
                }
        })
        .collect()
}

/// The names of the top-level functions and properties a file facade or
/// part declares.
fn top_level_names(cl: StructClass) -> Vec<String> {
    KotlinChooser::parse_metadata_for(cl);
    if !matches!(KtAttrs::get(cl, &KElement::KEY()), Some(KElement::File(_))) {
        return Vec::new();
    }
    let mut names = Vec::new();
    for mt in cl.get_methods().to_vec() {
        match KtAttrs::get(mt, &KElement::KEY()) {
            Some(KElement::Function(f)) => names.push(f.name.clone()),
            Some(KElement::Property(p)) => names.push(p.name.clone()),
            _ => {}
        }
    }
    for fd in cl.get_fields().to_vec() {
        names.push(fd.get_name());
    }
    names
}

/// The classes whose members are in scope in the class being written: it,
/// its outer classes and the companions of each (a file facade's
/// declarations are top-level, not members).
pub fn scope_classes() -> Vec<StructClass> {
    let mut out = Vec::new();
    let mut node = DecompilerContext::get_current_class_node();
    while let Some(n) = node {
        let cl = n.with(|d| d.class_struct);
        KotlinChooser::parse_metadata_for(cl);
        if !matches!(
            KtAttrs::get(cl, &KElement::KEY()),
            Some(KElement::File(_)) | Some(KElement::MultifileFacade(_))
        ) {
            out.push(cl);
            out.extend(companion(cl));
        }
        node = n.with(|d| d.parent);
    }
    out
}

/// The companion object of `cl`, by its metadata.
pub fn companion(cl: StructClass) -> Option<StructClass> {
    KotlinChooser::parse_metadata_for(cl);
    let Some(KElement::Class(kclass)) = KtAttrs::get(cl, &KElement::KEY()) else {
        return None;
    };
    if !kclass.proto.has_companion_object_name() {
        return None;
    }
    let name = kclass
        .resolver
        .as_ref()?
        .resolve(kclass.proto.get_companion_object_name());
    DecompilerContext::get_struct_context().get_class(&format!("{}${}", cl.qualified_name(), name))
}

/// `cl` and its loaded supertypes.
pub fn supertypes(cl: StructClass) -> Vec<StructClass> {
    let context = DecompilerContext::get_struct_context();
    let mut out: Vec<StructClass> = Vec::new();
    let mut work = vec![cl];
    while let Some(c) = work.pop() {
        if out.contains(&c) {
            continue;
        }
        out.push(c);
        if let Some(s) = c.super_class() {
            work.extend(context.get_class(&s.get_string()));
        }
        work.extend(
            c.get_interface_names()
                .iter()
                .filter_map(|i| context.get_class(i)),
        );
    }
    out
}

/// Whether a member of `cl` or of a supertype may be named `name` in Kotlin
/// (a function, a property, a field or a Java getter's property).
pub fn declares_member(cl: StructClass, name: &str) -> bool {
    supertypes(cl).into_iter().any(|c| {
        KotlinChooser::parse_metadata_for(c);
        let methods = c.get_methods().to_vec();
        let fields = c.get_fields().to_vec();
        methods.into_iter().any(|mt| {
            if mt.get_name() == name || java_property(&mt.get_name()).as_deref() == Some(name) {
                return true;
            }
            match KtAttrs::get(mt, &KElement::KEY()) {
                Some(KElement::Function(f)) => f.name == name,
                Some(KElement::Property(p)) => p.name == name,
                _ => false,
            }
        }) || fields.into_iter().any(|fd| fd.get_name() == name)
    })
}

/// The property name Kotlin gives a Java getter `getFoo`/`isFoo`.
pub fn java_property(method: &str) -> Option<String> {
    if let Some(rest) = method.strip_prefix("get").filter(|r| starts_upper(r)) {
        return Some(decapitalize(rest));
    }
    method
        .strip_prefix("is")
        .filter(|r| starts_upper(r))
        .map(|_| method.to_string())
}

fn starts_upper(s: &str) -> bool {
    s.chars().next().is_some_and(char::is_uppercase)
}

fn decapitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_lowercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// Whether a variable in scope where the class is written may be named
/// `name`: a local variable table entry of a method of the class or of an
/// outer class (the Kotlin writer does not say which method it writes).
pub fn variable_named(name: &str) -> bool {
    let mut node = DecompilerContext::get_current_class_node();
    while let Some(n) = node {
        if local_names(n.with(|d| d.class_struct)).iter().any(|v| v == name) {
            return true;
        }
        node = n.with(|d| d.parent);
    }
    false
}

const LOCAL_NAMES: &str = "kuna.kt_facade_call.local_names";

/// The names of the local variable tables of `cl`'s methods.
fn local_names(cl: StructClass) -> Rc<Vec<String>> {
    let key: Key<Rc<Vec<String>>> = Key::of(LOCAL_NAMES);
    if let Some(names) = KtAttrs::get(cl, &key) {
        return names;
    }
    let mut names: Vec<String> = Vec::new();
    for mt in cl.get_methods().to_vec() {
        if let Some(lvt) = mt.get_local_variable_attr() {
            names.extend(lvt.get_variables().into_iter().map(|v| v.name));
        }
    }
    let names = Rc::new(names);
    KtAttrs::put(cl, &key, names.clone());
    names
}

/// Whether the bare `name` of a top-level declaration may mean another
/// member or variable where the class is written.
fn shadowed(name: &str) -> bool {
    variable_named(name) || scope_classes().into_iter().any(|cl| declares_member(cl, name))
}

/// How a top-level declaration's name is written: bare (`Some("")`), with
/// its package (`Some("pkg.")`), or not at all.
pub fn qualifier(package: &str, name: &str) -> Option<String> {
    if !shadowed(name) && claim_name(package, name) {
        return Some(String::new());
    }
    (!package.is_empty()).then(|| format!("{package}."))
}

/// The class `this` is in `expr`, when `expr` is a `this` of the method
/// being written.
pub fn this_class(expr: Exprent) -> Option<String> {
    let var = expr.try_as_var()?;
    let pair = VarVersionPair::from_var(var);
    let processor = var.get_processor().or_else(|| {
        DecompilerContext::get_current_method_wrapper().map(|mw| mw.with(|d| d.varproc))
    })?;
    let this_vars = processor.get_this_vars();
    let class = this_vars.borrow().get_cloned(&pair);
    class
}

/// Whether `expr` is a parameter (or the extension or value-class receiver)
/// of its method whose Kotlin type is an inline value class: the header
/// gives it that type, and Kotlin resolves a call on it by it, not by the
/// JVM type the bytecode passes.
pub fn value_class_parameter(expr: Exprent) -> bool {
    let Some(var) = expr.try_as_var() else {
        return false;
    };
    let Some(processor) = var.get_processor() else {
        return false;
    };
    let method = processor.get_method();
    let Some(cl) = DecompilerContext::get_struct_context().get_class(&method.get_class_qualified_name()) else {
        return false;
    };
    KotlinChooser::parse_metadata_for(cl);
    let is_static = method.has_modifier(ACC_STATIC);
    let mut slot = 0;
    let mut types: Vec<Option<KType>> = Vec::new();
    let function = match KtAttrs::get(method, &KElement::KEY()) {
        Some(KElement::Function(f)) => Some(f),
        _ => None,
    };
    let in_value_class = matches!(KtAttrs::get(cl, &KElement::KEY()),
        Some(KElement::Class(k)) if Flags::IS_VALUE_CLASS.get(k.flags()));
    if !is_static {
        if var.get_index() == 0 {
            return false;
        }
        slot = 1;
    } else if in_value_class {
        types.push(None);
    }
    if let Some(f) = &function {
        types.extend(f.receiver_type.clone().map(Some));
        types.extend(f.parameters.iter().map(|p| Some(p.type_.clone())));
    }
    let index = var.get_index();
    for (i, param) in method.method_descriptor().params.iter().enumerate() {
        if slot == index {
            return match types.get(i) {
                Some(Some(t)) => kuna_fix_kt_extension_call::value_class(t),
                Some(None) => true,
                None => in_value_class,
            };
        }
        slot += param.stack_size;
    }
    false
}

/// Whether an argument of `invocation` is such a parameter, which Kotlin
/// may pass to another overload of the name the call is written with.
pub fn value_class_argument(invocation: InvocationExprent) -> bool {
    invocation
        .get_lst_parameters()
        .to_vec()
        .into_iter()
        .any(value_class_parameter)
}

/// Whether the JVM name `jvm` is the Kotlin `name` mangled for an inline
/// value class parameter (`f-QeA7wkU`), which a call by the Kotlin name may
/// not reach.
pub fn mangled(jvm: &str, name: &str) -> bool {
    jvm.strip_prefix(name).is_some_and(|rest| rest.starts_with('-'))
}

/// The written call of a top-level function that is not an extension,
/// `name(args)` or `pkg.name(args)`; `None` to leave it to Vineflower.
pub fn write_function(
    invocation: InvocationExprent,
    function: &Rc<KFunction>,
    indent: i32,
) -> Option<TextBuffer> {
    if function.receiver_type.is_some()
        || !function.context_receiver_types.is_empty()
        || mangled(&invocation.get_name(), &function.name)
        || value_class_argument(invocation)
    {
        return None;
    }
    let package = top_level_package(&invocation.get_classname())?;
    let prefix = qualifier(&package, &function.name)?;
    let mut buf = TextBuffer::new();
    buf.append(&prefix);
    buf.append_method_descriptor(
        &kuna_fix_kt_unescaped_name::member(&function.name),
        false,
        &invocation.get_classname(),
        &invocation.get_name(),
        &invocation.get_descriptor(),
    );
    if kuna_fix_kt_explicit_type_arguments::active() {
        kuna_fix_kt_explicit_type_arguments::append(&mut buf, &invocation.get_generic_args());
    }
    buf.append("(");
    buf.append_buffer(&invocation.append_param_list(indent));
    buf.append(")");
    buf.add_bytecode_mapping(invocation.get_bytecode().as_ref());
    Some(buf)
}

/// The name a top-level property is read or assigned by (`name`,
/// `pkg.name`), for a call of its accessor; `None` to leave it to Vineflower.
pub fn property_prefix(invocation: InvocationExprent, property: &KProperty) -> Option<String> {
    let package = top_level_package(&invocation.get_classname())?;
    qualifier(&package, &property.name)
}

/// The written read of a top-level property's field (`@JvmField`, `const`)
/// through its facade, `name` or `pkg.name`.
pub fn write_field(classname: &str, name: &str, descriptor: &str) -> Option<TextBuffer> {
    if !kuna_fixes::enabled(KunaFix::KtFacadeCall) {
        return None;
    }
    let package = top_level_package(classname)?;
    let cl = DecompilerContext::get_struct_context().get_class(classname)?;
    let field = cl.get_field(name, descriptor)?;
    if !field.has_modifier(ACC_STATIC) || field.has_modifier(ACC_SYNTHETIC) {
        return None;
    }
    let prefix = qualifier(&package, name)?;
    let mut buf = TextBuffer::new();
    buf.append(&prefix);
    buf.append_field(
        &kuna_fix_kt_unescaped_name::member(name),
        false,
        classname,
        name,
        descriptor,
    );
    Some(buf)
}
