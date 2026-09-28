use super::{shard, SynthPlan, SynthRequest};

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
