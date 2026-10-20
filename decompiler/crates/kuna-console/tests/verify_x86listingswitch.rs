//! Bounded i386 switch coverage before the AIF gap walk.

use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::common;
use kuna_analysis::listing::model::{DiscoveredFunction, FlowType, RefKind};
use kuna_analysis::listing::{Listing, ListingDetail, WalkPlan};
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use object::{Object, ObjectSection};

const DISPATCH: u64 = 0x100400;
const BRANCH: u64 = 0x10040d;
const CASES: [u64; 4] = [0x100440, 0x100460, 0x1004b0, 0x1004d0];
const INDEPENDENT: u64 = 0x100480;
const GAP: u64 = 0x100490;
const DEFAULT: u64 = 0x1004f0;
const HELPER: u64 = 0x100510;
const ENTRY: u64 = 0x100000;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/x86listingswitch_i386")
}

struct Loaded {
    prog: ConsoleProgram,
    bytes: Vec<u8>,
    path: String,
}

impl Loaded {
    fn new(enabled: bool) -> Self {
        Self::from_path(&fixture(), enabled)
    }

    fn from_path(path: &Path, enabled: bool) -> Self {
        let path = path.to_str().unwrap().to_owned();
        let specs = vec![repo_root().join("specs").to_str().unwrap().to_owned()];
        let mut prog = bootstrap_from_object(&path, "", &specs)
            .expect("bootstrap i386 fixture with built processor specs");
        prog.arch_mut()
            .set_kuna_option("x86listingswitch", if enabled { "on" } else { "off" })
            .expect("set x86listingswitch before discovery");
        let bytes = std::fs::read(&path).expect("read i386 fixture");
        Self { prog, bytes, path }
    }

    fn listing(&self, detail: ListingDetail, plan: &WalkPlan, independent_seed: bool) -> Listing {
        let mut seeds = vec![DEFAULT, ENTRY];
        let mut names = vec![
            (DEFAULT, "kept_default".into()),
            (ENTRY, "kept_entry".into()),
        ];
        if independent_seed {
            seeds.push(INDEPENDENT);
            names.push((INDEPENDENT, "kept_independent".into()));
        }
        seeds.sort_unstable();
        self.with_seeds(detail, plan, &seeds, &names)
    }

    fn with_seeds(
        &self,
        detail: ListingDetail,
        plan: &WalkPlan,
        seeds: &[u64],
        names: &[(u64, String)],
    ) -> Listing {
        let file = object::File::parse(&*self.bytes).expect("parse stripped i386 fixture");
        let image =
            kuna_analysis::loadimage_object::ObjectLoadImage::from_bytes(&self.path, &self.bytes)
                .expect("open i386 fixture image");
        let arch = self.prog.arch();
        Listing::build_with_meta_planned(
            &file,
            &image,
            arch,
            arch.translate(),
            seeds,
            seeds,
            names,
            detail,
            plan,
        )
    }

    fn aif(&self, listing: &Listing) -> Vec<u64> {
        let file = object::File::parse(&*self.bytes).expect("parse stripped i386 fixture");
        let arch = self.prog.arch();
        let bracket = kuna_analysis::aif::kuna_aifbracket::BracketEvidence::from_object(
            &file,
            listing.exec_ranges(),
        );
        assert!(
            listing.function_count() >= 20,
            "the AIF fingerprint corpus must be active"
        );
        kuna_analysis::aif::run_aif(
            listing,
            arch.translate(),
            Rc::clone(
                arch.manage()
                    .get_default_code_space()
                    .expect("i386 code space"),
            ),
            listing.exec_ranges(),
            true,
            false,
            Some(&bracket),
            true,
            None,
        )
    }
}

type Instructions = Vec<(u64, u32, Option<u64>, FlowType, Vec<u64>)>;
type Functions = Vec<(u64, DiscoveredFunction)>;
type References = Vec<(u64, Vec<(u64, u64, RefKind)>)>;

fn instructions(listing: &Listing) -> Instructions {
    listing
        .instructions()
        .map(|(&a, i)| (a, i.len, i.fall_through, i.flow, i.flows.clone()))
        .collect()
}

fn functions(listing: &Listing) -> Functions {
    listing.functions().map(|(&a, f)| (a, f.clone())).collect()
}

fn references(listing: &Listing, incoming: bool) -> References {
    let mut keys: Vec<u64> = if incoming {
        listing
            .ref_source_iter()
            .flat_map(|a| listing.refs_from(a).iter().map(|r| r.to))
            .collect()
    } else {
        listing.ref_source_iter().collect()
    };
    keys.sort_unstable();
    keys.dedup();
    keys.into_iter()
        .map(|a| {
            let refs = if incoming {
                listing.refs_to(a)
            } else {
                listing.refs_from(a)
            };
            (a, refs.iter().map(|r| (r.from, r.to, r.kind)).collect())
        })
        .collect()
}

fn assert_same_listing(actual: &Listing, expected: &Listing) {
    assert_eq!(
        instructions(actual),
        instructions(expected),
        "instruction partition changed"
    );
    assert_eq!(
        functions(actual),
        functions(expected),
        "function identity changed"
    );
    assert_eq!(
        references(actual, false),
        references(expected, false),
        "outgoing references changed"
    );
    assert_eq!(
        references(actual, true),
        references(expected, true),
        "incoming references changed"
    );
    assert_eq!(actual.exec_ranges(), expected.exec_ranges());
    let assembly = |listing: &Listing| {
        listing
            .instructions()
            .map(|(&a, i)| (a, i.mnemonic.to_string(), i.operands.clone()))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        assembly(actual),
        assembly(expected),
        "assembly capture changed"
    );
}

fn assert_coverage(listing: &Listing, enabled: bool, independent_seed: bool) {
    assert!(listing.is_instruction_start(DISPATCH));
    assert!(listing.is_instruction_start(DEFAULT));
    assert_eq!(
        listing.function_at(DEFAULT).unwrap().name.as_deref(),
        Some("kept_default")
    );
    assert!(listing.function_at(DEFAULT).unwrap().from_symbol);
    assert_eq!(
        listing.function_at(ENTRY).unwrap().name.as_deref(),
        Some("kept_entry")
    );
    for case in CASES {
        assert_eq!(
            listing.is_instruction_start(case),
            enabled,
            "case {case:#x}"
        );
        assert!(
            listing.function_at(case).is_none(),
            "a case is an instruction successor"
        );
    }
    assert_eq!(
        listing.is_instruction_start(HELPER),
        enabled,
        "case-only callee coverage"
    );
    assert_eq!(
        listing.function_at(HELPER).is_some(),
        enabled,
        "a direct case call claims a function"
    );
    if enabled {
        assert!(!listing.function_at(HELPER).unwrap().from_symbol);
    }
    assert_eq!(listing.is_instruction_start(INDEPENDENT), independent_seed);
    assert_eq!(listing.function_at(INDEPENDENT).is_some(), independent_seed);
    if independent_seed {
        let function = listing.function_at(INDEPENDENT).unwrap();
        assert_eq!(function.name.as_deref(), Some("kept_independent"));
        assert!(function.from_symbol);
    }
    for address in GAP..GAP + 0x20 {
        assert!(
            listing.is_undefined(address),
            "interleaved gap {address:#x} was claimed"
        );
    }
    let targets: Vec<u64> = listing
        .refs_from(BRANCH)
        .iter()
        .filter(|r| r.kind == RefKind::Code)
        .map(|r| r.to)
        .collect();
    let expected = if enabled && listing.has_refs() {
        CASES.to_vec()
    } else {
        Vec::new()
    };
    assert_eq!(
        targets, expected,
        "the callback word must never become a fifth case"
    );
    assert!(!listing
        .refs_from(BRANCH)
        .iter()
        .any(|r| r.to == INDEPENDENT));
}

#[test]
fn bounded_cases_replace_false_aif_entries_and_recover_the_case_only_callee() {
    for detail in [ListingDetail::FULL, ListingDetail::PARTITION_ONLY] {
        let off = Loaded::new(false);
        let ordinary = off.listing(detail, &WalkPlan::serial(), false);
        assert_coverage(&ordinary, false, false);
        let mut expected = CASES.to_vec();
        expected.push(INDEPENDENT);
        expected.sort_unstable();
        assert_eq!(
            off.aif(&ordinary),
            expected,
            "the off arm must reproduce AIF case entries"
        );

        let on = Loaded::new(true);
        let expanded = on.listing(detail, &WalkPlan::serial(), false);
        assert_coverage(&expanded, true, false);
        assert_eq!(
            on.aif(&expanded),
            vec![INDEPENDENT],
            "independent code stays discoverable"
        );
    }
}

#[test]
fn case_membership_preserves_an_interleaved_named_function_and_gap() {
    for enabled in [false, true] {
        let loaded = Loaded::new(enabled);
        let full = loaded.listing(ListingDetail::FULL, &WalkPlan::serial(), true);
        let lean = loaded.listing(ListingDetail::PARTITION_ONLY, &WalkPlan::serial(), true);
        assert_coverage(&full, enabled, true);
        assert_coverage(&lean, enabled, true);
        assert_eq!(instructions(&full), instructions(&lean));
        assert_eq!(functions(&full), functions(&lean));
        assert_eq!(full.exec_ranges(), lean.exec_ranges());
        assert!(lean.ref_source_iter().next().is_none());
    }
}

#[test]
fn guard_and_cases_cross_real_parallel_interval_boundaries_without_changing_the_model() {
    for enabled in [false, true] {
        let loaded = Loaded::new(enabled);
        for independent_seed in [false, true] {
            for detail in [ListingDetail::FULL, ListingDetail::PARTITION_ONLY] {
                let serial = loaded.listing(detail, &WalkPlan::serial(), independent_seed);
                assert_coverage(&serial, enabled, independent_seed);
                for lanes in [2, 3, 4] {
                    for intervals in [1, 4, 32] {
                        let plan = WalkPlan::for_lanes(loaded.prog.arch(), lanes, intervals, 0);
                        assert_eq!(plan.declined(), None);
                        let parallel = loaded.listing(detail, &plan, independent_seed);
                        assert!(
                            plan.engaged() > 0,
                            "{lanes} lanes x {intervals} did not run"
                        );
                        assert_same_listing(&parallel, &serial);
                    }
                }
            }
        }
    }
}

#[test]
#[cfg(all(target_os = "linux", any(target_arch = "x86", target_arch = "x86_64")))]
fn native_i386_fixture_checks_all_cases_the_case_callee_and_out_of_range_inputs() {
    let status = std::process::Command::new(fixture())
        .status()
        .expect("execute native i386 fixture");
    assert!(
        status.success(),
        "i386 dispatch behavior disagreed with its assembly oracle: {status}"
    );
}

struct PatchedFixture {
    bytes: Vec<u8>,
    path: PathBuf,
}

impl PatchedFixture {
    fn new(label: &str) -> Self {
        Self {
            bytes: std::fs::read(fixture()).expect("read patchable i386 fixture"),
            path: common::scratch_file(label, "elf"),
        }
    }

    fn patch(&mut self, address: u64, value: &[u8]) {
        let file = object::File::parse(&*self.bytes).unwrap();
        let offset = file
            .sections()
            .find_map(|section| {
                let delta = address.checked_sub(section.address())?;
                let (offset, size) = section.file_range()?;
                (delta.checked_add(value.len() as u64)? <= size)
                    .then_some((offset + delta) as usize)
            })
            .expect("patch must fit a file-backed section");
        self.bytes[offset..offset + value.len()].copy_from_slice(value);
    }

    fn word(&mut self, address: u64, value: u64) {
        self.patch(address, &u32::try_from(value).unwrap().to_le_bytes());
    }

    fn relative(&mut self, address: u64, opcode: u8, target: u64) {
        self.patch(address, &[opcode]);
        let delta = i32::try_from(target as i64 - address as i64 - 5).unwrap();
        self.patch(address + 1, &delta.to_le_bytes());
    }

    fn section_header(&mut self, name: &str, field: usize, value: u32) {
        let file = object::File::parse(&*self.bytes).unwrap();
        assert!(!file.is_64() && file.is_little_endian());
        let index = file.section_by_name(name).unwrap().index().0;
        let table = u32::from_le_bytes(self.bytes[0x20..0x24].try_into().unwrap()) as usize;
        let stride = u16::from_le_bytes(self.bytes[0x2e..0x30].try_into().unwrap()) as usize;
        let offset = table + index * stride + field;
        self.bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }

    fn clear_text(&mut self) {
        let file = object::File::parse(&*self.bytes).unwrap();
        let section = file.section_by_name(".text").unwrap();
        let (offset, size) = section.file_range().unwrap();
        self.bytes[offset as usize..(offset + size) as usize].fill(0xcc);
    }

    fn save(&self) -> &Path {
        std::fs::write(&self.path, &self.bytes).expect("write temporary i386 control");
        &self.path
    }
}

impl Drop for PatchedFixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

fn assert_whole_candidate_refused(fixture: &PatchedFixture, seeds: &[u64], branches: &[u64]) {
    let path = fixture.save();
    let off = Loaded::from_path(path, false);
    let on = Loaded::from_path(path, true);
    let names: Vec<_> = seeds
        .iter()
        .map(|&entry| (entry, format!("kept_{entry:x}")))
        .collect();
    for detail in [ListingDetail::FULL, ListingDetail::PARTITION_ONLY] {
        let ordinary = off.with_seeds(detail, &WalkPlan::serial(), seeds, &names);
        for &branch in branches {
            let insn = ordinary
                .instruction_at(branch)
                .expect("ordinary walk must reach dispatch");
            assert!(insn.flow.is_computed && insn.flow.is_jump);
            assert!(ordinary.refs_from(branch).is_empty());
        }
        let serial = on.with_seeds(detail, &WalkPlan::serial(), seeds, &names);
        assert_same_listing(&serial, &ordinary);
        let plan = WalkPlan::for_lanes(on.prog.arch(), 3, 4, 0);
        let parallel = on.with_seeds(detail, &plan, seeds, &names);
        assert!(
            plan.engaged() > 0,
            "refusal must also run on real decode lanes"
        );
        assert_same_listing(&parallel, &ordinary);
    }
}

#[test]
fn unproved_guards_and_unreadable_tables_never_fall_back_to_pointer_scanning() {
    for variant in [
        "missing",
        "wrong_index",
        "byte_compare",
        "signed",
        "lower_bound",
        "ah_write",
        "ax_write",
        "wrong_stride",
        "import_veneer",
        "writable",
        "short",
        "invalid",
    ] {
        let mut control = PatchedFixture::new(variant);
        let mut branch = BRANCH;
        match variant {
            "missing" => control.patch(DISPATCH + 4, &[0x90, 0x90, 0x90]),
            "wrong_index" => control.patch(DISPATCH + 5, &[0xf9]),
            "byte_compare" => control.patch(DISPATCH + 4, &[0x3c, 3, 0x90]),
            "signed" => control.patch(DISPATCH + 8, &[0x8f]),
            "lower_bound" => control.patch(DISPATCH + 8, &[0x82]),
            "ah_write" | "ax_write" => {
                let clobber: &[u8] = if variant == "ah_write" {
                    &[0xb4, 1]
                } else {
                    &[0x66, 0xb8, 4, 0]
                };
                control.patch(BRANCH, clobber);
                branch += clobber.len() as u64;
                control.patch(branch, &[0xff, 0x24, 0x85, 0, 0x10, 0x10, 0]);
            }
            "wrong_stride" => control.patch(BRANCH + 2, &[0xc5]),
            "import_veneer" => control.patch(BRANCH, &[0xff, 0x25, 0, 0x10, 0x10, 0]),
            "writable" => control.section_header(
                ".rodata",
                8,
                object::elf::SHF_ALLOC | object::elf::SHF_WRITE,
            ),
            "short" => control.section_header(".rodata", 20, 12),
            "invalid" => control.word(0x101004, 0x200000),
            _ => unreachable!(),
        }
        assert_whole_candidate_refused(&control, &[ENTRY, DEFAULT], &[branch]);
    }
}

#[test]
fn initial_bypasses_and_late_edges_or_function_claims_discard_the_whole_candidate() {
    let initial_seed = PatchedFixture::new("alternate_entry");
    assert_whole_candidate_refused(&initial_seed, &[ENTRY, BRANCH, DEFAULT], &[BRANCH]);

    let mut initial_jump = PatchedFixture::new("initial_bypass");
    initial_jump.relative(GAP, 0xe9, BRANCH);
    assert_whole_candidate_refused(&initial_jump, &[ENTRY, GAP, DEFAULT], &[BRANCH]);

    for variant in ["late_bypass", "late_case_call"] {
        let mut control = PatchedFixture::new(variant);
        if variant == "late_bypass" {
            control.patch(CASES[0], &[0xb8, 4, 0, 0, 0]);
            control.relative(CASES[0] + 5, 0xe9, BRANCH);
        } else {
            control.relative(CASES[0], 0xe8, CASES[1]);
            control.patch(CASES[0] + 5, &[0xc3]);
        }
        assert_whole_candidate_refused(&control, &[ENTRY, DEFAULT], &[BRANCH]);
    }
}

#[test]
fn a_case_one_byte_into_another_cases_instruction_discards_the_whole_candidate() {
    let mut control = PatchedFixture::new("overlapping_case");
    control.patch(CASES[0], &[0xb8]);
    control.relative(CASES[0] + 1, 0xe8, INDEPENDENT);
    control.patch(CASES[0] + 6, &[0xc0, 0xc3, 0x90, 0xc3]);
    control.word(0x101004, CASES[0] + 1);
    assert_whole_candidate_refused(&control, &[ENTRY, DEFAULT], &[BRANCH]);
}

fn guarded_jump(
    control: &mut PatchedFixture,
    guard: u64,
    register: u8,
    table: u64,
    default: u64,
) -> u64 {
    control.patch(guard, &[0x83, 0xf8 + register, 3, 0x0f, 0x87]);
    let displacement = i32::try_from(default as i64 - guard as i64 - 9).unwrap();
    control.patch(guard + 5, &displacement.to_le_bytes());
    control.patch(guard + 9, &[0xff, 0x24, 0x85 + register * 8]);
    control.word(guard + 12, table);
    guard + 9
}

fn constant_return(control: &mut PatchedFixture, address: u64, value: u8) {
    control.patch(address, &[0xb8, value, 0, 0, 0, 0xc3]);
}

#[test]
fn a_computed_bypass_rolls_back_an_independently_admitted_partner_switch() {
    let mut control = PatchedFixture::new("computed_bypass");
    let partner = 0x100200;
    let table = 0x100300;
    control.patch(partner, &[0x8b, 0x4c, 0x24, 4, 0xb8, 4, 0, 0, 0]);
    let branch = guarded_jump(&mut control, partner + 9, 1, table, 0x1002c0);
    constant_return(&mut control, 0x1002c0, 98);
    control.word(table, BRANCH);
    for (index, address) in [0x100260, 0x100280, 0x1002a0].into_iter().enumerate() {
        constant_return(&mut control, address, 67 + index as u8);
        control.word(table + 4 * (index as u64 + 1), address);
    }
    let mut independent = PatchedFixture::new("independent_partner");
    independent.bytes.clone_from(&control.bytes);
    constant_return(&mut independent, 0x100240, 66);
    independent.word(table, 0x100240);
    let loaded = Loaded::from_path(independent.save(), true);
    for detail in [ListingDetail::FULL, ListingDetail::PARTITION_ONLY] {
        let accepted = loaded.with_seeds(detail, &WalkPlan::serial(), &[partner, DISPATCH], &[]);
        for target in CASES
            .into_iter()
            .chain([0x100240, 0x100260, 0x100280, 0x1002a0])
        {
            assert!(
                accepted.is_instruction_start(target),
                "independent proof did not admit {target:#x}"
            );
        }
        assert!(accepted.function_at(HELPER).is_some());
    }
    assert_whole_candidate_refused(&control, &[partner, DISPATCH], &[branch, BRANCH]);
}

#[test]
fn sixteen_nested_tables_publish_and_a_seventeenth_discards_every_expansion() {
    for depth in [2u64, 16, 17] {
        let mut control = PatchedFixture::new("nested_switches");
        control.clear_text();
        control.patch(ENTRY, &[0x8b, 0x44, 0x24, 4]);
        control.relative(ENTRY + 4, 0xe9, ENTRY + 0x10);
        let terminal = ENTRY + 0x180;
        let end = ENTRY + 0x190;
        let default = ENTRY + 0x1a0;
        constant_return(&mut control, terminal, 77);
        constant_return(&mut control, end, 22);
        constant_return(&mut control, default, 99);
        let mut branches = Vec::new();
        for index in 0..depth {
            let guard = ENTRY + 0x10 + index * 16;
            let table = ENTRY + 0x300 + index * 16;
            branches.push(guarded_jump(&mut control, guard, 0, table, default));
            control.word(
                table,
                if index + 1 == depth {
                    terminal
                } else {
                    guard + 16
                },
            );
            for slot in 1..4 {
                control.word(table + slot * 4, end);
            }
        }
        if depth == 17 {
            assert_whole_candidate_refused(&control, &[ENTRY], &branches[..1]);
            continue;
        }
        let path = control.save();
        let off = Loaded::from_path(path, false);
        let on = Loaded::from_path(path, true);
        for detail in [ListingDetail::FULL, ListingDetail::PARTITION_ONLY] {
            let ordinary = off.with_seeds(detail, &WalkPlan::serial(), &[ENTRY], &[]);
            assert!(!ordinary.is_instruction_start(terminal));
            let expanded = on.with_seeds(detail, &WalkPlan::serial(), &[ENTRY], &[]);
            for &branch in &branches {
                assert!(expanded.is_instruction_start(branch));
            }
            assert!(expanded.is_instruction_start(terminal));
            assert!(expanded.is_instruction_start(end));
            let plan = WalkPlan::for_lanes(on.prog.arch(), 3, 4, 0);
            let parallel = on.with_seeds(detail, &plan, &[ENTRY], &[]);
            assert!(plan.engaged() > 0);
            assert_same_listing(&parallel, &expanded);
        }
    }
}

fn decompiled_fixture_function(address: u64, enabled: bool) -> String {
    use kuna_console::ifacedecomp::{
        execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
    };
    use kuna_console::ifaceterm::ConsoleCommands;

    let mut loaded = Loaded::new(enabled);
    let arch = loaded.prog.arch_mut();
    arch.apply_mode("aggressive")
        .expect("apply aggressive mode");
    arch.set_kuna_option("fast_funcdisc", "off").unwrap();
    arch.set_kuna_option("listing", "on").unwrap();
    arch.set_kuna_option("x86listingswitch", if enabled { "on" } else { "off" })
        .unwrap();
    loaded
        .prog
        .commit_pending_analysis()
        .expect("commit fixture discovery");
    let commands = vec![
        format!("load addr {address:#x} sub_{address:x}"),
        "decompile".into(),
        "print C".into(),
    ];
    let count = commands.len();
    let mut status = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut status);
    let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
    data.as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap()
        .conf = Some(loaded.prog);
    for index in 0..count {
        if index + 1 == count {
            status.optr.clear();
        }
        execute(&mut status);
    }
    status.optr.clone()
}

#[test]
fn emitted_c_preserves_every_case_default_and_the_case_helper_side_effect() {
    use std::process::Command;

    let helper = decompiled_fixture_function(HELPER, true);
    let bytes = std::fs::read(fixture()).unwrap();
    let file = object::File::parse(&*bytes).unwrap();
    let marker = format!("dat_{:x}", file.section_by_name(".data").unwrap().address());
    let available = match Command::new("gcc").arg("--version").output() {
        Ok(output) => {
            assert!(output.status.success());
            true
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => panic!("cannot run gcc: {error}"),
    };
    for enabled in [false, true] {
        let dispatcher = decompiled_fixture_function(DISPATCH, enabled);
        if !available {
            assert!(
                dispatcher.contains("switch(") && dispatcher.contains("sub_100510()"),
                "{dispatcher}"
            );
            continue;
        }
        let source = common::scratch_file("x86listingswitch-emitted", "c");
        let code = format!(
            r#"
unsigned int {marker};
{helper}
{dispatcher}
int main(void) {{
    unsigned int inputs[] = {{0, 1, 2, 3, 4, 0xffffffffu, 0x80000000u}};
    unsigned int expected[] = {{11, 22, 33, 44, 0xfffffff9u, 0xfffffff9u, 0xfffffff9u}};
    for (unsigned int i = 0; i < sizeof(inputs) / sizeof(inputs[0]); ++i) {{
        {marker} = 0;
        if ((unsigned int)sub_100400(inputs[i]) != expected[i]) return 1;
        if ({marker} != (inputs[i] == 2)) return 2;
    }}
    return 0;
}}
"#
        );
        std::fs::write(&source, &code).unwrap();
        for optimization in ["-O0", "-O2"] {
            let executable = source.with_extension(&optimization[1..]);
            let output = Command::new("gcc")
                .args([
                    "-std=c11",
                    optimization,
                    "-Werror=implicit-function-declaration",
                ])
                .arg(&source)
                .arg("-o")
                .arg(&executable)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}\n{code}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert!(
                Command::new(&executable).status().unwrap().success(),
                "{code}"
            );
            std::fs::remove_file(executable).unwrap();
        }
        std::fs::remove_file(source).unwrap();
    }
    if !available {
        eprintln!("gcc absent; emitted-C execution skipped");
    }
}
