//! A browser request's mode reaches the PPC32 PIC loader and stops there:
//! `aggressive` names the secure-PLT import `extra`, a later `reliable` request
//! in the same process leaves it `sub_1800`, and the environment is as it was.
//! A test binary of its own, since each request writes the load-option
//! environment while its image loads.

use std::path::PathBuf;

use kuna_wasm::{run_request, Request};

fn decompile(mode: &str) -> String {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("repository root");
    let binary = root.join("decompiler/crates/kuna-cli/tests/fixtures/ppc32-pic-imports.elf");
    let specs = root.join("specs");
    run_request(&Request {
        binary: binary.to_str().unwrap(),
        spec_root: specs.to_str().unwrap(),
        cmd: "decompile",
        args: &[],
        mode: Some(mode),
        language: None,
        asserts: &[],
    })
    .expect("decompile request")
}

#[test]
fn a_request_mode_names_ppc32_pic_imports_for_its_own_load_only() {
    let variable = kuna_decomp::kuna_ppc32picimports::ENV;
    std::env::remove_var(variable);

    let aggressive = decompile("aggressive");
    assert!(aggressive.contains("extra("), "{aggressive}");
    assert!(!aggressive.contains("sub_1800("), "{aggressive}");
    assert_eq!(std::env::var_os(variable), None);

    let reliable = decompile("reliable");
    assert!(reliable.contains("sub_1800("), "{reliable}");
    assert!(!reliable.contains("extra("), "{reliable}");
}
