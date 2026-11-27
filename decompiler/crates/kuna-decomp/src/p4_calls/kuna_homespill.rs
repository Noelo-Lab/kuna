//! (kuna) A Windows x64 function that spills an argument register into the
//! home slot the convention reserves for it takes that argument.
//!
//! Every Windows x64 call leaves 32 bytes above the return address, one 8-byte
//! home slot per argument register: `RCX`/`XMM0` at entry `RSP+0x8`,
//! `RDX`/`XMM1` at `+0x10`, `R8`/`XMM2` at `+0x18` and `R9`/`XMM3` at `+0x20`.
//! An unoptimized MSVC, gcc or clang function begins by storing each parameter
//! it declares into its own slot, at the declared width (`mov [rsp+0x20],r9b`
//! for a `char`), and reads it back from there:
//!
//! ```text
//!   mov  byte ptr [rsp+0x20],r9b
//!   mov  dword ptr [rsp+0x18],r8d
//!   mov  dword ptr [rsp+0x10],edx
//!   mov  dword ptr [rsp+0x8],ecx
//!   sub  rsp,0x68
//! ```
//!
//! A parameter the body never reads back leaves only its spill, a store into the
//! caller's frame that nothing reads, which is gone by the time
//! `ActionInputPrototype` scores the function's inputs. The parameter went with
//! it, so a function printed with fewer parameters than its prologue reads, and
//! how many it kept depended on which reloads happened to stay live: once a
//! declared prototype told a caller that the callee takes four arguments, the
//! reload of `a3` it had been passing fifth was dead, and `void` replaced all
//! four parameters of the caller.
//!
//! [`record`] notes, right after the first heritage (registers in SSA form, the
//! stack not yet), every `STORE` of an argument register's incoming value, or
//! of its low bytes, to the entry stack pointer plus that register's own home
//! offset, unless another store of the function writes the same slot (MSVC /O2
//! keeps locals in home slots). [`mark`] makes each noted register an active
//! input trial of the function's own recovery, at the width it was stored, so
//! the parameter is recovered whether or not anything reads the slot back. A
//! store of one register into another's slot, a value the function computed,
//! and a spill of a callee-saved register are not noted, and the function's own
//! model still has to accept the storage as a parameter. Neither is a variadic
//! function's register save ([`vararg_save`]): the whole registers from the
//! first unnamed argument's slot through `R9`'s, whose first slot's address
//! `va_start` takes, or which gcc stores without one below a slot left
//! unwritten. Inert off Windows x64.

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

use crate::context::{OpId, VarnodeId};
use crate::fspec::ParamActive;
use crate::funcdata::Funcdata;

/// The argument registers of each home slot, in slot order.
const HOMES: [[&str; 2]; 4] =
    [["RCX", "XMM0_Qa"], ["RDX", "XMM1_Qa"], ["R8", "XMM2_Qa"], ["R9", "XMM3_Qa"]];

/// Most copies and low pieces one register's value is followed through.
const FAMILY_CAP: usize = 16;

/// Most ops one store address is walked back through to the stack pointer.
const ADDRESS_DEPTH: usize = 8;

struct Home {
    space: Rc<AddrSpace>,
    offset: u64,
    slot: i64,
    int: bool,
}

/// Note the home-slot spills of `fd`; called once, after the first heritage.
pub(crate) fn record(fd: &mut Funcdata) {
    if fd.get_heritage_pass() != 1 || !fd.get_arch().win64_home_slots {
        return;
    }
    fd.kuna_home_spills = spills(fd);
}

/// Make each noted spill an active trial of the function's own input recovery.
pub(crate) fn mark(fd: &Funcdata, active: &mut ParamActive) {
    for (addr, size) in &fd.kuna_home_spills {
        if !fd.get_func_proto().possible_input_param(addr, *size) {
            continue;
        }
        let mut covered = false;
        for i in 0..active.get_num_trials() {
            let t = active.get_trial(i);
            if overlaps(t.get_address(), t.get_size(), addr, *size) {
                covered = true;
                if !t.is_active() {
                    active.get_trial_mut(i).mark_active();
                }
            }
        }
        if !covered {
            let slot = active.get_num_trials();
            active.register_trial(addr, *size);
            let t = active.get_trial_mut(slot);
            t.mark_unref();
            t.mark_active();
        }
    }
}

fn overlaps(a: &Address, asize: int4, b: &Address, bsize: int4) -> bool {
    a.get_space().map(|s| s.get_index()) == b.get_space().map(|s| s.get_index())
        && a.get_offset() < b.get_offset() + bsize as u64
        && b.get_offset() < a.get_offset() + asize as u64
}

fn homes(fd: &Funcdata) -> Vec<Home> {
    let Some(lookup) = fd.get_arch().manage().register_lookup() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (i, names) in HOMES.iter().enumerate() {
        for (j, name) in names.iter().enumerate() {
            if let Some(r) = lookup.probe_register(name) {
                if let Some(space) = r.space {
                    out.push(Home { space, offset: r.offset, slot: 8 * (i as i64 + 1), int: j == 0 });
                }
            }
        }
    }
    out
}

fn stack_pointer(fd: &Funcdata, inputs: &[VarnodeId]) -> Option<VarnodeId> {
    let rsp = fd.get_arch().manage().register_lookup()?.probe_register("RSP")?;
    let space = rsp.space?.get_index();
    inputs.iter().copied().find(|&vn| {
        fd.vbank().get(vn).is_some_and(|v| {
            v.get_space().get_index() == space && v.get_offset() == rsp.offset && v.get_size() == 8
        })
    })
}

fn spills(fd: &Funcdata) -> Vec<(Address, int4)> {
    let inputs: Vec<VarnodeId> =
        fd.vbank().iter_def_flag(crate::varnode::varnode_flags::input).collect();
    let Some(sp) = stack_pointer(fd, &inputs) else {
        return Vec::new();
    };
    let homes = homes(fd);
    let mut found: Vec<(OpId, i64, Address, int4)> = Vec::new();
    for vn in inputs {
        let Some(v) = fd.vbank().get(vn) else { continue };
        let Some(home) = homes.iter().find(|h| {
            h.space.get_index() == v.get_space().get_index() && h.offset == v.get_offset()
        }) else {
            continue;
        };
        for (value, size) in low_pieces(fd, vn) {
            if size > 8 {
                continue;
            }
            let Some(val) = fd.vbank().get(value) else { continue };
            for op in val.descend_iter() {
                let Some(o) = fd.obank().get(op) else { continue };
                if o.code() == OpCode::CPUI_STORE
                    && o.get_in(2) == Some(value)
                    && o.get_in(1).and_then(|a| sp_offset(fd, a, sp)) == Some(home.slot)
                {
                    found.push((op, home.slot, Address::new(home.space.clone(), home.offset), size));
                }
            }
        }
    }
    if found.is_empty() {
        return Vec::new();
    }
    let mut reused: Vec<i64> = Vec::new();
    let mut written = [false; 4];
    for (_, slot, ..) in &found {
        written[index(*slot)] = true;
    }
    for op in fd.obank().iter_code(OpCode::CPUI_STORE) {
        if found.iter().any(|(spill, ..)| *spill == op) {
            continue;
        }
        let Some(o) = fd.obank().get(op) else { continue };
        let Some(size) = o.get_in(2).and_then(|v| fd.vbank().get(v)).map(|v| v.get_size() as i64) else {
            continue;
        };
        let Some(at) = o.get_in(1).and_then(|a| sp_offset(fd, a, sp)) else { continue };
        for (i, w) in written.iter_mut().enumerate() {
            let slot = 8 * (i as i64 + 1);
            *w |= at < slot + 8 && slot < at + size;
        }
        for (_, slot, ..) in &found {
            if at < slot + 8 && *slot < at + size && !reused.contains(slot) {
                reused.push(*slot);
            }
        }
    }
    let save = vararg_save(fd, sp, &homes, &found, &written);
    let mut out: Vec<(Address, int4)> = Vec::new();
    for (_, slot, addr, size) in found {
        if save.is_some_and(|k| slot >= k) {
            continue;
        }
        if !reused.contains(&slot) && !out.iter().any(|(a, s)| *a == addr && *s == size) {
            out.push((addr, size));
        }
    }
    out
}

fn index(slot: i64) -> usize {
    (slot / 8 - 1) as usize
}

/// The first slot of a variadic function's register save: every slot from it
/// through `R9`'s holds the whole integer register, and `va_start` takes that
/// slot's address or, when no home slot's address is taken, the function writes
/// no slot below it (gcc saves the registers of every variadic function, also
/// when it never calls `va_start`). Those registers carry varargs.
fn vararg_save(
    fd: &Funcdata,
    sp: VarnodeId,
    homes: &[Home],
    found: &[(OpId, i64, Address, int4)],
    written: &[bool; 4],
) -> Option<i64> {
    let whole = |slot: i64| {
        found.iter().any(|(_, s, addr, size)| {
            *s == slot
                && *size == 8
                && homes.iter().any(|h| h.slot == slot && h.int && h.offset == addr.get_offset())
        })
    };
    if !whole(0x20) {
        return None;
    }
    let escaped = escaped_homes(fd, sp);
    let taken = escaped.contains(&true);
    (1..=4).map(|i| 8 * i).find(|&k| {
        (k..=0x20).step_by(8).all(whole)
            && if taken { escaped[index(k)] } else { k > 8 && !written[..index(k)].contains(&true) }
    })
}

/// The home slots whose address the function uses other than to load or store
/// through it, as `va_start` and an out-parameter pointer do.
fn escaped_homes(fd: &Funcdata, sp: VarnodeId) -> [bool; 4] {
    let mut escaped = [false; 4];
    for op in fd.obank().iter_alive() {
        let Some(o) = fd.obank().get(op) else { continue };
        if !matches!(o.code(), OpCode::CPUI_COPY | OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB) {
            continue;
        }
        let Some(out) = o.get_out() else { continue };
        let Some(at) = sp_offset(fd, out, sp) else { continue };
        if (8..0x28).contains(&at) && !escaped[index(at)] && escapes(fd, out) {
            escaped[index(at)] = true;
        }
    }
    escaped
}

fn escapes(fd: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = fd.vbank().get(vn) else { return false };
    v.descend_iter().any(|op| {
        let Some(o) = fd.obank().get(op) else { return true };
        let constant = |slot: int4| {
            o.get_in(slot).and_then(|c| fd.vbank().get(c)).is_some_and(|c| c.is_constant())
        };
        match o.code() {
            OpCode::CPUI_LOAD => o.get_in(1) != Some(vn),
            OpCode::CPUI_STORE => o.get_in(1) != Some(vn) || o.get_in(2) == Some(vn),
            OpCode::CPUI_COPY => false,
            OpCode::CPUI_INT_ADD => !(constant(0) || constant(1)),
            OpCode::CPUI_INT_SUB => !constant(1),
            _ => true,
        }
    })
}

/// `vn` and the values that hold its low bytes unchanged: copies, and the
/// `SUBPIECE` at offset 0 that reads `ecx` or `r9b` out of a wider input.
fn low_pieces(fd: &Funcdata, vn: VarnodeId) -> Vec<(VarnodeId, int4)> {
    let Some(size) = fd.vbank().get(vn).map(|v| v.get_size()) else {
        return Vec::new();
    };
    let mut family = vec![(vn, size)];
    let mut i = 0;
    while i < family.len() && family.len() < FAMILY_CAP {
        let (cur, _) = family[i];
        i += 1;
        let Some(v) = fd.vbank().get(cur) else { continue };
        for op in v.descend_iter() {
            let Some(o) = fd.obank().get(op) else { continue };
            let low = match o.code() {
                OpCode::CPUI_COPY => true,
                OpCode::CPUI_SUBPIECE => o
                    .get_in(1)
                    .and_then(|c| fd.vbank().get(c))
                    .is_some_and(|c| c.is_constant() && c.get_offset() == 0),
                _ => false,
            };
            if !low || o.get_in(0) != Some(cur) {
                continue;
            }
            if let Some(out) = o.get_out() {
                if let Some(w) = fd.vbank().get(out).map(|x| x.get_size()) {
                    if !family.iter().any(|(f, _)| *f == out) {
                        family.push((out, w));
                    }
                }
            }
        }
    }
    family
}

/// The constant `vn` adds to the entry stack pointer `sp`, walked back through
/// copies and constant additions and subtractions.
fn sp_offset(fd: &Funcdata, mut vn: VarnodeId, sp: VarnodeId) -> Option<i64> {
    let mut acc: i64 = 0;
    for _ in 0..ADDRESS_DEPTH {
        if vn == sp {
            return Some(acc);
        }
        let op = fd.obank().get(fd.vbank().get(vn)?.get_def()?)?;
        let constant = |slot: int4| {
            op.get_in(slot)
                .and_then(|c| fd.vbank().get(c))
                .filter(|c| c.is_constant())
                .map(|c| c.get_offset() as i64)
        };
        match op.code() {
            OpCode::CPUI_COPY => vn = op.get_in(0)?,
            OpCode::CPUI_INT_ADD => {
                if let Some(c) = constant(1) {
                    acc = acc.wrapping_add(c);
                    vn = op.get_in(0)?;
                } else {
                    acc = acc.wrapping_add(constant(0)?);
                    vn = op.get_in(1)?;
                }
            }
            OpCode::CPUI_INT_SUB => {
                acc = acc.wrapping_sub(constant(1)?);
                vn = op.get_in(0)?;
            }
            _ => return None,
        }
    }
    None
}
