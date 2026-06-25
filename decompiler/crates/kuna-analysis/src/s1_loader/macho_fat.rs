//! Mach-O fat / universal (`0xcafebabe` `FAT_MAGIC`) slice selection — the
//! single, canonical point where a multi-arch universal binary is peeled to one
//! arch slice before the rest of the loader runs (design §3.4 / §8 PR-8).
//!
//! ## Why one point
//!
//! `object::File::parse` is a *thin*, single-arch view — it cannot parse a fat
//! header (`File::parse` has no `MachOFat*` arm and returns "Unsupported file
//! format"). So a fat Mach-O must be reduced to **one** slice's bytes *before*
//! `ObjectLoadImage::from_bytes` runs; everything downstream (segment/section
//! mapping, the `__stubs` import walk, entry discovery) is then single-arch
//! exactly as for a thin Mach-O.
//!
//! The peel happens once, at the engine dispatch (`bootstrap_from_object`):
//! [`select_fat_slice`] returns the chosen slice's sub-slice of the file bytes,
//! and the engine substitutes it for the fat-wrapped bytes so the loader, the
//! analysis passes, and the deferred-Listing stash all see the *same* one slice.
//! The import walk ([`crate::s1_loader::macho_stubs`]) routes its own fat input
//! through this same selector, so the loadimage and the import naming can never
//! drift onto different slices.
//!
//! ## Which slice
//!
//! With no override the preference is **x86-64, then arm64, then the first arch**
//! present (design §3.4: a deterministic, host-relevant default — x86-64 is the
//! arch this container's `.sla`s and fixtures exercise). An explicit override
//! ([`SlicePref`], the `--slice`/`--target` token threaded from the CLI) names
//! an arch and wins when that arch is present.
//!
//! Pure & total: a non-fat input, an unparsable header, or an override naming an
//! absent arch yields the documented fallback (or `None`) — never a panic, never
//! an error.

use object::read::macho::{FatArch, MachOFatFile32, MachOFatFile64};
use object::{Architecture, FileKind};

/// A caller's slice preference for a fat binary — the `--slice` / `--target`
/// override token, parsed to a target [`Architecture`]. `None` selects the
/// deterministic default (x86-64 → arm64 → first).
#[derive(Copy, Clone, Debug, Default)]
pub struct SlicePref(pub Option<Architecture>);

impl SlicePref {
    /// Parse a user-supplied slice token (`--slice <name>` / a `--target` whose
    /// leading stem names an arch) into a [`SlicePref`]. Recognizes the common
    /// `lipo`/`file` arch spellings (`x86_64`/`x86-64`/`amd64`, `arm64`/`aarch64`,
    /// `arm64e`, `i386`/`x86`, `arm`, `ppc`/`powerpc`, `ppc64`). An unrecognized
    /// token yields the default preference (`None`) — wrong/typo'd slice names
    /// fall back rather than erroring.
    pub fn parse(token: &str) -> SlicePref {
        let t = token.trim().to_ascii_lowercase();
        // A `--target` SLEIGH id (`x86:LE:64:default:gcc`, `AARCH64:LE:64:...`)
        // leads with the arch stem; take the first `:`-delimited field so the
        // existing `--target` override also steers slice selection. The `:`-split
        // fields disambiguate the size for the size-ambiguous stems (`x86` /
        // `arm`, which a SLEIGH id writes the same for 32- and 64-bit, with the
        // third field giving `32`/`64`).
        let stem = t.split(':').next().unwrap_or(&t);
        // The SLEIGH size field is the third (`stem:endian:size:...`); only used
        // to disambiguate the size-overloaded `x86` / `ppc` stems.
        let size64 = t.split(':').nth(2) == Some("64");
        let arch = match stem {
            "x86_64" | "x86-64" | "amd64" | "x64" => Some(Architecture::X86_64),
            // arm64e is still an AArch64 slice on disk; the AppleSilicon spec
            // selection is a separate, gated concern (see `macho` / `language_id_for`).
            "arm64" | "aarch64" | "arm64e" => Some(Architecture::Aarch64),
            "i386" | "i486" | "i586" | "i686" => Some(Architecture::I386),
            // Size-ambiguous SLEIGH stems: the `:64:` field promotes to the 64-bit
            // arch (`x86:LE:64:...` → x86-64; `ARM`/`AARCH64` are distinct stems so
            // a bare `arm` is 32-bit ARM).
            "x86" => Some(if size64 {
                Architecture::X86_64
            } else {
                Architecture::I386
            }),
            "arm" | "armv7" | "thumb" => Some(Architecture::Arm),
            "ppc" | "powerpc" => Some(if size64 {
                Architecture::PowerPc64
            } else {
                Architecture::PowerPc
            }),
            "ppc64" | "powerpc64" => Some(Architecture::PowerPc64),
            "mips" => Some(Architecture::Mips),
            _ => None,
        };
        SlicePref(arch)
    }
}

/// Select one slice of a fat / universal Mach-O, returning the sub-slice of
/// `bytes` covering that slice's thin Mach-O (which `object::File::parse` can
/// then load directly). Honors `pref` (the `--slice`/`--target` override) when
/// the named arch is present, else the deterministic default. Returns `None` for
/// a non-fat input, an unparsable fat header, or a fat with no usable slice.
pub fn select_fat_slice(bytes: &[u8], pref: SlicePref) -> Option<&[u8]> {
    match FileKind::parse(bytes) {
        Ok(FileKind::MachOFat32) => {
            let fat = MachOFatFile32::parse(bytes).ok()?;
            select_fat_arch(fat.arches(), pref).and_then(|a| a.data(bytes).ok())
        }
        Ok(FileKind::MachOFat64) => {
            let fat = MachOFatFile64::parse(bytes).ok()?;
            select_fat_arch(fat.arches(), pref).and_then(|a| a.data(bytes).ok())
        }
        _ => None,
    }
}

/// Whether `bytes` begins with a fat / universal Mach-O magic (`FAT_MAGIC` /
/// `FAT_MAGIC_64`, either byte order). The cheap pre-check the engine dispatch
/// uses to decide whether to peel a slice.
pub fn is_fat(bytes: &[u8]) -> bool {
    matches!(
        FileKind::parse(bytes),
        Ok(FileKind::MachOFat32 | FileKind::MachOFat64)
    )
}

/// The arch-selection policy over a fat header's arches: the override arch if it
/// is present, else x86-64, then arm64, then the first arch (design §3.4). Shared
/// by the 32- and 64-bit fat readers via the [`FatArch`] trait so the policy is
/// stated once.
fn select_fat_arch<Fat: FatArch>(arches: &[Fat], pref: SlicePref) -> Option<&Fat> {
    if let SlicePref(Some(want)) = pref {
        if let Some(a) = arches.iter().find(|a| a.architecture() == want) {
            return Some(a);
        }
        // Override names an absent arch: fall through to the default preference
        // (a present slice still decompiles — a missing-slice override is a
        // softer failure than no decompile).
    }
    arches
        .iter()
        .find(|a| a.architecture() == Architecture::X86_64)
        .or_else(|| {
            arches
                .iter()
                .find(|a| a.architecture() == Architecture::Aarch64)
        })
        .or_else(|| arches.first())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_pref_parses_arch_names_and_target_ids() {
        assert_eq!(SlicePref::parse("x86_64").0, Some(Architecture::X86_64));
        assert_eq!(SlicePref::parse("amd64").0, Some(Architecture::X86_64));
        assert_eq!(SlicePref::parse("arm64").0, Some(Architecture::Aarch64));
        assert_eq!(SlicePref::parse("aarch64").0, Some(Architecture::Aarch64));
        assert_eq!(SlicePref::parse("arm64e").0, Some(Architecture::Aarch64));
        assert_eq!(SlicePref::parse("i386").0, Some(Architecture::I386));
        // A `--target` SLEIGH id steers selection by its leading arch stem.
        assert_eq!(
            SlicePref::parse("x86:LE:64:default:gcc").0,
            Some(Architecture::X86_64)
        );
        // Case-insensitive.
        assert_eq!(SlicePref::parse("ARM64").0, Some(Architecture::Aarch64));
        // An unknown token is the default preference (no panic, no error).
        assert_eq!(SlicePref::parse("riscv-banana").0, None);
        assert_eq!(SlicePref::default().0, None);
    }

    #[test]
    fn is_fat_rejects_non_fat() {
        assert!(!is_fat(&[0x7f, b'E', b'L', b'F']));
        assert!(!is_fat(&[0xcf, 0xfa, 0xed, 0xfe])); // thin Mach-O LE64
        assert!(!is_fat(&[]));
        // select_fat_slice on a non-fat input is None (no peel).
        assert!(select_fat_slice(&[0x7f, b'E', b'L', b'F'], SlicePref::default()).is_none());
    }

    // The selection *policy* (override-wins, then x86-64 → arm64 → first) is
    // exercised end-to-end on the real 2-slice `macho_fat` fixture by the console
    // gate (`verify_macho_fat`), where actual `FatArch` records are present; here
    // we cover the pure token-parse + non-fat-reject logic that needs no binary.
}
