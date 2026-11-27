//! What the two sides did, compared: calls, memory, system calls and traps,
//! and the reruns that check whether a difference shows on the machine.

//! Tier C: run both sides of one function and compare what they do.
//!
//! A run compares, in order: each call (its target, kuna's arguments, and the
//! memory the callee can read), whether the function returns at all, the return
//! value, and the memory left outside the stack. The return value is compared
//! under the physical ABI (the bytes kuna's claimed return occupies) and, when a
//! typed contract is given, at the contract's width with kuna's value converted
//! to the contract type by C's rules.

use std::collections::HashMap;
use std::rc::Rc;

use kuna_base::space::spacetype;
use kuna_console::engine::ConsoleProgram;

use crate::abi::{self};
use crate::divergence::{self};
use crate::calls::{Calls, Record, UserEvent};
use crate::state::{Env, State, Writes};
use crate::storage::{hex, Storage};
use crate::value::{mask, Halt};
use crate::{graph, machine};
use super::returns::{Compare, ReturnValue};

/// The first byte the two sides left different, of the memory either wrote.
/// `read`: the frame bytes a callee run for real read, when only those count.
pub(crate) fn compare_writes(env: &Env, run: u32, machine: &Writes, kuna: &Writes, read: Option<&crate::state::FrameReads>) -> Option<String> {
    let manage = env.prog.arch().manage();
    let ram = env.ram.get_index();
    let compared = |sp: i32, off: u64| {
        let both = |o: &(u64, u64)| off >= o.0 && off < o.1 && kuna.objects.contains(o);
        let past = |o: &(u64, u64)| off >= o.1 && off < env.sp0 && kuna.objects.contains(o);
        let reached = machine.objects.iter().any(both) || read.is_some_and(|r| r.contains(&off) && machine.objects.iter().any(past));
        !(sp == ram && env.in_stack(off, 1))
            || (reached && read.is_none_or(|r| r.contains(&off)) && !machine.skip.contains(&off) && !kuna.skip.contains(&off))
    };
    let mut first = None;
    let mut differ = 0usize;
    let mut ms = machine.bytes.iter().peekable();
    let mut ks = kuna.bytes.iter().peekable();
    loop {
        let (key, m, k) = match (ms.peek(), ks.peek()) {
            (None, None) => break,
            (Some(&(&mk, &mv)), Some(&(&kk, &kv))) if mk == kk => {
                ms.next();
                ks.next();
                (mk, Some(mv), Some(kv))
            }
            (Some(&(&mk, &mv)), Some(&(&kk, _))) if mk < kk => {
                ms.next();
                (mk, Some(mv), None)
            }
            (Some(&(&mk, &mv)), None) => {
                ms.next();
                (mk, Some(mv), None)
            }
            (_, Some(&(&kk, &kv))) => {
                ks.next();
                (kk, None, Some(kv))
            }
        };
        let (sp, off) = key;
        if !compared(sp, off) {
            continue;
        }
        let Some(space) = manage.get_space(sp) else { continue };
        let m = m.unwrap_or_else(|| env.base_byte(run, machine.epoch, &machine.escaped, space, off));
        let k = k.unwrap_or_else(|| env.base_byte(run, kuna.epoch, &kuna.escaped, space, off));
        if m != k {
            differ += 1;
            first.get_or_insert(format!("{}:{off:#x}: machine 0x{m:02x}, kuna 0x{k:02x}", space.get_name()));
        }
    }
    first.map(|f| if differ > 1 { format!("{f} ({differ} bytes differ)") } else { f })
}

/// A computed call target as an address in the code space: wrapped to its
/// size, and without the ARM or MIPS instruction-set bit.
pub(crate) fn code_target(env: &Env, t: Option<u64>) -> Option<u64> {
    let id = &env.prog.arch().archid;
    let isa_bit = id.starts_with("ARM") || id.starts_with("MIPS");
    t.map(|t| env.code.wrap_offset(t) & if isa_bit { !1 } else { !0 })
}

pub(crate) fn target(t: Option<u64>) -> String {
    t.map_or_else(|| "?".to_string(), |t| format!("{t:#x}"))
}

/// Stack-protector failure handlers, without leading underscores. A call to one
/// on the machine side only means a call the model let rewrite the frame hit
/// the canary.
pub(crate) const STACK_PROTECTOR: &[&str] = &["stack_chk_fail", "security_check_cookie", "report_gsfailure"];

/// How the two sides' calls compare: the first difference, a declared
/// divergence among them, whether it ends what the run can compare, and the
/// arguments a recovered callee prototype may have wrong.
pub(crate) struct CallCompare {
    pub(crate) differs: Option<Vec<String>>,
    pub(crate) declared: Option<&'static str>,
    /// The declared divergence changes what follows it (a check the machine
    /// fails and panics in), so the rest of the run is not compared.
    pub(crate) stop: bool,
    pub(crate) suspects: Vec<String>,
}

/// Compare the calls in order. A call kuna synthesized, and a Go write
/// barrier the machine calls where kuna stores directly, are declared: what
/// either side does after one is not comparable (a stub stands where the
/// other side ran code), so the calls before the first are compared and the
/// rest of the run is not.
pub(crate) fn compare_calls(prog: &ConsoleProgram, env: &Env, run: u32, calls: &Calls, m: &[Record], g: &[Record]) -> CallCompare {
    let mut suspects = Vec::new();
    let synthesized = g.iter().position(|c| c.at == 0 || (!c.indirect && c.target.is_none()));
    let barrier = m.iter().position(|c| {
        c.target.and_then(|t| prog.function_named_at(t)).is_some_and(|n| n.starts_with("runtime.gcWriteBarrier"))
            && !g.iter().any(|k| k.at == c.at)
    });
    let declared = match (synthesized, barrier) {
        (Some(s), Some(b)) if b < s => Some((b, divergence::GO_WRITE_BARRIER.id)),
        (Some(s), _) => Some((s, divergence::SYNTHESIZED_CALL.id)),
        (None, Some(b)) => Some((b, divergence::GO_WRITE_BARRIER.id)),
        (None, None) => None,
    };
    let cut = declared.map_or(usize::MAX, |(k, _)| k);
    let (m, g) = (&m[..cut.min(m.len())], &g[..cut.min(g.len())]);
    let declared = declared.map(|(_, id)| id);
    let stop = declared.is_some();
    let differs = |lines: Vec<String>, suspects: Vec<String>| CallCompare { differs: Some(lines), declared: None, stop: false, suspects };
    for k in 0..m.len().max(g.len()) {
        let n = k + 1;
        let (mc, gc) = match (m.get(k), g.get(k)) {
            (Some(mc), Some(gc)) => (mc, gc),
            (Some(mc), None) => {
                let name = mc.target.and_then(|t| prog.function_named_at(t)).unwrap_or_default();
                if kuna_decomp::kuna_gobounds::is_go_check_name(&name) || kuna_decomp::kuna_securitycheck::is_security_check_name(&name) {
                    return CallCompare { differs: None, declared: Some(divergence::PANIC_CHECK.id), stop: true, suspects };
                }
                let l = format!("call {n} at {:#x} to {}: the machine makes it, kuna does not", mc.at, target(mc.target));
                if STACK_PROTECTOR.contains(&name.trim_start_matches('_')) {
                    return CallCompare { differs: None, declared, stop, suspects: vec![format!("canary: {l}")] };
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
        let (mr, kr) = (code_target(env, mc.target), code_target(env, gc.target));
        let (mt, kt) = (mr.map(|t| machine::transparent_target(env, t)), kr.map(|t| machine::transparent_target(env, t)));
        let direct = !mc.indirect && !gc.indirect && !calls.site(gc.at).resolved;
        let code_or_outside = |t: u64| env.is_code(t) || !env.code_mapped(t);
        let both = |a: Option<u64>, b: Option<u64>| a.zip(b).is_some_and(|(m, k)| code_or_outside(m) && code_or_outside(k));
        let comparable = both(mr, kr) || both(mt, kt);
        let resolved = calls.site(gc.at).resolved;
        let both_code = mt.zip(kt).is_some_and(|(m, k)| env.is_code(m) && env.is_code(k));
        let mixed = mc.indirect != gc.indirect && !resolved && both_code;
        let compared = (mc.indirect && gc.indirect) || (direct && comparable) || mixed;
        let same = mr == kr || (mt == kt && mt.is_some_and(|t| env.is_code(t)));
        if compared && !same {
            let l = format!("call {n} at {:#x}: the machine calls {}, kuna calls {}", gc.at, target(mc.target), target(gc.target));
            return differs(vec![l], suspects);
        }
        let site = calls.site(gc.at);
        if let Some(stub) = site.plt_stub.filter(|&s| !mc.indirect && !reaches_stub(env, mc.target, s)) {
            let l = format!("call {n} at {:#x}: the machine calls {}, kuna binds the call past the PLT stub at {stub:#x}", gc.at, target(mc.target));
            return differs(vec![l], suspects);
        }
        if gc.args.len() < site.params {
            let l = format!(
                "call {n} to {}: kuna passes {} arguments, and the prototype it gives the callee takes {}",
                target(gc.target),
                gc.args.len(),
                site.params
            );
            return differs(vec![l], suspects);
        }
        for (i, ((ma, ka), arg)) in mc.args.iter().zip(&gc.args).zip(&site.args).enumerate() {
            let Some(st) = arg.as_ref().map(|a| &a.storage) else { continue };
            if ma & mask(st.size) == ka & mask(st.size) {
                continue;
            }
            if same && site.stack_input(i).is_some_and(|s| {
                s.off == st.off && s.size == st.size && s.space.get_index() == st.space.get_index()
                    && mc.stack_in.as_ref().is_some_and(|reads| reads.contains(s.off, s.size))
            }) {
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
    CallCompare { differs: None, declared, stop, suspects }
}

/// Compare the recorded effects of two completed runs of the same callee.
pub(super) fn callee_differs(
    prog: &ConsoleProgram, calls: &Calls, a: Option<&Record>, b: Option<&Record>, stack: bool,
) -> Result<Option<String>, &'static str> {
    let (Some(a), Some(b)) = (a, b) else { return Err("the callee was not reached in both runs") };
    if a.at != b.at || a.target != b.target {
        return Err("the replay reached a different callee");
    }
    let (Some(ea), Some(eb)) = (&a.effect, &b.effect) else { return Err("the callee did not complete a concrete replay") };
    if stack && (a.stack_in.is_none() || b.stack_in.is_none()) {
        return Err("the callee did not complete a concrete stack-argument replay");
    }
    let site = calls.site(a.at);
    if let Some(out) = site.output.as_ref().filter(|o| o.space.get_type() == spacetype::IPTR_PROCESSOR) {
        let value = ReturnValue::new(prog, out, site.output_float);
        let st = value.physical();
        let within = |r: &&(u64, u8)| r.0 >= st.off && r.0 < st.off + u64::from(st.size);
        let (ra, rb): (Vec<_>, Vec<_>) = (ea.regs.iter().filter(within).collect(), eb.regs.iter().filter(within).collect());
        if ra != rb {
            let get = |regs: &[&(u64, u8)]| -> Option<u128> {
                if st.size > 16 {
                    return None;
                }
                let mut raw = 0u128;
                for pos in 0..u64::from(st.size) {
                    let (_, b) = regs.iter().find(|r| r.0 == st.off + pos)?;
                    let i = if st.space.is_big_endian() { u64::from(st.size) - pos - 1 } else { pos };
                    raw |= u128::from(*b) << (8 * i);
                }
                Some(value.normalize(raw))
            };
            if !get(&ra).zip(get(&rb)).is_some_and(|(a, b)| value.agrees(a, b)) {
                return Ok(Some(format!("callee returns another {}", out.name(prog))));
            }
        }
    }
    let writes = |e: &crate::state::Effect| -> Vec<(i32, u64, u8)> {
        e.events.iter().filter_map(|ev| if let crate::state::Event::Write(s, o, b) = ev { Some((*s, *o, *b)) } else { None }).collect()
    };
    Ok((writes(ea) != writes(eb)).then(|| "callee writes other memory".to_string()))
}

/// The machine stopped in a pointer-authentication trap kuna does not reach.
pub(crate) fn ptrauth_trap(m: &[UserEvent], g: &[UserEvent]) -> bool {
    m.last().is_some_and(|e| {
        e.name.as_bytes() == kuna_decomp::kuna_ptrauthtrap::BREAKPOINT_USEROP
            && e.ins.first().is_some_and(|&i| kuna_decomp::kuna_ptrauthtrap::is_ptrauth_trap_immediate(i as u64))
            && !g.iter().any(|k| k.at == e.at)
    })
}

/// The machine's call goes to `stub`, directly or through thunks.
pub(crate) fn reaches_stub(env: &Env, machine: Option<u64>, stub: u64) -> bool {
    machine == Some(stub)
        || code_target(env, machine).is_some_and(|m| m == stub || machine::transparent_target(env, m) == machine::transparent_target(env, stub))
}

/// The first system call or trap the two sides reach differently.
pub(crate) fn compare_events(prog: &ConsoleProgram, m: &[UserEvent], g: &[UserEvent]) -> Option<Vec<String>> {
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

/// How kuna's system-call inputs match the machine's registers.
#[derive(PartialEq)]
enum EventMatch {
    Same,
    /// The values match registers in order, but not the first ones: kuna's
    /// IR skips a register, and the order its C passes them in does not say
    /// which.
    Gap,
    Differs,
}

/// Same op at the same place with the same inputs. Past the machine op's own
/// inputs kuna attaches the registers a system call reads: the number register
/// and then some of the argument registers in order, each matched against
/// what the machine's register file holds in it.
pub(crate) fn same_event(m: &UserEvent, g: &UserEvent, regs: &[Storage]) -> bool {
    match_event(m, g, regs) != EventMatch::Differs
}

fn match_event(m: &UserEvent, g: &UserEvent, regs: &[Storage]) -> EventMatch {
    if m.at != g.at || m.name != g.name || g.ins.len() < m.ins.len() || m.ins[..] != g.ins[..m.ins.len()] {
        return EventMatch::Differs;
    }
    let extra = &g.ins[m.ins.len()..];
    if extra.is_empty() || regs.is_empty() || !crate::calls::SYSTEM_USEROPS.contains(&m.name.as_str()) {
        return if extra.is_empty() || !crate::calls::SYSTEM_USEROPS.contains(&m.name.as_str()) { EventMatch::Same } else { EventMatch::Differs };
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
    let fits = |v: u128, st: &Storage| held(st).is_some_and(|h| h == v & crate::value::mask(st.size));
    if extra.len() > regs.len() || !fits(extra[0], &regs[0]) {
        return EventMatch::Differs;
    }
    if extra[1..].iter().zip(&regs[1..]).all(|(&v, st)| fits(v, st)) {
        return EventMatch::Same;
    }
    let mut left = regs[1..].iter();
    if extra[1..].iter().all(|&v| left.any(|st| fits(v, st))) {
        EventMatch::Gap
    } else {
        EventMatch::Differs
    }
}

/// The first system call whose inputs kuna takes from the registers with a
/// gap: its IR reads them, but its C's argument order does not say which.
pub(crate) fn event_gap(prog: &ConsoleProgram, m: &[UserEvent], g: &[UserEvent]) -> Option<String> {
    if !g.iter().any(|e| crate::calls::SYSTEM_USEROPS.contains(&e.name.as_str())) {
        return None;
    }
    let regs = crate::calls::SyscallAbi::of(prog).inputs;
    m.iter().zip(g).find(|(a, b)| match_event(a, b, &regs) == EventMatch::Gap).map(|(a, b)| {
        format!(
            "system call at {:#x}: kuna passes {} of its argument registers, not the first ones in order, so its C does not say which it reads",
            a.at,
            b.ins.len().saturating_sub(a.ins.len() + 1)
        )
    })
}

/// The first observable two machine runs differ in.
pub(super) fn machine_differs(
    prog: &ConsoleProgram,
    env: &Env,
    run: u32,
    calls: &Calls,
    compare: &Compare,
    a: &machine::Outcome,
    b: &machine::Outcome,
) -> Result<Option<String>, Halt> {
    if a.noreturn != b.noreturn {
        return Ok(Some("run differs in whether it returns".into()));
    }
    if a.bad_return.is_some() != b.bad_return.is_some() {
        return Ok(Some("run differs in whether it reaches the return address".into()));
    }
    if !a.noreturn && a.copy_operands.is_none() && b.copy_operands.is_none() && a.bad_return.is_none() {
        if let Some(d) = compare.machine_return_diff(&mut a.state.clone(), &mut b.state.clone())? {
            return Ok(Some(d));
        }
    }
    if let Some(l) = compare_calls(prog, env, run, calls, &a.calls, &b.calls).differs {
        return Ok(Some(format!("calls differ: {}", l.join("; "))));
    }
    if let Some(l) = compare_events(prog, &a.events, &b.events) {
        return Ok(Some(format!("system calls differ: {}", l.join("; "))));
    }
    if a.copy_operands != b.copy_operands {
        return Ok(Some("overlapping copy operands differ".to_string()));
    }
    Ok(compare_writes(env, run, &a.state.writes(), &b.state.writes(), None).map(|d| format!("memory at exit differs: {d}")))
}

/// The first observable two runs of kuna's side differ in.
pub(crate) fn depends(
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
        let value = ReturnValue::new(prog, out, claim.float);
        if !value.agrees(x & mask(out.size), y & mask(out.size)) {
            return Some(format!("its return {} or {}", hex(x, out.size), hex(y, out.size)));
        }
    }
    if let Some(l) = compare_calls(prog, env, run, calls, &a.calls, &b.calls).differs {
        return Some(l.join("; "));
    }
    if let Some(l) = compare_events(prog, &a.events, &b.events) {
        return Some(format!("its system calls: {}", l.join("; ")));
    }
    compare_writes(env, run, &a.writes, &b.writes, None).map(|d| format!("memory at exit {d}"))
}

/// Whether the machine overwrote the return address the function started
/// with: past the frame C defines, as an overrun of a local does.
pub(crate) fn smashed(env: &Env, ms: &State) -> bool {
    let Some(ret) = env.ret_addr else { return false };
    env.tracked
        .iter()
        .filter(|&&(sp, _, _, v)| v == ret && sp == env.ram.get_index())
        .filter_map(|&(sp, off, size, _)| env.prog.arch().manage().get_space(sp).map(|s| (Rc::clone(s), off, size)))
        .any(|(s, off, size)| ms.wrote(&s, off, size))
}
