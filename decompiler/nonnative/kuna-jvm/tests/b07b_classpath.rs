//! `ClasspathHelper.findMethod` against Vineflower's own reflection-based
//! answers (`data/b07b/ClasspathOracle.java` -> `data/b07b/classpath_helper.tsv`),
//! answered here from the embedded JDK table, with or without a runtime in
//! the struct context.

use kuna_jvm::classpath_helper::ClasspathHelper;
use kuna_jvm::jdk_runtime::JdkRuntime;
use kuna_jvm::method_descriptor::MethodDescriptor;

use crate::common;
use crate::f6_support::{jdk_home, skip};

const VECTORS: &str = include_str!("data/b07b/classpath_helper.tsv");
const TABLE: &str = include_str!("../src/p4_calls/classpath_helper_jdk.txt");

fn check_vectors() -> usize {
    let mut checked = 0;
    for line in VECTORS.lines().filter(|l| !l.starts_with('#')) {
        let cols: Vec<&str> = line.split('\t').collect();
        let (class, name, desc, expected, var_args) = (cols[0], cols[1], cols[2], cols[3], cols[4]);
        let found =
            ClasspathHelper::find_method(class, name, &MethodDescriptor::parse_descriptor(desc));
        assert_eq!(
            found.as_ref().is_some_and(|m| m.is_var_args()),
            var_args == "true",
            "{line}"
        );
        if let Some(method) = found {
            assert_eq!(method.to_string, expected, "{line}");
        }
        checked += 1;
    }
    checked
}

#[test]
fn classpath_helper_matches_vineflower_reflection() {
    let version = VECTORS
        .lines()
        .find_map(|l| l.strip_prefix("# java.version="))
        .expect("java.version header");
    assert!(
        TABLE.lines().nth(1).is_some_and(|l| l.contains(version)),
        "vectors and table come from different JDKs"
    );
    let _guard = common::install_default_context();
    assert!(check_vectors() > 50);
}

#[test]
fn classpath_helper_ignores_the_struct_context() {
    let test = "classpath_helper_ignores_the_struct_context";
    let Some(home) = jdk_home() else {
        return skip(test, "no JDK installed");
    };
    let guard = common::install_default_context();
    assert!(JdkRuntime::add_runtime_home(guard.struct_context(), &home));
    assert!(check_vectors() > 50);
}
