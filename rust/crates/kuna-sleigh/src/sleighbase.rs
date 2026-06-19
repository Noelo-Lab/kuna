//! Port of `decompiler/cpp/sleighbase.{hh,cc}` (W2, item `w2-sleigh-core`):
//! [`SourceFileIndexer`] and [`SleighBase`], the common core of everything
//! that reads or writes SLEIGH specification files natively.
//!
//! Architecture mapping (C++ has no multiple inheritance to reproduce):
//!
//! - C++ `SleighBase : Translate` *is* an `AddrSpaceManager` and *holds* the
//!   `TranslateBase` state.  The Rust [`SleighBase`] **owns** an
//!   `AddrSpaceManager`, a [`TranslateBase`], the [`SymbolTable`], the
//!   register cross-reference map, the user-op list, and the
//!   `ConstructTpl` store (the `ConstructTplHandle` backing the
//!   [`SleighBaseTrans`] seam).  The concrete `Sleigh` engine (sleigh.rs)
//!   embeds a `SleighBase` and adds the load image / context machinery.
//! - The C++ register virtuals (`getRegister`/`getRegisterName`/...) are the
//!   [`RegisterLookup`] surface; [`SleighBase`] is installed as the manager's
//!   register lookup so kuna-base decode paths reach the symbol-table-backed
//!   register map.  Because the lookup is shared by `Rc`, the register map is
//!   built once after decode and stored behind a clone-on-read accessor.
//! - The `SleighBaseTrans` seam needed by `SymbolTable::decode`
//!   (`getConstantSpace` + `ConstructTpl` decode/encode) is satisfied by a
//!   short-lived [`SlaTrans`] borrowing the constant space and the template
//!   store, so it stays disjoint from the `&mut symtab` borrow.

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{Decoder, Encoder, IdRegistry};
use kuna_base::space::{
    addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, OtherSpace,
    RegisterLookup, UniqueSpace, VarnodeStorage,
};

use kuna_num::opcodes::{OpCode, OpcodeDecoder, OpcodeEncoder};
use kuna_num::pcoderaw::VarnodeData;

use crate::semantics::ConstructTpl;
use crate::slaformat as sla;
use crate::slghpatexpress::PatternValue;
use crate::slghsymbol::{
    ConstructTplHandle, SleighBaseTrans, SleighSymbol, SymbolKind, SymbolTable, SymbolType,
};
use crate::translate::{storage_from_varnode_data, TranslateBase};

/// C++ `SourceFileIndexer`: associates each constructor in a SLEIGH language
/// with the source file where it is defined.
#[derive(Debug, Default)]
pub struct SourceFileIndexer {
    /// One-up count for assigning indices to files (C++ `leastUnusedIndex`).
    least_unused_index: i32,
    /// Map from indices to files (C++ `indexToFile`).
    index_to_file: std::collections::BTreeMap<i32, Vec<u8>>,
    /// Map from files to indices (C++ `fileToIndex`).
    file_to_index: std::collections::BTreeMap<Vec<u8>, i32>,
}

impl SourceFileIndexer {
    /// C++ `SourceFileIndexer()`.
    pub fn new() -> SourceFileIndexer {
        SourceFileIndexer::default()
    }

    /// C++ `SourceFileIndexer::index`: returns the index of the file, adding
    /// it if absent.
    pub fn index(&mut self, filename: &[u8]) -> i32 {
        if let Some(i) = self.file_to_index.get(filename) {
            return *i;
        }
        self.file_to_index.insert(filename.to_vec(), self.least_unused_index);
        self.index_to_file.insert(self.least_unused_index, filename.to_vec());
        let res = self.least_unused_index;
        self.least_unused_index += 1;
        res
    }

    /// C++ `SourceFileIndexer::getIndex` (`map::operator[]` default-inserts 0
    /// on a miss; the Rust returns 0 without mutating).
    pub fn get_index(&self, filename: &[u8]) -> i32 {
        self.file_to_index.get(filename).copied().unwrap_or(0)
    }

    /// C++ `SourceFileIndexer::getFilename` (empty on a miss, mirroring
    /// `map::operator[]`'s default-constructed `string`).
    pub fn get_filename(&self, index: i32) -> Vec<u8> {
        self.index_to_file.get(&index).cloned().unwrap_or_default()
    }

    /// C++ `SourceFileIndexer::decode`.
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let el = decoder.open_element_id(&sla::ELEM_SOURCEFILES)?;
        while decoder.peek_element()? == sla::ELEM_SOURCEFILE {
            let subel = decoder.open_element()?;
            let filename = decoder.read_string_id(&sla::ATTRIB_NAME)?;
            let index = decoder.read_signed_integer_id(&sla::ATTRIB_INDEX)? as i32; // C++ implicit intb -> int4
            decoder.close_element(subel)?;
            self.file_to_index.insert(filename.clone(), index);
            self.index_to_file.insert(index, filename);
            // C++ `decode` does not touch `leastUnusedIndex`; that is harmless
            // upstream because `encode` only ever runs on a compiler-populated
            // indexer (built via `index()`).  kuna's WS5 round-trip *does*
            // re-encode a decoded indexer, and `encode` iterates
            // `0..leastUnusedIndex` -- so keep the one-up count consistent with
            // the restored indices (the contiguous `0..=max` invariant the
            // compiler maintains).  This re-establishes the invariant without
            // altering any compiler-path behavior. (kuna)
            if index >= self.least_unused_index {
                self.least_unused_index = index + 1;
            }
        }
        decoder.close_element(el)
    }

    /// C++ `SourceFileIndexer::encode`.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&sla::ELEM_SOURCEFILES);
        for i in 0..self.least_unused_index {
            encoder.open_element(&sla::ELEM_SOURCEFILE);
            // C++ `indexToFile.at(i)`; a missing key would throw, mirrored as
            // an empty default here (the encode invariant fills 0..least).
            encoder.write_string(
                &sla::ATTRIB_NAME,
                self.index_to_file.get(&i).map(|v| v.as_slice()).unwrap_or(b""),
            );
            encoder.write_signed_integer(&sla::ATTRIB_INDEX, i64::from(i));
            encoder.close_element(&sla::ELEM_SOURCEFILE);
        }
        encoder.close_element(&sla::ELEM_SOURCEFILES);
    }
}

/// C++ `SleighBase::MAX_UNIQUE_SIZE`.
pub const MAX_UNIQUE_SIZE: u32 = 256;

/// C++ `SleighBase`: the common core of SLEIGH spec readers/writers.
pub struct SleighBase {
    /// The address spaces (C++ inherited `AddrSpaceManager`).
    ///
    /// Held behind an [`Rc`] so the single space set the SLEIGH lift populates
    /// can be **shared** with the [`Architecture`] / `Funcdata::glb`
    /// (LOSS-132 unification): the C++ `Architecture` *is-a* `AddrSpaceManager`,
    /// so there is exactly one manager and the lifted varnodes, the
    /// architecture, and every analysis pass key state by the same
    /// `Rc<AddrSpace>` identities and indices.  Mutated (via [`Rc::get_mut`])
    /// only during initialization — `decodeSlaSpaces` here and the
    /// `Architecture::restoreFromSpec` fspec/iop/join insert — while the engine
    /// is still the sole `Rc` owner; thereafter every access is `&self`
    /// (join-record creation goes through the manager's interior `RefCell`).
    pub(crate) manager: Rc<AddrSpaceManager>,
    /// The concrete `Translate` base-class state (C++ inherited `Translate`).
    pub(crate) base: TranslateBase,
    /// Names of user-defined p-code ops (C++ `userop`).
    userop: Vec<Vec<u8>>,
    /// Register-space Varnode -> register name map (C++ `varnode_xref`),
    /// keyed by the kuna-base storage triple in `VarnodeData::operator<`
    /// order.
    varnode_xref: std::collections::BTreeMap<VarnodeStorage, Vec<u8>>,
    /// The root SLEIGH decoding symbol id (C++ `SubtableSymbol *root`).
    pub(crate) root: Option<u32>,
    /// The SLEIGH symbol table (C++ `symtab`).
    pub(crate) symtab: SymbolTable,
    /// Backing store for `ConstructTpl` (the `ConstructTplHandle` index space).
    pub(crate) templates: Vec<ConstructTpl>,
    /// C++ `maxdelayslotbytes`.
    pub(crate) maxdelayslotbytes: u32,
    /// C++ `unique_allocatemask`.
    pub(crate) unique_allocatemask: u32,
    /// C++ `numSections`.
    pub(crate) num_sections: u32,
    /// C++ `indexer`.
    indexer: SourceFileIndexer,
    /// The marshal id registry used to decode `.sla` (kuna seam for the C++
    /// global id registration).
    pub(crate) registry: IdRegistry,
}

impl SleighBase {
    /// C++ `SleighBase()` — an uninitialized translator.
    pub fn new() -> SleighBase {
        // The id registry maps element/attribute *names* to ids; only the XML
        // protocol consults it (`.sla` is packed and uses numeric ids
        // directly).  Register the full table anyway so XML-format callers
        // share the same id space.
        let mut registry = IdRegistry::with_base_ids();
        crate::translate::register_translate_ids(&mut registry);
        crate::globalcontext::register_globalcontext_ids(&mut registry);
        sla::register_sla_ids(&mut registry);
        SleighBase {
            manager: Rc::new(AddrSpaceManager::new()),
            base: TranslateBase::new(),
            userop: Vec::new(),
            varnode_xref: std::collections::BTreeMap::new(),
            root: None,
            symtab: SymbolTable::new(),
            templates: Vec::new(),
            maxdelayslotbytes: 0,
            unique_allocatemask: 0,
            num_sections: 0,
            indexer: SourceFileIndexer::new(),
            registry,
        }
    }

    /// C++ `SleighBase::isInitialized`.
    pub fn is_initialized(&self) -> bool {
        self.root.is_some()
    }

    /// C++ `SleighBase::findSymbol(const string&)`.
    pub fn find_symbol(&self, nm: &[u8]) -> Option<&crate::slghsymbol::SleighSymbol> {
        self.symtab.find_symbol(nm)
    }

    /// C++ `SleighBase::findSymbol(uintm id)`.
    pub fn find_symbol_by_id(&self, id: u32) -> Option<&crate::slghsymbol::SleighSymbol> {
        self.symtab.find_symbol_by_id(id)
    }

    /// C++ `SleighBase::findGlobalSymbol`.
    pub fn find_global_symbol(&self, nm: &[u8]) -> Option<&crate::slghsymbol::SleighSymbol> {
        self.symtab.find_global_symbol(nm)
    }

    /// C++ `SleighBase::buildXrefs`: iterate the global scope collecting
    /// registers (for the map), user-op names, and context fields.  The
    /// `registerContext` callback is delivered via the supplied closure
    /// (the engine wires it to its `ContextDatabase`).
    pub(crate) fn build_xrefs(
        &mut self,
        error_pairs: &mut Vec<Vec<u8>>,
        mut register_context: impl FnMut(&[u8], i32, i32) -> KunaResult<()>,
    ) -> KunaResult<()> {
        // Collect the global-scope symbol ids in scope (BTreeMap) order, the
        // C++ `SymbolTree` (set<SleighSymbol*,SymbolCompare>) iteration order.
        let glb = self
            .symtab
            .get_global_scope()
            .ok_or_else(|| KunaError::sleigh("symbol table has no global scope"))?;
        let ids: Vec<u32> = glb.symbol_ids().collect();
        for sym_id in ids {
            let sym = self
                .symtab
                .find_symbol_by_id(sym_id)
                .ok_or_else(|| KunaError::sleigh("undefined global symbol"))?;
            let name = sym.get_name().to_vec();
            match sym.kind() {
                SymbolKind::Varnode(v) => {
                    let key = storage_from_varnode_data(v.get_fixed_varnode());
                    if let Some(existing) = self.varnode_xref.get(&key) {
                        error_pairs.push(name);
                        error_pairs.push(existing.clone());
                    } else {
                        self.varnode_xref.insert(key, name);
                    }
                }
                SymbolKind::UserOp(u) => {
                    let index = u.get_index() as usize; // C++ int4 index >= 0
                    while self.userop.len() <= index {
                        self.userop.push(Vec::new());
                    }
                    self.userop[index] = name;
                }
                SymbolKind::Context(_) => {
                    let (sb, eb) = context_field_bits(sym)?;
                    register_context(&name, sb, eb)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// C++ `SleighBase::reregisterContext`.
    pub(crate) fn reregister_context(
        &self,
        mut register_context: impl FnMut(&[u8], i32, i32) -> KunaResult<()>,
    ) -> KunaResult<()> {
        let glb = self
            .symtab
            .get_global_scope()
            .ok_or_else(|| KunaError::sleigh("symbol table has no global scope"))?;
        let ids: Vec<u32> = glb.symbol_ids().collect();
        for sym_id in ids {
            let sym = self
                .symtab
                .find_symbol_by_id(sym_id)
                .ok_or_else(|| KunaError::sleigh("undefined global symbol"))?;
            if sym.get_type() == SymbolType::Context {
                let (sb, eb) = context_field_bits(sym)?;
                register_context(sym.get_name(), sb, eb)?;
            }
        }
        Ok(())
    }

    /// C++ `SleighBase::getRegister(const string&)`.
    pub fn get_register(&self, nm: &[u8]) -> KunaResult<VarnodeData> {
        let sym = self
            .symtab
            .find_symbol(nm)
            .ok_or_else(|| KunaError::sleigh(format!("Unknown register name: {}", lossy(nm))))?;
        match sym.kind() {
            SymbolKind::Varnode(v) => Ok(v.get_fixed_varnode().clone()),
            _ => Err(KunaError::sleigh(format!("Symbol is not a register: {}", lossy(nm)))),
        }
    }

    /// C++ `SleighBase::getRegisterName(AddrSpace*,uintb,int4)`.
    pub fn get_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: i32) -> Vec<u8> {
        register_name_from_xref(&self.varnode_xref, base, off, size)
    }

    /// C++ `SleighBase::getExactRegisterName`.
    pub fn get_exact_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: i32) -> Vec<u8> {
        exact_register_name_from_xref(&self.varnode_xref, base, off, size)
    }

    /// C++ `SleighBase::getAllRegisters`.
    // mutable_key_type: VarnodeStorage's interior-mutable space flags/shortcut
    // Cells do not participate in its Ord (space index, offset, size only), so
    // a key's order cannot change while it is in the map (same justification as
    // translate.rs get_all_registers / loadimage.rs).
    #[allow(clippy::mutable_key_type)]
    pub fn get_all_registers(&self) -> &std::collections::BTreeMap<VarnodeStorage, Vec<u8>> {
        &self.varnode_xref
    }

    /// C++ `SleighBase::getUserOpNames`.
    pub fn get_user_op_names(&self) -> &[Vec<u8>] {
        &self.userop
    }

    /// C++ `SleighBase::decodeSlaSpace`: add a space parsed from a `.sla`.
    /// `big_end_default` carries the processor endianness the C++ space
    /// constructors read from the (here-absent) Translate back-pointer.
    fn decode_sla_space(
        &mut self,
        decoder: &mut dyn Decoder,
        big_end_default: bool,
    ) -> KunaResult<Rc<AddrSpace>> {
        let elem_id = decoder.open_element()?;
        let mut index: i32 = 0;
        let mut address_size: i32 = 0;
        let mut delay: i32 = -1;
        let mut name: Vec<u8> = Vec::new();
        let mut wordsize: i32 = 1;
        let mut big_end = false;
        let mut flags: u32 = 0;
        loop {
            let attrib_id = decoder.get_next_attribute_id()?;
            if attrib_id == 0 {
                break;
            }
            if attrib_id == sla::ATTRIB_NAME {
                name = decoder.read_string()?;
            }
            // C++ ladder: first `if (NAME)`, then `if (INDEX) ... else if (...)`
            if attrib_id == sla::ATTRIB_INDEX {
                index = decoder.read_signed_integer()? as i32; // C++ implicit intb -> int4
            } else if attrib_id == sla::ATTRIB_SIZE {
                address_size = decoder.read_signed_integer()? as i32;
            } else if attrib_id == sla::ATTRIB_WORDSIZE {
                wordsize = decoder.read_signed_integer()? as i32;
            } else if attrib_id == sla::ATTRIB_BIGENDIAN {
                big_end = decoder.read_bool()?;
            } else if attrib_id == sla::ATTRIB_DELAY {
                delay = decoder.read_signed_integer()? as i32;
            } else if attrib_id == sla::ATTRIB_PHYSICAL && decoder.read_bool()? {
                flags |= addrspace_flags::hasphysical;
            }
        }
        decoder.close_element(elem_id)?;
        // C++ inits deadcodedelay = -1 and, since the sla space format never
        // carries it, the `if (deadcodedelay == -1)` always sets it to delay.
        let deadcodedelay = delay;
        if index == 0 {
            return Err(KunaError::lowlevel("Expecting index attribute"));
        }
        let res = if elem_id == sla::ELEM_SPACE_UNIQUE {
            // C++ new UniqueSpace(this,trans,index,flags): endianness from
            // the processor (Translate) — here threaded as big_end_default.
            UniqueSpace::new(index, flags, big_end_default)
        } else if elem_id == sla::ELEM_SPACE_OTHER {
            OtherSpace::new(index)
        } else {
            if address_size == 0 || delay == -1 || name.is_empty() {
                return Err(KunaError::lowlevel("Expecting size/delay/name attributes"));
            }
            AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                &lossy(&name),
                big_end,
                address_size as u32, // C++ int4 -> uint4 (validated non-zero)
                wordsize as u32,
                index,
                flags,
                delay,
                deadcodedelay,
            )
        };
        Ok(Rc::new(res))
    }

    /// C++ `SleighBase::decodeSlaSpaces`.  `big_end` is the processor
    /// endianness (already set on `self.base` before this call).
    fn decode_sla_spaces(&mut self, decoder: &mut dyn Decoder, big_end: bool) -> KunaResult<()> {
        // The shared manager is mutated only during this initialization phase,
        // while the engine is still the sole `Rc` owner (no `glb` has cloned it
        // yet).  `Rc::get_mut` therefore always succeeds; a `None` would mean an
        // `Architecture`/`Funcdata` was wired before the `.sla` decoded, which
        // the build flow forbids.  Each mutation re-borrows because the
        // surrounding decode helpers take `&self`.
        // The first space should always be the constant space.
        manager_get_mut(&mut self.manager).insert_space(Rc::new(ConstantSpace::new()))?;

        let elem_id = decoder.open_element_id(&sla::ELEM_SPACES)?;
        let defname = decoder.read_string_id(&sla::ATTRIB_DEFAULTSPACE)?;
        while decoder.peek_element()? != 0 {
            let spc = self.decode_sla_space(decoder, big_end)?;
            // Re-borrow per iteration: `decode_sla_space` takes `&self`.
            manager_get_mut(&mut self.manager).insert_space(spc)?;
        }
        decoder.close_element(elem_id)?;
        let spc_index = match self.manager.get_space_by_name(&lossy(&defname)) {
            Some(spc) => spc.get_index(),
            None => {
                return Err(KunaError::lowlevel(format!(
                    "Bad 'defaultspace' attribute: {}",
                    lossy(&defname)
                )))
            }
        };
        manager_get_mut(&mut self.manager).set_default_code_space(spc_index)?;
        Ok(())
    }

    /// C++ `SleighBase::decode`: parse the main `<sleigh>` tag from a `.sla`.
    /// `register_context` is the engine's `ContextDatabase::registerVariable`
    /// callback (invoked by `buildXrefs`).
    pub fn decode(
        &mut self,
        decoder: &mut dyn OpcodeDecoder,
        register_context: impl FnMut(&[u8], i32, i32) -> KunaResult<()>,
    ) -> KunaResult<()> {
        self.maxdelayslotbytes = 0;
        self.unique_allocatemask = 0;
        self.num_sections = 0;
        let mut version: i32 = 0;
        let el = decoder.open_element_id(&sla::ELEM_SLEIGH)?;
        let mut attrib = decoder.get_next_attribute_id()?;
        while attrib != 0 {
            if attrib == sla::ATTRIB_BIGENDIAN {
                let be = decoder.read_bool()?;
                self.base.set_big_endian(be);
            } else if attrib == sla::ATTRIB_ALIGN {
                self.base.alignment = decoder.read_signed_integer()? as i32; // C++ intb -> int4
            } else if attrib == sla::ATTRIB_UNIQBASE {
                let ub = decoder.read_unsigned_integer()? as u32; // C++ uintb -> uint4
                self.base.set_unique_base(ub);
            } else if attrib == sla::ATTRIB_MAXDELAY {
                self.maxdelayslotbytes = decoder.read_unsigned_integer()? as u32;
            } else if attrib == sla::ATTRIB_UNIQMASK {
                self.unique_allocatemask = decoder.read_unsigned_integer()? as u32;
            } else if attrib == sla::ATTRIB_NUMSECTIONS {
                self.num_sections = decoder.read_unsigned_integer()? as u32;
            } else if attrib == sla::ATTRIB_VERSION {
                version = decoder.read_signed_integer()? as i32;
            }
            attrib = decoder.get_next_attribute_id()?;
        }
        if version != sla::FORMAT_VERSION {
            return Err(KunaError::lowlevel(".sla file has wrong format"));
        }
        let big_end = self.base.is_big_endian();
        self.indexer.decode(decoder)?;
        self.decode_sla_spaces(decoder, big_end)?;

        // SymbolTable::decode needs the SleighBaseTrans seam disjoint from the
        // &mut symtab borrow.  Move the symtab and templates out, decode, and
        // restore (the constant space is cloned out of the manager first).
        let const_space = self
            .manager
            .get_constant_space()
            .cloned()
            .ok_or_else(|| KunaError::sleigh("constant space not registered"))?;
        let mut symtab = std::mem::take(&mut self.symtab);
        let mut templates = std::mem::take(&mut self.templates);
        let decode_res = {
            let mut trans = SlaTrans { const_space: const_space.clone(), templates: &mut templates };
            symtab.decode(decoder, &mut trans)
        };
        self.symtab = symtab;
        self.templates = templates;
        decode_res?;

        decoder.close_element(el)?;
        // root = symtab.getGlobalScope()->findSymbol("instruction")
        self.root = self
            .symtab
            .get_global_scope()
            .and_then(|s| s.find_symbol(b"instruction"));
        let mut error_pairs: Vec<Vec<u8>> = Vec::new();
        self.build_xrefs(&mut error_pairs, register_context)?;
        if !error_pairs.is_empty() {
            return Err(KunaError::sleigh("Duplicate register pairs"));
        }
        Ok(())
    }

    /// C++ `SleighBase::encodeSlaSpace` (sleighbase.cc:197-225): write one
    /// `<space>`/`<space_other>`/`<space_unique>` element fully describing an
    /// address space.  Mirrors the C++ attribute order exactly (name, index,
    /// bigendian, delay, size, optional wordsize, physical).
    fn encode_sla_space(&self, encoder: &mut dyn Encoder, spc: &AddrSpace) {
        let elem = if spc.get_type() == spacetype::IPTR_INTERNAL {
            &sla::ELEM_SPACE_UNIQUE
        } else if spc.is_other_space() {
            &sla::ELEM_SPACE_OTHER
        } else {
            &sla::ELEM_SPACE
        };
        encoder.open_element(elem);
        encoder.write_string(&sla::ATTRIB_NAME, spc.get_name().as_bytes());
        encoder.write_signed_integer(&sla::ATTRIB_INDEX, i64::from(spc.get_index()));
        encoder.write_bool(&sla::ATTRIB_BIGENDIAN, self.base.is_big_endian());
        encoder.write_signed_integer(&sla::ATTRIB_DELAY, i64::from(spc.get_delay()));
        encoder.write_signed_integer(&sla::ATTRIB_SIZE, i64::from(spc.get_addr_size()));
        if spc.get_word_size() > 1 {
            encoder.write_signed_integer(&sla::ATTRIB_WORDSIZE, i64::from(spc.get_word_size()));
        }
        encoder.write_bool(&sla::ATTRIB_PHYSICAL, spc.has_physical());
        encoder.close_element(elem);
    }

    /// C++ `SleighBase::encode` (sleighbase.cc:226-255): write the whole `.sla`
    /// element stream.  Opens `<sleigh>`, writes the header attributes
    /// (version/bigendian/align/uniqbase + the optional maxdelay/uniqmask/
    /// numsections), then the source-file indexer, the `<spaces>` block (skipping
    /// the internal constant/fspec/iop/join spaces), then the symbol table.
    ///
    /// This is the top-level orchestrator the SLEIGH compiler calls at the end of
    /// `run_compilation`.  Every sub-part (`SourceFileIndexer::encode`,
    /// `SymbolTable::encode`, and the per-symbol/pattern/semantics `encode`
    /// methods underneath) was already ported for the decoder round-trip and is
    /// reused verbatim.
    pub fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        encoder.open_element(&sla::ELEM_SLEIGH);
        encoder.write_signed_integer(&sla::ATTRIB_VERSION, i64::from(sla::FORMAT_VERSION));
        encoder.write_bool(&sla::ATTRIB_BIGENDIAN, self.base.is_big_endian());
        encoder.write_signed_integer(&sla::ATTRIB_ALIGN, i64::from(self.base.alignment));
        encoder
            .write_unsigned_integer(&sla::ATTRIB_UNIQBASE, u64::from(self.base.get_unique_base()));
        if self.maxdelayslotbytes > 0 {
            encoder
                .write_unsigned_integer(&sla::ATTRIB_MAXDELAY, u64::from(self.maxdelayslotbytes));
        }
        if self.unique_allocatemask != 0 {
            encoder
                .write_unsigned_integer(&sla::ATTRIB_UNIQMASK, u64::from(self.unique_allocatemask));
        }
        if self.num_sections != 0 {
            encoder
                .write_unsigned_integer(&sla::ATTRIB_NUMSECTIONS, u64::from(self.num_sections));
        }
        self.indexer.encode(encoder);

        encoder.open_element(&sla::ELEM_SPACES);
        let defaultspace = self
            .manager
            .get_default_code_space()
            .ok_or_else(|| KunaError::sleigh("no default code space to encode"))?
            .get_name()
            .as_bytes()
            .to_vec();
        encoder.write_string(&sla::ATTRIB_DEFAULTSPACE, &defaultspace);
        let n = self.manager.num_spaces();
        for i in 0..n {
            let spc = match self.manager.get_space(i) {
                None => continue,
                Some(s) => Rc::clone(s),
            };
            match spc.get_type() {
                spacetype::IPTR_CONSTANT
                | spacetype::IPTR_FSPEC
                | spacetype::IPTR_IOP
                | spacetype::IPTR_JOIN => continue,
                _ => {}
            }
            self.encode_sla_space(encoder, &spc);
        }
        encoder.close_element(&sla::ELEM_SPACES);

        // SymbolTable::encode needs the SleighBaseTrans seam (for the per-section
        // ConstructTpl encode).  The seam borrows the template store; encode only
        // reads it, but SlaTrans holds `&mut`, so clone the store to satisfy the
        // signature (encode never mutates the templates).
        let const_space = self
            .manager
            .get_constant_space()
            .cloned()
            .ok_or_else(|| KunaError::sleigh("constant space not registered"))?;
        let mut templates = self.templates.clone();
        let trans = SlaTrans { const_space, templates: &mut templates };
        self.symtab.encode(encoder, &trans)?;

        encoder.close_element(&sla::ELEM_SLEIGH);
        Ok(())
    }
}

impl Default for SleighBase {
    fn default() -> Self {
        SleighBase::new()
    }
}

/// Lossy UTF-8 view of a `.sla` byte-string name (identifiers are ASCII).
fn lossy(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

/// Mutably borrow the shared manager during initialization (LOSS-132).  The
/// space set is mutated only while the engine is the sole `Rc` owner (before
/// any `glb` clones it), so `Rc::get_mut` always succeeds; a `None` is a build
/// ordering bug (the manager was shared too early).
fn manager_get_mut(manager: &mut Rc<AddrSpaceManager>) -> &mut AddrSpaceManager {
    Rc::get_mut(manager)
        .expect("engine address-space manager already shared (Rc not unique)")
}

/// C++ `ContextSymbol::getPatternValue()` -> `ContextField`'s start/end bits
/// (`buildXrefs`/`reregisterContext`).  The cast is unchecked in C++; the
/// Rust errors if the pattern value is not the expected `ContextField`.
fn context_field_bits(sym: &SleighSymbol) -> KunaResult<(i32, i32)> {
    match sym.get_pattern_value() {
        Some(PatternValue::ContextField(field)) => {
            Ok((field.get_start_bit(), field.get_end_bit()))
        }
        _ => Err(KunaError::sleigh("context symbol has no context field")),
    }
}

/// C++ raw-pointer equality over a possibly-null space and a non-null base.
fn space_eq(a: &Option<Rc<AddrSpace>>, base: &Rc<AddrSpace>) -> bool {
    matches!(a, Some(s) if Rc::ptr_eq(s, base))
}

/// The register cross-reference map type (location -> register name).
type VarnodeXref = std::collections::BTreeMap<VarnodeStorage, Vec<u8>>;

/// C++ `SleighBase::getRegisterName` over a register cross-reference map (the
/// `varnode_xref` location->name table).  Factored out so both [`SleighBase`]
/// and [`SnapshotRegisterLookup`] resolve names identically.
#[allow(clippy::mutable_key_type)]
fn register_name_from_xref(
    xref: &VarnodeXref,
    base: &Rc<AddrSpace>,
    off: u64,
    size: i32,
) -> Vec<u8> {
    // C++ builds the lookup key VarnodeData{space=base,offset=off,size}.
    let key = VarnodeStorage {
        space: Some(Rc::clone(base)),
        offset: off,
        size: size as u32, // C++ assigns int4 -> uint4 (size is non-negative)
    };
    // C++:
    //   iter = upper_bound(key);   // first key strictly greater than `key`
    //   if (iter == begin()) return "";
    //   iter--;                    // greatest element <= key
    // `--upper_bound(key)` is the greatest element <= key, so the starting
    // iterator is `range(.., Included(&key)).next_back()`.  (Using Excluded
    // here would skip an exact match — the F1 bug.)  `next_back() == None`
    // reproduces `iter == begin()` (nothing is <= key).
    let mut prev_iter = xref.range((std::ops::Bound::Unbounded, std::ops::Bound::Included(&key)));
    let Some((point, name)) = prev_iter.next_back() else {
        return Vec::new();
    };
    if !space_eq(&point.space, base) {
        return Vec::new();
    }
    let offbase = point.offset;
    // C++ `point.offset + point.size >= off + size`
    if point.offset.wrapping_add(u64::from(point.size)) >= off.wrapping_add(size as u64) {
        return name.clone();
    }
    // Walk back through same-base, same-offset entries.
    let mut back = xref.range((std::ops::Bound::Unbounded, std::ops::Bound::Excluded(point)));
    while let Some((p, n)) = back.next_back() {
        if !space_eq(&p.space, base) || p.offset != offbase {
            return Vec::new();
        }
        if p.offset.wrapping_add(u64::from(p.size)) >= off.wrapping_add(size as u64) {
            return n.clone();
        }
    }
    Vec::new()
}

/// C++ `SleighBase::getExactRegisterName` over a register cross-reference map.
#[allow(clippy::mutable_key_type)]
fn exact_register_name_from_xref(
    xref: &VarnodeXref,
    base: &Rc<AddrSpace>,
    off: u64,
    size: i32,
) -> Vec<u8> {
    let key = VarnodeStorage {
        space: Some(Rc::clone(base)),
        offset: off,
        size: size as u32, // C++ int4 -> uint4
    };
    xref.get(&key).cloned().unwrap_or_default()
}

/// A standalone [`RegisterLookup`] snapshot built from the engine's register
/// cross-reference (the `varnode_xref` location->name map).  This is installed
/// on the engine's [`AddrSpaceManager`] (the kuna stand-in for the C++
/// `AddrSpace::trans` back-pointer) so the `<context_data>`/`<tracked_set>`
/// spec decode — and any later `Translate::getRegister`-by-name path — resolves
/// register names without an `Rc` cycle back into the owning [`Sleigh`].
///
/// It resolves names exactly as [`SleighBase`] does (the same factored
/// algorithms); the name->storage direction is the inverse of the same map.
#[derive(Debug)]
pub struct SnapshotRegisterLookup {
    /// location -> register name (clone of the engine `varnode_xref`).
    #[allow(clippy::mutable_key_type)]
    xref: VarnodeXref,
    /// register name -> storage (inverse of `xref`, for `getRegister`).
    by_name: std::collections::BTreeMap<Vec<u8>, VarnodeStorage>,
}

impl SnapshotRegisterLookup {
    /// Build the snapshot from a [`SleighBase`]'s register cross-reference.
    #[allow(clippy::mutable_key_type)]
    pub fn from_base(base: &SleighBase) -> SnapshotRegisterLookup {
        let xref = base.get_all_registers().clone();
        let mut by_name = std::collections::BTreeMap::new();
        for (storage, name) in xref.iter() {
            by_name.insert(name.clone(), storage.clone());
        }
        SnapshotRegisterLookup { xref, by_name }
    }
}

impl RegisterLookup for SnapshotRegisterLookup {
    fn get_register(&self, nm: &str) -> KunaResult<VarnodeStorage> {
        // C++ `SleighBase::getRegister` throws SleighError on an unknown name.
        self.by_name
            .get(nm.as_bytes())
            .cloned()
            .ok_or_else(|| KunaError::sleigh(format!("Unknown register name: {nm}")))
    }

    fn get_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: i32) -> String {
        String::from_utf8_lossy(&register_name_from_xref(&self.xref, base, off, size)).into_owned()
    }

    fn get_exact_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: i32) -> String {
        String::from_utf8_lossy(&exact_register_name_from_xref(&self.xref, base, off, size))
            .into_owned()
    }
}

/// Short-lived [`SleighBaseTrans`] seam used during `SymbolTable::decode`:
/// supplies the constant space and decodes/encodes `ConstructTpl` sections
/// into the template store.  Borrows the store mutably so it stays disjoint
/// from the `&mut symtab` borrow.
struct SlaTrans<'a> {
    const_space: Rc<AddrSpace>,
    templates: &'a mut Vec<ConstructTpl>,
}

impl SleighBaseTrans for SlaTrans<'_> {
    fn get_constant_space(&self) -> Rc<AddrSpace> {
        Rc::clone(&self.const_space)
    }

    fn decode_construct_tpl(
        &mut self,
        decoder: &mut dyn Decoder,
    ) -> KunaResult<(i32, ConstructTplHandle)> {
        // ConstructTpl::decode needs the OpcodeDecoder surface; the seam only
        // hands a &mut dyn Decoder, so wrap it in a shim that reads opcodes
        // via the packed protocol (signed integer; the .sla format).
        let mut tpl = ConstructTpl::new();
        let mut shim = OpcodeShim { inner: decoder };
        let sectionid = tpl.decode(&mut shim)?;
        let handle = self.templates.len();
        self.templates.push(tpl);
        Ok((sectionid, handle))
    }

    fn encode_construct_tpl(
        &self,
        handle: ConstructTplHandle,
        section_id: i32,
        encoder: &mut dyn Encoder,
    ) -> KunaResult<()> {
        let tpl = self
            .templates
            .get(handle)
            .ok_or_else(|| KunaError::sleigh("bad ConstructTpl handle"))?;
        let mut shim = OpcodeEncodeShim { inner: encoder };
        tpl.encode(&mut shim, section_id);
        Ok(())
    }
}

/// Wraps a `&mut dyn Decoder` and supplies the [`OpcodeDecoder`] surface by
/// reading opcodes through the packed protocol (a positive signed integer
/// holding the raw enum value, marshal.cc `PackedDecode::readOpcode`).  The
/// `.sla` stream is always packed, so this matches `PackedDecode::readOpcode`
/// exactly.
struct OpcodeShim<'a, 'b> {
    inner: &'a mut (dyn Decoder + 'b),
}

/// C++ `opcode_from_packed_integer` (marshal.cc): the packed `readOpcode`
/// body, reproduced because the helper is private to kuna-num.
fn opcode_from_packed_integer(raw: i64) -> KunaResult<OpCode> {
    let val = raw as i32; // cast: C++ `(int4)readSignedInteger()` truncation
    if val < 0 || val >= OpCode::CPUI_MAX as i32 {
        return Err(KunaError::decoder("Bad encoded OpCode"));
    }
    OpCode::from_i32(val).ok_or_else(|| KunaError::decoder("Bad encoded OpCode"))
}

impl OpcodeDecoder for OpcodeShim<'_, '_> {
    fn read_opcode(&mut self) -> KunaResult<OpCode> {
        opcode_from_packed_integer(self.inner.read_signed_integer()?)
    }
    fn read_opcode_id(
        &mut self,
        attrib_id: &kuna_base::marshal::AttributeId,
    ) -> KunaResult<OpCode> {
        opcode_from_packed_integer(self.inner.read_signed_integer_id(attrib_id)?)
    }
}

/// Encoder counterpart of [`OpcodeShim`] (the packed `writeOpcode` body:
/// a signed integer holding the raw enum value).
struct OpcodeEncodeShim<'a, 'b> {
    inner: &'a mut (dyn Encoder + 'b),
}

impl OpcodeEncoder for OpcodeEncodeShim<'_, '_> {
    fn write_opcode(&mut self, attrib_id: &kuna_base::marshal::AttributeId, opc: OpCode) {
        // marshal.cc `PackedEncode::writeOpcode`: write the enum as a signed
        // integer.
        self.inner.write_signed_integer(attrib_id, opc as i64);
    }
}

// Forward the Decoder surface from the shims to the inner decoder.
macro_rules! forward_decoder {
    ($t:ty) => {
        impl Decoder for $t {
            fn get_addr_space_manager(&self) -> &AddrSpaceManager {
                self.inner.get_addr_space_manager()
            }
            fn ingest_stream(&mut self, s: &[u8]) -> KunaResult<()> {
                self.inner.ingest_stream(s)
            }
            fn peek_element(&mut self) -> KunaResult<u32> {
                self.inner.peek_element()
            }
            fn open_element(&mut self) -> KunaResult<u32> {
                self.inner.open_element()
            }
            fn open_element_id(
                &mut self,
                elem_id: &kuna_base::marshal::ElementId,
            ) -> KunaResult<u32> {
                self.inner.open_element_id(elem_id)
            }
            fn close_element(&mut self, id: u32) -> KunaResult<()> {
                self.inner.close_element(id)
            }
            fn close_element_skipping(&mut self, id: u32) -> KunaResult<()> {
                self.inner.close_element_skipping(id)
            }
            fn get_next_attribute_id(&mut self) -> KunaResult<u32> {
                self.inner.get_next_attribute_id()
            }
            fn get_indexed_attribute_id(
                &mut self,
                attrib_id: &kuna_base::marshal::AttributeId,
            ) -> KunaResult<u32> {
                self.inner.get_indexed_attribute_id(attrib_id)
            }
            fn rewind_attributes(&mut self) {
                self.inner.rewind_attributes()
            }
            fn read_bool(&mut self) -> KunaResult<bool> {
                self.inner.read_bool()
            }
            fn read_bool_id(
                &mut self,
                attrib_id: &kuna_base::marshal::AttributeId,
            ) -> KunaResult<bool> {
                self.inner.read_bool_id(attrib_id)
            }
            fn read_signed_integer(&mut self) -> KunaResult<i64> {
                self.inner.read_signed_integer()
            }
            fn read_signed_integer_id(
                &mut self,
                attrib_id: &kuna_base::marshal::AttributeId,
            ) -> KunaResult<i64> {
                self.inner.read_signed_integer_id(attrib_id)
            }
            fn read_signed_integer_expect_string(
                &mut self,
                expect: &[u8],
                expectval: i64,
            ) -> KunaResult<i64> {
                self.inner.read_signed_integer_expect_string(expect, expectval)
            }
            fn read_signed_integer_expect_string_id(
                &mut self,
                attrib_id: &kuna_base::marshal::AttributeId,
                expect: &[u8],
                expectval: i64,
            ) -> KunaResult<i64> {
                self.inner.read_signed_integer_expect_string_id(attrib_id, expect, expectval)
            }
            fn read_unsigned_integer(&mut self) -> KunaResult<u64> {
                self.inner.read_unsigned_integer()
            }
            fn read_unsigned_integer_id(
                &mut self,
                attrib_id: &kuna_base::marshal::AttributeId,
            ) -> KunaResult<u64> {
                self.inner.read_unsigned_integer_id(attrib_id)
            }
            fn read_string(&mut self) -> KunaResult<Vec<u8>> {
                self.inner.read_string()
            }
            fn read_string_id(
                &mut self,
                attrib_id: &kuna_base::marshal::AttributeId,
            ) -> KunaResult<Vec<u8>> {
                self.inner.read_string_id(attrib_id)
            }
            fn read_space(&mut self) -> KunaResult<Rc<AddrSpace>> {
                self.inner.read_space()
            }
            fn read_space_id(
                &mut self,
                attrib_id: &kuna_base::marshal::AttributeId,
            ) -> KunaResult<Rc<AddrSpace>> {
                self.inner.read_space_id(attrib_id)
            }
        }
    };
}

forward_decoder!(OpcodeShim<'_, '_>);

// Forward the Encoder surface from the encode shim to the inner encoder.
impl Encoder for OpcodeEncodeShim<'_, '_> {
    fn open_element(&mut self, elem_id: &kuna_base::marshal::ElementId) {
        self.inner.open_element(elem_id)
    }
    fn close_element(&mut self, elem_id: &kuna_base::marshal::ElementId) {
        self.inner.close_element(elem_id)
    }
    fn write_bool(&mut self, attrib_id: &kuna_base::marshal::AttributeId, val: bool) {
        self.inner.write_bool(attrib_id, val)
    }
    fn write_signed_integer(&mut self, attrib_id: &kuna_base::marshal::AttributeId, val: i64) {
        self.inner.write_signed_integer(attrib_id, val)
    }
    fn write_unsigned_integer(&mut self, attrib_id: &kuna_base::marshal::AttributeId, val: u64) {
        self.inner.write_unsigned_integer(attrib_id, val)
    }
    fn write_string(&mut self, attrib_id: &kuna_base::marshal::AttributeId, val: &[u8]) {
        self.inner.write_string(attrib_id, val)
    }
    fn write_string_indexed(
        &mut self,
        attrib_id: &kuna_base::marshal::AttributeId,
        index: u32,
        val: &[u8],
    ) {
        self.inner.write_string_indexed(attrib_id, index, val)
    }
    fn write_space(&mut self, attrib_id: &kuna_base::marshal::AttributeId, spc: &AddrSpace) {
        self.inner.write_space(attrib_id, spc)
    }
}

impl SleighBase {
    /// Convenience: the manager owning the spaces (for the engine).
    pub fn manager(&self) -> &AddrSpaceManager {
        &self.manager
    }

    /// Share the single address-space manager (LOSS-132): hand out a cloned
    /// [`Rc`] so the `Architecture` / `Funcdata::glb` references the **same**
    /// space set the lift populated (same `Rc<AddrSpace>` identities, same
    /// indices), exactly as the C++ `Architecture` *is* its `AddrSpaceManager`.
    pub fn manager_rc(&self) -> Rc<AddrSpaceManager> {
        Rc::clone(&self.manager)
    }

    /// Mutably borrow the single manager during initialization (the
    /// `Architecture::restoreFromSpec` fspec/iop/join insert).  Requires the
    /// engine still be the sole `Rc` owner — true before any `glb` is built.
    pub fn manager_mut(&mut self) -> &mut AddrSpaceManager {
        manager_get_mut(&mut self.manager)
    }

    /// The marshal id registry (name -> id), for callers decoding the engine's
    /// data in the XML protocol (the `.sla` packed protocol uses numeric ids
    /// directly and does not consult it).
    pub fn registry(&self) -> &IdRegistry {
        &self.registry
    }
}

// ---------------------------------------------------------------------------
// SnippetLanguage (pcodeparse.cc:3215 PcodeSnippet::lex + the SleighBase
// surface PcodeSnippet pulls — getDefaultCodeSpace/getConstantSpace/
// getUniqueSpace/numSpaces/getSpace).  Lets `parse_inject` compile a
// `<callotherfixup>`/`<callfixup>` body against the real loaded language.
// ---------------------------------------------------------------------------

impl crate::pcodeparse::SnippetLanguage for SleighBase {
    /// C++ `sleigh->findSymbol(name)`, classified by `PcodeSnippet::lex`'s
    /// `switch(sym->getType())`.  Returns `None` for an unknown name (the lexer
    /// then falls through to STRING) and for symbol kinds the snippet compiler
    /// deliberately keeps invisible (the C++ `dummy_symbol`/`default` arms).
    fn find_snippet_symbol(&self, name: &[u8]) -> Option<crate::pcodeparse::SnippetSymbol> {
        use crate::pcodeparse::SnippetSymbol;
        let sym = self.find_symbol(name)?;
        match sym.kind() {
            // case space_symbol: yylval.spacesym; return SPACESYM
            SymbolKind::Space(s) => Some(SnippetSymbol::Space(Rc::clone(s.get_space()))),
            // case userop_symbol: yylval.useropsym; return USEROPSYM
            SymbolKind::UserOp(u) => Some(SnippetSymbol::UserOp(u.clone())),
            // case varnode_symbol: yylval.varsym; return VARSYM
            SymbolKind::Varnode(v) => Some(SnippetSymbol::Varnode(v.clone(), name.to_vec())),
            // case operand_symbol: yylval.operandsym; return OPERANDSYM
            SymbolKind::Operand(o) => Some(SnippetSymbol::Operand(o.get_index(), name.to_vec())),
            // case start/end/next2/flowdest/flowref: return JUMPSYM
            SymbolKind::Start(_) => Some(SnippetSymbol::Start(name.to_vec())),
            SymbolKind::End(_) => Some(SnippetSymbol::End(name.to_vec())),
            SymbolKind::Next2(_) => Some(SnippetSymbol::Next2(name.to_vec())),
            SymbolKind::FlowDest(_) => Some(SnippetSymbol::FlowDest(name.to_vec())),
            SymbolKind::FlowRef(_) => Some(SnippetSymbol::FlowRef(name.to_vec())),
            // The translator may have other symbols (value/context/subtable/…)
            // that the snippet compiler does not want visible: the C++ `default`
            // arm falls through to STRING (no special symbol).
            _ => None,
        }
    }

    /// C++ `sleigh->getDefaultCodeSpace()`.
    fn get_default_code_space(&self) -> Rc<AddrSpace> {
        Rc::clone(
            self.manager()
                .get_default_code_space()
                .expect("SnippetLanguage: no default code space"),
        )
    }

    /// C++ `sleigh->getConstantSpace()`.
    fn get_constant_space(&self) -> Rc<AddrSpace> {
        Rc::clone(self.manager().get_constant_space().expect("SnippetLanguage: no constant space"))
    }

    /// C++ `sleigh->getUniqueSpace()`.
    fn get_unique_space(&self) -> Rc<AddrSpace> {
        Rc::clone(self.manager().get_unique_space().expect("SnippetLanguage: no unique space"))
    }

    /// C++ `sleigh->numSpaces()`.
    fn num_spaces(&self) -> i32 {
        self.manager().num_spaces()
    }

    /// C++ `sleigh->getSpace(i)`.
    fn get_space(&self, i: i32) -> Option<Rc<AddrSpace>> {
        self.manager().get_space(i).cloned()
    }
}
