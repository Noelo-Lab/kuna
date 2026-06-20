//! Adversarial verifier tests for item `w4-fw-architecture` (round 1).
//!
//! Targets the hunt-list spots flagged as most fragile for the
//! `architecture.{cc,hh}` / `capability.{cc,hh}` / `libdecomp.{cc,hh}` port:
//!
//!   * the non-angr `func_` default name — does it match C++
//!     `Architecture::nameFunction` -> `Address::printRaw` byte-for-byte?
//!     (iteration-order is not the risk here; *format fidelity* is);
//!   * `ArchitectureCapability::sortCapabilities` relative-order preservation
//!     for the >1-element-after-raw case and the raw-at-front rotation
//!     (off-by-one in the C++ shift loop `thelist[j-1]=thelist[j]`);
//!   * `findCapability` first-match-wins *after* the sort has reordered the
//!     list (the sort must not change which capability a file routes to except
//!     by the documented raw-last rule);
//!   * `startDecompilerLibrary` init order and the spec-path accumulation order
//!     (sleighhome scanned before the extra paths).
//!
//! Driven entirely through the public crate API so the diff stands on its own.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};

use kuna_decomp::capability::{CapabilityRegistry, NamedCapability};
use kuna_decomp::libdecomp::start_decompiler_library;

use kuna_sleigh::globalcontext::ContextInternal;
use kuna_sleigh::loadimage::LoadImage;
use kuna_sleigh::sleigh::Sleigh;
use kuna_base::error::{KunaError, KunaResult};

use kuna_decomp::architecture::Architecture;

// --- minimal Sleigh + Address scaffolding -----------------------------------

struct DummyImg;
impl LoadImage for DummyImg {
    fn get_file_name(&self) -> &str {
        "dummy"
    }
    fn load_fill(&mut self, _ptr: &mut [u8], _addr: &Address) -> KunaResult<()> {
        Err(KunaError::data_unavail("dummy"))
    }
    fn get_arch_type(&self) -> Vec<u8> {
        Vec::new()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
}

fn bare_sleigh() -> Sleigh {
    Sleigh::new(Box::new(DummyImg), Box::new(ContextInternal::new()))
}

/// A 4-byte, word-size-1 `ram`-like processor space (so `byteToAddress` is a
/// no-op and the C++ `printRaw` is just the zero-padded hex offset).
fn ram_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        4, // addr size
        1, // word size
        7, // index
        addrspace_flags::hasphysical,
        1,
        1,
    ))
}

// ---------------------------------------------------------------------------
// F1: the non-angr `func_` default name must match C++ Address::printRaw.
//
// C++ `Architecture::nameFunction` (architecture.cc:539) on the upstream path:
//     ostringstream defname; defname << "func_"; addr.printRaw(defname);
// `Address::printRaw` -> `AddrSpace::printRaw` (space.cc:206) prints
//     "0x" << setw(2*sz) << hex << byteToAddress(offset,wordsize)
// with NO space-name prefix.  For ram:0x401000 (4-byte space) that is
//     func_0x00401000
// kuna-base already has a faithful `Address::print_raw`, so the C++ oracle is
// computed directly from it here.
// ---------------------------------------------------------------------------

#[test]
fn func_default_name_matches_cpp_printraw_oracle() {
    let mut arch = Architecture::new("t", bare_sleigh());
    arch.name_style_angr = false; // exercise the upstream `func_` path

    let addr = Address::new(ram_space(), 0x401000);

    // Oracle: exactly what C++ nameFunction would emit ("func_" + printRaw).
    let mut oracle = String::from("func_");
    addr.print_raw(&mut oracle).expect("print_raw");

    let got = arch.name_function(&addr);
    assert_eq!(
        got, oracle,
        "non-angr func_ name diverges from C++ Address::printRaw (got {got}, C++ {oracle})"
    );
    // And the C++ form specifically: zero-padded, no space-name prefix.
    assert_eq!(oracle, "func_0x00401000");
}

// ---------------------------------------------------------------------------
// sortCapabilities: >1 element after raw keeps relative order (off-by-one in
// the C++ shift loop would scramble them).
// ---------------------------------------------------------------------------

#[test]
fn sort_capabilities_preserves_order_of_many_trailing() {
    let mut reg = CapabilityRegistry::new();
    reg.register(Box::new(NamedCapability::new("a", false, None)));
    reg.register(Box::new(NamedCapability::new("raw", false, None)));
    reg.register(Box::new(NamedCapability::new("b", false, None)));
    reg.register(Box::new(NamedCapability::new("c", false, None)));
    reg.register(Box::new(NamedCapability::new("d", false, None)));
    reg.sort_capabilities();
    // C++: a stays, b,c,d slide down one, raw appended last.
    assert_eq!(reg.names(), vec!["a", "b", "c", "d", "raw"]);
}

#[test]
fn sort_capabilities_raw_at_front_rotates_left() {
    let mut reg = CapabilityRegistry::new();
    reg.register(Box::new(NamedCapability::new("raw", false, None)));
    reg.register(Box::new(NamedCapability::new("x", false, None)));
    reg.register(Box::new(NamedCapability::new("y", false, None)));
    reg.sort_capabilities();
    // Everything shifts down by one and raw goes last.
    assert_eq!(reg.names(), vec!["x", "y", "raw"]);
}

#[test]
fn sort_capabilities_first_raw_only_moves() {
    // Degenerate: two "raw" entries.  C++ `for(...) if(name=="raw") break;`
    // finds only the FIRST and moves only that one.
    let mut reg = CapabilityRegistry::new();
    reg.register(Box::new(NamedCapability::new("p", false, None)));
    reg.register(Box::new(NamedCapability::new("raw", false, None)));
    reg.register(Box::new(NamedCapability::new("raw", false, None)));
    reg.register(Box::new(NamedCapability::new("q", false, None)));
    reg.sort_capabilities();
    // First raw removed and appended; second raw stays where it slid to.
    assert_eq!(reg.names(), vec!["p", "raw", "q", "raw"]);
}

// ---------------------------------------------------------------------------
// findCapability first-match-wins survives the raw-last sort.
// ---------------------------------------------------------------------------

#[test]
fn find_capability_by_file_first_match_after_sort() {
    let mut reg = CapabilityRegistry::new();
    // "raw" matches files too (the real raw arch is a catch-all); it must lose
    // to an earlier specific matcher even before the sort, and especially after.
    reg.register(Box::new(NamedCapability::new("raw", true, None)));
    reg.register(Box::new(NamedCapability::new("specific", true, None)));
    // Before sort: raw is first, so raw would win — that's the bug sort fixes.
    assert_eq!(reg.find_capability_by_file("/x").map(|c| c.get_name()), Some("raw"));
    reg.sort_capabilities();
    // After sort raw is last, so the specific matcher wins.
    assert_eq!(
        reg.find_capability_by_file("/x").map(|c| c.get_name()),
        Some("specific")
    );
}

// ---------------------------------------------------------------------------
// startDecompilerLibrary init order + spec-path accumulation order.
// C++: scanForSleighDirectories(sleighhome) THEN specpaths.addDir2Path(extra)
// so sleighhome precedes the extra paths, in extra-path index order.
// ---------------------------------------------------------------------------

#[test]
fn start_library_spec_paths_sleighhome_then_extras_in_order() {
    let extras = vec!["/e1".to_string(), "/e2".to_string(), "/e3".to_string()];
    let lib = start_decompiler_library(Some("/home"), &extras, CapabilityRegistry::new());
    assert_eq!(lib.spec_paths, vec!["/home", "/e1", "/e2", "/e3"]);
    assert!(lib.capabilities.is_initialized());
}

#[test]
fn start_library_no_home_just_extras() {
    let extras = vec!["/only".to_string()];
    let lib = start_decompiler_library(None, &extras, CapabilityRegistry::new());
    assert_eq!(lib.spec_paths, vec!["/only"]);
}

#[test]
fn start_library_initializes_and_sorts_raw_last() {
    let mut caps = CapabilityRegistry::new();
    caps.register(Box::new(NamedCapability::new("raw", false, None)));
    caps.register(Box::new(NamedCapability::new("xml", false, None)));
    caps.register(Box::new(NamedCapability::new("sleigh", false, None)));
    let lib = start_decompiler_library(None, &[], caps);
    assert!(lib.capabilities.is_initialized());
    assert_eq!(lib.capabilities.names(), vec!["xml", "sleigh", "raw"]);
}
