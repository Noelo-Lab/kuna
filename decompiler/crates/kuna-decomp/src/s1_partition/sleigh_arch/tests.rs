//! Unit tests for the `sleigh_arch` frontend port: the `.ldefs` record decode
//! (`CompilerTag`/`LanguageDescription`), `getCompiler` fallback, the language-id
//! normalization family, and `resolveArchitecture`/`buildSpecFile` over a
//! synthetic in-memory `LanguageDatabase`.

use super::*;
use kuna_base::marshal::IdRegistry;
use kuna_sleigh::translate::register_translate_ids;

/// Build a registry covering the base + translate + sleigh_arch ids (the same
/// set the ldefs decode needs).
fn registry() -> IdRegistry {
    let mut r = IdRegistry::with_base_ids();
    register_translate_ids(&mut r);
    register_sleigh_arch_ids(&mut r);
    r
}

/// Decode a `<language_definitions>` blob into a fresh `LanguageDatabase`.
fn decode_ldefs(xml: &str) -> LanguageDatabase {
    let reg = registry();
    let mut db = LanguageDatabase::new();
    db.load_language_definitions(xml.as_bytes(), &reg).expect("decode definitions");
    db
}

// ---------------------------------------------------------------------------
// LanguageDescription / CompilerTag decode
// ---------------------------------------------------------------------------

#[test]
fn ldefs_decode_single_language_all_fields() {
    let xml = r#"<language_definitions>
  <language processor="x86" endian="little" size="64" variant="default"
            version="2.10" slafile="x86-64.sla" processorspec="x86-64.pspec"
            id="x86:LE:64:default" deprecated="false">
    <description>Intel/AMD 64-bit x86</description>
    <compiler name="Visual Studio" spec="x86-64-win.cspec" id="windows"/>
    <compiler name="gcc" spec="x86-64-gcc.cspec" id="gcc"/>
    <compiler name="default" spec="x86-64.cspec" id="default"/>
  </language>
</language_definitions>"#;
    let db = decode_ldefs(xml);
    let descs = db.descriptions();
    assert_eq!(descs.len(), 1);
    let d = &descs[0];
    assert_eq!(d.get_processor(), "x86");
    assert!(!d.is_big_endian());
    assert_eq!(d.get_size(), 64);
    assert_eq!(d.get_variant(), "default");
    assert_eq!(d.get_version(), "2.10");
    assert_eq!(d.get_sla_file(), "x86-64.sla");
    assert_eq!(d.get_processor_spec(), "x86-64.pspec");
    assert_eq!(d.get_id(), "x86:LE:64:default");
    assert_eq!(d.get_description(), "Intel/AMD 64-bit x86");
    assert!(!d.is_deprecated());
    assert_eq!(d.num_compilers(), 3);
    assert_eq!(d.get_compiler_index(0).get_name(), "Visual Studio");
    assert_eq!(d.get_compiler_index(0).get_id(), "windows");
    assert_eq!(d.get_compiler_index(1).get_spec(), "x86-64-gcc.cspec");
}

#[test]
fn ldefs_decode_big_endian_and_deprecated() {
    let xml = r#"<language_definitions>
  <language processor="MIPS" endian="big" size="32" variant="default"
            version="1.0" slafile="mips32be.sla" processorspec="mips.pspec"
            id="MIPS:BE:32:default" deprecated="true">
    <compiler name="default" spec="mips.cspec" id="default"/>
  </language>
</language_definitions>"#;
    let db = decode_ldefs(xml);
    let d = &db.descriptions()[0];
    assert!(d.is_big_endian());
    assert!(d.is_deprecated());
    assert_eq!(d.get_id(), "MIPS:BE:32:default");
}

#[test]
fn ldefs_decode_ignores_unknown_child_elements() {
    // An unknown child (<spanned>) must be opened+skipped, not crash.
    let xml = r#"<language_definitions>
  <language processor="ARM" endian="little" size="32" variant="v8"
            version="1.0" slafile="ARM8_le.sla" processorspec="ARMt.pspec"
            id="ARM:LE:32:v8">
    <spanned foo="bar"/>
    <compiler name="default" spec="ARM.cspec" id="default"/>
  </language>
</language_definitions>"#;
    let db = decode_ldefs(xml);
    let d = &db.descriptions()[0];
    assert_eq!(d.get_id(), "ARM:LE:32:v8");
    assert_eq!(d.num_compilers(), 1);
    // No <description> tag -> empty description.
    assert_eq!(d.get_description(), "");
}

#[test]
fn ldefs_decode_multiple_languages_in_document_order() {
    let xml = r#"<language_definitions>
  <language processor="A" endian="little" size="16" variant="default"
            version="1" slafile="a.sla" processorspec="a.pspec" id="A:LE:16:default">
    <compiler name="default" spec="a.cspec" id="default"/>
  </language>
  <language processor="B" endian="big" size="32" variant="default"
            version="1" slafile="b.sla" processorspec="b.pspec" id="B:BE:32:default">
    <compiler name="default" spec="b.cspec" id="default"/>
  </language>
</language_definitions>"#;
    let db = decode_ldefs(xml);
    let descs = db.descriptions();
    assert_eq!(descs.len(), 2);
    assert_eq!(descs[0].get_id(), "A:LE:16:default");
    assert_eq!(descs[1].get_id(), "B:BE:32:default");
}

// ---------------------------------------------------------------------------
// getCompiler fallback (sleigh_arch.cc:101)
// ---------------------------------------------------------------------------

#[test]
fn get_compiler_exact_then_default_then_first() {
    let xml = r#"<language_definitions>
  <language processor="x" endian="little" size="32" variant="default"
            version="1" slafile="x.sla" processorspec="x.pspec" id="x:LE:32:default">
    <compiler name="GCC" spec="gcc.cspec" id="gcc"/>
    <compiler name="Default" spec="default.cspec" id="default"/>
  </language>
</language_definitions>"#;
    let db = decode_ldefs(xml);
    let d = &db.descriptions()[0];
    // Exact match.
    assert_eq!(d.get_compiler("gcc").get_spec(), "gcc.cspec");
    // Unknown id -> "default" compiler.
    assert_eq!(d.get_compiler("clang").get_spec(), "default.cspec");

    // No "default": unknown id -> the first compiler.
    let xml2 = r#"<language_definitions>
  <language processor="y" endian="little" size="32" variant="default"
            version="1" slafile="y.sla" processorspec="y.pspec" id="y:LE:32:default">
    <compiler name="First" spec="first.cspec" id="first"/>
    <compiler name="Second" spec="second.cspec" id="second"/>
  </language>
</language_definitions>"#;
    let db2 = decode_ldefs(xml2);
    let d2 = &db2.descriptions()[0];
    assert_eq!(d2.get_compiler("nonexistent").get_spec(), "first.cspec");
    assert_eq!(d2.get_compiler("second").get_spec(), "second.cspec");
}

// ---------------------------------------------------------------------------
// normalization family (sleigh_arch.cc:483-565)
// ---------------------------------------------------------------------------

#[test]
fn normalize_processor_endian_size() {
    assert_eq!(normalize_processor("Pentium386"), "x86");
    assert_eq!(normalize_processor("ARM"), "ARM");
    assert_eq!(normalize_endian("bigsomething"), "BE");
    assert_eq!(normalize_endian("littleend"), "LE");
    assert_eq!(normalize_endian("BE"), "BE");
    // normalizeSize erases first "bit" then first '-'.
    assert_eq!(normalize_size("32bit"), "32");
    assert_eq!(normalize_size("32-bit"), "32"); // erase "bit" -> "32-", then erase '-' -> "32"
    assert_eq!(normalize_size("64"), "64");
}

#[test]
fn normalize_architecture_field_count_boundaries() {
    // exactly 4 fields -> compiler "default" appended.
    assert_eq!(normalize_architecture("ARM:LE:32:v8").unwrap(), "ARM:LE:32:v8:default");
    // exactly 5 fields preserved.
    assert_eq!(
        normalize_architecture("ARM:LE:32:v8:default").unwrap(),
        "ARM:LE:32:v8:default"
    );
    // 1, 2, 3 fields -> error (need >= 4).
    assert!(normalize_architecture("x86").is_err());
    assert!(normalize_architecture("x86:LE").is_err());
    assert!(normalize_architecture("x86:LE:64").is_err());
}

// ---------------------------------------------------------------------------
// resolveArchitecture / buildSpecFile over a synthetic database
// ---------------------------------------------------------------------------

/// A small in-memory database with two languages (no spec files on disk; only
/// the index resolution is exercised).
fn synthetic_db() -> LanguageDatabase {
    let xml = r#"<language_definitions>
  <language processor="x86" endian="little" size="64" variant="default"
            version="1" slafile="x86-64.sla" processorspec="x86-64.pspec"
            id="x86:LE:64:default">
    <compiler name="gcc" spec="x86-64-gcc.cspec" id="gcc"/>
    <compiler name="default" spec="x86-64.cspec" id="default"/>
  </language>
  <language processor="ARM" endian="little" size="32" variant="v8"
            version="1" slafile="ARM8_le.sla" processorspec="ARMt.pspec"
            id="ARM:LE:32:v8" deprecated="true">
    <compiler name="default" spec="ARM.cspec" id="default"/>
  </language>
</language_definitions>"#;
    decode_ldefs(xml)
}

#[test]
fn resolve_architecture_from_target_and_strips_prefixes() {
    let db = synthetic_db();

    // target gives a 5-field archid; baseid (minus :compiler) matches the ldefs id.
    let mut sa = SleighArchitecture::new("file", "x86:LE:64:default:gcc");
    sa.resolve_architecture(&db, "ignored-arch-type").unwrap();
    assert_eq!(sa.language_index(), 0);
    assert_eq!(sa.arch_id(), "x86:LE:64:default:gcc");

    // "binary-" prefix stripped before normalization.
    let mut sa2 = SleighArchitecture::new("file", "binary-x86:LE:64:default:gcc");
    sa2.resolve_architecture(&db, "").unwrap();
    assert_eq!(sa2.language_index(), 0);
    assert_eq!(sa2.arch_id(), "x86:LE:64:default:gcc");

    // "default-" prefix stripped.
    let mut sa3 = SleighArchitecture::new("file", "default-x86:LE:64:default:gcc");
    sa3.resolve_architecture(&db, "").unwrap();
    assert_eq!(sa3.language_index(), 0);
}

#[test]
fn resolve_architecture_uses_arch_type_when_target_empty_or_default() {
    let db = synthetic_db();
    // Empty target -> archid comes from arch_type (the loader's getArchType).
    let mut sa = SleighArchitecture::new("file", "");
    sa.resolve_architecture(&db, "x86:LE:64:default:gcc").unwrap();
    assert_eq!(sa.language_index(), 0);

    // target == "default" -> same.
    let mut sa2 = SleighArchitecture::new("file", "default");
    sa2.resolve_architecture(&db, "x86:LE:64:default:gcc").unwrap();
    assert_eq!(sa2.language_index(), 0);
}

#[test]
fn resolve_architecture_deprecated_warns() {
    let db = synthetic_db();
    let mut sa = SleighArchitecture::new("file", "ARM:LE:32:v8:default");
    sa.resolve_architecture(&db, "").unwrap();
    assert_eq!(sa.language_index(), 1);
    // The deprecation warning was written to the error console.
    assert!(sa.error_text().contains("deprecated"), "expected deprecation warning");
}

#[test]
fn resolve_architecture_unknown_language_errors() {
    let db = synthetic_db();
    let mut sa = SleighArchitecture::new("file", "PowerPC:BE:32:default:default");
    let res = sa.resolve_architecture(&db, "");
    assert!(res.is_err());
    let msg = format!("{}", res.unwrap_err());
    assert!(msg.contains("No sleigh specification"), "got: {msg}");
    assert_eq!(sa.language_index(), -1);
}

#[test]
fn set_archid_overrides_resolution_source() {
    // RawBinaryArchitecture's `archid = getTarget()` path: pre-seeding archid
    // makes resolve_architecture skip the target/arch_type branch and use archid.
    let db = synthetic_db();
    let mut sa = SleighArchitecture::new("file", "ARM:LE:32:v8:default");
    sa.set_archid("x86:LE:64:default:gcc");
    sa.resolve_architecture(&db, "").unwrap();
    // archid was already set, so the (target=ARM) source is ignored.
    assert_eq!(sa.language_index(), 0);
    assert_eq!(sa.arch_id(), "x86:LE:64:default:gcc");
}

// ---------------------------------------------------------------------------
// header encode/restore (sleigh_arch.cc:464-477)
// ---------------------------------------------------------------------------

#[test]
fn encode_restore_header_round_trip() {
    let mut sa = SleighArchitecture::new("/path/to/a.out", "x86:LE:64:default:gcc");
    let hdr = sa.encode_header();
    assert_eq!(hdr.len(), 2);
    assert_eq!(hdr[0].0.get_name(), "name");
    assert_eq!(hdr[0].1, "/path/to/a.out");
    assert_eq!(hdr[1].0.get_name(), "target");
    assert_eq!(hdr[1].1, "x86:LE:64:default:gcc");

    sa.restore_xml_header("/other.bin", "ARM:LE:32:v8");
    assert_eq!(sa.get_filename(), "/other.bin");
    assert_eq!(sa.get_target(), "ARM:LE:32:v8");
}

// ---------------------------------------------------------------------------
// is_translate_reused (the not-ported reuse cache always reports false)
// ---------------------------------------------------------------------------

#[test]
fn translate_reuse_always_false() {
    let sa = SleighArchitecture::new("f", "x86:LE:64:default");
    assert!(!sa.is_translate_reused());
}
