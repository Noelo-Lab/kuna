//! (kuna) The ABI seam: what a function's recovered calling convention looks like
//! in the output language.
//!
//! # Why this axis is thin, deliberately
//!
//! The other two output-language axes are thick because they have to be: every
//! statement has a shape ([`crate::kuna_lang::LangForms`]) and every value has a
//! type ([`crate::kuna_langtypes::TypeSpeller`]). The ABI axis is thin because
//! `extern "Rust"` is **unspecified** and, for the scalar arguments a decompiler
//! actually recovers, System-V-shaped -- the same convention the cspec already
//! describes. A `build_param_list("rust")` strategy would encode a guess as an
//! engine fact, which is exactly what `fspec.rs`'s allowlist (`""`/`"standard"`/
//! `"register"`, error otherwise) exists to prevent.
//!
//! Rust's genuinely distinct ABI surface -- a niche-optimized `Option<&T>` that
//! is a nullable pointer, a `Result<T, E>` tagged across `rax:rdx`, a slice
//! passed as a `(ptr, len)` register pair -- is an **enum and discriminant
//! inference** problem living in P5, not a convention problem living here.
//! Modelling it as a convention would be modelling the wrong thing.
//!
//! So this trait carries exactly one decision, and that decision is real: which
//! `extern` the signature must declare. It is load-bearing rather than
//! decorative -- rustc rejects a C-variadic on anything but an
//! `unsafe extern "C" fn`, so the output does not compile without it.
//!
//! # What a third language adds, and where
//!
//! Go is the case that justifies the seam existing at all, because its ABI
//! genuinely differs rather than merely being spelled differently: a register ABI
//! since 1.17, multi-value returns, and a two-word representation for interfaces
//! and slices. Those need two more members -- a preferred prototype model
//! consulted where `ActionPrototypeTypes` picks one, and a multi-return form
//! consulted where the `RETURN` op is emitted. Neither is added here, because a
//! trait method with no consumer is a wish list rather than an interface: the
//! `ArchContext` the P4 action reads carries only `defaultfp`/`evalfp_current`
//! and no named-model registry, so a `preferred_model` hook would be plumbing in
//! service of a function that returns `None` for both languages kuna emits.

/// Per-language rendering of a recovered calling convention.
pub trait LangAbi {
    /// The `extern "..."` marker this function's signature must declare, or
    /// `None` to declare none.
    ///
    /// `model_name` is the recovered prototype model (`__stdcall`, `__fastcall`,
    /// …); `is_variadic` is whether the recovered prototype takes `...`.
    fn extern_marker(&self, model_name: &str, is_variadic: bool) -> Option<&'static str>;
}

/// C declares no `extern` in a definition -- the convention, when it is shown at
/// all, is the `option conventionprinting` keyword (`__cdecl`), which is a
/// different token in a different position.
pub struct CAbi;

/// The singleton reached through `OutLang::C.abi()`.
pub static C_ABI: CAbi = CAbi;

impl LangAbi for CAbi {
    fn extern_marker(&self, _model_name: &str, _is_variadic: bool) -> Option<&'static str> {
        None
    }
}

/// Rust's rule.
pub struct RustAbi;

/// The singleton reached through `OutLang::Rust.abi()`.
pub static RUST_ABI: RustAbi = RustAbi;

impl LangAbi for RustAbi {
    /// `extern "C"` exactly when the prototype is variadic, and otherwise
    /// nothing.
    ///
    /// The variadic case is forced: a C-variadic parameter is only legal on an
    /// `unsafe extern "C" fn`, and rustc rejects it anywhere else (a semantic
    /// rule, so a token-level parser accepts the shorter form and the compiler
    /// does not).
    ///
    /// Every other function declares nothing, which means `extern "Rust"` -- the
    /// default, and unspellable. That is the honest answer rather than a
    /// conservative one: marking every recovered function `extern "C"` would
    /// assert a convention the recovery cannot support, and a Rust binary's own
    /// functions are precisely the ones that are NOT `extern "C"`.
    fn extern_marker(&self, _model_name: &str, is_variadic: bool) -> Option<&'static str> {
        is_variadic.then_some("extern \"C\" ")
    }
}

#[cfg(test)]
mod tests;
