//! `@FILE` values for the selection flags (issue #1978).
//!
//! `--addr`, `--entry` and `--functions`, and `fid query --addr`, read their
//! entries from a file when the value starts with `@`, so a selection of any size
//! fits a command line. The format is the `--define-function @FILE` one: an entry
//! per line, surrounding blanks trimmed, `#` to the end of a line a comment, blank
//! lines skipped. A line is one whole entry, so a `--functions` file can hold a
//! name with spaces or commas that the inline comma list cannot; there a `#` starts
//! a comment only at the start of a line or after a blank, because names kuna
//! prints hold one. `@-` reads standard input, once per command line. A value
//! starting `@@` is that value with one `@` removed, which keeps a name that starts
//! with `@` expressible.

use std::io::{IsTerminal as _, Read as _};

/// A selection flag's value: written inline, or listed in a file.
pub(crate) enum Listed<'a> {
    /// The value as written, a leading `@@` reduced to `@`.
    Inline(&'a str),
    /// The entries of `@FILE`, each with its 1-based line number.
    File { source: String, entries: Vec<(usize, String)> },
}

impl Listed<'_> {
    /// Parse each entry with `inline` or `listed`; a file entry's error names the
    /// flag, the file and the line.
    pub(crate) fn parse_each<T>(
        self,
        flag: &str,
        inline: impl Fn(&str) -> Result<T, String>,
        listed: impl Fn(&str) -> Result<T, String>,
    ) -> Result<Vec<T>, String> {
        match self {
            Listed::Inline(value) => Ok(vec![inline(value)?]),
            Listed::File { source, entries } => entries
                .iter()
                .map(|(line, entry)| listed(entry).map_err(|e| format!("{flag} @{source}:{line}: {e}")))
                .collect(),
        }
    }
}

/// Reads the `@` values of one command line, of which one may be `@-`.
#[derive(Default)]
pub(crate) struct ListReader {
    stdin_taken: bool,
}

impl ListReader {
    /// An address list, where any `#` starts a comment.
    pub(crate) fn read<'a>(&mut self, flag: &str, value: &'a str) -> Result<Listed<'a>, String> {
        self.read_with(flag, value, |line| line.find('#'))
    }

    /// A name list, where a `#` starts a comment only at the start of a line or
    /// after a blank: a Swift or Rust v0 name such as `M.T.vtable#2.method` or
    /// `f::{closure#0}` holds one.
    pub(crate) fn read_names<'a>(&mut self, flag: &str, value: &'a str) -> Result<Listed<'a>, String> {
        self.read_with(flag, value, |line| {
            line.match_indices('#')
                .map(|(at, _)| at)
                .find(|&at| line[..at].chars().next_back().is_none_or(char::is_whitespace))
        })
    }

    fn read_with<'a>(
        &mut self,
        flag: &str,
        value: &'a str,
        comment: impl Fn(&str) -> Option<usize>,
    ) -> Result<Listed<'a>, String> {
        let Some(source) = value.strip_prefix('@') else {
            return Ok(Listed::Inline(value));
        };
        if source.starts_with('@') {
            return Ok(Listed::Inline(source));
        }
        let text = if source == "-" {
            self.stdin(flag)?
        } else {
            std::fs::read_to_string(source).map_err(|e| match e.kind() {
                std::io::ErrorKind::NotFound => {
                    format!("{flag} @{source}: {e}; a value that starts with @ is written @@{source}")
                }
                _ => format!("{flag} @{source}: {e}"),
            })?
        };
        let entries: Vec<(usize, String)> = text
            .strip_prefix('\u{feff}')
            .unwrap_or(&text)
            .lines()
            .enumerate()
            .filter_map(|(n, line)| {
                let entry = line[..comment(line).unwrap_or(line.len())].trim();
                (!entry.is_empty()).then(|| (n + 1, entry.to_string()))
            })
            .collect();
        if entries.is_empty() {
            return Err(format!("{flag} @{source}: lists no entries"));
        }
        Ok(Listed::File { source: source.to_string(), entries })
    }

    fn stdin(&mut self, flag: &str) -> Result<String, String> {
        if std::mem::replace(&mut self.stdin_taken, true) {
            return Err(format!("{flag} @-: standard input already feeds an earlier @-"));
        }
        let mut stdin = std::io::stdin().lock();
        if stdin.is_terminal() {
            return Err(format!("{flag} @-: standard input is a terminal; pipe the list in or name a file"));
        }
        let mut text = String::new();
        stdin.read_to_string(&mut text).map_err(|e| format!("{flag} @-: {e}"))?;
        Ok(text)
    }
}

/// The entry a `--define-function` line declares, so an address list can be the
/// same file `--define-function @FILE` reads.
pub(crate) fn declared_start(entry: &str) -> Option<u64> {
    kuna_console::assertsyntax::parse_function_spec(entry).ok().map(|spec| spec.start)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn file(text: &str) -> (tempfile::TempPath, String) {
        let file = tempfile::NamedTempFile::new().unwrap();
        std::fs::write(file.path(), text).unwrap();
        let value = format!("@{}", file.path().display());
        (file.into_temp_path(), value)
    }

    fn entries(listed: Listed) -> Vec<(usize, String)> {
        match listed {
            Listed::File { entries, .. } => entries,
            Listed::Inline(v) => panic!("inline {v:?}"),
        }
    }

    #[test]
    fn a_file_lists_one_trimmed_entry_per_line_without_comments() {
        let (_keep, value) = file("# picked\n  main  \n\nparse # the parser\r\noperator new\n");
        let got = entries(ListReader::default().read("--functions", &value).unwrap());
        assert_eq!(
            got,
            vec![(2, "main".into()), (4, "parse".into()), (5, "operator new".into())]
        );
    }

    #[test]
    fn a_name_keeps_a_hash_that_follows_no_blank() {
        let (_keep, value) =
            file("\u{feff}M.T.vtable#2.method\n#x\nf::{closure#0}  # a closure\nmain\t#tab\n");
        let got = entries(ListReader::default().read_names("--functions", &value).unwrap());
        assert_eq!(
            got,
            vec![(1, "M.T.vtable#2.method".into()), (3, "f::{closure#0}".into()), (4, "main".into())]
        );
        let got = entries(ListReader::default().read("--addr", &value).unwrap());
        assert_eq!(got, vec![(1, "M.T.vtable".into()), (3, "f::{closure".into()), (4, "main".into())]);
    }

    #[test]
    fn a_double_at_is_a_literal_value() {
        assert!(matches!(ListReader::default().read("--functions", "@@f@8").unwrap(), Listed::Inline("@f@8")));
        assert!(matches!(ListReader::default().read("--functions", "main,@f@8").unwrap(), Listed::Inline("main,@f@8")));
    }

    #[test]
    fn a_missing_or_empty_file_is_an_error() {
        let err = ListReader::default().read("--functions", "@/nonexistent/kuna-list.txt").err().unwrap();
        assert!(err.starts_with("--functions @/nonexistent/kuna-list.txt: "), "{err}");
        assert!(err.contains("written @@/nonexistent/kuna-list.txt"), "{err}");
        let (_keep, value) = file("# nothing\n\n");
        let err = ListReader::default().read("--addr", &value).err().unwrap();
        assert!(err.ends_with("lists no entries"), "{err}");
    }

    #[test]
    fn a_bad_file_entry_names_its_line() {
        let (path, value) = file("0x10\nzz\n");
        let err = ListReader::default()
            .read("--addr", &value)
            .unwrap()
            .parse_each("--addr", |_| Ok(0), |e| u64::from_str_radix(e.trim_start_matches("0x"), 16).map_err(|_| format!("invalid address {e:?}")))
            .err()
            .unwrap();
        assert_eq!(err, format!("--addr @{}:2: invalid address \"zz\"", path.display()));
    }

    #[test]
    fn a_declaration_line_selects_its_start() {
        assert_eq!(declared_start("0x4013c9-0x401420 = stage1"), Some(0x4013c9));
        assert_eq!(declared_start("401420=stage2"), Some(0x401420));
        assert_eq!(declared_start("main"), None);
    }
}
