//! Deterministic accounting and byte-format checks for action profiles.

use super::*;
use std::time::Duration;

fn frame(key: &str, at: Instant, children: u128) -> OpenFrame {
    OpenFrame {
        key: key.to_owned(),
        at,
        children,
    }
}

#[test]
fn parent_time_is_exclusive_of_its_children() {
    let at = Instant::now();
    let mut profiler = Profiler::default();
    profiler.stack.push(frame("decompile/universal", at, 0));
    profiler.stack.push(frame(
        "decompile/heritage",
        at + Duration::from_nanos(10),
        0,
    ));
    assert!(!profiler.close(at + Duration::from_nanos(30)));
    profiler
        .stack
        .push(frame("decompile/types", at + Duration::from_nanos(40), 0));
    assert!(!profiler.close(at + Duration::from_nanos(70)));
    assert!(profiler.close(at + Duration::from_nanos(100)));

    assert_eq!(profiler.totals["decompile/universal"], (50, 1));
    assert_eq!(profiler.totals["decompile/heritage"], (20, 1));
    assert_eq!(profiler.totals["decompile/types"], (30, 1));
}

#[test]
fn rows_capture_the_root_at_entry() {
    PROFILE.with(|p| *p.borrow_mut() = Profiler::default());
    enter("unrooted");
    PROFILE.with(|p| p.borrow_mut().root = "decompile".into());
    enter("universal");
    PROFILE.with(|p| p.borrow_mut().root = "jumptable".into());
    enter("universal");
    PROFILE.with(|p| {
        let mut profiler = p.borrow_mut();
        for _ in 0..3 {
            profiler.close(Instant::now());
        }
        let mut keys: Vec<_> = profiler.totals.keys().map(String::as_str).collect();
        keys.sort();
        assert_eq!(
            keys,
            ["decompile/universal", "jumptable/universal", "unrooted"]
        );
        *profiler = Profiler::default();
    });
}

#[test]
fn render_preserves_format_and_breaks_cost_ties_by_name() {
    let mut profiler = Profiler::default();
    assert_eq!(profiler.render(), "total_exclusive_ms 0.0\n");
    profiler.totals.extend([
        ("root/z".to_owned(), (2_000_000, 3)),
        ("root/a".to_owned(), (2_000_000, 1)),
        ("root/dear".to_owned(), (6_000_000, 2)),
        ("root/zero".to_owned(), (0, 4)),
    ]);
    assert_eq!(
        profiler.render(),
        concat!(
            "total_exclusive_ms 10.0\n",
            "       6.0 ms   60.00%          2 calls  root/dear\n",
            "       2.0 ms   20.00%          1 calls  root/a\n",
            "       2.0 ms   20.00%          3 calls  root/z\n",
            "       0.0 ms    0.00%          4 calls  root/zero\n",
        )
    );
    profiler.totals.retain(|key, _| key == "root/zero");
    assert_eq!(
        profiler.render(),
        "total_exclusive_ms 0.0\n       0.0 ms    0.00%          4 calls  root/zero\n"
    );
}

#[test]
fn repeated_and_nested_frames_accumulate_in_one_row() {
    let at = Instant::now();
    let mut profiler = Profiler::default();
    profiler.stack.push(frame("root/action", at, 0));
    profiler
        .stack
        .push(frame("root/action", at + Duration::from_nanos(10), 0));
    assert!(!profiler.close(at + Duration::from_nanos(30)));
    assert!(profiler.close(at + Duration::from_nanos(50)));
    profiler
        .stack
        .push(frame("root/action", at + Duration::from_nanos(60), 0));
    assert!(profiler.close(at + Duration::from_nanos(90)));
    assert_eq!(profiler.totals["root/action"], (80, 3));
}

#[test]
fn child_time_larger_than_elapsed_saturates() {
    let at = Instant::now();
    let mut profiler = Profiler::default();
    profiler.stack.push(frame("root/action", at, 100));
    assert!(profiler.close(at + Duration::from_nanos(50)));
    assert_eq!(profiler.totals["root/action"], (0, 1));
}

#[test]
fn leave_without_enter_is_inert() {
    let mut profiler = Profiler::default();
    assert!(!profiler.close(Instant::now()));
    assert!(profiler.totals.is_empty());
    PROFILE.with(|p| *p.borrow_mut() = profiler);
    leave();
    assert_eq!(render(), "total_exclusive_ms 0.0\n");
}
