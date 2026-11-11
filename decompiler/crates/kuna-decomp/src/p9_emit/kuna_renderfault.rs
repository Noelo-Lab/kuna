//! (kuna) Test-only render fault: `KUNA_RENDER_FAULT=<addr>` (`0x` hex or
//! decimal) panics the printer partway through the document of the function
//! whose entry is `<addr>`, after its opening brace. It exists so a test can
//! check that one failed render leaves every later function of the batch
//! rendered as the run configured the printer. Unset in every real run.

use std::sync::OnceLock;

pub const RENDER_FAULT_ENV: &str = "KUNA_RENDER_FAULT";

fn armed() -> Option<u64> {
    static AT: OnceLock<Option<u64>> = OnceLock::new();
    *AT.get_or_init(|| {
        let raw = std::env::var(RENDER_FAULT_ENV).ok()?;
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
        panic!("{RENDER_FAULT_ENV}: injected render panic at {entry:#x}");
    }
}
