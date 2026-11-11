//! Verified older Rust piece-array printing and its refusal boundaries.

use kuna_console::engine::bootstrap_from_object;
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::kuna_rustfmt::RustFmtMode;
use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..")
}

fn decompile(
    fixture: &str,
    name: &str,
    enabled: bool,
    language: &str,
    configure: impl FnOnce(&mut Architecture),
    edit: impl FnOnce(&mut Funcdata),
) -> String {
    let binary = root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(fixture);
    let mut program = bootstrap_from_object(
        binary.to_str().unwrap(),
        "",
        &[root().join("specs").display().to_string()],
    )
    .unwrap();
    assert!(program.arch().rustc_pieces_layout);
    program.arch_mut().rust_fmt = if enabled {
        RustFmtMode::On
    } else {
        RustFmtMode::Off
    };
    configure(program.arch_mut());
    program.commit_pending_analysis().unwrap();
    let commands = vec![
        format!("option setlanguage {language}-language"),
        format!("load function {name}"),
        "decompile".into(),
        "print C".into(),
    ];
    let mut status = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut status);
    status
        .get_data_mut(DECOMPILE_MODULE)
        .unwrap()
        .as_any_mut()
        .downcast_mut::<IfaceDecompData>()
        .unwrap()
        .conf = Some(program);
    for _ in 0..3 {
        execute(&mut status);
        assert!(!status.inerror, "{}", status.optr);
    }
    edit(
        status
            .get_data_mut(DECOMPILE_MODULE)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<IfaceDecompData>()
            .unwrap()
            .fd
            .as_mut()
            .unwrap(),
    );
    status.optr.clear();
    execute(&mut status);
    assert!(!status.inerror, "{}", status.optr);
    status.optr
}

fn output(fixture: &str, name: &str, enabled: bool) -> String {
    decompile(fixture, name, enabled, "rust", |_| {}, |_| {})
}

const NATIVE: [&str; 3] = [
    "rust_fmt_pieces_1.89.0",
    "rust_fmt_pieces_1.90.0",
    "rust_fmt_pieces_nightly-2025-05-22",
];
const SHAPES: &str = "rust_fmt_pieces_x86_64";

#[test]
fn verified_producers_recover_literal_and_primitive_printing() {
    for fixture in NATIVE {
        for (name, expected) in [
            ("pieces_literal", "println!(\"literal {{é🙂}}\\n\")"),
            (
                "pieces_plain",
                "print!(\"quote=\\\" slash=\\\\ nul=\\0 {{}} \"",
            ),
            ("pieces_stderr", "eprintln!(\"stderr\\r\")"),
            ("pieces_empty", "print!(\"\")"),
            ("pieces_u32", "println!(\"value {} end\""),
            ("pieces_i64", "print!(\"{}\""),
            ("pieces_pair", "eprint!(\"{} = {}\""),
            ("pieces_narrow", "println!(\"[{}|{}|{}|{}]\""),
            ("pieces_adjacent", "println!(\"{}{}\""),
        ] {
            let on = output(fixture, name, true);
            let off = output(fixture, name, false);
            assert!(on.contains(expected), "{fixture}/{name}: {on}");
            assert!(!off.contains(expected), "{fixture}/{name}: {off}");
            assert!(!on.contains("_print(") && !on.contains("_eprint("), "{on}");
        }
    }
    assert!(output(SHAPES, "literal_shape", true).contains("println!(\"{{x}}\")"));
    assert!(output(SHAPES, "stderr_shape", true).contains("eprintln!(\"{{x}}\")"));
}

#[test]
fn unsupported_formatting_and_untrusted_descriptors_remain_explicit() {
    for fixture in NATIVE {
        for name in [
            "pieces_repeat",
            "pieces_flags",
            "pieces_dynamic",
            "pieces_generic",
            "pieces_float",
            "pieces_debug",
        ] {
            assert_eq!(
                output(fixture, name, true),
                output(fixture, name, false),
                "{fixture}/{name}"
            );
        }
    }
    for name in [
        "reject_root",
        "reject_missing_scope",
        "reject_extra_scope",
        "reject_global",
        "reject_mutable_table",
        "reject_mutable_text",
        "reject_invalid_utf8",
        "reject_large",
        "reject_unmapped",
        "reject_overflow",
        "reject_null_text",
        "reject_piece_count",
        "reject_argument_count",
        "reject_format_specs",
        "reject_dynamic_table",
        "reject_dynamic_count",
        "reject_dynamic_format",
        "reject_forwarded_descriptor",
    ] {
        let on = output(SHAPES, name, true);
        assert_eq!(on, output(SHAPES, name, false), "{name}");
        assert!(on.contains("_print("), "{name}: {on}");
    }
}

#[test]
fn language_options_and_layout_evidence_bound_recovery() {
    for address in [0x2000, 0x2010] {
        let run = |enabled| {
            decompile(
                SHAPES,
                "literal_shape",
                enabled,
                "rust",
                |arch| {
                    let space = arch.manage().get_default_data_space().unwrap().clone();
                    arch.symboltab.set_property_range(
                        kuna_decomp::varnode::varnode_flags::volatil,
                        &kuna_base::address::Address::new(space.clone(), address),
                        &kuna_base::address::Address::new(space, address + 1),
                    );
                },
                |_| {},
            )
        };
        let on = run(true);
        assert_eq!(on, run(false));
        assert!(on.contains("_print("));
    }
    for source in [
        kuna_decomp::kuna_sourcelang::SourceLang::C,
        kuna_decomp::kuna_sourcelang::SourceLang::Go,
    ] {
        let run = |enabled| {
            decompile(
                NATIVE[1],
                "pieces_u32",
                enabled,
                "rust",
                |arch| arch.source_lang_forced = Some(source),
                |_| {},
            )
        };
        assert_eq!(run(true), run(false));
    }
    for language in ["c", "go"] {
        assert_eq!(
            decompile(NATIVE[1], "pieces_u32", true, language, |_| {}, |_| {}),
            decompile(NATIVE[1], "pieces_u32", false, language, |_| {}, |_| {})
        );
    }
    for mutation in 0..4 {
        let run = |enabled| {
            decompile(
                NATIVE[1],
                "pieces_u32",
                enabled,
                "rust",
                |arch| match mutation {
                    0 => arch.rustc_pieces_layout = false,
                    1 => arch.rust_fmt_pieces_relro = None,
                    2 => {
                        arch.rust_display_formatters.clear();
                        arch.rust_integer_formatters.clear();
                    }
                    _ => {
                        let (lo, hi) = arch.rust_fmt_pieces_relro.unwrap();
                        let space = arch.manage().get_default_data_space().unwrap().clone();
                        arch.symboltab.set_property_range(
                            kuna_decomp::varnode::varnode_flags::volatil,
                            &kuna_base::address::Address::new(space.clone(), lo),
                            &kuna_base::address::Address::new(space, hi),
                        );
                    }
                },
                |_| {},
            )
        };
        assert_eq!(run(true), run(false), "mutation {mutation}");
    }
}

#[test]
fn layout_metadata_rejects_other_images_and_bounds_relro_to_full_pages() {
    let bytes = std::fs::read(
        root()
            .join("decompiler/crates/kuna-analysis/tests/fixtures")
            .join(NATIVE[1]),
    )
    .unwrap();
    let layout = kuna_analysis::sourcelang::fmt_pieces_layout(&bytes).unwrap();
    let (lo, hi) = layout.relro.unwrap();
    assert!(lo < hi && hi % 4096 == 0);
    let phoff = u64::from_le_bytes(bytes[32..40].try_into().unwrap()) as usize;
    let width = u16::from_le_bytes(bytes[54..56].try_into().unwrap()) as usize;
    let count = u16::from_le_bytes(bytes[56..58].try_into().unwrap()) as usize;
    let relro = (0..count)
        .map(|i| phoff + i * width)
        .find(|&p| u32::from_le_bytes(bytes[p..p + 4].try_into().unwrap()) == 0x6474e552)
        .unwrap();
    let mut partial = bytes.clone();
    partial[relro + 40..relro + 48].copy_from_slice(&(hi - lo - 1).to_le_bytes());
    assert_eq!(
        kuna_analysis::sourcelang::fmt_pieces_layout(&partial)
            .unwrap()
            .relro,
        Some((lo, hi - 4096))
    );
    let mut no_relro = bytes.clone();
    no_relro[relro..relro + 4].copy_from_slice(&0u32.to_le_bytes());
    assert!(kuna_analysis::sourcelang::fmt_pieces_layout(&no_relro)
        .unwrap()
        .relro
        .is_none());
    for mutation in 0..3 {
        let mut other = bytes.clone();
        match mutation {
            0 => other[18..20].copy_from_slice(&183u16.to_le_bytes()),
            1 => other[16..18].copy_from_slice(&1u16.to_le_bytes()),
            _ => {
                let p = other
                    .windows(20)
                    .position(|w| w == b"rustc version 1.90.0")
                    .unwrap();
                other[p + 17] = b'1';
            }
        }
        assert!(
            kuna_analysis::sourcelang::fmt_pieces_layout(&other).is_none(),
            "{mutation}"
        );
    }
    assert!(kuna_analysis::sourcelang::fmt_pieces_layout(b"not ELF").is_none());
}

#[test]
fn incomplete_call_and_stack_evidence_prevent_recovery() {
    use kuna_num::opcodes::OpCode;
    for mutation in 0..4 {
        let run = |enabled| {
            decompile(
                NATIVE[1],
                "pieces_literal",
                enabled,
                "rust",
                |_| {},
                |fd| {
                    assert_eq!(fd.num_calls(), 1);
                    let call = fd.get_call_specs(0).get_op();
                    match mutation {
                        0 => fd.op_remove_input(call, 1),
                        1 => {
                            let value = fd.new_constant(8, 7);
                            fd.op_insert_input(call, value, 2).unwrap();
                        }
                        _ => {
                            let indirect = fd
                                .obank()
                                .iter_alive()
                                .find(|id| {
                                    fd.obank().get(*id).unwrap().code() == OpCode::CPUI_INDIRECT
                                })
                                .unwrap();
                            if mutation == 2 {
                                let effect = fd.new_constant(8, 0);
                                fd.op_set_input(indirect, effect, 1).unwrap();
                            } else {
                                fd.obank_mut()
                                    .get_mut(indirect)
                                    .unwrap()
                                    .set_flag(kuna_decomp::op::pcodeop_flags::indirect_creation);
                            }
                        }
                    }
                },
            )
        };
        assert_eq!(run(true), run(false), "mutation {mutation}");
    }
}

#[test]
fn emitted_literals_and_primitive_macros_reproduce_native_streams() {
    use crate::common;
    use std::process::Command;
    if Command::new("rustc")
        .arg("--version")
        .output()
        .is_err_and(|e| e.kind() == std::io::ErrorKind::NotFound)
    {
        return;
    }
    for fixture in NATIVE {
        let mut source = String::new();
        for name in [
            "pieces_literal",
            "pieces_plain",
            "pieces_stderr",
            "pieces_empty",
        ] {
            let body = output(fixture, name, true);
            source.push_str(&body[body.find("#[allow(").unwrap()..]);
        }
        for (name, parameters, bindings) in [
            ("pieces_u32", "a:u32", "let v2=&a as *const u32;"),
            ("pieces_i64", "a:i64", "let v2=&a as *const i64;"),
            ("pieces_pair", "a:i8,b:u64", "let v3=&a as *const i8;let v6=&b as *const u64;"),
            ("pieces_narrow", "a:u8,b:i16,c:u16,d:i32", "let v5=&a as *const u8;let v8=&b as *const i16;let v10=&c as *const u16;let v12=&d as *const i32;"),
            ("pieces_adjacent", "a:u64,b:i64", "let v3=&a as *const u64;let v6=&b as *const i64;"),
        ] {
            let body=output(fixture,name,true);
            let macros:Vec<_>=body.lines().filter(|line| line.contains("print!(")||line.contains("println!(")).collect();
            assert_eq!(macros.len(),1,"{body}");
            source.push_str(&format!("unsafe fn {name}({parameters}) {{ {bindings} {} }}\n",macros[0]));
        }
        source.push_str("fn main(){unsafe{pieces_literal();pieces_plain();pieces_stderr();pieces_empty();pieces_u32(23);pieces_i64(i64::MIN);pieces_pair(-7,u64::MAX);pieces_narrow(255,i16::MIN,u16::MAX,i32::MIN);pieces_adjacent(11,-22);}}\n");
        let path = common::scratch_file("rustfmtpieces", "rs");
        let binary = path.with_extension("exe");
        std::fs::write(&path, source).unwrap();
        let build = Command::new("rustc")
            .args(["--edition=2021", "--crate-name", "recovered_pieces"])
            .arg(&path)
            .arg("-o")
            .arg(&binary)
            .output()
            .unwrap();
        assert!(
            build.status.success(),
            "{}",
            String::from_utf8_lossy(&build.stderr)
        );
        let recovered = Command::new(&binary).output().unwrap();
        let native = Command::new(
            root()
                .join("decompiler/crates/kuna-analysis/tests/fixtures")
                .join(fixture),
        )
        .output()
        .unwrap();
        assert!(recovered.status.success() && native.status.success());
        assert_eq!(recovered.stdout, native.stdout, "{fixture} stdout");
        assert_eq!(recovered.stderr, native.stderr, "{fixture} stderr");
        std::fs::remove_file(path).unwrap();
        std::fs::remove_file(binary).unwrap();
    }
}
