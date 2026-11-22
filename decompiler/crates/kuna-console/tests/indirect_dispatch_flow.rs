//! Actual flow recovery: edge closure, failed retry diagnostics and costly stages.
use kuna_base::address::Address;
use kuna_console::engine::bootstrap_from_raw;
use kuna_decomp::decompile_drive::build_and_follow_flow;
use kuna_num::opcodes::OpCode;
use std::path::PathBuf;

fn flow(
    relative: &str,
    both_off: bool,
) -> (
    kuna_console::engine::ConsoleProgram,
    kuna_decomp::funcdata::Funcdata,
) {
    flow_with(relative, if both_off { &[("returnbranch", "off"), ("constselectjump", "off")] } else { &[] })
}

fn flow_with(
    relative: &str,
    options: &[(&str, &str)],
) -> (
    kuna_console::engine::ConsoleProgram,
    kuna_decomp::funcdata::Funcdata,
) {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut program = bootstrap_from_raw(
        root.join(relative).to_str().unwrap(),
        "MIPS:BE:64:octeon:default",
        0x400000,
        &[0x400000],
        None,
        &[root.join("specs").to_string_lossy().into_owned()],
    )
    .unwrap();
    for (name, value) in options {
        program.arch_mut().set_kuna_option(name, value).unwrap();
    }
    program.commit_pending_analysis().unwrap();
    let entry = Address::new(
        program
            .arch()
            .manage()
            .get_default_code_space()
            .unwrap()
            .clone(),
        0x400000,
    );
    let fd = build_and_follow_flow(program.arch_mut(), "bounce", entry, 0).unwrap();
    (program, fd)
}

#[test]
fn four_hundred_steps_use_two_full_recovery_stages() {
    let (program, fd) = flow(
        "tests/fixtures/indirect-flow-closure/chain-400-64.bin",
        false,
    );
    assert_eq!(fd.num_jump_tables(), 1);
    let table = fd.get_jump_table(0);
    assert_eq!(table.num_entries(), 401, "400 stores and the caller return");
    assert_eq!(table.get_recover_count(), 2, "entry and final SSA closure");
    assert!(!program
        .arch()
        .commentdb
        .comments()
        .iter()
        .any(|c| c.text.contains("more targets")));
}

#[test]
fn failed_real_retry_keeps_its_target_and_indirect_exit() {
    let (_, fd) = flow(
        "tests/fixtures/indirect-flow-closure/retry-failure-64.bin",
        false,
    );
    assert_eq!(fd.num_jump_tables(), 0);
    assert_eq!(open_cases(&fd), [1]);
}

/// Count direct case edges at each remaining indirect call. A return guard
/// uses a p-code-relative target and does not represent a local case.
fn open_cases(fd: &kuna_decomp::funcdata::Funcdata) -> Vec<usize> {
    fd.obank().iter_alive().filter(|&id| fd.obank().get(id).unwrap().code() == OpCode::CPUI_CALLIND).map(|id| {
        let site = fd.obank().get(id).unwrap().get_addr();
        fd.obank().iter_at(site).filter(|(_, id)| {
            let op = fd.obank().get(*id).unwrap();
            op.code() == OpCode::CPUI_CBRANCH && op.get_in(0)
                .and_then(|vn| fd.vbank().get(vn)).is_some_and(|v| !v.is_constant())
        }).count()
    }).collect()
}

#[test]
fn ordinary_register_closure_needs_only_the_reentry_value() {
    let path = "tests/fixtures/indirect-dispatch/reentry-64-r9-2.bin";
    let (_, fd) = flow_with(path, &[("returnbranch", "off")]);
    assert_eq!(fd.num_jump_tables(), 1);
    assert_eq!(fd.get_jump_table(0).num_entries(), 3);
    let (_, fd) = flow(path, true);
    assert_eq!(open_cases(&fd), [1], "off follows the first case and calls through other values");
}

fn warnings(program: &kuna_console::engine::ConsoleProgram) -> usize {
    program.arch().commentdb.comments().iter()
        .filter(|c| c.text.contains("Indirect jump recovery limit reached"))
        .count()
}

fn entries(fd: &kuna_decomp::funcdata::Funcdata) -> Vec<usize> {
    (0..fd.num_jump_tables()).map(|i| fd.get_jump_table(i as i32).num_entries() as usize).collect()
}

#[test]
fn unrelated_single_target_jumps_spend_no_retry_budget() {
    let path = "tests/fixtures/indirect-flow-closure/singletons-128-64.bin";
    let (program, fd) = flow(path, false);
    let sizes = entries(&fd);
    assert_eq!(sizes.len(), 129);
    assert_eq!(sizes.iter().filter(|&&n| n == 1).count(), 128, "{sizes:?}");
    assert_eq!(sizes.iter().filter(|&&n| n == 2).count(), 1, "the dispatcher keeps both steps");
    assert_eq!(warnings(&program), 0);
    let (_, fd) = flow_with(path, &[("constselectjump", "off")]);
    assert!(entries(&fd).iter().all(|&n| n == 1));
}

#[test]
fn every_dispatch_left_at_the_retry_cap_is_warned() {
    let (program, fd) = flow("tests/fixtures/indirect-flow-closure/two-chains-64.bin", false);
    let sizes = open_cases(&fd);
    assert_eq!(sizes.len(), 2, "{sizes:?}");
    assert!(sizes.iter().all(|&n| (2..80).contains(&n)), "both chains need more than the cap: {sizes:?}");
    assert_eq!(warnings(&program), 2);
}

/// Follow flow over a raw x86-64 image loaded and entered at 0x2000.
fn x86_flow(hex: &str) -> kuna_decomp::funcdata::Funcdata {
    let bytes: Vec<u8> = (0..hex.len()).step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect();
    let path = crate::common::scratch_file("x86-dispatch", "bin");
    std::fs::write(&path, bytes).unwrap();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut program = bootstrap_from_raw(
        path.to_str().unwrap(),
        "x86:LE:64:default:gcc",
        0x2000,
        &[0x2000],
        None,
        &[root.join("specs").to_string_lossy().into_owned()],
    )
    .unwrap();
    program.commit_pending_analysis().unwrap();
    let space = program.arch().manage().get_default_code_space().unwrap().clone();
    let fd = build_and_follow_flow(program.arch_mut(), "loop", Address::new(space, 0x2000), 0).unwrap();
    std::fs::remove_file(path).unwrap();
    fd
}

#[test]
fn an_unguarded_table_reentered_through_its_load_keeps_its_cases() {
    // top: movzbl (%rdi),%eax; add $1,%rdi; and $3,%eax; jmp *0x2040(,%rax,8)
    // Three cases jump back to top and one returns, as in a bytecode loop.
    let fd = x86_flow(concat!(
        "31d20fb6074883c70183e003ff24c54020000089d0c383c203ebe783f205ebe2",
        "6bd207ebdd000000000000000000000000000000000000000000000000000000",
        "132000000000000016200000000000001b200000000000002020000000000000",
    ));
    assert_eq!(fd.num_jump_tables(), 1);
    assert_eq!(fd.get_jump_table(0).num_entries(), 4);
    assert!(open_cases(&fd).is_empty());
}

#[test]
fn a_loaded_singleton_reentered_before_its_guard_keeps_an_indirect_exit() {
    // mov $0x2040,%ecx; guard: cmp $1,%edi; ja done; load (%rcx,%rax,8); jmp *%rax.
    // The first case moves the base to the third row and jumps back to the
    // guard. The zero second row ends the first recovery after one target.
    let fd = x86_flow(concat!(
        "b94020000083ff01771b89f8488b04c1ffe0c70607000000b950200000ebe6c7",
        "0617000000c30000000000000000000000000000000000000000000000000000",
        "122000000000000000000000000000001f20000000000000",
    ));
    assert_eq!(fd.num_jump_tables(), 0);
    assert_eq!(open_cases(&fd), [1]);
}
