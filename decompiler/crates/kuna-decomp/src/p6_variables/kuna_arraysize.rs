//! (kuna) `arraysize` -- an escaped stack buffer is as long as the length a
//! call states for it, not the gap to the next frame object.
//!
//! `ScopeLocal::restructure` (`varmap.cc:1294`) ends an open range (a frame
//! address that escapes, usually to a call) where the next hint starts. The
//! compiler aligns a buffer of 16 bytes or more to 16, so the gap includes the
//! padding before the next object: `char buf[8192]` under the stack-protector
//! guard prints as `char buf[8200]`, `char buf[21]` as `char buf[24]`.
//!
//! With the option on, [`gather`] reads the byte count each call states for the
//! exact start of a frame address it receives: the length argument of a library
//! buffer function (`snprintf(buf, 0x400, ..)`, `fgets`, `read`, `memset`, the
//! product of `fread`'s size and count), or the destination object size a
//! `_FORTIFY_SOURCE` `__*_chk` call carries. A call that returns how many bytes
//! it stored states one byte more when the function indexes the buffer by that
//! result (`n = readlink(p, buf, 4096); buf[n] = 0;`). At `calls` any constant
//! argument of any call beside the address counts too. [`fit`] then ends the
//! open range at the largest such count instead of at the next hint, when the
//! count
//!
//! - is at least 16 bytes and a whole number of elements;
//! - lies within the 15 bytes of padding below the gap (a count that far short
//!   of the gap describes part of the buffer, not all of it);
//! - covers every hint the range already absorbed and every index bound it
//!   carries.
//!
//! A count one byte short of the gap, and a library count one byte short of a
//! multiple of 16 that still fits the gap, is the `sizeof(buf) - 1` idiom
//! (`readlink(p, buf, sizeof buf - 1)`), so the array keeps that byte. The
//! bytes past the array stay unmapped, as alignment padding is. Upstream's gap
//! is restored with `option arraysize off`.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::sign_extend;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, intb, uintb};
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use crate::varmap::{MapState, RangeHint};

/// Upstream's gap to the next hint.
pub const LEVEL_OFF: int4 = 0;
/// Counts stated by library buffer functions.
pub const LEVEL_ON: int4 = 1;
/// Also any constant argument beside the address.
pub const LEVEL_CALLS: int4 = 2;

/// Smallest array the count may size.
const MIN_BYTES: intb = 16;
/// Most bytes of alignment padding the count may leave before the gap's end.
const PADDING: intb = 15;
/// Largest count read.
const MAX_BYTES: intb = 1 << 24;

/// `option arraysize off|on|calls`.
pub struct OptionArraySize;

impl OptionArraySize {
    /// Parse the level and its confirmation message; the caller writes it into
    /// `Architecture::array_size`.
    pub fn apply(&self, p1: &str) -> KunaResult<(int4, String)> {
        match p1 {
            "off" => Ok((LEVEL_OFF, "Stack buffer sizing from call lengths turned off".to_string())),
            "on" | "" => Ok((LEVEL_ON, "Stack buffer sizing from library call lengths turned on".to_string())),
            "calls" => Ok((
                LEVEL_CALLS,
                "Stack buffer sizing from library call lengths and constant call arguments turned on".to_string(),
            )),
            _ => Err(KunaError::parse("Must specify one of off, on, calls")),
        }
    }
}

/// `(callee, buffer argument, arguments whose product is the byte count)`.
const LENGTH_CALLS: &[(&str, usize, &[usize])] = &[
    ("memset", 0, &[2]),
    ("memcpy", 0, &[2]),
    ("memcpy", 1, &[2]),
    ("memmove", 0, &[2]),
    ("memmove", 1, &[2]),
    ("mempcpy", 0, &[2]),
    ("memcmp", 0, &[2]),
    ("memcmp", 1, &[2]),
    ("bzero", 0, &[1]),
    ("explicit_bzero", 0, &[1]),
    ("read", 1, &[2]),
    ("pread", 1, &[2]),
    ("pread64", 1, &[2]),
    ("write", 1, &[2]),
    ("recv", 1, &[2]),
    ("recvfrom", 1, &[2]),
    ("send", 1, &[2]),
    ("fread", 0, &[1, 2]),
    ("fread_unlocked", 0, &[1, 2]),
    ("fwrite", 0, &[1, 2]),
    ("fwrite_unlocked", 0, &[1, 2]),
    ("fgets", 0, &[1]),
    ("fgets_unlocked", 0, &[1]),
    ("snprintf", 0, &[1]),
    ("vsnprintf", 0, &[1]),
    ("strftime", 0, &[1]),
    ("strncpy", 0, &[2]),
    ("stpncpy", 0, &[2]),
    ("strxfrm", 0, &[2]),
    ("strlcpy", 0, &[2]),
    ("getcwd", 0, &[1]),
    ("gethostname", 0, &[1]),
    ("getdomainname", 0, &[1]),
    ("getlogin_r", 0, &[1]),
    ("ttyname_r", 1, &[2]),
    ("ptsname_r", 1, &[2]),
    ("readlink", 1, &[2]),
    ("readlinkat", 2, &[3]),
    ("strerror_r", 1, &[2]),
    ("__xpg_strerror_r", 1, &[2]),
    ("confstr", 1, &[2]),
    ("setvbuf", 1, &[3]),
    ("inet_ntop", 2, &[3]),
    ("getnameinfo", 2, &[3]),
    ("getnameinfo", 4, &[5]),
    ("getentropy", 0, &[1]),
    ("getrandom", 0, &[1]),
    ("__memset_chk", 0, &[3]),
    ("__memcpy_chk", 0, &[3]),
    ("__memmove_chk", 0, &[3]),
    ("__mempcpy_chk", 0, &[3]),
    ("__explicit_bzero_chk", 0, &[2]),
    ("__strcpy_chk", 0, &[2]),
    ("__stpcpy_chk", 0, &[2]),
    ("__strcat_chk", 0, &[2]),
    ("__strncpy_chk", 0, &[3]),
    ("__stpncpy_chk", 0, &[3]),
    ("__strncat_chk", 0, &[3]),
    ("__sprintf_chk", 0, &[2]),
    ("__vsprintf_chk", 0, &[2]),
    ("__snprintf_chk", 0, &[3]),
    ("__vsnprintf_chk", 0, &[3]),
    ("__read_chk", 1, &[3]),
    ("__pread_chk", 1, &[4]),
    ("__pread64_chk", 1, &[4]),
    ("__recv_chk", 1, &[3]),
    ("__recvfrom_chk", 1, &[3]),
    ("__fgets_chk", 0, &[1]),
    ("__fgets_unlocked_chk", 0, &[1]),
    ("__fread_chk", 0, &[1]),
    ("__fread_unlocked_chk", 0, &[1]),
    ("__getcwd_chk", 0, &[2]),
    ("__gethostname_chk", 0, &[2]),
    ("__getdomainname_chk", 0, &[2]),
    ("__getlogin_r_chk", 0, &[2]),
    ("__ttyname_r_chk", 1, &[3]),
    ("__ptsname_r_chk", 1, &[3]),
    ("__readlink_chk", 1, &[3]),
    ("__readlinkat_chk", 2, &[4]),
    ("__realpath_chk", 1, &[2]),
    ("__confstr_chk", 1, &[3]),
];

/// Library functions whose result is how many bytes they stored at the buffer,
/// at most their length argument (not their `__*_chk` object size).
const RETURNS_STORED: &[&str] =
    &["read", "pread", "pread64", "recv", "recvfrom", "readlink", "readlinkat", "fread", "fread_unlocked"];

/// A byte count one call states for a frame address, and whether a library
/// function states it.
#[derive(Clone, Copy, Default)]
pub(crate) struct Stated {
    bytes: intb,
    library: bool,
}

/// A constant argument: a constant Varnode, or the pointer constant
/// `PTRSUB(#0, #c)` the type pass makes of an integer that happens to be a
/// mapped address.
fn constant(fd: &Funcdata, vn: Option<VarnodeId>) -> Option<intb> {
    let v = fd.vbank().get(vn?)?;
    let c = if v.is_constant() {
        v.get_offset()
    } else {
        let op = fd.obank().get(v.get_def()?)?;
        if op.code() != OpCode::CPUI_PTRSUB {
            return None;
        }
        let zero = fd.vbank().get(op.get_in(0)?)?;
        let off = fd.vbank().get(op.get_in(1)?)?;
        if !zero.is_constant() || zero.get_offset() != 0 || !off.is_constant() {
            return None;
        }
        off.get_offset()
    } as intb;
    (c > 0 && c <= MAX_BYTES).then_some(c)
}

/// Most Varnodes [`start_of`] and [`carries`] visit.
const MAX_WALK: usize = 64;

/// The frame offset of the one escaped address whose base `vn` may hold, copied
/// or carried unchanged around a loop. A program that passes a call `len`
/// bytes at `vn` on a path where `vn` is that base needs that many bytes there;
/// a value that may be either of two bases states nothing about either.
fn start_of(fd: &Funcdata, vn: VarnodeId, bases: &BTreeMap<VarnodeId, intb>) -> Option<intb> {
    let mut found: Option<intb> = None;
    let mut pending = vec![vn];
    let mut seen: Vec<VarnodeId> = Vec::new();
    while let Some(v) = pending.pop() {
        if seen.contains(&v) {
            continue;
        }
        if seen.len() >= MAX_WALK {
            return None;
        }
        seen.push(v);
        if let Some(&off) = bases.get(&v) {
            if found.is_some_and(|f| f != off) {
                return None;
            }
            found = Some(off);
            continue;
        }
        let Some(op) = fd.vbank().get(v).and_then(|x| x.get_def()).and_then(|d| fd.obank().get(d)) else {
            continue;
        };
        match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INDIRECT => pending.extend(op.get_in(0)),
            OpCode::CPUI_MULTIEQUAL => pending.extend((0..op.num_input()).filter_map(|s| op.get_in(s))),
            _ => {}
        }
    }
    found
}

/// Does `vn` carry the value `from`, through copies, extensions, truncations and
/// phis?
fn carries(fd: &Funcdata, vn: VarnodeId, from: VarnodeId) -> bool {
    let mut pending = vec![vn];
    let mut seen: Vec<VarnodeId> = Vec::new();
    while let Some(v) = pending.pop() {
        if v == from {
            return true;
        }
        if seen.contains(&v) || seen.len() >= MAX_WALK {
            continue;
        }
        seen.push(v);
        let Some(op) = fd.vbank().get(v).and_then(|x| x.get_def()).and_then(|d| fd.obank().get(d)) else {
            continue;
        };
        match op.code() {
            OpCode::CPUI_COPY
            | OpCode::CPUI_CAST
            | OpCode::CPUI_INDIRECT
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_SUBPIECE => pending.extend(op.get_in(0)),
            OpCode::CPUI_MULTIEQUAL => pending.extend((0..op.num_input()).filter_map(|s| op.get_in(s))),
            _ => {}
        }
    }
    false
}

/// Record, per frame offset an escaped address starts at, the byte counts the
/// calls receiving that address state (`MapState::set_stated_sizes`). A count
/// whose call returns how many bytes it stored, when the function indexes the
/// buffer by that result (`n = readlink(p, buf, len); buf[n] = 0;`), is one
/// byte more: the terminator lands at most `len` bytes in.
pub(super) fn gather(fd: &Funcdata, state: &mut MapState, space: &Rc<AddrSpace>) {
    let level = fd.get_arch().array_size;
    if level == LEVEL_OFF {
        return;
    }
    let bits = space.get_addr_size() as int4 * 8 - 1;
    let (bases, indexed): (BTreeMap<VarnodeId, intb>, Vec<(intb, VarnodeId)>) = {
        let checker = state.checker_mut();
        let signed = |off: uintb| sign_extend(space.wrap_offset(off) as intb, bits);
        let refs = checker.get_alias().iter().zip(checker.get_add_base());
        (
            refs.clone().filter(|(_, b)| b.index.is_none()).map(|(&off, b)| (b.base, signed(off))).collect(),
            refs.filter_map(|(&off, b)| b.index.map(|i| (signed(off), i))).collect(),
        )
    };
    if bases.is_empty() {
        return;
    }
    let mut sizes: BTreeMap<intb, Vec<Stated>> = BTreeMap::new();
    for i in 0..fd.num_calls() {
        let fc = fd.get_call_specs(i);
        let Some(o) = fd.obank().get(fc.get_op()) else { continue };
        if o.code() != OpCode::CPUI_CALL || o.is_dead() {
            continue;
        }
        let name = fc.get_name();
        let mut known = false;
        for (callee, ptr, counts) in LENGTH_CALLS {
            if *callee != name {
                continue;
            }
            known = true;
            let Some(start) = o.get_in(*ptr as int4 + 1).and_then(|v| start_of(fd, v, &bases)) else { continue };
            let mut n: Option<intb> = Some(1);
            for arg in counts.iter() {
                n = match (n, constant(fd, o.get_in(*arg as int4 + 1))) {
                    (Some(n), Some(c)) => n.checked_mul(c).filter(|n| *n <= MAX_BYTES),
                    _ => None,
                };
            }
            let Some(bytes) = n else { continue };
            let stored = RETURNS_STORED.contains(callee)
                && (counts.len() == 1 || constant(fd, o.get_in(counts[0] as int4 + 1)) == Some(1));
            let result = o.get_out().filter(|_| stored);
            let terminated =
                result.is_some_and(|r| indexed.iter().any(|&(off, i)| off == start && carries(fd, i, r)));
            sizes.entry(start).or_default().push(Stated { bytes: bytes + terminated as intb, library: true });
        }
        if known || level < LEVEL_CALLS {
            continue;
        }
        let starts: Vec<(int4, intb)> =
            (1..o.num_input()).filter_map(|s| o.get_in(s).and_then(|v| start_of(fd, v, &bases)).map(|st| (s, st))).collect();
        for (slot, start) in starts {
            for s in 1..o.num_input() {
                if s == slot {
                    continue;
                }
                if let Some(bytes) = constant(fd, o.get_in(s)) {
                    sizes.entry(start).or_default().push(Stated { bytes, library: false });
                }
            }
        }
    }
    state.set_stated_sizes(sizes);
}

/// The byte size the open range `cur`, whose absorbed hints end at `cur_end`,
/// takes instead of the `gap` to the next hint; `None` keeps the gap.
pub fn fit(cur: &RangeHint, cur_end: intb, gap: intb, state: &MapState) -> Option<int4> {
    let counts = state.stated_sizes(cur.sstart)?;
    let elem = cur.type_.get_align_size().max(1) as intb;
    let floor = (cur_end - cur.sstart).max(MIN_BYTES).max(gap - PADDING);
    let floor = if cur.highind >= 0 { floor.max((cur.highind as intb + 1) * elem) } else { floor };
    let mut best: Option<intb> = None;
    for s in counts {
        let mut bytes = s.bytes;
        if s.library && (bytes + 1) % 16 == 0 && bytes < gap {
            bytes += 1;
        }
        if bytes < floor || bytes + 1 >= gap || bytes % elem != 0 {
            continue;
        }
        best = Some(best.map_or(bytes, |b| b.max(bytes)));
    }
    best.and_then(|b| int4::try_from(b).ok())
}

#[cfg(test)]
#[path = "kuna_arraysize/tests.rs"]
mod tests;
