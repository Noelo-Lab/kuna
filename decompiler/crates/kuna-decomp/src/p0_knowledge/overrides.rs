//! Port of `decompiler/cpp/override.{cc,hh}` — the P0 override store.
//!
//! NAMING LOSS: the C++ class is `Override`, but `override` is a Rust reserved
//! keyword, so the module is named `overrides` and the type stays [`Override`]
//! (the type name is fine; only the *module path* is renamed).
//!
//! [`Override`] is a container of commands that override the decompiler's
//! default behavior for a single function:
//!
//!   - sub-functions: how they are called (`flowoverride`) and where they call
//!     to (`indirectover` direct-call redirect, `protoover` prototype replace);
//!   - jumptables: indirect jumps needing multistage analysis (`multistagejump`);
//!   - deadcode: per-address-space dead-code-elimination delay (`deadcodedelay`);
//!   - data-flow / control-flow: force a branch to be an unstructured goto
//!     (`forcegoto`).
//!
//! These commands live *independently* of the main data-flow / control-flow /
//! symbol structures and **survive a decompilation restart** — that is the whole
//! point of the P0 store (ADR 0007).  A few analyses, mid-transformation, insert
//! a new command to fix a problem discovered too late and then force a restart
//! via `Funcdata::setRestartPending()`.
//!
//! ### Dead-code-delay bump (how `heritage.cc` consumes it)
//!
//! `Heritage::bumpDeadcodeDelay(spc)` (heritage.cc:2572) is the auto-bump:
//!   1. it only fires for `IPTR_PROCESSOR` / `IPTR_SPACEBASE` spaces and only
//!      when `spc->getDelay() == spc->getDeadcodeDelay()` (no global delay yet);
//!   2. if [`Override::has_deadcode_delay`] already returns `true` for the space
//!      it returns without re-bumping (the delay is installed *once*);
//!   3. otherwise it calls
//!      `Override::insert_deadcode_delay(spc, spc->getDeadcodeDelay()+1)` and
//!      `setRestartPending(true)`.
//!
//! Because the [`Override`] is the P0 store, the bumped delay is read back on the
//! *next* decompilation pass (the restart) via [`Override::deadcode_delays`] →
//! `Funcdata::setDeadCodeDelay` → `Heritage::setDeadCodeDelay`.  The
//! [`Override::has_deadcode_delay`] guard — `val != spc->getDeadcodeDelay()` — is
//! what makes the bump idempotent across restarts: once installed, the stored
//! value differs from the space's static delay, so the predicate stays `true`
//! and the bump is never repeated.
//!
//! ### Seams
//!
//! - The prototype override (`protoover : map<Address,FuncProto*>`) holds the W4
//!   `FuncProto` (`fspec.cc`, not yet ported).  Its insert/query/encode/printRaw
//!   surface is captured by the [`FuncProtoOverride`] trait so the store, the
//!   merge-on-insert rule, and the encode walk are faithful; the W4 wiring point
//!   implements [`FuncProtoOverride`] for the real `FuncProto`.  // STUB(W4)
//! - `applyPrototype`/`applyIndirect` copy into a `FuncCallSpecs` (W4) and
//!   `applyDeadCodeDelay`/`applyForceGoto` drive `Funcdata`/`Heritage`.  Those
//!   are exposed as query/iterator accessors here ([`Override::find_proto_override`],
//!   [`Override::find_indirect_override`], [`Override::deadcode_delays`],
//!   [`Override::force_gotos`]) so the apply logic can live at the W4/W5 call
//!   site without this module depending on the unported subsystems.  // STUB(W4)

use std::collections::BTreeMap;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{Decoder, ElementId, Encoder, IdRegistry, ATTRIB_SPACE, ATTRIB_TYPE};
use kuna_base::space::{AddrSpace, AddrSpaceManager, ATTRIB_DELAY};
use kuna_base::types::{int4, uint4};

// ---------------------------------------------------------------------------
// Marshaling ids (verbatim from override.cc:21-27)
// ---------------------------------------------------------------------------

/// Marshaling element `<deadcodedelay>` (`override.cc:21`).
pub const ELEM_DEADCODEDELAY: ElementId = ElementId::new("deadcodedelay", 218);
/// Marshaling element `<flow>` (`override.cc:22`).
pub const ELEM_FLOW: ElementId = ElementId::new("flow", 219);
/// Marshaling element `<forcegoto>` (`override.cc:23`).
pub const ELEM_FORCEGOTO: ElementId = ElementId::new("forcegoto", 220);
/// Marshaling element `<indirectoverride>` (`override.cc:24`).
pub const ELEM_INDIRECTOVERRIDE: ElementId = ElementId::new("indirectoverride", 221);
/// Marshaling element `<multistagejump>` (`override.cc:25`).
pub const ELEM_MULTISTAGEJUMP: ElementId = ElementId::new("multistagejump", 222);
/// Marshaling element `<override>` (`override.cc:26`).
pub const ELEM_OVERRIDE: ElementId = ElementId::new("override", 223);
/// Marshaling element `<protooverride>` (`override.cc:27`).
pub const ELEM_PROTOOVERRIDE: ElementId = ElementId::new("protooverride", 224);

/// Register every override marshaling id with an [`IdRegistry`] so a name-driven
/// `XmlDecode` resolves `<override>`/`<forcegoto>`/etc. (the C++ global
/// static-init side effect; the Rust registry is explicit).
pub fn register_ids(reg: &mut IdRegistry) {
    for e in [
        &ELEM_DEADCODEDELAY,
        &ELEM_FLOW,
        &ELEM_FORCEGOTO,
        &ELEM_INDIRECTOVERRIDE,
        &ELEM_MULTISTAGEJUMP,
        &ELEM_OVERRIDE,
        &ELEM_PROTOOVERRIDE,
    ] {
        reg.register_element(e);
    }
}

// ---------------------------------------------------------------------------
// Branch override types (override.hh:53-59)
// ---------------------------------------------------------------------------

/// Enumeration of possible branch overrides (C++ `Override` anonymous enum).
pub mod flow_type {
    use kuna_base::types::uint4;
    /// No override.
    pub const NONE: uint4 = 0;
    /// Replace primary CALL or RETURN with suitable BRANCH operation.
    pub const BRANCH: uint4 = 1;
    /// Replace primary BRANCH or RETURN with suitable CALL operation.
    pub const CALL: uint4 = 2;
    /// Replace primary BRANCH or RETURN with suitable CALL/RETURN operation.
    pub const CALL_RETURN: uint4 = 3;
    /// Replace primary BRANCH or CALL with a suitable RETURN operation.
    pub const RETURN: uint4 = 4;
}

// ---------------------------------------------------------------------------
// FuncProto seam (override.hh: map<Address,FuncProto *> protoover)
// ---------------------------------------------------------------------------

/// \brief The slice of the W4 `FuncProto` that [`Override`] drives.
///
/// STUB(W4): the prototype override stores a `FuncProto*` (fspec.cc, not yet
/// ported).  [`Override`] only ever (a) marks it as an override
/// (`p->setOverride(true)`), (b) encodes it (`(*fiter).second->encode(encoder)`),
/// and (c) prints it (`(*fiter).second->printRaw("func", s)`); the copy into a
/// `FuncCallSpecs` happens at the W4 call site, not here.  This trait captures
/// exactly that surface so the store + merge-on-insert rule are faithful.
pub trait FuncProtoOverride {
    /// Mark this prototype as an override (C++ `FuncProto::setOverride(true)`).
    fn set_override(&mut self, val: bool);

    /// Encode the prototype to a stream (C++ `FuncProto::encode`).
    fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()>;

    /// Append a debug description of the prototype (C++ `FuncProto::printRaw`,
    /// here always with the C++ literal name `"func"`).
    fn print_raw(&self, s: &mut String);

    /// The parsed [`PrototypePieces`](crate::fspec::PrototypePieces) this override
    /// forces, when it is a parsed-declaration override (`override prototype <addr>
    /// <decl>`).  `None` for an XML-decoded / non-pieces override.  Drives
    /// `FlowInfo`'s flow-time `applyPrototype` (C++ `Override::applyPrototype` →
    /// `fspecs.copy(*proto)`): the pieces are built into a locked `FuncProto`
    /// copied onto the matching call spec.
    fn pieces(&self) -> Option<&crate::fspec::PrototypePieces> {
        None
    }
}

/// A [`FuncProtoOverride`] backed by parsed [`PrototypePieces`](crate::fspec::PrototypePieces),
/// the value an `override prototype <addr> <decl>` installs (C++ `IfcProtooverride`
/// builds a `FuncProto` from the pieces; the kuna store keeps the pieces and
/// reconstructs the `FuncProto` at flow-time `applyPrototype`).  Mirrors the
/// console's wrapper; lives here so the re-seed path
/// ([`crate::decompile_drive`]) can construct it without depending on the console
/// crate.
pub struct PiecesProtoOverride {
    /// The parsed prototype declaration.
    pub pieces: crate::fspec::PrototypePieces,
}

impl FuncProtoOverride for PiecesProtoOverride {
    fn set_override(&mut self, _val: bool) {
        // The override flag rides on the rebuilt `FuncProto` (set in
        // `ArchFlowEnv::build_override_proto`), not on the pieces.
    }
    fn encode(&self, _encoder: &mut dyn Encoder) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "kuna rust port: prototype-override encode needs the W4 FuncProto::encode",
        ))
    }
    fn print_raw(&self, s: &mut String) {
        s.push_str("func(");
        s.push_str(&self.pieces.intypes.len().to_string());
        s.push(')');
    }
    fn pieces(&self) -> Option<&crate::fspec::PrototypePieces> {
        Some(&self.pieces)
    }
}

/// The W4 construct-and-decode callback used by [`Override::decode`] for the
/// `<protooverride>` branch.
///
/// STUB(W4): the C++ builds a `FuncProto` from `glb->defaultfp` /
/// `glb->types->getTypeVoid()` and calls `fp->decode(decoder, glb)`.  Both the
/// default prototype model and the type factory are W4, so the construction is
/// supplied by the caller as this callback (which decodes the child element and
/// returns the boxed override).
pub type ProtoDecoder<'a> =
    dyn FnMut(&mut dyn Decoder) -> KunaResult<Box<dyn FuncProtoOverride>> + 'a;

// ---------------------------------------------------------------------------
// Override (override.hh:50-92, override.cc:29-433)
// ---------------------------------------------------------------------------

/// \brief A container of commands that override the decompiler's default
/// behavior for a single function (C++ `Override`, the P0 store).
///
/// The C++ maps key on `Address` (`std::map`, ordered) — ported to `BTreeMap`
/// with the same [`Address`] ordering.  `deadcodedelay` and `multistagejump` are
/// the C++ `vector`s, ported to `Vec`.  The prototype-override values are boxed
/// [`FuncProtoOverride`] trait objects (STUB(W4)).
///
/// `Default` constructs the empty store (the C++ default-constructed `Override`).
#[derive(Default)]
pub struct Override {
    /// Force goto on jump at \b targetpc to \b destpc.
    forcegoto: BTreeMap<Address, Address>,
    /// Delay count indexed by address space (C++ `vector<int4>`; `-1` is "no
    /// override installed for this space").
    deadcodedelay: Vec<int4>,
    /// Override indirect at \b call-point into direct to \b addr.
    indirectover: BTreeMap<Address, Address>,
    /// Override prototype at \b call-point (STUB(W4): boxed `FuncProto`).
    protoover: BTreeMap<Address, Box<dyn FuncProtoOverride>>,
    /// Addresses of indirect jumps that need multistage recovery.
    multistagejump: Vec<Address>,
    /// Override the CALL <-> BRANCH interpretation of a branch instruction.
    flowoverride: BTreeMap<Address, uint4>,
}

impl Override {
    /// Construct the empty override store (C++ default constructor).
    pub fn new() -> Override {
        Override::default()
    }

    /// Clear the entire set of overrides (C++ `Override::clear`).
    ///
    /// The C++ `delete`s each `protoover` value; the Rust `Box`es drop on clear.
    pub fn clear(&mut self) {
        self.forcegoto.clear();
        self.deadcodedelay.clear();
        self.indirectover.clear();
        self.protoover.clear();
        self.multistagejump.clear();
        self.flowoverride.clear();
    }

    /// Generate the \e warning message related to a dead-code delay
    /// (C++ `Override::generateDeadcodeDelayMessage`).
    ///
    /// \param index is the index of the address space
    /// \param glb is the Architecture's address-space manager
    fn generate_deadcode_delay_message(index: int4, glb: &AddrSpaceManager) -> String {
        let spc = glb.get_space(index).expect("generateDeadcodeDelayMessage: null space");
        format!("Restarted to delay deadcode elimination for space: {}", spc.get_name())
    }

    /// Force a specific branch instruction to be an unstructured \e goto
    /// (C++ `Override::insertForceGoto`).
    pub fn insert_force_goto(&mut self, targetpc: Address, destpc: Address) {
        self.forcegoto.insert(targetpc, destpc);
    }

    /// Override the number of passes executed before \e dead-code elimination
    /// starts, for a specific address space (C++ `Override::insertDeadcodeDelay`).
    ///
    /// The `deadcodedelay` vector is grown with `-1` placeholders up to the
    /// space's index — exactly the C++ `while (size <= index) push_back(-1)`.
    pub fn insert_deadcode_delay(&mut self, spc: &AddrSpace, delay: int4) {
        // C++ compares `size()` (a size_t) against `getIndex()` (an int4); the
        // index is always non-negative for a real space, so the comparison is
        // safe as `usize <= usize`.
        let index = spc.get_index();
        while (self.deadcodedelay.len() as i64) <= index as i64 {
            self.deadcodedelay.push(-1);
        }
        self.deadcodedelay[index as usize] = delay;
    }

    /// Check if a delay override is already installed for an address space
    /// (C++ `Override::hasDeadcodeDelay`).
    ///
    /// Returns `false` when no slot exists, when the slot is the `-1`
    /// placeholder, or when the stored value equals the space's static delay
    /// (`val != spc->getDeadcodeDelay()`).  This last check is what makes the
    /// heritage auto-bump idempotent across restarts (see module docs).
    pub fn has_deadcode_delay(&self, spc: &AddrSpace) -> bool {
        let index = spc.get_index();
        // C++ `if (index >= deadcodedelay.size())`: index (int4) vs size (size_t).
        if index as usize >= self.deadcodedelay.len() {
            return false;
        }
        let val = self.deadcodedelay[index as usize];
        if val == -1 {
            return false;
        }
        val != spc.get_deadcode_delay()
    }

    /// Override an indirect call, turning it into a direct call
    /// (C++ `Override::insertIndirectOverride`).
    pub fn insert_indirect_override(&mut self, callpoint: Address, directcall: Address) {
        self.indirectover.insert(callpoint, directcall);
    }

    /// Override the assumed function prototype at a specific call site
    /// (C++ `Override::insertProtoOverride`).
    ///
    /// The C++ deletes any pre-existing override at the same call point, marks
    /// the new prototype as an override (`setOverride(true)`), and takes
    /// ownership.  The `BTreeMap::insert` drops the old `Box` (the `delete`),
    /// preserving the merge rule "newest override at a call point wins".
    pub fn insert_proto_override(&mut self, callpoint: Address, mut p: Box<dyn FuncProtoOverride>) {
        p.set_override(true); // Mark this as an override
        self.protoover.insert(callpoint, p); // Takes ownership; drops any prior value
    }

    /// Flag an indirect jump for multistage analysis
    /// (C++ `Override::insertMultistageJump`).
    pub fn insert_multistage_jump(&mut self, addr: Address) {
        self.multistagejump.push(addr);
    }

    /// Mark a branch instruction with a different flow type
    /// (C++ `Override::insertFlowOverride`).
    pub fn insert_flow_override(&mut self, addr: Address, type_: uint4) {
        self.flowoverride.insert(addr, type_);
    }

    /// Look up a function-prototype override at a call point (C++
    /// `Override::applyPrototype`'s map find).
    ///
    /// STUB(W4): the C++ then does `fspecs.copy(*proto)`; the copy into the W4
    /// `FuncCallSpecs` happens at the call site, so the port returns the matched
    /// override (or `None`) for the caller to copy in.  Returns `None`
    /// immediately when the store is empty (the C++ `if (!protoover.empty())`).
    pub fn find_proto_override(&self, callpoint: &Address) -> Option<&dyn FuncProtoOverride> {
        if self.protoover.is_empty() {
            return None;
        }
        self.protoover.get(callpoint).map(|b| b.as_ref())
    }

    /// Look up a destination override for an indirect call (C++
    /// `Override::applyIndirect`'s map find).
    ///
    /// STUB(W4): the C++ then does `fspecs.setAddress(addr)`; the port returns
    /// the overriding direct-call target (or `None`).  Returns `None` when the
    /// store is empty (the C++ `if (!indirectover.empty())`).
    pub fn find_indirect_override(&self, callpoint: &Address) -> Option<&Address> {
        if self.indirectover.is_empty() {
            return None;
        }
        self.indirectover.get(callpoint)
    }

    /// Check for a multistage marker for a specific indirect jump
    /// (C++ `Override::queryMultistageJumptable`).
    ///
    /// Faithfully a linear scan in insertion order (the C++ `vector` walk), not
    /// a set lookup — a duplicate-insert stays a duplicate.
    pub fn query_multistage_jumptable(&self, addr: &Address) -> bool {
        for entry in &self.multistagejump {
            if entry == addr {
                return true;
            }
        }
        false
    }

    /// The force-goto overrides, in [`Address`] order (C++ `applyForceGoto`'s
    /// `forcegoto` walk feeding `data.forceGoto(target, dest)`).
    ///
    /// STUB(W5): the C++ `applyForceGoto` calls `Funcdata::forceGoto` for each
    /// entry; the port exposes the ordered `(target, dest)` pairs so the W5 call
    /// site drives the (already-ported) `Funcdata::force_goto`.
    pub fn force_gotos(&self) -> impl Iterator<Item = (&Address, &Address)> {
        self.forcegoto.iter()
    }

    /// The installed dead-code delays as `(space_index, delay)` pairs, skipping
    /// the `-1` placeholders (C++ `applyDeadCodeDelay`'s `deadcodedelay` walk).
    ///
    /// STUB(W5): the C++ `applyDeadCodeDelay` looks up `glb->getSpace(i)` and
    /// calls `data.setDeadCodeDelay(spc, delay)`; the port yields the
    /// `(index, delay)` pairs for the W5 call site to resolve and apply.
    pub fn deadcode_delays(&self) -> impl Iterator<Item = (int4, int4)> + '_ {
        self.deadcodedelay
            .iter()
            .enumerate()
            // i is a vector index (< len); int4 is i32, the C++ loop counter type.
            .filter(|(_, &delay)| delay >= 0)
            .map(|(i, &delay)| (i as int4, delay))
    }

    /// Return the particular flow override at a given address
    /// (C++ `Override::getFlowOverride`).  Returns [`flow_type::NONE`] when there
    /// is no override at the address.
    pub fn get_flow_override(&self, addr: &Address) -> uint4 {
        match self.flowoverride.get(addr) {
            None => flow_type::NONE,
            Some(&v) => v,
        }
    }

    /// Are there any flow overrides (C++ `Override::hasFlowOverride`).
    pub fn has_flow_override(&self) -> bool {
        !self.flowoverride.is_empty()
    }

    /// Dump a description of the overrides to a string, one per line
    /// (C++ `Override::printRaw`).
    ///
    /// \param s receives the debug text
    /// \param glb is the Architecture's address-space manager
    pub fn print_raw(&self, s: &mut String, glb: &AddrSpaceManager) {
        for (target, dest) in &self.forcegoto {
            let mut a = String::new();
            let mut b = String::new();
            // print_raw on a valid address cannot fail; an m_maximal sentinel
            // would be C++ UB and panics there too.
            target.print_raw(&mut a).expect("printRaw: forcegoto target");
            dest.print_raw(&mut b).expect("printRaw: forcegoto dest");
            s.push_str(&format!("force goto at {a} jumping to {b}\n"));
        }

        for (i, &delay) in self.deadcodedelay.iter().enumerate() {
            if delay < 0 {
                continue;
            }
            let spc = glb.get_space(i as int4).expect("printRaw: null deadcode space");
            s.push_str(&format!("dead code delay on {} set to {}\n", spc.get_name(), delay));
        }

        for (callpoint, directcall) in &self.indirectover {
            let mut a = String::new();
            let mut b = String::new();
            callpoint.print_raw(&mut a).expect("printRaw: indirect callpoint");
            directcall.print_raw(&mut b).expect("printRaw: indirect target");
            s.push_str(&format!("override indirect at {a} to call directly to {b}\n"));
        }

        for (callpoint, proto) in &self.protoover {
            let mut a = String::new();
            callpoint.print_raw(&mut a).expect("printRaw: proto callpoint");
            s.push_str(&format!("override prototype at {a} to "));
            proto.print_raw(s);
            s.push('\n');
        }
    }

    /// Create warning messages that describe current overrides
    /// (C++ `Override::generateOverrideMessages`).
    ///
    /// Messages are designed to be displayed in the function header comment.
    /// Only the dead-code delays generate messages.
    pub fn generate_override_messages(&self, glb: &AddrSpaceManager) -> Vec<String> {
        let mut messagelist = Vec::new();
        for (i, &delay) in self.deadcodedelay.iter().enumerate() {
            if delay >= 0 {
                messagelist.push(Override::generate_deadcode_delay_message(i as int4, glb));
            }
        }
        messagelist
    }

    /// Encode the override commands to a stream (C++ `Override::encode`).
    ///
    /// All commands are written as children of a root `<override>` element; the
    /// element is omitted entirely when every store is empty.
    ///
    /// \param encoder is the stream encoder
    /// \param glb is the Architecture's address-space manager
    pub fn encode(&self, encoder: &mut dyn Encoder, glb: &AddrSpaceManager) -> KunaResult<()> {
        if self.forcegoto.is_empty()
            && self.deadcodedelay.is_empty()
            && self.indirectover.is_empty()
            && self.protoover.is_empty()
            && self.multistagejump.is_empty()
            && self.flowoverride.is_empty()
        {
            return Ok(());
        }
        encoder.open_element(&ELEM_OVERRIDE);

        for (target, dest) in &self.forcegoto {
            encoder.open_element(&ELEM_FORCEGOTO);
            target.encode(encoder)?;
            dest.encode(encoder)?;
            encoder.close_element(&ELEM_FORCEGOTO);
        }

        for (i, &delay) in self.deadcodedelay.iter().enumerate() {
            if delay < 0 {
                continue;
            }
            let spc = glb.get_space(i as int4).expect("encode: null deadcode space");
            encoder.open_element(&ELEM_DEADCODEDELAY);
            encoder.write_space(&ATTRIB_SPACE, spc);
            encoder.write_signed_integer(&ATTRIB_DELAY, delay as i64);
            encoder.close_element(&ELEM_DEADCODEDELAY);
        }

        for (callpoint, directcall) in &self.indirectover {
            encoder.open_element(&ELEM_INDIRECTOVERRIDE);
            callpoint.encode(encoder)?;
            directcall.encode(encoder)?;
            encoder.close_element(&ELEM_INDIRECTOVERRIDE);
        }

        for (callpoint, proto) in &self.protoover {
            encoder.open_element(&ELEM_PROTOOVERRIDE);
            callpoint.encode(encoder)?;
            proto.encode(encoder)?;
            encoder.close_element(&ELEM_PROTOOVERRIDE);
        }

        for addr in &self.multistagejump {
            encoder.open_element(&ELEM_MULTISTAGEJUMP);
            addr.encode(encoder)?;
            encoder.close_element(&ELEM_MULTISTAGEJUMP);
        }

        for (addr, &type_) in &self.flowoverride {
            encoder.open_element(&ELEM_FLOW);
            encoder.write_string(&ATTRIB_TYPE, Override::type_to_string(type_).as_bytes());
            addr.encode(encoder)?;
            encoder.close_element(&ELEM_FLOW);
        }
        encoder.close_element(&ELEM_OVERRIDE);
        Ok(())
    }

    /// Parse an `<override>` element containing override commands
    /// (C++ `Override::decode`).
    ///
    /// STUB(W4): the `<protooverride>` branch constructs a `FuncProto` from
    /// `glb->defaultfp` / `glb->types->getTypeVoid()` and decodes it — both W4.
    /// The port takes a `proto_decoder` callback that performs that W4
    /// construction+decode (and returns the boxed [`FuncProtoOverride`]); the
    /// rest of the element walk is faithful.
    pub fn decode(
        &mut self,
        decoder: &mut dyn Decoder,
        proto_decoder: &mut ProtoDecoder<'_>,
    ) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_OVERRIDE)?;
        loop {
            let sub_id = decoder.open_element()?;
            if sub_id == 0 {
                break;
            }
            if sub_id == ELEM_INDIRECTOVERRIDE {
                let callpoint = Address::decode(decoder)?;
                let directcall = Address::decode(decoder)?;
                self.insert_indirect_override(callpoint, directcall);
            } else if sub_id == ELEM_PROTOOVERRIDE {
                let callpoint = Address::decode(decoder)?;
                let fp = proto_decoder(decoder)?;
                self.insert_proto_override(callpoint, fp);
            } else if sub_id == ELEM_FORCEGOTO {
                let targetpc = Address::decode(decoder)?;
                let destpc = Address::decode(decoder)?;
                self.insert_force_goto(targetpc, destpc);
            } else if sub_id == ELEM_DEADCODEDELAY {
                let delay = decoder.read_signed_integer_id(&ATTRIB_DELAY)?;
                let spc = decoder.read_space_id(&ATTRIB_SPACE)?;
                if delay < 0 {
                    return Err(KunaError::lowlevel("Bad deadcodedelay tag"));
                }
                // delay was just checked non-negative; it fits int4 as in the
                // C++ `readSignedInteger` -> int4 assignment.
                self.insert_deadcode_delay(&spc, delay as int4);
            } else if sub_id == ELEM_MULTISTAGEJUMP {
                let callpoint = Address::decode(decoder)?;
                self.insert_multistage_jump(callpoint);
            } else if sub_id == ELEM_FLOW {
                let type_ = Override::string_to_type(&decoder.read_string_id(&ATTRIB_TYPE)?);
                let addr = Address::decode(decoder)?;
                if type_ == flow_type::NONE || addr.is_invalid() {
                    return Err(KunaError::lowlevel("Bad flowoverride tag"));
                }
                self.insert_flow_override(addr, type_);
            }
            decoder.close_element(sub_id)?;
        }
        decoder.close_element(elem_id)?;
        Ok(())
    }

    /// Convert a flow override type to a string (C++ `Override::typeToString`).
    pub fn type_to_string(tp: uint4) -> &'static str {
        if tp == flow_type::BRANCH {
            return "branch";
        }
        if tp == flow_type::CALL {
            return "call";
        }
        if tp == flow_type::CALL_RETURN {
            return "callreturn";
        }
        if tp == flow_type::RETURN {
            return "return";
        }
        "none"
    }

    /// Convert a string to a flow override type (C++ `Override::stringToType`).
    pub fn string_to_type(nm: &[u8]) -> uint4 {
        if nm == b"branch" {
            flow_type::BRANCH
        } else if nm == b"call" {
            flow_type::CALL
        } else if nm == b"callreturn" {
            flow_type::CALL_RETURN
        } else if nm == b"return" {
            flow_type::RETURN
        } else {
            flow_type::NONE
        }
    }
}

#[cfg(test)]
mod tests;
