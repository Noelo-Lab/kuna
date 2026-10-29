//! A restored frame base does not make pointer-written locals immutable.
#![cfg(all(target_os = "linux", target_arch = "x86_64"))]
use crate::common;
use std::process::{Command, Output};

fn checked(command: &mut Command) -> Output {
    let output = command.output().expect("start fixture command");
    assert!(
        output.status.success(),
        "{command:?}: {:?}\n{}\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

#[test]
fn declared_pointer_writes_survive_a_restored_frame_base() {
    if !Command::new("clang")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success())
    {
        eprintln!("Clang unavailable; skipping i386 fixture");
        return;
    }
    let root = common::scratch_file("declared-pointer-frame-alias", "dir");
    std::fs::create_dir_all(&root).unwrap();
    for (name, contents) in [
        (
            "sample.s",
            include_str!("fixtures/declared_pointer_frame_alias/sample.s"),
        ),
        (
            "start.s",
            include_str!("fixtures/declared_pointer_frame_alias/start.s"),
        ),
        (
            "support.c",
            include_str!("fixtures/declared_pointer_frame_alias/support.c"),
        ),
        (
            "callbacks.h",
            include_str!("fixtures/declared_pointer_frame_alias/callbacks.h"),
        ),
        (
            "driver.c",
            include_str!("fixtures/declared_pointer_frame_alias/driver.c"),
        ),
    ] {
        std::fs::write(root.join(name), contents).unwrap();
    }
    let image = root.join("sample.so");
    checked(
        Command::new("clang")
            .current_dir(&root)
            .args([
                "--target=i386-linux-gnu",
                "-shared",
                "-nostdlib",
                "-fPIC",
                "-Wl,-Bsymbolic-functions",
                "sample.s",
                "support.c",
                "-o",
            ])
            .arg(&image),
    );
    let project = root.join("project");
    let specs = common::repo_root().join("specs");
    let (_, error, status) = common::run_kuna(&[
        "decompile-project",
        image.to_str().unwrap(),
        "--functions",
        "sample",
        "--sleighpath",
        specs.to_str().unwrap(),
        "--option",
        "stackviews",
        "on",
        "--assert",
        "typedef struct pair {float real;float imaginary;} pair;",
        "--assert",
        "prototype sample float __cdecl sample(float a,float b)",
        "--assert",
        "prototype helper void __cdecl helper(void)",
        "--assert",
        "prototype fill void __cdecl fill(pair *output)",
        "--assert-strict",
        "-o",
        project.to_str().unwrap(),
    ]);
    assert_eq!(status, 0, "{error}");
    for compiler in ["gcc", "clang"] {
        if !Command::new(compiler)
            .arg("--version")
            .output()
            .is_ok_and(|output| output.status.success())
        {
            continue;
        }
        for optimization in ["-O0", "-O2"] {
            for regenerated in [false, true] {
                let executable = root.join(format!("{compiler}-{optimization}-{regenerated}"));
                let mut command = Command::new(compiler);
                command.current_dir(&root).args([
                    "-m32",
                    "-nostdlib",
                    "-static",
                    "-fno-pic",
                    "-fno-pie",
                    "-fno-stack-protector",
                    optimization,
                    "start.s",
                    "support.c",
                    "driver.c",
                ]);
                if regenerated {
                    command
                        .args(["-DREGENERATED", "-include", "callbacks.h"])
                        .arg(project.join("sample.so.c"));
                } else {
                    command.arg("sample.s");
                }
                checked(command.arg("-o").arg(&executable));
                match Command::new(&executable).output() {
                    Ok(output) => assert!(
                        output.status.success(),
                        "{compiler} {optimization} regenerated={regenerated}: {:?}",
                        output.status
                    ),
                    Err(error) if error.raw_os_error() == Some(8) => {
                        eprintln!("Kernel cannot execute i386 ELF; fixture compiled successfully");
                        std::fs::remove_dir_all(root).unwrap();
                        return;
                    }
                    Err(error) => panic!("execute fixture: {error}"),
                }
            }
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn a_normalized_declared_pointer_refutes_a_stale_unaliased_mark() {
    use kuna_base::address::Address;
    use kuna_base::space::spacetype;
    use kuna_console::ifacedecomp::{execute, register_console_commands, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
    use kuna_console::ifaceterm::IfaceTerm;
    use kuna_console::kuna_console::register_kuna_commands;
    use kuna_decomp::action::Rule;
    use kuna_decomp::op::pcodeop_addlflags;
    use kuna_decomp::ruleaction_3::RuleIndirectCollapse;
    use kuna_decomp::varnode::varnode_flags;
    use std::rc::Rc;

    if !Command::new("clang").arg("--version").output().is_ok_and(|o| o.status.success()) {
        eprintln!("Clang unavailable; skipping i386 fixture");
        return;
    }
    let root = common::scratch_file("declared-pointer-alias-refresh", "dir");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("sample.s"), include_str!("fixtures/declared_pointer_frame_alias/sample.s")).unwrap();
    std::fs::write(root.join("support.c"), include_str!("fixtures/declared_pointer_frame_alias/support.c")).unwrap();
    let image = root.join("sample.so");
    checked(Command::new("clang").current_dir(&root).args([
        "--target=i386-linux-gnu", "-shared", "-nostdlib", "-fPIC", "-Wl,-Bsymbolic-functions",
        "sample.s", "support.c", "-o",
    ]).arg(&image));
    let script = format!("load file {}\noption stackviews on\noption togglerule indirectcollapse off\nread symbols\nparse line typedef struct pair {{float real;float imaginary;}} pair;\nmap prototype sample float __cdecl sample(float a,float b)\nmap prototype helper void __cdecl helper(void)\nmap prototype fill void __cdecl fill(pair *output)\nload function sample\ndecompile\n", image.display());
    let mut status = IfaceTerm::into_status("", script.as_bytes());
    register_decomp_commands(&mut status);
    register_kuna_commands(&mut status);
    register_console_commands(&mut status);
    let data = status.get_data_mut(DECOMPILE_MODULE).unwrap().as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
    data.spec_roots = vec![common::repo_root().join("specs").to_string_lossy().into_owned()];
    while !status.is_stream_finished() && !status.done {
        execute(&mut status);
        assert!(!status.is_in_error(), "{}", status.optr);
        status.optr.clear();
    }
    let data = status.get_data_mut(DECOMPILE_MODULE).unwrap().as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
    let fd = data.fd.as_mut().unwrap();
    let call = (0..fd.num_calls()).map(|i| fd.get_call_specs(i))
        .find(|call| call.proto().num_params() == 1).expect("declared fill call").get_op();
    let call_pc = fd.obank().get(call).unwrap().get_addr().clone();
    let guards: Vec<_> = fd.obank().iter_alive().filter_map(|id| {
        let op = fd.obank().get(id)?;
        let output = op.get_out()?;
        let value = fd.vbank().get(output)?;
        let annotation = fd.vbank().get(op.get_in(1)?)?;
        (op.get_addr() == &call_pc && annotation.get_space().get_type() == spacetype::IPTR_IOP
            && value.get_space().get_type() == spacetype::IPTR_SPACEBASE
            && [0xfffffff4, 0xfffffff8].contains(&value.get_offset())).then_some((id, output))
    }).collect();
    assert_eq!(guards.len(), 2, "both words written through the pair pointer");
    // A restored frame pointer can still be a merge when local alias marks
    // become available. Keep its writes until the merge has normalized.
    let original = fd.obank().get(call).unwrap().get_in(1).unwrap();
    let merge = fd.new_op(2, call_pc.clone());
    fd.op_set_opcode(merge, kuna_decomp::context::TypeOp::new(kuna_num::opcodes::OpCode::CPUI_MULTIEQUAL, 0, "MULTIEQUAL"));
    fd.op_set_input(merge, original, 0).unwrap();
    let unresolved = fd.new_unique(4, None);
    fd.op_set_input(merge, unresolved, 1).unwrap();
    let merged = fd.new_unique_out(4, merge).unwrap();
    fd.op_set_input(call, merged, 1).unwrap();
    for &(guard, output) in &guards {
        fd.vbank_mut().get_mut(output).unwrap().set_flags_pub(varnode_flags::nolocalalias);
        fd.obank_mut().get_mut(guard).unwrap().clear_additional_flag(pcodeop_addlflags::no_indirect_collapse);
        assert_eq!(RuleIndirectCollapse.apply_op(guard, fd), 0, "unresolved frame merge must retain the write");
    }
    fd.op_set_input(call, original, 1).unwrap();
    for (guard, output) in guards {
        // Reproduce local mapping's stale alias mark after the frame address has
        // normalized. The pointer now supplies stronger, concrete information.
        fd.vbank_mut().get_mut(output).unwrap().set_flags_pub(varnode_flags::nolocalalias);
        fd.obank_mut().get_mut(guard).unwrap().clear_additional_flag(pcodeop_addlflags::no_indirect_collapse);
        assert_eq!(RuleIndirectCollapse.apply_op(guard, fd), 0, "a declared pointer reaches this word");
    }
    // Positive alias evidence must not turn into an unconditional call barrier.
    let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
    let disjoint = fd.new_indirect_op(call, &Address::new(stack, 0xffffffe0), 4, 0);
    let output = fd.obank().get(disjoint).unwrap().get_out().unwrap();
    fd.vbank_mut().get_mut(output).unwrap().set_flags_pub(varnode_flags::nolocalalias);
    assert_eq!(RuleIndirectCollapse.apply_op(disjoint, fd), 1, "disjoint slot still collapses");
    std::fs::remove_dir_all(root).unwrap();
}
