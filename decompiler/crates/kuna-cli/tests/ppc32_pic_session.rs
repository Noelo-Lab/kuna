//! In-process PPC32 PIC sessions. The option writes the process environment the
//! loader reads, so these tests run serially in their own binary.

mod common;
#[path = "fixtures/ppc32_import_fixture.rs"]
#[allow(dead_code)]
mod fixture;

use kuna_decomp::kuna_ppc32picimports::{Mode, ENV};
use std::sync::{Mutex, MutexGuard};

static ENVIRONMENT: Mutex<()> = Mutex::new(());

fn serial() -> MutexGuard<'static, ()> {
    let guard = ENVIRONMENT.lock().unwrap_or_else(|e| e.into_inner());
    std::env::remove_var(ENV);
    guard
}

fn load(path: &std::path::Path) -> kuna_console::engine::ConsoleProgram {
    let specs = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../specs")
        .canonicalize()
        .unwrap();
    let mut prog = kuna_console::engine::bootstrap_from_image(
        path.to_str().unwrap(),
        "",
        &[specs.to_string_lossy().into_owned()],
    )
    .unwrap();
    prog.commit_pending_analysis().unwrap();
    prog
}

/// A new image starts in `abi` exactly when its loader applied `abi`, so the
/// live option names what the image was loaded with; `on` stays per image.
#[test]
fn a_reloaded_image_reports_the_mode_its_loader_applied() {
    let _env = serial();
    let path = common::scratch_file("ppc32-pic-session", "elf");
    std::fs::write(&path, fixture::abi_caller()).unwrap();
    let named = |prog: &kuna_console::engine::ConsoleProgram| {
        prog.function_named_at(0x1800).is_some_and(|name| name == "extra")
    };
    let mut prog = load(&path);
    assert_eq!(prog.arch().analysis_ppc32picimports, Mode::Off);
    assert!(!named(&prog));
    prog.arch_mut().set_kuna_option("ppc32picimports", "abi").unwrap();
    let mut prog = load(&path);
    assert_eq!(prog.arch().analysis_ppc32picimports, Mode::Abi);
    assert!(named(&prog));
    prog.arch_mut().set_kuna_option("ppc32picimports", "on").unwrap();
    assert_eq!(prog.arch().analysis_ppc32picimports, Mode::On);
    let mut prog = load(&path);
    assert_eq!(prog.arch().analysis_ppc32picimports, Mode::Off);
    assert!(!named(&prog));
    prog.arch_mut().set_kuna_option("ppc32picimports", "abi").unwrap();
    prog.arch_mut().reset_wire_defaults();
    assert_eq!(prog.arch().analysis_ppc32picimports, Mode::Off);
    assert_eq!(Mode::from_env(), Mode::Off);
    assert!(!named(&load(&path)));
    std::fs::remove_file(path).unwrap();
}

/// The xrefs proof reads the bindings off one unpublished flow; they equal the
/// bindings a decompile applies, and leave the comment database untouched.
#[test]
fn proof_only_bindings_match_the_applied_flow() {
    let _env = serial();
    for little in [false, true] {
        let mut f = fixture::Fixture::new(fixture::Plt::Short);
        f.little = little;
        f.pic_collision = true;
        f.caller = Some(vec![0x3bc02800, 0x480007fd, 0, 0]);
        let path = common::scratch_file("ppc32-pic-proof-only", "elf");
        std::fs::write(&path, f.bytes()).unwrap();
        let mut prog = load(&path);
        prog.arch_mut().set_kuna_option("ppc32picimports", "on").unwrap();
        let entry = prog.lookup_symbol("check").unwrap();
        let before = prog.arch().commentdb.comments().len();
        let proved = kuna_decomp::decompile_drive::ppc32_pic_call_bindings(prog.arch_mut(), "check", entry.clone(), 0).unwrap();
        assert_eq!(prog.arch().commentdb.comments().len(), before, "little={little}");
        let fd = kuna_decomp::decompile_drive::build_and_follow_flow(prog.arch_mut(), "check", entry, 0).unwrap();
        let applied: Vec<_> = fd.ppc32_import_calls().cloned().collect();
        assert_eq!(proved, applied);
        assert_eq!(proved.len(), 1);
        assert_eq!((proved[0].site, proved[0].import.name.as_str()), (0x1004, "__stack_chk_fail"));
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn api_call_bindings_are_entry_local_and_cleared() {
    let _env = serial();
    let mut f = fixture::Fixture::new(fixture::Plt::Short);
    f.pic_collision = true;
    f.caller = Some(vec![0x3bc02800,0x480007fd,0x38600007,0x4e800020]);
    let path = common::scratch_file("ppc32-api", "elf");
    std::fs::write(&path, f.bytes()).unwrap();
    let mut prog = load(&path);
    prog.arch_mut().set_kuna_option("ppc32picimports", "on").unwrap();
    let entry = prog.lookup_symbol("check").unwrap();
    let mut fd = kuna_decomp::decompile_drive::build_and_follow_flow(prog.arch_mut(), "check", entry, 16).unwrap();
    assert_eq!(fd.ppc32_import_calls().count(), 1);
    let call = fd.ppc32_import_calls().next().unwrap();
    assert_eq!((call.site, call.target, call.import.address), (0x1004, Some(0x1800), 0x3000));
    assert!(fd.get_call_specs(0).proto().is_no_return());
    assert!(prog.function_named_at(0x1800).is_none_or(|name| name != "__stack_chk_fail"));
    fd.clear();
    assert_eq!(fd.ppc32_import_calls().count(), 0);
    std::fs::remove_file(path).unwrap();
}