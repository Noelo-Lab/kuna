//! Port of `decompiler/cpp/emulateutil.hh` + `emulateutil.cc` (W2, item
//! `w2-sleigh-emulate`): (lightweight) emulation for executing snippets
//! defined with PcodeOpRaw objects.
//!
//! Scope note: the C++ file holds two classes.
//!
//! - **`EmulateSnippet` is ported here.**  Its `Architecture *glb` member is
//!   used for exactly two things — `glb->loader` (reading initial values
//!   out of the load image) and, through `getSpaceFromConst()`, the space
//!   table — so the port stores those two slices directly
//!   (`Rc<RefCell<dyn LoadImage>>` + `Rc<AddrSpaceManager>`, the same
//!   substitution pattern as `FloatFormatProvider` in kuna-num and the
//!   manager boundary in `emulate.rs`).  The C++ `getArch()` accessor has no
//!   meaning without `Architecture` and is not ported.
//! - **`EmulatePcodeOp` is NOT yet ported.**  It emulates over the syntax
//!   tree's `PcodeOp`/`Varnode`/`FlowBlock` objects (`op.hh`,
//!   `varnode.hh`, `block.hh`) and `glb->userops`, none of which exist
//!   until the kuna-decomp IR wave lands (ADR 0001 arenas: its methods
//!   will take Funcdata-resident IDs, so porting it now would invent the
//!   IR API ahead of that wave).  It must be added when `op.rs` exists.
//!
//! Other notes:
//!
//! - `tempValues` (`map<uintb,uintb>`) is a `BTreeMap<u64, u64>` (ADR 0002).
//! - `opList`/`varList`: ops are `Rc<PcodeOpRaw>` and the separate varnode
//!   list disappears (the Rust `PcodeOpRaw` owns its varnodes by value; see
//!   `kuna_num::pcoderaw` and `emulate.rs` module docs), so `buildEmitter`
//!   returns a [`PcodeEmitCache`] borrowing only the op list.
//! - `getLoadImageValue` reads a full `sizeof(uintb)` = 8 bytes through
//!   `(uint1 *)&res`; the port pins `HOST_ENDIAN` to 0 (little-endian
//!   oracle host) as in `memstate.rs`, making the read + conditional
//!   byte_swap equal to `construct_value(buf, 8, space-is-big-endian)`.
//!   Unlike `MemoryImage`, a `DataUnavailError` is *not* caught here — it
//!   propagates, as in C++.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager};
use kuna_base::types::Wrap;
use kuna_num::opbehavior::OpBehavior;
use kuna_num::opcodes::{get_opname, OpCode};
use kuna_num::pcoderaw::{PcodeOpRaw, VarnodeData};

use crate::emulate::{Emulate, EmulateCore, PcodeEmitCache};
use crate::loadimage::LoadImage;
use crate::memstate::construct_value;

/// \brief Emulate a \e snippet of PcodeOps out of a functional context
///
/// Emulation is performed on a short sequence (\b snippet) of PcodeOpRaw
/// objects.  Control-flow emulation is limited to this snippet; BRANCH and
/// CBRANCH operations can happen using p-code relative branching.  Executing
/// BRANCHIND, CALL, CALLIND, CALLOTHER, STORE, MULTIEQUAL, INDIRECT,
/// SEGMENTOP, CPOOLOP, and NEW ops is treated as illegal and an error is
/// returned.  Expressions can only use temporary registers or read from the
/// LoadImage.
///
/// The set of PcodeOpRaw objects in the snippet is provided by emitting
/// p-code to the object returned by build_emitter().  This is designed for
/// one-time initialization of this struct, which can be repeatedly used by
/// calling reset_memory() between executions.
pub struct EmulateSnippet {
    /// C++ `Emulate` members
    core: EmulateCore,
    /// The load image of the program being emulated (the slice of the C++
    /// `Architecture *glb` this class reads; module docs)
    loader: Rc<RefCell<dyn LoadImage>>,
    /// The space table (the other slice of `glb`; resolves `<spaceid>`
    /// constants — module docs)
    manage: Rc<AddrSpaceManager>,
    /// Sequence of p-code ops to be executed
    op_list: Vec<Rc<PcodeOpRaw>>,
    /// Values stored in temporary registers
    temp_values: BTreeMap<u64, u64>,
    /// Current p-code op being executed (C++ `PcodeOpRaw *currentOp`)
    current_op: Option<Rc<PcodeOpRaw>>,
    /// Index of current p-code op being executed
    pos: i32,
}

impl EmulateSnippet {
    /// The C++ constructor takes an `Architecture *g`; the port takes only the
    /// two slices of it this class uses (module docs).
    pub fn new(loader: Rc<RefCell<dyn LoadImage>>, manage: Rc<AddrSpaceManager>) -> Self {
        EmulateSnippet {
            core: EmulateCore::new(),
            loader,
            manage,
            op_list: Vec::new(),
            temp_values: BTreeMap::new(),
            current_op: None,
            pos: 0,
        }
    }

    /// \brief Pull a value from the load-image given a specific address
    ///
    /// A contiguous chunk of memory is pulled from the load-image and
    /// returned as a constant value, respecting the endianness of the
    /// address space.
    /// \param spc is the address space to pull the value from
    /// \param off is the starting address offset (from within the space) to
    ///        pull the value from
    /// \param sz is the number of bytes to pull from memory
    /// \return indicated bytes arranged as a constant value
    fn get_load_image_value(&self, spc: &Rc<AddrSpace>, off: u64, sz: i32) -> KunaResult<u64> {
        // C++ loadFill((uint1 *)&res, sizeof(uintb), ...): a full 8-byte
        // read regardless of sz; (a DataUnavailError is NOT caught here)
        let mut buf = [0u8; 8];
        self.loader
            .borrow_mut()
            .load_fill(&mut buf, &Address::new(Rc::clone(spc), off))?;
        // host(LE)-order bytes + byte_swap when host and space endianness
        // differ == construct in space order (HOST_ENDIAN pinned to 0;
        // module docs)
        let mut res = construct_value(&buf, 8, spc.is_big_endian());
        // C++ `sz < sizeof(uintb)`: int4 vs size_t comparison (a negative
        // sz would wrap and compare false; in-contract sz >= 1)
        if spc.is_big_endian() && sz < 8 {
            res >>= (8 - sz) * 8; // shift < 64 for sz >= 1
        } else {
            res &= calc_mask(sz);
        }
        Ok(res)
    }

    /// The C++ `currentOp` dereference (null is C++ UB).
    fn current_op(&self) -> &Rc<PcodeOpRaw> {
        self.current_op
            .as_ref()
            .expect("EmulateSnippet: currentOp not set (C++ would dereference null)")
    }

    /// The throw shared by every illegal-in-a-snippet op.
    fn illegal_op_error(&self) -> KunaError {
        KunaError::lowlevel(format!(
            "Illegal p-code operation in snippet: {}",
            get_opname(self.current_op().get_opcode())
        ))
    }

    /// \brief Reset the emulation snippet
    ///
    /// Reset the memory state, and set the first p-code op as current.
    pub fn reset_memory(&mut self) {
        self.temp_values.clear();
        self.set_current_op(0);
        self.core.emu_halted = false;
    }

    /// \brief Provide the caller with an emitter for building the p-code
    /// snippet
    ///
    /// Any p-code produced by the PcodeEmit, when triggered by the caller,
    /// becomes part of the \e snippet that will get emulated by \b this.
    /// (C++ heap-allocates the emitter; the Rust emitter borrows the op
    /// list and is dropped after use.)
    /// \param inst is the \e opcode to \e behavior map the emitter will use
    /// \param uniq_reserve is the starting offset within the \e unique
    ///        address space for any temporary registers
    /// \return the newly constructed emitter
    pub fn build_emitter<'a>(
        &'a mut self,
        inst: &'a [Option<Rc<dyn OpBehavior>>],
        uniq_reserve: u64,
    ) -> PcodeEmitCache<'a> {
        PcodeEmitCache::new(&mut self.op_list, inst, uniq_reserve)
    }

    /// \brief Check for p-code that is deemed illegal for a \e snippet
    ///
    /// This method facilitates enforcement of the formal rules for snippet
    /// code.
    ///   - Branches must use p-code relative addressing.
    ///   - Snippets can only read/write from temporary registers
    ///   - Snippets cannot use BRANCHIND, CALL, CALLIND, CALLOTHER, STORE,
    ///     SEGMENTOP, CPOOLREF, NEW, MULTIEQUAL, or INDIRECT
    ///
    /// \return \b true if the current snippet is legal
    pub fn check_for_legal_code(&self) -> bool {
        for op in &self.op_list {
            let opc = op.get_opcode();
            if opc == OpCode::CPUI_BRANCHIND
                || opc == OpCode::CPUI_CALL
                || opc == OpCode::CPUI_CALLIND
                || opc == OpCode::CPUI_CALLOTHER
                || opc == OpCode::CPUI_STORE
                || opc == OpCode::CPUI_SEGMENTOP
                || opc == OpCode::CPUI_CPOOLREF
                || opc == OpCode::CPUI_NEW
                || opc == OpCode::CPUI_MULTIEQUAL
                || opc == OpCode::CPUI_INDIRECT
            {
                return false;
            }
            if opc == OpCode::CPUI_BRANCH {
                let vn = op.get_input(0);
                let spc = vn
                    .space
                    .as_ref()
                    .expect("checkForLegalCode: varnode with null space (C++ UB)");
                if spc.get_type() != spacetype::IPTR_CONSTANT {
                    // Only relative branching allowed
                    return false;
                }
            }
            if let Some(vn) = op.get_output() {
                let spc = vn
                    .space
                    .as_ref()
                    .expect("checkForLegalCode: varnode with null space (C++ UB)");
                if spc.get_type() != spacetype::IPTR_INTERNAL {
                    return false; // Can only write to temporaries
                }
            }
            let mut j: i32 = 0;
            while j < op.num_input() {
                let vn = op.get_input(j);
                let spc = vn
                    .space
                    .as_ref()
                    .expect("checkForLegalCode: varnode with null space (C++ UB)");
                if spc.get_type() == spacetype::IPTR_PROCESSOR {
                    return false; // Cannot read from normal registers
                }
                j += 1;
            }
        }
        true
    }

    /// \brief Set the current executing p-code op by index
    ///
    /// The i-th p-code op in the snippet sequence is set as the currently
    /// executing op.
    /// \param i is the index
    pub fn set_current_op(&mut self, i: i32) {
        self.pos = i;
        let op = Rc::clone(&self.op_list[i as usize]); // cast: int4 index, C++ UB when out of range
        self.core.current_behave = op.get_behavior().cloned();
        self.current_op = Some(op);
    }

    /// \brief Set a temporary register value in the machine state
    ///
    /// The temporary Varnode's storage offset is used as key into the
    /// machine state map.
    /// \param offset is the temporary storage offset
    /// \param val is the value to put into the machine state
    pub fn set_varnode_value(&mut self, offset: u64, val: u64) {
        self.temp_values.insert(offset, val);
    }

    /// \brief Retrieve the value of a Varnode from the current machine state
    ///
    /// If the Varnode is a temporary register, the storage offset is used to
    /// look up the value from the machine state cache. If the Varnode
    /// represents a RAM location, the value is pulled directly out of the
    /// load-image.  If the value does not exist, a "Read before write" error
    /// is returned.
    /// \param vn is the Varnode to read
    /// \return the retrieved value
    pub fn get_varnode_value(&self, vn: &VarnodeData) -> KunaResult<u64> {
        let spc = vn
            .space
            .as_ref()
            .expect("getVarnodeValue: varnode with null space (C++ UB)");
        if spc.get_type() == spacetype::IPTR_CONSTANT {
            return Ok(vn.offset);
        }
        if spc.get_type() == spacetype::IPTR_INTERNAL {
            return match self.temp_values.get(&vn.offset) {
                // We have seen this varnode before
                Some(val) => Ok(*val),
                None => Err(KunaError::lowlevel("Read before write in snippet emulation")),
            };
        }

        self.get_load_image_value(spc, vn.offset, vn.size as i32) // cast: uint4 size as C++ int4
    }

    /// \brief Retrieve a temporary register value directly
    ///
    /// This allows the user to obtain the final value of the snippet
    /// calculation, without having to have the Varnode object in hand.
    /// \param offset is the offset of the temporary register to retrieve
    /// \return the calculated value or 0 if the register was never written
    pub fn get_temp_value(&self, offset: u64) -> u64 {
        match self.temp_values.get(&offset) {
            None => 0,
            Some(val) => *val,
        }
    }
}

impl Emulate for EmulateSnippet {
    fn emulate_core(&self) -> &EmulateCore {
        &self.core
    }

    fn emulate_core_mut(&mut self) -> &mut EmulateCore {
        &mut self.core
    }

    fn execute_unary(&mut self) -> KunaResult<()> {
        let op = Rc::clone(self.current_op());
        let behave = Rc::clone(
            self.core
                .current_behave
                .as_ref()
                .expect("EmulateSnippet: currentBehave not set (C++ would dereference null)"),
        );
        let in1 = self.get_varnode_value(op.get_input(0))?;
        let out_vn = op.get_output().expect("executeUnary: op without output (C++ UB)");
        let out = behave.evaluate_unary(
            out_vn.size as i32,          // cast: uint4 size as the C++ int4 param
            op.get_input(0).size as i32, // cast: uint4 size as the C++ int4 param
            in1,
        )?;
        self.set_varnode_value(out_vn.offset, out);
        Ok(())
    }

    fn execute_binary(&mut self) -> KunaResult<()> {
        let op = Rc::clone(self.current_op());
        let behave = Rc::clone(
            self.core
                .current_behave
                .as_ref()
                .expect("EmulateSnippet: currentBehave not set (C++ would dereference null)"),
        );
        let in1 = self.get_varnode_value(op.get_input(0))?;
        let in2 = self.get_varnode_value(op.get_input(1))?;
        let out_vn = op.get_output().expect("executeBinary: op without output (C++ UB)");
        let out = behave.evaluate_binary(
            out_vn.size as i32,          // cast: uint4 size as the C++ int4 param
            op.get_input(0).size as i32, // cast: uint4 size as the C++ int4 param
            in1,
            in2,
        )?;
        self.set_varnode_value(out_vn.offset, out);
        Ok(())
    }

    fn execute_load(&mut self) -> KunaResult<()> {
        // op will be null, use current_op
        let op = Rc::clone(self.current_op());
        let off = self.get_varnode_value(op.get_input(1))?;
        let spc = op
            .get_input(0)
            .get_space_from_const(&self.manage)
            .expect("executeLoad: <spaceid> constant does not resolve (C++ pointer always valid)");
        let off = AddrSpace::address_to_byte(off, spc.get_word_size());
        let sz = op.get_output().expect("executeLoad: op without output (C++ UB)").size as i32; // cast: uint4 as int4
        let res = self.get_load_image_value(&spc, off, sz)?;
        let out_offset = op.get_output().expect("just checked").offset;
        self.set_varnode_value(out_offset, res);
        Ok(())
    }

    fn execute_store(&mut self) -> KunaResult<()> {
        Err(self.illegal_op_error())
    }

    fn execute_branch(&mut self) -> KunaResult<()> {
        let op = Rc::clone(self.current_op());
        let vn = op.get_input(0);
        let spc = vn
            .space
            .as_ref()
            .expect("executeBranch: varnode with null space (C++ UB)");
        if spc.get_type() != spacetype::IPTR_CONSTANT {
            return Err(KunaError::lowlevel(
                "Tried to emulate absolute branch in snippet code",
            ));
        }
        let rel = vn.offset as i32; // cast: C++ (int4)vn->offset (truncating)
        self.pos = self.pos.wadd(rel); // C++ int overflow is UB; wraps deterministically here
        // C++ `(pos < 0)||(pos>opList.size())`: the int4 vs size_t
        // comparison only runs after the explicit pos < 0 check
        if self.pos < 0 || (self.pos as usize) > self.op_list.len() {
            return Err(KunaError::lowlevel(
                "Relative branch out of bounds in snippet code",
            ));
        }
        if self.pos as usize == self.op_list.len() {
            // cast: int4 == size_t comparison, pos >= 0 here
            self.core.emu_halted = true;
            return Ok(());
        }
        self.set_current_op(self.pos);
        Ok(())
    }

    fn execute_cbranch(&mut self) -> KunaResult<bool> {
        // op will be null, use current_op
        let op = Rc::clone(self.current_op());
        let cond = self.get_varnode_value(op.get_input(1))?;
        // We must take into account the booleanflip bit with pcode from the
        // syntax tree (raw snippet ops have none)
        Ok(cond != 0)
    }

    fn execute_branchind(&mut self) -> KunaResult<()> {
        Err(self.illegal_op_error())
    }

    fn execute_call(&mut self) -> KunaResult<()> {
        Err(self.illegal_op_error())
    }

    fn execute_callind(&mut self) -> KunaResult<()> {
        Err(self.illegal_op_error())
    }

    fn execute_callother(&mut self) -> KunaResult<()> {
        Err(self.illegal_op_error())
    }

    fn execute_multiequal(&mut self) -> KunaResult<()> {
        Err(self.illegal_op_error())
    }

    fn execute_indirect(&mut self) -> KunaResult<()> {
        Err(self.illegal_op_error())
    }

    fn execute_segment_op(&mut self) -> KunaResult<()> {
        Err(self.illegal_op_error())
    }

    fn execute_cpool_ref(&mut self) -> KunaResult<()> {
        Err(self.illegal_op_error())
    }

    fn execute_new(&mut self) -> KunaResult<()> {
        Err(self.illegal_op_error())
    }

    fn fallthru_op(&mut self) -> KunaResult<()> {
        self.pos += 1;
        if self.pos as usize == self.op_list.len() {
            // cast: C++ int4 == size_t comparison
            self.core.emu_halted = true;
            return Ok(());
        }
        self.set_current_op(self.pos);
        Ok(())
    }

    fn set_execute_address(&mut self, _addr: &Address) -> KunaResult<()> {
        self.set_current_op(0);
        Ok(())
    }

    fn get_execute_address(&self) -> Address {
        self.current_op().get_addr().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emulate::test_support::{
        cst, ram_vn, reg, spaceid_vn, test_manager, uvn, TestLoader, TestTranslator,
    };
    use crate::emulate::TranslateFloatFormats;
    use crate::translate::{PcodeEmit, Translate};
    use kuna_num::opbehavior::{register_instructions, FloatFormatProvider};

    /// Coerce the concrete test loader handle to the trait-object handle.
    fn dyn_loader(l: &Rc<RefCell<TestLoader>>) -> Rc<RefCell<dyn LoadImage>> {
        let coerced: Rc<RefCell<dyn LoadImage>> = l.clone();
        coerced
    }

    struct SnippetEnv {
        manager: Rc<AddrSpaceManager>,
        loader: Rc<RefCell<TestLoader>>,
        inst: Vec<Option<Rc<dyn OpBehavior>>>,
        ram: Rc<AddrSpace>,
    }

    fn build_snippet_env() -> SnippetEnv {
        let manager = Rc::new(test_manager(false));
        let trans: Rc<dyn Translate> =
            Rc::new(TestTranslator::new(&manager, BTreeMap::new(), Vec::new()));
        let loader = Rc::new(RefCell::new(TestLoader {
            start: 0x2000,
            bytes: vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
        }));
        let mut inst: Vec<Option<Rc<dyn OpBehavior>>> = Vec::new();
        let provider: Rc<dyn FloatFormatProvider> = Rc::new(TranslateFloatFormats(trans));
        register_instructions(&mut inst, &provider);
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        SnippetEnv { manager, loader, inst, ram }
    }

    fn run(snippet: &mut EmulateSnippet) -> KunaResult<()> {
        snippet.reset_memory();
        let mut steps = 0;
        while !snippet.get_halt() {
            snippet.execute_current_op()?;
            steps += 1;
            assert!(steps < 100, "runaway snippet");
        }
        Ok(())
    }

    /// The snippet (temporaries in the unique space at 0x80..):
    ///
    /// ```text
    /// 0: u80 = COPY 4
    /// 1: u88 = INT_MULT u80, 3        -> 12
    /// 2: u90 = LOAD ram[0x2000]       -> 0x8877665544332211
    /// 3: u98 = INT_ADD u88, u90       -> 0x887766554433221d
    /// 4: CBRANCH +2, 1                -> skips op 5
    /// 5: u98 = COPY 0                  (skipped)
    /// 6: ua0 = INT_SUB u98, 1         -> 0x887766554433221c
    /// ```
    fn build_arith_snippet(env: &SnippetEnv) -> EmulateSnippet {
        let mut snippet = EmulateSnippet::new(dyn_loader(&env.loader), Rc::clone(&env.manager));
        let addr = Address::new(Rc::clone(&env.ram), 0x1000);
        let m = &env.manager;
        {
            let mut emit = snippet.build_emitter(&env.inst, 0x200);
            emit.dump(&addr, OpCode::CPUI_COPY, Some(&uvn(m, 0x80, 8)), &[cst(m, 4, 8)]);
            emit.dump(
                &addr,
                OpCode::CPUI_INT_MULT,
                Some(&uvn(m, 0x88, 8)),
                &[uvn(m, 0x80, 8), cst(m, 3, 8)],
            );
            emit.dump(
                &addr,
                OpCode::CPUI_LOAD,
                Some(&uvn(m, 0x90, 8)),
                &[spaceid_vn(m, "ram"), cst(m, 0x2000, 8)],
            );
            emit.dump(
                &addr,
                OpCode::CPUI_INT_ADD,
                Some(&uvn(m, 0x98, 8)),
                &[uvn(m, 0x88, 8), uvn(m, 0x90, 8)],
            );
            emit.dump(&addr, OpCode::CPUI_CBRANCH, None, &[cst(m, 2, 8), cst(m, 1, 1)]);
            emit.dump(&addr, OpCode::CPUI_COPY, Some(&uvn(m, 0x98, 8)), &[cst(m, 0, 8)]);
            emit.dump(
                &addr,
                OpCode::CPUI_INT_SUB,
                Some(&uvn(m, 0xa0, 8)),
                &[uvn(m, 0x98, 8), cst(m, 1, 8)],
            );
        }
        snippet
    }

    #[test]
    fn test_emulateutil_snippet_execution() {
        let env = build_snippet_env();
        let mut snippet = build_arith_snippet(&env);
        assert!(snippet.check_for_legal_code());

        run(&mut snippet).unwrap();
        assert_eq!(snippet.get_temp_value(0x80), 4);
        assert_eq!(snippet.get_temp_value(0x88), 12);
        assert_eq!(snippet.get_temp_value(0x90), 0x8877665544332211);
        // the CBRANCH skipped op 5, so u98 keeps the INT_ADD result
        assert_eq!(snippet.get_temp_value(0x98), 0x887766554433221d);
        assert_eq!(snippet.get_temp_value(0xa0), 0x887766554433221c);
        // an unwritten temporary reads 0
        assert_eq!(snippet.get_temp_value(0xb0), 0);
        // the execute address is the address the ops were emitted at
        assert_eq!(snippet.get_execute_address().get_offset(), 0x1000);

        // reset_memory makes the snippet rerunnable with identical results
        run(&mut snippet).unwrap();
        assert_eq!(snippet.get_temp_value(0xa0), 0x887766554433221c);

        // direct injection of a temporary value
        snippet.reset_memory();
        snippet.set_varnode_value(0x80, 10);
        assert_eq!(snippet.get_temp_value(0x80), 10);
    }

    #[test]
    fn test_emulateutil_snippet_illegal_code() {
        let env = build_snippet_env();
        let m = &env.manager;
        let addr = Address::new(Rc::clone(&env.ram), 0x1000);

        // STORE is illegal: detected by checkForLegalCode and an error at
        // execution time
        let mut snippet = EmulateSnippet::new(dyn_loader(&env.loader), Rc::clone(&env.manager));
        {
            let mut emit = snippet.build_emitter(&env.inst, 0x200);
            emit.dump(
                &addr,
                OpCode::CPUI_STORE,
                None,
                &[spaceid_vn(m, "ram"), cst(m, 0x3000, 8), cst(m, 1, 8)],
            );
        }
        assert!(!snippet.check_for_legal_code());
        snippet.reset_memory();
        assert_eq!(
            snippet.execute_current_op().unwrap_err().explain(),
            format!("Illegal p-code operation in snippet: {}", get_opname(OpCode::CPUI_STORE))
        );

        // absolute BRANCH: illegal code, and an execution-time error
        let mut snippet = EmulateSnippet::new(dyn_loader(&env.loader), Rc::clone(&env.manager));
        {
            let mut emit = snippet.build_emitter(&env.inst, 0x200);
            emit.dump(&addr, OpCode::CPUI_BRANCH, None, &[ram_vn(m, 0x4000, 8)]);
        }
        assert!(!snippet.check_for_legal_code());
        snippet.reset_memory();
        assert_eq!(
            snippet.execute_current_op().unwrap_err().explain(),
            "Tried to emulate absolute branch in snippet code"
        );

        // relative branch out of bounds
        let mut snippet = EmulateSnippet::new(dyn_loader(&env.loader), Rc::clone(&env.manager));
        {
            let mut emit = snippet.build_emitter(&env.inst, 0x200);
            emit.dump(&addr, OpCode::CPUI_BRANCH, None, &[cst(m, 100, 8)]);
        }
        snippet.reset_memory();
        assert_eq!(
            snippet.execute_current_op().unwrap_err().explain(),
            "Relative branch out of bounds in snippet code"
        );

        // writing a non-temporary or reading a register is illegal code
        let mut snippet = EmulateSnippet::new(dyn_loader(&env.loader), Rc::clone(&env.manager));
        {
            let mut emit = snippet.build_emitter(&env.inst, 0x200);
            emit.dump(&addr, OpCode::CPUI_COPY, Some(&reg(m, 0)), &[cst(m, 1, 8)]);
        }
        assert!(!snippet.check_for_legal_code());
        let mut snippet = EmulateSnippet::new(dyn_loader(&env.loader), Rc::clone(&env.manager));
        {
            let mut emit = snippet.build_emitter(&env.inst, 0x200);
            emit.dump(&addr, OpCode::CPUI_COPY, Some(&uvn(m, 0x80, 8)), &[reg(m, 0)]);
        }
        assert!(!snippet.check_for_legal_code());

        // reading a temporary before writing it
        let mut snippet = EmulateSnippet::new(dyn_loader(&env.loader), Rc::clone(&env.manager));
        {
            let mut emit = snippet.build_emitter(&env.inst, 0x200);
            emit.dump(
                &addr,
                OpCode::CPUI_INT_ADD,
                Some(&uvn(m, 0x88, 8)),
                &[uvn(m, 0x80, 8), cst(m, 1, 8)],
            );
        }
        assert!(snippet.check_for_legal_code());
        snippet.reset_memory();
        assert_eq!(
            snippet.execute_current_op().unwrap_err().explain(),
            "Read before write in snippet emulation"
        );
    }

    #[test]
    fn test_emulateutil_snippet_branch_to_end_halts() {
        let env = build_snippet_env();
        let m = &env.manager;
        let addr = Address::new(Rc::clone(&env.ram), 0x1000);
        let mut snippet = EmulateSnippet::new(dyn_loader(&env.loader), Rc::clone(&env.manager));
        {
            let mut emit = snippet.build_emitter(&env.inst, 0x200);
            emit.dump(&addr, OpCode::CPUI_COPY, Some(&uvn(m, 0x80, 8)), &[cst(m, 1, 8)]);
            // BRANCH +2 from index 1 lands exactly one past the end: halt
            emit.dump(&addr, OpCode::CPUI_BRANCH, None, &[cst(m, 2, 8)]);
            emit.dump(&addr, OpCode::CPUI_COPY, Some(&uvn(m, 0x80, 8)), &[cst(m, 99, 8)]);
        }
        run(&mut snippet).unwrap();
        assert!(snippet.get_halt());
        assert_eq!(snippet.get_temp_value(0x80), 1);
    }

    /// getLoadImageValue endianness handling: the 8-byte read, the
    /// big-endian downshift for small sizes and the little-endian mask.
    #[test]
    fn test_emulateutil_snippet_load_image_value() {
        // little endian: mask the low sz bytes
        let env = build_snippet_env();
        let m = &env.manager;
        let addr = Address::new(Rc::clone(&env.ram), 0x1000);
        let mut snippet = EmulateSnippet::new(dyn_loader(&env.loader), Rc::clone(&env.manager));
        {
            let mut emit = snippet.build_emitter(&env.inst, 0x200);
            emit.dump(
                &addr,
                OpCode::CPUI_LOAD,
                Some(&uvn(m, 0x80, 4)),
                &[spaceid_vn(m, "ram"), cst(m, 0x2000, 8)],
            );
        }
        run(&mut snippet).unwrap();
        assert_eq!(snippet.get_temp_value(0x80), 0x44332211);

        // big endian: the value is shifted down from the top of the uintb
        let manager_be = Rc::new(test_manager(true));
        let ram_be = Rc::clone(manager_be.get_space_by_name("ram").unwrap());
        let loader_be = Rc::new(RefCell::new(TestLoader {
            start: 0x2000,
            bytes: vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
        }));
        let mut snippet = EmulateSnippet::new(dyn_loader(&loader_be), Rc::clone(&manager_be));
        let addr_be = Address::new(Rc::clone(&ram_be), 0x1000);
        {
            let mut emit = snippet.build_emitter(&env.inst, 0x200);
            emit.dump(
                &addr_be,
                OpCode::CPUI_LOAD,
                Some(&uvn(&manager_be, 0x80, 4)),
                &[spaceid_vn(&manager_be, "ram"), cst(&manager_be, 0x2000, 8)],
            );
            emit.dump(
                &addr_be,
                OpCode::CPUI_LOAD,
                Some(&uvn(&manager_be, 0x88, 8)),
                &[spaceid_vn(&manager_be, "ram"), cst(&manager_be, 0x2000, 8)],
            );
        }
        run(&mut snippet).unwrap();
        assert_eq!(snippet.get_temp_value(0x80), 0x11223344);
        assert_eq!(snippet.get_temp_value(0x88), 0x1122334455667788);

        // a LOAD outside the image propagates DataUnavail (not caught here)
        let mut snippet = EmulateSnippet::new(dyn_loader(&env.loader), Rc::clone(&env.manager));
        {
            let mut emit = snippet.build_emitter(&env.inst, 0x200);
            emit.dump(
                &addr,
                OpCode::CPUI_LOAD,
                Some(&uvn(m, 0x80, 8)),
                &[spaceid_vn(m, "ram"), cst(m, 0x9000, 8)],
            );
        }
        snippet.reset_memory();
        assert!(matches!(
            snippet.execute_current_op(),
            Err(KunaError::DataUnavail { .. })
        ));
    }
}
