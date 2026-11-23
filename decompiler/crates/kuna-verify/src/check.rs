//! Tier C: run both sides of one function and compare what they do.
//!
//! A run compares, in order: each call (its target, kuna's arguments, and the
//! memory the callee can read), whether the function returns at all, the return
//! value, and the memory left outside the stack. The return value is compared
//! under the physical ABI (the bytes kuna's claimed return occupies) and, when a
//! typed contract is given, at the contract's width with kuna's value converted
//! to the contract type by C's rules.

use std::collections::{BTreeSet, HashMap};
use std::rc::Rc;
use std::time::Instant;

use kuna_base::space::spacetype;
use kuna_console::engine::ConsoleProgram;
use kuna_decomp::funcdata::Funcdata;

use crate::abi::{self, Claim};
use crate::contract::{CType, Contract};
use crate::divergence::{self, ReturnMismatch};
use crate::calls::{self, Calls, Record, UserEvent};
use crate::program::{self, Program};
use crate::report::{Report, Verdict};
use crate::state::{Env, State, Writes};
use crate::storage::{hex, Storage};
use crate::value::{mask, Halt};
use crate::{graph, machine};

#[derive(Debug, Clone, Copy)]
pub struct Options {
    pub runs: u32,
    pub seed: u64,
    /// Ops per side per run.
    pub budget: u64,
    /// Runs that may exhaust the budget before the function is given up as
    /// `UNKNOWN(budget)`: a loop bounded by an argument exhausts it on almost
    /// every random input.
    pub budget_runs: u32,
    /// Add both sides' calls to a `DIFFERS` report.
    pub trace: bool,
}

impl Default for Options {
    fn default() -> Self {
        Options { runs: 256, seed: 0x6b75_6e61, budget: 200_000, budget_runs: 4, trace: false }
    }
}

/// Upper return registers the machine sets to the same value on every run while
/// kuna's claimed return leaves them out (#794's shape).
struct UpperWatch {
    extra: Vec<Storage>,
    value: Option<Vec<u128>>,
    every_run: bool,
}

impl UpperWatch {
    fn new(prog: &ConsoleProgram, fd: &Funcdata, claim: &Claim) -> Option<Self> {
        let out = claim.storage.as_ref()?;
        let wide = abi::return_storage(prog, fd, &CType { spelling: String::new(), size: out.size * 2, signed: false, pointer: false }).ok()?;
        let pieces = wide.pieces(prog);
        let extra: Vec<Storage> = pieces.iter().filter(|p| !p.overlaps(out)).cloned().collect();
        (!extra.is_empty() && pieces.iter().any(|p| p.covers(out))).then_some(UpperWatch {
            extra,
            value: None,
            every_run: true,
        })
    }

    fn observe(&mut self, ms: &mut State) {
        let mut vals = Vec::with_capacity(self.extra.len());
        for p in &self.extra {
            if !ms.wrote(&p.space, p.off, p.size) {
                self.every_run = false;
                return;
            }
            match ms.read_storage(&p.space, p.off, p.size) {
                Ok(v) => vals.push(v),
                Err(_) => {
                    self.every_run = false;
                    return;
                }
            }
        }
        match &self.value {
            None => self.value = Some(vals),
            Some(prev) if *prev != vals => self.every_run = false,
            _ => {}
        }
    }

    fn suspects(self, prog: &ConsoleProgram) -> Vec<String> {
        let Some(vals) = self.value.filter(|_| self.every_run) else { return Vec::new() };
        self.extra
            .iter()
            .zip(vals)
            .map(|(p, v)| {
                format!(
                    "the machine sets {} to {} on every run; the claimed return leaves it out",
                    p.name(prog),
                    hex(v, p.size)
                )
            })
            .collect()
    }
}

/// The whole of x87's 80-bit `ST0` when `out` is it, where a `float` or
/// `double` return lives in the 80-bit format: the machine's value is rounded
/// to the claimed size before it is compared.
fn x87_register(prog: &ConsoleProgram, out: &Storage) -> Option<Storage> {
    if !(out.size == 4 || out.size == 8) || !prog.arch().archid.starts_with("x86") {
        return None;
    }
    let [piece] = out.pieces(prog).try_into().ok()?;
    (piece.name(prog) == "ST0").then(|| Storage::new(&piece.space, piece.off, 10))
}

/// One run's comparison: the report lines when the two sides differ.
struct Compare<'a> {
    prog: &'a ConsoleProgram,
    claim: &'a Claim,
    contract: Option<(&'a Contract, &'a Storage)>,
}

enum Outcome {
    Same,
    Differs(Vec<String>),
    Declared(&'static str),
}

impl Compare<'_> {
    fn run(&self, ms: &mut State, ret: Option<u128>) -> Result<Outcome, Halt> {
        let mut lines = Vec::new();
        if let Some(out) = &self.claim.storage {
            let k = ret.ok_or_else(|| Halt::Harness("RETURN without the prototype's value".into()))?;
            let mv = if let Some(st) = x87_register(self.prog, out).filter(|_| self.claim.float) {
                crate::x87::to_ieee(ms.read_storage(&st.space, st.off, st.size)?, out.size)
            } else {
                ms.read_storage(&out.space, out.off, out.size)?
            };
            if self.claim.float && divergence::nan_quieting(out.size, mv, k & mask(out.size)) {
                return Ok(Outcome::Declared(divergence::NAN_QUIETING.id));
            }
            if mv != k & mask(out.size) {
                lines.push(format!("machine  {} = {}", out.name(self.prog), hex(mv, out.size)));
                lines.push(format!("kuna     {} = {}", self.claim.text, hex(k, out.size)));
                return Ok(Outcome::Differs(lines));
            }
        }
        let Some((c, cs)) = self.contract else { return Ok(Outcome::Same) };
        let mv = ms.read_storage(&cs.space, cs.off, cs.size)?;
        let kuna = match (&self.claim.storage, ret) {
            (Some(out), Some(k)) => Some((out.size, abi::convert(k, out.size, self.claim.signed, cs.size))),
            _ => None,
        };
        if kuna.is_some_and(|(_, kv)| kv == mv) {
            return Ok(Outcome::Same);
        }
        if let Some((ks, kv)) = kuna {
            let m = ReturnMismatch { arch: &self.prog.arch().archid, contract_size: cs.size, kuna_size: ks, machine: mv, kuna: kv };
            if let Some(d) = divergence::find(&m) {
                return Ok(Outcome::Declared(d.id));
            }
        }
        lines.push(format!("contract {}  ({})", c.declaration, c.source));
        lines.push(format!("machine  {} = {}", cs.name(self.prog), hex(mv, cs.size)));
        match (&self.claim.storage, ret, kuna) {
            (Some(out), Some(k), Some((_, kv))) => lines.push(format!(
                "kuna     {} = {}; as {} {}",
                self.claim.text,
                hex(k, out.size),
                c.ret.spelling,
                hex(kv, cs.size)
            )),
            _ => lines.push(format!("kuna     {} returns no value", self.claim.text)),
        }
        Ok(Outcome::Differs(lines))
    }
}

/// The first byte the two sides left different, of the memory either wrote.
/// `read`: the frame bytes a callee run for real read, when only those count.
fn compare_writes(env: &Env, run: u32, machine: &Writes, kuna: &Writes, read: Option<&BTreeSet<u64>>) -> Option<String> {
    let manage = env.prog.arch().manage();
    let ram = env.ram.get_index();
    let keys: BTreeSet<(i32, u64)> = machine
        .bytes
        .keys()
        .chain(kuna.bytes.keys())
        .copied()
        .filter(|&(sp, off)| {
            let both = |o: &(u64, u64)| off >= o.0 && off < o.1 && kuna.objects.contains(o);
            !(sp == ram && env.in_stack(off, 1))
                || (machine.objects.iter().any(both)
                    && read.is_none_or(|r| r.contains(&off))
                    && !machine.skip.contains(&off)
                    && !kuna.skip.contains(&off))
        })
        .collect();
    let mut first = None;
    let mut differ = 0usize;
    for (sp, off) in keys {
        let Some(space) = manage.get_space(sp) else { continue };
        let side = |w: &Writes| {
            w.bytes.get(&(sp, off)).copied().unwrap_or_else(|| env.base_byte(run, w.epoch, &w.escaped, space, off))
        };
        let (m, k) = (side(machine), side(kuna));
        if m != k {
            differ += 1;
            first.get_or_insert(format!("{}:{off:#x}: machine 0x{m:02x}, kuna 0x{k:02x}", space.get_name()));
        }
    }
    first.map(|f| if differ > 1 { format!("{f} ({differ} bytes differ)") } else { f })
}

/// A computed call target as an address in the code space: wrapped to its
/// size, and without the ARM or MIPS instruction-set bit.
fn code_target(env: &Env, t: Option<u64>) -> Option<u64> {
    let id = &env.prog.arch().archid;
    let isa_bit = id.starts_with("ARM") || id.starts_with("MIPS");
    t.map(|t| env.code.wrap_offset(t) & if isa_bit { !1 } else { !0 })
}

fn target(t: Option<u64>) -> String {
    t.map_or_else(|| "?".to_string(), |t| format!("{t:#x}"))
}

/// Stack-protector failure handlers, without leading underscores. A call to one
/// on the machine side only means a call the model let rewrite the frame hit
/// the canary.
const STACK_PROTECTOR: &[&str] = &["stack_chk_fail", "security_check_cookie", "report_gsfailure"];

/// How the two sides' calls compare: the first difference, and the arguments a
/// recovered callee prototype may have wrong.
struct CallCompare {
    differs: Option<Vec<String>>,
    declared: Option<&'static str>,
    suspects: Vec<String>,
}

fn compare_calls(prog: &ConsoleProgram, env: &Env, run: u32, calls: &Calls, m: &[Record], g: &[Record]) -> CallCompare {
    let mut suspects = Vec::new();
    if g.iter().any(|c| c.at == 0 || (!c.indirect && c.target.is_none())) {
        return CallCompare { differs: None, declared: Some(divergence::SYNTHESIZED_CALL.id), suspects };
    }
    let differs = |lines: Vec<String>, suspects: Vec<String>| CallCompare { differs: Some(lines), declared: None, suspects };
    for k in 0..m.len().max(g.len()) {
        let n = k + 1;
        let (mc, gc) = match (m.get(k), g.get(k)) {
            (Some(mc), Some(gc)) => (mc, gc),
            (Some(mc), None) => {
                let name = mc.target.and_then(|t| prog.function_named_at(t)).unwrap_or_default();
                if name.starts_with("runtime.gcWriteBarrier") {
                    return CallCompare { differs: None, declared: Some(divergence::GO_WRITE_BARRIER.id), suspects };
                }
                if kuna_decomp::kuna_gobounds::is_go_check_name(&name) || kuna_decomp::kuna_securitycheck::is_security_check_name(&name) {
                    return CallCompare { differs: None, declared: Some(divergence::PANIC_CHECK.id), suspects };
                }
                let l = format!("call {n} at {:#x} to {}: the machine makes it, kuna does not", mc.at, target(mc.target));
                if STACK_PROTECTOR.contains(&name.trim_start_matches('_')) {
                    return CallCompare { differs: None, declared: None, suspects: vec![format!("canary: {l}")] };
                }
                return differs(vec![l], suspects);
            }
            (None, Some(gc)) => {
                let l = format!("call {n} at {:#x} to {}: kuna makes it, the machine does not", gc.at, target(gc.target));
                return differs(vec![l], suspects);
            }
            (None, None) => break,
        };
        if mc.at != gc.at {
            return differs(vec![format!("call {n}: the machine calls from {:#x}, kuna from {:#x}", mc.at, gc.at)], suspects);
        }
        let (mt, kt) = (code_target(env, mc.target), code_target(env, gc.target));
        let (mt, kt) = (mt.map(|t| machine::through_thunks(env, t)), kt.map(|t| machine::through_thunks(env, t)));
        let both_code = mt.zip(kt).is_some_and(|(m, k)| env.is_code(m) && env.is_code(k));
        let compared = (mc.indirect && gc.indirect) || (both_code && !calls.site(gc.at).resolved);
        if compared && mt != kt {
            let l = format!("call {n} at {:#x}: the machine calls {}, kuna calls {}", gc.at, target(mc.target), target(gc.target));
            return differs(vec![l], suspects);
        }
        let site = calls.site(gc.at);
        for (i, ((ma, ka), arg)) in mc.args.iter().zip(&gc.args).zip(&site.args).enumerate() {
            let Some(st) = arg.as_ref().map(|a| &a.storage) else { continue };
            if ma & mask(st.size) == ka & mask(st.size) {
                continue;
            }
            let l = format!(
                "call {n} to {}, argument {} ({}): machine {}, kuna {}",
                target(gc.target),
                i + 1,
                st.name(prog),
                hex(*ma & mask(st.size), st.size),
                hex(*ka & mask(st.size), st.size)
            );
            if site.locked {
                return differs(vec![l], suspects);
            }
            let s = format!("{l}; the callee's recovered prototype may not take it");
            if !suspects.contains(&s) {
                suspects.push(s);
            }
        }
        if let Some(d) = compare_writes(env, run, &mc.writes, &gc.writes, mc.frame_reads.as_ref()) {
            return differs(vec![format!("memory before call {n} to {}: {d}", target(gc.target))], suspects);
        }
    }
    CallCompare { differs: None, declared: None, suspects }
}

/// The parameters kuna's prototype types `bool`, in a register or in the
/// caller's stack.
fn booleans(fd: &Funcdata, env: &Env) -> Vec<(i32, u64, u32)> {
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
fn tracked(prog: &ConsoleProgram, fd: &Funcdata, env: &mut Env) -> Vec<(i32, u64, u32, u64)> {
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
const SEGMENT_BASES: [&str; 2] = ["FS_OFFSET", "GS_OFFSET"];

/// Runs that may stop on an operation the model cannot run before the
/// function is given up as `UNKNOWN(unsupported-op)`: such a function is
/// `UNKNOWN` whatever its other runs show, unless one of them differs, and
/// later runs mostly take the same path into the same operation.
const UNSUPPORTED_RUNS: u32 = 16;

/// Runs that are repeated with other values in the storage kuna's C never
/// assigns.
const PERTURBED_RUNS: u32 = 16;

/// What kuna's C never assigns but its IR reads: argument or scratch
/// registers read at entry with no parameter for them, and registers a call
/// kills that are read after it. Registers a caller leaves meaningful values
/// in (segment bases, PIC and frame registers, the stack pointer) are not
/// among them. `None` when there is none, so the second run is skipped.
fn unassigned(
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
struct Unassigned {
    perturb: graph::Perturb,
    names: String,
    entry: Vec<(u64, u32)>,
    /// A dependence the machine shares, given the same values, is the
    /// binary's own and not kuna's. Not so for a call's output register:
    /// the binary reads what the callee returned there.
    shared: bool,
}

/// The bytes of the perturbed entry registers for one run.
fn entry_bytes(env: &Env, u: &Unassigned, run: u32) -> Vec<(u64, u8)> {
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

/// What a callee run for real did differently given other arguments: the
/// value it returns, or what it wrote to memory.
fn callee_differs(prog: &ConsoleProgram, calls: &Calls, a: Option<&Record>, b: Option<&Record>) -> Option<String> {
    let (ea, eb) = (a?.effect.as_ref()?, b?.effect.as_ref()?);
    let site = calls.site(a?.at);
    if let Some(out) = site.output.as_ref().filter(|o| o.space.get_type() == spacetype::IPTR_PROCESSOR) {
        let within = |r: &&(u64, u8)| r.0 >= out.off && r.0 < out.off + u64::from(out.size);
        let (ra, rb): (Vec<_>, Vec<_>) = (ea.regs.iter().filter(within).collect(), eb.regs.iter().filter(within).collect());
        if ra != rb {
            return Some(format!("callee returns another {}", out.name(prog)));
        }
    }
    let writes = |e: &crate::state::Effect| -> Vec<(i32, u64, u8)> {
        e.events.iter().filter_map(|ev| if let crate::state::Event::Write(s, o, b) = ev { Some((*s, *o, *b)) } else { None }).collect()
    };
    (writes(ea) != writes(eb)).then(|| "callee writes other memory".to_string())
}

/// The machine stopped in a pointer-authentication trap kuna does not reach.
fn ptrauth_trap(m: &[UserEvent], g: &[UserEvent]) -> bool {
    m.last().is_some_and(|e| {
        e.name.as_bytes() == kuna_decomp::kuna_ptrauthtrap::BREAKPOINT_USEROP
            && e.ins.first().is_some_and(|&i| kuna_decomp::kuna_ptrauthtrap::is_ptrauth_trap_immediate(i as u64))
            && !g.iter().any(|k| k.at == e.at)
    })
}

/// The first system call or trap the two sides reach differently.
fn compare_events(prog: &ConsoleProgram, m: &[UserEvent], g: &[UserEvent]) -> Option<Vec<String>> {
    let regs = if g.iter().any(|e| crate::calls::SYSTEM_USEROPS.contains(&e.name.as_str())) {
        crate::calls::SyscallAbi::of(prog).inputs
    } else {
        Vec::new()
    };
    let show = |e: Option<&UserEvent>| {
        e.map_or_else(
            || "none".to_string(),
            |e| {
                let ins: Vec<String> = e.ins.iter().map(|v| format!("{v:#x}")).collect();
                format!("{} at {:#x} ({})", e.name, e.at, ins.join(", "))
            },
        )
    };
    let n = (0..m.len().max(g.len())).find(|&k| match (m.get(k), g.get(k)) {
        (Some(a), Some(b)) => !same_event(a, b, &regs),
        _ => true,
    })?;
    Some(vec![format!("user op {}: machine {}, kuna {}", n + 1, show(m.get(n)), show(g.get(n)))])
}

/// Same op at the same place with the same inputs. Past the machine op's own
/// inputs kuna attaches the registers a system call reads: the number register
/// and then some of the argument registers in order, each matched against
/// what the machine's register file holds in it.
fn same_event(m: &UserEvent, g: &UserEvent, regs: &[Storage]) -> bool {
    if m.at != g.at || m.name != g.name || g.ins.len() < m.ins.len() || m.ins[..] != g.ins[..m.ins.len()] {
        return false;
    }
    let extra = &g.ins[m.ins.len()..];
    if extra.is_empty() || regs.is_empty() || !crate::calls::SYSTEM_USEROPS.contains(&m.name.as_str()) {
        return extra.is_empty() || !crate::calls::SYSTEM_USEROPS.contains(&m.name.as_str());
    }
    let file: HashMap<u64, u8> = m.file.iter().copied().collect();
    let held = |st: &Storage| -> Option<u128> {
        let big = st.space.is_big_endian();
        let mut v = 0u128;
        for i in 0..st.size as u64 {
            let b = *file.get(&(st.off + i))? as u128;
            let shift = if big { (st.size as u64 - 1 - i) * 8 } else { i * 8 };
            v |= b << shift;
        }
        Some(v)
    };
    let fits = |v: u128, st: &Storage| held(st).is_none_or(|h| h == v & crate::value::mask(st.size));
    if !fits(extra[0], &regs[0]) {
        return false;
    }
    let mut left = regs[1..].iter();
    extra[1..].iter().all(|&v| left.any(|st| fits(v, st)))
}

/// Whether the machine's system calls write the kernel's result registers:
/// everywhere but where `syscallregs auto` leaves them unwritten, on an image
/// that names no operating system.
fn kernel_returns(prog: &ConsoleProgram) -> bool {
    use kuna_decomp::kuna_syscallregs::{SyscallFamily, SyscallRegsMode};
    let arch = prog.arch();
    SyscallFamily::from_archid(&arch.archid).is_none() || arch.image_os_userland || arch.syscall_regs != SyscallRegsMode::Auto
}

/// The first observable two machine runs differ in.
fn machine_differs(
    prog: &ConsoleProgram,
    env: &Env,
    run: u32,
    calls: &Calls,
    claim: &abi::Claim,
    a: &machine::Outcome,
    b: &machine::Outcome,
) -> Option<String> {
    if a.noreturn != b.noreturn {
        return Some("run differs in whether it returns".into());
    }
    if let Some(out) = claim.storage.as_ref().filter(|_| !a.noreturn) {
        let (mut sa, mut sb) = (a.state.clone(), b.state.clone());
        if let (Ok(x), Ok(y)) = (sa.read_storage(&out.space, out.off, out.size), sb.read_storage(&out.space, out.off, out.size)) {
            if x != y {
                return Some(format!("return is {} instead of {}", hex(y, out.size), hex(x, out.size)));
            }
        }
    }
    if let Some(l) = compare_calls(prog, env, run, calls, &a.calls, &b.calls).differs {
        return Some(format!("calls differ: {}", l.join("; ")));
    }
    compare_writes(env, run, &a.state.writes(), &b.state.writes(), None).map(|d| format!("memory at exit differs: {d}"))
}

/// The first observable two runs of kuna's side differ in.
fn depends(
    prog: &ConsoleProgram,
    env: &Env,
    run: u32,
    calls: &Calls,
    claim: &abi::Claim,
    a: &graph::Outcome,
    b: &graph::Outcome,
) -> Option<String> {
    if a.noreturn != b.noreturn {
        return Some("whether it returns".into());
    }
    if let (Some(out), Some(x), Some(y)) = (&claim.storage, a.ret, b.ret) {
        if x & mask(out.size) != y & mask(out.size) {
            return Some(format!("its return {} or {}", hex(x, out.size), hex(y, out.size)));
        }
    }
    if let Some(l) = compare_calls(prog, env, run, calls, &a.calls, &b.calls).differs {
        return Some(l.join("; "));
    }
    compare_writes(env, run, &a.writes, &b.writes, None).map(|d| format!("memory at exit {d}"))
}

/// Whether the machine overwrote the return address the function started
/// with: past the frame C defines, as an overrun of a local does.
fn smashed(env: &Env, ms: &State) -> bool {
    let Some(ret) = env.ret_addr else { return false };
    env.tracked
        .iter()
        .filter(|&&(sp, _, _, v)| v == ret && sp == env.ram.get_index())
        .filter_map(|&(sp, off, size, _)| env.prog.arch().manage().get_space(sp).map(|s| (Rc::clone(s), off, size)))
        .any(|(s, off, size)| ms.wrote(&s, off, size))
}

fn input_line(prog: &ConsoleProgram, env: &Env, run: u32, claim: &Claim) -> String {
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
pub fn check_function(prog: &ConsoleProgram, fd: &Funcdata, contract: Option<&Contract>, opts: &Options) -> Report {
    let started = Instant::now();
    let finish = |verdict: Verdict, lines: Vec<String>, suspects: Vec<String>| Report {
        function: fd.get_name().to_string(),
        verdict,
        lines,
        suspects,
        elapsed: started.elapsed(),
    };
    let mut env = match Env::new(prog, opts.seed) {
        Ok(e) => e,
        Err(h) => return finish(Verdict::from_halt(&h), Vec::new(), Vec::new()),
    };
    env.fd = Some(fd);
    let claim = abi::claim(prog, fd);
    if let Some(out) = claim
        .storage
        .as_ref()
        .filter(|o| matches!(o.space.get_type(), spacetype::IPTR_INTERNAL | spacetype::IPTR_CONSTANT))
    {
        let at = format!("{}:{:#x}", out.space.get_name(), out.off);
        return finish(Verdict::Unknown(format!("UNKNOWN(return storage {at} is not machine state)")), Vec::new(), Vec::new());
    }
    env.extensions = abi::extensions(fd, &env, prog);
    env.tracked = tracked(prog, fd, &mut env);
    env.booleans = booleans(fd, &env);
    env.kernel.set(kernel_returns(prog));
    let contract_storage = match contract.filter(|c| !c.ret.is_void()) {
        Some(c) => match abi::return_storage(prog, fd, &c.ret) {
            Ok(s) => Some(s),
            Err(e) => return finish(Verdict::Unknown(format!("UNKNOWN(contract: {e})")), Vec::new(), Vec::new()),
        },
        None => None,
    };
    let compare = Compare { prog, claim: &claim, contract: contract.zip(contract_storage.as_ref()) };
    let mut upper = UpperWatch::new(prog, fd, &claim);
    let calls = calls::sites(fd, &env);
    let program: Program = program::compile(prog, fd, &calls);
    let entry = fd.get_address().get_offset();
    let left_cfg = |mo: &machine::Outcome| mo.indirect.iter().any(|t| !program.starts.contains(t));
    let mut agreed = 0u32;
    let mut trapped_runs = 0u32;
    let mut trapped: Option<String> = None;
    let mut budget_runs = 0u32;
    let mut unsupported_runs = 0u32;
    let mut unknown: Option<Halt> = None;
    let mut diverged: Option<&'static str> = None;
    let mut call_suspects: Vec<String> = Vec::new();
    let perturb = unassigned(prog, &env, fd, &claim, &program, &calls);
    let mut perturbed_runs = 0u32;
    let mut kernel_shows = false;
    for run in 0..opts.runs {
        let m = machine::run(&env, run, entry, opts.budget, &calls, None);
        let replay: &[Record] = m.as_ref().map(|mo| mo.calls.as_slice()).unwrap_or(&[]);
        let g = graph::run(&env, &program, run, opts.budget, &calls, replay, None);
        let (mo, go) = match (m, g) {
            (Err(Halt::Trap(t)), Err(Halt::Trap(_))) => {
                trapped.get_or_insert(t);
                trapped_runs += 1;
                continue;
            }
            (Err(Halt::Trap(t)), Ok(_)) if t.starts_with(crate::state::READONLY_STORE) => {
                trapped.get_or_insert(t);
                trapped_runs += 1;
                continue;
            }
            (Ok(mo), Err(Halt::NoRow(_))) if left_cfg(&mo) => {
                diverged.get_or_insert(divergence::SWITCH_RUNON.id);
                continue;
            }
            (Err(h), _) | (_, Err(h)) => {
                match h {
                    Halt::Budget => budget_runs += 1,
                    Halt::Unsupported(_) => unsupported_runs += 1,
                    _ => {}
                }
                unknown.get_or_insert(h);
                if budget_runs >= opts.budget_runs || unsupported_runs >= UNSUPPORTED_RUNS {
                    break;
                }
                continue;
            }
            (Ok(mo), Ok(go)) => {
                if diverged.is_none() && !env.kernel.get() && mo.events.iter().any(|e| calls::SYSTEM_USEROPS.contains(&e.name.as_str())) {
                    env.kernel.set(true);
                    let alt = machine::run(&env, run, entry, opts.budget, &calls, None);
                    env.kernel.set(false);
                    if alt.is_ok_and(|alt| machine_differs(prog, &env, run, &calls, &claim, &mo, &alt).is_some()) {
                        kernel_shows = true;
                    }
                }
                if go.unmatched_switch && left_cfg(&mo) {
                    diverged.get_or_insert(divergence::SWITCH_RUNON.id);
                    continue;
                }
                if go.overlapping_copy {
                    diverged.get_or_insert(divergence::COPY_OVERLAP.id);
                    continue;
                }
                (mo, go)
            }
        };
        let differs = |mut lines: Vec<String>| {
            let at = usize::from(lines.first().is_some_and(|l| l.starts_with("contract")));
            lines.insert(at, input_line(prog, &env, run, &claim));
            for (site, callee) in &mo.followed {
                lines.push(format!("note     the machine ran {callee:#x} from {site:#x} itself: kuna has no call there"));
            }
            if opts.trace {
                for (side, recs) in [("machine", &mo.calls), ("kuna   ", &go.calls)] {
                    for c in recs.iter() {
                        let known = &calls.site(c.at).args;
                        let args: Vec<String> = c
                            .args
                            .iter()
                            .enumerate()
                            .map(|(i, a)| if matches!(known.get(i), Some(Some(_))) { format!("{a:#x}") } else { "?".into() })
                            .collect();
                        lines.push(format!("trace    {side} call at {:#x} to {} ({})", c.at, target(c.target), args.join(", ")));
                    }
                }
            }
            lines
        };
        for &(at, o) in &mo.convention {
            if call_suspects.len() >= 4 {
                break;
            }
            let reg = Storage::new(&env.register, o & !(u64::from(env.sp.1) - 1), env.sp.1).name(prog);
            let s = format!("the callee at {at:#x} changes {reg}, which the calling convention says it preserves");
            if !call_suspects.contains(&s) {
                call_suspects.push(s);
            }
        }
        if ptrauth_trap(&mo.events, &go.events) {
            diverged.get_or_insert(divergence::PTRAUTH_TRAP.id);
            continue;
        }
        let mut cc = compare_calls(prog, &env, run, &calls, &mo.calls, &go.calls);
        if let Some(ci) = go.call_input.as_ref().filter(|_| cc.differs.is_none() && cc.declared.is_none()) {
            if let Ok(alt) = machine::run(&env, run, entry, opts.budget, &calls, Some(&ci.with)) {
                let callee = ci.with.call.and_then(|k| callee_differs(prog, &calls, mo.calls.get(k), alt.calls.get(k)));
                if let Some(d) = machine_differs(prog, &env, run, &calls, &claim, &mo, &alt).or(callee) {
                    let l = format!("{}; given kuna's arguments, the machine's {d}", ci.msg);
                    if ci.locked {
                        cc.suspects.push(format!("{l}; the callee's declared prototype does not take it"));
                    } else {
                        cc.differs = Some(vec![l]);
                    }
                }
            }
        }
        if cc.differs.is_none() && cc.declared.is_none() {
            cc.differs = compare_events(prog, &mo.events, &go.events);
        }
        let canary = cc.suspects.iter().any(|s| s.starts_with("canary: "));
        if canary {
            unknown.get_or_insert(Halt::Undefined("a call the model let write the frame changed the stack canary".into()));
            continue;
        }
        let suspect = !cc.suspects.is_empty();
        let doubtful = suspect || mo.guessed;
        let doubt = |lines: &[String]| if suspect { Halt::Doubtful(lines.join("; ")) } else { Halt::Guessed(lines.join("; ")) };
        let undefined = (go.read_uninitialized && (mo.state.read_uninitialized || mo.state.read_saved)) || smashed(&env, &mo.state);
        for s in cc.suspects {
            if call_suspects.len() < 4 && !call_suspects.contains(&s) {
                call_suspects.push(s);
            }
        }
        if let Some(id) = cc.declared {
            diverged.get_or_insert(id);
            continue;
        }
        if let Some(lines) = cc.differs {
            if undefined {
                unknown.get_or_insert(Halt::Undefined(lines.join("; ")));
                continue;
            }
            if doubtful {
                unknown.get_or_insert(doubt(&lines));
                continue;
            }
            return finish(Verdict::Differs { run, of: opts.runs }, differs(lines), call_suspects);
        }
        if mo.noreturn != go.noreturn && (doubtful || undefined) {
            unknown.get_or_insert(doubt(&["one side returns".to_string()]));
            continue;
        }
        if mo.noreturn != go.noreturn {
            let which = if mo.noreturn { "the machine stops in a no-return call, kuna returns" } else { "kuna stops in a no-return call, the machine returns" };
            return finish(Verdict::Differs { run, of: opts.runs }, differs(vec![which.to_string()]), Vec::new());
        }
        let mut ms = mo.state;
        let outcome = if mo.noreturn { Ok(Outcome::Same) } else { compare.run(&mut ms, go.ret) };
        let outcome = match outcome {
            Ok(o @ (Outcome::Same | Outcome::Declared(_))) => match compare_writes(&env, run, &ms.writes(), &go.writes, None) {
                Some(d) => Ok(Outcome::Differs(vec![format!("memory at exit: {d}")])),
                None => Ok(o),
            },
            other => other,
        };
        match outcome {
            Err(h) => {
                unknown.get_or_insert(h);
                continue;
            }
            Ok(Outcome::Declared(id)) => {
                diverged.get_or_insert(id);
            }
            Ok(Outcome::Differs(lines)) if undefined => {
                unknown.get_or_insert(Halt::Undefined(lines.join("; ")));
                continue;
            }
            Ok(Outcome::Differs(lines)) if doubtful => {
                unknown.get_or_insert(doubt(&lines));
                continue;
            }
            Ok(Outcome::Differs(lines)) => {
                return finish(Verdict::Differs { run, of: opts.runs }, differs(lines), call_suspects);
            }
            Ok(Outcome::Same) => {}
        }
        if kernel_shows {
            diverged.get_or_insert(divergence::SYSCALL_RESULT.id);
        }
        if let Some(w) = upper.as_mut().filter(|_| !mo.noreturn) {
            w.observe(&mut ms);
        }
        if perturbed_runs < PERTURBED_RUNS && !undefined && !perturb.is_empty() {
            perturbed_runs += 1;
            let replay: &[Record] = mo.calls.as_slice();
            for u in &perturb {
                let Ok(gp) = graph::run(&env, &program, run, opts.budget, &calls, replay, Some(&u.perturb)) else { continue };
                let Some(d) = depends(prog, &env, run, &calls, &claim, &go, &gp) else { continue };
                if u.shared {
                    let with = if u.entry.is_empty() {
                        machine::WithArgs { call: None, bytes: Vec::new(), kill_salt: 0x5a5a_5a5a }
                    } else {
                        machine::WithArgs { call: None, bytes: entry_bytes(&env, u, run), kill_salt: 0 }
                    };
                    let base = machine::run(&env, run, entry, opts.budget, &calls, None);
                    let alt = machine::run(&env, run, entry, opts.budget, &calls, Some(&with));
                    match (base, alt) {
                        (Ok(b), Ok(a)) if machine_differs(prog, &env, run, &calls, &claim, &b, &a).is_none() => {}
                        _ => continue,
                    }
                }
                let lines = vec![format!("kuna's result depends on {}, which its C never assigns: {d}", u.names)];
                return finish(Verdict::Differs { run, of: opts.runs }, differs(lines), call_suspects);
            }
        }
        agreed += 1;
    }
    let mut suspects = abi::unclaimed_inputs(prog, &env, fd, &claim);
    suspects.extend(call_suspects);
    if agreed == opts.runs {
        if let Some(w) = upper {
            suspects.extend(w.suspects(prog));
        }
    }
    let verdict = match (unknown, diverged, trapped) {
        (Some(h), _, _) => Verdict::from_halt(&h),
        (None, Some(d), _) => Verdict::Diverges(d),
        (None, None, Some(t)) if agreed == 0 => Verdict::from_halt(&Halt::Trap(t)),
        (None, None, _) => Verdict::Agrees(agreed),
    };
    let mut lines = match contract {
        Some(c) if matches!(verdict, Verdict::Agrees(_)) => vec![format!("contract {}  ({})", c.declaration, c.source)],
        _ => Vec::new(),
    };
    if trapped_runs > 0 && matches!(verdict, Verdict::Agrees(_)) {
        lines.push(format!(
            "note     {trapped_runs} runs trapped on both sides or stored into read-only memory; C leaves them undefined, so they are not compared"
        ));
    }
    finish(verdict, lines, suspects)
}
