//! (kuna `conststr`) A character-pointer constant handed to a C library
//! function that reads it only as a string prints as that string, in the two
//! cases upstream's literal falls through to the address.
//!
//! `PrintC::pushConstant`'s pointer arm prints a character-pointer constant as
//! the string at its address when the string manager accepts the bytes there,
//! and otherwise as a forced-hex integer behind a cast (or, once `globalref`
//! names program data, as `&dat_<addr>`). Two strings fall through:
//!
//! - The empty tail of another string. A linker that merges string constants
//!   stores the program's `""` as the terminating NUL of some other literal,
//!   and `emptystrconst` declines a zero-character literal whose FOLLOWING
//!   bytes are not string data, so `nanf("")` prints `nanf((char *)0x2011)`.
//! - A string whose bytes are not valid UTF-8 (`"\x81\x88"`, dash's
//!   `{CTLESC, CTLQUOTEMARK}` set). The string manager rejects the encoding.
//!
//! Neither is a string by its bytes alone. The NUL that ends one string is
//! also the end pointer `s + strlen(s)` a loop compares against, and a binary
//! table with a zero byte in it does not end there. The use settles both: a
//! library parameter that reads its argument only as a NUL-terminated string,
//! and keeps and returns no pointer into it ([`STRING_PARAMS`]: `strcmp`'s
//! operands, `setlocale`'s locale, a `printf` format, `strpbrk`'s set, but not
//! `strchr`'s subject or `strtol`'s), reads exactly the bytes through the first
//! NUL and nothing of the pointer's identity. So the constant prints as a
//! literal only as that argument of a direct call; a user function, a
//! variable, a return value or any other use keeps the address.
//!
//! The literal holds exactly the image's bytes ([`byte_literal`]), at a
//! read-only address inside a section of program data, no byte of it a
//! dynamic-relocation slot (whose runtime bytes the file does not hold).

use kuna_base::types::uint1;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// C library functions and the zero-based indexes of their parameters that
/// read the argument only as a NUL-terminated string and keep and return no
/// pointer into it. Sorted by name.
pub const STRING_PARAMS: &[(&str, &[usize])] = &[
    ("__asprintf_chk", &[2]),
    ("__dprintf_chk", &[2]),
    ("__fprintf_chk", &[2]),
    ("__isoc99_fscanf", &[1]),
    ("__isoc99_scanf", &[0]),
    ("__isoc99_sscanf", &[0, 1]),
    ("__printf_chk", &[1]),
    ("__snprintf_chk", &[4]),
    ("__sprintf_chk", &[3]),
    ("__stpcpy_chk", &[1]),
    ("__strcat_chk", &[1]),
    ("__strcpy_chk", &[1]),
    ("__strdup", &[0]),
    ("__syslog_chk", &[2]),
    ("__vfprintf_chk", &[2]),
    ("__vprintf_chk", &[1]),
    ("__vsnprintf_chk", &[4]),
    ("__vsprintf_chk", &[3]),
    ("access", &[0]),
    ("asprintf", &[1]),
    ("atof", &[0]),
    ("atoi", &[0]),
    ("atol", &[0]),
    ("atoll", &[0]),
    ("chdir", &[0]),
    ("creat", &[0]),
    ("dprintf", &[1]),
    ("err", &[1]),
    ("error", &[2]),
    ("error_at_line", &[2, 4]),
    ("errx", &[1]),
    ("fnmatch", &[0, 1]),
    ("fopen", &[0, 1]),
    ("fopen64", &[0, 1]),
    ("fprintf", &[1]),
    ("fputs", &[0]),
    ("fputs_unlocked", &[0]),
    ("fscanf", &[1]),
    ("getenv", &[0]),
    ("mkdir", &[0]),
    ("nan", &[0]),
    ("nanf", &[0]),
    ("nanl", &[0]),
    ("open", &[0]),
    ("open64", &[0]),
    ("opendir", &[0]),
    ("perror", &[0]),
    ("popen", &[0, 1]),
    ("printf", &[0]),
    ("puts", &[0]),
    ("remove", &[0]),
    ("rename", &[0, 1]),
    ("rmdir", &[0]),
    ("scanf", &[0]),
    ("secure_getenv", &[0]),
    ("setenv", &[0, 1]),
    ("setlocale", &[1]),
    ("snprintf", &[2]),
    ("sprintf", &[1]),
    ("sscanf", &[0, 1]),
    ("stpcpy", &[1]),
    ("strcasecmp", &[0, 1]),
    ("strcat", &[1]),
    ("strcmp", &[0, 1]),
    ("strcoll", &[0, 1]),
    ("strcpy", &[1]),
    ("strcspn", &[0, 1]),
    ("strdup", &[0]),
    ("strftime", &[2]),
    ("strlen", &[0]),
    ("strncasecmp", &[0, 1]),
    ("strncat", &[1]),
    ("strncmp", &[0, 1]),
    ("strncpy", &[1]),
    ("strndup", &[0]),
    ("strnlen", &[0]),
    ("strpbrk", &[1]),
    ("strsep", &[1]),
    ("strspn", &[0, 1]),
    ("strstr", &[1]),
    ("strtok", &[1]),
    ("strtok_r", &[1]),
    ("strverscmp", &[0, 1]),
    ("syslog", &[1]),
    ("system", &[0]),
    ("unlink", &[0]),
    ("unsetenv", &[0]),
    ("vfprintf", &[1]),
    ("vprintf", &[0]),
    ("vsnprintf", &[2]),
    ("vsprintf", &[1]),
    ("warn", &[0]),
    ("warnx", &[0]),
];

/// Does parameter `index` of the library function `name` read its argument
/// only as a string ([`STRING_PARAMS`])?
pub fn reads_only_as_string(name: &str, index: usize) -> bool {
    STRING_PARAMS
        .binary_search_by(|(n, _)| n.cmp(&name))
        .is_ok_and(|i| STRING_PARAMS[i].1.contains(&index))
}

/// Is the constant `vn`, read by `op`, an argument of a direct call to a
/// [`STRING_PARAMS`] parameter? `op` is the call, or an operation (the analysis
/// tier's `PTRSUB` of a planted array) whose output prints inside its one
/// reader, the call.
pub fn string_argument(fd: &Funcdata, op: OpId, vn: VarnodeId) -> bool {
    let Some(o) = fd.obank().get(op) else { return false };
    let (call, arg) = if o.code() == OpCode::CPUI_CALL {
        (op, vn)
    } else {
        let Some(out) = o.get_out() else { return false };
        let Some(v) = fd.vbank().get(out) else { return false };
        let mut readers = v.descend_iter();
        match (readers.next(), readers.next()) {
            (Some(r), None) if v.is_implied() => (r, out),
            _ => return false,
        }
    };
    let Some(c) = fd.obank().get(call) else { return false };
    if c.code() != OpCode::CPUI_CALL {
        return false;
    }
    let slot = c.get_slot(arg);
    if slot < 1 || slot >= c.num_input() {
        return false;
    }
    let Some(idx) = fd.get_call_specs_index(call) else { return false };
    reads_only_as_string(fd.get_call_specs(idx).get_name(), (slot - 1) as usize)
}

/// Does the inclusive byte range `[lo, hi]` meet one of the sorted, disjoint,
/// inclusive `ranges`?
pub fn overlaps(ranges: &[(u64, u64)], lo: u64, hi: u64) -> bool {
    let i = ranges.partition_point(|&(_, stop)| stop < lo);
    i < ranges.len() && ranges[i].0 <= hi
}

/// The C string literal of the NUL-terminated byte string `bytes` (without the
/// NUL), or `None` when it is empty. A valid UTF-8 character is spelled as
/// upstream spells it ([`crate::printc::print_unicode`]) when that spelling is
/// its own bytes: an ASCII character, escaped or not, or a multi-byte one that
/// prints raw and whose encoding is the canonical one (an overlong form or a
/// surrogate decodes to a character of other bytes). Every other byte is
/// `\xNN`, so the literal holds exactly the image's bytes, and the literal is
/// split (`"\xa1" "e"`) where the next character would be read as more hex
/// digits of an escape.
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
        let canonical = cp > 0
            && skip > 0
            && u32::try_from(cp).ok().and_then(char::from_u32).is_some_and(|c| c.len_utf8() == skip as usize);
        let n = if canonical && (skip == 1 || !unicode_needs_escape(cp)) {
            crate::printc::print_unicode(&mut piece, cp);
            skip as usize
        } else {
            let n = if canonical { skip as usize } else { 1 };
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

#[cfg(test)]
#[path = "kuna_conststr/tests.rs"]
mod tests;
