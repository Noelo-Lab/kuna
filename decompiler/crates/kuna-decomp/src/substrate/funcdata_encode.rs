//! Encode a [`Funcdata`] as the `<function>`/`<ast>` document that answers
//! Ghidra's `decompileAt` request — a faithful port of `Funcdata::encode`
//! (`decompiler/cpp/funcdata.cc:734`) and the sub-encoders it drives.
//!
//! The consumer of this document is the Java `HighFunction.decode`
//! (`PcodeSyntaxTree.decode`), which rebuilds the SSA AST and keys every node by
//! integer ref (`Varnode::getCreateIndex`) / time (`PcodeOp::getTime`).  The
//! markup C emitted alongside it (`p9_emit/prettyprint.rs`, `EmitMarkup`) then
//! resolves each token back to those same ids, so the click-to-address contract
//! holds **by construction**: this encoder emits exactly what
//! `get_create_index()` / `get_time()` return, unmodified (see the id-consistency
//! note below).
//!
//! ## Scope (v1)
//!
//! This is the minimal `<function>` that satisfies `HighFunction.decode` and the
//! click-to-address contract: `<function id name size [nocode]>` + `<addr>`
//! (baseaddr) + `<ast>`.  The `<ast>` is `<varnodes>` (every non-`IPTR_IOP`
//! Varnode, full form) + per basic block a `<block index>` (cover `<rangelist>`
//! + each `<op>`) + per in-edged block a `<blockedge index>` of `<edge>`s.
//!
//! The larger secondary encoders are **deferred** (marked `// v1.1:` at their
//! skip sites in [`Funcdata::encode`]): `<prototype>` (`FuncProto::encode`,
//! optional in `HighFunction.decode`), `<localdb>` (`LocalSymbolMap`),
//! `<highlist>`/`<jumptablelist>` (Phase 4), and `<override>`.
//!
//! ## C++ origins
//!
//!   * [`Funcdata::encode`]      → `decompiler/cpp/funcdata.cc:734`
//!   * [`Funcdata::encode_tree`] → `decompiler/cpp/funcdata.cc:686` (`encodeTree`)
//!   * [`Funcdata::encode_varnode`] → `decompiler/cpp/varnode.cc:1201` (`Varnode::encode`)
//!   * [`Funcdata::encode_op`]   → `decompiler/cpp/op.cc:412` (`PcodeOp::encode`)
//!   * `<seqnum>`                → `decompiler/cpp/address.cc:60` (`SeqNum::encode`)
//!   * cover `<rangelist>`       → `decompiler/cpp/block.cc` (`BlockBasic::encodeBody`)
//!   * `<blockedge>`/`<edge>`    → `decompiler/cpp/block.cc:2489,35` (`FlowBlock::encodeEdges`/`BlockEdge::encode`)

use kuna_base::address::ELEM_ADDR;
use kuna_base::error::KunaResult;
use kuna_base::marshal::{
    AttributeId, ElementId, Encoder, ATTRIB_ID, ATTRIB_INDEX, ATTRIB_NAME, ATTRIB_REF, ATTRIB_SIZE,
    ATTRIB_VALUE, ELEM_VOID,
};
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::{ATTRIB_CODE, ELEM_SPACEID};
use kuna_sleigh::translate::ELEM_OP;

use crate::block::{ELEM_BLOCK, ELEM_BLOCKEDGE};
use crate::funcdata::Funcdata;
use crate::seams::{OpId, VarnodeId};

// ---------------------------------------------------------------------------
// New marshaling ids (DECOMPILER scope).
//
// These live in a separate id namespace from the SLEIGH-compiler attributes
// that reuse the same numbers (`kuna-sleigh/src/slaformat.rs` etc.) — exactly
// the coexistence kuna already relies on for ATTRIB_NAME (sleigh=12 vs
// decomp=14).  Encoding writes `id.get_id()` directly (no registry lookup), so
// these are never registered on any `IdRegistry`; a duplicate numeric id on the
// SLEIGH registry would panic, so if any of these are ever registered for a
// decode round-trip they must go on the decompiler/ghidra registry only.
// ---------------------------------------------------------------------------

/// Marshaling element `<iop>` (C++ `ELEM_IOP`, `op.cc`, id 113).
pub const ELEM_IOP: ElementId = ElementId::new("iop", 113);
/// Marshaling element `<ast>` (C++ `ELEM_AST`, `funcdata.cc`, id 115).
pub const ELEM_AST: ElementId = ElementId::new("ast", 115);
/// Marshaling element `<function>` (C++ `ELEM_FUNCTION`, `funcdata.cc`, id 116).
///
/// Re-exported by `p9_emit/prettyprint.rs`'s `ids` module (the markup
/// `<function>` document uses the same numeric id), replacing the placeholder
/// that anticipated this port.
pub const ELEM_FUNCTION: ElementId = ElementId::new("function", 116);
/// Marshaling element `<varnodes>` (C++ `ELEM_VARNODES`, `funcdata.cc`, id 119).
pub const ELEM_VARNODES: ElementId = ElementId::new("varnodes", 119);

/// Marshaling attribute "addrtied" (C++ `ATTRIB_ADDRTIED`, `funcdata.cc`, id 30).
pub const ATTRIB_ADDRTIED: AttributeId = AttributeId::new("addrtied", 30);
/// Marshaling attribute "grp" (C++ `ATTRIB_GRP`, `funcdata.cc`, id 31).
pub const ATTRIB_GRP: AttributeId = AttributeId::new("grp", 31);
/// Marshaling attribute "input" (C++ `ATTRIB_INPUT`, `funcdata.cc`, id 32).
pub const ATTRIB_INPUT: AttributeId = AttributeId::new("input", 32);
/// Marshaling attribute "persists" (C++ `ATTRIB_PERSISTS`, `funcdata.cc`, id 33).
pub const ATTRIB_PERSISTS: AttributeId = AttributeId::new("persists", 33);
/// Marshaling attribute "unaff" (C++ `ATTRIB_UNAFF`, `funcdata.cc`, id 34).
pub const ATTRIB_UNAFF: AttributeId = AttributeId::new("unaff", 34);
/// Marshaling attribute "volatile" (C++ `ATTRIB_VOLATILE`, `database.cc`, id 65).
pub const ATTRIB_VOLATILE: AttributeId = AttributeId::new("volatile", 65);
/// Marshaling attribute "nocode" (C++ `ATTRIB_NOCODE`, `funcdata.cc`, id 84).
pub const ATTRIB_NOCODE: AttributeId = AttributeId::new("nocode", 84);

impl Funcdata {
    /// C++ `Funcdata::encode(Encoder&,uint8 id,bool savetree)` (`funcdata.cc:734`).
    ///
    /// Emits the `<function>` document consumed by Ghidra's `HighFunction.decode`.
    /// v1 covers `<function>` + baseaddr `<addr>` + (when `savetree`) the `<ast>`;
    /// the secondary encoders are deferred (see the `// v1.1:` skip sites).
    pub fn encode(&self, enc: &mut dyn Encoder, id: u64, savetree: bool) -> KunaResult<()> {
        enc.open_element(&ELEM_FUNCTION);
        if id != 0 {
            enc.write_unsigned_integer(&ATTRIB_ID, id);
        }
        enc.write_string(&ATTRIB_NAME, self.get_name().as_bytes());
        enc.write_signed_integer(&ATTRIB_SIZE, self.get_size() as i64);
        if self.has_no_code() {
            enc.write_bool(&ATTRIB_NOCODE, true);
        }
        self.get_address().encode(enc)?;
        // v1.1: localmap->encodeRecursive(encoder,false)  (<localdb>) — deferred.
        if savetree {
            self.encode_tree(enc)?;
            // v1.1 (Phase 4): encodeHigh(encoder)  (<highlist>) — deferred.
        }
        // v1.1 (Phase 4): encodeJumpTable(encoder)  (<jumptablelist>) — deferred.
        // v1.1: funcp.encode(encoder)  (<prototype>, FuncProto::encode) — deferred.
        //       A large independent encoder (ProtoStore/ProtoParameter/
        //       Datatype::encodeRef/effect lists); `HighFunction.decode` treats
        //       `<prototype>` as optional, so it does not block v1.
        // v1.1: localoverride.encode(encoder,name)  (<override>) — deferred.
        enc.close_element(&ELEM_FUNCTION);
        Ok(())
    }

    /// C++ `Funcdata::encodeTree(Encoder&)` (`funcdata.cc:686`).
    ///
    /// `<ast>` = `<varnodes>` (every non-`IPTR_IOP` Varnode) + per basic block a
    /// `<block index>` (cover + ops) + per in-edged block a `<blockedge index>`.
    fn encode_tree(&self, enc: &mut dyn Encoder) -> KunaResult<()> {
        enc.open_element(&ELEM_AST);

        // <varnodes>: all Varnodes except the iop space (funcdata.cc:695).  The
        // iop-space annotation Varnode (an INDIRECT's second input) is referenced
        // instead from inside its <op> as <iop value=time/>, never as an <addr>.
        enc.open_element(&ELEM_VARNODES);
        for vn in self.vbank().iter_loc() {
            let v = self.vbank().get(vn).expect("encode_tree: stale loc Varnode");
            if v.get_space().get_type() == spacetype::IPTR_IOP {
                continue;
            }
            self.encode_varnode(enc, vn)?;
        }
        enc.close_element(&ELEM_VARNODES);

        let n = self.bblocks_get_size();
        // Per basic block: <block index> = cover <rangelist> + each <op>.
        for i in 0..n {
            let bl = self.bblocks_get_block(i);
            enc.open_element(&ELEM_BLOCK);
            // The block's OWN index (Java places blocks by it), not the loop
            // counter `i` — they can differ.
            enc.write_signed_integer(&ATTRIB_INDEX, self.bblocks_ref().block(bl).get_index() as i64);
            // BlockBasic::encodeBody: the cover <rangelist> (mandatory — emitted
            // even when empty).
            self.bblocks_ref().encode_body(bl, enc);
            for op in self.bb_ops(bl) {
                self.encode_op(enc, op)?;
            }
            enc.close_element(&ELEM_BLOCK);
        }
        // Per block with in-edges: <blockedge index> = each in <edge>.
        for i in 0..n {
            let bl = self.bblocks_get_block(i);
            if self.bblocks_ref().block(bl).size_in() != 0 {
                enc.open_element(&ELEM_BLOCKEDGE);
                enc.write_signed_integer(
                    &ATTRIB_INDEX,
                    self.bblocks_ref().block(bl).get_index() as i64,
                );
                self.bblocks_ref().encode_edges(bl, enc);
                enc.close_element(&ELEM_BLOCKEDGE);
            }
        }

        enc.close_element(&ELEM_AST);
        Ok(())
    }

    /// C++ `Varnode::encode(Encoder&)` (`varnode.cc:1201`).
    ///
    /// The **full** `<addr>` form under `<varnodes>` carries space, offset, size,
    /// ref, and the guarded flag attributes.  Do NOT use `Address::encode_sized`
    /// — it closes the element before ref/flags can be written.
    fn encode_varnode(&self, enc: &mut dyn Encoder, vn: VarnodeId) -> KunaResult<()> {
        let v = self.vbank().get(vn).expect("encode_varnode: stale Varnode");
        enc.open_element(&ELEM_ADDR);
        v.get_space().encode_attributes_sized(enc, v.get_offset(), v.get_size())?;
        enc.write_unsigned_integer(&ATTRIB_REF, v.get_create_index() as u64);
        let mergegroup = v.get_merge_group();
        if mergegroup != 0 {
            enc.write_signed_integer(&ATTRIB_GRP, mergegroup as i64);
        }
        // Flag order matches varnode.cc exactly.
        if v.is_persist() {
            enc.write_bool(&ATTRIB_PERSISTS, true);
        }
        if v.is_addr_tied() {
            enc.write_bool(&ATTRIB_ADDRTIED, true);
        }
        if v.is_unaffected() {
            enc.write_bool(&ATTRIB_UNAFF, true);
        }
        if v.is_input() {
            enc.write_bool(&ATTRIB_INPUT, true);
        }
        if v.is_volatile() {
            enc.write_bool(&ATTRIB_VOLATILE, true);
        }
        enc.close_element(&ELEM_ADDR);
        Ok(())
    }

    /// C++ `PcodeOp::encode(Encoder&)` (`op.cc:412`).
    ///
    /// `<op code>` + `<seqnum>` + the output (`<addr ref>` or `<void>`) + each
    /// input by the four-way rule (op.cc:426-462).
    fn encode_op(&self, enc: &mut dyn Encoder, op: OpId) -> KunaResult<()> {
        let o = self.obank().get(op).expect("encode_op: stale op");
        let code = o.code();
        enc.open_element(&ELEM_OP);
        enc.write_signed_integer(&ATTRIB_CODE, code as i64);
        o.get_seq_num().encode(enc)?;

        // Output: <addr ref/> (bare back-reference) or <void/>.
        match o.get_out() {
            Some(out) => {
                let create =
                    self.vbank().get(out).expect("encode_op: stale output").get_create_index();
                self.encode_bare_ref(enc, create);
            }
            None => self.encode_void(enc),
        }

        // Inputs — the four-way rule.
        let ninput = o.num_input();
        for i in 0..ninput {
            match o.get_in(i) {
                None => self.encode_void(enc),
                Some(invn) => {
                    let iv = self.vbank().get(invn).expect("encode_op: stale input");
                    let sptype = iv.get_space().get_type();
                    if sptype == spacetype::IPTR_IOP {
                        // INDIRECT's iop input: <iop value=indop.getTime()/>.  The
                        // iop offset encodes the guarded op (op_iop_encode); resolve
                        // it back and emit its time.  Any other iop input → <void/>.
                        if i == 1 && code == OpCode::CPUI_INDIRECT {
                            let indop = crate::funcdata_varnode::op_iop_decode(iv.get_offset());
                            let time = self
                                .obank()
                                .get(indop)
                                .expect("encode_op: stale INDIRECT iop target")
                                .get_time();
                            enc.open_element(&ELEM_IOP);
                            enc.write_unsigned_integer(&ATTRIB_VALUE, time as u64);
                            enc.close_element(&ELEM_IOP);
                        } else {
                            self.encode_void(enc);
                        }
                    } else if sptype == spacetype::IPTR_CONSTANT
                        && i == 0
                        && (code == OpCode::CPUI_STORE || code == OpCode::CPUI_LOAD)
                    {
                        // LOAD/STORE space operand: <spaceid name=space/>.  The
                        // constant's offset is the manager index of the space it
                        // encodes (C++ getSpaceFromConst).
                        let idx = iv.get_offset() as i32;
                        let spc = self
                            .get_arch()
                            .manage()
                            .get_space(idx)
                            .expect("encode_op: LOAD/STORE space operand out of range");
                        enc.open_element(&ELEM_SPACEID);
                        enc.write_space(&ATTRIB_NAME, spc);
                        enc.close_element(&ELEM_SPACEID);
                    } else {
                        // Ordinary input (including a non-space constant): a bare
                        // <addr ref/> back-reference.
                        self.encode_bare_ref(enc, iv.get_create_index());
                    }
                }
            }
        }

        enc.close_element(&ELEM_OP);
        Ok(())
    }

    /// A bare back-reference `<addr ref=createIndex/>` (op.cc output/input form) —
    /// only `ATTRIB_REF`, no space/offset.  `Varnode.decode` short-circuits on a
    /// known ref via `factory.getRef(ref)`, so the full space form appears only
    /// under `<varnodes>`.
    fn encode_bare_ref(&self, enc: &mut dyn Encoder, create_index: u32) {
        enc.open_element(&ELEM_ADDR);
        enc.write_unsigned_integer(&ATTRIB_REF, create_index as u64);
        enc.close_element(&ELEM_ADDR);
    }

    /// An empty `<void/>` element (a null / non-referable op slot).
    fn encode_void(&self, enc: &mut dyn Encoder) {
        enc.open_element(&ELEM_VOID);
        enc.close_element(&ELEM_VOID);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::rc::Rc;

    use kuna_base::address::{Address, ATTRIB_UNIQ, ELEM_RANGELIST, ELEM_SEQNUM};
    use kuna_base::marshal::{Decoder, PackedDecode, PackedEncode};
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace,
        IopSpace, UniqueSpace,
    };
    use kuna_base::types::int4;
    use kuna_num::opcodes::OpCode;

    use crate::seams::{Architecture, BlockId, OpId, TypeOp};

    use super::*;

    /// A manager with constant / unique / iop / fspec / ram spaces (the minimal
    /// set `new_varnode_iop` + LOAD-spaceid need).
    fn build_manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
        m.insert_space(Rc::new(FspecSpace::new(3))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            4,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m
    }

    fn build_fd() -> Funcdata {
        let manage = build_manager();
        let glb = Rc::new(Architecture::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
    }

    fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
        Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
    }

    fn new_op(fd: &mut Funcdata, inputs: int4, opc: OpCode, off: u64) -> OpId {
        let r = ram(fd);
        let op = fd.new_op(inputs, Address::new(r, off));
        fd.op_set_opcode(op, TypeOp::new(opc, 0, "OP"));
        op
    }

    /// What the round-trip decode observed about the `<ast>`.
    #[derive(Default)]
    struct Observed {
        name: Vec<u8>,
        size: i64,
        varnode_refs: BTreeSet<u64>,
        op_uniqs: BTreeSet<u64>,
        saw_iop: bool,
        saw_spaceid: bool,
        saw_void: bool,
        saw_bare_ref: bool,
        block_count: usize,
        op_count: usize,
    }

    /// Structurally decode a `<function>` document with kuna's PackedDecode and
    /// collect the structure + the ref/uniq ids for the load-bearing invariant.
    fn decode_function(manage: &AddrSpaceManager, bytes: &[u8]) -> KunaResult<Observed> {
        let mut obs = Observed::default();
        let mut dec = PackedDecode::new(manage);
        dec.ingest_stream(bytes)?;

        let fid = dec.open_element()?;
        assert_eq!(fid, ELEM_FUNCTION.get_id(), "root is not <function>");
        obs.name = dec.read_string_id(&ATTRIB_NAME)?;
        obs.size = dec.read_signed_integer_id(&ATTRIB_SIZE)?;

        // <function> children: <addr> (baseaddr) then <ast>.
        loop {
            let c = dec.peek_element()?;
            if c == 0 {
                break;
            }
            if c == ELEM_AST.get_id() {
                let ast = dec.open_element()?;
                decode_ast(&mut dec, &mut obs)?;
                dec.close_element(ast)?;
            } else {
                dec.skip_element()?; // baseaddr <addr>, or anything unexpected.
            }
        }
        dec.close_element(fid)?;
        Ok(obs)
    }

    fn decode_ast(dec: &mut PackedDecode, obs: &mut Observed) -> KunaResult<()> {
        loop {
            let c = dec.peek_element()?;
            if c == 0 {
                break;
            }
            if c == ELEM_VARNODES.get_id() {
                let vid = dec.open_element()?;
                loop {
                    let a = dec.peek_element()?;
                    if a == 0 {
                        break;
                    }
                    assert_eq!(a, ELEM_ADDR.get_id(), "<varnodes> child is not <addr>");
                    let aid = dec.open_element()?;
                    obs.varnode_refs.insert(dec.read_unsigned_integer_id(&ATTRIB_REF)?);
                    dec.close_element(aid)?;
                }
                dec.close_element(vid)?;
            } else if c == ELEM_BLOCK.get_id() {
                obs.block_count += 1;
                let bid = dec.open_element()?;
                decode_block(dec, obs)?;
                dec.close_element(bid)?;
            } else {
                // <blockedge> (or any later-phase element): skip whole subtree.
                dec.skip_element()?;
            }
        }
        Ok(())
    }

    fn decode_block(dec: &mut PackedDecode, obs: &mut Observed) -> KunaResult<()> {
        // ATTRIB_INDEX is present but not asserted here.
        loop {
            let c = dec.peek_element()?;
            if c == 0 {
                break;
            }
            if c == ELEM_OP.get_id() {
                obs.op_count += 1;
                let oid = dec.open_element()?;
                // Every op carries a code + a <seqnum uniq>.
                let _code = dec.read_signed_integer_id(&ATTRIB_CODE)?;
                let mut first = true;
                loop {
                    let oc = dec.peek_element()?;
                    if oc == 0 {
                        break;
                    }
                    if first {
                        assert_eq!(oc, ELEM_SEQNUM.get_id(), "first <op> child is not <seqnum>");
                        let sid = dec.open_element()?;
                        obs.op_uniqs.insert(dec.read_unsigned_integer_id(&ATTRIB_UNIQ)?);
                        dec.close_element(sid)?;
                        first = false;
                    } else {
                        // output + inputs: classify then skip.
                        if oc == ELEM_IOP.get_id() {
                            obs.saw_iop = true;
                        } else if oc == ELEM_SPACEID.get_id() {
                            obs.saw_spaceid = true;
                        } else if oc == ELEM_VOID.get_id() {
                            obs.saw_void = true;
                        } else if oc == ELEM_ADDR.get_id() {
                            obs.saw_bare_ref = true;
                        }
                        dec.skip_element()?;
                    }
                }
                dec.close_element(oid)?;
            } else {
                // The cover <rangelist> (mandatory, may be empty).
                assert_eq!(c, ELEM_RANGELIST.get_id(), "unexpected <block> child");
                dec.skip_element()?;
            }
        }
        Ok(())
    }

    /// Build a synthetic single-block Funcdata exercising every op-input branch,
    /// encode it, round-trip through PackedDecode, and assert the structure and
    /// the load-bearing ref/uniq invariant.
    #[test]
    fn encode_roundtrip_covers_all_four_op_input_branches() {
        let mut fd = build_fd();
        let r = ram(&fd);

        // One basic block with a real cover range.
        let root = fd.bblocks_ref().root.unwrap();
        let bl: BlockId = fd.bblocks_mut().new_block_basic(root);
        fd.set_basic_block_range(
            bl,
            &Address::new(Rc::clone(&r), 0x2000),
            &Address::new(Rc::clone(&r), 0x2040),
        );

        // (1) A referenced op with NO output — exercises the void OUTPUT branch,
        //     and is the target the INDIRECT's iop input points at.
        let ref_op = new_op(&mut fd, 1, OpCode::CPUI_CALL, 0x2000);
        let uv = fd.new_unique(4, None);
        fd.op_set_input(ref_op, uv, 0).unwrap();

        // (2) A binary op with two real refs in + a ref out — normal-ref inputs
        //     and a ref output.
        let add_op = new_op(&mut fd, 2, OpCode::CPUI_INT_ADD, 0x2004);
        let a0 = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x100), None);
        let a1 = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x108), None);
        let ao = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x110), None);
        fd.op_set_input(add_op, a0, 0).unwrap();
        fd.op_set_input(add_op, a1, 1).unwrap();
        fd.op_set_output(add_op, ao).unwrap();

        // (3) An op with a null input slot — the void INPUT branch.
        let void_op = new_op(&mut fd, 2, OpCode::CPUI_INT_ADD, 0x2008);
        let v0 = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x118), None);
        let vo = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x120), None);
        fd.op_set_input(void_op, v0, 0).unwrap();
        // slot 1 left null on purpose.
        fd.op_set_output(void_op, vo).unwrap();

        // (4) A LOAD whose input(0) is a constant encoding the ram space index —
        //     the <spaceid> branch.
        let load_op = new_op(&mut fd, 2, OpCode::CPUI_LOAD, 0x200c);
        let space_const = fd.new_constant(8, 4); // 4 == ram's manager index
        let ptr = fd.new_varnode(8, &Address::new(Rc::clone(&r), 0x128), None);
        let lo = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x130), None);
        fd.op_set_input(load_op, space_const, 0).unwrap();
        fd.op_set_input(load_op, ptr, 1).unwrap();
        fd.op_set_output(load_op, lo).unwrap();

        // (5) An INDIRECT whose input(1) is an iop annotation — the <iop> branch.
        //     The iop annotation Varnode is in IPTR_IOP space and must be filtered
        //     out of <varnodes>.
        let ind_op = new_op(&mut fd, 2, OpCode::CPUI_INDIRECT, 0x2010);
        let i0 = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x138), None);
        let iop_vn = fd.new_varnode_iop(ref_op);
        let io = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x140), None);
        fd.op_set_input(ind_op, i0, 0).unwrap();
        fd.op_set_input(ind_op, iop_vn, 1).unwrap();
        fd.op_set_output(ind_op, io).unwrap();

        for op in [ref_op, add_op, void_op, load_op, ind_op] {
            fd.bb_insert_op(op, bl, None);
        }

        // The expected invariant sets: every non-IOP Varnode's create index, and
        // every op's time.
        let expected_refs: BTreeSet<u64> = fd
            .vbank()
            .iter_loc()
            .filter(|&vn| {
                fd.vbank().get(vn).unwrap().get_space().get_type() != spacetype::IPTR_IOP
            })
            .map(|vn| fd.vbank().get(vn).unwrap().get_create_index() as u64)
            .collect();
        let expected_uniqs: BTreeSet<u64> = fd
            .bb_ops(bl)
            .iter()
            .map(|&op| fd.obank().get(op).unwrap().get_time() as u64)
            .collect();

        // Encode.
        let mut bytes: Vec<u8> = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut bytes);
            fd.encode(&mut enc, 0x1234, true).expect("encode");
        }
        assert!(!bytes.is_empty(), "encode produced no bytes");

        // Round-trip decode.
        let obs = decode_function(fd.get_arch().manage(), &bytes).expect("decode");

        // Header.
        assert_eq!(obs.name, fd.get_name().as_bytes());
        assert_eq!(obs.size, fd.get_size() as i64);

        // Structure.
        assert_eq!(obs.block_count, 1, "expected exactly one <block>");
        assert_eq!(obs.op_count, 5, "expected five <op>s");

        // All four op-input branches + the void output branch fired.
        assert!(obs.saw_iop, "no <iop> input encoded");
        assert!(obs.saw_spaceid, "no <spaceid> input encoded");
        assert!(obs.saw_void, "no <void> slot encoded");
        assert!(obs.saw_bare_ref, "no bare <addr ref> encoded");

        // The load-bearing invariant: <addr ref> == get_create_index(),
        // <seqnum uniq> == get_time(), for every Varnode / op.
        assert_eq!(obs.varnode_refs, expected_refs, "<varnodes> refs != create indices");
        assert_eq!(obs.op_uniqs, expected_uniqs, "<op> uniqs != op times");

        // The iop annotation Varnode must NOT appear in <varnodes>.
        let iop_ref = fd.vbank().get(iop_vn).unwrap().get_create_index() as u64;
        assert!(
            !obs.varnode_refs.contains(&iop_ref),
            "iop annotation Varnode leaked into <varnodes>"
        );
    }
}
