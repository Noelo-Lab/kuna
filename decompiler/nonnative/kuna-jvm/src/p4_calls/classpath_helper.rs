//! Port of `org/jetbrains/java/decompiler/modules/decompiler/ClasspathHelper.java`.
//!
//! Vineflower answers `findMethod` by reflection on the JVM it runs on (the class from the
//! bootstrap class loader, then the first of its `getMethods()` whose `Method.toString()`
//! contains the wanted signature), never from the decompiler's class universe: its only caller,
//! `InvocationExprent.isVarArgCall`, asks the struct context first and comes here only for a
//! class the struct context lacks, with or without `include-runtime`. The port answers from
//! `classpath_helper_jdk.txt`, the `toString()` of every method such a lookup can find that is
//! varargs, taken from OpenJDK 21 by `tests/data/jdk_varargs/gen.sh`. A query that finds none of
//! them answers `None`, where reflection may find a non-varargs method; the caller reads only
//! `isVarArgs()`, so the two are indistinguishable there.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::method_descriptor::MethodDescriptor;
use crate::rt::JHashMap;
use crate::var_type::VarType;

pub struct ClasspathHelper;

/// The facts of a `java.lang.reflect.Method` that Vineflower reads.
#[derive(Clone, Debug)]
pub struct Method {
    /// `Method.toString()`, e.g. `public static java.util.List java.util.Arrays.asList(java.lang.Object[])`.
    pub to_string: String,
    pub var_args: bool,
}

impl Method {
    pub fn is_var_args(&self) -> bool {
        self.var_args
    }
}

const JDK_VARARGS_METHODS: &str = include_str!("classpath_helper_jdk.txt");

thread_local! {
    static METHOD_CACHE: RefCell<JHashMap<String, Option<Method>>> =
        const { RefCell::new(JHashMap::new()) };
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
        Self::jdk_varargs_methods()
            .get(target_class)?
            .iter()
            .find(|mtd| mtd.contains(method_signature))
            .map(|mtd| Method {
                to_string: mtd.to_string(),
                var_args: true,
            })
    }

    /// The embedded table, by declaring class (`Method.toString()` names it before the name).
    fn jdk_varargs_methods() -> &'static BTreeMap<&'static str, Vec<&'static str>> {
        static TABLE: OnceLock<BTreeMap<&'static str, Vec<&'static str>>> = OnceLock::new();
        TABLE.get_or_init(|| {
            let mut table: BTreeMap<&'static str, Vec<&'static str>> = BTreeMap::new();
            for line in JDK_VARARGS_METHODS.lines().filter(|l| !l.starts_with('#')) {
                let open = line.find('(').expect("method toString");
                let member = &line[line[..open].rfind(' ').expect("method toString") + 1..open];
                let class = &member[..member.rfind('.').expect("method toString")];
                table.entry(class).or_default().push(line);
            }
            table
        })
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
