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

fn decompile(libcsigs: &str) -> Option<String> {
    let root = repo_root();
    let specs = vec![root.join("specs").to_str()?.to_string()];
    let binary = root.join("decompiler/crates/kuna-analysis/tests/fixtures/libcsigs_pe_x86_64.exe");
    let mut program = match bootstrap_from_object(binary.to_str()?, "", &specs) {
        Ok(program) => program,
        Err(error) => {
            eprintln!(
                "verify_libc_proto_addresses: skipping (bootstrap failed; build `.sla` with \
                 `make specs`): {}",
                error.explain()
            );
            return None;
        }
    };
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
    Some(kuna_decomp::decompile_drive::print_c(
        program.arch_mut(),
        &function,
    ))
}

#[test]
fn memcmp_veneer_receives_the_imports_three_argument_prototype() {
    let Some(off) = decompile("off") else {
        return;
    };
    assert!(
        off.contains("memcmp(0x140002100)"),
        "without libcsigs the duplicate-name veneer loses two arguments:\n{off}"
    );

    let on = decompile("on").expect("the second bootstrap succeeds");
    assert!(
        on.contains("memcmp((void *)0x140002100,(void *)0x140002110,3)"),
        "the veneer must receive both buffers and the byte count:\n{on}"
    );
}
