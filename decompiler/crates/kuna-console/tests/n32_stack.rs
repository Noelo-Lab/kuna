//! Original GCC fixtures exercising stack addresses narrower than their GPRs.
use kuna_console::{
    engine::{bootstrap_from_object, EntrySelector},
    project::decompile_targets,
};
use std::path::PathBuf;

#[test]
fn gcc_stack_and_split_arguments_recover_the_declared_value() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for variant in ["octeon", "octeonplus", "octeon2", "octeon3"] {
        for abi in ["n32", "64"] {
            let target = format!(
                "MIPS:BE:64:{variant}{}:default",
                if abi == "n32" { "-32addr" } else { "" }
            );
            let mut prog = bootstrap_from_object(
                root.join(format!("tests/fixtures/octeon-abi/arguments-{abi}.elf"))
                    .to_str()
                    .unwrap(),
                &target,
                &[root.join("specs").to_string_lossy().into_owned()],
            )
            .unwrap();
            prog.arch_mut().set_kuna_option("ctypes", "on").unwrap();
            prog.commit_pending_analysis().unwrap();
            for (name, expression) in [
                ("pair_split_first", "p.a"),
                ("pair_split_second", "p.b"),
                ("pair_stack_second", "p.b"),
                ("quad_last_after", "value"),
                ("quad_stack_after", "value"),
                ("float_stack", "value"),
            ] {
                let entry = prog
                    .resolve_entry(&EntrySelector::Name(name.into()))
                    .unwrap();
                let result = decompile_targets(&mut prog, vec![entry], false, false, false);
                assert!(
                    result[0].error.is_none(),
                    "{target} {name}: {:?}",
                    result[0].error
                );
                let code = result[0].code.as_ref().unwrap();
                assert!(
                    code.contains(&format!("return {expression};")),
                    "{target} {name}: {code}"
                );
            }
        }
    }
}

#[test]
fn gcc_scalar_stack_slots_and_wide_values_remain_distinct() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for abi in ["n32", "64"] {
        let target = format!(
            "MIPS:BE:64:octeon{}:default",
            if abi == "n32" { "-32addr" } else { "" }
        );
        let mut prog = bootstrap_from_object(
            root.join(format!("tests/fixtures/mips-n32-stack/stack-{abi}.elf"))
                .to_str()
                .unwrap(),
            &target,
            &[root.join("specs").to_string_lossy().into_owned()],
        )
        .unwrap();
        prog.arch_mut().set_kuna_option("ctypes", "on").unwrap();
        prog.commit_pending_analysis().unwrap();
        for (name, expected) in [
            ("pointer_ninth", "return *p;"),
            ("signed_ninth", "return value;"),
            ("float_ninth", ".from = value"),
            ("upper_half", "return value >> 0x20;"),
            ("indirect_frame", "return (*callee)(value) + value;"),
        ] {
            let entry = prog
                .resolve_entry(&EntrySelector::Name(name.into()))
                .unwrap();
            let result = decompile_targets(&mut prog, vec![entry], false, false, false);
            assert!(
                result[0].error.is_none(),
                "{target} {name}: {:?}",
                result[0].error
            );
            let code = result[0].code.as_ref().unwrap();
            assert!(code.contains(expected), "{target} {name}: {code}");
        }
        let entry = prog
            .resolve_entry(&EntrySelector::Name("sum_ten".into()))
            .unwrap();
        let result = decompile_targets(&mut prog, vec![entry], false, false, false);
        let code = result[0].code.as_ref().unwrap();
        let returned = code
            .split_once("return ")
            .unwrap()
            .1
            .split_once(';')
            .unwrap()
            .0;
        let mut terms: Vec<_> = returned.split('+').map(str::trim).collect();
        terms.sort_unstable();
        assert_eq!(
            terms,
            ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j"],
            "{target}: {code}"
        );
    }
}

#[test]
fn gcc_pointer_uses_preserve_inferred_integers_and_physical_sp() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    for variant in ["octeon", "octeonplus", "octeon2", "octeon3"] {
        for inferred in [false, true] {
            let suffix = if inferred { "-inferred" } else { "" };
            let target = format!("MIPS:BE:64:{variant}-32addr:default");
            let mut prog = bootstrap_from_object(
                root.join(format!(
                    "tests/fixtures/mips-n32-stack/width-n32{suffix}.elf"
                ))
                .to_str()
                .unwrap(),
                &target,
                &[root.join("specs").to_string_lossy().into_owned()],
            )
            .unwrap();
            prog.arch_mut().set_kuna_option("ctypes", "on").unwrap();
            prog.commit_pending_analysis().unwrap();
            for (name, inferred_contract, declared_contract) in [
                (
                    "compare_bits",
                    "compare_bits(long long a0,long long a1)",
                    "raw == expected",
                ),
                (
                    "return_bits",
                    "unsigned long long return_bits(unsigned long long a0,",
                    "return raw;",
                ),
                (
                    "add_bits",
                    "add_bits(long long a0,long long a1,long long a2)",
                    "raw + delta == expected",
                ),
                ("sp_high", "// sp_hi", "// sp_hi"),
                ("sp_full", "CONCAT44(", "CONCAT44("),
                ("sp_and_stack", "// sp_hi", "+ ninth;"),
            ] {
                let entry = prog
                    .resolve_entry(&EntrySelector::Name(name.into()))
                    .unwrap();
                let result = decompile_targets(&mut prog, vec![entry], false, false, false);
                assert!(
                    result[0].error.is_none(),
                    "{target} {name}: {:?}",
                    result[0].error
                );
                let code = result[0].code.as_ref().unwrap();
                let expected = if inferred {
                    inferred_contract
                } else {
                    declared_contract
                };
                assert!(
                    code.contains(expected),
                    "{target} inferred={inferred} {name}: {code}"
                );
                if name.starts_with("sp_") {
                    assert!(code.contains("// sp_hi"), "{target} {name}: {code}");
                    assert!(!code.contains("return 0;"), "{target} {name}: {code}");
                }
            }
        }
    }
}
