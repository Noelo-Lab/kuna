use crate::common;
use common::process;
use std::path::Path;
use std::process::Command;

const NAMES: &str = "null_pointer,maybe_pointer,pointer_is_null,clear_pointer";
const INPUTS: &[u64] = &[
    0,
    1,
    2,
    41,
    42,
    43,
    127,
    255,
    0x7fff_ffff,
    0x8000_0000,
    0xffff_ffff,
    0x1_0000_0000,
    0x7fff_ffff_ffff_ffff,
    0x8000_0000_0000_0000,
    u64::MAX,
];

fn records() -> serde_json::Value {
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    let result = process::required_output(Command::new(kuna).args([
        "decompile-all",
        &common::fixture("rust_null_pointer_x86_64"),
        "--json",
        "--language",
        "rust",
        "--functions",
        NAMES,
        "--assert",
        &format!("@{}", common::fixture("rust_null_pointer.kuna")),
    ]));
    let doc: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(doc["functions"].as_array().unwrap().len(), 4);
    for function in doc["functions"].as_array().unwrap() {
        assert!(function["error"].is_null(), "{function}");
    }
    doc
}

#[test]
fn null_returns_comparisons_and_stores_use_callable_rust_syntax() {
    let doc = records();
    for function in doc["functions"].as_array().unwrap() {
        let code = function["code"].as_str().unwrap();
        assert_eq!(code.matches("core::ptr::null_mut()").count(), 1, "{code}");
        assert!(!code.contains("core::ptr::null_mut__"), "{code}");
    }
}

fn observe(binary: &Path, choice: u64) -> String {
    let output = process::required_output(Command::new(binary).arg(choice.to_string()));
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn complete_rust_bodies_preserve_null_values_and_pointer_identity() {
    if process::optional_output(Command::new("rustc").arg("--version")).is_none() {
        return;
    }
    let doc = records();
    let bodies = doc["functions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["code"].as_str().unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    let source = common::scratch_file("rust-null-pointer", "rs");
    let main = r#"
fn main() {
    let choice:u64=std::env::args().nth(1).unwrap().parse().unwrap();
    let mut byte=17u8; let pointer=&mut byte as *mut u8; let mut slot=pointer;
    unsafe {
        let null=null_pointer(); let selected=maybe_pointer(choice,pointer); clear_pointer(&mut slot);
        println!("{} {} {} {} {}",null.is_null(),selected==pointer,slot.is_null(),pointer_is_null(null),pointer_is_null(pointer));
    }
}
"#;
    std::fs::write(&source, format!("{bodies}\n{main}")).unwrap();
    let original = Path::new(&common::fixture("rust_null_pointer_x86_64")).to_path_buf();
    let expected = INPUTS
        .iter()
        .map(|&choice| {
            let output = observe(&original, choice);
            assert_eq!(output, format!("true {} true 1 0", choice == 42));
            (choice, output)
        })
        .collect::<Vec<_>>();
    for optimization in ["0", "2"] {
        let binary = source.with_extension(format!("O{optimization}"));
        process::required_output(
            Command::new("rustc")
                .args([
                    "--edition=2021",
                    "--crate-name",
                    "recovered_null_pointers",
                    "-C",
                    &format!("opt-level={optimization}"),
                ])
                .arg(&source)
                .arg("-o")
                .arg(&binary),
        );
        for (choice, expected) in &expected {
            assert_eq!(
                observe(&binary, *choice),
                *expected,
                "O{optimization}, choice={choice}"
            );
        }
        std::fs::remove_file(binary).unwrap();
    }
    std::fs::remove_file(source).unwrap();
}
