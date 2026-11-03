//! (kuna) The Octeon language IDs in one place: the loader builds them from ELF
//! evidence, and call recovery reads the compiler model back out of them.
//!
//! An Octeon language is `MIPS:BE:64:<variant>[-32addr]:<compiler>`. Every
//! variant has the soft-float `default` compiler and its ABI alias (`n64`, or
//! `n32` for a 32-bit-address variant), both naming the same compiler spec;
//! Octeon III adds `hard`.

/// The Octeon core variants, oldest first.
pub const VARIANTS: [&str; 4] = ["octeon", "octeonplus", "octeon2", "octeon3"];

/// The language ID of an Octeon `variant` with N32 or N64 addressing and the
/// hard-float or soft-float (`default`) compiler model.
pub fn language_id(variant: &str, n32: bool, hard: bool) -> String {
    format!(
        "MIPS:BE:64:{variant}{}:{}",
        if n32 { "-32addr" } else { "" },
        if hard { "hard" } else { "default" }
    )
}

/// The floating-point convention an Octeon language ID's compiler model
/// selects: `Some(true)` for `hard`; `Some(false)` for the soft-float `default`
/// and its ABI alias; `None` for any other language or compiler ID.
pub fn hard_float(id: &str) -> Option<bool> {
    let mut parts = id.strip_prefix("MIPS:BE:64:")?.split(':');
    let variant = parts.next()?;
    let (core, n32) = match variant.strip_suffix("-32addr") {
        Some(core) => (core, true),
        None => (variant, false),
    };
    if !VARIANTS.contains(&core) {
        return None;
    }
    match (parts.next()?, n32) {
        ("hard", _) => Some(true),
        ("default", _) | ("n32", true) | ("n64", false) => Some(false),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn built_ids_read_back_and_every_soft_float_alias_is_the_soft_model() {
        for variant in VARIANTS {
            for (n32, alias, other) in [(false, "n64", "n32"), (true, "n32", "n64")] {
                for hard in [false, true] {
                    assert_eq!(hard_float(&language_id(variant, n32, hard)), Some(hard));
                }
                let suffix = if n32 { "-32addr" } else { "" };
                let id = |model: &str| format!("MIPS:BE:64:{variant}{suffix}:{model}");
                assert_eq!(hard_float(&id(alias)), Some(false));
                assert_eq!(hard_float(&id(other)), None);
                assert_eq!(hard_float(&id("unknown")), None);
            }
        }
        for id in ["MIPS:BE:64:64-32addr:n32", "MIPS:BE:64:default:default", "MIPS:LE:64:octeon3:default",
            "x86:LE:64:default:gcc"] {
            assert_eq!(hard_float(id), None);
        }
    }
}
