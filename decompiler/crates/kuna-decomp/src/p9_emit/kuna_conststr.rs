//! (kuna `conststr`) A character-pointer constant prints as the string it
//! addresses in the two cases upstream's literal falls through to the address.
//!
//! `PrintC::pushConstant`'s pointer arm prints a character-pointer constant as
//! the string at its address when the string manager accepts the bytes there,
//! and otherwise as a forced-hex integer behind a cast (or, once `globalref`
//! names program data, as `&dat_<addr>`). Two strings fall through:
//!
//! - The empty tail of another string. A linker that merges string constants
//!   stores the program's `""` as the terminating NUL of some other literal
//!   (`"tab\there\n"` ends where `""` starts), and `emptystrconst` declines a
//!   zero-character literal whose FOLLOWING bytes are not string data, so
//!   `nanf("")` prints `nanf((char *)0x2011)`. The bytes BEFORE settle it: a NUL
//!   that ends a run of text, itself begun at a NUL, is a string's terminator
//!   and therefore a genuine `""`.
//! - A string whose bytes are not valid UTF-8 (`"\xa1\ae"`, the GB18030 quote
//!   gnulib's `gettext_quote` returns). The string manager rejects the encoding.
//!   Here every byte up to the NUL is spelled by value: a byte that starts a
//!   valid UTF-8 character as upstream spells it when that spelling is its own
//!   bytes, any other as a `\x` escape, and the literal is split (`"\xa1" "e"`)
//!   where a hex digit would otherwise extend the escape. The literal's bytes
//!   are exactly the image's. A pointer-aligned address whose word is the
//!   address of program data or code is a pointer table (kmod's command table
//!   read through a merged `char *`), and keeps its address.
//!
//! Both need what upstream's literal needs -- a one-byte character pointee, a
//! read-only address, C output -- and an address inside a section of program
//! data (code is read-only too); a buffer the program writes to never prints as
//! a literal, and no literal is formed from two addresses.

use kuna_base::types::uint1;

/// How far back from an empty literal the terminated string is looked for.
pub const TAIL_WINDOW: usize = 64;

/// Is the NUL just after `before` the terminator of a string?
///
/// `before` holds the bytes that precede the NUL, nearest last, as far back as
/// they could be read (at most [`TAIL_WINDOW`]). The run of non-NUL bytes that
/// ends at the NUL must be text -- valid UTF-8 with no control character but
/// tab, newline and carriage return, as the analysis tier's string recognizer
/// reads it -- at least two characters of it, and must itself start just after
/// a NUL, or fill the whole window.
pub fn terminates_string(before: &[uint1]) -> bool {
    let run = before.iter().rev().take_while(|&&b| b != 0).count();
    let text = std::str::from_utf8(&before[before.len() - run..])
        .is_ok_and(|t| t.chars().count() >= 2 && t.chars().all(|c| !c.is_control() || matches!(c, '\t' | '\n' | '\r')));
    text && (run < before.len() || before.len() == TAIL_WINDOW)
}

/// The C string literal of the NUL-terminated byte string `bytes` (without the
/// NUL), or `None` when it is empty. A valid UTF-8 character is spelled as
/// upstream spells it ([`crate::printc::print_unicode`]) when that spelling is
/// its own bytes: an ASCII character, escaped or not, or a multi-byte one that
/// prints raw. Every other byte is `\xNN`, so the literal holds exactly the
/// image's bytes (upstream escapes a codepoint, `\x80` for the two bytes
/// `c2 80`), and the literal is split (`"\xa1" "e"`) where the next character
/// would be read as more hex digits of an escape.
pub fn byte_literal(bytes: &[uint1]) -> Option<String> {
    use crate::printlanguage::unicode_needs_escape;
    use crate::stringmanage::StringManager;
    if bytes.is_empty() {
        return None;
    }
    let mut s = String::from("\"");
    let mut after_hex = false;
    let mut i = 0;
    while i < bytes.len() {
        let mut piece = String::new();
        let mut skip = 1;
        let cp = StringManager::get_codepoint(&bytes[i..], 1, false, &mut skip);
        let n = if cp > 0 && skip > 0 && (skip == 1 || !unicode_needs_escape(cp)) {
            crate::printc::print_unicode(&mut piece, cp);
            skip as usize
        } else {
            let n = if cp > 0 && skip > 0 { skip as usize } else { 1 };
            for b in &bytes[i..i + n] {
                piece.push_str(&format!("\\x{b:02x}"));
            }
            n
        };
        if after_hex && piece.starts_with(|c: char| c.is_ascii_hexdigit()) {
            s.push_str("\" \"");
        }
        after_hex = piece.starts_with("\\x");
        s.push_str(&piece);
        i += n;
    }
    s.push('"');
    Some(s)
}

/// Is the pointer-sized `word` (in the image's byte order) the address of
/// program data (`data`, [`crate::kuna_globalref::in_ranges`]) or of code
/// (`code`, [`crate::kuna_litpoolconst::contains`])? Both are sorted, merged,
/// inclusive range lists.
pub fn is_image_pointer(word: &[uint1], bigend: bool, data: &[(u64, u64)], code: &[(u64, u64)]) -> bool {
    let v = if bigend {
        word.iter().fold(0u64, |acc, &b| (acc << 8) | u64::from(b))
    } else {
        word.iter().rev().fold(0u64, |acc, &b| (acc << 8) | u64::from(b))
    };
    v != 0 && (crate::kuna_globalref::in_ranges(data, v) || crate::kuna_litpoolconst::contains(code, v, 1))
}

#[cfg(test)]
#[path = "kuna_conststr/tests.rs"]
mod tests;
