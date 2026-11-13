use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use std::path::PathBuf;

fn program(language: &str) -> ConsoleProgram {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let fixture = root.join(format!(
        "decompiler/crates/kuna-analysis/tests/fixtures/dwarf_namespaces_{language}_x86_64"
    ));
    let mut program = bootstrap_from_object(
        fixture.to_str().unwrap(),
        "",
        &[root.join("specs").display().to_string()],
    )
    .unwrap();
    program
}

fn symbols(program: &ConsoleProgram, offset: u64, expected: &str, global: bool) {
    let arch = program.arch();
    let address = Address::new(
        arch.manage().get_default_code_space().unwrap().clone(),
        offset,
    );
    assert_eq!(
        arch.symboltab
            .function_display_name_across_scopes(&address)
            .as_deref(),
        Some(expected)
    );
    assert_eq!(
        arch.symboltab
            .find_function(arch.symboltab.get_global_scope().unwrap(), &address)
            .is_some(),
        global
    );
}

#[test]
fn cpp_debug_names_do_not_shadow_their_namespace() {
    let mut program = program("cpp");
    program.commit_pending_analysis().unwrap();
    for (offset, name) in [
        (0x1150, "left::target"),
        (0x1170, "left::deep::target"),
        (0x1180, "right::target"),
    ] {
        symbols(&program, offset, name, false);
    }
    symbols(&program, 0x11a0, "target", true);
    symbols(&program, 0x11b0, "direct_left", true);
}

#[test]
fn rust_debug_names_retain_their_crate_and_module_paths() {
    let mut program = program("rust");
    program.commit_pending_analysis().unwrap();
    for (offset, name) in [
        (0x158c0, "dwarf_namespaces::left::target"),
        (0x158e0, "dwarf_namespaces::left::deep::target"),
        (0x158f0, "dwarf_namespaces::right::target"),
        (0x15910, "dwarf_namespaces::target"),
    ] {
        symbols(&program, offset, name, false);
    }
    symbols(&program, 0x15920, "direct_left", true);
}

#[test]
fn existing_global_function_identity_is_preserved() {
    let mut program = program("cpp");
    let arch = program.arch_mut();
    let address = Address::new(
        arch.manage().get_default_code_space().unwrap().clone(),
        0x1150,
    );
    let ty = arch.types().get_type_code().unwrap();
    arch.symboltab
        .add_function(
            arch.symboltab.get_global_scope().unwrap(),
            &address,
            "asserted_target",
            arch.min_funcsymbol_size,
            ty,
        )
        .unwrap();
    program.commit_pending_analysis().unwrap();
    symbols(&program, 0x1150, "asserted_target", true);
    let db = &program.arch().symboltab;
    let (scope, _) = db.resolve_scope_from_symbol_name("left::target", "::", db.get_global_scope());
    assert!(db.find_function(scope.unwrap(), &address).is_some());
}
