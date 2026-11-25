use std::{path::PathBuf, rc::Rc, sync::atomic::{AtomicUsize, Ordering}};

use kuna_base::{address::Address, error::{KunaError, KunaResult}, xml::DocumentStorage};
use kuna_console::engine::{bootstrap_program, Bootstrap, ConsoleProgram, FlowKind};
use kuna_decomp::{decompile_drive::build_and_follow_flow, kuna_loadoptions::LoadOptions};

const TARGET: &str = "Carmel:BE:24:default:default";
const FRAME: &[u8] = &[
    0xa1, 0xac, 0x20, 0x21, 0xac, 0x62, // sequential
    0x80, 0x12, 0x34, 0xc6, 0x42, 0x40, // wide
    0x2b, 0xec, 0x00, 0x2b, 0xec, 0x00, // parallel
];

fn specs() -> [String; 1] {
    [PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs").display().to_string()]
}

fn raw(bytes: &[u8], base: u64, entries: &[u64]) -> KunaResult<ConsoleProgram> {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let path = std::env::temp_dir().join(format!(
        "kuna-carmel-{}-{}.bin", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed),
    ));
    let mut file = std::fs::File::create_new(&path).unwrap();
    std::io::Write::write_all(&mut file, bytes).unwrap();
    drop(file);
    let result = Bootstrap::new(&specs()).target(TARGET).options(&LoadOptions::default())
        .raw(path.to_str().unwrap(), base, entries);
    std::fs::remove_file(path).unwrap();
    result
}

#[test]
fn reproduces_fetch_framing_at_zero_and_nonzero_base() {
    for base in [0, 0x1000] {
        let mut program = raw(FRAME, base, &[base + 1]).unwrap();
        program.commit_pending_analysis().unwrap();
        for (slot, len) in [(0, 3), (1, 3), (2, 6), (4, 6)] {
            let word = base + slot;
            let byte = program.input_code_offset(word).unwrap();
            assert_eq!(byte, word * 3);
            assert_eq!(program.disassemble_at(byte).unwrap().0, len);
        }
        for slot in [3, 5, 6] {
            assert!(program.disassemble_at((base + slot) * 3).is_err());
        }
    }
}

#[test]
fn rejects_unmapped_and_mid_fetch_entries() {
    for (bytes, base, entry, diagnostic) in [
        (FRAME, 1, 1, "even word base"),
        (&FRAME[..17], 0, 0, "six-byte fetches"),
        (FRAME, 0xfffffc, 0xfffffc, "exceeds the target address space"),
        (FRAME, 0, 3, "not a mapped issue start"),
        (FRAME, 0, 5, "not a mapped issue start"),
        (FRAME, 0, 6, "outside mapped range"),
    ] {
        let error = match raw(bytes, base, &[entry]) {
            Err(error) => error,
            Ok(_) => panic!("accepted invalid base {base:#x}, entry {entry:#x}"),
        };
        assert!(error.to_string().contains(diagnostic), "{error}");
    }
}

#[test]
fn sequential_right_and_wide_have_distinct_register_outputs() {
    let program = raw(FRAME, 0x1000, &[0x1001]).unwrap();
    let register = |name: &[u8]| program.arch().translate().get_register_varnode(name).unwrap().offset;
    for (word, wanted, excluded) in [(0x1001, b"A1L", b"A0L"), (0x1002, b"A0L", b"A1L")] {
        let (_, ops) = program.raw_pcode_at(3 * word).unwrap();
        let writes = |offset| ops.iter().any(|op| op.output.as_ref().is_some_and(|vn| {
            vn.space == "register" && vn.offset == offset && vn.size == 2
        }));
        assert!(writes(register(wanted)) && !writes(register(excluded)));
    }
}

#[test]
fn direct_flow_preserves_odd_code_word_targets() {
    let bytes = [
        0x2b, 0x00, 0x38, 0x2b, 0xec, 0, // GOTO +7
        0x21, 0xac, 0x20, 0x2b, 0xec, 0,
        0x21, 0xac, 0x62, 0x2b, 0xec, 0,
        0xa1, 0xac, 0x20, 0x21, 0xac, 0xa4, // skip left, execute INC A2L
        0x2b, 0xd8, 0, 0x2b, 0xec, 0,
    ];
    let mut program = raw(&bytes, 0, &[0]).unwrap();
    let (_, ops) = program.raw_pcode_at(0).unwrap();
    let target = &ops.iter().find(|op| op.opcode == "BRANCH").unwrap().inputs[0];
    assert_eq!((&*target.space, target.offset), ("code", 21));
    let flow = program.insn_flow(0).unwrap();
    assert_eq!(flow.kind, FlowKind::Jump);
    assert_eq!(flow.targets, [21]);

    let space = Rc::clone(program.arch().manage().get_default_code_space().unwrap());
    let entry = Address::new(space, 0);
    let fd = build_and_follow_flow(program.arch_mut(), "odd_target", entry, 0).unwrap();
    let reached: Vec<_> = fd.obank().iter_all().map(|(seq, _)| seq.get_addr().get_offset()).collect();
    assert!(reached.contains(&21) && reached.contains(&24), "{reached:?}");
    assert!(reached.iter().all(|&addr| addr == 0 || addr >= 21), "{reached:?}");
}

#[test]
fn a_delay_needs_one_mapped_issue_with_supported_effects() {
    let goto = [0x2b, 0, 0x22, 0x2b, 0xec, 0];
    let ret = [0x2b, 0xd8, 0, 0x2b, 0xec, 0];
    let seq = [0xa1, 0xac, 0x20, 0x21, 0xac, 0x62];
    let wide = [0x80, 0x12, 0x34, 0xc6, 0x42, 0x40];
    for (next, pcode_len) in [(seq, 9), (wide, 12)] {
        let program = raw(&[goto, next, ret].concat(), 0, &[0]).unwrap();
        assert_eq!(program.disassemble_at(0).unwrap().0, 6);
        assert_eq!(program.raw_pcode_at(0).unwrap().0, pcode_len);
        assert_eq!(program.insn_flow(0).unwrap().targets, [12]);
    }
    for bytes in [goto.to_vec(), [goto, ret].concat()] {
        let program = raw(&bytes, 0, &[0]).unwrap();
        assert!(matches!(program.raw_pcode_at(0), Err(KunaError::Unimpl { .. })));
        assert!(program.insn_flow(0).is_none());
    }
}

#[test]
fn compact_dual_redirect_counts_only_retired_peers() {
    let base = 0x1000;
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    let mut bytes = [nop; 7].concat();
    bytes[..6].copy_from_slice(&[0x28, 0, 0x22, 0x2b, 0, 0x66]);
    bytes[18..21].copy_from_slice(&[0x21, 0xac, 0xa4]); // +6: INC A2L
    for (selector, delay_bytes, target, executes_increment) in [(0, 24, 12, true), (2, 18, 8, false)] {
        bytes[1] = selector;
        let program = raw(&bytes, base, &[base]).unwrap();
        let (length, ops) = program.raw_pcode_at(3 * base).unwrap();
        let a2l = program.arch().translate().get_register_varnode(b"A2L").unwrap();
        let writes_a2l = ops.iter().any(|op| op.output.as_ref().is_some_and(|vn|
            vn.space == "register" && vn.offset == a2l.offset && vn.size == a2l.size));
        let branch = ops.iter().find(|op| op.opcode == "BRANCH").unwrap();
        assert_eq!((length, writes_a2l), (delay_bytes, executes_increment));
        assert_eq!(branch.inputs[0].offset, 3 * (base + target));
        assert_eq!(program.insn_flow(3 * base).unwrap().targets, [3 * (base + target)]);
    }

    // Selector 1 chooses the backed left target, but the right lane's
    // speculative target still must be backed. Padding is not a CODE fetch.
    let program = raw(&bytes[..36], base, &[base]).unwrap();
    assert!(matches!(program.raw_pcode_at(3 * base), Err(KunaError::Unimpl { .. })));
    assert!(program.insn_flow(3 * base).is_none());
}

#[test]
fn conditional_return_veto_skips_stack_reads_after_six_retired_increments() {
    let base = 0x1000;
    let entry = base + 2;
    let compare = [0x20, 0x9a, 2, 0x2b, 0xec, 0];
    let inc = [0x21, 0xac, 0x20, 0x2b, 0xec, 0];
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    for (kind, target_width) in [(0xe0, 3), (0xe2, 4)] {
        let returning = [0x2b, kind, 0x77, 0x2b, 0xec, 0];
        let bytes = [compare, returning, inc, inc, inc, inc, inc, inc, nop].concat();
        let program = raw(&bytes, base, &[entry]).unwrap();
        let (len, ops) = program.raw_pcode_at(3 * entry).unwrap();
        assert_eq!(len, 42);
        let guard = ops.iter().position(|op| op.opcode == "CBRANCH").unwrap();
        let mut loads = ops.iter().enumerate().filter(|(_, op)| op.opcode == "LOAD");
        let first = loads.next().unwrap().0;
        let second = loads.next().unwrap().0;
        assert!(loads.next().is_none());
        let veto = guard + ops[guard].inputs[0].offset as usize;
        assert!(guard < first && second < veto);
        assert_eq!(ops[veto - 1].opcode, "RETURN");
        assert_eq!(ops[veto - 1].inputs[0].size, target_width);
        assert_eq!(ops[veto].opcode, "BRANCH");
        assert_eq!(ops[veto].inputs[0].offset, 3 * (entry + 14));

        let program = raw(&bytes[..48], base, &[entry]).unwrap();
        assert!(matches!(program.raw_pcode_at(3 * entry), Err(KunaError::Unimpl { .. })));
    }
}

#[test]
fn a_call_delayed_by_one_compact_slot_resumes_at_its_pushed_word() {
    let bytes = [
        0x2b, 0x20, 0x42, 0x2b, 0xec, 0, // CALL +8 delay 1 || NOP pushes word 4
        0xa1, 0xac, 0x20, 0x21, 0xac, 0x20, // the delay issue, then a slot the return skips
        0x2b, 0xd8, 0, 0x2b, 0xec, 0,
        0x2b, 0xec, 0, 0x2b, 0xec, 0,
        0x2b, 0xd8, 0, 0x2b, 0xec, 0,
    ];
    let mut program = raw(&bytes, 0, &[0]).unwrap();
    let space = Rc::clone(program.arch().manage().get_default_code_space().unwrap());
    let fd = build_and_follow_flow(program.arch_mut(), "delayed_call", Address::new(space, 0), 0).unwrap();
    let reached: Vec<_> = fd.obank().iter_all().map(|(seq, _)| seq.get_addr().get_offset()).collect();
    assert!(reached.contains(&12) && !reached.contains(&9), "{reached:?}");
}

#[test]
fn an_unknown_sequential_family_stops_both_inc_and_goto_before_issue() {
    let nop = [0x2b, 0xec, 0, 0x2b, 0xec, 0];
    for left in [[0xa1, 0xac, 0x20], [0xab, 0x00, 0x30]] {
        let fetch = [left, [0x7f, 0xff, 0xff]].concat();
        let program = raw(&[fetch.as_slice(), &nop].concat(), 0, &[0]).unwrap();
        assert_eq!(program.disassemble_at(0).unwrap().0, 3);
        assert!(matches!(program.raw_pcode_at(0), Err(KunaError::BadData { .. })));
        assert!(program.insn_flow(0).is_none());
    }
    let missing_opcode = [0xa1, 0xac, 0x20, 0x64, 0x91, 0x0d];
    let program = raw(&[missing_opcode, nop].concat(), 0, &[0]).unwrap();
    assert!(program.raw_pcode_at(0).unwrap().1.iter().any(|op| op.opcode == "INT_ADD"));
    assert!(matches!(program.raw_pcode_at(3), Err(KunaError::Unimpl { .. })));
}

#[test]
fn a_reserved_compact_operand_stops_its_issue_before_any_peer_executes() {
    let reserved = [0x2b, 0xf2, 0x01];
    let inc = [0x21, 0xac, 0x20];
    let parallel = raw(&[inc, reserved].concat(), 0, &[0]).unwrap();
    assert_eq!(parallel.disassemble_at(0).unwrap().0, 6);
    assert!(matches!(parallel.raw_pcode_at(0), Err(KunaError::BadData { .. })));

    let sequential = raw(&[[0xa1, inc[1], inc[2]], reserved].concat(), 0, &[0, 1]).unwrap();
    assert!(sequential.raw_pcode_at(0).unwrap().1.iter().any(|op| op.opcode == "INT_ADD"));
    assert!(matches!(sequential.raw_pcode_at(3), Err(KunaError::BadData { .. })));

    let assigned = raw(&[[0x2b, 0xf2, 0x81], [0x2b, 0xec, 0]].concat(), 0, &[0]).unwrap();
    assert!(matches!(assigned.raw_pcode_at(0), Err(KunaError::Unimpl { .. })));
}

#[test]
fn an_unbound_wide_extract_descriptor_cannot_issue() {
    for (selector, rejected) in [(0xfc, false), (0xfd, true), (0xfe, false), (0xff, true)] {
        let program = raw(&[selector, 0x89, 0x61, 0xf7, 0x8e, 0x8b], 0, &[0]).unwrap();
        assert_eq!(program.disassemble_at(0).unwrap().0, 6);
        let result = program.raw_pcode_at(0);
        if rejected {
            assert!(matches!(result, Err(KunaError::BadData { .. })));
        } else {
            assert!(matches!(result, Err(KunaError::Unimpl { .. })));
        }
    }
}

#[test]
fn an_unconnected_configuration_shifter_input_stops_before_execution() {
    let base = 0x1000;
    let blocked = raw(&[0x4e, 0xad, 0x81, 0xf1, 0x81, 0xf1], base, &[base]).unwrap();
    assert_eq!(blocked.disassemble_at(3 * base).unwrap().0, 6);
    assert!(matches!(blocked.raw_pcode_at(3 * base), Err(KunaError::BadData { .. })));
    let connected = raw(&[0x4e, 0xad, 0x81, 0xf1, 0x81, 0xe1], base, &[base]).unwrap();
    assert!(connected.raw_pcode_at(3 * base).unwrap().1.iter().any(|op| op.opcode == "INT_RIGHT"));
}

#[test]
fn unconnected_wide_mac_addends_reject_before_the_multiplier_latch() {
    let base = 0x1000;
    for packet in [
        [0xf8, 0x6d, 0xf8, 0xcb, 0x8d, 0xc4],
        [0xf8, 0x6d, 0xfd, 0xcb, 0x8d, 0xb4],
        [0xf8, 0x6d, 0xfa, 0xcb, 0x8d, 0x88],
    ] {
        let program = raw(&packet, base, &[base]).unwrap();
        assert_eq!(program.disassemble_at(base * 3).unwrap().0, 6);
        assert!(matches!(program.raw_pcode_at(base * 3), Err(KunaError::BadData { .. })));
    }
}

fn xml(chunks: &str) -> KunaResult<ConsoleProgram> {
    let source = format!("<binaryimage arch=\"{TARGET}\">{chunks}</binaryimage>");
    let mut docs = DocumentStorage::new();
    let root = docs.parse_document(source.as_bytes())?.get_root().clone();
    bootstrap_program(root, TARGET, &specs())
}

#[test]
fn only_complete_readonly_cliw_records_specialize_the_reference() {
    let code = r#"<bytechunk space="code" offset="0x18" readonly="true">8c6000dc61c22bd8002bec00</bytechunk>
                  <symbol space="code" offset="0x18" name="FUN_000008"/>"#;
    let bank = |template: &str, readonly: bool| {
        format!(r#"{code}<bytechunk space="cliw" offset="0" readonly="{readonly}">{template}</bytechunk>"#)
    };
    let product = "8097e0fffff03fffff7ffff8";
    let nop = "7ffffefffff03fffff7ffff8";
    let pcode = xml(&bank(product, true)).unwrap().raw_pcode_at(0x18).unwrap().1;
    assert!(pcode.iter().any(|op| op.opcode == "INT_MULT"));
    assert!(xml(&bank(nop, true)).unwrap().raw_pcode_at(0x18).unwrap().1.is_empty());

    for source in [code.to_owned(), bank(product, false), bank(&product[..22], true)] {
        let program = xml(&source).unwrap();
        let (len, mnemonic, index) = program.disassemble_at(0x18).unwrap();
        assert_eq!((len, mnemonic.as_str(), index.as_str()), (6, "cliwexe", "0x0()"));
        assert!(matches!(program.raw_pcode_at(0x18), Err(KunaError::Unimpl { .. })));
    }
}

#[test]
fn normalized_cliw_alias_retains_the_high_index_in_assembly() {
    let source = r#"<bytechunk space="code" offset="0" readonly="true">8c6001dce1c2</bytechunk>
                    <bytechunk space="cliw" offset="0xc" readonly="true">8097e0fffff03fffff7ffff8</bytechunk>"#;
    let program = xml(source).unwrap();
    let (_, mnemonic, index) = program.disassemble_at(0).unwrap();
    assert_eq!((mnemonic.as_str(), index.as_str()), ("cliwexe", "0x401()"));
    assert!(program.raw_pcode_at(0).unwrap().1.iter().any(|op| op.opcode == "INT_MULT"));
    let another_bank = r#"<bytechunk space="cliw" offset="0x3000">7ffffefffff03fffff7ffff8</bytechunk>"#;
    assert!(matches!(
        xml(&format!("{source}{another_bank}")).unwrap().raw_pcode_at(0),
        Err(KunaError::Unimpl { .. }),
    ));
}
