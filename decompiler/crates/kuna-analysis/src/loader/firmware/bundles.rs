//! Inputs that hold several images: tar archives (plain or gzip-compressed),
//! zip archives and U-Boot FIT images (a flattened device tree whose
//! `/images` nodes each carry one image).
//!
//! [`Bundle::parse`] lists the members without decoding them;
//! [`Bundle::unpack`] decodes one. Stored members borrow the input, deflate
//! and gzip members are inflated through `flate2`. Every offset and size is
//! taken from the input and checked against it; a decoded member is capped at
//! [`MAX_UNPACKED`] bytes and a listing at [`MAX_MEMBERS`] entries.
//!
//! Layouts: POSIX.1-2001 `ustar` and `pax` headers plus the GNU long-name
//! record; PKWARE APPNOTE 6.3 central directory with the zip64 records;
//! Devicetree Specification 0.4 for the flattened tree and the Flattened
//! Image Tree specification 0.8 for the `/images` properties.

use std::borrow::Cow;
use std::io::Read;
use std::ops::Range;

use super::containers::Flat;

/// Largest decoded member.
pub const MAX_UNPACKED: u64 = 1 << 30;
/// Most members listed.
pub const MAX_MEMBERS: usize = 1 << 20;
const MAX_DEPTH: usize = 32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Tar,
    Zip,
    Fit,
}

impl Kind {
    pub fn name(self) -> &'static str {
        match self {
            Kind::Tar => "tar archive",
            Kind::Zip => "zip archive",
            Kind::Fit => "FIT image",
        }
    }
}

/// How a member's bytes are stored.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Packed {
    Stored(Range<usize>),
    Deflate { data: Range<usize>, crc: u32 },
    Gzip(Range<usize>),
    Undecoded(String),
}

/// What a FIT image node states about its image.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FitImage {
    pub kind: String,
    pub arch: String,
    pub os: String,
    pub compression: String,
    pub load: Option<u64>,
    pub entry: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub name: String,
    /// Decoded size as the container states it.
    pub size: u64,
    pub fit: Option<FitImage>,
    packed: Packed,
}

pub struct Bundle<'a> {
    pub kind: Kind,
    pub entries: Vec<Entry>,
    /// The member a FIT's default configuration boots.
    pub default: Option<String>,
    data: Cow<'a, [u8]>,
}

/// Whether the first bytes of a file can start one of the formats.
pub fn sniff(head: &[u8]) -> bool {
    head.starts_with(b"PK\x03\x04")
        || head.starts_with(b"PK\x05\x06")
        || head.starts_with(&FDT_MAGIC)
        || head.starts_with(&GZIP_MAGIC)
        || tar_header(head)
}

impl<'a> Bundle<'a> {
    /// The members of `bytes`, or `None` when it is none of the formats: a
    /// device tree without `/images` and a gzip stream that is no tar are not.
    pub fn parse(bytes: &'a [u8]) -> Result<Option<Self>, String> {
        if bytes.starts_with(b"PK\x03\x04") || bytes.starts_with(b"PK\x05\x06") {
            let entries = zip(bytes)?;
            return Ok(Some(Bundle { kind: Kind::Zip, entries, default: None, data: Cow::Borrowed(bytes) }));
        }
        if bytes.starts_with(&FDT_MAGIC) {
            return Ok(fit(bytes)?.map(|(entries, default)| Bundle {
                kind: Kind::Fit,
                entries,
                default,
                data: Cow::Borrowed(bytes),
            }));
        }
        if tar_header(bytes) {
            let entries = tar(bytes)?;
            return Ok(Some(Bundle { kind: Kind::Tar, entries, default: None, data: Cow::Borrowed(bytes) }));
        }
        if bytes.starts_with(&GZIP_MAGIC) {
            let mut head = [0u8; 512];
            if flate2::read::GzDecoder::new(bytes).read_exact(&mut head).is_err() || !tar_header(&head) {
                return Ok(None);
            }
            let data = inflate(flate2::read::GzDecoder::new(bytes), "gzip-compressed tar")?;
            let entries = tar(&data)?;
            return Ok(Some(Bundle { kind: Kind::Tar, entries, default: None, data: Cow::Owned(data) }));
        }
        Ok(None)
    }

    /// The decoded bytes of `entry`.
    pub fn unpack(&self, entry: &Entry) -> Result<Cow<'_, [u8]>, String> {
        let slice = |range: &Range<usize>| {
            self.data.get(range.clone()).ok_or_else(|| format!("{}: data runs past the file", entry.name))
        };
        match &entry.packed {
            Packed::Stored(range) => slice(range).map(Cow::Borrowed),
            Packed::Gzip(range) => inflate(flate2::read::GzDecoder::new(slice(range)?), &entry.name).map(Cow::Owned),
            Packed::Deflate { data, crc } => {
                let out = inflate(flate2::read::DeflateDecoder::new(slice(data)?), &entry.name)?;
                let mut sum = flate2::Crc::new();
                sum.update(&out);
                if out.len() as u64 != entry.size || sum.sum() != *crc {
                    return Err(format!("{}: inflated data does not match its size and CRC-32", entry.name));
                }
                Ok(Cow::Owned(out))
            }
            Packed::Undecoded(why) => Err(format!("{}: {why}", entry.name)),
        }
    }

    /// A FIT image that states a load address, laid out as a flat image with
    /// the processor, base and entry its node states; `None` for any other
    /// member.
    pub fn flat(&self, entry: &Entry) -> Result<Option<Flat>, String> {
        let Some((fit, load)) = entry.fit.as_ref().and_then(|f| Some((f, f.load?))) else {
            return Ok(None);
        };
        let bytes = self.unpack(entry)?.into_owned();
        let arm = fit.arch == "arm";
        let start = fit.entry.unwrap_or(load);
        Ok(Some(Flat {
            format: "U-Boot FIT image",
            target: fit_target(&fit.arch, &bytes),
            base: load,
            entries: vec![start & !u64::from(arm)],
            thumb: arm && start & 1 == 1,
            bytes,
            ..Flat::default()
        }))
    }
}

fn inflate(reader: impl Read, what: &str) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    let mut bounded = reader.take(MAX_UNPACKED + 1);
    bounded.read_to_end(&mut out).map_err(|e| format!("{what}: {e}"))?;
    if out.len() as u64 > MAX_UNPACKED {
        return Err(format!("{what}: inflates past {MAX_UNPACKED:#x} bytes"));
    }
    Ok(out)
}

fn count(entries: &[Entry]) -> Result<(), String> {
    if entries.len() >= MAX_MEMBERS {
        return Err(format!("more than {MAX_MEMBERS} members"));
    }
    Ok(())
}

fn text(field: &[u8]) -> String {
    let end = field.iter().position(|&c| c == 0).unwrap_or(field.len());
    String::from_utf8_lossy(&field[..end]).into_owned()
}

const GZIP_MAGIC: [u8; 3] = [0x1f, 0x8b, 8];

/// A tar number field: octal digits, or big-endian binary when the top bit
/// of the first byte is set.
fn tar_number(field: &[u8]) -> Option<u64> {
    if field.first()? & 0x80 != 0 {
        let (high, low) = field.split_at(field.len().checked_sub(8)?);
        let clear = high.iter().enumerate().all(|(i, &b)| if i == 0 { b == 0x80 } else { b == 0 });
        return clear.then(|| u64::from_be_bytes(low.try_into().unwrap_or_default()));
    }
    let digits: Vec<u8> = field.iter().copied().filter(|c| !matches!(c, b' ' | 0)).collect();
    if digits.len() > 21 || digits.iter().any(|c| !(b'0'..=b'7').contains(c)) {
        return None;
    }
    digits.iter().try_fold(0u64, |n, c| n.checked_mul(8)?.checked_add(u64::from(c - b'0')))
}

/// Whether `block` starts with a tar header whose checksum holds.
fn tar_header(block: &[u8]) -> bool {
    let Some(header) = block.get(..512) else { return false };
    let sum: u64 = header.iter().enumerate().map(|(i, &b)| if (148..156).contains(&i) { 32 } else { u64::from(b) }).sum();
    header[0] != 0 && tar_number(&header[148..156]) == Some(sum)
}

/// The `path` record of a pax extended header.
fn pax_path(body: &[u8]) -> Option<String> {
    let mut rest = body;
    while !rest.is_empty() {
        let space = rest.iter().position(|&c| c == b' ')?;
        let len: usize = std::str::from_utf8(&rest[..space]).ok()?.parse().ok()?;
        let record = rest.get(space + 1..len)?;
        if let Some(path) = record.strip_prefix(b"path=") {
            return Some(String::from_utf8_lossy(path.strip_suffix(b"\n").unwrap_or(path)).into_owned());
        }
        rest = &rest[len..];
    }
    None
}

/// The regular files of a tar stream, up to its first all-zero block.
fn tar(b: &[u8]) -> Result<Vec<Entry>, String> {
    let mut entries = Vec::new();
    let mut at = 0usize;
    let mut long: Option<String> = None;
    while let Some(header) = b.get(at..).and_then(|rest| rest.get(..512)) {
        if header.iter().all(|&c| c == 0) {
            break;
        }
        if !tar_header(header) {
            return Err(format!("tar header at {at:#x} fails its checksum"));
        }
        let body = at + 512;
        let end = tar_number(&header[124..136])
            .and_then(|size| body.checked_add(usize::try_from(size).ok()?))
            .filter(|&end| end <= b.len())
            .ok_or_else(|| format!("tar member at {at:#x} runs past the file"))?;
        match header[156] {
            b'L' => long = Some(text(&b[body..end])),
            b'x' => long = pax_path(&b[body..end]).or(long),
            b'g' => {}
            0 | b'0' | b'7' => {
                let mut name = text(&header[..100]);
                if header[257..262] == *b"ustar" && header[345] != 0 {
                    name = format!("{}/{name}", text(&header[345..500]));
                }
                let name = long.take().unwrap_or(name);
                if !name.ends_with('/') {
                    count(&entries)?;
                    entries.push(Entry { name, size: (end - body) as u64, fit: None, packed: Packed::Stored(body..end) });
                }
            }
            _ => long = None,
        }
        at = match end.checked_next_multiple_of(512) {
            Some(next) => next,
            None => break,
        };
    }
    Ok(entries)
}

fn le16(b: &[u8], at: usize) -> Option<u64> {
    Some(u64::from(u16::from_le_bytes(b.get(at..at.checked_add(2)?)?.try_into().ok()?)))
}
fn le32(b: &[u8], at: usize) -> Option<u64> {
    Some(u64::from(u32::from_le_bytes(b.get(at..at.checked_add(4)?)?.try_into().ok()?)))
}
fn le64(b: &[u8], at: usize) -> Option<u64> {
    Some(u64::from_le_bytes(b.get(at..at.checked_add(8)?)?.try_into().ok()?))
}

/// `(entry count, central directory offset)` from the end-of-central-directory
/// record, or from the zip64 record its locator names.
fn zip_directory(b: &[u8]) -> Option<(u64, u64)> {
    let floor = b.len().saturating_sub(22 + 0xffff);
    let eocd = (floor..=b.len().checked_sub(22)?).rev().find(|&at| b[at..].starts_with(b"PK\x05\x06"))?;
    let (total, offset) = (le16(b, eocd + 10)?, le32(b, eocd + 16)?);
    if total != 0xffff && offset != 0xffff_ffff {
        return Some((total, offset));
    }
    let locator = eocd.checked_sub(20).filter(|&at| b[at..].starts_with(b"PK\x06\x07"))?;
    let record = usize::try_from(le64(b, locator + 8)?).ok()?;
    b.get(record..)?.starts_with(b"PK\x06\x06").then_some(())?;
    Some((le64(b, record.checked_add(32)?)?, le64(b, record.checked_add(48)?)?))
}

/// The zip64 extended-information field of a central directory entry fills
/// in, in order, each of `fields` the fixed header saturates.
fn zip64_extra(mut extra: &[u8], fields: [&mut u64; 3]) {
    let body = loop {
        let (Some(id), Some(len)) = (le16(extra, 0), le16(extra, 2)) else { return };
        let Some(body) = extra.get(4..4 + len as usize) else { return };
        if id == 1 {
            break body;
        }
        extra = &extra[4 + len as usize..];
    };
    let mut at = 0;
    for field in fields.into_iter().filter(|field| **field == 0xffff_ffff) {
        let Some(wide) = le64(body, at) else { return };
        *field = wide;
        at += 8;
    }
}

/// Where a member's data lies, from its local file header.
fn zip_data(b: &[u8], local: u64, packed: u64) -> Option<Range<usize>> {
    let local = usize::try_from(local).ok()?;
    b.get(local..)?.starts_with(b"PK\x03\x04").then_some(())?;
    let skip = 30 + le16(b, local.checked_add(26)?)? + le16(b, local.checked_add(28)?)?;
    let start = local.checked_add(skip as usize)?;
    let end = start.checked_add(usize::try_from(packed).ok()?)?;
    (end <= b.len()).then_some(start..end)
}

/// The files of a zip archive's central directory; directories are skipped.
fn zip(b: &[u8]) -> Result<Vec<Entry>, String> {
    let (total, mut at) = zip_directory(b)
        .and_then(|(total, offset)| Some((total, usize::try_from(offset).ok()?)))
        .ok_or("zip archive has no central directory")?;
    let mut entries = Vec::new();
    for _ in 0..total {
        let short = || "zip central directory runs past the file".to_string();
        if !b.get(at..).is_some_and(|rest| rest.starts_with(b"PK\x01\x02")) {
            return Err(short());
        }
        let field16 = |off| le16(b, at + off).ok_or_else(short);
        let field32 = |off| le32(b, at + off).ok_or_else(short);
        let (flags, method, crc) = (field16(8)?, field16(10)?, field32(16)? as u32);
        let (mut packed, mut size, mut local) = (field32(20)?, field32(24)?, field32(42)?);
        let (name_len, extra_len, comment_len) = (field16(28)? as usize, field16(30)? as usize, field16(32)? as usize);
        let name = b.get(at + 46..at + 46 + name_len).ok_or_else(short)?;
        let extra = b.get(at + 46 + name_len..at + 46 + name_len + extra_len).ok_or_else(short)?;
        zip64_extra(extra, [&mut size, &mut packed, &mut local]);
        at += 46 + name_len + extra_len + comment_len;
        if name.ends_with(b"/") {
            continue;
        }
        let packed = match (zip_data(b, local, packed), method) {
            _ if flags & 1 != 0 => Packed::Undecoded("the member is encrypted".into()),
            (None, _) => Packed::Undecoded("the member's data runs past the file".into()),
            (Some(data), 0) => Packed::Stored(data),
            (Some(data), 8) => Packed::Deflate { data, crc },
            (Some(_), other) => Packed::Undecoded(format!("zip compression method {other} is not decoded")),
        };
        count(&entries)?;
        entries.push(Entry { name: String::from_utf8_lossy(name).into_owned(), size, fit: None, packed });
    }
    Ok(entries)
}

const FDT_MAGIC: [u8; 4] = [0xd0, 0x0d, 0xfe, 0xed];

fn be32(b: &[u8], at: usize) -> Option<usize> {
    Some(u32::from_be_bytes(b.get(at..at.checked_add(4)?)?.try_into().ok()?) as usize)
}

/// A one- or two-cell big-endian address.
fn cells(value: &[u8]) -> Option<u64> {
    match value.len() {
        4 => Some(u64::from(u32::from_be_bytes(value.try_into().ok()?))),
        8 => Some(u64::from_be_bytes(value.try_into().ok()?)),
        _ => None,
    }
}

/// One `/images` node while its properties are read.
#[derive(Default)]
struct FitNode {
    name: String,
    image: FitImage,
    data: Option<Range<usize>>,
    offset: Option<u64>,
    position: Option<u64>,
    size: Option<u64>,
}

impl FitNode {
    fn property(&mut self, name: &str, value: &[u8], start: usize) {
        match name {
            "data" => self.data = Some(start..start + value.len()),
            "data-offset" => self.offset = cells(value),
            "data-position" => self.position = cells(value),
            "data-size" => self.size = cells(value),
            "type" => self.image.kind = text(value),
            "arch" => self.image.arch = text(value),
            "os" => self.image.os = text(value),
            "compression" => self.image.compression = text(value),
            "load" => self.image.load = cells(value),
            "entry" => self.image.entry = cells(value),
            _ => {}
        }
    }

    fn entry(self, b: &[u8], external: usize) -> Entry {
        let outside = match (self.position, self.offset, self.size) {
            (Some(position), _, Some(size)) => Some((Some(position), size)),
            (None, Some(offset), Some(size)) => Some(((external as u64).checked_add(offset), size)),
            _ => None,
        };
        let range = self.data.or_else(|| {
            let (start, size) = outside?;
            let start = usize::try_from(start?).ok()?;
            let end = start.checked_add(usize::try_from(size).ok()?)?;
            (end <= b.len()).then_some(start..end)
        });
        let (size, packed) = match (range, self.image.compression.as_str()) {
            (None, _) => (0, Packed::Undecoded("the image has no data in the file".into())),
            (Some(range), "" | "none") => (range.len() as u64, Packed::Stored(range)),
            (Some(range), "gzip") => {
                let stated = range.end.checked_sub(4).filter(|&at| at >= range.start).and_then(|at| le32(b, at));
                (stated.unwrap_or(0), Packed::Gzip(range))
            }
            (Some(range), other) => {
                (range.len() as u64, Packed::Undecoded(format!("FIT compression {other} is not decoded")))
            }
        };
        Entry { name: self.name, size, fit: Some(self.image), packed }
    }
}

/// What a walk of a FIT's device tree collects.
#[derive(Default)]
struct FitTree {
    path: Vec<String>,
    has_images: bool,
    nodes: Vec<FitNode>,
    default: Option<String>,
    /// `(configuration, property, image)` for the `kernel` and `firmware` properties.
    boots: Vec<(String, String, String)>,
}

impl FitTree {
    fn open(&mut self, name: String) -> Result<(), String> {
        if self.path.len() >= MAX_DEPTH {
            return Err(format!("device tree nests deeper than {MAX_DEPTH} nodes"));
        }
        self.path.push(name);
        let [_, images, rest @ ..] = self.path.as_slice() else { return Ok(()) };
        if images != "images" {
            return Ok(());
        }
        self.has_images = true;
        if let [image] = rest {
            if self.nodes.len() >= MAX_MEMBERS {
                return Err(format!("more than {MAX_MEMBERS} members"));
            }
            self.nodes.push(FitNode { name: image.clone(), ..FitNode::default() });
        }
        Ok(())
    }

    fn property(&mut self, name: &str, value: &[u8], start: usize) {
        match self.path.as_slice() {
            [_, images, _] if images == "images" => {
                if let Some(node) = self.nodes.last_mut() {
                    node.property(name, value, start);
                }
            }
            [_, configs] if configs == "configurations" && name == "default" => self.default = Some(text(value)),
            [_, configs, config] if configs == "configurations" && matches!(name, "kernel" | "firmware") => {
                self.boots.push((config.clone(), name.to_string(), text(value)));
            }
            _ => {}
        }
    }

    /// The image the default configuration boots: its kernel, else its firmware.
    fn booted(&self) -> Option<String> {
        let default = self.default.as_ref()?;
        let of = |property: &str| self.boots.iter().find(|b| b.0 == *default && b.1 == property).map(|b| b.2.clone());
        of("kernel").or_else(|| of("firmware")).filter(|name| self.nodes.iter().any(|n| n.name == *name))
    }
}

/// The `/images` nodes of a flattened device tree and the image its default
/// configuration boots; `None` when the tree has no `/images` node.
fn fit(b: &[u8]) -> Result<Option<(Vec<Entry>, Option<String>)>, String> {
    let short = || "device tree structure runs past the file".to_string();
    let header = |at| be32(b, at).ok_or_else(short);
    let padded = |at: usize, len: usize| at.checked_add(len).and_then(|n| n.checked_next_multiple_of(4)).ok_or_else(short);
    let (total, structs, strings) = (header(4)?, header(8)?, header(12)?);
    let end = structs.checked_add(header(36)?).filter(|&end| end <= b.len()).ok_or_else(short)?;
    let mut tree = FitTree::default();
    let mut at = structs;
    while at < end {
        let token = header(at)?;
        at += 4;
        match token {
            1 => {
                let name = text(b.get(at..end).ok_or_else(short)?);
                at = padded(at, name.len() + 1)?;
                tree.open(name)?;
            }
            2 => drop(tree.path.pop()),
            3 => {
                let (len, name_off, start) = (header(at)?, header(at + 4)?, at + 8);
                let name = strings.checked_add(name_off).and_then(|n| b.get(n..)).map(text).ok_or_else(short)?;
                let value = b.get(start..).and_then(|rest| rest.get(..len)).ok_or_else(short)?;
                at = padded(start, len)?;
                tree.property(&name, value, start);
            }
            4 => {}
            9 => break,
            other => return Err(format!("device tree token {other:#x} at {:#x}", at - 4)),
        }
    }
    if !tree.has_images {
        return Ok(None);
    }
    let booted = tree.booted();
    let external = total.checked_next_multiple_of(4).unwrap_or(total);
    Ok(Some((tree.nodes.into_iter().map(|node| node.entry(b, external)).collect(), booted)))
}

/// SLEIGH language for a FIT `arch` value.
fn fit_target(arch: &str, code: &[u8]) -> Option<String> {
    Some(match arch {
        "arm" => "ARM:LE:32:v7".into(),
        "arm64" => "AARCH64:LE:64:v8A".into(),
        "x86" | "i386" => "x86:LE:32:default".into(),
        "x86_64" => "x86:LE:64:default".into(),
        "mips" => format!("MIPS:{}:32:default", super::containers::mips_endian(code)),
        "mips64" => format!("MIPS:{}:64:default", super::containers::mips_endian(code)),
        "powerpc" | "ppc" => "PowerPC:BE:32:default".into(),
        "sh" => "SuperH4:LE:32:default".into(),
        "sparc" => "sparc:BE:32:default".into(),
        "sparc64" => "sparc:BE:64:default".into(),
        "m68k" => "68000:BE:32:default".into(),
        "xtensa" => "Xtensa:LE:32:default".into(),
        "riscv" => "RISCV:LE:64:default".into(),
        _ => return None,
    })
}

#[cfg(test)]
#[path = "bundles_tests.rs"]
mod tests;
