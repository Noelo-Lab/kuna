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
fn failed_real_retry_keeps_its_target_and_warns() {
    let (program, fd) = flow(
        "tests/fixtures/indirect-flow-closure/retry-failure-64.bin",
        false,
    );
    assert_eq!(fd.num_jump_tables(), 1);
    let table = fd.get_jump_table(0);
    assert_eq!(table.num_entries(), 1);
    assert_eq!(table.get_recover_count(), 2);
    let op = fd.obank().get(table.get_indirect_op().unwrap()).unwrap();
    assert_eq!(op.code(), OpCode::CPUI_BRANCHIND);
    assert!(program.arch().commentdb.comments().iter().any(|c| c
        .text
        .contains("Re-entered indirect jump may have more targets")));
}

#[test]
fn ordinary_register_closure_needs_only_the_reentry_value() {
    let path = "tests/fixtures/indirect-dispatch/reentry-64-r9-2.bin";
    let (_, fd) = flow_with(path, &[("returnbranch", "off")]);
    assert_eq!(fd.num_jump_tables(), 1);
    assert_eq!(fd.get_jump_table(0).num_entries(), 3);
    let (_, fd) = flow(path, true);
    assert_eq!(fd.get_jump_table(0).num_entries(), 1, "off keeps upstream's first target");
}

fn warnings(program: &kuna_console::engine::ConsoleProgram) -> usize {
    program.arch().commentdb.comments().iter()
        .filter(|c| c.text.contains("Re-entered indirect jump may have more targets"))
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
    let sizes: Vec<_> = entries(&fd).into_iter().filter(|&n| n > 1).collect();
    assert_eq!(sizes.len(), 2, "{sizes:?}");
    assert!(sizes.iter().all(|&n| n < 80), "both chains need more than the cap: {sizes:?}");
    assert_eq!(warnings(&program), 2);
}

#[test]
fn word_addressed_dispatch_follows_bytes_and_labels_selectors() {
    use kuna_decomp::decompile_drive::{decompile_func, print_c};

    // Original CP1600 words. MVII loads a word address into R1, JR R1
    // dispatches, each MVO stores R0, and the last step returns via R5.
    for (tag, align_bits) in [(0, 0), (1, 2)] {
        let mut words = [0u16; 80];
        words[..3].copy_from_slice(&[0x2b9, 0x18 | tag, 0x08f]);
        for (step, next, slot) in [(0x18, 0x30, 0x100), (0x30, 0x48, 0x101)] {
            words[step..step + 7].copy_from_slice(&[0x240, slot, 0x2b9, next | tag, 4, 0x300, 2]);
        }
        words[0x48..0x4b].copy_from_slice(&[0x240, 0x102, 0x0af]);
        let path = crate::common::scratch_file("word-dispatch", "bin");
        std::fs::write(
            &path,
            words
                .into_iter()
                .flat_map(u16::to_be_bytes)
                .collect::<Vec<_>>(),
        )
        .unwrap();
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
        let mut program = bootstrap_from_raw(
            path.to_str().unwrap(),
            "CP1600:BE:16:default",
            0,
            &[0],
            None,
            &[root.join("specs").to_string_lossy().into_owned()],
        )
        .unwrap();
        program.arch_mut().funcptr_align = align_bits;
        program.commit_pending_analysis().unwrap();
        let entry = Address::new(
            program
                .arch()
                .manage()
                .get_default_code_space()
                .unwrap()
                .clone(),
            0,
        );
        let fd = build_and_follow_flow(program.arch_mut(), "steps", entry.clone(), 0).unwrap();
        let jt = fd.get_jump_table(0);
        assert_eq!(
            jt.address_table()
                .iter()
                .map(Address::get_offset)
                .collect::<Vec<_>>(),
            [0x30, 0x60, 0x90]
        );
        assert_eq!(jt.get_recover_count(), 1, "all arriving values were proved locally");
        let fd = decompile_func(program.arch_mut(), "steps", entry, 0).unwrap();
        let body = print_c(program.arch_mut(), &fd);
        for word in [0x18, 0x30, 0x48] {
            assert!(body.contains(&format!("case {:#x}:", word | tag)), "{body}");
        }
        for slot in ["dat_200", "dat_202", "dat_204"] {
            assert!(body.contains(slot), "{body}");
        }
        std::fs::remove_file(path).unwrap();
    }
}

fn restart_with_hint(
    program: &mut kuna_console::engine::ConsoleProgram,
    mut fd: kuna_decomp::funcdata::Funcdata,
    jump: Address,
    targets: Vec<Address>,
) -> kuna_decomp::funcdata::Funcdata {
    use kuna_decomp::decompile_drive::{
        decompile_func_full_with_override_dyn_prefollowed, DrivePurpose,
    };
    let entry = fd.get_address().clone();
    fd.get_override_mut().insert_multistage_jump(jump.clone());
    fd.get_override_mut().insert_multistage_targets(jump, targets);
    fd.set_restart_pending(true);
    decompile_func_full_with_override_dyn_prefollowed(
        program.arch_mut(), "bounce", entry, 0, &[], &[], &[], None, &[], &[], &[], Some(fd),
        DrivePurpose::SingleFunction,
    ).unwrap()
}

#[test]
fn failed_retry_uses_a_compatible_multistage_hint() {
    for compatible in [true, false] {
        let (mut program, fd) = flow(
            "tests/fixtures/indirect-flow-closure/retry-failure-64.bin",
            false,
        );
        let entry = fd.get_address().clone();
        let at = |offset| Address::new(entry.get_space().unwrap().clone(), offset);
        // Seed the two targets a late model would carry into its restart. Without
        // the hint the very same image retains just the first one (tested above).
        let fd = restart_with_hint(&mut program, fd, at(0x40000c), vec![
            at(if compatible { 0x400014 } else { 0x400018 }), at(0x400028),
        ]);
        let jt = fd.get_jump_table(0);
        assert_eq!(jt.get_recover_count(), 2);
        let expected = if compatible {
            vec![0x400014, 0x400028]
        } else {
            vec![0x400014]
        };
        assert_eq!(
            jt.address_table()
                .iter()
                .map(Address::get_offset)
                .collect::<Vec<_>>(),
            expected
        );
    }
}

#[test]
fn ordinary_restart_uses_guarded_targets_when_the_partial_singleton_differs() {
    // LUI/ORI materialize the first return block in r9. JR r9 has no re-entry.
    // The other two return blocks provide distinct mapped targets for the hint.
    let words: [u32; 11] = [
        0x3c090040, 0x35290014, 0x01200008, 0, 0, 0x03e00008, 0, 0x03e00008, 0, 0x03e00008, 0,
    ];
    let path = crate::common::scratch_file("ordinary-hint", "bin");
    std::fs::write(&path, words.into_iter().flat_map(u32::to_be_bytes).collect::<Vec<_>>()).unwrap();
    let (mut program, fd) = flow(path.to_str().unwrap(), false);
    let space = fd.get_address().get_space().unwrap().clone();
    let at = |offset| Address::new(space.clone(), offset);
    assert_eq!(fd.get_jump_table(0).address_table(), &[at(0x400014)]);
    let guarded = [at(0x40001c), at(0x400024)];
    let fd = restart_with_hint(&mut program, fd, at(0x400008), guarded.to_vec());
    let table = fd.get_jump_table(0);
    assert_eq!(table.get_recover_count(), 2);
    assert_eq!(table.address_table(), &guarded);
    std::fs::remove_file(path).unwrap();
}

fn aligned_chain(language: &str, steps: u32) -> (Vec<u64>, i32, usize) {
    // The first instruction points r9 at step zero. Each later step sets r9
    // to the next step and branches back to the same indirect jump at 4.
    let arm = language.starts_with("AARCH64");
    let set = |addr: u32| {
        if arm {
            0xd2800009 | (addr << 5)
        } else {
            0x03800009 | (addr << 10)
        }
    };
    let mut words = vec![set(8u32), if arm { 0xd61f0120 } else { 0x4c000120 }];
    for i in 1..steps {
        words.push(set(8 + 8 * i)); // MOVZ X9 or ORI r9,zero
        let delta = 1u32.wrapping_sub(words.len() as u32) & 0x03ff_ffff;
        words.push(if arm {
            0x14000000 | delta
        } else {
            0x50000000 | ((delta & 0xffff) << 10) | (delta >> 16)
        });
    }
    words.push(if arm { 0xd65f03c0 } else { 0x4c000020 }); // RET
    let path = crate::common::scratch_file("aligned-chain", "bin");
    std::fs::write(
        &path,
        words
            .into_iter()
            .flat_map(u32::to_le_bytes)
            .collect::<Vec<_>>(),
    )
    .unwrap();
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut program = bootstrap_from_raw(
        path.to_str().unwrap(),
        language,
        0,
        &[0],
        None,
        &[root.join("specs").to_string_lossy().into_owned()],
    )
    .unwrap();
    program.commit_pending_analysis().unwrap();
    let entry = Address::new(
        program
            .arch()
            .manage()
            .get_default_code_space()
            .unwrap()
            .clone(),
        0,
    );
    let fd = build_and_follow_flow(program.arch_mut(), "steps", entry, 0).unwrap();
    assert_eq!(fd.num_jump_tables(), 1);
    let table = fd.get_jump_table(0);
    std::fs::remove_file(path).unwrap();
    (
        table
            .address_table()
            .iter()
            .map(Address::get_offset)
            .collect(),
        table.get_recover_count(),
        warnings(&program),
    )
}

#[test]
fn aligned_register_dispatch_chains_keep_the_local_fast_path() {
    for language in ["AARCH64:LE:64:v8A", "Loongarch:LE:64:lp64d"] {
        let (targets, stages, warnings) = aligned_chain(language, 200);
        assert_eq!(
            targets,
            (0..200).map(|i| 8 + i * 8).collect::<Vec<_>>(),
            "{language}"
        );
        assert!(stages <= 2, "{language}: {stages} full recoveries");
        assert_eq!(warnings, 0, "{language}");
    }
}
