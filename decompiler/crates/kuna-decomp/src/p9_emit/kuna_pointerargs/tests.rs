use super::*;

fn records() -> (Record, Record) {
    let callee = Record {
        entry: ("ram".into(), 0x1000),
        parameters: vec![
            None,
            Some(Parameter {
                storage: (("register".into(), 0x38), 8),
                spelling: "unsigned char *".into(),
            }),
        ],
        calls: vec![],
    };
    let caller = Record {
        entry: ("ram".into(), 0x2000),
        parameters: vec![],
        calls: vec![Call {
            callee: callee.entry.clone(),
            index: 1,
            storage: callee.parameters[1].as_ref().unwrap().storage.clone(),
            actual: "unsigned long *".into(),
            position: Position { line: 1, column: 7 },
            expression: "&v1".into(),
        }],
    };
    (caller, callee)
}

#[test]
fn final_declarations_decide_independently_of_order() {
    let (caller, callee) = records();
    let a = insertions(&caller, &definitions([&caller, &callee]));
    let b = insertions(&caller, &definitions([&callee, &caller]));
    assert_eq!(a, b);
    assert_eq!(
        apply("sink(0,&v1);", &a).code,
        "sink(0,(unsigned char *)&v1);"
    );
}

#[test]
fn missing_ambiguous_or_differently_stored_parameters_abstain() {
    let (mut caller, callee) = records();
    assert!(insertions(&caller, &definitions([&caller])).is_empty());
    let mut other = callee.clone();
    other.parameters[1].as_mut().unwrap().spelling = "char *".into();
    assert!(insertions(&caller, &definitions([&callee, &other])).is_empty());
    let defs = definitions([&callee]);
    caller.calls[0].index = 0;
    assert!(insertions(&caller, &defs).is_empty());
    caller.calls[0].index = 1;
    caller.calls[0].storage.1 = 4;
    assert!(insertions(&caller, &defs).is_empty());
    caller.calls[0].storage.1 = 8;
    caller.calls[0].storage.0 .1 += 1;
    assert!(insertions(&caller, &defs).is_empty());
}

#[test]
fn each_printed_occurrence_is_edited_and_neighbouring_text_is_preserved() {
    let (mut caller, callee) = records();
    let mut again = caller.calls[0].clone();
    again.position.line = 2;
    caller.calls.push(again);
    let edits = insertions(&caller, &definitions([&callee]));
    assert_eq!(
        apply("sink(0,&v1);\nsink(0,&v1); // &v1\n", &edits).code,
        "sink(0,(unsigned char *)&v1);\nsink(0,(unsigned char *)&v1); // &v1\n"
    );
}

#[test]
fn stale_and_overlapping_sites_stay_uncast_and_leave_the_document_intact() {
    let (caller, callee) = records();
    let edits = insertions(&caller, &definitions([&callee]));
    for code in ["sink(0,&v2);", "sink(0,&v10);", "a\nxxxxx&v1", "sink(0,"] {
        let applied = apply(code, &edits);
        assert_eq!(applied.code, code);
            assert_eq!(applied.skipped.len(), 1, "{code}");
    }
    let twice = apply("sink(0,&v1);", &[edits[0].clone(), edits[0].clone()]);
    assert_eq!(twice.code, "sink(0,(unsigned char *)&v1);");
    assert_eq!(twice.skipped[0].1, "pointer argument emission positions overlap");
    let mut broken = edits[0].clone();
    broken.spelling = "unsigned\nchar *".into();
    assert_eq!(apply("sink(0,&v1);", &[broken]).code, "sink(0,&v1);");
}

#[test]
fn one_drifted_site_does_not_cost_the_other_casts() {
    let (mut caller, callee) = records();
    let mut drifted = caller.calls[0].clone();
    drifted.position = Position { line: 2, column: 3 };
    caller.calls.push(drifted);
    let mut later = caller.calls[0].clone();
    later.position.line = 3;
    caller.calls.push(later);
    let edits = insertions(&caller, &definitions([&callee]));
    let applied = apply("sink(0,&v1);\nsink(0,&v1);\nsink(0,&v1);", &edits);
    assert_eq!(
        applied.code,
        "sink(0,(unsigned char *)&v1);\nsink(0,&v1);\nsink(0,(unsigned char *)&v1);"
    );
    assert_eq!(applied.skipped.len(), 1);
    assert_eq!(applied.skipped[0].0.position.line, 2);
}

#[test]
fn structure_renames_move_only_sites_after_them_on_the_same_line() {
    let (mut caller, _) = records();
    let code = "struct_1 *p; sink(0,&v1);\nsink(0,&v1);";
    caller.calls[0].position.column = 19;
    let mut again = caller.calls[0].clone();
    again.position = Position { line: 2, column: 7 };
    caller.calls.push(again);
    caller.renamed(code, &[(0, 8, "struct_100".into())]);
    assert_eq!(caller.calls[0].position.column, 21);
    assert_eq!(caller.calls[1].position.column, 7);

    // Unordered edits: before the site on its own line, after it, and on
    // another line; only the first moves the site.
    let code = "struct_1 *p; sink(0,&v1); struct_2 *q;\nstruct_3 *r; sink(0,&v1);";
    let (mut caller, _) = records();
    caller.calls[0].position.column = 20;
    let mut again = caller.calls[0].clone();
    again.position = Position { line: 2, column: 20 };
    caller.calls.push(again);
    caller.renamed(code, &[
        (39, 47, "s".into()),
        (26, 34, "struct_200".into()),
        (0, 8, "struct_100".into()),
    ]);
    assert_eq!(caller.calls[0].position.column, 22);
    assert_eq!(caller.calls[1].position.column, 13);
}

#[test]
fn trimming_changes_lines_without_changing_columns() {
    let (mut caller, _) = records();
    caller.calls[0].position.line = 3;
    caller.trim_lines("\n\nsink(0,&v1);\n");
    assert_eq!(caller.calls[0].position, Position { line: 1, column: 7 });
}

#[test]
fn emitter_positions_count_embedded_newlines_and_restart_after_taking_output() {
    use crate::prettyprint::{Emit, EmitNoMarkup, SyntaxHighlight};
    let mut emit = EmitNoMarkup::new();
    emit.print("comment\ntext\né", SyntaxHighlight::NoColor);
    assert_eq!(emit.position(), Position { line: 3, column: 2 });
    emit.take_output();
    emit.print("&v1", SyntaxHighlight::NoColor);
    assert_eq!(emit.position(), Position { line: 1, column: 3 });
}

#[test]
fn a_prototype_slot_without_a_parameter_keeps_the_record_and_its_place() {
    use crate::context::ArchContext;
    use crate::fspec::{ParameterPieces, ProtoStore, ProtoStoreInternal};
    use crate::funcdata::Funcdata;
    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };
    let mut manage = AddrSpaceManager::new();
    manage.insert_space(Rc::new(ConstantSpace::new())).unwrap();
    manage.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
    let ram = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "ram",
        false,
        8,
        1,
        2,
        addrspace_flags::hasphysical,
        1,
        1,
    ));
    manage.insert_space(Rc::clone(&ram)).unwrap();
    let glb = Rc::new(ArchContext::new(manage));
    let mut fd = Funcdata::new("f", "f", glb, Address::new(Rc::clone(&ram), 0x1000), 0x1000, 0x20)
        .unwrap();
    let byte = Rc::new(Datatype::new(1, type_metatype::TYPE_UINT));
    let mut store = ProtoStoreInternal::new(Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)));
    let pieces = ParameterPieces { addr: Address::new(ram, 0x38), type_: Some(byte), flags: 0 };
    store.set_input(1, "b", &pieces);
    fd.get_func_proto_mut().set_store(Box::new(store));
    assert!(fd.get_func_proto().get_param(0).is_none());
    let record = Record::new(&fd).expect("a sparse prototype still records its calls");
    assert_eq!(record.entry, ("ram".to_string(), 0x1000));
    assert_eq!(record.parameters, vec![None, None]);
}
