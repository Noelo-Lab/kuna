//! Recognize validated LLVM suffixes on legacy Rust names.

pub(super) fn stem(name: &str) -> Option<&str> {
    let (stem, suffix) = name.split_once(".llvm.")?;
    let hash = stem.rsplit_once("17h")?.1.strip_suffix('E')?;
    if suffix.is_empty()
        || !suffix
            .bytes()
            .all(|b| matches!(b, b'A'..=b'F' | b'0'..=b'9' | b'@'))
        || !super::is_legacy_rust_hash_digits(hash)
        || rustc_demangle::try_demangle(name).is_err()
    {
        return None;
    }
    Some(stem)
}

#[cfg(test)]
mod tests {
    const CLOSURE: &str =
        "_ZN3std2rt19lang_start_internal28_$u7b$$u7b$closure$u7d$$u7d$17hf421b6f6b8a4a2f4E";

    #[test]
    fn llvm_suffix_preserves_the_validated_rust_stem() {
        for suffix in ["9325873435131735662", "9D1C9369"] {
            let name = format!("{CLOSURE}.llvm.{suffix}");
            assert_eq!(super::stem(&name), Some(CLOSURE));
            assert!(crate::sourcelang::is_rust_mangled(&name));
            assert_eq!(
                crate::demangle::demangle_raw(&name).as_deref(),
                Some("std::rt::lang_start_internal::{{closure}}")
            );
            assert_eq!(
                crate::demangle::demangle_name(&name).as_deref(),
                Some("std::rt::lang_start_internal::{{closure}}")
            );
        }
        let versioned = format!("{CLOSURE}.llvm.9D1C9369@@16");
        assert_eq!(super::stem(&versioned), Some(CLOSURE));
        assert_eq!(crate::demangle::demangle_raw(&versioned), None);
        assert_eq!(crate::demangle::demangle_name(&versioned), None);
    }

    #[test]
    fn llvm_suffix_does_not_broaden_lookalike_recognition() {
        for name in [
            "_ZN4demo3fooE.llvm.9325873435131735662",
            "_ZN4demo17hABCDEF0123456789E.llvm.9325873435131735662",
            "_ZN999bad17h0123456789abcdefE.llvm.9325873435131735662",
            "_ZN4demo17h0123456789abcdezE.llvm.9325873435131735662",
            "_RBOJyX.llvm.9325873435131735662",
        ] {
            assert_eq!(super::stem(name), None, "{name}");
            assert!(!crate::sourcelang::is_rust_mangled(name), "{name}");
        }
        for suffix in ["", "abc", "G1", "123 suffix", "123.llvm.456"] {
            let name = format!("{CLOSURE}.llvm.{suffix}");
            assert_eq!(super::stem(&name), None, "{name}");
            assert!(!crate::sourcelang::is_rust_mangled(&name), "{name}");
        }
        assert!(crate::sourcelang::is_rust_mangled(CLOSURE));
        assert!(!crate::sourcelang::is_rust_mangled(
            "_ZN4demo17hABCDEF0123456789E"
        ));
    }
}
