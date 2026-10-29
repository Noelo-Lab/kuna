use super::*;
use crate::action::ActionGroupList;

#[test]
fn mask_covers_the_count_widths() {
    assert_eq!(mask(1), 0xff);
    assert_eq!(mask(4), 0xffff_ffff);
    assert_eq!(mask(8), u64::MAX);
}

#[test]
fn the_action_keeps_its_group_and_override_when_cloned() {
    let a = ActionStrlenScan::new(true, "returnsplit");
    assert_eq!(a.get_name(), "strlenscan");
    assert!(a.clone_filtered(&ActionGroupList::new()).is_none());
    let b = a.clone_filtered(&ActionGroupList::from_names(["returnsplit"])).expect("returnsplit is in the list");
    assert_eq!(b.get_name(), "strlenscan");
}
