//! Port of `decompiler/cpp/pcodeinject.{cc,hh}` (item `w3-ir-userop-inject`):
//! the p-code **injection** framework — `InjectParameter`, `InjectContext`,
//! the `InjectPayload` base class, the `ExecutablePcode` snippet/script payload,
//! and the `PcodeInjectLibrary` container of payloads.
//!
//! This is the *engine-neutral* half of injection. `inject_sleigh.rs` provides
//! the SLEIGH-backed concrete payloads/library that drive the W2 kuna-sleigh
//! `PcodeSnippet`/`EmulateSnippet` machinery.
//!
//! ## Seams
//!
//! Several C++ collaborators of `pcodeinject.cc` belong to later waves; this
//! file carries minimal local seam surfaces for exactly the members the C++
//! reaches, each annotated `// SEAM`:
//!   - [`InjectArchitecture`] — the `Architecture *glb` slice
//!     `ExecutablePcode::build`/`evaluate` and `PcodeInjectLibrary` touch
//!     (`getUniqueSpace`, etc.). `Architecture` is W4/W6.
//!   - The actual SLEIGH **emit** path (`InjectPayload::inject`, the emulator
//!     wiring inside `ExecutablePcode::build`) needs kuna-sleigh's
//!     `SleighBuilder`/`PcodeCacher`/`ParserWalkerChange`/`ParserContext`
//!     mutators, which are **private** to `kuna-sleigh`'s `sleigh.rs`. They are
//!     abstracted behind the [`InjectEngine`] trait here; `inject_sleigh.rs`
//!     records the precise missing-public-API note (LOSS reported in output).
//!
//! Per ADR 0004 fallible decode/registration returns `Result<_, KunaError>`;
//! per ADR 0002 the name->id namespaces are `BTreeMap` keyed on the C++
//! byte-string name order. Integer widths follow ADR 0003 (`int4`->`i32`,
//! `uint4`->`u32`, `uintb`->`u64`).

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{
    AttributeId, Decoder, ElementId, IdRegistry, ATTRIB_NAME, ATTRIB_SIZE, ELEM_INPUT, ELEM_OUTPUT,
};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uint4, uintb, Wrap};
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;

// ---------------------------------------------------------------------------
// Marshaling ids (verbatim from pcodeinject.cc:21-39)
// ---------------------------------------------------------------------------

/// Marshaling attribute "dynamic" (`pcodeinject.cc:21`).
pub const ATTRIB_DYNAMIC: AttributeId = AttributeId::new("dynamic", 70);
/// Marshaling attribute "incidentalcopy" (`pcodeinject.cc:22`).
pub const ATTRIB_INCIDENTALCOPY: AttributeId = AttributeId::new("incidentalcopy", 71);
/// Marshaling attribute "inject" (`pcodeinject.cc:23`).
pub const ATTRIB_INJECT: AttributeId = AttributeId::new("inject", 72);
/// Marshaling attribute "paramshift" (`pcodeinject.cc:24`).
pub const ATTRIB_PARAMSHIFT: AttributeId = AttributeId::new("paramshift", 73);
/// Marshaling attribute "targetop" (`pcodeinject.cc:25`).
pub const ATTRIB_TARGETOP: AttributeId = AttributeId::new("targetop", 74);

/// Marshaling element `<addr_pcode>` (`pcodeinject.cc:27`).
pub const ELEM_ADDR_PCODE: ElementId = ElementId::new("addr_pcode", 89);
/// Marshaling element `<body>` (`pcodeinject.cc:28`).
pub const ELEM_BODY: ElementId = ElementId::new("body", 90);
/// Marshaling element `<callfixup>` (`pcodeinject.cc:29`).
pub const ELEM_CALLFIXUP: ElementId = ElementId::new("callfixup", 91);
/// Marshaling element `<callotherfixup>` (`pcodeinject.cc:30`).
pub const ELEM_CALLOTHERFIXUP: ElementId = ElementId::new("callotherfixup", 92);
/// Marshaling element `<case_pcode>` (`pcodeinject.cc:31`).
pub const ELEM_CASE_PCODE: ElementId = ElementId::new("case_pcode", 93);
/// Marshaling element `<context>` (`pcodeinject.cc:32`).
pub const ELEM_CONTEXT: ElementId = ElementId::new("context", 94);
/// Marshaling element `<default_pcode>` (`pcodeinject.cc:33`).
pub const ELEM_DEFAULT_PCODE: ElementId = ElementId::new("default_pcode", 95);
/// Marshaling element `<inject>` (`pcodeinject.cc:34`).
pub const ELEM_INJECT: ElementId = ElementId::new("inject", 96);
/// Marshaling element `<injectdebug>` (`pcodeinject.cc:35`).
pub const ELEM_INJECTDEBUG: ElementId = ElementId::new("injectdebug", 97);
/// Marshaling element `<inst>` (`pcodeinject.cc:36`).
pub const ELEM_INST: ElementId = ElementId::new("inst", 98);
/// Marshaling element `<payload>` (`pcodeinject.cc:37`).
pub const ELEM_PAYLOAD: ElementId = ElementId::new("payload", 99);
/// Marshaling element `<pcode>` (`pcodeinject.cc:38`).
pub const ELEM_PCODE: ElementId = ElementId::new("pcode", 100);
/// Marshaling element `<size_pcode>` (`pcodeinject.cc:39`).
pub const ELEM_SIZE_PCODE: ElementId = ElementId::new("size_pcode", 101);

/// Convenience: register every pcodeinject marshaling id with an [`IdRegistry`]
/// so a name-driven `XmlDecode` resolves `<callfixup>`/`<pcode>`/etc. (the C++
/// global static-init side effect; the Rust registry is explicit).
pub fn register_ids(reg: &mut IdRegistry) {
    for a in [
        &ATTRIB_DYNAMIC,
        &ATTRIB_INCIDENTALCOPY,
        &ATTRIB_INJECT,
        &ATTRIB_PARAMSHIFT,
        &ATTRIB_TARGETOP,
    ] {
        reg.register_attribute(a);
    }
    for e in [
        &ELEM_ADDR_PCODE,
        &ELEM_BODY,
        &ELEM_CALLFIXUP,
        &ELEM_CALLOTHERFIXUP,
        &ELEM_CASE_PCODE,
        &ELEM_CONTEXT,
        &ELEM_DEFAULT_PCODE,
        &ELEM_INJECT,
        &ELEM_INJECTDEBUG,
        &ELEM_INST,
        &ELEM_PAYLOAD,
        &ELEM_PCODE,
        &ELEM_SIZE_PCODE,
    ] {
        reg.register_element(e);
    }
}

// ---------------------------------------------------------------------------
// InjectParameter (pcodeinject.hh:55-65)
// ---------------------------------------------------------------------------

/// \brief An input or output parameter to a p-code injection payload
///
/// Within the chunk of p-code being injected, this is a placeholder for
/// Varnodes that serve as inputs or outputs to the chunk, filled-in in the
/// context of the injection.
///
/// The C++ `name` is a `std::string`; the port keeps it as a `Vec<u8>` so that
/// equality and any name-keyed ordering match the C++ byte-string semantics
/// exactly (the snippet compiler resolves operand names byte-for-byte).
#[derive(Debug, Clone)]
pub struct InjectParameter {
    /// Name of the parameter (for use in parsing p-code \e source)
    name: Vec<u8>,
    /// Unique index assigned (cross-referencing the InjectContext Varnode)
    index: int4,
    /// Size of the parameter Varnode in bytes
    size: uint4,
}

impl InjectParameter {
    /// Constructor (C++ `InjectParameter(const string &nm,uint4 sz)`): index
    /// starts at 0 and is assigned by `orderParameters`.
    pub fn new(nm: &[u8], sz: uint4) -> InjectParameter {
        InjectParameter { name: nm.to_vec(), index: 0, size: sz }
    }

    /// Get the parameter name (C++ `getName`).
    pub fn get_name(&self) -> &[u8] {
        &self.name
    }

    /// Get the assigned index (C++ `getIndex`).
    pub fn get_index(&self) -> int4 {
        self.index
    }

    /// Get the size of the parameter in bytes (C++ `getSize`).
    pub fn get_size(&self) -> uint4 {
        self.size
    }
}

// ---------------------------------------------------------------------------
// InjectContext (pcodeinject.hh:79-94)
// ---------------------------------------------------------------------------

/// \brief Context needed to emit a p-code injection as a full set of p-code
/// operations
///
/// The C++ class is abstract (pure-virtual `encode`); the SLEIGH-backed concrete
/// form lives in `inject_sleigh.rs`. The data members common to every context —
/// the three context addresses and the input/output storage locations — are
/// modelled here as a plain struct that the concrete contexts embed.
///
/// SEAM(W4/W6): the C++ holds an `Architecture *glb`; this port leaves the
/// architecture handle to the concrete context in `inject_sleigh.rs` (the only
/// consumer), so the neutral half carries just the addresses and storage.
#[derive(Debug, Clone, Default)]
pub struct InjectContext {
    /// Address of instruction causing inject (C++ `baseaddr`).
    pub baseaddr: Option<Address>,
    /// Address of following instruction (C++ `nextaddr`).
    pub nextaddr: Option<Address>,
    /// If the injected instruction is a call, the address being called
    /// (C++ `calladdr`).
    pub calladdr: Option<Address>,
    /// Storage location for input parameters (C++ `inputlist`).
    pub inputlist: Vec<VarnodeData>,
    /// Storage location for output (C++ `output`).
    pub output: Vec<VarnodeData>,
}

impl InjectContext {
    /// Release resources (from last injection) — C++ `clear`.
    pub fn clear(&mut self) {
        self.inputlist.clear();
        self.output.clear();
    }
}

// ---------------------------------------------------------------------------
// InjectPayload type tags (pcodeinject.hh:103-108)
// ---------------------------------------------------------------------------

/// Injection that replaces a CALL (C++ `CALLFIXUP_TYPE`).
pub const CALLFIXUP_TYPE: int4 = 1;
/// Injection that replaces a user-defined p-code op, CALLOTHER
/// (C++ `CALLOTHERFIXUP_TYPE`).
pub const CALLOTHERFIXUP_TYPE: int4 = 2;
/// Injection to patch up data-flow around the caller/callee boundary
/// (C++ `CALLMECHANISM_TYPE`).
pub const CALLMECHANISM_TYPE: int4 = 3;
/// Injection running as a stand-alone p-code script
/// (C++ `EXECUTABLEPCODE_TYPE`).
pub const EXECUTABLEPCODE_TYPE: int4 = 4;

// ---------------------------------------------------------------------------
// InjectPayload common data (pcodeinject.hh:101-148)
// ---------------------------------------------------------------------------

/// \brief The engine-neutral data of an `InjectPayload`.
///
/// The C++ `InjectPayload` is an abstract base class; the polymorphic
/// `inject`/`decode`/`printTemplate`/`getSource` methods are carried by the
/// [`InjectPayload`] trait. The shared protected state (name, type, flags,
/// param lists) lives in this struct, which every concrete payload embeds,
/// mirroring the C++ protected members and the non-virtual accessors.
#[derive(Debug, Clone)]
pub struct InjectPayloadCore {
    /// Formal name of the payload (C++ `name`, a `std::string`; kept as bytes
    /// for byte-faithful name ordering).
    pub name: Vec<u8>,
    /// Type of this payload: CALLFIXUP_TYPE, CALLOTHERFIXUP_TYPE, etc.
    pub ptype: int4,
    /// True if the injection is generated dynamically (C++ `dynamic`).
    pub dynamic: bool,
    /// True if injected COPYs are considered \e incidental
    /// (C++ `incidentalCopy`).
    pub incidental_copy: bool,
    /// Number of parameters shifted in the original call (C++ `paramshift`).
    pub paramshift: int4,
    /// List of input parameters to this payload (C++ `inputlist`).
    pub inputlist: Vec<InjectParameter>,
    /// List of output parameters (C++ `output`).
    pub output: Vec<InjectParameter>,
}

impl InjectPayloadCore {
    /// Construct for use with decode (C++ `InjectPayload(const string&,int4)`).
    pub fn new(nm: &[u8], tp: int4) -> InjectPayloadCore {
        InjectPayloadCore {
            name: nm.to_vec(),
            ptype: tp,
            dynamic: false,
            incidental_copy: false,
            paramshift: 0,
            inputlist: Vec::new(),
            output: Vec::new(),
        }
    }

    /// \brief Parse an `<input>`/`<output>` element describing an injection
    /// parameter (C++ static `InjectPayload::decodeParameter`).
    ///
    /// Returns `(name, size)`; errors with the C++ message when the name is
    /// empty.
    pub fn decode_parameter(decoder: &mut dyn Decoder) -> KunaResult<(Vec<u8>, uint4)> {
        let mut name: Vec<u8> = Vec::new();
        let mut size: uint4 = 0;
        let elem_id = decoder.open_element()?;
        loop {
            let attrib_id = decoder.get_next_attribute_id()?;
            if attrib_id == 0 {
                break;
            }
            if attrib_id == ATTRIB_NAME {
                name = decoder.read_string()?;
            } else if attrib_id == ATTRIB_SIZE {
                // C++ readUnsignedInteger -> uint4: narrow the u64 (sizes are
                // small; matches the C++ implicit narrowing into `size`).
                size = decoder.read_unsigned_integer()? as uint4;
            }
        }
        decoder.close_element(elem_id)?;
        if name.is_empty() {
            return Err(KunaError::lowlevel("Missing inject parameter name"));
        }
        Ok((name, size))
    }

    /// Assign an index to parameters (C++ `InjectPayload::orderParameters`):
    /// inputs first, then outputs, in declaration order.
    pub fn order_parameters(&mut self) {
        let mut id: int4 = 0;
        for p in self.inputlist.iter_mut() {
            p.index = id;
            id += 1;
        }
        for p in self.output.iter_mut() {
            p.index = id;
            id += 1;
        }
    }

    /// Parse the attributes of the current `<pcode>` tag
    /// (C++ `InjectPayload::decodePayloadAttributes`).
    ///
    /// The `<pcode>` element must be current and already opened.
    pub fn decode_payload_attributes(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        self.paramshift = 0;
        self.dynamic = false;
        loop {
            let attrib_id = decoder.get_next_attribute_id()?;
            if attrib_id == 0 {
                break;
            }
            if attrib_id == ATTRIB_PARAMSHIFT {
                // C++ readSignedInteger -> int4 (narrow the i64).
                self.paramshift = decoder.read_signed_integer()? as int4;
            } else if attrib_id == ATTRIB_DYNAMIC {
                self.dynamic = decoder.read_bool()?;
            } else if attrib_id == ATTRIB_INCIDENTALCOPY {
                self.incidental_copy = decoder.read_bool()?;
            } else if attrib_id == ATTRIB_INJECT {
                let upon_type = decoder.read_string()?;
                if upon_type == b"uponentry" {
                    self.name.extend_from_slice(b"@@inject_uponentry");
                } else {
                    self.name.extend_from_slice(b"@@inject_uponreturn");
                }
            }
        }
        Ok(())
    }

    /// Parse any `<input>`/`<output>` children of the current `<pcode>` tag
    /// (C++ `InjectPayload::decodePayloadParams`).
    ///
    /// Elements are processed until the first child that isn't `<input>` or
    /// `<output>` is encountered. The `<pcode>` element must be current/open.
    pub fn decode_payload_params(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        loop {
            let sub_id = decoder.peek_element()?;
            if sub_id == ELEM_INPUT {
                let (param_name, size) = InjectPayloadCore::decode_parameter(decoder)?;
                self.inputlist.push(InjectParameter::new(&param_name, size));
            } else if sub_id == ELEM_OUTPUT {
                let (param_name, size) = InjectPayloadCore::decode_parameter(decoder)?;
                self.output.push(InjectParameter::new(&param_name, size));
            } else {
                break;
            }
        }
        self.order_parameters();
        Ok(())
    }

    /// Get the number of parameters shifted (C++ `getParamShift`).
    pub fn get_param_shift(&self) -> int4 {
        self.paramshift
    }

    /// Return `true` if p-code is generated dynamically (C++ `isDynamic`).
    pub fn is_dynamic(&self) -> bool {
        self.dynamic
    }

    /// Return `true` if any injected COPY is \e incidental
    /// (C++ `isIncidentalCopy`).
    pub fn is_incidental_copy(&self) -> bool {
        self.incidental_copy
    }

    /// Return the number of input parameters (C++ `sizeInput`).
    pub fn size_input(&self) -> int4 {
        self.inputlist.len() as int4
    }

    /// Return the number of output parameters (C++ `sizeOutput`).
    pub fn size_output(&self) -> int4 {
        self.output.len() as int4
    }

    /// Get the i-th input parameter (C++ `getInput`).
    pub fn get_input(&self, i: int4) -> &InjectParameter {
        &self.inputlist[i as usize]
    }

    /// Get the i-th output parameter (C++ `getOutput`).
    pub fn get_output(&self, i: int4) -> &InjectParameter {
        &self.output[i as usize]
    }

    /// Return the name of the injection (C++ `getName`).
    pub fn get_name(&self) -> &[u8] {
        &self.name
    }

    /// Return the type of injection (C++ `getType`).
    pub fn get_type(&self) -> int4 {
        self.ptype
    }
}

// ---------------------------------------------------------------------------
// InjectPayload polymorphic surface (pcodeinject.hh:131-147)
// ---------------------------------------------------------------------------

/// \brief The polymorphic surface of a C++ `InjectPayload`.
///
/// Concrete payloads (`inject_sleigh.rs`'s `InjectPayloadSleigh`,
/// `ExecutablePcodeSleigh`, `InjectPayloadDynamic`) implement these on top of an
/// embedded [`InjectPayloadCore`]. The `inject` emit and template printing route
/// through the engine seam, so they take an [`InjectEngine`].
pub trait InjectPayload {
    /// Access the engine-neutral payload data.
    fn core(&self) -> &InjectPayloadCore;
    /// Mutable access to the engine-neutral payload data.
    fn core_mut(&mut self) -> &mut InjectPayloadCore;

    /// Decode this payload from a stream (C++ pure-virtual `decode`).
    fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()>;

    /// Return a string describing the \e source of the injection
    /// (C++ pure-virtual `getSource`).
    fn get_source(&self) -> Vec<u8>;
}

// ---------------------------------------------------------------------------
// InjectArchitecture seam (the `Architecture *glb` slice this file uses)
// ---------------------------------------------------------------------------

/// SEAM(W4/W6): the slice of the C++ `Architecture *glb` that
/// `ExecutablePcode::build`/`evaluate` and `PcodeInjectLibrary` reach.
///
/// `Architecture` is W4/W6; this minimal trait mirrors exactly the spaces the
/// injection framework touches when laying out a snippet's temporary
/// inputs/outputs.
pub trait InjectArchitecture {
    /// C++ `glb->getDefaultCodeSpace()`.
    fn get_default_code_space(&self) -> Rc<AddrSpace>;
    /// C++ `glb->getUniqueSpace()`.
    fn get_unique_space(&self) -> Rc<AddrSpace>;
}

// ---------------------------------------------------------------------------
// InjectEngine seam (the kuna-sleigh emit path, currently non-public)
// ---------------------------------------------------------------------------

/// SEAM: the SLEIGH p-code **emit** path used by `InjectPayload::inject` and
/// `ExecutablePcode::build`.
///
/// The C++ `inject()` body drives `SleighBuilder`/`PcodeCacher`/
/// `ParserWalkerChange` and the private `ParserContext` mutators
/// (`setAddr`/`allocateOperand`/`deallocateState`/...). In the Rust port those
/// types/methods are **private to `kuna-sleigh`'s `sleigh.rs`**, which this
/// crate (and the W3 wave) cannot edit. This trait names exactly that emit
/// surface so the framework structure is faithful and a later wave that makes
/// the kuna-sleigh emit API public can slot a real implementor in without
/// touching the injection framework.
///
/// (LOSS reported: the concrete emit is unreachable until kuna-sleigh exposes
/// `SleighBuilder`/`PcodeCacher`/`ParserWalkerChange`/`ParserContext` mutators.)
pub trait InjectEngine {
    /// Emit the p-code ops of the given payload into `emit`, resolving the
    /// payload's parameters against `context` (C++ `InjectPayloadSleigh::inject`
    /// / `ExecutablePcodeSleigh::inject`).
    fn emit(
        &self,
        payload: &dyn InjectPayload,
        context: &mut InjectContext,
        emit: &mut dyn PcodeEmit,
    ) -> KunaResult<()>;
}

// ---------------------------------------------------------------------------
// PcodeInjectLibrary base (pcodeinject.hh:187-274 + pcodeinject.cc:208-359)
// ---------------------------------------------------------------------------

/// \brief The engine-neutral name->id namespaces and registration logic of a
/// C++ `PcodeInjectLibrary`.
///
/// The C++ class is abstract: the storage scheme (`allocateInject`,
/// `registerInject`, `getCachedContext`, `getBehaviors`, `manualCall*`) is
/// supplied by a derived class. The reusable registration bookkeeping — the four
/// name->id maps and the four id->name vectors, plus `getPayloadId` — is ported
/// here as a standalone value that the concrete library (in `inject_sleigh.rs`)
/// embeds.
///
/// Per ADR 0002 the name->id maps are `BTreeMap<Vec<u8>, int4>` keyed by the
/// C++ byte-string name order (`std::map<string,int4>`).
#[derive(Debug, Clone, Default)]
pub struct PcodeInjectLibraryBase {
    /// Offset within \e unique space for allocating temporaries within a payload
    /// (C++ `tempbase`).
    pub tempbase: uint4,
    /// Map of registered call-fixup names to injection id (C++ `callFixupMap`).
    pub call_fixup_map: BTreeMap<Vec<u8>, int4>,
    /// Map of registered callother-fixup names to injection id
    /// (C++ `callOtherFixupMap`).
    pub call_other_fixup_map: BTreeMap<Vec<u8>, int4>,
    /// Map of registered mechanism names to injection id
    /// (C++ `callMechFixupMap`).
    pub call_mech_fixup_map: BTreeMap<Vec<u8>, int4>,
    /// Map of registered script names to ExecutablePcode id (C++ `scriptMap`).
    pub script_map: BTreeMap<Vec<u8>, int4>,
    /// Map from injectid to call-fixup name (C++ `callFixupNames`).
    pub call_fixup_names: Vec<Vec<u8>>,
    /// Map from injectid to callother-fixup target-op name
    /// (C++ `callOtherTarget`).
    pub call_other_target: Vec<Vec<u8>>,
    /// Map from injectid to call-mech name (C++ `callMechTarget`).
    pub call_mech_target: Vec<Vec<u8>>,
    /// Map from injectid to script name (C++ `scriptNames`).
    pub script_names: Vec<Vec<u8>>,
}

impl PcodeInjectLibraryBase {
    /// Constructor (C++ `PcodeInjectLibrary(Architecture*,uint4 tmpbase)`):
    /// only the `tempbase` is engine-neutral.
    pub fn new(tmpbase: uint4) -> PcodeInjectLibraryBase {
        PcodeInjectLibraryBase { tempbase: tmpbase, ..Default::default() }
    }

    /// Get the (current) offset for building temporary registers
    /// (C++ `getUniqueBase`).
    pub fn get_unique_base(&self) -> uint4 {
        self.tempbase
    }

    /// \brief Map a \e call-fixup name to a payload id
    /// (C++ `registerCallFixup`).
    pub fn register_call_fixup(&mut self, fixup_name: &[u8], injectid: int4) -> KunaResult<()> {
        if self.call_fixup_map.contains_key(fixup_name) {
            return Err(KunaError::lowlevel(format!(
                "Duplicate <callfixup>: {}",
                String::from_utf8_lossy(fixup_name)
            )));
        }
        self.call_fixup_map.insert(fixup_name.to_vec(), injectid);
        while (self.call_fixup_names.len() as int4) <= injectid {
            self.call_fixup_names.push(Vec::new());
        }
        self.call_fixup_names[injectid as usize] = fixup_name.to_vec();
        Ok(())
    }

    /// \brief Map a \e callother-fixup name to a payload id
    /// (C++ `registerCallOtherFixup`).
    pub fn register_call_other_fixup(
        &mut self,
        fixup_name: &[u8],
        injectid: int4,
    ) -> KunaResult<()> {
        if self.call_other_fixup_map.contains_key(fixup_name) {
            return Err(KunaError::lowlevel(format!(
                "Duplicate <callotherfixup>: {}",
                String::from_utf8_lossy(fixup_name)
            )));
        }
        self.call_other_fixup_map.insert(fixup_name.to_vec(), injectid);
        while (self.call_other_target.len() as int4) <= injectid {
            self.call_other_target.push(Vec::new());
        }
        self.call_other_target[injectid as usize] = fixup_name.to_vec();
        Ok(())
    }

    /// \brief Map a \e call \e mechanism name to a payload id
    /// (C++ `registerCallMechanism`).
    pub fn register_call_mechanism(&mut self, fixup_name: &[u8], injectid: int4) -> KunaResult<()> {
        if self.call_mech_fixup_map.contains_key(fixup_name) {
            return Err(KunaError::lowlevel(format!(
                "Duplicate <callmechanism>: {}",
                String::from_utf8_lossy(fixup_name)
            )));
        }
        self.call_mech_fixup_map.insert(fixup_name.to_vec(), injectid);
        while (self.call_mech_target.len() as int4) <= injectid {
            self.call_mech_target.push(Vec::new());
        }
        self.call_mech_target[injectid as usize] = fixup_name.to_vec();
        Ok(())
    }

    /// \brief Map a \e p-code \e script name to a payload id
    /// (C++ `registerExeScript`).
    pub fn register_exe_script(&mut self, script_name: &[u8], injectid: int4) -> KunaResult<()> {
        if self.script_map.contains_key(script_name) {
            return Err(KunaError::lowlevel(format!(
                "Duplicate <script>: {}",
                String::from_utf8_lossy(script_name)
            )));
        }
        self.script_map.insert(script_name.to_vec(), injectid);
        while (self.script_names.len() as int4) <= injectid {
            self.script_names.push(Vec::new());
        }
        self.script_names[injectid as usize] = script_name.to_vec();
        Ok(())
    }

    /// \brief Map name and type to the payload id (C++ `getPayloadId`).
    ///
    /// Returns -1 if there is no matching payload.
    pub fn get_payload_id(&self, ptype: int4, nm: &[u8]) -> int4 {
        let found = if ptype == CALLFIXUP_TYPE {
            self.call_fixup_map.get(nm)
        } else if ptype == CALLOTHERFIXUP_TYPE {
            self.call_other_fixup_map.get(nm)
        } else if ptype == CALLMECHANISM_TYPE {
            self.call_mech_fixup_map.get(nm)
        } else {
            self.script_map.get(nm)
        };
        match found {
            Some(id) => *id,
            None => -1,
        }
    }

    /// \brief Get the call-fixup name associated with an id
    /// (C++ `getCallFixupName`).
    pub fn get_call_fixup_name(&self, injectid: int4) -> Vec<u8> {
        if injectid < 0 || injectid >= self.call_fixup_names.len() as int4 {
            return Vec::new();
        }
        self.call_fixup_names[injectid as usize].clone()
    }

    /// \brief Get the callother-fixup name associated with an id
    /// (C++ `getCallOtherTarget`).
    pub fn get_call_other_target(&self, injectid: int4) -> Vec<u8> {
        if injectid < 0 || injectid >= self.call_other_target.len() as int4 {
            return Vec::new();
        }
        self.call_other_target[injectid as usize].clone()
    }

    /// \brief Get the call mechanism name associated with an id
    /// (C++ `getCallMechanismName`).
    pub fn get_call_mechanism_name(&self, injectid: int4) -> Vec<u8> {
        if injectid < 0 || injectid >= self.call_mech_target.len() as int4 {
            return Vec::new();
        }
        self.call_mech_target[injectid as usize].clone()
    }
}

// ---------------------------------------------------------------------------
// ExecutablePcode build/evaluate layout (pcodeinject.cc:137-206)
// ---------------------------------------------------------------------------

/// The temporary-register layout an `ExecutablePcode` reserves for its inputs
/// and outputs before emitting/emulating the snippet.
///
/// This is the engine-neutral part of `ExecutablePcode::build` (the loops that
/// assign each `<input>`/`<output>` a `unique`-space slot at `0x10 + k*0x20`).
/// The concrete `ExecutablePcodeSleigh` in `inject_sleigh.rs` uses it to fill
/// the `InjectContext` storage and to feed `EmulateSnippet`.
#[derive(Debug, Clone, Default)]
pub struct SnippetLayout {
    /// Temporary ids of input varnodes (C++ `inputList`).
    pub input_list: Vec<uintb>,
    /// Temporary ids of output varnodes (C++ `outputList`).
    pub output_list: Vec<uintb>,
    /// The final reservation high-water mark (C++ `uniqReserve`), the
    /// `uniq_reserve` argument to `buildEmitter`.
    pub uniq_reserve: uintb,
}

impl SnippetLayout {
    /// Lay out the unique-space slots for a payload's inputs and outputs,
    /// filling the given `InjectContext` storage in the process (the body of
    /// C++ `ExecutablePcode::build` up to `buildEmitter`).
    ///
    /// `arch` supplies the unique space (C++ `glb->getUniqueSpace()`).
    pub fn build(
        core: &InjectPayloadCore,
        context: &mut InjectContext,
        arch: &dyn InjectArchitecture,
    ) -> SnippetLayout {
        let mut layout = SnippetLayout::default();
        // C++ uintb uniqReserve = 0x10;  // reserved for inputs and output
        let mut uniq_reserve: uintb = 0x10;
        let uniq_space = arch.get_unique_space();
        for i in 0..core.size_input() {
            let param = core.get_input(i);
            context.inputlist.push(VarnodeData {
                space: Some(Rc::clone(&uniq_space)),
                offset: uniq_reserve,
                size: param.get_size(),
            });
            layout.input_list.push(uniq_reserve);
            uniq_reserve = uniq_reserve.wadd(0x20);
        }
        for i in 0..core.size_output() {
            let param = core.get_output(i);
            context.output.push(VarnodeData {
                space: Some(Rc::clone(&uniq_space)),
                offset: uniq_reserve,
                size: param.get_size(),
            });
            layout.output_list.push(uniq_reserve);
            uniq_reserve = uniq_reserve.wadd(0x20);
        }
        layout.uniq_reserve = uniq_reserve;
        layout
    }
}

#[cfg(test)]
mod tests;
