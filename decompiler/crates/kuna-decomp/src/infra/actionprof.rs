//! Exclusive-time profiling of the Action tree, enabled by `KUNA_ACTION_PROF`.
//!
//! Each thread accumulates `<root>/<action>` rows. A group excludes time spent
//! in its children. Closing the outermost frame, including during panic unwind,
//! rewrites the configured file with that thread's totals, sorted by cost and
//! then name. Writes are best-effort; threads and worker processes do not merge
//! their tables. When disabled, each `apply` performs one cached flag check.

use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::OnceLock;
use std::time::Instant;

struct OpenFrame {
    key: String,
    at: Instant,
    children: u128,
}

#[derive(Default)]
struct Profiler {
    root: String,
    stack: Vec<OpenFrame>,
    #[expect(
        clippy::disallowed_types,
        reason = "recording is lookup-only; rendered rows are fully sorted"
    )]
    totals: std::collections::HashMap<String, (u128, u64)>,
}

impl Profiler {
    fn close(&mut self, now: Instant) -> bool {
        let Some(frame) = self.stack.pop() else {
            return false;
        };
        let elapsed = now.saturating_duration_since(frame.at).as_nanos();
        if let Some(parent) = self.stack.last_mut() {
            parent.children += elapsed;
        }
        let row = self.totals.entry(frame.key).or_insert((0, 0));
        row.0 += elapsed.saturating_sub(frame.children);
        row.1 += 1;
        self.stack.is_empty()
    }

    fn render(&self) -> String {
        let mut rows: Vec<_> = self.totals.iter().collect();
        rows.sort_by(|(key_a, total_a), (key_b, total_b)| {
            total_b.0.cmp(&total_a.0).then_with(|| key_a.cmp(key_b))
        });
        let total: u128 = rows.iter().map(|(_, value)| value.0).sum();
        let mut out = format!("total_exclusive_ms {:.1}\n", total as f64 / 1e6);
        for (key, &(ns, calls)) in rows {
            let pct = if total == 0 {
                0.0
            } else {
                ns as f64 / total as f64 * 100.0
            };
            out.push_str(&format!(
                "{:>10.1} ms  {pct:>6.2}%  {calls:>9} calls  {key}\n",
                ns as f64 / 1e6
            ));
        }
        out
    }
}

thread_local! {
    static PROFILE: RefCell<Profiler> = RefCell::new(Profiler::default());
}

/// A timing frame must close on its originating thread, including on unwind.
#[must_use]
pub(crate) struct ActionFrame(PhantomData<Rc<()>>);

impl ActionFrame {
    pub(crate) fn new(name: &str) -> Self {
        enter(name);
        Self(PhantomData)
    }
}

impl Drop for ActionFrame {
    fn drop(&mut self) {
        leave();
    }
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
    PROFILE.with(|p| {
        let mut profiler = p.borrow_mut();
        profiler.root.clear();
        profiler.root.push_str(name);
    });
}

/// Open a frame for the action named `name`.
///
/// Rows are keyed `<root>/<name>` — the schedule [`set_root`] last named, and
/// the action inside it.
pub fn enter(name: &str) {
    PROFILE.with(|p| {
        let mut profiler = p.borrow_mut();
        let root = &profiler.root;
        let key = if root.is_empty() {
            name.to_string()
        } else {
            format!("{root}/{name}")
        };
        profiler.stack.push(OpenFrame {
            key,
            at: Instant::now(),
            children: 0,
        });
    });
}

/// Close the innermost frame, charging its exclusive time.
///
/// Writes the table out whenever the schedule unwinds to empty.
pub fn leave() {
    let unwound = PROFILE.with(|p| p.borrow_mut().close(Instant::now()));
    if unwound {
        dump();
    }
}

/// Render the running totals to the path in [`ENV_VAR`].
///
/// A write failure is ignored: a profile that cannot be written must not change
/// what the engine does.
pub fn dump() {
    let Some(path) = std::env::var_os(ENV_VAR) else {
        return;
    };
    let _ = std::fs::write(path, render());
}

/// The table, as text.
pub fn render() -> String {
    PROFILE.with(|p| p.borrow().render())
}

#[cfg(test)]
mod tests;
