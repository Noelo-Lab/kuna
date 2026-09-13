//! (kuna) The c-language policy objects: the concrete half of the output-language
//! seam for C.
//!
//! [`CSpeller`] carries the declarator algorithm transcribed from
//! `PrintC::pushTypeStart` / `pushTypeEnd` (printc.cc:265/314) plus
//! `buildTypeStack` (printc.cc:143), together with the kuna `realtypes`/`ctypes`
//! relabelling (DIV-5/DIV-6). The bodies are moved verbatim out of `printc.rs`,
//! which now keeps only thin dispatchers so its existing free-function callers
//! and their tests are unchanged.

use std::borrow::Cow;
use std::rc::Rc;

use kuna_base::types::int4;

use crate::dtype::{type_metatype, Datatype};
use crate::kuna_langtypes::{SpellCtx, TypeSpeller};

/// The c-language type speller.
pub struct CSpeller;

/// The singleton reached through `OutLang::C.speller()`.
pub static C_SPELLER: CSpeller = CSpeller;

/// The part of a C declarator that surrounds its identifier.
///
/// C's prefix (`*`) and postfix (`[]`/`()`) operators have opposite binding
/// directions.  Keeping the two halves explicit lets us add modifiers from the
/// outside of the type toward its base without losing that precedence.  In
/// particular, a postfix applied after an ungrouped pointer must wrap the WHOLE
/// declarator accumulated so far: `*a` + `[4]` becomes `(*a)[4]`, while
/// `a[4]` + `*` becomes `*a[4]`.
#[derive(Default)]
struct CDeclarator {
    front: String,
    back: String,
    /// A pointer prefix has been added since the last postfix grouped it.
    ungrouped_pointer: bool,
}

impl CDeclarator {
    fn pointer(&mut self) {
        // We are walking from the outer type toward the base.  A newly reached
        // pointer belongs nearest the base type, so it prefixes the complete
        // declarator already accumulated around the identifier.
        self.front.insert(0, '*');
        self.ungrouped_pointer = true;
    }

    fn postfix(&mut self, suffix: &str) {
        if self.ungrouped_pointer {
            // Group the complete current declarator, including any postfixes
            // already accumulated.  Closing before `self.back` would turn
            // `*a[2]` + `[3]` into `(*a)[2][3]` instead of `(*a[2])[3]`.
            self.front.insert(0, '(');
            self.back.push(')');
            self.ungrouped_pointer = false;
        }
        self.back.push_str(suffix);
    }
}

impl TypeSpeller for CSpeller {
    /// Real C name for a residual `TYPE_UNKNOWN` base, or `None` when the gate is
    /// off / the type is not unknown. Conservative on sign (multi-byte unknowns
    /// are unsigned, since the real sign is genuinely unknown); a pointer-to-
    /// unknown base relabels through the same size table as a scalar, so the
    /// pointee keeps its width and `void` is only the residual fallback.
    fn relabel(
        &self,
        cx: &SpellCtx,
        dt: &Rc<Datatype>,
        under_pointer: bool,
    ) -> Option<Cow<'static, str>> {
        if dt.get_metatype() == type_metatype::TYPE_UNKNOWN {
            if !cx.enabled {
                return None;
            }
            return self.unknown_base(dt.get_size(), under_pointer, cx.long_is_8);
        }
        // (kuna `ctypes`) Every OTHER core type -- the named `int4`/`uint1`/
        // `float8`/`code` vocabulary -- spells as the target's own C type.
        // Restricted to core types so a user-defined or DWARF-recovered type keeps
        // the name it was declared with; that name is already C.
        if !cx.ctypes || !dt.is_core_type() {
            return None;
        }
        crate::kuna_ctypes::core_type_spelling(
            &cx.model,
            dt.get_metatype(),
            dt.get_size(),
            dt.is_char_print(),
        )
        .map(Cow::Borrowed)
    }

    fn anonymous(&self, dt: &Rc<Datatype>) -> String {
        match dt.get_metatype() {
            type_metatype::TYPE_VOID => "void".to_string(),
            _ => format!("undefined{}", dt.get_size()),
        }
    }

    /// The stack is built base-up exactly as `buildTypeStack`; pointer modifiers
    /// go on the front (`*`), array modifiers on the tail (`[N]`), and a `*`
    /// front nested inside an array tail is parenthesised -- the precedence the
    /// RPN `ptr_expr`/`array_expr` tokens encode. `CDeclarator::postfix` is generic
    /// enough for function suffixes, but Datatype traversal currently supplies
    /// pointer and array modifiers only.
    fn declarator(&self, cx: &SpellCtx, ct: &Rc<Datatype>) -> (String, String) {
        // buildTypeStack: walk to the base (named) type, recording the modifiers.
        let mut stack: Vec<Rc<Datatype>> = Vec::new();
        let mut cur = Rc::clone(ct);
        loop {
            stack.push(Rc::clone(&cur));
            if !cur.get_name().is_empty() {
                break; // base type
            }
            let next = match cur.get_metatype() {
                type_metatype::TYPE_PTR => cur.get_ptr_to(),
                type_metatype::TYPE_ARRAY => cur.get_array_base(),
                _ => None, // other anonymous type: stop
            };
            match next {
                Some(n) => cur = n,
                None => break,
            }
        }
        // The base type's display name (anonymous -> `undefined<N>` / `void`).
        let base = stack.last().expect("declarator: non-empty stack");
        // (kuna) realtypes: relabel a residual TYPE_UNKNOWN base as a real C type
        // by size. Under a pointer modifier the same size table applies, so the
        // pointee width survives into the declaration; only a residual size with
        // no natural C type degrades to `void` there (the `*` chain is laid out by
        // the walk below).
        let under_pointer = stack[..stack.len() - 1]
            .iter()
            .any(|m| matches!(m.get_metatype(), type_metatype::TYPE_PTR));
        let base_name = if let Some(n) = self.relabel(cx, base, under_pointer) {
            n.into_owned()
        } else if base.get_name().is_empty() {
            self.anonymous(base)
        } else {
            base.get_display_name().to_string()
        };

        // Walk modifiers OUTERMOST-to-base (`stack[0]..stack[len-2]`).  This is
        // the order in which a C declarator is built around its identifier.  A
        // pointer followed inward by an array postfix must be grouped;
        // the mirror ordering is an array of pointers and must not be grouped.
        let mut decl = CDeclarator::default();
        for ct_mod in stack.iter().take(stack.len() - 1) {
            match ct_mod.get_metatype() {
                type_metatype::TYPE_PTR => decl.pointer(),
                type_metatype::TYPE_ARRAY => {
                    let n = ct_mod.num_elements().unwrap_or_else(|| {
                        let base = ct_mod
                            .get_array_base()
                            .map(|b| b.get_size())
                            .unwrap_or(1)
                            .max(1);
                        ct_mod.get_size() / base
                    });
                    decl.postfix(&format!("[{n}]"));
                }
                _ => {}
            }
        }
        // `<base> <front>` with a single separating space before any `*` modifiers
        // (the `type_expr_space` token); a bare base type has no trailing space
        // here (the caller adds the space before the identifier).
        let front_full = if decl.front.is_empty() {
            base_name
        } else {
            format!("{base_name} {}", decl.front)
        };
        (front_full, decl.back)
    }

    fn type_name(&self, cx: &SpellCtx, t: &Rc<Datatype>) -> String {
        // (kuna) realtypes: a scalar residual TYPE_UNKNOWN (the named `xunknownN`
        // core type or an anonymous `undefined<N>`) becomes its real C type by size.
        if let Some(n) = self.relabel(cx, t, false) {
            return n.into_owned();
        }
        let name = t.get_name();
        if !name.is_empty() {
            return name.to_string();
        }
        match t.get_metatype() {
            // An anonymous pointer renders as `<pointee> *` (recursively), exactly
            // as `push_cast_type` does for a `(char *)` cast. `declarator` walks
            // the modifier chain to the named base and lays out the `*` front.
            type_metatype::TYPE_PTR => self.declarator(cx, t).0,
            _ => self.anonymous(t),
        }
    }

    /// Size -> standard-C name for an unknown value. `None` for sizes with no
    /// natural single C type (3/5/6/7/10/16...), which keep the `undefined<N>`
    /// form; under a pointer those residual sizes fall back to `void` (the
    /// modifier walk adds the `*` chain), since `void *` is the only spelling that
    /// carries no width claim. A pointee whose size *does* have a natural type
    /// keeps it, so the declaration agrees with the stride the index/cast
    /// expressions were built from.
    fn unknown_base(
        &self,
        size: int4,
        under_pointer: bool,
        long_is_8: bool,
    ) -> Option<Cow<'static, str>> {
        Some(Cow::Borrowed(match size {
            1 => "char",
            2 => "unsigned short",
            4 => "unsigned int",
            8 => {
                if long_is_8 {
                    "unsigned long"
                } else {
                    "unsigned long long"
                }
            }
            _ => {
                return if under_pointer {
                    Some(Cow::Borrowed("void"))
                } else {
                    None
                };
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::CDeclarator;

    #[derive(Clone, Copy)]
    enum Modifier {
        Pointer,
        Array(usize),
        SyntheticPostfix(&'static str),
    }

    fn spell(modifiers_outer_to_inner: &[Modifier]) -> String {
        let mut decl = CDeclarator::default();
        for modifier in modifiers_outer_to_inner {
            match modifier {
                Modifier::Pointer => decl.pointer(),
                Modifier::Array(n) => decl.postfix(&format!("[{n}]")),
                Modifier::SyntheticPostfix(suffix) => decl.postfix(suffix),
            }
        }
        format!("int {}x{}", decl.front, decl.back)
    }

    /// The generic helper must distinguish every adjacent pointer/postfix
    /// ordering. Datatype traversal currently supplies only array postfixes;
    /// synthetic function suffixes here fence the helper's grouping rule. The
    /// right-hand strings are declarations accepted by a C compiler.
    #[test]
    fn pointer_array_and_function_precedence_table() {
        use Modifier::{Array as A, Pointer as P, SyntheticPostfix as S};

        let cases: &[(&[Modifier], &str)] = &[
            (&[P], "int *x"),
            (&[A(2)], "int x[2]"),
            (&[P, A(2)], "int (*x)[2]"),
            (&[A(2), P], "int *x[2]"),
            (&[P, S("(void)")], "int (*x)(void)"),
            (&[S("(void)"), P], "int *x(void)"),
            (&[P, P, A(2)], "int (**x)[2]"),
            (&[P, A(2), P], "int *(*x)[2]"),
            (&[A(2), P, A(3)], "int (*x[2])[3]"),
            (&[A(2), P, S("(void)")], "int (*x[2])(void)"),
            (&[P, S("(void)"), P], "int *(*x)(void)"),
            (&[P, S("(void)"), P, A(3)], "int (*(*x)(void))[3]"),
        ];

        for (mods, expected) in cases {
            assert_eq!(spell(mods), *expected);
        }
    }
}
