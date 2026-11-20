//! Bounded method-table statements for named Go AMD64 ELF binaries.

use std::collections::{BTreeMap, BTreeSet};

use object::{Object, ObjectSection, ObjectSymbol};

use crate::pclntab::MethodEntries;

use super::{
    read_go_name, read_type, word, GoImage, GoType, TypeMeta, KIND_FUNC, KIND_INT, KIND_INT64,
    KIND_POINTER, KIND_STRUCT, KIND_UINT, KIND_UINT64, TFLAG_NAMED,
};

const UNCOMMON: u8 = 1;

#[derive(Clone, Debug)]
pub struct LinkedMethod {
    pub(crate) symbol: String,
    pub(crate) package: String,
    pub(crate) receiver: u32,
    pub(crate) record: u32,
    pub(crate) record_name: String,
    pub(crate) record_size: u64,
    pub(crate) method: String,
    pub(crate) argument: LinkedWord,
    pub(crate) result: LinkedWord,
    pub(crate) fields: Vec<(String, u64, LinkedWord)>,
}

#[derive(Clone, Debug)]
pub(crate) struct LinkedWord {
    pub(crate) offset: u32,
    pub(crate) kind: u8,
    pub(crate) name: String,
}

fn identifier(name: &str) -> bool {
    let mut chars = name.bytes();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == b'_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == b'_')
        && name != "_"
}

fn offset(meta: &TypeMeta, address: u64) -> Option<u32> {
    (address < meta.etypes).then_some(())?;
    u32::try_from(address.checked_sub(meta.types)?).ok()
}

fn in_types<'a>(img: &'a GoImage, meta: &TypeMeta, address: u64, len: usize) -> Option<&'a [u8]> {
    if address < meta.types || address.checked_add(len as u64)? > meta.etypes {
        return None;
    }
    img.read(address, len)
}

fn name_at(img: &GoImage, meta: &TypeMeta, at: u64) -> Option<String> {
    offset(meta, at)?;
    read_go_name(img, at)
}

fn positive_offset(bytes: &[u8]) -> Option<u32> {
    let val = i32::from_le_bytes(bytes.try_into().ok()?);
    u32::try_from(val).ok()
}

fn offset_name(img: &GoImage, meta: &TypeMeta, bytes: &[u8]) -> Option<String> {
    let relative = positive_offset(bytes)?;
    (relative != 0).then_some(())?;
    name_at(img, meta, meta.types.checked_add(relative.into())?)
}

type MethodRow = (String, Option<u32>, Option<u32>, Option<u32>);

fn methods(
    img: &GoImage,
    meta: &TypeMeta,
    ty: &GoType,
    base: u64,
) -> Option<(String, Vec<MethodRow>)> {
    if ty.tflag & UNCOMMON == 0 {
        return None;
    }
    let start = ty.vma.checked_add(base)?;
    let header = in_types(img, meta, start, 16)?;
    let package = offset_name(img, meta, &header[..4])?;
    let count = u16::from_le_bytes(header[4..6].try_into().ok()?) as usize;
    let exported = u16::from_le_bytes(header[6..8].try_into().ok()?) as usize;
    let first = u32::from_le_bytes(header[8..12].try_into().ok()?);
    if !identifier(&package) || count > 256 || exported > count || (count != 0 && first < 16) {
        return None;
    }
    if count == 0 {
        return Some((package, Vec::new()));
    }
    let mut names = BTreeSet::new();
    let rows = in_types(img, meta, start.checked_add(first.into())?, count * 16)?;
    let mut result = Vec::with_capacity(count);
    for row in rows.chunks_exact(16) {
        let name = offset_name(img, meta, &row[..4])?;
        if !names.insert(name.clone()) {
            return None;
        }
        result.push((
            name,
            positive_offset(&row[4..8]),
            positive_offset(&row[8..12]),
            positive_offset(&row[12..]),
        ));
    }
    Some((package, result))
}

fn scalar(img: &GoImage, meta: &TypeMeta, address: u64) -> Option<LinkedWord> {
    let relative = offset(meta, address)?;
    let ty = read_type(img, meta.types, meta.etypes, relative)?;
    (ty.size == 8
        && matches!(
            (ty.kind, ty.name.as_str()),
            (KIND_INT, "int")
                | (KIND_INT64, "int64")
                | (KIND_UINT, "uint")
                | (KIND_UINT64, "uint64")
        ))
    .then_some(LinkedWord {
        offset: relative,
        kind: ty.kind,
        name: ty.name,
    })
}

fn signature(img: &GoImage, meta: &TypeMeta, sig: u32) -> Option<(LinkedWord, LinkedWord)> {
    let ty = read_type(img, meta.types, meta.etypes, sig)?;
    if ty.kind != KIND_FUNC
        || ty.size != 8
        || ty.tflag & (UNCOMMON | TFLAG_NAMED) != 0
        || in_types(img, meta, ty.vma + 48, 4)? != [1, 0, 1, 0]
    {
        return None;
    }
    in_types(img, meta, ty.vma + 56, 16)?;
    Some((
        scalar(img, meta, word(img, ty.vma + 56)?)?,
        scalar(img, meta, word(img, ty.vma + 64)?)?,
    ))
}

fn fields(
    img: &GoImage,
    meta: &TypeMeta,
    record: &GoType,
) -> Option<Vec<(String, u64, LinkedWord)>> {
    in_types(img, meta, record.vma, 80)?;
    let start = word(img, record.vma + 56)?;
    let count = word(img, record.vma + 64)?;
    if !(1..=32).contains(&count) || count != word(img, record.vma + 72)? {
        return None;
    }
    in_types(img, meta, start, count as usize * 24)?;
    let mut fields = Vec::with_capacity(count as usize);
    let mut end = 0;
    let mut names = BTreeSet::new();
    for i in 0..count {
        let at = start + i * 24;
        let name = name_at(img, meta, word(img, at)?)?;
        let ty = scalar(img, meta, word(img, at + 8)?)?;
        let displacement = word(img, at + 16)?;
        if !identifier(&name) || !names.insert(name.clone()) || displacement < end {
            return None;
        }
        end = displacement.checked_add(8)?;
        if end > record.size {
            return None;
        }
        fields.push((name, displacement, ty));
    }
    Some(fields)
}

pub(super) fn collect<'a>(
    file: &'a object::File<'a>,
    img: &GoImage,
    meta: &TypeMeta,
    header: u64,
    module: u64,
    version: Option<(u16, u16)>,
) -> Option<BTreeMap<u64, LinkedMethod>> {
    if file.format() != object::BinaryFormat::Elf
        || file.architecture() != object::Architecture::X86_64
        || !matches!(version, Some((1, 22 | 27)))
        || img.word_size != 8
        || file.kind() != object::ObjectKind::Executable
        || file
            .section_by_name(".debug_info")
            .or_else(|| file.section_by_name(".zdebug_info"))
            .is_none()
    {
        return None;
    }
    let mut symbols = BTreeMap::new();
    let mut duplicates = BTreeSet::new();
    for symbol in file.symbols().filter(|symbol| symbol.is_definition()) {
        let Ok(name) = symbol.name() else { continue };
        if !matches!(
            name,
            "runtime.firstmoduledata" | "runtime.types" | "runtime.etypes" | "runtime.text"
        ) && !name.contains(".(*")
        {
            continue;
        }
        if symbols.insert(name, symbol.address()).is_some() {
            duplicates.insert(name);
        }
    }
    for (name, address) in [
        ("runtime.firstmoduledata", module),
        ("runtime.types", meta.types),
        ("runtime.etypes", meta.etypes),
    ] {
        if duplicates.contains(name) || symbols.get(name) != Some(&address) {
            return None;
        }
    }
    let text = *symbols.get("runtime.text")?;
    if duplicates.contains("runtime.text") || text != word(img, module + 22 * 8)? {
        return None;
    }
    let code = file.section_by_name(".text")?;
    let code_end = code.address().checked_add(code.size())?;
    let linked = MethodEntries::new(file, header, text)?;

    let mut by_name = BTreeMap::<String, BTreeSet<u64>>::new();
    let mut pointers = BTreeMap::new();
    for &relative in &meta.offsets {
        let Some(ty) = read_type(img, meta.types, meta.etypes, relative) else {
            continue;
        };
        if ty.kind == KIND_STRUCT && ty.tflag & TFLAG_NAMED != 0 {
            by_name.entry(ty.name).or_default().insert(ty.vma);
        } else if ty.kind == KIND_POINTER && ty.size == 8 && ty.tflag & UNCOMMON != 0 {
            let Some(elem) = word(img, ty.vma + 48).and_then(|at| offset(meta, at)) else {
                continue;
            };
            let Some(record) = read_type(img, meta.types, meta.etypes, elem) else {
                continue;
            };
            if record.kind == KIND_STRUCT && record.tflag & TFLAG_NAMED != 0 {
                by_name
                    .entry(record.name.clone())
                    .or_default()
                    .insert(record.vma);
                by_name.entry(ty.name.clone()).or_default().insert(ty.vma);
                pointers.insert(ty.vma, (relative, ty, elem, record));
            }
        }
    }
    let mut statements = BTreeMap::new();
    let mut ambiguous = BTreeSet::new();
    for (relative, pointer, element, record) in pointers.into_values() {
        if by_name[&pointer.name].len() != 1
            || by_name[&record.name].len() != 1
            || pointer.name.strip_prefix('*') != Some(record.name.as_str())
        {
            continue;
        }
        let Some((package, pointer_methods)) = methods(img, meta, &pointer, 56) else {
            continue;
        };
        let Some((value_package, value_methods)) = methods(img, meta, &record, 80) else {
            continue;
        };
        let Some((prefix, receiver)) = record.name.rsplit_once('.') else {
            continue;
        };
        if package != value_package || package != prefix || !identifier(receiver) {
            continue;
        }
        let Some(fields) = fields(img, meta, &record) else {
            continue;
        };
        for (method, sig, ifn, tfn) in pointer_methods {
            let (Some(sig), Some(ifn), Some(tfn)) = (sig, ifn, tfn) else {
                continue;
            };
            if ifn != tfn || !identifier(&method) || value_methods.iter().any(|row| row.0 == method)
            {
                continue;
            }
            let Some(entry) = text.checked_add(ifn.into()) else {
                continue;
            };
            let symbol = format!("{package}.(*{receiver}).{method}");
            if !(code.address()..code_end).contains(&entry)
                || duplicates.contains(symbol.as_str())
                || symbols.get(symbol.as_str()) != Some(&entry)
                || linked.ordinary_name(entry).as_deref() != Some(symbol.as_str())
            {
                continue;
            }
            let Some((argument, result)) = signature(img, meta, sig) else {
                continue;
            };
            let statement = LinkedMethod {
                symbol,
                package: package.clone(),
                receiver: relative,
                record: element,
                record_name: record.name.clone(),
                record_size: record.size,
                method,
                argument,
                result,
                fields: fields.clone(),
            };
            if statements.insert(entry, statement).is_some() {
                ambiguous.insert(entry);
            }
        }
    }
    statements.retain(|entry, _| !ambiguous.contains(entry));
    Some(statements)
}
