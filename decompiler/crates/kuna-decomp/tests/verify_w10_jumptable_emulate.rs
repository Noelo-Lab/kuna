//! Verifier tests for the W10 jump-table-recovery foundation
//! (item `rport/w10-jumptable-switch`): the syntax-tree emulator support the
//! recovery chain needs — `Architecture::get_load_image_value` (the read-only
//! switch-table read, C++ `EmulatePcodeOp::getLoadImageValue`,
//! `emulateutil.cc:30`) and its endian/mask handling.
//!
//! These pin the load-image read primitive that `EmulateFunction::executeLoad`
//! (jumptable.cc:114) drives over the read-only jump table.  The address-flow
//! and model recovery on top of it remain seamed (reported as the wave's
//! remaining blocker); these lock down the leaf the rest builds on, on a
//! hand-built little-/big-endian `ram` space so the byte order matches the C++
//! `loadFill`+`byte_swap`+mask sequence exactly.

use std::cell::RefCell;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
};

use kuna_sleigh::loadimage::LoadImage;

use kuna_decomp::context::ArchContext;

/// A trivial in-memory load image over a single base address, mirroring the
/// corpus's read-only `<bytechunk>`.  `load_fill` copies the window's bytes,
/// raising `DataUnavail` (C++ `DataUnavailError`) outside it.
struct MemImage {
    base: u64,
    bytes: Vec<u8>,
}

impl LoadImage for MemImage {
    fn get_file_name(&self) -> &str {
        "mem"
    }
    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        let off = addr.get_offset();
        for (i, b) in ptr.iter_mut().enumerate() {
            let pos = off.wrapping_add(i as u64);
            if pos < self.base || (pos - self.base) as usize >= self.bytes.len() {
                return Err(KunaError::data_unavail("out of image"));
            }
            *b = self.bytes[(pos - self.base) as usize];
        }
        Ok(())
    }
    fn get_arch_type(&self) -> Vec<u8> {
        b"mem".to_vec()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

fn manager(big_endian: bool) -> (Rc<AddrSpaceManager>, Rc<AddrSpace>) {
    let mut m = AddrSpaceManager::new();
    m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        big_endian,
        8, // addr size
        1, // word size
        2, // index
        addrspace_flags::hasphysical,
        1, // delay
        1, // deadcodedelay
    ));
    m.insert_space(Rc::clone(&ram)).unwrap();
    (Rc::new(m), ram)
}

fn arch_with_image(big_endian: bool, base: u64, bytes: Vec<u8>) -> (ArchContext, Rc<AddrSpace>) {
    let (manage, ram) = manager(big_endian);
    let mut arch = ArchContext::new_shared(manage);
    arch.loader = Some(Rc::new(RefCell::new(
        Box::new(MemImage { base, bytes }) as Box<dyn LoadImage>
    )));
    (arch, ram)
}

#[test]
fn w10_loadimage_value_little_endian_masks_to_size() {
    // The x86-64 corpus jump table is little-endian 4-byte signed offsets.
    // 0xa1 0xff 0xff 0xff at 0x1000c0 -> 0xffffffa1 read as a 4-byte value.
    let (arch, ram) = arch_with_image(false, 0x1000c0, vec![0xa1, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00]);
    let addr = Address::new(Rc::clone(&ram), 0x1000c0);
    let v = arch.get_load_image_value(&addr, 4).unwrap();
    assert_eq!(v, 0xffff_ffa1, "little-endian 4-byte read masked to size");
}

#[test]
fn w10_loadimage_value_size_one_byte_is_masked_not_shifted_on_le() {
    let (arch, ram) = arch_with_image(false, 0, vec![0x7e, 0x12, 0x34, 0x56, 0, 0, 0, 0]);
    let addr = Address::new(Rc::clone(&ram), 0);
    // sz=1, little-endian: result &= calc_mask(1) -> low byte only.
    assert_eq!(arch.get_load_image_value(&addr, 1).unwrap(), 0x7e);
}

#[test]
fn w10_loadimage_value_big_endian_shifts_high_bytes_down() {
    // Big-endian: the C++ path does `res >>= (sizeof(uintb)-sz)*8` (NOT a mask),
    // so a 2-byte read of 0x12 0x34 yields 0x1234.
    let (arch, ram) = arch_with_image(true, 0, vec![0x12, 0x34, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa]);
    let addr = Address::new(Rc::clone(&ram), 0);
    assert_eq!(arch.get_load_image_value(&addr, 2).unwrap(), 0x1234);
}

#[test]
fn w10_loadimage_value_out_of_image_propagates_dataunavail() {
    let (arch, ram) = arch_with_image(false, 0x1000, vec![0u8; 8]);
    let addr = Address::new(Rc::clone(&ram), 0x2000); // outside the window
    assert!(arch.get_load_image_value(&addr, 4).is_err());
}

#[test]
fn w10_no_loader_is_a_caught_error_not_a_panic() {
    // A hand-built fixture without a shared loader must surface a LowlevelError,
    // never panic (the recovery treats it as a failed emulation path).
    let (manage, ram) = manager(false);
    let arch = ArchContext::new_shared(manage);
    let addr = Address::new(Rc::clone(&ram), 0);
    assert!(arch.get_load_image_value(&addr, 4).is_err());
}
