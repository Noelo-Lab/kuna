//! Port of `decompiler/cpp/userop.{cc,hh}` (item `w3-ir-userop-inject`):
//! detailed definitions of user-defined p-code operations (the CALLOTHER
//! black-box ops) and their manager [`UserOpManage`].
//!
//! The C++ `UserPcodeOp` is a small virtual hierarchy
//! (`UnspecializedPcodeOp`/`DatatypeUserOp`/`InjectedUserOp`/`VolatileReadOp`/
//! `VolatileWriteOp`/`SegmentOp`/`JumpAssistOp`/`InternalStringOp`). Since the
//! variants differ only in a handful of extra fields and a few `decode`/display
//! virtuals, the port models the hierarchy as an enum [`UserPcodeOp`] carrying
//! the shared base ([`UserOpBase`]) plus a per-kind payload, with the virtual
//! methods dispatched by `match`. This keeps the manager's by-id and by-name
//! tables as a single owning container (no `Box<dyn>` web), faithful to the C++
//! `vector<UserPcodeOp*>` / `map<string,UserPcodeOp*>` semantics.
//!
//! ## Seams
//!
//! `userop.cc` reaches a wide slice of `Architecture *glb` (the translator's
//! userop names, the inject library, the type factory, the symbol table, the
//! default data space). The type-factory/symbol-table paths feed the
//! `getOutputLocal`/`getInputLocal` data-type virtuals, which belong to W6/W4;
//! they are seam-deferred (`// STUB`). The decode + registration + lookup paths
//! — the test target — use only the [`UseropArchitecture`] slice defined here
//! (translator userop names + the inject library) and are fully ported.

use std::collections::BTreeMap;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{
    AttributeId, Decoder, ElementId, IdRegistry, ATTRIB_FORMAT, ATTRIB_NAME, ATTRIB_SPACE,
};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uint4};
use kuna_num::pcoderaw::VarnodeData;
use std::rc::Rc;

use crate::pcodeinject::{
    InjectArchitecture, CALLOTHERFIXUP_TYPE, ELEM_CASE_PCODE, ELEM_DEFAULT_PCODE, ELEM_PCODE,
    ELEM_SIZE_PCODE, EXECUTABLEPCODE_TYPE,
};

// ---------------------------------------------------------------------------
// Marshaling ids (verbatim from userop.cc:21-28)
// ---------------------------------------------------------------------------

/// Marshaling attribute "farpointer" (`userop.cc:21`).
pub const ATTRIB_FARPOINTER: AttributeId = AttributeId::new("farpointer", 85);
/// Marshaling attribute "inputop" (`userop.cc:22`).
pub const ATTRIB_INPUTOP: AttributeId = AttributeId::new("inputop", 86);
/// Marshaling attribute "outputop" (`userop.cc:23`).
pub const ATTRIB_OUTPUTOP: AttributeId = AttributeId::new("outputop", 87);
/// Marshaling attribute "userop" (`userop.cc:24`).
pub const ATTRIB_USEROP: AttributeId = AttributeId::new("userop", 88);

/// Marshaling element `<constresolve>` (`userop.cc:26`).
pub const ELEM_CONSTRESOLVE: ElementId = ElementId::new("constresolve", 127);
/// Marshaling element `<jumpassist>` (`userop.cc:27`).
pub const ELEM_JUMPASSIST: ElementId = ElementId::new("jumpassist", 128);
/// Marshaling element `<segmentop>` (`userop.cc:28`).
pub const ELEM_SEGMENTOP: ElementId = ElementId::new("segmentop", 129);

/// Register every userop marshaling id with an [`IdRegistry`] (the C++ global
/// static-init side effect; explicit in the port).
pub fn register_ids(reg: &mut IdRegistry) {
    for a in [&ATTRIB_FARPOINTER, &ATTRIB_INPUTOP, &ATTRIB_OUTPUTOP, &ATTRIB_USEROP] {
        reg.register_attribute(a);
    }
    for e in [&ELEM_CONSTRESOLVE, &ELEM_JUMPASSIST, &ELEM_SEGMENTOP] {
        reg.register_element(e);
    }
}

// ---------------------------------------------------------------------------
// userop_flags / userop_type / BUILTIN_* (userop.hh:50-72)
// ---------------------------------------------------------------------------

/// \brief Enumeration of different boolean properties that can be assigned to a
/// CALLOTHER (C++ `UserPcodeOp::userop_flags`).
pub mod userop_flags {
    use kuna_base::types::uint4;
    /// Displayed as assignment, `in1 = in2`, where in1 is an annotation.
    pub const ANNOTATION_ASSIGNMENT: uint4 = 1;
    /// Don't emit a special token, just emit the first input parameter.
    pub const NO_OPERATOR: uint4 = 2;
    /// Emit as a string constant.
    pub const DISPLAY_STRING: uint4 = 4;
}

/// \brief User-op class encoded as an enum (C++ `UserPcodeOp::userop_type`).
///
/// The discriminant values are load-bearing (`getType()` is observable), so
/// they are pinned verbatim to `userop.hh:56-65`.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum userop_type {
    /// Encoding for UnspecializedPcodeOp.
    unspecialized = 1,
    /// InjectedUserOp.
    injected = 2,
    /// VolatileReadOp.
    volatile_read = 3,
    /// VolatileWriteOp.
    volatile_write = 4,
    /// SegmentOp.
    segment = 5,
    /// JumpAssistOp.
    jumpassist = 6,
    /// InternalStringOp.
    string_data = 7,
    /// DatatypeUserOp.
    datatype = 8,
}

/// Built-in id for the InternalStringOp (`userop.cc:30`).
pub const BUILTIN_STRINGDATA: uint4 = 0x10000000;
/// Built-in id for VolatileReadOp (`userop.cc:31`).
pub const BUILTIN_VOLATILE_READ: uint4 = 0x10000001;
/// Built-in id for VolatileWriteOp (`userop.cc:32`).
pub const BUILTIN_VOLATILE_WRITE: uint4 = 0x10000002;
/// Built-in id for memcpy (`userop.cc:33`).
pub const BUILTIN_MEMCPY: uint4 = 0x10000003;
/// Built-in id for strncpy (`userop.cc:34`).
pub const BUILTIN_STRNCPY: uint4 = 0x10000004;
/// Built-in id for wcsncpy (`userop.cc:35`).
pub const BUILTIN_WCSNCPY: uint4 = 0x10000005;
/// (kuna) Built-in id for memset, GH-9230 constant-fill recovery
/// (`userop.cc:36`; a kuna anchor edit — see UPSTREAM.md *Divergence*).
pub const BUILTIN_MEMSET: uint4 = 0x10000006;

// ---------------------------------------------------------------------------
// UserOpBase + UserPcodeOp hierarchy (userop.hh:47-318)
// ---------------------------------------------------------------------------

/// \brief The base members of a C++ `UserPcodeOp`.
///
/// The C++ base carries `name`/`type`/`useropindex`/`flags` (plus the
/// `Architecture *glb`, which the port threads in explicitly where needed).
/// `name` is a `std::string`; kept as `Vec<u8>` for byte-faithful name keys.
#[derive(Debug, Clone)]
pub struct UserOpBase {
    /// Low-level name of p-code operator (C++ `name`).
    pub name: Vec<u8>,
    /// Encoded class type (C++ `type`).
    pub utype: userop_type,
    /// Index passed in the CALLOTHER op (C++ `useropindex`).
    pub useropindex: int4,
    /// Boolean attributes of the CALLOTHER (C++ `flags`).
    pub flags: uint4,
}

impl UserOpBase {
    /// Construct from name and index (C++
    /// `UserPcodeOp(const string&,Architecture*,uint4,int4)`); flags start 0.
    pub fn new(nm: &[u8], tp: userop_type, ind: int4) -> UserOpBase {
        UserOpBase { name: nm.to_vec(), utype: tp, useropindex: ind, flags: 0 }
    }
}

/// \brief The \e segmented \e address operator (C++ `SegmentOp`).
///
/// Placeholder for address mappings involving \b segments. See `userop.hh`.
#[derive(Debug, Clone)]
pub struct SegmentOp {
    /// The physical address space into which a segmented pointer points
    /// (C++ `spc`).
    pub spc: Option<Rc<AddrSpace>>,
    /// Id of InjectPayload that emulates this operation (C++ `injectId`).
    pub inject_id: int4,
    /// Size in bytes of the \e base or \e segment value (C++ `baseinsize`).
    pub baseinsize: int4,
    /// Size in bytes of the \e near pointer value (C++ `innerinsize`).
    pub innerinsize: int4,
    /// `true` if the joined pair base:near acts as a \b far pointer
    /// (C++ `supportsfarpointer`).
    pub supportsfarpointer: bool,
    /// How to resolve constant near pointers (C++ `constresolve`).
    pub constresolve: VarnodeData,
}

/// \brief A user defined p-code op for assisting jump-table recovery
/// (C++ `JumpAssistOp`).
#[derive(Debug, Clone)]
pub struct JumpAssistOp {
    /// Id of p-code script performing index2case (== -1 if absent and
    /// index==case) (C++ `index2case`).
    pub index2case: int4,
    /// Id of p-code script performing index2addr (must be present)
    /// (C++ `index2addr`).
    pub index2addr: int4,
    /// Id of p-code script calculating the default address (must be present)
    /// (C++ `defaultaddr`).
    pub defaultaddr: int4,
    /// Id of p-code script that calculates number of indices (== -1 if absent)
    /// (C++ `calcsize`).
    pub calcsize: int4,
}

/// \brief Generic user-op providing input/output data-types
/// (C++ `DatatypeUserOp`).
///
/// STUB(W6): the C++ holds `Datatype *outType` / `vector<Datatype*> inTypes`,
/// consumed by `getOutputLocal`/`getInputLocal`. The W6 `TypeFactory` builds
/// those; until then this variant carries the type handles as
/// [`Rc<Datatype>`](crate::dtype::Datatype) seam skeletons.
#[derive(Debug, Clone)]
pub struct DatatypeUserOp {
    /// Data-type of the output (C++ `outType`). STUB(W6).
    pub out_type: Option<Rc<crate::dtype::Datatype>>,
    /// Data-types of the inputs (C++ `inTypes`). STUB(W6).
    pub in_types: Vec<Rc<crate::dtype::Datatype>>,
}

/// The per-kind payload of a [`UserPcodeOp`] (the data members of each C++
/// derived class that aren't in the base).
#[derive(Debug, Clone)]
pub enum UserOpKind {
    /// `UnspecializedPcodeOp` (no extra fields), `VolatileReadOp`,
    /// `VolatileWriteOp`, and `InternalStringOp` carry only base fields.
    Unspecialized,
    /// `VolatileReadOp` (base fields + the volatile-read display/type virtuals).
    VolatileRead,
    /// `VolatileWriteOp`.
    VolatileWrite,
    /// `InternalStringOp`.
    InternalString,
    /// `InjectedUserOp`: the id of the injection object (C++ `injectid`).
    Injected { injectid: uint4 },
    /// `SegmentOp`.
    Segment(SegmentOp),
    /// `JumpAssistOp`.
    JumpAssist(JumpAssistOp),
    /// `DatatypeUserOp`.
    Datatype(DatatypeUserOp),
}

/// \brief A detailed definition of a user-defined p-code operation
/// (C++ `UserPcodeOp` + its derived classes, collapsed into an enum-tagged
/// struct).
#[derive(Debug, Clone)]
pub struct UserPcodeOp {
    /// The shared base members.
    pub base: UserOpBase,
    /// The per-kind payload.
    pub kind: UserOpKind,
}

impl UserPcodeOp {
    /// C++ `UnspecializedPcodeOp(const string&,Architecture*,int4)`.
    pub fn new_unspecialized(nm: &[u8], ind: int4) -> UserPcodeOp {
        UserPcodeOp {
            base: UserOpBase::new(nm, userop_type::unspecialized, ind),
            kind: UserOpKind::Unspecialized,
        }
    }

    /// C++ `InjectedUserOp(const string&,Architecture*,int4,int4)`.
    pub fn new_injected(nm: &[u8], ind: int4, injid: int4) -> UserPcodeOp {
        UserPcodeOp {
            base: UserOpBase::new(nm, userop_type::injected, ind),
            // C++ `InjectedUserOp(...) { injectid = injid; }` — injid is an
            // int4 stored into a uint4 member; preserve the bit pattern.
            kind: UserOpKind::Injected { injectid: injid as uint4 },
        }
    }

    /// C++ `VolatileReadOp(const string&,Architecture*,bool functional)`:
    /// flags = functional ? 0 : no_operator; index = BUILTIN_VOLATILE_READ.
    pub fn new_volatile_read(nm: &[u8], functional: bool) -> UserPcodeOp {
        let mut base = UserOpBase::new(nm, userop_type::volatile_read, BUILTIN_VOLATILE_READ as int4);
        base.flags = if functional { 0 } else { userop_flags::NO_OPERATOR };
        UserPcodeOp { base, kind: UserOpKind::VolatileRead }
    }

    /// C++ `VolatileWriteOp(const string&,Architecture*,bool functional)`:
    /// flags = functional ? 0 : annotation_assignment; index =
    /// BUILTIN_VOLATILE_WRITE.
    pub fn new_volatile_write(nm: &[u8], functional: bool) -> UserPcodeOp {
        let mut base =
            UserOpBase::new(nm, userop_type::volatile_write, BUILTIN_VOLATILE_WRITE as int4);
        base.flags = if functional { 0 } else { userop_flags::ANNOTATION_ASSIGNMENT };
        UserPcodeOp { base, kind: UserOpKind::VolatileWrite }
    }

    /// C++ `InternalStringOp(Architecture*)`: name "stringdata", index
    /// BUILTIN_STRINGDATA, flags |= display_string.
    pub fn new_internal_string() -> UserPcodeOp {
        let mut base =
            UserOpBase::new(b"stringdata", userop_type::string_data, BUILTIN_STRINGDATA as int4);
        base.flags |= userop_flags::DISPLAY_STRING;
        UserPcodeOp { base, kind: UserOpKind::InternalString }
    }

    /// C++ `DatatypeUserOp(const string&,Architecture*,int4,Datatype*,...)`:
    /// up to 4 non-null input types are pushed in order.
    pub fn new_datatype(
        nm: &[u8],
        ind: int4,
        out: Option<Rc<crate::dtype::Datatype>>,
        ins: &[Option<Rc<crate::dtype::Datatype>>],
    ) -> UserPcodeOp {
        let mut in_types = Vec::new();
        // C++ pushes each of the up-to-4 input data-types iff non-null, in order.
        for t in ins.iter().flatten() {
            in_types.push(Rc::clone(t));
        }
        UserPcodeOp {
            base: UserOpBase::new(nm, userop_type::datatype, ind),
            kind: UserOpKind::Datatype(DatatypeUserOp { out_type: out, in_types }),
        }
    }

    /// C++ `SegmentOp(const string&,Architecture*,int4)`: constresolve.space
    /// starts null. (`baseinsize`/`innerinsize`/etc. are set by `decode`.)
    pub fn new_segment(nm: &[u8], ind: int4) -> UserPcodeOp {
        UserPcodeOp {
            base: UserOpBase::new(nm, userop_type::segment, ind),
            kind: UserOpKind::Segment(SegmentOp {
                spc: None,
                inject_id: -1,
                baseinsize: 0,
                innerinsize: 0,
                supportsfarpointer: false,
                constresolve: VarnodeData::default(),
            }),
        }
    }

    /// C++ `JumpAssistOp(Architecture*)`: name "", index 0, all ids -1.
    pub fn new_jump_assist() -> UserPcodeOp {
        UserPcodeOp {
            base: UserOpBase::new(b"", userop_type::jumpassist, 0),
            kind: UserOpKind::JumpAssist(JumpAssistOp {
                index2case: -1,
                index2addr: -1,
                defaultaddr: -1,
                calcsize: -1,
            }),
        }
    }

    /// Get the low-level name of the p-code op (C++ `getName`).
    pub fn get_name(&self) -> &[u8] {
        &self.base.name
    }

    /// Get the encoded class type (C++ `getType`).
    pub fn get_type(&self) -> userop_type {
        self.base.utype
    }

    /// Get the constant id of the op (C++ `getIndex`).
    pub fn get_index(&self) -> int4 {
        self.base.useropindex
    }

    /// Get display type (0 = functional) (C++ `getDisplay`).
    pub fn get_display(&self) -> uint4 {
        self.base.flags
            & (userop_flags::ANNOTATION_ASSIGNMENT
                | userop_flags::NO_OPERATOR
                | userop_flags::DISPLAY_STRING)
    }

    /// `true` if this is an `UnspecializedPcodeOp` (C++
    /// `dynamic_cast<UnspecializedPcodeOp*>` test).
    pub fn is_unspecialized(&self) -> bool {
        matches!(self.kind, UserOpKind::Unspecialized)
    }

    /// Get the id of the injection object, for an `InjectedUserOp`
    /// (C++ `InjectedUserOp::getInjectId`).
    pub fn get_inject_id(&self) -> Option<uint4> {
        match &self.kind {
            UserOpKind::Injected { injectid } => Some(*injectid),
            _ => None,
        }
    }

    /// Borrow the `SegmentOp` payload, if this is a segment op.
    pub fn as_segment(&self) -> Option<&SegmentOp> {
        match &self.kind {
            UserOpKind::Segment(s) => Some(s),
            _ => None,
        }
    }

    /// Borrow the `JumpAssistOp` payload, if this is a jump-assist op.
    pub fn as_jump_assist(&self) -> Option<&JumpAssistOp> {
        match &self.kind {
            UserOpKind::JumpAssist(j) => Some(j),
            _ => None,
        }
    }

    /// C++ virtual `UserPcodeOp::getOperatorName(op)` — the symbol formally
    /// displayed in source for this CALLOTHER (`typeop.cc` `getOperatorName`
    /// resolves the userop index and calls this).  The base returns the
    /// low-level `name`; the volatile read/write overrides append the access
    /// size suffix (`read_volatile_1`, `write_volatile_4`, …).
    ///
    /// `out_size` is the output varnode size (used by `VolatileReadOp`); `in2_size`
    /// is the third input's size (used by `VolatileWriteOp`).  Both are `None`
    /// when the op lacks that operand, matching the C++ early `return name`.
    pub fn get_operator_name(&self, out_size: Option<int4>, in2_size: Option<int4>) -> Vec<u8> {
        match self.base.utype {
            // VolatileReadOp::getOperatorName (userop.cc:122): append out.size,
            // or bare name when there is no output.
            userop_type::volatile_read => match out_size {
                Some(sz) => Self::append_size(&self.base.name, sz),
                None => self.base.name.clone(),
            },
            // VolatileWriteOp::getOperatorName (userop.cc:153): append in(2).size,
            // or bare name when numInput < 3.
            userop_type::volatile_write => match in2_size {
                Some(sz) => Self::append_size(&self.base.name, sz),
                None => self.base.name.clone(),
            },
            // Base UserPcodeOp::getOperatorName (userop.hh:95): the low-level name.
            _ => self.base.name.clone(),
        }
    }

    /// C++ virtual `UserPcodeOp::extractAnnotationSize(vn, op)` (userop.cc:144/175):
    /// the byte width the printer should query the symbol scope with for a
    /// volatile annotation.  `VolatileReadOp` uses the read op's output size (1
    /// when there is no output); `VolatileWriteOp` uses the written value size
    /// (input slot 2).  The base op has no annotation and returns 0 (the C++
    /// default is never reached for a non-volatile CALLOTHER, since `pushAnnotation`
    /// only fires on an `isAnnotation()` operand).
    ///
    /// `out_size` is the op output varnode size; `in2_size` is the op's input(2)
    /// size.  Both `None` when that operand is absent.
    pub fn extract_annotation_size(&self, out_size: Option<int4>, in2_size: Option<int4>) -> int4 {
        match self.base.utype {
            userop_type::volatile_read => out_size.unwrap_or(1),
            userop_type::volatile_write => in2_size.unwrap_or(0),
            _ => 0,
        }
    }

    /// Append a size suffix to a name (C++ static `VolatileOp::appendSize`).
    pub fn append_size(base: &[u8], size: int4) -> Vec<u8> {
        let mut out = base.to_vec();
        let suffix = match size {
            1 => "_1".to_string(),
            2 => "_2".to_string(),
            4 => "_4".to_string(),
            8 => "_8".to_string(),
            // C++ `s << base << '_' << dec << size;`
            other => format!("_{other}"),
        };
        out.extend_from_slice(suffix.as_bytes());
        out
    }

    /// \brief Decode the detailed description from a stream element.
    ///
    /// Dispatches on the kind (C++ virtual `decode`). The injecting variants
    /// (`Injected`/`Segment`/`JumpAssist`) need access to the injection library
    /// (held by `arch`, mirroring `glb->pcodeinjectlib`) and the existing userop
    /// name table (`names`). The non-injecting variants (`Unspecialized`, the
    /// volatile ops, `InternalString`, `Datatype`) have a no-op `decode`,
    /// matching C++.
    pub fn decode<A: UseropArchitecture>(
        &mut self,
        decoder: &mut dyn Decoder,
        arch: &mut A,
        names: &NameLookup,
    ) -> KunaResult<()> {
        match self.base.utype {
            userop_type::injected => self.decode_injected(decoder, arch, names),
            userop_type::segment => self.decode_segment(decoder, arch, names),
            userop_type::jumpassist => self.decode_jump_assist(decoder, arch, names),
            // C++ UnspecializedPcodeOp/VolatileOp/InternalStringOp/DatatypeUserOp
            // all have `virtual void decode(Decoder&) {}`.
            _ => Ok(()),
        }
    }

    /// C++ `InjectedUserOp::decode` (userop.cc:86-99).
    fn decode_injected<A: UseropArchitecture>(
        &mut self,
        decoder: &mut dyn Decoder,
        arch: &mut A,
        names: &NameLookup,
    ) -> KunaResult<()> {
        let injectid = arch.decode_inject(b"userop", b"", CALLOTHERFIXUP_TYPE, decoder)?;
        let name = arch.get_call_other_target(injectid);
        // This tag overrides the base functionality of a userop, so the core
        // userop name and index may already be defined.
        let base = names.get_by_name(&name).ok_or_else(|| {
            KunaError::lowlevel(format!(
                "Unknown userop name in <callotherfixup>: {}",
                String::from_utf8_lossy(&name)
            ))
        })?;
        if !base.unspecialized {
            return Err(KunaError::lowlevel(format!(
                "<callotherfixup> overloads userop with another purpose: {}",
                String::from_utf8_lossy(&name)
            )));
        }
        self.base.name = name;
        self.base.useropindex = base.index; // Get the index from the core userop
        self.kind = UserOpKind::Injected { injectid: injectid as uint4 };
        Ok(())
    }

    /// C++ `SegmentOp::decode` (userop.cc:226-291).
    fn decode_segment<A: UseropArchitecture>(
        &mut self,
        decoder: &mut dyn Decoder,
        arch: &mut A,
        names: &NameLookup,
    ) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_SEGMENTOP)?;
        let mut spc: Option<Rc<AddrSpace>> = None;
        let mut inject_id: int4 = -1;
        let mut baseinsize: int4 = 0;
        let innerinsize: int4;
        let mut supportsfarpointer = false;
        // Default name, might be overridden by userop attribute.
        self.base.name = b"segment".to_vec();
        loop {
            let attrib_id = decoder.get_next_attribute_id()?;
            if attrib_id == 0 {
                break;
            }
            if attrib_id == ATTRIB_SPACE {
                spc = Some(decoder.read_space()?);
            } else if attrib_id == ATTRIB_FARPOINTER {
                supportsfarpointer = true;
            } else if attrib_id == ATTRIB_USEROP {
                // Based on existing sleigh op.
                self.base.name = decoder.read_string()?;
            }
        }
        let spc = spc.ok_or_else(|| KunaError::lowlevel("<segmentop> expecting space attribute"))?;
        let otherop = names.get_by_name(&self.base.name).ok_or_else(|| {
            KunaError::lowlevel(format!(
                "<segmentop> unknown userop {}",
                String::from_utf8_lossy(&self.base.name)
            ))
        })?;
        self.base.useropindex = otherop.index;
        if !otherop.unspecialized {
            return Err(KunaError::lowlevel(format!(
                "Redefining userop {}",
                String::from_utf8_lossy(&self.base.name)
            )));
        }

        let mut constresolve = VarnodeData::default();
        loop {
            let sub_id = decoder.peek_element()?;
            if sub_id == 0 {
                break;
            }
            if sub_id == ELEM_CONSTRESOLVE {
                decoder.open_element()?;
                if decoder.peek_element()? != 0 {
                    let (addr, sz) = Address::decode_sized(decoder)?;
                    constresolve.space = addr.get_space().cloned();
                    constresolve.offset = addr.get_offset();
                    // C++ `constresolve.size = sz;` (int4 -> uint4 member).
                    constresolve.size = sz as uint4;
                }
                decoder.close_element(sub_id)?;
            } else if sub_id == ELEM_PCODE {
                let nm = {
                    let mut nm = self.base.name.clone();
                    nm.extend_from_slice(b"_pcode");
                    nm
                };
                let source = b"cspec";
                inject_id = arch.decode_inject(source, &nm, EXECUTABLEPCODE_TYPE, decoder)?;
            } else {
                // C++ has only the two arms; any other child is left for the
                // closeElement to consume. (No spec exercises a third child.)
                break;
            }
        }
        decoder.close_element(elem_id)?;
        if inject_id < 0 {
            return Err(KunaError::lowlevel("Missing <pcode> child in <segmentop> tag"));
        }
        let (size_output, size_input, in0, in1) = arch.payload_io_sizes(inject_id)?;
        if size_output != 1 {
            return Err(KunaError::lowlevel(
                "<pcode> child of <segmentop> tag must declare one <output>",
            ));
        }
        if size_input == 1 {
            innerinsize = in0;
        } else if size_input == 2 {
            baseinsize = in0;
            innerinsize = in1;
        } else {
            return Err(KunaError::lowlevel(
                "<pcode> child of <segmentop> tag must declare one or two <input> tags",
            ));
        }
        self.kind = UserOpKind::Segment(SegmentOp {
            spc: Some(spc),
            inject_id,
            baseinsize,
            innerinsize,
            supportsfarpointer,
            constresolve,
        });
        Ok(())
    }

    /// C++ `JumpAssistOp::decode` (userop.cc:303-354).
    fn decode_jump_assist<A: UseropArchitecture>(
        &mut self,
        decoder: &mut dyn Decoder,
        arch: &mut A,
        names: &NameLookup,
    ) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_JUMPASSIST)?;
        self.base.name = decoder.read_string_id(&ATTRIB_NAME)?;
        let mut index2case: int4 = -1; // Mark as not present until we see a tag
        let mut index2addr: int4 = -1;
        let mut defaultaddr: int4 = -1;
        let mut calcsize: int4 = -1;
        loop {
            let sub_id = decoder.peek_element()?;
            if sub_id == 0 {
                break;
            }
            if sub_id == ELEM_CASE_PCODE {
                if index2case != -1 {
                    return Err(KunaError::lowlevel("Too many <case_pcode> tags"));
                }
                let nm = suffixed(&self.base.name, b"_index2case");
                index2case =
                    arch.decode_inject(b"jumpassistop", &nm, EXECUTABLEPCODE_TYPE, decoder)?;
            } else if sub_id == ELEM_ADDR_PCODE {
                if index2addr != -1 {
                    return Err(KunaError::lowlevel("Too many <addr_pcode> tags"));
                }
                let nm = suffixed(&self.base.name, b"_index2addr");
                index2addr =
                    arch.decode_inject(b"jumpassistop", &nm, EXECUTABLEPCODE_TYPE, decoder)?;
            } else if sub_id == ELEM_DEFAULT_PCODE {
                if defaultaddr != -1 {
                    return Err(KunaError::lowlevel("Too many <default_pcode> tags"));
                }
                let nm = suffixed(&self.base.name, b"_defaultaddr");
                defaultaddr =
                    arch.decode_inject(b"jumpassistop", &nm, EXECUTABLEPCODE_TYPE, decoder)?;
            } else if sub_id == ELEM_SIZE_PCODE {
                if calcsize != -1 {
                    return Err(KunaError::lowlevel("Too many <size_pcode> tags"));
                }
                let nm = suffixed(&self.base.name, b"_calcsize");
                calcsize =
                    arch.decode_inject(b"jumpassistop", &nm, EXECUTABLEPCODE_TYPE, decoder)?;
            } else {
                break;
            }
        }
        decoder.close_element(elem_id)?;

        if index2addr == -1 {
            return Err(KunaError::lowlevel(format!(
                "userop: {} is missing <addr_pcode>",
                String::from_utf8_lossy(&self.base.name)
            )));
        }
        if defaultaddr == -1 {
            return Err(KunaError::lowlevel(format!(
                "userop: {} is missing <default_pcode>",
                String::from_utf8_lossy(&self.base.name)
            )));
        }
        let base = names.get_by_name(&self.base.name).ok_or_else(|| {
            KunaError::lowlevel(format!(
                "Unknown userop name in <jumpassist>: {}",
                String::from_utf8_lossy(&self.base.name)
            ))
        })?;
        if !base.unspecialized {
            return Err(KunaError::lowlevel(format!(
                "<jumpassist> overloads userop with another purpose: {}",
                String::from_utf8_lossy(&self.base.name)
            )));
        }
        self.base.useropindex = base.index; // Get the index from the core userop
        self.kind = UserOpKind::JumpAssist(JumpAssistOp {
            index2case,
            index2addr,
            defaultaddr,
            calcsize,
        });
        Ok(())
    }
}

// C++ string concatenation `name + "_calcsize"`, etc.
fn suffixed(name: &[u8], suffix: &[u8]) -> Vec<u8> {
    let mut out = name.to_vec();
    out.extend_from_slice(suffix);
    out
}

// The two pcodeinject element ids the jumpassist decode keys on, re-exported
// locally for readability (they live in pcodeinject.rs).
use crate::pcodeinject::ELEM_ADDR_PCODE;

// ---------------------------------------------------------------------------
// UseropArchitecture seam (the `Architecture *glb` slice userop.cc uses)
// ---------------------------------------------------------------------------

/// STUB(W4/W6): the `Architecture *glb` surface `userop.cc` reaches for decode
/// and initialization.
///
/// `Architecture`/`TypeFactory`/`SymbolTable` are W4/W6. This trait names
/// exactly the operations the decode + registration paths use. The implementor
/// **owns the injection library** (C++ `glb->pcodeinjectlib`), so the
/// inject-decode methods mutate `self` rather than threading a separate library
/// handle:
///   - the translator's user-op names (`glb->translate->getUserOpNames`), used
///     by `UserOpManage::initialize`;
///   - the inject library decode (`glb->pcodeinjectlib->decodeInject`), which
///     in C++ allocates+decodes+registers a payload and returns its id; in the
///     port the actual SLEIGH allocate/decode lives in `inject_sleigh.rs`
///     (`PcodeInjectLibrarySleigh::decode_inject`), so this trait method is the
///     seam that wave fills.
///
/// (Data-type construction for the BUILTIN_MEM* DatatypeUserOps and the
/// VolatileRead/Write `getOutputLocal`/`getInputLocal` paths are W6; see
/// [`UseropTypeArchitecture`].)
pub trait UseropArchitecture: InjectArchitecture {
    /// C++ `glb->translate->getUserOpNames(res)`: the SLEIGH-assigned user-op
    /// names, indexed by CALLOTHER constant id (empty string => no op).
    fn get_user_op_names(&self) -> Vec<Vec<u8>>;

    /// C++ `glb->pcodeinjectlib->decodeInject(src,suffix,tp,decoder)`: allocate,
    /// decode, and register an injection payload, returning its id.
    fn decode_inject(
        &mut self,
        src: &[u8],
        suffix: &[u8],
        tp: int4,
        decoder: &mut dyn Decoder,
    ) -> KunaResult<int4>;

    /// C++ `glb->pcodeinjectlib->getCallOtherTarget(injectid)`: the
    /// callother-fixup target-op name for a registered injection id (used by
    /// `InjectedUserOp::decode` to recover the userop name).
    fn get_call_other_target(&self, injectid: int4) -> Vec<u8>;

    /// Return `(sizeOutput, sizeInput, input0Size, input1Size)` for a decoded
    /// payload, the four `payload->size*()`/`getInput(k).getSize()` reads
    /// `SegmentOp::decode` performs after the `<pcode>` child is parsed.
    fn payload_io_sizes(&self, injectid: int4) -> KunaResult<(int4, int4, int4, int4)>;
}

/// STUB(W6): the type-factory slice that builds the BUILTIN_MEM* DatatypeUserOp
/// types and the volatile-op local types.
///
/// `TypeFactory` is W6, so [`UserOpManage::register_builtin`] takes this as a
/// distinct trait: a caller that has W6 wired can register the typed builtins;
/// the others (string/volatile) need no type factory.
pub trait UseropTypeArchitecture {
    /// C++ `glb->types->getSizeOfPointer()`.
    fn get_size_of_pointer(&self) -> int4;
    /// C++ `glb->getDefaultDataSpace()->getWordSize()`.
    fn get_default_data_space_word_size(&self) -> int4;
    /// C++ `glb->types->getTypeVoid()`.
    fn get_type_void(&self) -> Rc<crate::dtype::Datatype>;
    /// C++ `glb->types->getTypePointer(ptrSize,baseType,wordSize)`.
    fn get_type_pointer(
        &self,
        ptr_size: int4,
        base: Rc<crate::dtype::Datatype>,
        word_size: int4,
    ) -> Rc<crate::dtype::Datatype>;
    /// C++ `glb->types->getBase(size,TYPE_INT)`.
    fn get_base_int(&self, size: int4) -> Rc<crate::dtype::Datatype>;
    /// C++ `glb->types->getTypeChar(glb->types->getSizeOfChar())`.
    fn get_type_char(&self) -> Rc<crate::dtype::Datatype>;
    /// C++ `glb->types->getTypeChar(glb->types->getSizeOfWChar())`.
    fn get_type_wchar(&self) -> Rc<crate::dtype::Datatype>;
}

// ---------------------------------------------------------------------------
// NameLookup: the snapshot of (name -> index, unspecialized?) the decode paths
// need from the manager mid-decode (avoids aliasing the &mut manager).
// ---------------------------------------------------------------------------

/// A small projection of the userop name table that `decode` consults: the
/// index and whether the existing op is `UnspecializedPcodeOp`. The C++
/// `decode` calls `glb->userops.getOp(name)` and reads `getIndex()` +
/// `dynamic_cast<UnspecializedPcodeOp*>`; this captures exactly those two reads.
#[derive(Debug, Clone, Default)]
pub struct NameLookup {
    map: BTreeMap<Vec<u8>, NameEntry>,
}

/// One entry of a [`NameLookup`].
#[derive(Debug, Clone)]
pub struct NameEntry {
    /// The op's CALLOTHER index (C++ `getIndex`).
    pub index: int4,
    /// Whether the op is an `UnspecializedPcodeOp` (C++ dynamic_cast test).
    pub unspecialized: bool,
}

impl NameLookup {
    /// Look up an entry by name (C++ `useropmap.find`).
    pub fn get_by_name(&self, name: &[u8]) -> Option<&NameEntry> {
        self.map.get(name)
    }
}

// ---------------------------------------------------------------------------
// UserOpManage (userop.hh:327-359 + userop.cc:368-637)
// ---------------------------------------------------------------------------

/// \brief Manager/container for description objects of user-defined p-code ops
/// (C++ `UserOpManage`).
///
/// The description objects are referenced by the CALLOTHER constant id (by name
/// during initialization). `useroplist` is sparse (holes are `None`), indexed by
/// id; `builtinmap`/`useropmap` are the C++ `std::map`s (BTree-ordered per ADR
/// 0002); `segmentop` is indexed by address-space index.
#[derive(Debug, Clone, Default)]
pub struct UserOpManage {
    /// Description objects indexed by CALLOTHER constant id (C++ `useroplist`).
    useroplist: Vec<Option<UserPcodeOp>>,
    /// Map from builtin ids to description objects (C++ `builtinmap`).
    builtinmap: BTreeMap<uint4, UserPcodeOp>,
    /// Map from op name to its index into `useroplist` (C++ `useropmap`, here a
    /// name->id map; the object lives in `useroplist`).
    useropmap: BTreeMap<Vec<u8>, int4>,
    /// Segment operations supported by this Architecture, indexed by space
    /// index (C++ `segmentop`; stores the index into `useroplist`).
    segmentop: Vec<Option<int4>>,
}

impl UserOpManage {
    /// Construct an empty manager (C++ `UserOpManage()`).
    pub fn new() -> UserOpManage {
        UserOpManage::default()
    }

    /// Build the [`NameLookup`] snapshot the decode paths consult.
    fn name_lookup(&self) -> NameLookup {
        let mut map = BTreeMap::new();
        for (name, &id) in self.useropmap.iter() {
            if let Some(Some(op)) = self.useroplist.get(id as usize) {
                map.insert(
                    name.clone(),
                    NameEntry { index: op.get_index(), unspecialized: op.is_unspecialized() },
                );
            }
        }
        NameLookup { map }
    }

    /// Number of segment operations supported (C++ `numSegmentOps`).
    pub fn num_segment_ops(&self) -> int4 {
        self.segmentop.len() as int4
    }

    /// \brief Initialize description objects for all user defined ops
    /// (C++ `UserOpManage::initialize`).
    ///
    /// Every user-defined p-code op presented by the architecture's translator
    /// is assigned a default `UnspecializedPcodeOp` description.
    pub fn initialize<A: UseropArchitecture>(&mut self, arch: &A) -> KunaResult<()> {
        let basicops = arch.get_user_op_names();
        for (i, nm) in basicops.iter().enumerate() {
            if nm.is_empty() {
                continue;
            }
            // C++ `new UnspecializedPcodeOp(basicops[i],glb,i)` — i is the
            // CALLOTHER index (uint4 -> int4 in the ctor).
            let userop = UserPcodeOp::new_unspecialized(nm, i as int4);
            self.register_op(userop)?;
        }
        Ok(())
    }

    /// \brief Retrieve a user-op description object by index (C++
    /// `getOp(uint4)`).
    ///
    /// Falls back to the builtin map for ids beyond `useroplist`.
    pub fn get_op(&self, i: uint4) -> Option<&UserPcodeOp> {
        if (i as usize) < self.useroplist.len() {
            return self.useroplist[i as usize].as_ref();
        }
        self.builtinmap.get(&i)
    }

    /// \brief Retrieve description by name (C++ `getOp(const string&)`).
    pub fn get_op_by_name(&self, nm: &[u8]) -> Option<&UserPcodeOp> {
        let id = *self.useropmap.get(nm)?;
        self.useroplist.get(id as usize)?.as_ref()
    }

    /// \brief Retrieve a segment-op description object by index
    /// (C++ `getSegmentOp`).
    pub fn get_segment_op(&self, i: int4) -> Option<&UserPcodeOp> {
        if i >= self.segmentop.len() as int4 {
            return None;
        }
        let id = self.segmentop[i as usize]?;
        self.useroplist.get(id as usize)?.as_ref()
    }

    /// \brief Make sure an active record exists for the given built-in op
    /// (C++ `UserOpManage::registerBuiltin`).
    ///
    /// The typed builtins (`BUILTIN_MEMCPY`/`STRNCPY`/`WCSNCPY`/`MEMSET`) need
    /// the W6 type factory, so this takes a [`UseropTypeArchitecture`]. Returns
    /// the (now present) builtin record.
    pub fn register_builtin<T: UseropTypeArchitecture>(
        &mut self,
        i: uint4,
        types: &T,
    ) -> KunaResult<&UserPcodeOp> {
        if self.builtinmap.contains_key(&i) {
            return Ok(self.builtinmap.get(&i).unwrap());
        }
        let res = match i {
            BUILTIN_STRINGDATA => UserPcodeOp::new_internal_string(),
            BUILTIN_VOLATILE_READ => UserPcodeOp::new_volatile_read(b"read_volatile", false),
            BUILTIN_VOLATILE_WRITE => UserPcodeOp::new_volatile_write(b"write_volatile", false),
            BUILTIN_MEMCPY => {
                let ptr_size = types.get_size_of_pointer();
                let word_size = types.get_default_data_space_word_size();
                let v_type = types.get_type_void();
                let ptr_type = types.get_type_pointer(ptr_size, v_type, word_size);
                let int_type = types.get_base_int(4);
                UserPcodeOp::new_datatype(
                    b"builtin_memcpy",
                    BUILTIN_MEMCPY as int4,
                    Some(Rc::clone(&ptr_type)),
                    &[Some(Rc::clone(&ptr_type)), Some(ptr_type), Some(int_type)],
                )
            }
            BUILTIN_STRNCPY => {
                let ptr_size = types.get_size_of_pointer();
                let word_size = types.get_default_data_space_word_size();
                let c_type = types.get_type_char();
                let ptr_type = types.get_type_pointer(ptr_size, c_type, word_size);
                let int_type = types.get_base_int(4);
                UserPcodeOp::new_datatype(
                    b"builtin_strncpy",
                    BUILTIN_STRNCPY as int4,
                    Some(Rc::clone(&ptr_type)),
                    &[Some(Rc::clone(&ptr_type)), Some(ptr_type), Some(int_type)],
                )
            }
            BUILTIN_WCSNCPY => {
                let ptr_size = types.get_size_of_pointer();
                let word_size = types.get_default_data_space_word_size();
                let c_type = types.get_type_wchar();
                let ptr_type = types.get_type_pointer(ptr_size, c_type, word_size);
                let int_type = types.get_base_int(4);
                UserPcodeOp::new_datatype(
                    b"builtin_wcsncpy",
                    BUILTIN_WCSNCPY as int4,
                    Some(Rc::clone(&ptr_type)),
                    &[Some(Rc::clone(&ptr_type)), Some(ptr_type), Some(int_type)],
                )
            }
            BUILTIN_MEMSET => {
                // (kuna) GH-9230 void *memset(void *dst,int val,int len)
                let ptr_size = types.get_size_of_pointer();
                let word_size = types.get_default_data_space_word_size();
                let v_type = types.get_type_void();
                let ptr_type = types.get_type_pointer(ptr_size, v_type, word_size);
                let int_type = types.get_base_int(4);
                UserPcodeOp::new_datatype(
                    b"builtin_memset",
                    BUILTIN_MEMSET as int4,
                    Some(Rc::clone(&ptr_type)),
                    &[Some(ptr_type), Some(Rc::clone(&int_type)), Some(int_type)],
                )
            }
            _ => return Err(KunaError::lowlevel("Bad built-in userop id")),
        };
        self.builtinmap.insert(i, res);
        Ok(self.builtinmap.get(&i).unwrap())
    }

    /// \brief Insert a new UserPcodeOp description object in the map(s)
    /// (C++ `UserOpManage::registerOp`).
    ///
    /// Sanity-checks conflicting indices and duplicate ops; if a `SegmentOp` is
    /// registered, also records it in the per-space `segmentop` table.
    pub fn register_op(&mut self, op: UserPcodeOp) -> KunaResult<()> {
        let ind = op.get_index();
        if ind < 0 {
            return Err(KunaError::lowlevel("UserOp not assigned an index"));
        }

        if let Some(&other_id) = self.useropmap.get(op.get_name()) {
            if let Some(Some(other)) = self.useroplist.get(other_id as usize) {
                if other.get_index() != ind {
                    return Err(KunaError::lowlevel(format!(
                        "Conflicting indices for userop name {}",
                        String::from_utf8_lossy(op.get_name())
                    )));
                }
            }
        }

        while (self.useroplist.len() as int4) <= ind {
            self.useroplist.push(None);
        }
        if let Some(existing) = &self.useroplist[ind as usize] {
            if existing.get_name() != op.get_name() {
                return Err(KunaError::lowlevel(format!(
                    "User op {} has same index as {}",
                    String::from_utf8_lossy(op.get_name()),
                    String::from_utf8_lossy(existing.get_name())
                )));
            }
            // We assume this registration customizes an existing userop; the
            // old spec is replaced below (C++ `delete useroplist[ind]`).
        }
        let name = op.get_name().to_vec();
        // Record segment ops in the per-space table (C++ dynamic_cast path).
        let segment_space_index = op.as_segment().and_then(|s| s.spc.as_ref().map(|s| s.get_index()));
        self.useroplist[ind as usize] = Some(op); // Index crossref
        self.useropmap.insert(name, ind); // Name crossref

        if let Some(index) = segment_space_index {
            while (self.segmentop.len() as int4) <= index {
                self.segmentop.push(None);
            }
            if self.segmentop[index as usize].is_some() {
                return Err(KunaError::lowlevel("Multiple segmentops defined for same space"));
            }
            self.segmentop[index as usize] = Some(ind);
        }
        Ok(())
    }

    /// \brief Parse a `<segmentop>` element and register it (C++
    /// `decodeSegmentOp`).
    pub fn decode_segment_op<A: UseropArchitecture>(
        &mut self,
        decoder: &mut dyn Decoder,
        arch: &mut A,
    ) -> KunaResult<()> {
        // C++ `new SegmentOp("",glb,useroplist.size())`.
        let mut s_op = UserPcodeOp::new_segment(b"", self.useroplist.len() as int4);
        let names = self.name_lookup();
        s_op.decode(decoder, arch, &names)?;
        self.register_op(s_op)
    }

    /// \brief Parse a `<volatile>` element and register the read/write ops
    /// (C++ `decodeVolatile`).
    ///
    /// The C++ reads only the `inputop`/`outputop`/`format` attributes here
    /// (the `<range>` children are consumed by the caller). The two
    /// VolatileRead/Write ops are placed directly into `builtinmap`.
    pub fn decode_volatile(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let mut read_op_name: Vec<u8> = Vec::new();
        let mut write_op_name: Vec<u8> = Vec::new();
        let mut functional_display = false;
        loop {
            let attrib_id = decoder.get_next_attribute_id()?;
            if attrib_id == 0 {
                break;
            }
            if attrib_id == ATTRIB_INPUTOP {
                read_op_name = decoder.read_string()?;
            } else if attrib_id == ATTRIB_OUTPUTOP {
                write_op_name = decoder.read_string()?;
            } else if attrib_id == ATTRIB_FORMAT {
                let format = decoder.read_string()?;
                if format == b"functional" {
                    functional_display = true;
                }
            }
        }
        if read_op_name.is_empty() || write_op_name.is_empty() {
            return Err(KunaError::lowlevel(
                "Missing inputop/outputop attributes in <volatile> element",
            ));
        }
        if self.builtinmap.contains_key(&BUILTIN_VOLATILE_READ) {
            return Err(KunaError::lowlevel("read_volatile user-op registered more than once"));
        }
        if self.builtinmap.contains_key(&BUILTIN_VOLATILE_WRITE) {
            return Err(KunaError::lowlevel("write_volatile user-op registered more than once"));
        }
        let vr_op = UserPcodeOp::new_volatile_read(&read_op_name, functional_display);
        self.builtinmap.insert(BUILTIN_VOLATILE_READ, vr_op);
        let vw_op = UserPcodeOp::new_volatile_write(&write_op_name, functional_display);
        self.builtinmap.insert(BUILTIN_VOLATILE_WRITE, vw_op);
        Ok(())
    }

    /// \brief Parse a `<callotherfixup>` element and register it (C++
    /// `decodeCallOtherFixup`).
    pub fn decode_call_other_fixup<A: UseropArchitecture>(
        &mut self,
        decoder: &mut dyn Decoder,
        arch: &mut A,
    ) -> KunaResult<()> {
        // C++ `new InjectedUserOp("",glb,0,0)`.
        let mut op = UserPcodeOp::new_injected(b"", 0, 0);
        let names = self.name_lookup();
        op.decode(decoder, arch, &names)?;
        self.register_op(op)
    }

    /// \brief Parse a `<jumpassist>` element and register it (C++
    /// `decodeJumpAssist`).
    pub fn decode_jump_assist<A: UseropArchitecture>(
        &mut self,
        decoder: &mut dyn Decoder,
        arch: &mut A,
    ) -> KunaResult<()> {
        let mut op = UserPcodeOp::new_jump_assist();
        let names = self.name_lookup();
        op.decode(decoder, arch, &names)?;
        self.register_op(op)
    }

    /// \brief Manually install an InjectedUserOp given just names of the user
    /// defined op and the p-code snippet (C++ `manualCallOtherFixup`).
    ///
    /// STUB(`inject_sleigh.rs`): the actual snippet compile
    /// (`pcodeinjectlib->manualCallOtherFixup`) is the SLEIGH library's; the
    /// caller passes the resulting injectid through.
    pub fn manual_call_other_fixup(
        &mut self,
        useropname: &[u8],
        injectid: int4,
    ) -> KunaResult<()> {
        let (exists, unspecialized, index) = match self.get_op_by_name(useropname) {
            Some(op) => (true, op.is_unspecialized(), op.get_index()),
            None => (false, false, 0),
        };
        if !exists {
            return Err(KunaError::lowlevel(format!(
                "Unknown userop: {}",
                String::from_utf8_lossy(useropname)
            )));
        }
        if !unspecialized {
            return Err(KunaError::lowlevel(format!(
                "Cannot fixup userop: {}",
                String::from_utf8_lossy(useropname)
            )));
        }
        let op = UserPcodeOp::new_injected(useropname, index, injectid);
        self.register_op(op)
    }
}

#[cfg(test)]
mod tests;
