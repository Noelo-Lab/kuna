//! The load image of a managed assembly: the mapped bytes of the raw image
//! under the section table the metadata implies.
//!
//! The raw image is one buffer, which its loader reports as one code section.
//! In a managed PE the code is the method bodies; the PE headers, the CLI
//! header, the metadata and the resources around them are not, and a surface
//! that sweeps code sections would list them as instructions.

use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

use kuna_base::address::{Address, RangeList};
use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpace;
use kuna_sleigh::loadimage::{ImageBytes, LoadImage, LoadImageFunc, LoadImageSection};

/// A load image that answers every read as `inner` does and reports
/// `sections` in place of its section table.
pub struct CilLoadImage {
    inner: Box<dyn LoadImage>,
    space: Rc<AddrSpace>,
    /// `(address, size, flags)`, ascending and disjoint.
    sections: Vec<(u64, u64, u32)>,
    next: Cell<usize>,
}

impl CilLoadImage {
    pub fn new(inner: Box<dyn LoadImage>, space: Rc<AddrSpace>, sections: Vec<(u64, u64, u32)>) -> CilLoadImage {
        CilLoadImage { inner, space, sections, next: Cell::new(0) }
    }
}

impl LoadImage for CilLoadImage {
    fn arm_vfp_args(&self) -> bool {
        self.inner.arm_vfp_args()
    }

    fn float_arg_registers(&self) -> Option<bool> {
        self.inner.float_arg_registers()
    }

    fn float_hardware(&self) -> Option<bool> {
        self.inner.float_hardware()
    }

    fn callee_extends_returns(&self) -> Option<bool> {
        self.inner.callee_extends_returns()
    }

    fn get_file_name(&self) -> &str {
        self.inner.get_file_name()
    }

    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        self.inner.load_fill(ptr, addr)
    }

    fn read_window(&self) -> Option<u64> {
        self.inner.read_window()
    }

    fn restore_read_window(&self, offset: u64) {
        self.inner.restore_read_window(offset);
    }

    fn open_symbols(&self) {
        self.inner.open_symbols();
    }

    fn close_symbols(&self) {
        self.inner.close_symbols();
    }

    fn get_next_symbol(&self, record: &mut LoadImageFunc) -> bool {
        self.inner.get_next_symbol(record)
    }

    fn open_section_info(&self) {
        self.next.set(0);
    }

    fn get_next_section(&self, record: &mut LoadImageSection) -> bool {
        let at = self.next.get();
        let Some(&(address, size, flags)) = self.sections.get(at) else { return false };
        self.next.set(at + 1);
        record.address = Address::new(Rc::clone(&self.space), address);
        record.size = size;
        record.flags = flags;
        at + 1 < self.sections.len()
    }

    fn kuna_overlay_bytes(&mut self, addr: &Address, data: &[u8]) -> KunaResult<()> {
        self.inner.kuna_overlay_bytes(addr, data)
    }

    fn get_segments(&self) -> Vec<(u64, u64, u32)> {
        self.inner.get_segments()
    }

    fn inaccessible_segments(&self) -> Vec<(u64, u64)> {
        self.inner.inaccessible_segments()
    }

    fn shared_bytes(&self) -> Option<Arc<dyn ImageBytes>> {
        self.inner.shared_bytes()
    }

    fn get_readonly(&self, list: &mut RangeList) {
        self.inner.get_readonly(list);
    }

    fn get_arch_type(&self) -> Vec<u8> {
        self.inner.get_arch_type()
    }

    fn adjust_vma(&mut self, adjust: i64) {
        self.inner.adjust_vma(adjust);
    }
}
