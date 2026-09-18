//! (kuna `formatstring`) The three-valued gate for printf/scanf varargs typing.
//!
//! `off` types nothing. `static` runs the LOAD-TIME resolver — the format
//! constant is read out of the image at the call site and a per-call-site
//! prototype override is parked before the function is ever decompiled, so the
//! typing costs one decompile. `full` adds the original decompile → read the
//! constant off the lifted `CALL` → re-decompile loop for the sites the static
//! resolver could not answer, which costs a second decompile of every caller it
//! changes.
//!
//! `on` is accepted as a spelling of `full`: that is what the option meant when
//! it was two-valued, and a run that asked for `on` asked for the loop.
//!
//! Both halves close the call's prototype: the format's conversions become
//! ordinary parameters, and the compiler spec assigns them storage as it would
//! assign a named argument's. That is only true where the target passes a
//! variadic argument the way it passes a named one, which [`VarargAbi`] records
//! per target and every override is checked against.

use kuna_base::error::{KunaError, KunaResult};

use crate::architecture::Architecture;
use crate::dtype::{type_metatype, Datatype};
use crate::fspec::PrototypePieces;

/// One call-site override the load-time resolver parked, with the answer it was
/// built from, so the decompile step can check the drive agrees with it.
#[derive(Clone, Debug)]
pub struct ParkedFormatSite {
    /// VMA of the call instruction the override applies at.
    pub callpoint: u64,
    /// The closed prototype: the callee's fixed parameters, then one per
    /// conversion.
    pub pieces: PrototypePieces,
    /// Which parameter is the format.
    pub format_slot: usize,
    /// The format string's VMA, as the resolver read it.
    pub format_vma: u64,
}

/// How much of the format-string varargs typing a run performs.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FormatStringMode {
    /// Type nothing; no resolver runs and no override is installed.
    Off,
    /// The load-time resolver alone (one decompile per function).
    #[default]
    Static,
    /// The load-time resolver plus the decompile→override→re-decompile loop.
    Full,
}

impl FormatStringMode {
    /// Is any format-string typing performed at all?
    pub fn enabled(self) -> bool {
        !matches!(self, FormatStringMode::Off)
    }

    /// Does the LOAD-TIME resolver run?
    pub fn statik(self) -> bool {
        matches!(self, FormatStringMode::Static | FormatStringMode::Full)
    }

    /// Does the decompile→override→re-decompile loop run?
    pub fn loop_pass(self) -> bool {
        matches!(self, FormatStringMode::Full)
    }

    /// The `option formatstring <p1>` token for this mode.
    pub fn as_str(self) -> &'static str {
        match self {
            FormatStringMode::Off => "off",
            FormatStringMode::Static => "static",
            FormatStringMode::Full => "full",
        }
    }
}

/// How far a closed prototype may stand in for a call's variadic arguments on a
/// target: which of them travel exactly as a named argument of the same type
/// would.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VarargAbi {
    /// Every one does (x86, AArch64 AAPCS64 outside Apple and Windows).
    Named,
    /// Integer and pointer arguments no wider than a pointer do; a floating one
    /// does not. ARM32 hard-float and RISC-V pass a floating vararg in core
    /// registers and a named one in `d0`/`fa0`, Windows on AArch64 passes it in
    /// `x` registers, and RISC-V aligns a double-width vararg to an even register
    /// pair where a named one is not.
    Integer,
    /// None is known to. Apple AArch64 passes every variadic argument on the
    /// stack, and a target not listed here has not been checked.
    Nothing,
}

/// The object-file family an image came in, as far as its variadic calling
/// convention depends on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageFamily {
    /// Mach-O: an Apple platform.
    Apple,
    /// PE/COFF: Windows.
    Windows,
    /// Anything else (ELF, raw).
    Other,
}

impl VarargAbi {
    /// May an override declare a variadic argument of type `ty`? `ptr_size` is
    /// the target's pointer width, the widest integer [`VarargAbi::Integer`]
    /// admits.
    pub fn admits(self, ty: &Datatype, ptr_size: i32) -> bool {
        match self {
            VarargAbi::Named => true,
            VarargAbi::Integer => {
                ty.get_metatype() != type_metatype::TYPE_FLOAT && ty.get_size() <= ptr_size
            }
            VarargAbi::Nothing => false,
        }
    }
}

/// The variadic convention of the target `archid` names, in an image of
/// `family` (`None` when the image is not known). An Apple SLEIGH variant or a
/// `windows` compiler spec in the id settles the family on its own.
pub fn vararg_abi(archid: &str, family: Option<ImageFamily>) -> VarargAbi {
    let family = if archid.contains("AppleSilicon") {
        Some(ImageFamily::Apple)
    } else if archid.ends_with(":windows") {
        Some(ImageFamily::Windows)
    } else {
        family
    };
    match archid.split(':').next().unwrap_or("") {
        "x86" => VarargAbi::Named,
        "AARCH64" => match family {
            Some(ImageFamily::Other) => VarargAbi::Named,
            Some(ImageFamily::Windows) => VarargAbi::Integer,
            Some(ImageFamily::Apple) | None => VarargAbi::Nothing,
        },
        "ARM" | "RISCV" | "MIPS" | "PowerPC" => VarargAbi::Integer,
        _ => VarargAbi::Nothing,
    }
}

/// The variadic convention of the loaded target: the loader's answer when it
/// recorded one, else what the language id alone says.
pub fn target_vararg_abi(arch: &Architecture) -> VarargAbi {
    arch.format_vararg_abi.unwrap_or_else(|| vararg_abi(&arch.archid, None))
}

/// Parse the `option formatstring off|static|full` argument, producing the
/// resolved mode plus the confirmation message (the established value-option
/// idiom — cf. `parse_cppsig_mode`).
///
/// `on` is an accepted alias for `full`, the meaning it carried while the option
/// was `on|off`.
pub fn parse_formatstring_mode(p1: &str) -> KunaResult<(FormatStringMode, String)> {
    let mode = match p1 {
        "off" => FormatStringMode::Off,
        "static" => FormatStringMode::Static,
        "full" | "on" => FormatStringMode::Full,
        _ => return Err(KunaError::parse("Must specify off, static or full (`on` = full)")),
    };
    Ok((mode, format!("Format-string varargs typing set to {}", mode.as_str())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_three_values_and_the_on_alias() {
        assert_eq!(parse_formatstring_mode("off").unwrap().0, FormatStringMode::Off);
        assert_eq!(parse_formatstring_mode("static").unwrap().0, FormatStringMode::Static);
        assert_eq!(parse_formatstring_mode("full").unwrap().0, FormatStringMode::Full);
        // The two-valued spelling keeps working and keeps meaning the loop.
        assert_eq!(parse_formatstring_mode("on").unwrap().0, FormatStringMode::Full);
        assert!(parse_formatstring_mode("").is_err());
        assert!(parse_formatstring_mode("loop").is_err());
    }

    #[test]
    fn varargs_are_named_arguments_only_where_the_abi_says_so() {
        use ImageFamily::*;
        assert_eq!(vararg_abi("x86:LE:64:default:gcc", Some(Other)), VarargAbi::Named);
        assert_eq!(vararg_abi("x86:LE:64:default:windows", Some(Windows)), VarargAbi::Named);
        assert_eq!(vararg_abi("x86:LE:32:default:gcc", None), VarargAbi::Named);
        assert_eq!(vararg_abi("AARCH64:LE:64:v8A:default", Some(Other)), VarargAbi::Named);
        // Apple passes every vararg on the stack; the container or the Apple
        // SLEIGH variant says so, and an unknown container is not assumed Linux.
        assert_eq!(vararg_abi("AARCH64:LE:64:v8A:default", Some(Apple)), VarargAbi::Nothing);
        assert_eq!(vararg_abi("AARCH64:LE:64:AppleSilicon:default", None), VarargAbi::Nothing);
        assert_eq!(vararg_abi("AARCH64:LE:64:v8A:default", None), VarargAbi::Nothing);
        assert_eq!(vararg_abi("AARCH64:LE:64:v8A:windows", None), VarargAbi::Integer);
        assert_eq!(vararg_abi("AARCH64:LE:64:v8A:default", Some(Windows)), VarargAbi::Integer);
        assert_eq!(vararg_abi("ARM:LE:32:v8:default", Some(Other)), VarargAbi::Integer);
        assert_eq!(vararg_abi("RISCV:LE:64:RV64GC:gcc", Some(Other)), VarargAbi::Integer);
        assert_eq!(vararg_abi("MIPS:BE:32:default:default", Some(Other)), VarargAbi::Integer);
        assert_eq!(vararg_abi("AVR8:LE:16:atmega256:gcc", Some(Other)), VarargAbi::Nothing);
    }

    #[test]
    fn the_integer_convention_admits_no_float_and_nothing_wider_than_a_pointer() {
        use crate::dtype::TypeFactory;
        let types = crate::dtype::TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(8);
        types.setup_sizes(Some(4), 4, 4);
        types.set_core_type("char", 1, type_metatype::TYPE_INT, true).expect("char core type");
        types.cache_core_types().expect("cache core types");
        let int4 = types.get_base(4, type_metatype::TYPE_INT).unwrap();
        let int8 = types.get_base(8, type_metatype::TYPE_INT).unwrap();
        let dbl = types.get_base(8, type_metatype::TYPE_FLOAT).unwrap();
        assert!(VarargAbi::Integer.admits(&int4, 4));
        assert!(!VarargAbi::Integer.admits(&int8, 4));
        assert!(!VarargAbi::Integer.admits(&dbl, 8));
        assert!(VarargAbi::Named.admits(&dbl, 8));
        assert!(!VarargAbi::Nothing.admits(&int4, 4));
    }

    #[test]
    fn the_tiers_compose() {
        assert!(!FormatStringMode::Off.enabled());
        assert!(!FormatStringMode::Off.statik());
        assert!(!FormatStringMode::Off.loop_pass());
        assert!(FormatStringMode::Static.enabled());
        assert!(FormatStringMode::Static.statik());
        assert!(!FormatStringMode::Static.loop_pass());
        assert!(FormatStringMode::Full.statik());
        assert!(FormatStringMode::Full.loop_pass());
        assert_eq!(FormatStringMode::Full.as_str(), "full");
    }
}
