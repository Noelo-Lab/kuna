//! UEFI TE image bootstrap and address-translation regressions.

mod common;

use std::path::PathBuf;

use kuna_analysis::listing::kuna_entrythumbflow::MAX_INSTRUCTIONS;
use kuna_analysis::loadimage_te::synthetic::{TeImage, CODE_RVA};
use kuna_base::address::Address;
use kuna_console::assertions::{self, Body, Directive};
use kuna_console::engine::{
    bootstrap_from_file, bootstrap_from_te, ArmIsa, ConsoleProgram, EntrySelector,
};
use kuna_console::project::decompile_targets;

struct TeFixture(PathBuf);

impl TeFixture {
    fn thumb_return_7() -> Self {
        Self::write("te-thumb", TeImage::thumb(&[0x07, 0x20, 0x70, 0x47]).build())
    }

    fn mixed_arm_with_odd_thumb_entry() -> Self {
        let image = TeImage::arm(&[0x1e, 0xff, 0x2f, 0xe1, 0x07, 0x20, 0x70, 0x47])
            .entry_rva(CODE_RVA + 5);
        Self::write("te-mixed-arm-thumb", image.build())
    }

    fn thumb_entry_before_arm() -> Self {
        let image = TeImage::arm(&[0x07, 0x20, 0x70, 0x47, 0x1e, 0xff, 0x2f, 0xe1])
            .entry_rva(CODE_RVA | 1);
        Self::write("te-thumb-entry-before-arm", image.build())
    }

    fn thumb_entry_overlaid_with_branch() -> Self {
        let image = TeImage::arm(&[
            0x70, 0x47, 0x00, 0xbf, 0x00, 0xbf, 0x00, 0xbf, 0x07, 0x20, 0x70, 0x47,
        ])
        .entry_rva(CODE_RVA | 1);
        Self::write("te-thumb-entry-overlay-branch", image.build())
    }

    fn mixed_machine_thumb2_and_arm() -> Self {
        let image = TeImage::thumb(&[
            0x1e, 0xff, 0x2f, 0xe1, 0x4f, 0xf0, 0x07, 0x00, 0x70, 0x47,
        ])
        .entry_rva(CODE_RVA + 5);
        Self::write("te-armthumb-mixed", image.build())
    }

    fn thumb_calls_arm() -> Self {
        let image = TeImage::arm(&[
            0x00, 0xf0, 0x02, 0xf8, 0x70, 0x47, 0x00, 0xbf, 0x00, 0xf0, 0x02, 0xe8, 0x70,
            0x47, 0x00, 0xbf, 0x07, 0x00, 0xa0, 0xe3, 0x1e, 0xff, 0x2f, 0xe1,
        ])
        .entry_rva(CODE_RVA | 1);
        Self::write("te-thumb-calls-arm", image.build())
    }

    /// `MAX_INSTRUCTIONS + 1` Thumb `nop`s then `bx lr`: one instruction more
    /// than the walk's budget.
    /// `blx` to an A32 helper that sits BELOW a Thumb callee reached by `bl`:
    /// the `blx` decode's own `globalset(TMode=0)` at its target would, if it
    /// reached the database, flatten every Thumb address above the helper.
    fn thumb_blx_below_thumb_callee() -> Self {
        let image = TeImage::arm(&[
            0x00, 0xf0, 0x06, 0xe8, // 0x1000 blx 0x1010 (A32 helper)
            0x00, 0xf0, 0x08, 0xf8, // 0x1004 bl  0x1018 (Thumb callee)
            0x70, 0x47, 0x00, 0xbf, 0x00, 0xbf, 0x00, 0xbf, // 0x1008 bx lr; nops
            0x07, 0x00, 0xa0, 0xe3, 0x1e, 0xff, 0x2f, 0xe1, // 0x1010 A32: mov r0,#7; bx lr
            0x07, 0x20, 0x00, 0xbf, 0x70, 0x47, // 0x1018 Thumb: movs r0,#7; nop; bx lr
        ])
        .entry_rva(CODE_RVA | 1);
        Self::write("te-thumb-blx-below-callee", image.build())
    }

    /// Two Thumb no-ops that fall through into a zero-filled virtual tail.
    fn thumb_entry_into_zero_tail() -> Self {
        let mut bytes = TeImage::arm(&[0x00, 0xbf, 0x00, 0xbf]).entry_rva(CODE_RVA | 1).build();
        bytes[48..52].copy_from_slice(&0x20u32.to_le_bytes());
        Self::write("te-thumb-zero-tail", bytes)
    }

    fn long_thumb_entry() -> Self {
        let mut code = Vec::with_capacity((MAX_INSTRUCTIONS + 1) * 2 + 2);
        for _ in 0..=MAX_INSTRUCTIONS {
            code.extend_from_slice(&[0xc0, 0x46]);
        }
        code.extend_from_slice(&[0x70, 0x47]);
        Self::write("te-long-thumb-entry", TeImage::arm(&code).entry_rva(CODE_RVA | 1).build())
    }

    fn arm_literal_pool_return_7() -> Self {
        let mut code = vec![0x00, 0x00, 0x9f, 0xe5, 0x1e, 0xff, 0x2f, 0xe1];
        code.extend_from_slice(&7u32.to_le_bytes());
        Self::write("te-arm-literal-pool", TeImage::arm(&code).build())
    }

    fn write(stem: &str, bytes: Vec<u8>) -> Self {
        let path = common::scratch_file(stem, "te");
        std::fs::write(&path, bytes).unwrap();
        Self(path)
    }
}

impl Drop for TeFixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn specs() -> Vec<String> {
    vec![std::env::var("KUNA_SPECS")
        .unwrap_or_else(|_| repo_root().join("specs").to_string_lossy().into_owned())]
}

/// Bootstrap a TE fixture, or skip when the ARM/x86 `.sla` has not been built.
///
/// Only a missing SLEIGH specification is a skip: every other bootstrap error
/// is a real regression and panics, so a broken load can never read as a green
/// specs-less run.
fn load_or_skip(path: &str, target: &str, isa: Option<ArmIsa>) -> Option<ConsoleProgram> {
    match bootstrap_from_te(path, target, &specs(), isa) {
        Ok(program) => Some(program),
        Err(error) if error.explain().contains("No sleigh specification") => {
            eprintln!(
                "verify_te_image: skipping (build the `.sla` with `make specs`): {}",
                error.explain()
            );
            None
        }
        Err(error) => panic!("TE bootstrap failed: {}", error.explain()),
    }
}

#[test]
fn thumb_te_dispatches_maps_and_decompiles() {
    let fixture = TeFixture::thumb_return_7();
    let path = fixture.0.to_string_lossy();
    let mut program = match bootstrap_from_file(&path, "", &specs()) {
        Ok(program) => program,
        Err(error) if error.explain().contains("No sleigh specification") => {
            eprintln!(
                "verify_te_image: skipping (build the `.sla` with `make specs`): {}",
                error.explain()
            );
            return;
        }
        Err(error) => panic!("TE dispatch from `load file` failed: {}", error.explain()),
    };
    program.commit_pending_analysis().unwrap();
    assert_eq!(
        program.sections(),
        vec![(0x4001b0, 0xe50, 24), (0x401000, 4, 20)]
    );
    assert!(program.vma_bytes_mapped(0x4001b0));
    let odd = program
        .find_entry_at(0x401001)
        .expect("odd TE entry must resolve");
    let even = program
        .find_entry_at(0x401000)
        .expect("normalized TE entry must resolve");
    assert_eq!(odd.addr.get_offset(), 0x401000);
    assert_eq!(even.addr.get_offset(), 0x401000);
    assert!(program.entry_bytes_mapped(&even.addr));

    let results = decompile_targets(&mut program, vec![odd], true, false, false);
    assert_eq!(results.len(), 1);
    assert!(results[0].error.is_none(), "{:?}", results[0].error);
    assert!(
        results[0].code.as_deref().unwrap().contains("return 7;"),
        "unexpected TE Thumb body:\n{}",
        results[0].code.as_deref().unwrap()
    );
}

#[test]
fn explicit_target_keeps_te_mappings() {
    let fixture = TeFixture::thumb_return_7();
    let path = fixture.0.to_string_lossy();
    let Some(mut program) = load_or_skip(&path, "ARM:LE:32:v4t:default", None) else {
        return;
    };
    program.commit_pending_analysis().unwrap();
    assert!(program.find_entry_at(0x401001).is_some());
    assert!(program.vma_bytes_mapped(0x401000));
    assert!(!program.vma_bytes_mapped(0x400000));
}

#[test]
fn armthumb_mixed_uses_thumb2_without_painting_a32_code() {
    let fixture = TeFixture::mixed_machine_thumb2_and_arm();
    let path = fixture.0.to_string_lossy();
    let Some(mut program) = load_or_skip(&path, "", None) else {
        return;
    };
    program.commit_pending_analysis().unwrap();
    assert!(program.description().starts_with("ARM:LE:32:v8:default"));

    let arm = program
        .resolve_entry(&EntrySelector::Numeric(0x401000))
        .expect("A32 function");
    let thumb = program.find_entry_at(0x401005).expect("odd Thumb-2 entry");
    assert_eq!(
        program
            .arch()
            .with_context_db_mut(|db| db.get_variable_value(b"TMode", &arm.addr))
            .unwrap(),
        0
    );
    assert_eq!(
        program
            .arch()
            .with_context_db_mut(|db| db.get_variable_value(b"TMode", &thumb.addr))
            .unwrap(),
        1
    );

    let results = decompile_targets(&mut program, vec![arm, thumb], true, false, false);
    for result in &results {
        assert!(result.error.is_none(), "{:?}", result.error);
    }
    assert!(
        results
            .iter()
            .find(|result| result.address == 0x401000)
            .unwrap()
            .code
            .as_deref()
            .unwrap()
            .contains("return;")
    );
    assert!(
        results
            .iter()
            .find(|result| result.address == 0x401004)
            .unwrap()
            .code
            .as_deref()
            .unwrap()
            .contains("return 7;")
    );
}

#[test]
fn odd_thumb_entry_carries_mode_across_bl_but_not_blx() {
    let fixture = TeFixture::thumb_calls_arm();
    let path = fixture.0.to_string_lossy();
    let Some(mut program) = load_or_skip(&path, "ARM:LE:32:v8:default", None) else {
        return;
    };
    program.commit_pending_analysis().unwrap();

    let entry = program.find_entry_at(0x401001).expect("odd Thumb entry");
    let thumb_helper = program
        .resolve_entry(&EntrySelector::Numeric(0x401008))
        .expect("Thumb BL target");
    let arm_helper = program
        .resolve_entry(&EntrySelector::Numeric(0x401010))
        .expect("A32 BLX target");
    assert_eq!(
        program
            .arch()
            .with_context_db_mut(|db| db.get_variable_value(b"TMode", &thumb_helper.addr))
            .unwrap(),
        1,
        "BL preserves Thumb state at its direct target"
    );
    assert_eq!(
        program
            .arch()
            .with_context_db_mut(|db| db.get_variable_value(b"TMode", &arm_helper.addr))
            .unwrap(),
        0,
        "BLX interworks to A32"
    );

    let results = decompile_targets(
        &mut program,
        vec![entry, thumb_helper, arm_helper],
        true,
        false,
        false,
    );
    for result in &results {
        assert!(result.error.is_none(), "{:?}", result.error);
    }
    assert!(
        results
            .iter()
            .find(|result| result.address == 0x401010)
            .unwrap()
            .code
            .as_deref()
            .unwrap()
            .contains("return 7;")
    );
}

fn tmode_at(program: &kuna_console::engine::ConsoleProgram, offset: u64) -> u32 {
    let space = program.arch().manage().get_default_code_space().cloned().unwrap();
    program
        .arch()
        .with_context_db_mut(|db| db.get_variable_value(b"TMode", &Address::new(space, offset)))
        .unwrap()
}

#[test]
fn it_guarded_branch_preserves_both_return_paths() {
    assert_it_return_paths(&[
        0x00, 0x28, // cmp r0,#0
        0x08, 0xbf, // it eq
        0x01, 0xe0, // beq 0x40100a
        0x07, 0x20, 0x70, 0x47, // movs r0,#7; bx lr
        0x00, 0x20, 0x70, 0x47, // movs r0,#0; bx lr
    ]);
}

#[test]
fn it_guarded_return_preserves_fall_through() {
    assert_it_return_paths(&[
        0x00, 0x28, // cmp r0,#0
        0x08, 0xbf, // it eq
        0x70, 0x47, // bxeq lr
        0x07, 0x20, 0x70, 0x47, // movs r0,#7; bx lr
    ]);
}

fn assert_it_return_paths(code: &[u8]) {
    let fixture = TeFixture::write(
        "te-thumb-it",
        TeImage::arm(code).entry_rva(CODE_RVA | 1).build(),
    );
    let path = fixture.0.to_string_lossy();
    let mut outputs = Vec::new();
    for isa in [None, Some(ArmIsa::Thumb)] {
        let Some(mut program) = load_or_skip(&path, "", isa) else {
            return;
        };
        program.commit_pending_analysis().unwrap();
        assert_eq!(tmode_at(&program, 0x401006), 1, "the conditional fall-through is Thumb");
        assert_eq!(tmode_at(&program, 0x401008), 1, "the fall-through return is Thumb");
        let entry = program.find_entry_at(0x401001).unwrap();
        let results = decompile_targets(&mut program, vec![entry], true, false, false);
        assert!(results[0].error.is_none(), "{:?}", results[0].error);
        let code = results[0].code.clone().unwrap();
        assert!(code.contains("if (") && code.contains("return 7;"), "{code}");
        outputs.push(code);
    }
    assert_eq!(outputs[0], outputs[1], "entry-local and explicit Thumb must agree");
}

/// The walk's budget is a bound on its own work, not on the load: the ranges
/// it did walk are painted, the load succeeds, and the code past the bound is
/// left at the language default. The paint is one context region, not one
/// split point per instruction, and a second commit does not re-run the walk.
#[test]
fn odd_thumb_entry_walk_truncates_at_its_budget_and_keeps_the_load_usable() {
    let fixture = TeFixture::long_thumb_entry();
    let path = fixture.0.to_string_lossy();
    let Some(mut program) = load_or_skip(&path, "", None) else {
        return;
    };
    program
        .commit_pending_analysis()
        .expect("an exhausted Thumb context walk must not fail the load");

    let walked_end = 0x401000 + 2 * MAX_INSTRUCTIONS as u64;
    assert_eq!(tmode_at(&program, 0x401000), 1);
    assert_eq!(tmode_at(&program, walked_end - 2), 1, "the last walked instruction is Thumb");
    assert_eq!(tmode_at(&program, walked_end), 0, "past the budget decodes as A32");
    assert_eq!(tmode_at(&program, walked_end + 2), 0);

    let space = program.arch().manage().get_default_code_space().cloned().unwrap();
    let (first, last) = program.arch().with_context_db_mut(|db| {
        let (_, first, last) = db.get_context_bounds(&Address::new(space, 0x401000));
        (first, last)
    });
    assert_eq!(
        (first, last),
        (0x401000, walked_end - 1),
        "the walked ranges must be painted as one context region, not per instruction"
    );

    program
        .commit_pending_analysis()
        .expect("a second commit must not re-run the walk");
    assert_eq!(tmode_at(&program, walked_end), 0);
    let entry = program.find_entry_at(0x401001).expect("odd Thumb entry");
    assert_eq!(entry.addr.get_offset(), 0x401000);

    let mut explicit = bootstrap_from_te(&path, "", &specs(), Some(ArmIsa::Thumb))
        .unwrap_or_else(|error| {
            panic!(
                "explicit Thumb mode should bypass the walk: {}",
                error.explain()
            )
        });
    explicit.commit_pending_analysis().unwrap();
    assert_eq!(tmode_at(&explicit, walked_end + 2), 1, "--isa thumb paints the whole image");
}

/// The `blx` decode runs the language's `globalset(TMode=0)` at its target.
/// With the helper below the Thumb callee, letting that write reach the
/// database during the walk flattens the callee to A32; the walk decodes with
/// context sets suppressed, so the callee keeps its mode.
#[test]
fn a_blx_target_below_a_thumb_callee_does_not_flatten_it() {
    let fixture = TeFixture::thumb_blx_below_thumb_callee();
    let path = fixture.0.to_string_lossy();
    let Some(mut program) = load_or_skip(&path, "ARM:LE:32:v8:default", None) else {
        return;
    };
    program.commit_pending_analysis().unwrap();
    assert_eq!(tmode_at(&program, 0x401010), 0, "the BLX target is A32");
    assert_eq!(tmode_at(&program, 0x401018), 1, "the BL callee is Thumb");
    assert_eq!(tmode_at(&program, 0x40101c), 1, "and so is its second instruction");
    let callee = program
        .resolve_entry(&EntrySelector::Numeric(0x401018))
        .expect("Thumb callee");
    let results = decompile_targets(&mut program, vec![callee], true, false, false);
    assert!(results[0].error.is_none(), "{:?}", results[0].error);
    assert!(
        results[0].code.as_deref().unwrap().contains("return 7;"),
        "{:?}",
        results[0].code
    );
}

/// The walk covers file-backed bytes only: a zero-filled tail decodes as a
/// run of Thumb no-ops and would otherwise be marched through and painted.
#[test]
fn the_walk_stops_at_the_file_backed_bytes() {
    let fixture = TeFixture::thumb_entry_into_zero_tail();
    let path = fixture.0.to_string_lossy();
    let Some(mut program) = load_or_skip(&path, "", None) else {
        return;
    };
    program.commit_pending_analysis().unwrap();
    assert_eq!(tmode_at(&program, 0x401002), 1, "the second no-op is walked");
    assert_eq!(tmode_at(&program, 0x401004), 0, "the zero tail is not");
    assert_eq!(tmode_at(&program, 0x401010), 0);
}

/// Two-pass gate for `entrythumbflow`: off, no context is painted from the
/// entry bit and the walk never touches the context database.
#[test]
fn entrythumbflow_off_leaves_the_entry_at_the_language_default() {
    let fixture = TeFixture::thumb_entry_before_arm();
    let path = fixture.0.to_string_lossy();
    let Some(mut program) = load_or_skip(&path, "", None) else {
        return;
    };
    program
        .arch_mut()
        .set_kuna_option("entrythumbflow", "off")
        .unwrap();
    program.commit_pending_analysis().unwrap();
    assert_eq!(tmode_at(&program, 0x401000), 0);
    assert_eq!(tmode_at(&program, 0x401002), 0);
    assert_eq!(tmode_at(&program, 0x401004), 0);
    let space = program.arch().manage().get_default_code_space().cloned().unwrap();
    let (first, last) = program.arch().with_context_db_mut(|db| {
        let (_, first, last) = db.get_context_bounds(&Address::new(space, 0x401000));
        (first, last)
    });
    assert_eq!((first, last), (0, u32::MAX as u64), "off must leave the context partition untouched");
    assert!(program.find_entry_at(0x401001).is_some());
}

#[test]
fn odd_generic_arm_entry_does_not_paint_the_whole_code_section_thumb() {
    let fixture = TeFixture::mixed_arm_with_odd_thumb_entry();
    let path = fixture.0.to_string_lossy();
    let Some(mut program) = load_or_skip(&path, "", None) else {
        return;
    };
    program.commit_pending_analysis().unwrap();

    let arm = program
        .resolve_entry(&EntrySelector::Numeric(0x401000))
        .expect("mapped A32 function before the entry");
    let thumb = program
        .find_entry_at(0x401005)
        .expect("odd TE entry must normalize and remain Thumb");
    let results = decompile_targets(&mut program, vec![arm, thumb], true, false, false);
    let arm = results
        .iter()
        .find(|result| result.address == 0x401000)
        .expect("A32 result");
    assert!(arm.error.is_none(), "{:?}", arm.error);
    assert!(
        arm.code.as_deref().unwrap().contains("return;"),
        "{:?}",
        arm.code
    );
    let thumb = results
        .iter()
        .find(|result| result.address == 0x401004)
        .expect("Thumb result");
    assert!(thumb.error.is_none(), "{:?}", thumb.error);
    assert!(
        thumb.code.as_deref().unwrap().contains("return 7;"),
        "{:?}",
        thumb.code
    );
}

#[test]
fn odd_generic_arm_entry_does_not_paint_later_code_thumb() {
    let fixture = TeFixture::thumb_entry_before_arm();
    let path = fixture.0.to_string_lossy();
    let Some(mut program) = load_or_skip(&path, "", None) else {
        return;
    };
    program.commit_pending_analysis().unwrap();

    let thumb = program.find_entry_at(0x401001).expect("odd Thumb entry");
    let arm = program
        .resolve_entry(&EntrySelector::Numeric(0x401004))
        .expect("later A32 function");
    assert_eq!(
        program
            .arch()
            .with_context_db_mut(|db| db.get_variable_value(b"TMode", &thumb.addr))
            .unwrap(),
        1
    );
    let thumb_continuation = kuna_base::address::Address::new(
        thumb.addr.get_space().cloned().unwrap(),
        0x401002,
    );
    assert_eq!(
        program
            .arch()
            .with_context_db_mut(|db| db.get_variable_value(b"TMode", &thumb_continuation))
            .unwrap(),
        1,
        "reachable Thumb instructions retain entry mode"
    );
    assert_eq!(
        program
            .arch()
            .with_context_db_mut(|db| db.get_variable_value(b"TMode", &arm.addr))
            .unwrap(),
        0,
        "entry-derived Thumb state must not reach later code"
    );
    let results = decompile_targets(&mut program, vec![thumb, arm], true, false, false);
    let thumb = results
        .iter()
        .find(|result| result.address == 0x401000)
        .expect("Thumb result");
    assert!(thumb.error.is_none(), "{:?}", thumb.error);
    assert!(
        thumb.code.as_deref().unwrap().contains("return 7;"),
        "{:?}",
        thumb.code
    );
    let arm = results
        .iter()
        .find(|result| result.address == 0x401004)
        .expect("A32 result");
    assert!(arm.error.is_none(), "{:?}", arm.error);
    assert!(arm.code.as_deref().unwrap().contains("return;"), "{:?}", arm.code);
}

#[test]
fn entry_thumb_walk_decodes_overlaid_bytes() {
    let fixture = TeFixture::thumb_entry_overlaid_with_branch();
    let path = fixture.0.to_string_lossy();
    let Some(mut program) = load_or_skip(&path, "", None) else {
        return;
    };
    program.set_assertions(vec![Directive {
        raw: "bytes 0x401000 02e0".to_string(),
        body: Body::Bytes {
            addr: 0x401000,
            data: vec![0x02, 0xe0],
        },
    }]);
    assertions::apply_image_scoped(&mut program);
    program.commit_pending_analysis().unwrap();
    assert_eq!(program.assertion_outcomes()[0].status, "applied");

    let entry = program.find_entry_at(0x401001).expect("odd Thumb entry");
    let target = program
        .resolve_entry(&EntrySelector::Numeric(0x401008))
        .expect("overlaid Thumb branch target");
    assert_eq!(
        program
            .arch()
            .with_context_db_mut(|db| db.get_variable_value(b"TMode", &target.addr))
            .unwrap(),
        1,
        "the entry walk must follow the overlaid branch"
    );
    let results = decompile_targets(&mut program, vec![entry], true, false, false);
    assert!(results[0].error.is_none(), "{:?}", results[0].error);
    assert!(
        results[0].code.as_deref().unwrap().contains("return 7;"),
        "{:?}",
        results[0].code
    );
}

#[test]
fn entry_thumb_walk_reaches_a_materialized_virtual_tail() {
    let mut bytes = TeImage::arm(&[0x00, 0xbf, 0x00, 0xbf]).entry_rva(CODE_RVA | 1).build();
    bytes[48..52].copy_from_slice(&16u32.to_le_bytes());
    let fixture = TeFixture::write("te-thumb-tail-overlay", bytes);
    let path = fixture.0.to_string_lossy();
    let Some(mut program) = load_or_skip(&path, "", None) else {
        return;
    };
    apply_byte_assertions(&mut program, &[
        (0x401000, &[0x02, 0xe0]),
        (0x401008, &[0x07, 0x20, 0x70, 0x47]),
    ]);
    program.commit_pending_analysis().unwrap();
    assert_eq!(tmode_at(&program, 0x401008), 1, "materialized branch target");
    assert_eq!(tmode_at(&program, 0x40100a), 1, "materialized return");
    for offset in [0x401002, 0x401004, 0x401006, 0x40100c, 0x40100e] {
        assert_eq!(tmode_at(&program, offset), 0, "unreached or uninitialized: {offset:x}");
    }
    assert_entry_returns_7(&mut program);
}

#[test]
fn entry_thumb_walk_merges_adjacent_and_overlapping_materialized_spans() {
    let mut bytes = TeImage::arm(&[0x4f, 0xf0]).entry_rva(CODE_RVA | 1).build();
    bytes[48..52].copy_from_slice(&16u32.to_le_bytes());
    let fixture = TeFixture::write("te-thumb-split-overlay", bytes);
    let path = fixture.0.to_string_lossy();
    let Some(mut program) = load_or_skip(&path, "", None) else {
        return;
    };
    apply_byte_assertions(&mut program, &[
        (0x401004, &[0x70, 0x47]),
        (0x401003, &[0x00]),
        (0x401001, &[0xf0, 0x07]),
    ]);
    program.commit_pending_analysis().unwrap();
    assert_eq!(tmode_at(&program, 0x401000), 1, "mov.w spans the file and two overlays");
    assert_eq!(tmode_at(&program, 0x401004), 1);
    assert_eq!(tmode_at(&program, 0x401006), 0, "uninitialized tail");
    assert_entry_returns_7(&mut program);
}

#[test]
fn entry_thumb_walk_excludes_uninitialized_gaps_and_rejected_overlays() {
    let mut bytes = TeImage::arm(&[0x02, 0xe0]).entry_rva(CODE_RVA | 1).build();
    bytes[48..52].copy_from_slice(&16u32.to_le_bytes());
    let fixture = TeFixture::write("te-thumb-incomplete-overlay", bytes);
    let path = fixture.0.to_string_lossy();
    let Some(mut program) = load_or_skip(&path, "", None) else {
        return;
    };
    apply_byte_assertions(&mut program, &[
        (0x401008, &[0x4f, 0xf0]),
        (0x40100c, &[0x70, 0x47]),
    ]);
    program.set_assertions(vec![Directive {
        raw: "bytes 0x40100a 0700704700000000".into(),
        body: Body::Bytes { addr: 0x40100a, data: vec![0x07, 0x00, 0x70, 0x47, 0, 0, 0, 0] },
    }]);
    assertions::apply_image_scoped(&mut program);
    assert_eq!(program.assertion_outcomes()[0].status, "rejected");
    program.commit_pending_analysis().unwrap();
    assert_eq!(tmode_at(&program, 0x401000), 1);
    assert_eq!(tmode_at(&program, 0x401008), 0, "incomplete Thumb-2 instruction");
    assert_eq!(tmode_at(&program, 0x40100a), 0, "rejected bytes did not fill the gap");
    assert_eq!(tmode_at(&program, 0x40100c), 0, "unreachable return beyond the gap");
}

#[test]
fn entry_thumb_walk_excludes_overlays_in_nonexecutable_mappings() {
    let fixture = TeFixture::write(
        "te-thumb-data-overlay",
        TeImage::arm(&[0xfa, 0xe7]).entry_rva(CODE_RVA | 1).build(),
    );
    let path = fixture.0.to_string_lossy();
    let Some(mut program) = load_or_skip(&path, "", None) else {
        return;
    };
    apply_byte_assertions(&mut program, &[(0x400ff8, &[0x07, 0x20, 0x70, 0x47])]);
    program.commit_pending_analysis().unwrap();
    assert_eq!(tmode_at(&program, 0x401000), 1);
    assert_eq!(tmode_at(&program, 0x400ff8), 0, "the branch target is not executable");
}

fn apply_byte_assertions(program: &mut ConsoleProgram, overlays: &[(u64, &[u8])]) {
    program.set_assertions(overlays.iter().map(|&(addr, data)| {
        let hex = data.iter().map(|b| format!("{b:02x}")).collect::<String>();
        Directive {
            raw: format!("bytes 0x{addr:x} {hex}"),
            body: Body::Bytes { addr, data: data.to_vec() },
        }
    }).collect());
    assertions::apply_image_scoped(program);
    assert!(program.assertion_outcomes().iter().all(|outcome| outcome.status == "applied"));
}

fn assert_entry_returns_7(program: &mut ConsoleProgram) {
    let entry = program.find_entry_at(0x401001).unwrap();
    let results = decompile_targets(program, vec![entry], true, false, false);
    assert!(results[0].error.is_none(), "{:?}", results[0].error);
    assert!(results[0].code.as_deref().unwrap().contains("return 7;"), "{:?}", results[0].code);
}

#[test]
fn explicit_non_arm_target_ignores_thumb_machine_hint() {
    let fixture = TeFixture::thumb_return_7();
    let path = fixture.0.to_string_lossy();
    let Some(program) = load_or_skip(&path, "x86:LE:32:default:gcc", None) else {
        return;
    };
    assert!(program.description().starts_with("x86:LE:32"));
    assert!(program.vma_bytes_mapped(0x401000));
}

#[test]
fn arm_te_literal_pool_folds_only_with_litpoolconst() {
    let fixture = TeFixture::arm_literal_pool_return_7();
    let path = fixture.0.to_string_lossy();
    let Some(mut enabled) = load_or_skip(&path, "", None) else {
        return;
    };
    assert_eq!(
        enabled.arch().litpool_const.as_slice(),
        &[(0x401000, 0x40100b)]
    );
    assert!(!enabled.arch().readonlypropagate);
    enabled.commit_pending_analysis().unwrap();
    let entry = enabled.find_entry_at(0x401000).expect("TE entry");
    let results = decompile_targets(&mut enabled, vec![entry], true, false, false);
    assert!(results[0].error.is_none(), "{:?}", results[0].error);
    let enabled_c = results[0].code.as_deref().unwrap();
    assert!(enabled_c.contains("return 7;"), "{enabled_c}");
    assert!(!enabled_c.contains("dat_401008"), "{enabled_c}");

    let mut disabled = bootstrap_from_te(&path, "", &specs(), None).unwrap();
    disabled
        .arch_mut()
        .set_kuna_option("litpoolconst", "off")
        .unwrap();
    disabled.commit_pending_analysis().unwrap();
    let entry = disabled.find_entry_at(0x401000).expect("TE entry");
    let results = decompile_targets(&mut disabled, vec![entry], true, false, false);
    assert!(results[0].error.is_none(), "{:?}", results[0].error);
    assert!(
        results[0]
            .code
            .as_deref()
            .unwrap()
            .contains("dat_401008"),
        "{:?}",
        results[0].code
    );
}
