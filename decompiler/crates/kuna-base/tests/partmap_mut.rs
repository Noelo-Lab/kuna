use kuna_base::partmap::PartMap;

#[test]
fn mutable_lookup_updates_only_its_existing_interval() {
    let mut map = PartMap::new(vec![0]);
    *map.split(&10) = vec![10];
    *map.split(&20) = vec![20];
    for (point, value) in [(9, 1), (10, 2), (19, 3), (20, 4), (i32::MAX, 5)] {
        map.get_value_mut(&point).push(value);
    }
    assert_eq!(map.default_value(), &[0, 1]);
    assert_eq!(map.get_value(&10), &[10, 2, 3]);
    assert_eq!(map.get_value(&19), &[10, 2, 3]);
    assert_eq!(map.get_value(&20), &[20, 4, 5]);
    assert_eq!(map.get_value(&i32::MAX), &[20, 4, 5]);
    assert_eq!(map.iter().map(|(key, _)| *key).collect::<Vec<_>>(), [10, 20]);
}
