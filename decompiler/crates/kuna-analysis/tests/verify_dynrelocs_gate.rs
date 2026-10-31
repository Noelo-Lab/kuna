//! (kuna `dynrelocs`) The `dynrelocs` option gate, in its own test binary.
//!
//! The gate is a process-global environment variable (the relocations are applied
//! while the loader snapshots the image, upstream of every per-function
//! `option`), so its two arms cannot share a process with the default-on
//! assertions in the crate's lib tests — cargo runs each integration-test target
//! as its own binary, which is exactly the isolation this needs.
//!
//! Driven against the real PIE fixture rather than a synthetic image, so the arm
//! that matters is the one a user actually gets.

use kuna_analysis::loader::kuna_dynrelocs::resolve;
use kuna_decomp::kuna_dynrelocs::DYNRELOCS_ENV;

/// Serializes the tests below: each flips the one process-global gate.
static GATE: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// `off` yields nothing at all (the pre-fix bytes); unset yields the PIE's three
/// `R_X86_64_RELATIVE` writes and the two `PT_GNU_RELRO`-covered const slots.
#[test]
fn gate_off_is_inert_and_default_on_applies() {
    let _gate = GATE.lock().unwrap_or_else(|e| e.into_inner());
    let path = format!("{}/tests/fixtures/cet_pie_x86_64", env!("CARGO_MANIFEST_DIR"));
    let bytes = std::fs::read(&path).expect("PIE fixture");
    let file = object::File::parse(&bytes[..]).expect("parse");

    std::env::set_var(DYNRELOCS_ENV, "off");
    let off = resolve(&file, &bytes);
    std::env::remove_var(DYNRELOCS_ENV);
    assert!(off.writes.is_empty(), "off must apply nothing");
    assert!(off.const_ranges.is_empty(), "off must declare nothing constant");

    let on = resolve(&file, &bytes);
    assert_eq!(on.writes.len(), 3, "the three RELATIVE slots (imports are skipped)");
    assert_eq!(
        on.const_ranges,
        vec![(0x3d78u64, 0x3d7fu64), (0x3d80, 0x3d87)],
        "only the RELRO-covered slots"
    );
}

/// The Mach-O arm: `off` leaves the chained-fixup words as the linker wrote them;
/// on, each rebase is its target and the bind a zero, in Apple's
/// `DYLD_CHAINED_PTR_64_OFFSET` layout as in `ld64.lld`'s `DYLD_CHAINED_PTR_64`.
#[test]
fn macho_chained_fixups_follow_the_gate() {
    let _gate = GATE.lock().unwrap_or_else(|e| e.into_inner());
    for name in ["macho_ptrslots_arm64", "macho_ptrslots_offset_arm64"] {
        let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(&path).expect("Mach-O fixture");
        let file = object::File::parse(&bytes[..]).expect("parse");

        std::env::set_var(DYNRELOCS_ENV, "off");
        let off = resolve(&file, &bytes);
        std::env::remove_var(DYNRELOCS_ENV);
        assert!(off.writes.is_empty(), "{name}: off must apply nothing");

        let on: Vec<(u64, u64, u8)> =
            resolve(&file, &bytes).writes.iter().map(|w| (w.vma, w.value, w.width)).collect();
        assert_eq!(
            on,
            vec![(0x100004000, 0x100000440, 8), (0x100004008, 0x1000003c8, 8), (0x100004010, 0, 8)],
            "{name}: two rebases and a zeroed bind"
        );
    }
}
