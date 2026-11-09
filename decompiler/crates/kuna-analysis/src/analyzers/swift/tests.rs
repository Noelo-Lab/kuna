//! (kuna `swiftmetadata`) The name derivations the pass makes from imported
//! descriptor names, and the bounds that keep a crafted image's records
//! from multiplying; the end-to-end naming is `verify_swiftmetadata`.

use super::*;

#[test]
fn member_of_an_imported_method_descriptor() {
    assert_eq!(
        member_after_protocol("Swift.Hashable.hash(into:)").as_deref(),
        Some("hash(into:)")
    );
    assert_eq!(
        member_after_protocol("Swift.Hashable.hashValue.getter").as_deref(),
        Some("hashValue.getter")
    );
    assert_eq!(
        member_after_protocol("Swift.Equatable.==(_:_:)").as_deref(),
        Some("==(_:_:)")
    );
    assert_eq!(
        member_after_protocol("__C.UIHostingController.init(coder:rootView:)").as_deref(),
        Some("init(coder:rootView:)")
    );
    // A dot inside a parameter list does not split the member.
    assert_eq!(member_after_protocol("M.P.f(_:)").as_deref(), Some("f(_:)"));
    assert_eq!(
        member_after_protocol("M.P.sub(x:)").as_deref(),
        Some("sub(x:)")
    );
    assert_eq!(member_after_protocol("nodots"), None);
}

#[test]
fn import_names_strip_their_descriptor_tag() {
    let mut imports = HashMap::new();
    imports.insert(0x1000u64, "_$s7SwiftUIMXM".to_string());
    imports.insert(0x1008u64, "_$sSHMp".to_string());
    imports.insert(0x1010u64, "_$sSo8NSObjectCMn".to_string());
    assert_eq!(
        import_name(&imports, 0x1000, "module_descriptor").as_deref(),
        Some("SwiftUI")
    );
    assert_eq!(
        import_name(&imports, 0x1008, "protocol_descriptor").as_deref(),
        Some("Swift.Hashable")
    );
    assert_eq!(
        import_name(&imports, 0x1010, "nominal_type_descriptor").as_deref(),
        Some("__C.NSObject")
    );
    // The wrong tag, or an unknown slot, names nothing.
    assert_eq!(import_name(&imports, 0x1008, "module_descriptor"), None);
    assert_eq!(import_name(&imports, 0x2000, "module_descriptor"), None);
}

/// Global-actor isolation (bit 19) and conformance-of-protocol (bit 18) add
/// nothing before the witnesses; only the pack shape count in the top byte
/// does.
#[test]
fn only_real_trailing_objects_move_the_witnesses() {
    use descriptors::witnesses_offset;
    assert_eq!(witnesses_offset(1 << 16), 16);
    assert_eq!(witnesses_offset(1 << 16 | 1 << 19), 16);
    assert_eq!(witnesses_offset(1 << 16 | 1 << 18), 16);
    assert_eq!(witnesses_offset(1 << 6), 20);
    assert_eq!(witnesses_offset(3 << 8), 16 + 36);
    assert_eq!(witnesses_offset(2 << 24), 16 + 16);
    assert_eq!(witnesses_offset(1 << 6 | 1 << 8 | 1 << 24), 16 + 4 + 12 + 8);
}

/// The fixture with `bits` set in every resilient conformance's flags.
fn with_conformance_flags(bits: u32) -> Vec<u8> {
    let original = fixture();
    let file = object::File::parse(&*original).unwrap();
    let conformances = read(&original, original.len()).conformances;
    let mut bytes = original.clone();
    for c in conformances.iter().filter(|c| !c.witnesses.is_empty()) {
        let at = c.addr + 12;
        let sec = file
            .sections()
            .find(|s| s.address() <= at && at < s.address() + s.size())
            .unwrap();
        let off = (at - sec.address() + sec.file_range().unwrap().0) as usize;
        let flags = u32::from_le_bytes(bytes[off..off + 4].try_into().unwrap());
        bytes[off..off + 4].copy_from_slice(&(flags | bits).to_le_bytes());
    }
    bytes
}

/// An isolated conformance (`extension T: @MainActor P`) reads the same
/// witnesses as a plain one.
#[test]
fn an_isolated_conformance_keeps_its_witnesses() {
    let witnesses = |bytes: &[u8]| {
        let mut v: Vec<(u64, usize)> = read(bytes, bytes.len())
            .conformances
            .iter()
            .filter(|c| c.flags & (1 << 16) != 0)
            .map(|c| (c.addr, c.witnesses.len()))
            .collect();
        v.sort_unstable();
        v
    };
    let plain = witnesses(&fixture());
    assert!(plain.iter().all(|(_, n)| *n > 0) && !plain.is_empty());
    assert_eq!(witnesses(&with_conformance_flags(1 << 19)), plain);
    assert_eq!(witnesses(&with_conformance_flags(1 << 18)), plain);
}

fn fixture() -> Vec<u8> {
    std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/swift_metadata_elf"
    ))
    .unwrap()
}

fn read(bytes: &[u8], image_len: usize) -> Metadata {
    let file = object::File::parse(bytes).unwrap();
    let img = MachoImage::new(&file);
    let bind_slots = std::collections::HashSet::new();
    let reader = Reader {
        img: &img,
        bind_slots: &bind_slots,
    };
    reader.read_all(&file, &ELF_SECTIONS, image_len)
}

fn names(bytes: &[u8], budget: usize) -> AnalysisOutput {
    let file = object::File::parse(bytes).unwrap();
    let md = read(bytes, bytes.len());
    let imports = HashMap::new();
    let namer = Namer {
        md: &md,
        imports: &imports,
        memo: Default::default(),
        code: file
            .sections()
            .filter(|s| s.kind() == object::SectionKind::Text)
            .map(|s| (s.address(), s.address().saturating_add(s.size())))
            .collect(),
        budget: std::cell::Cell::new(budget),
    };
    let mut out = AnalysisOutput::default();
    namer.emit(&mut out);
    out
}

/// The fixture with every conformance-section entry pointed at the first
/// entry's descriptor.
fn one_conformance_many_entries() -> Vec<u8> {
    let mut bytes = fixture();
    let (off, addr, len) = {
        let file = object::File::parse(&*bytes).unwrap();
        let sec = file.section_by_name(ELF_SECTIONS.conformances).unwrap();
        let (off, len) = sec.file_range().unwrap();
        (off as usize, sec.address(), len as usize)
    };
    let first = i32::from_le_bytes(bytes[off..off + 4].try_into().unwrap()) as i64;
    let target = addr as i64 + first;
    for i in 0..len / 4 {
        let rel = (target - (addr as i64 + 4 * i as i64)) as i32;
        bytes[off + 4 * i..off + 4 * i + 4].copy_from_slice(&rel.to_le_bytes());
    }
    bytes
}

#[test]
fn a_conformance_named_by_many_entries_is_read_once() {
    assert_eq!(read(&fixture(), usize::MAX).conformances.len(), 3);
    let bytes = one_conformance_many_entries();
    assert_eq!(read(&bytes, bytes.len()).conformances.len(), 1);
    let out = names(&bytes, usize::MAX);
    let mut seen = std::collections::HashSet::new();
    for s in &out.symbols {
        assert!(seen.insert(s.addr), "{:#x} named twice", s.addr);
    }
}

#[test]
fn record_rows_are_bounded_by_the_image_length() {
    let bytes = fixture();
    let full = read(&bytes, bytes.len());
    assert_eq!(full.contexts.len(), read(&bytes, usize::MAX).contexts.len());
    assert!(read(&bytes, 0).contexts.is_empty());
    let few = read(&bytes, 40);
    let rows: usize = few
        .contexts
        .values()
        .map(|c| 1 + c.vtable.len() + c.overrides.len() + c.requirements.len())
        .sum();
    assert!(rows <= 10, "{rows} rows read on a 10-row budget");
    assert!(few.contexts.len() < full.contexts.len());
}

#[test]
fn names_stop_at_the_byte_budget() {
    let bytes = fixture();
    let full = names(&bytes, usize::MAX);
    assert!(full.symbols.len() > 16);
    assert!(names(&bytes, 0).symbols.is_empty());
    let few = names(&bytes, 200);
    let spent: usize = few.symbols.iter().map(|s| s.name.len()).sum();
    assert!(spent <= 200);
    assert!(few.symbols.len() < full.symbols.len());
}
