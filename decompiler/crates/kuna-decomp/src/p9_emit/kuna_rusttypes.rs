//! (kuna) The Rust type speller: how a recovered `Datatype` is spelled in Rust.
//!
//! The sibling of [`crate::kuna_langc`]'s `CSpeller`, and the reason
//! `TypeSpeller::declarator` is specified as `<front><name><back>` rather than
//! as a C declarator: Rust types are **pure prefixes**, so `back` is always
//! empty and the whole type is built by wrapping inward-out (`*mut [u8; 4]`)
//! instead of splitting around the identifier (`int4 (*a)[1]`).
//!
//! # Deliberate spelling choices
//!
//! - kuna's `char`/`wchar2`/`wchar4` spell as `u8`/`u16`/`u32`, never as Rust
//!   `char`. A Rust `char` is a 4-byte Unicode scalar value with a validity
//!   invariant; a decompiled byte is not one, and spelling it `char` would be a
//!   claim the recovery cannot support.
//! - A residual `TYPE_UNKNOWN` of a width Rust has no integer for (3/5/6/7,
//!   10, 16) spells `[u8; N]`, which is *more* faithful than C's `undefined3` —
//!   it names the storage exactly and does not pretend to be a scalar.
//! - `TYPE_CODE` spells `*const ()` rather than a real `fn` type: the recovered
//!   prototype is not carried through this path, and an `unsafe extern "C" fn()`
//!   with the wrong arity would be worse than an honest opaque pointer.
//! - Pointers spell `*mut T` — raw, and uniformly `mut`. Not references: kuna
//!   recovers no borrow information, and `&T` would assert an aliasing invariant
//!   that is not known to hold. Uniformly `mut` because const-qualification is
//!   not recovered either, and guessing `*const` on a pointer that is written
//!   through would be the worse error.
//!
//! Unlike the C speller, the Rust speller does not consult the `realtypes` /
//! `ctypes` gates. Those exist to preserve upstream Ghidra's `xunknownN` /
//! `int4` vocabulary on demand; Rust has no such vocabulary to preserve, so it
//! always spells.

use std::borrow::Cow;
use std::rc::Rc;

use kuna_base::types::int4;

use crate::dtype::{type_metatype, Datatype};
use crate::kuna_langtypes::{SpellCtx, TypeSpeller};

/// The rust-language type speller.
pub struct RustSpeller;

/// The singleton reached through `OutLang::Rust.speller()`.
pub static RUST_SPELLER: RustSpeller = RustSpeller;

/// The Rust integer name for a `size`-byte value of the given signedness, or
/// `None` when Rust has no integer of that width.
fn int_name(size: int4, signed: bool) -> Option<&'static str> {
    Some(match (size, signed) {
        (1, true) => "i8",
        (2, true) => "i16",
        (4, true) => "i32",
        (8, true) => "i64",
        (16, true) => "i128",
        (1, false) => "u8",
        (2, false) => "u16",
        (4, false) => "u32",
        (8, false) => "u64",
        (16, false) => "u128",
        _ => return None,
    })
}

/// The Rust float name for a `size`-byte value, or `None`. x87's 10-byte and the
/// 16-byte long double have no Rust type; they fall through to a byte array
/// rather than being widened to `f64`, which would misstate the storage.
fn float_name(size: int4) -> Option<&'static str> {
    match size {
        4 => Some("f32"),
        8 => Some("f64"),
        _ => None,
    }
}

/// The storage-exact fallback for a width Rust cannot name as a scalar.
fn byte_array(size: int4) -> String {
    format!("[u8; {}]", size.max(1))
}

impl RustSpeller {
    /// The full prefix spelling of `ct`, built inward-out.
    fn type_expr(&self, cx: &SpellCtx, ct: &Rc<Datatype>, depth: u32) -> String {
        // A recovered type graph can be cyclic through a struct pointer; the C
        // declarator walk stops at the first NAMED type, and this one needs the
        // same guard because it recurses through pointees.
        if depth > 16 {
            return "*const ()".to_string();
        }
        match ct.get_metatype() {
            type_metatype::TYPE_PTR => {
                let inner = match ct.get_ptr_to() {
                    Some(p) => self.type_expr(cx, &p, depth + 1),
                    None => "()".to_string(),
                };
                format!("*mut {inner}")
            }
            type_metatype::TYPE_ARRAY => {
                let base = ct.get_array_base();
                let elem = match &base {
                    Some(b) => self.type_expr(cx, b, depth + 1),
                    None => "u8".to_string(),
                };
                let n = ct.num_elements().unwrap_or_else(|| {
                    let w = base.map(|b| b.get_size()).unwrap_or(1).max(1);
                    ct.get_size() / w
                });
                format!("[{elem}; {n}]")
            }
            _ => match self.relabel(cx, ct, false) {
                Some(n) => n.into_owned(),
                None => {
                    let name = ct.get_name();
                    if name.is_empty() {
                        self.anonymous(ct)
                    } else {
                        type_path(ct.get_display_name())
                    }
                }
            },
        }
    }
}

/// A recovered name as a Rust **path expression**: `a::b::c`, each component an
/// identifier.
///
/// `::` is legal wherever a name is *used* — a call, a static — because that
/// position takes a path, not an identifier. Only a *definition* needs a bare
/// identifier, which is what [`path_tail`] is for. Rewriting `::` to `__`
/// everywhere (which is what a naive identifier sanitizer does) throws away the
/// module structure for nothing and produces
/// `alloc__vec__Vec__resize` where `alloc::vec::Vec::resize` is both valid and
/// what a Rust reader expects.
pub(crate) fn sanitize_path(name: &str) -> String {
    let out = name
        .split("::")
        .map(sanitize)
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>()
        .join("::");
    if out.is_empty() { sanitize(name) } else { out }
}

/// The last component of a path — the identifier a `fn` definition can carry.
///
/// The full path is not lost: the prototype emitter prints it in a comment
/// directly above the function.
pub(crate) fn path_tail(name: &str) -> String {
    let tail = name.rsplit("::").find(|c| !c.trim().is_empty()).unwrap_or(name);
    sanitize(tail)
}

/// A recovered **type** name, spelled as Rust rather than flattened to an
/// identifier.
///
/// A recovered composite carries the source spelling the debug info recorded --
/// `Result<u8, u32>`, `Vec<u8, alloc::alloc::Global>`, `(u8, u32)`. Every
/// character in those is legal in Rust *type* position, so running them through
/// the identifier sanitizer turns `Result<u8, u32>` into `Result_u8__u32_` and
/// throws away the one thing a reader wanted. Type position is not identifier
/// position, and this is the spelling for it.
///
/// Anything outside the type grammar (a DWARF `{closure#0}`, a codegen-unit
/// suffix) still collapses to `_`, and a name whose brackets do not balance
/// falls back to [`sanitize`] whole -- an unbalanced `<` would swallow the rest
/// of the declaration.
pub(crate) fn type_path(name: &str) -> String {
    let keep = |c: char| {
        c.is_ascii_alphanumeric()
            || matches!(c, '_' | '<' | '>' | ',' | ':' | '&' | '[' | ']' | ';' | '*' | '(' | ')' | '\'' | '+' | ' ')
    };
    let (mut out, mut angle, mut square, mut round) = (String::new(), 0i32, 0i32, 0i32);
    let mut it = name.chars().peekable();
    while let Some(c) = it.next() {
        if c == '-' && it.peek() == Some(&'>') {
            it.next();
            out.push_str("->");
            continue;
        }
        match c {
            '<' => angle += 1,
            '>' => angle -= 1,
            '[' => square += 1,
            ']' => square -= 1,
            '(' => round += 1,
            ')' => round -= 1,
            _ => {}
        }
        out.push(if keep(c) { c } else { '_' });
    }
    if angle != 0 || square != 0 || round != 0 {
        return sanitize(name);
    }
    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    out
}

/// Rust identifiers admit only `[A-Za-z0-9_]`.
///
/// A recovered name can carry a demangled path (`hello::main`), a template or
/// generic argument list (`driftsort_main<T, U>`), or an anonymous tag. C emits
/// those verbatim too and is equally not-C for it; the difference is that Rust
/// output is parsed, so the same latent problem has to be fixed rather than
/// tolerated. The original is not lost -- the prototype emitter records it in a
/// comment above the function when the two differ.
pub(crate) fn sanitize(name: &str) -> String {
    let mut out: String = name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() || c == '_' { c } else { '_' })
        .collect();
    if out.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    out
}

impl TypeSpeller for RustSpeller {
    fn relabel(
        &self,
        _cx: &SpellCtx,
        dt: &Rc<Datatype>,
        _under_pointer: bool,
    ) -> Option<Cow<'static, str>> {
        let size = dt.get_size();
        let name: Option<&'static str> = match dt.get_metatype() {
            // A recovered text byte is a byte, not a Rust `char`.
            type_metatype::TYPE_INT if dt.is_char_print() => int_name(size, false),
            type_metatype::TYPE_INT => int_name(size, true),
            type_metatype::TYPE_UINT => int_name(size, false),
            type_metatype::TYPE_FLOAT => float_name(size),
            type_metatype::TYPE_BOOL => Some("bool"),
            type_metatype::TYPE_VOID => Some("()"),
            // Ghidra's pseudo-type for a function body; it only reaches the output
            // through a pointer.
            type_metatype::TYPE_CODE => Some("*const ()"),
            // Sign is genuinely unknown for an un-inferred value, so unsigned.
            type_metatype::TYPE_UNKNOWN => {
                return Some(match int_name(size, false) {
                    Some(n) => Cow::Borrowed(n),
                    None => Cow::Owned(byte_array(size)),
                })
            }
            _ => None,
        };
        match name {
            Some(n) => Some(Cow::Borrowed(n)),
            // A named scalar of a width Rust cannot spell still needs storage.
            None if matches!(
                dt.get_metatype(),
                type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_FLOAT
            ) =>
            {
                Some(Cow::Owned(byte_array(size)))
            }
            None => None,
        }
    }

    fn anonymous(&self, dt: &Rc<Datatype>) -> String {
        match dt.get_metatype() {
            type_metatype::TYPE_VOID => "()".to_string(),
            _ => match int_name(dt.get_size(), false) {
                Some(n) => n.to_string(),
                None => byte_array(dt.get_size()),
            },
        }
    }

    /// Rust types are pure prefixes, so `back` is always empty.
    fn declarator(&self, cx: &SpellCtx, ct: &Rc<Datatype>) -> (String, String) {
        (self.type_expr(cx, ct, 0), String::new())
    }

    fn type_name(&self, cx: &SpellCtx, t: &Rc<Datatype>) -> String {
        self.type_expr(cx, t, 0)
    }

    fn unknown_base(
        &self,
        size: int4,
        _under_pointer: bool,
        _long_is_8: bool,
    ) -> Option<Cow<'static, str>> {
        Some(match int_name(size, false) {
            Some(n) => Cow::Borrowed(n),
            None => Cow::Owned(byte_array(size)),
        })
    }
}

#[cfg(test)]
mod tests;
