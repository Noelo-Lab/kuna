//! Port of `decompiler/cpp/emulate.hh` + `emulate.cc` (W2, item
//! `w2-sleigh-emulate`): classes for emulating p-code.
//!
//! Paradigm mapping (each noted again at its use site):
//!
//! - The C++ abstract class `Emulate` splits into [`EmulateCore`] (the
//!   concrete data members `emu_halted` / `currentBehave`) and the
//!   [`Emulate`] trait — the protected virtuals as required methods plus the
//!   non-virtual `setHalt`/`getHalt`/`executeCurrentOp` as provided methods
//!   (the `TranslateBase`/`Translate` boundary precedent).
//! - The C++ intermediate class `EmulateMemory` becomes the
//!   [`EmulateMemory`] trait (accessors for its data members `memstate` /
//!   `currentOp`, plus the manager boundary below) together with the
//!   [`emulate_memory`] module holding its method bodies as free functions;
//!   a concrete engine implements `Emulate::execute_*` by delegating there
//!   — that delegation *is* the C++ inheritance edge, and an engine
//!   overriding a method (as `EmulatePcodeCache` does for `executeBranch` /
//!   `executeCallother`) simply provides its own body instead.
//! - **Manager boundary**: C++ `Translate` *is an* `AddrSpaceManager`, and
//!   `executeLoad`/`executeStore` reach the space table through
//!   `getSpaceFromConst()` (a reinterpreted pointer).  The port stores a
//!   manager index in the constant (see `kuna_num::pcoderaw`), so the
//!   emulator carries an explicit `Rc<AddrSpaceManager>` handle
//!   ([`EmulateMemory::addr_space_manager`]).
//! - **Breakpoint back-pointers**: C++ `BreakCallBack::setEmulate` /
//!   `BreakTable::setEmulate` store a raw `Emulate *` that callbacks reach
//!   back through while the emulator is mid-execution.  Rust cannot hold
//!   that mutable back-pointer, so the emulator is passed *into* the
//!   callback at invocation time (`&mut dyn EmulateMemory` parameters on
//!   [`BreakTable`]/[`BreakCallBack`]); the `setEmulate` plumbing disappears
//!   with identical observable association.  A callback that re-enters its
//!   own break table (C++ would allow it) panics on the `RefCell` borrow.
//! - `PcodeOpRaw *` handles into the op cache are `Rc<PcodeOpRaw>` (the
//!   ops are immutable once cached).  The C++ `varcache` of `VarnodeData *`
//!   disappears: the Rust `PcodeOpRaw` owns its varnodes by value (decision
//!   recorded in `kuna_num::pcoderaw` module docs), so [`PcodeEmitCache`]
//!   manages only the op cache.
//! - Errors (ADR 0004): every C++ throw becomes `Result` with the same
//!   explain string; `executeCurrentOp`'s dispatch errors propagate to the
//!   caller exactly where the C++ exception would.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{AddrSpace, AddrSpaceManager};
use kuna_base::types::Wrap;
use kuna_num::float::FloatFormat;
use kuna_num::opbehavior::{register_instructions, FloatFormatProvider, OpBehavior};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::{PcodeOpRaw, VarnodeData};

use crate::memstate::MemoryState;
use crate::translate::{PcodeEmit, Translate};

// ---------------------------------------------------------------------------
// BreakCallBack / BreakTable / BreakTableCallBack
// ---------------------------------------------------------------------------

/// \brief A breakpoint object
///
/// This is a base class for breakpoint objects in an emulator.  The
/// breakpoints are implemented as callback methods, which are overridden for
/// the particular behavior needed by the emulator.  Each implementation must
/// override either
///   - pcode_callback()
///   - address_callback()
///
/// depending on whether the breakpoint is tailored for a particular pcode op
/// or for a machine address.  (The C++ `emulate` member and `setEmulate` are
/// replaced by the `emulate` parameter — module docs.)
pub trait BreakCallBack {
    /// Call back method for pcode based breakpoints.
    ///
    /// This routine is invoked during emulation, if this breakpoint has
    /// somehow been associated with this kind of pcode op.  The callback can
    /// perform any operation on the emulator context it wants.  It then
    /// returns \b true if these actions are intended to replace the action
    /// of the pcode op itself.  Or it returns \b false if the pcode op
    /// should still have its normal effect on the emulator context.
    /// \param emulate is the emulator context (the C++ stored back-pointer)
    /// \param op is the particular pcode operation where the break occurs
    /// \return \b true if the normal pcode op action should not occur
    fn pcode_callback(
        &mut self,
        emulate: &mut dyn EmulateMemory,
        op: &PcodeOpRaw,
    ) -> KunaResult<bool> {
        let _ = (emulate, op);
        Ok(true)
    }

    /// Call back method for address based breakpoints.
    ///
    /// This routine is invoked during emulation, if this breakpoint has
    /// somehow been associated with this address.  The callback can perform
    /// any operation on the emulator context it wants. It then returns
    /// \b true if these actions are intended to replace the action of the
    /// \b entire machine instruction at this address. Or it returns \b false
    /// if the machine instruction should still be executed normally.
    /// \param emulate is the emulator context (the C++ stored back-pointer)
    /// \param addr is the address where the break has occurred
    /// \return \b true if the machine instruction should not be executed
    fn address_callback(
        &mut self,
        emulate: &mut dyn EmulateMemory,
        addr: &Address,
    ) -> KunaResult<bool> {
        let _ = (emulate, addr);
        Ok(true)
    }
}

/// \brief A collection of breakpoints for the emulator
///
/// A BreakTable keeps track of an arbitrary number of breakpoints for an
/// emulator.  Breakpoints are either associated with a particular
/// user-defined pcode op, or with a specific machine address (as in a
/// standard debugger). Through the BreakTable object, an emulator can invoke
/// breakpoints through the two methods
///  - do_pcode_op_break()
///  - do_address_break()
///
/// depending on the type of breakpoint they currently want to invoke.
/// (C++ `setEmulate` is replaced by the `emulate` parameters — module docs.)
pub trait BreakTable {
    /// \brief Invoke any breakpoints associated with this particular pcodeop
    ///
    /// Within the table, the first breakpoint which is designed to work with
    /// this particular kind of pcode operation is invoked.  If there was a
    /// breakpoint and it was designed to \e replace the action of the pcode
    /// op, then \b true is returned.
    /// \param emulate is the emulator context for the breakpoints
    /// \param curop is the instance of a pcode op to test for breakpoints
    /// \return \b true if the action of the pcode op is performed by the
    ///         breakpoint
    fn do_pcode_op_break(
        &mut self,
        emulate: &mut dyn EmulateMemory,
        curop: &PcodeOpRaw,
    ) -> KunaResult<bool>;

    /// \brief Invoke any breakpoints associated with this machine address
    ///
    /// Within the table, the first breakpoint which is designed to work with
    /// at this address is invoked.  If there was a breakpoint, and if it was
    /// designed to \e replace the action of the machine instruction, then
    /// \b true is returned.
    /// \param emulate is the emulator context for the breakpoints
    /// \param addr is address to test for breakpoints
    /// \return \b true if the machine instruction has been replaced by a
    ///         breakpoint
    fn do_address_break(
        &mut self,
        emulate: &mut dyn EmulateMemory,
        addr: &Address,
    ) -> KunaResult<bool>;
}

/// \brief A basic instantiation of a breakpoint table
///
/// This object allows breakpoints to be registered in the table via either
///   - register_pcode_callback()  or
///   - register_address_callback()
///
/// Breakpoints are stored in map containers, and the core BreakTable methods
/// are implemented to search in these containers
pub struct BreakTableCallBack {
    /// The translator
    trans: Rc<dyn Translate>,
    /// A container of address based breakpoints (C++ `map<Address,
    /// BreakCallBack *>`; ADR 0002 BTreeMap over the transcribed Address
    /// ordering).
    // mutable_key_type: Address's Ord reads only the space's immutable index
    // and the offset (kuna-base address.rs); the AddrSpace interior-mutable
    // Cells never participate, so a key's order cannot change in the map.
    #[allow(clippy::mutable_key_type)]
    addresscallback: BTreeMap<Address, Rc<RefCell<dyn BreakCallBack>>>,
    /// A container of pcode based breakpoints
    pcodecallback: BTreeMap<u64, Rc<RefCell<dyn BreakCallBack>>>,
}

impl BreakTableCallBack {
    /// Basic breaktable constructor.
    ///
    /// The break table needs a translator object so user-defined pcode ops
    /// can be registered against by name.  (The C++ `emulate` member and the
    /// `setEmulate` wiring are replaced by the invocation-time parameter —
    /// module docs.)
    /// \param t is the translator object
    pub fn new(t: Rc<dyn Translate>) -> Self {
        BreakTableCallBack {
            trans: t,
            addresscallback: BTreeMap::new(),
            pcodecallback: BTreeMap::new(),
        }
    }

    /// Register a pcode based breakpoint.
    ///
    /// Any time the emulator is about to execute a user-defined pcode op
    /// with the given name, the indicated breakpoint is invoked first. The
    /// break table does \e not assume responsibility for freeing the
    /// breakpoint object (the `Rc` is shared with the caller).
    /// \param name is the name of the user-defined pcode op
    /// \param func is the breakpoint object to associate with the pcode op
    pub fn register_pcode_callback(
        &mut self,
        name: &str,
        func: Rc<RefCell<dyn BreakCallBack>>,
    ) -> KunaResult<()> {
        // (C++ func->setEmulate(emulate) is the replaced back-pointer)
        let mut userops: Vec<String> = Vec::new();
        self.trans.get_user_op_names(&mut userops);
        let mut i: i32 = 0;
        while (i as usize) < userops.len() {
            // cast: C++ int4 i < vector::size() comparison
            if userops[i as usize] == name {
                self.pcodecallback.insert(i as u64, func); // cast: (uintb)i, i >= 0
                return Ok(());
            }
            i += 1;
        }
        Err(KunaError::lowlevel(format!("Bad userop name: {name}")))
    }

    /// Register an address based breakpoint.
    ///
    /// Any time the emulator is about to execute (the pcode translation of)
    /// a particular machine instruction at this address, the indicated
    /// breakpoint is invoked first. The break table does \e not assume
    /// responsibility for freeing the breakpoint object.
    /// \param addr is the address associated with the breakpoint
    /// \param func is the breakpoint being registered
    pub fn register_address_callback(
        &mut self,
        addr: &Address,
        func: Rc<RefCell<dyn BreakCallBack>>,
    ) {
        // (C++ func->setEmulate(emulate) is the replaced back-pointer)
        self.addresscallback.insert(addr.clone(), func);
    }
}

impl BreakTable for BreakTableCallBack {
    /// Invoke any breakpoints for the given pcode op.
    ///
    /// This routine examines the pcode-op based container for any
    /// breakpoints associated with the given op.  If one is found, its
    /// pcode_callback method is invoked.
    /// \param curop is pcode op being checked for breakpoints
    /// \return \b true if the breakpoint exists and returns \b true,
    ///         otherwise return \b false
    fn do_pcode_op_break(
        &mut self,
        emulate: &mut dyn EmulateMemory,
        curop: &PcodeOpRaw,
    ) -> KunaResult<bool> {
        let val = curop.get_input(0).offset;

        match self.pcodecallback.get(&val) {
            None => Ok(false),
            Some(func) => func.borrow_mut().pcode_callback(emulate, curop),
        }
    }

    /// Invoke any breakpoints for the given address.
    ///
    /// This routine examines the address based container for any breakpoints
    /// associated with the given address. If one is found, its
    /// address_callback method is invoked.
    /// \param addr is the address being checked for breakpoints
    /// \return \b true if the breakpoint exists and returns \b true,
    ///         otherwise return \b false
    fn do_address_break(
        &mut self,
        emulate: &mut dyn EmulateMemory,
        addr: &Address,
    ) -> KunaResult<bool> {
        match self.addresscallback.get(addr) {
            None => Ok(false),
            Some(func) => func.borrow_mut().address_callback(emulate, addr),
        }
    }
}

// ---------------------------------------------------------------------------
// Emulate
// ---------------------------------------------------------------------------

/// The concrete data members of the C++ abstract class `Emulate`
/// (`emu_halted`, `currentBehave`).  An engine embeds one and exposes it
/// through [`Emulate::emulate_core`].  The fields are public, mirroring the
/// C++ protected members.
pub struct EmulateCore {
    /// Set to \b true if the emulator is halted
    pub emu_halted: bool,
    /// Behavior of the next op to execute (C++ `OpBehavior *`, null when
    /// unset)
    pub current_behave: Option<Rc<dyn OpBehavior>>,
}

impl EmulateCore {
    /// Generic emulator constructor (C++ `Emulate::Emulate`).
    pub fn new() -> Self {
        EmulateCore { emu_halted: true, current_behave: None }
    }
}

impl Default for EmulateCore {
    fn default() -> Self {
        Self::new()
    }
}

/// \brief A pcode-based emulator interface.
///
/// The interface expects that the underlying emulation engine operates on
/// individual pcode operations as its atomic operation.  The interface
/// allows execution stepping through individual pcode operations. The
/// interface allows querying of the \e current pcode op, the current machine
/// address, and the rest of the machine state.
///
/// (The C++ protected virtuals are required methods here; Rust traits have
/// no protected methods, so they are public.)
pub trait Emulate {
    /// Access the concrete C++ base-class members (kuna boundary; see
    /// [`EmulateCore`]).
    fn emulate_core(&self) -> &EmulateCore;

    /// Mutable access to the concrete C++ base-class members.
    fn emulate_core_mut(&mut self) -> &mut EmulateCore;

    /// Execute a unary arithmetic/logical operation
    fn execute_unary(&mut self) -> KunaResult<()>;

    /// Execute a binary arithmetic/logical operation
    fn execute_binary(&mut self) -> KunaResult<()>;

    /// Standard behavior for a p-code LOAD
    fn execute_load(&mut self) -> KunaResult<()>;

    /// Standard behavior for a p-code STORE
    fn execute_store(&mut self) -> KunaResult<()>;

    /// \brief Standard behavior for a BRANCH
    ///
    /// This routine performs a standard p-code BRANCH operation on the
    /// memory state.  This same routine is used for CBRANCH operations if
    /// the condition has evaluated to \b true.
    fn execute_branch(&mut self) -> KunaResult<()>;

    /// \brief Check if the conditional of a CBRANCH is \b true
    ///
    /// This routine only checks if the condition for a p-code CBRANCH is
    /// true.  It does \e not perform the actual branch.
    /// \return the boolean state indicated by the condition
    fn execute_cbranch(&mut self) -> KunaResult<bool>;

    /// Standard behavior for a BRANCHIND
    fn execute_branchind(&mut self) -> KunaResult<()>;

    /// Standard behavior for a p-code CALL
    fn execute_call(&mut self) -> KunaResult<()>;

    /// Standard behavior for a CALLIND
    fn execute_callind(&mut self) -> KunaResult<()>;

    /// Standard behavior for a user-defined p-code op
    fn execute_callother(&mut self) -> KunaResult<()>;

    /// Standard behavior for a MULTIEQUAL (phi-node)
    fn execute_multiequal(&mut self) -> KunaResult<()>;

    /// Standard behavior for an INDIRECT op
    fn execute_indirect(&mut self) -> KunaResult<()>;

    /// Behavior for a SEGMENTOP
    fn execute_segment_op(&mut self) -> KunaResult<()>;

    /// Standard behavior for a CPOOLREF (constant pool reference) op
    fn execute_cpool_ref(&mut self) -> KunaResult<()>;

    /// Standard behavior for (low-level) NEW op
    fn execute_new(&mut self) -> KunaResult<()>;

    /// Standard p-code fall-thru semantics
    fn fallthru_op(&mut self) -> KunaResult<()>;

    /// Set the address of the next instruction to emulate
    fn set_execute_address(&mut self, addr: &Address) -> KunaResult<()>;

    /// Get the address of the current instruction being executed
    fn get_execute_address(&self) -> Address;

    /// Set the \e halt state of the emulator.
    ///
    /// Applications and breakpoints can use this method and its companion
    /// get_halt() to terminate and restart the main emulator loop as needed.
    /// The emulator itself makes no use of this routine or the associated
    /// state variable \b emu_halted.
    /// \param val is what the halt state of the emulator should be set to
    fn set_halt(&mut self, val: bool) {
        self.emulate_core_mut().emu_halted = val;
    }

    /// Get the \e halt state of the emulator.
    /// \return \b true if the emulator is in a "halted" state.
    fn get_halt(&self) -> bool {
        self.emulate_core().emu_halted
    }

    /// Do a single pcode op step.
    ///
    /// This method executes a single pcode operation, the current one.  The
    /// MemoryState of the emulator is queried and changed as needed to
    /// accomplish this.  (C++ `Emulate::executeCurrentOp`, non-virtual.)
    fn execute_current_op(&mut self) -> KunaResult<()> {
        let current_behave = match &self.emulate_core().current_behave {
            None => {
                // Presumably a NO-OP
                return self.fallthru_op();
            }
            Some(behave) => Rc::clone(behave),
        };
        if current_behave.is_special() {
            match current_behave.get_opcode() {
                OpCode::CPUI_LOAD => {
                    self.execute_load()?;
                    self.fallthru_op()?;
                }
                OpCode::CPUI_STORE => {
                    self.execute_store()?;
                    self.fallthru_op()?;
                }
                OpCode::CPUI_BRANCH => {
                    self.execute_branch()?;
                }
                OpCode::CPUI_CBRANCH => {
                    if self.execute_cbranch()? {
                        self.execute_branch()?;
                    } else {
                        self.fallthru_op()?;
                    }
                }
                OpCode::CPUI_BRANCHIND => {
                    self.execute_branchind()?;
                }
                OpCode::CPUI_CALL => {
                    self.execute_call()?;
                }
                OpCode::CPUI_CALLIND => {
                    self.execute_callind()?;
                }
                OpCode::CPUI_CALLOTHER => {
                    self.execute_callother()?;
                }
                OpCode::CPUI_RETURN => {
                    self.execute_branchind()?;
                }
                OpCode::CPUI_MULTIEQUAL => {
                    self.execute_multiequal()?;
                    self.fallthru_op()?;
                }
                OpCode::CPUI_INDIRECT => {
                    self.execute_indirect()?;
                    self.fallthru_op()?;
                }
                OpCode::CPUI_SEGMENTOP => {
                    self.execute_segment_op()?;
                    self.fallthru_op()?;
                }
                OpCode::CPUI_CPOOLREF => {
                    self.execute_cpool_ref()?;
                    self.fallthru_op()?;
                }
                OpCode::CPUI_NEW => {
                    self.execute_new()?;
                    self.fallthru_op()?;
                }
                _ => return Err(KunaError::lowlevel("Bad special op")),
            }
        } else if current_behave.is_unary() {
            // Unary operation
            self.execute_unary()?;
            self.fallthru_op()?;
        } else {
            // Binary operation
            self.execute_binary()?;
            self.fallthru_op()?; // All binary ops are fallthrus
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// EmulateMemory
// ---------------------------------------------------------------------------

/// \brief An abstract Emulate class using a MemoryState object as the
/// backing machine state
///
/// Most p-code operations are implemented using the MemoryState to fetch and
/// store values.  Control-flow is implemented partially in that
/// set_execute_address() is called to indicate which instruction is being
/// executed. The implementing engine must provide
///   - fallthru_op()
///   - set_execute_address()
///   - get_execute_address()
///
/// This trait carries the C++ `EmulateMemory` data members as accessors; the
/// method bodies live in the [`emulate_memory`] module and an engine's
/// `impl Emulate` delegates to them (module docs).  The following p-code
/// operations are stubbed out and return an error: CALLOTHER, MULTIEQUAL,
/// INDIRECT, CPOOLREF, SEGMENTOP, and NEW.  Of course the engine can
/// override these.
pub trait EmulateMemory: Emulate {
    /// Get the emulator's memory state (the C++ protected `memstate` member
    /// and the public `getMemoryState`; shared with the application).
    fn get_memory_state(&self) -> &Rc<RefCell<MemoryState>>;

    /// The current op to execute (the C++ protected `currentOp` member).
    fn current_op_raw(&self) -> Option<&Rc<PcodeOpRaw>>;

    /// kuna boundary: the `AddrSpaceManager` that the C++ `Translate`
    /// inheritance made implicitly reachable (resolves `<spaceid>` constants
    /// through `VarnodeData::get_space_from_const`; module docs).
    fn addr_space_manager(&self) -> &AddrSpaceManager;
}

/// The C++ `EmulateMemory` method bodies, as free functions over any
/// [`EmulateMemory`] engine.  A concrete engine's `impl Emulate` delegates
/// its `execute_*` methods here (the C++ inheritance edge); overriding a
/// C++ virtual means *not* delegating for that method.
pub mod emulate_memory {
    use super::*;

    fn current_op<T: EmulateMemory + ?Sized>(emu: &T) -> Rc<PcodeOpRaw> {
        Rc::clone(
            emu.current_op_raw()
                .expect("EmulateMemory: currentOp not set (C++ would dereference null)"),
        )
    }

    fn current_behave<T: EmulateMemory + ?Sized>(emu: &T) -> Rc<dyn OpBehavior> {
        Rc::clone(
            emu.emulate_core()
                .current_behave
                .as_ref()
                .expect("EmulateMemory: currentBehave not set (C++ would dereference null)"),
        )
    }

    /// C++ `EmulateMemory::executeUnary`
    pub fn execute_unary<T: EmulateMemory + ?Sized>(emu: &mut T) -> KunaResult<()> {
        let op = current_op(emu);
        let behave = current_behave(emu);
        let memstate = Rc::clone(emu.get_memory_state());
        let in1 = memstate.borrow().get_value_data(op.get_input(0))?;
        let out_vn = op.get_output().expect("executeUnary: op without output (C++ UB)");
        let out = behave.evaluate_unary(
            out_vn.size as i32,         // cast: uint4 size as the C++ int4 param
            op.get_input(0).size as i32, // cast: uint4 size as the C++ int4 param
            in1,
        )?;
        memstate.borrow_mut().set_value_data(out_vn, out)?;
        Ok(())
    }

    /// C++ `EmulateMemory::executeBinary`
    pub fn execute_binary<T: EmulateMemory + ?Sized>(emu: &mut T) -> KunaResult<()> {
        let op = current_op(emu);
        let behave = current_behave(emu);
        let memstate = Rc::clone(emu.get_memory_state());
        let in1 = memstate.borrow().get_value_data(op.get_input(0))?;
        let in2 = memstate.borrow().get_value_data(op.get_input(1))?;
        let out_vn = op.get_output().expect("executeBinary: op without output (C++ UB)");
        let out = behave.evaluate_binary(
            out_vn.size as i32,         // cast: uint4 size as the C++ int4 param
            op.get_input(0).size as i32, // cast: uint4 size as the C++ int4 param
            in1,
            in2,
        )?;
        memstate.borrow_mut().set_value_data(out_vn, out)?;
        Ok(())
    }

    /// C++ `EmulateMemory::executeLoad`
    pub fn execute_load<T: EmulateMemory + ?Sized>(emu: &mut T) -> KunaResult<()> {
        let op = current_op(emu);
        let memstate = Rc::clone(emu.get_memory_state());
        let off = memstate.borrow().get_value_data(op.get_input(1))?;
        let spc = op
            .get_input(0)
            .get_space_from_const(emu.addr_space_manager())
            .expect("executeLoad: <spaceid> constant does not resolve (C++ pointer always valid)");

        let off = AddrSpace::address_to_byte(off, spc.get_word_size());
        let out_vn = op.get_output().expect("executeLoad: op without output (C++ UB)");
        let res = memstate.borrow().get_value(&spc, off, out_vn.size as i32)?; // cast: uint4 size as int4
        memstate.borrow_mut().set_value_data(out_vn, res)?;
        Ok(())
    }

    /// C++ `EmulateMemory::executeStore`
    pub fn execute_store<T: EmulateMemory + ?Sized>(emu: &mut T) -> KunaResult<()> {
        let op = current_op(emu);
        let memstate = Rc::clone(emu.get_memory_state());
        let val = memstate.borrow().get_value_data(op.get_input(2))?; // Value being stored
        let off = memstate.borrow().get_value_data(op.get_input(1))?; // Offset to store at
        let spc = op
            .get_input(0) // Space to store in
            .get_space_from_const(emu.addr_space_manager())
            .expect("executeStore: <spaceid> constant does not resolve (C++ pointer always valid)");

        let off = AddrSpace::address_to_byte(off, spc.get_word_size());
        memstate.borrow_mut().set_value(&spc, off, op.get_input(2).size as i32, val)?; // cast: uint4 size as int4
        Ok(())
    }

    /// C++ `EmulateMemory::executeBranch`
    pub fn execute_branch<T: EmulateMemory + ?Sized>(emu: &mut T) -> KunaResult<()> {
        let op = current_op(emu);
        emu.set_execute_address(&op.get_input(0).get_addr())
    }

    /// C++ `EmulateMemory::executeCbranch`
    pub fn execute_cbranch<T: EmulateMemory + ?Sized>(emu: &mut T) -> KunaResult<bool> {
        let op = current_op(emu);
        let memstate = Rc::clone(emu.get_memory_state());
        let cond = memstate.borrow().get_value_data(op.get_input(1))?;
        Ok(cond != 0)
    }

    /// C++ `EmulateMemory::executeBranchind`
    pub fn execute_branchind<T: EmulateMemory + ?Sized>(emu: &mut T) -> KunaResult<()> {
        let op = current_op(emu);
        let memstate = Rc::clone(emu.get_memory_state());
        let off = memstate.borrow().get_value_data(op.get_input(0))?;
        let spc = Rc::clone(
            op.get_addr()
                .get_space()
                .expect("executeBranchind: instruction address with null space (C++ UB)"),
        );
        emu.set_execute_address(&Address::new(spc, off))
    }

    /// C++ `EmulateMemory::executeCall`
    pub fn execute_call<T: EmulateMemory + ?Sized>(emu: &mut T) -> KunaResult<()> {
        let op = current_op(emu);
        emu.set_execute_address(&op.get_input(0).get_addr())
    }

    /// C++ `EmulateMemory::executeCallind`
    pub fn execute_callind<T: EmulateMemory + ?Sized>(emu: &mut T) -> KunaResult<()> {
        let op = current_op(emu);
        let memstate = Rc::clone(emu.get_memory_state());
        let off = memstate.borrow().get_value_data(op.get_input(0))?;
        let spc = Rc::clone(
            op.get_addr()
                .get_space()
                .expect("executeCallind: instruction address with null space (C++ UB)"),
        );
        emu.set_execute_address(&Address::new(spc, off))
    }

    /// C++ `EmulateMemory::executeCallother`
    pub fn execute_callother<T: EmulateMemory + ?Sized>(_emu: &mut T) -> KunaResult<()> {
        Err(KunaError::lowlevel("CALLOTHER emulation not currently supported"))
    }

    /// C++ `EmulateMemory::executeMultiequal`
    pub fn execute_multiequal<T: EmulateMemory + ?Sized>(_emu: &mut T) -> KunaResult<()> {
        Err(KunaError::lowlevel("MULTIEQUAL appearing in unheritaged code?"))
    }

    /// C++ `EmulateMemory::executeIndirect`
    pub fn execute_indirect<T: EmulateMemory + ?Sized>(_emu: &mut T) -> KunaResult<()> {
        Err(KunaError::lowlevel("INDIRECT appearing in unheritaged code?"))
    }

    /// C++ `EmulateMemory::executeSegmentOp`
    pub fn execute_segment_op<T: EmulateMemory + ?Sized>(_emu: &mut T) -> KunaResult<()> {
        Err(KunaError::lowlevel("SEGMENTOP emulation not currently supported"))
    }

    /// C++ `EmulateMemory::executeCpoolRef`
    pub fn execute_cpool_ref<T: EmulateMemory + ?Sized>(_emu: &mut T) -> KunaResult<()> {
        Err(KunaError::lowlevel("Cannot currently emulate cpool operator"))
    }

    /// C++ `EmulateMemory::executeNew`
    pub fn execute_new<T: EmulateMemory + ?Sized>(_emu: &mut T) -> KunaResult<()> {
        Err(KunaError::lowlevel("Cannot currently emulate new operator"))
    }
}

// ---------------------------------------------------------------------------
// PcodeEmitCache
// ---------------------------------------------------------------------------

/// \brief P-code emitter that dumps its raw Varnodes and PcodeOps to an in
/// memory cache
///
/// This is used for emulation when full Varnode and PcodeOp objects aren't
/// needed.  (The C++ `varcache` parameter disappears: the Rust `PcodeOpRaw`
/// owns its varnodes by value — module docs.)
pub struct PcodeEmitCache<'a> {
    /// The cache of current p-code ops
    opcache: &'a mut Vec<Rc<PcodeOpRaw>>,
    /// Array of behaviors for translating OpCode
    inst: &'a [Option<Rc<dyn OpBehavior>>],
    /// Starting offset for defining temporaries in \e unique space (C++
    /// `uintm`)
    uniq: u32,
}

impl<'a> PcodeEmitCache<'a> {
    /// Constructor.
    ///
    /// Provide the emitter with the container that will hold the cached
    /// p-code ops.
    /// \param ocache is the container for cached PcodeOpRaw
    /// \param in_ is the map of OpBehavior
    /// \param uniq_reserve is the starting offset for temporaries in the
    ///        \e unique space
    pub fn new(
        ocache: &'a mut Vec<Rc<PcodeOpRaw>>,
        in_: &'a [Option<Rc<dyn OpBehavior>>],
        uniq_reserve: u64,
    ) -> Self {
        PcodeEmitCache {
            opcache: ocache,
            inst: in_,
            uniq: uniq_reserve as u32, // cast: C++ stores the uintb uniqReserve into the uintm member (truncating)
        }
    }
}

impl PcodeEmit for PcodeEmitCache<'_> {
    fn dump(
        &mut self,
        addr: &Address,
        opc: OpCode,
        outvar: Option<&VarnodeData>,
        vars: &[VarnodeData],
    ) {
        // (C++ pushes the op into the cache and then mutates it through the
        // pointer; the Rust op is fully built first — same cached result)
        let mut op = PcodeOpRaw::default();
        op.set_seq_num(addr, self.uniq);
        // C++ setBehavior(inst[opc]) stores whatever pointer is in the
        // table; an unregistered (null) entry stays unset here, making the
        // op a NO-OP for executeCurrentOp, as in C++.
        if let Some(behave) = &self.inst[opc as usize] {
            op.set_behavior(Rc::clone(behave));
        }
        self.uniq = self.uniq.wadd(1); // uintm increment
        if let Some(outvar) = outvar {
            op.set_output(Some(outvar.clone()));
        }
        for vn in vars {
            op.add_input(vn.clone());
        }
        self.opcache.push(Rc::new(op));
    }
}

// ---------------------------------------------------------------------------
// EmulatePcodeCache
// ---------------------------------------------------------------------------

/// Adapter giving [`register_instructions`] the one slice of [`Translate`]
/// it needs.  (C++ passes the `Translate *` directly; the W1 port
/// parameterized the float behaviors by [`FloatFormatProvider`] — see
/// kuna-num opbehavior.rs module docs.)
pub struct TranslateFloatFormats(pub Rc<dyn Translate>);

impl FloatFormatProvider for TranslateFloatFormats {
    fn get_float_format(&self, size: i32) -> Option<&FloatFormat> {
        self.0.get_float_format(size)
    }
}

/// \brief A SLEIGH based implementation of the Emulate interface
///
/// This implementation uses a Translate object to translate machine
/// instructions into pcode and caches pcode ops for later use by the
/// emulator.  The pcode is cached as soon as the execution address is set,
/// either explicitly, or via branches and fallthrus.  There are additional
/// methods for inspecting the pcode ops in the current instruction as a
/// sequence.
pub struct EmulatePcodeCache {
    /// C++ `Emulate` members
    core: EmulateCore,
    /// C++ `EmulateMemory::memstate` (shared with the application)
    memstate: Rc<RefCell<MemoryState>>,
    /// C++ `EmulateMemory::currentOp`
    current_op_ptr: Option<Rc<PcodeOpRaw>>,
    /// The SLEIGH translator
    trans: Rc<dyn Translate>,
    /// kuna manager boundary (module docs)
    manage: Rc<AddrSpaceManager>,
    /// The cache of current p-code ops (C++ also keeps a `varcache`; the
    /// Rust ops own their varnodes by value — module docs)
    opcache: Vec<Rc<PcodeOpRaw>>,
    /// Map from OpCode to OpBehavior
    inst: Vec<Option<Rc<dyn OpBehavior>>>,
    /// The table of breakpoints (shared with the application, which keeps
    /// registering breakpoints on it)
    breaktable: Rc<RefCell<dyn BreakTable>>,
    /// Address of current instruction being executed
    current_address: Address,
    /// \b true if next pcode op is start of instruction
    instruction_start: bool,
    /// Index of current pcode op within machine instruction
    current_op: i32,
    /// Length of current instruction in bytes
    instruction_length: i32,
}

impl EmulatePcodeCache {
    /// Pcode cache emulator constructor.
    ///
    /// \param t is the SLEIGH translator
    /// \param manage is the AddrSpaceManager the C++ `Translate` inherits
    ///        (kuna boundary; module docs)
    /// \param s is the MemoryState the emulator should manipulate
    /// \param b is the table of breakpoints the emulator should invoke
    pub fn new(
        t: Rc<dyn Translate>,
        manage: Rc<AddrSpaceManager>,
        s: Rc<RefCell<MemoryState>>,
        b: Rc<RefCell<dyn BreakTable>>,
    ) -> Self {
        let mut inst: Vec<Option<Rc<dyn OpBehavior>>> = Vec::new();
        let provider: Rc<dyn FloatFormatProvider> =
            Rc::new(TranslateFloatFormats(Rc::clone(&t)));
        register_instructions(&mut inst, &provider);
        // (C++ breaktable->setEmulate(this) is the replaced back-pointer —
        // the emulator is passed to the table at invocation time instead)
        EmulatePcodeCache {
            core: EmulateCore::new(),
            memstate: s,
            current_op_ptr: None,
            trans: t,
            manage,
            opcache: Vec::new(),
            inst,
            breaktable: b,
            // C++ leaves these uninitialized until createInstruction runs;
            // initialized to definite values here.
            current_address: Address::default(),
            instruction_start: false,
            current_op: 0,
            instruction_length: 0,
        }
    }

    /// Clear the p-code cache.
    ///
    /// (The C++ frees all the VarnodeData and PcodeOpRaw objects; here the
    /// `Rc`s drop.)
    fn clear_cache(&mut self) {
        self.opcache.clear();
    }

    /// Cache pcode for instruction at given address.
    ///
    /// This is a private routine which does the work of translating a
    /// machine instruction into pcode, putting it into the cache, and
    /// setting up the iterators
    /// \param addr is the address of the instruction to translate
    fn create_instruction(&mut self, addr: &Address) -> KunaResult<()> {
        self.clear_cache();
        let trans = Rc::clone(&self.trans);
        let mut emit = PcodeEmitCache::new(&mut self.opcache, &self.inst, 0);
        // (a C++ throw from oneInstruction leaves instruction_length /
        // current_op / instruction_start untouched — same here via `?`)
        self.instruction_length = trans.one_instruction(&mut emit, addr)?;
        self.current_op = 0;
        self.instruction_start = true;
        Ok(())
    }

    /// Set-up currentOp and currentBehave
    fn establish_op(&mut self) {
        if (self.current_op as usize) < self.opcache.len() {
            // cast: C++ int4 < vector::size() comparison (int converts)
            let op = Rc::clone(&self.opcache[self.current_op as usize]);
            self.core.current_behave = op.get_behavior().cloned();
            self.current_op_ptr = Some(op);
            return;
        }
        self.current_op_ptr = None;
        self.core.current_behave = None;
    }

    /// Return \b true if we are at an instruction start.
    ///
    /// Since the emulator can single step through individual pcode
    /// operations, the machine state may be halted in the \e middle of a
    /// single machine instruction, unlike conventional debuggers.  This
    /// routine can be used to determine if execution is actually at the
    /// beginning of a machine instruction.
    /// \return \b true if the next pcode operation is at the start of the
    ///         instruction translation
    pub fn is_instruction_start(&self) -> bool {
        self.instruction_start
    }

    /// Return number of pcode ops in translation of current instruction.
    ///
    /// A typical machine instruction translates into a sequence of pcode
    /// ops.
    /// \return the number of ops in the sequence
    pub fn num_current_ops(&self) -> i32 {
        self.opcache.len() as i32 // cast: C++ int4 return of vector::size()
    }

    /// Get the index of current pcode op within current instruction.
    ///
    /// This routine can be used to determine where, within the sequence of
    /// ops in the translation of the entire machine instruction, the
    /// currently executing op is.
    /// \return the index of the current (next) pcode op.
    pub fn get_current_op_index(&self) -> i32 {
        self.current_op
    }

    /// Get pcode op in current instruction translation by index.
    ///
    /// This routine can be used to examine ops other than the currently
    /// executing op in the machine instruction's translation sequence.
    /// \param i is the desired op index
    /// \return the pcode op at the indicated index
    pub fn get_op_by_index(&self, i: i32) -> &Rc<PcodeOpRaw> {
        &self.opcache[i as usize] // cast: int4 index, C++ UB when out of range
    }

    /// Execute (the rest of) a single machine instruction.
    ///
    /// This routine executes an entire machine instruction at once, as a
    /// conventional debugger step function would do.  If execution is at the
    /// start of an instruction, the breakpoints are checked and invoked as
    /// needed for the current address.  If this routine is invoked while
    /// execution is in the middle of a machine instruction, execution is
    /// continued until the current instruction completes.
    pub fn execute_instruction(&mut self) -> KunaResult<()> {
        if self.instruction_start {
            let breaktable = Rc::clone(&self.breaktable);
            let addr = self.current_address.clone();
            if breaktable.borrow_mut().do_address_break(self, &addr)? {
                return Ok(());
            }
        }
        loop {
            self.execute_current_op()?;
            if self.instruction_start {
                break;
            }
        }
        Ok(())
    }
}

impl Emulate for EmulatePcodeCache {
    fn emulate_core(&self) -> &EmulateCore {
        &self.core
    }

    fn emulate_core_mut(&mut self) -> &mut EmulateCore {
        &mut self.core
    }

    // inherited from C++ EmulateMemory (the delegation is the inheritance
    // edge; module docs)
    fn execute_unary(&mut self) -> KunaResult<()> {
        emulate_memory::execute_unary(self)
    }

    fn execute_binary(&mut self) -> KunaResult<()> {
        emulate_memory::execute_binary(self)
    }

    fn execute_load(&mut self) -> KunaResult<()> {
        emulate_memory::execute_load(self)
    }

    fn execute_store(&mut self) -> KunaResult<()> {
        emulate_memory::execute_store(self)
    }

    /// Execute branch (including relative branches).
    ///
    /// Since the full instruction is cached, we can do relative branches
    /// properly (C++ `EmulatePcodeCache::executeBranch` override).
    fn execute_branch(&mut self) -> KunaResult<()> {
        let current_op_ptr = Rc::clone(
            self.current_op_ptr
                .as_ref()
                .expect("executeBranch: currentOp not set (C++ would dereference null)"),
        );
        let destaddr = current_op_ptr.get_input(0).get_addr();
        if destaddr.is_constant() {
            let mut id = destaddr.get_offset() as u32; // cast: C++ uintm id = getOffset() (truncating)
            id = id.wadd(self.current_op as u32); // cast: (uintm)current_op
            self.current_op = id as i32; // cast: C++ int4 = uintm assignment (wraps)
            if self.current_op as usize == self.opcache.len() {
                // cast: C++ int4 == size_t comparison (int converts)
                self.fallthru_op()?;
            } else if self.current_op < 0 || (self.current_op as usize) >= self.opcache.len() {
                // cast: C++ int4 >= size_t comparison (negative int wraps)
                return Err(KunaError::lowlevel("Bad intra-instruction branch"));
            } else {
                self.establish_op();
            }
        } else {
            self.set_execute_address(&destaddr)?;
        }
        Ok(())
    }

    fn execute_cbranch(&mut self) -> KunaResult<bool> {
        emulate_memory::execute_cbranch(self)
    }

    fn execute_branchind(&mut self) -> KunaResult<()> {
        emulate_memory::execute_branchind(self)
    }

    fn execute_call(&mut self) -> KunaResult<()> {
        emulate_memory::execute_call(self)
    }

    fn execute_callind(&mut self) -> KunaResult<()> {
        emulate_memory::execute_callind(self)
    }

    /// Execute breakpoint for this user-defined op.
    ///
    /// Look for a breakpoint for the given user-defined op and invoke it.
    /// If it doesn't exist, or doesn't replace the action, an error is
    /// returned (C++ `EmulatePcodeCache::executeCallother` override).
    fn execute_callother(&mut self) -> KunaResult<()> {
        let breaktable = Rc::clone(&self.breaktable);
        let current_op_ptr = Rc::clone(
            self.current_op_ptr
                .as_ref()
                .expect("executeCallother: currentOp not set (C++ would dereference null)"),
        );
        if !breaktable.borrow_mut().do_pcode_op_break(self, &current_op_ptr)? {
            return Err(KunaError::lowlevel("Userop not hooked"));
        }
        self.fallthru_op()
    }

    fn execute_multiequal(&mut self) -> KunaResult<()> {
        emulate_memory::execute_multiequal(self)
    }

    fn execute_indirect(&mut self) -> KunaResult<()> {
        emulate_memory::execute_indirect(self)
    }

    fn execute_segment_op(&mut self) -> KunaResult<()> {
        emulate_memory::execute_segment_op(self)
    }

    fn execute_cpool_ref(&mut self) -> KunaResult<()> {
        emulate_memory::execute_cpool_ref(self)
    }

    fn execute_new(&mut self) -> KunaResult<()> {
        emulate_memory::execute_new(self)
    }

    /// Execute fallthru semantics for the pcode cache.
    ///
    /// Update the iterator into the current pcode cache, and if necessary,
    /// generate the pcode for the fallthru instruction and reset the
    /// iterator.
    fn fallthru_op(&mut self) -> KunaResult<()> {
        self.instruction_start = false;
        self.current_op += 1;
        if self.current_op as usize >= self.opcache.len() {
            // cast: C++ int4 >= size_t comparison (int converts)
            self.current_address =
                &self.current_address + i64::from(self.instruction_length);
            let addr = self.current_address.clone();
            self.create_instruction(&addr)?;
        }
        self.establish_op();
        Ok(())
    }

    /// Set current execution address.
    ///
    /// Set the current execution address and cache the pcode translation of
    /// the machine instruction at that address
    /// \param addr is the address where execution should continue
    fn set_execute_address(&mut self, addr: &Address) -> KunaResult<()> {
        // Copy -addr- BEFORE calling createInstruction
        // as it calls clear and may delete -addr-
        self.current_address = addr.clone();
        let addr = self.current_address.clone();
        self.create_instruction(&addr)?;
        self.establish_op();
        Ok(())
    }

    /// Get current execution address.
    /// \return the currently executing machine address
    fn get_execute_address(&self) -> Address {
        self.current_address.clone()
    }
}

impl EmulateMemory for EmulatePcodeCache {
    fn get_memory_state(&self) -> &Rc<RefCell<MemoryState>> {
        &self.memstate
    }

    fn current_op_raw(&self) -> Option<&Rc<PcodeOpRaw>> {
        self.current_op_ptr.as_ref()
    }

    fn addr_space_manager(&self) -> &AddrSpaceManager {
        &self.manage
    }
}

// ---------------------------------------------------------------------------
// Shared test support (used by the memstate / emulate / emulateutil tests)
// ---------------------------------------------------------------------------

#[cfg(test)]
pub(crate) mod test_support {
    use super::*;
    use crate::loadimage::LoadImage;
    use crate::translate::{AssemblyEmit, RegisterLookup, TranslateBase, VarnodeStorage};
    use kuna_base::space::{addrspace_flags, spacetype, ConstantSpace, OtherSpace, UniqueSpace};
    use kuna_base::xml::DocumentStorage;

    /// Manager with const(0)/OTHER(1)/unique(2)/ram(3)/register(4) spaces,
    /// mirroring the minimal C++ Architecture bootstrap order (the
    /// translate.rs test fixture without the join space).
    pub fn test_manager(big_end: bool) -> AddrSpaceManager {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        manager.insert_space(Rc::new(OtherSpace::new(1))).unwrap();
        manager.insert_space(Rc::new(UniqueSpace::new(2, 0, big_end))).unwrap();
        manager
            .insert_space(Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                "ram",
                big_end,
                8,
                1,
                3,
                addrspace_flags::hasphysical,
                1,
                1,
            )))
            .unwrap();
        manager
            .insert_space(Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                "register",
                big_end,
                4,
                1,
                4,
                0,
                0,
                0,
            )))
            .unwrap();
        manager.set_default_code_space(3).unwrap();
        manager
    }

    /// A load image exposing one contiguous byte region.  Bytes outside the
    /// region zero-fill; a request entirely outside errs with DataUnavail
    /// (the RawLoadImage-style contract MemoryImage relies on).
    pub struct TestLoader {
        pub start: u64,
        pub bytes: Vec<u8>,
    }

    impl LoadImage for TestLoader {
        fn get_file_name(&self) -> &str {
            "testloader"
        }

        fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
            let off = addr.get_offset();
            let end = self.start + self.bytes.len() as u64;
            if off >= end || off + ptr.len() as u64 <= self.start {
                return Err(KunaError::data_unavail("TestLoader: range outside image"));
            }
            for (i, b) in ptr.iter_mut().enumerate() {
                let a = off + i as u64;
                *b = if a >= self.start && a < end {
                    self.bytes[(a - self.start) as usize]
                } else {
                    0
                };
            }
            Ok(())
        }

        fn get_arch_type(&self) -> Vec<u8> {
            b"test:LE:64:default".to_vec()
        }

        fn adjust_vma(&mut self, _adjust: i64) {}
    }

    /// One canned p-code op of a canned instruction.
    pub struct TestOp {
        pub opc: OpCode,
        pub out: Option<VarnodeData>,
        pub ins: Vec<VarnodeData>,
    }

    /// A canned-program translator: a map from instruction offset (in the
    /// default code space) to (length, p-code ops), plus a register file
    /// r0..r7 at register:0,8,..,56 (8 bytes each).
    pub struct TestTranslator {
        base: TranslateBase,
        register_space: Rc<AddrSpace>,
        program: BTreeMap<u64, (i32, Vec<TestOp>)>,
        userops: Vec<String>,
    }

    impl TestTranslator {
        pub fn new(
            manager: &AddrSpaceManager,
            program: BTreeMap<u64, (i32, Vec<TestOp>)>,
            userops: Vec<String>,
        ) -> Self {
            TestTranslator {
                base: TranslateBase::new(),
                register_space: Rc::clone(manager.get_space_by_name("register").unwrap()),
                program,
                userops,
            }
        }

        fn reg_storage(&self, index: u64) -> VarnodeStorage {
            VarnodeStorage {
                space: Some(Rc::clone(&self.register_space)),
                offset: index * 8,
                size: 8,
            }
        }
    }

    impl RegisterLookup for TestTranslator {
        fn get_register(&self, nm: &str) -> KunaResult<VarnodeStorage> {
            if let Some(num) = nm.strip_prefix('r') {
                if let Ok(index) = num.parse::<u64>() {
                    if index < 8 {
                        return Ok(self.reg_storage(index));
                    }
                }
            }
            Err(KunaError::sleigh(format!("Unknown register name: {nm}")))
        }

        fn get_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: i32) -> String {
            self.get_exact_register_name(base, off, size)
        }

        fn get_exact_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: i32) -> String {
            if Rc::ptr_eq(base, &self.register_space)
                && off.is_multiple_of(8)
                && off < 64
                && size == 8
            {
                format!("r{}", off / 8)
            } else {
                String::new()
            }
        }
    }

    impl Translate for TestTranslator {
        fn translate_base(&self) -> &TranslateBase {
            &self.base
        }

        fn translate_base_mut(&mut self) -> &mut TranslateBase {
            &mut self.base
        }

        fn initialize(&mut self, _store: &mut DocumentStorage) -> KunaResult<()> {
            Ok(())
        }

        // mutable_key_type: see the trait declaration (translate.rs)
        #[allow(clippy::mutable_key_type)]
        fn get_all_registers(&self, reglist: &mut BTreeMap<VarnodeData, String>) {
            for i in 0..8u64 {
                let vs = self.reg_storage(i);
                reglist.insert(
                    VarnodeData { space: vs.space, offset: vs.offset, size: vs.size },
                    format!("r{i}"),
                );
            }
        }

        fn get_user_op_names(&self, res: &mut Vec<String>) {
            res.extend(self.userops.iter().cloned());
        }

        fn instruction_length(&self, baseaddr: &Address) -> KunaResult<i32> {
            match self.program.get(&baseaddr.get_offset()) {
                None => Err(KunaError::bad_data(format!(
                    "TestTranslator: no instruction at 0x{:x}",
                    baseaddr.get_offset()
                ))),
                Some((len, _)) => Ok(*len),
            }
        }

        fn one_instruction(&self, emit: &mut dyn PcodeEmit, baseaddr: &Address) -> KunaResult<i32> {
            match self.program.get(&baseaddr.get_offset()) {
                None => Err(KunaError::bad_data(format!(
                    "TestTranslator: no instruction at 0x{:x}",
                    baseaddr.get_offset()
                ))),
                Some((len, ops)) => {
                    for op in ops {
                        emit.dump(baseaddr, op.opc, op.out.as_ref(), &op.ins);
                    }
                    Ok(*len)
                }
            }
        }

        fn print_assembly(&self, _emit: &mut dyn AssemblyEmit, _baseaddr: &Address) -> KunaResult<i32> {
            Err(KunaError::lowlevel("TestTranslator: no assembly"))
        }
    }

    // --- varnode construction helpers ---

    /// A constant varnode.
    pub fn cst(manager: &AddrSpaceManager, val: u64, size: u32) -> VarnodeData {
        VarnodeData {
            space: Some(Rc::clone(manager.get_constant_space().unwrap())),
            offset: val,
            size,
        }
    }

    /// Register rN (8 bytes at register:8*N).
    pub fn reg(manager: &AddrSpaceManager, n: u64) -> VarnodeData {
        VarnodeData {
            space: Some(Rc::clone(manager.get_space_by_name("register").unwrap())),
            offset: n * 8,
            size: 8,
        }
    }

    /// A varnode in the unique space.
    pub fn uvn(manager: &AddrSpaceManager, offset: u64, size: u32) -> VarnodeData {
        VarnodeData {
            space: Some(Rc::clone(manager.get_space_by_name("unique").unwrap())),
            offset,
            size,
        }
    }

    /// A varnode in the ram space (e.g. a branch destination).
    pub fn ram_vn(manager: &AddrSpaceManager, offset: u64, size: u32) -> VarnodeData {
        VarnodeData {
            space: Some(Rc::clone(manager.get_space_by_name("ram").unwrap())),
            offset,
            size,
        }
    }

    /// The `<spaceid>` constant input of LOAD/STORE: constant space, offset
    /// = the manager index of the referenced space (the port's encoding of
    /// the C++ stuffed pointer; see `kuna_num::pcoderaw`), size 8.
    pub fn spaceid_vn(manager: &AddrSpaceManager, name: &str) -> VarnodeData {
        VarnodeData {
            space: Some(Rc::clone(manager.get_constant_space().unwrap())),
            offset: manager.get_space_by_name(name).unwrap().get_index() as u64, // cast: index >= 0
            size: 8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::test_support::*;
    use super::*;
    use crate::memstate::{MemoryBank, MemoryHashOverlay, MemoryImage, MemoryPageOverlay};

    /// Test environment: manager/translator/memory state wired like the
    /// emulate.hh documentation example (page overlay over a load image for
    /// ram, hash overlays for registers and temporaries).
    struct Env {
        memstate: Rc<RefCell<MemoryState>>,
        breaktable: Rc<RefCell<BreakTableCallBack>>,
        emulator: EmulatePcodeCache,
        ram: Rc<AddrSpace>,
    }

    fn build_env(
        make_program: impl FnOnce(&AddrSpaceManager) -> BTreeMap<u64, (i32, Vec<TestOp>)>,
        userops: Vec<String>,
    ) -> Env {
        let manager = Rc::new(test_manager(false));
        let program = make_program(&manager);
        let trans: Rc<dyn Translate> =
            Rc::new(TestTranslator::new(&manager, program, userops));
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        let register = Rc::clone(manager.get_space_by_name("register").unwrap());
        let unique = Rc::clone(manager.get_space_by_name("unique").unwrap());

        let loader = Rc::new(RefCell::new(TestLoader {
            start: 0x2000,
            bytes: vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88],
        }));
        let image: Rc<RefCell<dyn MemoryBank>> =
            Rc::new(RefCell::new(MemoryImage::new(Rc::clone(&ram), 8, 64, loader)));
        let ramstate: Rc<RefCell<dyn MemoryBank>> = Rc::new(RefCell::new(
            MemoryPageOverlay::new(Rc::clone(&ram), 8, 64, Some(image)),
        ));
        let registerstate: Rc<RefCell<dyn MemoryBank>> = Rc::new(RefCell::new(
            MemoryHashOverlay::new(Rc::clone(&register), 8, 64, 64, None),
        ));
        let uniquestate: Rc<RefCell<dyn MemoryBank>> = Rc::new(RefCell::new(
            MemoryHashOverlay::new(Rc::clone(&unique), 8, 64, 64, None),
        ));

        let mut state = MemoryState::new(Rc::clone(&trans));
        state.set_memory_bank(ramstate);
        state.set_memory_bank(registerstate);
        state.set_memory_bank(uniquestate);
        let memstate = Rc::new(RefCell::new(state));

        let breaktable = Rc::new(RefCell::new(BreakTableCallBack::new(Rc::clone(&trans))));
        let bt_dyn: Rc<RefCell<dyn BreakTable>> = breaktable.clone();
        let emulator = EmulatePcodeCache::new(
            Rc::clone(&trans),
            Rc::clone(&manager),
            Rc::clone(&memstate),
            bt_dyn,
        );

        Env { memstate, breaktable, emulator, ram }
    }

    /// The synthetic program (little endian, all instructions 4 bytes):
    ///
    /// ```text
    /// 0x1000: r0 = COPY 5
    /// 0x1004: r1 = COPY 7
    /// 0x1008: r0 = INT_ADD r0, r1
    /// 0x100c: u0 = INT_EQUAL r0, 12 ; CBRANCH +2, u0 ; r5 = COPY 0xdead
    /// 0x1010: r2 = LOAD ram[0x2000]
    /// 0x1014: STORE ram[0x3000] = r0
    /// 0x1018: BRANCH 0x1020
    /// 0x1020: CALLOTHER userop#0 ("double_r0")
    /// 0x1024: r4 = COPY 0x1100
    /// 0x1028: RETURN r4
    /// 0x1100: r1 = COPY r1   (translation pad; never executed)
    /// ```
    fn build_program(manager: &AddrSpaceManager) -> BTreeMap<u64, (i32, Vec<TestOp>)> {
        let mut prog = BTreeMap::new();
        prog.insert(
            0x1000,
            (4, vec![TestOp {
                opc: OpCode::CPUI_COPY,
                out: Some(reg(manager, 0)),
                ins: vec![cst(manager, 5, 8)],
            }]),
        );
        prog.insert(
            0x1004,
            (4, vec![TestOp {
                opc: OpCode::CPUI_COPY,
                out: Some(reg(manager, 1)),
                ins: vec![cst(manager, 7, 8)],
            }]),
        );
        prog.insert(
            0x1008,
            (4, vec![TestOp {
                opc: OpCode::CPUI_INT_ADD,
                out: Some(reg(manager, 0)),
                ins: vec![reg(manager, 0), reg(manager, 1)],
            }]),
        );
        prog.insert(
            0x100c,
            (
                4,
                vec![
                    TestOp {
                        opc: OpCode::CPUI_INT_EQUAL,
                        out: Some(uvn(manager, 0x100, 1)),
                        ins: vec![reg(manager, 0), cst(manager, 12, 8)],
                    },
                    TestOp {
                        opc: OpCode::CPUI_CBRANCH,
                        out: None,
                        ins: vec![cst(manager, 2, 8), uvn(manager, 0x100, 1)],
                    },
                    TestOp {
                        opc: OpCode::CPUI_COPY,
                        out: Some(reg(manager, 5)),
                        ins: vec![cst(manager, 0xdead, 8)],
                    },
                ],
            ),
        );
        prog.insert(
            0x1010,
            (4, vec![TestOp {
                opc: OpCode::CPUI_LOAD,
                out: Some(reg(manager, 2)),
                ins: vec![spaceid_vn(manager, "ram"), cst(manager, 0x2000, 8)],
            }]),
        );
        prog.insert(
            0x1014,
            (4, vec![TestOp {
                opc: OpCode::CPUI_STORE,
                out: None,
                ins: vec![spaceid_vn(manager, "ram"), cst(manager, 0x3000, 8), reg(manager, 0)],
            }]),
        );
        prog.insert(
            0x1018,
            (4, vec![TestOp {
                opc: OpCode::CPUI_BRANCH,
                out: None,
                ins: vec![ram_vn(manager, 0x1020, 8)],
            }]),
        );
        prog.insert(
            0x1020,
            (4, vec![TestOp {
                opc: OpCode::CPUI_CALLOTHER,
                out: None,
                ins: vec![cst(manager, 0, 8)],
            }]),
        );
        prog.insert(
            0x1024,
            (4, vec![TestOp {
                opc: OpCode::CPUI_COPY,
                out: Some(reg(manager, 4)),
                ins: vec![cst(manager, 0x1100, 8)],
            }]),
        );
        prog.insert(
            0x1028,
            (4, vec![TestOp {
                opc: OpCode::CPUI_RETURN,
                out: None,
                ins: vec![reg(manager, 4)],
            }]),
        );
        prog.insert(
            0x1100,
            (4, vec![TestOp {
                opc: OpCode::CPUI_COPY,
                out: Some(reg(manager, 1)),
                ins: vec![reg(manager, 1)],
            }]),
        );
        prog
    }

    /// Userop breakpoint: r3 = 2 * r0, replacing the CALLOTHER.
    struct DoubleR0;

    impl BreakCallBack for DoubleR0 {
        fn pcode_callback(
            &mut self,
            emulate: &mut dyn EmulateMemory,
            _op: &PcodeOpRaw,
        ) -> KunaResult<bool> {
            let mem = Rc::clone(emulate.get_memory_state());
            let r0 = mem.borrow().get_value_by_name("r0")?;
            mem.borrow_mut().set_value_by_name("r3", 2 * r0)?;
            Ok(true)
        }
    }

    /// Address breakpoint replacing the instruction at its address: sets
    /// r1 = 100 and continues at 0x1008 (the emulate.hh `puts` pattern).
    struct SkipAndSetR1;

    impl BreakCallBack for SkipAndSetR1 {
        fn address_callback(
            &mut self,
            emulate: &mut dyn EmulateMemory,
            addr: &Address,
        ) -> KunaResult<bool> {
            let mem = Rc::clone(emulate.get_memory_state());
            mem.borrow_mut().set_value_by_name("r1", 100)?;
            let spc = Rc::clone(addr.get_space().unwrap());
            emulate.set_execute_address(&Address::new(spc, 0x1008))?;
            Ok(true) // This replaces the indicated instruction
        }
    }

    fn run_to(env: &mut Env, stop: u64) {
        let mut steps = 0;
        while env.emulator.get_execute_address().get_offset() != stop {
            env.emulator.execute_instruction().unwrap();
            steps += 1;
            assert!(steps < 100, "runaway emulation");
        }
    }

    #[test]
    fn test_emulate_pcodecache_program() {
        let mut env = build_env(build_program, vec!["double_r0".to_string()]);
        env.breaktable
            .borrow_mut()
            .register_pcode_callback("double_r0", Rc::new(RefCell::new(DoubleR0)))
            .unwrap();

        env.emulator
            .set_execute_address(&Address::new(Rc::clone(&env.ram), 0x1000))
            .unwrap();
        assert!(env.emulator.is_instruction_start());
        run_to(&mut env, 0x1100);

        // hand-derived end state from the C++ semantics
        let mem = env.memstate.borrow();
        assert_eq!(mem.get_value_by_name("r0").unwrap(), 12);
        assert_eq!(mem.get_value_by_name("r1").unwrap(), 7);
        assert_eq!(mem.get_value_by_name("r2").unwrap(), 0x8877665544332211);
        assert_eq!(mem.get_value_by_name("r3").unwrap(), 24);
        assert_eq!(mem.get_value_by_name("r4").unwrap(), 0x1100);
        // the CBRANCH (r0 == 12) skipped the r5 write
        assert_eq!(mem.get_value_by_name("r5").unwrap(), 0);
        // the STORE landed in ram (little-endian bytes)
        assert_eq!(mem.get_value(&env.ram, 0x3000, 8).unwrap(), 12);
        let mut bytes = [0u8; 8];
        mem.get_chunk(&mut bytes, &env.ram, 0x3000).unwrap();
        assert_eq!(bytes, [12, 0, 0, 0, 0, 0, 0, 0]);
        // the LOAD source is still the load-image data
        assert_eq!(mem.get_value(&env.ram, 0x2000, 8).unwrap(), 0x8877665544332211);
    }

    #[test]
    fn test_emulate_pcodecache_address_breakpoint() {
        let mut env = build_env(build_program, vec!["double_r0".to_string()]);
        env.breaktable
            .borrow_mut()
            .register_pcode_callback("double_r0", Rc::new(RefCell::new(DoubleR0)))
            .unwrap();
        env.breaktable.borrow_mut().register_address_callback(
            &Address::new(Rc::clone(&env.ram), 0x1004),
            Rc::new(RefCell::new(SkipAndSetR1)),
        );

        env.emulator
            .set_execute_address(&Address::new(Rc::clone(&env.ram), 0x1000))
            .unwrap();
        run_to(&mut env, 0x1100);

        // breakpoint variant: r1 = 100 (callback), r0 = 5 + 100, the
        // CBRANCH is NOT taken (105 != 12) so r5 = 0xdead executes
        let mem = env.memstate.borrow();
        assert_eq!(mem.get_value_by_name("r0").unwrap(), 105);
        assert_eq!(mem.get_value_by_name("r1").unwrap(), 100);
        assert_eq!(mem.get_value_by_name("r3").unwrap(), 210);
        assert_eq!(mem.get_value_by_name("r5").unwrap(), 0xdead);
        assert_eq!(mem.get_value(&env.ram, 0x3000, 8).unwrap(), 105);
    }

    #[test]
    fn test_emulate_pcodecache_single_step() {
        let mut env = build_env(build_program, Vec::new());

        // position on the 3-op instruction and inspect the cache
        env.emulator
            .set_execute_address(&Address::new(Rc::clone(&env.ram), 0x100c))
            .unwrap();
        // r0 is 0 (unwritten): INT_EQUAL is false, CBRANCH falls through
        assert!(env.emulator.is_instruction_start());
        assert_eq!(env.emulator.num_current_ops(), 3);
        assert_eq!(env.emulator.get_current_op_index(), 0);
        assert_eq!(env.emulator.get_op_by_index(0).get_opcode(), OpCode::CPUI_INT_EQUAL);
        assert_eq!(env.emulator.get_op_by_index(1).get_opcode(), OpCode::CPUI_CBRANCH);
        assert_eq!(env.emulator.get_op_by_index(2).get_opcode(), OpCode::CPUI_COPY);

        env.emulator.execute_current_op().unwrap();
        assert!(!env.emulator.is_instruction_start());
        assert_eq!(env.emulator.get_current_op_index(), 1);
        env.emulator.execute_current_op().unwrap(); // CBRANCH false: fallthru
        assert_eq!(env.emulator.get_current_op_index(), 2);
        env.emulator.execute_current_op().unwrap(); // r5 = 0xdead, next instr
        assert!(env.emulator.is_instruction_start());
        assert_eq!(env.emulator.get_execute_address().get_offset(), 0x1010);
        assert_eq!(env.memstate.borrow().get_value_by_name("r5").unwrap(), 0xdead);

        // halt flag is application-controlled state only
        assert!(env.emulator.get_halt());
        env.emulator.set_halt(false);
        assert!(!env.emulator.get_halt());
    }

    #[test]
    fn test_emulate_pcodecache_errors() {
        let make_program = |manager: &AddrSpaceManager| {
            let mut prog = BTreeMap::new();
            // 0x1000: BRANCH +5 (out of bounds: 1 op in the cache)
            prog.insert(
                0x1000,
                (4, vec![TestOp {
                    opc: OpCode::CPUI_BRANCH,
                    out: None,
                    ins: vec![cst(manager, 5, 8)],
                }]),
            );
            // 0x1004: BRANCH with a relative offset that truncates to -1
            prog.insert(
                0x1004,
                (4, vec![TestOp {
                    opc: OpCode::CPUI_BRANCH,
                    out: None,
                    ins: vec![cst(manager, u64::MAX, 8)],
                }]),
            );
            // 0x1008: CALLOTHER userop#0 with nothing registered
            prog.insert(
                0x1008,
                (4, vec![TestOp {
                    opc: OpCode::CPUI_CALLOTHER,
                    out: None,
                    ins: vec![cst(manager, 0, 8)],
                }]),
            );
            prog
        };
        let mut env = build_env(make_program, vec!["nothooked".to_string()]);

        env.emulator
            .set_execute_address(&Address::new(Rc::clone(&env.ram), 0x1000))
            .unwrap();
        assert_eq!(
            env.emulator.execute_instruction().unwrap_err().explain(),
            "Bad intra-instruction branch"
        );
        env.emulator
            .set_execute_address(&Address::new(Rc::clone(&env.ram), 0x1004))
            .unwrap();
        assert_eq!(
            env.emulator.execute_instruction().unwrap_err().explain(),
            "Bad intra-instruction branch"
        );
        env.emulator
            .set_execute_address(&Address::new(Rc::clone(&env.ram), 0x1008))
            .unwrap();
        assert_eq!(
            env.emulator.execute_instruction().unwrap_err().explain(),
            "Userop not hooked"
        );

        // translating an unmapped address errs out of set_execute_address
        assert!(env
            .emulator
            .set_execute_address(&Address::new(Rc::clone(&env.ram), 0x9999))
            .is_err());

        // registering against an unknown userop name
        assert_eq!(
            env.breaktable
                .borrow_mut()
                .register_pcode_callback("bogus", Rc::new(RefCell::new(DoubleR0)))
                .unwrap_err()
                .explain(),
            "Bad userop name: bogus"
        );
    }
}
