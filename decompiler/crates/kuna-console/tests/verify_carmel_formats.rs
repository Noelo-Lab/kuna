use std::path::PathBuf;

use kuna_base::error::KunaResult;
use kuna_console::engine::{Bootstrap, ConsoleProgram, FlowKind};
use kuna_decomp::kuna_loadoptions::LoadOptions;
use kuna_sleigh::loadimage::section_flags;

const TARGET: &str = "Carmel:BE:24:default:default";

fn load(name: &str, target: &str, text_code: bool) -> KunaResult<ConsoleProgram> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let path = root.join("tests/fixtures/carmel/containers").join(name);
    let specs = [root.join("specs").display().to_string()];
    let options = LoadOptions { tasking_carmel_code: text_code, ..LoadOptions::default() };
    Bootstrap::new(&specs).options(&options).target(target).file(path.to_str().unwrap())
}

#[test]
fn relocated_calls_target_the_named_definition_or_unmapped_import() {
    for (file, caller, callee, prefix, mapped) in [
        ("caller-object.ieee695", 0, 12, [0x80, 0, 4, 0xcf, 0x82, 0], false),
        ("caller-linked.ieee695", 6, 0, [0xbf, 0xff, 0xfe, 0xcf, 0x82, 3], true),
    ] {
        let program = load(file, "", false).unwrap();
        assert_eq!(program.lookup_symbol("_call_addone").unwrap().get_offset(), caller);
        assert_eq!(program.lookup_symbol("_addone").unwrap().get_offset(), callee);
        assert_eq!(program.read_bytes(caller, 6).unwrap(), prefix);
        let flow = program.insn_flow(caller).unwrap();
        assert_eq!(flow.kind, FlowKind::Call);
        assert_eq!(flow.targets, [callee]);
        assert_eq!(program.read_bytes(callee, 6).is_some(), mapped);
    }
}

#[test]
fn absolute_banks_supply_the_actual_cliw_template_and_keep_rom_data_out_of_flow() {
    for name in ["issues-absolute.ieee695", "issues-absolute.tiof"] {
        let program = load(name, "", false).unwrap();
        assert_eq!(program.sections(), [(0, 0x48, section_flags::CODE | section_flags::READONLY)]);
        let (size, name, operands) = program.disassemble_at(0).unwrap();
        assert_eq!((size, name.as_str(), operands.as_str()), (6, "cliwexe", "0x0()"));
        assert!(program.raw_pcode_at(0).unwrap().1.iter().any(|op| op.opcode == "INT_MULT"));
    }
    for name in ["ram-copy.ieee695", "ram-copy.tiof"] {
        let program = load(name, "", false).unwrap();
        let start = 3 * 0x10162;
        assert_eq!(program.read_bytes(start, 6).unwrap(), [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]);
        assert!(program.sections().contains(&(start, 6, section_flags::DATA | section_flags::READONLY)));
        assert!(program.raw_pcode_at(start).is_err());
    }
}

#[test]
fn text_exports_reproduce_sparse_code_and_entry_without_implying_a_bank() {
    let absolute = load("sparse.ieee695", "", false).unwrap();
    for name in ["sparse.srec", "sparse.ihex"] {
        let err = load(name, TARGET, false).err().expect("requires a CODE-only assertion");
        assert!(err.explain().contains("KUNA_TASKING_CARMEL_CODE=on"));
        let text = load(name, "", true).unwrap();
        assert_eq!(text.image_metadata().unwrap().entry, Some(3 * 0x10100));
        for (word, words) in [(0x20, 0x2e), (0xfffc, 0x50), (0x10100, 0x62)] {
            let offset = 3 * word;
            let expected = absolute.read_bytes(offset, 3 * words).unwrap();
            assert_eq!(text.input_code_offset(word).unwrap(), offset);
            assert_eq!(text.read_bytes(offset, 3 * words).unwrap(), expected);
        }
        assert_eq!(text.read_bytes(3 * 0x80, 3), None);
        assert_eq!(text.disassemble_at(3 * 0xfffc).unwrap(), absolute.disassemble_at(3 * 0xfffc).unwrap());
    }
}
