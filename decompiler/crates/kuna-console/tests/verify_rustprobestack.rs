use kuna_base::address::Address;
use kuna_console::engine::bootstrap_from_object;
use kuna_decomp::action::ActionContext;
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::kuna_rustprobestack::ActionRustProbestack;
use kuna_num::opcodes::OpCode;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn fixture() -> PathBuf {
    root().join("decompiler/crates/kuna-analysis/tests/fixtures/rust_probestack_x86_64")
}

fn raw(binary: &Path, offset: u64, mode: &str, language: &str) -> Funcdata {
    raw_policy(binary, offset, mode, language, None)
}

fn raw_policy(
    binary: &Path,
    offset: u64,
    mode: &str,
    language: &str,
    policy: Option<&str>,
) -> Funcdata {
    let mut program = bootstrap_from_object(
        binary.to_str().unwrap(),
        "",
        &[root().join("specs").display().to_string()],
    )
    .unwrap();
    program
        .arch_mut()
        .set_kuna_option("rustprobestack", mode)
        .unwrap();
    program.arch_mut().source_lang_forced = match language {
        "auto" => None,
        "c" => Some(kuna_decomp::kuna_sourcelang::SourceLang::C),
        "go" => Some(kuna_decomp::kuna_sourcelang::SourceLang::Go),
        _ => panic!("unsupported test language"),
    };
    if let Some(policy) = policy {
        program.arch_mut().set_kuna_option(policy, "on").unwrap();
    }
    program.commit_pending_analysis().unwrap();
    let space = program
        .arch()
        .manage()
        .get_default_code_space()
        .unwrap()
        .clone();
    kuna_decomp::decompile_drive::build_and_follow_flow(
        program.arch_mut(),
        "probe",
        Address::new(space, offset),
        0,
    )
    .unwrap()
}

fn apply(fd: &mut Funcdata) -> i32 {
    ActionRustProbestack::boxed("protorecovery").apply(fd, &mut ActionContext::default())
}

#[test]
fn raw_summary_preserves_stack_writes_and_page_residuals() {
    for (offset, residual) in [
        (0x56420, 4096),
        (0x56460, 8),
        (0x564a0, 4096),
        (0x564e0, 8),
        (0x56550, 0),
        (0x56570, 1),
        (0x56590, 4096),
        (0x565b0, 1),
        (0x565d0, 4096),
        (0x565f0, 8),
        (0x56610, 8),
    ] {
        let mut fd = raw(&fixture(), offset, "auto", "auto");
        assert!(fd.get_arch().source_lang.is_rust());
        assert_eq!(fd.num_calls(), 1);
        let call = fd.get_call_specs(0).get_op();
        let store = fd.op_previous_op(call).unwrap();
        let return_slot = fd.obank().get(store).unwrap().get_in(2).unwrap();
        let return_address = fd.vbank().get(return_slot).unwrap().get_offset();
        let store_count = fd.obank().iter_code(OpCode::CPUI_STORE).count();
        let window = fd
            .get_arch()
            .loader
            .as_ref()
            .unwrap()
            .borrow()
            .read_window();
        assert_eq!(apply(&mut fd), 1, "{offset:x}");
        assert_eq!(
            fd.get_arch()
                .loader
                .as_ref()
                .unwrap()
                .borrow()
                .read_window(),
            window
        );
        assert_eq!(fd.num_calls(), 0);
        assert_eq!(
            fd.obank().iter_code(OpCode::CPUI_STORE).count(),
            store_count + 1
        );
        let retained = fd.obank().get(store).unwrap();
        assert_eq!(retained.get_in(2), Some(return_slot));
        assert_eq!(
            fd.vbank().get(return_slot).unwrap().get_offset(),
            return_address
        );
        let writes: Vec<_> = fd
            .obank()
            .iter_alive()
            .filter(|&id| {
                fd.obank()
                    .get(id)
                    .is_some_and(|op| op.code() == OpCode::CPUI_COPY)
            })
            .filter_map(|id| {
                let op = fd.obank().get(id)?;
                let out = fd.vbank().get(op.get_out()?)?;
                (out.get_offset() == 0x98 && out.get_size() == 8).then(|| {
                    let input = fd.vbank().get(op.get_in(0).unwrap()).unwrap();
                    assert!(input.is_constant());
                    input.get_offset()
                })
            })
            .collect();
        assert_eq!(writes, [residual]);
        assert_eq!(apply(&mut fd), 0);
    }
}

#[test]
fn gates_and_unverified_contexts_keep_the_call() {
    for (offset, mode, language, expected) in [
        (0x56420, "off", "auto", 0),
        (0x56420, "auto", "c", 0),
        (0x56420, "auto", "go", 0),
        (0x56420, "always", "c", 1),
        (0x56510, "always", "auto", 0),
        (0x56640, "always", "auto", 0),
        (0x56660, "always", "auto", 0),
    ] {
        let mut fd = raw(&fixture(), offset, mode, language);
        assert_eq!(apply(&mut fd), expected, "{offset:x} {mode} {language}");
        assert_eq!(fd.num_calls(), 1 - expected);
    }
}

#[test]
fn custom_call_facts_and_interior_target_are_not_overridden() {
    for mutation in 0..4 {
        let mut fd = raw(&fixture(), 0x56420, "always", "auto");
        let fc = fd.get_call_specs_mut(0);
        match mutation {
            0 => fc.proto_mut().set_inline(true),
            1 => fc.proto_mut().set_no_return(true),
            2 => fc.proto_mut().set_inject_id(0),
            _ => {
                let target = fc.get_entry_address();
                fc.set_address(Address::new(
                    target.get_space().unwrap().clone(),
                    target.get_offset() + 1,
                ));
            }
        }
        assert_eq!(apply(&mut fd), 0, "mutation {mutation}");
        assert_eq!(fd.num_calls(), 1);
    }
}

#[test]
fn unknown_or_oversized_frame_and_modified_return_slot_decline() {
    for mutation in 0..4 {
        let mut fd = raw(&fixture(), 0x56420, "always", "auto");
        let call = fd.get_call_specs(0).get_op();
        if mutation == 3 {
            let store = fd.op_previous_op(call).unwrap();
            let wrong = fd.new_constant(8, 0x1234);
            fd.op_set_input(store, wrong, 2).unwrap();
        } else {
            let copy = fd
                .obank()
                .iter_alive()
                .filter(|&id| {
                    fd.obank()
                        .get(id)
                        .is_some_and(|op| op.code() == OpCode::CPUI_COPY)
                })
                .find(|&id| {
                    fd.obank()
                        .get(id)
                        .and_then(|o| o.get_out())
                        .and_then(|v| fd.vbank().get(v))
                        .is_some_and(|v| v.get_offset() == 0 && v.get_size() == 8)
                })
                .unwrap();
            let input = if mutation == 0 {
                let reg = fd
                    .get_arch()
                    .manage()
                    .register_lookup()
                    .unwrap()
                    .probe_register("RDI")
                    .unwrap();
                fd.new_varnode(8, &Address::new(reg.space.unwrap(), reg.offset), None)
            } else {
                fd.new_constant(
                    8,
                    if mutation == 1 {
                        16 * 1024 * 1024 + 1
                    } else {
                        u64::MAX
                    },
                )
            };
            fd.op_set_input(copy, input, 0).unwrap();
        }
        assert_eq!(apply(&mut fd), 0, "mutation {mutation}");
        assert_eq!(fd.num_calls(), 1);
    }
}

#[test]
fn same_named_modified_helper_remains_a_call() {
    let mut bytes = std::fs::read(fixture()).unwrap();
    let signature = [0x55, 0x48, 0x89, 0xe5, 0x49, 0x89, 0xc3, 0x49, 0x81, 0xfb];
    let matches: Vec<_> = bytes
        .windows(signature.len())
        .enumerate()
        .filter_map(|(i, b)| (b == signature).then_some(i))
        .collect();
    assert_eq!(matches.len(), 1);
    bytes[matches[0]] = 0x90;
    let path = std::env::temp_dir().join(format!("kuna-modified-rustprobe-{}", std::process::id()));
    std::fs::write(&path, bytes).unwrap();
    let mut fd = raw(&path, 0x56420, "always", "auto");
    std::fs::remove_file(path).unwrap();
    assert_eq!(fd.get_call_specs(0).get_name(), "__rust_probestack");
    assert_eq!(apply(&mut fd), 0);
    assert_eq!(fd.num_calls(), 1);
}

struct Unmapped(std::sync::Arc<dyn kuna_sleigh::loadimage::ImageBytes>);

impl std::fmt::Debug for Unmapped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Unmapped")
    }
}

impl kuna_sleigh::loadimage::ImageBytes for Unmapped {
    fn fill_span(&self, dst: &mut [u8], start: u64) -> usize {
        self.0.fill_span(dst, start)
    }
    fn mapped_covers(&self, _lo: u64, _hi: u64) -> bool {
        false
    }
}

struct EvidenceLoader(Option<std::sync::Arc<dyn kuna_sleigh::loadimage::ImageBytes>>);

impl kuna_sleigh::loadimage::LoadImage for EvidenceLoader {
    fn get_file_name(&self) -> &str {
        "evidence-control"
    }
    fn get_arch_type(&self) -> Vec<u8> {
        b"x86:LE:64:default:gcc".to_vec()
    }
    fn adjust_vma(&mut self, _adjust: i64) {}
    fn load_fill(
        &mut self,
        _dst: &mut [u8],
        _address: &Address,
    ) -> kuna_base::error::KunaResult<()> {
        Err(kuna_base::error::KunaError::data_unavail(
            "unavailable evidence",
        ))
    }
    fn shared_bytes(&self) -> Option<std::sync::Arc<dyn kuna_sleigh::loadimage::ImageBytes>> {
        self.0.clone()
    }
}

#[test]
fn missing_image_interface_or_mapping_proof_declines_without_loader_reads() {
    for unmapped in [false, true] {
        let mut fd = raw(&fixture(), 0x56420, "always", "auto");
        {
            let loader = fd.get_arch().loader.as_ref().unwrap();
            let image = loader.borrow().shared_bytes().unwrap();
            *loader.borrow_mut() = Box::new(EvidenceLoader(if unmapped {
                Some(std::sync::Arc::new(Unmapped(image)))
            } else {
                None
            }));
        }
        assert_eq!(apply(&mut fd), 0);
        assert_eq!(fd.num_calls(), 1);
    }
}

#[test]
fn optional_stack_memory_policies_retain_the_raw_call() {
    for policy in ["stackviews", "stackalias"] {
        for mode in ["auto", "always"] {
            let mut fd = raw_policy(&fixture(), 0x56420, mode, "auto", Some(policy));
            assert_eq!(apply(&mut fd), 0, "{policy} {mode}");
            assert_eq!(fd.num_calls(), 1);
        }
    }
}
