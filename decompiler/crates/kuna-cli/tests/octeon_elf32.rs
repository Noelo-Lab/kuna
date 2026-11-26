use crate::common;

const O64: &str = "MIPS:BE:64:octeon-32addr:o64";

fn image(name: &str) -> String {
    common::repo_root().join("tests/fixtures/octeon-elf32").join(name).to_str().unwrap().to_owned()
}

#[test]
fn octeon_elf32_declaring_o64_or_no_abi_loads_as_octeon_o64_without_a_target() {
    for name in ["o64.elf", "noabi.elf", "bare.elf"] {
        let (out, err, code) = common::run_kuna(&["disassemble", &image(name), "crc16"]);
        assert_eq!(code, 0, "{name}: {err}");
        assert!(out.contains("bbit0") && !out.contains("lwc2"), "{name}: {out}");
        let note = err.contains(&format!("no ABI declaration; assuming O64 ({O64})"));
        assert_eq!(note, name != "o64.elf", "{name}: {err}");
        let (auto, _, _) = common::run_kuna(&["decompile", &image(name), "describe"]);
        let (explicit, _, _) = common::run_kuna(&["decompile", "--target", O64, &image(name), "describe"]);
        assert_eq!(auto, explicit, "{name}");
        assert!(auto.contains("a1 >> 0x23 & 0x3ff,(unsigned int)(a1 >> 3) & 7)"), "{name}: {auto}");
    }
}

#[test]
fn a_contradicting_abi_marker_keeps_generic_selection_with_a_note_and_a_cop2_load_stays_one() {
    let (out, err, code) = common::run_kuna(&["disassemble", &image("mdebug32.elf"), "crc16"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("lwc2"), "{out}");
    assert!(err.contains("Octeon ELF32 image not loaded as Octeon O64: its .mdebug ABI marker names an ABI other than O64"), "{err}");
    let (out, err, code) = common::run_kuna(&["disassemble", "--target", O64, &image("mdebug32.elf"), "crc16"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("bbit0") && !err.contains("not loaded as Octeon O64"), "{out}{err}");
    let (out, err, code) = common::run_kuna(&["disassemble", &image("cop2.elf"), "load"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("c86ffffa") && out.contains("lwc2"), "{out}");
    assert!(!err.contains("Octeon"), "{err}");
}

#[test]
fn a_pic_shared_object_names_its_got_imports_and_a_later_generation_keeps_generic_selection() {
    let (auto, err, code) = common::run_kuna(&["decompile-all", &image("pic.so")]);
    assert_eq!(code, 0, "{err}");
    let (explicit, _, _) = common::run_kuna(&["decompile-all", "--target", O64, &image("pic.so")]);
    assert_eq!(auto, explicit);
    for want in ["free(", "strlen(a0) + 1", "switch", "counter"] {
        assert!(auto.contains(want), "{want}: {auto}");
    }
    assert!(!auto.contains("halt_baddata"), "{auto}");
    let (out, err, code) = common::run_kuna(&["disassemble", &image("octeon2.elf"), "crc16"]);
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("lwc2"), "{out}");
    assert!(err.contains("Octeon ELF32 image not loaded as Octeon O64: only the original Octeon has an O64 model"), "{err}");
}

