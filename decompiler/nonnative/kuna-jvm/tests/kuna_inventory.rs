//! The class-file inventory against the engine's own class model.

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::kuna_input::{self, LoadOptions};
use kuna_jvm::kuna_inventory::{self, class_access_names, method_access_names};
use kuna_jvm::struct_class::StructClass;

use crate::common::install_default_context;
use crate::kuna_support::{copy_classes, corpus, scratch};

#[test]
fn inventory_of_nested_and_anonymous_classes() {
    let main = corpus("java8/pkg/TestAnonymousClassNaming.class");
    let set = kuna_input::load(&main, &LoadOptions::default()).unwrap();
    let classes = kuna_inventory::list(&set);
    let names: Vec<&str> = classes.iter().map(|c| c.internal.as_str()).collect();
    assert_eq!(
        names,
        [
            "pkg/TestAnonymousClassNaming",
            "pkg/TestAnonymousClassNaming$1",
            "pkg/TestAnonymousClassNaming$2",
            "pkg/TestAnonymousClassNaming$InnerClass",
            "pkg/TestAnonymousClassNaming$InnerClass$1",
        ]
    );
    let by_name = |n: &str| classes.iter().find(|c| c.internal == n).unwrap();
    let root = by_name("pkg/TestAnonymousClassNaming");
    assert_eq!(root.outer, None);
    assert_eq!(
        root.source_file.as_deref(),
        Some("TestAnonymousClassNaming.java")
    );
    assert_eq!(root.size, std::fs::read(&main).unwrap().len());
    assert_eq!(
        by_name("pkg/TestAnonymousClassNaming$InnerClass")
            .outer
            .as_deref(),
        Some("pkg/TestAnonymousClassNaming")
    );
    assert_eq!(
        by_name("pkg/TestAnonymousClassNaming$InnerClass$1")
            .outer
            .as_deref(),
        Some("pkg/TestAnonymousClassNaming$InnerClass")
    );
    assert_eq!(
        by_name("pkg/TestAnonymousClassNaming$1").outer.as_deref(),
        Some("pkg/TestAnonymousClassNaming")
    );
    assert!(by_name("pkg/TestAnonymousClassNaming$1")
        .inner_access
        .is_some());
}

#[test]
fn methods_match_the_class_model() {
    let dir = scratch("inventory");
    copy_classes(
        &dir,
        &[
            "java8/pkg/TestClassLoop.class",
            "java8/pkg/TestAccessReplace.class",
            "java8/pkg/TestAccessReplace$Inner.class",
            "java8/pkg/TestAbstractMethods.class",
        ],
    );
    let set = kuna_input::load(&dir, &LoadOptions::default()).unwrap();
    let classes = kuna_inventory::list(&set);
    assert_eq!(classes.len(), 4);
    let sorted: Vec<&String> = classes.iter().map(|c| &c.internal).collect();
    let mut expected = sorted.clone();
    expected.sort();
    assert_eq!(sorted, expected);

    let _guard = install_default_context();
    for file in set.classes() {
        let model = StructClass::create(&mut ByteCursor::new(file.bytes.to_vec()), true);
        let info = classes
            .iter()
            .find(|c| c.internal == model.qualified_name())
            .unwrap();
        assert_eq!(info.access as i32, model.get_access_flags());
        let methods = model.get_methods().to_vec();
        assert_eq!(info.methods.len(), methods.len(), "{}", info.internal);
        for (mine, theirs) in info.methods.iter().zip(&methods) {
            assert_eq!(mine.name, theirs.get_name());
            assert_eq!(mine.descriptor, theirs.get_descriptor());
            assert_eq!(mine.access as i32, theirs.get_access_flags());
            assert_eq!(
                mine.code_length.is_some(),
                theirs.contains_code(),
                "{}",
                mine.name
            );
        }
    }
    let abstract_class = classes
        .iter()
        .find(|c| c.internal == "pkg/TestAbstractMethods")
        .unwrap();
    assert!(abstract_class
        .methods
        .iter()
        .any(|m| m.code_length.is_none()));
    assert!(abstract_class
        .methods
        .iter()
        .any(|m| m.code_length.is_some_and(|n| n > 0)));
}

#[test]
fn access_flag_names() {
    assert_eq!(
        class_access_names(0x0001 | 0x0010 | 0x0020),
        ["public", "final"]
    );
    assert_eq!(
        class_access_names(0x0601),
        ["public", "abstract", "interface"]
    );
    assert_eq!(
        method_access_names(0x0001 | 0x0008 | 0x0020 | 0x1000),
        ["public", "static", "synchronized", "synthetic"]
    );
    assert_eq!(method_access_names(0x0402), ["private", "abstract"]);
}
