//! (kuna `flirt make`) Write a FLIRT `.sig` file from patterns.
//!
//! The file is format version 10, uncompressed: a header, then a prefix tree
//! over the leading [`PATTERN_LEN`] bytes of every function. A leaf lists its
//! modules by the CRC-16 of the bytes after the pattern.
//!
//! Functions with the same leading bytes and the same CRC collide: the tree
//! alone cannot tell them apart. A collision is resolved the way a signature
//! librarian resolves one, in this order:
//!
//! 1. a tail byte: an offset after the CRC region where every function of the
//!    group has a fixed byte and they are not all equal splits the group by
//!    that byte, and each part is resolved again;
//! 2. a referenced name: an offset where every function refers to a global
//!    name, none of them a data object of the library, splits the group by that
//!    name. A module holds one reference, so only a part of one function is
//!    resolved by it.
//!
//! Functions that still collide go into no module: they are returned as
//! [`Collision`]s for the caller to report. Functions with the same bytes and
//! the same name are one module.

use std::collections::{BTreeMap, BTreeSet};

use object::{Architecture, BinaryFormat};

use super::kuna_flirtpat::{FuncPattern, Target, PATTERN_LEN};

/// The format version written.
pub const SIG_VERSION: u8 = 10;

/// Functions the `.sig` leaves out because nothing tells them apart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collision {
    pub names: Vec<String>,
}

/// A written signature file.
pub struct BuiltSig {
    pub bytes: Vec<u8>,
    pub modules: usize,
    pub collisions: Vec<Collision>,
}

/// A function with what resolved its collision.
struct Resolved<'a> {
    pattern: &'a FuncPattern,
    tails: Vec<(u32, u8)>,
    reference: Option<(u32, &'a str)>,
}

type Head = [Option<u8>; PATTERN_LEN];
type Leaf<'a> = (Head, Vec<Resolved<'a>>);

/// Build the `.sig` bytes for `patterns`. `data_names` are the names the
/// library defines as data, which a reference check can never confirm.
pub fn build_sig(
    patterns: &[FuncPattern],
    data_names: &BTreeSet<String>,
    target: Target,
    library: &str,
) -> Result<BuiltSig, String> {
    let processor = processor_id(target.architecture)
        .ok_or_else(|| format!("no signature processor id for {:?} objects", target.architecture))?;
    let unique: BTreeSet<&FuncPattern> = patterns.iter().filter(|p| writable(p)).collect();
    let mut groups: BTreeMap<(Head, usize, u16), Vec<&FuncPattern>> = BTreeMap::new();
    for p in unique {
        groups.entry((p.head(), p.crc_bytes().len(), p.crc())).or_default().push(p);
    }
    let mut leaves: BTreeMap<Head, Vec<Resolved<'_>>> = BTreeMap::new();
    let mut collisions = Vec::new();
    for ((head, ..), group) in groups {
        resolve(group, Vec::new(), data_names, leaves.entry(head).or_default(), &mut collisions);
    }
    leaves.retain(|_, leaf| !leaf.is_empty());
    let modules: usize = leaves.values().map(Vec::len).sum();
    if modules == 0 {
        return Err("no function is left to write".into());
    }

    let library: Vec<u8> = library
        .chars()
        .map(|c| if c.is_ascii() && !c.is_ascii_control() { c as u8 } else { b'?' })
        .take(usize::from(u8::MAX))
        .collect();
    let mut out = b"IDASGN".to_vec();
    out.extend([SIG_VERSION, processor]);
    out.extend(file_formats(target.format).to_le_bytes());
    out.extend(u16::MAX.to_le_bytes());
    out.extend(u16::MAX.to_le_bytes());
    out.extend(0u16.to_le_bytes());
    out.extend(u16::try_from(modules).unwrap_or(u16::MAX).to_le_bytes());
    out.extend([0; 2 + 12]);
    out.push(library.len() as u8);
    out.extend([0; 2]);
    out.extend(u32::try_from(modules).map_err(|_| "too many modules")?.to_le_bytes());
    out.extend((PATTERN_LEN as u16).to_le_bytes());
    out.extend([0; 2]);
    out.extend(library);
    let leaves: Vec<Leaf<'_>> = leaves.into_iter().collect();
    write_node(&mut out, &leaves, 0);
    Ok(BuiltSig { bytes: out, modules, collisions })
}

/// Whether the version 10 encoding can hold the pattern's names: a name ends
/// at the first byte below 0x20.
fn writable(p: &FuncPattern) -> bool {
    !p.publics.is_empty() && p.publics.iter().all(|n| !n.is_empty() && n.bytes().all(|b| b >= 0x20))
}

fn resolve<'a>(
    mut group: Vec<&'a FuncPattern>,
    tails: Vec<(u32, u8)>,
    data_names: &BTreeSet<String>,
    out: &mut Vec<Resolved<'a>>,
    collisions: &mut Vec<Collision>,
) {
    if group.iter().all(|p| p.bytes == group[0].bytes && p.publics[0] == group[0].publics[0]) {
        group.truncate(1);
    }
    if let [pattern] = group[..] {
        out.push(Resolved { pattern, tails, reference: None });
        return;
    }
    if let (Some(i), true) = (differing_tail(&group), tails.len() < usize::from(u8::MAX)) {
        let mut parts: BTreeMap<Option<u8>, Vec<&FuncPattern>> = BTreeMap::new();
        for p in group {
            parts.entry(p.tail()[i]).or_default().push(p);
        }
        for (value, part) in parts {
            let mut tails = tails.clone();
            tails.extend(value.map(|v| (i as u32, v)));
            resolve(part, tails, data_names, out, collisions);
        }
        return;
    }
    let usable = |name: &str| !data_names.contains(name) && name.len() < 0x8000;
    let at = |p: &'a FuncPattern, offset: u32| {
        p.refs.iter().find(|(o, n)| *o == offset && usable(n)).map(|(_, n)| n.as_str())
    };
    let by_reference = group[0].refs.iter().map(|(o, _)| *o).find(|&o| {
        let names: Vec<Option<&str>> = group.iter().map(|p| at(p, o)).collect();
        names.iter().all(Option::is_some) && names.iter().any(|n| *n != names[0])
    });
    let mut parts: BTreeMap<Option<&str>, Vec<&'a FuncPattern>> = BTreeMap::new();
    for p in group {
        parts.entry(by_reference.and_then(|o| at(p, o))).or_default().push(p);
    }
    for (name, part) in parts {
        if let ([pattern], Some(name), Some(offset)) = (&part[..], name, by_reference) {
            out.push(Resolved { pattern, tails: tails.clone(), reference: Some((offset, name)) });
            continue;
        }
        let mut names: Vec<String> = part.iter().map(|p| p.publics[0].clone()).collect();
        names.sort();
        collisions.push(Collision { names });
    }
}

/// The first offset after the CRC region where every function of the group
/// has a fixed byte and two of them differ.
fn differing_tail(group: &[&FuncPattern]) -> Option<usize> {
    let shortest = group.iter().map(|p| p.tail().len()).min()?;
    (0..shortest).find(|&i| {
        let first = group[0].tail()[i];
        group.iter().all(|p| p.tail()[i].is_some()) && group.iter().any(|p| p.tail()[i] != first)
    })
}

/// Write the tree node whose leaves all share their first `depth` bytes: its
/// child count, then per child the bytes all its leaves have in common and
/// the child node. A node at full depth is a leaf.
fn write_node(out: &mut Vec<u8>, leaves: &[Leaf<'_>], depth: usize) {
    if depth == PATTERN_LEN {
        out.push(0);
        write_leaf(out, &leaves[0].1);
        return;
    }
    let mut children: Vec<&[Leaf<'_>]> = Vec::new();
    let mut rest = leaves;
    while let Some((first, _)) = rest.first() {
        let n = rest.iter().take_while(|(h, _)| h[depth] == first[depth]).count();
        children.push(&rest[..n]);
        rest = &rest[n..];
    }
    var15(out, children.len() as u16);
    for child in children {
        let (first, last) = (&child[0].0, &child[child.len() - 1].0);
        let len = (depth..PATTERN_LEN).take_while(|&i| first[i] == last[i]).count();
        let run = &first[depth..depth + len];
        var15(out, len as u16);
        let mask = run.iter().fold(0u32, |m, b| m << 1 | u32::from(b.is_none()));
        if len < 0x10 {
            var15(out, mask as u16);
        } else {
            var32(out, mask);
        }
        out.extend(run.iter().flatten());
        write_node(out, child, depth + len);
    }
}

/// Write a leaf: its modules grouped by CRC length and CRC. The byte after a
/// module's last name says what follows it.
fn write_leaf(out: &mut Vec<u8>, modules: &[Resolved<'_>]) {
    let mut by_crc: BTreeMap<(u8, u16), Vec<&Resolved<'_>>> = BTreeMap::new();
    for m in modules {
        by_crc.entry((m.pattern.crc_bytes().len() as u8, m.pattern.crc())).or_default().push(m);
    }
    let groups = by_crc.len();
    for (g, ((crc_len, crc), modules)) in by_crc.into_iter().enumerate() {
        out.push(crc_len);
        out.extend(crc.to_be_bytes());
        for (i, m) in modules.iter().enumerate() {
            let last = i + 1 == modules.len();
            write_module(out, m, u8::from(!last) << 3 | u8::from(last && g + 1 < groups) << 4);
        }
    }
}

/// Write a module: its length, its public names, and the tail bytes and
/// reference that resolved its collision. `after` says whether another module
/// or another CRC group follows.
fn write_module(out: &mut Vec<u8>, m: &Resolved<'_>, after: u8) {
    var32(out, m.pattern.bytes.len() as u32);
    let publics = &m.pattern.publics;
    for (n, name) in publics.iter().enumerate() {
        var32(out, 0);
        out.extend(name.bytes());
        out.push(if n + 1 < publics.len() {
            1
        } else {
            after | u8::from(!m.tails.is_empty()) << 1 | u8::from(m.reference.is_some()) << 2
        });
    }
    if !m.tails.is_empty() {
        out.push(m.tails.len() as u8);
        for &(offset, value) in &m.tails {
            var32(out, offset);
            out.push(value);
        }
    }
    if let Some((offset, name)) = m.reference {
        out.push(1);
        var32(out, offset);
        match u8::try_from(name.len()) {
            Ok(len) if len > 0 => out.push(len),
            _ => {
                out.push(0);
                var15(out, name.len() as u16);
            }
        }
        out.extend(name.bytes());
    }
}

fn var15(out: &mut Vec<u8>, v: u16) {
    if v < 0x80 {
        out.push(v as u8);
    } else {
        out.extend([0x80 | (v >> 8) as u8, v as u8]);
    }
}

fn var32(out: &mut Vec<u8>, v: u32) {
    if v < 0x4000 {
        var15(out, v as u16);
    } else if v < 0x2000_0000 {
        out.extend((v | 0xc000_0000).to_be_bytes());
    } else {
        out.push(0xe0);
        out.extend(v.to_be_bytes());
    }
}

/// The processor id a signature header carries for an architecture.
fn processor_id(architecture: Architecture) -> Option<u8> {
    Some(match architecture {
        Architecture::I386 | Architecture::X86_64 | Architecture::X86_64_X32 => 0,
        Architecture::Mips | Architecture::Mips64 => 12,
        Architecture::Arm | Architecture::Aarch64 | Architecture::Aarch64_Ilp32 => 13,
        Architecture::PowerPc | Architecture::PowerPc64 => 15,
        Architecture::Sparc | Architecture::Sparc64 => 23,
        Architecture::Riscv32 | Architecture::Riscv64 => 72,
        _ => return None,
    })
}

/// The header's file-type mask: the executable formats objects of `format`
/// link into, as the bit indices [`super::load_sigs`] tests.
fn file_formats(format: BinaryFormat) -> u32 {
    match format {
        BinaryFormat::Elf => 1 << 18,
        BinaryFormat::Coff | BinaryFormat::Pe => 1 << 10 | 1 << 11,
        BinaryFormat::MachO => 1 << 25,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flirt_sig::{Endian, Matcher, Signature};

    const TARGET: Target =
        Target { architecture: Architecture::X86_64, format: BinaryFormat::Elf, little_endian: true };

    /// A 48-byte function: `seed` in its leading bytes, a relocated field at
    /// 4..8, and `last` as its final byte.
    fn func(name: &str, seed: u8, last: u8, reference: Option<&str>) -> FuncPattern {
        let mut bytes: Vec<Option<u8>> = (0..48u8).map(|i| Some(seed.wrapping_add(i))).collect();
        bytes[4..8].fill(None);
        bytes[40..44].fill(None);
        bytes[47] = Some(last);
        let refs = reference.map(|r| (40, r.to_string())).into_iter().collect();
        FuncPattern { bytes, publics: vec![name.to_string()], refs }
    }

    fn concrete(p: &FuncPattern) -> Vec<u8> {
        p.bytes.iter().map(|b| b.unwrap_or(0xee)).collect()
    }

    #[test]
    fn a_written_sig_reads_back_with_its_collisions_resolved() {
        let mut aliased = func("plain", 0x10, 0, None);
        aliased.publics.push("plain_alias".into());
        let patterns = vec![
            aliased.clone(),
            func("tail_a", 0x30, 0xa1, None),
            func("tail_b", 0x30, 0xb2, None),
            func("ref_a", 0x50, 0, Some("callee_a")),
            func("ref_b", 0x50, 0, Some("callee_b")),
            func("same_a", 0x70, 0, Some("table")),
            func("same_b", 0x70, 0, Some("other")),
            func("twice", 0x90, 0, None),
            func("twice", 0x90, 0, None),
        ];
        let data: BTreeSet<String> = ["table".to_string()].into();
        let built = build_sig(&patterns, &data, TARGET, "round trip").unwrap();
        assert_eq!(built.collisions, [Collision { names: vec!["same_a".into(), "same_b".into()] }]);
        assert_eq!(built.modules, 6);

        let sig = Signature::parse(&built.bytes).expect("the reader accepts the written file");
        assert_eq!((sig.header.version, sig.header.pattern_len), (SIG_VERSION, 32));
        assert_eq!((sig.header.library_name.as_str(), sig.header.module_count), ("round trip", Some(6)));
        assert!(crate::flirt::applies_to(&built.bytes, Some(18)) && !crate::flirt::applies_to(&built.bytes, Some(11)));
        let module = |name: &str| {
            sig.modules.iter().find(|m| m.publics[0].name == name.as_bytes()).unwrap_or_else(|| panic!("{name}"))
        };
        let names: Vec<&[u8]> = module("plain").publics.iter().map(|p| p.name.as_slice()).collect();
        assert_eq!(names, [&b"plain"[..], b"plain_alias"]);
        assert_eq!(*module("plain").pattern, aliased.head());
        assert_eq!((module("plain").length, module("plain").crc_len, module("plain").crc), (48, 8, aliased.crc()));
        for (name, value) in [("tail_a", 0xa1), ("tail_b", 0xb2)] {
            let tails: Vec<(u32, u8)> = module(name).tail_bytes.iter().map(|t| (t.offset, t.value)).collect();
            assert_eq!(tails, [(7, value)], "{name}");
            assert!(module(name).reference.is_none());
        }
        for (name, callee) in [("ref_a", "callee_a"), ("ref_b", "callee_b")] {
            let r = module(name).reference.as_ref().expect("a reference tells the pair apart");
            assert_eq!((r.offset, r.name.as_slice()), (40, callee.as_bytes()), "{name}");
        }
        assert_eq!(sig.modules.iter().filter(|m| m.publics[0].name == b"twice").count(), 1);

        let matcher = Matcher::new(&sig, Endian::Little);
        for p in &patterns[..5] {
            let found: Vec<&[u8]> = matcher.match_at(&concrete(p)).iter().map(|m| m.publics[0].name.as_slice()).collect();
            let mut want = vec![p.publics[0].as_bytes()];
            if p.publics[0].starts_with("ref_") {
                want = vec![&b"ref_a"[..], b"ref_b"];
            }
            assert_eq!(found, want, "{}", p.publics[0]);
        }
    }

    #[test]
    fn a_short_function_at_the_end_of_its_section_matches() {
        let short = |name: &str, len: u8| FuncPattern {
            bytes: (0..len).map(|i| Some(0xc0 + i)).collect(),
            publics: vec![name.to_string()],
            refs: Vec::new(),
        };
        let patterns = [short("fits", 14), short("longer", 20)];
        let built = build_sig(&patterns, &BTreeSet::new(), TARGET, "x").unwrap();
        let sig = Signature::parse(&built.bytes).unwrap();
        let matcher = Matcher::new(&sig, Endian::Little);
        let names = |code: &[u8]| -> Vec<Vec<u8>> {
            crate::flirt::modules_at(&matcher, code, 32).iter().map(|m| m.publics[0].name.clone()).collect()
        };
        let code = concrete(&patterns[0]);
        assert!(matcher.match_at(&code).is_empty(), "the trie alone needs 32 bytes");
        assert_eq!(names(&code), [b"fits".to_vec()]);
        assert_eq!(names(&concrete(&patterns[1])[..16]), [b"fits".to_vec()], "`longer` runs past the section");
        assert!(names(&[]).is_empty());
    }

    #[test]
    fn nothing_to_write_and_an_unknown_processor_are_errors() {
        let twins = [func("one", 1, 0, None), func("two", 1, 0, None)];
        assert!(build_sig(&twins, &BTreeSet::new(), TARGET, "x").is_err());
        let avr = Target { architecture: Architecture::Avr, ..TARGET };
        assert!(build_sig(&twins[..1], &BTreeSet::new(), avr, "x").is_err());
    }
}
