mod common;
#[path = "../../kuna-analysis/tests/fixtures/arm_aliases.rs"]
mod fixture;
use object::{Object, ObjectSymbol};
use std::process::Command;

#[test]
fn arm_aliases_select_the_definition_instead_of_its_import() {
    for linked in [true, false] {
        for veneer in [false, true] {
            for reverse in [false, true] {
                let bytes = fixture::image(veneer, reverse, linked);
                let file = object::File::parse(bytes.as_slice()).unwrap();
                let address = if linked { 0x1000 } else { 0 };
                for name in ["answer", "__answer_from_arm", "answer_alias"] {
                    assert!(file
                        .symbols()
                        .any(|s| s.name() == Ok(name) && s.address() == address));
                    if linked {
                        assert!(file
                            .dynamic_symbols()
                            .any(|s| s.name() == Ok(name) && s.address() == address));
                    }
                }
                let path = common::scratch_file("arm-aliases", "elf");
                std::fs::write(&path, bytes).unwrap();
                let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
                    .args(["functions", path.to_str().unwrap(), "--json"])
                    .output()
                    .unwrap();
                assert!(
                    out.status.success(),
                    "{}",
                    String::from_utf8_lossy(&out.stderr)
                );
                let rendered = String::from_utf8_lossy(&out.stdout);
                for name in ["answer", "__answer_from_arm", "answer_alias"] {
                    assert!(rendered.contains(&format!("\"{name}\"")), "{rendered}");
                }
                let specs =
                    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
                let prog = kuna_console::engine::bootstrap_from_file(
                    path.to_str().unwrap(),
                    "",
                    &[specs.to_str().unwrap().into()],
                )
                .unwrap();
                let entries = prog.function_entries_canonical();
                let entry = prog.find_entry_by_name("answer").unwrap();
                let definition = entry.addr.get_offset();
                assert_eq!(entries.iter().filter(|e| e.addr == entry.addr).count(), 1);
                for name in ["answer_alias", "__answer_from_arm"] {
                    assert_eq!(prog.find_entry_by_name(name).unwrap().addr, entry.addr);
                }
                assert_eq!(entry.name, "__answer_from_arm", "the first name stays reported");
                assert_eq!(entry.aliases.len(), 2);
                assert_eq!(prog.find_entry_at(definition).unwrap().name, entry.name);
                if veneer {
                    assert_eq!(
                        prog.find_entry_by_name("__real_answer")
                            .unwrap()
                            .addr
                            .get_offset(),
                        definition + 16
                    );
                }
                for selector in [
                    "answer".to_owned(),
                    "answer_alias".into(),
                    "__answer_from_arm".into(),
                    format!("0x{definition:x}"),
                ] {
                    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
                    command.args([
                        "decompile",
                        path.to_str().unwrap(),
                        &selector,
                        "--mode",
                        "aggressive",
                    ]);
                    if selector.starts_with("0x") {
                        command.arg("--addr");
                    }
                    let out = command.output().unwrap();
                    let text = String::from_utf8_lossy(&out.stdout);
                    assert!(
                        out.status.success(),
                        "{text}\n{}",
                        String::from_utf8_lossy(&out.stderr)
                    );
                    assert!(
                        text.contains("return 7;"),
                        "linked={linked} veneer={veneer} reverse={reverse} {selector}: {text}"
                    );
                }
            }
        }
    }
}
