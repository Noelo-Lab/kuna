//! Tests for the call encodings `option bejoin callers` scans for.

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

/// gcc's `.cpload`: `lui $28,%hi(_gp_disp); addiu $28,$28,%lo(_gp_disp);
/// addu $28,$28,$25`, then `lw $25,%call16(add64)($28); jalr $25`.
const GCC_PIC: [u32; 9] = [
    0x3c1c_0002, 0x279c_8104, 0x0399_e021, 0x27bd_ffe0, 0x8f99_8018, 0xafbf_001c, 0xafbc_0010, 0x0320_f809, 0x0000_0000,
];

#[test]
fn the_pic_prologue_gives_the_got_pointer_and_its_registers() {
    assert_eq!(mips_gp(0x30c, &GCC_PIC), Some((0x1_8410, vec![28])));
    // clang computes it into a saved register: `addu $17,$2,$25`.
    let clang = [0x3c02_0002, 0x2442_8000, 0x27bd_ffd8, 0xafb1_0018, 0x0059_8821];
    assert_eq!(mips_gp(0x1000, &clang), Some((0x1_9000, vec![17])));
    // A function that sets no GOT pointer has none.
    assert_eq!(mips_gp(0x1000, &[0x27bd_ffe0, 0x0320_f809]), None);
}

#[test]
fn a_pic_call_reads_its_target_from_the_got_entry_its_load_names() {
    let got = |slot: u64| (slot == 0x1_8410 - 0x7fe8).then_some(0x2e0u32);
    let (mut calls, mut pic) = (Vec::new(), BTreeMap::new());
    mips_pic_calls(0x30c, &GCC_PIC, &mut |a| got(a), &mut calls, &mut pic);
    assert_eq!(calls, vec![(0x2e0, 0x328)]);
    assert_eq!(pic.get(&0x328), Some(&0x31c));
    // A page load then `addiu $25,$1,%lo(f)` (clang), through `$17`.
    let words = [0x3c02_0002, 0x2442_8000, 0x0059_8821, 0x8e21_8010, 0x2439_0120, 0x0320_f809, 0x0000_0000];
    let page = |slot: u64| (slot == 0x1_9000 - 0x7ff0).then_some(0x1_0000u32);
    let (mut calls, mut pic) = (Vec::new(), BTreeMap::new());
    mips_pic_calls(0x1000, &words, &mut |a| page(a), &mut calls, &mut pic);
    assert_eq!(calls, vec![(0x1_0120, 0x1014)]);
}

#[test]
fn a_pic_call_after_a_branch_or_another_write_of_its_register_is_not_read() {
    let got = |_: u64| Some(0x2e0u32);
    // `b` between the load and the call, and `move $25,$16` after the load.
    for (at, w) in [(5, 0x1000_0003u32), (6, 0x0200_c825)] {
        let mut words = GCC_PIC;
        words[at] = w;
        let (mut calls, mut pic) = (Vec::new(), BTreeMap::new());
        mips_pic_calls(0x30c, &words, &mut |a| got(a), &mut calls, &mut pic);
        assert!(calls.is_empty(), "{w:#x}: {calls:x?}");
    }
    // A saved GOT register written again other than by a returning reload
    // is no GOT pointer.
    let words = [0x3c02_0002, 0x2442_8000, 0x0059_8821, 0x1000_0001, 0x0000_0000, 0x0080_8825, 0x8e39_8010, 0x0320_f809, 0x0];
    let (mut calls, mut pic) = (Vec::new(), BTreeMap::new());
    mips_pic_calls(0x1000, &words, &mut |a| got(a), &mut calls, &mut pic);
    assert!(calls.is_empty(), "{calls:x?}");
}

#[test]
fn gp_holds_only_through_reloads_of_the_slot_it_was_saved_to() {
    let got = |_: u64| Some(0x2e0u32);
    // `sw $gp,16($sp)` in the prologue, then past a branch `$gp` is written by
    // the word at 7 before `lw $25,%call16(f)($gp); jalr $25`.
    let words = |w7: u32| {
        [0x3c1c_0002, 0x279c_8104, 0x0399_e021, 0x27bd_ffe0, 0xafbc_0010, 0x1000_0001, 0x0, w7, 0x8f99_8018, 0x0320_f809, 0x0]
    };
    let (mut calls, mut pic) = (Vec::new(), BTreeMap::new());
    mips_pic_calls(0x30c, &words(0x8fbc_0010), &mut |a| got(a), &mut calls, &mut pic);
    assert_eq!(calls, vec![(0x2e0, 0x330)]);
    // A load from a slot nothing saved `$gp` to, and a store conditional
    // into `$gp`, leave it holding something else.
    for w7 in [0x8fbc_0014u32, 0xe09c_0000] {
        let (mut calls, mut pic) = (Vec::new(), BTreeMap::new());
        mips_pic_calls(0x30c, &words(w7), &mut |a| got(a), &mut calls, &mut pic);
        assert!(calls.is_empty(), "{w7:#x}: {calls:x?}");
    }
}
