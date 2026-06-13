//! C++ `test.cc` / `test.hh`: the minimal unit-test framework that drives the
//! 207 engine unit tests (`UnitTest`), plus the `decomp_test_dbg`-equivalent
//! console summary text the Python harness (`kuna/run_tests.py`) parses off
//! **stderr**.
//!
//! ## What is and is not ported
//!
//! The C++ `TEST(name) { ... }` macro registers a free function into a process-
//! global `vector<UnitTest *>` via a static initializer, and `UnitTest::run`
//! walks that registry, printing the exact lines
//!
//! ```text
//! testing : <name> ...
//!   passed.
//!   fail: <msg>
//! ==============================
//! <passed>/<total> tests passed.
//! ```
//!
//! The Rust port keeps the **output grammar byte-faithful** (it is the contract
//! `run_tests.py` greps) and keeps the run-loop semantics (name filtering,
//! pass/fail counting, the `LowlevelError::explain` vs catch-all distinction),
//! but represents a unit test as an explicit [`UnitTest`] value in a
//! [`UnitTestSuite`] rather than a magic static registry: Rust has no
//! constructor-time global registration (no portable `__attribute__((constructor))`
//! equivalent without extra crates, banned here), and the engine's `TEST(...)`
//! bodies live in the kuna-decomp/kuna-base crates' own `#[test]` functions
//! already.  This module is the *framework* — the harness binary collects the
//! ported engine unit tests into a [`UnitTestSuite`] and calls [`UnitTestSuite::run`].
//!
//! A test body signals failure by returning `Err` (mirroring the C++ `throw`):
//! a [`UnitTestError`] whose message is printed after `  fail: ` (the
//! `LowlevelError &err` arm, `err.explain`), or the unit form [`UnitTestError::anon`]
//! for the catch-all `catch(...)` arm that prints the bare `  fail` with no
//! message.

use std::fmt;

/// The error a unit-test body returns to signal failure (C++: `throw`).
///
/// Two shapes, matching the two `catch` arms of C++ `UnitTest::run`:
/// - [`UnitTestError::msg`] → the `catch(LowlevelError &err)` arm: prints
///   `  fail: <explain>`.
/// - [`UnitTestError::anon`] → the `catch(...)` arm: prints the bare `  fail`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitTestError {
    /// The `LowlevelError::explain` message, or `None` for the catch-all arm.
    pub explain: Option<String>,
}

impl UnitTestError {
    /// A failure carrying an explanation (the `LowlevelError` arm).
    pub fn msg(explain: impl Into<String>) -> Self {
        UnitTestError { explain: Some(explain.into()) }
    }

    /// An anonymous failure (the `catch(...)` arm: bare `  fail`).
    pub fn anon() -> Self {
        UnitTestError { explain: None }
    }
}

impl fmt::Display for UnitTestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.explain {
            Some(e) => f.write_str(e),
            None => f.write_str("<anonymous failure>"),
        }
    }
}

impl std::error::Error for UnitTestError {}

/// The result of a unit-test body: `Ok(())` on pass, `Err` on fail.
pub type UnitTestResult = Result<(), UnitTestError>;

/// A unit-test call-back (C++ `testfunc_t = void (*)()`, throwing on failure).
pub type UnitTestFn = Box<dyn Fn() -> UnitTestResult>;

/// C++ `struct UnitTest` (`test.hh`): a named test with its call-back.
pub struct UnitTest {
    /// C++ `name`: the identifier printed in the summary.
    pub name: String,
    /// C++ `func`: the call-back executing the test.
    pub func: UnitTestFn,
}

impl UnitTest {
    /// C++ `UnitTest(name, func)` (without the static-registry side effect,
    /// which the [`UnitTestSuite`] supplies explicitly).
    pub fn new(name: impl Into<String>, func: UnitTestFn) -> Self {
        UnitTest { name: name.into(), func }
    }
}

/// The collection of registered unit tests (C++ `UnitTest::tests()`, the static
/// `vector<UnitTest *>`), made an explicit value.
#[derive(Default)]
pub struct UnitTestSuite {
    tests: Vec<UnitTest>,
}

impl UnitTestSuite {
    /// An empty suite.
    pub fn new() -> Self {
        UnitTestSuite::default()
    }

    /// Register a test (the C++ `TEST` macro's `tests().push_back(this)`).
    pub fn register(&mut self, test: UnitTest) {
        self.tests.push(test);
    }

    /// Convenience: register a name + body in one call.
    pub fn add(&mut self, name: impl Into<String>, func: UnitTestFn) {
        self.register(UnitTest::new(name, func));
    }

    /// The number of registered tests.
    pub fn len(&self) -> usize {
        self.tests.len()
    }

    /// Whether no tests are registered.
    pub fn is_empty(&self) -> bool {
        self.tests.is_empty()
    }

    /// C++ `UnitTest::run(set<string> &testNames)`.
    ///
    /// Run all the tests unless a non-empty set of names is passed in, in which
    /// case only the named tests in the set are run.  Output is written to
    /// `out` exactly as C++ writes to `cerr` (the Python harness reads this off
    /// **stderr**).  Returns the number of FAILED tests (`total - passed`).
    ///
    /// `test_names` is the C++ `set<string>`; iteration order over `tests`
    /// follows registration order (the C++ `vector` order), and the
    /// membership test mirrors `testNames.find(t->name) == testNames.end()`.
    pub fn run<I, S>(&self, test_names: I, out: &mut String) -> i32
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        // C++ `set<string> &testNames`: a sorted set membership filter.  Empty
        // means "run everything".
        let names: std::collections::BTreeSet<String> =
            test_names.into_iter().map(Into::into).collect();

        let mut total = 0i32;
        let mut passed = 0i32;

        for t in &self.tests {
            // C++: if (testNames.size() > 0 && testNames.find(t->name) == end) continue;
            if !names.is_empty() && !names.contains(&t.name) {
                continue;
            }
            out.push_str("testing : ");
            out.push_str(&t.name);
            out.push_str(" ...\n");
            total += 1;
            match (t.func)() {
                Ok(()) => {
                    passed += 1;
                    out.push_str("  passed.\n");
                }
                // C++ `catch(LowlevelError &err)`: print `  fail: <explain>`.
                Err(UnitTestError { explain: Some(msg) }) => {
                    out.push_str("  fail: ");
                    out.push_str(&msg);
                    out.push('\n');
                }
                // C++ `catch(...)`: print the bare `  fail`.
                Err(UnitTestError { explain: None }) => {
                    out.push_str("  fail\n");
                }
            }
        }
        out.push_str("==============================\n");
        // C++ `cerr << passed << "/" << total << " tests passed." << endl;`
        out.push_str(&format!("{passed}/{total} tests passed.\n"));
        total - passed
    }
}

/// C++ free function `add_exit_code(int current, int add)` (`test.cc`):
/// a saturating add for a CLI exit code, clamped to `[0,255]`.
///
/// Faithful transcription including the C++ sanity checks: a negative
/// `current`, either operand over the clamp, or an overflow (`ret < current`)
/// all clamp to 255.  `wrapping_add` reproduces the C++ `int` overflow that the
/// `ret < current` guard then catches.
pub fn add_exit_code(current: i32, add: i32) -> i32 {
    const CLAMP: i32 = 255;
    let mut ret = current.wrapping_add(add);
    // Faithful transcription of the C++ four-way `||` guard; the `ret < current`
    // overflow check is not a range-membership test, so the chain is kept whole
    // (clippy's range-contains rewrite would only fold the first two terms and
    // obscure the 1:1 mapping to the C++ source).
    #[allow(clippy::manual_range_contains)]
    if current < 0          // Sanity checks
        || current > CLAMP
        || ret < current    // Can only happen due to overflow
        || ret > CLAMP
    {
        ret = CLAMP; // Set to max exit code
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    fn suite_3() -> UnitTestSuite {
        let mut s = UnitTestSuite::new();
        s.add("alpha", Box::new(|| Ok(())));
        s.add("beta", Box::new(|| Err(UnitTestError::msg("two != three"))));
        s.add("gamma", Box::new(|| Err(UnitTestError::anon())));
        s
    }

    #[test]
    fn run_all_emits_exact_grammar() {
        let s = suite_3();
        let mut out = String::new();
        let failed = s.run(Vec::<String>::new(), &mut out);
        assert_eq!(failed, 2); // beta + gamma fail
        let expect = "\
testing : alpha ...
  passed.
testing : beta ...
  fail: two != three
testing : gamma ...
  fail
==============================
1/3 tests passed.
";
        assert_eq!(out, expect);
    }

    #[test]
    fn name_filter_runs_only_named() {
        let s = suite_3();
        let mut out = String::new();
        let failed = s.run(["alpha"], &mut out);
        assert_eq!(failed, 0);
        assert_eq!(
            out,
            "testing : alpha ...\n  passed.\n==============================\n1/1 tests passed.\n"
        );
    }

    #[test]
    fn name_filter_subset_with_a_failure() {
        let s = suite_3();
        let mut out = String::new();
        // Run beta + gamma only; alpha must not appear.
        let failed = s.run(["beta", "gamma"], &mut out);
        assert_eq!(failed, 2);
        assert!(!out.contains("alpha"));
        assert!(out.contains("0/2 tests passed."));
    }

    #[test]
    fn empty_suite_footer() {
        let s = UnitTestSuite::new();
        let mut out = String::new();
        let failed = s.run(Vec::<String>::new(), &mut out);
        assert_eq!(failed, 0);
        assert_eq!(out, "==============================\n0/0 tests passed.\n");
    }

    #[test]
    fn add_exit_code_saturates() {
        assert_eq!(add_exit_code(0, 0), 0);
        assert_eq!(add_exit_code(10, 5), 15);
        assert_eq!(add_exit_code(250, 10), 255); // over clamp
        assert_eq!(add_exit_code(255, 1), 255);
        assert_eq!(add_exit_code(-1, 5), 255); // negative current
        assert_eq!(add_exit_code(i32::MAX, 1), 255); // overflow -> ret < current
    }
}
