//! (kuna) The staging-window half of a [`LoadImage`], written once and shared:
//! the 512-byte read window ([`windowed_load_fill`]) and a [`LoadImage`] over
//! bytes someone else owns ([`SharedBytesImage`]).
//!
//! `ObjectLoadImage` (kuna-analysis) and `SharedBytesImage` serve the same bytes
//! through the same window logic, so there is no second copy of the read
//! semantics to drift. What each keeps of its own is what cannot be shared: the
//! window itself (a `RefCell` cursor) and the `Rc<AddrSpace>` the image checks
//! incoming addresses against -- space identity is `Rc::ptr_eq`, so a reader on
//! another thread must build its `Address`es in, and check them against, its own
//! engine's space.

use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::sync::Arc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_base::types::Wrap;

use crate::loadimage::{ImageBytes, LoadImage, IMAGE_WINDOW_BYTES};

/// (kuna) Refill the staging window of [`windowed_load_fill`] at `offset`, as
/// a read there left it, or empty it for `!0` or an unmapped `offset`.
pub fn restore_window(
    bytes: &dyn ImageBytes,
    buffer: &RefCell<Vec<u8>>,
    bufoffset: &RefCell<u64>,
    offset: u64,
) {
    if *bufoffset.borrow() == offset {
        return;
    }
    let filled = offset != !0u64 && bytes.fill_span(&mut buffer.borrow_mut()[..], offset) == 0;
    *bufoffset.borrow_mut() = if filled { offset } else { !0u64 };
}

/// Serve `ptr` from `bytes` through a [`IMAGE_WINDOW_BYTES`]-byte staging
/// window (the C++ `LoadImageBfd::loadFill` read path).
///
/// `buffer`/`bufoffset` are the caller's own window: the bytes last staged and
/// the address they start at, `!0` meaning "nothing buffered". A request longer
/// than the window is filled straight into `ptr` and leaves the window alone --
/// a span that long can never be answered out of it, and the window a
/// neighbouring small read is being served from stays valid.
///
/// The caller has already checked that `addr` is in its own space.
pub fn windowed_load_fill(
    bytes: &dyn ImageBytes,
    buffer: &RefCell<Vec<u8>>,
    bufoffset: &RefCell<u64>,
    ptr: &mut [u8],
    addr: &Address,
) -> KunaResult<()> {
    let curaddr0: u64 = addr.get_offset();

    // (kuna) A request larger than the staging buffer cannot be served out
    // of it. Upstream copies the answer back with `memcpy(ptr,buffer,size)`
    // and so reads past the end of a 512-byte buffer for any such request --
    // a silent heap over-read that only upstream's own <= 16-byte callers
    // keep out of reach. kuna reads whole objects (a typed global's
    // datatype size), which routinely exceed 512 bytes, and the same copy
    // spelled as a Rust slice panics instead.
    if ptr.len() > IMAGE_WINDOW_BYTES {
        let remaining = bytes.fill_span(ptr, curaddr0);
        if remaining > 0 {
            let mut errmsg =
                format!("Unable to load {} bytes at {}", remaining, addr.get_shortcut());
            addr.print_raw(&mut errmsg)?;
            return Err(KunaError::data_unavail(errmsg));
        }
        return Ok(());
    }

    let mut bufoffset = bufoffset.borrow_mut();
    let mut buffer = buffer.borrow_mut();

    // (kuna) Upstream tests `curaddr0 + size < bufoffset + BUFSIZE`, which
    // wraps for a request at the top of the address space (a sign-extended
    // negative pointer) and indexes far outside the window. Compare the
    // in-window offset instead.
    let off = curaddr0.wrapping_sub(*bufoffset);
    if curaddr0 >= *bufoffset
        && off < IMAGE_WINDOW_BYTES as u64
        && (ptr.len() as u64) < IMAGE_WINDOW_BYTES as u64 - off
    {
        let start = (curaddr0 - *bufoffset) as usize; // cast: in-buffer offset
        ptr.copy_from_slice(&buffer[start..start + ptr.len()]);
        return Ok(());
    }

    // Load the buffer with bytes from the new address.
    *bufoffset = curaddr0;
    let cursize = bytes.fill_span(&mut buffer[..], curaddr0);
    if cursize > 0 {
        // (offset==0 break path) Unable to load N bytes at <addr>.
        //
        // (kuna) Restore the "nothing buffered" sentinel first.  `bufoffset`
        // was claimed at the top of the fill, before any byte was read, so
        // leaving it set on the failure path makes the fast path above hand
        // out the 512-byte window starting at an address that is NOT MAPPED --
        // stale bytes, reported as a successful read, for every request
        // within the window of the one that just failed.
        *bufoffset = !0u64;
        let mut errmsg = format!("Unable to load {} bytes at {}", cursize, addr.get_shortcut());
        addr.print_raw(&mut errmsg)?;
        return Err(KunaError::data_unavail(errmsg));
    }
    // Copy the requested bytes out.
    ptr.copy_from_slice(&buffer[..ptr.len()]);
    Ok(())
}

/// (kuna) A reader's "a fetch started on an unmapped byte" flag.
///
/// [`ImageBytes::fill_span`] reports failure only when the very FIRST byte of a
/// span is unmapped, and that is the one case in which the staging window in
/// front of the bytes can change an answer: a window that happens to cover the
/// address serves zeroes where a fresh fill reports an error. A reader that has
/// checked [`ImageBytes::mapped_covers`] over everything it will visit can never
/// reach it, so a set flag means the check was wrong and whatever the reader
/// produced must be thrown away.
pub type UnmappedTripwire = Rc<Cell<bool>>;

/// \brief (kuna) A [`LoadImage`] over bytes another image owns.
///
/// Everything that cannot be shared is its own: the staging window and the
/// space handle. Everything that can is behind the `Arc`. Built for a decoder
/// that must read the *live, patched* bytes the owning image holds -- dynamic
/// relocations resolved, `--assert bytes` overlays applied -- rather than a
/// second parse of the file, which would answer differently.
#[derive(Debug)]
pub struct SharedBytesImage {
    /// The mapped bytes, owned elsewhere.
    bytes: Arc<dyn ImageBytes>,
    /// Address space these bytes map to (`None` until [`Self::attach_to_space`]).
    spaceid: Option<Rc<AddrSpace>>,
    /// This reader's own staging window.
    buffer: RefCell<Vec<u8>>,
    /// Starting offset of the buffered bytes (`!0` = "nothing buffered").
    bufoffset: RefCell<u64>,
    /// Name of the loadimage (the `LoadImage` base-class `filename` member).
    filename: String,
    /// Set the first time a fill starts on an unmapped byte.
    unmapped: UnmappedTripwire,
}

impl SharedBytesImage {
    /// A reader over `bytes`, not yet attached to a space.
    pub fn new(filename: &str, bytes: Arc<dyn ImageBytes>) -> SharedBytesImage {
        SharedBytesImage {
            bytes,
            spaceid: None,
            buffer: RefCell::new(vec![0u8; IMAGE_WINDOW_BYTES]),
            bufoffset: RefCell::new(!0u64),
            filename: filename.to_string(),
            unmapped: Rc::new(Cell::new(false)),
        }
    }

    /// A handle on this reader's [`UnmappedTripwire`], readable after the image
    /// has been moved into an engine.
    pub fn tripwire(&self) -> UnmappedTripwire {
        Rc::clone(&self.unmapped)
    }

    /// Attach to the space whose addresses these bytes answer (C++
    /// `LoadImage::attachToSpace`).
    pub fn attach_to_space(&mut self, id: Rc<AddrSpace>) {
        self.spaceid = Some(id);
    }
}

impl LoadImage for SharedBytesImage {
    fn get_file_name(&self) -> &str {
        &self.filename
    }

    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        let space = addr
            .get_space()
            .expect("SharedBytesImage::loadFill: address with null space (C++ UB)");
        match &self.spaceid {
            Some(sp) if Rc::ptr_eq(sp, space) => {}
            _ => {
                return Err(KunaError::data_unavail(format!(
                    "Trying to get loadimage bytes from space: {}",
                    space.get_name()
                )));
            }
        }
        let filled = windowed_load_fill(&*self.bytes, &self.buffer, &self.bufoffset, ptr, addr);
        if filled.is_err() {
            self.unmapped.set(true);
        }
        filled
    }

    fn get_arch_type(&self) -> Vec<u8> {
        // The owning image resolved the language; a reader over its bytes is
        // built against an already-decoded engine and is never asked.
        Vec::new()
    }

    fn adjust_vma(&mut self, _adjust: i64) {
        // The shared bytes are a mirror of an image whose vmas are already
        // final: the owner adjusts at load time, long before anything reads
        // through this view, and a second adjustment here would desynchronize
        // the two. Deliberately a no-op.
    }

    fn shared_bytes(&self) -> Option<Arc<dyn ImageBytes>> {
        Some(Arc::clone(&self.bytes))
    }

    fn read_window(&self) -> Option<u64> {
        Some(*self.bufoffset.borrow())
    }

    fn restore_read_window(&self, offset: u64) {
        restore_window(&*self.bytes, &self.buffer, &self.bufoffset, offset);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::space::{addrspace_flags, spacetype};

    #[derive(Debug)]
    struct AllMapped;

    impl ImageBytes for AllMapped {
        fn fill_span(&self, dst: &mut [u8], start: u64) -> usize {
            for (i, b) in dst.iter_mut().enumerate() {
                *b = start.wrapping_add(i as u64) as u8;
            }
            0
        }
        fn mapped_covers(&self, _lo: u64, _hi: u64) -> bool { true }
    }

    #[test]
    fn read_at_top_of_address_space_does_not_wrap_into_window() {
        let ram = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR, "ram", false, 8, 1, 1, addrspace_flags::hasphysical, 1, 1,
        ));
        let buffer = RefCell::new(vec![0u8; IMAGE_WINDOW_BYTES]);
        let bufoffset = RefCell::new(!0u64);
        let mut small = [0u8; 4];
        windowed_load_fill(&AllMapped, &buffer, &bufoffset, &mut small, &Address::new(Rc::clone(&ram), 0x1000)).unwrap();
        assert_eq!(*bufoffset.borrow(), 0x1000);

        let top = 0xffff_ffff_ffff_fffeu64;
        windowed_load_fill(&AllMapped, &buffer, &bufoffset, &mut small, &Address::new(Rc::clone(&ram), top)).unwrap();
        assert_eq!(small, [0xfe, 0xff, 0x00, 0x01]);
    }
}
