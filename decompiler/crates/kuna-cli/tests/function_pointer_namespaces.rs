use crate::common;
use common::process;
use std::process::Command;

const NAMES: &[&str] = &[
    "left::same_address",
    "right::cross_address",
    "foreign_address",
    "nested_address",
    "right_address",
    "global_shadow",
    "scope_shadow",
];

fn capture(strategy: &str) -> serde_json::Value {
    let binary =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let selected = format!("{},interior_address,data_address", NAMES.join(","));
    let result = process::required_output(Command::new(binary).args([
        "decompile-all",
        &common::fixture("function_pointer_namespaces_x86_64"),
        "--json",
        "--language",
        "c",
        "--assert-strict",
        "--assert",
        &format!("@{}", common::fixture("function_pointer_namespaces.kuna")),
        "--option",
        "namespacestrategy",
        strategy,
        "--functions",
        &selected,
    ]));
    let doc: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(doc["functions"].as_array().unwrap().len(), 9);
    assert!(doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .all(|f| f["error"].is_null()));
    doc
}

fn code<'a>(doc: &'a serde_json::Value, name: &str) -> &'a str {
    doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["name"] == name)
        .unwrap()["code"]
        .as_str()
        .unwrap()
}

#[test]
fn no_namespaces_and_non_function_controls_remain_explicit() {
    let none = capture("none");
    let minimal = capture("minimal");
    let all = capture("all");
    assert!(code(&none, "foreign_address").contains("return (unsigned long)target;"));
    assert!(code(&minimal, "foreign_address").contains("return (unsigned long)left::target;"));
    assert!(code(&minimal, "left::same_address").contains("return (unsigned long)target;"));
    assert!(code(&all, "left::same_address").contains("return (unsigned long)left::target;"));
    for doc in [&minimal, &all] {
        assert!(code(doc, "global_shadow").contains("return (unsigned long)::target;"));
        assert!(code(doc, "scope_shadow").contains("return (unsigned long)::left::target;"));
        for name in ["interior_address", "data_address"] {
            assert_eq!(code(doc, name), code(&none, name));
        }
    }
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn emitted_function_addresses_select_the_original_native_targets() {
    let source = common::scratch_file("function-pointer-namespaces", "cpp");
    let names = NAMES
        .iter()
        .map(|name| name.rsplit("::").next().unwrap())
        .chain(["main"])
        .collect::<Vec<_>>();
    let mut prefix = names
        .iter()
        .map(|name| format!("#define {name} original_{name}\n"))
        .collect::<String>();
    prefix.push_str(&format!(
        "#include \"{}\"\n",
        common::fixture("function_pointer_namespaces.cpp")
    ));
    prefix.extend(names.iter().map(|name| format!("#undef {name}\n")));
    prefix.push_str("namespace left { uintptr_t same_address(); }\nnamespace right { uintptr_t cross_address(); }\n");
    let driver = r#"
int main() {
    unsigned inputs[] = {0, 1, 7, 12345, 0x7fffffffU, 0x80000000U, 0xffffffd7U, 0xfffffff5U, 0xffffffffU};
    for (unsigned x : inputs) {
        uintptr_t actual[] = {left::same_address(), right::cross_address(), foreign_address(), nested_address(), right_address(), global_shadow(x), scope_shadow(x)};
        uintptr_t expected[] = {left::original_same_address(), right::original_cross_address(), original_foreign_address(), original_nested_address(), original_right_address(), original_global_shadow(x), original_scope_shadow(x)};
        for (unsigned i = 0; i < 7; ++i) {
            if (actual[i] != expected[i]) return 10 + i;
            if (reinterpret_cast<Callback>(actual[i])(x) != reinterpret_cast<Callback>(expected[i])(x)) return 20 + i;
        }
    }
    return 0;
}
"#;
    for strategy in ["minimal", "all"] {
        let doc = capture(strategy);
        let body = NAMES
            .iter()
            .map(|name| code(&doc, name))
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(&source, format!("{prefix}\n{body}\n{driver}")).unwrap();
        for compiler in ["clang++", "g++"] {
            if process::optional_output(Command::new(compiler).arg("--version")).is_none() {
                continue;
            }
            for optimization in ["-O0", "-O2"] {
                let binary = source.with_extension(format!("{strategy}-{compiler}-{optimization}"));
                process::required_output(
                    Command::new(compiler)
                        .args(["-std=c++17", optimization])
                        .arg(&source)
                        .arg("-o")
                        .arg(&binary),
                );
                process::required_output(&mut Command::new(&binary));
            }
        }
    }
}
