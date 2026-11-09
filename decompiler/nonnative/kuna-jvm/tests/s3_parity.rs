//! Whole-class outputs against Vineflower 1.12.0 for paths the corpus fixture
//! never shows. `data/s3_parity/*.java` are `vf-decompile --opt decompile-enums=0
//! --opt decompile-inner=0 --opt include-runtime= --opt thread-count=1` of the
//! vendored class files.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_jvm::fernflower::{DecompiledClass, Fernflower};
use kuna_jvm::i_fernflower_logger::CollectingLogger;
use kuna_jvm::i_fernflower_preferences::IFernflowerPreferences;

fn vendored(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../tests/jvm/classes/java21")
        .join(path)
}

/// JDK 21 javac gives mandated parameters (`valueOf`'s `name`, an inner class's
/// `this$0`) a nameless `MethodParameters` entry; the `null` name Vineflower
/// stores is replaced by the LVT name.
#[test]
fn nameless_mandated_parameters_take_lvt_names() {
    let options: Vec<(String, String)> = [
        (IFernflowerPreferences::DECOMPILE_ENUM, "0"),
        (IFernflowerPreferences::DECOMPILE_INNER, "0"),
        (IFernflowerPreferences::INCLUDE_JAVA_RUNTIME, ""),
    ]
    .iter()
    .map(|(k, v)| (k.to_string(), v.to_string()))
    .collect();
    let mut fernflower = Fernflower::new(&options, Rc::new(CollectingLogger::new()));
    fernflower.add_source_file(&vendored("ext/TestEnum2.class"));
    fernflower.add_source_file(&vendored("pkg/TestInnerClassesJ21$TestInner.class"));
    let mut classes: Vec<DecompiledClass> = Vec::new();
    fernflower.decompile_context(&mut |c| classes.push(c));
    fernflower.clear_context();

    let expected = [
        (
            "ext/TestEnum2",
            include_str!("data/s3_parity/TestEnum2.java"),
        ),
        (
            "pkg/TestInnerClassesJ21$TestInner",
            include_str!("data/s3_parity/TestInnerClassesJ21$TestInner.java"),
        ),
    ];
    assert_eq!(classes.len(), expected.len());
    for (name, text) in expected {
        let class = classes
            .iter()
            .find(|c| c.qualified_name == name)
            .unwrap_or_else(|| panic!("{name} not delivered"));
        assert_eq!(class.content.trim(), text.trim(), "{name}");
    }
}
