//! Port of `decompiler/cpp/loadimage.hh` + `loadimage.cc` (W2, item
//! `w2-sleigh-loadimage`) — classes and API for accessing a binary load
//! image.
//!
//! API mapping (vs C++):
//!
//! - The abstract class `LoadImage` becomes the [`LoadImage`] trait.  The
//!   protected `filename` member cannot live on a trait, so `getFileName()`
//!   becomes the required method [`LoadImage::get_file_name`] and each
//!   implementor stores its own filename.  The non-virtual convenience
//!   `load()` is a provided trait method.
//! - `loadFill(uint1 *ptr,int4 size,const Address &addr)` becomes
//!   `load_fill(&mut self, ptr: &mut [u8], addr: &Address)`: the
//!   (pointer,size) pair collapses into a byte slice whose length is the
//!   C++ `size` (an `int4`; requests beyond 2^31-1 bytes are unsupported,
//!   as in C++).  The unfilled-read error contract is kept exactly: where
//!   C++ throws `DataUnavailError` the Rust port returns
//!   `Err(KunaError::DataUnavail)` carrying the same explain string
//!   (`kuna_base::error` ports the exception type itself).
//! - The `const`-but-stateful symbol/section iteration methods
//!   (`openSymbols`/`getNextSymbol`/...) stay `&self`; implementors mirror
//!   the C++ `mutable` cursor members with interior mutability.
//! - `getArchType()` returns a byte string (`Vec<u8>`), following the
//!   workspace convention that marshal/XML-derived strings are byte
//!   strings (`LoadImageXml` reads its arch type straight from an XML
//!   attribute).
//! - `adjustVma(long adjust)` becomes `adjust_vma(&mut self, adjust: i64)`.
//!
//! `RawLoadImage` reads bytes directly from a file on disk.  C++ holds an
//! `ifstream *` (null until `open()`); the Rust port holds an
//! `Option<std::fs::File>`.  C++ never checks the stream state after
//! `seekg`/`read` — a failed read silently leaves the destination buffer
//! with its previous contents — so an OS-level I/O failure has no defined
//! oracle behavior; the port surfaces it as `KunaError::Lowlevel`.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_base::types::Wrap;

// The C++ `DataUnavailError` (loadimage.hh) is ported as the
// `KunaError::DataUnavail` variant in kuna-base (`error.rs`), preserving the
// "is a LowlevelError" inheritance for catch-frame purposes.

/// \brief A record indicating a function symbol
///
/// This is a lightweight object holding the Address and name of a function
#[derive(Debug, Clone, Default)]
pub struct LoadImageFunc {
    /// Start of function
    pub address: Address,
    /// Name of function (byte string; symbol names arrive over marshal/XML)
    pub name: Vec<u8>,
}

/// Boolean properties a section might have (C++ anonymous `enum` inside
/// `LoadImageSection`).
pub mod section_flags {
    /// Not allocated in memory (debug info)
    pub const UNALLOC: u32 = 1;
    /// uninitialized section
    pub const NOLOAD: u32 = 2;
    /// code only
    pub const CODE: u32 = 4;
    /// data only
    pub const DATA: u32 = 8;
    /// read only section
    pub const READONLY: u32 = 16;
}

/// \brief A record describing a section bytes in the executable
///
/// A lightweight object specifying the location and size of the section and
/// basic properties
#[derive(Debug, Clone, Default)]
pub struct LoadImageSection {
    /// Starting address of section
    pub address: Address,
    /// Number of bytes in section
    pub size: u64,
    /// Properties of the section ([`section_flags`])
    pub flags: u32,
}

/// \brief An interface into a particular binary executable image
///
/// This class provides the abstraction needed by the decompiler for the
/// numerous load file formats used to encode binary executables.  The data
/// encoding the machine instructions for the executable can be accessed via
/// the addresses where that data would be loaded into RAM.
/// Properties other than the main data and instructions of the binary are
/// not supposed to repeatedly queried through this interface. This
/// information is intended to be read from this class exactly once, during
/// initialization, and used to populate the main decompiler database. This
/// class currently has only rudimentary support for accessing such
/// properties.
pub trait LoadImage {
    /// Get the name of the LoadImage.
    ///
    /// The loadimage is usually associated with a file. This routine
    /// retrieves the name as a string.
    fn get_file_name(&self) -> &str;

    /// Get data from the LoadImage.
    ///
    /// This is the \e core routine of a LoadImage.  Given a particular
    /// address range, this routine retrieves the exact byte values that are
    /// stored at that address when the executable is loaded into RAM.  The
    /// caller must supply a pre-allocated array of bytes where the returned
    /// bytes should be stored.  If the requested address range does not
    /// exist in the image, or otherwise can't be retrieved, this method
    /// returns `Err(KunaError::DataUnavail)` (C++ throws
    /// `DataUnavailError`).
    /// \param ptr points to where the resulting bytes will be stored (its
    ///        length is the C++ `size` parameter)
    /// \param addr is the starting address of the bytes to retrieve
    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()>;

    /// Prepare to read symbols.
    ///
    /// This routine should read in and parse any symbol information that
    /// the load image contains about executable.  Once this method is
    /// called, individual symbol records are read out using the
    /// getNextSymbol() method.
    fn open_symbols(&self) {}

    /// Stop reading symbols.
    ///
    /// Once all the symbol information has been read out from the load
    /// image via the openSymbols() and getNextSymbol() calls, the
    /// application should call this method to free up resources used in
    /// parsing the symbol information.
    fn close_symbols(&self) {}

    /// Get the next symbol record.
    ///
    /// This method is used to read out an individual symbol record,
    /// LoadImageFunc, from the load image.  Right now, the only information
    /// that can be read out are function starts and the associated function
    /// name.  This method can be called repeatedly to iterate through all
    /// the symbols, until it returns \b false.  This indicates the end of
    /// the symbols.
    /// \param record is a reference to the symbol record to be filled in
    /// \return \b true if there are more records to read
    fn get_next_symbol(&self, record: &mut LoadImageFunc) -> bool {
        let _ = record;
        false
    }

    /// Prepare to read section info.
    ///
    /// This method initializes iteration over all the sections of bytes
    /// that are mapped by the load image.  Once this is called, information
    /// on individual sections should be read out with the getNextSection()
    /// method.
    fn open_section_info(&self) {}

    /// Stop reading section info.
    ///
    /// Once all the section information is read from the load image using
    /// the getNextSection() method, this method should be called to free up
    /// any resources used in parsing the section info.
    fn close_section_info(&self) {}

    /// Get info on the next section.
    ///
    /// This method is used to read out a record that describes a single
    /// section of bytes mapped by the load image. This method can be called
    /// repeatedly until it returns \b false, to get info on additional
    /// sections.
    /// \param record is a reference to the info record to be filled in
    /// \return \b true if there are more records to read
    fn get_next_section(&self, record: &mut LoadImageSection) -> bool {
        let _ = record;
        false
    }

    /// (kuna) The mapped **load segments** as `(vma, size, flags)`, `flags` per
    /// [`section_flags`] — the coarser mapping unit underneath the section
    /// table.
    ///
    /// Not a C++ method. BFD builds its `asection` list from an ELF's section
    /// headers, so upstream has no answer at all for an image that carries none,
    /// and every section-keyed reader silently sees an empty world. This reports
    /// what the program headers still say, so a reader can fall back to the
    /// segment that contains an address when no section does. Empty by default:
    /// a loader that does not model segments keeps reporting nothing.
    fn get_segments(&self) -> Vec<(u64, u64, u32)> {
        Vec::new()
    }

    /// Return list of \e readonly address ranges.
    ///
    /// This method should read out information about \e all address ranges
    /// within the load image that are known to be \b readonly.  This method
    /// is intended to be called only once, so all information should be
    /// written to the passed RangeList object.
    /// \param list is where readonly info will get put
    fn get_readonly(&self, list: &mut RangeList) {
        let _ = list;
    }

    /// Get a string indicating the architecture type.
    ///
    /// The load image class is intended to be a generic front-end to the
    /// large variety of load formats in use.  This method should return a
    /// string that identifies the particular architecture this particular
    /// image is intended to run on.  It is currently the responsibility of
    /// any derived LoadImage class to establish a format for this string,
    /// but it should generally contain some indication of the operating
    /// system and the processor.
    fn get_arch_type(&self) -> Vec<u8>;

    /// Adjust load addresses with a global offset.
    ///
    /// Most load image formats automatically encode information about the
    /// true loading address(es) for the data in the image.  But if this is
    /// missing or incorrect, this routine can be used to make a global
    /// adjustment to the load address. Only one adjustment is made across
    /// \e all addresses in the image.  The offset passed to this method is
    /// added to the stored or default value for any address queried in the
    /// image.  This is most often used in a \e raw binary file format.  In
    /// this case, the entire executable file is intended to be read
    /// straight into RAM, as one contiguous chunk, in order to be executed.
    /// In the absence of any other info, the first byte of the image file
    /// is loaded at offset 0. This method then would adjust the load
    /// address of the first byte.
    /// \param adjust is the offset amount to be added to default values
    fn adjust_vma(&mut self, adjust: i64);

    /// Load a chunk of image.
    ///
    /// This is a convenience method wrapped around the core loadFill()
    /// routine.  It automatically allocates an array of the desired size,
    /// and then fills it with load image data.  (C++ returns a raw `new[]`
    /// buffer the caller must free; the Rust port returns a `Vec<u8>`.)
    /// \param size is the number of bytes to read from the image
    /// \param addr is the address of the first byte being read
    /// \return the desired bytes
    fn load(&mut self, size: i32, addr: &Address) -> KunaResult<Vec<u8>> {
        // cast: int4 size as the C++ `new uint1[size]` extent; a negative
        // size is UB in C++ (and aborts allocation here)
        let mut buf = vec![0u8; size as usize];
        self.load_fill(&mut buf, addr)?;
        Ok(buf)
    }
}

/// \brief A simple raw binary loadimage
///
/// This is probably the simplest loadimage.  Bytes from the image are read
/// directly from a file stream.  The address associated with each byte is
/// determined by a single value, the vma, which is the address of the first
/// byte in the file.  No symbols or sections are supported
#[derive(Debug)]
pub struct RawLoadImage {
    /// Name of the loadimage (the `LoadImage` base-class member)
    filename: String,
    /// Address of first byte in the file
    vma: u64,
    /// Main file stream for image (C++ `ifstream *`, null until `open`)
    thefile: Option<File>,
    /// Total number of bytes in the loadimage/file
    filesize: u64,
    /// Address space that the file bytes are mapped to (C++ raw pointer,
    /// null until `attachToSpace`)
    spaceid: Option<Rc<AddrSpace>>,
}

impl RawLoadImage {
    /// RawLoadImage constructor
    pub fn new(f: &str) -> RawLoadImage {
        RawLoadImage {
            filename: f.to_string(),
            vma: 0,
            thefile: None,
            filesize: 0,
            spaceid: None,
        }
    }

    /// Attach the raw image to a particular space
    pub fn attach_to_space(&mut self, id: Rc<AddrSpace>) {
        self.spaceid = Some(id);
    }

    /// Open the raw file for reading.
    ///
    /// The file is opened and its size immediately recovered.
    pub fn open(&mut self) -> KunaResult<()> {
        if self.thefile.is_some() {
            return Err(KunaError::lowlevel("loadimage is already open"));
        }
        // C++ `new ifstream(filename.c_str())`: default (text) mode, which
        // is identical to binary mode on the platforms kuna targets.
        let mut file = match File::open(&self.filename) {
            Ok(f) => f,
            Err(_) => {
                let errmsg = format!("Unable to open raw image file: {}", self.filename);
                return Err(KunaError::lowlevel(errmsg));
            }
        };
        // (C++ does not check the stream state; a seek failure here is an
        // OS-level error with no oracle behavior, reported as the same
        // open failure.)
        match file.seek(SeekFrom::End(0)) {
            Ok(sz) => self.filesize = sz,
            Err(_) => {
                let errmsg = format!("Unable to open raw image file: {}", self.filename);
                return Err(KunaError::lowlevel(errmsg));
            }
        }
        self.thefile = Some(file);
        Ok(())
    }
}

impl LoadImage for RawLoadImage {
    fn get_file_name(&self) -> &str {
        &self.filename
    }

    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        // cast: the C++ `int4 size` parameter (slice length; see trait docs)
        let mut size: i32 = ptr.len() as i32;
        let mut curaddr: u64 = addr.get_offset();
        let mut offset: u64 = 0;
        let mut readsize: u64;

        curaddr = curaddr.wsub(self.vma); // Get relative offset of first byte
        // C++ dereferences the (possibly null) `thefile` pointer — UB when
        // loadFill is called before open() (ADR 0004: panic)
        let file = self
            .thefile
            .as_mut()
            .expect("RawLoadImage::loadFill before open() (C++ null ifstream deref)");
        while size > 0 {
            if curaddr >= self.filesize {
                if offset == 0 {
                    // Initial address not within file
                    break;
                }
                // fill out the rest of the buffer with 0 (offset+size
                // always equals the slice length)
                for b in &mut ptr[offset as usize..] {
                    // cast: offset < slice length here
                    *b = 0;
                }
                return Ok(());
            }
            readsize = size as u64; // cast: int4 -> uintb, size > 0 here
            if curaddr.wadd(readsize) > self.filesize {
                // Adjust to biggest possible read
                readsize = self.filesize.wsub(curaddr);
            }
            // C++ ignores the stream state; an OS-level I/O failure (no
            // oracle behavior) surfaces as a LowlevelError here.
            file.seek(SeekFrom::Start(curaddr)).map_err(|e| {
                KunaError::lowlevel(format!("I/O error reading raw image file: {e}"))
            })?;
            // cast: offset/readsize are within the slice length here
            file.read_exact(&mut ptr[offset as usize..(offset.wadd(readsize)) as usize])
                .map_err(|e| {
                    KunaError::lowlevel(format!("I/O error reading raw image file: {e}"))
                })?;
            offset = offset.wadd(readsize);
            size -= readsize as i32; // cast: readsize <= size (an int4) here
            curaddr = curaddr.wadd(readsize);
        }
        if size > 0 {
            let mut errmsg = format!("Unable to load {} bytes at {}", size, addr.get_shortcut());
            addr.print_raw(&mut errmsg)?;
            return Err(KunaError::data_unavail(errmsg));
        }
        Ok(())
    }

    fn get_arch_type(&self) -> Vec<u8> {
        b"unknown".to_vec()
    }

    fn adjust_vma(&mut self, adjust: i64) {
        // C++ dereferences the (possibly null) `spaceid` pointer — UB when
        // adjustVma is called before attachToSpace (ADR 0004: panic)
        let spaceid = self
            .spaceid
            .as_ref()
            .expect("RawLoadImage::adjustVma before attachToSpace (C++ null space deref)");
        // addressToByte: the `long` argument converts to uintb
        // (sign-extension), the uintb result converts back to long; vma +=
        // adjust then wraps in uintb.
        let adjust = AddrSpace::address_to_byte(adjust as u64, spaceid.get_word_size());
        self.vma = self.vma.wadd(adjust);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::space::{addrspace_flags, spacetype, AddrSpaceManager, ConstantSpace};

    /// const(0) + ram(1), little endian, 4-byte addresses.
    fn manager(wordsize: u32) -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            4,
            wordsize,
            1,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m.set_default_code_space(1).unwrap();
        m
    }

    /// Write a 16-byte raw file 00 01 02 ... 0f and return its path.
    fn write_raw_file(name: &str) -> std::path::PathBuf {
        let mut path = std::env::temp_dir();
        path.push(format!("kuna_rawloadimage_{}_{}", std::process::id(), name));
        let bytes: Vec<u8> = (0u8..16).collect();
        std::fs::write(&path, bytes).unwrap();
        path
    }

    #[test]
    fn raw_load_image_exact_reads_and_zero_fill() {
        let m = manager(1);
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let path = write_raw_file("basic");
        let mut img = RawLoadImage::new(path.to_str().unwrap());
        img.attach_to_space(Rc::clone(&ram));
        img.open().unwrap();
        assert_eq!(img.get_arch_type(), b"unknown".to_vec());
        assert_eq!(img.get_file_name(), path.to_str().unwrap());

        // vma defaults to 0: address == file offset
        let mut buf = [0xaau8; 4];
        img.load_fill(&mut buf, &Address::new(Rc::clone(&ram), 0)).unwrap();
        assert_eq!(buf, [0, 1, 2, 3]);
        img.load_fill(&mut buf, &Address::new(Rc::clone(&ram), 12)).unwrap();
        assert_eq!(buf, [12, 13, 14, 15]);

        // Read straddling EOF: bytes from the file, then zero fill
        let mut buf = [0xaau8; 8];
        img.load_fill(&mut buf, &Address::new(Rc::clone(&ram), 12)).unwrap();
        assert_eq!(buf, [12, 13, 14, 15, 0, 0, 0, 0]);

        // load() convenience wrapper
        let loaded = img.load(3, &Address::new(Rc::clone(&ram), 5)).unwrap();
        assert_eq!(loaded, vec![5, 6, 7]);

        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn raw_load_image_unfilled_error_contract() {
        let m = manager(1);
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let path = write_raw_file("err");
        let mut img = RawLoadImage::new(path.to_str().unwrap());
        img.attach_to_space(Rc::clone(&ram));
        img.open().unwrap();

        // Initial address entirely past EOF: DataUnavailError with the
        // exact C++ message (shortcut 'r' for "ram", 4-byte space printRaw)
        let mut buf = [0u8; 4];
        let err = img.load_fill(&mut buf, &Address::new(Rc::clone(&ram), 0x20)).unwrap_err();
        match &err {
            KunaError::DataUnavail { explain } => {
                assert_eq!(explain, "Unable to load 4 bytes at r0x00000020");
            }
            other => panic!("expected DataUnavail, got {other:?}"),
        }

        // double open is a LowlevelError
        let err = img.open().unwrap_err();
        match &err {
            KunaError::Lowlevel { explain } => assert_eq!(explain, "loadimage is already open"),
            other => panic!("expected Lowlevel, got {other:?}"),
        }

        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn raw_load_image_open_failure_and_vma() {
        let m = manager(1);
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let mut img = RawLoadImage::new("/nonexistent/kuna_raw_image");
        let err = img.open().unwrap_err();
        match &err {
            KunaError::Lowlevel { explain } => {
                assert_eq!(explain, "Unable to open raw image file: /nonexistent/kuna_raw_image");
            }
            other => panic!("expected Lowlevel, got {other:?}"),
        }

        // adjustVma shifts every queried address: with vma = 0x100, file
        // byte 0 lives at address 0x100
        let path = write_raw_file("vma");
        let mut img = RawLoadImage::new(path.to_str().unwrap());
        img.attach_to_space(Rc::clone(&ram));
        img.open().unwrap();
        img.adjust_vma(0x100);
        let mut buf = [0u8; 2];
        img.load_fill(&mut buf, &Address::new(Rc::clone(&ram), 0x100)).unwrap();
        assert_eq!(buf, [0, 1]);
        // Address below the vma wraps negative in the uintb subtraction and
        // lands past filesize with offset==0: unfilled error
        let err = img.load_fill(&mut buf, &Address::new(Rc::clone(&ram), 0x80)).unwrap_err();
        assert!(matches!(err, KunaError::DataUnavail { .. }));

        std::fs::remove_file(&path).unwrap();
    }

    #[test]
    fn raw_load_image_adjust_vma_scales_by_wordsize() {
        // addressToByte(adjust, wordsize): wordsize-2 spaces scale the
        // adjustment from addressable words to bytes
        let m = manager(2);
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let path = write_raw_file("ws2");
        let mut img = RawLoadImage::new(path.to_str().unwrap());
        img.attach_to_space(Rc::clone(&ram));
        img.open().unwrap();
        img.adjust_vma(0x10); // 0x10 words -> 0x20 bytes
        let mut buf = [0u8; 2];
        img.load_fill(&mut buf, &Address::new(Rc::clone(&ram), 0x20)).unwrap();
        assert_eq!(buf, [0, 1]);

        std::fs::remove_file(&path).unwrap();
    }
}
