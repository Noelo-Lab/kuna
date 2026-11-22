//! The resolved call target is discovered even when it is not a walk seed.
use kuna_analysis::listing::xrefs::{self, XrefKind};
use kuna_console::engine::bootstrap_from_object;
use object::{Object, ObjectSymbol};
use std::path::PathBuf;

#[test]
fn an_immutable_slot_call_discovers_its_unseeded_target() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let path =
        root.join("decompiler/crates/kuna-analysis/tests/fixtures/immutable_got_calls_x86_64");
    let mut program = bootstrap_from_object(
        path.to_str().unwrap(),
        "",
        &[root.join("specs").display().to_string()],
    )
    .unwrap();
    program.commit_pending_analysis().unwrap();
    let caller = program.lookup_symbol("through_got").unwrap().get_offset();
    let target = program.lookup_symbol("callback").unwrap().get_offset();
    let bytes = std::fs::read(path).unwrap();
    let file = object::File::parse(bytes.as_slice()).unwrap();
    let index = xrefs::build(&file, program.arch(), program.arch().translate(), &[caller]);
    assert!(index.is_function_entry(target), "target was not a seed");
    assert!(index.is_decoded(target));
    let calls: Vec<_> = index
        .refs_from_function(caller)
        .into_iter()
        .filter(|r| r.kind == XrefKind::Call)
        .collect();
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].to, target);
    let slot = file
        .symbols()
        .find(|s| s.name().ok() == Some("callback_slot"))
        .unwrap()
        .address();
    assert!(index
        .refs_from_function(caller)
        .iter()
        .any(|r| r.kind == XrefKind::Read && r.to == slot));
    assert!(index.has_indirect_calls(caller));
}

#[test]
fn a_stated_callee_excludes_its_physical_site_from_got_proof() {
    use kuna_base::address::Address;
    use kuna_base::space::{spacetype, AddrSpace};
    use std::rc::Rc;

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let path =
        root.join("decompiler/crates/kuna-analysis/tests/fixtures/immutable_got_calls_x86_64");
    let mut program = bootstrap_from_object(
        path.to_str().unwrap(),
        "",
        &[root.join("specs").display().to_string()],
    )
    .unwrap();
    program.commit_pending_analysis().unwrap();
    let caller = program.lookup_symbol("through_got").unwrap();
    let original = program.lookup_symbol("callback").unwrap();
    let alternate = program.lookup_symbol("alternate").unwrap();
    let other_owner = program.lookup_symbol("through_mutable").unwrap();
    let code = Rc::clone(caller.get_space().unwrap());
    let site = Address::new(Rc::clone(&code), caller.get_offset() + 4);
    let bytes = std::fs::read(path).unwrap();
    let file = object::File::parse(bytes.as_slice()).unwrap();
    let slot = file
        .symbols()
        .find(|s| s.name().ok() == Some("callback_slot"))
        .unwrap()
        .address();
    for case in 0..5 {
        program.arch_mut().kuna_callee_overrides.clear();
        let expected = match case {
            0 | 1 => {
                if case == 1 {
                    kuna_console::kuna_calleeoverride::plant(
                        &mut program,
                        &caller,
                        &site,
                        "callback",
                    )
                    .unwrap();
                }
                kuna_console::kuna_calleeoverride::plant(&mut program, &caller, &site, "alternate")
                    .unwrap();
                slot
            }
            2 => {
                kuna_console::kuna_calleeoverride::plant(
                    &mut program,
                    &other_owner,
                    &site,
                    "alternate",
                )
                .unwrap();
                slot
            }
            3 => {
                let other_site = Address::new(Rc::clone(&code), site.get_offset() + 1);
                program
                    .arch_mut()
                    .kuna_callee_overrides
                    .entry(caller.clone())
                    .or_default()
                    .insert(other_site, alternate.clone());
                original.get_offset()
            }
            _ => {
                let foreign = Rc::new(AddrSpace::new_for_decode(spacetype::IPTR_INTERNAL));
                program
                    .arch_mut()
                    .kuna_callee_overrides
                    .entry(caller.clone())
                    .or_default()
                    .insert(site.clone(), Address::new(foreign, alternate.get_offset()));
                slot
            }
        };
        let stated = program.arch().kuna_callee_overrides.clone();
        let index = xrefs::build(
            &file,
            program.arch(),
            program.arch().translate(),
            &[caller.get_offset()],
        );
        let refs = index.refs_from_function(caller.get_offset());
        let calls: Vec<_> = refs
            .iter()
            .filter(|r| r.kind == XrefKind::Call)
            .map(|r| r.to)
            .collect();
        assert_eq!(calls, vec![expected], "case {case}");
        if expected != slot {
            assert!(
                refs.iter()
                    .any(|r| r.kind == XrefKind::Read && r.to == slot),
                "case {case}"
            );
        }
        assert!(index.has_indirect_calls(caller.get_offset()));
        assert_eq!(program.arch().kuna_callee_overrides, stated);
    }
}
