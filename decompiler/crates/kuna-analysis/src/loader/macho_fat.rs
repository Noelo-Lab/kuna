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
//! The import walk ([`crate::loader::macho_stubs`]) routes its own fat input
//! through this same selector, so the loadimage and the import naming can never
//! drift onto different slices.
//!
//! The dispatch is not the only reader of an image, though: every surface that
//! re-parses the file for itself (`strings`, `xrefs`, the `functions --summary`
//! call graph, the project export) needs the same peel, or it dies on the fat
//! header while the decompile of the same file succeeds. Those surfaces read
//! through [`crate::loader::elf_shdr::read_image`], which applies
//! [`peel_fat_image`] with the preference [`slice_pref`] resolves — so the peel
//! policy, including the `--slice` override, is stated here once and obeyed
//! everywhere.
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

/// The environment variable carrying a `--slice <arch>` fat-slice override.
/// Read live (per image read) so a test — and the in-process CLI surfaces — can
/// set it without threading a token; empty/unset selects the default slice.
pub const SLICE_ENV: &str = "KUNA_MACHO_SLICE";

/// The slice preference a run asks for, in precedence order: an explicit
/// `--slice` token, else [`SLICE_ENV`], else the `--target` token's leading arch
/// stem, else the deterministic default. An explicit slice that names nothing
/// recognizable stays the default rather than falling through to `--target`, so
/// a typo'd `--slice` cannot be silently answered by the language override.
pub fn slice_pref(slice: Option<&str>, target: Option<&str>) -> SlicePref {
    let explicit = slice
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(str::to_string)
        .or_else(|| std::env::var(SLICE_ENV).ok().filter(|t| !t.trim().is_empty()));
    if let Some(token) = explicit {
        return SlicePref::parse(&token);
    }
    match target.map(str::trim).filter(|token| !token.is_empty()) {
        Some(token) => SlicePref::parse(token),
        None => SlicePref::default(),
    }
}

/// Reduce a fat / universal Mach-O to one arch slice's bytes — the peel every
/// surface that parses an image itself must apply, because `object::File::parse`
/// has no fat arm and answers "Unsupported file format" for the whole file.
///
/// A thin (non-fat) input is returned **verbatim** (an exact, zero-copy move),
/// so the ELF / thin-Mach-O / PE / COFF paths are byte-identical. A fat header
/// that cannot be peeled (unparsable, or no usable slice) is likewise left
/// untouched, so the downstream parse produces the existing error rather than
/// this silently mis-loading.
pub fn peel_fat_image(bytes: Vec<u8>, pref: SlicePref) -> Vec<u8> {
    if !is_fat(&bytes) {
        return bytes;
    }
    match select_fat_slice(&bytes, pref) {
        Some(slice) => slice.to_vec(),
        None => bytes,
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

    /// The preference order the CLI and the dispatch share. The environment
    /// override is asserted through an explicit token rather than by setting the
    /// var, because `std::env` is process-global and cargo runs these in
    /// parallel.
    #[test]
    fn slice_pref_prefers_an_explicit_slice_over_a_target_stem() {
        assert_eq!(
            slice_pref(Some("arm64"), Some("x86:LE:64:default:gcc")).0,
            Some(Architecture::Aarch64),
            "--slice wins over the --target stem"
        );
        // The `--target`-only fallbacks read the environment first, so they are
        // only meaningful with no ambient override set.
        if std::env::var_os(SLICE_ENV).is_none() {
            assert_eq!(
                slice_pref(None, Some("x86:LE:64:default:gcc")).0,
                Some(Architecture::X86_64),
                "--target alone still steers the slice"
            );
            // A blank token is not an override.
            assert_eq!(slice_pref(Some("  "), Some("arm64")).0, Some(Architecture::Aarch64));
            assert_eq!(slice_pref(None, None).0, None);
        }
        // An unrecognized --slice stays the DEFAULT rather than falling through
        // to --target: a typo must not be answered by the language override.
        assert_eq!(slice_pref(Some("riscv-banana"), Some("arm64")).0, None);
    }

    /// A non-fat image is handed back byte for byte -- the ELF / thin-Mach-O /
    /// PE / COFF paths must be untouched by a peel that does not apply.
    #[test]
    fn peel_returns_a_non_fat_image_verbatim() {
        let elf = vec![0x7f, b'E', b'L', b'F', 2, 1, 1, 0];
        assert_eq!(peel_fat_image(elf.clone(), SlicePref::default()), elf);
        // A fat magic with nothing behind it cannot be peeled; it is left for the
        // downstream parse to reject rather than truncated here.
        let stub = vec![0xca, 0xfe, 0xba, 0xbe, 0, 0, 0, 2];
        assert_eq!(peel_fat_image(stub.clone(), SlicePref::default()), stub);
    }

    // The selection *policy* (override-wins, then x86-64 → arm64 → first) is
    // exercised end-to-end on the real 2-slice `macho_fat` fixture by the console
    // gate (`verify_macho_fat`), where actual `FatArch` records are present; here
    // we cover the pure token-parse + non-fat-reject logic that needs no binary.
}
