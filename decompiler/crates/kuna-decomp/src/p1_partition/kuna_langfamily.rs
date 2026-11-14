//! (kuna) A language's processor, address size and instruction byte order, as
//! its `.ldefs` entry states them.
//!
//! Ghidra's FID decides whether a fingerprint database built for one language
//! applies to a program in another by exactly these three facts
//! (`FidFile.ProcessorSizeComparator`): the variant and the compiler spec do not
//! take part. They come from the `.ldefs` attributes, not from the language id's
//! fields, which differ for some languages (`MIPS:BE:64:64-32addr` has a 32-bit
//! address size; `ARM:LEBE:32:v8LEInstruction` has little-endian instructions;
//! `z80` and `z8401x` are both processor `Z80`).

use std::sync::Arc;

use crate::sleigh_arch::LanguageDatabase;

/// What FID compares between two languages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LanguageFamily {
    pub processor: String,
    pub size: i32,
    pub instruction_big_endian: bool,
}

/// Every language in `db`, by id, in `.ldefs` order.
pub fn families(db: &LanguageDatabase) -> Arc<[(String, LanguageFamily)]> {
    db.descriptions()
        .iter()
        .map(|d| {
            (
                d.get_id().to_string(),
                LanguageFamily {
                    processor: d.get_processor().to_string(),
                    size: d.get_size(),
                    instruction_big_endian: d.is_instruction_big_endian(),
                },
            )
        })
        .collect()
}

/// The language id part of an architecture id (`x86:LE:64:default:gcc` ->
/// `x86:LE:64:default`): its first four `:`-separated fields.
pub fn language_id(archid: &str) -> &str {
    match archid.match_indices(':').nth(3) {
        Some((at, _)) => &archid[..at],
        None => archid,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::marshal::IdRegistry;

    fn vendored(processor: &str, ldefs: &str) -> Arc<[(String, LanguageFamily)]> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../specs/Ghidra/Processors")
            .join(processor)
            .join("data/languages")
            .join(ldefs);
        let mut registry = IdRegistry::with_base_ids();
        kuna_sleigh::translate::register_translate_ids(&mut registry);
        crate::sleigh_arch::register_sleigh_arch_ids(&mut registry);
        let mut db = LanguageDatabase::new();
        db.load_language_definitions(&std::fs::read(path).unwrap(), &registry).unwrap();
        families(&db)
    }

    fn family(all: &[(String, LanguageFamily)], id: &str) -> (String, i32, bool) {
        let (_, f) = all.iter().find(|(i, _)| i == id).unwrap();
        (f.processor.clone(), f.size, f.instruction_big_endian)
    }

    #[test]
    fn vendored_ldefs_state_instruction_order_and_size_apart_from_the_id() {
        let arm = vendored("ARM", "ARM.ldefs");
        assert_eq!(family(&arm, "ARM:LE:32:v8"), ("ARM".into(), 32, false));
        assert_eq!(family(&arm, "ARM:LEBE:32:v8LEInstruction"), ("ARM".into(), 32, false));
        assert_eq!(family(&arm, "ARM:BE:32:v8"), ("ARM".into(), 32, true));
        let aarch64 = vendored("AARCH64", "AARCH64.ldefs");
        assert_eq!(family(&aarch64, "AARCH64:BE:64:v8A"), ("AARCH64".into(), 64, false));
        let nds32 = vendored("NDS32", "nds32.ldefs");
        assert_eq!(family(&nds32, "NDS32:LE:32:default"), ("NDS32".into(), 32, true));
        let mips = vendored("MIPS", "mips.ldefs");
        assert_eq!(family(&mips, "MIPS:BE:64:64-32addr"), ("MIPS".into(), 32, true));
        assert_eq!(family(&mips, "MIPS:BE:64:default"), ("MIPS".into(), 64, true));
        let x86 = vendored("x86", "x86.ldefs");
        assert_eq!(family(&x86, "x86:LE:64:default"), ("x86".into(), 64, false));
        assert_eq!(family(&x86, "x86:LE:32:default"), ("x86".into(), 32, false));
    }

    #[test]
    fn language_id_drops_the_compiler_spec() {
        assert_eq!(language_id("x86:LE:64:default:gcc"), "x86:LE:64:default");
        assert_eq!(language_id("x86:LE:32:System Management Mode:default"), "x86:LE:32:System Management Mode");
        assert_eq!(language_id("ARM:LE:32:v8"), "ARM:LE:32:v8");
        assert_eq!(language_id("raw"), "raw");
    }
}
