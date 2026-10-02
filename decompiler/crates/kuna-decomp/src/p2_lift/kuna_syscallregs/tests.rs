//! Logic-level tests for the family table.

use super::*;

#[test]
fn families_follow_the_processor_field() {
    assert_eq!(
        SyscallFamily::from_archid("ARM:LE:32:v8"),
        Some(SyscallFamily::Arm)
    );
    assert_eq!(
        SyscallFamily::from_archid("AARCH64:LE:64:v8A"),
        Some(SyscallFamily::AArch64)
    );
    assert_eq!(
        SyscallFamily::from_archid("RISCV:LE:64:RV64GC"),
        Some(SyscallFamily::RiscV)
    );
    assert_eq!(
        SyscallFamily::from_archid("MIPS:BE:32:default"),
        Some(SyscallFamily::Mips)
    );
    assert_eq!(
        SyscallFamily::from_archid("PowerPC:BE:32:default"),
        Some(SyscallFamily::PowerPc)
    );
    assert_eq!(SyscallFamily::from_archid("x86:LE:64:default:gcc"), None);
    assert_eq!(SyscallFamily::from_archid("sparc:BE:32:default"), None);
}

#[test]
fn the_result_register_is_a_candidate_input() {
    use SyscallFamily::*;
    for f in [Arm, AArch64, RiscV, PowerPc] {
        assert!(f.inputs(false).contains(&f.result()), "{f:?}");
    }
    assert_eq!(SyscallFamily::Mips.result(), "v0");
    assert_eq!(
        SyscallFamily::Mips.inputs(false),
        ["v0", "a0", "a1", "a2", "a3"]
    );
    assert_eq!(
        SyscallFamily::Mips.inputs(true),
        ["v0", "a0", "a1", "a2", "a3", "t0", "t1"]
    );
}

#[test]
fn the_number_register_comes_first() {
    assert_eq!(SyscallFamily::Arm.inputs(false)[0], "r7");
    assert_eq!(SyscallFamily::AArch64.inputs(false)[0], "x8");
    assert_eq!(SyscallFamily::RiscV.inputs(false)[0], "a7");
    assert_eq!(SyscallFamily::PowerPc.inputs(false)[0], "r0");
}

#[test]
fn auto_needs_the_userland_fact() {
    assert!(!SyscallRegsMode::Off.fires(true));
    assert!(!SyscallRegsMode::Auto.fires(false));
    assert!(SyscallRegsMode::Auto.fires(true));
    assert!(SyscallRegsMode::On.fires(false));
    assert_eq!(SyscallRegsMode::default(), SyscallRegsMode::Off);
}

#[test]
fn option_values_parse() {
    for (v, m) in [
        ("off", SyscallRegsMode::Off),
        ("auto", SyscallRegsMode::Auto),
        ("on", SyscallRegsMode::On),
    ] {
        assert_eq!(OptionSyscallRegs.apply(v).unwrap().0, m);
    }
    assert!(OptionSyscallRegs.apply("yes").is_err());
    for m in [
        SyscallRegsMode::Off,
        SyscallRegsMode::Auto,
        SyscallRegsMode::On,
    ] {
        assert_eq!(OptionSyscallRegs.apply(m.as_str()).unwrap().0, m);
    }
}
