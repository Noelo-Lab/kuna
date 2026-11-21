//! ECMA-335 metadata of a managed PE: the CLI header, the metadata root and
//! its heaps, and the `#~` tables the loader reads (II.22, II.24, II.25.3.3).

/// Table numbers (II.22).
pub const MODULE: usize = 0x00;
pub const TYPE_REF: usize = 0x01;
pub const TYPE_DEF: usize = 0x02;
pub const FIELD: usize = 0x04;
pub const METHOD_DEF: usize = 0x06;
pub const PARAM: usize = 0x08;
pub const MEMBER_REF: usize = 0x0a;
pub const STANDALONE_SIG: usize = 0x11;
pub const TYPE_SPEC: usize = 0x1b;
pub const FIELD_RVA: usize = 0x1d;
pub const NESTED_CLASS: usize = 0x29;
pub const METHOD_SPEC: usize = 0x2b;
const TABLES: usize = 0x2d;

/// The largest row count a table may declare; a count past it cannot be backed
/// by the 16 MiB of metadata a row index addresses.
const MAX_ROWS: u32 = 0x00ff_ffff;

#[derive(Clone, Copy)]
enum Col {
    U8x2,
    U16,
    U32,
    Str,
    Guid,
    Blob,
    Idx(usize),
    Coded(&'static [usize], u32),
}

const NONE: usize = usize::MAX;
const TYPE_DEF_OR_REF: Col = Col::Coded(&[TYPE_DEF, TYPE_REF, TYPE_SPEC], 2);
const HAS_CONSTANT: Col = Col::Coded(&[FIELD, PARAM, 0x17], 2);
const HAS_CUSTOM_ATTRIBUTE: Col = Col::Coded(
    &[
        METHOD_DEF, FIELD, TYPE_REF, TYPE_DEF, PARAM, 0x09, MEMBER_REF, MODULE, 0x0e, 0x17, 0x14,
        STANDALONE_SIG, 0x1a, TYPE_SPEC, 0x20, 0x23, 0x26, 0x27, 0x28, 0x2a, 0x2c, METHOD_SPEC,
    ],
    5,
);
const HAS_FIELD_MARSHAL: Col = Col::Coded(&[FIELD, PARAM], 1);
const HAS_DECL_SECURITY: Col = Col::Coded(&[TYPE_DEF, METHOD_DEF, 0x20], 2);
const MEMBER_REF_PARENT: Col = Col::Coded(&[TYPE_DEF, TYPE_REF, 0x1a, METHOD_DEF, TYPE_SPEC], 3);
const HAS_SEMANTICS: Col = Col::Coded(&[0x14, 0x17], 1);
const METHOD_DEF_OR_REF: Col = Col::Coded(&[METHOD_DEF, MEMBER_REF], 1);
const MEMBER_FORWARDED: Col = Col::Coded(&[FIELD, METHOD_DEF], 1);
const IMPLEMENTATION: Col = Col::Coded(&[0x26, 0x23, 0x27], 2);
const CUSTOM_ATTRIBUTE_TYPE: Col = Col::Coded(&[NONE, NONE, METHOD_DEF, MEMBER_REF, NONE], 3);
const RESOLUTION_SCOPE: Col = Col::Coded(&[MODULE, 0x1a, 0x23, TYPE_REF], 2);
const TYPE_OR_METHOD_DEF: Col = Col::Coded(&[TYPE_DEF, METHOD_DEF], 1);

use Col::{Blob, Guid, Idx, Str, U16, U32, U8x2};

/// The column layout of every table, by table number.
const SCHEMA: [&[Col]; TABLES] = [
    &[U16, Str, Guid, Guid, Guid],                                  // 00 Module
    &[RESOLUTION_SCOPE, Str, Str],                                  // 01 TypeRef
    &[U32, Str, Str, TYPE_DEF_OR_REF, Idx(FIELD), Idx(METHOD_DEF)], // 02 TypeDef
    &[Idx(FIELD)],                                                  // 03 FieldPtr
    &[U16, Str, Blob],                                              // 04 Field
    &[Idx(METHOD_DEF)],                                             // 05 MethodPtr
    &[U32, U16, U16, Str, Blob, Idx(PARAM)],                        // 06 MethodDef
    &[Idx(PARAM)],                                                  // 07 ParamPtr
    &[U16, U16, Str],                                               // 08 Param
    &[Idx(TYPE_DEF), TYPE_DEF_OR_REF],                              // 09 InterfaceImpl
    &[MEMBER_REF_PARENT, Str, Blob],                                // 0a MemberRef
    &[U8x2, HAS_CONSTANT, Blob],                                    // 0b Constant
    &[HAS_CUSTOM_ATTRIBUTE, CUSTOM_ATTRIBUTE_TYPE, Blob],           // 0c CustomAttribute
    &[HAS_FIELD_MARSHAL, Blob],                                     // 0d FieldMarshal
    &[U16, HAS_DECL_SECURITY, Blob],                                // 0e DeclSecurity
    &[U16, U32, Idx(TYPE_DEF)],                                     // 0f ClassLayout
    &[U32, Idx(FIELD)],                                             // 10 FieldLayout
    &[Blob],                                                        // 11 StandAloneSig
    &[Idx(TYPE_DEF), Idx(0x14)],                                    // 12 EventMap
    &[Idx(0x14)],                                                   // 13 EventPtr
    &[U16, Str, TYPE_DEF_OR_REF],                                   // 14 Event
    &[Idx(TYPE_DEF), Idx(0x17)],                                    // 15 PropertyMap
    &[Idx(0x17)],                                                   // 16 PropertyPtr
    &[U16, Str, Blob],                                              // 17 Property
    &[U16, Idx(METHOD_DEF), HAS_SEMANTICS],                         // 18 MethodSemantics
    &[Idx(TYPE_DEF), METHOD_DEF_OR_REF, METHOD_DEF_OR_REF],         // 19 MethodImpl
    &[Str],                                                         // 1a ModuleRef
    &[Blob],                                                        // 1b TypeSpec
    &[U16, MEMBER_FORWARDED, Str, Idx(0x1a)],                       // 1c ImplMap
    &[U32, Idx(FIELD)],                                             // 1d FieldRVA
    &[U32, U32],                                                    // 1e EncLog
    &[U32],                                                         // 1f EncMap
    &[U32, U16, U16, U16, U16, U32, Blob, Str, Str],                // 20 Assembly
    &[U32],                                                         // 21 AssemblyProcessor
    &[U32, U32, U32],                                               // 22 AssemblyOS
    &[U16, U16, U16, U16, U32, Blob, Str, Str, Blob],               // 23 AssemblyRef
    &[U32, Idx(0x23)],                                              // 24 AssemblyRefProcessor
    &[U32, U32, U32, Idx(0x23)],                                    // 25 AssemblyRefOS
    &[U32, Str, Blob],                                              // 26 File
    &[U32, U32, Str, Str, IMPLEMENTATION],                          // 27 ExportedType
    &[U32, U32, Str, IMPLEMENTATION],                               // 28 ManifestResource
    &[Idx(TYPE_DEF), Idx(TYPE_DEF)],                                // 29 NestedClass
    &[U16, U16, TYPE_OR_METHOD_DEF, Str],                           // 2a GenericParam
    &[METHOD_DEF_OR_REF, Blob],                                     // 2b MethodSpec
    &[Idx(0x2a), TYPE_DEF_OR_REF],                                  // 2c GenericParamConstraint
];

fn u16_at(b: &[u8], off: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(off..off.checked_add(2)?)?.try_into().ok()?))
}

fn u32_at(b: &[u8], off: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(off..off.checked_add(4)?)?.try_into().ok()?))
}

/// One PE section's mapping.
#[derive(Clone, Copy, Debug)]
pub struct Section {
    pub rva: u32,
    pub vsize: u32,
    pub raw: u32,
    pub raw_size: u32,
}

/// The PE layout a managed image needs: its sections and the CLI header's
/// data directory.
#[derive(Debug)]
pub struct PeLayout {
    pub sections: Vec<Section>,
    pub cli_rva: u32,
}

/// Whether the start of a file is a PE whose data directory 14 names a CLI
/// header; `None` when `prefix` ends before that directory.
pub fn has_cli_header(prefix: &[u8]) -> Option<bool> {
    if prefix.get(..2) != Some(b"MZ") {
        return Some(false);
    }
    let pe = u32_at(prefix, 0x3c)? as usize;
    if prefix.get(pe..pe.checked_add(4)?)? != b"PE\0\0" {
        return Some(false);
    }
    let opt = pe + 24;
    let dirs = match u16_at(prefix, opt)? {
        0x10b => opt + 96,
        0x20b => opt + 112,
        _ => return Some(false),
    };
    if u32_at(prefix, dirs - 4)? < 15 {
        return Some(false);
    }
    Some(u32_at(prefix, dirs + 14 * 8)? != 0)
}

impl PeLayout {
    /// Read the PE headers; `None` unless the image carries a CLI header.
    pub fn parse(b: &[u8]) -> Option<PeLayout> {
        if b.get(..2)? != b"MZ" {
            return None;
        }
        let pe = u32_at(b, 0x3c)? as usize;
        if b.get(pe..pe.checked_add(4)?)? != b"PE\0\0" {
            return None;
        }
        let coff = pe + 4;
        let nsections = usize::from(u16_at(b, coff + 2)?);
        let opt_size = usize::from(u16_at(b, coff + 16)?);
        let opt = coff + 20;
        let dirs = match u16_at(b, opt)? {
            0x10b => opt + 96,
            0x20b => opt + 112,
            _ => return None,
        };
        let ndirs = u32_at(b, dirs - 4)?;
        if ndirs < 15 || dirs + 15 * 8 > opt + opt_size {
            return None;
        }
        let cli_rva = u32_at(b, dirs + 14 * 8)?;
        if cli_rva == 0 {
            return None;
        }
        let table = opt.checked_add(opt_size)?;
        let mut sections = Vec::with_capacity(nsections.min(96));
        for i in 0..nsections.min(96) {
            let s = table + i * 40;
            sections.push(Section {
                vsize: u32_at(b, s + 8)?,
                rva: u32_at(b, s + 12)?,
                raw_size: u32_at(b, s + 16)?,
                raw: u32_at(b, s + 20)?,
            });
        }
        Some(PeLayout { sections, cli_rva })
    }

    /// The file offset holding `rva`, when a section's raw data covers it.
    pub fn offset(&self, rva: u32) -> Option<usize> {
        self.sections.iter().find_map(|s| {
            let delta = rva.checked_sub(s.rva)?;
            (delta < s.raw_size).then(|| s.raw.checked_add(delta)).flatten().map(|o| o as usize)
        })
    }
}

/// A decoded row: its columns widened to `u32`.
pub type Row = [u32; 9];

/// The parsed metadata of one module.
pub struct Metadata<'a> {
    strings: &'a [u8],
    blobs: &'a [u8],
    user_strings: &'a [u8],
    rows: Vec<Vec<Row>>,
    /// The CLI header's entry point token, zero when it has none.
    pub entry_token: u32,
}

impl<'a> Metadata<'a> {
    /// Read the CLI header at `pe.cli_rva`, the metadata root it names and the
    /// `#~` stream's tables.
    pub fn parse(file: &'a [u8], pe: &PeLayout) -> Option<Metadata<'a>> {
        let cli = pe.offset(pe.cli_rva)?;
        let md_rva = u32_at(file, cli + 8)?;
        let md_size = u32_at(file, cli + 12)? as usize;
        let entry_token = u32_at(file, cli + 20)?;
        let md_off = pe.offset(md_rva)?;
        let md = file.get(md_off..md_off.checked_add(md_size)?)?;
        if u32_at(md, 0)? != 0x424a_5342 {
            return None;
        }
        let version_len = u32_at(md, 12)? as usize;
        let mut at = 16usize.checked_add(version_len)?.checked_add(2)?;
        let nstreams = u16_at(md, at)?;
        at += 2;
        let (mut tables, mut strings, mut blobs, mut user_strings) = (None, &md[..0], &md[..0], &md[..0]);
        for _ in 0..nstreams {
            let off = u32_at(md, at)? as usize;
            let size = u32_at(md, at + 4)? as usize;
            let name_start = at + 8;
            let name_len = md.get(name_start..)?.iter().take(32).position(|&c| c == 0)?;
            let name = &md[name_start..name_start + name_len];
            at = name_start + ((name_len + 4) & !3);
            let data = md.get(off..off.checked_add(size)?)?;
            match name {
                b"#~" => tables = Some(data),
                b"#Strings" => strings = data,
                b"#Blob" => blobs = data,
                b"#US" => user_strings = data,
                _ => {}
            }
        }
        let rows = read_tables(tables?)?;
        Some(Metadata { strings, blobs, user_strings, rows, entry_token })
    }

    /// The rows of `table`.
    pub fn table(&self, table: usize) -> &[Row] {
        self.rows.get(table).map_or(&[], Vec::as_slice)
    }

    /// Row `rid` (1-based) of `table`.
    pub fn row(&self, table: usize, rid: u32) -> Option<&Row> {
        self.table(table).get((rid as usize).checked_sub(1)?)
    }

    /// The `#Strings` entry at `off`.
    pub fn string(&self, off: u32) -> &'a str {
        let tail = self.strings.get(off as usize..).unwrap_or(&[]);
        let len = tail.iter().position(|&c| c == 0).unwrap_or(tail.len());
        std::str::from_utf8(&tail[..len]).unwrap_or("")
    }

    /// The `#Blob` entry at `off`.
    pub fn blob(&self, off: u32) -> &'a [u8] {
        heap_entry(self.blobs, off).unwrap_or(&[])
    }

    /// The `#US` entry at `off` as UTF-16 code units.
    pub fn user_string(&self, off: u32) -> Option<Vec<u16>> {
        let entry = heap_entry(self.user_strings, off)?;
        Some(entry.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect())
    }

    /// The rows `[first, end)` of `child` owned by row `rid` of `parent`, where
    /// column `col` of `parent` is the first owned row (II.22.37).
    pub fn owned(&self, parent: usize, rid: u32, col: usize, child: usize) -> std::ops::Range<u32> {
        let total = self.table(child).len() as u32 + 1;
        let first = self.row(parent, rid).map_or(total, |r| r[col]).clamp(1, total);
        let end = self.row(parent, rid + 1).map_or(total, |r| r[col]).clamp(first, total);
        first..end
    }
}

/// A compressed unsigned integer (II.23.2) and the bytes it took.
pub fn compressed(b: &[u8]) -> Option<(u32, usize)> {
    let first = *b.first()?;
    if first & 0x80 == 0 {
        Some((u32::from(first), 1))
    } else if first & 0xc0 == 0x80 {
        Some(((u32::from(first & 0x3f) << 8) | u32::from(*b.get(1)?), 2))
    } else if first & 0xe0 == 0xc0 {
        let rest = b.get(1..4)?;
        Some((
            (u32::from(first & 0x1f) << 24)
                | (u32::from(rest[0]) << 16)
                | (u32::from(rest[1]) << 8)
                | u32::from(rest[2]),
            4,
        ))
    } else {
        None
    }
}

fn heap_entry(heap: &[u8], off: u32) -> Option<&[u8]> {
    let tail = heap.get(off as usize..)?;
    let (len, used) = compressed(tail)?;
    tail.get(used..used.checked_add(len as usize)?)
}

fn read_tables(t: &[u8]) -> Option<Vec<Vec<Row>>> {
    let heap_sizes = *t.get(6)?;
    let valid = u64::from_le_bytes(t.get(8..16)?.try_into().ok()?);
    if valid >> TABLES != 0 {
        return None;
    }
    let mut counts = [0u32; TABLES];
    let mut at = 24usize;
    for (i, count) in counts.iter_mut().enumerate() {
        if valid >> i & 1 != 0 {
            *count = u32_at(t, at)?;
            if *count > MAX_ROWS {
                return None;
            }
            at += 4;
        }
    }
    let width = |col: Col| -> usize {
        let wide = |rows: u32, bits: u32| if rows < 1 << (16 - bits) { 2 } else { 4 };
        match col {
            Col::U8x2 | Col::U16 => 2,
            Col::U32 => 4,
            Col::Str => if heap_sizes & 1 != 0 { 4 } else { 2 },
            Col::Guid => if heap_sizes & 2 != 0 { 4 } else { 2 },
            Col::Blob => if heap_sizes & 4 != 0 { 4 } else { 2 },
            Col::Idx(table) => wide(counts[table], 0),
            Col::Coded(tables, bits) => {
                let most = tables.iter().filter(|&&t| t != NONE).map(|&t| counts[t]).max().unwrap_or(0);
                wide(most, bits)
            }
        }
    };
    let mut rows = Vec::with_capacity(TABLES);
    for (i, schema) in SCHEMA.iter().enumerate() {
        let widths: Vec<usize> = schema.iter().map(|&c| width(c)).collect();
        let row_size: usize = widths.iter().sum();
        let count = counts[i] as usize;
        let end = at.checked_add(count.checked_mul(row_size)?)?;
        let data = t.get(at..end)?;
        let mut table = Vec::with_capacity(count);
        for raw in data.chunks_exact(row_size.max(1)).take(count) {
            let mut row: Row = [0; 9];
            let mut off = 0;
            for (slot, &w) in row.iter_mut().zip(&widths) {
                *slot = if w == 2 { u32::from(u16_at(raw, off)?) } else { u32_at(raw, off)? };
                off += w;
            }
            table.push(row);
        }
        rows.push(table);
        at = end;
    }
    Some(rows)
}

/// Split a coded index into `(table, rid)`.
pub fn decode_coded(value: u32, tables: &[usize], bits: u32) -> Option<(usize, u32)> {
    let table = *tables.get((value & ((1 << bits) - 1)) as usize)?;
    Some((table, value >> bits))
}

/// `TypeDefOrRef` (II.24.2.6).
pub fn type_def_or_ref(value: u32) -> Option<(usize, u32)> {
    decode_coded(value, &[TYPE_DEF, TYPE_REF, TYPE_SPEC], 2)
}

/// `MemberRefParent` (II.24.2.6).
pub fn member_ref_parent(value: u32) -> Option<(usize, u32)> {
    decode_coded(value, &[TYPE_DEF, TYPE_REF, 0x1a, METHOD_DEF, TYPE_SPEC], 3)
}

/// `MethodDefOrRef` (II.24.2.6).
pub fn method_def_or_ref(value: u32) -> Option<(usize, u32)> {
    decode_coded(value, &[METHOD_DEF, MEMBER_REF], 1)
}
