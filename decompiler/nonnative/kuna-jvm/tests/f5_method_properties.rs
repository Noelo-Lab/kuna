//! `MethodPropertiesProcessor` against Vineflower 1.12.0: `tests/data/cfg/PropsDump.java` runs
//! `parse` on every method of the committed classes (`inputs/PropCases.java` covers assertion
//! fields and lambdas that can print as method references) and `golden/props.txt` is its output.

use std::path::{Path, PathBuf};

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::method_properties::MethodProperties;
use kuna_jvm::method_properties_processor::MethodPropertiesProcessor;
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_method::StructMethod;

use crate::f5_cfg_golden::collect_classes;

const SETS: [&str; 6] = ["props8", "props21", "r8", "r21", "jasm", "legacy"];

/// `PropsDump`'s output for `inputs`.
fn dump(inputs: &[PathBuf]) -> String {
    let mut out = String::from("#props vineflower-1.12.0\n");
    for input in inputs {
        for bytes in collect_classes(input) {
            let cl = StructClass::create(&mut ByteCursor::new(bytes), true);
            DecompilerContext::set_current_class(Some(cl));
            let methods: Vec<StructMethod> = cl.get_methods().iter().copied().collect();
            for mt in methods {
                let mut props = MethodProperties::new(mt);
                MethodPropertiesProcessor::parse(mt, cl, &mut props);
                let assert_field = props.assert_field.map_or_else(
                    || "-".to_string(),
                    |f| format!("{}:{}", f.get_name(), f.get_descriptor()),
                );
                out.push_str(&format!(
                    "m {} {} {} syn={} assert={assert_field}\n",
                    cl.qualified_name(),
                    mt.get_name(),
                    mt.get_descriptor(),
                    i32::from(props.is_synthetic_reference_lambda)
                ));
            }
            DecompilerContext::set_current_class(None);
        }
    }
    out
}

fn check(expected: &str, actual: &str) {
    for (n, (e, a)) in expected.lines().zip(actual.lines()).enumerate() {
        assert_eq!(e, a, "line {}", n + 1);
    }
    assert_eq!(expected.lines().count(), actual.lines().count());
}

#[test]
fn method_properties_match_vineflower() {
    let _context = crate::common::install_default_context();
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/cfg");
    let path = root.join("golden/props.txt");
    let expected =
        std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    let inputs: Vec<PathBuf> = SETS.iter().map(|s| root.join("classes").join(s)).collect();
    check(&expected, &dump(&inputs));
    assert_eq!(expected.matches(" syn=1 ").count(), 10);
    assert_eq!(
        expected.matches(" assert=$assertionsDisabled:Z").count(),
        10
    );
}

/// Opt-in: `KUNA_JVM_PROPS_SWEEP` names a `PropsDump` output and `KUNA_JVM_PROPS_INPUTS` the
/// `:`-separated inputs it was run on.
#[test]
fn method_properties_sweep() {
    let (Ok(golden), Ok(inputs)) = (
        std::env::var("KUNA_JVM_PROPS_SWEEP"),
        std::env::var("KUNA_JVM_PROPS_INPUTS"),
    ) else {
        return;
    };
    let _context = crate::common::install_default_context();
    let expected = std::fs::read_to_string(&golden).unwrap_or_else(|e| panic!("{golden}: {e}"));
    let inputs: Vec<PathBuf> = inputs.split(':').map(PathBuf::from).collect();
    check(&expected, &dump(&inputs));
    eprintln!("{golden}: {} methods match", expected.lines().count() - 1);
}
