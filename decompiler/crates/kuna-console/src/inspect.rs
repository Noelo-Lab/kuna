//! What a single-function study view needs beyond the batch record: the token
//! source map of the rendered C ([`FuncDetail`]), the function's instruction
//! listing ([`function_rows`]), its references ([`function_xrefs`]), the uses
//! of the image's string literals ([`string_uses`]) and the image's section
//! table with file offsets ([`section_rows`]).

use std::collections::{BTreeMap, BTreeSet};

use kuna_decomp::decompile_drive::VarInfo;
use kuna_decomp::funcdata::Funcdata;
use kuna_decomp::kuna_srcmap::{self, CodeToken};
use kuna_decomp::prettyprint::EmitToken;
use kuna_sleigh::loadimage::section_flags;

use crate::disasm::{self, mapped_run_end, Row, Window};
use crate::engine::ConsoleProgram;

/// How many rows [`function_rows`] lists before it reports a truncated listing.
pub const FUNCTION_ROW_CAP: usize = 4096;

/// The token source map of one rendered function: the tokens when they rebuild
/// the code exactly, else none and the reason; and the global data each
/// variable token names.
#[derive(Debug, Clone, Default)]
pub struct FuncDetail {
    pub tokens: Vec<CodeToken>,
    pub tokens_error: Option<String>,
    /// The global data symbols the code names, in order of first use.
    pub globals: Vec<GlobalRef>,
    /// Per token, the index in `globals` of the global it names.
    pub token_globals: Vec<Option<usize>>,
}

/// A global data symbol the code names: its name, address, size and type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlobalRef {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub type_name: String,
}

impl FuncDetail {
    /// Resolve and verify the captured tokens of `code`, reporting addresses in
    /// the program's output units.
    pub fn resolve(
        prog: &ConsoleProgram,
        fd: &Funcdata,
        var_refs: &[BTreeSet<u64>],
        raw: &[EmitToken],
        untrimmed: &str,
        code: &str,
        variables: &[VarInfo],
    ) -> Self {
        let mut tokens = kuna_srcmap::resolve(fd, var_refs, raw, untrimmed, variables);
        if let Err(error) = kuna_srcmap::verify(&tokens, code) {
            return FuncDetail { tokens_error: Some(error), ..FuncDetail::default() };
        }
        for t in &mut tokens {
            t.address = t.address.map(|a| prog.output_code_offset(a));
            t.callee = t.callee.map(|a| prog.output_code_offset(a));
        }
        let (globals, token_globals) = global_refs(prog, fd, &tokens);
        FuncDetail { tokens, tokens_error: None, globals, token_globals }
    }
}

/// The global data symbol each variable token names: a name the function's own
/// scope does not hold that the global scope maps, whole, in the default data
/// space, with a type that is not code. A local with the same name shadows the
/// global, as it does in the C.
fn global_refs(prog: &ConsoleProgram, fd: &Funcdata, tokens: &[CodeToken]) -> (Vec<GlobalRef>, Vec<Option<usize>>) {
    use kuna_decomp::dtype::type_metatype::TYPE_CODE;
    use kuna_decomp::prettyprint::TokenKind;
    let arch = prog.arch();
    let db = &arch.symboltab;
    let (Some(scope), Some(data_space)) = (db.get_global_scope(), arch.manage().get_default_data_space()) else {
        return (Vec::new(), vec![None; tokens.len()]);
    };
    let mut globals: Vec<GlobalRef> = Vec::new();
    let mut by_name: BTreeMap<&str, Option<usize>> = BTreeMap::new();
    let lookup = |name: &str| -> Option<GlobalRef> {
        if fd.get_scope_local().is_some_and(|local| !local.query_by_name(name).is_empty()) {
            return None;
        }
        db.find_by_name(scope, name).into_iter().find_map(|sid| {
            let symbol = db.symbol(sid);
            let ct = symbol.dtype.as_ref()?;
            if ct.get_metatype() == TYPE_CODE {
                return None;
            }
            symbol.mapentry.iter().find_map(|&eref| {
                let entry = db.entry(scope, eref);
                let space = entry.get_addr().get_space()?;
                (entry.get_offset() == 0 && space.get_index() == data_space.get_index()).then(|| GlobalRef {
                    name: name.to_string(),
                    address: entry.get_addr().get_offset(),
                    size: u64::try_from(ct.get_size()).unwrap_or(0),
                    type_name: kuna_decomp::printc::type_to_c_string(arch, ct),
                })
            })
        })
    };
    let token_globals = tokens
        .iter()
        .map(|t| {
            if t.kind != TokenKind::Variable || t.var.is_some() || t.decl.is_some() {
                return None;
            }
            *by_name.entry(t.text.as_str()).or_insert_with(|| {
                let found = lookup(&t.text)?;
                Some(globals.iter().position(|g| g.address == found.address).unwrap_or_else(|| {
                    globals.push(found);
                    globals.len() - 1
                }))
            })
        })
        .collect();
    (globals, token_globals)
}

/// One loaded section, for mapping an address back to the input file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectionRow {
    pub name: String,
    pub address: u64,
    pub size: u64,
    /// Where the section's bytes start in the input file, when they come from it.
    pub file_offset: Option<u64>,
    /// How many bytes from `file_offset` the file holds.
    pub file_size: Option<u64>,
    pub executable: bool,
    pub writable: bool,
}

/// The program's loaded sections in address order: the loader's named table
/// where it names any allocated section, else its unnamed section spans. A section is
/// writable when the segment mapping it is (the loader leaves the READONLY bit
/// off the loader-owned tables on purpose), else when its own flags say so.
pub fn section_rows(prog: &ConsoleProgram) -> Vec<SectionRow> {
    let segments = prog.segments();
    let row = |name: String, address: u64, size: u64, file: (Option<u64>, Option<u64>), flags: u32| {
        let mapping = segments
            .iter()
            .find(|&&(start, len, _)| address >= start && address - start < len)
            .map(|&(_, _, seg)| seg);
        SectionRow {
            name,
            address,
            size,
            file_offset: file.0,
            file_size: file.1,
            executable: flags & section_flags::CODE != 0,
            writable: mapping.unwrap_or(flags) & section_flags::READONLY == 0,
        }
    };
    let mut rows: Vec<SectionRow> = prog
        .image_metadata()
        .map(|meta| {
            meta.sections
                .iter()
                .filter(|s| s.flags & section_flags::UNALLOC == 0)
                .map(|s| row(s.name.clone(), s.vma, s.size, (s.file_offset, s.file_size), s.flags))
                .collect()
        })
        .unwrap_or_default();
    if rows.is_empty() {
        rows = prog
            .sections()
            .into_iter()
            .filter(|&(_, _, flags)| flags & section_flags::UNALLOC == 0)
            .map(|(vma, size, flags)| row(String::new(), vma, size, (None, None), flags))
            .collect();
    }
    rows.sort_by(|a, b| a.address.cmp(&b.address).then_with(|| a.name.cmp(&b.name)));
    rows
}

/// The instruction listing of the function at `entry` spanning `size` bytes:
/// the CLI's `disassemble --follow` walk (branch targets start rows, literal
/// pools fold, undecodable bytes are `.byte` rows), at most `cap` rows. A
/// `size` of 0 (no known extent) lists forward to the end of the mapped run.
/// Returns the rows and whether the listing stopped short of the extent.
pub fn function_rows(prog: &ConsoleProgram, entry: u64, size: u64, cap: usize) -> (Vec<Row>, bool) {
    if !prog.vma_bytes_mapped(entry) {
        return (Vec::new(), false);
    }
    let window = Window {
        start: entry,
        end: (size > 0).then(|| entry.saturating_add(size)),
        derived: false,
        mapped_end: mapped_run_end(prog, entry).map(|(end, _)| end),
    };
    let walk = disasm::walk(prog, &window, Some(cap), true);
    let listed_to = walk.rows.last().map_or(entry, |r| r.addr.saturating_add(r.size));
    let short = window.stop().is_none_or(|stop| listed_to < stop) && walk.rows.len() >= cap;
    (walk.rows, walk.truncated || short)
}

/// One reference row: the other end's name and address, the instruction that
/// makes the reference, its kind (`call`, `jump`, `data`, `read`, `write`), and
/// that instruction's text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XrefRow {
    /// The caller for a caller row, the target for a callee or data row.
    pub name: Option<String>,
    pub address: u64,
    /// The referencing instruction.
    pub site: u64,
    pub kind: &'static str,
    pub instruction: String,
}

/// Who calls or jumps to a function, what it calls or jumps to, and the data it
/// reads, writes or takes the address of.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FunctionXrefs {
    pub callers: Vec<XrefRow>,
    pub callees: Vec<XrefRow>,
    pub data_refs: Vec<XrefRow>,
}

/// The references of the function at `entry`, from the `kuna xrefs` walk
/// (`kuna_analysis::listing::xrefs`) seeded with the program's inventory and
/// focused on `entry`. Callers are named by the function holding the call
/// site; targets by the function or global data symbol at them (a jump that
/// stays inside the function is control flow, and
/// `XrefIndex::refs_from_function` leaves it out). Every list is in
/// instruction order. Addresses are the program's byte addresses.
pub fn function_xrefs(prog: &ConsoleProgram, file: &object::File, entry: u64) -> FunctionXrefs {
    use kuna_analysis::listing::xrefs::{self as xr, XrefKind};
    let entries = prog.function_entries_canonical();
    let names: BTreeMap<u64, String> =
        entries.iter().map(|e| (e.addr.get_offset(), e.name.clone())).collect();
    let data: BTreeMap<u64, String> =
        prog.global_data_symbols().into_iter().map(|(name, vma, _)| (vma, name)).collect();
    let inventory: Vec<u64> = names.keys().copied().collect();
    let seeds = xr::discovery_seeds(file, &inventory, prog.arch().analysis_funcstart_patterns);
    let index = xr::build_with_focus(file, prog.arch(), prog.arch().translate(), &seeds, &[entry]);
    let name_of = |vma: u64| names.get(&vma).or_else(|| data.get(&vma)).cloned();
    let mut out = FunctionXrefs::default();
    for r in index.refs_to_unified(entry) {
        let caller = index.function_containing(r.from);
        out.callers.push(XrefRow {
            name: caller.and_then(name_of),
            address: caller.unwrap_or(r.from),
            site: r.from,
            kind: r.kind.as_str(),
            instruction: r.instruction.clone(),
        });
    }
    for r in index.refs_from_function(entry) {
        let row = XrefRow {
            name: name_of(r.to),
            address: r.to,
            site: r.from,
            kind: r.kind.as_str(),
            instruction: r.instruction.clone(),
        };
        match r.kind {
            XrefKind::Call | XrefKind::Jump => out.callees.push(row),
            XrefKind::Data | XrefKind::Read | XrefKind::Write => out.data_refs.push(row),
        }
    }
    out.callers.sort_by_key(|r| (r.site, r.address));
    out.callees.sort_by_key(|r| (r.site, r.address));
    out.data_refs.sort_by_key(|r| (r.site, r.address));
    out
}

/// One instruction that uses a string literal. `row` names the function the
/// instruction sits in, as a caller row does; `via` is the data word holding
/// the string's address, with its symbol name, when the instruction reads that
/// word instead of naming the string (`static const char *secret = "…"`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringUse {
    pub row: XrefRow,
    pub via: Option<(u64, Option<String>)>,
}

/// The uses of each string extent `(address, byte length)` in `strings`, in the
/// order given: every data reference landing anywhere in the extent, plus every
/// data reference to a pointer-aligned data word whose value is the start of a
/// string outside executable sections ([`pointer_slots`]). A branch into an
/// extent, or a reference to a function's entry or its Thumb address, is code
/// that happens to read as text, and a word pointing into code is a jump table
/// or a function pointer, so none of them is a use; nor is a load or store
/// into a string inside code, which is a literal-pool access (firmware keeps
/// its strings there, used by address). The reference walk is the
/// one behind [`function_xrefs`], built once for the whole set. Each list is in
/// instruction order.
pub fn string_uses(prog: &ConsoleProgram, file: &object::File, strings: &[(u64, u64)]) -> Vec<Vec<StringUse>> {
    use kuna_analysis::listing::xrefs as xr;
    if strings.is_empty() {
        return Vec::new();
    }
    let walk = UseWalk::build(prog, file);
    let pointed = strings.iter().map(|&(addr, _)| addr).filter(|&addr| addr != 0 && !walk.in_code(addr)).collect();
    let slots = pointer_slots(file, &pointed);
    strings
        .iter()
        .map(|&(addr, len)| {
            let code_text = walk.in_code(addr);
            let mut uses: Vec<StringUse> = (addr..addr.saturating_add(len.max(1)))
                .filter(|&vma| !walk.is_entry(vma))
                .flat_map(|vma| walk.index.refs_to(vma))
                .filter(|r| if code_text { r.kind == xr::XrefKind::Data } else { is_data(r) })
                .map(|r| walk.use_of(r, None))
                .collect();
            if !walk.is_entry(addr) {
                for &slot in slots.get(&addr).into_iter().flatten() {
                    uses.extend(walk.index.refs_to(slot).iter().filter(|r| is_data(r)).map(|r| walk.use_of(r, Some(slot))));
                }
            }
            uses.sort_by_key(|u| (u.row.site, u.via.as_ref().map(|v| v.0)));
            uses.dedup_by_key(|u| (u.row.site, u.via.as_ref().map(|v| v.0)));
            uses
        })
        .collect()
}

/// The uses of the data in `[start, end)`, each with the address it names:
/// every data reference landing inside the range, at the byte it names, and
/// every data reference to a pointer-aligned data word whose value is one of
/// `starts` (the items of the range) or a byte the code names directly, at
/// that item. As for [`string_uses`], a branch, a reference to a function's
/// entry, a word pointing into code and a load or store into code are not
/// uses. In address, then instruction order.
pub fn data_uses(
    prog: &ConsoleProgram,
    file: &object::File,
    start: u64,
    end: u64,
    starts: &[u64],
) -> Vec<(u64, StringUse)> {
    use kuna_analysis::listing::xrefs as xr;
    if start >= end {
        return Vec::new();
    }
    let walk = UseWalk::build(prog, file);
    let mut uses: Vec<(u64, StringUse)> = Vec::new();
    for vma in (start..end).filter(|&vma| !walk.is_entry(vma)) {
        let code_data = walk.in_code(vma);
        for r in walk.index.refs_to(vma) {
            if if code_data { r.kind == xr::XrefKind::Data } else { is_data(r) } {
                uses.push((vma, walk.use_of(r, None)));
            }
        }
    }
    let pointed: BTreeSet<u64> = starts
        .iter()
        .copied()
        .chain(uses.iter().map(|(to, _)| *to))
        .filter(|&addr| addr != 0 && addr >= start && addr < end && !walk.in_code(addr) && !walk.is_entry(addr))
        .collect();
    for (&target, list) in &pointer_slots(file, &pointed) {
        for &slot in list.iter().filter(|&&slot| slot != target) {
            uses.extend(walk.index.refs_to(slot).iter().filter(|r| is_data(r)).map(|r| (target, walk.use_of(r, Some(slot)))));
        }
    }
    uses.sort_by_key(|(to, u)| (*to, u.row.site, u.via.as_ref().map(|v| v.0)));
    uses.dedup_by_key(|(to, u)| (*to, u.row.site, u.via.as_ref().map(|v| v.0)));
    uses
}

fn is_data(r: &kuna_analysis::listing::xrefs::Xref) -> bool {
    use kuna_analysis::listing::xrefs::XrefKind;
    !matches!(r.kind, XrefKind::Call | XrefKind::Jump)
}

/// The whole-image reference walk behind [`string_uses`] and [`data_uses`],
/// with the names of the functions and global data it reports.
struct UseWalk {
    names: BTreeMap<u64, String>,
    data: BTreeMap<u64, String>,
    index: kuna_analysis::listing::xrefs::XrefIndex,
    code: Vec<(u64, u64)>,
}

impl UseWalk {
    fn build(prog: &ConsoleProgram, file: &object::File) -> Self {
        use kuna_analysis::listing::xrefs as xr;
        use object::{Object, ObjectSection, SectionKind};
        let names: BTreeMap<u64, String> = prog
            .function_entries_canonical()
            .into_iter()
            .map(|e| (e.addr.get_offset(), e.name))
            .collect();
        let data: BTreeMap<u64, String> =
            prog.global_data_symbols().into_iter().map(|(name, vma, _)| (vma, name)).collect();
        let inventory: Vec<u64> = names.keys().copied().collect();
        let seeds = xr::discovery_seeds(file, &inventory, prog.arch().analysis_funcstart_patterns);
        let index = xr::build(file, prog.arch(), prog.arch().translate(), &seeds);
        let code = file
            .sections()
            .filter(|s| s.kind() == SectionKind::Text)
            .map(|s| (s.address(), s.address().saturating_add(s.size())))
            .collect();
        UseWalk { names, data, index, code }
    }

    fn in_code(&self, addr: u64) -> bool {
        self.code.iter().any(|&(lo, hi)| addr >= lo && addr < hi)
    }

    /// A function's entry, or its Thumb address.
    fn is_entry(&self, vma: u64) -> bool {
        self.names.contains_key(&vma) || (vma & 1 == 1 && self.names.contains_key(&(vma & !1)))
    }

    fn use_of(&self, r: &kuna_analysis::listing::xrefs::Xref, via: Option<u64>) -> StringUse {
        let function = self.index.function_containing(r.from);
        StringUse {
            row: XrefRow {
                name: function.and_then(|f| self.names.get(&f).cloned()),
                address: function.unwrap_or(r.from),
                site: r.from,
                kind: r.kind.as_str(),
                instruction: r.instruction.clone(),
            },
            via: via.map(|slot| (slot, self.data.get(&slot).cloned())),
        }
    }
}

/// The pointer-aligned words of `file`'s initialized data sections whose value
/// is one of `targets`, keyed by that value. Words are read at the image's
/// pointer width and byte order, as the file stores them.
fn pointer_slots(file: &object::File, targets: &BTreeSet<u64>) -> BTreeMap<u64, Vec<u64>> {
    use object::{Object, ObjectSection, SectionKind};
    let width: u64 = if file.is_64() { 8 } else { 4 };
    let little = file.is_little_endian();
    let mut out: BTreeMap<u64, Vec<u64>> = BTreeMap::new();
    for section in file.sections() {
        if !matches!(
            section.kind(),
            SectionKind::Data | SectionKind::ReadOnlyData | SectionKind::ReadOnlyDataWithRel
        ) {
            continue;
        }
        let Ok(bytes) = section.data() else { continue };
        let base = section.address();
        let skip = (width - base % width) % width;
        let Some(aligned) = bytes.get(skip as usize..) else { continue };
        for (i, word) in aligned.chunks_exact(width as usize).enumerate() {
            let mut buf = [0u8; 8];
            let value = if little {
                buf[..word.len()].copy_from_slice(word);
                u64::from_le_bytes(buf)
            } else {
                buf[8 - word.len()..].copy_from_slice(word);
                u64::from_be_bytes(buf)
            };
            if targets.contains(&value) {
                out.entry(value).or_default().push(base + skip + i as u64 * width);
            }
        }
    }
    out
}
