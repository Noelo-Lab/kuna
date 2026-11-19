//! (kuna `dexmodel`) The DEX `LoadImage`: the whole file, read-only, at address 0.
//!
//! Ghidra's `DexLoader` maps the file as one `.dex` block at offset 0 and
//! copies each method's instructions to a second block; kuna decodes the
//! instructions where they are, so the one mapping is the whole image. The
//! method table and constant pool live in [`kuna_decomp::kuna_dex::DexModel`].

use std::cell::Cell;
use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_decomp::kuna_dex::DexModel;
use kuna_sleigh::loadimage::{section_flags, LoadImage, LoadImageSection};

/// The language a DEX file decodes with unless the caller names another.
pub const DEX_LANGUAGE: &str = "Dalvik:LE:32:DEX_Android12";

pub use kuna_decomp::kuna_dex::format::is_dex;

/// A DEX file mapped at address 0 of the attached space.
pub struct DexLoadImage {
    filename: String,
    model: Rc<DexModel>,
    space: Option<Rc<AddrSpace>>,
    section_cursor: Cell<bool>,
}

impl DexLoadImage {
    pub fn new(filename: &str, model: Rc<DexModel>) -> DexLoadImage {
        DexLoadImage { filename: filename.to_string(), model, space: None, section_cursor: Cell::new(false) }
    }

    /// The default code space the image maps into.
    pub fn attach_to_space(&mut self, space: Rc<AddrSpace>) {
        self.space = Some(space);
    }

    fn len(&self) -> u64 {
        self.model.file.bytes().len() as u64
    }
}

impl LoadImage for DexLoadImage {
    fn get_file_name(&self) -> &str {
        &self.filename
    }

    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        let bytes = self.model.file.bytes();
        let start = addr.get_offset();
        let attached = match (addr.get_space(), self.space.as_ref()) {
            (Some(space), Some(own)) => Rc::ptr_eq(space, own),
            _ => false,
        };
        if !attached || start >= bytes.len() as u64 {
            return Err(KunaError::data_unavail(format!(
                "Unable to load {} bytes at {}:0x{start:x}",
                ptr.len(),
                addr.get_shortcut()
            )));
        }
        let start = start as usize;
        let count = ptr.len().min(bytes.len() - start);
        ptr[..count].copy_from_slice(&bytes[start..start + count]);
        ptr[count..].fill(0);
        Ok(())
    }

    fn open_section_info(&self) {
        self.section_cursor.set(true);
    }

    fn get_next_section(&self, record: &mut LoadImageSection) -> bool {
        let (Some(space), true) = (self.space.as_ref(), self.section_cursor.replace(false)) else {
            return false;
        };
        record.address = Address::new(Rc::clone(space), 0);
        record.size = self.len();
        record.flags = section_flags::CODE | section_flags::READONLY;
        false
    }

    fn get_segments(&self) -> Vec<(u64, u64, u32)> {
        vec![(0, self.len(), section_flags::CODE | section_flags::READONLY)]
    }

    fn get_readonly(&self, list: &mut RangeList) {
        if let Some(space) = self.space.as_ref() {
            list.insert_range(Rc::clone(space), 0, self.len().saturating_sub(1));
        }
    }

    fn get_arch_type(&self) -> Vec<u8> {
        DEX_LANGUAGE.as_bytes().to_vec()
    }

    fn adjust_vma(&mut self, _adjust: i64) {}
}
