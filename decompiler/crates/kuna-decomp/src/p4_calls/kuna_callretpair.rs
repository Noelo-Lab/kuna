//! (kuna) Connect a two-register return at the CALL, whatever language produced
//! the callee.
//!
//! # The symptom
//!
//! A QuickJS interpreter built by plain `gcc -O2`. `JSValue` is
//! `{ JSValueUnion u; int64_t tag; }`, sixteen bytes, so the System V x86-64 ABI
//! returns it in `RAX:RDX` and the caller tests the tag register immediately:
//!
//! ```text
//! 0x9d9d8  CALL 0x875e0
//! 0x9d9dd  CMP  EDX,0x6          ; tag == JS_TAG_EXCEPTION
//! 0x9d9e6  MOV  R13,RAX          ; keep the payload
//! 0x9d9e9  MOV  R12,RDX
//! ```
//!
//! Ask kuna about the callee and it agrees there is a sixteen-byte return --
//! `undefined16 sub_875e0(int8,char *,int8,uint4)`. Ask it about the caller and
//! the same call has no output at all:
//!
//! ```text
//! int4 *v1;          // rax   <- declared, read five times, NEVER ASSIGNED
//! unsigned long v5;  // rdx   <- declared, read four times, NEVER ASSIGNED
//! sub_875e0(a0,a1,a2,1);
//! v4 = (int4)v5;
//! ```
//!
//! Every use of the returned value reads a variable with no definition anywhere
//! in the function, and the call that produced it looks like it was made for its
//! side effects.
//!
//! # What is missing
//!
//! Not the ABI decision, and not the recovery. The x86-64 cspec's
//! `<join_dual_class/>` output rule already describes this storage, and kuna
//! already puts two active output trials on the call. What drops them is the
//! consumer seam: `FuncCallSpecs::buildOutputFromTrials` (`fspec.cc:5777`)
//! handles one used output trial and upstream handles two, but kuna shipped the
//! multi-trial arm as a stub. The INDIRECT creations that stood for "the callee
//! wrote something here" survive untouched, which is what renders as a local the
//! function never assigns.
//!
//! [`crate::kuna_rustabi`] completed that arm, but gated it on the image being
//! rustc-produced. The completion is not Rust-specific and neither is the
//! evidence behind it: a sixteen-byte aggregate return is ordinary C on this
//! ABI, and `option rustabi auto` cannot see a GCC image at all. This option is
//! the same arm on the same classification with the language test dropped.
//!
//! # What the seam can prove
//!
//! [`crate::kuna_rustabi::classify_call_output_pair`] is the whole list, and it
//! is unchanged here: the model's output rule matched a justified,
//! non-overlapping register pair; the caller reads both halves out of the call;
//! and a bounded decode of the resolved callee did not refute it by proving the
//! payload register is never written. That last check is one-sided -- it can
//! refute a pair, never confirm one -- so forming the pair means *no
//! counter-example*, which is also the evidence upstream ships this arm on.
//!
//! # The gate
//!
//! `option callretpair on|off`, default **on**. `option rustabi auto|always`
//! still reaches the same arm, so a Rust image behaves as before whichever of
//! the two is set.

use crate::funcdata::Funcdata;

/// Is the language-agnostic call-output pair arm live for this function?
pub fn live(data: &Funcdata) -> bool {
    data.get_arch().call_ret_pair
}

/// Is the call-output pair arm live on the engine-side `Architecture`?
///
/// The seeding hook in [`crate::kuna_rustabi::seed_callee_return_writes`] runs
/// from the driver, where only the engine handle exists.
pub fn live_arch(arch: &crate::architecture::Architecture) -> bool {
    arch.call_ret_pair
}

#[cfg(test)]
#[path = "kuna_callretpair/tests.rs"]
mod tests;
