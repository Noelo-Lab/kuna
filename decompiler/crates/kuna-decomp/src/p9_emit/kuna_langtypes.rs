//! (kuna) The type-spelling seam: how a recovered [`Datatype`] is *named* in the
//! output language.
//!
//! # Recovery is language-independent; only spelling is not
//!
//! P5 recovers a `Datatype`; this module decides whether that shows up as
//! `uint4`, `unsigned int`, or `u32`. Spelling deliberately lives in the printer
//! rather than in the type factory, for the reason `kuna_ctypes.rs` already
//! records: [`Datatype::hash_name`] makes the registered name determine the type
//! id, identifier generation derives from the first character of the type name
//! (`float8` → `fVar1`), and the console's C-type parser resolves base types by
//! `TypeFactory::find_by_name` — so renaming the interned core types would break
//! the Ghidra wire protocol.
//!
//! # Why the language rides on [`SpellCtx`]
//!
//! `SpellCtx` (the former `printc::RealTypeCtx`) is `Copy` and is already
//! threaded through every declarator chokepoint. Carrying [`OutLang`] on it means
//! the free-function declarator family learns the language with no new threading
//! and no new parameter: `rt.speller()` is available wherever `rt` already was.
//!
//! # The C declarator's `(front, back)` is a C-ism
//!
//! C declarators wrap the identifier (`int4 (*a)[1]`), so the C speller returns a
//! front/back pair. Rust's `[T; N]` and Go's `[N]T` are prefixes and would have
//! to fake the split, which is why [`TypeSpeller::declarator`] is documented in
//! terms of `<front><name><back>` rather than promising a meaningful `back`.
//!
//! [`Datatype`]: crate::dtype::Datatype
//! [`Datatype::hash_name`]: crate::dtype::Datatype

use std::borrow::Cow;
use std::rc::Rc;

use kuna_base::types::int4;

use crate::dtype::Datatype;
use crate::kuna_lang::OutLang;

/// The per-document type-rendering context (C++ has no analogue: the `realtypes`
/// and `ctypes` gates are kuna divergences).
///
/// `Copy`, so it threads cheaply through the declarator chokepoints.
#[derive(Clone, Copy)]
pub struct SpellCtx {
    /// The output language, which selects the [`TypeSpeller`].
    pub(crate) lang: OutLang,
    /// (kuna `realtypes`) Relabel residual `TYPE_UNKNOWN` bases as real types.
    pub(crate) enabled: bool,
    /// The data-model fact that `long` is 8 bytes, so an 8-byte unknown reads
    /// `unsigned long` on LP64 and `unsigned long long` on LLP64.
    pub(crate) long_is_8: bool,
    /// (kuna `ctypes`) Spell the NAMED core types (`int4`/`uint1`/`float8`/`code`)
    /// as the target's own type names too, not just the residual unknowns.
    pub(crate) ctypes: bool,
    /// (kuna `ctypes`) The target's declared C scalar widths, which is what makes
    /// the spelling per-architecture rather than a guess.
    pub(crate) model: crate::kuna_ctypes::CDataModel,
}

impl SpellCtx {
    /// The disabled context — never relabels (preserves the upstream
    /// `xunknownN`/`undefined<N>` rendering).
    pub const OFF: SpellCtx = SpellCtx {
        lang: OutLang::C,
        enabled: false,
        long_is_8: true,
        ctypes: false,
        model: crate::kuna_ctypes::CDataModel::LP64,
    };

    /// The speller for this context's output language.
    #[inline]
    pub fn speller(&self) -> &'static dyn TypeSpeller {
        self.lang.speller()
    }
}

/// Per-language spelling of a recovered type.
///
/// Implementations are zero-sized policy objects reached through
/// [`OutLang::speller`]; the C one delegates to the ported declarator algorithm
/// verbatim, so routing through this trait is byte-identical for C.
pub trait TypeSpeller {
    /// The base-type name to use for `dt`, or `None` to keep the type's own
    /// declared name. `under_pointer` is true when a pointer modifier sits
    /// between `dt` and the declared object, which C uses to decide whether a
    /// width-less residual size may degrade to `void`.
    fn relabel(
        &self,
        cx: &SpellCtx,
        dt: &Rc<Datatype>,
        under_pointer: bool,
    ) -> Option<Cow<'static, str>>;

    /// The name for an anonymous (unnamed) base type — C's `undefined<N>`/`void`
    /// fallback.
    fn anonymous(&self, dt: &Rc<Datatype>) -> String;

    /// `(front, back)` such that `<front><name><back>` declares an object named
    /// `name` of type `ct`. A language whose types are pure prefixes returns an
    /// empty `back`.
    fn declarator(&self, cx: &SpellCtx, ct: &Rc<Datatype>) -> (String, String);

    /// The type token to render in a declaration's type position.
    fn type_name(&self, cx: &SpellCtx, t: &Rc<Datatype>) -> String;

    /// Real name for a residual unknown of `size` bytes, or `None` when the
    /// language has no natural single type of that width.
    fn unknown_base(&self, size: int4, under_pointer: bool, long_is_8: bool)
        -> Option<Cow<'static, str>>;
}
