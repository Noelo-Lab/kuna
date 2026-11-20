//! Static DWARF int/error contracts preserve each raw result word through calls.

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_decomp::kuna_langfacet::LangFacet;
use std::path::PathBuf;

const CARRIER: &str = "__kuna_go_int_error_result";

fn program(image: &str, enabled: bool) -> ConsoleProgram {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let mut program = bootstrap_from_object(
        root.join("tests/fixtures/go-int-error-results").join(image).to_str().unwrap(),
        "", &[root.join("specs").display().to_string()],
    ).unwrap();
    program.arch_mut().set_kuna_option("godwarfsig", if enabled { "on" } else { "off" }).unwrap();
    program
}

fn bodies(mut program: ConsoleProgram, names: &[&str]) -> Vec<String> {
    program.commit_pending_analysis().unwrap();
    let mut commands = vec!["option outlang c".to_string()];
    for name in names {
        commands.extend([format!("load function main.{name}"), "decompile".into(), "print C".into()]);
    }
    let mut status = ConsoleCommands::into_status(commands);
    register_decomp_commands(&mut status);
    kuna_console::kuna_console::register_kuna_commands(&mut status);
    status.get_data_mut(DECOMPILE_MODULE).unwrap().as_any_mut()
        .downcast_mut::<IfaceDecompData>().unwrap().conf = Some(program);
    execute(&mut status);
    assert!(!status.inerror, "{}", status.optr);
    names.iter().map(|name| {
        for _ in 0..2 {
            execute(&mut status);
            assert!(!status.inerror, "{name}: {}", status.optr);
        }
        let start = status.optr.len();
        execute(&mut status);
        assert!(!status.inerror, "{name}: {}", status.optr);
        status.optr[start..].trim().to_string()
    }).collect()
}

#[test]
fn int_error_contract_binds_three_words_through_forwarding() {
    for image in ["contracts", "contracts-plain"] {
        let names = ["check", "forward", "modernForward", "consume"];
        let off = bodies(program(image, false), &names);
        assert!(off.iter().all(|body| !body.contains(CARRIER)));
        let on = bodies(program(image, true), &names);
        for body in &on[..3] {
            assert!(body.contains(CARRIER), "{image}: {body}");
        }
        assert!(on[1].contains("main.check(n)"), "{}", on[1]);
        assert!(on[0].contains("v1.error_tab = &dat_403030;\n    v1.result0 = 0x11;\n    v1.error_data = NULL;"), "typed nil: {}", on[0]);
        assert!(on[0].contains("return (__kuna_go_int_error_result)ZEXT824(n + 1ULL);"), "nil: {}", on[0]);
        for field in ["result0", "error_tab", "error_data"] {
            assert!(on[3].contains(field), "{image} lost {field}: {}", on[3]);
        }
        assert_eq!(on[3].matches("main.forward(").count(), 1, "{}", on[3]);
        assert!(on[3].contains("if (v2.error_tab) {\n    if ((v2.error_tab == &dat_403030) && (!v2.error_data))\n      return v1 + 100;"), "nil and typed nil must stay distinct: {}", on[3]);
    }
}

#[test]
fn incomplete_int_error_contracts_retain_inference() {
    let names = ["wrongFirst", "badKind", "badOffset", "singleRegister", "missingLocation",
        "thirdResult", "interfaceInput", "wrongProducer", "duplicate", "forward.abi0"];
    for image in ["contracts", "contracts-plain"] {
        let off = bodies(program(image, false), &names);
        let on = bodies(program(image, true), &names);
        for ((name, off), on) in names.iter().zip(off).zip(on) {
            assert_eq!(off, on, "unsupported {image} {name} acquired a contract");
        }
    }
}

#[test]
fn int_error_metadata_is_deferred_and_collision_guarded() {
    let mut off = program("contracts", false);
    off.commit_pending_analysis().unwrap();
    assert!(off.arch().types().find_by_name(CARRIER).unwrap().is_none());
    assert!(off.arch().types().kuna_lang_facet(CARRIER).is_none());
    let mut on = program("contracts", true);
    on.commit_pending_analysis().unwrap();
    let types = on.arch().types();
    let ty = types.find_by_name(CARRIER).unwrap().unwrap();
    assert_eq!(types.kuna_lang_facet(CARRIER), Some(LangFacet::GoIntErrorResults));
    assert!(LangFacet::GoIntErrorResults.go_int_error_fields(&ty).is_some());
    for collision in 0..3 {
        let program = program("contracts", true);
        let types = program.arch().types();
        if collision != 1 {
            types.get_type_struct(CARRIER).unwrap();
        }
        if collision != 0 {
            types.kuna_register_lang_facet(CARRIER, LangFacet::GoIntErrorResults).unwrap();
        }
        let output = bodies(program, &["forward"]);
        assert!(!output[0].contains(CARRIER), "collision {collision}: {}", output[0]);
    }
}
