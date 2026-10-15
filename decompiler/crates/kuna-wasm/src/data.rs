//! The Data tab's documents: `data` (the labelled, typed items in a window of
//! the image around an address, with its bytes and what its pointer words point
//! at) and `uses` (the instructions that use the data in a range).

use std::rc::Rc;

use kuna_console::assertions::Outcome;
use kuna_console::disasm::{hex, mapped_run_end, read_upto};
use kuna_console::engine::ConsoleProgram;
use kuna_console::inspect::{data_uses, section_rows, SectionRow};
use kuna_decomp::dtype::Datatype;

use crate::inspect::target_json;
use crate::json::{arr, assertions_json, Obj};

/// The most bytes one `data` window spans.
pub const DATA_MAX: u64 = 16 * 1024;
/// How far a centred window reaches before the item it shows.
const BEFORE: u64 = 128;
/// How many bytes a centred window spans at least.
const SPAN: u64 = 512;
/// The longest C string a pointer word's target is read for.
const TEXT_MAX: usize = 96;
/// The most fields a struct's layout lists.
const FIELD_MAX: usize = 256;

/// One global data symbol: its name, address and type.
struct Item {
    name: String,
    address: u64,
    ty: Rc<Datatype>,
}

impl Item {
    fn end(&self) -> u64 {
        self.address.saturating_add(u64::try_from(self.ty.get_size()).unwrap_or(0).max(1))
    }
}

/// The global scope's whole data symbols in the default data space, by
/// address: the labels the program's symbol table, debug info and the
/// session's `data` directives give.
fn items(prog: &ConsoleProgram) -> Vec<Item> {
    use kuna_decomp::dtype::type_metatype::TYPE_CODE;
    let arch = prog.arch();
    let (Some(scope), Some(space)) = (arch.symboltab.get_global_scope(), arch.manage().get_default_data_space()) else {
        return Vec::new();
    };
    let mut out: Vec<Item> = arch
        .symboltab
        .scope_space_symbol_specs(scope, space.get_index() as usize)
        .into_iter()
        .filter(|(_, ct, _, _)| ct.get_metatype() != TYPE_CODE)
        .map(|(name, ty, addr, _)| Item { name, address: addr.get_offset(), ty })
        .collect();
    out.sort_by(|a, b| a.address.cmp(&b.address).then_with(|| b.ty.get_size().cmp(&a.ty.get_size())));
    out
}

/// `type [N]`/`type *`/`ret (*)(args)` with `name` put where C declares it.
pub fn declare(type_name: &str, name: &str) -> String {
    if let Some(at) = type_name.find("(*") {
        return format!("{}{name}{}", &type_name[..at + 2], &type_name[at + 2..]);
    }
    let (front, back) = type_name.split_at(type_name.find('[').unwrap_or(type_name.len()));
    let front = front.trim_end();
    let gap = if front.ends_with('*') { "" } else { " " };
    format!("{front}{gap}{name}{back}")
}

/// The layout of `ty` the page decodes values with: `{kind, size, type}` plus
/// `signed` (integers and enums), `to` (a pointer's target, one level),
/// `elem`/`count` (arrays) and `fields` (structs and unions, by offset).
fn layout_json(prog: &ConsoleProgram, ty: &Rc<Datatype>, depth: usize) -> String {
    use kuna_decomp::dtype::type_metatype::*;
    let arch = prog.arch();
    let mut base = Rc::clone(ty);
    for _ in 0..8 {
        match base.get_typedef() {
            Some(next) if !Rc::ptr_eq(next, &base) => base = Rc::clone(next),
            _ => break,
        }
    }
    let size = base.get_size();
    let kind = match base.get_metatype() {
        TYPE_BOOL => "bool",
        TYPE_INT | TYPE_UINT if base.is_char_print() => "char",
        TYPE_INT | TYPE_UINT => "int",
        TYPE_FLOAT => "float",
        TYPE_PTR | TYPE_PTRREL => "pointer",
        TYPE_CODE => "code",
        TYPE_ARRAY => "array",
        TYPE_STRUCT => "struct",
        TYPE_UNION => "union",
        TYPE_ENUM_INT | TYPE_ENUM_UINT => "enum",
        TYPE_VOID => "void",
        _ => "unknown",
    };
    let mut o = Obj::new()
        .str("kind", kind)
        .num("size", size)
        .str("type", &kuna_decomp::printc::type_to_c_string(arch, ty));
    if matches!(base.get_metatype(), TYPE_INT | TYPE_ENUM_INT) {
        o = o.bool("signed", true);
    } else if matches!(base.get_metatype(), TYPE_UINT | TYPE_ENUM_UINT | TYPE_BOOL) {
        o = o.bool("signed", false);
    }
    match kind {
        "pointer" => {
            if let Some(to) = base.get_ptr_to() {
                o = o.raw("to", &layout_json(prog, &to, 0));
            }
        }
        "array" if depth > 0 => {
            if let (Some(elem), Some(count)) = (base.get_array_base(), base.num_elements()) {
                o = o.raw("elem", &layout_json(prog, &elem, depth - 1)).num("count", count);
            }
        }
        "struct" | "union" if depth > 0 => {
            let n = usize::try_from(base.num_depend()).unwrap_or(0).min(FIELD_MAX);
            let fields = arr((0..n).filter_map(|i| {
                let f = base.get_field(i32::try_from(i).ok()?)?;
                Some(
                    Obj::new()
                        .str("name", &f.name)
                        .num("offset", f.offset)
                        .raw("type", &layout_json(prog, &f.field_type, depth - 1))
                        .end(),
                )
            }));
            o = o.raw("fields", &fields);
        }
        _ => {}
    }
    o.end()
}

/// The section holding `addr`: the allocated section table's row, else none.
fn section_of(sections: &[SectionRow], addr: u64) -> Option<&SectionRow> {
    sections.iter().find(|s| addr >= s.address && addr - s.address < s.size.max(1))
}

/// The window a `data` request shows: `[addr, addr + len)` when the caller
/// names a length, else about [`SPAN`] bytes from [`BEFORE`] ahead of the item
/// holding `addr`, widened to whole items at both ends; never more than
/// [`DATA_MAX`] bytes, never outside the section (or mapped run) holding `addr`.
fn window(prog: &ConsoleProgram, sections: &[SectionRow], all: &[Item], addr: u64, len: Option<u64>) -> (u64, u64) {
    let (lo, hi) = match section_of(sections, addr) {
        Some(s) => (s.address, s.address.saturating_add(s.size.max(1))),
        None => match mapped_run_end(prog, addr) {
            Some((end, _)) => (addr.saturating_sub(BEFORE), end),
            None => (addr.saturating_sub(BEFORE), addr.saturating_add(SPAN)),
        },
    };
    if let Some(len) = len {
        let start = addr.max(lo);
        return (start, start.saturating_add(len.min(DATA_MAX)).min(hi));
    }
    let holder = all.iter().filter(|i| i.address <= addr && addr < i.end()).max_by_key(|i| i.address);
    let (first, last) = holder.map_or((addr, addr.saturating_add(1)), |i| (i.address, i.end()));
    let mut start = if last.saturating_sub(first) > DATA_MAX { addr } else { first }.saturating_sub(BEFORE);
    start = (start.max(lo) & !0xf).max(lo);
    if let Some(i) = all.iter().find(|i| i.address < start && start < i.end()) {
        if last.saturating_sub(i.address) <= DATA_MAX {
            start = i.address.max(lo);
        }
    }
    let mut end = last.max(start.saturating_add(SPAN)).min(start.saturating_add(DATA_MAX)).min(hi);
    if let Some(i) = all.iter().find(|i| i.address < end && end < i.end()) {
        if i.end().saturating_sub(start) <= DATA_MAX {
            end = i.end().min(hi);
        }
    }
    (start, end.max(start))
}

/// The NUL-terminated printable text at `addr`, when there is some.
fn c_string(prog: &ConsoleProgram, addr: u64) -> Option<String> {
    let mut bytes = Vec::new();
    let end = mapped_run_end(prog, addr).map_or(TEXT_MAX as u64, |(end, _)| end.saturating_sub(addr));
    let got = read_upto(prog, addr, (TEXT_MAX as u64).min(end) as usize + 1, &mut bytes);
    bytes.truncate(got);
    let nul = bytes.iter().position(|&b| b == 0)?;
    let text = &bytes[..nul];
    let printable = |b: &u8| (0x20..0x7f).contains(b) || matches!(b, b'\t' | b'\n' | b'\r');
    (!text.is_empty() && text.iter().all(printable)).then(|| String::from_utf8_lossy(text).into_owned())
}

/// The `data` document: `{binary, address, start, size, bytes, pointer_size,
/// target, section, items:[{name, address, size, type, declaration, layout}],
/// targets:[{at, to, name, offset, text}], assertions}`. `bytes` are the
/// window's bytes as loaded (overlays applied; a section's tail past its file
/// data reads as the loader's zeros), stopping where the mapped run does.
/// `items` are the global data symbols overlapping the window. `targets` are
/// the window's pointer-aligned words that hold an address inside an allocated
/// section: the data symbol or function there (`name`, `offset` into it) and
/// the C string starting there (`text`), when there is one.
pub fn data_json(binary: &str, prog: &ConsoleProgram, addr: u64, len: Option<u64>, assertions: &[Outcome]) -> String {
    let sections = section_rows(prog);
    let all = items(prog);
    let (start, end) = window(prog, &sections, &all, addr, len);
    let mut bytes = Vec::new();
    let run_end = mapped_run_end(prog, start).map_or(end, |(e, _)| e.min(end));
    let got = if prog.vma_bytes_mapped(start) {
        read_upto(prog, start, run_end.saturating_sub(start) as usize, &mut bytes)
    } else {
        0
    };
    bytes.truncate(got);
    let pointer_size = prog
        .arch()
        .manage()
        .get_default_data_space()
        .map_or(8, |s| u64::from(s.get_addr_size()));
    let big = prog.arch().archid.split(':').nth(1) == Some("BE");
    let functions: Vec<(u64, String)> =
        prog.function_entries_canonical().into_iter().map(|e| (e.addr.get_offset(), e.name)).collect();
    let in_window = |i: &&Item| i.address < end.max(start.saturating_add(1)) && i.end() > start;
    let shown: Vec<&Item> = all.iter().filter(in_window).collect();
    let item_rows = arr(shown.iter().map(|i| {
        let type_name = kuna_decomp::printc::type_to_c_string(prog.arch(), &i.ty);
        Obj::new()
            .str("name", &i.name)
            .addr("address", i.address)
            .num("size", i.ty.get_size())
            .str("type", &type_name)
            .str("declaration", &declare(&type_name, &i.name))
            .raw("layout", &layout_json(prog, &i.ty, 3))
            .end()
    }));
    let mut targets = Vec::new();
    let held = start.saturating_add(got as u64);
    let first = start.div_ceil(pointer_size.max(1)).checked_mul(pointer_size);
    if let (true, Some(first)) = ((1..=8).contains(&pointer_size), first) {
        let width = pointer_size as usize;
        let mut at = first;
        while at.checked_add(pointer_size).is_some_and(|next| next <= held) {
            let k = (at - start) as usize;
            let word = &bytes[k..k + width];
            let mut value: u64 = 0;
            for i in 0..width {
                let b = if big { word[i] } else { word[width - 1 - i] };
                value = (value << 8) | u64::from(b);
            }
            if value != 0 && section_of(&sections, value).is_some() {
                let symbol = all.iter().filter(|i| i.address <= value && value < i.end()).max_by_key(|i| i.address);
                let function = functions.iter().find(|(a, _)| *a == value);
                let (name, offset) = match (symbol, function) {
                    (_, Some((_, n))) => (Some(n.as_str()), 0),
                    (Some(i), None) => (Some(i.name.as_str()), value - i.address),
                    _ => (None, 0),
                };
                let code = section_of(&sections, value).is_some_and(|s| s.executable);
                let text = if code { None } else { c_string(prog, value) };
                targets.push(
                    Obj::new()
                        .addr("at", at)
                        .addr("to", value)
                        .opt_str("name", name)
                        .num("offset", offset)
                        .opt_str("text", text.as_deref())
                        .end(),
                );
            }
            at += pointer_size;
        }
    }
    let section = section_of(&sections, addr).map_or_else(
        || "null".to_string(),
        |s| {
            Obj::new()
                .str("name", &s.name)
                .addr("address", s.address)
                .num("size", s.size)
                .opt_num("file_offset", s.file_offset)
                .opt_num("file_size", s.file_size)
                .bool("executable", s.executable)
                .bool("writable", s.writable)
                .end()
        },
    );
    Obj::new()
        .str("binary", binary)
        .addr("address", addr)
        .addr("start", start)
        .num("size", got)
        .num("span", end - start)
        .str("bytes", &hex(&bytes))
        .num("pointer_size", pointer_size)
        .raw("target", &target_json(prog))
        .raw("section", &section)
        .raw("items", &item_rows)
        .raw("targets", &arr(targets))
        .raw("assertions", &assertions_json(assertions))
        .end()
}

/// The `uses` document: `{binary, address, size, uses:[{to, name, address,
/// at, kind, instruction, via}], assertions}`, the instructions that use the
/// data in `[addr, addr + len)` ([`data_uses`]); a use read through a pointer
/// word names that word in `via`. The items a pointer word can point at are
/// the range's data symbols, its first byte, and every byte the code names
/// directly.
pub fn uses_json(
    binary: &str,
    prog: &ConsoleProgram,
    file: &object::File,
    addr: u64,
    len: u64,
    assertions: &[Outcome],
) -> String {
    let end = addr.saturating_add(len);
    let mut starts: Vec<u64> = items(prog).iter().map(|i| i.address).filter(|&a| a >= addr && a < end).collect();
    starts.push(addr);
    let uses = data_uses(prog, file, addr, end, &starts);
    let rows = arr(uses.iter().map(|(to, u)| {
        let via = u.via.as_ref().map_or_else(
            || "null".to_string(),
            |(slot, name)| Obj::new().opt_str("name", name.as_deref()).addr("address", *slot).end(),
        );
        Obj::new()
            .addr("to", *to)
            .opt_str("name", u.row.name.as_deref())
            .addr("address", prog.output_code_offset(u.row.address))
            .addr("at", prog.output_code_offset(u.row.site))
            .str("kind", u.row.kind)
            .str("instruction", &u.row.instruction)
            .raw("via", &via)
            .end()
    }));
    Obj::new()
        .str("binary", binary)
        .addr("address", addr)
        .num("size", len)
        .raw("uses", &rows)
        .raw("assertions", &assertions_json(assertions))
        .end()
}

#[cfg(test)]
mod tests {
    use super::declare;

    #[test]
    fn declarations_put_the_name_where_c_does() {
        assert_eq!(declare("int", "count"), "int count");
        assert_eq!(declare("char *", "banner"), "char *banner");
        assert_eq!(declare("int [8]", "primes"), "int primes[8]");
        assert_eq!(declare("char [12]", "greeting"), "char greeting[12]");
        assert_eq!(declare("int (*)(int)", "op"), "int (*op)(int)");
        assert_eq!(declare("struct point", "origin"), "struct point origin");
        assert_eq!(declare("char *[4]", "names"), "char *names[4]");
        assert_eq!(declare("int (*)[4]", "rows"), "int (*rows)[4]");
    }
}
