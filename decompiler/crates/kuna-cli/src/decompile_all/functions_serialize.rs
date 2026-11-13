//! Serialize borrowed inventory records without building a second function list.
//! JSON layout belongs to serde_json; the formatter adds ASCII-safe strings.

use std::io::{self, Write};

use kuna_console::engine::FunctionEntry;
use serde::ser::Error as _;
use serde::{Serialize, Serializer};
use serde_json::ser::{Formatter, PrettyFormatter};

use crate::funcdecl::Declared;
use crate::jsonfmt::{Json, UnicodeEscape};

#[allow(clippy::too_many_arguments)]
pub(super) fn write(
    out: &mut dyn Write,
    binary: &str,
    entries: &[FunctionEntry],
    total: usize,
    error: Option<&str>,
    limits: Option<&Json>,
    display_address: &dyn Fn(u64) -> u64,
    declared: &Declared,
) -> io::Result<()> {
    let document = Document {
        binary,
        count: entries.len(),
        total,
        error,
        notes: declared.notes(),
        functions: Entries {
            entries,
            display_address,
            declared,
        },
        limits: limits.map(JsonView),
    };
    let mut out = io::BufWriter::with_capacity(1 << 16, out);
    let mut serializer =
        serde_json::Serializer::with_formatter(&mut out, AsciiPretty(PrettyFormatter::new()));
    document
        .serialize(&mut serializer)
        .map_err(io::Error::from)?;
    out.write_all(b"\n")?;
    out.flush()
}

#[derive(Serialize)]
struct Document<'a> {
    binary: &'a str,
    count: usize,
    total: usize,
    error: Option<&'a str>,
    #[serde(skip_serializing_if = "<[String]>::is_empty")]
    notes: &'a [String],
    functions: Entries<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limits: Option<JsonView<'a>>,
}

struct Entries<'a> {
    entries: &'a [FunctionEntry],
    display_address: &'a dyn Fn(u64) -> u64,
    declared: &'a Declared,
}

impl Serialize for Entries<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.entries.iter().map(|entry| {
            let address = (self.display_address)(entry.addr.get_offset());
            Entry {
                name: &entry.name,
                address,
                address_hex: format!("0x{address:x}"),
                aliases: &entry.aliases,
                object_location: entry.object_location.as_ref().map(|location| Location {
                    section_index: location.section_index,
                    section: &location.section,
                    offset: location.offset,
                    offset_hex: format!("0x{:x}", location.offset),
                }),
                size: entry.size,
                declared: self
                    .declared
                    .bytes_label(entry.addr.get_offset())
                    .map(|bytes| Marker { bytes }),
            }
        }))
    }
}

#[derive(Serialize)]
struct Entry<'a> {
    name: &'a str,
    address: u64,
    address_hex: String,
    aliases: &'a [String],
    object_location: Option<Location<'a>>,
    size: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    declared: Option<Marker<'a>>,
}

#[derive(Serialize)]
struct Location<'a> {
    section_index: usize,
    section: &'a str,
    offset: u64,
    offset_hex: String,
}

#[derive(Serialize)]
struct Marker<'a> {
    bytes: &'a str,
}

struct JsonView<'a>(&'a Json);

impl Serialize for JsonView<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self.0 {
            Json::Null => serializer.serialize_none(),
            Json::Bool(value) => serializer.serialize_bool(*value),
            Json::Str(value) => serializer.serialize_str(value),
            Json::Number(value) => {
                let raw = serde_json::value::RawValue::from_string(value.clone())
                    .map_err(S::Error::custom)?;
                raw.serialize(serializer)
            }
            Json::Array(values) => serializer.collect_seq(values.iter().map(JsonView)),
            Json::Object(fields) => {
                serializer.collect_map(fields.iter().map(|(key, value)| (key, JsonView(value))))
            }
        }
    }
}

struct AsciiPretty(PrettyFormatter<'static>);

impl Formatter for AsciiPretty {
    fn write_string_fragment<W: ?Sized + Write>(
        &mut self,
        writer: &mut W,
        fragment: &str,
    ) -> io::Result<()> {
        let mut start = 0;
        for (offset, ch) in fragment.char_indices() {
            if ch < '\u{7f}' {
                continue;
            }
            writer.write_all(&fragment.as_bytes()[start..offset])?;
            write!(writer, "{}", UnicodeEscape(ch))?;
            start = offset + ch.len_utf8();
        }
        writer.write_all(&fragment.as_bytes()[start..])
    }

    fn begin_array<W: ?Sized + Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.0.begin_array(writer)
    }

    fn end_array<W: ?Sized + Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.0.end_array(writer)
    }

    fn begin_array_value<W: ?Sized + Write>(
        &mut self,
        writer: &mut W,
        first: bool,
    ) -> io::Result<()> {
        self.0.begin_array_value(writer, first)
    }

    fn end_array_value<W: ?Sized + Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.0.end_array_value(writer)
    }

    fn begin_object<W: ?Sized + Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.0.begin_object(writer)
    }

    fn end_object<W: ?Sized + Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.0.end_object(writer)
    }

    fn begin_object_key<W: ?Sized + Write>(
        &mut self,
        writer: &mut W,
        first: bool,
    ) -> io::Result<()> {
        self.0.begin_object_key(writer, first)
    }

    fn begin_object_value<W: ?Sized + Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.0.begin_object_value(writer)
    }

    fn end_object_value<W: ?Sized + Write>(&mut self, writer: &mut W) -> io::Result<()> {
        self.0.end_object_value(writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::address::{AddrSpacePtr, Address};
    use kuna_console::engine::{EntryProvenance, ObjectLocation};

    #[test]
    fn records_and_optional_limits_match_legacy_bytes() {
        let entries = [
            FunctionEntry {
                name: "函数🚀\x7f\n\"\\".into(),
                addr: Address::from_space_ptr(AddrSpacePtr::Null, 0),
                aliases: vec!["é\t".into(), "别名".into()],
                size: u64::MAX,
                object_location: Some(ObjectLocation {
                    section_index: 7,
                    section: "__text\u{2028}".into(),
                    offset: u64::MAX,
                }),
                provenance: EntryProvenance::Mapped,
                binding: None,
            },
            FunctionEntry {
                name: String::new(),
                addr: Address::from_space_ptr(AddrSpacePtr::Null, u64::MAX),
                aliases: vec![],
                size: 0,
                object_location: None,
                provenance: EntryProvenance::Mapped,
                binding: None,
            },
        ];
        let limits = Json::Object(vec![
            (
                "maxinstruction".into(),
                Json::Number("18446744073709551615".into()),
            ),
            ("jumptablemax".into(), Json::Number("0".into())),
            (
                "over".into(),
                Json::Array(vec![Json::Object(vec![
                    ("name".into(), Json::Str("é".into())),
                    ("over_maxinstruction".into(), Json::Bool(true)),
                    ("switches_over_jumptablemax".into(), Json::Array(vec![])),
                ])]),
            ),
        ]);
        for entries in [&entries[..0], &entries[..1], &entries[..]] {
            for error in [None, Some("失败\0\x1f")] {
                for limits in [None, Some(&limits)] {
                    let display_address = |address| address ^ 0x1234;
                    let declared = Declared::default();
                    let mut bytes = Vec::new();
                    write(
                        &mut bytes,
                        "输入🚀",
                        entries,
                        123,
                        error,
                        limits,
                        &display_address,
                        &declared,
                    )
                    .unwrap();
                    let expected = super::super::functions_json(
                        "输入🚀",
                        entries,
                        123,
                        error,
                        limits,
                        &display_address,
                        &declared,
                    );
                    assert_eq!(bytes, expected.as_bytes());
                }
            }
        }
    }

    #[test]
    fn every_unicode_scalar_has_the_legacy_escape_spelling() {
        let text: String = (0..=0x10ffff).filter_map(char::from_u32).collect();
        let mut bytes = Vec::new();
        let mut out = io::BufWriter::with_capacity(1 << 16, &mut bytes);
        let mut serializer =
            serde_json::Serializer::with_formatter(&mut out, AsciiPretty(PrettyFormatter::new()));
        text.serialize(&mut serializer).unwrap();
        out.flush().unwrap();
        drop(out);
        assert_eq!(
            bytes,
            crate::jsonfmt::dumps_indent2(&Json::Str(text)).as_bytes()
        );
    }

    #[test]
    fn write_errors_preserve_their_kind_and_run_verdict() {
        struct FailingWriter(io::ErrorKind);

        impl Write for FailingWriter {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::new(self.0, "output failure"))
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        for kind in [io::ErrorKind::BrokenPipe, io::ErrorKind::PermissionDenied] {
            for run_error in [None, Some("no functions discovered")] {
                let result = write(
                    &mut FailingWriter(kind),
                    "fixture",
                    &[],
                    0,
                    run_error,
                    None,
                    &|address| address,
                    &Declared::default(),
                );
                assert_eq!(result.as_ref().unwrap_err().kind(), kind);
                let status = super::super::finish_with_run_error(result, run_error);
                let expected = if kind == io::ErrorKind::BrokenPipe {
                    i32::from(run_error.is_some())
                } else {
                    1
                };
                assert_eq!(status, expected);
            }
        }
    }
}
