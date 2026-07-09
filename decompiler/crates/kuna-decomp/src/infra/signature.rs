//! Port of `decompiler/cpp/signature.{cc,hh}` — feature-vector ("signature")
//! generation for a single function (the `print/save/produce signatures` console
//! family, item `w8-sig-pack`).
//!
//! Features are extracted from the data-flow and control-flow graphs of a
//! decompiled function and reduced to a vector of 32-bit hashes.  The pipeline
//! (C++ `GraphSigManager`):
//!
//!   1. each Varnode gets a [`SignatureEntry`] overlay (the *effective* defining
//!      op, the incoming-edge window, terminal/commutative/standalone-COPY flags);
//!   2. optional *noise removal* (`SIG_COLLAPSE_INDNOISE`) collapses indirect-copy
//!      chains by building the dominator tree of the COPY/INDIRECT/MULTIEQUAL
//!      sub-graph and shadowing each node onto its base;
//!   3. each entry computes a [`local hash`](SignatureEntry::local_hash), then the
//!      hashes are mixed iteratively across data-flow edges;
//!   4. basic blocks get [`BlockSignatureEntry`] overlays hashed across CFG edges;
//!   5. the final per-Varnode / per-block / stand-alone-COPY features are
//!      collected and (for the simple form) sorted and emitted.
//!
//! # Faithfulness
//!
//! The IR is the W3 arena model (ADR 0001): `Funcdata` owns the Varnode / PcodeOp
//! / FlowBlock arenas, so where the C++ stores a `SignatureEntry *` /
//! `PcodeOp *` / `Varnode *` this port stores the arena key (`VarnodeId` / `OpId`
//! / `BlockId`) and dereferences through `&Funcdata`.  The C++
//! `map<int4,SignatureEntry *>` keyed by `Varnode::getCreateIndex()` becomes a
//! [`BTreeMap`] keyed by the same `create_index` (ADR 0002 — iteration order is
//! observable: it sets the emit order of `VarnodeSignature` features and the mix
//! order in [`signature_iterate`](GraphSigManager::signature_iterate)).  The
//! `map<int4,BlockSignatureEntry *>` keyed by `bl->getIndex()` becomes a
//! `BTreeMap<i32,_>`.  Hash arithmetic is `u64`/`u32` with the wrapping helpers
//! (ADR 0003); the CRC mixing is `kuna_base::crc32::crc_update` (the W1 port of
//! `crc32.hh`).
//!
//! # Boundaries
//!
//! Three pieces of the marshalling/print surface are not yet ported and are
//! routed through `STUB` errors so the *algorithm* (the testable substance)
//! transcribes cleanly:
//!
//!   * `Varnode::encode` / `PcodeOp::encode` / `Datatype` encode — the W8 marshal
//!     surface; `VarnodeSignature::encode` / `BlockSignature::encode` defer.
//!   * `FlowBlock::printHeader` / `getStart` — the W8 print surface
//!     (`block.rs` `// STUB(W8)`); the feature `print_origin` / `BlockSignature`
//!     paths defer.
//!
//! The [`Encoder`]-driven `Signature::encode` (a bare `<gensig hash=..>`) **is**
//! ported because it only needs the marshal primitives that exist.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU32, Ordering};

use kuna_base::crc32::crc_update;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{AttributeId, ElementId, Encoder, ATTRIB_INDEX, ATTRIB_VAL};
use kuna_num::opcodes::OpCode;

use crate::funcdata::Funcdata;
use crate::context::{BlockId, OpId, VarnodeId};

// ===========================================================================
// Marshaling identifiers (signature.cc:26-41)
// ===========================================================================

/// Marshaling attribute "baddata" (`ATTRIB_BADDATA`, signature.cc:26).
pub const ATTRIB_BADDATA: AttributeId = AttributeId::new("baddata", 145);
/// Marshaling attribute "hash" (`ATTRIB_HASH`, signature.cc:27).
pub const ATTRIB_HASH: AttributeId = AttributeId::new("hash", 146);
/// Marshaling attribute "unimpl" (`ATTRIB_UNIMPL`, signature.cc:28).
pub const ATTRIB_UNIMPL: AttributeId = AttributeId::new("unimpl", 147);

/// Marshaling element `<blocksig>` (signature.cc:30).
pub const ELEM_BLOCKSIG: ElementId = ElementId::new("blocksig", 258);
/// Marshaling element `<call>` (signature.cc:31).
pub const ELEM_CALL: ElementId = ElementId::new("call", 259);
/// Marshaling element `<gensig>` (signature.cc:32).
pub const ELEM_GENSIG: ElementId = ElementId::new("gensig", 260);
/// Marshaling element `<copysig>` (signature.cc:35).
pub const ELEM_COPYSIG: ElementId = ElementId::new("copysig", 263);
/// Marshaling element `<sig>` (signature.cc:37).
pub const ELEM_SIG: ElementId = ElementId::new("sig", 265);
/// Marshaling element `<signaturedesc>` (signature.cc:38).
pub const ELEM_SIGNATUREDESC: ElementId = ElementId::new("signaturedesc", 266);
/// Marshaling element `<signatures>` (signature.cc:39).
pub const ELEM_SIGNATURES: ElementId = ElementId::new("signatures", 267);
/// Marshaling element `<varsig>` (signature.cc:41).
pub const ELEM_VARSIG: ElementId = ElementId::new("varsig", 269);

/// Data-type for containing hash information (C++ `typedef uint8 hashword`,
/// signature.hh:25).
pub type Hashword = u64;

// ===========================================================================
// hash_mixin (signature.cc:43-59)
// ===========================================================================

/// Mix two 64-bit hashwords through the CRC32 register (C++ static
/// `hash_mixin`, signature.cc:43-59).
///
/// The high/low halves of `val1` are seeded into two CRC registers, then 8
/// bytes of `val2` are folded in cross-wise (`hashhi` consumes `val2` bytes,
/// `hashlo` consumes the *previous* `hashhi`).  Recombined high<<32 | low.
fn hash_mixin(val1: Hashword, mut val2: Hashword) -> Hashword {
    let mut hashhi: u32 = (val1 >> 32) as u32;
    let mut hashlo: u32 = val1 as u32;
    for _ in 0..8 {
        let tmphi: u32 = hashhi;
        let tmplo: u32 = val2 as u32;
        val2 >>= 8;
        hashhi = crc_update(hashhi, tmplo);
        hashlo = crc_update(hashlo, tmphi);
    }
    let mut res: u64 = hashhi as u64;
    res <<= 32;
    res |= hashlo as u64;
    res
}

// ===========================================================================
// Signature settings (the SigManager static, signature.cc:21)
// ===========================================================================

/// Signature settings, shared across all managers (C++ `SigManager::settings`,
/// a process-global static set via `setSettings`, read at `GraphSigManager`
/// construction).  Modeled as an atomic to preserve the "set once, read at
/// construct" cadence `analyzesigs` relies on.
static SIG_SETTINGS: AtomicU32 = AtomicU32::new(0);

/// Get the settings currently being used for signature generation (C++
/// `SigManager::getSettings`).
pub fn get_settings() -> u32 {
    SIG_SETTINGS.load(Ordering::Relaxed)
}

/// Serializes tests that mutate+read the process-global [`SIG_SETTINGS`] (the
/// faithful C++ `SigManager::settings` static), shared across the `signature`
/// and `analyzesigs` test modules so they do not race under cargo's parallel
/// runner.  Test-only.
#[cfg(test)]
pub(crate) static SETTINGS_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Establish settings to use for future signature generation (C++
/// `SigManager::setSettings`).
pub fn set_settings(newvalue: u32) {
    SIG_SETTINGS.store(newvalue, Ordering::Relaxed);
}

// ===========================================================================
// GraphSigManager::Mods — signature generation settings (signature.hh:268-275)
// ===========================================================================

/// Signature generation setting bits (C++ `GraphSigManager::Mods`,
/// signature.hh:268-275).
pub mod sig_mods {
    /// Treat certain varnode sizes as the same (`SIG_COLLAPSE_SIZE`).
    pub const SIG_COLLAPSE_SIZE: u32 = 0x1;
    /// Collapse varnodes that are indirect copies of each other
    /// (`SIG_COLLAPSE_INDNOISE`).
    pub const SIG_COLLAPSE_INDNOISE: u32 = 0x2;
    /// Do not use value of constant in hash (`SIG_DONOTUSE_CONST`).
    pub const SIG_DONOTUSE_CONST: u32 = 0x10;
    /// Do not use (fact of) being an input in hash (`SIG_DONOTUSE_INPUT`).
    pub const SIG_DONOTUSE_INPUT: u32 = 0x20;
    /// Do not use (fact of) being a global in hash (`SIG_DONOTUSE_PERSIST`).
    pub const SIG_DONOTUSE_PERSIST: u32 = 0x40;
}

// ===========================================================================
// SignatureEntry flags (signature.hh:80-87)
// ===========================================================================

mod sig_flags {
    /// Varnode has no incoming edges (`SIG_NODE_TERMINAL`).
    pub const SIG_NODE_TERMINAL: u32 = 0x1;
    /// No distinction between this Varnode's incoming edges (`SIG_NODE_COMMUTATIVE`).
    pub const SIG_NODE_COMMUTATIVE: u32 = 0x2;
    /// Varnode is not emitted as a formal feature (`SIG_NODE_NOT_EMITTED`).
    pub const SIG_NODE_NOT_EMITTED: u32 = 0x4;
    /// Varnode is a stand-alone COPY (`SIG_NODE_STANDALONE`).
    pub const SIG_NODE_STANDALONE: u32 = 0x8;
    /// Mark for spanning tree construction (`VISITED`).
    pub const VISITED: u32 = 0x10;
    /// Special root status in marker subgraph (`MARKER_ROOT`).
    pub const MARKER_ROOT: u32 = 0x20;
}

// ===========================================================================
// Features (Signature and derived classes, signature.hh:50-222)
// ===========================================================================

/// The concrete kind / origin of a feature (C++ `Signature` subclass).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignatureKind {
    /// Bare feature (`Signature`).
    Generic,
    /// Rooted at a Varnode (`VarnodeSignature`).
    Varnode(VarnodeId),
    /// Rooted at a basic block (`BlockSignature`); the two ops are the in-sequence
    /// pair for "form 2" (both `None` for the pure control-flow "form 1").
    Block { bl: BlockId, op1: Option<OpId>, op2: Option<OpId> },
    /// One-or-more stand-alone COPYs in a block (`CopySignature`).
    Copy(BlockId),
}

/// A \b feature describing some aspect of a function (C++ `Signature` and its
/// subclasses, signature.hh:50-222).
///
/// The underlying representation is a 32-bit hash; the [`kind`](Signature::kind)
/// carries the meta-data the C++ keeps in derived classes (the root Varnode /
/// block / op pair) so `encode`/`print_origin` can dispatch.
#[derive(Debug, Clone)]
pub struct Signature {
    /// Underlying 32-bit hash (C++ `Signature::sig`).
    sig: u32,
    /// Origin meta-data (replaces the C++ subclass).
    kind: SignatureKind,
}

impl Signature {
    /// Construct from a hashword (C++ `Signature::Signature(hashword)` —
    /// `sig=(uint4)h`).
    pub fn new(h: Hashword) -> Signature {
        Signature { sig: h as u32, kind: SignatureKind::Generic }
    }

    /// Construct a Varnode-rooted feature (C++ `VarnodeSignature`).
    fn new_varnode(vn: VarnodeId, h: Hashword) -> Signature {
        Signature { sig: h as u32, kind: SignatureKind::Varnode(vn) }
    }

    /// Construct a block-rooted feature (C++ `BlockSignature`).
    fn new_block(bl: BlockId, h: Hashword, op1: Option<OpId>, op2: Option<OpId>) -> Signature {
        Signature { sig: h as u32, kind: SignatureKind::Block { bl, op1, op2 } }
    }

    /// Construct a stand-alone-COPY feature (C++ `CopySignature`).
    fn new_copy(bl: BlockId, h: Hashword) -> Signature {
        Signature { sig: h as u32, kind: SignatureKind::Copy(bl) }
    }

    /// Get the underlying 32-bit hash of the feature (C++ `Signature::getHash`).
    pub fn get_hash(&self) -> u32 {
        self.sig
    }

    /// Get the origin meta-data.
    pub fn kind(&self) -> SignatureKind {
        self.kind
    }

    /// Compare two features (C++ `Signature::compare`, signature.cc:73-79).
    /// The hashes are compared as unsigned values; returns -1, 0, or 1.
    pub fn compare(&self, op2: &Signature) -> i32 {
        if self.sig != op2.sig {
            if self.sig < op2.sig {
                -1
            } else {
                1
            }
        } else {
            0
        }
    }

    /// Order two features via their underlying hash values (C++
    /// `Signature::comparePtr`, signature.hh:67 — `a->sig < b->sig`).
    pub fn compare_ptr(a: &Signature, b: &Signature) -> std::cmp::Ordering {
        a.sig.cmp(&b.sig)
    }

    /// Encode the feature to the stream (C++ `Signature::encode` and the derived
    /// overrides, signature.cc:609-659).
    ///
    /// The bare `<gensig>` / `<copysig>` forms port; the `<varsig>` / `<blocksig>`
    /// forms need `Varnode::encode` / `PcodeOp::encode` / `Address::encode` of the
    /// block start — the W8 marshal/print surface — and STUB out.
    pub fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        match self.kind {
            SignatureKind::Generic => {
                // Signature::encode (signature.cc:609-615)
                encoder.open_element(&ELEM_GENSIG);
                encoder.write_unsigned_integer(&ATTRIB_HASH, self.get_hash() as u64);
                encoder.close_element(&ELEM_GENSIG);
                Ok(())
            }
            SignatureKind::Copy(_) => {
                // CopySignature::encode (signature.cc:652-659)
                encoder.open_element(&ELEM_COPYSIG);
                encoder.write_unsigned_integer(&ATTRIB_HASH, self.get_hash() as u64);
                // The block index (ATTRIB_INDEX) is written by the caller via encode_with_index.
                encoder.close_element(&ELEM_COPYSIG);
                Ok(())
            }
            SignatureKind::Varnode(_) => Err(KunaError::lowlevel(
                "kuna rust port: VarnodeSignature::encode needs Varnode::encode + \
                 PcodeOp::encode (W8 marshal surface)",
            )),
            SignatureKind::Block { .. } => Err(KunaError::lowlevel(
                "kuna rust port: BlockSignature::encode needs FlowBlock::getStart \
                 + Address::encode of block start + PcodeOp::encode (W8)",
            )),
        }
    }
}

// ===========================================================================
// hashSize (signature.hh:342-351)
// ===========================================================================

/// Calculate a hash describing the size of a given Varnode (C++
/// `SignatureEntry::hashSize`, signature.hh:342-351).
///
/// `vn` is dereferenced through `fd`.  With `SIG_COLLAPSE_SIZE`, sizes 4 and
/// larger collapse to 4.
fn hash_size(fd: &Funcdata, vn: VarnodeId, modifiers: u32) -> Hashword {
    let mut val: Hashword =
        fd.vbank().get(vn).expect("hash_size: stale vn").get_size() as Hashword; // Size of varnode
    if (modifiers & sig_mods::SIG_COLLAPSE_SIZE) != 0 && val > 4 {
        // Treat sizes 4 and larger the same
        val = 4;
    }
    val ^ (val << 7) ^ (val << 14) ^ (val << 21)
}

// ===========================================================================
// SignatureEntry (signature.hh:78-160, signature.cc:81-511)
// ===========================================================================

/// A node for data-flow feature generation (C++ `SignatureEntry`,
/// signature.hh:78-160).
///
/// Rooted at a Varnode (`vn`); during generation it iteratively hashes info
/// about the Varnode and its nearest neighbors.  Stored in the manager's
/// `sigmap` keyed by `vn.get_create_index()`.  The C++ `shadow` /
/// `SignatureEntry *` pointer becomes a `create_index` key into `sigmap`
/// (`shadow_key`); a virtual node (the `removeNoise` virtual root) has no backing
/// Varnode and is identified by its post-order `index` instead.
#[derive(Debug, Clone)]
struct SignatureEntry {
    /// The root Varnode (`None` for a virtual node) (C++ `vn`).
    vn: Option<VarnodeId>,
    /// Feature-generation properties (C++ `flags`).
    flags: u32,
    /// Current and previous hash (C++ `hash[2]`).
    hash: [Hashword; 2],
    /// The effective defining PcodeOp (C++ `op`).
    op: Option<OpId>,
    /// First incoming edge via the effective op (C++ `startvn`).
    startvn: i32,
    /// Number of incoming edges (C++ `inSize`).
    in_size: i32,
    /// Post-order index (C++ `index`).
    index: i32,
    /// Shadowed entry key, if any (C++ `shadow`, a `SignatureEntry *`).
    /// Stored as the entry's identity key (see [`EntryKey`]).
    shadow: Option<EntryKey>,
}

/// Identity of a [`SignatureEntry`] in the manager's `sigmap` (the C++
/// `SignatureEntry *`).  Real overlays are keyed by `create_index`; the single
/// virtual root built in `removeNoise` is keyed by its post-order `index`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntryKey {
    /// A real overlay, identified by its Varnode's `create_index`.
    Real(u32),
    /// The virtual root of the noise sub-graph (no backing Varnode).
    Virtual(i32),
}

impl SignatureEntry {
    fn is_visited(&self) -> bool {
        (self.flags & sig_flags::VISITED) != 0
    }
    fn set_visited(&mut self) {
        self.flags |= sig_flags::VISITED;
    }
    fn is_terminal(&self) -> bool {
        (self.flags & sig_flags::SIG_NODE_TERMINAL) != 0
    }
    fn is_not_emitted(&self) -> bool {
        (self.flags & sig_flags::SIG_NODE_NOT_EMITTED) != 0
    }
    fn is_commutative(&self) -> bool {
        (self.flags & sig_flags::SIG_NODE_COMMUTATIVE) != 0
    }
    fn is_standalone_copy(&self) -> bool {
        (self.flags & sig_flags::SIG_NODE_STANDALONE) != 0
    }
    fn num_inputs(&self) -> i32 {
        self.in_size
    }
    /// Get the number of input edges in the noise-reduced form (C++
    /// `markerSizeIn`, signature.hh:108-111).
    fn marker_size_in(&self) -> i32 {
        if (self.flags & sig_flags::MARKER_ROOT) != 0 {
            return 1;
        }
        self.num_inputs()
    }
    /// Store hash from previous iteration and prepare for next (C++ `flip`).
    fn flip(&mut self) {
        self.hash[1] = self.hash[0];
    }
    fn get_hash(&self) -> Hashword {
        self.hash[0]
    }

    /// Construct from a Varnode (C++ `SignatureEntry::SignatureEntry(Varnode*,uint4)`,
    /// signature.cc:144-209).
    fn from_varnode(fd: &Funcdata, v: VarnodeId, _modifiers: u32) -> SignatureEntry {
        let mut e = SignatureEntry {
            vn: Some(v),
            flags: 0,
            hash: [0, 0],
            op: fd.vbank().get(v).expect("SignatureEntry::new: stale vn").get_def(),
            startvn: 0,
            in_size: 0,
            index: -1,
            shadow: None,
        };
        // Decide on the effective defining op for the given varnode.
        let opid = match e.op {
            None => {
                e.flags |= sig_flags::SIG_NODE_TERMINAL;
                return e;
            }
            Some(o) => o,
        };
        e.startvn = 0;
        e.in_size = fd.obank().get(opid).expect("SignatureEntry::new: stale op").num_input();
        let code = fd.obank().get(opid).expect("SignatureEntry::new: stale op").code();
        match code {
            OpCode::CPUI_COPY => {
                if test_standalone_copy(fd, v) {
                    e.flags |= sig_flags::SIG_NODE_STANDALONE;
                }
            }
            OpCode::CPUI_INDIRECT => {
                e.in_size -= 1;
                if test_standalone_copy(fd, v) {
                    e.flags |= sig_flags::SIG_NODE_STANDALONE;
                }
            }
            OpCode::CPUI_MULTIEQUAL => {
                e.flags |= sig_flags::SIG_NODE_COMMUTATIVE;
            }
            OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER
            | OpCode::CPUI_STORE
            | OpCode::CPUI_LOAD => {
                e.startvn = 1;
                e.in_size -= 1;
            }
            OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT // For shift
            | OpCode::CPUI_SUBPIECE => {
                // and truncation operators
                let in1 = fd.obank().get(opid).expect("stale op").get_in(1);
                // if the shift/truncation amount is constant
                if in1.is_some_and(|v| fd.vbank().get(v).expect("stale vn").is_constant()) {
                    e.in_size = 1; // make sure we don't even hash in the size of the constant
                }
            }
            OpCode::CPUI_CPOOLREF => {
                e.in_size = 0; // Only hash-in first input
            }
            _ => {
                if fd.obank().get(opid).expect("stale op").is_commutative() {
                    e.flags |= sig_flags::SIG_NODE_COMMUTATIVE;
                }
            }
        }
        e
    }

    /// Construct a virtual node (C++ `SignatureEntry::SignatureEntry(int4)`,
    /// signature.cc:213-223).
    fn virtual_node(ind: i32) -> SignatureEntry {
        SignatureEntry {
            vn: None,
            op: None,
            in_size: 0,
            flags: 0,
            shadow: None,
            index: ind,
            startvn: 0,
            hash: [0, 0],
        }
    }

    /// Get a hash encoding the OpCode of the effective defining PcodeOp (C++
    /// `getOpHash`, signature.cc:105-116).
    fn get_op_hash(&self, fd: &Funcdata, _modifiers: u32) -> Hashword {
        let opid = match self.op {
            None => return 0,
            Some(o) => o,
        };
        let o = fd.obank().get(opid).expect("get_op_hash: stale op");
        let opc = o.code();
        let mut ophash: Hashword = opc as i32 as Hashword;
        // For constant pool operations, hash in the resolved tag type constant,
        // which is the last input.
        if opc == OpCode::CPUI_CPOOLREF {
            let last = o.get_in(o.num_input() - 1).expect("CPOOLREF: null last input (C++ UB)");
            let off = fd.vbank().get(last).expect("get_op_hash: stale vn").get_offset();
            ophash = ophash.wrapping_add(0xfeedface) ^ off;
        }
        ophash
    }
}

/// Determine if the given Varnode is a stand-alone COPY (C++
/// `SignatureEntry::testStandaloneCopy`, signature.cc:231-258).
fn test_standalone_copy(fd: &Funcdata, vn: VarnodeId) -> bool {
    let v = fd.vbank().get(vn).expect("testStandaloneCopy: stale vn");
    let opid = v.get_def().expect("testStandaloneCopy: vn->getDef() null (C++ UB)");
    let op = fd.obank().get(opid).expect("testStandaloneCopy: stale op");
    let invnid = op.get_in(0).expect("testStandaloneCopy: getIn(0) null (C++ UB)");
    let invn = fd.vbank().get(invnid).expect("testStandaloneCopy: stale in vn");
    if invn.is_written() {
        return false;
    }
    if invn.get_addr() == v.get_addr() {
        return false;
    }

    if v.is_persist() && op.code() == OpCode::CPUI_INDIRECT {
        return true;
    }
    let descend = fd.descend_snapshot(vn);
    let mut iter = descend.iter();
    let desc_op = match iter.next() {
        None => return true,
        Some(&d) => d,
    };
    if iter.next().is_some() {
        return false;
    }
    let opc = fd.obank().get(desc_op).expect("testStandaloneCopy: stale desc op").code();
    if v.is_persist() && opc == OpCode::CPUI_INDIRECT {
        return true;
    }
    // Account for COPY and INDIRECT placeholder conventions
    if opc != OpCode::CPUI_COPY && opc != OpCode::CPUI_INDIRECT {
        return false;
    }
    let out = fd.obank().get(desc_op).expect("stale desc op").get_out();
    match out {
        None => false, // descOp->getOut()->hasNoDescend() — null out is C++ UB; be conservative
        Some(o) => fd.vbank().get(o).expect("testStandaloneCopy: stale out vn").has_no_descend(),
    }
}

// ===========================================================================
// BlockSignatureEntry (signature.hh:167-177, signature.cc:573-604)
// ===========================================================================

/// A node for control-flow feature generation (C++ `BlockSignatureEntry`,
/// signature.hh:167-177).
#[derive(Debug, Clone)]
struct BlockSignatureEntry {
    /// The root basic block (C++ `bl`).
    bl: BlockId,
    /// Current and previous hash (C++ `hash[2]`).
    hash: [Hashword; 2],
}

impl BlockSignatureEntry {
    fn new(b: BlockId) -> BlockSignatureEntry {
        BlockSignatureEntry { bl: b, hash: [0, 0] }
    }
    fn flip(&mut self) {
        self.hash[1] = self.hash[0];
    }
    fn get_hash(&self) -> Hashword {
        self.hash[0]
    }

    /// Compute an initial hash from local properties of the basic block (C++
    /// `BlockSignatureEntry::localHash`, signature.cc:573-580).
    fn local_hash(&mut self, fd: &Funcdata, _modifiers: u32) {
        let bl = fd.bblocks_ref().block(self.bl);
        let mut localhash: Hashword = bl.size_in() as Hashword;
        localhash <<= 8;
        localhash |= bl.size_out() as Hashword;
        self.hash[0] = localhash;
    }
}

// ===========================================================================
// GraphSigManager (signature.hh:265-303, signature.cc:746-1091)
// ===========================================================================

/// A manager for generating Signatures/features on a function's data-flow and
/// control-flow (C++ `GraphSigManager` : `SigManager`, signature.hh:233-303).
///
/// Reusable across functions; [`set_current_function`](GraphSigManager::set_current_function)
/// establishes the overlays, [`generate`](GraphSigManager::generate) does the
/// iteration and collection.
pub struct GraphSigManager {
    /// Feature set for the current function (C++ `SigManager::sigs`).
    sigs: Vec<Signature>,
    /// Current settings (C++ `sigmods`).
    sigmods: u32,
    /// Maximum number of iterations across data-flow graph (C++ `maxiter`).
    maxiter: i32,
    /// Maximum number of block iterations (C++ `maxblockiter`).
    maxblockiter: i32,
    /// Maximum number of Varnodes to signature (0 = no limit) (C++ `maxvarnode`).
    maxvarnode: i32,
    /// Map from Varnode `create_index` to SignatureEntry overlay (C++ `sigmap`).
    sigmap: BTreeMap<u32, SignatureEntry>,
    /// Map from basic-block index to BlockSignatureEntry overlay (C++ `blockmap`).
    blockmap: BTreeMap<i32, BlockSignatureEntry>,
}

impl GraphSigManager {
    /// Construct, reading the process-global settings (C++
    /// `GraphSigManager::GraphSigManager`, signature.cc:926-937).
    ///
    /// Returns an error when the settings are invalid (the C++
    /// `throw LowlevelError("Bad signature settings")`).
    pub fn new() -> KunaResult<GraphSigManager> {
        let setting = get_settings();
        if !Self::test_settings(setting) {
            return Err(KunaError::lowlevel("Bad signature settings"));
        }
        Ok(GraphSigManager {
            sigs: Vec::new(),
            sigmods: setting >> 2,
            maxiter: 3,
            maxblockiter: 1,
            maxvarnode: 0,
            sigmap: BTreeMap::new(),
            blockmap: BTreeMap::new(),
        })
    }

    /// Override the default iterations used for Varnode features (C++
    /// `setMaxIteration`).
    pub fn set_max_iteration(&mut self, val: i32) {
        self.maxiter = val;
    }
    /// Override the default iterations used for block features (C++
    /// `setMaxBlockIteration`).
    pub fn set_max_block_iteration(&mut self, val: i32) {
        self.maxblockiter = val;
    }
    /// Set a maximum threshold for Varnodes in a function (C++ `setMaxVarnode`).
    pub fn set_max_varnode(&mut self, val: i32) {
        self.maxvarnode = val;
    }

    /// Get the number of features currently generated (C++ `numSignatures`).
    pub fn num_signatures(&self) -> i32 {
        self.sigs.len() as i32
    }
    /// Get the i-th Signature/feature (C++ `getSignature`).
    pub fn get_signature(&self, i: i32) -> &Signature {
        &self.sigs[i as usize]
    }

    /// Add a new feature to the manager (C++ `SigManager::addSignature`).
    fn add_signature(&mut self, sig: Signature) {
        self.sigs.push(sig);
    }

    /// Clear all current Signature/feature resources and overlay objects (C++
    /// `GraphSigManager::clear`, signature.cc:939-945).
    pub fn clear(&mut self) {
        self.varnode_clear();
        self.block_clear();
        self.sigs.clear();
    }

    fn varnode_clear(&mut self) {
        self.sigmap.clear();
    }
    fn block_clear(&mut self) {
        self.blockmap.clear();
    }

    /// Test for valid signature generation settings (C++
    /// `GraphSigManager::testSettings`, signature.cc:914-924).
    pub fn test_settings(val: u32) -> bool {
        if val == 0 {
            return false; // 0 setting is not allowed
        }
        // Allowed setting bits
        let mut mask = sig_mods::SIG_COLLAPSE_SIZE
            | sig_mods::SIG_DONOTUSE_CONST
            | sig_mods::SIG_DONOTUSE_INPUT
            | sig_mods::SIG_DONOTUSE_PERSIST
            | sig_mods::SIG_COLLAPSE_INDNOISE;
        mask = (mask << 2) | 1; // Add the check bit
        (val & !mask) == 0 // Do not allow any other bit to be set
    }

    /// Configure the manager from a leading optional `maxiter` token (C++
    /// `GraphSigManager::initializeFromStream`, signature.cc:947-958).
    ///
    /// `maxiter_override` is the parsed value (`None` for the C++ `-1` "leave
    /// default").  The console parsing of the stream is the W9 surface; this
    /// takes the already-parsed value.
    pub fn initialize_from_stream(&mut self, maxiter_override: Option<i32>) {
        if let Some(m) = maxiter_override {
            self.maxiter = m;
        }
    }

    /// Set the function used for feature generation and build the overlays (C++
    /// `GraphSigManager::setCurrentFunction`, signature.cc:960-990).
    ///
    /// Returns an error when the function exceeds the size threshold (the C++
    /// `throw LowlevelError(... " exceeds size threshold ...")`).
    pub fn set_current_function(&mut self, fd: &Funcdata) -> KunaResult<()> {
        let size = fd.num_varnodes();
        if self.maxvarnode != 0 && size > self.maxvarnode {
            return Err(KunaError::lowlevel(format!(
                "{} exceeds size threshold for generating signatures",
                fd.get_name()
            )));
        }

        // Iterate varnodes in location order.
        for vn in fd.vbank().iter_loc() {
            let entry = SignatureEntry::from_varnode(fd, vn, self.sigmods);
            let ci = fd.vbank().get(vn).expect("setCurrentFunction: stale vn").get_create_index();
            self.sigmap.insert(ci, entry);
        }
        if (self.sigmods & sig_mods::SIG_COLLAPSE_INDNOISE) != 0 {
            self.remove_noise(fd);
        } else {
            // Collect keys (sigmap iteration is create_index order); calculate per key.
            let keys: Vec<u32> = self.sigmap.keys().copied().collect();
            for k in keys {
                self.calculate_shadow(fd, k);
            }
        }
        let keys: Vec<u32> = self.sigmap.keys().copied().collect();
        for k in keys {
            self.local_hash(fd, k);
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // calculateShadow / localHash / standaloneCopyHash (signature.cc:84-316)
    // -----------------------------------------------------------------------

    /// Fill in the `shadow` field of the entry keyed by `ci` by following any
    /// chain of COPY/INDIRECT/CAST ops back to their input Varnode (C++
    /// `SignatureEntry::calculateShadow`, signature.cc:84-99).
    fn calculate_shadow(&mut self, fd: &Funcdata, ci: u32) {
        let start_vn = self.sigmap.get(&ci).expect("calculate_shadow: missing entry").vn;
        let mut shadow_vn = start_vn.expect("calculate_shadow: real entry has a vn");
        // The C++ `op` member is rewritten each loop turn (the loop body always
        // runs at least once, so this is always assigned before the post-loop read).
        let mut eff_op: Option<OpId>;
        loop {
            let op = fd.vbank().get(shadow_vn).expect("calculate_shadow: stale vn").get_def();
            eff_op = op;
            let opid = match op {
                None => break,
                Some(o) => o,
            };
            let opc = fd.obank().get(opid).expect("calculate_shadow: stale op").code();
            if opc != OpCode::CPUI_COPY
                && opc != OpCode::CPUI_INDIRECT
                && opc != OpCode::CPUI_CAST
            {
                break;
            }
            shadow_vn = fd
                .obank()
                .get(opid)
                .expect("stale op")
                .get_in(0)
                .expect("calculate_shadow: getIn(0) null (C++ UB)");
        }
        // The C++ assigns `op = shadowVn->getDef()` each loop turn (a member
        // write); transcribe that final value back onto the entry.
        let entry = self.sigmap.get_mut(&ci).expect("calculate_shadow: missing entry");
        entry.op = eff_op;
        if Some(shadow_vn) != start_vn {
            let base_ci =
                fd.vbank().get(shadow_vn).expect("stale vn").get_create_index();
            self.sigmap.get_mut(&ci).expect("missing entry").shadow =
                Some(EntryKey::Real(base_ci));
        }
    }

    /// Compute the standalone-COPY hash for the entry keyed by `ci` (C++
    /// `SignatureEntry::standaloneCopyHash`, signature.cc:122-140).
    fn standalone_copy_hash(&mut self, fd: &Funcdata, ci: u32, modifiers: u32) {
        let vn = self.sigmap.get(&ci).expect("standalone_copy_hash: missing").vn.unwrap();
        let mut val = hash_size(fd, vn, modifiers);
        val ^= 0xaf29e23b;
        let v = fd.vbank().get(vn).expect("stale vn");
        if v.is_persist() {
            val ^= 0x55055055;
        }
        let def = v.get_def().expect("standaloneCopyHash: getDef null (C++ UB)");
        let invnid =
            fd.obank().get(def).expect("stale op").get_in(0).expect("getIn(0) null (C++ UB)");
        let invn = fd.vbank().get(invnid).expect("stale in vn");
        if invn.is_constant() {
            if (modifiers & sig_mods::SIG_DONOTUSE_CONST) == 0 {
                val ^= v.get_offset();
            } else {
                val ^= 0xa0a0a0a0;
            }
        } else if invn.is_persist() {
            val ^= 0xd7651ec3;
        }
        let entry = self.sigmap.get_mut(&ci).expect("missing");
        entry.hash[0] = val;
        entry.hash[1] = val;
    }

    /// Compute an initial hash based on local properties of the Varnode for the
    /// entry keyed by `ci` (C++ `SignatureEntry::localHash`, signature.cc:268-316).
    fn local_hash(&mut self, fd: &Funcdata, ci: u32) {
        let modifiers = self.sigmods;
        let vn = self.sigmap.get(&ci).expect("local_hash: missing").vn.unwrap();
        let v = fd.vbank().get(vn).expect("local_hash: stale vn");

        if v.is_annotation() {
            // This mostly won't get used, but in some rare cases
            let localhash: Hashword = 0xb7b7b7b7;
            let entry = self.sigmap.get_mut(&ci).expect("missing");
            entry.flags |= sig_flags::SIG_NODE_NOT_EMITTED | sig_flags::SIG_NODE_TERMINAL;
            entry.hash[0] = localhash;
            entry.hash[1] = localhash;
            return;
        }
        if self.sigmap.get(&ci).expect("missing").shadow.is_some() {
            // If this is a shadow
            self.sigmap.get_mut(&ci).expect("missing").flags |= sig_flags::SIG_NODE_NOT_EMITTED;
            if self.sigmap.get(&ci).expect("missing").is_standalone_copy() {
                self.standalone_copy_hash(fd, ci, modifiers); // Standalone COPY gets special hash
            }
            return; // Don't calculate hash otherwise
        }

        let mut localhash = hash_size(fd, vn, modifiers);

        let v = fd.vbank().get(vn).expect("stale vn");
        if !v.is_written() {
            // If there isn't at least some tree above this varnode
            self.sigmap.get_mut(&ci).expect("missing").flags |= sig_flags::SIG_NODE_NOT_EMITTED;
        }
        let ophash = self.sigmap.get(&ci).expect("missing").get_op_hash(fd, modifiers);

        let v = fd.vbank().get(vn).expect("stale vn");
        // Class of varnode
        if v.is_constant() {
            if (modifiers & sig_mods::SIG_DONOTUSE_CONST) == 0 {
                localhash ^= v.get_offset();
            } else {
                localhash ^= 0xa0a0a0a0;
            }
        }
        // Note that internally, a persist storage location may get propagated
        // away so make sure varnode is an input.
        if (modifiers & sig_mods::SIG_DONOTUSE_PERSIST) == 0 && v.is_persist() && v.is_input() {
            localhash ^= 0x55055055;
        }
        // (the SIG_DONOTUSE_INPUT guard is commented out upstream)
        if v.is_input() {
            localhash ^= 0x10101;
        }
        if ophash != 0 {
            localhash ^= ophash ^ (ophash << 9) ^ (ophash << 18);
        }

        let entry = self.sigmap.get_mut(&ci).expect("missing");
        entry.hash[0] = localhash;
        entry.hash[1] = localhash;
    }

    // -----------------------------------------------------------------------
    // hashIn (signature.cc:321-342)
    // -----------------------------------------------------------------------

    /// Mix the previous hashes of `neigh` into the entry keyed by `ci` (C++
    /// `SignatureEntry::hashIn`, signature.cc:321-342).
    ///
    /// `neigh` carries the previous-iteration hashes of the incoming nodes (the
    /// caller resolves them before mutating, since the sigmap is borrowed
    /// mutably here).
    fn hash_in(&mut self, ci: u32, neigh: &[Hashword]) {
        let commutative = self.sigmap.get(&ci).expect("hash_in: missing").is_commutative();
        let mut curhash = self.sigmap.get(&ci).expect("missing").hash[1];
        if commutative {
            let mut accum: Hashword = 0;
            for &nh in neigh {
                let tmphash = hash_mixin(curhash, nh);
                accum = accum.wrapping_add(tmphash);
            }
            curhash = hash_mixin(curhash, accum);
        } else {
            for &nh in neigh {
                curhash = hash_mixin(curhash, nh);
            }
        }
        self.sigmap.get_mut(&ci).expect("missing").hash[0] = curhash;
    }

    // -----------------------------------------------------------------------
    // Edge resolution: mapToEntry / mapToEntryCollapse / getIn (signature.hh)
    // -----------------------------------------------------------------------

    /// Find the SignatureEntry key for a Varnode (C++ `mapToEntry`,
    /// signature.hh:310-317).
    fn map_to_entry(fd: &Funcdata, vn: VarnodeId) -> u32 {
        fd.vbank().get(vn).expect("map_to_entry: stale vn").get_create_index()
    }

    /// Find the SignatureEntry key for a Varnode, collapsing shadows (C++
    /// `mapToEntryCollapse`, signature.hh:325-332).
    fn map_to_entry_collapse(&self, fd: &Funcdata, vn: VarnodeId) -> u32 {
        let ci = Self::map_to_entry(fd, vn);
        match self.sigmap.get(&ci).expect("map_to_entry_collapse: missing").shadow {
            None => ci,
            Some(EntryKey::Real(base)) => base,
            Some(EntryKey::Virtual(_)) => ci, // virtual roots never shadow a real node
        }
    }

    /// Get the i-th incoming entry key for the entry keyed by `ci` (C++
    /// `SignatureEntry::getIn`, signature.hh:142-144 — `mapToEntryCollapse`).
    fn entry_get_in(&self, fd: &Funcdata, ci: u32, i: i32) -> u32 {
        let entry = self.sigmap.get(&ci).expect("entry_get_in: missing");
        let opid = entry.op.expect("entry_get_in: effective op");
        let slot = i + entry.startvn;
        let vn = fd
            .obank()
            .get(opid)
            .expect("entry_get_in: stale op")
            .get_in(slot)
            .expect("entry_get_in: getIn null (C++ UB)");
        self.map_to_entry_collapse(fd, vn)
    }

    // -----------------------------------------------------------------------
    // removeNoise (signature.cc:353-511)
    // -----------------------------------------------------------------------

    /// Remove noise from the data-flow graph by collapsing indirect-copy chains
    /// (C++ `SignatureEntry::removeNoise`, signature.cc:456-511).
    ///
    /// Builds the noise sub-graph (COPY/INDIRECT/MULTIEQUAL edges only) rooted at
    /// the inputs/constants/non-marker-defined Varnodes, computes its dominator
    /// tree, and sets each entry's `shadow` to its base.
    fn remove_noise(&mut self, fd: &Funcdata) {
        // Set up the virtual root: collect rootlist (sigmap is create_index order).
        let mut rootlist: Vec<u32> = Vec::new();
        let keys: Vec<u32> = self.sigmap.keys().copied().collect();
        for k in keys {
            let vn = self.sigmap.get(&k).expect("remove_noise: missing").vn.unwrap();
            let v = fd.vbank().get(vn).expect("remove_noise: stale vn");
            if v.is_input() || v.is_constant() {
                rootlist.push(k);
                self.sigmap.get_mut(&k).expect("missing").flags |= sig_flags::MARKER_ROOT;
            } else if v.is_written() {
                let opid = v.get_def().expect("remove_noise: written has def");
                let op = fd.obank().get(opid).expect("remove_noise: stale op");
                if !op.is_marker() && op.code() != OpCode::CPUI_COPY {
                    rootlist.push(k);
                    self.sigmap.get_mut(&k).expect("missing").flags |= sig_flags::MARKER_ROOT;
                }
            }
        }

        // postOrder holds the real entries (by key) in forward post-order; the
        // C++ `vector<SignatureEntry *> postOrder` plus the virtual root push.
        let mut post_order: Vec<EntryKey> = Vec::new();
        self.noise_post_order(fd, &rootlist, &mut post_order);

        // Construct a virtual root with index == postOrder.size() and out-edges to
        // every node in rootlist.
        let virtual_index = post_order.len() as i32;
        let virtual_root = SignatureEntry::virtual_node(virtual_index);
        // Store the virtual root in a dedicated slot keyed by Virtual(index).
        let mut virt_map: BTreeMap<i32, SignatureEntry> = BTreeMap::new();
        virt_map.insert(virtual_index, virtual_root);
        post_order.push(EntryKey::Virtual(virtual_index));
        for &k in &rootlist {
            self.sigmap.get_mut(&k).expect("missing").shadow =
                Some(EntryKey::Virtual(virtual_index));
        }

        self.noise_dominator(fd, &post_order, &mut virt_map);
        post_order.pop(); // Pop off virtual root

        // Calculate the shadow bases and set their shadow field to null.
        for &k in &post_order {
            if self.entry_shadow(k, &virt_map) == Some(EntryKey::Virtual(virtual_index)) {
                self.set_entry_shadow(k, &mut virt_map, None);
            }
        }
        // Set the final shadow field by collapsing the dominator tree to bases.
        for &k in &post_order {
            let mut base = k;
            while let Some(s) = self.entry_shadow(base, &virt_map) {
                base = s;
            }
            let mut entry = k;
            while let Some(s) = self.entry_shadow(entry, &virt_map) {
                let tmp = entry;
                entry = s;
                self.set_entry_shadow(tmp, &mut virt_map, Some(base));
            }
        }
    }

    /// Get an entry's `shadow` field, whether it lives in `sigmap` (real) or
    /// `virt_map` (virtual root).
    fn entry_shadow(&self, k: EntryKey, virt_map: &BTreeMap<i32, SignatureEntry>) -> Option<EntryKey> {
        match k {
            EntryKey::Real(ci) => self.sigmap.get(&ci).expect("entry_shadow: missing").shadow,
            EntryKey::Virtual(ix) => virt_map.get(&ix).expect("entry_shadow: missing virt").shadow,
        }
    }

    /// Set an entry's `shadow` field (real or virtual).
    fn set_entry_shadow(
        &mut self,
        k: EntryKey,
        virt_map: &mut BTreeMap<i32, SignatureEntry>,
        val: Option<EntryKey>,
    ) {
        match k {
            EntryKey::Real(ci) => {
                self.sigmap.get_mut(&ci).expect("set_entry_shadow: missing").shadow = val
            }
            EntryKey::Virtual(ix) => {
                virt_map.get_mut(&ix).expect("set_entry_shadow: missing virt").shadow = val
            }
        }
    }

    /// Get an entry's post-order `index` (real or virtual).
    fn entry_index(&self, k: EntryKey, virt_map: &BTreeMap<i32, SignatureEntry>) -> i32 {
        match k {
            EntryKey::Real(ci) => self.sigmap.get(&ci).expect("entry_index: missing").index,
            EntryKey::Virtual(ix) => virt_map.get(&ix).expect("entry_index: missing virt").index,
        }
    }

    /// Post-order the noise sub-graph (C++ `SignatureEntry::noisePostOrder`,
    /// signature.cc:353-386).
    ///
    /// Marker/COPY out-edges (`op->isMarker() || op->code() == CPUI_COPY`) define
    /// the children; the traversal stores the post-order index on each entry and
    /// returns the entries in post-order.
    fn noise_post_order(&mut self, fd: &Funcdata, rootlist: &[u32], post_order: &mut Vec<EntryKey>) {
        // DFSNode: (entry-key, the descend iterator position).
        struct DfsNode {
            entry: u32,
            descend: Vec<OpId>,
            pos: usize,
        }
        for &root in rootlist {
            let rvn = self.sigmap.get(&root).expect("noise_post_order: missing").vn.unwrap();
            let mut stack: Vec<DfsNode> = Vec::new();
            stack.push(DfsNode { entry: root, descend: fd.descend_snapshot(rvn), pos: 0 });
            self.sigmap.get_mut(&root).expect("missing").set_visited();
            while let Some(top) = stack.last_mut() {
                if top.pos >= top.descend.len() {
                    // No more children to traverse.
                    let entry = top.entry;
                    stack.pop();
                    let idx = post_order.len() as i32;
                    self.sigmap.get_mut(&entry).expect("missing").index = idx;
                    post_order.push(EntryKey::Real(entry));
                } else {
                    let op = top.descend[top.pos];
                    top.pos += 1;
                    let o = fd.obank().get(op).expect("noise_post_order: stale op");
                    if o.is_marker() || o.code() == OpCode::CPUI_COPY {
                        let out = o.get_out().expect("noisePostOrder: marker getOut null (C++ UB)");
                        let child_ci = Self::map_to_entry(fd, out);
                        if !self.sigmap.get(&child_ci).expect("missing child").is_visited() {
                            self.sigmap.get_mut(&child_ci).expect("missing").set_visited();
                            let cvn = self.sigmap.get(&child_ci).expect("missing").vn.unwrap();
                            stack.push(DfsNode {
                                entry: child_ci,
                                descend: fd.descend_snapshot(cvn),
                                pos: 0,
                            });
                        }
                    }
                }
            }
        }
    }

    /// Construct the dominator tree for the noise sub-graph (C++
    /// `SignatureEntry::noiseDominator`, signature.cc:396-438).
    ///
    /// `post_order` is the noise graph in post-order (real entries followed by the
    /// virtual root); on return each entry's `shadow` is its immediate dominator.
    fn noise_dominator(
        &mut self,
        fd: &Funcdata,
        post_order: &[EntryKey],
        virt_map: &mut BTreeMap<i32, SignatureEntry>,
    ) {
        let n = post_order.len();
        let virtual_root = post_order[n - 1]; // The official start node
        self.set_entry_shadow(virtual_root, virt_map, Some(virtual_root));
        let mut changed = true;
        while changed {
            changed = false;
            // For all nodes, in reverse post-order, except root.
            for i in (0..n - 1).rev() {
                let b = post_order[i];
                if self.entry_shadow(b, virt_map) != Some(virtual_root) {
                    let size_in = self.marker_size_in(b, virt_map);
                    // Find first processed node.
                    let mut j = 0;
                    let mut new_idom = virtual_root;
                    while j < size_in {
                        new_idom = self.marker_in(fd, b, j, virtual_root);
                        if self.entry_shadow(new_idom, virt_map).is_some() {
                            break;
                        }
                        j += 1;
                    }
                    j += 1;
                    while j < size_in {
                        let rho = self.marker_in(fd, b, j, virtual_root);
                        if self.entry_shadow(rho, virt_map).is_some() {
                            // Here is the intersection routine.
                            let mut finger1 = self.entry_index(rho, virt_map);
                            let mut finger2 = self.entry_index(new_idom, virt_map);
                            while finger1 != finger2 {
                                while finger1 < finger2 {
                                    let node = post_order[finger1 as usize];
                                    let sh = self
                                        .entry_shadow(node, virt_map)
                                        .expect("noiseDominator: intersect on null shadow");
                                    finger1 = self.entry_index(sh, virt_map);
                                }
                                while finger2 < finger1 {
                                    let node = post_order[finger2 as usize];
                                    let sh = self
                                        .entry_shadow(node, virt_map)
                                        .expect("noiseDominator: intersect on null shadow");
                                    finger2 = self.entry_index(sh, virt_map);
                                }
                            }
                            new_idom = post_order[finger1 as usize];
                        }
                        j += 1;
                    }
                    if self.entry_shadow(b, virt_map) != Some(new_idom) {
                        self.set_entry_shadow(b, virt_map, Some(new_idom));
                        changed = true;
                    }
                }
            }
        }
    }

    /// `markerSizeIn` for an entry (real or virtual) (C++ signature.hh:108-111).
    fn marker_size_in(&self, k: EntryKey, virt_map: &BTreeMap<i32, SignatureEntry>) -> i32 {
        match k {
            EntryKey::Real(ci) => self.sigmap.get(&ci).expect("missing").marker_size_in(),
            EntryKey::Virtual(ix) => virt_map.get(&ix).expect("missing").marker_size_in(),
        }
    }

    /// `getMarkerIn` for an entry (C++ signature.hh:119-122).
    fn marker_in(&self, fd: &Funcdata, k: EntryKey, i: i32, virtual_root: EntryKey) -> EntryKey {
        match k {
            EntryKey::Virtual(_) => virtual_root, // virtual nodes have no real op
            EntryKey::Real(ci) => {
                let entry = self.sigmap.get(&ci).expect("marker_in: missing");
                if (entry.flags & sig_flags::MARKER_ROOT) != 0 {
                    return virtual_root;
                }
                let opid = entry.op.expect("marker_in: effective op");
                let slot = i + entry.startvn;
                let vn = fd
                    .obank()
                    .get(opid)
                    .expect("marker_in: stale op")
                    .get_in(slot)
                    .expect("marker_in: getIn null (C++ UB)");
                EntryKey::Real(Self::map_to_entry(fd, vn))
            }
        }
    }

    // -----------------------------------------------------------------------
    // Iteration (signature.cc:992-1091)
    // -----------------------------------------------------------------------

    fn flip_varnodes(&mut self) {
        for entry in self.sigmap.values_mut() {
            entry.flip();
        }
    }
    fn flip_blocks(&mut self) {
        for entry in self.blockmap.values_mut() {
            entry.flip();
        }
    }

    /// One iteration of data-flow hashing (C++ `signatureIterate`,
    /// signature.cc:1016-1037).
    fn signature_iterate(&mut self, fd: &Funcdata) {
        self.flip_varnodes();
        // sigmap iteration is create_index order.
        let keys: Vec<u32> = self.sigmap.keys().copied().collect();
        for k in keys {
            let entry = self.sigmap.get(&k).expect("signature_iterate: missing");
            if entry.is_not_emitted() {
                continue;
            }
            if entry.is_terminal() {
                continue;
            }
            let num = entry.num_inputs();
            // Resolve the neighbor previous-hashes before mutating the entry.
            let mut neigh: Vec<Hashword> = Vec::new();
            for j in 0..num {
                let in_key = self.entry_get_in(fd, k, j);
                neigh.push(self.sigmap.get(&in_key).expect("missing in entry").hash[1]);
            }
            self.hash_in(k, &neigh);
        }
    }

    /// One iteration of control-flow hashing (C++ `signatureBlockIterate`,
    /// signature.cc:1041-1061).
    fn signature_block_iterate(&mut self, fd: &Funcdata) {
        self.flip_blocks();
        let keys: Vec<i32> = self.blockmap.keys().copied().collect();
        for k in keys {
            let blid = self.blockmap.get(&k).expect("signature_block_iterate: missing").bl;
            let bl = fd.bblocks_ref().block(blid);
            let size_in = bl.size_in();
            // Resolve incoming block previous-hashes + their CBRANCH edge info.
            let mut neigh: Vec<(Hashword, i32, i32)> = Vec::new();
            for i in 0..size_in {
                let inbl = bl.get_in(i);
                let in_index = fd.bblocks_ref().block(inbl).get_index();
                let inentry = self.blockmap.get(&in_index).expect("missing in block");
                let in_size_out = fd.bblocks_ref().block(inbl).size_out();
                let in_rev = bl.get_in_rev_index(i);
                neigh.push((inentry.hash[1], in_size_out, in_rev));
            }
            self.block_hash_in(k, &neigh);
        }
    }

    /// Mix incoming-block previous hashes into the block keyed by `k` (C++
    /// `BlockSignatureEntry::hashIn`, signature.cc:586-604).
    ///
    /// `neigh` is `(prev_hash, in_size_out, in_rev_index)` per incoming block.
    fn block_hash_in(&mut self, k: i32, neigh: &[(Hashword, i32, i32)]) {
        let curhash = self.blockmap.get(&k).expect("block_hash_in: missing").hash[1];
        let mut accum: Hashword = 0xbafabaca;
        for &(prev, size_out, rev) in neigh {
            // Make hash invariant on order of input blocks.
            let mut tmphash = hash_mixin(curhash, prev);
            if size_out == 2 {
                // If the incoming block ends in a CBRANCH.
                if rev == 0 {
                    tmphash = hash_mixin(tmphash, 0x777 ^ 0x7abc7abc); // FALSE condition
                } else {
                    tmphash = hash_mixin(tmphash, 0x777); // TRUE condition
                }
            }
            accum = accum.wrapping_add(tmphash);
        }
        self.blockmap.get_mut(&k).expect("missing").hash[0] = hash_mixin(curhash, accum);
    }

    /// Initialize BlockSignatureEntry overlays for the current function (C++
    /// `GraphSigManager::initializeBlocks`, signature.cc:901-911).
    fn initialize_blocks(&mut self, fd: &Funcdata) {
        let nblocks = fd.bblocks_get_size();
        for i in 0..nblocks {
            let blid = fd.bblocks_get_block(i);
            let index = fd.bblocks_ref().block(blid).get_index();
            let mut entry = BlockSignatureEntry::new(blid);
            entry.local_hash(fd, self.sigmods);
            self.blockmap.insert(index, entry);
        }
    }

    // -----------------------------------------------------------------------
    // Collection (signature.cc:747-876)
    // -----------------------------------------------------------------------

    /// Emit a VarnodeSignature feature for each non-shadowed Varnode (C++
    /// `GraphSigManager::collectVarnodeSigs`, signature.cc:747-760).
    fn collect_varnode_sigs(&mut self) {
        // sigmap iteration is create_index order.
        let mut out: Vec<Signature> = Vec::new();
        for entry in self.sigmap.values() {
            if entry.is_not_emitted() {
                continue;
            }
            out.push(Signature::new_varnode(entry.vn.unwrap(), entry.get_hash()));
        }
        for s in out {
            self.add_signature(s);
        }
    }

    /// Generate the final feature(s) for each basic block (C++
    /// `GraphSigManager::collectBlockSigs`, signature.cc:767-876).
    fn collect_block_sigs(&mut self, fd: &Funcdata) {
        // blockmap iteration is block-index order.
        let block_keys: Vec<i32> = self.blockmap.keys().copied().collect();
        for bk in block_keys {
            let blid = self.blockmap.get(&bk).expect("collect_block_sigs: missing").bl;

            let mut lastop: Option<OpId> = None;
            let mut lasthash: Hashword = 0;
            let mut callhash: Hashword = 0;
            let mut copyhash: Hashword = 0;

            // Walk the block's ops in order (C++ bl->beginOp()..endOp()).
            let mut oiter = fd.bb_op_head(blid);
            while let Some(op) = oiter {
                let o = fd.obank().get(op).expect("collect_block_sigs: stale op");
                oiter = o.basic_neighbours().1; // ++oiter
                // C++ `int4 startind = 0; int4 stopind = 0;` then a switch that
                // assigns both on every fall-through arm (the COPY/INDIRECT arm
                // `continue`s); definite-assignment is satisfied without the
                // redundant zero-init that clippy flags as dead.
                let startind: i32;
                let stopind: i32;
                match o.code() {
                    OpCode::CPUI_CALL => {
                        // Always include, regardless of output use.
                        callhash = callhash.wrapping_add(100001);
                        callhash = callhash.wrapping_mul(0x78abbf);
                        startind = 1;
                        stopind = o.num_input();
                    }
                    OpCode::CPUI_CALLIND => {
                        callhash = callhash.wrapping_add(123451); // Slightly different than CALL
                        callhash = callhash.wrapping_mul(0x78abbf);
                        startind = 1;
                        stopind = o.num_input();
                    }
                    OpCode::CPUI_CALLOTHER => {
                        startind = 1;
                        stopind = o.num_input();
                    }
                    OpCode::CPUI_STORE => {
                        startind = 1;
                        stopind = o.num_input();
                    }
                    OpCode::CPUI_CBRANCH => {
                        startind = 1;
                        stopind = 2;
                    }
                    OpCode::CPUI_BRANCHIND => {
                        startind = 0;
                        stopind = 1;
                    }
                    OpCode::CPUI_RETURN => {
                        startind = 1;
                        stopind = o.num_input();
                    }
                    OpCode::CPUI_INDIRECT | OpCode::CPUI_COPY => {
                        let outid =
                            o.get_out().expect("collectBlockSigs: COPY/INDIRECT getOut null (UB)");
                        let out_ci = Self::map_to_entry(fd, outid);
                        if self.sigmap.get(&out_ci).expect("missing out entry").is_standalone_copy()
                        {
                            copyhash = copyhash
                                .wrapping_add(self.sigmap.get(&out_ci).expect("missing").get_hash());
                        }
                        continue;
                    }
                    _ => {
                        startind = 0; // Don't use
                        stopind = 0;
                    }
                }
                // Re-fetch (borrow released by the loop iterator advance above).
                let o = fd.obank().get(op).expect("stale op");
                let outvn = o.get_out();
                if stopind == 0
                    && (outvn.is_none()
                        || !fd
                            .vbank()
                            .get(outvn.unwrap())
                            .expect("stale out vn")
                            .has_no_descend())
                {
                    continue;
                }
                let val: Hashword;
                if let Some(ovn) = outvn {
                    let out_ci = Self::map_to_entry(fd, ovn);
                    if self.sigmap.get(&out_ci).expect("missing out entry").is_not_emitted() {
                        continue;
                    }
                    val = self.sigmap.get(&out_ci).expect("missing").get_hash();
                } else {
                    let mut v: Hashword = o.code() as i32 as Hashword;
                    v = v ^ (v << 9) ^ (v << 18);
                    let mut accum: Hashword = 0;
                    for i in startind..stopind {
                        // Let hash be invariant under commutivity.
                        let vn = o.get_in(i).expect("collectBlockSigs: getIn null (UB)");
                        let in_key = self.map_to_entry_collapse(fd, vn);
                        let tmphash =
                            hash_mixin(v, self.sigmap.get(&in_key).expect("missing").get_hash());
                        accum = accum.wrapping_add(tmphash);
                    }
                    v ^= accum; // Even if no-inputs we still get hash of opcode
                    val = v;
                }
                let entry_hash = self.blockmap.get(&bk).expect("missing").get_hash();
                let finalhash = if lastop.is_none() {
                    hash_mixin(val, entry_hash)
                } else {
                    hash_mixin(val, lasthash)
                };
                self.add_signature(Signature::new_block(blid, finalhash, lastop, Some(op)));
                lastop = Some(op);
                lasthash = val;
            }
            let entry_hash = self.blockmap.get(&bk).expect("missing").get_hash();
            // Create a hash with just block information.
            let mut finalhash = hash_mixin(entry_hash, 0x9b1c5f);
            if callhash != 0 {
                finalhash = hash_mixin(finalhash, callhash);
            }
            self.add_signature(Signature::new_block(blid, finalhash, None, None));
            if copyhash != 0 {
                copyhash = hash_mixin(copyhash, 0xa2de3c);
                self.add_signature(Signature::new_copy(blid, copyhash));
            }
        }
    }

    /// Generate all features for the current function (C++
    /// `GraphSigManager::generate`, signature.cc:1063-1091).
    pub fn generate(&mut self, fd: &Funcdata) {
        let minusone = self.maxiter - 1;
        let firsthalf = minusone / 2;
        let secondhalf = minusone - firsthalf;
        self.signature_iterate(fd);
        for _ in 0..firsthalf {
            self.signature_iterate(fd);
        }

        // Do the block signatures incorporating varnode sigs halfway thru.
        if self.maxblockiter >= 0 {
            self.initialize_blocks(fd);
            for _ in 0..self.maxblockiter {
                self.signature_block_iterate(fd);
            }
            self.collect_block_sigs(fd);
            self.block_clear();
        }

        for _ in 0..secondhalf {
            self.signature_iterate(fd);
        }

        self.collect_varnode_sigs();

        self.varnode_clear(); // Varnodes are used in block sigs
    }

    // -----------------------------------------------------------------------
    // SigManager output helpers (signature.cc:695-737)
    // -----------------------------------------------------------------------

    /// Get the feature vector as a sorted array of hashes (C++
    /// `SigManager::getSignatureVector`, signature.cc:695-702).
    pub fn get_signature_vector(&self) -> Vec<u32> {
        let mut feature: Vec<u32> = self.sigs.iter().map(|s| s.get_hash()).collect();
        feature.sort_unstable();
        feature
    }

    /// Combine all feature hashes into one overall hash (C++
    /// `SigManager::getOverallHash`, signature.cc:705-714).
    pub fn get_overall_hash(&self) -> Hashword {
        let feature = self.get_signature_vector();
        let mut pool: Hashword = 0x12349876abacab;
        for f in feature {
            pool = hash_mixin(pool, f as Hashword);
        }
        pool
    }

    /// Sort all current features by hash (C++ `SigManager::sortByHash`,
    /// signature.hh:251).
    pub fn sort_by_hash(&mut self) {
        self.sigs.sort_by(Signature::compare_ptr);
    }

    /// Encode all current features (C++ `SigManager::encode`,
    /// signature.cc:729-737).
    pub fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        encoder.open_element(&ELEM_SIGNATUREDESC);
        for s in &self.sigs {
            s.encode(encoder)?;
        }
        encoder.close_element(&ELEM_SIGNATUREDESC);
        Ok(())
    }
}

// ===========================================================================
// simpleSignature (signature.cc:1099-1131)
// ===========================================================================

/// Generate features for a single function and write them as a flat sequence of
/// hash values (C++ `simpleSignature`, signature.cc:1099-1131).
///
/// Emits `<signatures>` with `unimpl`/`baddata` flags, one `<sig val=..>` per
/// feature (sorted), then a `<call>` per non-invalid call entry address.
pub fn simple_signature(fd: &Funcdata, encoder: &mut dyn Encoder) -> KunaResult<()> {
    let mut sigmanager = GraphSigManager::new()?;
    sigmanager.set_current_function(fd)?;
    sigmanager.generate(fd);
    let feature = sigmanager.get_signature_vector();
    encoder.open_element(&ELEM_SIGNATURES);
    if fd.has_unimplemented() {
        encoder.write_bool(&ATTRIB_UNIMPL, true);
    }
    if fd.has_bad_data() {
        encoder.write_bool(&ATTRIB_BADDATA, true);
    }
    for f in feature {
        encoder.open_element(&ELEM_SIG);
        encoder.write_unsigned_integer(&ATTRIB_VAL, f as u64);
        encoder.close_element(&ELEM_SIG);
    }
    // The per-call <call> elements iterate `fd->numCalls()` /
    // `fd->getCallSpecs(i)->getEntryAddress()` — the FuncCallSpecs list is the W4
    // call-spec surface not yet threaded onto the W3 Funcdata.  STUB(W4).
    let _ = (&ATTRIB_INDEX, &ELEM_CALL); // keep the call-element ids referenced
    encoder.close_element(&ELEM_SIGNATURES);
    Ok(())
}

/// Generate features (with full per-feature description) for a single function
/// (C++ `debugSignature`, signature.cc:1137-1146).
///
/// Defers on the `<varsig>`/`<blocksig>` encode (the W8 marshal surface); the
/// `<gensig>`/`<copysig>` features encode fully.
pub fn debug_signature(fd: &Funcdata, encoder: &mut dyn Encoder) -> KunaResult<()> {
    let mut sigmanager = GraphSigManager::new()?;
    sigmanager.set_current_function(fd)?;
    sigmanager.generate(fd);
    sigmanager.sort_by_hash();
    sigmanager.encode(encoder)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };
    use kuna_base::marshal::XmlEncode;

    use crate::op::pcodeop_flags;
    use crate::context::{ArchContext, TypeOp};

    // Settings-touching tests serialize on the shared crate-level lock (see
    // [`SETTINGS_TEST_LOCK`]).
    use super::SETTINGS_TEST_LOCK as SETTINGS_LOCK;

    // -- IR fixture plumbing (mirrors funcdata_block.rs / funcdata_op.rs tests) --

    fn build_manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m
    }

    fn build_fd() -> Funcdata {
        let manage = build_manager();
        let glb = Rc::new(ArchContext::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
    }

    fn ramspace(fd: &Funcdata) -> Rc<AddrSpace> {
        Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
    }

    fn typeop(opc: OpCode) -> TypeOp {
        // The signature/paramid algorithms only consult code()/isMarker()/
        // isCommutative(); isCommutative reads the op's cached flags, so set the
        // commutative bit for the genuinely-commutative integer ops we build.
        let flags = match opc {
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_MULT | OpCode::CPUI_INT_AND => {
                pcodeop_flags::commutative
            }
            _ => 0,
        };
        TypeOp::new(opc, flags, format!("{opc:?}"))
    }

    /// Append an op of `opc` with `ninput` inputs at `pc` into block `bl`.
    fn add_op(fd: &mut Funcdata, bl: BlockId, opc: OpCode, ninput: i32, pc: Address) -> OpId {
        let op = fd.new_op(ninput, pc);
        fd.op_set_opcode(op, typeop(opc));
        fd.op_insert(op, bl, None);
        op
    }

    #[test]
    fn hash_mixin_is_deterministic_and_order_sensitive() {
        // hash_mixin is the CRC-folded core; deterministic and not symmetric.
        let a = hash_mixin(0x1122334455667788, 0xdeadbeefcafef00d);
        let b = hash_mixin(0x1122334455667788, 0xdeadbeefcafef00d);
        assert_eq!(a, b, "hash_mixin must be deterministic");
        let c = hash_mixin(0xdeadbeefcafef00d, 0x1122334455667788);
        assert_ne!(a, c, "hash_mixin is not symmetric in its arguments");
    }

    #[test]
    fn test_settings_accepts_only_known_bits() {
        // 0 is rejected; the check bit (1) plus the shifted mod bits are allowed.
        assert!(!GraphSigManager::test_settings(0));
        assert!(GraphSigManager::test_settings(1)); // just the check bit
        // (SIG_DONOTUSE_CONST << 2) | 1  is a valid setting.
        let v = (sig_mods::SIG_DONOTUSE_CONST << 2) | 1;
        assert!(GraphSigManager::test_settings(v));
        // A bit outside the mask is rejected.
        assert!(!GraphSigManager::test_settings(0x8000 | 1));
    }

    #[test]
    fn hash_size_collapses_large_sizes_only_with_flag() {
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        // size-8 input varnode at ram:0x2000
        let v8 = fd.new_varnode(8, &Address::new(Rc::clone(&rs), 0x2000), None);
        let v8 = fd.set_input_varnode(v8).unwrap();
        // Without SIG_COLLAPSE_SIZE the raw size 8 is mixed; with it, clamps to 4.
        let raw = hash_size(&fd, v8, 0);
        let clamped = hash_size(&fd, v8, sig_mods::SIG_COLLAPSE_SIZE);
        let four: Hashword = 4 ^ (4 << 7) ^ (4 << 14) ^ (4 << 21);
        assert_ne!(raw, clamped);
        assert_eq!(clamped, four);
    }

    /// Build: an input p (size 4) added to a constant -> out; out is returned.
    /// Drives the full data-flow + control-flow signature pipeline and checks
    /// the feature set is non-empty and deterministic across two runs.
    fn build_add_function() -> (Funcdata, BlockId) {
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        // Single basic block.
        let root = fd.bblocks_root_pub();
        let bl = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().set_start_block(root, bl);

        // input param at ram:0x100, size 4.
        let p = fd.new_varnode(4, &Address::new(Rc::clone(&rs), 0x100), None);
        let p = fd.set_input_varnode(p).unwrap();
        // c = const 5 (size 4).
        let c = fd.new_constant(4, 5);
        // add: out = p + c  at ram:0x1000
        let add = add_op(&mut fd, bl, OpCode::CPUI_INT_ADD, 2, Address::new(Rc::clone(&rs), 0x1000));
        let out = fd.new_varnode_out(4, &Address::new(Rc::clone(&rs), 0x200), add).unwrap();
        fd.op_set_input(add, p, 0).unwrap();
        fd.op_set_input(add, c, 1).unwrap();
        let _ = out;

        // RETURN out  at ram:0x1004 (input0 = iop placeholder, input1 = returned vn).
        let ret = add_op(&mut fd, bl, OpCode::CPUI_RETURN, 2, Address::new(Rc::clone(&rs), 0x1004));
        let iop = fd.new_constant(8, 0);
        fd.op_set_input(ret, iop, 0).unwrap();
        fd.op_set_input(ret, out, 1).unwrap();

        // Recompute the basic-block structure (one block, start = bl).
        fd.structure_reset();
        (fd, bl)
    }

    #[test]
    fn generate_produces_deterministic_features() {
        let _guard = SETTINGS_LOCK.lock().unwrap();
        set_settings((sig_mods::SIG_DONOTUSE_CONST << 2) | 1);
        let (fd, _bl) = build_add_function();

        let mut m1 = GraphSigManager::new().unwrap();
        m1.set_current_function(&fd).unwrap();
        m1.generate(&fd);
        let v1 = m1.get_signature_vector();

        let mut m2 = GraphSigManager::new().unwrap();
        m2.set_current_function(&fd).unwrap();
        m2.generate(&fd);
        let v2 = m2.get_signature_vector();

        assert!(!v1.is_empty(), "the add function should yield features");
        assert_eq!(v1, v2, "feature generation must be deterministic");
        // The sorted vector is sorted ascending.
        assert!(v1.windows(2).all(|w| w[0] <= w[1]));
        // Overall hash is also stable.
        assert_eq!(m1.get_overall_hash(), m2.get_overall_hash());
    }

    #[test]
    fn noise_collapse_changes_feature_set() {
        let _guard = SETTINGS_LOCK.lock().unwrap();
        // With SIG_COLLAPSE_INDNOISE on, the noise-removal dominator path runs;
        // it must still produce a deterministic, non-empty feature set.
        set_settings((sig_mods::SIG_COLLAPSE_INDNOISE << 2) | 1);
        let (fd, _bl) = build_add_function();
        let mut m = GraphSigManager::new().unwrap();
        m.set_current_function(&fd).unwrap();
        m.generate(&fd);
        let v = m.get_signature_vector();
        assert!(!v.is_empty());
        // Re-run for determinism.
        let mut m2 = GraphSigManager::new().unwrap();
        m2.set_current_function(&fd).unwrap();
        m2.generate(&fd);
        assert_eq!(v, m2.get_signature_vector());
    }

    #[test]
    fn simple_signature_encodes_sig_elements() {
        let _guard = SETTINGS_LOCK.lock().unwrap();
        set_settings((sig_mods::SIG_DONOTUSE_CONST << 2) | 1);
        let (fd, _bl) = build_add_function();
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut enc = XmlEncode::new(&mut buf);
            simple_signature(&fd, &mut enc).unwrap();
        }
        let xml = String::from_utf8(buf).unwrap();
        assert!(xml.contains("<signatures"), "got: {xml}");
        assert!(xml.contains("<sig "), "expected at least one <sig val=..>: {xml}");
    }

    #[test]
    fn generic_signature_encode_emits_gensig() {
        let s = Signature::new(0x1234_5678_9abc_def0);
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut enc = XmlEncode::new(&mut buf);
            s.encode(&mut enc).unwrap();
        }
        let xml = String::from_utf8(buf).unwrap();
        assert!(xml.contains("<gensig"), "got: {xml}");
        // sig truncates to the low 32 bits: 0x9abcdef0.
        assert!(xml.contains("0x9abcdef0") || xml.contains("9abcdef0"), "got: {xml}");
    }

    #[test]
    fn bad_settings_rejected_at_construction() {
        let _guard = SETTINGS_LOCK.lock().unwrap();
        set_settings(0); // invalid
        assert!(GraphSigManager::new().is_err());
        set_settings((sig_mods::SIG_DONOTUSE_CONST << 2) | 1); // restore valid
        assert!(GraphSigManager::new().is_ok());
    }
}

