//! The ghidra-mode translator — a port of
//! `decompiler/cpp/ghidra_translate.cc` (`GhidraTranslate`): a [`Translate`]
//! whose disassembly/p-code and register lookups are answered by queries to
//! the Java host instead of a local `.sla`.
//!
//! In ghidra mode no `.sla` file exists.  The entire "language" the native
//! side knows is the stripped-down `<sleigh>` document the Java client sends
//! at registerProgram (produced by `SleighLanguage.encodeTranslator`,
//! SleighLanguage.java) — endianness, the unique base, and the address-space
//! list whose per-space `index` attributes are the Java
//! `AddressSpace.getUnique()` values.  Those indices are the deepest interop
//! invariant of the packed protocol: `PackedDecode::read_space` /
//! `PackedEncode::write_space` exchange spaces by exactly these numbers, so
//! parsing the tspec into a real [`AddrSpaceManager`] (sparse `insert_space`,
//! kuna-base/src/space.rs) is what gives `decompileAt` a working `<addr>`
//! decoder.
//!
//! The C++ `decodeSpaces` loop cannot be called directly in Rust (the decoder
//! borrows the manager the loop mutates); the sanctioned stepwise pattern — a
//! fresh decoder per child element around each `decode_space`/`insert_space`
//! pair — comes from the kuna-sleigh space manager test
//! (kuna-sleigh/src/translate.rs).
//!
//! Two shapes live here.  [`TspecModel`] is the pure, client-free tspec decode
//! (`AddrSpaceManager` + endianness + unique base); the phase-1 command loop
//! ([`crate::process`]) needs exactly this and nothing more, so the parse stays
//! free of the query client.  [`GhidraTranslate`] is the phase-2 live
//! translator: a `TspecModel` folded into a [`TranslateBase`], plus the shared
//! query `GhidraClient`, the register caches, a query-backed [`GhidraLoadImage`],
//! and a context database.  It implements [`RegisterLookup`], [`Translate`] and
//! [`EngineTranslate`] by issuing queries (`getPcode`, `getRegister`,
//! `getRegisterName`, `getUserOpName`) on the host.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::{Read, Write};
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{
    Decoder, Encoder, IdRegistry, PackedDecode, XmlDecode, ATTRIB_BIGENDIAN, ATTRIB_OFFSET,
    ELEM_INPUT, ELEM_OUTPUT,
};
use kuna_base::space::{
    spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, RegisterLookup, VarnodeStorage,
    ATTRIB_DEFAULTSPACE,
};
use kuna_base::types::int4;
use kuna_base::xml::{xml_tree, DocumentStorage};
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::globalcontext::{ContextDatabase, ContextInternal};
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::translate::{
    register_translate_ids, varnode_data_from_storage, AssemblyEmit, PcodeEmit, Translate,
    TranslateBase, TruncationTag, ATTRIB_UNIQBASE, ELEM_SLEIGH,
};

use kuna_decomp::engine_translate::EngineTranslate;
use kuna_decomp::pcodeinject::{
    InjectContext, CALLFIXUP_TYPE, CALLMECHANISM_TYPE, CALLOTHERFIXUP_TYPE, ELEM_CONTEXT,
};

use crate::client::InjectType;
use crate::ids::{register_ghidra_ids, ELEM_UNIMPL};
use crate::protocol::WireError;
use crate::provider::{wire_to_kuna, GhidraLoadImage, SharedClient};

/// Build the marshaling [`IdRegistry`] a ghidra-mode session needs: the
/// kuna-base tables, the translate.cc ids (the tspec vocabulary), and the
/// ghidra protocol ids (the same per-module registration pattern as
/// `kuna-console`'s `build_registry`).
pub fn build_registry() -> IdRegistry {
    let mut registry = IdRegistry::with_base_ids();
    register_translate_ids(&mut registry);
    register_ghidra_ids(&mut registry);
    registry
}

/// The client-free tspec decode: the address-space model plus endianness /
/// unique base that C++ `GhidraTranslate::decode` (ghidra_translate.cc:161-176)
/// extracts from the `<sleigh>` document.
///
/// This is the seed [`GhidraTranslate::new`] folds into a [`TranslateBase`] +
/// [`AddrSpaceManager`]; it also stands alone as the only piece the phase-1
/// command loop needs (it decodes `<addr>` parameters but issues no queries).
#[derive(Debug)]
pub struct TspecModel {
    /// The address-space model, indices bound to the Java
    /// `AddressSpace.getUnique()` numbering.
    pub manager: AddrSpaceManager,
    /// The processor endianness (C++ `Translate::setBigEndian`).
    pub big_endian: bool,
    /// The starting offset for temporaries in the unique space (C++
    /// `Translate::setUniqueBase`; Java sends
    /// `UniqueLayout.SLEIGH_BASE.getOffset(language)`).
    pub unique_base: u64,
}

impl TspecModel {
    /// Parse the `<sleigh>` tspec document (C++ `GhidraTranslate::decode`,
    /// ghidra_translate.cc:161-176):
    ///
    /// ```text
    ///   <sleigh bigendian=... uniqbase=...>
    ///     <spaces defaultspace=...>
    ///       <space|space_unique|space_other|space_overlay .../> ...
    ///     </spaces>
    ///     <truncate_space space=... size=.../> ...
    ///   </sleigh>
    /// ```
    pub fn decode(tspec: &[u8], registry: &IdRegistry) -> KunaResult<TspecModel> {
        let document = xml_tree(tspec)?;
        let root = Rc::clone(document.get_root());
        if root.get_name() != "sleigh" {
            return Err(KunaError::lowlevel(format!(
                "Could not find sleigh tag (got <{}>)",
                root.get_name()
            )));
        }

        let mut manager = AddrSpaceManager::new();

        let (big_endian, unique_base) = {
            let mut dec = XmlDecode::new_with_root(&manager, registry, &root, 0);
            dec.open_element_id(&ELEM_SLEIGH)?;
            let be = dec.read_bool_id(&ATTRIB_BIGENDIAN)?;
            let ub = dec.read_unsigned_integer_id(&ATTRIB_UNIQBASE)?;
            (be, ub)
        };

        // decodeSpaces (translate.cc): constant space first, then one child
        // element per space, then the defaultspace resolution.  Driven
        // stepwise (fresh decoder per child) per the kuna decode_spaces
        // aliasing note.
        manager.insert_space(Rc::new(ConstantSpace::new()))?;
        let spaces_el = root
            .get_children()
            .iter()
            .find(|c| c.get_name() == "spaces")
            .cloned()
            .ok_or_else(|| KunaError::lowlevel("Missing <spaces> in <sleigh> tag"))?;
        let defname = {
            let mut dec = XmlDecode::new_with_root(&manager, registry, &spaces_el, 0);
            dec.open_element()?;
            String::from_utf8_lossy(&dec.read_string_id(&ATTRIB_DEFAULTSPACE)?).into_owned()
        };
        for child in spaces_el.get_children() {
            let spc = {
                let mut dec = XmlDecode::new_with_root(&manager, registry, child, 0);
                manager.decode_space(&mut dec)?
            };
            manager.insert_space(spc)?;
        }
        let def_index = match manager.get_space_by_name(&defname) {
            Some(spc) => spc.get_index(),
            None => {
                return Err(KunaError::lowlevel(format!(
                    "Bad 'defaultspace' attribute: {defname}"
                )))
            }
        };
        manager.set_default_code_space(def_index)?;

        for child in root.get_children() {
            if child.get_name() != "truncate_space" {
                continue;
            }
            let mut tag = TruncationTag::default();
            {
                let mut dec = XmlDecode::new_with_root(&manager, registry, child, 0);
                tag.decode(&mut dec)?;
            }
            manager.truncate_space(tag.get_name(), tag.get_size())?;
        }

        Ok(TspecModel {
            manager,
            big_endian,
            unique_base,
        })
    }
}

/// C++ `GhidraTranslate` (ghidra_translate.hh:36-57): the live ghidra-mode
/// translator.  Its disassembly/p-code (`oneInstruction`) and register
/// lookups (`getRegister`/`getRegisterName`) are served by queries on the
/// shared [`GhidraClient`] (the kuna analog of the C++ `glb` back-pointer).
///
/// All the query I/O and cache fills run through `&self` (the trait methods
/// are `&self`, matching C++ `const`/`mutable`), so mutation goes through the
/// `RefCell` caches and the `RefCell` inside [`SharedClient`] — the sanctioned
/// kuna interior-mutability pattern.
pub struct GhidraTranslate<R: Read, W: Write> {
    /// The concrete C++ `Translate` base-class state — endianness, unique
    /// base, float formats (reached through [`Translate::translate_base`]).
    base: TranslateBase,
    /// The address-space model; indices bound to the Java
    /// `AddressSpace.getUnique()` numbering.  Held as `Rc` so
    /// [`EngineTranslate::manager_rc`] shares the same space set (identities
    /// and indices) the tspec decode built.
    manager: Rc<AddrSpaceManager>,
    /// The shared query client (C++ `ArchitectureGhidra *glb`).
    client: SharedClient<R, W>,
    /// The query-backed register resolver holding the C++ `mutable`
    /// `nm2addr`/`addr2nm` caches — shared with the [`AddrSpaceManager`]
    /// (Phase 3: `set_register_lookup`, mirroring
    /// `Sleigh::install_register_lookup`) so manager-side register lookups
    /// (`name=` decodes, the naming pass's is-register test) resolve too.
    reglookup: Rc<GhidraRegisterLookup<R, W>>,
    /// The program load image (C++ `LoadImageGhidra`): every read of program
    /// bytes becomes a `getBytes` query.  Boxed behind `Rc<RefCell<…>>` to
    /// mirror `Sleigh::loader`, so [`EngineTranslate::loader_rc`] can hand out
    /// a shared handle and `set_loader` can replace it in place.
    loader: Rc<RefCell<Box<dyn LoadImage>>>,
    /// The context database (C++ `ContextGhidra`).
    ///
    /// STEP-2 STUB: a real [`ContextInternal`] with empty tracked sets.  In
    /// ghidra mode the host disassembles, so local disassembly context is
    /// essentially never consulted; an empty context is correct for step 2.
    /// TODO(phase-3): back `get_tracked_set` with the `getTrackedRegisters`
    /// query (`GhidraClient::get_tracked_registers` + `decode_tracked`), which
    /// needs a fetch-and-cache design resolving the
    /// `&TrackedSet`-from-`&self` borrow (spec §5 risks).
    context: RefCell<ContextInternal>,
}

/// The query-backed [`RegisterLookup`] installed on the ghidra-mode
/// [`AddrSpaceManager`] (Phase 3).  The standalone Sleigh installs itself on
/// the manager (`Sleigh::install_register_lookup`) so manager-side consumers —
/// `VarnodeData::decode_from_attributes`' `name=` register path, and the
/// naming pass's is-this-a-register test (`coreaction_cleanup`'s
/// `manage().register_lookup()`) — can resolve registers; without one, every
/// register-storage high misclassifies as global data, is skipped by local
/// naming, and renders as a raw `EAX`/`RBX` token.  This adapter answers the
/// same three lookups with getRegister/getRegisterName queries and the same
/// `mutable` caches as `GhidraTranslate` (which delegates to it).
///
/// The manager holds this as `Rc<dyn RegisterLookup>` while this holds the
/// manager back as a `Weak` (set once the manager `Rc` exists) — no cycle.
pub struct GhidraRegisterLookup<R: Read, W: Write> {
    client: SharedClient<R, W>,
    manager: RefCell<std::rc::Weak<AddrSpaceManager>>,
    nm2addr: RefCell<BTreeMap<String, VarnodeStorage>>,
    addr2nm: RefCell<BTreeMap<VarnodeStorage, String>>,
}

impl<R: Read, W: Write> GhidraRegisterLookup<R, W> {
    fn new(client: SharedClient<R, W>) -> Self {
        GhidraRegisterLookup {
            client,
            manager: RefCell::new(std::rc::Weak::new()),
            nm2addr: RefCell::new(BTreeMap::new()),
            addr2nm: RefCell::new(BTreeMap::new()),
        }
    }

    /// C++ `GhidraTranslate::cacheRegister`: record a resolved register both ways.
    #[allow(clippy::mutable_key_type)] // AddrSpace interior mutability does not affect Ord
    fn cache_register(&self, nm: &str, vndata: &VarnodeStorage) {
        self.nm2addr
            .borrow_mut()
            .insert(nm.to_string(), vndata.clone());
        self.addr2nm
            .borrow_mut()
            .insert(vndata.clone(), nm.to_string());
    }
}

impl<R: Read, W: Write> RegisterLookup for GhidraRegisterLookup<R, W> {
    /// C++ `GhidraTranslate::getRegister` (ghidra_translate.cc:45-70).
    fn get_register(&self, nm: &str) -> KunaResult<VarnodeStorage> {
        {
            let cache = self.nm2addr.borrow();
            if let Some(vndata) = cache.get(nm) {
                return Ok(vndata.clone());
            }
        }
        let manager = self
            .manager
            .borrow()
            .upgrade()
            .ok_or_else(|| KunaError::lowlevel("ghidra register lookup: manager gone"))?;
        let mut decoder = PackedDecode::new(&manager);
        let found = self
            .client
            .borrow_mut()
            .get_register(nm, &mut decoder)
            .map_err(wire_to_kuna)?;
        if !found {
            return Err(KunaError::lowlevel(format!("No register named {nm}")));
        }
        let (regaddr, regsize) = Address::decode_sized(&mut decoder)?;
        let vndata = VarnodeStorage {
            space: regaddr.get_space().cloned(),
            offset: regaddr.get_offset(),
            size: regsize as u32,
        };
        self.cache_register(nm, &vndata);
        Ok(vndata)
    }

    /// Cache-only [`RegisterLookup::probe_register`]: a speculative name test
    /// must not become a `getRegister` query, because the Java host answers an
    /// undefined name by throwing `No Register Defined` (GH-388).
    fn probe_register(&self, nm: &str) -> Option<VarnodeStorage> {
        self.nm2addr.borrow().get(nm).cloned()
    }

    /// C++ `GhidraTranslate::getRegisterName` (ghidra_translate.cc:72-87).
    #[allow(clippy::mutable_key_type)] // see cache_register
    fn get_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: i32) -> String {
        if base.get_type() != spacetype::IPTR_PROCESSOR {
            return String::new();
        }
        let vndata = VarnodeStorage {
            space: Some(Rc::clone(base)),
            offset: off,
            size: size as u32,
        };
        {
            let cache = self.addr2nm.borrow();
            if let Some(nm) = cache.get(&vndata) {
                return nm.clone();
            }
        }
        let bytes = match self.client.borrow_mut().get_register_name(&vndata) {
            Ok(b) => b,
            Err(_) => return String::new(),
        };
        if bytes.is_empty() {
            return String::new();
        }
        let name = String::from_utf8_lossy(&bytes).into_owned();
        // Cache the FULL register (the queried vndata may be a truncated piece).
        let _ = self.get_register(&name);
        name
    }

    /// C++ `GhidraTranslate::getExactRegisterName` (ghidra_translate.cc:89-106).
    #[allow(clippy::mutable_key_type)] // see cache_register
    fn get_exact_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: i32) -> String {
        if base.get_type() != spacetype::IPTR_PROCESSOR {
            return String::new();
        }
        let vndata = VarnodeStorage {
            space: Some(Rc::clone(base)),
            offset: off,
            size: size as u32,
        };
        {
            let cache = self.addr2nm.borrow();
            if let Some(nm) = cache.get(&vndata) {
                return nm.clone();
            }
        }
        let bytes = match self.client.borrow_mut().get_register_name(&vndata) {
            Ok(b) => b,
            Err(_) => return String::new(),
        };
        if bytes.is_empty() {
            return String::new();
        }
        let name = String::from_utf8_lossy(&bytes).into_owned();
        match self.get_register(&name) {
            Ok(full) if full.size == size as u32 => name,
            _ => String::new(),
        }
    }
}

// `'static` on the stream types: the loader coerces `GhidraLoadImage<R,W>`
// into a `Box<dyn LoadImage>` (a `dyn … + 'static` trait object), so the
// erased stream types must outlive it.  The real binary uses stdin/stdout and
// the tests use owned `Cursor`/`Vec`, all `'static`.
impl<R: Read + 'static, W: Write + 'static> GhidraTranslate<R, W> {
    /// Build the live translator from the tspec document and the shared query
    /// client (the kuna analog of the C++ `ArchitectureGhidra` installing
    /// `buildTranslator`→GhidraTranslate + `buildLoader`→LoadImageGhidra,
    /// ghidra_arch.cc:283-299).
    ///
    /// The `<sleigh>` parse ([`TspecModel::decode`]) yields the address-space
    /// manager and endianness/unique base, which are folded into the
    /// [`TranslateBase`]; the loader is a [`GhidraLoadImage`] over the same
    /// shared client.
    pub fn new(
        tspec: &[u8],
        registry: &IdRegistry,
        client: SharedClient<R, W>,
    ) -> KunaResult<GhidraTranslate<R, W>> {
        let TspecModel {
            manager,
            big_endian,
            unique_base,
        } = TspecModel::decode(tspec, registry)?;
        let mut base = TranslateBase::new();
        base.set_big_endian(big_endian);
        base.set_unique_base(unique_base as u32); // uintb -> uintm (uint4), as C++ setUniqueBase
                                                  // Downstream stages (float-op typing) expect IEEE formats present.
        base.set_default_float_formats();

        let loader: Box<dyn LoadImage> = Box::new(GhidraLoadImage::new(Rc::clone(&client)));

        // Phase 3: install the query-backed register resolver on the manager
        // (the ghidra-mode mirror of `Sleigh::install_register_lookup`) BEFORE
        // wrapping it in `Rc`.  The adapter's `Weak` back-pointer is wired
        // LAZILY ([`Self::ensure_lookup_wired`]) because an outstanding `Weak`
        // blocks the `Rc::get_mut` in [`EngineTranslate::manager_mut`], which
        // engine init still calls (spec-decoded space insertion) — the same
        // sole-owner-during-init invariant the plain `Rc` already relied on.
        let reglookup = Rc::new(GhidraRegisterLookup::new(Rc::clone(&client)));
        let mut manager = manager;
        manager.set_register_lookup(Rc::clone(&reglookup) as Rc<dyn RegisterLookup>);
        let manager = Rc::new(manager);

        Ok(GhidraTranslate {
            base,
            manager,
            client,
            reglookup,
            loader: Rc::new(RefCell::new(loader)),
            context: RefCell::new(ContextInternal::new()),
        })
    }
}

impl<R: Read, W: Write> GhidraTranslate<R, W> {
    /// Wire the register adapter's `Weak` manager back-pointer once init-time
    /// `manager_mut` mutation is over (see [`Self::new`]).  Idempotent, cheap.
    fn ensure_lookup_wired(&self) {
        if self.reglookup.manager.borrow().upgrade().is_none() {
            *self.reglookup.manager.borrow_mut() = Rc::downgrade(&self.manager);
        }
    }
}

impl<R: Read, W: Write> RegisterLookup for GhidraTranslate<R, W> {
    /// C++ `GhidraTranslate::getRegister` — delegated to the shared
    /// [`GhidraRegisterLookup`] (one cache, also installed on the manager).
    fn get_register(&self, nm: &str) -> KunaResult<VarnodeStorage> {
        self.ensure_lookup_wired();
        self.reglookup.get_register(nm)
    }

    /// Cache-only [`RegisterLookup::probe_register`] — delegated.
    fn probe_register(&self, nm: &str) -> Option<VarnodeStorage> {
        self.reglookup.probe_register(nm)
    }

    /// C++ `GhidraTranslate::getRegisterName` — delegated.
    fn get_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: i32) -> String {
        self.ensure_lookup_wired();
        self.reglookup.get_register_name(base, off, size)
    }

    /// C++ `GhidraTranslate::getExactRegisterName` — delegated.
    fn get_exact_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: i32) -> String {
        self.ensure_lookup_wired();
        self.reglookup.get_exact_register_name(base, off, size)
    }
}

impl<R: Read, W: Write> Translate for GhidraTranslate<R, W> {
    fn translate_base(&self) -> &TranslateBase {
        &self.base
    }
    fn translate_base_mut(&mut self) -> &mut TranslateBase {
        &mut self.base
    }
    /// The tspec is parsed at construction ([`GhidraTranslate::new`]), so the
    /// `DocumentStorage` initialize path is vestigial (C++
    /// `GhidraTranslate::initialize` pulls the `<sleigh>` tag from the store;
    /// kuna already has it).
    fn initialize(&mut self, _store: &mut DocumentStorage) -> KunaResult<()> {
        Ok(())
    }
    /// C++ `GhidraTranslate::getAllRegisters` throws `LowlevelError`
    /// (ghidra_translate.hh:49-50); kuna's infallible signature can only
    /// no-op.  The Java host owns the register table; this is never reached in
    /// ghidra mode.
    #[allow(clippy::mutable_key_type)] // VarnodeData interior mutability does not affect its Ord (see the trait)
    fn get_all_registers(&self, _reglist: &mut BTreeMap<VarnodeData, String>) {}
    /// C++ `GhidraTranslate::getUserOpNames` (ghidra_translate.cc:108-118):
    /// the init-time index probe — query `getUserOpName(i)` for `i=0,1,…`
    /// until the host answers an empty name.  Infallible: a query error also
    /// ends the loop (spec risks).
    fn get_user_op_names(&self, res: &mut Vec<String>) {
        let mut i = 0i32;
        loop {
            let bytes = match self.client.borrow_mut().get_user_op_name(i) {
                Ok(b) => b,
                Err(_) => break,
            };
            if bytes.is_empty() {
                break;
            }
            res.push(String::from_utf8_lossy(&bytes).into_owned());
            i += 1;
        }
    }
    /// C++ throws (ghidra_translate.hh:53-54) — the length is only knowable by
    /// disassembling, which is the host's job.
    fn instruction_length(&self, _baseaddr: &Address) -> KunaResult<i32> {
        Err(KunaError::lowlevel(
            "Cannot get instruction length through this interface",
        ))
    }
    /// C++ `GhidraTranslate::oneInstruction` (ghidra_translate.cc:120-156): a
    /// `getPcode` query whose response is either `<unimpl offset=.../>` or
    /// `<inst offset=..><addr/> <op ...>* </inst>`.  The load-bearing method.
    fn one_instruction(&self, emit: &mut dyn PcodeEmit, baseaddr: &Address) -> KunaResult<i32> {
        // The decoder's space indices come from the tspec manager, which is
        // what makes the packed <addr>/<spaceid> elements decode correctly.
        let mut decoder = PackedDecode::new(&self.manager);
        let success = match self.client.borrow_mut().get_pcode(baseaddr, &mut decoder) {
            Ok(s) => s,
            // C++ catch(JavaError&): rethrow LowlevelError with the address.
            Err(WireError::Kuna(KunaError::Java { .. })) => {
                let mut msg = String::from("Error generating pcode at address: ");
                let _ = baseaddr.print_raw(&mut msg);
                return Err(KunaError::lowlevel(msg));
            }
            Err(other) => return Err(wire_to_kuna(other)),
        };
        if !success {
            // C++ BadDataError — the flow decode-error ladder's BadData branch.
            let mut msg = String::from("No pcode could be generated at address: ");
            let _ = baseaddr.print_raw(&mut msg);
            return Err(KunaError::bad_data(msg));
        }
        let el = decoder.open_element()?;
        let offset = decoder.read_signed_integer_id(&ATTRIB_OFFSET)? as i32; // intb -> int4
        if el == ELEM_UNIMPL.get_id() {
            // C++ UnimplError(msg, offset) — the Unimpl variant must carry the
            // instruction length the flow ladder reads.
            let mut msg = String::from("Instruction not implemented in pcode:\n ");
            let _ = baseaddr.print_raw(&mut msg);
            return Err(KunaError::unimpl(msg, offset));
        }
        let pc = Address::decode(&mut decoder)?;
        // The <inst> element is never closed; peek returns 0 at </inst>.
        while decoder.peek_element()? != 0 {
            emit.decode_op(&pc, &mut decoder)?;
        }
        Ok(offset)
    }
    /// C++ throws (ghidra_translate.hh:55-56) — disassembly is the host's job.
    fn print_assembly(&self, _emit: &mut dyn AssemblyEmit, _baseaddr: &Address) -> KunaResult<i32> {
        Err(KunaError::lowlevel(
            "Cannot disassemble through this interface",
        ))
    }
}

impl<R: Read, W: Write> EngineTranslate for GhidraTranslate<R, W> {
    fn manager(&self) -> &AddrSpaceManager {
        &self.manager
    }
    fn manager_rc(&self) -> Rc<AddrSpaceManager> {
        // The first shared clone marks the end of init-time mutation: wire the
        // register adapter's back-pointer (see `ensure_lookup_wired`).
        self.ensure_lookup_wired();
        Rc::clone(&self.manager)
    }
    fn manager_mut(&mut self) -> &mut AddrSpaceManager {
        // Mirrors Sleigh: mutable access is only taken during init, while the
        // engine is still the sole `Rc` owner (before any `glb` clones it).
        // The register adapter's `Weak` (Phase 3) would block `Rc::get_mut`,
        // so drop it first — it re-wires lazily on the next shared use
        // (`ensure_lookup_wired`); a `Weak` upgrade can never race the
        // mutation because the adapter only upgrades inside its own query
        // methods, which init never runs concurrently with this borrow.
        *self.reglookup.manager.borrow_mut() = std::rc::Weak::new();
        Rc::get_mut(&mut self.manager)
            .expect("ghidra engine address-space manager already shared (Rc not unique)")
    }
    fn loader_rc(&self) -> Rc<RefCell<Box<dyn LoadImage>>> {
        Rc::clone(&self.loader)
    }
    fn set_loader(&mut self, loader: Box<dyn LoadImage>) {
        *self.loader.borrow_mut() = loader;
    }
    /// Read `sz` (<=8) bytes of a Varnode's value from the load image (mirrors
    /// `Sleigh::read_loadimage_value` / C++ `Emulate*::getLoadImageValue`):
    /// `loadFill` a full `uintb`, byte-swap to host order if the space
    /// endianness differs, then mask/shift down to `sz`.
    fn read_loadimage_value(&self, addr: &Address, sz: i32) -> KunaResult<u64> {
        use kuna_base::address::{byte_swap, calc_mask};
        use kuna_base::types::HOST_ENDIAN;
        let mut buf = [0u8; 8]; // sizeof(uintb)
        self.loader.borrow_mut().load_fill(&mut buf, addr)?;
        let big = addr.is_big_endian();
        let mut res = if HOST_ENDIAN == 1 {
            u64::from_be_bytes(buf)
        } else {
            u64::from_le_bytes(buf)
        };
        if (HOST_ENDIAN == 1) != big {
            res = byte_swap(res, 8);
        }
        if big && (sz as usize) < 8 {
            res >>= (8 - sz as u32) * 8;
        } else {
            res &= calc_mask(sz);
        }
        Ok(res)
    }
    /// C++ `Translate::getRegister(name)` returning the canonical
    /// [`VarnodeData`] — adapts [`RegisterLookup::get_register`] (which yields
    /// the kuna-base [`VarnodeStorage`] triple).
    fn get_register_varnode(&self, nm: &[u8]) -> KunaResult<VarnodeData> {
        let name = String::from_utf8_lossy(nm);
        let vndata = RegisterLookup::get_register(self, &name)?;
        Ok(varnode_data_from_storage(&vndata))
    }
    /// Speculative [`EngineTranslate::probe_register_varnode`], answered from
    /// the register cache alone — see [`RegisterLookup::probe_register`].
    fn probe_register_varnode(&self, nm: &[u8]) -> Option<VarnodeData> {
        let name = String::from_utf8_lossy(nm);
        RegisterLookup::probe_register(self, &name).map(|v| varnode_data_from_storage(&v))
    }
    /// Run a closure with mutable access to the context database (C++
    /// `glb->context`).  STEP-2 STUB: the [`ContextInternal`] field (see the
    /// struct doc); TODO(phase-3) query-backed tracked registers.
    fn with_context_db_dyn(&self, f: &mut dyn FnMut(&mut dyn ContextDatabase)) {
        let mut db = self.context.borrow_mut();
        f(&mut *db);
    }
    /// C++ `InjectPayloadGhidra::inject` (inject_ghidra.cc:49-69): with no
    /// local `.sla` there is no compiled snippet, so the host is asked to
    /// lift the payload against this call site and the answer is streamed
    /// straight into `emit`.  The response is bound to `context` — the ops
    /// carry this site's address and this site's storage — so it is never
    /// reusable and never cached.
    fn fetch_inject_pcode(
        &self,
        name: &[u8],
        ptype: int4,
        context: &InjectContext,
        emit: &mut dyn PcodeEmit,
    ) -> KunaResult<()> {
        let itype = match ptype {
            CALLFIXUP_TYPE => InjectType::CallFixup,
            CALLOTHERFIXUP_TYPE => InjectType::CallOtherFixup,
            CALLMECHANISM_TYPE => InjectType::CallMechanism,
            _ => InjectType::ExecutablePcode,
        };
        let nm = String::from_utf8_lossy(name).into_owned();
        // The decoder's space indices come from the tspec manager, as in
        // `one_instruction`; the borrow is a statement temporary so the client
        // is free again before `emit` is touched (the query re-entry rule).
        let mut decoder = PackedDecode::new(&self.manager);
        let answered = match self.client.borrow_mut().get_pcode_inject(
            &nm,
            itype,
            |e| encode_inject_context(context, e),
            &mut decoder,
        ) {
            Ok(a) => a,
            // C++ catch(JavaError&): the host's message becomes a LOW-LEVEL
            // error, so the warning names the payload rather than repeating the
            // host's bare text.
            Err(WireError::Kuna(KunaError::Java { explain, .. })) => {
                return Err(KunaError::lowlevel(format!("Injection error: {explain}")))
            }
            // C++ catch(DecoderError&).
            Err(WireError::Kuna(KunaError::Decoder { explain })) => {
                return Err(KunaError::lowlevel(format!(
                    "Error decoding injection: {explain}"
                )))
            }
            Err(other) => return Err(wire_to_kuna(other)),
        };
        if !answered {
            return Err(KunaError::lowlevel(format!(
                "Could not retrieve injection: {nm}"
            )));
        }
        let elem = decoder.open_element()?;
        // Ghidra answers a lone UNIMPLEMENTED op with `<unimpl>` instead of
        // `<inst>` (DecompileCallback.encodeInstruction); upstream walks into
        // `Address::decode` there and faults.
        if elem == ELEM_UNIMPL.get_id() {
            return Err(KunaError::lowlevel(format!(
                "Unimplemented p-code injection: {nm}"
            )));
        }
        let addr = Address::decode(&mut decoder)?;
        while decoder.peek_element()? != 0 {
            emit.decode_op(&addr, &mut decoder)?;
        }
        decoder.close_element(elem)?;
        Ok(())
    }
    // as_sleigh / as_sleigh_mut: take the trait defaults (None) — this is not
    // the standalone Sleigh engine.
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    use kuna_base::marshal::{Encoder, PackedEncode};
    use kuna_num::opcodes::{OpCode, OpcodeEncoder};
    use kuna_sleigh::translate::{ATTRIB_CODE, ELEM_OP};

    use crate::client::GhidraClient;

    /// A realistic tspec, shaped exactly like SleighLanguage.encodeTranslator
    /// output (single line, XmlEncode(false)): OTHER at unique index 1
    /// (required by insertSpace), then unique/ram/register.
    pub const TEST_TSPEC: &[u8] = b"<sleigh bigendian=\"false\" uniqbase=\"0x10000000\">\
<spaces defaultspace=\"ram\">\
<space_other name=\"OTHER\" index=\"1\" size=\"8\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
<space_unique name=\"unique\" index=\"2\" size=\"4\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
<space name=\"ram\" index=\"3\" size=\"8\" bigendian=\"false\" delay=\"1\" physical=\"true\"/>\
<space name=\"register\" index=\"4\" size=\"4\" bigendian=\"false\" delay=\"0\" physical=\"true\"/>\
</spaces></sleigh>";

    // -- TspecModel decode (the former GhidraTranslate::decode tests) --------

    #[test]
    fn test_decode_tspec() {
        let registry = build_registry();
        let tr = TspecModel::decode(TEST_TSPEC, &registry).unwrap();
        assert!(!tr.big_endian);
        assert_eq!(tr.unique_base, 0x10000000);
        let ram = tr.manager.get_space_by_name("ram").expect("ram space");
        assert_eq!(ram.get_index(), 3);
        assert_eq!(ram.get_addr_size(), 8);
        let reg = tr
            .manager
            .get_space_by_name("register")
            .expect("register space");
        assert_eq!(reg.get_index(), 4);
        let unique = tr
            .manager
            .get_space_by_name("unique")
            .expect("unique space");
        assert_eq!(unique.get_index(), 2);
        let def = tr.manager.get_default_code_space().expect("default space");
        assert_eq!(def.get_name(), "ram");
        let cspace = tr.manager.get_constant_space().expect("constant space");
        assert_eq!(cspace.get_index(), 0);
    }

    #[test]
    fn test_decode_tspec_truncate_space() {
        let registry = build_registry();
        let tspec = b"<sleigh bigendian=\"true\" uniqbase=\"0x100\">\
<spaces defaultspace=\"ram\">\
<space name=\"ram\" index=\"1\" size=\"8\" bigendian=\"true\" delay=\"1\" physical=\"true\"/>\
</spaces><truncate_space space=\"ram\" size=\"4\"/></sleigh>";
        let tr = TspecModel::decode(tspec, &registry).unwrap();
        assert!(tr.big_endian);
        let ram = tr.manager.get_space_by_name("ram").unwrap();
        assert_eq!(ram.get_addr_size(), 4);
    }

    #[test]
    fn test_decode_tspec_rejects_non_sleigh_root() {
        let registry = build_registry();
        assert!(TspecModel::decode(b"<x/>", &registry).is_err());
    }

    // -- GhidraTranslate live-translator tests (canned-stream MockJava) ------

    type TestClient = SharedClient<Cursor<Vec<u8>>, Vec<u8>>;

    fn shared(reader: Vec<u8>) -> TestClient {
        Rc::new(RefCell::new(GhidraClient::new(
            Cursor::new(reader),
            Vec::new(),
        )))
    }

    fn translate(client: TestClient) -> GhidraTranslate<Cursor<Vec<u8>>, Vec<u8>> {
        GhidraTranslate::new(TEST_TSPEC, &build_registry(), client).expect("tspec parses")
    }

    fn burst(v: &mut Vec<u8>, code: u8) {
        v.extend_from_slice(&[0, 0, 1, code]);
    }

    /// Wrap a packed document as a query response: response open (8), string
    /// open (14), the packed bytes, string close (15), response close (9).
    fn packed_response(packed: &[u8]) -> Vec<u8> {
        let mut r = Vec::new();
        burst(&mut r, 8);
        burst(&mut r, 14);
        r.extend_from_slice(packed);
        burst(&mut r, 15);
        burst(&mut r, 9);
        r
    }

    /// A plain-string query response (getRegisterName / getUserOpName):
    /// response open (8), string open (14), bytes, string close (15),
    /// response close (9).
    fn string_response(s: &[u8]) -> Vec<u8> {
        let mut r = Vec::new();
        burst(&mut r, 8);
        burst(&mut r, 14);
        r.extend_from_slice(s);
        burst(&mut r, 15);
        burst(&mut r, 9);
        r
    }

    /// An empty query response: response open (8) then response close (9),
    /// with no string stream — `read_all` returns `false`.
    fn empty_response() -> Vec<u8> {
        let mut r = Vec::new();
        burst(&mut r, 8);
        burst(&mut r, 9);
        r
    }

    fn ram_space(tr: &GhidraTranslate<Cursor<Vec<u8>>, Vec<u8>>) -> Rc<AddrSpace> {
        Rc::clone(tr.manager.get_space_by_name("ram").expect("ram"))
    }

    /// A recording `PcodeEmit` that captures each decoded op.
    #[derive(Default)]
    struct RecordEmit {
        ops: Vec<(OpCode, Option<VarnodeData>, Vec<VarnodeData>)>,
    }
    impl PcodeEmit for RecordEmit {
        fn dump(
            &mut self,
            _addr: &Address,
            opc: OpCode,
            outvar: Option<&VarnodeData>,
            vars: &[VarnodeData],
        ) {
            self.ops.push((opc, outvar.cloned(), vars.to_vec()));
        }
    }

    /// (a) `one_instruction` against a canned `<inst><addr/><op ...></inst>`
    /// getPcode response: assert the byte-exact request framing AND the
    /// decoded ops handed to the emit.
    #[test]
    fn one_instruction_decodes_inst_ops() {
        // Build the getPcode response document over the tspec manager.
        let model = TspecModel::decode(TEST_TSPEC, &build_registry()).unwrap();
        let ram = Rc::clone(model.manager.get_space_by_name("ram").unwrap());
        // ELEM_INST id 98 (kuna-decomp/pcodeinject.rs) — any non-unimpl id.
        let elem_inst = kuna_base::marshal::ElementId::new("inst", 98);
        let mut doc = Vec::new();
        {
            let mut e = PackedEncode::new(&mut doc);
            e.open_element(&elem_inst);
            e.write_signed_integer(&ATTRIB_OFFSET, 4); // instruction length
                                                       // the pc <addr>
            Address::new(Rc::clone(&ram), 0x401000)
                .encode(&mut e)
                .unwrap();
            // one <op>: INT_ADD out=ram:0x20:4  in=ram:0x30:4, ram:0x34:4
            e.open_element(&ELEM_OP);
            e.write_signed_integer(&kuna_base::marshal::ATTRIB_SIZE, 2); // isize
            e.write_opcode(&ATTRIB_CODE, OpCode::CPUI_INT_ADD);
            Address::new(Rc::clone(&ram), 0x20)
                .encode_sized(&mut e, 4)
                .unwrap();
            Address::new(Rc::clone(&ram), 0x30)
                .encode_sized(&mut e, 4)
                .unwrap();
            Address::new(Rc::clone(&ram), 0x34)
                .encode_sized(&mut e, 4)
                .unwrap();
            e.close_element(&ELEM_OP);
            e.close_element(&elem_inst);
        }

        let client = shared(packed_response(&doc));
        let tr = translate(Rc::clone(&client));
        let ram = ram_space(&tr);
        let baseaddr = Address::new(Rc::clone(&ram), 0x401000);

        let mut emit = RecordEmit::default();
        let len = tr
            .one_instruction(&mut emit, &baseaddr)
            .expect("one_instruction");
        assert_eq!(len, 4, "returned instruction length is the <inst> offset");

        assert_eq!(emit.ops.len(), 1);
        let (opc, out, ins) = &emit.ops[0];
        assert_eq!(*opc, OpCode::CPUI_INT_ADD);
        let out = out.as_ref().expect("output varnode");
        assert_eq!((out.offset, out.size), (0x20, 4));
        assert_eq!(ins.len(), 2);
        assert_eq!((ins[0].offset, ins[0].size), (0x30, 4));
        assert_eq!((ins[1].offset, ins[1].size), (0x34, 4));

        // Byte-exact getPcode request framing (query open, string, packed
        // command doc, string close, query close).
        drop(tr);
        let (_r, w) = Rc::try_unwrap(client).unwrap().into_inner().into_inner();
        let mut cmd = Vec::new();
        {
            let ram2 = Rc::clone(
                TspecModel::decode(TEST_TSPEC, &build_registry())
                    .unwrap()
                    .manager
                    .get_space_by_name("ram")
                    .unwrap(),
            );
            let mut e = PackedEncode::new(&mut cmd);
            e.open_element(&crate::ids::ELEM_COMMAND_GETPCODE);
            Address::new(ram2, 0x401000).encode(&mut e).unwrap();
            e.close_element(&crate::ids::ELEM_COMMAND_GETPCODE);
        }
        let mut expected = Vec::new();
        burst(&mut expected, 4);
        burst(&mut expected, 14);
        expected.extend_from_slice(&cmd);
        burst(&mut expected, 15);
        burst(&mut expected, 5);
        assert_eq!(w, expected, "getPcode request framing");
    }

    /// (b) `<unimpl offset=N/>` → `KunaError::Unimpl` carrying the offset.
    #[test]
    fn one_instruction_unimpl_carries_length() {
        let mut doc = Vec::new();
        {
            let mut e = PackedEncode::new(&mut doc);
            e.open_element(&ELEM_UNIMPL);
            e.write_signed_integer(&ATTRIB_OFFSET, 2);
            e.close_element(&ELEM_UNIMPL);
        }
        let client = shared(packed_response(&doc));
        let tr = translate(client);
        let ram = ram_space(&tr);
        let err = tr
            .one_instruction(&mut RecordEmit::default(), &Address::new(ram, 0x1000))
            .expect_err("unimpl must error");
        match err {
            KunaError::Unimpl {
                instruction_length, ..
            } => assert_eq!(instruction_length, 2),
            other => panic!("expected Unimpl, got {other:?}"),
        }
    }

    /// (c) An empty getPcode response → `KunaError::BadData`.
    #[test]
    fn one_instruction_empty_is_bad_data() {
        let client = shared(empty_response());
        let tr = translate(client);
        let ram = ram_space(&tr);
        let err = tr
            .one_instruction(&mut RecordEmit::default(), &Address::new(ram, 0x1000))
            .expect_err("empty response must error");
        assert!(
            matches!(err, KunaError::BadData { .. }),
            "expected BadData, got {err:?}"
        );
    }

    /// (d) `get_register` cache miss (canned `<addr>` response) then cache hit
    /// (no second query issued).
    #[test]
    fn get_register_caches() {
        let model = TspecModel::decode(TEST_TSPEC, &build_registry()).unwrap();
        let reg = Rc::clone(model.manager.get_space_by_name("register").unwrap());
        let mut doc = Vec::new();
        {
            let mut e = PackedEncode::new(&mut doc);
            // A bare <addr> for the register (register:0x10 size 4) — the
            // shape AddressXML.encode sends as the getRegister response.
            Address::new(Rc::clone(&reg), 0x10)
                .encode_sized(&mut e, 4)
                .unwrap();
        }
        // Only ONE response is canned: a second query would run off the end
        // of the reader and error, so a cache hit is proven by success.
        let client = shared(packed_response(&doc));
        let tr = translate(Rc::clone(&client));

        let first = RegisterLookup::get_register(&tr, "myreg").expect("first lookup");
        assert_eq!(first.offset, 0x10);
        assert_eq!(first.size, 4);
        assert_eq!(
            first.space.as_ref().unwrap().get_name(),
            "register",
            "decoded against the tspec register space"
        );

        // Cache hit: no bytes left in the reader, yet this must succeed.
        let second = RegisterLookup::get_register(&tr, "myreg").expect("cache hit");
        assert_eq!(second, first);

        // Exactly one query was written (one request framing).
        drop(tr);
        let (_r, w) = Rc::try_unwrap(client).unwrap().into_inner().into_inner();
        let mut expected_one = Vec::new();
        {
            let mut cmd = Vec::new();
            {
                let mut e = PackedEncode::new(&mut cmd);
                e.open_element(&crate::ids::ELEM_COMMAND_GETREGISTER);
                e.write_string(&kuna_base::marshal::ATTRIB_NAME, b"myreg");
                e.close_element(&crate::ids::ELEM_COMMAND_GETREGISTER);
            }
            burst(&mut expected_one, 4);
            burst(&mut expected_one, 14);
            expected_one.extend_from_slice(&cmd);
            burst(&mut expected_one, 15);
            burst(&mut expected_one, 5);
        }
        assert_eq!(w, expected_one, "exactly one getRegister query issued");
    }

    /// (e) `get_user_op_names` probe loop terminates on the empty answer.
    #[test]
    fn get_user_op_names_probe_loop() {
        // Two named ops then an empty answer ends the loop.
        let mut reader = Vec::new();
        reader.extend_from_slice(&string_response(b"COUNTLEADINGZEROS"));
        reader.extend_from_slice(&string_response(b"COUNTLEADINGONES"));
        reader.extend_from_slice(&empty_response());
        let client = shared(reader);
        let tr = translate(client);

        let mut names = Vec::new();
        tr.get_user_op_names(&mut names);
        assert_eq!(names, vec!["COUNTLEADINGZEROS", "COUNTLEADINGONES"]);
    }
}

/// C++ `InjectContextGhidra::encode` (inject_ghidra.cc:20-47): the `<context>`
/// the host decodes before lifting the snippet — baseaddr, calladdr, then the
/// sized operand lists, each omitted when empty.  `nextaddr` is never sent;
/// the host recomputes it from the instruction's fall-through.
fn encode_inject_context(context: &InjectContext, encoder: &mut dyn Encoder) -> KunaResult<()> {
    let null = Address::default();
    encoder.open_element(&ELEM_CONTEXT);
    context.baseaddr.as_ref().unwrap_or(&null).encode(encoder)?;
    context.calladdr.as_ref().unwrap_or(&null).encode(encoder)?;
    for (elem, list) in [
        (&ELEM_INPUT, &context.inputlist),
        (&ELEM_OUTPUT, &context.output),
    ] {
        if list.is_empty() {
            continue;
        }
        encoder.open_element(elem);
        for vn in list.iter() {
            let addr = match &vn.space {
                Some(spc) => Address::new(Rc::clone(spc), vn.offset),
                None => Address::default(),
            };
            addr.encode_sized(encoder, vn.size as i32)?;
        }
        encoder.close_element(elem);
    }
    encoder.close_element(&ELEM_CONTEXT);
    Ok(())
}
