//! What kuna says about each call the function makes, shared by both sides so
//! a call has one effect whichever side makes it.
//!
//! A call kills the registers its prototype model lists as killed-by-call and
//! every register an output can occupy, pops `extrapop` bytes of stack, and
//! may write the image's writable data and memory near the pointers passed to
//! it (`State::call`). Where kuna has no call at an instruction that calls code
//! in the image, kuna inlined, injected or dropped it: the machine runs the
//! callee for real, so a deliberate removal agrees and a wrong one shows in
//! what the callee does. A call kuna lacks to code outside the image gets a
//! stub with the function's own model, and the comparison reports it.

use std::collections::{BTreeMap, BTreeSet};

use kuna_decomp::fspec::{effect_type, FuncProto, EXTRAPOP_UNKNOWN};
use kuna_decomp::context::VarnodeId;
use kuna_decomp::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

use crate::state::{Effect, Env, Writes};
use crate::value::Halt;

/// Calls one run may make before it is given up as `UNKNOWN(budget)`: each
/// call keeps a snapshot of the memory its callee could read.
pub const MAX_CALLS: usize = 256;

/// Keep a call's record, or end the run when there have been too many.
pub fn record(records: &mut Vec<Record>, r: Record) -> Result<(), Halt> {
    if records.len() >= MAX_CALLS {
        return Err(Halt::Budget);
    }
    records.push(r);
    Ok(())
}
use crate::storage::Storage;

/// Where an argument is passed: the callee prototype's parameter storage. A
/// stack location is relative to the stack pointer the callee starts with. An
/// argument past the prototype's parameters (a variadic one) has no known
/// storage, and the input varnode's own storage is where the value was
/// computed, not where it is passed, so it is not compared.
#[derive(Clone, Debug)]
pub struct Arg {
    pub storage: Storage,
    pub callee_frame: bool,
}

#[derive(Clone, Debug)]
pub struct Site {
    /// The callee kuna names, when it is a known address.
    pub target: Option<u64>,
    /// Bytes of stack the call removes, the return address included; `None`
    /// when kuna does not know.
    pub extrapop: Option<i32>,
    pub noreturn: bool,
    /// Register ranges the call kills.
    pub killed: Vec<(u64, u32)>,
    /// Where each of kuna's arguments is passed, by input slot; `None` where
    /// it is not machine state.
    pub args: Vec<Option<Arg>>,
    /// The callee's parameters are declared, not recovered: an argument that
    /// differs is a difference, not a doubtful prototype.
    pub locked: bool,
    /// Where the callee's prototype returns its value.
    pub output: Option<Storage>,
    /// kuna bound the call past a PLT stub or replaced it with an injection:
    /// its target comes from relocations or a fixup the machine does not run.
    pub resolved: bool,
    /// Where each CALL input lives for the callee: its prototype parameter,
    /// or the trial kuna's recovery matched it to.
    pub input_storage: Vec<Option<Storage>>,
    /// How the ABI extends that value across its register, which a stub
    /// keeps (a 32-bit `int` in a 64-bit RISC-V register is sign-extended).
    pub ret_ext: Option<crate::state::Extension>,
}

pub struct Calls {
    /// By the address of the call instruction.
    pub by_addr: BTreeMap<u64, Site>,
    pub default: Site,
    /// Every instruction kuna kept an op of, and where its blocks start: a
    /// branch there stays in the function.
    pub inside: BTreeSet<u64>,
    /// Instructions kuna made an indirect jump: a CALLIND the machine lifts
    /// there is executed as one (an option reclassified it).
    pub jumps: BTreeSet<u64>,
    /// Instructions kuna ends in a branch and not a return: a RETURN the
    /// machine lifts there is followed (kuna resolved where it goes).
    pub branches: BTreeSet<u64>,
    /// The convention's argument registers, pointer-sized: where the machine
    /// looks for a frame object a callee without a recovered prototype gets.
    pub pointer_regs: Vec<Storage>,
    /// The convention's argument registers, whole: `(offset, size)`.
    pub input_regs: Vec<(u64, u32)>,
    /// Its general-purpose argument registers, in the order it assigns them.
    pub int_regs: Vec<Storage>,
    /// The registers it returns values in.
    pub output_regs: Vec<(u64, u32)>,
}

/// A system call or a trap, as a side reached it: an operation outside the
/// program that both sides must reach in the same order with the same inputs.
#[derive(Clone, Debug)]
pub struct UserEvent {
    pub at: u64,
    pub name: String,
    pub ins: Vec<u128>,
    /// The machine's side: its register file there, `(offset, byte)`.
    pub file: Vec<(u64, u8)>,
}

/// User ops that enter the operating system and come back: an uninterpreted
/// function of their inputs whose output, if any, both sides get alike. What
/// the kernel does to registers is not modeled.
pub const SYSTEM_USEROPS: &[&str] = &["syscall", "swi", "software_interrupt", "ecall", "CallSupervisor"];

/// User ops that trap and do not come back: the run ends there.
pub const TRAP_USEROPS: &[&str] = &["SoftwareBreakpoint", "UndefinedInstructionException"];

/// The value a system call's output gets: fixed by the run and its index.
/// What a system call reads and writes, in the registers kuna's `syscallregs`
/// and `x64syscall` give it: the number register then the argument registers,
/// in the order kuna attaches them, the result register, and the register
/// the kernel flags a failure in.
#[derive(Default)]
pub struct SyscallAbi {
    pub inputs: Vec<Storage>,
    pub result: Option<Storage>,
    pub error: Option<Storage>,
}

impl SyscallAbi {
    /// The target's system-call registers; empty for a target neither models.
    pub fn of(prog: &kuna_console::engine::ConsoleProgram) -> Self {
        use kuna_decomp::{kuna_syscallregs::SyscallFamily, kuna_x64syscall};
        let arch = prog.arch();
        let Some(lookup) = arch.manage().register_lookup() else { return Self::default() };
        let reg = |nm: &str| {
            let st = lookup.probe_register(nm)?;
            Some(Storage::new(st.space.as_ref()?, st.offset, st.size))
        };
        let (names, result, error): (Vec<&str>, &str, Option<&str>) = match SyscallFamily::from_archid(&arch.archid) {
            Some(f) => (f.inputs(reg(f.result()).is_some_and(|r| r.size == 8)).to_vec(), f.result(), f.error_flag()),
            None if reg(kuna_x64syscall::NUM_REGISTER).is_some_and(|r| r.size == 8) => (
                std::iter::once(kuna_x64syscall::NUM_REGISTER).chain(kuna_x64syscall::ARG_REGISTERS).collect(),
                kuna_x64syscall::NUM_REGISTER,
                None,
            ),
            None => return Self::default(),
        };
        let Some(inputs) = names.iter().map(|nm| reg(nm)).collect::<Option<Vec<_>>>() else { return Self::default() };
        SyscallAbi { inputs, result: reg(result), error: error.and_then(reg) }
    }
}

/// The value the kernel flags system call `index` with, in both runs.
pub fn syscall_error_value(run: u32, index: usize, size: u32) -> u128 {
    crate::graph::scramble(0x0e77_f1a6, run, index as u64) & crate::value::mask(size)
}

pub fn userop_value(run: u32, index: usize, size: u32) -> u128 {
    crate::graph::scramble(0x05ca_11ed, run, index as u64) & crate::value::mask(size)
}

/// One call as a side made it.
#[derive(Clone, Debug)]
pub struct Record {
    /// The address of the call instruction.
    pub at: u64,
    pub target: Option<u64>,
    /// The target was computed (CALLIND, an indirect tail jump) rather than
    /// encoded in the instruction.
    pub indirect: bool,
    pub args: Vec<u128>,
    /// Memory the callee can read: what this side wrote since its last call.
    pub writes: Writes,
    /// What the callee did when the machine ran it for real.
    pub effect: Option<Effect>,
    /// The frame bytes that callee read, when the machine ran it for real:
    /// only those of the frame objects it was given are compared.
    pub frame_reads: Option<std::collections::BTreeSet<u64>>,
    /// The argument-register bytes that callee read before writing, with
    /// the machine's values: what kuna's call must pass there.
    pub live_in: Option<Vec<(u64, u8)>>,
}

impl Calls {
    pub fn site(&self, at: u64) -> &Site {
        self.by_addr.get(&at).unwrap_or(&self.default)
    }

    pub fn known(&self, at: u64) -> bool {
        self.by_addr.contains_key(&at)
    }

    /// A branch at `at` to `target` that kuna turned into a call.
    pub fn tail(&self, at: u64, target: u64) -> bool {
        self.known(at) && !self.inside.contains(&target)
    }
}

/// Whether flow planted a no-return halt after the call at `at`: a callee
/// kuna knows does not return by name or by its own body, not by prototype.
fn halts_after(fd: &Funcdata, at: &kuna_base::address::Address) -> bool {
    fd.obank()
        .iter_at(at)
        .any(|(_, o)| fd.obank().get(o).is_some_and(|o| o.get_halt_type() & kuna_decomp::op::pcodeop_flags::noreturn != 0))
}

/// The registers a stub for `proto` kills: those its effect list says a call
/// kills, the call's own overrides first (what kuna proved about the callee),
/// and its output registers, unless the call is known to leave them alone.
fn killed(env: &Env, proto: &FuncProto) -> Vec<(u64, u32)> {
    let reg = env.register.get_index();
    let model = proto.model();
    let mut out: Vec<(u64, u32)> = proto
        .effect_list()
        .iter()
        .filter(|e| e.get_type() == effect_type::KILLEDBYCALL)
        .filter_map(|e| {
            let a = e.get_address();
            (a.get_space()?.get_index() == reg).then(|| (a.get_offset(), e.get_size() as u32))
        })
        .collect();
    out.extend(
        model
            .output()
            .get_entry()
            .iter()
            .filter(|e| e.get_space().get_index() == reg)
            .map(|e| (e.get_base(), e.get_size() as u32)),
    );
    out.retain(|&(off, size)| {
        let at = kuna_base::address::Address::new(std::rc::Rc::clone(&env.register), off);
        proto.has_effect(&at, size as i32) != effect_type::UNAFFECTED
    });
    out.sort_unstable();
    out.dedup();
    out
}

/// The register a call's value is copied or cast into, for a call whose
/// prototype has no output yet keeps its value in a temporary.
fn copied_to_register(fd: &Funcdata, env: &Env, out: Option<VarnodeId>) -> Option<Storage> {
    let v = fd.vbank().get(out?)?;
    v.descend_iter().filter_map(|d| fd.obank().get(d)).find_map(|d| {
        if !matches!(d.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            return None;
        }
        let w = fd.vbank().get(d.get_out()?)?;
        let st = Storage::at(w.get_addr(), w.get_size() as u32)?;
        (st.space.get_index() == env.register.get_index()).then_some(st)
    })
}

fn extrapop(pop: i32) -> Option<i32> {
    (pop != EXTRAPOP_UNKNOWN).then_some(pop)
}

/// Every call site of `fd`, keyed by its instruction address.
pub fn sites(fd: &Funcdata, env: &Env) -> Calls {
    let mut by_addr = BTreeMap::new();
    for i in 0..fd.num_calls() {
        let fc = fd.get_call_specs(i);
        let Some(op) = fd.obank().get(fc.get_op()) else { continue };
        let proto = fc.proto();
        let pop = extrapop(fc.get_effective_extra_pop()).or_else(|| extrapop(fc.get_extra_pop()));
        let sp = Storage::new(&env.register, env.sp.0, env.sp.1);
        let active = fc.active_input();
        let input_storage = (1..op.num_input())
            .map(|k| {
                if let Some(p) = proto.get_param(k - 1) {
                    return Storage::at(&p.get_address(), p.get_size() as u32);
                }
                (active.get_num_trials() > (k as i32) - 1).then(|| {
                    let t = active.get_trial_for_input_varnode(k as i32);
                    Storage::at(t.get_address(), t.get_size() as u32)
                })?
            })
            .collect();
        let args = (1..op.num_input())
            .map(|k| {
                let p = proto.get_param(k - 1)?;
                let storage = Storage::at(&p.get_address(), p.get_size() as u32)?;
                (!storage.overlaps(&sp)).then_some(Arg { storage, callee_frame: true })
            })
            .collect();
        let entry = fc.get_entry_address();
        let target = (!entry.is_invalid()).then(|| entry.get_offset());
        by_addr.insert(
            op.get_addr().get_offset(),
            Site {
                target,
                extrapop: pop,
                noreturn: proto.is_no_return() || halts_after(fd, op.get_addr()),
                killed: killed(env, proto)
                    .into_iter()
                    .filter(|&(off, size)| {
                        let at = kuna_base::address::Address::new(std::rc::Rc::clone(&env.register), off);
                        !kuna_decomp::kuna_calleepreserves::callee_preserves_range(fd, fc, &at, size as i32)
                    })
                    .collect(),
                args,
                locked: proto.is_input_locked(),
                output: (proto.get_output().get_size() > 0)
                    .then(|| Storage::at(&proto.get_output().get_address(), proto.get_output().get_size() as u32))
                    .flatten()
                    .or_else(|| copied_to_register(fd, env, op.get_out())),
                ret_ext: crate::abi::return_extension(env, proto),
                input_storage,
                resolved: fc.plt_stub().is_some() || proto.get_inject_id() >= 0,
            },
        );
    }
    let own = fd.get_func_proto();
    let default = Site {
        target: None,
        extrapop: extrapop(own.model().get_extra_pop()),
        noreturn: false,
        killed: killed(env, own),
        args: Vec::new(),
        locked: false,
        output: None,
        ret_ext: None,
        input_storage: Vec::new(),
        resolved: false,
    };
    let graph = fd.bblocks_ref();
    let blocks: Vec<_> = (0..fd.bblocks_get_size()).map(|i| fd.bblocks_get_block(i)).collect();
    let inside = blocks
        .iter()
        .map(|&b| kuna_decomp::block::block_get_start(&graph.arena, b).get_offset())
        .chain(blocks.iter().flat_map(|&b| fd.bb_ops(b)).filter_map(|op| fd.obank().get(op).map(|o| o.get_addr().get_offset())))
        .collect();
    use kuna_num::opcodes::OpCode;
    let ops: Vec<_> = blocks.iter().flat_map(|&b| fd.bb_ops(b)).filter_map(|op| fd.obank().get(op)).collect();
    let at = |codes: &[OpCode]| -> BTreeSet<u64> {
        ops.iter().filter(|o| codes.contains(&o.code())).map(|o| o.get_addr().get_offset()).collect()
    };
    let jumps = at(&[OpCode::CPUI_BRANCHIND]);
    let returns = at(&[OpCode::CPUI_RETURN]);
    let branches = at(&[OpCode::CPUI_BRANCH, OpCode::CPUI_CBRANCH, OpCode::CPUI_BRANCHIND])
        .into_iter()
        .filter(|a| !returns.contains(a))
        .collect();
    let width = u32::from(env.ram.get_addr_size()).min(8);
    let pointer_regs = own
        .model()
        .input()
        .get_entry()
        .iter()
        .filter(|e| e.get_space().get_index() == env.register.get_index() && e.get_size() as u32 >= width)
        .map(|e| {
            let low = if env.register.is_big_endian() { e.get_base() + u64::from(e.get_size() as u32 - width) } else { e.get_base() };
            Storage::new(&env.register, low, width)
        })
        .collect();
    let input_regs = own
        .model()
        .input()
        .get_entry()
        .iter()
        .filter(|e| e.get_space().get_index() == env.register.get_index() && e.get_base() != env.sp.0)
        .map(|e| (e.get_base(), e.get_size() as u32))
        .collect();
    let int_regs = own
        .model()
        .input()
        .get_entry()
        .iter()
        .filter(|e| {
            e.get_space().get_index() == env.register.get_index()
                && e.get_type() == kuna_decomp::dtype::type_class::TYPECLASS_GENERAL
                && e.get_base() != env.sp.0
        })
        .map(|e| Storage::new(&env.register, e.get_base(), e.get_size() as u32))
        .collect();
    let output_regs = own
        .model()
        .output()
        .get_entry()
        .iter()
        .filter(|e| e.get_space().get_index() == env.register.get_index())
        .map(|e| (e.get_base(), e.get_size() as u32))
        .collect();
    Calls { by_addr, default, inside, jumps, branches, pointer_regs, input_regs, int_regs, output_regs }
}
