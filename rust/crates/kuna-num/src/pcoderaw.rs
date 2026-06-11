//! Port of `decompiler/cpp/pcoderaw.hh` + `pcoderaw.cc` (W1, item
//! `w1-num-pcode-semantics`): raw descriptions of varnodes and p-code ops.
//!
//! Pointer-representation decisions (each noted at its use site):
//!
//! - `VarnodeData::space` (C++ `AddrSpace *`, possibly null) is
//!   `Option<Rc<AddrSpace>>`, following the kuna-base convention
//!   (`Address`/`Range`).  Pointer comparisons become `Rc::ptr_eq`.
//! - C++ `VarnodeData::getSpaceFromConst` reinterprets the `offset` field as
//!   a raw `AddrSpace *` heap pointer (`(AddrSpace *)(uintp)offset`), and
//!   `PcodeOpRaw::decode` stuffs that pointer in when it meets a
//!   `<spaceid>` input.  A heap address cannot live in a Rust `u64` and come
//!   back out as an `Rc`, so the port stores the space's **manager index**
//!   in `offset` and resolves it back through the `AddrSpaceManager`
//!   (`get_space_from_const` takes the manager the C++ pointer implied).
//!   This is *more* deterministic than the C++ pointer value — the golden
//!   harness already has to scrub the nondeterministic C++ pointers
//!   (`docs/rust-port/losses.md` LOSS-009).
//! - `PcodeOpRaw` holds its behavior as `Rc<dyn OpBehavior>` (shared with
//!   the `register_instructions` table) and its varnodes **by value**: the
//!   C++ `VarnodeData *out` / `vector<VarnodeData *> in` point into an
//!   emulator-owned cache whose entries are immutable while referenced, so
//!   plain values carry the same information without the aliasing.
//! - The `name=` (register) form of `VarnodeData::decodeFromAttributes`
//!   requires `Translate::getRegister`, which arrives with the sleigh wave;
//!   until then that path returns an error (same deferral as
//!   `kuna_base::address::Address::decode`).

use std::cmp::Ordering;
use std::rc::Rc;

use kuna_base::address::{Address, AddrSpacePtr, SeqNum};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{AttributeId, Decoder, ElementId, ATTRIB_NAME, ATTRIB_SPACE, ELEM_VOID};
use kuna_base::space::{AddrSpace, AddrSpaceManager};
use kuna_base::types::Wrap;

use crate::opbehavior::OpBehavior;
use crate::opcodes::{OpCode, OpcodeDecoder};

// Ids from translate.hh/translate.cc, defined here ahead of the sleigh-wave
// translate port (which should re-export these).  The numeric values are
// pinned by the packed wire format.

/// Marshaling attribute "code" (translate.cc: `AttributeId ATTRIB_CODE =
/// AttributeId("code",43)`)
pub const ATTRIB_CODE: AttributeId = AttributeId::new("code", 43);
/// Marshaling element \<spaceid\> (translate.cc: `ElementId ELEM_SPACEID =
/// ElementId("spaceid",30)`)
pub const ELEM_SPACEID: ElementId = ElementId::new("spaceid", 30);

/// \brief Data defining a specific memory location
///
/// Within the decompiler's model of a processor, any register,
/// memory location, or other variable can always be represented
/// as an address space, an offset within the space, and the
/// size of the sequence of bytes.  This is more commonly referred
/// to as a Varnode, but this is a bare-bones container
/// for the data that doesn't have the cached attributes and
/// the dataflow links of the Varnode within its syntax tree.
#[derive(Debug, Clone, Default)]
pub struct VarnodeData {
    /// The address space (C++ `AddrSpace *`; `None` is the null pointer)
    pub space: Option<Rc<AddrSpace>>,
    /// The offset within the space
    pub offset: u64,
    /// The number of bytes in the location
    pub size: u32,
}

/// C++ raw-pointer equality on the space fields (null == null).
fn space_ptr_eq(a: &Option<Rc<AddrSpace>>, b: &Option<Rc<AddrSpace>>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => Rc::ptr_eq(a, b),
        (None, None) => true,
        _ => false,
    }
}

impl PartialEq for VarnodeData {
    /// Compare VarnodeData for equality. The space, offset, and size
    /// must all be exactly equal
    /// \param op2 is the object being compared to
    /// \return true if \e this is equal to \e op2
    fn eq(&self, op2: &VarnodeData) -> bool {
        if !space_ptr_eq(&self.space, &op2.space) {
            return false;
        }
        if self.offset != op2.offset {
            return false;
        }
        self.size == op2.size
    }
}
impl Eq for VarnodeData {}

impl Ord for VarnodeData {
    /// An ordering for VarnodeData (C++ `operator<`).
    ///
    /// VarnodeData can be sorted in terms of the space its in
    /// (the space's \e index), the offset within the space,
    /// and finally by the size.
    fn cmp(&self, op2: &VarnodeData) -> Ordering {
        // C++ `if (space != op2.space)` is a pointer compare; distinct
        // spaces order by their index.  Dereferencing a null space here is
        // C++ UB -> panic (ADR 0004).
        if !space_ptr_eq(&self.space, &op2.space) {
            let ind1 = self
                .space
                .as_ref()
                .expect("VarnodeData::cmp: null space pointer (C++ UB)")
                .get_index();
            let ind2 = op2
                .space
                .as_ref()
                .expect("VarnodeData::cmp: null space pointer (C++ UB)")
                .get_index();
            if ind1 != ind2 {
                return ind1.cmp(&ind2);
            }
            // Distinct space objects sharing an index cannot happen within
            // one manager; fall through so the order stays total.
        }
        if self.offset != op2.offset {
            return self.offset.cmp(&op2.offset);
        }
        op2.size.cmp(&self.size) // BIG sizes come first
    }
}

impl PartialOrd for VarnodeData {
    fn partial_cmp(&self, op2: &VarnodeData) -> Option<Ordering> {
        Some(self.cmp(op2))
    }
}

impl VarnodeData {
    /// Get the location of the varnode as an address.
    ///
    /// This is a convenience function to construct a full Address from the
    /// VarnodeData's address space and offset
    /// \return the address of the varnode
    pub fn get_addr(&self) -> Address {
        match &self.space {
            Some(spc) => Address::new(Rc::clone(spc), self.offset),
            // C++ would construct an Address with a null space pointer
            None => Address::from_space_ptr(AddrSpacePtr::Null, self.offset),
        }
    }

    /// Treat \b this as a constant and recover encoded address space.
    ///
    /// C++ reinterprets `offset` as a raw `AddrSpace *`
    /// (`(AddrSpace *)(uintp)offset`); the port stores the space's manager
    /// index in `offset` instead (see module docs and `PcodeOpRaw::decode`),
    /// so recovery goes through the manager the C++ pointer implied.
    /// \return the encoded AddrSpace
    pub fn get_space_from_const(&self, manage: &AddrSpaceManager) -> Option<Rc<AddrSpace>> {
        manage.get_space(self.offset as i32).cloned() // cast: stored manager index (see decode)
    }

    /// Recover this object from a stream.
    ///
    /// Build this VarnodeData from an \<addr\>, \<register\>, or \<varnode\>
    /// element.
    /// \param decoder is the stream decoder
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element()?;
        self.decode_from_attributes(decoder)?;
        decoder.close_element(elem_id)
    }

    /// Recover \b this object from attributes of the current open element.
    ///
    /// Collect attributes for the VarnodeData possibly from amidst other
    /// attributes
    /// \param decoder is the stream decoder
    pub fn decode_from_attributes(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        self.space = None;
        self.size = 0;
        loop {
            let attrib_id = decoder.get_next_attribute_id()?;
            if attrib_id == 0 {
                break; // Its possible to have no attributes in an <addr/> tag
            }
            if attrib_id == ATTRIB_SPACE.get_id() {
                let spc = decoder.read_space()?;
                decoder.rewind_attributes();
                self.offset = spc.decode_attributes(decoder, &mut self.size)?;
                self.space = Some(spc);
                break;
            } else if attrib_id == ATTRIB_NAME.get_id() {
                // C++: trans->getRegister(decoder.readString()) through the
                // default code space's Translate.  Translate arrives with
                // the sleigh wave (same deferral as Address::decode).
                return Err(KunaError::lowlevel(
                    "kuna rust port: register-name varnode decode requires Translate (sleigh wave)",
                ));
            }
        }
        Ok(())
    }

    /// Does \b this contain another given VarnodeData.
    ///
    /// Return \b true, if \b this, as an address range, contains the other
    /// address range
    /// \param op2 is the other VarnodeData to test for containment
    /// \return \b true if \b this contains the other
    pub fn contains(&self, op2: &VarnodeData) -> bool {
        if !space_ptr_eq(&self.space, &op2.space) {
            return false;
        }
        if op2.offset < self.offset {
            return false;
        }
        // C++ `offset + (size-1)`: size-1 wraps in uint4 (size 0 ->
        // 0xffffffff), zero-extends to uintb, and the add wraps mod 2^64
        if self.offset.wadd(u64::from(self.size.wsub(1)))
            < op2.offset.wadd(u64::from(op2.size.wsub(1)))
        {
            return false;
        }
        true
    }

    /// Is \b this contiguous (as the most significant piece) with the given
    /// VarnodeData.
    ///
    /// If \b this and \b lo form a contiguous range of bytes, where \b this
    /// makes up the most significant bytes and \b lo makes up the least
    /// significant bytes, return \b true.
    /// \param lo is the given VarnodeData to compare with
    /// \return \b true if the two byte ranges are contiguous and in order
    pub fn is_contiguous(&self, lo: &VarnodeData) -> bool {
        if !space_ptr_eq(&self.space, &lo.space) {
            return false;
        }
        // C++ dereferences the space pointer below: null is UB -> panic
        let space =
            self.space.as_ref().expect("VarnodeData::is_contiguous: null space pointer (C++ UB)");
        if space.is_big_endian() {
            let nextoff = space.wrap_offset(self.offset.wadd(u64::from(self.size)));
            if nextoff == lo.offset {
                return true;
            }
        } else {
            let nextoff = space.wrap_offset(lo.offset.wadd(u64::from(lo.size)));
            if nextoff == self.offset {
                return true;
            }
        }
        false
    }
}

/// \brief A low-level representation of a single pcode operation
///
/// This is just the minimum amount of data to represent a pcode operation
/// An opcode, sequence number, optional output varnode
/// and input varnodes
#[derive(Clone, Default)]
pub struct PcodeOpRaw {
    /// The opcode for this operation (C++ `OpBehavior *behave`, shared with
    /// the behavior table; `None` until set_behavior, like the C++
    /// uninitialized pointer)
    behave: Option<Rc<dyn OpBehavior>>,
    /// Identifying address and index of this operation
    seq: SeqNum,
    /// Output varnode triple (C++ `VarnodeData *out`, held by value here)
    out: Option<VarnodeData>,
    /// Raw varnode inputs to this op (C++ `vector<VarnodeData *>`)
    in_: Vec<VarnodeData>,
}

impl PcodeOpRaw {
    /// Set the opcode for this op.
    ///
    /// The core behavior for this operation is controlled by an OpBehavior
    /// object which knows how output is determined given inputs. This
    /// routine sets that object
    /// \param be is the behavior object
    pub fn set_behavior(&mut self, be: Rc<dyn OpBehavior>) {
        self.behave = Some(be);
    }

    /// Retrieve the behavior for this op.
    ///
    /// Get the underlying behavior object for this pcode operation.  From
    /// this object you can determine how the object evaluates inputs to get
    /// the output
    /// \return the behavior object (`None` if never set; the C++ pointer
    ///         would be uninitialized)
    pub fn get_behavior(&self) -> Option<&Rc<dyn OpBehavior>> {
        self.behave.as_ref()
    }

    /// Get the opcode for this op.
    ///
    /// The possible types of pcode operations are enumerated by OpCode
    /// This routine retrieves the enumeration value for this particular op
    /// \return the opcode value
    pub fn get_opcode(&self) -> OpCode {
        // C++ dereferences behave unconditionally; unset is UB -> panic
        self.behave
            .as_ref()
            .expect("PcodeOpRaw::get_opcode: behavior not set (C++ UB)")
            .get_opcode()
    }

    /// Set the sequence number.
    ///
    /// Every pcode operation has a \b sequence \b number
    /// which associates the operation with the address of the machine
    /// instruction being translated and an order number which provides an
    /// index for this particular operation within the entire translation of
    /// the machine instruction
    /// \param a is the instruction address
    /// \param b is the order number
    pub fn set_seq_num(&mut self, a: &Address, b: u32) {
        self.seq = SeqNum::new(a.clone(), b);
    }

    /// Retrieve the sequence number.
    ///
    /// Every pcode operation has a \b sequence \b number which associates
    /// the operation with the address of the machine instruction being
    /// translated and an index number for this operation within the
    /// translation.
    /// \return a reference to the sequence number
    pub fn get_seq_num(&self) -> &SeqNum {
        &self.seq
    }

    /// Get address of this operation.
    ///
    /// This is a convenience function to get the address of the machine
    /// instruction (of which this pcode op is a translation)
    /// \return the machine instruction address
    pub fn get_addr(&self) -> &Address {
        self.seq.get_addr()
    }

    /// Set the output varnode for this op.
    ///
    /// Most pcode operations output to a varnode.  This routine sets what
    /// that varnode is.  (C++ takes a `VarnodeData *`, possibly null —
    /// hence the `Option`.)
    /// \param o is the varnode to set as output
    pub fn set_output(&mut self, o: Option<VarnodeData>) {
        self.out = o;
    }

    /// Retrieve the output varnode for this op.
    ///
    /// Most pcode operations have an output varnode. This routine retrieves
    /// that varnode.
    /// \return the output varnode or \b None if there is no output
    pub fn get_output(&self) -> Option<&VarnodeData> {
        self.out.as_ref()
    }

    /// Add an additional input varnode to this op.
    ///
    /// A PcodeOpRaw is initially created with no input varnodes.  Inputs are
    /// added with this method.  Varnodes are added in order, so the first
    /// add_input call creates input 0, for example.
    /// \param i is the varnode to be added as input
    pub fn add_input(&mut self, i: VarnodeData) {
        self.in_.push(i);
    }

    /// Remove all input varnodes to this op.
    ///
    /// If the inputs to a pcode operation need to be changed, this routine
    /// clears the existing inputs so new ones can be added.
    pub fn clear_inputs(&mut self) {
        self.in_.clear();
    }

    /// Get the number of input varnodes to this op.
    /// \return the number of inputs
    pub fn num_input(&self) -> i32 {
        self.in_.len() as i32 // cast: C++ returns int4 from vector::size()
    }

    /// Get the i-th input varnode for this op.
    ///
    /// Input varnodes are indexed starting at 0.  This retrieves the input
    /// varnode by index.  The index \e must be in range, or unpredicatable
    /// behavior will result (a panic here, where C++ reads out of bounds).
    /// Use the num_input method to get the number of inputs.
    /// \param i is the index of the desired input
    /// \return the desired input varnode
    pub fn get_input(&self, i: i32) -> &VarnodeData {
        &self.in_[i as usize] // cast: int4 index, C++ UB when out of range
    }

    /// \brief Decode the raw OpCode and input/output Varnode data for a PcodeOp
    ///
    /// This assumes the \<op\> element is already open.
    /// Decode info suitable for call to PcodeEmit::dump.  The output is set
    /// to `None` if there is no output for this op, otherwise the storage is
    /// filled in (C++ nulls/writes through the `outvar` handle).
    /// \param decoder is the stream decoder
    /// \param isize is the (preparsed) number of input parameters for the
    ///        p-code op
    /// \param invar is an array of storage for the input Varnodes
    /// \param outvar is the (handle to the) storage for the output Varnode
    /// \return the p-code op OpCode
    pub fn decode(
        decoder: &mut dyn OpcodeDecoder,
        isize: i32,
        invar: &mut [VarnodeData],
        outvar: &mut Option<VarnodeData>,
    ) -> KunaResult<OpCode> {
        let opcode = decoder.read_opcode_id(&ATTRIB_CODE)?;
        let mut sub_id = decoder.peek_element()?;
        if sub_id == ELEM_VOID.get_id() {
            decoder.open_element()?;
            decoder.close_element(sub_id)?;
            *outvar = None;
        } else {
            // C++ writes through the caller's existing storage pointer
            let mut vn = outvar.take().unwrap_or_default();
            vn.decode(decoder)?;
            *outvar = Some(vn);
        }
        let mut i: i32 = 0;
        while i < isize {
            sub_id = decoder.peek_element()?;
            if sub_id == ELEM_SPACEID.get_id() {
                decoder.open_element()?;
                // C++: getConstantSpace() (assumed present; a manager
                // without one would make C++ store a null pointer here)
                invar[i as usize].space = // cast: int4 loop index, i < isize
                    decoder.get_addr_space_manager().get_constant_space().cloned();
                // C++ stuffs the raw AddrSpace* into offset
                // ((uintb)(uintp)readSpace(ATTRIB_NAME)); the port stores
                // the space's manager index — see get_space_from_const
                let spc = decoder.read_space_id(&ATTRIB_NAME)?;
                invar[i as usize].offset = spc.get_index() as u64; // cast: non-negative space index
                invar[i as usize].size = 8; // C++ sizeof(void *) on the 64-bit oracle host
                decoder.close_element(sub_id)?;
            } else {
                invar[i as usize].decode(decoder)?; // cast: int4 loop index
            }
            i += 1;
        }
        Ok(opcode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::address::ELEM_ADDR;
    use kuna_base::marshal::{Encoder, IdRegistry, PackedDecode, PackedEncode, XmlDecode};
    use kuna_base::space::{addrspace_flags, spacetype, ConstantSpace};

    use crate::opcodes::OpcodeEncoder;

    /// Build a manager with a constant space (index 0) and a small "ram"
    /// space (index 1), mirroring the minimal C++ test setups.
    fn test_manager() -> AddrSpaceManager {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        let ram = AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            1,
            addrspace_flags::hasphysical,
            1,
            1,
        );
        manager.insert_space(Rc::new(ram)).unwrap();
        manager
    }

    fn ram(manager: &AddrSpaceManager) -> Rc<AddrSpace> {
        Rc::clone(manager.get_space_by_name("ram").unwrap())
    }

    fn vn(space: Rc<AddrSpace>, offset: u64, size: u32) -> VarnodeData {
        VarnodeData { space: Some(space), offset, size }
    }

    #[test]
    fn test_pcoderaw_varnodedata_compare() {
        let manager = test_manager();
        let r = ram(&manager);
        let constant = Rc::clone(manager.get_constant_space().unwrap());
        // Equality requires identical space/offset/size.
        assert_eq!(vn(Rc::clone(&r), 0x10, 4), vn(Rc::clone(&r), 0x10, 4));
        assert_ne!(vn(Rc::clone(&r), 0x10, 4), vn(Rc::clone(&r), 0x10, 8));
        assert_ne!(vn(Rc::clone(&r), 0x10, 4), vn(Rc::clone(&r), 0x11, 4));
        assert_ne!(vn(Rc::clone(&r), 0x10, 4), vn(Rc::clone(&constant), 0x10, 4));
        // Ordering: space index, then offset, then BIG sizes first.
        assert!(vn(Rc::clone(&constant), 0x10, 4) < vn(Rc::clone(&r), 0x0, 4));
        assert!(vn(Rc::clone(&r), 0x10, 4) < vn(Rc::clone(&r), 0x11, 4));
        assert!(vn(Rc::clone(&r), 0x10, 8) < vn(Rc::clone(&r), 0x10, 4));
    }

    #[test]
    fn test_pcoderaw_varnodedata_contains_contiguous() {
        let manager = test_manager();
        let r = ram(&manager);
        let big = vn(Rc::clone(&r), 0x100, 8);
        assert!(big.contains(&vn(Rc::clone(&r), 0x100, 8)));
        assert!(big.contains(&vn(Rc::clone(&r), 0x104, 4)));
        assert!(!big.contains(&vn(Rc::clone(&r), 0x105, 4)));
        assert!(!big.contains(&vn(Rc::clone(&r), 0xff, 2)));
        let constant = Rc::clone(manager.get_constant_space().unwrap());
        assert!(!big.contains(&vn(constant, 0x100, 4)));
        // little-endian: `this` is the most significant piece, so lo's end
        // must reach this's offset
        let hi = vn(Rc::clone(&r), 0x104, 4);
        let lo = vn(Rc::clone(&r), 0x100, 4);
        assert!(hi.is_contiguous(&lo));
        assert!(!lo.is_contiguous(&hi));
        assert!(!hi.is_contiguous(&vn(Rc::clone(&r), 0x101, 4)));
    }

    #[test]
    fn test_pcoderaw_opraw_accessors() {
        use crate::opbehavior::OpBehaviorIntAdd;
        let manager = test_manager();
        let r = ram(&manager);
        let mut op = PcodeOpRaw::default();
        assert!(op.get_behavior().is_none());
        op.set_behavior(Rc::new(OpBehaviorIntAdd));
        assert_eq!(op.get_opcode(), OpCode::CPUI_INT_ADD);
        let addr = Address::new(Rc::clone(&r), 0x1000);
        op.set_seq_num(&addr, 7);
        assert_eq!(op.get_seq_num().get_time(), 7);
        assert_eq!(op.get_addr().get_offset(), 0x1000);
        assert!(op.get_output().is_none());
        op.set_output(Some(vn(Rc::clone(&r), 0x20, 4)));
        assert_eq!(op.get_output().unwrap().offset, 0x20);
        assert_eq!(op.num_input(), 0);
        op.add_input(vn(Rc::clone(&r), 0x30, 4));
        op.add_input(vn(Rc::clone(&r), 0x34, 4));
        assert_eq!(op.num_input(), 2);
        assert_eq!(op.get_input(1).offset, 0x34);
        op.clear_inputs();
        assert_eq!(op.num_input(), 0);
    }

    /// XML path of PcodeOpRaw::decode: an INT_ADD with an output and two
    /// register inputs, then a LOAD whose first input is a <spaceid>.
    #[test]
    fn test_pcoderaw_decode_xml() {
        let manager = test_manager();
        let mut registry = IdRegistry::with_base_ids();
        // "spaceid" belongs to the (unported) translate tables; tests
        // register it explicitly, as the architecture bootstrap will.
        registry.register_element(&ELEM_SPACEID);
        let mut dec = XmlDecode::new(&manager, &registry);
        dec.ingest_stream(
            b"<op code=\"INT_ADD\"><addr space=\"ram\" offset=\"0x20\" size=\"4\"/>\
              <addr space=\"ram\" offset=\"0x30\" size=\"4\"/>\
              <addr space=\"ram\" offset=\"0x34\" size=\"4\"/></op>",
        )
        .unwrap();
        let el = dec.open_element().unwrap();
        let mut invar = vec![VarnodeData::default(), VarnodeData::default()];
        let mut outvar = Some(VarnodeData::default());
        let opc = PcodeOpRaw::decode(&mut dec, 2, &mut invar, &mut outvar).unwrap();
        dec.close_element(el).unwrap();
        assert_eq!(opc, OpCode::CPUI_INT_ADD);
        let out = outvar.unwrap();
        assert_eq!((out.offset, out.size), (0x20, 4));
        assert!(Rc::ptr_eq(out.space.as_ref().unwrap(), &ram(&manager)));
        assert_eq!((invar[0].offset, invar[0].size), (0x30, 4));
        assert_eq!((invar[1].offset, invar[1].size), (0x34, 4));

        // LOAD: void output, <spaceid> first input.
        let mut dec = XmlDecode::new(&manager, &registry);
        dec.ingest_stream(
            b"<op code=\"LOAD\"><void/><spaceid name=\"ram\"/>\
              <addr space=\"ram\" offset=\"0x40\" size=\"8\"/></op>",
        )
        .unwrap();
        let el = dec.open_element().unwrap();
        let mut invar = vec![VarnodeData::default(), VarnodeData::default()];
        let mut outvar = Some(VarnodeData::default());
        let opc = PcodeOpRaw::decode(&mut dec, 2, &mut invar, &mut outvar).unwrap();
        dec.close_element(el).unwrap();
        assert_eq!(opc, OpCode::CPUI_LOAD);
        assert!(outvar.is_none());
        // spaceid input: constant space, offset = encoded space, size 8
        assert!(Rc::ptr_eq(
            invar[0].space.as_ref().unwrap(),
            manager.get_constant_space().unwrap()
        ));
        assert_eq!(invar[0].size, 8);
        let recovered = invar[0].get_space_from_const(&manager).unwrap();
        assert!(Rc::ptr_eq(&recovered, &ram(&manager)));
        assert_eq!((invar[1].offset, invar[1].size), (0x40, 8));
    }

    /// Packed path: encode an op with the OpcodeEncoder/Encoder halves and
    /// decode it back through PcodeOpRaw::decode.
    #[test]
    fn test_pcoderaw_decode_packed_roundtrip() {
        let manager = test_manager();
        let r = ram(&manager);
        let mut stream: Vec<u8> = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut stream);
            enc.open_element(&ELEM_ADDR); // any element: decode assumes <op> is open
            enc.write_opcode(&ATTRIB_CODE, OpCode::CPUI_INT_SUB);
            // output varnode
            Address::new(Rc::clone(&r), 0x50).encode_sized(&mut enc, 4).unwrap();
            // two input varnodes
            Address::new(Rc::clone(&r), 0x60).encode_sized(&mut enc, 4).unwrap();
            Address::new(Rc::clone(&r), 0x64).encode_sized(&mut enc, 4).unwrap();
            enc.close_element(&ELEM_ADDR);
        }
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&stream).unwrap();
        let el = dec.open_element().unwrap();
        let mut invar = vec![VarnodeData::default(), VarnodeData::default()];
        let mut outvar = Some(VarnodeData::default());
        let opc = PcodeOpRaw::decode(&mut dec, 2, &mut invar, &mut outvar).unwrap();
        dec.close_element(el).unwrap();
        assert_eq!(opc, OpCode::CPUI_INT_SUB);
        let out = outvar.unwrap();
        assert_eq!((out.offset, out.size), (0x50, 4));
        assert_eq!((invar[0].offset, invar[0].size), (0x60, 4));
        assert_eq!((invar[1].offset, invar[1].size), (0x64, 4));
        assert!(Rc::ptr_eq(invar[1].space.as_ref().unwrap(), &r));
    }

    /// VarnodeData::decode rejects the register-name form for now (the
    /// Translate deferral) and bad opcodes surface as decoder errors.
    #[test]
    fn test_pcoderaw_decode_errors() {
        let manager = test_manager();
        let registry = IdRegistry::with_base_ids();
        let mut dec = XmlDecode::new(&manager, &registry);
        dec.ingest_stream(b"<addr name=\"EAX\"/>").unwrap();
        let mut v = VarnodeData::default();
        assert!(matches!(v.decode(&mut dec), Err(KunaError::Lowlevel { .. })));

        let mut dec = XmlDecode::new(&manager, &registry);
        dec.ingest_stream(b"<op code=\"NOT_AN_OP\"><void/></op>").unwrap();
        dec.open_element().unwrap();
        let mut invar: Vec<VarnodeData> = Vec::new();
        let mut outvar = Some(VarnodeData::default());
        assert!(matches!(
            PcodeOpRaw::decode(&mut dec, 0, &mut invar, &mut outvar),
            Err(KunaError::Decoder { .. })
        ));
    }
}
