//! Port of `decompiler/cpp/memstate.hh` + `memstate.cc` (W2, item
//! `w2-sleigh-emulate`): classes for keeping track of memory state during
//! emulation.
//!
//! Paradigm mapping (each noted again at its use site):
//!
//! - The C++ abstract base `MemoryBank` splits into [`MemoryBankCore`] (the
//!   concrete data members: wordsize / pagesize / space) and the
//!   [`MemoryBank`] trait — the virtuals `insert`/`find`/`get_page`/
//!   `set_page` (the latter two with the C++ default bodies as provided
//!   methods) plus the non-virtual public interface (`set_value`,
//!   `get_value`, `set_chunk`, `get_chunk`) as provided methods — following
//!   the `TranslateBase`/`Translate` boundary precedent in `translate.rs`.
//! - C++ `MemoryBank *` handles (the `underlie` links, the banks registered
//!   in a `MemoryState`, the emulator's state) are
//!   `Rc<RefCell<dyn MemoryBank>>`: the C++ pointers are shared and
//!   non-owning ("The MemoryState object does \e not assume responsibility
//!   for freeing the MemoryBank") and both reads and writes flow through
//!   them.  Likewise `MemoryImage`'s `LoadImage *` is
//!   `Rc<RefCell<dyn LoadImage>>` (`LoadImage::load_fill` takes `&mut self`).
//! - **`HOST_ENDIAN` is pinned to 0 (little-endian)**: the C++
//!   host-endian-dependent reinterpretations (`(uint1 *)&curval`,
//!   `*((const uintb *)val)`) are transcribed against the little-endian
//!   oracle host that produces the golden output, so the port computes
//!   identically on every host.
//! - `MemoryPageOverlay`'s `map<uintb,uint1 *>` of heap pages becomes a
//!   `BTreeMap<u64, Vec<u8>>` (ADR 0002); the explicit destructor is `Drop`.
//! - Errors (ADR 0004): the C++ throws (`LowlevelError` on read-only
//!   writes, a full hash table, unmapped spaces) become
//!   `Result<_, KunaError>` with the same explain strings; `MemoryImage`
//!   catches `DataUnavailError` exactly where C++ does (treating unmapped
//!   pages as zero) and propagates every other error.
//! - C++ `MemoryState` overloads become suffixed names: `set_value` /
//!   `get_value` (space + offset + size), `set_value_by_name` /
//!   `get_value_by_name` (named register via `Translate`), and
//!   `set_value_data` / `get_value_data` (a `VarnodeData`, matching the
//!   `get_register_data` naming in `translate.rs`).
//!
//! Two upstream anomalies are transcribed rather than repaired (the C++ is
//! the spec), with the port's behavior pinned at the use sites:
//!
//! 1. The default `getPage`/`setPage` adjust the first partial word against
//!    `addr` (the page start) instead of `addr + skip`; for a `skip` that is
//!    not a multiple of the wordsize (reachable through `get_chunk` /
//!    `set_chunk` on a bank using the default page methods, e.g.
//!    `MemoryHashOverlay`) the C++ reads the wrong bytes and overruns the
//!    caller's buffer.  The port transcribes the arithmetic exactly; the
//!    C++ buffer overrun becomes a slice-bounds panic (ADR 0004: UB state).
//! 2. The full-word path of the default `setPage` reads
//!    `sizeof(uintb)` = 8 bytes through `*((const uintb *)val)` regardless
//!    of the wordsize; for wordsize < 8 the high bytes are an overread of
//!    the caller's buffer.  The port reads the bytes that exist and
//!    zero-fills the rest (the garbage is unobservable except through a
//!    `MemoryHashOverlay` storing the raw word, where C++ exposes
//!    uninitialized memory).

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::{byte_swap, calc_mask, Address};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::Wrap;
use kuna_num::pcoderaw::VarnodeData;

use crate::loadimage::LoadImage;
use crate::translate::Translate;

// ---------------------------------------------------------------------------
// constructValue / deconstructValue (C++ statics on MemoryBank)
// ---------------------------------------------------------------------------

/// Decode bytes to value (C++ static `MemoryBank::constructValue`).
///
/// This is a static convenience routine for decoding a value from a sequence
/// of bytes depending on the desired endianness
/// \param ptr is the pointer to the bytes to decode
/// \param size is the number of bytes
/// \param bigendian is \b true if the bytes are encoded in big endian form
/// \return the decoded value
pub fn construct_value(ptr: &[u8], size: i32, bigendian: bool) -> u64 {
    let mut res: u64 = 0;

    if bigendian {
        let mut i: i32 = 0;
        while i < size {
            res = res.wshl(8); // uintb shift wraps for size > 8 inputs
            res = res.wadd(u64::from(ptr[i as usize])); // cast: int4 loop index
            i += 1;
        }
    } else {
        let mut i: i32 = size - 1;
        while i >= 0 {
            res = res.wshl(8);
            res = res.wadd(u64::from(ptr[i as usize])); // cast: int4 loop index
            i -= 1;
        }
    }
    res
}

/// Encode value to bytes (C++ static `MemoryBank::deconstructValue`).
///
/// This is a static convenience routine for encoding bytes from a given
/// value, depending on the desired endianness
/// \param ptr is a pointer to the location to write the encoded bytes
/// \param val is the value to be encoded
/// \param size is the number of bytes to encode
/// \param bigendian is \b true if a big endian encoding is desired
pub fn deconstruct_value(ptr: &mut [u8], val: u64, size: i32, bigendian: bool) {
    let mut val = val;
    if bigendian {
        let mut i: i32 = size - 1;
        while i >= 0 {
            ptr[i as usize] = (val & 0xff) as u8; // cast: (uint1)(val & 0xff)
            val >>= 8;
            i -= 1;
        }
    } else {
        let mut i: i32 = 0;
        while i < size {
            ptr[i as usize] = (val & 0xff) as u8; // cast: (uint1)(val & 0xff)
            val >>= 8;
            i += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// MemoryBank
// ---------------------------------------------------------------------------

/// The concrete data members of the C++ abstract class `MemoryBank`
/// (`wordsize`, `pagesize`, `space`).  A bank implementation embeds one and
/// exposes it through [`MemoryBank::bank_core`] (the `TranslateBase`
/// precedent).
#[derive(Debug, Clone)]
pub struct MemoryBankCore {
    /// Number of bytes in an aligned word access
    wordsize: i32,
    /// Number of bytes in an aligned page access
    pagesize: i32,
    /// The address space associated with this memory
    space: Rc<AddrSpace>,
}

impl MemoryBankCore {
    /// Generic constructor for a memory bank (C++ `MemoryBank::MemoryBank`).
    ///
    /// A MemoryBank must be associated with a specific address space, have a
    /// preferred or natural \e wordsize and a natural \e pagesize.  Both the
    /// \e wordsize and \e pagesize must be a power of 2.
    /// \param spc is the associated address space
    /// \param ws is the number of bytes in the preferred wordsize
    /// \param ps is the number of bytes in a page
    pub fn new(spc: Rc<AddrSpace>, ws: i32, ps: i32) -> MemoryBankCore {
        MemoryBankCore { space: spc, wordsize: ws, pagesize: ps }
    }

    /// Get the number of bytes in a word for this memory bank
    pub fn get_word_size(&self) -> i32 {
        self.wordsize
    }

    /// Get the number of bytes in a page for this memory bank
    pub fn get_page_size(&self) -> i32 {
        self.pagesize
    }

    /// Get the address space associated with this memory bank
    pub fn get_space(&self) -> &Rc<AddrSpace> {
        &self.space
    }
}

/// \brief Memory storage/state for a single AddressSpace
///
/// Class for setting and getting memory values within a space
/// The basic API is to get/set arrays of byte values via offset within the
/// space.  Helper functions get_value and set_value easily retrieve/store
/// integers of various sizes from memory, using the endianness encoding
/// specified by the space.  Accesses through the public interface are
/// automatically broken down into \b word accesses, through the
/// insert/find methods, and \b page accesses through get_page/set_page.  So
/// these are the virtual methods that need to be overridden in
/// implementations.
///
/// (C++ `insert`/`find`/`getPage`/`setPage` are protected; Rust traits have
/// no protected methods, so they are public here.)
pub trait MemoryBank {
    /// Access the concrete C++ base-class members (kuna boundary; see
    /// [`MemoryBankCore`]).
    fn bank_core(&self) -> &MemoryBankCore;

    /// Insert a word in memory bank at an aligned location
    fn insert(&mut self, addr: u64, val: u64) -> KunaResult<()>;

    /// Retrieve a word from memory bank at an aligned location
    fn find(&self, addr: u64) -> KunaResult<u64>;

    /// Retrieve data from a memory \e page
    ///
    /// This routine only retrieves data from a single \e page in the memory
    /// bank. Bytes need not be retrieved from the exact start of a page, but
    /// all bytes must come from \e one page.  A page is a fixed number of
    /// bytes, and the address of a page is always aligned based on that
    /// number of bytes.  This routine may be overridden for a page based
    /// implementation of the MemoryBank.  The default implementation
    /// retrieves the page as aligned words using the find method.
    /// \param addr is the \e aligned offset of the desired page
    /// \param res is where fetched data is written (its length is the C++
    ///        `size` parameter)
    /// \param skip is the offset \e into \e the \e page to get the bytes from
    fn get_page(&self, addr: u64, res: &mut [u8], skip: i32) -> KunaResult<()> {
        // Default implementation just iterates using find
        // but could be optimized
        let wordsize = self.bank_core().wordsize;
        let size = res.len() as i32; // C++ int4 size parameter
        let wordmask = (wordsize - 1) as i64 as u64; // cast: (uintb)(wordsize-1) sign-extends
        let ptraddr = addr.wadd(skip as i64 as u64); // cast: int4 skip converts to uintb
        let endaddr = ptraddr.wadd(size as i64 as u64); // cast: int4 size converts to uintb
        let mut startalign = ptraddr & !wordmask;
        let mut endalign = endaddr & !wordmask;
        if (endaddr & wordmask) != 0 {
            endalign = endalign.wadd(wordsize as i64 as u64); // cast: int4 -> uintb
        }

        // HOST_ENDIAN pinned to 0 (little-endian oracle host; module docs)
        let bswap = self.bank_core().space.is_big_endian();
        let mut respos = 0usize;
        loop {
            let mut curval = self.find(startalign)?;
            if bswap {
                curval = byte_swap(curval, wordsize);
            }
            // (uint1 *)&curval on the little-endian oracle host
            let host = curval.to_le_bytes();
            let mut ptr_off = 0usize;
            let mut sz = wordsize;
            // Upstream anomaly (1) in the module docs: the comparison uses
            // `addr` (the page start), not `ptraddr`; transcribed exactly.
            if startalign < addr {
                ptr_off = (addr.wsub(startalign)) as usize; // cast: < wordsize when reached
                sz = wordsize - (addr.wsub(startalign)) as i32; // cast: int4 = int4 - uintb (small)
            }
            if startalign.wadd(wordsize as i64 as u64) > endaddr {
                sz -= (startalign.wadd(wordsize as i64 as u64).wsub(endaddr)) as i32; // cast: small
            }
            // memcpy(res,ptr,sz): a C++ overrun of `res` panics here (ADR 0004)
            res[respos..respos + sz as usize]
                .copy_from_slice(&host[ptr_off..ptr_off + sz as usize]);
            respos += sz as usize; // cast: sz >= 0 in all defined C++ executions
            startalign = startalign.wadd(wordsize as i64 as u64);
            if startalign == endalign {
                break;
            }
        }
        Ok(())
    }

    /// Write data into a memory page
    ///
    /// This routine writes data only to a single \e page of the memory bank.
    /// Bytes need not be written to the exact start of the page, but all
    /// bytes must be written to only one page when using this routine.  This
    /// routine may be overridden for a page based implementation of the
    /// MemoryBank. The default implementation writes the page as a sequence
    /// of aligned words, using the insert method.
    /// \param addr is the \e aligned offset of the desired page
    /// \param val are the bytes to be written into the page (its length is
    ///        the C++ `size` parameter)
    /// \param skip is the offset \e into \e the \e page where bytes will be
    ///        written
    fn set_page(&mut self, addr: u64, val: &[u8], skip: i32) -> KunaResult<()> {
        // Default implementation just iterates using insert
        // but could be optimized
        let wordsize = self.bank_core().wordsize;
        let size = val.len() as i32; // C++ int4 size parameter
        let wordmask = (wordsize - 1) as i64 as u64; // cast: (uintb)(wordsize-1) sign-extends
        let ptraddr = addr.wadd(skip as i64 as u64); // cast: int4 skip converts to uintb
        let endaddr = ptraddr.wadd(size as i64 as u64); // cast: int4 size converts to uintb
        let mut startalign = ptraddr & !wordmask;
        let mut endalign = endaddr & !wordmask;
        if (endaddr & wordmask) != 0 {
            endalign = endalign.wadd(wordsize as i64 as u64); // cast: int4 -> uintb
        }

        // HOST_ENDIAN pinned to 0 (little-endian oracle host; module docs)
        let bswap = self.bank_core().space.is_big_endian();
        let mut valpos = 0usize;
        loop {
            let mut ptr_off = 0usize;
            let mut sz = wordsize;
            // Upstream anomaly (1) in the module docs: compares against
            // `addr`, not `ptraddr`; transcribed exactly.
            if startalign < addr {
                ptr_off = (addr.wsub(startalign)) as usize; // cast: < wordsize when reached
                sz = wordsize - (addr.wsub(startalign)) as i32; // cast: int4 = int4 - uintb (small)
            }
            if startalign.wadd(wordsize as i64 as u64) > endaddr {
                sz -= (startalign.wadd(wordsize as i64 as u64).wsub(endaddr)) as i32; // cast: small
            }
            let mut curval: u64;
            if sz != wordsize {
                curval = self.find(startalign)?; // Part of word is copied from underlying
                // memcpy(ptr,val,sz) into the host(LE)-order bytes of curval;
                // the rest is taken from -val-
                let mut host = curval.to_le_bytes();
                host[ptr_off..ptr_off + sz as usize]
                    .copy_from_slice(&val[valpos..valpos + sz as usize]);
                curval = u64::from_le_bytes(host);
            } else {
                // -val- supplies entire word: C++ `*((const uintb *)val)`
                // reads 8 host bytes regardless of wordsize (upstream
                // anomaly (2) in the module docs: bytes past the slice are
                // a C++ overread; the port zero-fills them)
                let mut host = [0u8; 8];
                let avail = std::cmp::min(8, val.len() - valpos);
                host[..avail].copy_from_slice(&val[valpos..valpos + avail]);
                curval = u64::from_le_bytes(host);
            }
            if bswap {
                curval = byte_swap(curval, wordsize);
            }
            self.insert(startalign, curval)?;
            valpos += sz as usize; // cast: sz >= 0 in all defined C++ executions
            startalign = startalign.wadd(wordsize as i64 as u64);
            if startalign == endalign {
                break;
            }
        }
        Ok(())
    }

    // --- C++ non-virtual public interface, provided over bank_core() ---

    /// Get the number of bytes in a word for this memory bank.
    ///
    /// A MemoryBank is instantiated with a \e natural word size. Requests
    /// for arbitrary byte ranges may be broken down into units of this size.
    fn get_word_size(&self) -> i32 {
        self.bank_core().wordsize
    }

    /// Get the number of bytes in a page for this memory bank.
    ///
    /// A MemoryBank is instantiated with a \e natural page size. Requests
    /// for large chunks of data may be broken down into units of this size.
    fn get_page_size(&self) -> i32 {
        self.bank_core().pagesize
    }

    /// Get the address space associated with this memory bank.
    ///
    /// A MemoryBank is a contiguous sequence of bytes associated with a
    /// particular address space.
    fn get_space(&self) -> &Rc<AddrSpace> {
        &self.bank_core().space
    }

    /// Set the value of a (small) range of bytes
    ///
    /// This routine is used to set a single value in the memory bank at an
    /// arbitrary address.  It takes into account the endianness of the
    /// associated address space when encoding the value as bytes in the
    /// bank.  The value is broken up into aligned pieces of \e wordsize and
    /// the actual \b write is performed with the insert routine.  If only
    /// parts of aligned words are written to, then the remaining parts are
    /// filled in with the original value, via the find routine.
    /// \param offset is the start of the byte range to write
    /// \param size is the number of bytes in the range to write
    /// \param val is the value to be written
    fn set_value(&mut self, offset: u64, size: i32, val: u64) -> KunaResult<()> {
        let wordsize = self.bank_core().wordsize;
        let alignmask = (wordsize - 1) as i64 as u64; // cast: (uintb)(wordsize-1)
        let ind = offset & !alignmask;
        let mut skip = (offset & alignmask) as i32; // cast: int4 = uintb (value < wordsize)
        let mut size1 = wordsize - skip;
        let size2: i32;
        let mut gap: i32;
        let mut val1: u64;
        let mut val2: u64;

        if size > size1 {
            // We have spill over
            size2 = size - size1;
            val1 = self.find(ind)?;
            val2 = self.find(ind.wadd(wordsize as i64 as u64))?; // cast: int4 -> uintb
            gap = wordsize - size2;
        } else {
            if size == wordsize {
                return self.insert(ind, val);
            }
            val1 = self.find(ind)?;
            val2 = 0;
            gap = size1 - size;
            size1 = size;
            size2 = 0;
        }

        skip *= 8; // Convert from byte skip to bit skip
        gap *= 8; // Convert from byte to bits
        // The 8*size1 / 8*size2 shifts below can reach 64 only for
        // out-of-contract sizes; wshl/wshr reproduce the x86 oracle's
        // count-masking for that (C++ UB) case.
        if self.bank_core().space.is_big_endian() {
            if size2 == 0 {
                val1 &= !(calc_mask(size1) << gap); // gap < 64 by construction
                val1 |= val << gap;
                self.insert(ind, val1)?;
            } else {
                val1 &= u64::MAX.wshl((8 * size1) as u32); // cast: shift count
                val1 |= val.wshr((8 * size2) as u32); // cast: shift count
                self.insert(ind, val1)?;
                val2 &= u64::MAX.wshr((8 * size2) as u32); // cast: shift count
                val2 |= val << gap;
                self.insert(ind.wadd(wordsize as i64 as u64), val2)?; // cast: int4 -> uintb
            }
        } else if size2 == 0 {
            val1 &= !(calc_mask(size1) << skip); // skip < 64 by construction
            val1 |= val << skip;
            self.insert(ind, val1)?;
        } else {
            val1 &= u64::MAX.wshr((8 * size1) as u32); // cast: shift count
            val1 |= val << skip;
            self.insert(ind, val1)?;
            val2 &= u64::MAX.wshl((8 * size2) as u32); // cast: shift count
            val2 |= val.wshr((8 * size1) as u32); // cast: shift count
            self.insert(ind.wadd(wordsize as i64 as u64), val2)?; // cast: int4 -> uintb
        }
        Ok(())
    }

    /// Retrieve the value encoded in a (small) range of bytes
    ///
    /// This routine gets the value from a range of bytes at an arbitrary
    /// address.  It takes into account the endianness of the underlying
    /// space when decoding the value.  The value is constructed by making
    /// one or more aligned word queries, using the find method.  The desired
    /// value may span multiple words and is reconstructed properly.
    /// \param offset is the start of the byte range encoding the value
    /// \param size is the number of bytes in the range
    /// \return the decoded value
    fn get_value(&self, offset: u64, size: i32) -> KunaResult<u64> {
        let res: u64;

        let wordsize = self.bank_core().wordsize;
        let alignmask = (wordsize - 1) as i64 as u64; // cast: (uintb)(wordsize-1)
        let ind = offset & !alignmask;
        let skip = (offset & alignmask) as i32; // cast: int4 = uintb (value < wordsize)
        let mut size1 = wordsize - skip;
        let size2: i32;
        let gap: i32;
        let val1: u64;
        let val2: u64;
        if size > size1 {
            // We have spill over
            size2 = size - size1;
            val1 = self.find(ind)?;
            val2 = self.find(ind.wadd(wordsize as i64 as u64))?; // cast: int4 -> uintb
            gap = wordsize - size2;
        } else {
            val1 = self.find(ind)?;
            val2 = 0;
            if size == wordsize {
                return Ok(val1);
            }
            gap = size1 - size;
            size1 = size;
            size2 = 0;
        }

        // (the spill-over shifts can reach 64 only out of contract; wshl/wshr
        // reproduce the x86 oracle's count masking, as in set_value)
        if self.bank_core().space.is_big_endian() {
            if size2 == 0 {
                res = val1 >> (8 * gap); // gap < wordsize bytes by construction
            } else {
                res = val1.wshl((8 * size2) as u32) | (val2 >> (8 * gap)); // cast: shift count
            }
        } else if size2 == 0 {
            res = val1 >> (skip * 8); // skip < wordsize bytes by construction
        } else {
            res = (val1 >> (skip * 8)) | val2.wshl((size1 * 8) as u32); // cast: shift count
        }
        Ok(res & calc_mask(size))
    }

    /// Set values of an arbitrary sequence of bytes
    ///
    /// This is the most general method for writing a sequence of bytes into
    /// the memory bank.  There is no restriction on the offset to write to
    /// or the number of bytes to be written, except that the range must be
    /// contained in the address space.
    /// \param offset is the start of the byte range to be written
    /// \param val is the sequence of bytes to be written into the bank (its
    ///        length is the C++ `size` parameter)
    fn set_chunk(&mut self, offset: u64, val: &[u8]) -> KunaResult<()> {
        let size = val.len() as i32; // C++ int4 size parameter
        let pagesize = self.bank_core().pagesize;
        let mut cursize: i32;
        let mut count: i32 = 0;
        let pagemask = (pagesize - 1) as i64 as u64; // cast: (uintb)(pagesize-1)
        let mut offalign: u64;
        let mut skip: i32;
        let mut offset = offset;
        let mut valpos = 0usize;

        while count < size {
            cursize = pagesize;
            offalign = offset & !pagemask;
            skip = 0;
            if offalign != offset {
                skip = (offset.wsub(offalign)) as i32; // cast: int4 = uintb (value < pagesize)
                cursize -= skip;
            }
            if size - count < cursize {
                cursize = size - count;
            }
            self.set_page(offalign, &val[valpos..valpos + cursize as usize], skip)?;
            count += cursize;
            offset = offset.wadd(cursize as i64 as u64); // cast: int4 -> uintb
            valpos += cursize as usize; // cast: cursize > 0 here
        }
        Ok(())
    }

    /// Retrieve an arbitrary sequence of bytes
    ///
    /// This is the most general method for reading a sequence of bytes from
    /// the memory bank.  There is no restriction on the offset or the number
    /// of bytes to read, except that the range must be contained in the
    /// address space.
    /// \param offset is the start of the byte range to read
    /// \param res is where the retrieved bytes are stored (its length is the
    ///        C++ `size` parameter)
    fn get_chunk(&self, offset: u64, res: &mut [u8]) -> KunaResult<()> {
        let size = res.len() as i32; // C++ int4 size parameter
        let pagesize = self.bank_core().pagesize;
        let mut cursize: i32;
        let mut count: i32 = 0;
        let pagemask = (pagesize - 1) as i64 as u64; // cast: (uintb)(pagesize-1)
        let mut offalign: u64;
        let mut skip: i32;
        let mut offset = offset;
        let mut respos = 0usize;

        while count < size {
            cursize = pagesize;
            offalign = offset & !pagemask;
            skip = 0;
            if offalign != offset {
                skip = (offset.wsub(offalign)) as i32; // cast: int4 = uintb (value < pagesize)
                cursize -= skip;
            }
            if size - count < cursize {
                cursize = size - count;
            }
            self.get_page(offalign, &mut res[respos..respos + cursize as usize], skip)?;
            count += cursize;
            offset = offset.wadd(cursize as i64 as u64); // cast: int4 -> uintb
            respos += cursize as usize; // cast: cursize > 0 here
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// MemoryImage
// ---------------------------------------------------------------------------

/// \brief A kind of MemoryBank which retrieves its data from an underlying
/// LoadImage
///
/// Any bytes requested on the bank which lie in the LoadImage are retrieved
/// from the LoadImage.  Other addresses in the space are filled in with
/// zero.  This bank cannot be written to.
pub struct MemoryImage {
    core: MemoryBankCore,
    /// The underlying LoadImage
    loader: Rc<RefCell<dyn LoadImage>>,
}

impl MemoryImage {
    /// Constructor for a loadimage memorybank.
    ///
    /// A MemoryImage needs everything a basic memory bank needs and it needs
    /// to know the underlying LoadImage object to forward read requests to.
    /// \param spc is the address space associated with the memory bank
    /// \param ws is the number of bytes in the preferred wordsize (must be
    ///        power of 2)
    /// \param ps is the number of bytes in a page (must be power of 2)
    /// \param ld is the underlying LoadImage
    pub fn new(spc: Rc<AddrSpace>, ws: i32, ps: i32, ld: Rc<RefCell<dyn LoadImage>>) -> Self {
        MemoryImage { core: MemoryBankCore::new(spc, ws, ps), loader: ld }
    }
}

impl MemoryBank for MemoryImage {
    fn bank_core(&self) -> &MemoryBankCore {
        &self.core
    }

    /// Exception is thrown for write attempts
    fn insert(&mut self, _addr: u64, _val: u64) -> KunaResult<()> {
        Err(KunaError::lowlevel("Writing to read-only MemoryBank"))
    }

    /// Overridden find method.
    ///
    /// Find an aligned word from the bank.  First an attempt is made to
    /// fetch the data from the LoadImage.  If this fails, the value is
    /// returned as 0.
    /// \param addr is the address of the word to fetch
    /// \return the fetched value
    fn find(&self, addr: u64) -> KunaResult<u64> {
        // Assume that -addr- is word aligned
        // Make sure all bytes start as 0, as load may not fill all bytes
        let wordsize = self.core.wordsize;
        let spc = &self.core.space;
        // C++ writes through (uint1 *)&res (the low wordsize bytes on the
        // little-endian oracle host; HOST_ENDIAN pinned, module docs)
        let mut buf = [0u8; 8];
        match self
            .loader
            .borrow_mut()
            .load_fill(&mut buf[..wordsize as usize], &Address::new(Rc::clone(spc), addr))
        {
            Ok(()) => {}
            Err(KunaError::DataUnavail { .. }) => {
                // Pages not mapped in the load image, are assumed to be zero
                buf = [0u8; 8];
            }
            Err(e) => return Err(e),
        }
        let mut res = u64::from_le_bytes(buf);
        // (HOST_ENDIAN==1) != spc->isBigEndian(), with HOST_ENDIAN pinned 0
        if spc.is_big_endian() {
            res = byte_swap(res, wordsize);
        }
        Ok(res)
    }

    /// Overridden getPage method.
    ///
    /// Retrieve an aligned page from the bank.  First an attempt is made to
    /// retrieve the page from the LoadImage, which may do its own zero
    /// filling.  If the attempt fails, the page is entirely filled in with
    /// zeros.
    fn get_page(&self, addr: u64, res: &mut [u8], skip: i32) -> KunaResult<()> {
        // Assume that -addr- is page aligned
        let spc = &self.core.space;
        match self.loader.borrow_mut().load_fill(
            res,
            &Address::new(Rc::clone(spc), addr.wadd(skip as i64 as u64)), // cast: int4 -> uintb
        ) {
            Ok(()) => Ok(()),
            Err(KunaError::DataUnavail { .. }) => {
                // Pages not mapped in the load image, are assumed to be zero
                res.fill(0);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

// ---------------------------------------------------------------------------
// MemoryPageOverlay
// ---------------------------------------------------------------------------

/// \brief Memory bank that overlays some other memory bank, using a "copy on
/// write" behavior.
///
/// Pages are copied from the underlying object only when there is a write.
/// The underlying access routines are overridden to make optimal use of this
/// page implementation.  The underlying memory bank can be `None` (the C++
/// null pointer) in which case, this memory bank behaves as if it were
/// initially filled with zeros.
pub struct MemoryPageOverlay {
    core: MemoryBankCore,
    /// Underlying memory object
    underlie: Option<Rc<RefCell<dyn MemoryBank>>>,
    /// Overlayed pages (C++ `map<uintb,uint1 *>`; ADR 0002 BTreeMap, pages
    /// owned by value so the C++ destructor's `delete []` loop is `Drop`)
    page: BTreeMap<u64, Vec<u8>>,
}

impl MemoryPageOverlay {
    /// Constructor for page overlay.
    ///
    /// A page overlay memory bank needs all the parameters for a generic
    /// memory bank and it needs to know the underlying memory bank being
    /// overlayed.
    /// \param spc is the address space associated with the memory bank
    /// \param ws is the number of bytes in the preferred wordsize (must be
    ///        power of 2)
    /// \param ps is the number of bytes in a page (must be power of 2)
    /// \param ul is the underlying MemoryBank (or `None`)
    pub fn new(
        spc: Rc<AddrSpace>,
        ws: i32,
        ps: i32,
        ul: Option<Rc<RefCell<dyn MemoryBank>>>,
    ) -> Self {
        MemoryPageOverlay {
            core: MemoryBankCore::new(spc, ws, ps),
            underlie: ul,
            page: BTreeMap::new(),
        }
    }
}

impl MemoryBank for MemoryPageOverlay {
    fn bank_core(&self) -> &MemoryBankCore {
        &self.core
    }

    /// Overridden aligned word insert.
    ///
    /// This derived method looks for a previously cached page of the
    /// underlying memory bank.  If the cached page does not exist, it
    /// creates it and fills in its initial value by retrieving the page from
    /// the underlying bank.  The new value is then written into the cached
    /// page.
    /// \param addr is the aligned address of the word to be written
    /// \param val is the value to be written at that word
    fn insert(&mut self, addr: u64, val: u64) -> KunaResult<()> {
        let pagesize = self.core.pagesize;
        let pageaddr = addr & !((pagesize - 1) as i64 as u64); // cast: (uintb)(getPageSize()-1)

        if !self.page.contains_key(&pageaddr) {
            // pageptr = new uint1[getPageSize()] (zero-filled here; the C++
            // explicitly zeroes the no-underlie case and overwrites the rest)
            let mut pageptr = vec![0u8; pagesize as usize];
            if let Some(underlie) = &self.underlie {
                underlie.borrow().get_page(pageaddr, &mut pageptr, 0)?;
            }
            self.page.insert(pageaddr, pageptr);
        }

        let pageoffset = (addr & ((pagesize - 1) as i64 as u64)) as usize; // cast: < pagesize
        let wordsize = self.core.wordsize;
        let bigendian = self.core.space.is_big_endian();
        let pageptr = self.page.get_mut(&pageaddr).expect("page just ensured present");
        deconstruct_value(
            &mut pageptr[pageoffset..pageoffset + wordsize as usize],
            val,
            wordsize,
            bigendian,
        );
        Ok(())
    }

    /// Overridden aligned word find.
    ///
    /// This derived method first looks for the aligned word in the mapped
    /// pages. If the address is not mapped, the search is forwarded to the
    /// \e underlying memory bank.  If there is no underlying bank, zero is
    /// returned.
    /// \param addr is the aligned offset of the word
    /// \return the retrieved value
    fn find(&self, addr: u64) -> KunaResult<u64> {
        let pagesize = self.core.pagesize;
        let pageaddr = addr & !((pagesize - 1) as i64 as u64); // cast: (uintb)(getPageSize()-1)

        match self.page.get(&pageaddr) {
            None => match &self.underlie {
                None => Ok(0),
                Some(underlie) => underlie.borrow().find(addr),
            },
            Some(pageptr) => {
                let pageoffset = (addr & ((pagesize - 1) as i64 as u64)) as usize; // cast: < pagesize
                let wordsize = self.core.wordsize;
                Ok(construct_value(
                    &pageptr[pageoffset..pageoffset + wordsize as usize],
                    wordsize,
                    self.core.space.is_big_endian(),
                ))
            }
        }
    }

    /// Overridden getPage.
    ///
    /// The desired page is looked for in the page cache.  If it doesn't
    /// exist, the request is forwarded to the \e underlying bank.  If there
    /// is no underlying bank, the result buffer is filled with zeros.
    /// \param addr is the aligned offset of the page
    /// \param res is where retrieved bytes are stored
    /// \param skip is the offset \e into \e the \e page from where bytes
    ///        should be retrieved
    fn get_page(&self, addr: u64, res: &mut [u8], skip: i32) -> KunaResult<()> {
        match self.page.get(&addr) {
            None => match &self.underlie {
                None => {
                    res.fill(0);
                    Ok(())
                }
                Some(underlie) => underlie.borrow().get_page(addr, res, skip),
            },
            Some(pageptr) => {
                let size = res.len();
                res.copy_from_slice(&pageptr[skip as usize..skip as usize + size]); // cast: int4 skip
                Ok(())
            }
        }
    }

    /// Overridden setPage.
    ///
    /// First, a cached version of the desired page is searched for via its
    /// address. If it doesn't exist, it is created, and its initial value is
    /// filled via the \e underlying bank. The bytes to be written are then
    /// copied into the cached page.
    /// \param addr is the aligned offset of the page to write
    /// \param val are the bytes to be written into the page
    /// \param skip is the offset \e into \e the \e page where bytes should
    ///        be written
    fn set_page(&mut self, addr: u64, val: &[u8], skip: i32) -> KunaResult<()> {
        let pagesize = self.core.pagesize;
        let size = val.len() as i32; // C++ int4 size parameter

        if !self.page.contains_key(&addr) {
            // (C++ leaves a full-page write's new page uninitialized; it is
            // entirely overwritten below.  Zero-filled here.)
            let mut pageptr = vec![0u8; pagesize as usize];
            if size != pagesize {
                if let Some(underlie) = &self.underlie {
                    underlie.borrow().get_page(addr, &mut pageptr, 0)?;
                }
            }
            self.page.insert(addr, pageptr);
        }

        let pageptr = self.page.get_mut(&addr).expect("page just ensured present");
        pageptr[skip as usize..skip as usize + val.len()].copy_from_slice(val); // cast: int4 skip
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// MemoryHashOverlay
// ---------------------------------------------------------------------------

/// \brief A memory bank that implements reads and writes using a hash table.
///
/// The initial state of the bank is taken from an \e underlying memory bank
/// or is all zero, if this bank is initialized with `None`.  This
/// implementation will not be very efficient for accessing entire pages.
pub struct MemoryHashOverlay {
    core: MemoryBankCore,
    /// Underlying memory bank
    underlie: Option<Rc<RefCell<dyn MemoryBank>>>,
    /// How many LSBs are thrown away from address when doing hash table
    /// lookup
    alignshift: i32,
    /// How many slots to skip after a hashtable collision
    collideskip: u64,
    /// The hashtable addresses
    address: Vec<u64>,
    /// The hashtable values
    value: Vec<u64>,
}

impl MemoryHashOverlay {
    /// Constructor for hash overlay.
    ///
    /// A MemoryBank implemented as a hash table needs everything associated
    /// with a generic memory bank, but the constructor also needs to know
    /// the size of the hashtable and the underlying memorybank to forward
    /// reads and writes to.
    /// \param spc is the address space associated with the memory bank
    /// \param ws is the number of bytes in the preferred wordsize (must be
    ///        power of 2)
    /// \param ps is the number of bytes in a page (must be a power of 2)
    /// \param hashsize is the maximum number of entries in the hashtable
    /// \param ul is the underlying memory bank being overlayed (or `None`)
    pub fn new(
        spc: Rc<AddrSpace>,
        ws: i32,
        ps: i32,
        hashsize: i32,
        ul: Option<Rc<RefCell<dyn MemoryBank>>>,
    ) -> Self {
        // vector<uintb> address(hashsize,0xBADBEEF), value(hashsize,0)
        let address = vec![0xBADBEEFu64; hashsize as usize]; // cast: int4 hashsize as length
        let value = vec![0u64; hashsize as usize]; // cast: int4 hashsize as length

        let mut tmp = (ws - 1) as u32; // cast: C++ uint4 tmp = ws - 1
        let mut alignshift: i32 = 0;
        while tmp != 0 {
            alignshift += 1;
            tmp >>= 1;
        }
        MemoryHashOverlay {
            core: MemoryBankCore::new(spc, ws, ps),
            underlie: ul,
            alignshift,
            collideskip: 1023,
            address,
            value,
        }
    }
}

impl MemoryBank for MemoryHashOverlay {
    fn bank_core(&self) -> &MemoryBankCore {
        &self.core
    }

    /// Overridden aligned word insert.
    ///
    /// Write the value into the hashtable, using \b addr as a key.
    /// **This is the C++ hash, transcribed exactly** (open addressing keyed
    /// on `(addr >> alignshift) % size`, a fixed `collideskip` probe stride
    /// of 1023, and the `0xBADBEEF` empty-slot marker).
    /// \param addr is the aligned address of the word being written
    /// \param val is the value of the word to write
    fn insert(&mut self, addr: u64, val: u64) -> KunaResult<()> {
        let size = self.address.len() as i32; // C++ int4 size = address.size()
        // C++ `(addr>>alignshift) % size`: the int4 size converts to uintb
        let mut offset = (addr >> self.alignshift) % (size as i64 as u64); // cast: int4 -> uintb
        let mut i: i32 = 0;
        while i < size {
            if self.address[offset as usize] == addr {
                // Address has been seen before
                self.value[offset as usize] = val; // Replace old value
                return Ok(());
            } else if self.address[offset as usize] == 0xBADBEEF {
                // Address not seen before
                self.address[offset as usize] = addr; // Claim this hash slot
                self.value[offset as usize] = val; // Set value
                return Ok(());
            }
            offset = offset.wadd(self.collideskip) % (size as i64 as u64); // cast: int4 -> uintb
            i += 1;
        }
        Err(KunaError::lowlevel("Memory state hash_table is full"))
    }

    /// Overridden aligned word find.
    ///
    /// First search for an entry in the hashtable using \b addr as a key.
    /// If there is no entry, forward the query to the underlying memory
    /// bank, or return 0 if there is no underlying bank
    /// \param addr is the aligned address of the word to retrieve
    /// \return the retrieved value
    fn find(&self, addr: u64) -> KunaResult<u64> {
        // Find address in hash-table, or return find from underlying memory
        let size = self.address.len() as i32; // C++ int4 size = address.size()
        let mut offset = (addr >> self.alignshift) % (size as i64 as u64); // cast: int4 -> uintb
        let mut i: i32 = 0;
        while i < size {
            if self.address[offset as usize] == addr {
                // Address has been seen before
                return Ok(self.value[offset as usize]);
            } else if self.address[offset as usize] == 0xBADBEEF {
                // Address not seen before
                break;
            }
            offset = offset.wadd(self.collideskip) % (size as i64 as u64); // cast: int4 -> uintb
            i += 1;
        }

        // We didn't find the address in the hashtable
        match &self.underlie {
            None => Ok(0),
            Some(underlie) => underlie.borrow().find(addr),
        }
    }
}

// ---------------------------------------------------------------------------
// MemoryState
// ---------------------------------------------------------------------------

/// \brief All storage/state for a pcode machine
///
/// Every piece of information in a pcode machine is representable as a
/// triple (AddrSpace, offset, size).  This class allows getting and setting
/// of all state information of this form.
pub struct MemoryState {
    /// Architecture information about memory spaces (the C++ `Translate *`)
    trans: Rc<dyn Translate>,
    /// Memory banks associated with each address space (indexed by the
    /// space's index, `None` for unregistered slots — the C++ null pointers)
    memspace: Vec<Option<Rc<RefCell<dyn MemoryBank>>>>,
}

impl MemoryState {
    /// A constructor for MemoryState.
    ///
    /// The MemoryState needs a Translate object in order to be able to
    /// convert register names into varnodes
    /// \param t is the translator
    pub fn new(t: Rc<dyn Translate>) -> Self {
        MemoryState { trans: t, memspace: Vec::new() }
    }

    /// Get the Translate object.
    ///
    /// Retrieve the actual pcode translator being used by this machine state
    pub fn get_translate(&self) -> &Rc<dyn Translate> {
        &self.trans
    }

    /// Map a memory bank into the state.
    ///
    /// MemoryBanks associated with specific address spaces must be
    /// registered with this MemoryState via this method.  Each address space
    /// that will be used during emulation must be registered separately.
    /// The MemoryState object does \e not assume responsibility for freeing
    /// the MemoryBank (the `Rc` is shared with the caller).
    /// \param bank is the MemoryBank to be registered
    pub fn set_memory_bank(&mut self, bank: Rc<RefCell<dyn MemoryBank>>) {
        let index = bank.borrow().get_space().get_index();

        while index as usize >= self.memspace.len() {
            // cast: C++ int4 index >= vector::size() comparison
            self.memspace.push(None);
        }

        self.memspace[index as usize] = Some(bank); // cast: index >= 0
    }

    /// Get a memory bank associated with a particular space.
    ///
    /// Any MemoryBank that has been registered with this MemoryState can be
    /// retrieved via this method if the MemoryBank's associated address
    /// space is known.
    /// \param spc is the address space of the desired MemoryBank
    /// \return the MemoryBank or `None` if no bank is associated with \e spc
    pub fn get_memory_bank(&self, spc: &Rc<AddrSpace>) -> Option<&Rc<RefCell<dyn MemoryBank>>> {
        let index = spc.get_index();
        if index as usize >= self.memspace.len() {
            // cast: C++ int4 index >= vector::size() comparison
            return None;
        }
        self.memspace[index as usize].as_ref() // cast: index >= 0
    }

    /// Set a value on the memory state.
    ///
    /// This is the main interface for writing values to the MemoryState.
    /// If there is no registered MemoryBank for the desired address space,
    /// or if there is some other error, an error is returned (C++ throws).
    /// \param spc is the address space to write to
    /// \param off is the offset where the value should be written
    /// \param size is the number of bytes to be written
    /// \param cval is the value to be written
    pub fn set_value(
        &mut self,
        spc: &Rc<AddrSpace>,
        off: u64,
        size: i32,
        cval: u64,
    ) -> KunaResult<()> {
        match self.get_memory_bank(spc) {
            None => Err(KunaError::lowlevel(format!(
                "Setting value for unmapped memory space: {}",
                spc.get_name()
            ))),
            Some(mspace) => mspace.borrow_mut().set_value(off, size, cval),
        }
    }

    /// Retrieve a memory value from the memory state.
    ///
    /// This is the main interface for reading values from the MemoryState.
    /// If there is no registered MemoryBank for the desired address space,
    /// or if there is some other error, an error is returned (C++ throws).
    /// \param spc is the address space being queried
    /// \param off is the offset of the value being queried
    /// \param size is the number of bytes to query
    /// \return the queried value
    pub fn get_value(&self, spc: &Rc<AddrSpace>, off: u64, size: i32) -> KunaResult<u64> {
        if spc.get_type() == spacetype::IPTR_CONSTANT {
            return Ok(off);
        }
        match self.get_memory_bank(spc) {
            None => Err(KunaError::lowlevel(format!(
                "Getting value from unmapped memory space: {}",
                spc.get_name()
            ))),
            Some(mspace) => mspace.borrow().get_value(off, size),
        }
    }

    /// Set a value on a named register in the memory state (C++
    /// `setValue(const string &,uintb)`).
    ///
    /// This is a convenience method for setting registers by name.  Any
    /// register name known to the Translate object can be used as a write
    /// location.  The associated address space, offset, and size is looked
    /// up and automatically passed to the main set_value routine.
    /// \param nm is the name of the register
    /// \param cval is the value to write to the register
    pub fn set_value_by_name(&mut self, nm: &str, cval: u64) -> KunaResult<()> {
        // Set a "register" value
        let vdata = self.trans.get_register_data(nm)?;
        let spc = vdata
            .space
            .as_ref()
            .expect("MemoryState: register with null space pointer (C++ UB)");
        let spc = Rc::clone(spc);
        self.set_value(&spc, vdata.offset, vdata.size as i32, cval) // cast: uint4 size as C++ int4
    }

    /// Retrieve a value from a named register in the memory state (C++
    /// `getValue(const string &)`).
    ///
    /// This is a convenience method for reading registers by name.  Any
    /// register name known to the Translate object can be used as a read
    /// location.  The associated address space, offset, and size is looked
    /// up and automatically passed to the main get_value routine.
    /// \param nm is the name of the register
    /// \return the value associated with that register
    pub fn get_value_by_name(&self, nm: &str) -> KunaResult<u64> {
        // Get a "register" value
        let vdata = self.trans.get_register_data(nm)?;
        let spc = vdata
            .space
            .as_ref()
            .expect("MemoryState: register with null space pointer (C++ UB)");
        self.get_value(spc, vdata.offset, vdata.size as i32) // cast: uint4 size as C++ int4
    }

    /// Set value on a given \b varnode (C++ `setValue(const VarnodeData *,
    /// uintb)`).
    ///
    /// A convenience method for setting a value directly on a varnode rather
    /// than breaking out the components
    /// \param vn is the varnode to be written
    /// \param cval is the value to write into the varnode
    pub fn set_value_data(&mut self, vn: &VarnodeData, cval: u64) -> KunaResult<()> {
        let spc = vn
            .space
            .as_ref()
            .expect("MemoryState: varnode with null space pointer (C++ UB)");
        let spc = Rc::clone(spc);
        self.set_value(&spc, vn.offset, vn.size as i32, cval) // cast: uint4 size as C++ int4
    }

    /// Get a value from a \b varnode (C++ `getValue(const VarnodeData *)`).
    ///
    /// A convenience method for reading a value directly from a varnode
    /// rather than querying for the offset and space
    /// \param vn is the varnode to be read
    /// \return the value read from the varnode
    pub fn get_value_data(&self, vn: &VarnodeData) -> KunaResult<u64> {
        let spc = vn
            .space
            .as_ref()
            .expect("MemoryState: varnode with null space pointer (C++ UB)");
        self.get_value(spc, vn.offset, vn.size as i32) // cast: uint4 size as C++ int4
    }

    /// Get a chunk of data from memory state.
    ///
    /// This is the main interface for reading a range of bytes from the
    /// MemoryState.  The MemoryBank associated with the address space of the
    /// query is looked up and the request is forwarded to the get_chunk
    /// method on the MemoryBank. If there is no registered MemoryBank or
    /// some other error, an error is returned (C++ throws).
    /// \param res is the result buffer for storing retrieved bytes (its
    ///        length is the C++ `size` parameter)
    /// \param spc is the desired address space
    /// \param off is the starting offset of the byte range being queried
    pub fn get_chunk(&self, res: &mut [u8], spc: &Rc<AddrSpace>, off: u64) -> KunaResult<()> {
        match self.get_memory_bank(spc) {
            None => Err(KunaError::lowlevel(format!(
                "Getting chunk from unmapped memory space: {}",
                spc.get_name()
            ))),
            Some(mspace) => mspace.borrow().get_chunk(off, res),
        }
    }

    /// Set a chunk of data from memory state.
    ///
    /// This is the main interface for setting values for a range of bytes in
    /// the MemoryState.  The MemoryBank associated with the desired address
    /// space is looked up and the write is forwarded to the set_chunk method
    /// on the MemoryBank. If there is no registered MemoryBank or some other
    /// error, an error is returned (C++ throws).
    /// \param val are the byte values to be written into the MemoryState
    ///        (its length is the C++ `size` parameter)
    /// \param spc is the address space being written
    /// \param off is the starting offset of the range being written
    pub fn set_chunk(&mut self, val: &[u8], spc: &Rc<AddrSpace>, off: u64) -> KunaResult<()> {
        match self.get_memory_bank(spc) {
            None => Err(KunaError::lowlevel(format!(
                "Setting chunk of unmapped memory space: {}",
                spc.get_name()
            ))),
            Some(mspace) => mspace.borrow_mut().set_chunk(off, val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emulate::test_support::{test_manager, TestLoader, TestTranslator};
    use kuna_base::space::AddrSpaceManager;

    /// A loader with 16 mapped bytes at 0x2000:
    /// `11 22 33 44 55 66 77 88 a1 a2 a3 a4 a5 a6 a7 a8`.
    fn test_loader() -> Rc<RefCell<TestLoader>> {
        let mut bytes = vec![0x11u8, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88];
        bytes.extend_from_slice(&[0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8]);
        Rc::new(RefCell::new(TestLoader { start: 0x2000, bytes }))
    }

    fn ram(manager: &AddrSpaceManager) -> Rc<AddrSpace> {
        Rc::clone(manager.get_space_by_name("ram").unwrap())
    }

    #[test]
    fn test_memstate_construct_deconstruct_value() {
        let bytes = [0x12u8, 0x34, 0x56, 0x78];
        assert_eq!(construct_value(&bytes, 4, true), 0x12345678);
        assert_eq!(construct_value(&bytes, 4, false), 0x78563412);
        assert_eq!(construct_value(&bytes, 1, true), 0x12);
        assert_eq!(construct_value(&bytes, 1, false), 0x12);

        let mut out = [0u8; 4];
        deconstruct_value(&mut out, 0x12345678, 4, true);
        assert_eq!(out, [0x12, 0x34, 0x56, 0x78]);
        deconstruct_value(&mut out, 0x12345678, 4, false);
        assert_eq!(out, [0x78, 0x56, 0x34, 0x12]);
        // round trip an 8-byte value both ways
        let mut out = [0u8; 8];
        deconstruct_value(&mut out, 0x1122334455667788, 8, true);
        assert_eq!(construct_value(&out, 8, true), 0x1122334455667788);
        deconstruct_value(&mut out, 0x1122334455667788, 8, false);
        assert_eq!(construct_value(&out, 8, false), 0x1122334455667788);
    }

    #[test]
    fn test_memstate_memory_image() {
        let manager = test_manager(false);
        let image = MemoryImage::new(ram(&manager), 8, 16, test_loader());
        assert_eq!(image.get_word_size(), 8);
        assert_eq!(image.get_page_size(), 16);
        assert!(Rc::ptr_eq(image.get_space(), &ram(&manager)));

        // aligned word reads (little endian)
        assert_eq!(image.find(0x2000).unwrap(), 0x8877665544332211);
        assert_eq!(image.find(0x2008).unwrap(), 0xa8a7a6a5a4a3a2a1);
        // value within a word, and a value spanning two words
        assert_eq!(image.get_value(0x2002, 2).unwrap(), 0x4433);
        assert_eq!(image.get_value(0x2004, 8).unwrap(), 0xa4a3a2a188776655);
        // unmapped data is zero (DataUnavailError caught inside find)
        assert_eq!(image.find(0x4000).unwrap(), 0);
        assert_eq!(image.get_value(0x4000, 8).unwrap(), 0);
        // the bank is read-only
        let mut image = image;
        assert_eq!(
            image.set_value(0x2000, 8, 1).unwrap_err().explain(),
            "Writing to read-only MemoryBank"
        );

        // big endian variant
        let manager_be = test_manager(true);
        let image_be = MemoryImage::new(ram(&manager_be), 8, 16, test_loader());
        assert_eq!(image_be.find(0x2000).unwrap(), 0x1122334455667788);
        assert_eq!(image_be.get_value(0x2002, 2).unwrap(), 0x3344);

        // chunk read crossing the image start: the page below the region is
        // zero-filled via the DataUnavail path of the overridden get_page
        let image = MemoryImage::new(ram(&manager), 8, 16, test_loader());
        let mut res = [0xffu8; 16];
        image.get_chunk(0x1ff8, &mut res).unwrap();
        assert_eq!(&res[..8], &[0u8; 8]);
        assert_eq!(&res[8..], &[0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);
    }

    #[test]
    fn test_memstate_page_overlay_little_endian() {
        let manager = test_manager(false);
        let r = ram(&manager);
        let image: Rc<RefCell<dyn MemoryBank>> =
            Rc::new(RefCell::new(MemoryImage::new(Rc::clone(&r), 8, 16, test_loader())));
        let mut overlay = MemoryPageOverlay::new(Rc::clone(&r), 8, 16, Some(Rc::clone(&image)));

        // read-through to the underlying image (no page cached)
        assert_eq!(overlay.get_value(0x2000, 8).unwrap(), 0x8877665544332211);

        // copy-on-write: a 2-byte write inside the first word
        overlay.set_value(0x2003, 2, 0xBEEF).unwrap();
        assert_eq!(overlay.get_value(0x2000, 8).unwrap(), 0x887766BEEF332211);
        assert_eq!(overlay.get_value(0x2003, 2).unwrap(), 0xBEEF);
        // the underlying image is untouched
        assert_eq!(image.borrow().find(0x2000).unwrap(), 0x8877665544332211);

        // a write spanning the word boundary inside the cached page
        overlay.set_value(0x2006, 4, 0xAABBCCDD).unwrap();
        assert_eq!(overlay.find(0x2000).unwrap(), 0xCCDD66BEEF332211);
        assert_eq!(overlay.find(0x2008).unwrap(), 0xA8A7A6A5A4A3AABB);
        assert_eq!(overlay.get_value(0x2006, 4).unwrap(), 0xAABBCCDD);

        // word read across PAGES: word 0x2008 comes from the cached page,
        // word 0x2010 from the underlying image (outside the region: zero)
        assert_eq!(overlay.get_value(0x200c, 8).unwrap(), 0xA8A7A6A5);

        // chunk read across pages: cached page bytes then underlying zeros
        let mut res = [0u8; 16];
        overlay.get_chunk(0x2008, &mut res).unwrap();
        assert_eq!(&res[..8], &[0xBB, 0xAA, 0xA3, 0xA4, 0xA5, 0xA6, 0xA7, 0xA8]);
        assert_eq!(&res[8..], &[0u8; 8]);

        // chunk write across pages (the second page is created copy-on-write)
        overlay.set_chunk(0x200e, &[0xDE, 0xAD, 0xBE, 0xEF]).unwrap();
        assert_eq!(overlay.get_value(0x200e, 2).unwrap(), 0xADDE);
        assert_eq!(overlay.get_value(0x2010, 2).unwrap(), 0xEFBE);

        // overlay with no underlying bank starts as zeros
        let mut zeros = MemoryPageOverlay::new(Rc::clone(&r), 8, 16, None);
        assert_eq!(zeros.get_value(0x80, 8).unwrap(), 0);
        zeros.set_value(0x84, 4, 0xCAFEBABE).unwrap();
        assert_eq!(zeros.find(0x80).unwrap(), 0xCAFEBABE00000000);
        let mut res = [0u8; 4];
        zeros.get_chunk(0x84, &mut res).unwrap();
        assert_eq!(res, [0xBE, 0xBA, 0xFE, 0xCA]);
    }

    #[test]
    fn test_memstate_page_overlay_big_endian() {
        let manager = test_manager(true);
        let r = ram(&manager);
        let mut overlay = MemoryPageOverlay::new(Rc::clone(&r), 8, 16, None);

        overlay.set_value(0x0, 8, 0x1122334455667788).unwrap();
        let mut res = [0u8; 8];
        overlay.get_chunk(0x0, &mut res).unwrap();
        // big endian: most significant byte at the lowest address
        assert_eq!(res, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88]);

        // partial-word write lands in address order
        overlay.set_value(0x3, 2, 0xBEEF).unwrap();
        assert_eq!(overlay.find(0x0).unwrap(), 0x112233BEEF667788);
        overlay.get_chunk(0x0, &mut res).unwrap();
        assert_eq!(res, [0x11, 0x22, 0x33, 0xBE, 0xEF, 0x66, 0x77, 0x88]);
        assert_eq!(overlay.get_value(0x3, 2).unwrap(), 0xBEEF);

        // spill over a word boundary, big endian
        overlay.set_value(0x6, 4, 0xAABBCCDD).unwrap();
        assert_eq!(overlay.find(0x0).unwrap(), 0x112233BEEF66AABB);
        assert_eq!(overlay.find(0x8).unwrap(), 0xCCDD000000000000);
        assert_eq!(overlay.get_value(0x6, 4).unwrap(), 0xAABBCCDD);
    }

    #[test]
    fn test_memstate_hash_overlay() {
        let manager = test_manager(false);
        let r = ram(&manager);
        let image: Rc<RefCell<dyn MemoryBank>> =
            Rc::new(RefCell::new(MemoryImage::new(Rc::clone(&r), 8, 16, test_loader())));
        let mut hash = MemoryHashOverlay::new(Rc::clone(&r), 8, 16, 16, Some(Rc::clone(&image)));

        // unwritten addresses fall through to the underlying bank
        assert_eq!(hash.get_value(0x2000, 8).unwrap(), 0x8877665544332211);
        // aligned write and replacement
        hash.set_value(0x100, 8, 42).unwrap();
        assert_eq!(hash.get_value(0x100, 8).unwrap(), 42);
        hash.set_value(0x100, 8, 43).unwrap();
        assert_eq!(hash.get_value(0x100, 8).unwrap(), 43);
        hash.set_value(0x100, 8, 42).unwrap();

        // unaligned write spanning two words (read-modify-write through the
        // hash): word 0x100 keeps 42 in its low half
        hash.set_value(0x104, 8, 0x1122334455667788).unwrap();
        assert_eq!(hash.get_value(0x100, 8).unwrap(), 0x556677880000002A);
        assert_eq!(hash.get_value(0x108, 8).unwrap(), 0x11223344);
        assert_eq!(hash.get_value(0x104, 8).unwrap(), 0x1122334455667788);

        // chunk access goes through the DEFAULT page methods (word loops)
        let mut res = [0u8; 16];
        hash.get_chunk(0x100, &mut res).unwrap();
        assert_eq!(
            res,
            [0x2A, 0, 0, 0, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22, 0x11, 0, 0, 0, 0]
        );
        hash.set_chunk(0x110, &[1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
        assert_eq!(hash.get_value(0x110, 8).unwrap(), 0x0807060504030201);

        // collision probing: hashsize 4, addresses 0x0 and 0x20 both hash
        // to slot 0; the second probes (0 + 1023) % 4 = 3
        let mut small = MemoryHashOverlay::new(Rc::clone(&r), 8, 16, 4, None);
        small.set_value(0x0, 8, 7).unwrap();
        small.set_value(0x20, 8, 9).unwrap();
        assert_eq!(small.get_value(0x0, 8).unwrap(), 7);
        assert_eq!(small.get_value(0x20, 8).unwrap(), 9);

        // table full: hashsize 2 holds two addresses, the third errs
        let mut tiny = MemoryHashOverlay::new(Rc::clone(&r), 8, 16, 2, None);
        tiny.set_value(0x0, 8, 1).unwrap();
        tiny.set_value(0x8, 8, 2).unwrap();
        assert_eq!(
            tiny.set_value(0x10, 8, 3).unwrap_err().explain(),
            "Memory state hash_table is full"
        );
        // unwritten with no underlying bank reads zero
        let empty = MemoryHashOverlay::new(Rc::clone(&r), 8, 16, 4, None);
        assert_eq!(empty.get_value(0x40, 8).unwrap(), 0);
    }

    #[test]
    fn test_memstate_memory_state() {
        let manager = Rc::new(test_manager(false));
        let r = ram(&manager);
        let register = Rc::clone(manager.get_space_by_name("register").unwrap());
        let unique = Rc::clone(manager.get_space_by_name("unique").unwrap());
        let trans: Rc<dyn Translate> =
            Rc::new(TestTranslator::new(&manager, BTreeMap::new(), Vec::new()));

        let image: Rc<RefCell<dyn MemoryBank>> =
            Rc::new(RefCell::new(MemoryImage::new(Rc::clone(&r), 8, 16, test_loader())));
        let ramstate: Rc<RefCell<dyn MemoryBank>> =
            Rc::new(RefCell::new(MemoryPageOverlay::new(Rc::clone(&r), 8, 16, Some(image))));
        let registerstate: Rc<RefCell<dyn MemoryBank>> = Rc::new(RefCell::new(
            MemoryHashOverlay::new(Rc::clone(&register), 8, 16, 64, None),
        ));

        let mut state = MemoryState::new(Rc::clone(&trans));
        assert!(Rc::ptr_eq(state.get_translate(), &trans));
        state.set_memory_bank(Rc::clone(&ramstate));
        state.set_memory_bank(Rc::clone(&registerstate));
        assert!(Rc::ptr_eq(state.get_memory_bank(&r).unwrap(), &ramstate));
        assert!(state.get_memory_bank(&unique).is_none());

        // constant space reads return the offset directly
        let cst = Rc::clone(manager.get_constant_space().unwrap());
        assert_eq!(state.get_value(&cst, 12345, 8).unwrap(), 12345);

        // space/offset and named-register interfaces
        state.set_value(&r, 0x3000, 8, 0x1234).unwrap();
        assert_eq!(state.get_value(&r, 0x3000, 8).unwrap(), 0x1234);
        state.set_value_by_name("r0", 99).unwrap();
        assert_eq!(state.get_value_by_name("r0").unwrap(), 99);
        assert_eq!(state.get_value(&register, 0, 8).unwrap(), 99, "r0 lives at register:0");
        assert!(state.get_value_by_name("nope").is_err());

        // varnode interface
        let vn = VarnodeData { space: Some(Rc::clone(&register)), offset: 8, size: 8 };
        state.set_value_data(&vn, 0xfeed).unwrap();
        assert_eq!(state.get_value_data(&vn).unwrap(), 0xfeed);

        // chunk interface
        state.set_chunk(&[0xDE, 0xAD], &r, 0x3008).unwrap();
        let mut res = [0u8; 2];
        state.get_chunk(&mut res, &r, 0x3008).unwrap();
        assert_eq!(res, [0xDE, 0xAD]);

        // unmapped space errors carry the C++ strings
        assert_eq!(
            state.get_value(&unique, 0, 8).unwrap_err().explain(),
            "Getting value from unmapped memory space: unique"
        );
        assert_eq!(
            state.set_value(&unique, 0, 8, 1).unwrap_err().explain(),
            "Setting value for unmapped memory space: unique"
        );
        let mut res = [0u8; 1];
        assert_eq!(
            state.get_chunk(&mut res, &unique, 0).unwrap_err().explain(),
            "Getting chunk from unmapped memory space: unique"
        );
        assert_eq!(
            state.set_chunk(&[0], &unique, 0).unwrap_err().explain(),
            "Setting chunk of unmapped memory space: unique"
        );
    }
}
