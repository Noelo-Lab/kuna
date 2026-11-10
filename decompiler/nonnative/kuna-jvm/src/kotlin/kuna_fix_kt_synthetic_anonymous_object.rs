//! kuna fix `kt_synthetic_anonymous_object`: writing an anonymous
//! class whose Kotlin metadata is not a Class.
//!
//! kotlinc gives some anonymous classes only synthetic-class metadata
//! (`k = 3`) with no data: the class of a lambda or SAM conversion it cannot
//! compile to `invokedynamic` (a `Comparator { .. }`, `fun interface` or
//! function-type lambda in an inline function that uses a `crossinline`
//! parameter), a suspend function's continuation, a reference to a local
//! function. `KotlinWriter` reads an anonymous object's supertypes from its
//! Class metadata and throws without one. With the fix it writes such a
//! class as an `object` expression from the class file: the supertypes as it
//! writes those of a named class without metadata, and the methods, which
//! have no metadata either, with `write_method`. Two things differ from how
//! `write_method` treats the methods of a named class, because here they
//! stand where an object's functions stand: a `return` is printed as in a
//! function (`write_method` installs the method as the current method
//! wrapper, which `KExitExprent` takes for a lambda body and drops the
//! `return`), and a method that fails keeps its failure comment
//! (`write_method_body` hides a method whose body it could not write).
//!
//! An `object` expression has no superclass constructor call and no
//! constructor parameters, and it names what it captures, so only an object
//! that needs neither and whose names mean what the `new` passes is written:
//! its superclass is `Object`, every constructor parameter is a captured
//! variable, and at this `new` each one is passed the variable of the name
//! the object's body gives it. That holds where `NestedClassProcessor` found
//! the captures, in the class's enclosing method; kotlinc also creates the
//! object of one inline function in the inline functions that call it, where
//! it holds only when the caller's variable has the same name. Any other
//! such class still fails as in Vineflower.

use std::cell::Cell;

use crate::classes_processor::{ClassNode, ClassNodeType};
use crate::code_constants::INIT_NAME;
use crate::decompiler_context::DecompilerContext;
use crate::expr_processor::ExprProcessor;
use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;
use crate::kuna_fixes::{self, KunaFix};
use crate::method_descriptor::MethodDescriptor;
use crate::method_wrapper::MethodWrapper;
use crate::p8_structure::kuna_fix_anonymous_class_created_elsewhere;
use crate::p8_structure::kuna_fix_captured_var_clash_name;
use crate::p8_structure::kuna_fix_captured_var_through_cast;
use crate::text_buffer::TextBuffer;
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;
use crate::var_version_pair::VarVersionPair;

thread_local! {
    static DECLARATION: Cell<Option<MethodWrapper>> = const { Cell::new(None) };
    static SITE: Cell<Option<ClassNode>> = const { Cell::new(None) };
}

/// Restores a thread-local cell when dropped (also on an unwinding throw).
struct Restore<T: Copy + 'static>(&'static std::thread::LocalKey<Cell<Option<T>>>, Option<T>);

impl<T: Copy + 'static> Drop for Restore<T> {
    fn drop(&mut self) {
        self.0.with(|c| c.set(self.1));
    }
}

/// Runs `write` (`KNewExprent` writing the anonymous class `node` it
/// creates with `constructor`), recording whether the object's captures read
/// right at this `new`, for [`is_writable`].
pub fn write_at_site<R>(
    node: ClassNode,
    constructor: Option<InvocationExprent>,
    write: impl FnOnce() -> R,
) -> R {
    let site = captures_match(node, constructor).then_some(node);
    let _restore = Restore(&SITE, SITE.with(|c| c.replace(site)));
    write()
}

/// Whether every argument `constructor` passes is the variable of the name
/// `node`'s body gives the captured variable it fills.
fn captures_match(node: ClassNode, constructor: Option<InvocationExprent>) -> bool {
    let (Some(constructor), Some(wrapper)) = (constructor, node.get_wrapper()) else {
        return false;
    };
    let Some(init) = wrapper.get_method_wrapper(INIT_NAME, &constructor.get_string_descriptor())
    else {
        return false;
    };
    let args = constructor.get_lst_parameters().to_vec();
    let Some(mask) = init.with(|d| d.synth_parameters.clone()) else {
        return args.is_empty();
    };
    enclosing_method(node).is_some()
        && mask.len() == args.len()
        && mask
            .iter()
            .zip(&args)
            .all(|(pair, arg)| pair.is_some_and(|pair| capture_reads_at_site(node, pair, *arg)))
}

/// The method of `node`'s parent that declares it.
fn enclosing_method(node: ClassNode) -> Option<MethodWrapper> {
    let (parent, enclosing_method) = node.with(|d| (d.parent, d.enclosing_method.clone()));
    parent
        .and_then(|p| p.get_wrapper())
        .zip(enclosing_method)
        .and_then(|(w, key)| w.get_methods().borrow().get_with_key(&key))
}

/// Whether `arg`, passed at the `new` being written for the captured variable
/// `pair` of `node`'s enclosing method, is the variable of the name `node`'s
/// body gives it.
pub fn capture_reads_at_site(node: ClassNode, pair: VarVersionPair, arg: Exprent) -> bool {
    let Some(enclosing) = enclosing_method(node) else {
        return false;
    };
    let parent = node.with(|d| d.parent);
    if !arg.is_var()
        && kuna_fixes::enabled(KunaFix::AnonymousClassCreatedElsewhere)
        && kuna_fix_anonymous_class_created_elsewhere::field_capture_reads_at_site(node, pair, arg)
    {
        return true;
    }
    let arg = match kuna_fix_captured_var_through_cast::cast_var(arg) {
        Some(var) if kuna_fixes::enabled(KunaFix::CapturedVarThroughCast) => var.into(),
        _ => arg,
    };
    if pair.var == -1 || !arg.is_var() {
        return false;
    }
    let varproc = enclosing.with(|d| d.varproc);
    let clashing = kuna_fixes::enabled(KunaFix::CapturedVarClashName)
        .then(|| kuna_fix_captured_var_clash_name::clashing_name(varproc, pair))
        .flatten();
    let Some(name) = clashing.or_else(|| varproc.get_var_name(pair)) else {
        return false;
    };
    let arg = arg.as_var();
    if name == "this" {
        arg.get_index() == 0
            && VarExprent::get_name_base(arg) == "this"
            && DecompilerContext::get_current_class_node() == parent
    } else {
        VarExprent::get_name_base(arg) == name
    }
}

/// Whether `node` is an anonymous class this module writes completely as an
/// `object` expression: its superclass is `Object`, every parameter of its
/// constructors is a captured variable (synthetic), and the `new` being
/// written passes each the variable of the name the body uses
/// ([`write_at_site`]).
pub fn is_writable(node: ClassNode) -> bool {
    let (type_, cl) = node.with(|d| (d.type_, d.class_struct));
    if type_ != ClassNodeType::Anonymous
        || SITE.with(Cell::get) != Some(node)
        || !cl
            .super_class()
            .is_none_or(|s| &*s.get_string() == "java/lang/Object")
    {
        return false;
    }
    let Some(wrapper) = node.get_wrapper() else {
        return false;
    };
    let methods = wrapper.get_methods().borrow().to_vec();
    methods.into_iter().all(|method| {
        let (mt, mask) = method.with(|d| (d.method_struct, d.synth_parameters.clone()));
        if mt.get_name() != INIT_NAME {
            return true;
        }
        let params = MethodDescriptor::parse_descriptor(&mt.get_descriptor())
            .params
            .len();
        match mask {
            None => params == 0,
            Some(mask) => mask.len() == params && mask.iter().all(Option::is_some),
        }
    })
}

/// Appends `object` and the class's supertypes other than `Object`, from its
/// generic signature when it has one, as the Class-metadata branch of
/// `writeClassDefinition` lays them out (` : A, B`).
pub fn write_object_header(node: ClassNode, buffer: &mut TextBuffer) {
    write_object_header_with(node, None, buffer);
}

/// [`write_object_header`], with `superclass_arguments` (when given) written
/// in parentheses after the superclass.
pub fn write_object_header_with(
    node: ClassNode,
    superclass_arguments: Option<&TextBuffer>,
    buffer: &mut TextBuffer,
) {
    let cl = node.with(|d| d.class_struct);
    let descriptor = cl.get_signature();
    let superclass = cl
        .super_class()
        .map(|s| VarType::new_signature_cl(&s.get_string(), true));
    let interface_names = cl.get_interface_names();
    if let (Some(descriptor), Some(superclass)) = (&descriptor, &superclass) {
        let interfaces: Vec<VarType> = interface_names
            .iter()
            .map(|s| VarType::new_signature_cl(s, true))
            .collect();
        descriptor.verify_types(superclass, &interfaces);
    }

    let mut supertypes: Vec<VarType> = Vec::new();
    let mut with_arguments = None;
    if let Some(superclass) = superclass {
        if !VarType::VARTYPE_OBJECT().equals(&superclass) {
            with_arguments = superclass_arguments.map(|_| 0);
            supertypes.push(match &descriptor {
                Some(descriptor) => descriptor.superclass(),
                None => superclass,
            });
        }
    }
    for (i, name) in interface_names.iter().enumerate() {
        let type_ = match &descriptor {
            Some(descriptor) => descriptor.superinterfaces().get(i).cloned(),
            None => Some(VarType::new_signature_cl(name, true)),
        };
        supertypes.extend(type_);
    }

    buffer.append("object");
    for (i, type_) in supertypes.iter().enumerate() {
        buffer.append(if i == 0 { " : " } else { ", " });
        buffer.append(&ExprProcessor::get_cast_type_name(type_));
        if with_arguments == Some(i) {
            if let Some(arguments) = superclass_arguments {
                buffer
                    .append_char('(')
                    .append_buffer(arguments)
                    .append_char(')');
            }
        }
    }
}

/// Runs `write` (the `write_method` of one of the object's methods) with
/// `method` as the declaration being written, for [`is_declaration`].
pub fn write_declaration<R>(method: MethodWrapper, write: impl FnOnce() -> R) -> R {
    let _restore = Restore(&DECLARATION, DECLARATION.with(|c| c.replace(Some(method))));
    write()
}

/// Whether `method`, the current method wrapper, is a method of an object
/// this module writes rather than a lambda body: its `return`s are printed.
pub fn is_declaration(method: MethodWrapper) -> bool {
    DECLARATION.with(|d| d.get() == Some(method))
}
