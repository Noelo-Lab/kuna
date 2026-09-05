//! (kuna) `KUNA_ACTION_PROF` — an exclusive-time profile of the Action tree.
//!
//! # Why this exists
//!
//! `perf` is unavailable on the machines this engine is tuned on
//! (`perf_event_paranoid = 4`), and a sampling profiler stops being useful the
//! moment a profile goes flat: it cannot tell that two 15% frames are the same
//! cost reached down two paths, and it cannot count calls at all. Every
//! performance investigation on a large function has therefore rebuilt the same
//! throwaway timer around [`Action::perform`](crate::action::Action::perform)'s
//! `apply` call. This is that timer, kept.
//!
//! Set `KUNA_ACTION_PROF` to a path and the engine writes an exclusive-time
//! table there, sorted by cost:
//!
//! ```text
//! total_exclusive_ms 7436.1
//!     1627.8 ms   21.89%         25 calls  decompile/heritage
//!     1282.9 ms   17.25%        131 calls  decompile/oppool1
//!      184.0 ms    2.47%          9 calls  jumptable/heritage
//! ```
//!
//! Time is **exclusive**: an [`ActionGroup`](crate::action::ActionGroup) is
//! charged only what it spends outside its children, so the rows sum to the
//! wall time of the schedule and a container never hides a leaf. Each row is
//! keyed by the *root* action the work ran under, which separates a function's
//! own `decompile` pass from the reduced `jumptable` pipeline that jump-table
//! recovery runs on a partial clone.
//!
//! The file is rewritten every time the schedule unwinds, so it holds the
//! running total for the whole process — one `decompile-all` leaves one table
//! covering every function.
//!
//! # Cost when off
//!
//! One `OnceLock` load per `apply` call. `apply` is coarse — a pool applies
//! every rule to every op in a single call — so a large function makes only a
//! few hundred of them.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::time::Instant;

thread_local! {
    /// The open `apply` frames: (row key, entry time, time charged to children).
    static STACK: RefCell<Vec<(String, Instant, u128)>> = const { RefCell::new(Vec::new()) };
    /// Exclusive nanoseconds and call count per row key.
    static TOTALS: RefCell<HashMap<String, (u128, u64)>> = RefCell::new(HashMap::new());
    /// The root schedule rows are attributed to, set by
    /// [`ActionDatabase::set_current`](crate::action::ActionDatabase::set_current).
    static ROOT: RefCell<String> = const { RefCell::new(String::new()) };
}

/// The env var that names the output path.
pub const ENV_VAR: &str = "KUNA_ACTION_PROF";

/// Is profiling on? Read once per process.
pub fn enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os(ENV_VAR).is_some())
}

/// Name the root schedule that follows, so its rows can be told apart from
/// another root's.
///
/// The derived root Action keeps the universal tree's own name, so the only
/// place that knows a schedule is `decompile` rather than the reduced
/// `jumptable` pipeline is the database that selected it.
pub fn set_root(name: &str) {
    if !enabled() {
        return;
    }
    ROOT.with(|r| {
        let mut cur = r.borrow_mut();
        cur.clear();
        cur.push_str(name);
    });
}

/// Open a frame for the action named `name`.
///
/// Rows are keyed `<root>/<name>` — the schedule [`set_root`] last named, and
/// the action inside it.
pub fn enter(name: &str) {
    let key = ROOT.with(|r| {
        let root = r.borrow();
        if root.is_empty() { name.to_string() } else { format!("{root}/{name}") }
    });
    STACK.with(|s| s.borrow_mut().push((key, Instant::now(), 0)));
}

/// Close the innermost frame, charging its exclusive time.
///
/// Writes the table out whenever the schedule unwinds to empty.
pub fn leave() {
    let closed = STACK.with(|s| {
        let mut st = s.borrow_mut();
        let (key, at, children) = st.pop()?;
        let elapsed = at.elapsed().as_nanos();
        if let Some(parent) = st.last_mut() {
            parent.2 += elapsed;
        }
        Some((key, elapsed.saturating_sub(children), st.is_empty()))
    });
    let Some((key, exclusive, unwound)) = closed else { return };
    TOTALS.with(|t| {
        let mut m = t.borrow_mut();
        let row = m.entry(key).or_insert((0, 0));
        row.0 += exclusive;
        row.1 += 1;
    });
    if unwound {
        dump();
    }
}

/// Render the running totals to the path in [`ENV_VAR`].
///
/// A write failure is ignored: a profile that cannot be written must not change
/// what the engine does.
pub fn dump() {
    let Some(path) = std::env::var_os(ENV_VAR) else { return };
    let _ = std::fs::write(path, render());
}

/// The table, as text.
pub fn render() -> String {
    let mut rows: Vec<(String, u128, u64)> =
        TOTALS.with(|t| t.borrow().iter().map(|(k, v)| (k.clone(), v.0, v.1)).collect());
    rows.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    let total: u128 = rows.iter().map(|r| r.1).sum();
    let mut out = format!("total_exclusive_ms {:.1}\n", total as f64 / 1e6);
    for (key, ns, calls) in rows {
        let pct = if total == 0 { 0.0 } else { ns as f64 / total as f64 * 100.0 };
        out.push_str(&format!(
            "{:>10.1} ms  {pct:>6.2}%  {calls:>9} calls  {key}\n",
            ns as f64 / 1e6
        ));
    }
    out
}

#[cfg(test)]
mod tests;
