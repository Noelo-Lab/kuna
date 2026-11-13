//! End-to-end gate for `msvcfpconst`: MSVC `__real@` floating-point constant
//! COMDATs are recovered from their mangled symbol names.
//!
//! MSVC never encodes a floating-point literal into the instruction stream, so it
//! emits each one as a COMDAT whose *name spells the value*. COMDAT folding then
//! leaves that symbol **undefined** in every object but one, where the loader
//! binds it to a synthetic extern slot with no bytes behind it; and the defined
//! half, though mapped and read-only, only folds under the program-wide `option
//! readonly`, which is default-off. Either way the expression comes out written
//! in `dat_<addr>` operands.
//!
//! The `tests/stages/kuna-msvcfpconst.xml` datatest covers the emitted C. What
//! it *cannot* cover is the loader-level facts under the emitted C — that the
//! undefined slots gain real bytes and that both halves are reported as
//! constant-by-construction ranges, which is what makes the fold legal with
//! `readonly` still off.

use std::path::PathBuf;

use kuna_analysis::loadimage_object::ObjectLoadImage;
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/msvcfpconst_i386.obj")
}

/// Load the fixture.
fn load() -> ObjectLoadImage {
    let path = fixture();
    assert!(path.exists(), "missing fixture {path:?}");
    let bytes = std::fs::read(&path).unwrap();
    ObjectLoadImage::from_bytes(path.to_str().unwrap(), &bytes).expect("the COFF object must load")
}

/// The four `__real@` constants the fixture carries, as
/// `(symbol, decoded f64, storage width)`. Two are defined in the object (the
/// bare-bits spellings a modern compiler emits) and two are undefined externs
/// (the VC6-era x87 80-bit spelling), which is the state COMDAT folding produces.
const CONSTANTS: &[(&str, f64, usize)] = &[
    ("__real@3fb999999999999a", 0.1, 8),
    ("__real@3f000000", 0.5, 4),
    ("__real@8@3ffec90fdaa22168c000", std::f64::consts::FRAC_PI_4, 8),
    ("__real@4@3fff8000000000000000", 1.0, 4),
];

/// The decoder agrees with the object's own defined bytes and with the value each
/// name denotes — asserted here, at the crate boundary, so a change to the
/// mangling rules fails against a real MSVC-ABI object rather than only against
/// the unit tests' string literals.
#[test]
fn the_fixture_carries_all_four_spellings_and_each_decodes() {
    use kuna_analysis::loader::kuna_msvcfpconst::decode;
    for (name, want, size) in CONSTANTS {
        let c = decode(name).unwrap_or_else(|| panic!("{name} must decode"));
        assert_eq!(c.size, *size, "{name} storage width");
        let got = if c.size == 4 {
            f32::from_le_bytes(c.bytes[..4].try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(c.bytes[..8].try_into().unwrap())
        };
        assert_eq!(got, *want, "{name}");
    }
}

/// The loader half, both arms plus the shipped default.
///
/// On, all four constants are reported as constant-by-construction ranges — the
/// list that folds with the program-wide `option readonly` still off. Both halves
/// must be on it: listing only the undefined one would leave an expression with a
/// literal on one side and `dat_<addr>` on the other, which reads worse than
/// either extreme.
#[test]
fn both_halves_are_reported_foldable() {
    let widths: Vec<u64> = CONSTANTS.iter().map(|(_, _, s)| *s as u64).collect();

    let on = load();
    let ranges = on.dynreloc_const_ranges().to_vec();
    assert_eq!(
        ranges.len(),
        CONSTANTS.len(),
        "one range per __real@ constant, defined and undefined alike; got {ranges:x?}"
    );
    let mut sizes: Vec<u64> = ranges.iter().map(|(lo, hi)| hi - lo + 1).collect();
    sizes.sort_unstable();
    let mut want = widths.clone();
    want.sort_unstable();
    assert_eq!(sizes, want, "each range must span exactly its storage width");
}

/// The undefined half gains real bytes. Before this, a synthetic extern slot had
/// no backing at all, so *any* read of it failed outright — the range list alone
/// would have declared an unreadable address constant.
#[test]
fn the_undefined_slots_read_back_as_their_decoded_values() {
    use kuna_base::address::Address;
    use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};
    use kuna_sleigh::loadimage::LoadImage;

    let mut image = load();
    // A minimal little-endian `ram` space to address the image through; the loader
    // only checks pointer identity against the space it was attached to.
    let space = std::rc::Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        4,
        1,
        1,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    image.attach_to_space(std::rc::Rc::clone(&space));

    let mut found = 0usize;
    for (lo, hi) in image.dynreloc_const_ranges().to_vec() {
        let size = (hi - lo + 1) as usize;
        let mut buf = vec![0u8; size];
        image
            .load_fill(&mut buf, &Address::new(std::rc::Rc::clone(&space), lo))
            .unwrap_or_else(|e| panic!("a foldable range at {lo:#x} must be readable: {e:?}"));
        let value = if size == 4 {
            f32::from_le_bytes(buf[..4].try_into().unwrap()) as f64
        } else {
            f64::from_le_bytes(buf[..8].try_into().unwrap())
        };
        assert!(
            CONSTANTS.iter().any(|(_, want, w)| *w == size && *want == value),
            "the datum at {lo:#x} ({size} bytes) reads {value}, which is none of the \
             fixture's constants"
        );
        found += 1;
    }
    assert_eq!(found, CONSTANTS.len());
}

/// Bootstrap the object.
fn boot() -> ConsoleProgram {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    bootstrap_from_object(fixture().to_str().unwrap(), "", &spec_roots)
        .expect("bootstrap fixture with built processor specs")
}

/// Run `load function <name>` -> `decompile` -> `print C` and return the C.
fn decompile_func(prog: ConsoleProgram, name: &str) -> String {
    let cmds: Vec<String> = [format!("load function {name}"), "decompile".into(), "print C".into()]
        .into_iter()
        .collect();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

/// The headline, at the shipped default with no option set anywhere: the
/// expression is arithmetic over literals rather than over addresses.
#[test]
fn the_shipped_default_emits_literals_not_addresses() {
    let prog = boot();
    let out = decompile_func(prog, "_scale");
    assert!(
        out.contains("0.7853981633974483"),
        "the undefined x87 COMDAT must decode to pi/4, got:\n{out}"
    );
    assert!(
        out.contains("0.1"),
        "the defined IEEE-double COMDAT must fold beside it, got:\n{out}"
    );
    assert!(
        !out.contains("dat_"),
        "no operand may be left opaque at the shipped default, got:\n{out}"
    );
}
