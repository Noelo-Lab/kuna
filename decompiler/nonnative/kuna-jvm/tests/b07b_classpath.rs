//! `ClasspathHelper.findMethod` against Vineflower's own reflection-based
//! answers (`data/b07b/ClasspathOracle.java` -> `data/b07b/classpath_helper.tsv`),
//! answered here from the JDK runtime loaded into the struct context.

use kuna_jvm::classpath_helper::ClasspathHelper;
use kuna_jvm::jdk_runtime::JdkRuntime;
use kuna_jvm::method_descriptor::MethodDescriptor;

use crate::common;
use crate::f6_support::{jdk_home, skip};

const VECTORS: &str = include_str!("data/b07b/classpath_helper.tsv");

/// Classes outside the bootstrap loader's modules: reflection cannot see them,
/// the struct context (all runtime modules) can.
const NOT_ON_BOOT_LOADER: &[&str] = &["java/sql/DriverManager"];

fn vector_java_version() -> &'static str {
    VECTORS
        .lines()
        .find_map(|l| l.strip_prefix("# java.version="))
        .expect("java.version header")
}

fn release_java_version(home: &std::path::Path) -> Option<String> {
    let release = std::fs::read_to_string(home.join("release")).ok()?;
    release.lines().find_map(|l| {
        l.strip_prefix("JAVA_VERSION=")
            .map(|v| v.trim_matches('"').to_string())
    })
}

#[test]
fn classpath_helper_matches_vineflower_reflection() {
    let test = "classpath_helper_matches_vineflower_reflection";
    let Some(home) = jdk_home() else {
        return skip(test, "no JDK installed");
    };
    if release_java_version(&home).as_deref() != Some(vector_java_version()) {
        return skip(test, "vectors were generated on a different JDK");
    }

    let guard = common::install_default_context();
    assert!(JdkRuntime::add_runtime_home(guard.struct_context(), &home));

    let mut checked = 0;
    for line in VECTORS.lines().filter(|l| !l.starts_with('#')) {
        let cols: Vec<&str> = line.split('\t').collect();
        let (class, name, desc, expected, var_args) = (cols[0], cols[1], cols[2], cols[3], cols[4]);
        let found =
            ClasspathHelper::find_method(class, name, &MethodDescriptor::parse_descriptor(desc));
        if NOT_ON_BOOT_LOADER.contains(&class) {
            assert_eq!(expected, "null", "{line}");
            assert!(found.is_some(), "{line}");
            continue;
        }
        match found {
            None => assert_eq!(expected, "null", "{line}"),
            Some(method) => {
                assert_eq!(method.to_string, expected, "{line}");
                assert_eq!(method.is_var_args().to_string(), var_args, "{line}");
            }
        }
        checked += 1;
    }
    assert!(checked > 40);

    let again = ClasspathHelper::find_method(
        "java/lang/String",
        "format",
        &MethodDescriptor::parse_descriptor(
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;",
        ),
    );
    assert!(again.is_some_and(|m| m.is_var_args()));
}

#[test]
fn classpath_helper_without_runtime_finds_nothing() {
    let _guard = common::install_default_context();
    let found = ClasspathHelper::find_method(
        "java/lang/String",
        "format",
        &MethodDescriptor::parse_descriptor(
            "(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;",
        ),
    );
    assert!(found.is_none());
}

#[test]
fn classpath_helper_cache_follows_the_struct_context() {
    let test = "classpath_helper_cache_follows_the_struct_context";
    let Some(home) = jdk_home() else {
        return skip(test, "no JDK installed");
    };
    let desc = MethodDescriptor::parse_descriptor("([Ljava/lang/Object;)Ljava/util/List;");
    {
        let _guard = common::install_default_context();
        assert!(ClasspathHelper::find_method("java/util/Arrays", "asList", &desc).is_none());
    }
    {
        let guard = common::install_default_context();
        assert!(JdkRuntime::add_runtime_home(guard.struct_context(), &home));
        let found = ClasspathHelper::find_method("java/util/Arrays", "asList", &desc);
        assert!(found.is_some_and(|m| m.is_var_args()));
    }
}
