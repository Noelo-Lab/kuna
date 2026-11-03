//! (kuna `cfstrings`) Render a Mach-O Core Foundation constant string as the
//! `CFSTR("...")` it was written as.
//!
//! The `cfstrings` commit seeds each `__cfstring` entry address with the string
//! its char pointer resolves to, so `PrintC::push_ptr_char_constant_ir` finds a
//! literal there. The value is still the entry, a `CFStringRef`, not the
//! characters, so the C printer wraps the literal in `CFSTR(...)`; with
//! `cfstrings annotate` it also names the entry address in a comment, which is
//! the address a listing or hex view shows for it.

/// Wrap the already-escaped C literal `lit` for the `__cfstring` entry at `entry`.
pub fn wrap(lit: &str, entry: u64, annotate: bool) -> String {
    if annotate {
        format!("CFSTR({lit}) /* cfstring@0x{entry:x} */")
    } else {
        format!("CFSTR({lit})")
    }
}

#[cfg(test)]
mod tests {
    use super::wrap;

    #[test]
    fn wraps_and_annotates() {
        assert_eq!(wrap("\"hi\"", 0x100004008, false), "CFSTR(\"hi\")");
        assert_eq!(wrap("\"hi\"", 0x100004008, true), "CFSTR(\"hi\") /* cfstring@0x100004008 */");
    }
}
