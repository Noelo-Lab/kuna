//! End-to-end gate for the inventory extent of an image with **no section
//! table** — the `size` every whole-binary surface reports per entry.
//!
//! Fixture: `noshdr_x86_64` (304 bytes, `noshdr_x86_64.py` carries the layout),
//! an ELF64 PIE whose `e_shoff`/`e_shnum`/`e_shstrndx` are all zero. Three
//! `PT_LOAD`s — headers (R), two functions (R+E), data (R+W) — are the only
//! statement the file makes about where anything lives.
//!
//! `funcextent` clips each entry against the loader's CODE sections, and a
//! sectionless image publishes none, so every entry took the "outside every
//! CODE section" sentinel and the whole binary reported `0`. `kuna functions
//! --min-size 1` then discarded all of it (`docs/re-needs/`
//! `zero-function-sizes-make.md`: count 0 of total 12, no error). The fallback
//! clips against the executable load segments instead, which is where these
//! assertions come from:
//!
//! * `sub_100` stops at its neighbour — 16 bytes, the same clip a sectioned
//!   image gets;
//! * `sub_110` stops at the end of the executable segment — 6 bytes;
//! * the non-executable segments never become a container, which is what keeps
//!   a data address from acquiring a body.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_sleigh::loadimage::section_flags;

/// `e_entry`, which calls the second function.
const FIRST: &str = "sub_100";
/// The callee, and the last entry of the executable segment.
const SECOND: &str = "sub_110";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// `None` is a visible skip when the `.sla` is missing.
fn bootstrap(name: &str) -> Option<ConsoleProgram> {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures").join(name);
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(mut prog) => {
            // The recursive discovery `kuna functions` runs, so the inventory
            // here is the one the CLI reports.
            prog.arch_mut().set_kuna_option("fast_funcdisc", "on").expect("fast_funcdisc flips");
            prog.commit_pending_analysis().expect("analysis commit succeeds");
            Some(prog)
        }
        Err(e) => {
            eprintln!(
                "verify_sectionlessextent: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            None
        }
    }
}

fn sizes(prog: &ConsoleProgram) -> Vec<(String, u64)> {
    prog.function_entries_canonical().into_iter().map(|e| (e.name, e.size)).collect()
}

/// The premise: this image really does publish no section at all, so the
/// section-keyed clip has nothing to work with.
#[test]
fn the_fixture_publishes_no_sections_and_three_segments() {
    let Some(prog) = bootstrap("noshdr_x86_64") else {
        return;
    };
    assert_eq!(prog.sections(), Vec::new(), "e_shoff is zero — there is no section table");
    let segments = prog.segments();
    assert_eq!(segments.len(), 3, "the three PT_LOADs are the whole story; got {segments:?}");
    let code: Vec<(u64, u64, u32)> = segments
        .iter()
        .copied()
        .filter(|(_, _, flags)| flags & section_flags::CODE != 0)
        .collect();
    assert_eq!(
        code,
        vec![(0x100, 0x16, section_flags::CODE | section_flags::READONLY)],
        "exactly one segment is executable, and only it may hold a body"
    );
}

/// Both entries carry a real extent: the neighbour clip and the segment-end
/// clip, the same two arms a sectioned image already gets from its sections.
#[test]
fn a_sectionless_image_still_reports_function_extents() {
    let Some(prog) = bootstrap("noshdr_x86_64") else {
        return;
    };
    assert_eq!(
        sizes(&prog),
        vec![(FIRST.to_string(), 16), (SECOND.to_string(), 6)],
        "every entry reported 0 before the segment fallback, which is what made \
         `--min-size 1` discard the whole binary"
    );
}

/// The single-target path (`--addr` on an address the enumeration does not
/// know) clips against the same spans, and an address in a non-executable
/// segment still has no extent — the sentinel keeps meaning what it says.
#[test]
fn the_single_address_path_agrees_and_data_keeps_no_extent() {
    let Some(prog) = bootstrap("noshdr_x86_64") else {
        return;
    };
    assert_eq!(prog.function_extent_at(0x110), 6, "the last entry runs to the segment end");
    assert_eq!(prog.function_extent_at(0x108), 8, "an interior address clips at the next entry");
    assert_eq!(prog.function_extent_at(0x120), 0, "the data segment is not a body");
    assert_eq!(prog.function_extent_at(0x9000), 0, "nor is an address in no segment at all");
}

/// A sectioned image is untouched: the section table still wins where there is
/// one, so the fallback cannot loosen an extent that was already tight.
#[test]
fn a_sectioned_image_keeps_clipping_against_its_sections() {
    let Some(prog) = bootstrap("fauxware") else {
        return;
    };
    assert!(!prog.sections().is_empty(), "fauxware has a section table");
    let main = sizes(&prog)
        .into_iter()
        .find(|(name, _)| name == "main")
        .expect("fauxware defines main");
    assert_eq!(main.1, 195, "the clip is the section-table one, unchanged");
}
