//! Mach-O export-trie names: the `LC_DYLD_EXPORTS_TRIE` (or `LC_DYLD_INFO`
//! `export_off`) blob dyld binds other images against.
//!
//! `object`'s `exports()` reads the `LC_DYSYMTAB` external-defined `nlist`
//! range, so a dylib whose symbol table was stripped (`strip -N`) or only
//! partly kept still exports names only this trie holds. [`exports`] walks it
//! as `mach-o/loader.h` lays it out: a node is a ULEB128 terminal size, the
//! terminal (ULEB128 flags, then an image-base offset; a re-export an ordinal
//! and a name instead, a stub-and-resolver a second offset), a child count,
//! and that many (NUL-terminated edge label, ULEB128 node offset) pairs.
//!
//! The walk is bounded by the blob, never by what the blob claims: each node
//! is entered at most once (a cycle or a shared child ends there), a ULEB128
//! wider than 64 bits or an offset past the end drops that node, an empty edge
//! label drops that edge and a name over [`MAX_NAME`] bytes the node's
//! remaining edges, the bytes read are capped at [`WORK_PER_BYTE`] per trie
//! byte and the names kept at [`NAMES_PER_BYTE`] per trie byte. A hostile trie
//! can neither hang the load nor allocate more than a small multiple of its
//! own size.

use object::macho::{
    EXPORT_SYMBOL_FLAGS_KIND_ABSOLUTE, EXPORT_SYMBOL_FLAGS_KIND_MASK,
    EXPORT_SYMBOL_FLAGS_KIND_REGULAR, EXPORT_SYMBOL_FLAGS_KIND_THREAD_LOCAL,
    EXPORT_SYMBOL_FLAGS_REEXPORT, EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER,
};
use object::read::macho::{LoadCommandVariant, MachHeader, MachOFile, Segment};
use object::read::ReadRef;

/// What an export's value means (`EXPORT_SYMBOL_FLAGS_KIND_*`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ExportKind {
    /// An address in this image (a weak definition and a stub-and-resolver's
    /// stub included).
    Regular,
    /// A thread-local variable's descriptor in this image.
    ThreadLocal,
    /// An absolute value, not an address in this image.
    Absolute,
}

/// One export the trie defines in this image (re-exports name another image's
/// symbol and are left out).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TrieExport {
    /// The name as the trie spells it (`_audit_exported_function`).
    pub name: Vec<u8>,
    /// The image base plus the export's offset, or the value of an absolute
    /// export.
    pub addr: u64,
    pub kind: ExportKind,
}

/// The longest name kept; a longer path is dropped.
const MAX_NAME: usize = 1 << 16;

/// Trie bytes the walk may read per byte of trie. A well-formed trie reads
/// each byte once.
const WORK_PER_BYTE: usize = 4;

/// Name bytes the walk may keep per byte of trie. Prefix sharing makes real
/// tries hold up to about three name bytes per trie byte.
const NAMES_PER_BYTE: usize = 16;

/// The exports a thin Mach-O's trie defines, sorted by address and name.
/// Empty for any other file, an image without a trie, or a trie outside the
/// file. A 32-bit ARM image's trie marks a Thumb function with the low address
/// bit, which its `nlist` entry does not carry; it is cleared, so both name the
/// same entry.
pub(crate) fn exports(file: &object::File) -> Vec<TrieExport> {
    use object::read::Object;
    let found = match file {
        object::File::MachO64(m) => locate(m),
        object::File::MachO32(m) => locate(m),
        _ => None,
    };
    let Some((base, blob)) = found else {
        return Vec::new();
    };
    let thumb = if file.architecture() == object::Architecture::Arm { !1 } else { !0 };
    let mut out: Vec<TrieExport> = walk(blob)
        .into_iter()
        .filter_map(|(name, kind, value)| {
            let addr = match kind {
                ExportKind::Absolute => value,
                ExportKind::Regular => base.checked_add(value)? & thumb,
                ExportKind::ThreadLocal => base.checked_add(value)?,
            };
            Some(TrieExport { name, addr, kind })
        })
        .collect();
    out.sort_unstable_by(|a, b| (a.addr, &a.name).cmp(&(b.addr, &b.name)));
    out
}

/// The file bytes a thin Mach-O's exports are read from: the header and load
/// commands (segments, sections, `LC_DYSYMTAB`), the `nlist` array, the
/// string table and the export trie. `None` for any other file.
pub(crate) fn export_inputs<'data>(file: &object::File<'data>) -> Option<[&'data [u8]; 4]> {
    match file {
        object::File::MachO64(m) => inputs(m),
        object::File::MachO32(m) => inputs(m),
        _ => None,
    }
}

fn inputs<'data, Mach, R>(file: &MachOFile<'data, Mach, R>) -> Option<[&'data [u8]; 4]>
where
    Mach: MachHeader,
    R: ReadRef<'data>,
{
    let endian = file.endian();
    let header = file.macho_header();
    let commands = std::mem::size_of::<Mach>() as u64 + u64::from(header.sizeofcmds(endian));
    let commands = file.data().read_bytes_at(0, commands).ok()?;
    let (mut symbols, mut strings): (&[u8], &[u8]) = (&[], &[]);
    let mut walk = file.macho_load_commands().ok()?;
    while let Ok(Some(command)) = walk.next() {
        if let Ok(Some(symtab)) = command.symtab() {
            let width = std::mem::size_of::<Mach::Nlist>() as u64;
            let count = u64::from(symtab.nsyms.get(endian));
            symbols = file
                .data()
                .read_bytes_at(symtab.symoff.get(endian).into(), count.saturating_mul(width))
                .unwrap_or(&[]);
            strings = file
                .data()
                .read_bytes_at(symtab.stroff.get(endian).into(), symtab.strsize.get(endian).into())
                .unwrap_or(&[]);
        }
    }
    let trie = locate(file).map_or(&[][..], |(_, blob)| blob);
    Some([commands, symbols, strings, trie])
}

/// The image base (the address of the segment that maps the header) and the
/// trie bytes of a thin image: `LC_DYLD_EXPORTS_TRIE`, else the
/// `LC_DYLD_INFO` export range.
fn locate<'data, Mach, R>(file: &MachOFile<'data, Mach, R>) -> Option<(u64, &'data [u8])>
where
    Mach: MachHeader,
    R: ReadRef<'data>,
{
    let endian = file.endian();
    let mut commands = file.macho_load_commands().ok()?;
    let mut base: Option<u64> = None;
    let mut trie: Option<(u64, u64)> = None;
    let mut info: Option<(u64, u64)> = None;
    while let Ok(Some(command)) = commands.next() {
        if let Ok(Some((segment, _))) = Mach::Segment::from_command(command) {
            if segment.fileoff(endian).into() == 0 && segment.filesize(endian).into() != 0 {
                base.get_or_insert(segment.vmaddr(endian).into());
            }
            continue;
        }
        match command.variant() {
            Ok(LoadCommandVariant::LinkeditData(d))
                if command.cmd() == object::macho::LC_DYLD_EXPORTS_TRIE =>
            {
                trie = Some((d.dataoff.get(endian).into(), d.datasize.get(endian).into()));
            }
            Ok(LoadCommandVariant::DyldInfo(d)) => {
                info = Some((d.export_off.get(endian).into(), d.export_size.get(endian).into()));
            }
            _ => {}
        }
    }
    let (off, size) = trie.or(info).filter(|&(_, size)| size != 0)?;
    Some((base?, file.data().read_bytes_at(off, size).ok()?))
}

/// Read a ULEB128 at `*at`, below `end`; `None` past `end` or wider than 64
/// bits.
fn uleb(data: &[u8], at: &mut usize, end: usize) -> Option<u64> {
    let mut value = 0u64;
    let mut shift = 0u32;
    loop {
        let byte = *data.get(*at).filter(|_| *at < end)?;
        *at += 1;
        let low = u64::from(byte & 0x7f);
        if shift > 63 || (shift == 63 && low > 1) {
            return None;
        }
        value |= low << shift;
        if byte & 0x80 == 0 {
            return Some(value);
        }
        shift += 7;
    }
}

/// The kind and value of the terminal in `data[at..end]`, or `None` for a
/// re-export, an unknown kind, or a terminal that runs past `end`.
fn terminal(data: &[u8], mut at: usize, end: usize) -> Option<(ExportKind, u64)> {
    let flags = uleb(data, &mut at, end)?;
    if flags & u64::from(EXPORT_SYMBOL_FLAGS_REEXPORT) != 0 {
        return None;
    }
    let value = uleb(data, &mut at, end)?;
    if flags & u64::from(EXPORT_SYMBOL_FLAGS_STUB_AND_RESOLVER) != 0 {
        uleb(data, &mut at, end)?;
    }
    let kind = match (flags & u64::from(EXPORT_SYMBOL_FLAGS_KIND_MASK)) as u8 {
        EXPORT_SYMBOL_FLAGS_KIND_REGULAR => ExportKind::Regular,
        EXPORT_SYMBOL_FLAGS_KIND_THREAD_LOCAL => ExportKind::ThreadLocal,
        EXPORT_SYMBOL_FLAGS_KIND_ABSOLUTE => ExportKind::Absolute,
        _ => return None,
    };
    Some((kind, value))
}

/// The walk's state: the nodes entered, the name of the current path, and
/// what is left of the read and name budgets. Running out of either ends the
/// walk with the names found so far.
struct Walk<'a> {
    trie: &'a [u8],
    entered: Vec<u64>,
    name: Vec<u8>,
    work: usize,
    names: usize,
    out: Vec<(Vec<u8>, ExportKind, u64)>,
}

impl Walk<'_> {
    /// Charge `bytes` read to the budget; `false` once it is spent.
    fn charge(&mut self, bytes: usize) -> bool {
        match self.work.checked_sub(bytes) {
            Some(left) => self.work = left,
            None => self.work = 0,
        }
        self.work != 0
    }

    /// Enter the node at `node` under the current name: record its terminal
    /// and return its edge cursor `(first edge, edge count, name length)`.
    /// `None` for a node already entered or one that runs past the trie.
    fn enter(&mut self, node: usize) -> Option<(usize, u8, usize)> {
        let (word, bit) = (node / 64, 1u64 << (node % 64));
        if *self.entered.get(word)? & bit != 0 {
            return None;
        }
        self.entered[word] |= bit;
        let len = self.trie.len();
        let mut at = node;
        let size = uleb(self.trie, &mut at, len);
        if !self.charge(at - node) {
            return None;
        }
        let size = usize::try_from(size?).ok()?;
        let children = at.checked_add(size).filter(|&c| c < len)?;
        if !self.charge(children + 1 - at) {
            return None;
        }
        if size != 0 && !self.name.is_empty() {
            if let Some((kind, value)) = terminal(self.trie, at, children) {
                match self.names.checked_sub(self.name.len()) {
                    Some(left) => self.names = left,
                    None => {
                        self.work = 0;
                        return None;
                    }
                }
                self.out.push((self.name.clone(), kind, value));
            }
        }
        Some((children + 1, self.trie[children], self.name.len()))
    }

    /// Read the edge at `cursor` under a name `depth` bytes long: its label's
    /// end, the child offset, and the cursor after it. `None` ends this node's
    /// edges (no label end within [`MAX_NAME`], or a bad offset).
    fn edge(&mut self, cursor: usize, depth: usize) -> Option<(usize, usize, usize)> {
        let window = self.trie.get(cursor..)?;
        let room = (MAX_NAME - depth + 1).min(self.work).min(window.len());
        let Some(label) = window[..room].iter().position(|&b| b == 0) else {
            self.charge(room);
            return None;
        };
        let mut at = cursor + label + 1;
        let child = uleb(self.trie, &mut at, self.trie.len());
        if !self.charge(at - cursor) {
            return None;
        }
        Some((cursor + label, usize::try_from(child?).ok()?, at))
    }
}

/// Every terminal of `trie` as `(name, kind, value)`, depth first in edge
/// order; see the module docs for the bounds.
fn walk(trie: &[u8]) -> Vec<(Vec<u8>, ExportKind, u64)> {
    if trie.is_empty() {
        return Vec::new();
    }
    let mut w = Walk {
        trie,
        entered: vec![0; trie.len().div_ceil(64)],
        name: Vec::new(),
        work: trie.len().saturating_mul(WORK_PER_BYTE),
        names: trie.len().saturating_mul(NAMES_PER_BYTE),
        out: Vec::new(),
    };
    let mut stack: Vec<(usize, u8, usize)> = w.enter(0).into_iter().collect();
    while let Some(&(cursor, left, depth)) = stack.last() {
        if w.work == 0 {
            break;
        }
        if left == 0 {
            stack.pop();
            continue;
        }
        let Some((label_end, child, next)) = w.edge(cursor, depth) else {
            stack.pop();
            continue;
        };
        if let Some(top) = stack.last_mut() {
            *top = (next, left - 1, depth);
        }
        if label_end == cursor || child >= trie.len() {
            continue;
        }
        w.name.truncate(depth);
        w.name.extend_from_slice(&trie[cursor..label_end]);
        if let Some(frame) = w.enter(child) {
            stack.push(frame);
        }
    }
    w.out
}

#[cfg(test)]
mod tests;
