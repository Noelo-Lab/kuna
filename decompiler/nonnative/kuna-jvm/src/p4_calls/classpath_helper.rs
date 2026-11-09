//! Port of `org/jetbrains/java/decompiler/modules/decompiler/ClasspathHelper.java`.
//!
//! Vineflower resolves a method of a class missing from the struct context by
//! reflection on the JVM it runs on: the class from the bootstrap class
//! loader, then the first of its `getMethods()` whose `Method.toString()`
//! contains the wanted signature. There is no JVM here, so the port asks the
//! decompiler's own class universe instead: the class comes from the struct
//! context (where the JDK runtime lives when `include-runtime` is on),
//! `getMethods()` is rebuilt from the class files of the class and its
//! supertypes the way `java.lang.PublicMethods` merges them, and each
//! `toString()` is formatted from the class file as reflection prints it. A
//! class the struct context cannot load answers `null`, as a failing
//! `loadClass` does. Because answers depend on the struct context, the cache
//! is dropped when the context changes; within one context it behaves as the
//! JVM-wide Java cache.

use std::cell::RefCell;

use crate::code_constants::{
    ACC_ABSTRACT, ACC_FINAL, ACC_INTERFACE, ACC_NATIVE, ACC_PRIVATE, ACC_PROTECTED, ACC_PUBLIC,
    ACC_STATIC, ACC_STRICT, ACC_SYNCHRONIZED, ACC_VARARGS, CLINIT_NAME, INIT_NAME,
};
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::method_descriptor::MethodDescriptor;
use crate::rt::{rethrow, try_catch, JHashMap, JLinkedHashMap};
use crate::struct_class::StructClass;
use crate::struct_context::StructContext;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_method::StructMethod;
use crate::var_type::VarType;

pub struct ClasspathHelper;

/// The facts of a `java.lang.reflect.Method` that Vineflower reads.
#[derive(Clone, Debug)]
pub struct Method {
    /// `Method.toString()`, e.g. `public static java.lang.String java.lang.String.valueOf(java.lang.Object)`.
    pub to_string: String,
    pub var_args: bool,
}

impl Method {
    pub fn is_var_args(&self) -> bool {
        self.var_args
    }
}

type MethodCache = (Option<StructContext>, JHashMap<String, Option<Method>>);

thread_local! {
    static METHOD_CACHE: RefCell<MethodCache> = const { RefCell::new((None, JHashMap::new())) };
}

/// `Modifier.methodModifiers()`.
const METHOD_MODIFIERS: i32 = ACC_PUBLIC
    | ACC_PROTECTED
    | ACC_PRIVATE
    | ACC_ABSTRACT
    | ACC_STATIC
    | ACC_FINAL
    | ACC_SYNCHRONIZED
    | ACC_NATIVE
    | ACC_STRICT;

/// `Modifier.ACCESS_MODIFIERS`.
const ACCESS_MODIFIERS: i32 = ACC_PUBLIC | ACC_PROTECTED | ACC_PRIVATE;

/// A declared method together with its declaring class.
type ReflectedMethod = (StructClass, StructMethod);

impl ClasspathHelper {
    pub fn find_method(
        classname: &str,
        method_name: &str,
        descriptor: &MethodDescriptor,
    ) -> Option<Method> {
        let target_class = classname.replace('/', ".");
        let method_signature =
            Self::build_method_signature(&format!("{target_class}.{method_name}"), descriptor);

        let context = DecompilerContext::get_struct_context();
        let cached = METHOD_CACHE.with(|c| {
            let mut cache = c.borrow_mut();
            if cache.0 != Some(context) {
                *cache = (Some(context), JHashMap::new());
            }
            cache
                .1
                .contains_key(&method_signature)
                .then(|| cache.1.get_cloned(&method_signature).flatten())
        });
        match cached {
            Some(method) => method,
            None => {
                let method = Self::find_method_on_classpath(&target_class, &method_signature);
                METHOD_CACHE.with(|c| c.borrow_mut().1.put(method_signature, method.clone()));
                method
            }
        }
    }

    fn find_method_on_classpath(target_class: &str, method_signature: &str) -> Option<Method> {
        let found = try_catch(|| {
            let cls = DecompilerContext::get_struct_context()
                .get_class(&target_class.replace('.', "/"))?;
            Self::get_methods(cls)
                .into_iter()
                .find(|mtd| mtd.to_string.contains(method_signature))
        });
        match found {
            Ok(method) => method,
            Err(caught) if caught.is("UnportedError") => rethrow(caught),
            Err(_) => None,
        }
    }

    /// `cls.getMethods()`, each as its `Method`.
    fn get_methods(cls: StructClass) -> Vec<Method> {
        Self::public_methods(cls)
            .into_iter()
            .map(|(declaring, mtd)| Self::reflect(declaring, mtd))
            .collect()
    }

    /// `Class.privateGetPublicMethods()`: the public methods `cls` declares
    /// (initializers excluded; class-file order, where HotSpot's differs),
    /// then a class's superclass's, then each direct superinterface's
    /// non-static ones, merged as `PublicMethods.merge` does. Supertypes the
    /// struct context cannot load are skipped.
    fn public_methods(cls: StructClass) -> Vec<ReflectedMethod> {
        let context = DecompilerContext::get_struct_context();
        let mut pms: JLinkedHashMap<(String, String), Vec<ReflectedMethod>> = JLinkedHashMap::new();

        let methods = cls.get_methods();
        for i in 0..methods.size() {
            let mtd = methods.get(i);
            let name = mtd.get_name();
            if mtd.has_modifier(ACC_PUBLIC) && name != INIT_NAME && name != CLINIT_NAME {
                Self::merge(context, &mut pms, (cls, mtd));
            }
        }

        if !cls.has_modifier(ACC_INTERFACE) {
            let super_class = cls
                .super_class()
                .and_then(|sc| context.get_class(&sc.get_string()));
            if let Some(super_class) = super_class {
                for m in Self::public_methods(super_class) {
                    Self::merge(context, &mut pms, m);
                }
            }
        }

        for name in cls.get_interface_names() {
            if let Some(intf) = context.get_class(&name) {
                for m in Self::public_methods(intf) {
                    if !m.1.has_modifier(ACC_STATIC) {
                        Self::merge(context, &mut pms, m);
                    }
                }
            }
        }

        pms.values().flatten().copied().collect()
    }

    /// `PublicMethods.merge(Method)` with `MethodList.merge`: keyed by name and
    /// parameter types; among methods of one return type a more specific
    /// declaring class wins, and a class's method beats an interface's.
    fn merge(
        context: StructContext,
        pms: &mut JLinkedHashMap<(String, String), Vec<ReflectedMethod>>,
        method: ReflectedMethod,
    ) {
        let (dclass, mtd) = method;
        let (params, rtype) = Self::split_descriptor(&mtd.get_descriptor());
        let list = pms.compute_if_absent((mtd.get_name(), params), Vec::new);

        let d_interface = dclass.has_modifier(ACC_INTERFACE);
        let mut i = 0;
        while i < list.len() {
            let (xdclass, xmethod) = list[i];
            if rtype == Self::split_descriptor(&xmethod.get_descriptor()).1 {
                if d_interface == xdclass.has_modifier(ACC_INTERFACE) {
                    if Self::is_assignable_from(context, dclass, xdclass) {
                        return;
                    }
                    if Self::is_assignable_from(context, xdclass, dclass) {
                        list.remove(i);
                        continue;
                    }
                } else if d_interface {
                    return;
                } else {
                    list.remove(i);
                    continue;
                }
            }
            i += 1;
        }
        list.push((dclass, mtd));
    }

    /// `a.isAssignableFrom(b)`.
    fn is_assignable_from(context: StructContext, a: StructClass, b: StructClass) -> bool {
        context.instance_of(&b.qualified_name(), &a.qualified_name())
    }

    /// `(params)` and the return descriptor.
    fn split_descriptor(descriptor: &str) -> (String, String) {
        let close = descriptor.find(')').map_or(descriptor.len(), |i| i + 1);
        (
            descriptor[..close].to_string(),
            descriptor[close..].to_string(),
        )
    }

    /// The `Method` reflection makes of `mtd`: its `toString()` and `isVarArgs()`.
    fn reflect(declaring: StructClass, mtd: StructMethod) -> Method {
        let flags = mtd.get_access_flags();
        let is_default = (flags & (ACC_ABSTRACT | ACC_PUBLIC | ACC_STATIC)) == ACC_PUBLIC
            && declaring.has_modifier(ACC_INTERFACE);

        let mut sb = String::new();
        let mut mod_ = flags & METHOD_MODIFIERS;
        if mod_ != 0 && !is_default {
            sb.push_str(&Self::modifier_to_string(mod_));
            sb.push(' ');
        } else {
            let access_mod = mod_ & ACCESS_MODIFIERS;
            if access_mod != 0 {
                sb.push_str(&Self::modifier_to_string(access_mod));
                sb.push(' ');
            }
            if is_default {
                sb.push_str("default ");
            }
            mod_ &= !ACCESS_MODIFIERS;
            if mod_ != 0 {
                sb.push_str(&Self::modifier_to_string(mod_));
                sb.push(' ');
            }
        }

        let md = MethodDescriptor::parse_descriptor(&mtd.get_descriptor());
        sb.push_str(&Self::type_name(&md.ret));
        sb.push(' ');
        sb.push_str(&declaring.qualified_name().replace('/', "."));
        sb.push('.');
        sb.push_str(&mtd.get_name());
        sb.push('(');
        let params: Vec<String> = md.params.iter().map(Self::type_name).collect();
        sb.push_str(&params.join(","));
        sb.push(')');

        if let Some(exceptions) = mtd.get_attribute(StructGeneralAttribute::ATTRIBUTE_EXCEPTIONS) {
            let pool = declaring.get_pool();
            let count = exceptions.get_throws_exceptions().len();
            if count > 0 {
                let names: Vec<String> = (0..count)
                    .map(|i| {
                        exceptions
                            .get_exc_classname(i as i32, pool)
                            .replace('/', ".")
                    })
                    .collect();
                sb.push_str(" throws ");
                sb.push_str(&names.join(","));
            }
        }

        Method {
            to_string: sb,
            var_args: (flags & ACC_VARARGS) != 0,
        }
    }

    /// `java.lang.reflect.Modifier.toString(int)` for method modifiers.
    fn modifier_to_string(mod_: i32) -> String {
        const NAMES: [(i32, &str); 9] = [
            (ACC_PUBLIC, "public"),
            (ACC_PROTECTED, "protected"),
            (ACC_PRIVATE, "private"),
            (ACC_ABSTRACT, "abstract"),
            (ACC_STATIC, "static"),
            (ACC_FINAL, "final"),
            (ACC_SYNCHRONIZED, "synchronized"),
            (ACC_NATIVE, "native"),
            (ACC_STRICT, "strictfp"),
        ];
        NAMES
            .iter()
            .filter(|(flag, _)| mod_ & flag != 0)
            .map(|(_, name)| *name)
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// `Class.getTypeName()` of an erased descriptor type.
    fn type_name(type_: &VarType) -> String {
        let mut name = match type_.type_ {
            CodeType::Byte => "byte".to_string(),
            CodeType::Char => "char".to_string(),
            CodeType::Double => "double".to_string(),
            CodeType::Float => "float".to_string(),
            CodeType::Int => "int".to_string(),
            CodeType::Long => "long".to_string(),
            CodeType::Short => "short".to_string(),
            CodeType::Boolean => "boolean".to_string(),
            CodeType::Void => "void".to_string(),
            _ => type_.value_str().replace('/', "."),
        };
        for _ in 0..type_.array_dim {
            name.push_str("[]");
        }
        name
    }

    fn build_method_signature(name: &str, md: &MethodDescriptor) -> String {
        let mut sb = String::new();

        Self::append_type(&mut sb, &md.ret);
        sb.push(' ');
        sb.push_str(name);
        sb.push('(');
        for param in &md.params {
            Self::append_type(&mut sb, param);
            sb.push(',');
        }
        if sb.ends_with(',') {
            sb.pop();
        }
        sb.push(')');

        sb
    }

    fn append_type(sb: &mut String, type_: &VarType) {
        sb.push_str(&type_.value_str().replace('/', "."));
        for _ in 0..type_.array_dim {
            sb.push_str("[]");
        }
    }
}
