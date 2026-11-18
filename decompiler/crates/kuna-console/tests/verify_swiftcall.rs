//! (kuna `swiftcall`) The convention an indirect call is given in a Swift
//! image, read off the decompiled call specs.
//!
//! Fixture: `tests/fixtures/swift_call_hook_elf` (README): `callHook` calls a
//! `@convention(c)` function pointer it loads from a global, `measure`
//! dispatches `area()` through the object's isa.

use std::path::PathBuf;

use kuna_base::address::Address;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram, UNBOUNDED_SIZE};
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::decompile_drive::build_and_follow_flow;
use kuna_num::opcodes::OpCode;

fn load(fixture: &str) -> ConsoleProgram {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures").join(fixture);
    assert!(bin.exists(), "missing fixture {bin:?}");
    let mut prog = bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    prog.arch_mut().set_kuna_option("swiftmetadata", "on").expect("swiftmetadata flips on");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    prog
}

/// The model of every indirect call in the function at `vma`, decompiled.
fn indirect_models(fixture: &str, vma: u64) -> Vec<String> {
    let mut prog = load(fixture);
    let space = prog.arch().manage().get_default_code_space().expect("a default code space").clone();
    let addr = Address::new(space, vma);
    let name = prog.arch().name_function(&addr);
    let fd = build_and_follow_flow(prog.arch_mut(), &name, addr, UNBOUNDED_SIZE).expect("the function builds");
    let mut status = ConsoleCommands::into_status(vec!["decompile".to_string()]);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
        dcp.fd = Some(fd);
    }
    execute(&mut status);
    let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
    let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
    let fd = dcp.fd.as_ref().expect("decompiled");
    (0..fd.num_calls())
        .map(|i| fd.get_call_specs(i))
        .filter(|c| fd.obank().get(c.get_op()).is_some_and(|o| o.code() == OpCode::CPUI_CALLIND))
        .map(|c| c.proto().get_model_name().to_string())
        .collect()
}

/// A C function pointer loaded from a field of the object a Swift function
/// is given, with swiftself left alone,
/// keeps the C model; a method reached through the object's isa, with
/// swiftself set for it, takes `__swiftcall`.
#[test]
fn only_a_swift_shaped_indirect_call_takes_the_swift_model() {
    let hook = indirect_models("swift_call_hook_elf", 0x22f0);
    assert_eq!(hook.len(), 1);
    assert_ne!(hook[0], "__swiftcall", "the C callback");
    assert_eq!(indirect_models("swift_call_hook_elf", 0x2320), ["__swiftcall"], "the vtable call");
}
