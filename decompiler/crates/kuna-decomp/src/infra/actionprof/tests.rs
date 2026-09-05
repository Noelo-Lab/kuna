//! Tests for the `KUNA_ACTION_PROF` exclusive-time table.

use super::*;

fn reset() {
    STACK.with(|s| s.borrow_mut().clear());
    TOTALS.with(|t| t.borrow_mut().clear());
    ROOT.with(|r| r.borrow_mut().clear());
}

/// `set_root` is a no-op unless the env var is set, so the tests write the
/// thread-local directly.
fn root(name: &str) {
    ROOT.with(|r| {
        let mut cur = r.borrow_mut();
        cur.clear();
        cur.push_str(name);
    });
}

/// A parent is charged only what it spends outside its child, and both rows
/// carry the root action's name.
#[test]
fn parent_time_is_exclusive_of_its_children() {
    reset();
    root("decompile");
    enter("universal");
    enter("heritage");
    std::thread::sleep(std::time::Duration::from_millis(20));
    leave();
    leave();

    let (parent, child) = TOTALS.with(|t| {
        let m = t.borrow();
        (m["decompile/universal"], m["decompile/heritage"])
    });
    assert!(child.0 >= 20_000_000, "child kept its own time: {}", child.0);
    assert!(parent.0 < 20_000_000, "parent was charged the child's time: {}", parent.0);
    assert_eq!((parent.1, child.1), (1, 1));
    reset();
}

/// Two roots do not share a row, which is what separates a function's own
/// `decompile` pass from the reduced `jumptable` pipeline.
#[test]
fn rows_are_keyed_by_root_action() {
    reset();
    for name in ["decompile", "jumptable"] {
        root(name);
        enter("universal");
        enter("heritage");
        leave();
        leave();
    }
    let keys = TOTALS.with(|t| {
        let mut k: Vec<String> = t.borrow().keys().cloned().collect();
        k.sort();
        k
    });
    assert_eq!(
        keys,
        [
            "decompile/heritage",
            "decompile/universal",
            "jumptable/heritage",
            "jumptable/universal"
        ]
    );
    reset();
}

/// `render` sums to the recorded total and puts the most expensive row first.
#[test]
fn render_sorts_by_cost_and_totals() {
    reset();
    root("decompile");
    enter("universal");
    enter("cheap");
    leave();
    enter("dear");
    std::thread::sleep(std::time::Duration::from_millis(15));
    leave();
    leave();

    let text = render();
    let body: Vec<&str> = text.lines().skip(1).collect();
    assert!(body[0].ends_with("decompile/dear"), "dearest row first: {text}");
    assert!(text.starts_with("total_exclusive_ms "), "{text}");
    reset();
}

/// An unbalanced `leave` is a no-op rather than a panic: the instrument must
/// never be able to take the engine down.
#[test]
fn leave_without_enter_is_inert() {
    reset();
    leave();
    assert!(TOTALS.with(|t| t.borrow().is_empty()));
}
