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
//! exact start of a frame address it receives ([`Says`]): the length a library
//! buffer function may fill (`snprintf(buf, 0x400, ..)`, `fgets`, `read`), the
//! length a library function writes exactly or only reads (`memset`, a
//! `memcpy` source, `write`), or the destination object size a
//! `_FORTIFY_SOURCE` `__*_chk` call carries. A count whose call returns how
//! many bytes it stored is one byte more when the function writes one byte at
//! that result (`n = readlink(p, buf, len); buf[n] = 0;`). At `calls` any
//! constant argument of any other call beside the address counts too. It also
//! follows every escaped address to the accesses and library calls it
//! reaches. [`fit`] ends the open range at the largest count that may decide
//! (a fill length, an object size, a `calls` constant) instead of at the next
//! hint, when
//!
//! - no count, of any kind, reaches within one byte of the gap: a call that
//!   states the whole gap proves it;
//! - the count is at least 16 bytes, a whole number of elements, at most 15
//!   bytes short of the gap, and at least every other count, every hint the
//!   range absorbed and every index bound it carries;
//! - no access the function makes through the buffer is unbounded: no index
//!   other than such a terminator is added to an address near it, no loop
//!   walks a pointer into it, no library call takes it with a length that is
//!   neither a constant nor a byte-count result, and no constant-offset access
//!   lands past the count.
//!
//! A fill length one byte short of a multiple of 16 that still fits the gap is
//! the `sizeof(buf) - 1` idiom (`readlink(p, buf, sizeof buf - 1)`), so the
//! array keeps that byte. The bytes past the array stay unmapped, as alignment
//! padding is. Upstream's gap is restored with `option arraysize off`.

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

/// What a call's byte count says about the buffer it receives.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Says {
    /// A library length the callee may fill the buffer up to (a capacity).
    Fills,
    /// A library length the callee writes exactly (`memset`, a `memcpy`
    /// destination): the buffer is at least that long.
    Covers,
    /// A library length the callee only reads: the buffer is at least that long.
    Reads,
    /// A `__*_chk` destination object size: the whole object at the address
    /// for the `mem*`, `read`, `recv` and `fread` families
    /// (`__builtin_object_size` mode 0), and at `_FORTIFY_SOURCE=2` the closest
    /// enclosing sub-object for the `str*`, `printf`, `fgets`, `getcwd` and
    /// `readlink` families (mode 1), which for a structure's first member is
    /// the member.
    Object,
    /// A constant beside the buffer in a call outside the tables (`calls`).
    Constant,
}

/// `(callee, buffer argument, arguments whose product is the byte count, what
/// the count says)`.
const LENGTH_CALLS: &[(&str, usize, &[usize], Says)] = &[
    ("memset", 0, &[2], Says::Covers),
    ("memcpy", 0, &[2], Says::Covers),
    ("memcpy", 1, &[2], Says::Reads),
    ("memmove", 0, &[2], Says::Covers),
    ("memmove", 1, &[2], Says::Reads),
    ("mempcpy", 0, &[2], Says::Covers),
    ("mempcpy", 1, &[2], Says::Reads),
    ("memcmp", 0, &[2], Says::Reads),
    ("memcmp", 1, &[2], Says::Reads),
    ("bzero", 0, &[1], Says::Covers),
    ("explicit_bzero", 0, &[1], Says::Covers),
    ("read", 1, &[2], Says::Fills),
    ("pread", 1, &[2], Says::Fills),
    ("pread64", 1, &[2], Says::Fills),
    ("write", 1, &[2], Says::Reads),
    ("recv", 1, &[2], Says::Fills),
    ("recvfrom", 1, &[2], Says::Fills),
    ("send", 1, &[2], Says::Reads),
    ("fread", 0, &[1, 2], Says::Fills),
    ("fread_unlocked", 0, &[1, 2], Says::Fills),
    ("fwrite", 0, &[1, 2], Says::Reads),
    ("fwrite_unlocked", 0, &[1, 2], Says::Reads),
    ("fgets", 0, &[1], Says::Fills),
    ("fgets_unlocked", 0, &[1], Says::Fills),
    ("snprintf", 0, &[1], Says::Fills),
    ("vsnprintf", 0, &[1], Says::Fills),
    ("strftime", 0, &[1], Says::Fills),
    ("strncpy", 0, &[2], Says::Fills),
    ("stpncpy", 0, &[2], Says::Fills),
    ("strxfrm", 0, &[2], Says::Fills),
    ("strlcpy", 0, &[2], Says::Fills),
    ("getcwd", 0, &[1], Says::Fills),
    ("gethostname", 0, &[1], Says::Fills),
    ("getdomainname", 0, &[1], Says::Fills),
    ("getlogin_r", 0, &[1], Says::Fills),
    ("ttyname_r", 1, &[2], Says::Fills),
    ("ptsname_r", 1, &[2], Says::Fills),
    ("readlink", 1, &[2], Says::Fills),
    ("readlinkat", 2, &[3], Says::Fills),
    ("strerror_r", 1, &[2], Says::Fills),
    ("__xpg_strerror_r", 1, &[2], Says::Fills),
    ("confstr", 1, &[2], Says::Fills),
    ("setvbuf", 1, &[3], Says::Fills),
    ("inet_ntop", 2, &[3], Says::Fills),
    ("getnameinfo", 2, &[3], Says::Fills),
    ("getnameinfo", 4, &[5], Says::Fills),
    ("getentropy", 0, &[1], Says::Fills),
    ("getrandom", 0, &[1], Says::Fills),
    ("__memset_chk", 0, &[3], Says::Object),
    ("__memcpy_chk", 0, &[3], Says::Object),
    ("__memmove_chk", 0, &[3], Says::Object),
    ("__mempcpy_chk", 0, &[3], Says::Object),
    ("__explicit_bzero_chk", 0, &[2], Says::Object),
    ("__read_chk", 1, &[3], Says::Object),
    ("__pread_chk", 1, &[4], Says::Object),
    ("__pread64_chk", 1, &[4], Says::Object),
    ("__recv_chk", 1, &[3], Says::Object),
    ("__recvfrom_chk", 1, &[3], Says::Object),
    ("__fread_chk", 0, &[1], Says::Object),
    ("__fread_unlocked_chk", 0, &[1], Says::Object),
    ("__strcpy_chk", 0, &[2], Says::Object),
    ("__stpcpy_chk", 0, &[2], Says::Object),
    ("__strcat_chk", 0, &[2], Says::Object),
    ("__strncpy_chk", 0, &[3], Says::Object),
    ("__stpncpy_chk", 0, &[3], Says::Object),
    ("__strncat_chk", 0, &[3], Says::Object),
    ("__sprintf_chk", 0, &[2], Says::Object),
    ("__vsprintf_chk", 0, &[2], Says::Object),
    ("__snprintf_chk", 0, &[3], Says::Object),
    ("__vsnprintf_chk", 0, &[3], Says::Object),
    ("__fgets_chk", 0, &[1], Says::Object),
    ("__fgets_unlocked_chk", 0, &[1], Says::Object),
    ("__getcwd_chk", 0, &[2], Says::Object),
    ("__gethostname_chk", 0, &[2], Says::Object),
    ("__getdomainname_chk", 0, &[2], Says::Object),
    ("__getlogin_r_chk", 0, &[2], Says::Object),
    ("__ttyname_r_chk", 1, &[3], Says::Object),
    ("__ptsname_r_chk", 1, &[3], Says::Object),
    ("__readlink_chk", 1, &[3], Says::Object),
    ("__readlinkat_chk", 2, &[4], Says::Object),
    ("__realpath_chk", 1, &[2], Says::Object),
    ("__confstr_chk", 1, &[3], Says::Object),
];

/// Library functions whose result is how many bytes they stored at the buffer,
/// at most their length argument.
const RETURNS_STORED: &[&str] =
    &["read", "pread", "pread64", "recv", "recvfrom", "readlink", "readlinkat", "fread", "fread_unlocked"];

/// What the function's calls say about the buffer at one frame offset.
#[derive(Clone, Default)]
pub(crate) struct Evidence {
    /// The byte counts calls state for the start, what each says, and whether
    /// it already includes a terminator byte at the call's result.
    counts: Vec<(intb, Says, bool)>,
}

/// Where the accesses through one escaped address land, as signed frame
/// offsets.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Reach {
    /// The frame offset of the address.
    offset: intb,
    /// The lowest byte a constant-offset access through it touches.
    lo: intb,
    /// One past the highest byte a constant-offset access through it touches.
    hi: intb,
    /// Some access may land anywhere above the address: a loop advances it, a
    /// non-constant offset other than a terminator is added, a library call
    /// receives it with a non-constant length.
    open_up: bool,
    /// Some access may land anywhere below the address (a loop walks it down,
    /// or a non-constant offset is added).
    open_down: bool,
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
/// Most Varnodes [`trace`] visits before it calls the address's extent open.
const MAX_TRACE: usize = 256;
/// How far below a buffer's start an index base may sit (a negative constant
/// folded into the base, `buf[n - 1]`) and still reach the buffer.
const INDEX_BELOW: intb = 16;

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

/// Is `vn` the value `from` on every path, through copies, extensions,
/// truncations and phis? A phi input that is anything else (a constant cap,
/// `if (n == 64) n = 63;`) makes the answer no.
fn carries(fd: &Funcdata, vn: VarnodeId, from: VarnodeId) -> bool {
    let mut pending = vec![vn];
    let mut seen: Vec<VarnodeId> = Vec::new();
    let mut reached = false;
    while let Some(v) = pending.pop() {
        if v == from {
            reached = true;
            continue;
        }
        if seen.contains(&v) {
            continue;
        }
        if seen.len() >= MAX_WALK {
            return false;
        }
        seen.push(v);
        let Some(op) = fd.vbank().get(v).and_then(|x| x.get_def()).and_then(|d| fd.obank().get(d)) else {
            return false;
        };
        match op.code() {
            OpCode::CPUI_COPY
            | OpCode::CPUI_CAST
            | OpCode::CPUI_INDIRECT
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_SUBPIECE => pending.extend(op.get_in(0)),
            OpCode::CPUI_MULTIEQUAL => pending.extend((0..op.num_input()).filter_map(|s| op.get_in(s))),
            _ => return false,
        }
    }
    reached
}

/// Is every use of the address `vn` a one-byte LOAD or STORE through it?
fn byte_access_only(fd: &Funcdata, vn: VarnodeId) -> bool {
    let mut pending = vec![vn];
    let mut seen: Vec<VarnodeId> = Vec::new();
    while let Some(v) = pending.pop() {
        if seen.contains(&v) {
            continue;
        }
        if seen.len() >= MAX_WALK {
            return false;
        }
        seen.push(v);
        let Some(x) = fd.vbank().get(v) else { return false };
        for d in x.descend_iter() {
            let Some(op) = fd.obank().get(d) else { return false };
            if op.is_dead() {
                continue;
            }
            match op.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_CAST => pending.extend(op.get_out()),
                OpCode::CPUI_LOAD if op.get_in(1) == Some(v) => {
                    if op.get_out().and_then(|o| fd.vbank().get(o)).map(|o| o.get_size()) != Some(1) {
                        return false;
                    }
                }
                OpCode::CPUI_STORE if op.get_in(1) == Some(v) && op.get_in(2) != Some(v) => {
                    if op.get_in(2).and_then(|o| fd.vbank().get(o)).map(|o| o.get_size()) != Some(1) {
                        return false;
                    }
                }
                _ => return false,
            }
        }
    }
    true
}

/// The results of the byte-count calls on the buffer at each start, with the
/// length each call was given.
type StoredResults = Vec<(intb, VarnodeId, intb)>;

/// Is `index`, added to the address of the buffer at `start` to form `sum`, a
/// byte-count result of a call on that buffer used for one terminator byte
/// (`n = read(fd, buf, len); buf[n] = 0;`)?
fn terminator(fd: &Funcdata, start: intb, index: VarnodeId, sum: VarnodeId, stored: &StoredResults) -> bool {
    stored.iter().any(|&(s, r, _)| s == start && carries(fd, index, r)) && byte_access_only(fd, sum)
}

/// A constant Varnode's value, sign-extended from its size.
fn signed_value(v: &crate::varnode::Varnode) -> intb {
    sign_extend(v.get_offset() as intb, v.get_size() * 8 - 1)
}

/// Follow the address `base` at frame offset `offset` through copies, phis and
/// constant pointer arithmetic to every access and library call it reaches.
fn trace(fd: &Funcdata, base: VarnodeId, offset: intb, stored: &StoredResults) -> Reach {
    let mut r = Reach { offset, lo: offset, hi: offset, open_up: false, open_down: false };
    let mut seen: BTreeMap<VarnodeId, intb> = BTreeMap::new();
    let mut pending: Vec<(VarnodeId, intb)> = vec![(base, 0)];
    let touch = |r: &mut Reach, from: intb, len: intb| {
        r.lo = r.lo.min(offset + from);
        r.hi = r.hi.max(offset + from + len);
    };
    while let Some((v, delta)) = pending.pop() {
        if let Some(&d) = seen.get(&v) {
            if delta < d {
                r.open_down = true;
            } else if delta > d {
                r.open_up = true;
            }
            continue;
        }
        if seen.len() >= MAX_TRACE {
            r.open_up = true;
            r.open_down = true;
            return r;
        }
        seen.insert(v, delta);
        let Some(x) = fd.vbank().get(v) else { continue };
        let uses: Vec<crate::context::OpId> = x.descend_iter().collect();
        for d in uses {
            let Some(op) = fd.obank().get(d) else { continue };
            if op.is_dead() {
                continue;
            }
            let out = op.get_out();
            let constant_in = |slot: int4| op.get_in(slot).and_then(|a| fd.vbank().get(a)).filter(|a| a.is_constant());
            match op.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_MULTIEQUAL => pending.extend(out.map(|o| (o, delta))),
                OpCode::CPUI_INDIRECT => {
                    if op.get_in(0) == Some(v) {
                        pending.extend(out.map(|o| (o, delta)));
                    }
                }
                OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB => {
                    let slot = if op.get_in(0) == Some(v) { 1 } else { 0 };
                    match (constant_in(slot).map(signed_value), op.get_in(slot), out) {
                        (Some(c), _, Some(o)) => pending.push((o, delta.wrapping_add(c))),
                        (None, Some(i), Some(o)) if delta == 0 && terminator(fd, offset, i, o, stored) => {}
                        _ => {
                            r.open_up = true;
                            r.open_down = true;
                        }
                    }
                }
                OpCode::CPUI_PTRADD => {
                    if op.get_in(0) == Some(v) {
                        let size = constant_in(2).map(|a| a.get_offset() as intb);
                        match (constant_in(1).map(signed_value), size, out) {
                            (Some(i), Some(z), Some(o)) => pending.push((o, delta.wrapping_add(i.wrapping_mul(z)))),
                            (None, _, Some(o))
                                if delta == 0 && op.get_in(1).is_some_and(|i| terminator(fd, offset, i, o, stored)) => {}
                            _ => {
                                r.open_up = true;
                                r.open_down = true;
                            }
                        }
                    }
                }
                OpCode::CPUI_INT_SUB => {
                    if op.get_in(0) == Some(v) {
                        match (constant_in(1).map(signed_value), out) {
                            (Some(c), Some(o)) => pending.push((o, delta.wrapping_sub(c))),
                            _ => {
                                r.open_up = true;
                                r.open_down = true;
                            }
                        }
                    }
                }
                OpCode::CPUI_LOAD => {
                    if op.get_in(1) == Some(v) {
                        let size = out.and_then(|o| fd.vbank().get(o)).map_or(1, |o| o.get_size() as intb);
                        touch(&mut r, delta, size);
                    }
                }
                OpCode::CPUI_STORE => {
                    if op.get_in(1) == Some(v) {
                        let size = op.get_in(2).and_then(|o| fd.vbank().get(o)).map_or(1, |o| o.get_size() as intb);
                        touch(&mut r, delta, size);
                    }
                }
                OpCode::CPUI_CALL => {
                    let slots: Vec<usize> =
                        (1..op.num_input()).filter(|s| op.get_in(*s) == Some(v)).map(|s| s as usize - 1).collect();
                    let name = fd.get_call_specs_index(d).map(|i| fd.get_call_specs(i).get_name().to_string());
                    for (callee, ptr, counts, _) in LENGTH_CALLS {
                        if name.as_deref() != Some(*callee) || !slots.contains(ptr) {
                            continue;
                        }
                        let mut n: Option<intb> = Some(1);
                        for arg in counts.iter() {
                            let a = op.get_in(*arg as int4 + 1);
                            let bound = constant(fd, a).or_else(|| {
                                let a = a?;
                                stored.iter().find(|&&(s, r, _)| s == offset && carries(fd, a, r)).map(|&(_, _, k)| k)
                            });
                            n = n.zip(bound).map(|(a, b)| a.saturating_mul(b));
                        }
                        match n {
                            Some(n) => touch(&mut r, delta, n),
                            None => r.open_up = true,
                        }
                    }
                }
                _ => {}
            }
        }
    }
    r
}

/// Record, per frame offset an escaped address starts at, what the function's
/// calls and accesses say about the buffer there (`MapState::set_stated_sizes`).
/// A count whose call returns how many bytes it stored, when the function
/// writes one terminator byte at that result (`n = readlink(p, buf, len);
/// buf[n] = 0;`), is one byte more.
pub(super) fn gather(fd: &Funcdata, state: &mut MapState, space: &Rc<AddrSpace>) {
    let level = fd.get_arch().array_size;
    if level == LEVEL_OFF {
        return;
    }
    let bits = space.get_addr_size() as int4 * 8 - 1;
    let (bases, indexed): (BTreeMap<VarnodeId, intb>, Vec<(intb, VarnodeId, VarnodeId)>) = {
        let checker = state.checker_mut();
        let signed = |off: uintb| sign_extend(space.wrap_offset(off) as intb, bits);
        let refs = checker.get_alias().iter().zip(checker.get_add_base());
        (
            refs.clone().filter(|(_, b)| b.index.is_none()).map(|(&off, b)| (b.base, signed(off))).collect(),
            refs.filter_map(|(&off, b)| b.index.map(|i| (signed(off), b.base, i))).collect(),
        )
    };
    if bases.is_empty() {
        return;
    }
    let mut sizes: BTreeMap<intb, Evidence> = BTreeMap::new();
    let mut stored: StoredResults = Vec::new();
    let mut counted: Vec<(intb, intb, Says, Option<VarnodeId>)> = Vec::new();
    for i in 0..fd.num_calls() {
        let fc = fd.get_call_specs(i);
        let Some(o) = fd.obank().get(fc.get_op()) else { continue };
        if o.code() != OpCode::CPUI_CALL || o.is_dead() {
            continue;
        }
        let name = fc.get_name();
        let mut known = false;
        for (callee, ptr, counts, says) in LENGTH_CALLS {
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
            let result = o.get_out().filter(|_| {
                RETURNS_STORED.contains(callee)
                    && (counts.len() == 1 || constant(fd, o.get_in(counts[0] as int4 + 1)) == Some(1))
            });
            if let Some(r) = result {
                stored.push((start, r, bytes));
            }
            counted.push((start, bytes, *says, result));
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
                    counted.push((start, bytes, Says::Constant, None));
                }
            }
        }
    }
    if counted.is_empty() {
        return;
    }
    let mut unproven: Vec<intb> = Vec::new();
    for &(off, sum, index) in &indexed {
        if !terminator(fd, off, index, sum, &stored) {
            unproven.push(off);
        }
    }
    unproven.sort_unstable();
    for (start, bytes, says, result) in counted {
        let terminated = result.is_some_and(|r| {
            indexed.iter().any(|&(off, sum, i)| off == start && carries(fd, i, r) && byte_access_only(fd, sum))
        });
        sizes.entry(start).or_default().counts.push((bytes + terminated as intb, says, terminated));
    }
    let reaches = bases.iter().map(|(&base, &offset)| trace(fd, base, offset, &stored)).collect();
    state.set_stated_sizes(sizes, unproven, reaches);
}

/// The byte size the open range `cur`, whose absorbed hints end at `cur_end`,
/// takes instead of the `gap` to the next hint; `None` keeps the gap.
pub fn fit(cur: &RangeHint, cur_end: intb, gap: intb, state: &MapState) -> Option<int4> {
    let ev = state.stated_sizes(cur.sstart)?;
    let start = cur.sstart;
    if state.unproven_index_in(start - INDEX_BELOW, start + gap) {
        return None;
    }
    let mut floor = (cur_end - start).max(MIN_BYTES).max(gap - PADDING);
    let mut best: Option<intb> = None;
    for &(count, says, terminated) in &ev.counts {
        let bump = says == Says::Fills && !terminated && (count + 1) % 16 == 0 && count < gap;
        let bytes = count + bump as intb;
        if bytes + 1 >= gap {
            return None;
        }
        floor = floor.max(bytes);
        if !matches!(says, Says::Reads | Says::Covers) {
            best = Some(best.map_or(bytes, |b| b.max(bytes)));
        }
    }
    let elem = cur.type_.get_align_size().max(1) as intb;
    if cur.highind >= 0 {
        floor = floor.max((cur.highind as intb + 1) * elem);
    }
    let bytes = best?;
    if bytes < floor || bytes % elem != 0 {
        return None;
    }
    let (end, tail) = (start + bytes, start + gap);
    for r in state.reaches() {
        let near_below = r.offset >= start - INDEX_BELOW && r.offset < tail;
        let near_above = r.offset > start && r.offset <= tail + INDEX_BELOW;
        if (r.open_up && near_below) || (r.open_down && near_above) || (r.hi > r.lo && r.lo < tail && r.hi > end) {
            return None;
        }
    }
    int4::try_from(bytes).ok()
}

#[cfg(test)]
#[path = "kuna_arraysize/tests.rs"]
mod tests;
