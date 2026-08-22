//! Tests for the ABI seam.

use super::*;
use crate::kuna_lang::OutLang;

/// The rule that makes this seam load-bearing rather than decorative: rustc
/// rejects a C-variadic on anything but an `unsafe extern "C" fn`.
#[test]
fn rust_declares_extern_c_exactly_for_variadics() {
    let abi = OutLang::Rust.abi();
    assert_eq!(abi.extern_marker("__stdcall", true), Some("extern \"C\" "));
    assert_eq!(abi.extern_marker("__cdecl", true), Some("extern \"C\" "));
    // Everything else declares nothing, which means `extern "Rust"` -- the
    // default, and unspellable. A Rust binary's own functions are exactly the
    // ones that are NOT `extern "C"`, so claiming otherwise would be a lie about
    // the recovery.
    assert_eq!(abi.extern_marker("__stdcall", false), None);
    assert_eq!(abi.extern_marker("__cdecl", false), None);
}

/// C shows a convention through `option conventionprinting`'s keyword, in a
/// different position; it never declares an `extern`.
#[test]
fn c_declares_no_extern() {
    let abi = OutLang::C.abi();
    assert_eq!(abi.extern_marker("__cdecl", false), None);
    assert_eq!(abi.extern_marker("__cdecl", true), None);
}
