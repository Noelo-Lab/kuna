use super::{only_types_are_renamed, serial_run, shard, PoolConfig, SynthPlan, SynthRequest};
use super::super::tests::{cfg, sample_result};

fn plan() -> SynthPlan {
    SynthPlan {
        first: Vec::new(),
        sweep: Vec::new(),
        stale: Vec::new(),
        table: Vec::new(),
        minted: Default::default(),
    }
}

fn request(size: u32) -> SynthRequest {
    let mut wire = b"\x01\0\0\0\x01\0\0\0x".to_vec();
    wire.extend(size.to_le_bytes());
    wire.extend(size.to_le_bytes());
    wire.extend([0; 16]);
    shard::decode_table(&wire).unwrap().pop().unwrap().1
}

#[test]
fn absent_answers_and_answer_count_are_part_of_the_key() {
    let plan = plan();
    assert!(plan.matches(&[], &[]));
    assert!(plan.matches(&[None], &[None]));
    assert!(!plan.matches(&[None], &[]));
    assert!(!plan.matches(&[], &[None]));
    assert!(!plan.matches(&[None], &[None, None]));
    assert!(!plan.matches(&[None, None], &[None]));
}

#[test]
fn held_names_must_match_and_cannot_become_minted_names() {
    let mut plan = plan();
    let names = vec![Some("held".into())];
    let key = plan.key(&names);
    assert_eq!(key, [Some(("held".into(), None))]);
    assert!(plan.matches(&names, &key));
    assert!(!plan.matches(&[Some("other".into())], &key));
    assert!(!plan.matches(&[None], &key));
    plan.minted.insert("held".into(), request(4));
    assert!(!plan.matches(&names, &key));
}

#[test]
fn an_unchanged_name_does_not_hide_a_changed_definition() {
    let mut plan = plan();
    plan.minted.insert("struct_0".into(), request(4));
    let names = vec![Some("struct_0".into())];
    let key = plan.key(&names);
    assert!(plan.matches(&names, &key));
    plan.minted.insert("struct_0".into(), request(8));
    assert!(!plan.matches(&names, &key));
    plan.minted.remove("struct_0");
    assert!(!plan.matches(&names, &key));
}

#[test]
fn borrowed_comparison_agrees_with_owned_key_equality() {
    let mut plan = plan();
    plan.minted.insert("struct_0".into(), request(4));
    plan.minted.insert("struct_1".into(), request(8));
    let choices = [
        None,
        Some("held".into()),
        Some("struct_0".into()),
        Some("struct_1".into()),
    ];
    for code in 0usize..1024 {
        let names: Vec<_> = (0..5)
            .map(|i| choices[(code >> (i * 2)) & 3].clone())
            .collect();
        let key = plan.key(&names);
        assert!(plan.matches(&names, &key));
        for index in 0..key.len() {
            for replacement in [
                None,
                Some(("held".into(), None)),
                Some(("struct_0".into(), Some(request(8)))),
            ] {
                let mut changed = key.clone();
                changed[index] = replacement;
                assert_eq!(plan.matches(&names, &changed), key == changed);
            }
        }
    }
}

/// A `struct_N` that is a symbol's name, not a type's, must not be rewritten
/// with the type: such a function is decompiled again instead.
#[test]
fn a_rename_covering_a_symbol_name_is_refused() {
    let map = [("struct_3".to_string(), "struct_0".to_string())];
    let mut r = sample_result();
    assert!(only_types_are_renamed(&r, &map));
    r.variables[0].name = "struct_3".into();
    assert!(!only_types_are_renamed(&r, &map));
    r.variables[0].name = "param_1".into();
    r.aliases.push("struct_3".into());
    assert!(!only_types_are_renamed(&r, &map));
    r.aliases.clear();
    r.name = "struct_3".into();
    assert!(!only_types_are_renamed(&r, &map));
}

/// (kuna `protoorder`) The names a pool replays are a serial run's, and on
/// `decompile-all` that run is the one without the callee-first order: the
/// report has to name it, not `--jobs 1`.
#[test]
fn the_report_names_the_serial_run_it_can_actually_replay() {
    assert_eq!(serial_run(&cfg(0, 0.0)), "--jobs 1");
    let callee_first = PoolConfig { serial_callee_first: true, ..cfg(0, 0.0) };
    assert_eq!(serial_run(&callee_first), "--jobs 1 --option protoorder off");
}

#[test]
fn a_structure_rename_rebases_recorded_pointer_arguments() {
    use kuna_decomp::kuna_pointerargs::{Call, Position, Record};
    let mut r = sample_result();
    r.code = Some("struct_3 *p; sink(&v1);".into());
    r.pointerargs = Some(Record {
        entry: ("ram".into(), 0x2000), parameters: vec![],
        calls: vec![Call {
            callee: ("ram".into(), 0x1000), index: 0,
            storage: (("register".into(), 0x38), 8), actual: "long *".into(),
            position: Position { line: 1, column: 17 }, expression: "&v1".into(),
        }],
    });
    let own = [Some(("struct_3".into(), Some(request(4))))];
    let serial = [Some(("struct_100".into(), Some(request(4))))];
    let renamed = super::rename_result(&r, Some(&own), &serial, &[]).unwrap();
    assert_eq!(renamed.code.as_deref(), Some("struct_100 *p; sink(&v1);"));
    assert_eq!(renamed.pointerargs.unwrap().calls[0].position.column, 19);
}
