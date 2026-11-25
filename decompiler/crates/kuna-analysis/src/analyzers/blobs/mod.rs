//! (kuna `blobs`) Embedded images and archives: PNG, JPEG, GIF, BMP, ICO,
//! zip, gzip, xz and zstd files stored whole inside a mapped section.
//!
//! A candidate is a magic number; a blob is a candidate whose container
//! structure was walked to its end, which is what gives the exact length.
//! Nothing is decoded into pixels. The walks are:
//!
//! | kind | accepted when |
//! |---|---|
//! | `png` | every chunk from `IHDR` to `IEND` has a matching CRC-32 and an `IDAT` is present |
//! | `jpeg` | the marker segments reach `EOI` after a frame header and a scan |
//! | `gif` | the block stream reaches the trailer after at least one image |
//! | `bmp` | the header's file size equals the pixel array the DIB header describes |
//! | `ico` | the directory's images tile the file and each is a PNG or a DIB |
//! | `zip` | the end-of-central-directory record's directory names every local header |
//! | `gzip` | the deflate stream inflates to the trailer's CRC-32 and length |
//! | `xz` | the stream footer and index CRC-32s match and the index sums to the blocks |
//! | `zstd` | the frame's blocks reach the last block inside their size limits |
//!
//! A zip is found from its end record, so a self-extracting stub in front of it
//! is not part of the blob, and a zip64 archive is not reported. A blob inside
//! an earlier-starting blob (a PNG stored in a zip) is not reported separately.
//!
//! The input is hostile: every read is bounds-checked, and the work of one
//! [`scan`] is capped at a multiple of the region size, after which the scan
//! returns what it has.
//!
//! [`BlobPass`] runs the scan over the image at load; the commit plants each
//! blob as `<kind>_<addr>`, an `unsigned char[len]`, unless `--option blobs off`.
//! `kuna blobs` is the same scan as an inventory.

mod archives;
mod images;

use memchr::memmem;
use object::{Object, ObjectSection, ObjectSegment, SectionKind};

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, BlobFact, Phase};

/// What a blob is.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BlobKind {
    Png,
    Jpeg,
    Gif,
    Bmp,
    Ico,
    Zip,
    Gzip,
    Xz,
    Zstd,
}

impl BlobKind {
    pub const ALL: [BlobKind; 9] = [
        BlobKind::Png,
        BlobKind::Jpeg,
        BlobKind::Gif,
        BlobKind::Bmp,
        BlobKind::Ico,
        BlobKind::Zip,
        BlobKind::Gzip,
        BlobKind::Xz,
        BlobKind::Zstd,
    ];

    /// The lowercase name: the `kind` column, and the planted symbol's prefix.
    pub fn as_str(self) -> &'static str {
        match self {
            BlobKind::Png => "png",
            BlobKind::Jpeg => "jpeg",
            BlobKind::Gif => "gif",
            BlobKind::Bmp => "bmp",
            BlobKind::Ico => "ico",
            BlobKind::Zip => "zip",
            BlobKind::Gzip => "gzip",
            BlobKind::Xz => "xz",
            BlobKind::Zstd => "zstd",
        }
    }

    /// The file extension an extracted blob is written with.
    pub fn extension(self) -> &'static str {
        match self {
            BlobKind::Jpeg => "jpg",
            BlobKind::Gzip => "gz",
            BlobKind::Zstd => "zst",
            other => other.as_str(),
        }
    }
}

/// One blob inside a scanned byte range.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Blob {
    pub offset: usize,
    pub len: usize,
    pub kind: BlobKind,
}

/// The bytes a scan may still walk; a validator that runs out fails.
pub(crate) struct Work(usize);

impl Work {
    pub(crate) fn spend(&mut self, bytes: usize) -> Option<()> {
        let left = self.0.checked_sub(bytes);
        self.0 = left.unwrap_or(0);
        left.map(|_| ())
    }

    fn exhausted(&self) -> bool {
        self.0 == 0
    }
}

/// Walked bytes allowed per scanned byte, and the allowance of an empty region.
const WORK_PER_BYTE: usize = 16;
const WORK_FLOOR: usize = 1 << 20;

const MAGICS: [(BlobKind, &[u8]); 9] = [
    (BlobKind::Png, b"\x89PNG\r\n\x1a\n"),
    (BlobKind::Jpeg, b"\xff\xd8\xff"),
    (BlobKind::Gif, b"GIF8"),
    (BlobKind::Bmp, b"BM"),
    (BlobKind::Ico, b"\x00\x00\x01\x00"),
    (BlobKind::Zip, archives::ZIP_END),
    (BlobKind::Gzip, b"\x1f\x8b\x08"),
    (BlobKind::Xz, b"\xfd7zXZ\x00"),
    (BlobKind::Zstd, b"\x28\xb5\x2f\xfd"),
];

/// Every blob in `data`, sorted by offset and disjoint.
pub fn scan(data: &[u8]) -> Vec<Blob> {
    scan_within(data, &mut Work(data.len().saturating_mul(WORK_PER_BYTE).saturating_add(WORK_FLOOR)))
}

fn scan_within(data: &[u8], work: &mut Work) -> Vec<Blob> {
    let mut finders: Vec<_> =
        MAGICS.iter().map(|&(kind, magic)| (kind, memmem::find_iter(data, magic).peekable())).collect();
    let mut found: Vec<Blob> = Vec::new();
    let mut covered = 0usize;
    loop {
        let next = finders
            .iter_mut()
            .enumerate()
            .filter_map(|(i, (_, it))| it.peek().map(|&at| (at, i)))
            .min();
        let Some((at, i)) = next else { break };
        let (kind, it) = &mut finders[i];
        it.next();
        let kind = *kind;
        if kind != BlobKind::Zip && at < covered {
            continue;
        }
        let hit = match kind {
            BlobKind::Zip => archives::zip(data, at, work),
            _ => validate(kind, &data[at..], work).map(|len| (at, len)),
        };
        if let Some((offset, len)) = hit {
            covered = covered.max(offset + len);
            found.push(Blob { offset, len, kind });
        }
        if work.exhausted() {
            break;
        }
    }
    found.sort_by_key(|b| (b.offset, std::cmp::Reverse(b.len)));
    let mut end = 0usize;
    found.retain(|b| {
        let keep = b.offset >= end;
        if keep {
            end = b.offset + b.len;
        }
        keep
    });
    found
}

fn validate(kind: BlobKind, d: &[u8], work: &mut Work) -> Option<usize> {
    match kind {
        BlobKind::Png => images::png(d, work),
        BlobKind::Jpeg => images::jpeg(d, work),
        BlobKind::Gif => images::gif(d, work),
        BlobKind::Bmp => images::bmp(d),
        BlobKind::Ico => images::ico(d, work),
        BlobKind::Gzip => archives::gzip(d, work),
        BlobKind::Xz => archives::xz(d, work),
        BlobKind::Zstd => archives::zstd(d, work),
        BlobKind::Zip => None,
    }
}

pub(crate) fn le16(d: &[u8], at: usize) -> Option<usize> {
    Some(u16::from_le_bytes(d.get(at..at.checked_add(2)?)?.try_into().ok()?) as usize)
}

pub(crate) fn le32(d: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(d.get(at..at.checked_add(4)?)?.try_into().ok()?))
}

pub(crate) fn be32(d: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_be_bytes(d.get(at..at.checked_add(4)?)?.try_into().ok()?))
}

pub(crate) fn crc32(d: &[u8]) -> u32 {
    let mut crc = flate2::Crc::new();
    crc.update(d);
    crc.sum()
}

/// A mapped, file-backed byte range of an image.
pub struct Region<'d> {
    /// The section name; `None` for a segment of an image with no section table.
    pub name: Option<String>,
    pub vma: u64,
    pub data: &'d [u8],
}

fn is_mapped(sec: &object::Section) -> bool {
    if matches!(sec.kind(), SectionKind::UninitializedData | SectionKind::UninitializedTls) {
        return false;
    }
    match sec.flags() {
        object::SectionFlags::Elf { sh_flags } => sh_flags & u64::from(object::elf::SHF_ALLOC) != 0,
        object::SectionFlags::Coff { characteristics } => {
            characteristics & object::pe::IMAGE_SCN_CNT_UNINITIALIZED_DATA == 0
        }
        object::SectionFlags::MachO { flags } => {
            let t = flags & object::macho::SECTION_TYPE;
            t != object::macho::S_ZEROFILL && t != object::macho::S_GB_ZEROFILL
        }
        _ => false,
    }
}

/// The mapped sections that hold file bytes, or the loadable segments of an
/// image with no section table (the flag says which).
pub fn mapped_regions<'d>(file: &object::File<'d>) -> (Vec<Region<'d>>, bool) {
    let mut out = Vec::new();
    for sec in file.sections() {
        if !is_mapped(&sec) {
            continue;
        }
        let Ok(data) = sec.data() else { continue };
        if !data.is_empty() {
            let name = sec.name().ok().filter(|n| !n.is_empty()).map(str::to_string);
            out.push(Region { name, vma: sec.address(), data });
        }
    }
    if !out.is_empty() {
        return (out, false);
    }
    for seg in file.segments() {
        let Ok(data) = seg.data() else { continue };
        if !data.is_empty() {
            out.push(Region { name: None, vma: seg.address(), data });
        }
    }
    (out, true)
}

/// The load-time pass: one [`BlobFact`] per blob of the image.
pub struct BlobPass;

impl AnalysisPass for BlobPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "blobs"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        for region in mapped_regions(ctx.file).0 {
            for blob in scan(region.data) {
                let Some(addr) = region.vma.checked_add(blob.offset as u64) else { continue };
                out.blobs.push(BlobFact { addr, len: blob.len as u64, kind: blob.kind });
            }
        }
        out
    }
}

#[cfg(test)]
mod tests;
