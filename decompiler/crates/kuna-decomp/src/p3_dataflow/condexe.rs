//! Port of `decompiler/cpp/condexe.{cc,hh}` — control-flow simplification when
//! the same boolean condition is repeatedly branched on (W5).
//!
//! # What this is
//!
//! [`ConditionalExecution`] performs the CONDEXE block-merge analysis described
//! in `condexe.hh`: it recognizes a configuration where two CBRANCHs key on the
//! same (or complementary) condition, so the join at the second branch's block
//! (the `iblock`) is redundant.  The `verify*` family decides whether the
//! configuration is removable; [`ConditionalExecution::execute`] then rewires
//! the data-flow (the heritage-aware MULTIEQUAL/pull-back varnode replacement)
//! and collapses the block.  [`ActionConditionalExe`] wraps the analysis as an
//! [`Action`]; [`RuleOrPredicate`] simplifies the INT_OR/INT_XOR predication
//! form.
//!
//! # Faithfulness — what is fully ported vs. stub-deferred
//!
//! The **verify path** is entirely read-only and fully ported and tested:
//! [`test_iblock`](ConditionalExecution::test_iblock),
//! [`find_init_pre`](ConditionalExecution::find_init_pre),
//! [`verify_same_condition`](ConditionalExecution::verify_same_condition),
//! [`test_op_read`](ConditionalExecution::test_op_read),
//! [`test_multi_read`](ConditionalExecution::test_multi_read),
//! [`test_removability`](ConditionalExecution::test_removability),
//! [`verify`](ConditionalExecution::verify), and
//! [`trial`](ConditionalExecution::trial).  It keys on
//! [`crate::expression::BooleanExpressionMatch`].
//!
//! The **execute / mutation path** is transcribed structurally but threads the
//! two documented stubs the rule wave inherits:
//!
//!   - **`Funcdata::opSetOutput`** (LOSS-035/036): creating a new op with an
//!     output (`newOp` + `newVarnodeOut`/`newUniqueOut` → `opSetOutput`) needs the
//!     `banks_mut()` `(&mut VarnodeBank, &mut PcodeOpBank)` split-borrow accessor
//!     the `funcdata` owner must add (for `vbank.setDef` + `replace_reads`).  The
//!     port reproduces the op-shell creation and surfaces the existing
//!     `op_set_output` `Err` — so `pullback_op`, `get_new_multi`, `do_replacement`,
//!     `execute`, `apply`, and `RuleOrPredicate::apply_op`/`check_single` return
//!     `KunaResult` and the stub is visible, never silently mis-ported.
//!   - **`Funcdata::opSetOpcode`** (W6 `glb->inst[opc]`): the rule needs the
//!     resolved [`crate::context::TypeOp`] for a target opcode; the W6 op-info table
//!     is not present, so [`resolve_typeop`] surfaces a stub `Err`.  // STUB(W6)
//!   - **`Funcdata::numHeritagePasses`** (W7 heritage): `buildHeritageArray`
//!     queries per-space heritage progress; the W7 `Heritage` subsystem is not
//!     present, so it reports "no pass yet" (every space `false`).  // STUB(W7)
//!
//! These deferrals are recorded as losses; the verify-path content — the genuine
//! algorithmic novelty of this file — is complete.

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;
use std::collections::BTreeMap;

use crate::action::{
    Action, ActionBase, ActionContext, ActionGroupList, ApplyResult, Rule, RuleSpec, RuleState,
};
use crate::expression::BooleanExpressionMatch;
use crate::funcdata::Funcdata;
use crate::context::{BlockId, OpId, TypeOp, VarnodeId};

/// Resolve an [`OpCode`] to the [`TypeOp`] `opSetOpcode` needs.
///
/// The C++ `opSetOpcode(op,opc)` is `obank.changeOpcode(op,glb->inst[opc])` — it
/// looks the singleton `TypeOp` up in the architecture's op-info table.  That
/// canonical `inst[]` table is now [`crate::typeop::type_op_for`] (the W6 op-info
/// records, full cached property-flag word per opcode), so we resolve through it
/// directly — exactly the C++ `glb->inst[opc]` lookup.  Only `CPUI_MAX` (which
/// has no `inst[]` entry in C++ either, `inst[CPUI_MAX]` is the trailing null)
/// has no record; the conditional-execution rewrite never sets that opcode, so a
/// `CPUI_MAX` request is an internal invariant violation surfaced as an `Err`.
fn resolve_typeop(opc: OpCode) -> KunaResult<TypeOp> {
    if opc == OpCode::CPUI_MAX {
        return Err(KunaError::lowlevel(
            "condexe resolve_typeop: CPUI_MAX has no inst[] entry",
        ));
    }
    Ok(crate::typeop::type_op_for(opc))
}

// =============================================================================
// ConditionalExecution (condexe.hh:91-127, condexe.cc:20-475)
// =============================================================================

/// A class for simplifying a series of conditionally executed statements (C++
/// `ConditionalExecution`).  See the module docs and `condexe.hh` for the
/// `iblock`/`initblock`/`prea`/`preb`/`posta`/`postb` configuration.
pub struct ConditionalExecution {
    /// CBRANCH in iblock (C++ `PcodeOp *cbranch`).
    cbranch: Option<OpId>,
    /// The initial block computing the boolean value (C++ `BlockBasic *initblock`).
    initblock: Option<BlockId>,
    /// The block where flow is (unnecessarily) coming together (C++ `BlockBasic *iblock`).
    iblock: Option<BlockId>,
    /// `iblock->In(prea_inslot)` = pre a path (C++ `int4 prea_inslot`).
    prea_inslot: int4,
    /// Does \b true branch (in terms of iblock) go to path pre a (C++ `bool init2a_true`).
    init2a_true: bool,
    /// Does \b true branch go to path post a (C++ `bool iblock2posta_true`).
    iblock2posta_true: bool,
    /// init or pre slot to use, for data-flow thru post (C++ `int4 camethruposta_slot`).
    camethruposta_slot: int4,
    /// The \b out edge from iblock to posta (C++ `int4 posta_outslot`).
    posta_outslot: int4,
    /// First block in posta path (C++ `BlockBasic *posta_block`).
    posta_block: Option<BlockId>,
    /// First block in postb path (C++ `BlockBasic *postb_block`).
    postb_block: Option<BlockId>,
    /// Map from block to replacement Varnode for (current) Varnode
    /// (C++ `map<int4,Varnode *> replacement`, keyed by block index).
    replacement: BTreeMap<int4, VarnodeId>,
    /// Outputs of ops that have been pulled back from \b iblock for (current)
    /// Varnode (C++ `vector<Varnode *> pullback`).  Slots may be empty (`None`).
    pullback: Vec<Option<VarnodeId>>,
    /// Boolean array indexed by address space index indicating whether the space
    /// is heritaged (C++ `vector<bool> heritageyes`).
    heritageyes: Vec<bool>,
}

impl ConditionalExecution {
    /// Set up for testing ConditionalExecution on multiple iblocks (C++
    /// `ConditionalExecution::ConditionalExecution(Funcdata*)`, `condexe.cc:431`).
    pub fn new(fd: &Funcdata) -> ConditionalExecution {
        let mut ce = ConditionalExecution {
            cbranch: None,
            initblock: None,
            iblock: None,
            prea_inslot: 0,
            init2a_true: false,
            iblock2posta_true: false,
            camethruposta_slot: 0,
            posta_outslot: 0,
            posta_block: None,
            postb_block: None,
            replacement: BTreeMap::new(),
            pullback: Vec::new(),
            heritageyes: Vec::new(),
        };
        ce.build_heritage_array(fd); // Cache an array depending on the particular heritage pass
        ce
    }

    /// Calculate boolean array of all address spaces that have had a heritage
    /// pass run (C++ `ConditionalExecution::buildHeritageArray`, `condexe.cc:23-37`).
    ///
    /// Used to test if all the links out of the iblock have been calculated.
    fn build_heritage_array(&mut self, fd: &Funcdata) {
        self.heritageyes.clear();
        let glb = fd.get_arch();
        let nspaces = glb.manage().num_spaces();
        self.heritageyes.resize(nspaces as usize, false);
        for i in 0..nspaces {
            let spc = match glb.manage().get_space(i) {
                Some(s) => s,
                None => continue,
            };
            let index = spc.get_index();
            if !spc.is_heritaged() {
                continue;
            }
            // STUB(W7): numHeritagePasses queries the W7 Heritage subsystem, which
            // is not present at this item's boundary.  Treated as "no pass yet"
            // (every heritaged space stays false); the heritage-gated guard in
            // test_removability is then conservative (refuses to remove ops whose
            // output has no descendants).  Recorded as a loss.
            let num_heritage_passes = 0; // STUB(W7): fd.num_heritage_passes(spc)
            if num_heritage_passes > 0 {
                self.heritageyes[index as usize] = true;
            }
        }
    }

    /// Test the most basic requirements on \b iblock (C++
    /// `ConditionalExecution::testIBlock`, `condexe.cc:43-52`).
    ///
    /// The block must have 2 \b in edges, 2 \b out edges and a final CBRANCH op.
    fn test_iblock(&mut self, fd: &Funcdata) -> bool {
        let iblock = self.iblock.expect("testIBlock: no iblock");
        if fd.bblocks_ref().block(iblock).size_in() != 2 {
            return false;
        }
        if fd.bblocks_ref().block(iblock).size_out() != 2 {
            return false;
        }
        let cbranch = match fd.bb_op_tail(iblock) {
            Some(o) => o,
            None => return false,
        };
        self.cbranch = Some(cbranch);
        if fd.obank().get(cbranch).expect("testIBlock: stale cbranch").code() != OpCode::CPUI_CBRANCH {
            return false;
        }
        true
    }

    /// Find \b initblock, based on \b iblock (C++
    /// `ConditionalExecution::findInitPre`, `condexe.cc:55-75`).
    ///
    /// \return \b true if configuration between \b initblock and \b iblock is correct
    fn find_init_pre(&mut self, fd: &Funcdata) -> bool {
        let iblock = self.iblock.expect("findInitPre: no iblock");
        let mut tmp = fd.bblocks_ref().block(iblock).get_in(self.prea_inslot);
        let mut last = iblock;
        while fd.bblocks_ref().block(tmp).size_out() == 1 && fd.bblocks_ref().block(tmp).size_in() == 1 {
            last = tmp;
            tmp = fd.bblocks_ref().block(tmp).get_in(0);
        }
        if fd.bblocks_ref().block(tmp).size_out() != 2 {
            return false;
        }
        self.initblock = Some(tmp);
        let mut tmp2 = fd.bblocks_ref().block(iblock).get_in(1 - self.prea_inslot);
        while fd.bblocks_ref().block(tmp2).size_out() == 1 && fd.bblocks_ref().block(tmp2).size_in() == 1 {
            tmp2 = fd.bblocks_ref().block(tmp2).get_in(0);
        }
        if Some(tmp2) != self.initblock {
            return false;
        }
        if self.initblock == Some(iblock) {
            return false;
        }
        let initblock = self.initblock.expect("findInitPre: initblock set above");
        self.init2a_true = fd.bblocks_ref().block(initblock).get_true_out() == last;
        true
    }

    /// Verify that \b initblock and \b iblock branch on the same condition (C++
    /// `ConditionalExecution::verifySameCondition`, `condexe.cc:80-94`).
    ///
    /// The conditions must always have the same value or always have
    /// complementary values.
    fn verify_same_condition(&mut self, fd: &Funcdata) -> bool {
        let initblock = self.initblock.expect("verifySameCondition: no initblock");
        let init_cbranch = match fd.bb_op_tail(initblock) {
            Some(o) => o,
            None => return false,
        };
        if fd.obank().get(init_cbranch).expect("verifySameCondition: stale op").code() != OpCode::CPUI_CBRANCH
        {
            return false;
        }
        let cbranch = self.cbranch.expect("verifySameCondition: no cbranch");
        let mut tester = BooleanExpressionMatch::new();
        if !tester.verify_condition(cbranch, init_cbranch, fd.vbank(), fd.obank()) {
            return false;
        }
        if tester.get_flip() {
            self.init2a_true = !self.init2a_true;
        }
        true
    }

    /// Can we move the MULTIEQUAL defining p-code of the given Varnode (C++
    /// `ConditionalExecution::testMultiRead`, `condexe.cc:101-113`).
    ///
    /// \return \b false if it is not possible to move the defining op (because of
    /// the given op)
    fn test_multi_read(&self, vn: VarnodeId, op: OpId, fd: &Funcdata) -> bool {
        let iblock = self.iblock.expect("testMultiRead: no iblock");
        let o = fd.obank().get(op).expect("testMultiRead: stale op");
        if o.get_parent() == Some(iblock) {
            // The copy-like tested separately
            if o.code() == OpCode::CPUI_COPY || o.code() == OpCode::CPUI_SUBPIECE {
                return true; // If the COPY's output reads can be altered, then -vn- can be altered
            }
            return false;
        }
        if o.code() == OpCode::CPUI_RETURN {
            // Only test for flow thru to return value
            if o.num_input() < 2 || o.get_in(1) != Some(vn) {
                return false;
            }
        }
        true
    }

    /// Can we move the (non MULTIEQUAL) defining p-code of the given Varnode (C++
    /// `ConditionalExecution::testOpRead`, `condexe.cc:120-142`).
    fn test_op_read(&self, vn: VarnodeId, op: OpId, fd: &Funcdata) -> bool {
        let iblock = self.iblock.expect("testOpRead: no iblock");
        if fd.obank().get(op).expect("testOpRead: stale op").get_parent() == Some(iblock) {
            return true;
        }
        let write_op = fd.vbank().get(vn).expect("testOpRead: stale vn").get_def().expect("testOpRead: vn not written");
        let opc = fd.obank().get(write_op).expect("testOpRead: stale writeOp").code();
        if opc == OpCode::CPUI_COPY
            || opc == OpCode::CPUI_SUBPIECE
            || opc == OpCode::CPUI_INT_ADD
            || opc == OpCode::CPUI_PTRSUB
        {
            if opc == OpCode::CPUI_INT_ADD || opc == OpCode::CPUI_PTRSUB {
                let in1 = fd.obank().get(write_op).expect("testOpRead").get_in(1).expect("testOpRead: null in");
                if !fd.vbank().get(in1).expect("testOpRead: stale vn").is_constant() {
                    return false;
                }
            }
            let invn = fd.obank().get(write_op).expect("testOpRead").get_in(0).expect("testOpRead: null in");
            let invn_ref = fd.vbank().get(invn).expect("testOpRead: stale vn");
            if invn_ref.is_written() {
                let upop = invn_ref.get_def().expect("testOpRead: written no def");
                let upop_ref = fd.obank().get(upop).expect("testOpRead: stale upop");
                if upop_ref.get_parent() == Some(iblock) && upop_ref.code() != OpCode::CPUI_MULTIEQUAL {
                    return false;
                }
            } else if invn_ref.is_free() {
                return false;
            }
            return true;
        }
        false
    }

    /// Find previously constructed pull-back op (C++
    /// `ConditionalExecution::findPullback`, `condexe.cc:146-152`).
    ///
    /// \param inbranch is the iblock incoming branch to pullback through
    /// \return the output of the previous pullback op, or null
    fn find_pullback(&mut self, inbranch: int4) -> Option<VarnodeId> {
        while (self.pullback.len() as int4) <= inbranch {
            self.pullback.push(None);
        }
        self.pullback[inbranch as usize]
    }

    /// Pull-back PcodeOp out of the iblock (C++
    /// `ConditionalExecution::pullbackOp`, `condexe.cc:160-190`).
    ///
    /// Create a duplicate PcodeOp outside the iblock.  The first input can be
    /// defined by a MULTIEQUAL in the iblock, in which case the duplicate's input
    /// is selected from the MULTIEQUAL input; any other inputs must be constants.
    ///
    /// STUB(W6/opSetOutput): creating the new op's output (`newVarnodeOut` →
    /// `opSetOutput`) and resolving its opcode (`opSetOpcode` → W6) both surface
    /// the documented stub `Err`.  The op-shell creation, input wiring, and
    /// insertion are reproduced.
    fn pullback_op(&mut self, op: OpId, inbranch: int4, fd: &mut Funcdata) -> KunaResult<VarnodeId> {
        if let Some(invn) = self.find_pullback(inbranch) {
            return Ok(invn);
        }
        let iblock = self.iblock.expect("pullbackOp: no iblock");
        let mut invn = fd.obank().get(op).expect("pullbackOp: stale op").get_in(0).expect("pullbackOp: null in");
        let bl: BlockId;
        if fd.vbank().get(invn).expect("pullbackOp: stale vn").is_written() {
            let def_op = fd.vbank().get(invn).expect("pullbackOp").get_def().expect("pullbackOp: written no def");
            if fd.obank().get(def_op).expect("pullbackOp: stale defOp").get_parent() == Some(iblock) {
                bl = fd.bblocks_ref().block(iblock).get_in(inbranch);
                // defOp must be MULTIEQUAL
                invn = fd.obank().get(def_op).expect("pullbackOp").get_in(inbranch).expect("pullbackOp: null in");
            } else {
                bl = fd.bblocks_ref().block(iblock).get_immed_dom().expect("pullbackOp: no immedDom");
            }
        } else {
            bl = fd.bblocks_ref().block(iblock).get_immed_dom().expect("pullbackOp: no immedDom");
        }
        let numinput = fd.obank().get(op).expect("pullbackOp").num_input();
        let opaddr = fd.obank().get(op).expect("pullbackOp").get_addr().clone();
        let new_op = fd.new_op(numinput, opaddr);
        let orig_out_vn = fd.obank().get(op).expect("pullbackOp").get_out().expect("pullbackOp: op has no out");
        let out_size = fd.vbank().get(orig_out_vn).expect("pullbackOp: stale out").get_size();
        let out_addr = fd.vbank().get(orig_out_vn).expect("pullbackOp").get_addr().clone();
        let out_vn = self.new_varnode_out(out_size, &out_addr, new_op, fd)?;
        // fd->opSetOpcode(newOp,op->code());   -- STUB(W6)
        let opc = fd.obank().get(op).expect("pullbackOp").code();
        fd.op_set_opcode(new_op, resolve_typeop(opc)?);
        fd.op_set_input(new_op, invn, 0)?;
        for i in 1..numinput {
            let ini = fd.obank().get(op).expect("pullbackOp").get_in(i).expect("pullbackOp: null in");
            fd.op_set_input(new_op, ini, i)?;
        }
        fd.op_insert_end(new_op, bl);
        self.pullback[inbranch as usize] = Some(out_vn);
        Ok(out_vn)
    }

    /// Create a MULTIEQUAL in the given block that will hold data-flow from the
    /// given PcodeOp (C++ `ConditionalExecution::getNewMulti`, `condexe.cc:198-217`).
    ///
    /// STUB(W6/opSetOutput): `newUniqueOut`→`opSetOutput` and `opSetOpcode`→W6
    /// surface the documented stub `Err`; the op-shell + input wiring + insert are
    /// reproduced.
    fn get_new_multi(&mut self, op: OpId, bl: BlockId, fd: &mut Funcdata) -> KunaResult<VarnodeId> {
        let sizein = fd.bblocks_ref().block(bl).size_in();
        let start = crate::block::block_get_start(&fd.bblocks_ref().arena, bl);
        let newop = fd.new_op(sizein, start);
        let outvn = fd.obank().get(op).expect("getNewMulti: stale op").get_out().expect("getNewMulti: op has no out");
        let outsize = fd.vbank().get(outvn).expect("getNewMulti: stale out").get_size();
        // Using the original outvn address may cause merge conflicts (see C++ comment);
        let newoutvn = self.new_unique_out(outsize, newop, fd)?;
        // fd->opSetOpcode(newop,CPUI_MULTIEQUAL);   -- STUB(W6)
        fd.op_set_opcode(newop, resolve_typeop(OpCode::CPUI_MULTIEQUAL)?);
        // We create NEW references to outvn.
        for i in 0..sizein {
            fd.op_set_input(newop, outvn, i)?;
        }
        fd.op_insert_begin(newop, bl);
        Ok(newoutvn)
    }

    /// `newVarnodeOut(s,m,op)` — create a defined output Varnode for `op` (C++
    /// `Funcdata::newVarnodeOut`, `funcdata_varnode.cc:106`).
    ///
    /// STUB(opSetOutput, LOSS-035/036): `newVarnodeOut` is `newVarnode(s,m,ct)`
    /// then `op->setOutput(vn)` (`createDef`→xref).  The free varnode is created
    /// faithfully via [`Funcdata::new_varnode`]; the def-linking step routes
    /// through [`Funcdata::op_set_output`], which currently returns the documented
    /// `Err` (needs `banks_mut()`).  Surfaced here.
    fn new_varnode_out(
        &self,
        s: int4,
        m: &kuna_base::address::Address,
        op: OpId,
        fd: &mut Funcdata,
    ) -> KunaResult<VarnodeId> {
        let vn = fd.new_varnode(s, m, None);
        fd.op_set_output(op, vn)?; // STUB(opSetOutput)
        Ok(vn)
    }

    /// `newUniqueOut(s,op)` — create a defined unique output Varnode for `op`
    /// (C++ `Funcdata::newUniqueOut`, `funcdata_varnode.cc:131`).  Same stub as
    /// [`new_varnode_out`](ConditionalExecution::new_varnode_out).
    fn new_unique_out(&self, s: int4, op: OpId, fd: &mut Funcdata) -> KunaResult<VarnodeId> {
        let vn = fd.new_unique(s, None);
        fd.op_set_output(op, vn)?; // STUB(opSetOutput)
        Ok(vn)
    }

    /// Calculate the replacement Varnode for a read coming through an arbitrary
    /// block (C++ `ConditionalExecution::resolveRead`, `condexe.cc:224-237`).
    fn resolve_read(&mut self, op: OpId, bl: BlockId, fd: &mut Funcdata) -> KunaResult<VarnodeId> {
        if fd.bblocks_ref().block(bl).size_in() == 1 {
            // Since dominator is iblock, In(0) must be iblock. Figure what side we came through.
            let inrev = fd.bblocks_ref().block(bl).get_in_rev_index(0);
            let slot = if inrev == self.posta_outslot {
                self.camethruposta_slot
            } else {
                1 - self.camethruposta_slot
            };
            self.resolve_iblock_read(op, slot, fd)
        } else {
            self.get_new_multi(op, bl, fd)
        }
    }

    /// Resolve a read op coming through the \b iblock (C++
    /// `ConditionalExecution::resolveIblockRead`, `condexe.cc:242-262`).
    fn resolve_iblock_read(&mut self, op: OpId, inbranch: int4, fd: &mut Funcdata) -> KunaResult<VarnodeId> {
        let iblock = self.iblock.expect("resolveIblockRead: no iblock");
        let mut op = op;
        if fd.obank().get(op).expect("resolveIblockRead: stale op").code() == OpCode::CPUI_COPY {
            let vn = fd.obank().get(op).expect("resolveIblockRead").get_in(0).expect("resolveIblockRead: null in");
            if fd.vbank().get(vn).expect("resolveIblockRead: stale vn").is_written() {
                let def_op = fd.vbank().get(vn).expect("resolveIblockRead").get_def().expect("resolveIblockRead: written no def");
                let def_ref = fd.obank().get(def_op).expect("resolveIblockRead: stale defOp");
                if def_ref.code() == OpCode::CPUI_MULTIEQUAL && def_ref.get_parent() == Some(iblock) {
                    op = def_op;
                }
            } else {
                return Ok(vn);
            }
        }
        let opc = fd.obank().get(op).expect("resolveIblockRead").code();
        if opc == OpCode::CPUI_MULTIEQUAL {
            Ok(fd.obank().get(op).expect("resolveIblockRead").get_in(inbranch).expect("resolveIblockRead: null in"))
        } else if opc == OpCode::CPUI_SUBPIECE || opc == OpCode::CPUI_INT_ADD || opc == OpCode::CPUI_PTRSUB {
            self.pullback_op(op, inbranch, fd)
        } else {
            Err(KunaError::lowlevel("Conditional execution: Illegal op in iblock"))
        }
    }

    /// Get the replacement Varnode for the output of a MULTIEQUAL in the \b iblock,
    /// given the op reading it (C++ `ConditionalExecution::getMultiequalRead`,
    /// `condexe.cc:270-279`).
    fn get_multiequal_read(&mut self, op: OpId, readop: OpId, slot: int4, fd: &mut Funcdata) -> KunaResult<VarnodeId> {
        let iblock = self.iblock.expect("getMultiequalRead: no iblock");
        let bl = fd.obank().get(readop).expect("getMultiequalRead: stale readop").get_parent().expect("getMultiequalRead: readop no parent");
        let inbl = fd.bblocks_ref().block(bl).get_in(slot);
        if inbl != iblock {
            return self.get_replacement_read(op, inbl, fd);
        }
        let inrev = fd.bblocks_ref().block(bl).get_in_rev_index(slot);
        let s = if inrev == self.posta_outslot {
            self.camethruposta_slot
        } else {
            1 - self.camethruposta_slot
        };
        self.resolve_iblock_read(op, s, fd)
    }

    /// Find a replacement Varnode for the output of the given PcodeOp that is read
    /// in the given block (C++ `ConditionalExecution::getReplacementRead`,
    /// `condexe.cc:291-315`).
    fn get_replacement_read(&mut self, op: OpId, bl: BlockId, fd: &mut Funcdata) -> KunaResult<VarnodeId> {
        let iblock = self.iblock.expect("getReplacementRead: no iblock");
        let bl_index = fd.bblocks_ref().block(bl).get_index();
        if let Some(&res) = self.replacement.get(&bl_index) {
            return Ok(res);
        }
        // Flow must eventually come through iblock
        let mut curbl = bl;
        while fd.bblocks_ref().block(curbl).get_immed_dom() != Some(iblock) {
            curbl = match fd.bblocks_ref().block(curbl).get_immed_dom() {
                Some(d) => d,
                None => return Err(KunaError::lowlevel("Conditional execution: Could not find dominator")),
            };
        }
        let curbl_index = fd.bblocks_ref().block(curbl).get_index();
        if let Some(&res) = self.replacement.get(&curbl_index) {
            self.replacement.insert(bl_index, res);
            return Ok(res);
        }
        let res = self.resolve_read(op, curbl, fd)?;
        self.replacement.insert(curbl_index, res);
        if curbl != bl {
            self.replacement.insert(bl_index, res);
        }
        Ok(res)
    }

    /// Replace the data-flow for the given PcodeOp in \b iblock (C++
    /// `ConditionalExecution::doReplacement`, `condexe.cc:320-356`).
    ///
    /// STUB(opSetOutput / W6): the RETURN-input COPY creation and any pullback
    /// MULTIEQUAL creation surface the documented stub `Err`s through the helpers.
    fn do_replacement(&mut self, op: OpId, fd: &mut Funcdata) -> KunaResult<()> {
        self.replacement.clear();
        self.pullback.clear();
        let iblock = self.iblock.expect("doReplacement: no iblock");
        let vn = fd.obank().get(op).expect("doReplacement: stale op").get_out().expect("doReplacement: op has no out");
        loop {
            // The C++ re-fetches beginDescend() each iteration because the body
            // destroys the *current* descendant link; take the first descendant.
            let readop = match fd.vbank().get(vn).expect("doReplacement: stale vn").descend_iter().next() {
                Some(o) => o,
                None => break,
            };
            let mut slot = fd.obank().get(readop).expect("doReplacement: stale readop").get_slot(vn);
            let bl = fd.obank().get(readop).expect("doReplacement").get_parent().expect("doReplacement: readop no parent");
            let mut readop = readop;
            if bl == iblock {
                fd.op_unset_input(readop, slot);
            } else {
                let rvn: VarnodeId;
                let code = fd.obank().get(readop).expect("doReplacement").code();
                if code == OpCode::CPUI_MULTIEQUAL {
                    rvn = self.get_multiequal_read(op, readop, slot, fd)?;
                } else if code == OpCode::CPUI_RETURN && slot > 0 {
                    // Cannot replace input of RETURN directly, create COPY to hold input.
                    let raddr = fd.obank().get(readop).expect("doReplacement").get_addr().clone();
                    let newcopyop = fd.new_op(1, raddr);
                    // fd->opSetOpcode(newcopyop,CPUI_COPY);   -- STUB(W6)
                    fd.op_set_opcode(newcopyop, resolve_typeop(OpCode::CPUI_COPY)?);
                    // Preserve storage
                    let vsize = fd.vbank().get(vn).expect("doReplacement: stale vn").get_size();
                    let vaddr = fd.vbank().get(vn).expect("doReplacement").get_addr().clone();
                    let outvn = self.new_varnode_out(vsize, &vaddr, newcopyop, fd)?;
                    fd.op_set_input(readop, outvn, slot)?;
                    fd.op_insert_before(newcopyop, readop);
                    readop = newcopyop;
                    slot = 0;
                    // COPY gets the replacement read
                    rvn = self.get_replacement_read(op, bl, fd)?;
                } else {
                    rvn = self.get_replacement_read(op, bl, fd)?;
                }
                fd.op_set_input(readop, rvn, slot)?;
            }
            // The last descendant is now gone (loop re-fetches the first).
        }
        Ok(())
    }

    /// Test if the given PcodeOp can be removed from \b iblock (C++
    /// `ConditionalExecution::testRemovability`, `condexe.cc:360-396`).
    fn test_removability(&self, op: OpId, fd: &Funcdata) -> bool {
        let o = fd.obank().get(op).expect("testRemovability: stale op");
        if o.code() == OpCode::CPUI_MULTIEQUAL {
            let vn = o.get_out().expect("testRemovability: multiequal no out");
            let readers: Vec<OpId> = fd.vbank().get(vn).expect("testRemovability: stale vn").descend_iter().collect();
            for readop in readers {
                if !self.test_multi_read(vn, readop, fd) {
                    return false;
                }
            }
        } else {
            if o.is_flow_break() || o.is_call() {
                return false;
            }
            if o.code() == OpCode::CPUI_LOAD || o.code() == OpCode::CPUI_STORE {
                return false;
            }
            if o.code() == OpCode::CPUI_INDIRECT {
                return false;
            }
            let vn_opt = o.get_out();
            // NOTE (C++): the deref `vn->isAddrTied()` happens before the `vn != 0`
            // guard in the source; faithful ports keep the same evaluation but the
            // C++ relies on op outputs being non-null here.  We guard defensively:
            if let Some(vn) = vn_opt {
                if fd.vbank().get(vn).expect("testRemovability: stale vn").is_addr_tied() {
                    return false;
                }
                let mut hasnodescend = true;
                let readers: Vec<OpId> = fd.vbank().get(vn).expect("testRemovability").descend_iter().collect();
                for readop in readers {
                    if !self.test_op_read(vn, readop, fd) {
                        return false;
                    }
                    hasnodescend = false;
                }
                if hasnodescend {
                    // Check if heritage is performed for this varnode's space
                    let spc_index = fd.vbank().get(vn).expect("testRemovability").get_space().get_index();
                    if !self.heritageyes[spc_index as usize] {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// Verify that we have a removable \b iblock (C++
    /// `ConditionalExecution::verify`, `condexe.cc:401-427`).
    ///
    /// The \b iblock has been fixed.  Test all control-flow conditions, and test
    /// removability of all ops in the \b iblock.
    fn verify(&mut self, fd: &Funcdata) -> bool {
        self.prea_inslot = 0;
        self.posta_outslot = 0;

        if !self.test_iblock(fd) {
            return false;
        }
        if !self.find_init_pre(fd) {
            return false;
        }
        if !self.verify_same_condition(fd) {
            return false;
        }

        let iblock = self.iblock.expect("verify: no iblock");
        // Cache some useful values
        self.iblock2posta_true = self.posta_outslot == 1;
        self.camethruposta_slot = if self.init2a_true == self.iblock2posta_true {
            self.prea_inslot
        } else {
            1 - self.prea_inslot
        };
        self.posta_block = Some(fd.bblocks_ref().block(iblock).get_out(self.posta_outslot));
        self.postb_block = Some(fd.bblocks_ref().block(iblock).get_out(1 - self.posta_outslot));

        // Iterate the iblock's ops from the one BEFORE the last (branch) back to
        // the first, testing each.  The basic intrusive list gives us tail/head.
        let tail = fd.bb_op_tail(iblock);
        // Skip the branch (tail) if the block is non-empty.
        let mut iter = match tail {
            Some(t) => fd.op_previous_op(t), // the op before the branch
            None => None,
        };
        while let Some(cur) = iter {
            if !self.test_removability(cur, fd) {
                return false;
            }
            iter = fd.op_previous_op(cur);
        }
        true
    }

    /// Test for a modifiable configuration around the given block (C++
    /// `ConditionalExecution::trial`, `condexe.cc:447-453`).
    ///
    /// \return \b true if (some) configuration is recognized and can be modified
    pub fn trial(&mut self, ib: BlockId, fd: &Funcdata) -> bool {
        self.iblock = Some(ib);
        if !self.verify(fd) {
            return false;
        }
        true
    }

    /// Eliminate the unnecessary path join at \b iblock (C++
    /// `ConditionalExecution::execute`, `condexe.cc:456-475`).
    ///
    /// We assume the last call to [`verify`](ConditionalExecution::verify)
    /// returned \b true.  STUB(opSetOutput / W6): the data-flow rewrite
    /// (`doReplacement`) surfaces the documented stub `Err`s.
    pub fn execute(&mut self, fd: &mut Funcdata) -> KunaResult<()> {
        let iblock = self.iblock.expect("execute: no iblock");
        // Remove ops in reverse order
        let mut iter = fd.bb_op_tail(iblock);
        loop {
            let op = iter.expect("execute: empty iblock (verify guarantees a branch)");
            let prev = fd.op_previous_op(op);
            let notdone = prev.is_some();
            if notdone {
                iter = prev;
            }
            if !fd.obank().get(op).expect("execute: stale op").is_branch() {
                self.do_replacement(op, fd)?; // Remove all read refs of op
            }
            // Then destroy op
            fd.op_destroy(op);
            if !notdone {
                break;
            }
        }
        fd.remove_from_flow_split(iblock, self.posta_outslot != self.camethruposta_slot)?;
        Ok(())
    }
}

// =============================================================================
// ActionConditionalExe (condexe.hh:133-141, condexe.cc:477-502)
// =============================================================================

/// Search for and remove various forms of redundant CBRANCH operations (C++
/// `ActionConditionalExe`).
///
/// Wraps the analysis performed by [`ConditionalExecution`] to simplify
/// control-flow that repeatedly branches on the same (or slightly modified)
/// boolean expression.
pub struct ActionConditionalExe {
    base: ActionBase,
}

impl ActionConditionalExe {
    pub fn new(g: impl Into<String>) -> ActionConditionalExe {
        ActionConditionalExe { base: ActionBase::new(0, "conditionalexe", g) }
    }
}

impl Action for ActionConditionalExe {
    fn base(&self) -> &ActionBase {
        &self.base
    }

    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }

    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(&self.base.basegroup) {
            return None;
        }
        Some(Box::new(ActionConditionalExe::new(self.base.basegroup.clone())))
    }

    /// C++ `ActionConditionalExe::apply` (`condexe.cc:477-502`).
    ///
    /// The control loop (the round/`changethisround` fixpoint over the basic
    /// blocks) is reproduced exactly; `count += numhits` lands on
    /// [`ActionBase::count`].
    ///
    /// STUB(opSetOutput / W6): a successful trial whose `condexe.execute()` then
    /// surfaces the documented stub `Err` cannot abort an `apply` that returns a
    /// bare `int4`; such an execute degrades to "no change made this trial" (the
    /// hit is not counted and the round does not loop on it).  This is the only
    /// faithful adaptation possible until `opSetOutput` lands; recorded as a loss.
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if data.has_unreachable_blocks() {
            // Conditional execution elimination logic may not work with unreachable blocks
            return 0;
        }
        let mut numhits: int4 = 0;
        let mut condexe = ConditionalExecution::new(data);
        loop {
            let mut changethisround = false;
            // `getSize()` is re-evaluated every iteration in the C++ loop
            // condition: `condexe.execute()` can REMOVE a block (the conditional-
            // execution elimination), shrinking the graph, so the bound must be
            // re-read each step.  (Hoisting it into a `0..size` range over-indexes
            // `getBlock(i)` once the graph shrinks — a panic the deeper, deadcode-
            // pruned IR now reaches.)
            let mut i: int4 = 0;
            while i < data.bblocks_get_size() {
                let bb = data.bblocks_get_block(i);
                if condexe.trial(bb, data) {
                    // condexe.execute() now succeeds (the W6 resolve_typeop stub is
                    // closed); a residual Err still degrades to "no change this
                    // trial" rather than aborting the bare-int4 apply.
                    if condexe.execute(data).is_ok() {
                        numhits += 1; // Adjust dataflow succeeded
                        changethisround = true;
                    }
                }
                i += 1;
            }
            if !changethisround {
                break;
            }
        }
        self.base.count += numhits;
        0
    }
}

// =============================================================================
// RuleOrPredicate (condexe.hh:172-196, condexe.cc:504-709)
// =============================================================================

/// A helper class to mark up predicated INT_OR expressions (C++
/// `RuleOrPredicate::MultiPredicate`).
struct MultiPredicate {
    /// Base MULTIEQUAL op (C++ `PcodeOp *op`).
    op: Option<OpId>,
    /// Input slot containing path that sets zero (C++ `int4 zeroSlot`).
    zero_slot: int4,
    /// Final block in path that sets zero (C++ `const FlowBlock *zeroBlock`).
    zero_block: Option<BlockId>,
    /// Conditional block determining if zero is set or not (C++ `const FlowBlock *condBlock`).
    cond_block: Option<BlockId>,
    /// CBRANCH determining if zero is set (C++ `PcodeOp *cbranch`).
    cbranch: Option<OpId>,
    /// Other (non-zero) Varnode getting set on other path (C++ `Varnode *otherVn`).
    other_vn: Option<VarnodeId>,
    /// True if path to zero set is the \b true path out of condBlock (C++ `bool zeroPathIsTrue`).
    zero_path_is_true: bool,
}

impl MultiPredicate {
    fn new() -> MultiPredicate {
        MultiPredicate {
            op: None,
            zero_slot: 0,
            zero_block: None,
            cond_block: None,
            cbranch: None,
            other_vn: None,
            zero_path_is_true: false,
        }
    }

    /// Check if `vn` is produced by a 2-branch MULTIEQUAL, one side a zero
    /// constant (C++ `MultiPredicate::discoverZeroSlot`, `condexe.cc:508-528`).
    fn discover_zero_slot(&mut self, vn: VarnodeId, fd: &Funcdata) -> bool {
        if !fd.vbank().get(vn).expect("discoverZeroSlot: stale vn").is_written() {
            return false;
        }
        let op = fd.vbank().get(vn).expect("discoverZeroSlot").get_def().expect("discoverZeroSlot: written no def");
        self.op = Some(op);
        if fd.obank().get(op).expect("discoverZeroSlot: stale op").code() != OpCode::CPUI_MULTIEQUAL {
            return false;
        }
        if fd.obank().get(op).expect("discoverZeroSlot").num_input() != 2 {
            return false;
        }
        let mut zs = 0;
        while zs < 2 {
            self.zero_slot = zs;
            let tmpvn = fd.obank().get(op).expect("discoverZeroSlot").get_in(zs).expect("discoverZeroSlot: null in");
            if !fd.vbank().get(tmpvn).expect("discoverZeroSlot: stale vn").is_written() {
                zs += 1;
                continue;
            }
            let copyop = fd.vbank().get(tmpvn).expect("discoverZeroSlot").get_def().expect("discoverZeroSlot: written no def");
            // Multiequal must have CPUI_COPY input
            if fd.obank().get(copyop).expect("discoverZeroSlot: stale copyop").code() != OpCode::CPUI_COPY {
                zs += 1;
                continue;
            }
            let zerovn = fd.obank().get(copyop).expect("discoverZeroSlot").get_in(0).expect("discoverZeroSlot: null in");
            if !fd.vbank().get(zerovn).expect("discoverZeroSlot: stale vn").is_constant() {
                zs += 1;
                continue;
            }
            // which copies #0
            if fd.vbank().get(zerovn).expect("discoverZeroSlot").get_offset() != 0 {
                zs += 1;
                continue;
            }
            // store off varnode from other path
            let other_vn = fd.obank().get(op).expect("discoverZeroSlot").get_in(1 - zs).expect("discoverZeroSlot: null in");
            self.other_vn = Some(other_vn);
            if fd.vbank().get(other_vn).expect("discoverZeroSlot: stale vn").is_free() {
                return false;
            }
            return true;
        }
        false
    }

    /// Find CBRANCH determining whether zero is set or not (C++
    /// `MultiPredicate::discoverCbranch`, `condexe.cc:538-566`).
    fn discover_cbranch(&mut self, fd: &Funcdata) -> bool {
        let op = self.op.expect("discoverCbranch: no op");
        let base_block = fd.obank().get(op).expect("discoverCbranch: stale op").get_parent().expect("discoverCbranch: op no parent");
        let zero_block = fd.bblocks_ref().block(base_block).get_in(self.zero_slot);
        self.zero_block = Some(zero_block);
        let other_block = fd.bblocks_ref().block(base_block).get_in(1 - self.zero_slot);
        let zb_sizeout = fd.bblocks_ref().block(zero_block).size_out();
        let cond_block;
        if zb_sizeout == 1 {
            if fd.bblocks_ref().block(zero_block).size_in() != 1 {
                return false;
            }
            cond_block = fd.bblocks_ref().block(zero_block).get_in(0);
        } else if zb_sizeout == 2 {
            cond_block = zero_block;
        } else {
            return false;
        }
        self.cond_block = Some(cond_block);
        if fd.bblocks_ref().block(cond_block).size_out() != 2 {
            return false;
        }
        let ob_sizeout = fd.bblocks_ref().block(other_block).size_out();
        if ob_sizeout == 1 {
            if fd.bblocks_ref().block(other_block).size_in() != 1 {
                return false;
            }
            if cond_block != fd.bblocks_ref().block(other_block).get_in(0) {
                return false;
            }
        } else if ob_sizeout == 2 {
            if cond_block != other_block {
                return false;
            }
        } else {
            return false;
        }
        let cbranch = match fd.bb_op_tail(cond_block) {
            Some(o) => o,
            None => return false,
        };
        self.cbranch = Some(cbranch);
        if fd.obank().get(cbranch).expect("discoverCbranch: stale cbranch").code() != OpCode::CPUI_CBRANCH {
            return false;
        }
        true
    }

    /// Does the \b condBlock \b true outgoing edge flow to the block that sets
    /// zero (C++ `MultiPredicate::discoverPathIsTrue`, `condexe.cc:571-581`).
    fn discover_path_is_true(&mut self, fd: &Funcdata) {
        let cond_block = self.cond_block.expect("discoverPathIsTrue: no condBlock");
        let zero_block = self.zero_block.expect("discoverPathIsTrue: no zeroBlock");
        if fd.bblocks_ref().block(cond_block).get_true_out() == zero_block {
            self.zero_path_is_true = true;
        } else if fd.bblocks_ref().block(cond_block).get_false_out() == zero_block {
            self.zero_path_is_true = false;
        } else {
            // condBlock must be zeroBlock
            let op = self.op.expect("discoverPathIsTrue: no op");
            let parent = fd.obank().get(op).expect("discoverPathIsTrue: stale op").get_parent().expect("discoverPathIsTrue: op no parent");
            self.zero_path_is_true = fd.bblocks_ref().block(cond_block).get_true_out() == parent;
        }
    }

    /// Verify the CBRANCH boolean expression is (vn==0) or (vn!=0) (C++
    /// `MultiPredicate::discoverConditionalZero`, `condexe.cc:589-614`).
    fn discover_conditional_zero(&mut self, vn: VarnodeId, fd: &Funcdata) -> bool {
        let cbranch = self.cbranch.expect("discoverConditionalZero: no cbranch");
        let boolvn = fd.obank().get(cbranch).expect("discoverConditionalZero: stale cbranch").get_in(1).expect("discoverConditionalZero: null in");
        if !fd.vbank().get(boolvn).expect("discoverConditionalZero: stale vn").is_written() {
            return false;
        }
        let compareop = fd.vbank().get(boolvn).expect("discoverConditionalZero").get_def().expect("discoverConditionalZero: written no def");
        let opc = fd.obank().get(compareop).expect("discoverConditionalZero: stale compareop").code();
        if opc == OpCode::CPUI_INT_NOTEQUAL {
            self.zero_path_is_true = !self.zero_path_is_true;
        } else if opc != OpCode::CPUI_INT_EQUAL {
            return false;
        }
        let a1 = fd.obank().get(compareop).expect("discoverConditionalZero").get_in(0).expect("discoverConditionalZero: null in");
        let a2 = fd.obank().get(compareop).expect("discoverConditionalZero").get_in(1).expect("discoverConditionalZero: null in");
        let zerovn = if a1 == vn {
            a2
        } else if a2 == vn {
            a1
        } else {
            return false;
        };
        if !fd.vbank().get(zerovn).expect("discoverConditionalZero: stale vn").is_constant() {
            return false;
        }
        // Verify we are comparing to zero
        if fd.vbank().get(zerovn).expect("discoverConditionalZero").get_offset() != 0 {
            return false;
        }
        if fd.obank().get(cbranch).expect("discoverConditionalZero").is_boolean_flip() {
            self.zero_path_is_true = !self.zero_path_is_true;
        }
        true
    }
}

/// Simplify predication constructions involving the INT_OR operator (C++
/// `RuleOrPredicate`).  See `condexe.hh` for the forms.
pub struct RuleOrPredicate {
    state: RuleState,
}

impl RuleOrPredicate {
    pub fn new(g: impl Into<String>) -> RuleOrPredicate {
        RuleOrPredicate { state: RuleState::new(g, 0, "orpredicate") }
    }

    /// Boxed-ctor + group used by [`specs`].
    fn boxed() -> Box<dyn Rule> {
        Box::new(RuleOrPredicate::new("condexe"))
    }

    /// Check for the \e alternate form `tmp1 = (val2 == 0) ? val1 : 0;` (C++
    /// `RuleOrPredicate::checkSingle`, `condexe.cc:637-651`).
    ///
    /// STUB(opSetOutput / W6): `data.opSetOpcode(op,CPUI_COPY)` surfaces the W6
    /// stub; the input rewiring (`opSetInput`/`opRemoveInput`) is reproduced.
    fn check_single(
        &self,
        vn: VarnodeId,
        branch: &mut MultiPredicate,
        op: OpId,
        data: &mut Funcdata,
    ) -> KunaResult<int4> {
        if data.vbank().get(vn).expect("checkSingle: stale vn").is_free() {
            return Ok(0);
        }
        if !branch.discover_cbranch(data) {
            return Ok(0);
        }
        // Must only be one use of MULTIEQUAL
        let branch_op = branch.op.expect("checkSingle: branch has no op");
        let branch_out = data.obank().get(branch_op).expect("checkSingle: stale branch op").get_out().expect("checkSingle: branch op no out");
        if data.lone_descend(branch_out) != Some(op) {
            return Ok(0);
        }
        branch.discover_path_is_true(data);
        if !branch.discover_conditional_zero(vn, data) {
            return Ok(0);
        }
        // true condition (vn==0) must not go to zero set
        if branch.zero_path_is_true {
            return Ok(0);
        }
        data.op_set_input(branch_op, vn, branch.zero_slot)?;
        data.op_remove_input(op, 1);
        // data.opSetOpcode(op,CPUI_COPY);   -- STUB(W6)
        data.op_set_opcode(op, resolve_typeop(OpCode::CPUI_COPY)?);
        data.op_set_input(op, branch_out, 0)?;
        Ok(1)
    }
}

impl Rule for RuleOrPredicate {
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Note: the C++ returns int4; the stub `Err` paths cannot return through
        // an `int4`, so a stub error degrades to "no change" (0) here and is
        // reported by the helper's `Err`.  The applyOp structure is otherwise
        // faithful.  See STUB notes.
        self.apply_op_inner(op, data).unwrap_or(0)
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_OR, OpCode::CPUI_INT_XOR]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.state.basegroup) {
            return None;
        }
        Some(Box::new(RuleOrPredicate::new(self.state.basegroup.clone())))
    }
}

impl RuleOrPredicate {
    /// The `KunaResult`-returning body of [`Rule::apply_op`] (C++
    /// `RuleOrPredicate::applyOp`, `condexe.cc:653-709`).
    ///
    /// STUB(opSetOutput / W6): `newOp`+`newUniqueOut`+`opSetOpcode(MULTIEQUAL)` for
    /// the new MULTIEQUAL, and the final `opSetOpcode(op,CPUI_COPY)`, surface the
    /// documented stub `Err`s.
    fn apply_op_inner(&self, op: OpId, data: &mut Funcdata) -> KunaResult<int4> {
        let mut branch0 = MultiPredicate::new();
        let mut branch1 = MultiPredicate::new();
        let op_in0 = data.obank().get(op).expect("applyOp: stale op").get_in(0).expect("applyOp: null in");
        let op_in1 = data.obank().get(op).expect("applyOp").get_in(1).expect("applyOp: null in");
        let test0 = branch0.discover_zero_slot(op_in0, data);
        let test1 = branch1.discover_zero_slot(op_in1, data);
        if !test0 && !test1 {
            return Ok(0);
        }
        if !test0 {
            // branch1 has MULTIEQUAL form, but branch0 does not
            return self.check_single(op_in0, &mut branch1, op, data);
        } else if !test1 {
            // branch0 has MULTIEQUAL form, but branch1 does not
            return self.check_single(op_in1, &mut branch0, op, data);
        }
        if !branch0.discover_cbranch(data) {
            return Ok(0);
        }
        if !branch1.discover_cbranch(data) {
            return Ok(0);
        }
        let cond0 = branch0.cond_block;
        let cond1 = branch1.cond_block;
        if cond0 == cond1 {
            // zero sets must be along different paths
            if branch0.zero_block == branch1.zero_block {
                return Ok(0);
            }
        } else {
            // Make sure cbranches have shared condition and the different zero sets have complementary paths
            let mut condmarker = BooleanExpressionMatch::new();
            let b0cb = branch0.cbranch.expect("applyOp: branch0 no cbranch");
            let b1cb = branch1.cbranch.expect("applyOp: branch1 no cbranch");
            if !condmarker.verify_condition(b0cb, b1cb, data.vbank(), data.obank()) {
                return Ok(0);
            }
            if condmarker.get_multi_slot() != -1 {
                return Ok(0);
            }
            branch0.discover_path_is_true(data);
            branch1.discover_path_is_true(data);
            let mut final_bool = branch0.zero_path_is_true == branch1.zero_path_is_true;
            if condmarker.get_flip() {
                final_bool = !final_bool;
            }
            // One path hits both zero sets
            if final_bool {
                return Ok(0);
            }
        }
        let branch0_op = branch0.op.expect("applyOp: branch0 no op");
        let branch1_op = branch1.op.expect("applyOp: branch1 no op");
        let order = self.compare_order(branch0_op, branch1_op, data);
        if order == 0 {
            return Ok(0); // can this happen?
        }
        let final_block: BlockId;
        let slot0_sets_branch0: bool; // True if non-zero setting of branch0 flows throw slot0
        if order < 0 {
            // branch1 happens after
            final_block = data.obank().get(branch1_op).expect("applyOp: stale branch1 op").get_parent().expect("applyOp: branch1 op no parent");
            slot0_sets_branch0 = branch1.zero_slot == 0;
        } else {
            // branch0 happens after
            final_block = data.obank().get(branch0_op).expect("applyOp: stale branch0 op").get_parent().expect("applyOp: branch0 op no parent");
            slot0_sets_branch0 = branch0.zero_slot == 1;
        }
        let start = crate::block::block_get_start(&data.bblocks_ref().arena, final_block);
        let new_multi = data.new_op(2, start);
        // data.opSetOpcode(newMulti,CPUI_MULTIEQUAL);   -- STUB(W6)
        data.op_set_opcode(new_multi, resolve_typeop(OpCode::CPUI_MULTIEQUAL)?);
        let b0_other = branch0.other_vn.expect("applyOp: branch0 no otherVn");
        let b1_other = branch1.other_vn.expect("applyOp: branch1 no otherVn");
        if slot0_sets_branch0 {
            data.op_set_input(new_multi, b0_other, 0)?;
            data.op_set_input(new_multi, b1_other, 1)?;
        } else {
            data.op_set_input(new_multi, b1_other, 0)?;
            data.op_set_input(new_multi, b0_other, 1)?;
        }
        let other_size = data.vbank().get(b0_other).expect("applyOp: stale otherVn").get_size();
        let newvn = self.new_unique_out_data(other_size, new_multi, data)?;
        data.op_insert_begin(new_multi, final_block);
        data.op_remove_input(op, 1);
        data.op_set_input(op, newvn, 0)?;
        // data.opSetOpcode(op,CPUI_COPY);   -- STUB(W6)
        data.op_set_opcode(op, resolve_typeop(OpCode::CPUI_COPY)?);
        Ok(1)
    }

    /// `newUniqueOut(s,op)` for the rule's data-mutation context (same stub as
    /// [`ConditionalExecution::new_unique_out`]).
    fn new_unique_out_data(&self, s: int4, op: OpId, data: &mut Funcdata) -> KunaResult<VarnodeId> {
        let vn = data.new_unique(s, None);
        data.op_set_output(op, vn)?; // STUB(opSetOutput)
        Ok(vn)
    }

    /// `PcodeOp::compareOrder(op2)` (C++ `op.cc`, used by `applyOp`).
    ///
    /// Reproduces the two-op execution-order comparison via the block graph free
    /// fn [`crate::block::pcodeop_compare_order`] (same-block by `SeqNum` order,
    /// cross-block by common-dominator).
    fn compare_order(&self, op_a: OpId, op_b: OpId, data: &mut Funcdata) -> int4 {
        let parent_a = data.obank().get(op_a).expect("compareOrder: stale op").get_parent().expect("compareOrder: op_a no parent");
        let parent_b = data.obank().get(op_b).expect("compareOrder: stale op").get_parent().expect("compareOrder: op_b no parent");
        // Clone the lightweight op state needed by the comparator (it reads only
        // SeqNum / parent), avoiding holding an op borrow across the &mut graph.
        let a = data.obank().get(op_a).expect("compareOrder: stale op").clone();
        let b = data.obank().get(op_b).expect("compareOrder: stale op").clone();
        crate::block::pcodeop_compare_order(data.bblocks_mut(), &a, parent_a, &b, parent_b)
    }
}

/// Rule registration rows for this file, **in C++ definition order** (W8
/// assembles `universalAction` registration from these).  This file defines one
/// rule, `RuleOrPredicate`; [`ActionConditionalExe`] is a leaf [`Action`], not a
/// [`Rule`], and is registered separately by the action-group builder.
pub fn specs() -> Vec<RuleSpec> {
    vec![RuleSpec { group: "condexe", ctor: RuleOrPredicate::boxed }]
}

#[cfg(test)]
mod tests;
