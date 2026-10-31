//! Tests for the direct-call encodings `option bejoin callers` scans for.

use super::*;

#[test]
fn powerpc_bl_and_bla_name_their_target() {
    assert_eq!(ppc(0x1000_0100, 0x4bff_ffe1), Some(0x1000_00e0));
    assert_eq!(ppc(0x1000_0100, 0x4800_0021), Some(0x1000_0120));
    assert_eq!(ppc(0x1000_0100, 0x4800_2003), Some(0x2000));
    assert_eq!(ppc(0x1000_0100, 0x4bff_ffe0), None, "b without the link bit is no call");
}

#[test]
fn mips_jal_and_bal_name_their_target() {
    assert_eq!(mips(0x40_0200, 0x0c10_0048), Some(0x40_0120));
    assert_eq!(mips(0x40_0200, 0x0411_fff0), Some(0x40_01c4));
    assert_eq!(mips(0x40_0200, 0x0810_0048), None, "j is no call");
}

#[test]
fn sparc_call_names_its_target() {
    assert_eq!(sparc(0x1_0100, 0x7fff_ffc0), Some(0x1_0000));
    assert_eq!(sparc(0x1_0100, 0x4000_0010), Some(0x1_0140));
    assert_eq!(sparc(0x1_0100, 0x9de3_bfa0), None, "save is no call");
}

#[test]
fn arm_bl_and_blx_name_their_target() {
    assert_eq!(a32(0x8000, 0xebff_fbfe), Some(0x7000));
    assert_eq!(a32(0x8000, 0x1b00_0002), Some(0x8010), "a conditional bl is a call");
    assert_eq!(a32(0x8000, 0xfb00_0002), Some(0x8012), "blx keeps its half-word bit");
    assert_eq!(a32(0x8000, 0xea00_0002), None, "b is no call");
}

#[test]
fn thumb_bl_and_blx_name_their_target() {
    assert_eq!(t32(0x8000, 0xf000, 0xfffe), Some(0x9000));
    assert_eq!(t32(0x8002, 0xf000, 0xeffe), Some(0x9000), "blx aligns the base down");
    assert_eq!(t32(0x8000, 0xf7ff, 0xfffe), Some(0x8000), "a negative offset");
    assert_eq!(t32(0x8000, 0xf000, 0xbffe), None, "b.w is no call");
}

#[test]
fn the_scan_reads_each_encoding_at_its_alignment() {
    let mut out = Vec::new();
    Isa::Ppc.calls(0x1000_0100, &[0x4b, 0xff, 0xff, 0xe1, 0x60, 0, 0, 0], 8, &mut out);
    assert_eq!(out, vec![(0x1000_00e0, 0x1000_0100)]);
    out.clear();
    Isa::Arm { little: true }.calls(0x8000, &[0xfe, 0xfb, 0xff, 0xeb], 4, &mut out);
    assert_eq!(out, vec![(0x7000, 0x8000)]);
    out.clear();
    Isa::Arm { little: false }.calls(0x8000, &[0xf0, 0x00, 0xff, 0xfe], 4, &mut out);
    assert!(out.contains(&(0x9000, 0x8000)), "{out:x?}");
}

#[test]
fn only_32_bit_big_endian_data_is_scanned() {
    assert_eq!(Isa::of("PowerPC:BE:32:default:default"), Some(Isa::Ppc));
    assert_eq!(Isa::of("ARM:LEBE:32:v8LEInstruction:default"), Some(Isa::Arm { little: true }));
    assert_eq!(Isa::of("sparc:BE:32:default:default"), Some(Isa::Sparc));
    assert_eq!(Isa::of("PowerPC:BE:64:default:default"), None);
    assert_eq!(Isa::of("MIPS:LE:32:default:default"), None);
}
