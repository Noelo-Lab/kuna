//! Whether the bytes a caller-declared function names are in the input.
//!
//! A declaration (`--define-function`, `--assert function`, the console's
//! `function bounds`) is an assertion and is honoured as one wherever it lands:
//! the entry enumerates, resolves by name and names its call sites even where
//! the image maps nothing, which is what a routine in a boot ROM or another
//! module, called at a fixed address, needs. What an assertion cannot supply is
//! bytes. So each declaration is measured once, when it is made, against the
//! image's live byte map (the one the lifter refuses unmapped instructions by)
//! less the segments the image declares inaccessible, and one that is not
//! wholly mapped is said to be: a warning on stderr, a
//! `notes` entry in the JSON documents, and a `declared` marker on its
//! inventory row. Before, it was listed exactly like a discovered entry and
//! then refused by the first command that needed its bytes (issue #1438).

use crate::engine::ConsoleProgram;

/// How much of a declared extent the input maps. The extent is `[start, end)`,
/// or the entry byte alone for a declaration with no end.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeclaredBytes {
    /// Every byte of the extent is mapped.
    Mapped,
    /// The entry is mapped; `first_unmapped` is the first byte of the extent
    /// that is not.
    Partial { first_unmapped: u64 },
    /// The entry itself is not mapped.
    Unmapped,
}

impl DeclaredBytes {
    /// The JSON spelling.
    pub fn label(self) -> &'static str {
        match self {
            DeclaredBytes::Mapped => "mapped",
            DeclaredBytes::Partial { .. } => "partial",
            DeclaredBytes::Unmapped => "unmapped",
        }
    }
}

/// One declaration as measured: what the warning names.
#[derive(Clone, Debug)]
pub(crate) struct Declaration {
    pub(crate) name: String,
    pub(crate) size: u64,
    pub(crate) bytes: DeclaredBytes,
}

/// The console line prefix that carries a warning through a `decomp_dbg`
/// transcript, read back by [`from_transcript`].
const TRANSCRIPT_PREFIX: &str = "warning: declared function ";

/// Measure the declaration at byte address `start` spanning `size` bytes
/// (`0` = no declared end). The last byte of the address space ends no
/// half-open extent, so the lifter never counts it mapped and neither does this.
pub(crate) fn measure(prog: &ConsoleProgram, start: u64, size: u64) -> DeclaredBytes {
    let Some(end) = start.checked_add(size.max(1)) else {
        return DeclaredBytes::Unmapped;
    };
    match first_unmapped(prog, start, end) {
        None => DeclaredBytes::Mapped,
        Some(at) if at == start => DeclaredBytes::Unmapped,
        Some(at) => DeclaredBytes::Partial { first_unmapped: at },
    }
}

/// The first byte of `[lo, hi)` the input does not map, or `None`: the first
/// the loader does not serve, or the first in a segment the image declares
/// inaccessible (a Mach-O `__PAGEZERO`, which the byte map serves as zero fill),
/// whichever comes first.
fn first_unmapped(prog: &ConsoleProgram, lo: u64, hi: u64) -> Option<u64> {
    let loader = prog.arch().translate().loader_rc();
    let blocked = loader
        .borrow()
        .inaccessible_segments()
        .into_iter()
        .filter(|&(vma, size)| vma < hi && vma.saturating_add(size) > lo)
        .map(|(vma, _)| vma.max(lo))
        .min();
    match (first_unserved(prog, lo, hi), blocked) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (a, b) => a.or(b),
    }
}

/// The first byte of `[lo, hi)` the loader does not serve, or `None`.
///
/// The loader's shared byte map is the evidence when it has one: a staged
/// `load_fill` can answer a read out of a padded window. A loader without one
/// is asked through its section table, and one with neither through a probe
/// of the entry byte.
fn first_unserved(prog: &ConsoleProgram, lo: u64, hi: u64) -> Option<u64> {
    let shared = prog.arch().translate().loader_rc().borrow().shared_bytes();
    if let Some(bytes) = shared {
        if bytes.mapped_covers(lo, hi) {
            return None;
        }
        let (mut covered, mut short) = (lo, hi);
        while short - covered > 1 {
            let mid = covered + (short - covered) / 2;
            if bytes.mapped_covers(lo, mid) {
                covered = mid;
            } else {
                short = mid;
            }
        }
        return Some(covered);
    }
    let sections = prog.sections();
    if sections.is_empty() {
        return (!prog.vma_bytes_mapped(lo)).then_some(lo);
    }
    let mut cur = lo;
    while cur < hi {
        match sections
            .iter()
            .filter(|&&(start, size, _)| cur >= start && cur - start < size)
            .map(|&(start, size, _)| start.saturating_add(size))
            .max()
        {
            Some(reach) => cur = reach,
            None => return Some(cur),
        }
    }
    None
}

/// The sentence for a declaration that is not wholly mapped, in the caller's
/// address units, or `None` for one that is.
pub(crate) fn note(prog: &ConsoleProgram, start: u64, decl: &Declaration) -> Option<String> {
    let at = prog.output_code_offset(start);
    let extent = match decl.size {
        0 => format!("{at:#x}"),
        size => format!("{at:#x}-{:#x}", prog.output_code_offset(start.saturating_add(size))),
    };
    match decl.bytes {
        DeclaredBytes::Mapped => None,
        DeclaredBytes::Unmapped => Some(format!(
            "declared function {} at {extent}: its entry {at:#x} is not mapped in this input; \
             the function is kept as asserted (calls to it use its name), but there are no \
             bytes there to decompile or disassemble",
            decl.name
        )),
        DeclaredBytes::Partial { first_unmapped } => Some(format!(
            "declared function {} at {extent}: its extent runs past the bytes this input maps \
             (the first unmapped byte is {:#x}); the function is kept as declared",
            decl.name,
            prog.output_code_offset(first_unmapped)
        )),
    }
}

/// The console line for `sentence`.
pub(crate) fn transcript_line(sentence: &str) -> String {
    format!("warning: {sentence}\n")
}

/// The declaration warnings a `decomp_dbg` transcript carries, as sentences.
pub fn from_transcript(transcript: &str) -> Vec<String> {
    transcript
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            let line = line.strip_prefix("[decomp]>").unwrap_or(line).trim();
            line.starts_with(TRANSCRIPT_PREFIX)
                .then(|| line["warning: ".len()..].to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_transcript_warning_round_trips() {
        let sentence = "declared function rom at 0x50000 is not mapped in this input; it is kept";
        let transcript = format!(
            "[decomp]> function bounds 0x50000 as rom\nDeclared rom at 0x50000 spanning 0 bytes\n{}",
            transcript_line(sentence)
        );
        assert_eq!(from_transcript(&transcript), vec![sentence.to_string()]);
        assert!(from_transcript("warning: something else entirely").is_empty());
    }

    #[test]
    fn labels_are_the_documented_spellings() {
        assert_eq!(DeclaredBytes::Mapped.label(), "mapped");
        assert_eq!(DeclaredBytes::Partial { first_unmapped: 4 }.label(), "partial");
        assert_eq!(DeclaredBytes::Unmapped.label(), "unmapped");
    }
}
