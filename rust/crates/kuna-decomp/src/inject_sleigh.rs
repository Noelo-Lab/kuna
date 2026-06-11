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

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{Decoder, ATTRIB_CONTENT, ATTRIB_NAME, ATTRIB_TYPE, ELEM_TARGET};
use kuna_base::types::int4;

use kuna_sleigh::pcodeparse::{PcodeSnippet, SnippetLanguage};

use crate::pcodeinject::{
    InjectPayload, InjectPayloadCore, PcodeInjectLibraryBase, ATTRIB_TARGETOP, CALLFIXUP_TYPE,
    CALLOTHERFIXUP_TYPE, ELEM_ADDR_PCODE, ELEM_BODY, ELEM_CALLFIXUP, ELEM_CALLOTHERFIXUP,
    ELEM_CASE_PCODE, ELEM_DEFAULT_PCODE, ELEM_PCODE, ELEM_SIZE_PCODE, EXECUTABLEPCODE_TYPE,
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
}

impl PcodeInjectLibrarySleigh {
    /// Constructor (C++ `PcodeInjectLibrarySleigh(Architecture*)`): the C++
    /// chains `PcodeInjectLibrary(g, g->translate->getUniqueStart(INJECT))` —
    /// the caller supplies the unique-start tempbase.
    pub fn new(tempbase: kuna_base::types::uint4) -> PcodeInjectLibrarySleigh {
        PcodeInjectLibrarySleigh {
            base: PcodeInjectLibraryBase::new(tempbase),
            injection: Vec::new(),
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
        injectid
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
        // SEAM: parseInject(payload) compiles the SLEIGH `parsestring` into a
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

#[cfg(test)]
mod tests;
