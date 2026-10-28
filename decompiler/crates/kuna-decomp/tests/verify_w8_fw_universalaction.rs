//! Empty and unmatched action filters must suppress the rendered schedule.

use kuna_decomp::universalaction::{universal_sched, ActionListFilter};

#[test]
fn empty_or_unmatched_filter_drops_whole_tree() {
    let empty = ActionListFilter::from_names(Vec::<String>::new());
    let none = universal_sched(None, None, vec![]).list_action_dump(&empty);
    assert_eq!(none, "", "empty filter must render nothing");

    let bogus = ActionListFilter::from_names(["xyzzy_not_a_group"]);
    let none = universal_sched(None, None, vec![]).list_action_dump(&bogus);
    assert_eq!(none, "", "unmatched filter must render nothing");
}
