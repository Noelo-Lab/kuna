//! `MethodPropertiesProcessor` against Vineflower 1.12.0: `tests/data/cfg/PropsDump.java` runs
//! `parse` on every method of the committed classes (`inputs/PropCases.java` covers assertion
//! fields and lambdas that can print as method references) and `golden/props.txt` is its output.

use kuna_jvm::byte_cursor::ByteCursor;
use kuna_jvm::decompiler_context::DecompilerContext;
use kuna_jvm::method_properties::MethodProperties;
use kuna_jvm::method_properties_processor::MethodPropertiesProcessor;
use kuna_jvm::struct_class::StructClass;
use kuna_jvm::struct_method::StructMethod;

const SETS: [&str; 6] = ["props8", "props21", "r8", "r21", "jasm", "legacy"];

fn dump() -> String {
    let root = format!("{}/tests/data/cfg/classes", env!("CARGO_MANIFEST_DIR"));
    let mut out = String::from("#props vineflower-1.12.0\n");
    for set in SETS {
        let mut files: Vec<_> = std::fs::read_dir(format!("{root}/{set}"))
            .expect("class dir")
            .map(|e| e.expect("entry").path())
            .filter(|p| p.extension().is_some_and(|x| x == "class"))
            .collect();
        files.sort();
        for path in files {
            let bytes = std::fs::read(&path).expect("class file");
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

#[test]
fn method_properties_match_vineflower() {
    let _context = crate::common::install_default_context();
    let path = format!(
        "{}/tests/data/cfg/golden/props.txt",
        env!("CARGO_MANIFEST_DIR")
    );
    let expected = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let actual = dump();
    for (n, (e, a)) in expected.lines().zip(actual.lines()).enumerate() {
        assert_eq!(e, a, "line {}", n + 1);
    }
    assert_eq!(expected.lines().count(), actual.lines().count());
    assert_eq!(expected.matches(" syn=1 ").count(), 10);
    assert_eq!(
        expected.matches(" assert=$assertionsDisabled:Z").count(),
        10
    );
}
