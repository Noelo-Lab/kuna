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
    /// The compiler left none of the function after the call: evidence the
    /// call does not return that does not come from kuna.
    pub laid_out_noreturn: bool,
    /// Register ranges the call kills, as kuna's IR has it: the calling
    /// convention's, less what kuna found the callee preserves.
    pub killed: Vec<(u64, u32)>,
    /// The calling convention's alone. A callee run for real keeps only the
    /// other registers, so a register kuna wrongly thinks preserved differs.
    pub abi_killed: Vec<(u64, u32)>,
    /// The registers the calling convention's model says a callee preserves
    /// (`unaffected`), not kuna's prototype: all a callee run for real gets
    /// put back.
    pub preserved: Vec<(u64, u32)>,
    /// Where each of kuna's arguments is passed, by input slot; `None` where
    /// it is not machine state.
    pub args: Vec<Option<Arg>>,
    /// The callee's parameters are declared, not recovered: an argument that
    /// differs is a difference, not a doubtful prototype.
    pub locked: bool,
    /// How many parameters the prototype kuna gives the callee has.
    pub params: usize,
    /// Where the callee's prototype returns its value.
    pub output: Option<Storage>,
    /// kuna bound the call past a PLT stub or replaced it with an injection:
    /// its target comes from relocations or a fixup the machine does not run.
    pub resolved: bool,
    /// The PLT stub kuna bound the call past: the machine's call goes there.
    pub plt_stub: Option<u64>,
    /// Where each CALL input lives for the callee: its prototype parameter,
    /// or the trial kuna's recovery matched it to.
    pub input_storage: Vec<Option<Storage>>,
    /// An unlocked x86-32 cdecl call.
    cdecl_stack: bool,
    /// How the ABI extends that value across its register, which a stub
    /// keeps (a 32-bit `int` in a 64-bit RISC-V register is sign-extended).
    pub ret_ext: Option<crate::state::Extension>,
}

#[derive(Clone)]
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
    pub frame_reads: Option<crate::state::FrameReads>,
    /// The argument-register bytes that callee read before writing, with
    /// the machine's values: what kuna's call must pass there.
    pub live_in: Option<Vec<(u64, u8)>>,
    pub(crate) stack_in: Option<crate::state::StackInputs>,
}

impl Site {
    pub(crate) fn stack_input(&self, index: usize) -> Option<&Storage> {
        let st = self.input_storage.get(index)?.as_ref()?;
        (self.cdecl_stack
            && st.space.get_type() == kuna_base::space::spacetype::IPTR_SPACEBASE
            && (4..=u64::from(u32::MAX)).contains(&st.off)
            && (1..=16).contains(&st.size))
            .then_some(st)
    }
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

/// Whether flow planted a no-return halt after the call at `at`, or a
/// `CALL_RETURN` override cuts the flow after a call instruction (`error (1,
/// ...)`): a callee kuna knows does not return by name, by its own body or by
/// the call's arguments, not by prototype. The same override on a branch only
/// makes it a tail call.
fn halts_after(fd: &Funcdata, env: &Env, at: &kuna_base::address::Address) -> bool {
    let cut = fd.get_override().get_flow_override(at) == kuna_decomp::overrides::flow_type::CALL_RETURN
        && crate::machine::lift(env, at.get_offset())
            .is_ok_and(|i| i.ops.iter().any(|o| matches!(o.opc, OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)));
    cut || fd
            .obank()
            .iter_at(at)
            .any(|(_, o)| fd.obank().get(o).is_some_and(|o| o.get_halt_type() & kuna_decomp::op::pcodeop_flags::noreturn != 0))
}

/// Callees that do not return by their definition, whatever kuna inferred.
pub const KNOWN_NORETURN: &[&str] = &[
    "exit", "_exit", "_Exit", "quick_exit", "abort", "__assert_fail", "__assert_rtn", "__assert", "__stack_chk_fail",
    "__fortify_fail", "__chk_fail", "__libc_fatal", "longjmp", "_longjmp", "siglongjmp", "__longjmp_chk", "err", "errx",
    "verr", "verrx", "pthread_exit", "thrd_exit", "__cxa_throw", "__cxa_rethrow", "__cxa_bad_cast", "__cxa_bad_typeid",
    "_Unwind_Resume", "ExitProcess", "ExitThread", "TerminateProcess", "RaiseException", "_CxxThrowException",
];

/// Ghidra's curated lists of functions that do not return, by platform and
/// language: data about names, not anything kuna inferred about this binary.
const NORETURN_LISTS: [&str; 5] = [
    include_str!("../../../kuna-analysis/data/ElfFunctionsThatDoNotReturn"),
    include_str!("../../../kuna-analysis/data/PEFunctionsThatDoNotReturn"),
    include_str!("../../../kuna-analysis/data/PeMacFunctionsThatDoNotReturn"),
    include_str!("../../../kuna-analysis/data/RustFunctionsThatDoNotReturn"),
    include_str!("../../../kuna-analysis/data/GolangFunctionsThatDoNotReturn"),
];

fn listed(plain: &str) -> bool {
    NORETURN_LISTS.iter().flat_map(|l| l.lines()).map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')).any(|l| {
        let entry = l.trim_start_matches('_');
        match entry.strip_suffix('*') {
            Some(prefix) => plain.starts_with(prefix),
            None => plain == entry,
        }
    })
}

/// Whether `name` does not return by definition: a name on Ghidra's lists or
/// this one, or `error` and `error_at_line` with a nonzero `status`.
pub fn known_noreturn(name: &str, first_arg: Option<u128>) -> bool {
    let bare = name.split('@').next().unwrap_or(name);
    let plain = bare.trim_start_matches('_');
    if KNOWN_NORETURN.iter().any(|&k| k == bare || k.trim_start_matches('_') == plain)
        || listed(plain)
        || kuna_decomp::kuna_noreturn_externmatch::is_known_noreturn_name(bare)
    {
        return true;
    }
    matches!(plain, "error" | "error_at_line") && first_arg.is_some_and(|s| s as u32 != 0)
}

/// Whether only padding (instructions with no effect, or zero bytes) lies
/// between the call at `at` and the start of another function: the compiler left none of this function's code after the call, so
/// it does not return (`error (1, ...)`, a fatal wrapper no name marks).
fn falls_into_another(fd: &Funcdata, env: &Env, at: u64) -> bool {
    let own = fd.get_address().get_offset();
    let Ok(call) = crate::machine::lift(env, at) else { return false };
    let mut next = at.wrapping_add(call.len);
    for _ in 0..16 {
        if next != own && env.function_starts_at(next) {
            return true;
        }
        match crate::machine::lift(env, next) {
            Ok(i) if no_effect(env, &i) || (0..i.len).all(|k| env.image_byte(next.wrapping_add(k)) == Some(0)) => {
                next = next.wrapping_add(i.len)
            }
            _ => return false,
        }
    }
    false
}

/// Padding: an instruction that writes only temporaries and does not branch.
fn no_effect(env: &Env, insn: &crate::machine::Insn) -> bool {
    insn.ops.iter().all(|o| {
        !matches!(
            o.opc,
            OpCode::CPUI_STORE | OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND | OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER | OpCode::CPUI_RETURN
        ) && o.out.as_ref().is_none_or(|v| env.unique.as_ref().zip(v.space.as_ref()).is_some_and(|(u, s)| u.get_index() == s.get_index()))
    })
}

/// The registers a stub for `proto` kills: those its effect list says a call
/// kills, the call's own overrides first (what kuna proved about the callee),
/// and its output registers, unless the call is known to leave them alone.
fn preserved(env: &Env, proto: &FuncProto) -> Vec<(u64, u32)> {
    let reg = env.register.get_index();
    let mut out: Vec<(u64, u32)> = proto
        .model()
        .effect_list()
        .iter()
        .filter(|e| e.get_type() == effect_type::UNAFFECTED)
        .filter_map(|e| {
            let a = e.get_address();
            (a.get_space()?.get_index() == reg).then(|| (a.get_offset(), e.get_size() as u32))
        })
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

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

/// The register, or registers joined, a call's value is copied or cast into,
/// for a call whose prototype has no output yet keeps its value in a temporary.
fn copied_to_register(fd: &Funcdata, env: &Env, out: Option<VarnodeId>) -> Option<Storage> {
    let v = fd.vbank().get(out?)?;
    v.descend_iter().filter_map(|d| fd.obank().get(d)).find_map(|d| {
        if !matches!(d.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            return None;
        }
        let w = fd.vbank().get(d.get_out()?)?;
        let st = Storage::at(w.get_addr(), w.get_size() as u32)?;
        st.pieces(env.prog).iter().all(|p| p.space.get_index() == env.register.get_index()).then_some(st)
    })
}

fn extrapop(pop: i32) -> Option<i32> {
    (pop != EXTRAPOP_UNKNOWN).then_some(pop)
}

/// Every call site of `fd`, keyed by its instruction address.
pub fn sites(fd: &Funcdata, env: &Env) -> Calls {
    let mut by_addr = BTreeMap::new();
    let x86_cdecl_stack = env.prog.arch().archid.starts_with("x86:LE:32:");
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
                noreturn: proto.is_no_return() || halts_after(fd, env, op.get_addr()) || falls_into_another(fd, env, op.get_addr().get_offset()),
                laid_out_noreturn: falls_into_another(fd, env, op.get_addr().get_offset()),
                abi_killed: killed(env, proto),
                preserved: preserved(env, proto),
                killed: killed(env, proto)
                    .into_iter()
                    .filter(|&(off, size)| {
                        let at = kuna_base::address::Address::new(std::rc::Rc::clone(&env.register), off);
                        !kuna_decomp::kuna_calleepreserves::callee_preserves_range(fd, fc, &at, size as i32)
                    })
                    .collect(),
                args,
                locked: proto.is_input_locked(),
                params: proto.num_params().max(0) as usize,
                output: (proto.get_output().get_size() > 0)
                    .then(|| Storage::at(&proto.get_output().get_address(), proto.get_output().get_size() as u32))
                    .flatten()
                    .or_else(|| copied_to_register(fd, env, op.get_out())),
                ret_ext: crate::abi::return_extension(env, proto),
                input_storage,
                cdecl_stack: x86_cdecl_stack && proto.get_model_name() == "__cdecl" && !proto.is_input_locked(),
                resolved: fc.plt_stub().is_some() || proto.get_inject_id() >= 0,
                plt_stub: fc.plt_stub().map(|a| a.get_offset()),
            },
        );
    }
    let own = fd.get_func_proto();
    let default = Site {
        target: None,
        extrapop: extrapop(own.model().get_extra_pop()),
        noreturn: false,
        laid_out_noreturn: false,
        killed: killed(env, own),
        abi_killed: killed(env, own),
        preserved: preserved(env, own),
        args: Vec::new(),
        locked: false,
        params: 0,
        output: None,
        ret_ext: None,
        input_storage: Vec::new(),
        cdecl_stack: false,
        resolved: false,
        plt_stub: None,
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
