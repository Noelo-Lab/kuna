//! (kuna) A trial read out of a wider register is followed through the joins
//! heritage builds above its lane writes.
//!
//! x86-64 returns a `float` or `double` in `xmm0`, and the compiler spec's
//! floating output entry is its low eight bytes. SLEIGH models a packed
//! instruction lane by lane: MSVC's `int` to `float` conversion
//! `movd xmm0,ecx; cvtdq2ps xmm0,xmm0` writes all sixteen bytes and then each
//! 4-byte lane in turn. Heritage joins each lane write into the 16-byte
//! register with a PIECE of the new lane and SUBPIECEs of the previous value,
//! so the low eight bytes the trial reads sit four joins down, under a 12-byte
//! low piece. The realism walk failed at the first PIECE, whose low input is
//! wider than the trial, and the use walk ran out of levels before it reached a
//! lane: the function returned `void`.
//!
//! [`piece_holding`] lets both walks step into the input of a PIECE that holds
//! all of the trial's bytes. [`free_step`] says when the use walk's step is
//! such a join on the register's own storage: a SUBPIECE of a PIECE, or a PIECE
//! input holding the whole trial. Those steps move no data, so they cost no
//! level, up to [`MAX_FREE_STEPS`] in one walk; a step that truncates the trial
//! to one input, and every other op, still does. Both take only a PIECE wider
//! than a general register: `setz cl` or `mov al,..` inside `rcx` or `rax`
//! keeps upstream's verdict and cost, so a control-flow-flattened function
//! whose `eax` holds its dispatcher state at the RET still returns `void`.
//!
//! The same joins carry the zeros `vzeroall` writes into every vector register
//! as 8-byte lanes. Hand-written AVX code (OpenSSL's ChaCha20 and AES-GCM) ends
//! with it to wipe the registers, and the walks, now reaching the zero lane,
//! would score it returned (`return 0.0`). [`clears_bank`] says the use walk
//! found no use there: an instruction that clears the whole register bank
//! computes no value.

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

/// How many level-free join steps one use walk may take.
pub const MAX_FREE_STEPS: u32 = 64;

/// The widest join that is not a lane join: a PIECE no wider than this is a
/// general register's partial write (`al` or `eax` inside `rax`), and keeps
/// the cost and the verdict upstream gives it.
const GENERAL_WIDTH: int4 = 8;

/// How many COPYs of zero into registers one instruction must make to clear
/// the register bank: `vzeroall` makes 128 (64 outside 64-bit mode), and no
/// instruction that computes a value makes more than a few.
const BANK_CLEAR: usize = 32;

/// The input slot of a PIECE (`in0` most significant, `in1` least) that holds
/// all `size` bytes at `offset` of its output, and their offset within it,
/// when the PIECE joins lanes of a register wider than [`GENERAL_WIDTH`].
pub fn piece_holding(offset: int4, size: int4, in0_size: int4, in1_size: int4) -> Option<(int4, int4)> {
    if in0_size + in1_size <= GENERAL_WIDTH {
        None
    } else if offset + size <= in1_size {
        Some((1, offset))
    } else if offset >= in1_size && offset - in1_size + size <= in0_size {
        Some((0, offset - in1_size))
    } else {
        None
    }
}

/// Is the use walk's step from `vn`, defined by `def`, to its input `next` a
/// join on the same storage that moves none of the trial's `size` bytes at
/// `offset`? A SUBPIECE qualifies when its input is itself a PIECE, a PIECE when
/// `next` holds the whole trial.
pub fn free_step(data: &Funcdata, vn: VarnodeId, def: OpId, next: VarnodeId, offset: int4, size: int4) -> bool {
    let (Some(o), Some(v), Some(n)) = (data.obank().get(def), data.vbank().get(vn), data.vbank().get(next)) else {
        return false;
    };
    match o.code() {
        OpCode::CPUI_SUBPIECE => {
            let Some(c) = o.get_in(1).and_then(|c| data.vbank().get(c)) else { return false };
            n.get_size() > GENERAL_WIDTH
                && v.overlap(n) == c.get_offset() as int4
                && n.get_def().and_then(|d| data.obank().get(d)).is_some_and(|d| d.code() == OpCode::CPUI_PIECE)
        }
        OpCode::CPUI_PIECE => {
            let (Some(hi), Some(lo)) = (o.get_in(0), o.get_in(1)) else { return false };
            let (Some(hiv), Some(lov)) = (data.vbank().get(hi), data.vbank().get(lo)) else { return false };
            match piece_holding(offset, size, hiv.get_size(), lov.get_size()) {
                Some((1, _)) => next == lo && lov.overlap(v) == 0,
                Some(_) => next == hi && hiv.overlap(v) == lov.get_size(),
                None => false,
            }
        }
        _ => false,
    }
}

/// Is `def` a COPY of zero into a register that an instruction clearing the
/// whole register bank made? Every op heritage has not added at its address
/// is a COPY of zero (SLEIGH routes some lanes through temporaries), and there
/// are at least [`BANK_CLEAR`] of them.
pub fn clears_bank(data: &Funcdata, def: OpId) -> bool {
    let zero_copy = |id: OpId| {
        data.obank().get(id).filter(|o| o.code() == OpCode::CPUI_COPY).is_some_and(|o| {
            o.get_in(0).and_then(|c| data.vbank().get(c)).is_some_and(|c| c.is_constant() && c.get_offset() == 0)
        })
    };
    let into_register = data
        .obank()
        .get(def)
        .and_then(|o| o.get_out())
        .and_then(|v| data.vbank().get(v))
        .is_some_and(|v| v.get_space().get_type() == spacetype::IPTR_PROCESSOR);
    if !into_register || !zero_copy(def) {
        return false;
    }
    let Some(addr) = data.obank().get(def).map(|o| o.get_addr().clone()) else { return false };
    let mut zeros = 0;
    for (_, id) in data.obank().iter_at(&addr) {
        match data.obank().get(id).map(|o| o.code()) {
            Some(OpCode::CPUI_COPY) if zero_copy(id) => zeros += 1,
            Some(OpCode::CPUI_PIECE | OpCode::CPUI_SUBPIECE | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT) => {}
            _ => return false,
        }
    }
    zeros >= BANK_CLEAR
}
