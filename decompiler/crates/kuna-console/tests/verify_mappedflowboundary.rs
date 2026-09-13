//! Synthetic linked ELF tests of authoritative mapping and flow-lifetime byte snapshots.
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use kuna_base::address::Address;
use kuna_console::assertions::{self, Body, Directive};
use kuna_console::engine::{ConsoleProgram, EntrySelector, bootstrap_from_object};
use kuna_console::ifacedecomp::{DECOMPILE_MODULE, IfaceDecompData, register_decomp_commands};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_console::project::decompile_targets;
use kuna_decomp::decompile_drive::build_and_follow_flow;
use kuna_decomp::flow::flow_flags;
use kuna_decomp::op::pcodeop_flags;
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use kuna_sleigh::translate::PcodeEmit;

static NEXT: AtomicU64 = AtomicU64::new(0);

fn fixture(maps: &[(u32, &[u8], u32)]) -> PathBuf {
    let original = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/mapped_flow_boundary_32.elf");
    let mut bytes = std::fs::read(original).unwrap()[..52].to_vec();
    bytes[44..46].copy_from_slice(&(maps.len() as u16).to_le_bytes());
    let mut offset = 52 + 32 * maps.len();
    for &(addr, data, memsize) in maps {
        for field in [
            1,
            offset as u32,
            addr,
            addr,
            data.len() as u32,
            memsize,
            5,
            1,
        ] {
            bytes.extend_from_slice(&field.to_le_bytes());
        }
        offset += data.len();
    }
    for &(_, data, _) in maps {
        bytes.extend_from_slice(data);
    }
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("mappedflowboundary");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!(
        "image-{}-{}.elf",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    std::fs::write(&path, bytes).unwrap();
    path
}

fn load(path: &Path) -> ConsoleProgram {
    let specs = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    bootstrap_from_object(
        path.to_str().unwrap(),
        "",
        &[specs.to_string_lossy().into_owned()],
    )
    .unwrap()
}

fn address(prog: &ConsoleProgram, offset: u64) -> Address {
    Address::new(
        prog.arch()
            .manage()
            .get_default_code_space()
            .unwrap()
            .clone(),
        offset,
    )
}

fn code(prog: &mut ConsoleProgram) -> String {
    let entry = prog
        .resolve_entry(&EntrySelector::Numeric(0x10000))
        .unwrap();
    let funcs = decompile_targets(prog, vec![entry], false, false, false);
    funcs[0]
        .code
        .clone()
        .unwrap_or_else(|| panic!("decompilation failed: {:?}", funcs[0].error))
}

#[derive(Default)]
struct Emit(Vec<(Address, OpCode)>);
impl PcodeEmit for Emit {
    fn dump(&mut self, addr: &Address, op: OpCode, _: Option<&VarnodeData>, _: &[VarnodeData]) {
        self.0.push((addr.clone(), op));
    }
}

#[test]
fn truncated_instruction_is_rejected_before_any_pcode_even_with_warm_padding() {
    let path = fixture(&[(0x10000, &[0xb8, 7, 0], 3)]);
    for warm in [false, true] {
        let prog = load(&path);
        let start = address(&prog, 0x10000);
        let tr = prog.arch().translate();
        if warm {
            tr.loader_rc()
                .borrow_mut()
                .load_fill(&mut [0; 16], &start)
                .unwrap();
        }
        let image = tr.loader_rc().borrow().shared_bytes().unwrap();
        let mut emit = Emit::default();
        let err = tr
            .one_instruction_checked(&mut emit, &start, image.as_ref())
            .unwrap_err();
        assert!(err.explain().contains("not mapped"), "{err:?}");
        assert!(
            emit.0.is_empty(),
            "unmapped instruction emitted {:?}",
            emit.0
        );
        let gap = address(&prog, 0x10003);
        assert!(
            tr.one_instruction_checked(&mut emit, &gap, image.as_ref())
                .is_err()
        );
        assert!(emit.0.is_empty());
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn touching_and_overlapping_maps_supply_complete_instructions() {
    for second in [0x10002, 0x10003] {
        let data: &[u8] = if second == 0x10002 {
            &[0, 0, 0, 0xc3]
        } else {
            &[0, 0, 0xc3]
        };
        let path = fixture(&[
            (0x10000, &[0xb8, 7, 0], 3),
            (second, data, data.len() as u32),
        ]);
        let mut prog = load(&path);
        prog.commit_pending_analysis().unwrap();
        let text = code(&mut prog);
        assert!(text.contains("return 7;"), "{text}");
        assert!(!text.contains("halt_missing"), "{text}");
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn an_instruction_spanning_an_unmapped_gap_cannot_emit_pcode() {
    let path = fixture(&[(0x10000, &[0xb8, 7, 0], 3), (0x10004, &[0, 0xc3], 2)]);
    let prog = load(&path);
    let tr = prog.arch().translate();
    let image = tr.loader_rc().borrow().shared_bytes().unwrap();
    let mut emit = Emit::default();
    assert!(
        tr.one_instruction_checked(&mut emit, &address(&prog, 0x10000), image.as_ref())
            .is_err()
    );
    assert!(emit.0.is_empty());
    std::fs::remove_file(path).unwrap();
}

#[test]
fn mapped_zero_filled_memory_is_valid_instruction_input() {
    let path = fixture(&[(0x10000, &[0], 2)]);
    let mut bytes = std::fs::read(&path).unwrap();
    bytes[76..80].copy_from_slice(&4u32.to_le_bytes());
    std::fs::write(&path, bytes).unwrap();
    let prog = load(&path);
    let tr = prog.arch().translate();
    let image = tr.loader_rc().borrow().shared_bytes().unwrap();
    let mut emit = Emit::default();
    assert_eq!(
        tr.one_instruction_checked(&mut emit, &address(&prog, 0x10000), image.as_ref())
            .unwrap(),
        2
    );
    assert!(!emit.0.is_empty());
    std::fs::remove_file(path).unwrap();
}

#[test]
fn explicit_branches_cross_gaps_to_mapped_destinations() {
    let path = fixture(&[
        (0x10000, &[0xeb, 0x2e], 2),
        (0x10030, &[0xb8, 7, 0, 0, 0, 0xc3], 6),
    ]);
    let mut prog = load(&path);
    prog.commit_pending_analysis().unwrap();
    let text = code(&mut prog);
    assert!(text.contains("return 7;"), "{text}");
    assert!(!text.contains("halt_missing"), "{text}");
    std::fs::remove_file(path).unwrap();
}

#[test]
fn warmed_branch_target_padding_still_ends_in_a_missing_halt() {
    let path = fixture(&[(
        0x10000,
        &[0x85, 0xc0, 0x75, 0x1c, 0xb8, 7, 0, 0, 0, 0xc3],
        10,
    )]);
    for warm in [false, true] {
        let mut prog = load(&path);
        if warm {
            prog.arch()
                .translate()
                .loader_rc()
                .borrow_mut()
                .load_fill(&mut [0; 64], &address(&prog, 0x10000))
                .unwrap();
        }
        prog.commit_pending_analysis().unwrap();
        let text = code(&mut prog);
        assert!(text.contains("return 7;"), "{text}");
        assert!(text.contains("halt_missing"), "{text}");
        assert!(text.contains("unmapped memory"), "{text}");
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn byte_overlays_remain_exclusive_and_repeated_flows_use_current_bytes() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/mapped_flow_boundary_32.elf");
    let mut prog = load(&path);
    prog.set_assertions(vec![Directive {
        raw: "bytes 0x10000 b807000000c3".to_string(),
        body: Body::Bytes {
            addr: 0x10000,
            data: vec![0xb8, 7, 0, 0, 0, 0xc3],
        },
    }]);
    assertions::apply_image_scoped(&mut prog);
    assert_eq!(prog.assertion_outcomes()[0].status, "applied");
    prog.commit_pending_analysis().unwrap();
    let first = code(&mut prog);
    assert!(first.contains("return 7;"), "{first}");
    assert!(!first.contains("halt_missing"), "{first}");
    assert_eq!(first, code(&mut prog));
    prog.arch()
        .translate()
        .loader_rc()
        .borrow_mut()
        .kuna_overlay_bytes(&address(&prog, 0x10000), &[0xb8, 9, 0, 0, 0, 0xc3])
        .unwrap();
    let changed = code(&mut prog);
    assert!(changed.contains("return 9;"), "{changed}");
}

#[test]
fn recovered_flow_contains_no_instructions_from_unmapped_padding() {
    for data in [
        &[0x85, 0xc0, 0x75, 6, 0xb8, 7, 0, 0, 0, 0xc3, 0xbb, 5, 0, 0, 0][..],
        &[0x85, 0xc0, 0x75, 6, 0xb8, 7, 0, 0, 0, 0xc3, 0x85, 0xdb, 0x75, 2, 0x90, 0x90][..],
    ] {
        let path = fixture(&[(0x10000, data, data.len() as u32)]);
        let mut prog = load(&path);
        let entry = address(&prog, 0x10000);
        let fd = build_and_follow_flow(prog.arch_mut(), "sample", entry, 0).unwrap();
        let end = 0x10000 + data.len() as u64;
        let mut missing = 0;
        for (seq, id) in fd.obank().iter_all() {
            let offset = seq.get_addr().get_offset();
            let op = fd.obank().get(id).unwrap();
            if offset >= end {
                assert_eq!(offset, end);
                assert_eq!(op.code(), OpCode::CPUI_RETURN);
                assert_ne!(op.get_halt_type() & pcodeop_flags::missing, 0);
                missing += 1;
            }
        }
        assert_eq!(missing, 1);
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn a_known_no_return_call_has_no_missing_fallthrough() {
    let path = fixture(&[(0x10000, &[0xe8, 0x2b, 0, 0, 0], 5), (0x10030, &[0xc3], 1)]);
    let mut prog = load(&path);
    let callee = address(&prog, 0x10030);
    prog.seed_function_inventory(&[("terminal".to_string(), callee)])
        .unwrap();
    let sid = prog.arch().query_global_function("terminal").unwrap();
    prog.arch_mut().symboltab.set_function_no_return(sid, true);
    let entry = address(&prog, 0x10000);
    let fd = build_and_follow_flow(prog.arch_mut(), "sample", entry, 0).unwrap();
    assert!(fd.obank().iter_all().any(|(_, id)| {
        fd.obank().get(id).unwrap().get_halt_type() & pcodeop_flags::noreturn != 0
    }));
    assert!(fd.obank().iter_all().all(|(_, id)| {
        fd.obank().get(id).unwrap().get_halt_type() & pcodeop_flags::missing == 0
    }));
    assert!(
        !prog
            .arch()
            .commentdb
            .comments()
            .iter()
            .any(|c| c.text.contains("unmapped memory"))
    );
    std::fs::remove_file(path).unwrap();
}

#[test]
fn mapping_warnings_survive_failed_reloads_and_refresh_after_an_overlay() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/mapped_flow_boundary_32.elf");
    for enabled in [true, false] {
        let mut prog = load(&path);
        let entry = address(&prog, 0x10000);
        prog.arch_mut().commentdb.add_comment_no_duplicate(
            kuna_decomp::architecture::comment_type::warningheader,
            &entry,
            &entry,
            "WARNING: retained warning",
        );
        let commands = [
            "load addr 0x10000".to_string(),
            "decompile".to_string(),
            "print C".to_string(),
            format!(
                "option mappedflowboundary {}",
                if enabled { "on" } else { "off" }
            ),
            "option maxinstruction 1".to_string(),
            "load addr 0x10000".to_string(),
            "print C".to_string(),
            "option maxinstruction 100000".to_string(),
            "load addr 0x10000".to_string(),
            "decompile".to_string(),
            "print C".to_string(),
        ];
        let mut status = ConsoleCommands::into_status(commands.into());
        register_decomp_commands(&mut status);
        status
            .get_data_mut(DECOMPILE_MODULE)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<IfaceDecompData>()
            .unwrap()
            .conf = Some(prog);
        for _ in 0..2 {
            assert!(status.run_command().unwrap());
        }
        status.optr.clear();
        assert!(status.run_command().unwrap());
        let original = status.optr.clone();
        assert!(original.contains("return 7;"), "{original}");
        assert!(original.contains("halt_missing"), "{original}");
        assert_eq!(
            original
                .matches("Function flows into unmapped memory")
                .count(),
            1
        );
        assert_eq!(
            original
                .matches("Function flow reaches unmapped memory:")
                .count(),
            1
        );
        for _ in 0..2 {
            assert!(status.run_command().unwrap());
        }
        let error = status.run_command().unwrap_err();
        assert!(
            error
                .to_string()
                .contains("Flow exceeded maximum allowable instructions"),
            "{error}"
        );
        status.optr.clear();
        assert!(status.run_command().unwrap());
        assert_eq!(status.optr, original);
        let dcp = status
            .get_data_mut(DECOMPILE_MODULE)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<IfaceDecompData>()
            .unwrap();
        dcp.conf
            .as_ref()
            .unwrap()
            .arch()
            .translate()
            .loader_rc()
            .borrow_mut()
            .kuna_overlay_bytes(&entry, &[0xb8, 9, 0, 0, 0, 0xc3])
            .unwrap();
        for _ in 0..3 {
            assert!(status.run_command().unwrap());
        }
        status.optr.clear();
        assert!(status.run_command().unwrap());
        let text = status.optr;
        assert!(text.contains("return 9;"), "{text}");
        assert!(!text.contains("unmapped memory"), "{text}");
        assert!(text.contains("retained warning"), "{text}");
    }
}

#[test]
fn a_known_function_bound_precedes_an_unmapped_fallthrough() {
    let path = fixture(&[(0x10000, &[0xb8, 7, 0, 0, 0], 5)]);
    let mut prog = load(&path);
    let next = address(&prog, 0x10005);
    prog.seed_function_inventory(&[("next".into(), next)])
        .unwrap();
    let entry = address(&prog, 0x10000);
    let fd = build_and_follow_flow(prog.arch_mut(), "sample", entry, 0).unwrap();
    assert!(fd.obank().iter_all().any(|(_, id)| {
        fd.obank().get(id).unwrap().get_halt_type() & pcodeop_flags::noreturn != 0
    }));
    assert!(fd.obank().iter_all().all(|(_, id)| {
        fd.obank().get(id).unwrap().get_halt_type() & pcodeop_flags::missing == 0
    }));
    assert!(
        prog.arch()
            .commentdb
            .comments()
            .iter()
            .any(|c| c.text.contains("funcboundflow:"))
    );
    std::fs::remove_file(path).unwrap();
}

#[test]
fn declared_boundary_policy_precedes_mapping_recovery() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../kuna-analysis/tests/fixtures/mapped_flow_boundary_32.elf");
    for enabled in [false, true] {
        for policy in [flow_flags::error_outofbounds, 0, flow_flags::ignore_outofbounds] {
            let mut prog = load(&path);
            prog.arch_mut().mapped_flow_boundary = enabled;
            prog.arch_mut().flowoptions |= policy;
            let entry = address(&prog, 0x10000);
            let result = build_and_follow_flow(prog.arch_mut(), "sample", entry, 15);
            if policy == flow_flags::error_outofbounds {
                let err = result.err().expect("declared boundary must be fatal");
                assert!(err.explain().contains("Function flow out of bounds"), "{err:?}");
            } else {
                result.unwrap();
            }
            let comments = prog.arch().commentdb.comments();
            assert_eq!(
                comments.iter().any(|c| c.text.contains("Function flows out of bounds")),
                policy == 0,
            );
            assert!(comments.iter().all(|c| !c.text.contains("unmapped memory")));
        }
    }
}

#[test]
fn a_mapped_loader_failure_is_not_recovered_as_a_missing_edge() {
    use kuna_base::error::{KunaError, KunaResult};
    use kuna_sleigh::loadimage::{ImageBytes, LoadImage};
    use std::sync::Arc;
    struct FailingImage(Arc<dyn ImageBytes>, u64);
    impl LoadImage for FailingImage {
        fn get_file_name(&self) -> &str {
            "synthetic.elf"
        }
        fn get_arch_type(&self) -> Vec<u8> {
            Vec::new()
        }
        fn adjust_vma(&mut self, _: i64) {}
        fn shared_bytes(&self) -> Option<Arc<dyn ImageBytes>> {
            Some(Arc::clone(&self.0))
        }
        fn load_fill(&mut self, buf: &mut [u8], addr: &Address) -> KunaResult<()> {
            if addr.get_offset() >= self.1 {
                return Err(KunaError::data_unavail("synthetic mapped read failure"));
            }
            self.0.fill_span(buf, addr.get_offset());
            Ok(())
        }
    }
    let path = fixture(&[(0x10000, &[0xb8, 7, 0, 0, 0, 0xc3], 6)]);
    for fail_from in [0x10000, 0x10005] {
        let mut prog = load(&path);
        let image = prog
            .arch()
            .translate()
            .loader_rc()
            .borrow()
            .shared_bytes()
            .unwrap();
        *prog.arch().translate().loader_rc().borrow_mut() =
            Box::new(FailingImage(image, fail_from));
        let entry = address(&prog, 0x10000);
        let err = build_and_follow_flow(prog.arch_mut(), "sample", entry, 0)
            .err()
            .expect("mapped read must fail");
        assert!(
            err.explain().contains("synthetic mapped read failure"),
            "{err:?}"
        );
    }
    std::fs::remove_file(path).unwrap();
}

#[test]
fn a_straddling_instruction_contributes_only_its_missing_halt() {
    for data in [
        &[0x85, 0xc0, 0x75, 6, 0xb8, 7, 0, 0, 0, 0xc3, 0xbb, 5, 0, 0, 0, 0, 0, 0][..],
        &[0x85, 0xc0, 0x75, 6, 0xb8, 7, 0, 0, 0, 0xc3, 0x8e, 0xc8, 0, 0, 0][..],
    ] {
        let path = fixture(&[(0x10000, data, data.len() as u32)]);
        let mut prog = load(&path);
        let entry = address(&prog, 0x10000);
        let fd = build_and_follow_flow(prog.arch_mut(), "sample", entry, 0).unwrap();
        let end = 0x10000 + data.len() as u64;
        let straddle = end - 1;
        let mut at_straddle = Vec::new();
        for (seq, id) in fd.obank().iter_all() {
            let offset = seq.get_addr().get_offset();
            assert!(offset < end, "op past the mapped end at {offset:#x}");
            if offset == straddle {
                at_straddle.push(id);
            }
        }
        assert_eq!(at_straddle.len(), 1);
        let halt = fd.obank().get(at_straddle[0]).unwrap();
        assert_eq!(halt.code(), OpCode::CPUI_RETURN);
        assert_ne!(halt.get_halt_type() & pcodeop_flags::missing, 0);
        assert!(
            fd.obank()
                .iter_all()
                .any(|(seq, _)| seq.get_addr().get_offset() == straddle - 2),
            "the mapped padding before the straddle is still decoded"
        );
        assert!(
            prog.arch()
                .commentdb
                .comments()
                .iter()
                .any(|c| c.text.contains("runs past the mapped bytes"))
        );
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn an_inlined_callee_that_leaves_mapped_memory_stays_an_error() {
    for tail in [&[][..], &[0][..]] {
        let mut data = vec![0xe8, 6, 0, 0, 0, 0xb8, 7, 0, 0, 0, 0xc3, 0xbb, 5, 0, 0, 0];
        data.extend_from_slice(tail);
        let path = fixture(&[(0x10000, &data, data.len() as u32)]);
        let mut prog = load(&path);
        let callee = address(&prog, 0x1000b);
        prog.seed_function_inventory(&[("callee".to_string(), callee.clone())])
            .unwrap();
        let alone = build_and_follow_flow(prog.arch_mut(), "callee", callee, 0).unwrap();
        assert!(alone.obank().iter_all().any(|(_, id)| {
            alone.obank().get(id).unwrap().get_halt_type() & pcodeop_flags::missing != 0
        }));
        let sid = prog.arch().query_global_function("callee").unwrap();
        prog.arch_mut().symboltab.set_function_inline(sid, true);
        let entry = address(&prog, 0x10000);
        let err = build_and_follow_flow(prog.arch_mut(), "caller", entry, 0)
            .err()
            .expect("an in-lined callee must not return past its missing bytes");
        assert!(err.explain().contains("not mapped"), "{err:?}");
        std::fs::remove_file(path).unwrap();
    }
}
