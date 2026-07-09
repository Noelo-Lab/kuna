//! Port of `decompiler/cpp/loadimage_xml.hh` + `loadimage_xml.cc` (W2, item
//! `w2-sleigh-loadimage`) — support for programs stored using an XML schema.
//!
//! Structural mapping (vs C++):
//!
//! - `const Element *rootel` becomes an `Rc<Element>` from
//!   [`kuna_base::xml`]; `map`/`set` members become `BTreeMap`/`BTreeSet`
//!   keyed by [`Address`] (whose `Ord` transcribes the C++ `operator<`, ADR
//!   0002).
//! - The `const AddrSpaceManager *manage` member is **not** stored (the
//!   workspace convention bans manager back-pointers, see
//!   `kuna_base::space` module docs): it was only ever read inside `open()`
//!   where the same manager arrives as the parameter.  `open()` also takes
//!   the explicit [`IdRegistry`] that replaces the C++ global id tables.
//! - The `mutable map<Address,string>::const_iterator cursymbol` becomes a
//!   `RefCell<Option<Address>>` cursor holding the next key to report
//!   (equivalent while the symbol map is unchanged between
//!   `open_symbols`/`get_next_symbol` calls — mutating it mid-iteration is
//!   iterator-invalidation UB in C++).  Before `openSymbols` the C++
//!   iterator is uninitialized (UB to use); the Rust cursor starts at
//!   "end", so a premature `get_next_symbol` returns `false`.
//! - Symbol names and the arch type are byte strings (`Vec<u8>`), per the
//!   marshal byte-string convention.
//! - C++ quirks transcribed deliberately: `clear()` does **not** clear
//!   `readonlyset`, and `adjustVma()` rebuilds `chunk`/`addrtosymbol` but
//!   not `readonlyset` (so previously readonly chunks stop being reported
//!   by `getReadonly` after an adjustment), exactly as upstream.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Bound;
use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{
    AttributeId, Decoder, ElementId, Encoder, IdRegistry, XmlDecode, ATTRIB_CONTENT, ATTRIB_NAME,
    ATTRIB_READONLY, ATTRIB_SPACE, ELEM_SYMBOL,
};
use kuna_base::space::{AddrSpace, AddrSpaceManager};
use kuna_base::types::Wrap;
use kuna_base::xml::Element;

use crate::loadimage::{LoadImage, LoadImageFunc};

/// Marshaling attribute "arch"
pub const ATTRIB_ARCH: AttributeId = AttributeId::new("arch", 135);

/// Marshaling element \<binaryimage>
pub const ELEM_BINARYIMAGE: ElementId = ElementId::new("binaryimage", 230);
/// Marshaling element \<bytechunk>
pub const ELEM_BYTECHUNK: ElementId = ElementId::new("bytechunk", 231);

/// Register the loadimage_xml.cc AttributeIds/ElementIds (C++ global-ctor
/// registration; see `kuna_base::marshal::IdRegistry`).
pub fn register_loadimage_xml_ids(registry: &mut IdRegistry) {
    registry.register_attribute(&ATTRIB_ARCH);
    registry.register_element(&ELEM_BINARYIMAGE);
    registry.register_element(&ELEM_BYTECHUNK);
}

/// The real space behind an Address in the image containers.  These all come
/// from `Address::new` over a registered space; C++ would dereference a null
/// space pointer here (UB, ADR 0004).
fn addr_space(a: &Address) -> &Rc<AddrSpace> {
    a.get_space().expect("LoadImageXml: address with null space pointer (C++ UB)")
}

/// `istream::get()` over an in-memory byte string, truncated to `char` as in
/// the C++ hex-content loop: returns the next byte as a *signed* char (high
/// bit set comes back negative, terminating the caller's `> 0` loop), or -1
/// (`traits::eof()` truncated to char) past the end.
fn istream_get(content: &[u8], pos: &mut usize) -> i8 {
    if *pos >= content.len() {
        return -1;
    }
    let b = content[*pos];
    *pos += 1;
    b as i8 // cast: C++ assigns the int from get() into a (signed) char
}

/// `is >> ws`: skip the classic-locale whitespace characters.
fn skip_ws(content: &[u8], pos: &mut usize) {
    while *pos < content.len() {
        match content[*pos] {
            b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r' => *pos += 1,
            _ => break,
        }
    }
}

/// \brief Implementation of the LoadImage interface using underlying data
/// stored in an XML format
///
/// The image data is stored in an XML file in a \<binaryimage> file.
/// The data is encoded in \<bytechunk> and potentially \<symbol> files.
#[derive(Debug)]
pub struct LoadImageXml {
    /// Name of the loadimage (the `LoadImage` base-class member)
    filename: String,
    /// The root XML element
    rootel: Rc<Element>,
    /// The architecture string (byte string from the XML attribute)
    archtype: Vec<u8>,
    /// Starting address of read-only chunks
    readonlyset: BTreeSet<Address>,
    /// Chunks of image data, mapped by address
    chunk: BTreeMap<Address, Vec<u8>>,
    /// Symbols sorted by address (byte-string names)
    addrtosymbol: BTreeMap<Address, Vec<u8>>,
    /// Current symbol being reported (C++ `mutable` map iterator; holds the
    /// next key to report, `None` meaning `end()`)
    cursymbol: RefCell<Option<Address>>,
}

impl LoadImageXml {
    /// Constructor
    /// \param f is the (path to the) underlying XML file
    /// \param el is the parsed form of the file
    pub fn new(f: &str, el: Rc<Element>) -> KunaResult<LoadImageXml> {
        // Extract architecture information
        if el.get_name() != "binaryimage" {
            return Err(KunaError::lowlevel(format!("Missing binaryimage tag in {f}")));
        }
        let archtype = el.get_attribute_value("arch")?.to_vec();
        Ok(LoadImageXml {
            filename: f.to_string(),
            rootel: el,
            archtype,
            readonlyset: BTreeSet::new(),
            chunk: BTreeMap::new(),
            addrtosymbol: BTreeMap::new(),
            cursymbol: RefCell::new(None),
        })
    }

    /// Encode the image to a stream.
    ///
    /// Encode the byte chunks and symbols as elements
    /// \param encoder is the stream encoder
    pub fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        encoder.open_element(&ELEM_BINARYIMAGE);
        encoder.write_string(&ATTRIB_ARCH, &self.archtype);

        for (addr, vec) in &self.chunk {
            if vec.is_empty() {
                continue;
            }
            encoder.open_element(&ELEM_BYTECHUNK);
            addr_space(addr).encode_attributes(encoder, addr.get_offset())?;
            if self.readonlyset.contains(addr) {
                // C++ `encoder.writeBool(ATTRIB_READONLY, "true")`: the
                // string literal converts to the bool `true`
                encoder.write_bool(&ATTRIB_READONLY, true);
            }
            // hex, width-2, zero-padded bytes; a '\n' after every 20th
            // byte and a trailing '\n'
            let mut s = String::from("\n");
            for (i, b) in vec.iter().enumerate() {
                s.push_str(&format!("{b:02x}"));
                if i % 20 == 19 {
                    s.push('\n');
                }
            }
            s.push('\n');
            encoder.write_string(&ATTRIB_CONTENT, s.as_bytes());
            encoder.close_element(&ELEM_BYTECHUNK);
        }

        for (addr, nm) in &self.addrtosymbol {
            encoder.open_element(&ELEM_SYMBOL);
            addr_space(addr).encode_attributes(encoder, addr.get_offset())?;
            encoder.write_string(&ATTRIB_NAME, nm);
            encoder.close_element(&ELEM_SYMBOL);
        }
        encoder.close_element(&ELEM_BINARYIMAGE);
        Ok(())
    }

    /// Read XML tags into the containers.
    /// \param m is for looking up address space
    /// \param registry holds the name -> id tables (C++ global static
    /// registration; an explicit parameter per the W1 marshal conventions)
    pub fn open(&mut self, m: &AddrSpaceManager, registry: &IdRegistry) -> KunaResult<()> {
        // (C++ stores `manage = m` here; the Rust port does not keep the
        // back-pointer — see module docs)
        let mut sz: u32 = 0; // unused size

        // Read parsed xml file
        let root = Rc::clone(&self.rootel);
        let mut decoder = XmlDecode::new_with_root(m, registry, &root, 0);
        let elem_id = decoder.open_element_id(&ELEM_BINARYIMAGE)?;
        loop {
            let sub_id = decoder.open_element()?;
            if sub_id == 0 {
                break;
            }
            if sub_id == ELEM_SYMBOL {
                let base = decoder.read_space_id(&ATTRIB_SPACE)?;
                let addr =
                    Address::new(Rc::clone(&base), base.decode_attributes(&mut decoder, &mut sz)?);
                let nm = decoder.read_string_id(&ATTRIB_NAME)?;
                self.addrtosymbol.insert(addr, nm);
            } else if sub_id == ELEM_BYTECHUNK {
                let base = decoder.read_space_id(&ATTRIB_SPACE)?;
                let addr =
                    Address::new(Rc::clone(&base), base.decode_attributes(&mut decoder, &mut sz)?);
                let vec = self.chunk.entry(addr.clone()).or_default();
                vec.clear();
                decoder.rewind_attributes();
                loop {
                    let attrib_id = decoder.get_next_attribute_id()?;
                    if attrib_id == 0 {
                        break;
                    }
                    if attrib_id == ATTRIB_READONLY && decoder.read_bool()? {
                        self.readonlyset.insert(addr.clone());
                    }
                }
                // istringstream over the content attribute: whitespace is
                // skipped before each hex *pair* (never between the two
                // digits of a pair), and any non-positive char — EOF or a
                // high-bit byte — terminates the loop
                let content = decoder.read_string_id(&ATTRIB_CONTENT)?;
                let mut pos: usize = 0;
                skip_ws(&content, &mut pos); // is >> ws
                let mut c1 = istream_get(&content, &mut pos);
                let mut c2 = istream_get(&content, &mut pos);
                while c1 > 0 && c2 > 0 {
                    // The C++ digit conversion does no validation: chars in
                    // ('9','F'] are treated as upper-case hex, everything
                    // above 'F' as lower-case.  Arithmetic happens in int
                    // and truncates back to char on assignment (wrapping_*).
                    if c1 <= b'9' as i8 {
                        c1 = c1.wrapping_sub(b'0' as i8);
                    } else if c1 <= b'F' as i8 {
                        c1 = c1.wrapping_add(10).wrapping_sub(b'A' as i8);
                    } else {
                        c1 = c1.wrapping_add(10).wrapping_sub(b'a' as i8);
                    }
                    if c2 <= b'9' as i8 {
                        c2 = c2.wrapping_sub(b'0' as i8);
                    } else if c2 <= b'F' as i8 {
                        c2 = c2.wrapping_add(10).wrapping_sub(b'A' as i8);
                    } else {
                        c2 = c2.wrapping_add(10).wrapping_sub(b'a' as i8);
                    }
                    // int4 val = c1*16 + c2 (char promotes to int)
                    let val: i32 = i32::from(c1) * 16 + i32::from(c2);
                    vec.push(val as u8); // cast: (uint1)val truncation
                    skip_ws(&content, &mut pos); // is >> ws
                    c1 = istream_get(&content, &mut pos);
                    c2 = istream_get(&content, &mut pos);
                }
            } else {
                return Err(KunaError::lowlevel("Unknown LoadImageXml tag"));
            }
            decoder.close_element(sub_id)?;
        }
        decoder.close_element(elem_id)?;
        self.pad();
        Ok(())
    }

    /// Clear out all the caches
    pub fn clear(&mut self) {
        self.archtype.clear();
        // (C++ also nulls the `manage` back-pointer, which the Rust port
        // does not store.  Note it does NOT clear `readonlyset`.)
        self.chunk.clear();
        self.addrtosymbol.clear();
    }

    /// Make sure every chunk is followed by at least 512 bytes of pad
    fn pad(&mut self) {
        // Search for completely redundant chunks
        if self.chunk.is_empty() {
            return;
        }
        // C++ walks (lastiter, iter) pairs, erasing `iter` when its chunk
        // ends at or before the end of `lastiter`'s chunk.  Erasure during
        // iteration is reproduced over a pre-collected key list: after an
        // erase, `iter = lastiter; ++iter` is exactly the next key in the
        // original order.
        let keys: Vec<Address> = self.chunk.keys().cloned().collect();
        let mut lastkey: Address = keys[0].clone();
        for key in keys.iter().skip(1) {
            if Rc::ptr_eq(addr_space(&lastkey), addr_space(key)) {
                // end = offset + size - 1 in wrapping uintb arithmetic
                let end1 = lastkey
                    .get_offset()
                    .wadd(self.chunk[&lastkey].len() as u64) // cast: size_t chunk length
                    .wsub(1);
                let end2 = key
                    .get_offset()
                    .wadd(self.chunk[key].len() as u64) // cast: size_t chunk length
                    .wsub(1);
                if end1 >= end2 {
                    self.chunk.remove(key);
                    continue; // lastiter unchanged
                }
            }
            lastkey = key.clone();
        }

        // C++ inserts pad chunks *while* iterating; every insertion lands
        // strictly between the current and the next original chunk (or
        // after the last), so the freshly inserted pads are never
        // themselves visited.  Iterating the pre-collected surviving key
        // list reproduces that exactly.
        let keys: Vec<Address> = self.chunk.keys().cloned().collect();
        for i in 0..keys.len() {
            let curkey = &keys[i];
            // Address operator+ wraps through AddrSpace::wrapOffset
            let endaddr = curkey + self.chunk[curkey].len() as i64; // cast: size_t -> the int8 operator+ argument
            if endaddr < *curkey {
                continue; // All the way to end of space
            }
            let mut maxsize: i32 = 512;
            let mut room: u64 =
                addr_space(&endaddr).get_highest().wsub(endaddr.get_offset()).wadd(1);
            // (uintb)maxsize: the int4 sign-extends into the comparison
            if maxsize as i64 as u64 > room {
                maxsize = room as i32; // cast: (int4)room truncation as in C++
            }
            if i + 1 < keys.len() && Rc::ptr_eq(addr_space(&keys[i + 1]), addr_space(&endaddr)) {
                if endaddr.get_offset() >= keys[i + 1].get_offset() {
                    continue;
                }
                room = keys[i + 1].get_offset().wsub(endaddr.get_offset());
                if maxsize as i64 as u64 > room {
                    maxsize = room as i32; // cast: (int4)room truncation as in C++
                }
            }
            // operator[] creates the entry if absent (it can hit the
            // current chunk itself when its size is 0)
            let vec = self.chunk.entry(endaddr).or_default();
            for _i in 0..maxsize {
                vec.push(0);
            }
        }
    }
}

impl LoadImage for LoadImageXml {
    fn get_file_name(&self) -> &str {
        &self.filename
    }

    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        // cast: the C++ `int4 size` parameter (slice length; see trait docs)
        let mut size: i32 = ptr.len() as i32;
        let mut dest: usize = 0; // the C++ output pointer cursor
        let mut emptyhit = false;

        let mut curaddr: Address = addr.clone();
        // C++: the last chunk with key <= curaddr (upper_bound, then --iter)
        let mut iter_key: Option<Address> =
            match self.chunk.range(..=curaddr.clone()).next_back() {
                Some((k, _)) => Some(k.clone()),
                // upper_bound == begin(): the iterator stays at begin(),
                // which may be a chunk above curaddr or end() (empty map)
                None => self.chunk.keys().next().cloned(),
            };
        while size > 0 {
            let Some(key) = iter_key else {
                break; // iter == chunk.end()
            };
            let chnk = &self.chunk[&key];
            // cast: int4 chnksize = chnk.size() (C++ size_t -> int4)
            let mut chnksize: i32 = chnk.len() as i32;
            let over = curaddr.overlap(0, &key, chnksize);
            if over != -1 {
                if chnksize - over > size {
                    chnksize = over + size;
                }
                let mut i = over;
                while i < chnksize {
                    ptr[dest] = chnk[i as usize]; // cast: int4 index, non-negative here
                    dest += 1;
                    i += 1;
                }
                size -= chnksize - over;
                curaddr = &curaddr + i64::from(chnksize - over);
                // ++iter
                iter_key = self
                    .chunk
                    .range((Bound::Excluded(key), Bound::Unbounded))
                    .next()
                    .map(|(k, _)| k.clone());
            } else {
                emptyhit = true;
                break;
            }
        }
        if size > 0 || emptyhit {
            let mut errmsg = String::from("Bytes at ");
            curaddr.print_raw(&mut errmsg)?;
            errmsg.push_str(" are not mapped");
            return Err(KunaError::data_unavail(errmsg));
        }
        Ok(())
    }

    fn open_symbols(&self) {
        *self.cursymbol.borrow_mut() = self.addrtosymbol.keys().next().cloned();
    }

    fn get_next_symbol(&self, record: &mut LoadImageFunc) -> bool {
        let cur = self.cursymbol.borrow().clone();
        let Some(addr) = cur else {
            return false;
        };
        // (A cursor whose key has vanished — the map was mutated since
        // openSymbols — is iterator-invalidation UB in C++)
        let Some(name) = self.addrtosymbol.get(&addr) else {
            return false;
        };
        record.name = name.clone();
        record.address = addr.clone();
        // ++cursymbol
        *self.cursymbol.borrow_mut() = self
            .addrtosymbol
            .range((Bound::Excluded(addr), Bound::Unbounded))
            .next()
            .map(|(k, _)| k.clone());
        true
    }

    fn get_readonly(&self, list: &mut RangeList) {
        // List all the readonly chunks
        for (addr, chnk) in &self.chunk {
            if self.readonlyset.contains(addr) {
                let start = addr.get_offset();
                // stop = start + chnk.size() - 1 in wrapping uintb arithmetic
                let stop = start.wadd(chnk.len() as u64).wsub(1); // cast: size_t chunk length
                list.insert_range(Rc::clone(addr_space(addr)), start, stop);
            }
        }
    }

    fn get_arch_type(&self) -> Vec<u8> {
        self.archtype.clone()
    }

    fn adjust_vma(&mut self, adjust: i64) {
        // mutable_key_type: AddrSpace's interior-mutable fields
        // (flags/shortcut Cells) do not participate in Address's Ord, which
        // reads only the immutable space index and the offset — the key
        // order cannot change while a key is in the map.
        #[allow(clippy::mutable_key_type)]
        let mut newchunk: BTreeMap<Address, Vec<u8>> = BTreeMap::new();
        #[allow(clippy::mutable_key_type)] // see newchunk's note
        let mut newsymbol: BTreeMap<Address, Vec<u8>> = BTreeMap::new();

        // (C++ copies entries into the new maps and assigns over the old;
        // draining the old map first is observationally identical)
        for (a, v) in std::mem::take(&mut self.chunk) {
            let spc = Rc::clone(addr_space(&a));
            // addressToByte: the long argument converts to uintb
            // (sign-extension) and the uintb result truncates into the int4
            let off = AddrSpace::address_to_byte(adjust as u64, spc.get_word_size()) as i32;
            let newaddr = &a + i64::from(off); // int4 sign-extends into operator+
            newchunk.insert(newaddr, v);
        }
        self.chunk = newchunk;
        for (a, nm) in std::mem::take(&mut self.addrtosymbol) {
            let spc = Rc::clone(addr_space(&a));
            // (same conversion chain as above)
            let off = AddrSpace::address_to_byte(adjust as u64, spc.get_word_size()) as i32;
            let newaddr = &a + i64::from(off);
            newsymbol.insert(newaddr, nm);
        }
        self.addrtosymbol = newsymbol;
        // (C++ does NOT re-key readonlyset — transcribed faithfully)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::marshal::XmlEncode;
    use kuna_base::space::{addrspace_flags, spacetype, ConstantSpace};
    use kuna_base::xml::xml_tree;

    /// Build a manager with const(0) + one processor space.
    fn manager(name: &str, big_end: bool, addr_size: u32, wordsize: u32) -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            name,
            big_end,
            addr_size,
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

    fn registry() -> IdRegistry {
        let mut registry = IdRegistry::with_base_ids();
        register_loadimage_xml_ids(&mut registry);
        registry
    }

    /// Parse a datatest file and pull out its <binaryimage> element.
    fn fixture_binaryimage(name: &str) -> Rc<Element> {
        let path = format!(
            "{}/../../../tests/datatests/{}",
            env!("CARGO_MANIFEST_DIR"),
            name
        );
        let data = std::fs::read(&path).unwrap();
        let doc = xml_tree(&data).unwrap();
        let root = doc.get_root();
        assert_eq!(root.get_name(), "decompilertest");
        let el = root
            .get_children()
            .iter()
            .find(|c| c.get_name() == "binaryimage")
            .expect("fixture has no <binaryimage>");
        Rc::clone(el)
    }

    fn open_fixture(
        name: &str,
        m: &AddrSpaceManager,
        registry: &IdRegistry,
    ) -> LoadImageXml {
        let el = fixture_binaryimage(name);
        let mut img = LoadImageXml::new(name, el).unwrap();
        img.open(m, registry).unwrap();
        img
    }

    fn hex(s: &str) -> Vec<u8> {
        assert!(s.len().is_multiple_of(2));
        (0..s.len() / 2)
            .map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap())
            .collect()
    }

    /// retspecial.xml: AARCH64:LE:64, two readonly ram chunks at 0x455c14
    /// (48 bytes) and 0x455c50 (28 bytes), two symbols.
    #[test]
    fn fixture_aarch64_le_exact_reads_and_gap_fill() {
        let m = manager("ram", false, 8, 1);
        let reg = registry();
        let mut img = open_fixture("retspecial.xml", &m, &reg);
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());

        assert_eq!(img.get_arch_type(), b"AARCH64:LE:64:v8A:default".to_vec());

        // Exact bytes at the start of the first chunk
        let got = img.load(12, &Address::new(Rc::clone(&ram), 0x455c14)).unwrap();
        assert_eq!(got, hex("ff4300d1e10308aae00f00b9"));

        // Read crossing a line boundary inside the chunk
        let got = img.load(8, &Address::new(Rc::clone(&ram), 0x455c1c)).unwrap();
        assert_eq!(got, hex("e00f00b9400180d2"));

        // Chunk 1 covers [0x455c14,0x455c44); the 12-byte gap to chunk 2 at
        // 0x455c50 is padded with zeroes; read straddling chunk1 tail ->
        // gap -> chunk2 head
        let got = img.load(20, &Address::new(Rc::clone(&ram), 0x455c40)).unwrap();
        let mut expect = hex("c0035fd6");
        expect.extend(std::iter::repeat_n(0u8, 12));
        expect.extend(hex("fd7bbda9"));
        assert_eq!(got, expect);

        // Chunk 2 ends at 0x455c6c and is followed by exactly 512 pad bytes
        let got = img.load(4, &Address::new(Rc::clone(&ram), 0x455c6c + 508)).unwrap();
        assert_eq!(got, vec![0, 0, 0, 0]);

        // One byte past the pad: unfilled error with the exact message
        // (sz drops to 4 bytes for an offset below 2^32 in printRaw)
        let err = img.load(1, &Address::new(Rc::clone(&ram), 0x455c6c + 512)).unwrap_err();
        match &err {
            KunaError::DataUnavail { explain } => {
                assert_eq!(explain, "Bytes at 0x00455e6c are not mapped");
            }
            other => panic!("expected DataUnavail, got {other:?}"),
        }

        // A read that *starts* mapped but overruns the pad also fails, with
        // curaddr advanced to the first unmapped byte
        let err = img.load(0x300, &Address::new(Rc::clone(&ram), 0x455c50)).unwrap_err();
        match &err {
            KunaError::DataUnavail { explain } => {
                assert_eq!(explain, "Bytes at 0x00455e6c are not mapped");
            }
            other => panic!("expected DataUnavail, got {other:?}"),
        }

        // A read entirely below the image: emptyhit error at the original
        // address
        let err = img.load(4, &Address::new(Rc::clone(&ram), 0x1000)).unwrap_err();
        match &err {
            KunaError::DataUnavail { explain } => {
                assert_eq!(explain, "Bytes at 0x00001000 are not mapped");
            }
            other => panic!("expected DataUnavail, got {other:?}"),
        }
    }

    #[test]
    fn fixture_aarch64_le_symbols_and_readonly() {
        let m = manager("ram", false, 8, 1);
        let reg = registry();
        let img = open_fixture("retspecial.xml", &m, &reg);
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());

        // get_next_symbol before open_symbols: C++ UB (uninitialized
        // iterator); the port reports end-of-symbols
        let mut rec = LoadImageFunc::default();
        assert!(!img.get_next_symbol(&mut rec));

        img.open_symbols();
        assert!(img.get_next_symbol(&mut rec));
        assert_eq!(rec.address, Address::new(Rc::clone(&ram), 0x455c14));
        assert_eq!(rec.name, b"returnbig".to_vec());
        assert!(img.get_next_symbol(&mut rec));
        assert_eq!(rec.address, Address::new(Rc::clone(&ram), 0x455c50));
        assert_eq!(rec.name, b"read_returnbig".to_vec());
        assert!(!img.get_next_symbol(&mut rec));
        img.close_symbols();

        // Both fixture chunks are readonly="true"; the inserted pad chunks
        // are not in readonlyset, so exactly the original extents come back
        let mut list = RangeList::new();
        img.get_readonly(&mut list);
        let ranges: Vec<(u64, u64)> = list.iter().map(|r| (r.get_first(), r.get_last())).collect();
        assert_eq!(ranges, vec![(0x455c14, 0x455c43), (0x455c50, 0x455c6b)]);
    }

    /// bitfields2.xml: MIPS:BE:32, 16 readonly chunks; the first is 64
    /// bytes at 0x100000 with a 192-byte gap to 0x100100.
    #[test]
    fn fixture_mips_be_exact_reads_and_gap_fill() {
        let m = manager("ram", true, 4, 1);
        let reg = registry();
        let mut img = open_fixture("bitfields2.xml", &m, &reg);
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());

        assert_eq!(img.get_arch_type(), b"MIPS:BE:32:default:default".to_vec());

        let got = img.load(16, &Address::new(Rc::clone(&ram), 0x100000)).unwrap();
        assert_eq!(got, hex("0005110028a3000a3442000400431025"));

        // Tail of chunk 1, the zero-filled gap, and the head of chunk 2
        let got = img.load(16 + 192 + 4, &Address::new(Rc::clone(&ram), 0x100030)).unwrap();
        let mut expect = hex("3442006403e00008a482000800000000");
        expect.extend(std::iter::repeat_n(0u8, 192));
        expect.extend(hex("24020001"));
        assert_eq!(got, expect);

        // Unfilled error in a 4-byte space prints 8 hex digits
        let err = img.load(1, &Address::new(Rc::clone(&ram), 0x10)).unwrap_err();
        match &err {
            KunaError::DataUnavail { explain } => {
                assert_eq!(explain, "Bytes at 0x00000010 are not mapped");
            }
            other => panic!("expected DataUnavail, got {other:?}"),
        }
    }

    /// boolless.xml: 8051:BE:16, a 10-byte chunk at 0xa000 in the 2-byte
    /// "CODE" space whose content mixes line lengths ("e5" alone on the
    /// first line) — exercising the whitespace-between-pairs scan.
    #[test]
    fn fixture_8051_be_small_space() {
        let m = manager("CODE", true, 2, 1);
        let reg = registry();
        let mut img = open_fixture("boolless.xml", &m, &reg);
        let code = Rc::clone(m.get_space_by_name("CODE").unwrap());

        assert_eq!(img.get_arch_type(), b"8051:BE:16:default:default".to_vec());

        let got = img.load(10, &Address::new(Rc::clone(&code), 0xa000)).unwrap();
        assert_eq!(got, hex("e552b40b005002740122"));

        // 512 bytes of pad fit in the 16-bit space; past them: error with
        // the 2-byte-space (4 hex digit) rendering
        let got = img.load(2, &Address::new(Rc::clone(&code), 0xa00a + 510)).unwrap();
        assert_eq!(got, vec![0, 0]);
        let err = img.load(1, &Address::new(Rc::clone(&code), 0xa00a + 512)).unwrap_err();
        match &err {
            KunaError::DataUnavail { explain } => {
                assert_eq!(explain, "Bytes at 0xa20a are not mapped");
            }
            other => panic!("expected DataUnavail, got {other:?}"),
        }

        let mut rec = LoadImageFunc::default();
        img.open_symbols();
        assert!(img.get_next_symbol(&mut rec));
        assert_eq!(rec.address, Address::new(Rc::clone(&code), 0xa000));
        assert_eq!(rec.name, b"boolless".to_vec());
        assert!(!img.get_next_symbol(&mut rec));
    }

    /// x86:LE:64 fixture with several symbols on one chunk.
    #[test]
    fn fixture_x86_le_symbols() {
        let m = manager("ram", false, 8, 1);
        let reg = registry();
        let img = open_fixture("condconst.xml", &m, &reg);
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());

        let mut rec = LoadImageFunc::default();
        img.open_symbols();
        let mut syms: Vec<(u64, Vec<u8>)> = Vec::new();
        while img.get_next_symbol(&mut rec) {
            assert!(Rc::ptr_eq(rec.address.get_space().unwrap(), &ram));
            syms.push((rec.address.get_offset(), rec.name.clone()));
        }
        assert_eq!(
            syms,
            vec![
                (0x1006fa, b"condconst1".to_vec()),
                (0x10076a, b"condconst_copy".to_vec()),
                (0x1007a4, b"condconst_conn".to_vec()),
            ]
        );
    }

    fn image_from_str(xml: &str, m: &AddrSpaceManager, registry: &IdRegistry) -> LoadImageXml {
        let doc = xml_tree(xml.as_bytes()).unwrap();
        let mut img = LoadImageXml::new("inline", Rc::clone(doc.get_root())).unwrap();
        img.open(m, registry).unwrap();
        img
    }

    /// pad(): a chunk fully covered by its predecessor is erased, and the
    /// surviving chunk supplies the bytes for its address range.
    #[test]
    fn pad_erases_redundant_chunk() {
        let m = manager("ram", false, 4, 1);
        let reg = registry();
        let mut img = image_from_str(
            "<binaryimage arch=\"test\">\n\
             <bytechunk space=\"ram\" offset=\"0x100\">\n\
             0102030405060708090a0b0c0d0e0f10\n\
             </bytechunk>\n\
             <bytechunk space=\"ram\" offset=\"0x108\">\n\
             eeee\n\
             </bytechunk>\n\
             </binaryimage>",
            &m,
            &reg,
        );
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        // The contained chunk at 0x108 was erased: bytes come from the big
        // chunk, not the "eeee" payload
        let got = img.load(4, &Address::new(Rc::clone(&ram), 0x108)).unwrap();
        assert_eq!(got, hex("090a0b0c"));
    }

    /// pad(): the 512-byte pad is clamped by both the next chunk and the
    /// end of the address space.
    #[test]
    fn pad_clamps_to_next_chunk_and_space_end() {
        let m = manager("ram", false, 2, 1);
        let reg = registry();
        let mut img = image_from_str(
            "<binaryimage arch=\"test\">\n\
             <bytechunk space=\"ram\" offset=\"0x10\">\n\
             aabb\n\
             </bytechunk>\n\
             <bytechunk space=\"ram\" offset=\"0x18\">\n\
             ccdd\n\
             </bytechunk>\n\
             <bytechunk space=\"ram\" offset=\"0xfff0\">\n\
             0011223344556677\n\
             </bytechunk>\n\
             </binaryimage>",
            &m,
            &reg,
        );
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());

        // Gap [0x12,0x18) is fully padded: a straddling read succeeds
        let got = img.load(10, &Address::new(Rc::clone(&ram), 0x10)).unwrap();
        assert_eq!(got, hex("aabb000000000000ccdd"));

        // Last chunk [0xfff0,0xfff8) pads only to the space end (8 bytes:
        // room = 0xffff - 0xfff8 + 1), so 0xffff reads 0 and the wrap
        // around to 0x0000 is unmapped
        let got = img.load(1, &Address::new(Rc::clone(&ram), 0xffff)).unwrap();
        assert_eq!(got, vec![0]);
        let err = img.load(1, &Address::new(Rc::clone(&ram), 0x0)).unwrap_err();
        assert!(matches!(err, KunaError::DataUnavail { .. }));
    }

    /// Constructor and open() error contracts.
    #[test]
    fn constructor_and_open_errors() {
        let m = manager("ram", false, 4, 1);
        let reg = registry();

        // Root element is not <binaryimage>
        let doc = xml_tree(b"<other arch=\"x\"/>").unwrap();
        let err = LoadImageXml::new("file.xml", Rc::clone(doc.get_root())).unwrap_err();
        match &err {
            KunaError::Lowlevel { explain } => {
                assert_eq!(explain, "Missing binaryimage tag in file.xml");
            }
            other => panic!("expected Lowlevel, got {other:?}"),
        }

        // Missing arch attribute: DecoderError from getAttributeValue
        let doc = xml_tree(b"<binaryimage/>").unwrap();
        let err = LoadImageXml::new("file.xml", Rc::clone(doc.get_root())).unwrap_err();
        match &err {
            KunaError::Decoder { explain } => assert_eq!(explain, "Unknown attribute: arch"),
            other => panic!("expected Decoder, got {other:?}"),
        }

        // Unknown child tag inside <binaryimage>
        let doc = xml_tree(b"<binaryimage arch=\"x\"><foo/></binaryimage>").unwrap();
        let mut img = LoadImageXml::new("file.xml", Rc::clone(doc.get_root())).unwrap();
        let err = img.open(&m, &reg).unwrap_err();
        match &err {
            KunaError::Lowlevel { explain } => assert_eq!(explain, "Unknown LoadImageXml tag"),
            other => panic!("expected Lowlevel, got {other:?}"),
        }
    }

    /// encode() round-trips through the XML encoder: a re-decoded image
    /// returns the same bytes, symbols, arch and readonly ranges.
    #[test]
    fn encode_round_trip() {
        let m = manager("ram", false, 4, 1);
        let reg = registry();
        let mut img = image_from_str(
            "<binaryimage arch=\"test:arch\">\n\
             <bytechunk space=\"ram\" offset=\"0x100\" readonly=\"true\">\n\
             000102030405060708090a0b0c0d0e0f10111213\n\
             1415\n\
             </bytechunk>\n\
             <symbol space=\"ram\" offset=\"0x100\" name=\"main\"/>\n\
             <symbol space=\"ram\" offset=\"0x110\" name=\"helper\"/>\n\
             </binaryimage>",
            &m,
            &reg,
        );
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());

        let mut out: Vec<u8> = Vec::new();
        {
            let mut encoder = XmlEncode::new(&mut out);
            img.encode(&mut encoder).unwrap();
        }
        let doc = xml_tree(&out).unwrap();
        let mut img2 = LoadImageXml::new("roundtrip", Rc::clone(doc.get_root())).unwrap();
        img2.open(&m, &reg).unwrap();

        assert_eq!(img2.get_arch_type(), b"test:arch".to_vec());
        let a = img.load(22, &Address::new(Rc::clone(&ram), 0x100)).unwrap();
        let b = img2.load(22, &Address::new(Rc::clone(&ram), 0x100)).unwrap();
        assert_eq!(a, b);
        assert_eq!(b, hex("000102030405060708090a0b0c0d0e0f101112131415"));

        let mut rec = LoadImageFunc::default();
        img2.open_symbols();
        assert!(img2.get_next_symbol(&mut rec));
        assert_eq!((rec.address.get_offset(), rec.name.clone()), (0x100, b"main".to_vec()));
        assert!(img2.get_next_symbol(&mut rec));
        assert_eq!((rec.address.get_offset(), rec.name.clone()), (0x110, b"helper".to_vec()));
        assert!(!img2.get_next_symbol(&mut rec));

        // readonly survives the round trip on the original chunk extent
        let mut list = RangeList::new();
        img2.get_readonly(&mut list);
        let ranges: Vec<(u64, u64)> = list.iter().map(|r| (r.get_first(), r.get_last())).collect();
        assert_eq!(ranges, vec![(0x100, 0x115)]);
    }

    /// adjustVma rebuilds the chunk/symbol maps (scaled by wordsize) but —
    /// as in C++ — leaves readonlyset keyed by the old addresses.
    #[test]
    fn adjust_vma_moves_chunks_and_symbols_but_not_readonlyset() {
        let m = manager("ram", false, 4, 1);
        let reg = registry();
        let mut img = image_from_str(
            "<binaryimage arch=\"test\">\n\
             <bytechunk space=\"ram\" offset=\"0x100\" readonly=\"true\">\n\
             a1b2c3d4\n\
             </bytechunk>\n\
             <symbol space=\"ram\" offset=\"0x102\" name=\"sym\"/>\n\
             </binaryimage>",
            &m,
            &reg,
        );
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());

        img.adjust_vma(0x10);
        let got = img.load(4, &Address::new(Rc::clone(&ram), 0x110)).unwrap();
        assert_eq!(got, hex("a1b2c3d4"));
        let err = img.load(4, &Address::new(Rc::clone(&ram), 0x100)).unwrap_err();
        assert!(matches!(err, KunaError::DataUnavail { .. }));

        let mut rec = LoadImageFunc::default();
        img.open_symbols();
        assert!(img.get_next_symbol(&mut rec));
        assert_eq!(rec.address.get_offset(), 0x112);

        // readonlyset still holds 0x100, which no longer matches any chunk:
        // getReadonly reports nothing (faithful C++ quirk)
        let mut list = RangeList::new();
        img.get_readonly(&mut list);
        assert_eq!(list.num_ranges(), 0);
    }

    /// clear() empties archtype/chunks/symbols (and deliberately does not
    /// touch readonlyset, as in C++).
    #[test]
    fn clear_empties_caches() {
        let m = manager("ram", false, 4, 1);
        let reg = registry();
        let mut img = image_from_str(
            "<binaryimage arch=\"test\">\n\
             <bytechunk space=\"ram\" offset=\"0x100\" readonly=\"true\">\n\
             aa\n\
             </bytechunk>\n\
             </binaryimage>",
            &m,
            &reg,
        );
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        img.clear();
        assert_eq!(img.get_arch_type(), Vec::<u8>::new());
        let err = img.load(1, &Address::new(Rc::clone(&ram), 0x100)).unwrap_err();
        assert!(matches!(err, KunaError::DataUnavail { .. }));
        let mut rec = LoadImageFunc::default();
        img.open_symbols();
        assert!(!img.get_next_symbol(&mut rec));
    }
}
