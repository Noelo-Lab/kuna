//! FID PR3 — the `.fid` generator consistency + DB round-trip gate.
//!
//! Two things this proves end-to-end over the vendored fixture
//! (`kuna-analysis/tests/fixtures/fid_lib_x86_64.o`, an x86-64 `.o` with three
//! distinctive self-contained functions — `kuna_crc32`, `kuna_strlen`,
//! `kuna_memset`):
//!
//! 1. **Generator presence + naming:** `build_records` over the bootstrapped
//!    object produces a record named `kuna_crc32`, one named `kuna_strlen`, and
//!    one named `kuna_memset` — the functions are re-identifiable by name.
//!
//! 2. **DB serialization consistency:** the records, written to a `.fid` and
//!    reloaded, are byte-for-byte the same (`find_by_full_hash` recovers each),
//!    and they match the **vendored** `fid_lib_x86_64.fid` (the committed output
//!    of `kuna fid build` over the same `.o`) record-for-record. This ties the
//!    in-process generator to the vendored artifact: if the hash drifts, the
//!    vendored `.fid` and a fresh generate diverge and the test fails.
//!
//! # Architecture note
//!
//! The fixture is **x86-64** (`x86:LE:64:default`), the architecture the FID
//! design doc (`docs/fid-design.md` §7.2) targets. Earlier this fixture was a
//! 32-bit x86 workaround because the FID PR1 `Sleigh::instruction_mask` accessor
//! could not decode REX-prefixed (x86-64) instructions — its post-decode
//! decision-tree re-walk read the *final* multi-phase context (`instrPhase`
//! advanced past the REX prefix phase) and failed to re-resolve the constructor.
//! The PR1-fix captures each node's matched pattern DURING decode (correct
//! per-phase context), so REX-prefixed instructions now mask correctly and the
//! fixture is re-homed to x86-64. The byte-exact PR2 hasher, the `.fid` format,
//! the generator, and the rebasing are all architecture-agnostic.
//!
use std::path::PathBuf;

use kuna_analysis::loadimage_object::ObjectLoadImage;
use kuna_analysis::fid::build::build_records;
use kuna_analysis::fid::db::{FidDatabase, FidDb};
use kuna_console::engine::bootstrap_from_object;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture_o() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fid_lib_x86_64.o")
}

fn fixture_fid() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fid_lib_x86_64.fid")
}

const LANG: &str = "x86:LE:64:default";
const CSPEC: &str = "gcc";

/// Build the FID records for the fixture `.o` in-process.
fn records_for_fixture() -> Vec<kuna_analysis::fid::db::FidRecord> {
    let root = repo_root();
    let specs = root.join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];

    let bin = fixture_o().to_str().expect("UTF-8 fixture path").to_string();
    // The fixture is x86-64; pass the language explicitly so the bootstrap
    // resolves `x86:LE:64:default` (the `.o`'s machine is EM_X86_64, which
    // derives to the same id — the explicit target just pins it).
    let prog = bootstrap_from_object(&bin, LANG, &spec_roots)
        .expect("bootstrap fixture with built processor specs");
    let arch = prog.arch();

    let bytes = std::fs::read(&bin).expect("read fixture .o");
    let image = ObjectLoadImage::from_bytes(&bin, &bytes).expect("open loadimage");
    build_records(
        &bytes,
        &image,
        arch,
        arch.translate().as_sleigh().expect("standalone Sleigh engine"),
    )
}

#[test]
fn generator_recovers_the_three_functions_by_name() {
    let records = records_for_fixture();

    let names: Vec<&str> = records.iter().map(|r| r.name.as_ref()).collect();
    assert!(
        names.contains(&"kuna_crc32"),
        "kuna_crc32 must be fingerprinted; got {names:?}"
    );
    assert!(
        names.contains(&"kuna_strlen"),
        "kuna_strlen must be fingerprinted; got {names:?}"
    );
    assert!(
        names.contains(&"kuna_memset"),
        "kuna_memset must be fingerprinted; got {names:?}"
    );

    // Each is a real, distinctive fingerprint: non-trivial code-unit size and a
    // full hash that is not the FNV offset basis (i.e. bytes were actually mixed).
    for r in &records {
        assert!(r.code_unit_size >= 4, "{}: code_unit_size {} < 4", r.name, r.code_unit_size);
        assert_ne!(r.full_hash, 0xcbf2_9ce4_8422_2325, "{}: full hash is the empty basis", r.name);
    }

    // The three functions have distinct full hashes (no collision in the fixture).
    let mut hashes: Vec<u64> = records.iter().map(|r| r.full_hash).collect();
    hashes.sort_unstable();
    let n = hashes.len();
    hashes.dedup();
    assert_eq!(hashes.len(), n, "the fixture functions must have distinct full hashes");
}

#[test]
fn generator_records_round_trip_through_the_db() {
    let records = records_for_fixture();

    // Build a DB from the freshly generated records, serialize, reload, and
    // assert every record is recoverable by its full hash, field-for-field.
    let db = FidDb::from_records(LANG, CSPEC, records.clone());
    let bytes = db.serialize();
    let back = FidDb::load(&bytes).expect("round-trip load");

    assert_eq!(back.lang_id(), LANG);
    assert_eq!(back.cspec_id(), CSPEC);
    assert_eq!(back.record_count(), records.len());
    for want in &records {
        let got = back.find_by_full_hash(want.full_hash);
        assert!(got.iter().any(|r| r == want), "{} not recovered by full hash", want.name);
    }
}

#[test]
fn fresh_generate_matches_the_vendored_fid() {
    let records = records_for_fixture();

    // The vendored `.fid` is the committed output of `kuna fid build` over the
    // same `.o`. A fresh in-process generate must reproduce the SAME records —
    // this ties the engine to the artifact (a hash drift fails here).
    let vendored_bytes = std::fs::read(fixture_fid()).expect("read vendored .fid");
    let vendored = FidDb::load(&vendored_bytes).expect("load vendored .fid");

    assert_eq!(
        vendored.record_count(),
        records.len(),
        "vendored .fid record count differs from a fresh generate"
    );
    for r in &records {
        let got = vendored.find_by_full_hash(r.full_hash);
        assert!(
            got.iter().any(|v| v.full_hash == r.full_hash
                && v.specific_hash == r.specific_hash
                && v.code_unit_size == r.code_unit_size
                && v.specific_addl == r.specific_addl
                && v.name == r.name),
            "{}: fresh generate diverges from the vendored .fid (regenerate it with \
             `kuna fid build`)",
            r.name
        );
    }
}

#[derive(Clone, Copy)]
enum SwitchLayout {
    Checked,
    UnrelatedComparison,
    SignedComparison,
    CaseReentry,
}

/// A compiler-style signed-delta table with a checked index. Both the `lea`
/// and every table entry require a relocation before the switch can be read.
fn relocatable_switch(case_opcode: u8, rows: usize, layout: SwitchLayout) -> Vec<u8> {
    use object::{
        write, Architecture, BinaryFormat, Endianness, RelocationFlags, SymbolFlags, SymbolKind,
        SymbolScope,
    };

    fn patch_rel8(code: &mut [u8], at: usize, target: usize) {
        code[at] = i8::try_from(target as isize - (at + 1) as isize).unwrap() as u8;
    }
    let mut code = vec![
        0x83, 0xff, 2, // cmp $2,%edi
        0x77, 0,       // ja default
    ];
    match layout {
        SwitchLayout::UnrelatedComparison => {
            // An independent filter on the second parameter. It doesn't
            // reduce the number of possible cases of the checked first.
            code.extend([0x83, 0xfe, 1, 0x77, 0]); // cmp $1,%esi; ja default
        }
        SwitchLayout::SignedComparison => {
            // The first unsigned check still makes every dispatch safe.
            code.extend([0x83, 0xff, 1, 0x7f, 0]); // cmp $1,%edi; jg default
        }
        _ => {}
    }
    let zero_extend = code.len();
    let lea_disp = zero_extend + 5;
    code.extend([
        0x89, 0xff,                    // mov %edi,%edi
        0x48, 0x8d, 0x15, 0, 0, 0, 0, // lea table(%rip),%rdx
        0x48, 0x63, 0x04, 0xba,       // movslq (%rdx,%rdi,4),%rax
        0x48, 0x01, 0xd0,             // add %rdx,%rax
        0xff, 0xe0,                   // jmp *%rax
    ]);
    let case0 = code.len();
    code.extend([0xb8, 1, 0, 0, 0, 0xc3]);
    let case1 = code.len();
    code.extend([case_opcode, 0xc0, 0xc3]);
    let case2 = code.len();
    if matches!(layout, SwitchLayout::CaseReentry) {
        // A shared tail: case 2 reuses case 0, with its own safe selector.
        // This incoming edge becomes known only after visiting the cases.
        code.extend([0xbf, 0, 0, 0, 0, 0xeb, 0]); // mov $0,%edi; jmp zero_extend
        patch_rel8(&mut code, case2 + 6, zero_extend);
    } else {
        code.extend([0xb8, 3, 0, 0, 0, 0xc3]);
    }
    let default = code.len();
    code.extend([0xb8, 0xff, 0xff, 0xff, 0xff, 0xc3]);
    patch_rel8(&mut code, 4, default);
    if matches!(layout, SwitchLayout::UnrelatedComparison | SwitchLayout::SignedComparison) {
        patch_rel8(&mut code, 9, default);
    }
    let mut object = write::Object::new(BinaryFormat::Elf, Architecture::X86_64, Endianness::Little);
    let text = object.section_id(write::StandardSection::Text);
    let rodata = object.section_id(write::StandardSection::ReadOnlyData);
    object.append_section_data(text, &code, 4);
    object.append_section_data(rodata, &[0; 12][..rows * 4], 4);
    object.add_symbol(write::Symbol {
        name: b"dispatch".to_vec(),
        value: 0,
        size: code.len() as u64,
        kind: SymbolKind::Text,
        scope: SymbolScope::Dynamic,
        weak: false,
        section: write::SymbolSection::Section(text),
        flags: SymbolFlags::None,
    });
    let text_symbol = object.section_symbol(text);
    let table_symbol = object.section_symbol(rodata);
    let pc32 = RelocationFlags::Elf { r_type: object::elf::R_X86_64_PC32 };
    object.add_relocation(text, write::Relocation {
        offset: lea_disp as u64,
        symbol: table_symbol,
        addend: -4,
        flags: pc32,
    }).expect("lea relocation");
    for (i, target) in [case0, case1, case2].into_iter().take(rows).enumerate() {
        let offset = (4 * i) as u64;
        object.add_relocation(rodata, write::Relocation {
            offset,
            symbol: text_symbol,
            addend: target as i64 + offset as i64,
            flags: pc32,
        }).expect("table relocation");
    }
    object.write().expect("write switch ELF")
}

fn switch_fingerprints(case_opcode: u8, rows: usize, layout: SwitchLayout) -> (
    Vec<kuna_analysis::fid::db::FidRecord>,
    Result<kuna_analysis::fid::hash::FidHashQuad, &'static str>,
) {
    use kuna_analysis::fid::fingerprint_function;
    use kuna_analysis::listing::{Listing, ListingDetail};

    let bytes = relocatable_switch(case_opcode, rows, layout);
    let path = crate::common::scratch_file("fid-switch", "o");
    std::fs::write(&path, &bytes).expect("write switch input");
    let input = path.to_str().unwrap();
    let specs = vec![repo_root().join("specs").to_str().unwrap().to_string()];
    let prog = bootstrap_from_object(input, LANG, &specs);
    std::fs::remove_file(&path).expect("remove switch input");
    let prog = prog.expect("bootstrap switch object");
    let image = ObjectLoadImage::from_bytes(input, &bytes).expect("load switch ELF");
    let arch = prog.arch();
    let translate = arch.translate().as_sleigh().unwrap();
    let built = build_records(&bytes, &image, arch, translate);
    let entry = image.func_symbols().into_iter().find(|(_, name)| name == "dispatch").unwrap().0;
    let ranges: Vec<_> = image.executable_ranges().into_iter().map(|(lo, size)| (lo, lo + size)).collect();
    let space = arch.manage().get_default_code_space().unwrap();
    let listing = Listing::build_raw(arch, translate, space, &ranges, &[entry], ListingDetail::PARTITION_ONLY);
    (built, fingerprint_function(&listing, arch, entry))
}

#[test]
fn switch_only_case_mutation_changes_build_and_lookup_fingerprints() {
    let (original, lookup) = switch_fingerprints(0x31, 3, SwitchLayout::Checked); // xor %eax,%eax
    let (changed, changed_lookup) = switch_fingerprints(0x29, 3, SwitchLayout::Checked); // sub %eax,%eax
    let original = original.iter().find(|record| record.name.as_ref() == "dispatch").unwrap();
    let changed = changed.iter().find(|record| record.name.as_ref() == "dispatch").unwrap();
    let lookup = lookup.expect("complete switch fingerprints");
    let changed_lookup = changed_lookup.expect("changed switch fingerprints");
    assert_eq!(original.code_unit_size, 15, "{original:?}");
    assert_eq!(original.full_hash, lookup.full_hash);
    assert_eq!(changed.full_hash, changed_lookup.full_hash);
    assert_eq!(original.code_unit_size, lookup.code_unit_size);
    assert_eq!(changed.code_unit_size, changed_lookup.code_unit_size);
    assert_eq!(original.code_unit_size, changed.code_unit_size);
    assert_ne!(original.full_hash, changed.full_hash);
}

#[test]
fn partial_switch_table_cannot_be_built_or_queried() {
    // The valid prefix has two in-body targets; the guard requires three.
    assert_switch_unresolved(2, SwitchLayout::Checked);
}

#[test]
fn unrelated_comparison_cannot_bound_the_switch_index() {
    assert_switch_unresolved(3, SwitchLayout::UnrelatedComparison);
}

#[test]
fn unproven_signed_comparison_cannot_bound_the_switch_index() {
    assert_switch_unresolved(3, SwitchLayout::SignedComparison);
}

#[test]
fn case_body_cannot_bypass_the_switch_proof() {
    assert_switch_unresolved(3, SwitchLayout::CaseReentry);
}

fn assert_switch_unresolved(rows: usize, layout: SwitchLayout) {
    let (records, lookup) = switch_fingerprints(0x31, rows, layout);
    assert!(records.iter().all(|record| record.name.as_ref() != "dispatch"));
    assert_eq!(lookup.unwrap_err(), "unresolved_computed_jump");
}
