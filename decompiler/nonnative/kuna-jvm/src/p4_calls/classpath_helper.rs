//! Port of `org/jetbrains/java/decompiler/modules/decompiler/ClasspathHelper.java`.
//!
//! Vineflower resolves a method of a class missing from the struct context by
//! reflection on the JDK it runs on (bootstrap class loader, public methods,
//! matched by `Method.toString()`). The port answers the same query from the JDK
//! runtime library; only that lookup ([`ClasspathHelper::find_method_on_classpath`])
//! is left to be ported.
#![allow(unused_variables)]

use std::cell::RefCell;

use crate::method_descriptor::MethodDescriptor;
use crate::rt::JHashMap;
use crate::unported;
use crate::var_type::VarType;

pub struct ClasspathHelper;

/// The facts of a `java.lang.reflect.Method` that Vineflower reads.
#[derive(Clone, Debug)]
pub struct Method {
    /// `Method.toString()`, e.g. `public static java.lang.String java.lang.String.valueOf(char[])`.
    pub to_string: String,
    pub var_args: bool,
}

impl Method {
    pub fn is_var_args(&self) -> bool {
        self.var_args
    }
}

thread_local! {
    static METHOD_CACHE: RefCell<JHashMap<String, Option<Method>>> = const { RefCell::new(JHashMap::new()) };
}

impl ClasspathHelper {
    pub fn find_method(
        classname: &str,
        method_name: &str,
        descriptor: &MethodDescriptor,
    ) -> Option<Method> {
        let target_class = classname.replace('/', ".");
        let method_signature =
            Self::build_method_signature(&format!("{target_class}.{method_name}"), descriptor);

        let cached = METHOD_CACHE.with(|c| {
            let cache = c.borrow();
            cache
                .contains_key(&method_signature)
                .then(|| cache.get_cloned(&method_signature).flatten())
        });
        match cached {
            Some(method) => method,
            None => {
                let method = Self::find_method_on_classpath(&target_class, &method_signature);
                METHOD_CACHE.with(|c| c.borrow_mut().put(method_signature, method.clone()));
                method
            }
        }
    }

    fn find_method_on_classpath(target_class: &str, method_signature: &str) -> Option<Method> {
        unported!("ClasspathHelper.findMethodOnClasspath")
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
        sb.push_str(
            &type_
                .value
                .as_ref()
                .expect("VarType.value")
                .replace('/', "."),
        );
        for _ in 0..type_.array_dim {
            sb.push_str("[]");
        }
    }
}
