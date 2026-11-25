//! A datatest file's load-time `option` choices never reach the next file's
//! loads. Own test binary: it changes the working directory and the
//! load-option environment.

use kuna_decomp::kuna_libctypes::LibcTypesLayout;
use kuna_decomp::kuna_loadoptions::{EnvSnapshot, LoadOptions, ENV_VARS};
use kuna_decomp::kuna_ppc32picimports::Mode;
use kuna_decomp::kuna_ppcapu::Mode as PpcApuMode;
use kuna_harness::testfunction::run_test_files_with_specs;
use std::path::PathBuf;

#[test]
fn each_file_loads_with_the_options_the_run_started_with() {
    for name in ENV_VARS {
        std::env::remove_var(name);
    }
    let changed = LoadOptions {
        ppc32picimports: Mode::Abi,
        ifuncfpret: false,
        ppcapu: PpcApuMode::On,
        goabi: false,
        libctypes: LibcTypesLayout::Glibc,
        calltargettype: false,
        macho_arm64e: false,
        objcivars: false,
        objcsdk: Some(std::env::temp_dir()),
        dyldcrossrefs: false,
        ..LoadOptions::default()
    };
    let snapshot = EnvSnapshot::capture();
    for name in [
        "ppc32picimports", "ifuncfpret", "ppcapu", "goabi", "libctypes", "calltargettype",
        "macho-arm64e", "objcivars", "objcsdk", "dyldcrossrefs",
    ] {
        changed.export_env(name);
    }
    assert_eq!(LoadOptions::from_env(), changed);
    snapshot.restore();
    assert_eq!(LoadOptions::from_env(), LoadOptions::default());
    assert_eq!(EnvSnapshot::capture(), snapshot);

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap();
    std::env::set_current_dir(&root).unwrap();
    let files = [
        "kuna-ppc32picimports-abi-indirect.xml",
        "kuna-ppc32picimports-abi-isolation.xml",
        "kuna-ppc32picimports.xml",
    ]
    .map(|name| root.join("tests/stages").join(name).to_str().unwrap().to_string());
    let specs = root.join("specs").to_str().unwrap().to_string();
    let mut output = String::new();
    let failures = run_test_files_with_specs(&files, &[specs], &mut output);
    assert_eq!(failures, 0, "{output}");
    assert!(output.contains("Total passing tests = 12"), "{output}");
}
