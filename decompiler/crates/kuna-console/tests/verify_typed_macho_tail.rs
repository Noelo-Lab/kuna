//! Precedence regression for the typed Mach-O import tail: a prototype forced
//! at the CALLIND instruction must win over the callable type on its slot.

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::decompile_step::{decompile_one, DecompileSeed};
use kuna_console::engine::{bootstrap_from_object, EntrySelector};
use kuna_console::grammar::{parse_protopieces, DataOrg};
use kuna_decomp::dtype::type_metatype;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

#[test]
fn callsite_prototype_override_wins_over_slot_prototype() {
    let root = repo_root();
    let fixture = root.join("decompiler/crates/kuna-analysis/tests/fixtures/macho_imports");
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(fixture.to_str().unwrap(), "", &spec_roots) {
        Ok(prog) => prog,
        Err(err) => {
            eprintln!(
                "verify_typed_macho_tail: skipping (no `.sla`): {}",
                err.explain()
            );
            return;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit");
    let entry = prog
        .resolve_entry(&EntrySelector::Name("printf".to_string()))
        .expect("printf stub entry");
    let (addr_size, word_size) = prog.arch().data_org();
    let override_pieces = parse_protopieces(
        "int override(long x);",
        prog.arch().types(),
        DataOrg {
            addr_size,
            word_size,
        },
    )
    .expect("override declaration");
    let code = Rc::clone(prog.arch().manage().get_default_code_space().unwrap());
    let callpoint = Address::new(code, 0x1000005cc);
    let overrides = vec![(callpoint, override_pieces)];
    let seed = DecompileSeed::plain(&[], &[]);
    let step = decompile_one(
        prog.arch_mut(),
        &entry.name,
        entry.addr,
        entry.size as i32,
        &seed,
        &overrides,
    );
    let fd = step.result.expect("decompile printf stub");

    assert_eq!(fd.num_calls(), 1, "the veneer should contain one CALLIND");
    let call = fd.get_call_specs(0);
    assert!(
        call.is_input_locked(),
        "call-site override must remain locked"
    );
    assert!(
        !call.is_dotdotdot(),
        "slot's variadic printf prototype replaced the override"
    );
    assert_eq!(call.proto().num_params(), 1);
    let ty = call
        .proto()
        .get_param(0)
        .and_then(|param| param.get_type())
        .expect("override parameter");
    assert_eq!(ty.get_metatype(), type_metatype::TYPE_INT);
    assert_eq!(
        ty.get_size(),
        8,
        "the `long` override, not slot `char *`, must win"
    );
}
