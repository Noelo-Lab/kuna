//! (kuna, ida) Assert the x86 ABI's direction-flag guarantee on a decoded
//! prototype model, so the flag stops leaking into decompiled output.
//!
//! # The symptom
//!
//! gcc inlines `strcmp(file, "-")` as a two-byte `repe cmpsb`, and kuna emitted
//! the whole lowered loop with the direction flag as a live variable:
//!
//! ```text
//! uint1 v18;                              // df
//! v18 = 0;
//! ...
//! v11 = &v12[(uint8)v18 * -2 + 1];        // df-scaled pointer stride
//! v9  = &v10[(uint8)v18 * -2 + 1];
//! ```
//!
//! The `* -2 + 1` is SLEIGH's faithful lowering of the string-op stride,
//! `1 - 2*DF`: forward when `DF == 0`, backward when `DF == 1`. Every string
//! instruction on x86 carries it, so the noise shows up throughout string-heavy
//! code, and it hides the loop's actual shape.
//!
//! # Why the constant was lost
//!
//! `DF` is not unknown. The x86-64 processor spec pins it at function entry
//! (`<tracked_set><set name="DF" val="0"/>`), and the constant-base action
//! materializes that as `DF = 0:1` in the entry block, exactly so the stride can
//! fold. What breaks the chain is the *call*: the gcc compiler spec's default
//! prototype does not list `DF` among the registers a call leaves alone, so SSA
//! construction has to assume a callee might change it and plants
//! `DF = INDIRECT(DF, <call>)` at every call site. Constant propagation cannot
//! see past that, so the fold never happens for any function that calls anything.
//!
//! # Why asserting it is correct, not a guess
//!
//! Both x86 ABIs require the direction flag to be clear at every function
//! boundary — the System V psABI states it must be clear on entry and on return,
//! and the Windows x64 ABI says the same. A callee that sets it must clear it
//! again before returning. The Microsoft prototype in the same compiler spec
//! already lists `DF` as unaffected; the gcc one simply omits it. So this states
//! a rule the platform already guarantees rather than inventing one.
//!
//! The assertion is applied only where the spec is **silent**. A spec that names
//! `DF` — unaffected or killed — has made a deliberate statement and is left
//! alone, and a language with no `DF` register at all (every non-x86 target) is a
//! structural no-op.

use crate::fspec::{effect_type, EffectRecord, ProtoModel};

/// The SLEIGH register name for the x86 direction flag.
const DIRECTION_FLAG: &[u8] = b"DF";

/// Add the direction flag to `model`'s unaffected list when the language defines
/// one and the compiler spec did not mention it.
///
/// `lookup` is a *speculative* register probe, not the exact lookup: it yields
/// `None` on a language that has no such register, which is how every non-x86
/// target falls out. The exact lookup must not be used here — in ghidra mode it
/// is a host query that throws on an undefined name (GH-388).
pub fn assert_direction_flag_unaffected<F>(model: &mut ProtoModel, lookup: F)
where
    F: FnOnce(&[u8]) -> Option<kuna_num::pcoderaw::VarnodeData>,
{
    let Some(df) = lookup(DIRECTION_FLAG) else {
        return; // not an x86 language
    };
    let Some(space) = df.space.clone() else { return };
    let addr = kuna_base::address::Address::new(space, df.offset);
    // The spec already said something about this register: respect it.
    if model.has_effect(&addr, df.size as kuna_base::types::int4) != effect_type::UNKNOWN_EFFECT {
        return;
    }
    model.push_effect(EffectRecord::from_varnode(df, effect_type::UNAFFECTED));
}
