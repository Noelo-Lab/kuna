//! P3 heritage refinement of register pairs — the `pairrefine` decision point.
//!
//! # The gap
//!
//! `Heritage::placeMultiequals` (`heritage.cc:2611`) refines a disjoint range
//! only when no access covers all of it (`size > 4 && maxw < size`). On SPARC32
//! an `ldd`/`std` names an even/odd register pair as one 8-byte access, so a pair
//! that is loaded or stored whole even once is never refined. Every other access
//! to one of its registers — a loop counter in `%l4`, a sum in `%l5` — is then
//! joined to the 8-byte range through `PIECE`/`SUBPIECE`, the phis are placed on
//! the whole pair, and variable merging fuses the two unrelated 32-bit lifetimes
//! into one `unsigned long long` whose updates print as `CONCAT44` piece writes:
//!
//! ```text
//! v2 = (c) ? *p : (unsigned long long)(unsigned int)n << 0x20;
//! do {
//!   v1 = v2 >> 0x20;
//!   ...
//!   v2 = CONCAT44(v1,v3);
//! } while (0 <= v1);
//! ```
//!
//! # The refinement
//!
//! [`split_pair`] selects a named 8-byte register pair (SPARC32 `l4_5`) whose
//! two halves are each a named 4-byte register, that is accessed whole only by
//! memory moves — written by loads (the `ldd`, as a `LOAD` or as the `COPY` of
//! a memory varnode a frame load becomes) and read by stores (the `std`) — and
//! is also written through a half. That is the shape of GCC's block moves
//! through a pair whose registers are reused for 32-bit values. A pair a call
//! takes or returns whole, or that an 8-byte op uses, keeps upstream's
//! heritage, as does one with any floating-point access. SPARC32 64-bit
//! arithmetic works on the two registers (`addcc`/`addx`), so it does not count
//! as a whole-pair use.
//! Every access that crosses the midpoint must cover the whole pair. The
//! selected range is refined at its midpoint only: those whole-pair moves
//! ([`straddles_middle`]) become two halves through `PIECE`/`SUBPIECE`, and the
//! 4-byte accesses keep their varnodes. The halves are then heritaged as
//! independent registers.
//!
//! An 8-byte register read through its low half (x86-64 `rax`/`eax`) is not a
//! pair: its upper half is not a named register, so it is left whole. Two
//! adjacent registers with no pair register over them (x86-32 `EAX`/`ECX`) are
//! not one either. A pair in a floating-point register file ([`in_float_file`]:
//! ARM `d8` over `s16`/`s17`, MIPS32 `f0_1`, an x86-64 `XMM0_Qa` lane) is left
//! whole even when it is only moved, as a callee-saved `d8` is by `vpush` and
//! `vpop`.

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::VarnodeId;
use crate::dtype::type_class;
use crate::funcdata::Funcdata;
use crate::p5_types::kuna_ptrfromuse::is_float_op;

/// Whether the register range at `addr` (`size` bytes, widest access `maxw`) is
/// an integer register pair to refine at its midpoint (kuna `pairrefine`).
/// Requires a whole-pair load, not just half writes followed by a whole store.
pub fn split_pair(
    fd: &Funcdata,
    addr: &Address,
    size: int4,
    maxw: int4,
    readvars: &[VarnodeId],
    writevars: &[VarnodeId],
    inputvars: &[VarnodeId],
) -> bool {
    if !fd.get_arch().pair_refine || size != 8 || maxw != size {
        return false;
    }
    let Some(space) = addr.get_space() else {
        return false;
    };
    let is_register = fd
        .get_arch()
        .manage()
        .get_space_by_name("register")
        .is_some_and(|register| register.get_index() == space.get_index());
    if !is_register {
        return false;
    }
    let size_of = |vn: VarnodeId| fd.vbank().get(vn).map_or(0, |v| v.get_size());
    if !writevars.iter().any(|&vn| size_of(vn) == 4) {
        return false;
    }
    let partial = readvars
        .iter()
        .chain(writevars.iter())
        .chain(inputvars.iter())
        .any(|&vn| straddles_middle(fd, vn, addr, size) && !covers(fd, vn, addr, size));
    if partial {
        return false;
    }
    if !writevars.iter().any(|&vn| covers(fd, vn, addr, size) && written_by_load(fd, vn)) {
        return false;
    }
    let loaded = writevars
        .iter()
        .filter(|&&vn| straddles_middle(fd, vn, addr, size))
        .all(|&vn| written_by_load(fd, vn))
        && readvars
            .iter()
            .filter(|&&vn| straddles_middle(fd, vn, addr, size))
            .all(|&vn| read_by_store(fd, vn));
    if inputvars.iter().any(|&vn| straddles_middle(fd, vn, addr, size)) {
        return false;
    }
    let floating = readvars.iter().any(|&vn| feeds_float(fd, vn))
        || writevars.iter().any(|&vn| float_result(fd, vn));
    if !loaded || floating {
        return false;
    }
    let Some(lookup) = fd.get_arch().manage().register_lookup() else {
        return false;
    };
    let low = addr.get_offset();
    !lookup.get_exact_register_name(space, low, 8).is_empty()
        && !lookup.get_exact_register_name(space, low, 4).is_empty()
        && !lookup.get_exact_register_name(space, low + 4, 4).is_empty()
        && !in_float_file(fd, lookup.as_ref(), space, low)
}

/// Whether the 8 bytes at `low` lie in a floating-point register file: a run of
/// contiguous, equally sized named registers holding a float-class entry of the
/// function's or the default prototype model (ARM `s0`..`s31` and `d0`..`d31`
/// from the `s0`/`d0` entries, so a callee-saved `d8` is one).
fn in_float_file(
    fd: &Funcdata,
    lookup: &dyn kuna_base::space::RegisterLookup,
    space: &std::rc::Rc<kuna_base::space::AddrSpace>,
    low: u64,
) -> bool {
    let named = |off: u64, size: int4| !lookup.get_exact_register_name(space, off, size).is_empty();
    let proto = fd.get_func_proto();
    let models = proto.has_model().then(|| proto.model()).into_iter().chain(fd.get_arch().default_fp());
    models
        .flat_map(|m| m.input_opt().into_iter().chain(m.output_list()))
        .flat_map(|list| list.get_entry().iter())
        .filter(|e| {
            e.get_type() == type_class::TYPECLASS_FLOAT
                && e.get_size() > 0
                && e.get_space().get_index() == space.get_index()
        })
        .any(|e| {
            let (base, size) = (e.get_base(), e.get_size());
            let step = size as u64;
            if !named(base, size) {
                return false;
            }
            let (mut start, mut end) = (base, base + step);
            for _ in 0..256 {
                if start < step || !named(start - step, size) {
                    break;
                }
                start -= step;
            }
            for _ in 0..256 {
                if !named(end, size) {
                    break;
                }
                end += step;
            }
            start <= low && low + 8 <= end
        })
}

/// Whether `vn` is exactly the `size`-byte range at `addr`.
fn covers(fd: &Funcdata, vn: VarnodeId, addr: &Address, size: int4) -> bool {
    fd.vbank().get(vn).is_some_and(|v| v.get_addr().get_offset() == addr.get_offset() && v.get_size() == size)
}

/// Whether a memory varnode: a processor or frame space other than the register file.
fn in_memory(fd: &Funcdata, vn: VarnodeId) -> bool {
    let register = fd.get_arch().manage().get_space_by_name("register");
    fd.vbank().get(vn).and_then(|v| v.get_addr().get_space()).is_some_and(|space| {
        matches!(space.get_type(), spacetype::IPTR_PROCESSOR | spacetype::IPTR_SPACEBASE)
            && !register.is_some_and(|r| r.get_index() == space.get_index())
    })
}

/// Whether every op reading a whole pair stores it: a `STORE` of the pair as its
/// value, or the `COPY` into a memory varnode a frame store becomes.
fn read_by_store(fd: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = fd.vbank().get(vn) else {
        return false;
    };
    v.descend_iter().all(|op| {
        fd.obank().get(op).is_some_and(|o| match o.code() {
            OpCode::CPUI_STORE => o.get_in(2) == Some(vn),
            OpCode::CPUI_COPY => o.get_out().is_some_and(|out| in_memory(fd, out)),
            _ => false,
        })
    })
}

/// Whether a whole-pair write is defined by a load: a `LOAD`, or the `COPY` of a
/// memory varnode a frame or global load becomes.
fn written_by_load(fd: &Funcdata, vn: VarnodeId) -> bool {
    let Some(op) = fd.vbank().get(vn).and_then(|v| v.get_def()).and_then(|op| fd.obank().get(op)) else {
        return false;
    };
    match op.code() {
        OpCode::CPUI_LOAD => true,
        OpCode::CPUI_COPY => op.get_in(0).is_some_and(|input| in_memory(fd, input)),
        _ => false,
    }
}

/// Whether a read of the range feeds a floating-point op.
fn feeds_float(fd: &Funcdata, vn: VarnodeId) -> bool {
    fd.vbank().get(vn).is_some_and(|v| {
        v.descend_iter().any(|op| fd.obank().get(op).is_some_and(|o| is_float_op(o.code())))
    })
}

/// Whether a write of the range is the result of a floating-point op.
fn float_result(fd: &Funcdata, vn: VarnodeId) -> bool {
    fd.vbank()
        .get(vn)
        .and_then(|v| v.get_def())
        .and_then(|op| fd.obank().get(op))
        .is_some_and(|o| is_float_op(o.code()))
}

/// Whether `vn` crosses the middle of the `size`-byte range at `addr`: only these
/// accesses are cut when a pair is refined at its midpoint.
pub fn straddles_middle(fd: &Funcdata, vn: VarnodeId, addr: &Address, size: int4) -> bool {
    let half = (size / 2) as u64;
    fd.vbank().get(vn).is_some_and(|v| {
        let start = v.get_addr().get_offset().wrapping_sub(addr.get_offset());
        start < half && start + v.get_size() as u64 > half
    })
}

