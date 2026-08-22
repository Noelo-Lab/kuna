//! (kuna) Keep a returned register half that carries a function **input
//! parameter**, instead of discarding it as leftover.
//!
//! # The symptom
//!
//! Two functions that differ only in what the second returned half holds:
//!
//! ```c
//! typedef struct { unsigned long a, b; } P;
//! P wide(unsigned long s, unsigned long x){ P p; p.a=s+1; p.b=x;     return p; }
//! P w2  (unsigned long s, unsigned long x){ P p; p.a=s+1; p.b=x*3+7; return p; }
//! ```
//!
//! `gcc -O1`, x86-64, no DWARF, recovered:
//!
//! ```text
//! wide  ->  unsigned long wide(long a0)          // RDX half dropped, and `x` VANISHED
//! w2    ->  undefined16 w2(long a0,long a1)      // correct
//! ```
//!
//! `wide` compiles to `mov %rsi,%rdx; lea 0x1(%rdi),%rax; ret` — the high half of
//! the returned pair is a plain copy of the second argument. Dropping it does not
//! merely lose the return half: the parameter it came from then has no remaining
//! reader, so it disappears from the recovered signature too. A whole argument is
//! gone from the prototype of a two-argument function.
//!
//! # Why it happens
//!
//! [`crate::kuna_returnuncomputed`] exists to kill a genuine phantom: a returned
//! register that merely holds a value the function never computed — the epilogue's
//! callee-saved *restore*, where the register is a copy of a frame slot the
//! function only ever reads, or a callee's clobber at a no-return call. Its walk
//! chases move-only operations (copies, phis, indirects, piece/subpiece) back to a
//! terminal and calls an **unwritten** Varnode uncomputed.
//!
//! A formal input parameter is unwritten by definition. So the rule cannot tell
//! `RDX = COPY(<the frame slot the caller left something in>)` — leftover — from
//! `RDX = COPY(<RSI, the second argument>)` — a real value the function was handed
//! and is handing back.
//!
//! # The discriminator
//!
//! Storage, asked twice.
//!
//! **Is it parameter storage?** The prototype model already knows. A passthrough
//! terminal sits at a location the model characterizes as input-parameter storage
//! (`FuncProto::possibleInputParam`) -- for x86-64 SysV `RDI`/`RSI`/`RDX`/`RCX`/
//! `R8`/`R9`/`XMM0-7` and the stack region *above* the return address. The
//! callee-saved restore terminal is a **local frame slot**, stack storage *below*
//! the return address, which no input `ParamEntry` covers; and a clobber is an
//! INDIRECT creation, which the walk already rejects before reaching a terminal.
//!
//! **Did the function put it there?** Parameter storage alone is not enough,
//! because on most ABIs some argument register is also a return register. Compare
//! the terminal's address with the storage of the return half the walk started
//! from:
//!
//! * `RDX = COPY(RSI)` -- different addresses. The function executed an
//!   instruction to move the argument into the return register. Real.
//! * `RDX` reaching the RETURN as the unwritten `RDX` -- same address. The
//!   function never touched the register; the caller's value is passing straight
//!   through. Leftover, exactly what the sibling rule exists to drop. (Witness:
//!   libselinux `sub_1a330`, whose cached early return sets `RAX` from a global
//!   and leaves `RDX` alone. Without the placement test that phantom `RDX` beats
//!   the real `RAX` half and the function grows three invented parameters.)
//!
//! # Shapes this newly accepts
//!
//! Exactly one: a returned register half whose move-only chain ends at an
//! unwritten function-input Varnode that is in input-parameter storage **and at a
//! different address than the half it reaches**. The function executed an
//! instruction to move an argument into the return register.
//!
//! Everything the sibling rule was built to kill is untouched: the restore
//! terminal fails `possible_input_param` (a local frame slot is not parameter
//! storage), the clobber terminal is an INDIRECT creation rejected earlier in the
//! walk, and an untouched return register is still leftover however much it looks
//! like an argument.
//!
//! # The rule that was tried and rejected
//!
//! An earlier version also rescued the pair when **every** half was an untouched
//! incoming argument, on the theory that `double f(double x){ return x; }` on ARM
//! returns its argument in the registers it arrived in. It recovered three
//! betaflight soft-float helpers whose halves are exactly that -- and it also
//! resurrected the GH-6990 SPARC symptom, because a *void* `main` that touches
//! nothing at all leaves `o0:o1` passing through and SPARC passes arguments in
//! those same registers (`tests/stages/gh6990-returnpair.xml`). Nothing local to
//! the pair separates "returns its argument unchanged" from "never touched the
//! return registers", so the rescue is not taken and those three functions keep
//! today's answer.
//!
//! Gated by `option retinputhalf on|off`.

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;

/// Is `vn` an unwritten Varnode that is a **formal input parameter** of this
/// function, rather than leftover the function never computed?
///
/// Four conditions, all necessary: `option retinputhalf` is on, the Varnode is
/// unwritten (a written one is not this shape and is classified by its defining
/// op), heritage flagged it a function input (a free Varnode is not a parameter),
/// and it sits in storage the prototype model characterizes as input-parameter
/// storage. The last is what separates a passed argument from a local frame slot
/// the function only reads.
///
/// The caller adds the **placement** test — see `computes_from` in
/// [`crate::kuna_returnuncomputed`].
///
/// Runs inside `ActionOutputPrototype`, which is scheduled *before*
/// `ActionInputPrototype` — the proto's parameter list is not fixated yet, so the
/// question has to be put to the model (`possible_input_param` falls through to
/// it when no locked params exist), exactly as input recovery itself does.
pub fn is_input_parameter(data: &Funcdata, vn: VarnodeId) -> bool {
    if !data.get_arch().ret_input_half {
        return false;
    }
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.get_def().is_some() || !v.is_input() {
        return false;
    }
    let (addr, size) = (v.get_addr().clone(), v.get_size());
    data.get_func_proto().possible_input_param(&addr, size)
}

#[cfg(test)]
#[path = "kuna_retinputhalf/tests.rs"]
mod tests;
