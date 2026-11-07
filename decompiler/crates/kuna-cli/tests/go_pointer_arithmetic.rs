//! Complete emitted pointer loads, stores and returns retain Go allocation provenance.
use crate::common;
use common::process;
use std::process::Command;

#[test]
#[cfg(all(target_arch = "x86_64", target_os = "linux"))]
fn pointer_byte_arithmetic_survives_go_checkptr_at_both_native_widths() {
    if process::optional_output(Command::new("cc").arg("--version")).is_none()
        || process::optional_output(Command::new("go").arg("version")).is_none()
    {
        return;
    }
    let kuna =
        std::env::var_os("KUNA_TEST_BIN").unwrap_or_else(|| env!("CARGO_BIN_EXE_kuna").into());
    for (arch, flag, goarch, size) in [("x86_64", "-m64", "amd64", 8), ("i386", "-m32", "386", 4)] {
        let fixture = common::scratch_file("go_pointer_arithmetic", "elf");
        process::required_output(
            Command::new("cc")
                .args([
                    flag,
                    "-nostdlib",
                    "-no-pie",
                    "-Wl,--build-id=none,-e,load_offset",
                ])
                .arg(common::fixture(&format!("go_pointer_arithmetic_{arch}.s")))
                .arg("-o")
                .arg(&fixture),
        );
        let mut command = Command::new(&kuna);
        command.args(["decompile-all"]).arg(&fixture).args([
            "--language",
            "go",
            "--json",
            "--assert-strict",
        ]);
        for (name, declaration) in [
            (
                "load_offset",
                format!("uint{size} load_offset(void *base);"),
            ),
            (
                "store_offset",
                format!("void store_offset(void *base,uint{size} value);"),
            ),
            (
                "return_offset",
                format!("uint{size} *return_offset(void *base);"),
            ),
            (
                "load_dynamic",
                format!("uint{size} load_dynamic(void *base,int{size} offset);"),
            ),
            (
                "store_dynamic",
                format!("void store_dynamic(void *base,int{size} offset,uint{size} value);"),
            ),
            (
                "return_dynamic",
                format!("uint{size} *return_dynamic(void *base,int{size} offset);"),
            ),
            (
                "return_negative",
                format!("uint{size} *return_negative(void *base);"),
            ),
            (
                "integer_address",
                format!("uint{size} *integer_address(uint{size} base);"),
            ),
        ] {
            command.args(["--assert", &format!("prototype {name} {declaration}")]);
        }
        let output = process::required_output(&mut command);
        let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        for assertion in document["assertions"].as_array().unwrap() {
            assert_eq!(assertion["status"], "applied", "{assertion}");
        }
        let functions = document["functions"].as_array().unwrap();
        assert_eq!(functions.len(), 8);
        let bodies = functions
            .iter()
            .map(|function| {
                assert!(function["error"].is_null(), "{function}");
                function["code"].as_str().unwrap()
            })
            .collect::<Vec<_>>()
            .join("\n");
        let source = common::scratch_file("go_pointer_arithmetic", "go");
        let template =
            include_str!("../../kuna-analysis/tests/fixtures/go_pointer_arithmetic_harness.go");
        std::fs::write(
            &source,
            template
                .replace("/* EMITTED_BODIES */", &bodies)
                .replace("SIGNED", &format!("int{}", size * 8))
                .replace("WORD", &format!("uint{}", size * 8)),
        )
        .unwrap();
        let binary = source.with_extension("out");
        for flags in ["-d=checkptr=2", "-N -l -d=checkptr=2"] {
            process::required_output(
                Command::new("go")
                    .env("GOARCH", goarch)
                    .env("CGO_ENABLED", "0")
                    .arg("build")
                    .arg(format!("-gcflags=all={flags}"))
                    .arg("-o")
                    .arg(&binary)
                    .arg(&source),
            );
            let result = process::required_output(&mut Command::new(&binary));
            assert_eq!(
                String::from_utf8_lossy(&result.stdout),
                "PASS observations=193920\n"
            );
        }
        std::fs::remove_file(binary).unwrap();
        std::fs::remove_file(source).unwrap();
        std::fs::remove_file(fixture).unwrap();
    }
}
