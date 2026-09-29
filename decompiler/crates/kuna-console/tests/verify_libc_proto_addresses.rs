//! End-to-end regression for address-keyed libc prototypes on PE imports.
//!
//! `libcsigs_pe_x86_64.exe` imports `memcmp`; the PE resolver installs the same
//! name on its IAT slot and its `FF 25` veneer, alongside a same-named defined
//! export whose distinct provenance keeps it out of address-keyed library
//! knowledge. The caller reaches the veneer; because the export collides with the
//! import spelling, the unsafe global by-name park is suppressed altogether.
//! This test keeps the pass's off/on proof on the real object-loader path because
//! the XML datatest bootstrap never runs resolver-backed analysis passes.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::decompile_step::{decompile_one, DecompileSeed};
use kuna_console::engine::bootstrap_from_object;

const CALLER: u64 = 0x140001000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn decompile(libcsigs: &str) -> String {
    let root = repo_root();
    let specs = vec![root.join("specs").to_str().expect("UTF-8 specs path").to_string()];
    let binary = root.join("decompiler/crates/kuna-analysis/tests/fixtures/libcsigs_pe_x86_64.exe");
    let mut program = bootstrap_from_object(binary.to_str().expect("UTF-8 fixture path"), "", &specs)
        .expect("bootstrap fixture with built processor specs");
    program
        .arch_mut()
        .set_kuna_option("libcsigs", libcsigs)
        .expect("libcsigs is a registered option");
    program
        .commit_pending_analysis()
        .expect("analysis commit succeeds");

    let code_space = program
        .arch()
        .manage()
        .get_default_code_space()
        .expect("code space")
        .clone();
    let entry = Address::new(Rc::clone(&code_space), CALLER);
    let extent = program.declared_extent(CALLER);
    let step = decompile_one(
        program.arch_mut(),
        "sub_140001000",
        entry,
        extent,
        &DecompileSeed::plain(&[], &[]),
        &[],
    );
    let function = step.result.expect("the PE caller decompiles");
    kuna_decomp::decompile_drive::print_c(
        program.arch_mut(),
        &function,
    )
}

#[test]
fn memcmp_veneer_receives_the_imports_three_argument_prototype() {
    let off = decompile("off");
    assert!(
        off.contains("memcmp(0x140002100)"),
        "without libcsigs the duplicate-name veneer loses two arguments:\n{off}"
    );

    let on = decompile("on");
    // The buffers are typed `void *` either way: as casts, or (`globalref`,
    // default on) as the addresses of the globals they name.
    assert!(
        on.contains("memcmp((void *)0x140002100,(void *)0x140002110,3)")
            || on.contains("memcmp(&dat_140002100,&dat_140002110,3)"),
        "the veneer must receive both buffers and the byte count:\n{on}"
    );
}
