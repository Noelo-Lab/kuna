//! The run's starting state and what it watches: tracked and boolean inputs,
//! storage kuna never assigns, veneer registers, the input line.

//! Tier C: run both sides of one function and compare what they do.
//!
//! A run compares, in order: each call (its target, kuna's arguments, and the
//! memory the callee can read), whether the function returns at all, the return
//! value, and the memory left outside the stack. The return value is compared
//! under the physical ABI (the bytes kuna's claimed return occupies) and, when a
//! typed contract is given, at the contract's width with kuna's value converted
//! to the contract type by C's rules.


use kuna_base::space::spacetype;
use kuna_console::engine::ConsoleProgram;
use kuna_decomp::funcdata::Funcdata;

use crate::abi::{self, Claim};
use crate::calls::Calls;
use crate::state::{Env, State};
use crate::storage::{hex, Storage};
use crate::value::mask;
use crate::graph;

/// The parameters kuna's prototype types `bool`, in a register or in the
/// caller's stack.
pub(crate) fn booleans(fd: &Funcdata, env: &Env) -> Vec<(i32, u64, u32)> {
    let proto = fd.get_func_proto();
    (0..proto.num_params())
        .filter_map(|i| proto.get_param(i))
        .filter(|p| p.get_type().is_some_and(|t| t.get_metatype() == kuna_decomp::dtype::type_metatype::TYPE_BOOL))
        .filter_map(|p| Storage::at(&p.get_address(), p.get_size() as u32))
        .filter_map(|st| {
            if st.space.get_index() == env.register.get_index() {
                Some((st.space.get_index(), st.off, st.size))
            } else if st.space.get_type() == spacetype::IPTR_SPACEBASE {
                Some((env.ram.get_index(), env.ram.wrap_offset(env.sp0.wrapping_add(st.off)), st.size))
            } else {
                None
            }
        })
        .collect()
}

/// The storage whose value is known at the function's entry: what the context
/// tracks, what the loader seeds (a PowerPC TOC from a descriptor, MIPS t9),
/// x86's segment bases, and the return address, which gets an address nothing maps so a RETURN to
/// it is told apart from a branch.
pub(crate) fn tracked(prog: &ConsoleProgram, fd: &Funcdata, env: &mut Env) -> Vec<(i32, u64, u32, u64)> {
    let entry = fd.get_address();
    let arch = prog.arch();
    let to = |t: &kuna_sleigh::globalcontext::TrackedContext| {
        let sp = t.loc.space.as_ref()?;
        (t.loc.size <= 8).then(|| (sp.get_index(), t.loc.offset, t.loc.size, t.val))
    };
    let mut out: Vec<(i32, u64, u32, u64)> = arch.with_context_db(|db| db.get_tracked_set(entry).iter().filter_map(to).collect());
    if let Some(set) = arch.loader_entry_tracks.get(entry) {
        out.extend(set.iter().filter_map(to));
    }
    if let Some((vn, probe)) = &arch.loader_mips_t9 {
        if let (Some(sp), Some(v)) = (vn.space.as_ref(), probe(entry)) {
            if !out.iter().any(|&(s, o, _, _)| s == sp.get_index() && o == vn.offset) {
                out.push((sp.get_index(), vn.offset, vn.size, v));
            }
        }
    }
    let ret = fd
        .get_func_proto()
        .model()
        .effect_list()
        .iter()
        .find(|e| e.get_type() == kuna_decomp::fspec::effect_type::RETURN_ADDRESS)
        .and_then(|e| Storage::at(&e.get_address(), e.get_size() as u32));
    if let Some(st) = ret.filter(|st| st.size <= 8) {
        let sentinel = env.ram.wrap_offset(env.sp0.wrapping_add(0x800));
        let at = if st.space.get_type() == spacetype::IPTR_SPACEBASE {
            Some((env.ram.get_index(), env.ram.wrap_offset(env.sp0.wrapping_add(st.off))))
        } else {
            Some((st.space.get_index(), st.off))
        };
        if let Some((sp, off)) = at {
            out.push((sp, off, st.size, sentinel));
            env.ret_addr = Some(sentinel);
            if st.space.get_type() == spacetype::IPTR_SPACEBASE {
                env.ret_slot = st.size;
            }
        }
    }
    let tls = env.ram.wrap_offset(env.sp0 / 2) & !0xfff;
    for name in SEGMENT_BASES {
        if let Ok(vn) = arch.translate().get_register_data(name) {
            if let Some(sp) = vn.space.as_ref().filter(|_| vn.size <= 8) {
                if !out.iter().any(|&(s, o, _, _)| s == sp.get_index() && o == vn.offset) {
                    out.push((sp.get_index(), vn.offset, vn.size, tls));
                }
            }
        }
    }
    out
}

/// x86's segment bases: a thread's TLS or TEB, which is neither the image
/// nor the stack, so the stack-protector guard at `fs:0x28` stays put.
pub(crate) const SEGMENT_BASES: [&str; 2] = ["FS_OFFSET", "GS_OFFSET"];

/// What kuna's C never assigns but its IR reads: argument or scratch
/// registers read at entry with no parameter for them, and registers a call
/// kills that are read after it. Registers a caller leaves meaningful values
/// in (segment bases, PIC and frame registers, the stack pointer) are not
/// among them. `None` when there is none, so the second run is skipped.
pub(crate) fn unassigned(
    prog: &ConsoleProgram,
    env: &Env,
    fd: &Funcdata,
    claim: &abi::Claim,
    program: &crate::program::Program,
    calls: &Calls,
) -> Vec<Unassigned> {
    let tracked: Vec<Storage> = env
        .tracked
        .iter()
        .filter(|t| t.0 == env.register.get_index())
        .map(|t| Storage::new(&env.register, t.1, t.2))
        .collect();
    let scratch: Vec<(u64, u32)> = calls.input_regs.iter().copied().chain(calls.default.killed.iter().copied()).collect();
    let inputs: Vec<Storage> = abi::unclaimed_input_storage(prog, env, fd, claim)
        .into_iter()
        .filter(|s| !tracked.iter().any(|t| t.overlaps(s)) && !env.booleans.iter().any(|b| b.1 == s.off))
        .filter(|s| scratch.iter().any(|&(o, n)| s.off >= o && s.off + u64::from(s.size) <= o + u64::from(n)))
        .collect();
    let killed = program.blocks.iter().flat_map(|b| &b.ops).any(|o| matches!(o.kind, crate::program::Kind::Indirect(crate::program::Effect::Killed)));
    let salt = env.seed ^ 0x7a1e_5eed;
    let mut out = Vec::new();
    if killed {
        out.push(Unassigned {
            perturb: graph::Perturb { salt, inputs: Vec::new(), killed: Some(true) },
            names: "a call's output register it never connects".to_string(),
            entry: Vec::new(),
            shared: false,
        });
        out.push(Unassigned {
            perturb: graph::Perturb { salt, inputs: Vec::new(), killed: Some(false) },
            names: "a register a call kills, read after the call".to_string(),
            entry: Vec::new(),
            shared: true,
        });
    }
    if !inputs.is_empty() && !fd.get_func_proto().is_input_locked() {
        out.push(Unassigned {
            perturb: graph::Perturb { salt, inputs: inputs.iter().map(|s| (s.off, s.size)).collect(), killed: None },
            names: inputs.iter().map(|s| s.name(prog)).collect::<Vec<_>>().join(", "),
            entry: inputs.iter().map(|s| (s.off, s.size)).collect(),
            shared: true,
        });
    }
    out
}

/// One second run of kuna's side with other values in storage its C never
/// assigns. `entry`: registers read at entry, which the machine gets too, so
/// one the binary itself reads is told apart from one only kuna reads.
pub(crate) struct Unassigned {
    pub(crate) perturb: graph::Perturb,
    pub(crate) names: String,
    pub(crate) entry: Vec<(u64, u32)>,
    /// A dependence the machine shares, given the same values, is the
    /// binary's own and not kuna's. Not so for a call's output register:
    /// the binary reads what the callee returned there.
    pub(crate) shared: bool,
}

/// The bytes of the perturbed entry registers for one run.
pub(crate) fn entry_bytes(env: &Env, u: &Unassigned, run: u32) -> Vec<(u64, u8)> {
    let be = env.register.is_big_endian();
    let mut out = Vec::new();
    for &(off, size) in &u.entry {
        let v = graph::scramble(u.perturb.salt, run, off) & mask(size);
        for i in 0..u64::from(size) {
            let shift = if be { u64::from(size) - 1 - i } else { i };
            out.push((off + i, (v >> (8 * shift)) as u8));
        }
    }
    out
}

/// Whether the machine's system calls write the kernel's result registers:
/// everywhere but where `syscallregs auto` leaves them unwritten, on an image
/// that names no operating system.
pub(crate) fn kernel_returns(prog: &ConsoleProgram) -> bool {
    use kuna_decomp::kuna_syscallregs::{SyscallFamily, SyscallRegsMode};
    let arch = prog.arch();
    SyscallFamily::from_archid(&arch.archid).is_none() || arch.image_os_userland || arch.syscall_regs != SyscallRegsMode::Auto
}

pub(crate) fn input_line(prog: &ConsoleProgram, env: &Env, run: u32, claim: &Claim) -> String {
    let mut init = State::new(env, run);
    let inputs: Vec<String> = claim
        .params
        .iter()
        .map(|(n, st)| match init.read_storage(&st.space, st.off, st.size) {
            Ok(v) => format!("{n}={} ({})", hex(v, st.size), st.name(prog)),
            Err(_) => format!("{n}=? ({})", st.name(prog)),
        })
        .collect();
    let input = if inputs.is_empty() { "no declared parameters".to_string() } else { inputs.join("  ") };
    format!("input    {input}")
}

/// Check one decompiled function. `contract`, when given, is the function's
/// real type.
/// The registers the ABI reserves for veneers and PLT stubs, which a callee
/// may not expect to hold anything at its entry: ARM's `ip` (`r12`), AArch64's
/// `x16` and `x17`. A transparent thunk may write only these.
pub(crate) fn veneer_registers(prog: &ConsoleProgram) -> Vec<(u64, u32)> {
    let arch = prog.arch();
    let names: &[&str] = if arch.archid.starts_with("AARCH64") {
        &["x16", "x17"]
    } else if arch.archid.starts_with("ARM") {
        &["r12"]
    } else {
        &[]
    };
    let Some(lookup) = arch.manage().register_lookup() else { return Vec::new() };
    names.iter().filter_map(|n| lookup.probe_register(n)).map(|r| (r.offset, r.size)).collect()
}
