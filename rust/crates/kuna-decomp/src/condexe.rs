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
//! # Faithfulness — what is fully ported vs. seam-deferred
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
//! two documented seams the rule wave inherits:
//!
//!   - **`Funcdata::opSetOutput`** (LOSS-035/036): creating a new op with an
//!     output (`newOp` + `newVarnodeOut`/`newUniqueOut` → `opSetOutput`) needs the
//!     `banks_mut()` `(&mut VarnodeBank, &mut PcodeOpBank)` split-borrow accessor
//!     the `funcdata` owner must add (for `vbank.setDef` + `replace_reads`).  The
//!     port reproduces the op-shell creation and surfaces the existing
//!     `op_set_output` `Err` — so `pullback_op`, `get_new_multi`, `do_replacement`,
//!     `execute`, `apply`, and `RuleOrPredicate::apply_op`/`check_single` return
//!     `KunaResult` and the seam is visible, never silently mis-ported.
//!   - **`Funcdata::opSetOpcode`** (W6 `glb->inst[opc]`): the rule needs the
//!     resolved [`crate::seams::TypeOp`] for a target opcode; the W6 op-info table
//!     is not present, so [`resolve_typeop`] surfaces a seam `Err`.  // SEAM(W6)
//!   - **`Funcdata::numHeritagePasses`** (W7 heritage): `buildHeritageArray`
//!     queries per-space heritage progress; the W7 `Heritage` subsystem is not
//!     present, so it reports "no pass yet" (every space `false`).  // SEAM(W7)
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
use crate::seams::{BlockId, OpId, TypeOp, VarnodeId};

/// Resolve an [`OpCode`] to the [`TypeOp`] `opSetOpcode` needs.
///
/// SEAM(W6): the C++ `opSetOpcode(op,opc)` is `obank.changeOpcode(op,glb->inst[opc])`
/// — it looks the singleton `TypeOp` up in the architecture's op-info table.  That
/// table (`typeop`, W6) is not present at this item's boundary, so we cannot supply
/// the correct cached property-flag word.  Returns an `Err` so the mutation paths
/// that set an opcode surface the seam rather than fabricate flags.  // see losses
fn resolve_typeop(_opc: OpCode) -> KunaResult<TypeOp> {
    Err(KunaError::lowlevel(
        "kuna rust port: opSetOpcode needs glb->inst[opc] (W6 op-info table) to resolve the \
         TypeOp property flags; not available at this item's boundary",
    ))
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
            // if (fd->numHeritagePasses(spc) > 0) heritageyes[index] = true;
            // SEAM(W7): numHeritagePasses queries the W7 Heritage subsystem, which
            // is not present at this item's boundary.  Treated as "no pass yet"
            // (every heritaged space stays false); the heritage-gated guard in
            // test_removability is then conservative (refuses to remove ops whose
            // output has no descendants).  Recorded as a loss.
            let num_heritage_passes = 0; // SEAM(W7): fd.num_heritage_passes(spc)
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
        // if (iblock->sizeIn() != 2) return false;
        if fd.bblocks_ref().block(iblock).size_in() != 2 {
            return false;
        }
        // if (iblock->sizeOut() != 2) return false;
        if fd.bblocks_ref().block(iblock).size_out() != 2 {
            return false;
        }
        // cbranch = iblock->lastOp(); if (cbranch == 0) return false;
        let cbranch = match fd.bb_op_tail(iblock) {
            Some(o) => o,
            None => return false,
        };
        self.cbranch = Some(cbranch);
        // if (cbranch->code() != CPUI_CBRANCH) return false;
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
        // FlowBlock *tmp = iblock->getIn(prea_inslot); FlowBlock *last = iblock;
        let mut tmp = fd.bblocks_ref().block(iblock).get_in(self.prea_inslot);
        let mut last = iblock;
        // while((tmp->sizeOut()==1)&&(tmp->sizeIn()==1)) { last = tmp; tmp = tmp->getIn(0); }
        while fd.bblocks_ref().block(tmp).size_out() == 1 && fd.bblocks_ref().block(tmp).size_in() == 1 {
            last = tmp;
            tmp = fd.bblocks_ref().block(tmp).get_in(0);
        }
        // if (tmp->sizeOut() != 2) return false;
        if fd.bblocks_ref().block(tmp).size_out() != 2 {
            return false;
        }
        // initblock = (BlockBasic *)tmp;
        self.initblock = Some(tmp);
        // tmp = iblock->getIn(1-prea_inslot);
        let mut tmp2 = fd.bblocks_ref().block(iblock).get_in(1 - self.prea_inslot);
        // while((tmp->sizeOut()==1)&&(tmp->sizeIn()==1)) tmp = tmp->getIn(0);
        while fd.bblocks_ref().block(tmp2).size_out() == 1 && fd.bblocks_ref().block(tmp2).size_in() == 1 {
            tmp2 = fd.bblocks_ref().block(tmp2).get_in(0);
        }
        // if (tmp != initblock) return false;
        if Some(tmp2) != self.initblock {
            return false;
        }
        // if (initblock == iblock) return false;
        if self.initblock == Some(iblock) {
            return false;
        }
        // init2a_true = (initblock->getTrueOut() == last);
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
        // PcodeOp *init_cbranch = initblock->lastOp(); if (init_cbranch == 0) return false;
        let init_cbranch = match fd.bb_op_tail(initblock) {
            Some(o) => o,
            None => return false,
        };
        // if (init_cbranch->code() != CPUI_CBRANCH) return false;
        if fd.obank().get(init_cbranch).expect("verifySameCondition: stale op").code() != OpCode::CPUI_CBRANCH
        {
            return false;
        }
        let cbranch = self.cbranch.expect("verifySameCondition: no cbranch");
        // BooleanExpressionMatch tester; if (!tester.verifyCondition(cbranch,init_cbranch)) return false;
        let mut tester = BooleanExpressionMatch::new();
        if !tester.verify_condition(cbranch, init_cbranch, fd.vbank(), fd.obank()) {
            return false;
        }
        // if (tester.getFlip()) init2a_true = !init2a_true;
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
        // if (op->getParent() == iblock) { ... }
        if o.get_parent() == Some(iblock) {
            // The copy-like tested separately
            if o.code() == OpCode::CPUI_COPY || o.code() == OpCode::CPUI_SUBPIECE {
                return true; // If the COPY's output reads can be altered, then -vn- can be altered
            }
            return false;
        }
        // if (op->code() == CPUI_RETURN) { if ((op->numInput()<2)||(op->getIn(1)!=vn)) return false; }
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
        // if (op->getParent() == iblock) return true;
        if fd.obank().get(op).expect("testOpRead: stale op").get_parent() == Some(iblock) {
            return true;
        }
        // PcodeOp *writeOp = vn->getDef(); OpCode opc = writeOp->code();
        let write_op = fd.vbank().get(vn).expect("testOpRead: stale vn").get_def().expect("testOpRead: vn not written");
        let opc = fd.obank().get(write_op).expect("testOpRead: stale writeOp").code();
        if opc == OpCode::CPUI_COPY
            || opc == OpCode::CPUI_SUBPIECE
            || opc == OpCode::CPUI_INT_ADD
            || opc == OpCode::CPUI_PTRSUB
        {
            if opc == OpCode::CPUI_INT_ADD || opc == OpCode::CPUI_PTRSUB {
                // if (!writeOp->getIn(1)->isConstant()) return false;
                let in1 = fd.obank().get(write_op).expect("testOpRead").get_in(1).expect("testOpRead: null in");
                if !fd.vbank().get(in1).expect("testOpRead: stale vn").is_constant() {
                    return false;
                }
            }
            // Varnode *invn = writeOp->getIn(0);
            let invn = fd.obank().get(write_op).expect("testOpRead").get_in(0).expect("testOpRead: null in");
            let invn_ref = fd.vbank().get(invn).expect("testOpRead: stale vn");
            if invn_ref.is_written() {
                // PcodeOp *upop = invn->getDef();
                let upop = invn_ref.get_def().expect("testOpRead: written no def");
                // if ((upop->getParent()==iblock)&&(upop->code()!=CPUI_MULTIEQUAL)) return false;
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
        // while(pullback.size() <= inbranch) pullback.push_back(0);
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
    /// SEAM(W6/opSetOutput): creating the new op's output (`newVarnodeOut` →
    /// `opSetOutput`) and resolving its opcode (`opSetOpcode` → W6) both surface
    /// the documented seam `Err`.  The op-shell creation, input wiring, and
    /// insertion are reproduced.
    fn pullback_op(&mut self, op: OpId, inbranch: int4, fd: &mut Funcdata) -> KunaResult<VarnodeId> {
        // Varnode *invn = findPullback(inbranch); if (invn != 0) return invn;
        if let Some(invn) = self.find_pullback(inbranch) {
            return Ok(invn);
        }
        let iblock = self.iblock.expect("pullbackOp: no iblock");
        // invn = op->getIn(0);
        let mut invn = fd.obank().get(op).expect("pullbackOp: stale op").get_in(0).expect("pullbackOp: null in");
        let bl: BlockId;
        // if (invn->isWritten()) { ... } else { bl = iblock->getImmedDom(); }
        if fd.vbank().get(invn).expect("pullbackOp: stale vn").is_written() {
            let def_op = fd.vbank().get(invn).expect("pullbackOp").get_def().expect("pullbackOp: written no def");
            // if (defOp->getParent() == iblock) { bl = iblock->getIn(inbranch); invn = defOp->getIn(inbranch); }
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
        // PcodeOp *newOp = fd->newOp(op->numInput(),op->getAddr());
        let numinput = fd.obank().get(op).expect("pullbackOp").num_input();
        let opaddr = fd.obank().get(op).expect("pullbackOp").get_addr().clone();
        let new_op = fd.new_op(numinput, opaddr);
        // Varnode *origOutVn = op->getOut();
        let orig_out_vn = fd.obank().get(op).expect("pullbackOp").get_out().expect("pullbackOp: op has no out");
        let out_size = fd.vbank().get(orig_out_vn).expect("pullbackOp: stale out").get_size();
        let out_addr = fd.vbank().get(orig_out_vn).expect("pullbackOp").get_addr().clone();
        // Varnode *outVn = fd->newVarnodeOut(origOutVn->getSize(),origOutVn->getAddr(),newOp);
        let out_vn = self.new_varnode_out(out_size, &out_addr, new_op, fd)?;
        // fd->opSetOpcode(newOp,op->code());   -- SEAM(W6)
        let opc = fd.obank().get(op).expect("pullbackOp").code();
        fd.op_set_opcode(new_op, resolve_typeop(opc)?);
        // fd->opSetInput(newOp,invn,0);
        fd.op_set_input(new_op, invn, 0)?;
        // for(i=1;i<op->numInput();++i) fd->opSetInput(newOp,op->getIn(i),i);
        for i in 1..numinput {
            let ini = fd.obank().get(op).expect("pullbackOp").get_in(i).expect("pullbackOp: null in");
            fd.op_set_input(new_op, ini, i)?;
        }
        // fd->opInsertEnd(newOp, bl);
        fd.op_insert_end(new_op, bl);
        // pullback[inbranch] = outVn;
        self.pullback[inbranch as usize] = Some(out_vn);
        Ok(out_vn)
    }

    /// Create a MULTIEQUAL in the given block that will hold data-flow from the
    /// given PcodeOp (C++ `ConditionalExecution::getNewMulti`, `condexe.cc:198-217`).
    ///
    /// SEAM(W6/opSetOutput): `newUniqueOut`→`opSetOutput` and `opSetOpcode`→W6
    /// surface the documented seam `Err`; the op-shell + input wiring + insert are
    /// reproduced.
    fn get_new_multi(&mut self, op: OpId, bl: BlockId, fd: &mut Funcdata) -> KunaResult<VarnodeId> {
        // PcodeOp *newop = fd->newOp(bl->sizeIn(),bl->getStart());
        let sizein = fd.bblocks_ref().block(bl).size_in();
        let start = crate::block::block_get_start(&fd.bblocks_ref().arena, bl);
        let newop = fd.new_op(sizein, start);
        // Varnode *outvn = op->getOut();
        let outvn = fd.obank().get(op).expect("getNewMulti: stale op").get_out().expect("getNewMulti: op has no out");
        let outsize = fd.vbank().get(outvn).expect("getNewMulti: stale out").get_size();
        // Using the original outvn address may cause merge conflicts (see C++ comment);
        // newoutvn = fd->newUniqueOut(outvn->getSize(),newop);
        let newoutvn = self.new_unique_out(outsize, newop, fd)?;
        // fd->opSetOpcode(newop,CPUI_MULTIEQUAL);   -- SEAM(W6)
        fd.op_set_opcode(newop, resolve_typeop(OpCode::CPUI_MULTIEQUAL)?);
        // We create NEW references to outvn; for(i=0;i<bl->sizeIn();++i) fd->opSetInput(newop,outvn,i);
        for i in 0..sizein {
            fd.op_set_input(newop, outvn, i)?;
        }
        // fd->opInsertBegin(newop,bl);
        fd.op_insert_begin(newop, bl);
        Ok(newoutvn)
    }

    /// `newVarnodeOut(s,m,op)` — create a defined output Varnode for `op` (C++
    /// `Funcdata::newVarnodeOut`, `funcdata_varnode.cc:106`).
    ///
    /// SEAM(opSetOutput, LOSS-035/036): `newVarnodeOut` is `newVarnode(s,m,ct)`
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
        fd.op_set_output(op, vn)?; // SEAM(opSetOutput)
        Ok(vn)
    }

    /// `newUniqueOut(s,op)` — create a defined unique output Varnode for `op`
    /// (C++ `Funcdata::newUniqueOut`, `funcdata_varnode.cc:131`).  Same seam as
    /// [`new_varnode_out`](ConditionalExecution::new_varnode_out).
    fn new_unique_out(&self, s: int4, op: OpId, fd: &mut Funcdata) -> KunaResult<VarnodeId> {
        let vn = fd.new_unique(s, None);
        fd.op_set_output(op, vn)?; // SEAM(opSetOutput)
        Ok(vn)
    }

    /// Calculate the replacement Varnode for a read coming through an arbitrary
    /// block (C++ `ConditionalExecution::resolveRead`, `condexe.cc:224-237`).
    fn resolve_read(&mut self, op: OpId, bl: BlockId, fd: &mut Funcdata) -> KunaResult<VarnodeId> {
        // if (bl->sizeIn()==1) { ... } else res = getNewMulti(op,bl);
        if fd.bblocks_ref().block(bl).size_in() == 1 {
            // Since dominator is iblock, In(0) must be iblock. Figure what side we came through.
            // int4 slot = (bl->getInRevIndex(0)==posta_outslot)?camethruposta_slot:1-camethruposta_slot;
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
        // if (op->code() == CPUI_COPY) { ... }
        if fd.obank().get(op).expect("resolveIblockRead: stale op").code() == OpCode::CPUI_COPY {
            let vn = fd.obank().get(op).expect("resolveIblockRead").get_in(0).expect("resolveIblockRead: null in");
            if fd.vbank().get(vn).expect("resolveIblockRead: stale vn").is_written() {
                let def_op = fd.vbank().get(vn).expect("resolveIblockRead").get_def().expect("resolveIblockRead: written no def");
                let def_ref = fd.obank().get(def_op).expect("resolveIblockRead: stale defOp");
                // if (defOp->code()==CPUI_MULTIEQUAL && defOp->getParent()==iblock) op = defOp;
                if def_ref.code() == OpCode::CPUI_MULTIEQUAL && def_ref.get_parent() == Some(iblock) {
                    op = def_op;
                }
            } else {
                return Ok(vn);
            }
        }
        let opc = fd.obank().get(op).expect("resolveIblockRead").code();
        if opc == OpCode::CPUI_MULTIEQUAL {
            // return op->getIn(inbranch);
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
        // BlockBasic *bl = readop->getParent();
        let bl = fd.obank().get(readop).expect("getMultiequalRead: stale readop").get_parent().expect("getMultiequalRead: readop no parent");
        // BlockBasic *inbl = (BlockBasic *)bl->getIn(slot);
        let inbl = fd.bblocks_ref().block(bl).get_in(slot);
        // if (inbl != iblock) return getReplacementRead(op, inbl);
        if inbl != iblock {
            return self.get_replacement_read(op, inbl, fd);
        }
        // int4 s = (bl->getInRevIndex(slot)==posta_outslot)?camethruposta_slot:1-camethruposta_slot;
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
        // iter = replacement.find(bl->getIndex()); if (iter != end) return iter->second;
        let bl_index = fd.bblocks_ref().block(bl).get_index();
        if let Some(&res) = self.replacement.get(&bl_index) {
            return Ok(res);
        }
        // BlockBasic *curbl = bl; // Flow must eventually come through iblock
        let mut curbl = bl;
        // while(curbl->getImmedDom() != iblock) { curbl = curbl->getImmedDom(); if (curbl==0) throw; }
        while fd.bblocks_ref().block(curbl).get_immed_dom() != Some(iblock) {
            curbl = match fd.bblocks_ref().block(curbl).get_immed_dom() {
                Some(d) => d,
                None => return Err(KunaError::lowlevel("Conditional execution: Could not find dominator")),
            };
        }
        // iter = replacement.find(curbl->getIndex());
        let curbl_index = fd.bblocks_ref().block(curbl).get_index();
        if let Some(&res) = self.replacement.get(&curbl_index) {
            // replacement[bl->getIndex()] = iter->second; return iter->second;
            self.replacement.insert(bl_index, res);
            return Ok(res);
        }
        // Varnode *res = resolveRead(op,curbl);
        let res = self.resolve_read(op, curbl, fd)?;
        // replacement[curbl->getIndex()] = res;
        self.replacement.insert(curbl_index, res);
        // if (curbl != bl) replacement[bl->getIndex()] = res;
        if curbl != bl {
            self.replacement.insert(bl_index, res);
        }
        Ok(res)
    }

    /// Replace the data-flow for the given PcodeOp in \b iblock (C++
    /// `ConditionalExecution::doReplacement`, `condexe.cc:320-356`).
    ///
    /// SEAM(opSetOutput / W6): the RETURN-input COPY creation and any pullback
    /// MULTIEQUAL creation surface the documented seam `Err`s through the helpers.
    fn do_replacement(&mut self, op: OpId, fd: &mut Funcdata) -> KunaResult<()> {
        self.replacement.clear();
        self.pullback.clear();
        let iblock = self.iblock.expect("doReplacement: no iblock");
        // Varnode *vn = op->getOut();
        let vn = fd.obank().get(op).expect("doReplacement: stale op").get_out().expect("doReplacement: op has no out");
        // iter = vn->beginDescend(); while(iter != endDescend()) { ... iter = vn->beginDescend(); }
        loop {
            // The C++ re-fetches beginDescend() each iteration because the body
            // destroys the *current* descendant link; take the first descendant.
            let readop = match fd.vbank().get(vn).expect("doReplacement: stale vn").descend_iter().next() {
                Some(o) => o,
                None => break,
            };
            // int4 slot = readop->getSlot(vn); BlockBasic *bl = readop->getParent();
            let mut slot = fd.obank().get(readop).expect("doReplacement: stale readop").get_slot(vn);
            let bl = fd.obank().get(readop).expect("doReplacement").get_parent().expect("doReplacement: readop no parent");
            let mut readop = readop;
            // if (bl == iblock) { fd->opUnsetInput(readop,slot); }
            if bl == iblock {
                fd.op_unset_input(readop, slot);
            } else {
                let rvn: VarnodeId;
                let code = fd.obank().get(readop).expect("doReplacement").code();
                if code == OpCode::CPUI_MULTIEQUAL {
                    rvn = self.get_multiequal_read(op, readop, slot, fd)?;
                } else if code == OpCode::CPUI_RETURN && slot > 0 {
                    // Cannot replace input of RETURN directly, create COPY to hold input.
                    // PcodeOp *newcopyop = fd->newOp(1,readop->getAddr());
                    let raddr = fd.obank().get(readop).expect("doReplacement").get_addr().clone();
                    let newcopyop = fd.new_op(1, raddr);
                    // fd->opSetOpcode(newcopyop,CPUI_COPY);   -- SEAM(W6)
                    fd.op_set_opcode(newcopyop, resolve_typeop(OpCode::CPUI_COPY)?);
                    // Varnode *outvn = fd->newVarnodeOut(vn->getSize(),vn->getAddr(),newcopyop); // Preserve storage
                    let vsize = fd.vbank().get(vn).expect("doReplacement: stale vn").get_size();
                    let vaddr = fd.vbank().get(vn).expect("doReplacement").get_addr().clone();
                    let outvn = self.new_varnode_out(vsize, &vaddr, newcopyop, fd)?;
                    // fd->opSetInput(readop,outvn,slot);
                    fd.op_set_input(readop, outvn, slot)?;
                    // fd->opInsertBefore(newcopyop,readop);
                    fd.op_insert_before(newcopyop, readop);
                    // readop = newcopyop; slot = 0;
                    readop = newcopyop;
                    slot = 0;
                    // rvn = getReplacementRead(op,bl);  // COPY gets the replacement read
                    rvn = self.get_replacement_read(op, bl, fd)?;
                } else {
                    rvn = self.get_replacement_read(op, bl, fd)?;
                }
                // fd->opSetInput(readop,rvn,slot);
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
            // for(iter : vn->descend) if (!testMultiRead(vn,readop)) return false;
            let readers: Vec<OpId> = fd.vbank().get(vn).expect("testRemovability: stale vn").descend_iter().collect();
            for readop in readers {
                if !self.test_multi_read(vn, readop, fd) {
                    return false;
                }
            }
        } else {
            // if (op->isFlowBreak() || op->isCall()) return false;
            if o.is_flow_break() || o.is_call() {
                return false;
            }
            // if ((op->code()==CPUI_LOAD)||(op->code()==CPUI_STORE)) return false;
            if o.code() == OpCode::CPUI_LOAD || o.code() == OpCode::CPUI_STORE {
                return false;
            }
            // if (op->code()==CPUI_INDIRECT) return false;
            if o.code() == OpCode::CPUI_INDIRECT {
                return false;
            }
            // vn = op->getOut();
            let vn_opt = o.get_out();
            // if (vn->isAddrTied()) return false;
            // NOTE (C++): the deref `vn->isAddrTied()` happens before the `vn != 0`
            // guard in the source; faithful ports keep the same evaluation but the
            // C++ relies on op outputs being non-null here.  We guard defensively:
            if let Some(vn) = vn_opt {
                if fd.vbank().get(vn).expect("testRemovability: stale vn").is_addr_tied() {
                    return false;
                }
                // if (vn != 0) { bool hasnodescend = true; for(...) {...} if (hasnodescend && !heritageyes[..]) return false; }
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
        // iblock2posta_true = (posta_outslot == 1);
        self.iblock2posta_true = self.posta_outslot == 1;
        // camethruposta_slot = (init2a_true==iblock2posta_true)?prea_inslot:1-prea_inslot;
        self.camethruposta_slot = if self.init2a_true == self.iblock2posta_true {
            self.prea_inslot
        } else {
            1 - self.prea_inslot
        };
        // posta_block = (BlockBasic *)iblock->getOut(posta_outslot);
        self.posta_block = Some(fd.bblocks_ref().block(iblock).get_out(self.posta_outslot));
        // postb_block = (BlockBasic *)iblock->getOut(1-posta_outslot);
        self.postb_block = Some(fd.bblocks_ref().block(iblock).get_out(1 - self.posta_outslot));

        // iter = iblock->endOp(); if (iter != iblock->beginOp()) --iter; // Skip branch
        // while(iter != iblock->beginOp()) { --iter; if (!testRemovability(*iter)) return false; }
        //
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
    /// returned \b true.  SEAM(opSetOutput / W6): the data-flow rewrite
    /// (`doReplacement`) surfaces the documented seam `Err`s.
    pub fn execute(&mut self, fd: &mut Funcdata) -> KunaResult<()> {
        let iblock = self.iblock.expect("execute: no iblock");
        // iter = iblock->endOp(); --iter;  // Remove ops in reverse order
        let mut iter = fd.bb_op_tail(iblock);
        loop {
            let op = iter.expect("execute: empty iblock (verify guarantees a branch)");
            // notdone = iter != iblock->beginOp();
            let prev = fd.op_previous_op(op);
            let notdone = prev.is_some();
            // if (notdone) --iter;
            if notdone {
                iter = prev;
            }
            // if (!op->isBranch()) doReplacement(op);
            if !fd.obank().get(op).expect("execute: stale op").is_branch() {
                self.do_replacement(op, fd)?; // Remove all read refs of op
            }
            // fd->opDestroy(op);  // Then destroy op
            fd.op_destroy(op);
            if !notdone {
                break;
            }
        }
        // fd->removeFromFlowSplit(iblock,(posta_outslot != camethruposta_slot));
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
    /// Constructor (C++ `ActionConditionalExe(const string &g)`): group `g`,
    /// flags 0, name `"conditionalexe"`.
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
        // if (!grouplist.contains(getGroup())) return 0;
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
    /// SEAM(opSetOutput / W6): a successful trial whose `condexe.execute()` then
    /// surfaces the documented seam `Err` cannot abort an `apply` that returns a
    /// bare `int4`; such an execute degrades to "no change made this trial" (the
    /// hit is not counted and the round does not loop on it).  This is the only
    /// faithful adaptation possible until `opSetOutput` lands; recorded as a loss.
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // if (data.hasUnreachableBlocks()) return 0;
        if data.has_unreachable_blocks() {
            // Conditional execution elimination logic may not work with unreachable blocks
            return 0;
        }
        let mut numhits: int4 = 0;
        let mut condexe = ConditionalExecution::new(data);
        // do { changethisround=false; for(i...) { bb=getBlock(i); if (trial(bb)) { execute(); ...} } } while(...)
        loop {
            let mut changethisround = false;
            // const BlockGraph &bblocks(data.getBasicBlocks());  for(i=0;i<bblocks.getSize();++i)
            //
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
                    // condexe.execute() — SEAM(opSetOutput): a seam Err degrades to
                    // "no change this trial" (cannot propagate through `int4`).
                    if condexe.execute(data).is_ok() {
                        numhits += 1; // Adjust dataflow succeeded
                        changethisround = true;
                    }
                }
                i += 1; // ++i
            }
            if !changethisround {
                break;
            }
        }
        self.base.count += numhits; // count += numhits;
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
        // if (!vn->isWritten()) return false;
        if !fd.vbank().get(vn).expect("discoverZeroSlot: stale vn").is_written() {
            return false;
        }
        // op = vn->getDef();
        let op = fd.vbank().get(vn).expect("discoverZeroSlot").get_def().expect("discoverZeroSlot: written no def");
        self.op = Some(op);
        // if (op->code() != CPUI_MULTIEQUAL) return false;
        if fd.obank().get(op).expect("discoverZeroSlot: stale op").code() != OpCode::CPUI_MULTIEQUAL {
            return false;
        }
        // if (op->numInput() != 2) return false;
        if fd.obank().get(op).expect("discoverZeroSlot").num_input() != 2 {
            return false;
        }
        // for(zeroSlot=0;zeroSlot<2;++zeroSlot) { ... }
        let mut zs = 0;
        while zs < 2 {
            self.zero_slot = zs;
            // Varnode *tmpvn = op->getIn(zeroSlot);
            let tmpvn = fd.obank().get(op).expect("discoverZeroSlot").get_in(zs).expect("discoverZeroSlot: null in");
            // if (!tmpvn->isWritten()) continue;
            if !fd.vbank().get(tmpvn).expect("discoverZeroSlot: stale vn").is_written() {
                zs += 1;
                continue;
            }
            // PcodeOp *copyop = tmpvn->getDef();
            let copyop = fd.vbank().get(tmpvn).expect("discoverZeroSlot").get_def().expect("discoverZeroSlot: written no def");
            // if (copyop->code() != CPUI_COPY) continue;  // Multiequal must have CPUI_COPY input
            if fd.obank().get(copyop).expect("discoverZeroSlot: stale copyop").code() != OpCode::CPUI_COPY {
                zs += 1;
                continue;
            }
            // Varnode *zerovn = copyop->getIn(0);
            let zerovn = fd.obank().get(copyop).expect("discoverZeroSlot").get_in(0).expect("discoverZeroSlot: null in");
            // if (!zerovn->isConstant()) continue;
            if !fd.vbank().get(zerovn).expect("discoverZeroSlot: stale vn").is_constant() {
                zs += 1;
                continue;
            }
            // if (zerovn->getOffset() != 0) continue;  // which copies #0
            if fd.vbank().get(zerovn).expect("discoverZeroSlot").get_offset() != 0 {
                zs += 1;
                continue;
            }
            // otherVn = op->getIn(1-zeroSlot);  // store off varnode from other path
            let other_vn = fd.obank().get(op).expect("discoverZeroSlot").get_in(1 - zs).expect("discoverZeroSlot: null in");
            self.other_vn = Some(other_vn);
            // if (otherVn->isFree()) return false;
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
        // const FlowBlock *baseBlock = op->getParent();
        let base_block = fd.obank().get(op).expect("discoverCbranch: stale op").get_parent().expect("discoverCbranch: op no parent");
        // zeroBlock = baseBlock->getIn(zeroSlot);
        let zero_block = fd.bblocks_ref().block(base_block).get_in(self.zero_slot);
        self.zero_block = Some(zero_block);
        // const FlowBlock *otherBlock = baseBlock->getIn(1-zeroSlot);
        let other_block = fd.bblocks_ref().block(base_block).get_in(1 - self.zero_slot);
        // if (zeroBlock->sizeOut() == 1) { if (zeroBlock->sizeIn()!=1) return false; condBlock = zeroBlock->getIn(0); }
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
        // if (condBlock->sizeOut() != 2) return false;
        if fd.bblocks_ref().block(cond_block).size_out() != 2 {
            return false;
        }
        // if (otherBlock->sizeOut() == 1) { if (otherBlock->sizeIn()!=1) return false; if (condBlock!=otherBlock->getIn(0)) return false; }
        let ob_sizeout = fd.bblocks_ref().block(other_block).size_out();
        if ob_sizeout == 1 {
            if fd.bblocks_ref().block(other_block).size_in() != 1 {
                return false;
            }
            if cond_block != fd.bblocks_ref().block(other_block).get_in(0) {
                return false;
            }
        } else if ob_sizeout == 2 {
            // if (condBlock != otherBlock) return false;
            if cond_block != other_block {
                return false;
            }
        } else {
            return false;
        }
        // cbranch = condBlock->lastOp(); if (cbranch==0) return false;
        let cbranch = match fd.bb_op_tail(cond_block) {
            Some(o) => o,
            None => return false,
        };
        self.cbranch = Some(cbranch);
        // if (cbranch->code() != CPUI_CBRANCH) return false;
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
        // if (condBlock->getTrueOut() == zeroBlock) zeroPathIsTrue = true;
        if fd.bblocks_ref().block(cond_block).get_true_out() == zero_block {
            self.zero_path_is_true = true;
        } else if fd.bblocks_ref().block(cond_block).get_false_out() == zero_block {
            // else if (condBlock->getFalseOut() == zeroBlock) zeroPathIsTrue = false;
            self.zero_path_is_true = false;
        } else {
            // else { condBlock must be zeroBlock; zeroPathIsTrue = (getTrueOut()==op->getParent()); }
            let op = self.op.expect("discoverPathIsTrue: no op");
            let parent = fd.obank().get(op).expect("discoverPathIsTrue: stale op").get_parent().expect("discoverPathIsTrue: op no parent");
            self.zero_path_is_true = fd.bblocks_ref().block(cond_block).get_true_out() == parent;
        }
    }

    /// Verify the CBRANCH boolean expression is (vn==0) or (vn!=0) (C++
    /// `MultiPredicate::discoverConditionalZero`, `condexe.cc:589-614`).
    fn discover_conditional_zero(&mut self, vn: VarnodeId, fd: &Funcdata) -> bool {
        let cbranch = self.cbranch.expect("discoverConditionalZero: no cbranch");
        // Varnode *boolvn = cbranch->getIn(1);
        let boolvn = fd.obank().get(cbranch).expect("discoverConditionalZero: stale cbranch").get_in(1).expect("discoverConditionalZero: null in");
        // if (!boolvn->isWritten()) return false;
        if !fd.vbank().get(boolvn).expect("discoverConditionalZero: stale vn").is_written() {
            return false;
        }
        // PcodeOp *compareop = boolvn->getDef(); OpCode opc = compareop->code();
        let compareop = fd.vbank().get(boolvn).expect("discoverConditionalZero").get_def().expect("discoverConditionalZero: written no def");
        let opc = fd.obank().get(compareop).expect("discoverConditionalZero: stale compareop").code();
        // if (opc == CPUI_INT_NOTEQUAL) zeroPathIsTrue = !zeroPathIsTrue;
        if opc == OpCode::CPUI_INT_NOTEQUAL {
            self.zero_path_is_true = !self.zero_path_is_true;
        } else if opc != OpCode::CPUI_INT_EQUAL {
            // else if (opc != CPUI_INT_EQUAL) return false;
            return false;
        }
        // Varnode *a1 = compareop->getIn(0); Varnode *a2 = compareop->getIn(1);
        let a1 = fd.obank().get(compareop).expect("discoverConditionalZero").get_in(0).expect("discoverConditionalZero: null in");
        let a2 = fd.obank().get(compareop).expect("discoverConditionalZero").get_in(1).expect("discoverConditionalZero: null in");
        // if (a1 == vn) zerovn = a2; else if (a2 == vn) zerovn = a1; else return false;
        let zerovn = if a1 == vn {
            a2
        } else if a2 == vn {
            a1
        } else {
            return false;
        };
        // if (!zerovn->isConstant()) return false;
        if !fd.vbank().get(zerovn).expect("discoverConditionalZero: stale vn").is_constant() {
            return false;
        }
        // if (zerovn->getOffset() != 0) return false;  // Verify we are comparing to zero
        if fd.vbank().get(zerovn).expect("discoverConditionalZero").get_offset() != 0 {
            return false;
        }
        // if (cbranch->isBooleanFlip()) zeroPathIsTrue = !zeroPathIsTrue;
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
    /// Constructor (C++ `RuleOrPredicate(const string &g)`): `Rule(g,0,"orpredicate")`.
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
    /// SEAM(opSetOutput / W6): `data.opSetOpcode(op,CPUI_COPY)` surfaces the W6
    /// seam; the input rewiring (`opSetInput`/`opRemoveInput`) is reproduced.
    fn check_single(
        &self,
        vn: VarnodeId,
        branch: &mut MultiPredicate,
        op: OpId,
        data: &mut Funcdata,
    ) -> KunaResult<int4> {
        // if (vn->isFree()) return 0;
        if data.vbank().get(vn).expect("checkSingle: stale vn").is_free() {
            return Ok(0);
        }
        // if (!branch.discoverCbranch()) return 0;
        if !branch.discover_cbranch(data) {
            return Ok(0);
        }
        // if (branch.op->getOut()->loneDescend() != op) return 0;  // Must only be one use of MULTIEQUAL
        let branch_op = branch.op.expect("checkSingle: branch has no op");
        let branch_out = data.obank().get(branch_op).expect("checkSingle: stale branch op").get_out().expect("checkSingle: branch op no out");
        if data.lone_descend(branch_out) != Some(op) {
            return Ok(0);
        }
        // branch.discoverPathIsTrue();
        branch.discover_path_is_true(data);
        // if (!branch.discoverConditionalZero(vn)) return 0;
        if !branch.discover_conditional_zero(vn, data) {
            return Ok(0);
        }
        // if (branch.zeroPathIsTrue) return 0;  // true condition (vn==0) must not go to zero set
        if branch.zero_path_is_true {
            return Ok(0);
        }
        // data.opSetInput(branch.op,vn,branch.zeroSlot);
        data.op_set_input(branch_op, vn, branch.zero_slot)?;
        // data.opRemoveInput(op,1);
        data.op_remove_input(op, 1);
        // data.opSetOpcode(op,CPUI_COPY);   -- SEAM(W6)
        data.op_set_opcode(op, resolve_typeop(OpCode::CPUI_COPY)?);
        // data.opSetInput(op,branch.op->getOut(),0);
        data.op_set_input(op, branch_out, 0)?;
        Ok(1)
    }
}

impl Rule for RuleOrPredicate {
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Note: the C++ returns int4; the seam `Err` paths cannot return through
        // an `int4`, so a seam error degrades to "no change" (0) here and is
        // reported by the helper's `Err`.  The applyOp structure is otherwise
        // faithful.  See SEAM notes.
        self.apply_op_inner(op, data).unwrap_or(0)
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_INT_OR); oplist.push_back(CPUI_INT_XOR);
        vec![OpCode::CPUI_INT_OR, OpCode::CPUI_INT_XOR]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // if (!grouplist.contains(getGroup())) return 0;
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
    /// SEAM(opSetOutput / W6): `newOp`+`newUniqueOut`+`opSetOpcode(MULTIEQUAL)` for
    /// the new MULTIEQUAL, and the final `opSetOpcode(op,CPUI_COPY)`, surface the
    /// documented seam `Err`s.
    fn apply_op_inner(&self, op: OpId, data: &mut Funcdata) -> KunaResult<int4> {
        let mut branch0 = MultiPredicate::new();
        let mut branch1 = MultiPredicate::new();
        // bool test0 = branch0.discoverZeroSlot(op->getIn(0));
        let op_in0 = data.obank().get(op).expect("applyOp: stale op").get_in(0).expect("applyOp: null in");
        let op_in1 = data.obank().get(op).expect("applyOp").get_in(1).expect("applyOp: null in");
        let test0 = branch0.discover_zero_slot(op_in0, data);
        let test1 = branch1.discover_zero_slot(op_in1, data);
        // if ((test0==false) && (test1==false)) return 0;
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
        // if (!branch0.discoverCbranch()) return 0;
        if !branch0.discover_cbranch(data) {
            return Ok(0);
        }
        // if (!branch1.discoverCbranch()) return 0;
        if !branch1.discover_cbranch(data) {
            return Ok(0);
        }
        let cond0 = branch0.cond_block;
        let cond1 = branch1.cond_block;
        if cond0 == cond1 {
            // if (branch0.zeroBlock == branch1.zeroBlock) return 0;  // zero sets must be along different paths
            if branch0.zero_block == branch1.zero_block {
                return Ok(0);
            }
        } else {
            // Make sure cbranches have shared condition and the different zero sets have complementary paths
            let mut condmarker = BooleanExpressionMatch::new();
            let b0cb = branch0.cbranch.expect("applyOp: branch0 no cbranch");
            let b1cb = branch1.cbranch.expect("applyOp: branch1 no cbranch");
            // if (!condmarker.verifyCondition(branch0.cbranch,branch1.cbranch)) return 0;
            if !condmarker.verify_condition(b0cb, b1cb, data.vbank(), data.obank()) {
                return Ok(0);
            }
            // if (condmarker.getMultiSlot() != -1) return 0;
            if condmarker.get_multi_slot() != -1 {
                return Ok(0);
            }
            // branch0.discoverPathIsTrue(); branch1.discoverPathIsTrue();
            branch0.discover_path_is_true(data);
            branch1.discover_path_is_true(data);
            // bool finalBool = branch0.zeroPathIsTrue == branch1.zeroPathIsTrue;
            let mut final_bool = branch0.zero_path_is_true == branch1.zero_path_is_true;
            // if (condmarker.getFlip()) finalBool = !finalBool;
            if condmarker.get_flip() {
                final_bool = !final_bool;
            }
            // if (finalBool) return 0;  // One path hits both zero sets
            if final_bool {
                return Ok(0);
            }
        }
        let branch0_op = branch0.op.expect("applyOp: branch0 no op");
        let branch1_op = branch1.op.expect("applyOp: branch1 no op");
        // int4 order = branch0.op->compareOrder(branch1.op); if (order == 0) return 0;
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
        // PcodeOp *newMulti = data.newOp(2,finalBlock->getStart());
        let start = crate::block::block_get_start(&data.bblocks_ref().arena, final_block);
        let new_multi = data.new_op(2, start);
        // data.opSetOpcode(newMulti,CPUI_MULTIEQUAL);   -- SEAM(W6)
        data.op_set_opcode(new_multi, resolve_typeop(OpCode::CPUI_MULTIEQUAL)?);
        let b0_other = branch0.other_vn.expect("applyOp: branch0 no otherVn");
        let b1_other = branch1.other_vn.expect("applyOp: branch1 no otherVn");
        if slot0_sets_branch0 {
            // data.opSetInput(newMulti,branch0.otherVn,0); data.opSetInput(newMulti,branch1.otherVn,1);
            data.op_set_input(new_multi, b0_other, 0)?;
            data.op_set_input(new_multi, b1_other, 1)?;
        } else {
            // data.opSetInput(newMulti,branch1.otherVn,0); data.opSetInput(newMulti,branch0.otherVn,1);
            data.op_set_input(new_multi, b1_other, 0)?;
            data.op_set_input(new_multi, b0_other, 1)?;
        }
        // Varnode *newvn = data.newUniqueOut(branch0.otherVn->getSize(),newMulti);
        let other_size = data.vbank().get(b0_other).expect("applyOp: stale otherVn").get_size();
        let newvn = self.new_unique_out_data(other_size, new_multi, data)?;
        // data.opInsertBegin(newMulti,finalBlock);
        data.op_insert_begin(new_multi, final_block);
        // data.opRemoveInput(op,1);
        data.op_remove_input(op, 1);
        // data.opSetInput(op,newvn,0);
        data.op_set_input(op, newvn, 0)?;
        // data.opSetOpcode(op,CPUI_COPY);   -- SEAM(W6)
        data.op_set_opcode(op, resolve_typeop(OpCode::CPUI_COPY)?);
        Ok(1)
    }

    /// `newUniqueOut(s,op)` for the rule's data-mutation context (same seam as
    /// [`ConditionalExecution::new_unique_out`]).
    fn new_unique_out_data(&self, s: int4, op: OpId, data: &mut Funcdata) -> KunaResult<VarnodeId> {
        let vn = data.new_unique(s, None);
        data.op_set_output(op, vn)?; // SEAM(opSetOutput)
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
