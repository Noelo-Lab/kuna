//! (kuna) Test-only jump-table fault: `KUNA_JUMPTABLE_FAULT=<addr>` (`0x` hex
//! or decimal) panics the jump-table recovery pipeline of the function whose
//! entry is `<addr>`. It exists so a test can check that a panic while
//! recovering a table is that function's error, in a serial run and under
//! `--jobs`. Unset in every real run.

use std::sync::OnceLock;

pub const JUMPTABLE_FAULT_ENV: &str = "KUNA_JUMPTABLE_FAULT";

fn armed() -> Option<u64> {
    static AT: OnceLock<Option<u64>> = OnceLock::new();
    *AT.get_or_init(|| {
        let raw = std::env::var(JUMPTABLE_FAULT_ENV).ok()?;
        let raw = raw.trim();
        match raw.strip_prefix("0x").or_else(|| raw.strip_prefix("0X")) {
            Some(hex) => u64::from_str_radix(hex, 16).ok(),
            None => raw.parse().ok(),
        }
    })
}

/// Panic when `entry` is the armed address.
pub(crate) fn check(entry: u64) {
    if armed() == Some(entry) {
        panic!("{JUMPTABLE_FAULT_ENV}: injected jump-table recovery panic at {entry:#x}");
    }
}
