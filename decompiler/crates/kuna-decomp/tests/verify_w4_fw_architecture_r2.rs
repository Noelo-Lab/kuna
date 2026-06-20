//! Round-2 adversarial verifier tests for item `w4-fw-architecture`.
//!
//! Round 1 REJECTed on F1: the non-angr `func_` default name used a divergent
//! module-local `print_raw` (`<space>:0x<offset>`, no zero-pad, no
//! `byteToAddress`) instead of C++ `Architecture::nameFunction` ->
//! `Address::printRaw` -> `AddrSpace::printRaw` (space.cc:206). The repair
//! (ff45d18) swaps in kuna-base `Address::print_raw`.
//!
//! These tests pin the repaired path against the *exact* C++ `AddrSpace::printRaw`
//! algorithm, computed independently in this file from the C++ source — NOT via
//! the kuna-base helper the port now calls (so a regression in either the port
//! wiring OR the helper is caught):
//!
//!   `int4 sz = getAddrSize();`
//!   `if (sz > 4) { if ((offset>>32)==0) sz=4; else if ((offset>>48)==0) sz=6; }`
//!   `s << "0x" << setfill('0') << setw(2*sz) << hex << byteToAddress(offset,wordsize);`
//!   `if (wordsize>1) { int4 cut = offset % wordsize; if (cut!=0) s << '+' << cut; }`
//!   where `byteToAddress(val,ws) = val/ws`.
//!
//! Hunt-list targets exercised here: integer-width / zero-pad fidelity, the
//! `>4`-byte address shrink boundaries (`>>32`, `>>48`), word-size division and
//! the `+cut` tail (an off-by-one / truncation magnet).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};

use kuna_decomp::architecture::Architecture;

use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;

// --- scaffolding ------------------------------------------------------------

struct DummyImg;
impl LoadImage for DummyImg {
    fn get_file_name(&self) -> &str {
        "dummy"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("dummy"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

fn bare_sleigh() -> Sleigh {
    Sleigh::new(Box::new(DummyImg), Box::new(ContextInternal::new()))
}

fn proc_space(name: &str, addr_size: u32, wordsize: u32) -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        name,
        false,
        addr_size,
        wordsize,
        7,
        addrspace_flags::hasphysical,
        1,
        1,
    ))
}

/// Independent transcription of C++ `AddrSpace::printRaw` (space.cc:206),
/// computed straight from the C++ — the oracle the port must match.
fn cpp_print_raw_oracle(addr_size: u32, wordsize: u32, offset: u64) -> String {
    let mut sz: i32 = addr_size as i32;
    if sz > 4 {
        if (offset >> 32) == 0 {
            sz = 4;
        } else if (offset >> 48) == 0 {
            sz = 6;
        }
    }
    let val = offset / wordsize as u64; // byteToAddress
    let width = (2 * sz) as usize;
    let mut s = format!("0x{val:0width$x}");
    if wordsize > 1 {
        let cut = (offset % wordsize as u64) as i32;
        if cut != 0 {
            s.push_str(&format!("+{cut}"));
        }
    }
    s
}

/// What `Architecture::nameFunction` (non-angr) must produce.
fn name_function_oracle(addr_size: u32, wordsize: u32, offset: u64) -> String {
    format!("func_{}", cpp_print_raw_oracle(addr_size, wordsize, offset))
}

fn name_via_port(space: Rc<AddrSpace>, offset: u64) -> String {
    let mut arch = Architecture::new("t", bare_sleigh());
    arch.name_style_angr = false;
    arch.name_function(&Address::new(space, offset))
}

// --- tests ------------------------------------------------------------------

/// Zero-padding: a small offset in a 4-byte space pads to 8 hex digits with NO
/// space-name prefix (the exact F1 regression — was `func_ram:0x401000`).
#[test]
fn r2_func_name_zero_pads_4byte_no_space_prefix() {
    let got = name_via_port(proc_space("ram", 4, 1), 0x401000);
    assert_eq!(got, "func_0x00401000");
    assert_eq!(got, name_function_oracle(4, 1, 0x401000));
    assert!(!got.contains("ram"), "space name must not appear: {got}");
}

/// 8-byte space, low offset: the C++ `sz>4` branch shrinks the width to 4 when
/// `(offset>>32)==0`, so an 8-byte space still prints only 8 hex digits.
#[test]
fn r2_func_name_8byte_low_offset_shrinks_to_4() {
    let got = name_via_port(proc_space("ram64", 8, 1), 0xDEAD);
    let oracle = name_function_oracle(8, 1, 0xDEAD);
    assert_eq!(got, oracle);
    assert_eq!(got, "func_0x0000dead"); // 8 digits, not 16
}

/// 8-byte space, offset that uses bits [32,48): `(offset>>32)!=0` but
/// `(offset>>48)==0`, so width becomes 6 bytes -> 12 hex digits (the middle
/// boundary of the C++ shrink ladder — an easy off-by-one to drop).
#[test]
fn r2_func_name_8byte_mid_offset_width_6() {
    let off = 0x0000_1234_5678_9abc_u64; // bits 32..48 set, bits>=48 clear
    assert_ne!(off >> 32, 0);
    assert_eq!(off >> 48, 0);
    let got = name_via_port(proc_space("ram64", 8, 1), off);
    let oracle = name_function_oracle(8, 1, off);
    assert_eq!(got, oracle);
    // sz=6 -> setw(12); val already 12 hex digits, no extra leading pad.
    assert_eq!(got, "func_0x123456789abc");
}

/// 8-byte space, full-width offset (bit >=48 set): no shrink, prints all 16 hex
/// digits.
#[test]
fn r2_func_name_8byte_high_offset_full_width() {
    let off = 0xABCD_1234_5678_9abc_u64;
    assert_ne!(off >> 48, 0);
    let got = name_via_port(proc_space("ram64", 8, 1), off);
    let oracle = name_function_oracle(8, 1, off);
    assert_eq!(got, oracle);
    assert_eq!(got, "func_0xabcd123456789abc"); // 16 hex digits
}

/// Word-size > 1: `byteToAddress` divides the offset by wordsize, and an
/// off-cut offset appends `+cut`. This is the part the original divergent
/// helper dropped entirely.
#[test]
fn r2_func_name_wordsize_division_and_cut() {
    // 4-byte addr, wordsize 2, offset 0x4001 -> addr=0x4001/2=0x2000, cut=1.
    let got = name_via_port(proc_space("code", 4, 2), 0x4001);
    let oracle = name_function_oracle(4, 2, 0x4001);
    assert_eq!(got, oracle);
    assert_eq!(got, "func_0x00002000+1");

    // on-cut offset: no +cut tail.
    let got2 = name_via_port(proc_space("code", 4, 2), 0x4000);
    assert_eq!(got2, name_function_oracle(4, 2, 0x4000));
    assert_eq!(got2, "func_0x00002000");
}

/// Offset 0 in a 1-byte space: minimal width, just `func_0x00`. Guards the
/// `setw(2*sz)` boundary at the smallest sz.
#[test]
fn r2_func_name_zero_offset_1byte_space() {
    let got = name_via_port(proc_space("io", 1, 1), 0);
    let oracle = name_function_oracle(1, 1, 0);
    assert_eq!(got, oracle);
    assert_eq!(got, "func_0x00");
}
