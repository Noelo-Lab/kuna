//! Unit tests for the `symbolnamechars` sanitizer and its env gate.

use super::*;

fn safe(name: &[u8]) -> String {
    String::from_utf8(sanitize_symbol_name_bytes(name, NameChars::Safe).into_owned()).unwrap()
}

fn ident(name: &[u8]) -> String {
    String::from_utf8(sanitize_symbol_name_bytes(name, NameChars::Ident).into_owned()).unwrap()
}

/// The three structural shapes that corrupt the C document, and the invalid-UTF-8
/// shape that collapses two names onto one.
#[test]
fn safe_rewrites_exactly_the_structural_set() {
    assert_eq!(safe(b"a*/b"), "a_x2a_x2fb");
    assert_eq!(safe(b"a/*b"), "a_x2f_x2ab");
    assert_eq!(safe(b"a//b"), "a_x2f_x2fb");
    assert_eq!(safe(b"a\nb"), "a_x0ab");
    assert_eq!(safe(b"a\rb"), "a_x0db");
    assert_eq!(safe(b"a\tb"), "a_x09b");
    assert_eq!(safe(b"a\"b"), "a_x22b");
    assert_eq!(safe(b"a'b"), "a_x27b");
    assert_eq!(safe(b"a\\b"), "a_x5cb");
    assert_eq!(safe(b"a\x7fb"), "a_x7fb");
    assert_eq!(safe(b"a\x80b"), "a_x80b");
    // BOTH characters of a delimiter pair are rewritten, so no `*/` survives in
    // any spelling, including the overlapping ones.
    assert_eq!(safe(b"/*"), "_x2f_x2a");
    assert_eq!(safe(b"a/*/b"), "a_x2f_x2a_x2fb");
    assert_eq!(safe(b"a**/b"), "a*_x2a_x2fb");
    for out in [safe(b"a*/b"), safe(b"a/*b"), safe(b"a//b"), safe(b"a/*/b")] {
        assert!(!out.contains("*/") && !out.contains("/*") && !out.contains("//"), "{out}");
    }
}

/// The reason `safe` and `ident` are separate values: `safe` must not touch a
/// single one of the names a real optimizing toolchain emits. These are the
/// exact spellings carried by fixtures checked into this repo.
#[test]
fn safe_is_a_no_op_on_every_real_toolchain_name() {
    for name in [
        "main",
        "err_fatal.constprop.0",
        "err_warn.constprop.0",
        "main.part.1",
        "gettext_quote.part.1",
        "put_word.isra.1",
        "add_path.constprop.0.isra.0",
        "add_fdes.cold",
        "quotearg_buffer_restyled.cold.3",
        "__x86.get_pc_thunk.bx",
        ".L10",
        ".ctors.65535",
        "0000001b.plt_call.printf@@GLIBC_2.17",
        "std::vector<int>::push_back",
        "anonymous_namespace::Hidden",
        "operator[]",
        "a::b::c::Deep",
        "core::ptr::drop_in_place",
        "_ZN3foo3barEv",
        // A lone `*` or `/` is not a comment delimiter and stays put.
        "a*b",
        "a/b",
        // Valid multi-byte UTF-8 is content, not corruption.
        "caf\u{e9}_init",
    ] {
        assert_eq!(safe(name.as_bytes()), name, "safe must not touch {name:?}");
        assert!(
            matches!(sanitize_symbol_name_bytes(name.as_bytes(), NameChars::Safe), Cow::Borrowed(_)),
            "and must not even allocate for {name:?}"
        );
    }
}

/// The identity half of the defect: `from_utf8_lossy` maps every invalid byte to
/// U+FFFD, so two distinct symbols become one `String` and the export carries a
/// C redefinition. The hex escape is what keeps them apart.
#[test]
fn distinct_invalid_utf8_names_stay_distinct() {
    let a = b"a\x80b";
    let b = b"a\x81b";
    assert_eq!(String::from_utf8_lossy(a), String::from_utf8_lossy(b), "the bug");
    assert_ne!(safe(a), safe(b), "the fix");
    assert_eq!(safe(a), "a_x80b");
    assert_eq!(safe(b), "a_x81b");
    assert_ne!(ident(a), ident(b));
    // A truncated multi-byte sequence escapes every byte it owns, and the whole
    // result is valid UTF-8 by construction.
    assert_eq!(safe(b"a\xe2\x82"), "a_xe2_x82");
    assert_eq!(safe(b"\xf0\x9f"), "_xf0_x9f");
    // A VALID multi-byte sequence next to an invalid byte keeps its own bytes.
    assert_eq!(safe("\u{20ac}".as_bytes()), "\u{20ac}");
    assert_eq!(safe(b"\xe2\x82\xac\xff"), "\u{20ac}_xff");
}

/// The escape is injective on distinct structurally-dirty names, which a plain
/// `_` substitution is not — collapsing `a"b` and `a'b` onto `a_b` would
/// reproduce the redefinition bug with a different trigger.
#[test]
fn the_escape_keeps_distinct_dirty_names_distinct() {
    let names: Vec<&[u8]> = vec![b"a\"b", b"a'b", b"a\\b", b"a\nb", b"a\rb", b"a*/b", b"a//b"];
    let mut out: Vec<String> = names.iter().map(|n| safe(n)).collect();
    out.sort();
    let n = out.len();
    out.dedup();
    assert_eq!(out.len(), n, "sanitizing must not merge two distinct names");
}

/// `off` is verbatim: the bytes reach emitted C exactly as the binary spells
/// them, which is what someone auditing the symbol table itself wants.
#[test]
fn off_is_verbatim() {
    for raw in [b"a*/b".as_slice(), b"a\nb", b"a\x80b", b"main"] {
        assert!(matches!(
            sanitize_symbol_name_bytes(raw, NameChars::Off),
            Cow::Borrowed(_)
        ));
        assert_eq!(&*sanitize_symbol_name_bytes(raw, NameChars::Off), raw);
    }
}

/// `ident` folds everything outside `[A-Za-z0-9_]` per `::` component — the
/// clone suffixes included, which is exactly why it cannot be the default.
#[test]
fn ident_folds_the_clone_suffixes_safe_leaves_alone() {
    assert_eq!(ident(b"err_fatal.constprop.0"), "err_fatal_constprop_0");
    assert_eq!(ident(b"main.part.1"), "main_part_1");
    assert_eq!(ident(b"add_fdes.cold"), "add_fdes_cold");
    assert_eq!(ident(b"0000001b.plt_call.printf@@GLIBC_2.17"), "0000001b_plt_call_printf_GLIBC_2_17");
    // `::` survives as scope structure in both modes.
    assert_eq!(ident(b"a::b::c::Deep"), "a::b::c::Deep");
    assert_eq!(safe(b"a::b::c::Deep"), "a::b::c::Deep");
    // And the hostile shapes reduce to plain identifiers.
    assert_eq!(ident(b"a*/b"), "a_x2a_x2fb");
    assert_eq!(ident(b"a\nb"), "a_x0ab");
}

/// `ident` must never DELETE a name. The identifier reduction drops empty
/// components, so a name made only of non-identifier characters reduces to
/// nothing — and an empty name would put every such symbol under ONE name, the
/// exact collapse this option exists to prevent.
#[test]
fn ident_never_deletes_a_name() {
    for raw in ["***", "...", "@@@", "::", "(-)"] {
        let out = ident(raw.as_bytes());
        assert!(!out.is_empty(), "ident deleted {raw:?}");
        // The fallback escapes the name whole, so `ident` still keeps its
        // promise: what comes out is a C identifier.
        assert!(
            out.starts_with('_') && out.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'),
            "the ident fallback must still be an identifier: {raw:?} -> {out:?}"
        );
    }
    assert_eq!(ident(b"***"), "_x2a_x2a_x2a");
    assert_eq!(ident(b"..."), "_x2e_x2e_x2e");
    assert_ne!(ident(b"***"), ident(b"..."));
    // An empty name in, an empty name out — the RTTI contract, unchanged.
    assert_eq!(ident(b""), "");
    assert_eq!(safe(b""), "");
}

/// The hoisted identifier reduction — these are the assertions that lived on
/// `kuna-analysis`'s `sanitize_class_name` before the hoist, and they are the
/// regression net for it.
#[test]
fn sanitize_ident_chain_matches_the_rtti_class_name_contract() {
    assert_eq!(sanitize_ident_chain("Shape"), "Shape");
    assert_eq!(sanitize_ident_chain("a::b::c::Deep"), "a::b::c::Deep");
    assert_eq!(sanitize_ident_chain("leveldb::Env"), "leveldb::Env");
    assert_eq!(sanitize_ident_chain("Vec<int>"), "Vec_int");
    assert_eq!(sanitize_ident_chain("Vec<double>"), "Vec_double");
    assert_ne!(sanitize_ident_chain("Vec<int>"), sanitize_ident_chain("Vec<double>"));
    assert_eq!(
        sanitize_ident_chain("(anonymous namespace)::Hidden"),
        "anonymous_namespace::Hidden"
    );
    assert_eq!(
        sanitize_ident_chain("std::vector<std::pair<int, int> >"),
        "std::vector_std_pair_int_int"
    );
    assert_eq!(sanitize_ident_chain(""), "");
}

/// The gate defaults to `safe`, parses its three values (plus the on/off habit),
/// and round-trips through the env bridge. One test because the env is
/// process-global.
#[test]
fn env_gate_defaults_to_safe_and_round_trips() {
    std::env::remove_var(SYMBOLNAMECHARS_ENV);
    assert_eq!(symbolnamechars_mode(), NameChars::Safe, "unset ⇒ safe");

    for (token, want) in [
        ("off", NameChars::Off),
        ("OFF", NameChars::Off),
        ("0", NameChars::Off),
        ("false", NameChars::Off),
        ("verbatim", NameChars::Off),
        ("safe", NameChars::Safe),
        ("on", NameChars::Safe),
        ("ident", NameChars::Ident),
        ("IDENT", NameChars::Ident),
    ] {
        assert_eq!(NameChars::parse(token), Some(want), "{token}");
        std::env::set_var(SYMBOLNAMECHARS_ENV, token);
        assert_eq!(symbolnamechars_mode(), want, "{token}");
    }
    // An unrecognized value is not a silent third behavior: it falls back to the
    // shipped default rather than to `off`.
    assert_eq!(NameChars::parse("bogus"), None);
    std::env::set_var(SYMBOLNAMECHARS_ENV, "bogus");
    assert_eq!(symbolnamechars_mode(), NameChars::Safe);

    for mode in [NameChars::Off, NameChars::Safe, NameChars::Ident] {
        set_symbolnamechars_env(mode);
        assert_eq!(symbolnamechars_mode(), mode);
        assert_eq!(NameChars::parse(mode.as_str()), Some(mode));
    }
    std::env::remove_var(SYMBOLNAMECHARS_ENV);
}

/// The `str` entry point agrees with the byte one and borrows when it can.
#[test]
fn str_entry_point_agrees_with_the_byte_one() {
    for mode in [NameChars::Off, NameChars::Safe, NameChars::Ident] {
        for name in ["main", "a*/b", "err_fatal.constprop.0", "a::b"] {
            let via_bytes = sanitize_symbol_name_bytes(name.as_bytes(), mode).into_owned();
            assert_eq!(sanitize_symbol_name(name, mode).as_bytes(), via_bytes.as_slice());
        }
    }
    assert!(matches!(sanitize_symbol_name("main", NameChars::Safe), Cow::Borrowed(_)));
}
