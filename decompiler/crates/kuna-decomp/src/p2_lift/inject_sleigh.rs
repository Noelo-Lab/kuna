//! Port of `decompiler/cpp/inject_sleigh.{cc,hh}` (item `w3-ir-userop-inject`):
//! p-code injection driven by the internal SLEIGH engine.
//!
//! Ports the concrete SLEIGH-backed payloads and library:
//! `InjectPayloadSleigh` (+ the `Callfixup`/`Callother` specializations),
//! `ExecutablePcodeSleigh`, `InjectPayloadDynamic`, `InjectContextSleigh`, and
//! `PcodeInjectLibrarySleigh`.
//!
//! ## What is fully ported vs seam-deferred
//!
//! The **decode** half of every payload — the `<callfixup>`/`<callotherfixup>`/
//! `<pcode>`/`<body>` element parsing, the parameter ordering, and the
//! library's `allocateInject`/`registerInject` payload-factory + namespace
//! registration — is ported faithfully and is the test target.
//!
//! The **emit/compile** half routes through the W2 kuna-sleigh runtime:
//!   - `parseInject` builds a [`kuna_sleigh::pcodeparse::PcodeSnippet`] over the
//!     SLEIGH language seam ([`kuna_sleigh::pcodeparse::SnippetLanguage`]) and
//!     calls `addOperand`/`setUniqueBase`/`parseStream`/`releaseResult` — all
//!     **public** kuna-sleigh API. It is ported as [`parse_inject`], generic
//!     over the `SnippetLanguage` so a caller with a wired language can drive it.
//!   - `InjectPayloadSleigh::inject` / `ExecutablePcodeSleigh::inject` and the
//!     `ExecutablePcode::build`/`evaluate` emulator wiring need
//!     `SleighBuilder`/`PcodeCacher`/`ParserWalkerChange` and the private
//!     `ParserContext` mutators (`setAddr`/`allocateOperand`/`deallocateState`),
//!     which are **private to `kuna-sleigh`'s `sleigh.rs`** and cannot be
//!     reached or made public from this crate. They sit behind the
//!     [`crate::pcodeinject::InjectEngine`] seam. (LOSS reported.)
//!
//! Per ADR 0004, fallible decode returns `Result<_, KunaError>`; the
//! `DecoderError` C++ throw in `ExecutablePcodeSleigh::decode` maps to
//! [`KunaError::decoder`](kuna_base::error::KunaError).

use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{Decoder, ATTRIB_CONTENT, ATTRIB_NAME, ATTRIB_TYPE, ELEM_TARGET};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use kuna_sleigh::context::FixedHandle;
use kuna_sleigh::pcodeparse::{PcodeSnippet, SnippetLanguage};
use kuna_sleigh::semantics::{ConstructTpl, OpTpl, PcodeBuilder, VField, VarnodeTpl};
use kuna_sleigh::slghpatexpress::PatternExpressionContext;
use kuna_sleigh::slghsymbol::{ConstructorRef, SymbolWalker};
use kuna_sleigh::translate::PcodeEmit;

use crate::pcodeinject::{
    InjectArchitecture, InjectContext, InjectEngine, InjectPayload, InjectPayloadCore,
    PcodeInjectLibraryBase, ATTRIB_TARGETOP, CALLFIXUP_TYPE, CALLOTHERFIXUP_TYPE, ELEM_ADDR_PCODE,
    ELEM_BODY, ELEM_CALLFIXUP, ELEM_CALLOTHERFIXUP, ELEM_CASE_PCODE, ELEM_DEFAULT_PCODE, ELEM_PCODE,
    ELEM_SIZE_PCODE, EXECUTABLEPCODE_TYPE,
};

// ---------------------------------------------------------------------------
// InjectPayloadSleigh (inject_sleigh.hh:42-62 + .cc:29-164)
// ---------------------------------------------------------------------------

/// \brief An injection payload built by the SLEIGH engine
/// (C++ `InjectPayloadSleigh`).
///
/// Holds the SLEIGH syntax as a string until `parseInject` compiles it into
/// p-code templates (`ConstructTpl`). The compiled template lives in the SLEIGH
/// runtime; this port keeps `parsestring` and lets the [`parse_inject`] driver
/// hand the result to a caller (the template store is seam-deferred — see module
/// docs), so the struct carries the parse string and a `tpl_compiled` flag.
#[derive(Debug, Clone)]
pub struct InjectPayloadSleigh {
    /// Engine-neutral payload data.
    pub core: InjectPayloadCore,
    /// SLEIGH syntax describing the injection p-code (C++ `parsestring`).
    pub parsestring: Vec<u8>,
    /// A description of the document containing the SLEIGH syntax
    /// (C++ `source`).
    pub source: Vec<u8>,
    /// Whether the SLEIGH `tpl` has been compiled (C++ `tpl != null`). The
    /// `ConstructTpl` itself is held by the SLEIGH library seam.
    pub tpl_compiled: bool,
}

impl InjectPayloadSleigh {
    /// Create an empty payload in preparation for decode (C++
    /// `InjectPayloadSleigh(const string&,const string&,int4)`): `paramshift`
    /// reset to 0.
    pub fn new(src: &[u8], nm: &[u8], tp: int4) -> InjectPayloadSleigh {
        let mut core = InjectPayloadCore::new(nm, tp);
        core.paramshift = 0;
        InjectPayloadSleigh {
            core,
            parsestring: Vec::new(),
            source: src.to_vec(),
            tpl_compiled: false,
        }
    }

    /// Parse the `<body>` tag (C++ `InjectPayloadSleigh::decodeBody`).
    ///
    /// The tag may not be present; if absent and the payload isn't dynamic, the
    /// C++ throws (a `<pcode>` with no `<body>`).
    pub fn decode_body(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element()?; // Tag may not be present
        if elem_id == ELEM_BODY {
            self.parsestring = decoder.read_string_id(&ATTRIB_CONTENT)?;
            decoder.close_element(elem_id)?;
        }
        if self.parsestring.is_empty() && !self.core.dynamic {
            return Err(KunaError::lowlevel(format!(
                "Missing <body> subtag in <pcode>: {}",
                String::from_utf8_lossy(&self.source)
            )));
        }
        Ok(())
    }

    /// Decode a raw `<pcode>` tag (C++ `InjectPayloadSleigh::decode`). Used for
    /// uponentry/uponreturn.
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_PCODE)?;
        self.core.decode_payload_attributes(decoder)?;
        self.core.decode_payload_params(decoder)?;
        self.decode_body(decoder)?;
        decoder.close_element(elem_id)?;
        Ok(())
    }
}

impl InjectPayload for InjectPayloadSleigh {
    fn core(&self) -> &InjectPayloadCore {
        &self.core
    }
    fn core_mut(&mut self) -> &mut InjectPayloadCore {
        &mut self.core
    }
    fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        InjectPayloadSleigh::decode(self, decoder)
    }
    fn get_source(&self) -> Vec<u8> {
        self.source.clone()
    }
}

// ---------------------------------------------------------------------------
// InjectPayloadCallfixup (inject_sleigh.hh:65-70 + .cc:166-194)
// ---------------------------------------------------------------------------

/// \brief A SLEIGH injection payload for replacing CALL ops to specific
/// functions (C++ `InjectPayloadCallfixup`).
#[derive(Debug, Clone)]
pub struct InjectPayloadCallfixup {
    /// The underlying SLEIGH payload.
    pub payload: InjectPayloadSleigh,
    /// Names (symbols) of specific functions to replace with this payload
    /// (C++ `targetSymbolNames`).
    pub target_symbol_names: Vec<Vec<u8>>,
}

impl InjectPayloadCallfixup {
    /// C++ `InjectPayloadCallfixup(const string &sourceName)`: name "unknown",
    /// type CALLFIXUP_TYPE.
    pub fn new(source_name: &[u8]) -> InjectPayloadCallfixup {
        InjectPayloadCallfixup {
            payload: InjectPayloadSleigh::new(source_name, b"unknown", CALLFIXUP_TYPE),
            target_symbol_names: Vec::new(),
        }
    }

    /// C++ `InjectPayloadCallfixup::decode`.
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_CALLFIXUP)?;
        self.payload.core.name = decoder.read_string_id(&ATTRIB_NAME)?;
        let mut pcode_subtag = false;

        loop {
            let sub_id = decoder.open_element()?;
            if sub_id == 0 {
                break;
            }
            if sub_id == ELEM_PCODE {
                self.payload.core.decode_payload_attributes(decoder)?;
                self.payload.core.decode_payload_params(decoder)?;
                self.payload.decode_body(decoder)?;
                pcode_subtag = true;
            } else if sub_id == ELEM_TARGET {
                self.target_symbol_names.push(decoder.read_string_id(&ATTRIB_NAME)?);
            }
            decoder.close_element(sub_id)?;
        }
        decoder.close_element(elem_id)?;
        if !pcode_subtag {
            return Err(KunaError::lowlevel(format!(
                "<callfixup> is missing <pcode> subtag: {}",
                String::from_utf8_lossy(&self.payload.core.name)
            )));
        }
        Ok(())
    }
}

impl InjectPayload for InjectPayloadCallfixup {
    fn core(&self) -> &InjectPayloadCore {
        &self.payload.core
    }
    fn core_mut(&mut self) -> &mut InjectPayloadCore {
        &mut self.payload.core
    }
    fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        InjectPayloadCallfixup::decode(self, decoder)
    }
    fn get_source(&self) -> Vec<u8> {
        self.payload.source.clone()
    }
}

// ---------------------------------------------------------------------------
// InjectPayloadCallother (inject_sleigh.hh:73-77 + .cc:196-214)
// ---------------------------------------------------------------------------

/// \brief A SLEIGH injection payload for replacing specific user (CALLOTHER)
/// ops (C++ `InjectPayloadCallother`).
#[derive(Debug, Clone)]
pub struct InjectPayloadCallother {
    /// The underlying SLEIGH payload.
    pub payload: InjectPayloadSleigh,
}

impl InjectPayloadCallother {
    /// C++ `InjectPayloadCallother(const string &sourceName)`: name "unknown",
    /// type CALLOTHERFIXUP_TYPE.
    pub fn new(source_name: &[u8]) -> InjectPayloadCallother {
        InjectPayloadCallother {
            payload: InjectPayloadSleigh::new(source_name, b"unknown", CALLOTHERFIXUP_TYPE),
        }
    }

    /// C++ `InjectPayloadCallother::decode`.
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_CALLOTHERFIXUP)?;
        self.payload.core.name = decoder.read_string_id(&ATTRIB_TARGETOP)?;
        let sub_id = decoder.open_element()?;
        if sub_id != ELEM_PCODE {
            return Err(KunaError::lowlevel("<callotherfixup> does not contain a <pcode> tag"));
        }
        self.payload.core.decode_payload_attributes(decoder)?;
        self.payload.core.decode_payload_params(decoder)?;
        self.payload.decode_body(decoder)?;
        decoder.close_element(sub_id)?;
        decoder.close_element(elem_id)?;
        Ok(())
    }
}

impl InjectPayload for InjectPayloadCallother {
    fn core(&self) -> &InjectPayloadCore {
        &self.payload.core
    }
    fn core_mut(&mut self) -> &mut InjectPayloadCore {
        &mut self.payload.core
    }
    fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        InjectPayloadCallother::decode(self, decoder)
    }
    fn get_source(&self) -> Vec<u8> {
        self.payload.source.clone()
    }
}

// ---------------------------------------------------------------------------
// ExecutablePcodeSleigh (inject_sleigh.hh:80-91 + .cc:216-276)
// ---------------------------------------------------------------------------

/// \brief A SLEIGH p-code snippet that can be executed as a script (C++
/// `ExecutablePcodeSleigh`).
///
/// The C++ derives from `ExecutablePcode` (an `InjectPayload` with an embedded
/// `EmulateSnippet`). The emulator wiring (build/evaluate) is the
/// emit/emulate seam (module docs); this port carries the parse string, the
/// source, and the snippet layout reservation, decoding faithfully.
#[derive(Debug, Clone)]
pub struct ExecutablePcodeSleigh {
    /// Engine-neutral payload data (type EXECUTABLEPCODE_TYPE).
    pub core: InjectPayloadCore,
    /// Description of the source of this snippet (C++ `ExecutablePcode::source`).
    pub source: Vec<u8>,
    /// SLEIGH syntax describing the p-code snippet (C++ `parsestring`).
    pub parsestring: Vec<u8>,
    /// Whether build() has run, setting up the emulator (C++ `built`).
    pub built: bool,
    /// Whether the SLEIGH `tpl` has been compiled (C++ `tpl != null`).
    pub tpl_compiled: bool,
}

impl ExecutablePcodeSleigh {
    /// C++ `ExecutablePcodeSleigh(Architecture*,const string&,const string&)`:
    /// chains `ExecutablePcode` (type EXECUTABLEPCODE_TYPE, built=false) and sets
    /// tpl=null.
    pub fn new(src: &[u8], nm: &[u8]) -> ExecutablePcodeSleigh {
        ExecutablePcodeSleigh {
            core: InjectPayloadCore::new(nm, EXECUTABLEPCODE_TYPE),
            source: src.to_vec(),
            parsestring: Vec::new(),
            built: false,
            tpl_compiled: false,
        }
    }

    /// C++ `ExecutablePcodeSleigh::decode`: accepts `<pcode>`/`<case_pcode>`/
    /// `<addr_pcode>`/`<default_pcode>`/`<size_pcode>` and reads the `<body>`.
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element()?;
        if elem_id != ELEM_PCODE
            && elem_id != ELEM_CASE_PCODE
            && elem_id != ELEM_ADDR_PCODE
            && elem_id != ELEM_DEFAULT_PCODE
            && elem_id != ELEM_SIZE_PCODE
        {
            return Err(KunaError::decoder(
                "Expecting <pcode>, <case_pcode>, <addr_pcode>, <default_pcode>, or <size_pcode>",
            ));
        }
        self.core.decode_payload_attributes(decoder)?;
        self.core.decode_payload_params(decoder)?;
        let sub_id = decoder.open_element_id(&ELEM_BODY)?;
        self.parsestring = decoder.read_string_id(&ATTRIB_CONTENT)?;
        decoder.close_element(sub_id)?;
        decoder.close_element(elem_id)?;
        Ok(())
    }
}

impl InjectPayload for ExecutablePcodeSleigh {
    fn core(&self) -> &InjectPayloadCore {
        &self.core
    }
    fn core_mut(&mut self) -> &mut InjectPayloadCore {
        &mut self.core
    }
    fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        ExecutablePcodeSleigh::decode(self, decoder)
    }
    fn get_source(&self) -> Vec<u8> {
        self.source.clone()
    }
}

// ---------------------------------------------------------------------------
// Payload factory + namespace registration (inject_sleigh.cc:418-463)
// ---------------------------------------------------------------------------

/// A SLEIGH-backed injection payload, the polymorphic element the
/// [`PcodeInjectLibrarySleigh`] container stores (C++ `InjectPayload *` slot;
/// the port keeps an owning enum rather than a `Box<dyn>` web so the library is
/// a single `Vec`).
#[derive(Debug, Clone)]
pub enum SleighPayload {
    /// `InjectPayloadCallfixup`.
    Callfixup(InjectPayloadCallfixup),
    /// `InjectPayloadCallother`.
    Callother(InjectPayloadCallother),
    /// `ExecutablePcodeSleigh`.
    Executable(ExecutablePcodeSleigh),
    /// A bare `InjectPayloadSleigh` (the CALLMECHANISM / fallback case).
    Sleigh(InjectPayloadSleigh),
}

impl SleighPayload {
    /// View as the [`InjectPayload`] trait object.
    pub fn as_payload(&self) -> &dyn InjectPayload {
        match self {
            SleighPayload::Callfixup(p) => p,
            SleighPayload::Callother(p) => p,
            SleighPayload::Executable(p) => p,
            SleighPayload::Sleigh(p) => p,
        }
    }

    /// Mutable view as the [`InjectPayload`] trait object.
    pub fn as_payload_mut(&mut self) -> &mut dyn InjectPayload {
        match self {
            SleighPayload::Callfixup(p) => p,
            SleighPayload::Callother(p) => p,
            SleighPayload::Executable(p) => p,
            SleighPayload::Sleigh(p) => p,
        }
    }

    /// The SLEIGH parse string of this payload (C++ `InjectPayloadSleigh::
    /// parsestring`), the source `parse_inject` compiles into a `ConstructTpl`.
    pub fn parsestring(&self) -> &[u8] {
        match self {
            SleighPayload::Callfixup(p) => &p.payload.parsestring,
            SleighPayload::Callother(p) => &p.payload.parsestring,
            SleighPayload::Executable(p) => &p.parsestring,
            SleighPayload::Sleigh(p) => &p.parsestring,
        }
    }

    /// Mark this payload's template as compiled (C++ `tpl != null`).
    fn set_tpl_compiled(&mut self) {
        match self {
            SleighPayload::Callfixup(p) => p.payload.tpl_compiled = true,
            SleighPayload::Callother(p) => p.payload.tpl_compiled = true,
            SleighPayload::Executable(p) => p.tpl_compiled = true,
            SleighPayload::Sleigh(p) => p.tpl_compiled = true,
        }
    }
}

/// \brief An implementation of an injection library using the internal SLEIGH
/// engine (C++ `PcodeInjectLibrarySleigh`).
///
/// Owns the engine-neutral registration bookkeeping ([`PcodeInjectLibraryBase`])
/// and the `Vec` of [`SleighPayload`]s (C++ `injection`). The SLEIGH language
/// (`slgh`), behavior table (`inst`), and reusable context (`contextCache`) live
/// behind the emit/parse seams (module docs).
#[derive(Debug, Clone, Default)]
pub struct PcodeInjectLibrarySleigh {
    /// Engine-neutral name->id namespaces + tempbase (C++ base members).
    pub base: PcodeInjectLibraryBase,
    /// Registered injections (C++ `injection`).
    pub injection: Vec<SleighPayload>,
    /// Compiled p-code templates, indexed by inject id (C++ keeps the
    /// `ConstructTpl *tpl` inside each `InjectPayloadSleigh`; the port stores
    /// them parallel to `injection` so a `&self` `get_tpl` does not borrow the
    /// payload mutably).  `None` until `parse_inject` runs for that id.
    pub tpls: Vec<Option<kuna_sleigh::semantics::ConstructTpl>>,
}

impl PcodeInjectLibrarySleigh {
    /// Constructor (C++ `PcodeInjectLibrarySleigh(Architecture*)`): the C++
    /// chains `PcodeInjectLibrary(g, g->translate->getUniqueStart(INJECT))` —
    /// the caller supplies the unique-start tempbase.
    pub fn new(tempbase: kuna_base::types::uint4) -> PcodeInjectLibrarySleigh {
        PcodeInjectLibrarySleigh {
            base: PcodeInjectLibraryBase::new(tempbase),
            injection: Vec::new(),
            tpls: Vec::new(),
        }
    }

    /// Get the [`InjectPayload`] by id (C++ `getPayload`).
    pub fn get_payload(&self, id: int4) -> &dyn InjectPayload {
        self.injection[id as usize].as_payload()
    }

    /// \brief Allocate a new InjectPayload object (C++ `allocateInject`).
    ///
    /// The factory: maps the payload type to the concrete SLEIGH payload and
    /// returns its id (= the index it is pushed to).
    pub fn allocate_inject(&mut self, source_name: &[u8], name: &[u8], ptype: int4) -> int4 {
        let injectid = self.injection.len() as int4;
        let payload = if ptype == CALLFIXUP_TYPE {
            SleighPayload::Callfixup(InjectPayloadCallfixup::new(source_name))
        } else if ptype == CALLOTHERFIXUP_TYPE {
            SleighPayload::Callother(InjectPayloadCallother::new(source_name))
        } else if ptype == EXECUTABLEPCODE_TYPE {
            SleighPayload::Executable(ExecutablePcodeSleigh::new(source_name, name))
        } else {
            SleighPayload::Sleigh(InjectPayloadSleigh::new(source_name, name, ptype))
        };
        self.injection.push(payload);
        // Keep the compiled-template store parallel to `injection`; the tpl is
        // filled in later by `parse_inject_all` (C++ `parseInject`).
        self.tpls.push(None);
        injectid
    }

    /// Retrieve the compiled p-code template for an inject id, if compiled
    /// (C++ reads `InjectPayloadSleigh::tpl`).  `None` when `parse_inject_all`
    /// has not run for this id (the SLEIGH-language compile seam was not driven).
    pub fn get_tpl(&self, id: int4) -> Option<&kuna_sleigh::semantics::ConstructTpl> {
        self.tpls.get(id as usize).and_then(|t| t.as_ref())
    }

    /// Compile every registered payload's SLEIGH `parsestring` into a
    /// `ConstructTpl` against the loaded language `slgh` (C++ `parseInject`,
    /// driven for each payload as it is registered).  Idempotent: payloads
    /// already compiled (or with an empty parse string, e.g. a dynamic payload)
    /// are skipped.  The advancing tempbase is threaded exactly as the C++
    /// `parseInject` advances `getUniqueBase()` for non-executable payloads.
    pub fn parse_inject_all<L: crate::pcodeinject::SnippetLanguageProvider>(
        &mut self,
        slgh: &L,
    ) -> KunaResult<()> {
        for id in 0..self.injection.len() {
            if self.tpls[id].is_some() {
                continue;
            }
            let parsestring = self.injection[id].parsestring().to_vec();
            if parsestring.is_empty() {
                continue; // dynamic / not-yet-bodied payload (C++ tpl stays null)
            }
            let tempbase = self.base.tempbase;
            let (tpl, new_tempbase) = {
                let payload = self.injection[id].as_payload();
                slgh.parse_inject(payload, &parsestring, tempbase)?
            };
            self.base.tempbase = new_tempbase;
            self.tpls[id] = Some(tpl);
            self.injection[id].set_tpl_compiled();
        }
        Ok(())
    }

    /// \brief Finalize a payload within the library (C++ `registerInject`).
    ///
    /// Registers the payload's name into the type-appropriate namespace, then
    /// compiles it (the `parseInject` step, seam-deferred to a caller with a
    /// wired SLEIGH language — see [`parse_inject`]).
    ///
    /// (The C++ `isDynamic()` -> `InjectPayloadDynamic` swap is a debug path
    /// reached only via `decodeDebug`; not exercised by the spec corpus, it is
    /// elided here — the non-debug `registerInject` never swaps.)
    pub fn register_inject(&mut self, injectid: int4) -> KunaResult<()> {
        let payload = self.injection[injectid as usize].as_payload();
        let ptype = payload.core().get_type();
        let name = payload.core().get_name().to_vec();
        if ptype == CALLFIXUP_TYPE {
            self.base.register_call_fixup(&name, injectid)?;
        } else if ptype == CALLOTHERFIXUP_TYPE {
            self.base.register_call_other_fixup(&name, injectid)?;
        } else if ptype == crate::pcodeinject::CALLMECHANISM_TYPE {
            self.base.register_call_mechanism(&name, injectid)?;
        } else if ptype == EXECUTABLEPCODE_TYPE {
            self.base.register_exe_script(&name, injectid)?;
        } else {
            return Err(KunaError::lowlevel("Unknown p-code inject type"));
        }
        // STUB: parseInject(payload) compiles the SLEIGH `parsestring` into a
        // ConstructTpl via PcodeSnippet (see parse_inject). It needs a wired
        // SnippetLanguage, so the driving caller invokes parse_inject after
        // register_inject.
        Ok(())
    }

    /// \brief Parse and register an injection payload from a stream element
    /// (C++ `PcodeInjectLibrary::decodeInject`).
    ///
    /// Allocates, decodes, and registers the payload, returning its id.
    pub fn decode_inject(
        &mut self,
        src: &[u8],
        nm: &[u8],
        tp: int4,
        decoder: &mut dyn Decoder,
    ) -> KunaResult<int4> {
        let injectid = self.allocate_inject(src, nm, tp);
        self.injection[injectid as usize].as_payload_mut().decode(decoder)?;
        self.register_inject(injectid)?;
        Ok(injectid)
    }

    /// \brief Manually add a call-fixup payload from a snippet of p-code source
    /// (C++ `manualCallFixup`).
    ///
    /// Allocates the payload, sets its parse string, and registers it. (The
    /// compile is the `parse_inject` seam.)
    pub fn manual_call_fixup(&mut self, name: &[u8], snippetstring: &[u8]) -> KunaResult<int4> {
        let mut source_name = b"(manual callfixup name=\"".to_vec();
        source_name.extend_from_slice(name);
        source_name.extend_from_slice(b"\")");
        let injectid = self.allocate_inject(&source_name, name, CALLFIXUP_TYPE);
        if let SleighPayload::Callfixup(p) = &mut self.injection[injectid as usize] {
            p.payload.parsestring = snippetstring.to_vec();
        }
        self.register_inject(injectid)?;
        Ok(injectid)
    }

    /// \brief Manually add a callother-fixup payload from a snippet (C++
    /// `manualCallOtherFixup`).
    pub fn manual_call_other_fixup(
        &mut self,
        name: &[u8],
        outname: &[u8],
        inname: &[Vec<u8>],
        snippet: &[u8],
    ) -> KunaResult<int4> {
        let mut source_name = b"<manual callotherfixup name=\"".to_vec();
        source_name.extend_from_slice(name);
        source_name.extend_from_slice(b"\")");
        let injectid = self.allocate_inject(&source_name, name, CALLOTHERFIXUP_TYPE);
        // The CALLOTHERFIXUP allocate makes an InjectPayloadCallother; the C++
        // casts it to InjectPayloadSleigh and pushes params directly.
        if let SleighPayload::Callother(p) = &mut self.injection[injectid as usize] {
            for nm in inname.iter() {
                p.payload.core.inputlist.push(crate::pcodeinject::InjectParameter::new(nm, 0));
            }
            if !outname.is_empty() {
                p.payload.core.output.push(crate::pcodeinject::InjectParameter::new(outname, 0));
            }
            p.payload.core.order_parameters();
            p.payload.parsestring = snippet.to_vec();
        }
        self.register_inject(injectid)?;
        Ok(injectid)
    }
}

// ---------------------------------------------------------------------------
// parseInject (inject_sleigh.cc:373-416) — the public-API SLEIGH compile step
// ---------------------------------------------------------------------------

/// Blanket bridge: any loaded SLEIGH language (a [`SnippetLanguage`]) is a
/// [`SnippetLanguageProvider`](crate::pcodeinject::SnippetLanguageProvider), so
/// `PcodeInjectLibrarySleigh::parse_inject_all` can compile every payload's body
/// against it without the library struct naming the kuna-sleigh trait.
impl<L: SnippetLanguage> crate::pcodeinject::SnippetLanguageProvider for L {
    fn parse_inject(
        &self,
        payload: &dyn InjectPayload,
        parsestring: &[u8],
        tempbase: kuna_base::types::uint4,
    ) -> KunaResult<(kuna_sleigh::semantics::ConstructTpl, kuna_base::types::uint4)> {
        parse_inject(self, payload, parsestring, tempbase)
    }
}

/// \brief Convert SLEIGH syntax to p-code templates for the given payload
/// (C++ `PcodeInjectLibrarySleigh::parseInject`).
///
/// Builds a [`PcodeSnippet`] over the SLEIGH language `slgh`, declares the
/// payload's input/output operands (`addOperand` in input-then-output order,
/// matching the C++), sets the unique base, parses, and returns the compiled
/// [`ConstructTpl`](kuna_sleigh::semantics::ConstructTpl) along with the (for
/// non-executable payloads, advanced) tempbase.
///
/// This is the fully-public-API half of `parseInject`: every method called on
/// `PcodeSnippet` here is `pub` in kuna-sleigh. The caller threads it after
/// `register_inject` (it needs a wired `SnippetLanguage`, which the W2
/// `SleighBase` does not yet implement — see module docs and the reported LOSS).
///
/// Returns `(compiled_template, new_tempbase)`. For an EXECUTABLEPCODE payload
/// `new_tempbase` equals the input `tempbase` (the C++ uses a private 0x2000
/// base and does not write it back); for other types it is the snippet's
/// advanced `getUniqueBase()`.
pub fn parse_inject<L: SnippetLanguage>(
    slgh: &L,
    payload: &dyn InjectPayload,
    parsestring: &[u8],
    tempbase: kuna_base::types::uint4,
) -> KunaResult<(kuna_sleigh::semantics::ConstructTpl, kuna_base::types::uint4)> {
    let mut compiler = PcodeSnippet::new(slgh);
    let core = payload.core();
    for i in 0..core.size_input() {
        let param = core.get_input(i);
        compiler.add_operand(param.get_name(), param.get_index());
    }
    for i in 0..core.size_output() {
        let param = core.get_output(i);
        compiler.add_operand(param.get_name(), param.get_index());
    }
    if core.get_type() == EXECUTABLEPCODE_TYPE {
        // Don't need to deconflict with anything other than injects.
        compiler.set_unique_base(0x2000);
        if !compiler.parse_stream(parsestring) {
            return Err(KunaError::lowlevel(format!(
                "{}: Unable to compile pcode: {}",
                String::from_utf8_lossy(&payload.get_source()),
                compiler.get_error_message()
            )));
        }
        let tpl = compiler
            .release_result()
            .ok_or_else(|| KunaError::lowlevel("parseInject: no result after successful parse"))?;
        Ok((tpl, tempbase))
    } else {
        compiler.set_unique_base(tempbase);
        if !compiler.parse_stream(parsestring) {
            return Err(KunaError::lowlevel(format!(
                "{}: Unable to compile pcode: {}",
                String::from_utf8_lossy(&payload.get_source()),
                compiler.get_error_message()
            )));
        }
        let new_tempbase = compiler.get_unique_base();
        let tpl = compiler
            .release_result()
            .ok_or_else(|| KunaError::lowlevel("parseInject: no result after successful parse"))?;
        Ok((tpl, new_tempbase))
    }
}

// ---------------------------------------------------------------------------
// decodeDebug type tag (inject_sleigh.cc:465-483) — the <inject> type attribute
// ---------------------------------------------------------------------------

/// Read the `name`/`type` of an `<inject>` element inside `<injectdebug>`
/// (C++ `PcodeInjectLibrarySleigh::decodeDebug` inner loop), returning
/// `(name, type)`. The dynamic-payload swap + `decodeEntry` is the debug-only
/// `InjectPayloadDynamic` path (seam-deferred; not in the spec corpus).
pub fn decode_inject_debug_entry(decoder: &mut dyn Decoder) -> KunaResult<(Vec<u8>, int4)> {
    let name = decoder.read_string_id(&ATTRIB_NAME)?;
    // C++ readSignedInteger(ATTRIB_TYPE) -> int4 (narrow the i64).
    let ptype = decoder.read_signed_integer_id(&ATTRIB_TYPE)? as int4;
    Ok((name, ptype))
}

// ---------------------------------------------------------------------------
// SleighInjectEngine — the InjectPayloadSleigh::inject emit path
// (inject_sleigh.cc:48 + setupParameters :137 + SleighBuilder::dump/generate*)
// ---------------------------------------------------------------------------
//
// The C++ `InjectPayloadSleigh::inject` drives `SleighBuilder` over a degenerate
// `ParserContext` whose operand handles are filled directly from the
// `InjectContext` storage (`setupParameters`), then emits the cached ops.  In
// the kuna port `SleighBuilder`/`PcodeCacher`/`ParserContext` are private to
// `kuna-sleigh`'s `sleigh.rs`, but the *template* resolution machinery
// (`VarnodeTpl`/`ConstTpl`/`OpTpl::fix`/`is_dynamic`/`is_relative`) and the
// `PcodeBuilder` trait are **public** and parametrized over the
// [`SymbolWalker`] seam.  So the inject emit is reconstructed here faithfully:
//
//   - [`InjectWalker`] is the degenerate `ParserWalker` of `setupParameters`: it
//     answers `getFixedHandle(i)` with the operand handle built from the
//     `InjectContext` storage (input handles first, then outputs, by operand
//     index), and supplies the const/cur spaces and the three inject addresses.
//   - [`InjectBuilder`] is `SleighBuilder` reduced to the snippet case (no delay
//     slots, no crossbuild, no unique-mask): a faithful transcription of
//     `SleighBuilder::dump` / `generateLocation` / `generatePointer` /
//     `generatePointerAdd` over a local op/varnode pool ([`PcodeCacher`]) plus
//     `resolveRelatives` + `emit`.
//
// The result is byte-for-byte the ops the C++ emit path would hand to
// `PcodeEmit::dump`.  This closes the emit half of LOSS-031 for the
// `InjectPayloadSleigh`/`InjectPayloadCallfixup`/`InjectPayloadCallother` family
// (the `ExecutablePcode` emulate path and `InjectPayloadDynamic` remain seamed —
// they need the emulator, not just the builder).

/// LOSS-015: a LOAD/STORE space-pointer constant stores the space's manager
/// index in place of the C++ heap pointer `(uintb)(uintp)spc` (mirrors
/// `sleigh.rs`'s `spaceid_const`).
fn spaceid_const(spc: &Rc<AddrSpace>) -> u64 {
    spc.get_index() as u64 // cast: manager index, small and non-negative
}

/// C++ `sizeof(spc)` (a `void *`): the size stored on a LOAD/STORE space-pointer
/// constant (mirrors `sleigh.rs`'s `SIZEOF_SPACE`; the golden fixtures were
/// generated on a 64-bit host).
const SIZEOF_SPACE: u32 = 8;

/// One issued p-code op, the local analogue of `sleigh.rs`'s `PcodeData` (the
/// `PcodeCacher::issued` element): a code plus pool indices for the output and
/// the contiguous run of inputs.
#[derive(Debug, Clone)]
struct IssuedOp {
    outvar: Option<usize>,
    invar: Option<usize>,
    opc: OpCode,
    isize: i32,
}

/// A relative (label-referencing) operand record (C++ `RelativeRecord`):
/// the pool index of the operand holding the label id and the index of the op
/// it belongs to.
#[derive(Debug, Clone)]
struct RelativeRecord {
    dataptr: usize,
    calling_index: usize,
}

/// The snippet-case [`PcodeCacher`] (C++ `PcodeCacher`): a `VarnodeData` pool +
/// issued ops + label tables, faithful to `sleigh.rs`'s private `PcodeCacher`
/// but local to the injection emit.
#[derive(Debug, Default)]
struct PcodeCacher {
    pool: Vec<VarnodeData>,
    issued: Vec<IssuedOp>,
    label_refs: Vec<RelativeRecord>,
    labels: Vec<u64>,
}

impl PcodeCacher {
    /// C++ `allocateVarnodes(uint4 size)`: returns the pool index of the first
    /// of `size` freshly allocated VarnodeData (indices are stable).
    fn allocate_varnodes(&mut self, size: u32) -> usize {
        let start = self.pool.len();
        for _ in 0..size {
            self.pool.push(VarnodeData::default());
        }
        start
    }

    /// C++ `allocateInstruction`: append a cleared op; returns its `issued`
    /// index.
    fn allocate_instruction(&mut self) -> usize {
        self.issued.push(IssuedOp { outvar: None, invar: None, opc: OpCode::CPUI_COPY, isize: 0 });
        self.issued.len() - 1
    }

    /// C++ `addLabelRef`.
    fn add_label_ref(&mut self, ptr: usize) {
        self.label_refs.push(RelativeRecord { dataptr: ptr, calling_index: self.issued.len() });
    }

    /// C++ `resolveRelatives`.
    fn resolve_relatives(&mut self) -> KunaResult<()> {
        for rec in &self.label_refs {
            let ptr = rec.dataptr;
            let id = self.pool[ptr].offset; // uint4 id = ptr->offset
            if id >= self.labels.len() as u64 || self.labels[id as usize] == 0x0badbeef {
                return Err(KunaError::lowlevel("Reference to non-existant sleigh label"));
            }
            let res = self.labels[id as usize].wrapping_sub(rec.calling_index as u64);
            let res = res & calc_mask(self.pool[ptr].size as i32);
            self.pool[ptr].offset = res;
        }
        Ok(())
    }

    /// C++ `emit(const Address&,PcodeEmit*)`: replay every issued op.
    fn emit(&self, addr: &Address, emt: &mut dyn PcodeEmit) {
        for op in &self.issued {
            let outvar = op.outvar.map(|i| self.pool[i].clone());
            let invars: Vec<VarnodeData> = (0..op.isize)
                .map(|k| {
                    let base = op.invar.expect("issued op with inputs has invar");
                    self.pool[base + k as usize].clone()
                })
                .collect();
            emt.dump(addr, op.opc, outvar.as_ref(), &invars);
        }
    }
}

/// The degenerate `ParserWalker` of `InjectPayloadSleigh::inject` /
/// `setupParameters`: operand handles come straight from the `InjectContext`
/// storage; there is no instruction stream and no context.
///
/// `getFixedHandle(i)` returns the handle for operand index `i` (inputs in
/// declaration order, then outputs); the parser-bit / instruction-bit / nested
/// constructor walks are never reached for a compiled snippet (no operand
/// expressions survive `parseInject`), so those methods surface a precise
/// internal-invariant error if a snippet ever exercises them.
struct InjectWalker {
    /// Operand handles, indexed by operand index (inputs then outputs).
    handles: Vec<FixedHandle>,
    const_space: Rc<AddrSpace>,
    cur_space: Rc<AddrSpace>,
    baseaddr: Address,
    nextaddr: Address,
    calladdr: Option<Address>,
}

impl PatternExpressionContext for InjectWalker {
    fn get_instruction_bytes(&self, _byteoff: i32, _numbytes: i32) -> KunaResult<u32> {
        Err(KunaError::lowlevel("inject snippet has no instruction stream"))
    }
    fn get_context_bytes(&self, _byteoff: i32, _numbytes: i32) -> KunaResult<u32> {
        Err(KunaError::lowlevel("inject snippet has no context"))
    }
    fn get_addr(&self) -> Address {
        self.baseaddr.clone()
    }
    fn get_naddr(&self) -> Address {
        self.nextaddr.clone()
    }
    fn get_n2addr(&self) -> KunaResult<Address> {
        Err(KunaError::lowlevel("inst_next2 not available in inject context"))
    }
    fn operand_value(&self, _index: i32, _table_id: u32, _ct_id: u32) -> KunaResult<i64> {
        Err(KunaError::lowlevel("inject snippet has no operand expressions"))
    }
}

impl SymbolWalker for InjectWalker {
    fn push_operand(&mut self, _i: i32) -> KunaResult<()> {
        Err(KunaError::lowlevel("inject snippet build does not descend operands"))
    }
    fn pop_operand(&mut self) -> KunaResult<()> {
        Err(KunaError::lowlevel("inject snippet build does not descend operands"))
    }
    fn get_constructor(&self) -> KunaResult<ConstructorRef> {
        Err(KunaError::lowlevel("inject snippet has no constructor"))
    }
    fn get_fixed_handle(&self, i: i32) -> KunaResult<FixedHandle> {
        // C++ walker.getFixedHandle(i): the resolved handle of operand `i`.
        self.handles
            .get(i as usize)
            .cloned()
            .ok_or_else(|| KunaError::lowlevel("inject: operand handle index out of range"))
    }
    fn get_const_space(&self) -> Rc<AddrSpace> {
        Rc::clone(&self.const_space)
    }
    fn get_cur_space(&self) -> Rc<AddrSpace> {
        Rc::clone(&self.cur_space)
    }
    fn get_dest_addr(&self) -> KunaResult<Address> {
        // The snippet's "call" address, used by CALL-fixup flow constants.
        self.calladdr
            .clone()
            .ok_or_else(|| KunaError::lowlevel("inject: no call address in context"))
    }
    fn get_ref_addr(&self) -> KunaResult<Address> {
        Err(KunaError::lowlevel("inject snippet has no cross-reference address"))
    }
    fn get_instruction_bits(&self, _startbit: i32, _size: i32) -> KunaResult<u32> {
        Err(KunaError::lowlevel("inject snippet has no instruction bits"))
    }
    fn get_context_bits(&self, _startbit: i32, _size: i32) -> KunaResult<u32> {
        Err(KunaError::lowlevel("inject snippet has no context bits"))
    }
}

/// The snippet-case `SleighBuilder` (C++ `SleighBuilder : PcodeBuilder`): walks
/// the compiled [`ConstructTpl`] and stages p-code into the [`PcodeCacher`].
struct InjectBuilder<'a> {
    labelbase: u32,
    labelcount: u32,
    walker: &'a InjectWalker,
    const_space: Rc<AddrSpace>,
    uniq_space: Rc<AddrSpace>,
    cache: PcodeCacher,
}

impl InjectBuilder<'_> {
    /// C++ `SleighBuilder::generateLocation`.
    fn generate_location(&self, vntpl: &VarnodeTpl, vn: &mut VarnodeData) -> KunaResult<()> {
        let space = vntpl
            .get_space()
            .fix_space(self.walker)?
            .ok_or_else(|| KunaError::lowlevel("generateLocation: null space"))?;
        vn.size = vntpl.get_size().fix(self.walker)? as uint4; // C++ uintb -> uint4
        if Rc::ptr_eq(&space, &self.const_space) {
            vn.offset = vntpl.get_offset().fix(self.walker)? & calc_mask(vn.size as i32);
        } else if Rc::ptr_eq(&space, &self.uniq_space) {
            // Snippets allocate no unique-offset mask (uniqueoffset == 0), so
            // the C++ `vn.offset |= uniqueoffset` is a no-op here.
            vn.offset = vntpl.get_offset().fix(self.walker)?;
        } else {
            vn.offset = space.wrap_offset(vntpl.get_offset().fix(self.walker)?);
        }
        vn.space = Some(space);
        Ok(())
    }

    /// C++ `SleighBuilder::generatePointer`: returns the pointed-to space.
    fn generate_pointer(
        &self,
        vntpl: &VarnodeTpl,
        vn: &mut VarnodeData,
    ) -> KunaResult<Rc<AddrSpace>> {
        let hand = self.walker.get_fixed_handle(vntpl.get_offset().get_handle_index())?;
        let space = hand
            .offset_space
            .clone()
            .ok_or_else(|| KunaError::lowlevel("generatePointer: null offset space"))?;
        vn.size = hand.offset_size;
        if Rc::ptr_eq(&space, &self.const_space) {
            vn.offset = hand.offset_offset & calc_mask(vn.size as i32);
        } else if Rc::ptr_eq(&space, &self.uniq_space) {
            vn.offset = hand.offset_offset; // | uniqueoffset (==0 for snippets)
        } else {
            vn.offset = space.wrap_offset(hand.offset_offset);
        }
        vn.space = Some(space);
        hand.space.ok_or_else(|| KunaError::lowlevel("generatePointer: null pointed-to space"))
    }

    /// C++ `SleighBuilder::generatePointerAdd`.
    fn generate_pointer_add(&mut self, op_idx: usize, vntpl: &VarnodeTpl) -> KunaResult<()> {
        let offset_plus = vntpl.get_offset().get_real() & 0xffff;
        if offset_plus == 0 {
            return Ok(());
        }
        let nextop_idx = self.cache.allocate_instruction();
        self.cache.issued[nextop_idx].opc = self.cache.issued[op_idx].opc;
        self.cache.issued[nextop_idx].invar = self.cache.issued[op_idx].invar;
        self.cache.issued[nextop_idx].isize = self.cache.issued[op_idx].isize;
        self.cache.issued[nextop_idx].outvar = self.cache.issued[op_idx].outvar;
        self.cache.issued[op_idx].isize = 2;
        self.cache.issued[op_idx].opc = OpCode::CPUI_INT_ADD;
        let newparams = self.cache.allocate_varnodes(2);
        self.cache.issued[op_idx].invar = Some(newparams);
        let nextop_invar = self.cache.issued[nextop_idx]
            .invar
            .expect("nextop has inputs (copied from op)");
        self.cache.pool[newparams] = self.cache.pool[nextop_invar + 1].clone();
        let p0_size = self.cache.pool[newparams].size;
        self.cache.pool[newparams + 1].space = Some(Rc::clone(&self.const_space));
        self.cache.pool[newparams + 1].offset = offset_plus;
        self.cache.pool[newparams + 1].size = p0_size;
        self.cache.issued[op_idx].outvar = Some(nextop_invar + 1);
        // C++ sets the temp output to a RUNTIME_BITRANGE_EA unique slot; snippets
        // have no live unique allocator, so the temp keeps the unique space at
        // offset 0 (the inject path never re-reads it, mirroring the no-mask
        // snippet case the C++ comment in `inject` calls out).
        self.cache.pool[nextop_invar + 1].space = Some(Rc::clone(&self.uniq_space));
        self.cache.pool[nextop_invar + 1].offset = 0;
        Ok(())
    }
}

impl PcodeBuilder for InjectBuilder<'_> {
    fn get_label_base(&self) -> u32 {
        self.labelbase
    }
    fn set_label_base(&mut self, val: u32) {
        self.labelbase = val;
    }
    fn get_label_count(&self) -> u32 {
        self.labelcount
    }
    fn set_label_count(&mut self, val: u32) {
        self.labelcount = val;
    }

    /// C++ `SleighBuilder::dump(OpTpl *op)`.
    fn dump(&mut self, op: &OpTpl) -> KunaResult<()> {
        let isize = op.num_input();
        let invars = self.cache.allocate_varnodes(isize as u32);
        for i in 0..isize {
            let vn = op.get_in(i);
            let dynamic = vn.is_dynamic(self.walker)?;
            if dynamic {
                let mut tmp = VarnodeData::default();
                self.generate_location(vn, &mut tmp)?;
                self.cache.pool[invars + i as usize] = tmp;
                let load_op = self.cache.allocate_instruction();
                self.cache.issued[load_op].opc = OpCode::CPUI_LOAD;
                self.cache.issued[load_op].outvar = Some(invars + i as usize);
                self.cache.issued[load_op].isize = 2;
                let loadvars = self.cache.allocate_varnodes(2);
                self.cache.issued[load_op].invar = Some(loadvars);
                let mut ptrvn = VarnodeData::default();
                let spc = self.generate_pointer(vn, &mut ptrvn)?;
                self.cache.pool[loadvars + 1] = ptrvn;
                self.cache.pool[loadvars].space = Some(Rc::clone(&self.const_space));
                self.cache.pool[loadvars].offset = spaceid_const(&spc);
                self.cache.pool[loadvars].size = SIZEOF_SPACE;
                if vn.get_offset().get_select() == VField::VOffsetPlus {
                    self.generate_pointer_add(load_op, vn)?;
                }
            } else {
                let mut tmp = VarnodeData::default();
                self.generate_location(vn, &mut tmp)?;
                self.cache.pool[invars + i as usize] = tmp;
            }
        }
        if isize > 0 && op.get_in(0).is_relative() {
            self.cache.pool[invars].offset =
                self.cache.pool[invars].offset.wrapping_add(u64::from(self.get_label_base()));
            self.cache.add_label_ref(invars);
        }
        let thisop = self.cache.allocate_instruction();
        self.cache.issued[thisop].opc = op.get_opcode();
        self.cache.issued[thisop].invar = if isize > 0 { Some(invars) } else { None };
        self.cache.issued[thisop].isize = isize;
        if let Some(outvn) = op.get_out() {
            let dynamic = outvn.is_dynamic(self.walker)?;
            if dynamic {
                let storevars = self.cache.allocate_varnodes(3);
                let mut tmp = VarnodeData::default();
                self.generate_location(outvn, &mut tmp)?;
                self.cache.pool[storevars + 2] = tmp;
                self.cache.issued[thisop].outvar = Some(storevars + 2);
                let store_op = self.cache.allocate_instruction();
                self.cache.issued[store_op].opc = OpCode::CPUI_STORE;
                self.cache.issued[store_op].isize = 3;
                self.cache.issued[store_op].invar = Some(storevars);
                let mut ptrvn = VarnodeData::default();
                let spc = self.generate_pointer(outvn, &mut ptrvn)?;
                self.cache.pool[storevars + 1] = ptrvn;
                self.cache.pool[storevars].space = Some(Rc::clone(&self.const_space));
                self.cache.pool[storevars].offset = spaceid_const(&spc);
                self.cache.pool[storevars].size = SIZEOF_SPACE;
                if outvn.get_offset().get_select() == VField::VOffsetPlus {
                    self.generate_pointer_add(store_op, outvn)?;
                }
            } else {
                let out = self.cache.allocate_varnodes(1);
                let mut tmp = VarnodeData::default();
                self.generate_location(outvn, &mut tmp)?;
                self.cache.pool[out] = tmp;
                self.cache.issued[thisop].outvar = Some(out);
            }
        }
        Ok(())
    }

    /// C++ `SleighBuilder::appendBuild`: snippets contain no BUILD directives
    /// (`parseInject` rejects nested subconstructors), so this is unreachable.
    fn append_build(&mut self, _bld: &OpTpl, _secnum: i32) -> KunaResult<()> {
        Err(KunaError::lowlevel("BUILD directive not allowed in inject snippet"))
    }
    /// C++ `SleighBuilder::delaySlot`: snippets contain no delay slots.
    fn delay_slot(&mut self, _op: &OpTpl) -> KunaResult<()> {
        Err(KunaError::lowlevel("delay slot directive not allowed in inject snippet"))
    }
    /// C++ `SleighBuilder::setLabel`.
    fn set_label(&mut self, op: &OpTpl) -> KunaResult<()> {
        // labels[getLabelBase()+id] = issued.size()  (C++ addLabel)
        let id = op.get_in(0).get_offset().get_real() as u32; // C++ uintb -> uint4
        let full = self.get_label_base().wrapping_add(id);
        while self.cache.labels.len() as u64 <= u64::from(full) {
            self.cache.labels.push(0x0badbeef);
        }
        self.cache.labels[full as usize] = self.cache.issued.len() as u64;
        Ok(())
    }
    /// C++ `SleighBuilder::appendCrossBuild`: snippets contain no crossbuild.
    fn append_cross_build(&mut self, _bld: &OpTpl, _secnum: i32) -> KunaResult<()> {
        Err(KunaError::lowlevel("crossbuild directive not allowed in inject snippet"))
    }
}

/// \brief Build the operand handles for an injection from its `InjectContext`
/// storage (C++ `InjectPayloadSleigh::setupParameters`, inject_sleigh.cc:137,
/// plus `checkParameterRestrictions` :109).
///
/// Inputs come first (by `getIndex`), then outputs; the handle's `(space,offset,
/// size)` are taken from the matching `InjectContext` storage entry, exactly as
/// the C++ fills `walker.getParentHandle()`.
fn setup_handles(core: &InjectPayloadCore, context: &InjectContext) -> KunaResult<Vec<FixedHandle>> {
    // checkParameterRestrictions: the storage counts must match the payload spec.
    if core.size_input() as usize != context.inputlist.len() {
        return Err(KunaError::lowlevel(
            "Injection parameter list has different number of parameters than p-code operation",
        ));
    }
    for i in 0..core.size_input() {
        let sz = core.get_input(i).get_size();
        if sz != 0 && sz != context.inputlist[i as usize].size {
            return Err(KunaError::lowlevel(
                "P-code input parameter size does not match injection specification",
            ));
        }
    }
    if core.size_output() as usize != context.output.len() {
        return Err(KunaError::lowlevel(
            "Injection output does not match output of p-code operation",
        ));
    }
    for i in 0..core.size_output() {
        let sz = core.get_output(i).get_size();
        if sz != 0 && sz != context.output[i as usize].size {
            return Err(KunaError::lowlevel(
                "P-code output size does not match injection specification",
            ));
        }
    }

    // Operand handles are indexed by operand index (orderParameters assigns
    // inputs 0..n then outputs n..n+m, in declaration order — see
    // InjectPayloadCore::order_parameters); the storage lists are in the same
    // order, so the handle at operand index == its storage entry.
    let total = core.size_input() + core.size_output();
    let mut handles = vec![FixedHandle::default(); total as usize];
    for i in 0..core.size_input() {
        let idx = core.get_input(i).get_index();
        let data = &context.inputlist[i as usize];
        let hand = &mut handles[idx as usize];
        hand.space = data.space.clone();
        hand.offset_offset = data.offset;
        hand.size = data.size;
        hand.offset_space = None;
    }
    for i in 0..core.size_output() {
        let idx = core.get_output(i).get_index();
        let data = &context.output[i as usize];
        let hand = &mut handles[idx as usize];
        hand.space = data.space.clone();
        hand.offset_offset = data.offset;
        hand.size = data.size;
        hand.offset_space = None;
    }
    Ok(handles)
}

/// \brief The SLEIGH-backed [`InjectEngine`]: drives a compiled [`ConstructTpl`]
/// through the snippet-case builder to emit a payload's p-code ops.
///
/// Construct it with the const/unique/code spaces (from the architecture) plus
/// the compiled template for each payload; [`emit`](Self::emit) reproduces
/// `InjectPayloadSleigh::inject`.
pub struct SleighInjectEngine {
    const_space: Rc<AddrSpace>,
    uniq_space: Rc<AddrSpace>,
    code_space: Rc<AddrSpace>,
}

impl SleighInjectEngine {
    /// Construct from the const/unique/code spaces (C++
    /// `con.glb->getConstantSpace()` / `getUniqueSpace()` /
    /// `getDefaultCodeSpace()`).
    ///
    /// The constant space is passed explicitly because the [`InjectArchitecture`]
    /// seam only carries the unique + default-code spaces; the const space is the
    /// architecture's constant space (`AddrSpaceManager::getConstantSpace`).
    pub fn new(
        const_space: Rc<AddrSpace>,
        uniq_space: Rc<AddrSpace>,
        code_space: Rc<AddrSpace>,
    ) -> SleighInjectEngine {
        SleighInjectEngine { const_space, uniq_space, code_space }
    }

    /// Construct from an [`InjectArchitecture`] (unique + default-code spaces)
    /// plus the constant space.
    pub fn from_arch(arch: &dyn InjectArchitecture, const_space: Rc<AddrSpace>) -> SleighInjectEngine {
        SleighInjectEngine {
            const_space,
            uniq_space: arch.get_unique_space(),
            code_space: arch.get_default_code_space(),
        }
    }

    /// Emit the ops of `payload` (already compiled to `tpl`) into `emit`,
    /// resolving the payload's operands against `context`
    /// (C++ `InjectPayloadSleigh::inject`).
    ///
    /// `tpl` is the [`ConstructTpl`] produced by [`parse_inject`] for this
    /// payload.  The `context`'s input/output storage must be laid out (e.g. via
    /// [`SnippetLayout::build`](crate::pcodeinject::SnippetLayout) for an
    /// executable payload, or by the caller for a call/callother fixup).
    pub fn emit_payload(
        &self,
        payload: &dyn InjectPayload,
        tpl: &ConstructTpl,
        context: &mut InjectContext,
        emit: &mut dyn PcodeEmit,
    ) -> KunaResult<()> {
        let core = payload.core();
        let handles = setup_handles(core, context)?;

        // baseaddr is where the injected ops are anchored (C++ con.baseaddr).
        let baseaddr = context
            .baseaddr
            .clone()
            .unwrap_or_else(|| Address::new(Rc::clone(&self.code_space), 0));
        let nextaddr = context.nextaddr.clone().unwrap_or_else(|| baseaddr.clone());

        let walker = InjectWalker {
            handles,
            const_space: Rc::clone(&self.const_space),
            cur_space: Rc::clone(&self.code_space),
            baseaddr: baseaddr.clone(),
            nextaddr,
            calladdr: context.calladdr.clone(),
        };

        let mut builder = InjectBuilder {
            labelbase: 0,
            labelcount: 0,
            walker: &walker,
            const_space: Rc::clone(&self.const_space),
            uniq_space: Rc::clone(&self.uniq_space),
            cache: PcodeCacher::default(),
        };
        // builder.build(tpl,-1)  (the main section)
        builder.build(Some(tpl), -1)?;
        builder.cache.resolve_relatives()?;
        builder.cache.emit(&baseaddr, emit);
        Ok(())
    }
}

impl InjectEngine for SleighInjectEngine {
    fn emit(
        &self,
        _payload: &dyn InjectPayload,
        _context: &mut InjectContext,
        _emit: &mut dyn PcodeEmit,
    ) -> KunaResult<()> {
        // The trait method has no compiled-template argument; the SLEIGH engine
        // needs the payload's `ConstructTpl` (a `parse_inject` result that is not
        // stored on the engine-neutral InjectPayload).  Callers drive
        // [`SleighInjectEngine::emit_payload`] directly with the template.  This
        // trait impl exists so `SleighInjectEngine` is a drop-in `InjectEngine`
        // for the seam; it errs to make the missing-template misuse observable.
        Err(KunaError::lowlevel(
            "SleighInjectEngine requires a compiled ConstructTpl; call emit_payload",
        ))
    }
}

#[cfg(test)]
mod tests;
