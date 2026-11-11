use super::*;
use slotmap::SlotMap;

slotmap::new_key_type! { struct TestId; }

#[test]
fn cached_ids_match_slotmap_through_churn() {
    let mut arena = Arena::<TestId, u64>::with_key();
    let mut reference = SlotMap::<TestId, u64>::with_key();
    let mut ids = vec![
        TestId::null(),
        slotmap::KeyData::from_ffi(u64::MAX).into(),
        slotmap::KeyData::from_ffi(0).into(),
    ];
    let mut random = 1u64;
    for step in 0..1024u64 {
        random = random.wrapping_mul(6364136223846793005).wrapping_add(1);
        let id = ids[(random >> 32) as usize % ids.len()];
        if step % 127 == 126 {
            arena.clear();
            reference.clear();
        } else if step % 3 == 0 {
            assert_eq!(arena.remove(id), reference.remove(id));
        } else {
            let allocated = arena.insert(step);
            assert_eq!(allocated, reference.insert(step));
            ids.push(allocated);
        }
        if let Some(value) = arena.get_mut(id) {
            *value ^= step;
        }
        if let Some(value) = reference.get_mut(id) {
            *value ^= step;
        }
        for &key in &ids {
            assert_eq!(arena.get(key), reference.get(key), "step {step}, {key:?}");
            assert_eq!(arena.get_mut(key), reference.get_mut(key));
        }
        assert_eq!(
            arena.iter().collect::<Vec<_>>(),
            reference.iter().collect::<Vec<_>>()
        );
    }
}

#[test]
fn indexing_keeps_the_original_validity_checks() {
    let mut arena = Arena::<TestId, u64>::with_key();
    let stale = arena.insert(4);
    assert_eq!(arena[stale], 4);
    arena[stale] = 9;
    assert_eq!(arena.remove(stale), Some(9));
    let live = arena.insert(12);
    assert_ne!(stale, live);
    assert!(std::panic::catch_unwind(|| arena[stale]).is_err());
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        arena[stale] = 1;
    }))
    .is_err());
    assert_eq!(arena[live], 12);
}
