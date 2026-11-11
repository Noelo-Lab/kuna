use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::kuna_funcptrnames;
use kuna_decomp::printlanguage::NamespaceStrategy::{
    AllNamespaces, MinimalNamespaces, NoNamespaces,
};
use std::path::PathBuf;

fn inspect<T>(caller: &str, check: impl FnOnce(&mut IfaceDecompData) -> T) -> T {
    inspect_with_setup(caller, |_| {}, check)
}

fn inspect_with_setup<T>(
    caller: &str,
    setup: impl FnOnce(&mut ConsoleProgram),
    check: impl FnOnce(&mut IfaceDecompData) -> T,
) -> T {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let fixture = root
        .join("decompiler/crates/kuna-analysis/tests/fixtures/function_pointer_namespaces_x86_64");
    let mut program = bootstrap_from_object(
        fixture.to_str().unwrap(),
        "",
        &[root.join("specs").display().to_string()],
    )
    .unwrap();
    program.commit_pending_analysis().unwrap();
    setup(&mut program);
    let commands = vec![
        "parse line extern unsigned long global_shadow(unsigned long target);".into(),
        "parse line extern unsigned long scope_shadow(unsigned long left);".into(),
        format!("load function {caller}"),
        "decompile".into(),
    ];
    let count = commands.len();
    let mut status = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut status);
    status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap()
        .conf = Some(program);
    for _ in 0..count {
        execute(&mut status);
        assert!(!status.inerror, "{}", status.optr);
    }
    check(
        status
            .get_data_mut(DECOMPILE_MODULE)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<IfaceDecompData>()
            .unwrap(),
    )
}

#[test]
fn function_values_follow_the_use_namespace_and_requested_policy() {
    for (caller, minimal) in [
        ("foreign_address", vec!["left"]),
        ("left::same_address", vec![]),
        ("right::cross_address", vec!["left"]),
    ] {
        inspect(caller, |data| {
            let program = data.conf.as_ref().unwrap();
            let target = program.lookup_symbol("left::target").unwrap();
            let fd = data.fd.as_ref().unwrap();
            let at = fd.get_address();
            assert_eq!(
                kuna_funcptrnames::prefix(
                    fd,
                    program.arch(),
                    &target,
                    at,
                    "target",
                    MinimalNamespaces
                ),
                Some(minimal.into_iter().map(String::from).collect())
            );
            assert_eq!(
                kuna_funcptrnames::prefix(fd, program.arch(), &target, at, "target", AllNamespaces),
                Some(vec!["left".into()])
            );
            assert_eq!(
                kuna_funcptrnames::prefix(fd, program.arch(), &target, at, "target", NoNamespaces),
                Some(vec![])
            );
        });
    }
}

#[test]
fn local_names_require_the_global_root_even_with_all_namespaces() {
    for (caller, target_name, expected) in [
        ("global_shadow", "target", vec![""]),
        ("scope_shadow", "left::target", vec!["", "left"]),
    ] {
        inspect(caller, |data| {
            let program = data.conf.as_ref().unwrap();
            let target = program.lookup_symbol(target_name).unwrap();
            let fd = data.fd.as_ref().unwrap();
            for strategy in [MinimalNamespaces, AllNamespaces] {
                assert_eq!(
                    kuna_funcptrnames::prefix(
                        fd,
                        program.arch(),
                        &target,
                        fd.get_address(),
                        "target",
                        strategy
                    ),
                    Some(expected.iter().map(|s| s.to_string()).collect())
                );
            }
        });
    }
}

#[test]
fn renamed_and_interior_function_references_decline_qualification() {
    inspect("foreign_address", |data| {
        let program = data.conf.as_ref().unwrap();
        let target = program.lookup_symbol("left::target").unwrap();
        let interior = Address::new(target.get_space().unwrap().clone(), target.get_offset() + 1);
        let fd = data.fd.as_ref().unwrap();
        assert!(kuna_funcptrnames::prefix(
            fd,
            program.arch(),
            &target,
            fd.get_address(),
            "asserted_callback",
            MinimalNamespaces
        )
        .is_none());
        assert!(kuna_funcptrnames::prefix(
            fd,
            program.arch(),
            &interior,
            fd.get_address(),
            "target",
            MinimalNamespaces
        )
        .is_none());
    });
}

#[test]
fn data_symbols_do_not_gain_function_qualification() {
    inspect("data_address", |data| {
        let program = data.conf.as_ref().unwrap();
        let (_, target, _) = program
            .global_data_symbol_addresses()
            .into_iter()
            .find(|(name, _, _)| name.contains("left") && name.contains("data"))
            .unwrap();
        let fd = data.fd.as_ref().unwrap();
        let (name, _, _, _) = fd
            .get_arch()
            .name_for_global_varnode_scoped(&target, 1, fd.get_address())
            .unwrap();
        assert!(kuna_funcptrnames::prefix(
            fd,
            program.arch(),
            &target,
            fd.get_address(),
            &name,
            MinimalNamespaces
        )
        .is_none());
    });
}

#[test]
fn conflicting_live_and_snapshot_scopes_do_not_choose_a_namespace() {
    inspect("foreign_address", |data| {
        let program = data.conf.as_mut().unwrap();
        let target = program.lookup_symbol("left::target").unwrap();
        let arch = program.arch_mut();
        let ty = arch.types().get_type_code().unwrap();
        arch.symboltab
            .add_function(
                arch.symboltab.get_global_scope().unwrap(),
                &target,
                "target",
                arch.min_funcsymbol_size,
                ty,
            )
            .unwrap();
        let fd = data.fd.as_ref().unwrap();
        assert!(kuna_funcptrnames::prefix(
            fd,
            program.arch(),
            &target,
            fd.get_address(),
            "target",
            MinimalNamespaces
        )
        .is_none());
    });
}

#[test]
fn function_values_decline_scopes_that_are_not_plain_cpp_names() {
    for scope_name in ["&T", "*mut T", "char", "bool", "Box<int>", "{impl#0}", "crate-name", "0scope", "safe_scope"] {
        inspect_with_setup(
            "foreign_address",
            |program| {
                let target = program.lookup_symbol("left::target").unwrap();
                let arch = program.arch_mut();
                let (old, _) = arch.symboltab.find_function_across_scopes(&target).unwrap();
                arch.symboltab.remove_symbol(old);
                let spaces = arch.manage().num_spaces();
                let (scope, name) = arch.symboltab
                    .find_create_scope_from_symbol_name(&format!("{scope_name}::target"), "::", None, spaces)
                    .unwrap();
                let ty = arch.types().get_type_code().unwrap();
                arch.symboltab.add_function(scope, &target, &name, arch.min_funcsymbol_size, ty).unwrap();
            },
            |data| {
                let program = data.conf.as_ref().unwrap();
                let target = program.lookup_symbol("left::target").unwrap();
                let fd = data.fd.as_ref().unwrap();
                let (name, offset, _, path) = fd.get_arch()
                    .name_for_global_varnode_scoped(&target, 1, fd.get_address()).unwrap();
                assert_eq!(name, "target");
                assert_eq!(offset, 0);
                assert_eq!(path, vec![scope_name.to_string()]);
                for strategy in [MinimalNamespaces, AllNamespaces] {
                    assert_eq!(
                        kuna_funcptrnames::prefix(fd, program.arch(), &target, fd.get_address(), "target", strategy),
                        (scope_name == "safe_scope").then(|| vec![scope_name.to_string()])
                    );
                }
            },
        );
    }
}
