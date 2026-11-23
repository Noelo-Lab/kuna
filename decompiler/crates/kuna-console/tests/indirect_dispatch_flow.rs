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
    x86_flow_with_hint(hex, None)
}

fn x86_flow_with_hint(hex: &str, hint: Option<&str>) -> kuna_decomp::funcdata::Funcdata {
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
    if let Some(hint) = hint {
        program.set_assertions(vec![kuna_console::assertsyntax::parse_directive(hint).unwrap()]);
        kuna_console::assertions::apply_program_scoped(&mut program);
        assert_eq!(program.assertion_outcomes()[0].status, "applied");
    }
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
    let image = concat!(
        "b94020000083ff01771b89f8488b04c1ffe0c70607000000b950200000ebe6c7",
        "0617000000c30000000000000000000000000000000000000000000000000000",
        "122000000000000000000000000000001f20000000000000",
    );
    let fd = x86_flow(image);
    assert_eq!(fd.num_jump_tables(), 0);
    assert_eq!(open_cases(&fd), [1]);

    // The caller can supply the complete pair of local targets across both visits.
    let stated = x86_flow_with_hint(image, Some("jumptable 2010 201f 2012"));
    assert!(open_cases(&stated).is_empty());
    assert_eq!(stated.num_jump_tables(), 1);
    let table = stated.get_jump_table(0);
    let op = stated.obank().get(table.get_indirect_op().unwrap()).unwrap();
    let block = stated.bblocks_ref().block(op.get_parent().unwrap());
    let mut targets: Vec<_> = (0..block.size_out())
        .map(|i| stated.bblocks_block_start(block.get_out(i)).get_offset()).collect();
    targets.sort_unstable();
    assert_eq!(targets, [0x2012, 0x201f]);
}

#[test]
fn a_goto_again_dispatch_with_masked_or_constant_indices_keeps_its_table() {
    // d: jmp *0x2060(,%rdx,8). Two cases fetch the next byte, mask it with & 3
    // and jump back to d; one sets the index to the constant 3. Every value
    // reads one of the four recovered rows, so the switch stays closed.
    let fd = x86_flow(concat!(
        "31c00fb61783e203ff24d56020000083c0010fb657014883c70183e203ebe983",
        "c002ba03000000ebdfc301c00fb657014883c70183e203ebcf00000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "0f200000000000001f2000000000000029200000000000002a20000000000000",
    ));
    assert_eq!(fd.num_jump_tables(), 1);
    assert_eq!(fd.get_jump_table(0).num_entries(), 4);
    assert!(open_cases(&fd).is_empty());
}

#[test]
fn a_case_that_points_the_fetch_at_a_constant_address_keeps_its_table() {
    // top: movzbl (%rdi),%eax; add $1,%rdi; and $3,%eax; jmp *0x2040(,%rax,8).
    // Case 2 sets %rdi to 0x2100 and jumps back to top. That fetch reads data
    // at a proven address, not a row; the masked index still reads a row.
    let fd = x86_flow(concat!(
        "31d20fb6074883c70183e003ff24c54020000089d0c383c203ebe7bf00210000",
        "ebe06bd207ebdb00000000000000000000000000000000000000000000000000",
        "132000000000000016200000000000001b200000000000002220000000000000",
    ));
    assert_eq!(fd.num_jump_tables(), 1);
    assert_eq!(fd.get_jump_table(0).num_entries(), 4);
    assert!(open_cases(&fd).is_empty());
}

#[test]
fn an_odd_only_index_keeps_its_table() {
    // top: movzbl (%rdi),%edx; add $1,%rdi; and $3,%edx; or $1,%edx;
    // movzbl %dl,%edx; jmp *0x2040(,%rdx,8). Only rows 1 and 3 are reachable.
    // Enumerating the last zero extension also tries 0, which reads row 0.
    let fd = x86_flow(concat!(
        "31f60fb6174883c70183e20383ca010fb6d2ff24d54020000089f0c383c603eb",
        "e100000000000000000000000000000000000000000000000000000000000000",
        "02200000000000001c2000000000000002200000000000001920000000000000",
    ));
    assert_eq!(fd.num_jump_tables(), 1);
    assert!(open_cases(&fd).is_empty());
}

#[test]
fn a_case_that_moves_the_table_base_opens_the_dispatch() {
    // top: movzbl (%rdi),%eax; add $1,%rdi; and $3,%eax; jmp *(%rcx,%rax,8)
    // with %rcx = 0x2060. Case 0 sets %rcx to a second table at 0x2080 and
    // jumps back to top, so the next dispatch reads rows outside the first table.
    let fd = x86_flow(concat!(
        "b96020000031d20fb6074883c70183e003ff24c1b98020000083c203ebe983c2",
        "05ebe489d0c36bd207ebdcb864000000c3000000000000000000000000000000",
        "0000000000000000000000000000000000000000000000000000000000000000",
        "14200000000000001e20000000000000232000000000000026200000000000002b",
        "200000000000002b200000000000002b200000000000002b20000000000000",
    ));
    assert_eq!(fd.num_jump_tables(), 0);
    assert!(!open_cases(&fd).is_empty());
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
        if !compatible {
            assert_eq!(fd.num_jump_tables(), 0);
            assert_eq!(open_cases(&fd), [1]);
            continue;
        }
        let jt = fd.get_jump_table(0);
        assert_eq!(jt.get_recover_count(), 2);
        assert_eq!(
            jt.address_table()
                .iter()
                .map(Address::get_offset)
                .collect::<Vec<_>>(),
            vec![0x400014, 0x400028]
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

#[test]
fn stated_destinations_are_the_outgoing_edges_and_leave_the_guard_intact() {
    verify_stated_edges("dispatch.elf", "jumptable 401009 401050 401020 401030 401050",
                        0x401009, &[0x401020, 0x401030, 0x401050], [0x401005, 0x40100b]);
}

#[test]
fn position_independent_jumps_keep_both_taken_edges_and_the_guard_miss() {
    verify_stated_edges("pie.elf", "jumptable 1005 1007 100d",
                        0x1005, &[0x1007, 0x100d], [0x1005, 0x1013]);
}

fn verify_stated_edges(image: &str, hint: &str, site: u64, taken: &[u64], guard_edges: [u64; 2]) {
    use kuna_console::{assertions, assertsyntax, engine};
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut prog = engine::bootstrap_from_object(
        root.join("tests/fixtures/stated-jump").join(image).to_str().unwrap(), "",
        &[root.join("specs").to_string_lossy().into_owned()],
    ).unwrap();
    prog.commit_pending_analysis().unwrap();
    prog.set_assertions(vec![assertsyntax::parse_directive(
        hint
    ).unwrap()]);
    assertions::apply_program_scoped(&mut prog);
    assert_eq!(prog.assertion_outcomes()[0].status, "applied");
    let entry = prog.resolve_entry(&engine::EntrySelector::parse("dispatch")).unwrap();
    for _ in 0..2 {
        let fd = build_and_follow_flow(prog.arch_mut(), "dispatch", entry.addr.clone(), 0).unwrap();
        let successors = |id| {
            let block = fd.bblocks_ref().block(id);
            let mut targets: Vec<_> = (0..block.size_out())
                .map(|i| fd.bblocks_block_start(block.get_out(i)).get_offset()).collect();
            targets.sort_unstable();
            targets
        };
        assert_eq!(fd.num_jump_tables(), 1);
        let table = fd.get_jump_table(0);
        assert_eq!(table.get_op_address().get_offset(), site);
        let branch = fd.obank().get(table.get_indirect_op().unwrap()).unwrap();
        assert_eq!(successors(branch.get_parent().unwrap()), taken);
        let guard = (0..fd.bblocks_get_size()).map(|i| fd.bblocks_get_block(i))
            .find(|&b| fd.bblocks_block_start(b) == entry.addr).unwrap();
        assert_eq!(successors(guard), guard_edges);
    }
}
