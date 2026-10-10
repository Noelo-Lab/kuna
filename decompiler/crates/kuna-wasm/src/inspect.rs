//! The study view's documents: `inspect` (one function with its token source
//! map and instruction listing), `read` (raw bytes), `xrefs` (one function's
//! references), `strings` (the string literals and who uses them), and the
//! target, section and type facts `list` adds for it.

use std::collections::BTreeMap;

use kuna_console::assertions::Outcome;
use kuna_console::disasm::{hex, mapped_run_end, read_upto};
use kuna_console::engine::{ConsoleProgram, FunctionEntry};
use kuna_analysis::loader::kuna_relocrebase::RebasedView;
use kuna_analysis::strings::kuna_stringinv::{self, Termination};
use kuna_console::inspect::{
    function_rows, function_xrefs, section_rows, string_uses, SectionRow, XrefRow, FUNCTION_ROW_CAP,
};
use kuna_console::project::FuncResult;

use crate::json::{arr, assertions_json, hex_str, json_object_location, json_str, json_str_array, Obj};

/// The largest `read` a caller may ask for.
pub const READ_MAX: u64 = 64 * 1024;

/// `{archid, processor, endian, bits}` from the SLEIGH language id
/// (`x86:LE:64:default` → `x86`, `little`, 64).
pub fn target_json(prog: &ConsoleProgram) -> String {
    let archid = prog.arch().archid.clone();
    let mut fields = archid.split(':');
    let processor = fields.next().unwrap_or("").to_string();
    let endian = match fields.next() {
        Some("BE") => "big",
        Some("LE") => "little",
        _ => "unknown",
    };
    let bits = fields.next().and_then(|b| b.parse::<u32>().ok());
    Obj::new()
        .str("archid", &archid)
        .str("processor", &processor)
        .str("endian", endian)
        .opt_num("bits", bits)
        .end()
}

pub fn sections_json(rows: &[SectionRow]) -> String {
    arr(rows.iter().map(|s| {
        Obj::new()
            .str("name", &s.name)
            .addr("address", s.address)
            .num("size", s.size)
            .opt_num("file_offset", s.file_offset)
            .opt_num("file_size", s.file_size)
            .bool("executable", s.executable)
            .bool("writable", s.writable)
            .end()
    }))
}

/// Where `addr` lies in the input file, when the file holds the section bytes
/// there (a section's tail past its raw data, like `.bss`, has no file offset).
pub fn file_offset(sections: &[SectionRow], addr: u64) -> Option<u64> {
    sections.iter().find_map(|s| {
        let delta = addr.checked_sub(s.address)?;
        let held = s.file_size.unwrap_or(0).min(s.size);
        (delta < held).then_some(s.file_offset?.checked_add(delta)?)
    })
}

/// The named, non-core types the program's type factory holds — what a retype
/// can name besides the C scalars: `[{name, size, kind}]`.
pub fn known_types_json(prog: &ConsoleProgram) -> String {
    use kuna_decomp::dtype::type_metatype::*;
    let mut seen: BTreeMap<String, String> = BTreeMap::new();
    for ct in prog.arch().types_impl().dependent_order() {
        if ct.is_core_type() || ct.get_name().is_empty() {
            continue;
        }
        let kind = if ct.get_typedef().is_some() {
            "typedef"
        } else {
            match ct.get_metatype() {
                TYPE_STRUCT => "struct",
                TYPE_UNION => "union",
                TYPE_ENUM_INT | TYPE_ENUM_UINT => "enum",
                TYPE_INT | TYPE_UINT | TYPE_BOOL | TYPE_FLOAT | TYPE_UNKNOWN | TYPE_VOID => "scalar",
                _ => continue,
            }
        };
        let row = Obj::new()
            .str("name", ct.get_name())
            .num("size", ct.get_size())
            .str("kind", kind)
            .end();
        seen.entry(ct.get_name().to_string()).or_insert(row);
    }
    arr(seen.into_values())
}

fn addresses_json(addresses: &[u64]) -> (String, String) {
    (
        arr(addresses.iter().map(u64::to_string)),
        arr(addresses.iter().map(|a| json_str(&format!("0x{a:x}")))),
    )
}

fn function_json(prog: &ConsoleProgram, f: &FuncResult, kind: &str, sections: &[SectionRow]) -> String {
    let line_mappings = arr(f.line_mappings.iter().map(|m| {
        let (numbers, hex) = addresses_json(&m.addresses);
        Obj::new()
            .num("line_number", m.line_number)
            .raw("addresses", &numbers)
            .raw("addresses_hex", &hex)
            .end()
    }));
    let variables = arr(f.variables.iter().map(|v| {
        let (numbers, hex) = addresses_json(&v.addresses);
        Obj::new()
            .str("name", &v.name)
            .str("type", &v.type_name)
            .str("kind", if v.is_param { "arg" } else { "stack" })
            .opt_num("arg_index", v.arg_index)
            .opt_num("stack_offset", v.stack_offset)
            .num("size", v.size)
            .raw("line_numbers", &arr(v.line_numbers.iter().map(usize::to_string)))
            .raw("addresses", &numbers)
            .raw("addresses_hex", &hex)
            .end()
    }));
    let types = arr(f.types.iter().map(|t| {
        Obj::new().str("name", &t.name).str("definition", &t.definition).num("size", t.size).end()
    }));
    let globals = arr(f.globals.iter().map(|g| {
        Obj::new()
            .str("name", &g.name)
            .addr("address", g.address)
            .str("declaration", &g.declaration)
            .num("size", g.size)
            .end()
    }));
    let (tokens, tokens_error) = match (&f.detail, &f.code) {
        (Some(detail), _) => (
            arr(detail.tokens.iter().map(|t| {
                let mut o = Obj::new()
                    .num("line", t.line)
                    .num("col", t.col)
                    .num("len", t.len)
                    .str("kind", t.kind.name())
                    .str("color", t.color.name())
                    .str("text", &t.text);
                if let Some(a) = t.address {
                    o = o.addr("address", a);
                }
                if let Some(c) = t.callee {
                    o = o.addr("callee", c);
                }
                if let Some(v) = t.var {
                    o = o.num("var", v);
                }
                if let Some(d) = t.decl {
                    o = o.str("decl", d);
                }
                if let Some(ty) = &t.type_text {
                    o = o.str("type", ty);
                }
                o.end()
            })),
            detail.tokens_error.clone(),
        ),
        (None, Some(_)) => ("[]".to_string(), Some("no token map for this record".to_string())),
        (None, None) => ("[]".to_string(), None),
    };
    let entry = f.byte_address;
    let size = if f.size > 0 { f.size as u64 } else { prog.function_extent_at(entry) };
    let (rows, truncated) = function_rows(prog, entry, size, FUNCTION_ROW_CAP);
    let mut lines_at: BTreeMap<u64, Vec<usize>> = BTreeMap::new();
    for m in &f.line_mappings {
        for &a in &m.addresses {
            lines_at.entry(a).or_default().push(m.line_number);
        }
    }
    let entry_out = prog.output_code_offset(entry) as i128;
    let instructions = arr(rows.iter().map(|r| {
        let address = prog.output_code_offset(r.addr);
        let lines = lines_at.get(&address).map(Vec::as_slice).unwrap_or(&[]);
        let flow = prog.insn_flow(r.addr);
        Obj::new()
            .addr("address", address)
            .num("offset", address as i128 - entry_out)
            .num("size", r.size)
            .str("bytes", &r.hex())
            .str("mnemonic", &r.mnemonic)
            .str("operands", &r.operands)
            .str("text", &r.text())
            .opt_num("file_offset", file_offset(sections, r.addr))
            .raw("lines", &arr(lines.iter().map(usize::to_string)))
            .opt_str("flow", flow.as_ref().map(|f| f.kind.as_str()))
            .raw(
                "targets_hex",
                &arr(flow.iter().flat_map(|f| &f.targets).map(|&t| hex_str(prog.output_code_offset(t)))),
            )
            .end()
    }));
    Obj::new()
        .str("name", &f.name)
        .addr("address", f.address)
        .raw("aliases", &json_str_array(&f.aliases))
        .raw("object_location", &json_object_location(f.object_location.as_ref()))
        .str("kind", kind)
        .num("size", f.size)
        .opt_str("code", f.code.as_deref())
        .opt_str("error", f.error.as_deref())
        .opt_str("proto", f.proto.as_deref())
        .num("unstructured_gotos", f.unstructured_gotos())
        .raw("line_mappings", &line_mappings)
        .raw("variables", &variables)
        .raw("types", &types)
        .raw("globals", &globals)
        .raw("tokens", &tokens)
        .opt_str("tokens_error", tokens_error.as_deref())
        .raw("instructions", &instructions)
        .bool("instructions_truncated", truncated)
        .end()
}

/// The `inspect` document: one function, one load.
pub fn inspect_json(
    binary: &str,
    language: &str,
    prog: &ConsoleProgram,
    f: &FuncResult,
    kind: &str,
    assertions: &[Outcome],
) -> String {
    let sections = section_rows(prog);
    Obj::new()
        .str("binary", binary)
        .str("language", language)
        .raw("target", &target_json(prog))
        .raw("function", &function_json(prog, f, kind, &sections))
        .raw("assertions", &assertions_json(assertions))
        .end()
}

/// The `read` document: up to `len` bytes at `addr`, stopping at the end of
/// the mapped run that holds it.
pub fn read_json(binary: &str, prog: &ConsoleProgram, addr: u64, len: u64, assertions: &[Outcome]) -> String {
    let mut bytes = Vec::new();
    let start = prog.input_address_offset(addr).ok();
    let got = start.map_or(0, |start| {
        // The loader zero-fills a read that runs off the end of a mapped run;
        // those bytes are not in the image, so the read stops where the run does.
        let len = match mapped_run_end(prog, start) {
            Some((end, _)) => len.min(end.saturating_sub(start)),
            None => len,
        };
        read_upto(prog, start, len as usize, &mut bytes)
    });
    bytes.truncate(got);
    let text = hex(&bytes);
    let sections = section_rows(prog);
    let offset = start.filter(|_| got > 0).and_then(|start| file_offset(&sections, start));
    Obj::new()
        .str("binary", binary)
        .addr("address", addr)
        .num("size", got)
        .str("bytes", &text)
        .opt_num("file_offset", offset)
        .raw("assertions", &assertions_json(assertions))
        .end()
}

/// The `xrefs` document: `{binary, function:{name, address, address_hex},
/// callers:[{name, address, address_hex, from, from_hex, kind, instruction}],
/// callees:[{name, address, address_hex, at, at_hex, kind, instruction}],
/// data_refs:[…same as callees…], assertions}`. A caller's `address` is the
/// calling function's entry and `from` the calling instruction.
pub fn xrefs_json(
    binary: &str,
    prog: &ConsoleProgram,
    file: &object::File,
    target: &FunctionEntry,
    assertions: &[Outcome],
) -> String {
    let entry = target.addr.get_offset();
    let refs = function_xrefs(prog, file, entry);
    let rows = |rows: &[XrefRow], site: &str| {
        arr(rows.iter().map(|r| {
            Obj::new()
                .opt_str("name", r.name.as_deref())
                .addr("address", prog.output_code_offset(r.address))
                .addr(site, prog.output_code_offset(r.site))
                .str("kind", r.kind)
                .str("instruction", &r.instruction)
                .end()
        }))
    };
    Obj::new()
        .str("binary", binary)
        .raw(
            "function",
            &Obj::new()
                .str("name", &target.name)
                .addr("address", prog.output_code_offset(entry))
                .end(),
        )
        .raw("callers", &rows(&refs.callers, "from"))
        .raw("callees", &rows(&refs.callees, "at"))
        .raw("data_refs", &rows(&refs.data_refs, "at"))
        .raw("assertions", &assertions_json(assertions))
        .end()
}

/// The `strings` document: `{binary, scanned, count, strings:[{address,
/// address_hex, text, length, encoding, section, in_code, uses:[{name, address,
/// address_hex, at, at_hex, kind, instruction, via}]}], assertions}`. The rows
/// are `kuna strings --encoding all`'s (minimum length 5, any ending); `in_code`
/// marks a row inside an executable section, and a use's `via` is
/// `{name, address, address_hex}` of the data word it reads the string's address
/// from, else null.
pub fn strings_json(
    binary: &str,
    prog: &ConsoleProgram,
    file: &object::File,
    rebased: Option<&RebasedView>,
    assertions: &[Outcome],
) -> String {
    use object::{Object, ObjectSection, SectionKind};
    let query = kuna_stringinv::Query {
        min_len: 5,
        ascii: true,
        utf8: true,
        utf16: true,
        section: None,
        termination: Termination::Any,
    };
    let inv = match rebased {
        Some(view) => kuna_stringinv::inventory_in_image(file, &query, &view.ranges),
        None => kuna_stringinv::inventory(file, &query),
    };
    let code: Vec<(u64, u64)> = file
        .sections()
        .filter(|s| s.kind() == SectionKind::Text)
        .map(|s| (s.address(), s.address().saturating_add(s.size())))
        .collect();
    let extents: Vec<(u64, u64)> =
        inv.strings.iter().map(|s| (s.addr, u64::from(s.byte_len))).collect();
    let uses = string_uses(prog, file, &extents);
    let rows = arr(inv.strings.iter().zip(&uses).map(|(s, uses)| {
        let uses = arr(uses.iter().map(|u| {
            let via = u.via.as_ref().map_or_else(
                || "null".to_string(),
                |(slot, name)| Obj::new().opt_str("name", name.as_deref()).addr("address", *slot).end(),
            );
            Obj::new()
                .opt_str("name", u.row.name.as_deref())
                .addr("address", prog.output_code_offset(u.row.address))
                .addr("at", prog.output_code_offset(u.row.site))
                .str("kind", u.row.kind)
                .str("instruction", &u.row.instruction)
                .raw("via", &via)
                .end()
        }));
        Obj::new()
            .addr("address", s.addr)
            .str("text", &s.text)
            .num("length", s.char_len)
            .str("encoding", s.encoding.as_str())
            .opt_str("section", s.section.as_deref())
            .bool("in_code", code.iter().any(|&(lo, hi)| s.addr >= lo && s.addr < hi))
            .raw("uses", &uses)
            .end()
    }));
    Obj::new()
        .str("binary", binary)
        .str("scanned", if inv.from_segments { "segments" } else { "sections" })
        .num("count", inv.strings.len())
        .raw("strings", &rows)
        .raw("assertions", &assertions_json(assertions))
        .end()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn section(name: &str, address: u64, size: u64, file_offset: Option<u64>) -> SectionRow {
        SectionRow {
            name: name.into(),
            address,
            size,
            file_offset,
            file_size: file_offset.map(|_| size),
            executable: false,
            writable: false,
        }
    }

    #[test]
    fn file_offsets_come_from_the_file_backed_section_holding_the_address() {
        let sections = [
            section(".text", 0x1000, 0x100, Some(0x1000)),
            section(".bss", 0x4000, 0x20, None),
            section(".data", 0x3000, 0x10, Some(0x2000)),
        ];
        assert_eq!(file_offset(&sections, 0x1010), Some(0x1010));
        assert_eq!(file_offset(&sections, 0x300f), Some(0x200f));
        assert_eq!(file_offset(&sections, 0x3010), None);
        assert_eq!(file_offset(&sections, 0x4000), None);
        assert_eq!(file_offset(&sections, 0x10), None);
        let short = [SectionRow { file_size: Some(0x10), ..section(".data", 0x5000, 0x40, Some(0x3000)) }];
        assert_eq!(file_offset(&short, 0x500f), Some(0x300f));
        assert_eq!(file_offset(&short, 0x5010), None);
    }
}
