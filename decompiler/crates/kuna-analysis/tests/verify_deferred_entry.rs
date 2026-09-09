//! (kuna) The deferred entry-discovery contract, in its own test binary.
//!
//! `funcstart_patterns` is not a load-time pass: its whole-image `<patternpairs>`
//! sweep runs at the commit point, where the `--option funcstart_patterns on|off`
//! gate is finally in effect, instead of on every load with the result usually
//! discarded. Two things have to hold for that to be free:
//!
//! * it must be absent from every `passes_for` set (otherwise the sweep is paid
//!   twice), and
//! * the deferred run must see the same object view a load-time pass sees —
//!   `relocrebase`d for a relocatable object, whose sections the loader lays out
//!   synthetically. On the raw view the fixture's two un-symboled prologues report
//!   as 0x38/0x95, section-relative addresses the engine has no image for.
//!
//! Own binary because the second assertion reads the process-global
//! `KUNA_RELOCREBASE` gate.

use kuna_analysis::loadimage_object::ObjectLoadImage;
use kuna_analysis::passes::{passes_for, run_deferred_entry_passes};
use kuna_analysis::sourcelang::Compiler;
use kuna_decomp::architecture::Architecture;
use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::sleigh::Sleigh;

fn fixture(name: &str) -> String {
    format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn funcstart_patterns_is_deferred_not_registered_at_load() {
    for format in [
        object::BinaryFormat::Elf,
        object::BinaryFormat::Pe,
        object::BinaryFormat::MachO,
    ] {
        for compiler in [
            Compiler::Unknown,
            Compiler::Gcc,
            Compiler::Clang,
            Compiler::Rustc,
            Compiler::Go,
        ] {
            let ids: Vec<&str> = passes_for(compiler, format).iter().map(|p| p.id()).collect();
            assert!(
                !ids.contains(&"funcstart_patterns"),
                "{compiler:?}/{format:?} must not sweep the pattern corpus at load"
            );
        }
    }
}

/// The deferred run rebases a relocatable object, so its entries land in the
/// address space the loader gave the image (0x400000-based here), not the raw
/// object's section-relative one.
#[test]
fn deferred_entry_pass_rebases_a_relocatable_object() {
    let path = fixture("deferredentry_rel_x86_64.o");
    let Ok(bytes) = std::fs::read(&path) else {
        panic!("read the ET_REL fixture {path}");
    };
    let raw = object::File::parse(&bytes[..]).expect("parse the ET_REL fixture");
    assert_eq!(
        kuna_analysis::entry::full_pattern_starts(&raw),
        vec![0x38, 0x95],
        "the raw object view reports section-relative starts"
    );

    let image = ObjectLoadImage::from_bytes(&path, &bytes).expect("load the ET_REL fixture");
    let backing = ObjectLoadImage::from_bytes(&path, &bytes).expect("load the ET_REL fixture");
    let arch = Architecture::new(
        "x86:LE:64:default",
        Sleigh::new(Box::new(backing), Box::new(ContextInternal::new())),
    );

    let out = run_deferred_entry_passes(&bytes, &image, &arch);
    let ids: Vec<&str> = out.iter().map(|(id, _)| *id).collect();
    assert_eq!(ids, vec!["funcstart_patterns"]);
    let entries: Vec<u64> = out.iter().flat_map(|(_, o)| o.entries.iter().copied()).collect();
    assert_eq!(entries, vec![0x400038, 0x400095]);
}
