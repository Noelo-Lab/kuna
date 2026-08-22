//! Name-normalization and match tests for [`super`].
//!
//! Every mangled string below is a verbatim `kuna functions --json` name from a
//! real `rustc 1.90` binary (`private/FakeCrypt`), so the escaped forms the
//! matcher has to survive are the ones it actually meets.

use super::{is_cleanup_call_name, normalize_rust_name, CLEANUP_FUNCTIONS};

#[test]
fn legacy_escapes_and_generic_args_normalize_to_the_bare_path() {
    assert_eq!(
        normalize_rust_name(
            "core::ptr::drop_in_place$LT$core..result..Result$LT$usize$C$std..io..error..Error$GT$$GT$"
        ),
        "core::ptr::drop_in_place"
    );
    assert_eq!(
        normalize_rust_name("core::ptr::drop_in_place$LT$std..fs..File$GT$"),
        "core::ptr::drop_in_place"
    );
    assert_eq!(
        normalize_rust_name("alloc::raw_vec::RawVecInner$LT$A$GT$::deallocate"),
        "alloc::raw_vec::RawVecInner::deallocate"
    );
    // v0 mangling / a demangler that emits real angle brackets and a turbofish.
    assert_eq!(
        normalize_rust_name("core::ptr::drop_in_place::<alloc::string::String>"),
        "core::ptr::drop_in_place"
    );
}

#[test]
fn qualified_paths_normalize_to_the_trait_name() {
    // `<alloc::vec::Vec<T, A> as core::ops::drop::Drop>::drop`
    assert_eq!(
        normalize_rust_name(
            "_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop"
        ),
        "core::ops::drop::Drop::drop"
    );
    // Doubly nested: the DropGuard of IntoIter's own Drop impl.
    assert_eq!(
        normalize_rust_name(
            "_$LT$$LT$alloc..vec..into_iter..IntoIter$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$..drop..DropGuard$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop"
        ),
        "core::ops::drop::Drop::drop"
    );
    // `<alloc::alloc::Global as core::alloc::Allocator>::deallocate` keeps the
    // TRAIT name -- and that name is deliberately not on the list.
    assert_eq!(
        normalize_rust_name(
            "_$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$::deallocate"
        ),
        "core::alloc::Allocator::deallocate"
    );
}

#[test]
fn the_shipped_list_matches_the_real_symbol_forms() {
    for want in [
        "core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$",
        "core::ptr::drop_in_place$LT$std..fs..File$GT$",
        "_$LT$alloc..boxed..Box$LT$T$C$A$GT$$u20$as$u20$core..ops..drop..Drop$GT$::drop",
        "alloc::raw_vec::RawVecInner$LT$A$GT$::deallocate",
        "__rust_dealloc",
        "__rustc::__rust_dealloc",
    ] {
        assert!(is_cleanup_call_name(want), "{want} must be a cleanup call");
    }
}

/// The whole reason the list is not a `starts_with` test, and the whole reason
/// `free`/`close` are not on it.
#[test]
fn c_and_user_rust_names_are_never_matched() {
    for keep in [
        // Deleting these from a C binary would be a catastrophically wrong answer.
        "free",
        "close",
        "_close",
        "cfree",
        "xfree",
        // A user function whose NAME merely contains `drop`.
        "FakeCrypt::fileops::drop_ransom_note",
        "drop_privileges",
        "core::ptr::drop_in_place_helper",
        // A different trait that also has a `drop` method.
        "_$LT$T$u20$as$u20$crossbeam_epoch..atomic..Pointable$GT$::drop",
        // C++ templates normalize too, and still match nothing.
        "std::vector<int>::push_back",
        "leveldb::DBImpl::~DBImpl",
        "",
    ] {
        assert!(!is_cleanup_call_name(keep), "{keep} must be kept");
    }
}

#[test]
fn the_list_is_already_normalized() {
    for entry in CLEANUP_FUNCTIONS {
        assert_eq!(&normalize_rust_name(entry), entry, "{entry} must be canonical");
        assert!(is_cleanup_call_name(entry), "{entry} must match itself");
    }
    // Oxidizer's C-name entries stay out (see the module doc).
    assert!(!CLEANUP_FUNCTIONS.contains(&"free"));
    assert!(!CLEANUP_FUNCTIONS.contains(&"close"));
    assert!(!CLEANUP_FUNCTIONS.contains(&"_close"));
}

/// rustc's v0 mangling renders the drop shim without its `core::ptr::` prefix.
/// Those names ARE matched, but only when they carry generic arguments -- which
/// is what keeps a hypothetical C function called `drop_in_place` safe.
#[test]
fn the_v0_drop_shim_spelling_is_matched_only_when_generic() {
    for want in [
        "drop_in_place<alloc::sync::ArcInner<gimli::read::dwarf::Dwarf<gimli::endianity::LittleEndian>>>",
        "drop_in_place<[alloc::boxed::Box<(dyn core::ops::function::FnOnce<(), Output=()>), alloc::alloc::Global>]>",
        "drop_in_place$LT$alloc..string..String$GT$",
    ] {
        assert!(is_cleanup_call_name(want), "{want} must be a cleanup call");
        assert_eq!(normalize_rust_name(want), "drop_in_place");
    }
    // A bare, non-generic `drop_in_place` is NOT matched: only the qualified
    // `core::ptr::drop_in_place` is a Rust-only certainty.
    assert!(!is_cleanup_call_name("drop_in_place"));
    assert!(is_cleanup_call_name("core::ptr::drop_in_place"));
}

/// Malformed / adversarial input must terminate and never match.
#[test]
fn malformed_names_terminate() {
    for junk in [
        "core::ptr::drop_in_place$LT$",
        "core::ptr::drop_in_place$GT$",
        "drop<<<<<<<<<<",
        "drop>>>>>>",
        "$$$$drop$$$$",
        "dealloc$u$",
        "$u110000$drop",
    ] {
        let _ = normalize_rust_name(junk);
        assert!(!is_cleanup_call_name(junk), "{junk} must not match");
    }
}
