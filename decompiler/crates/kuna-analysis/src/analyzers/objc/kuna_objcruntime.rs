//! The C entry points every message send and every `NSLog` goes through.
//!
//! `objc_msgSend`, `objc_msgSendSuper` and `objc_msgSendSuper2` take the
//! receiver (or the `objc_super` record) and the selector in the first two
//! argument registers, then the method's own arguments exactly as the method
//! receives them. Unprototyped, a call to one is recovered from its register
//! trials in the model's entry order, which on AArch64 lists `d0..d7` before
//! `x0`: `[super initWithFrame:r]` printed
//! `objc_msgSendSuper2(r.x,r.y,r.width,r.height,&sup,sel_initWithFrame_)`.
//! `NSLog(NSString *format, ...)` is variadic, and Apple's AArch64 convention
//! passes every variadic argument on the stack, so with no prototype the stack
//! slots past seven unused `x` registers end the argument list and the `%@`
//! values are dropped.
//!
//! Each is declared as `<objc/message.h>`, `objc-abi.h` and
//! `<Foundation/NSObjCRuntime.h>` declare it: the fixed parameters, then `...`.
//! The method arguments of a send are then the call's open tail, taken from
//! the same register and stack trials as before but placed after the receiver
//! and the selector. The return is left to recovery for the sends,
//! since the method decides whether it comes back in `x0`, `d0` or nowhere.
//! Address-keyed on the image's resolved imports, so a definition of the same
//! name (the runtime itself) keeps what its body says.

use kuna_decomp::fspec::PrototypePieces;

use super::encoding::{self, TypeCtx};
use crate::pass::AnalysisOutput;

/// What one entry point declares before its `...`. Every fixed parameter is
/// an unnamed `void *`: a receiver may be an instance or a class, and a typed
/// or named parameter would rename the caller's variables and type a class
/// method's receiver as `objc_object *`.
#[derive(Clone, Copy)]
enum Fixed {
    /// `id self, SEL op`.
    Send,
    /// `struct objc_super *super, SEL op`.
    SuperSend,
    /// `NSString *format`, with a `void` return.
    Log,
}

const ENTRY_POINTS: &[(&str, Fixed)] = &[
    ("objc_msgSend", Fixed::Send),
    ("objc_msgSendSuper", Fixed::SuperSend),
    ("objc_msgSendSuper2", Fixed::SuperSend),
    ("NSLog", Fixed::Log),
];

fn pieces(name: &str, fixed: Fixed, tctx: &TypeCtx) -> Option<PrototypePieces> {
    let ptr = || encoding::decode_ivar_type("^v", tctx);
    let (outtype, intypes) = match fixed {
        Fixed::Send | Fixed::SuperSend => (None, vec![ptr()?, ptr()?]),
        Fixed::Log => (Some(tctx.types.get_type_void().ok()?), vec![ptr()?]),
    };
    Some(PrototypePieces {
        name: name.to_string(),
        outtype,
        innames: vec![String::new(); intypes.len()],
        first_var_arg_slot: intypes.len() as i32,
        intypes,
        ..Default::default()
    })
}

/// Park the declared prototype on every import of the image that names one of
/// [`ENTRY_POINTS`].
pub fn emit(imports: &[(String, u64)], tctx: &TypeCtx, out: &mut AnalysisOutput) {
    for (name, addr) in imports {
        let Some(&(_, fixed)) = ENTRY_POINTS.iter().find(|(n, _)| n == name) else { continue };
        if let Some(p) = pieces(name, fixed, tctx) {
            out.prototypes_at.push((*addr, p));
        }
    }
}
