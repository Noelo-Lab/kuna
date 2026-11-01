//! (kuna) Where a byte view's bytes come from, and what the image lets a
//! running program do with them (GH #1401).
//!
//! The load image serves a segment over its whole memory size and zero-fills
//! what the file does not hold (`kuna_analysis::loadimage_object`): a `.bss`, a
//! Mach-O segment whose `vmsize` runs past its `filesize` (`__PAGEZERO`, or the
//! page Apple's ld64 rounds `__LINKEDIT` up to), a PE section whose
//! `VirtualSize` runs past its raw data. That is the analysis model, and it
//! stays. But such a zero reads exactly like a zero in the file, and a segment
//! that declares no access at all reads like any other. So the listed span is
//! cut into ranges, each saying whether its bytes are the file's (and at which
//! offset) or zero fill, and naming the segment that holds them with the
//! permissions it declares.
//!
//! The segment map is rebuilt from the object file the way the loader builds
//! its own, and it is trusted only where its extents are the loader's
//! ([`ConsoleProgram::segments`]). An image the loader lays out itself (a
//! relocatable object) or reads with no object-file view (TE, MZ) answers
//! `unknown` instead of a guess.

use kuna_analysis::loader::format::detect;
use kuna_console::engine::ConsoleProgram;
use object::read::{Object, ObjectSegment};
use object::SegmentFlags;

use crate::jsonfmt::Json;

/// How many ranges a note names before it counts the rest.
const NOTE_RANGES: usize = 3;

/// Where one range's bytes come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Source {
    /// The file's bytes, at the range's file offset.
    File,
    /// Memory the segment declares and the file does not back: kuna reads zeroes.
    ZeroFill,
    /// Not attributable: the loader's map is not one the object file declares.
    Unknown,
}

impl Source {
    fn as_str(self) -> &'static str {
        match self {
            Source::File => "file",
            Source::ZeroFill => "zero-fill",
            Source::Unknown => "unknown",
        }
    }
}

/// The access a segment declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Perms {
    read: bool,
    write: bool,
    exec: bool,
}

impl Perms {
    /// ELF `p_flags`, Mach-O `initprot`, PE section `Characteristics`.
    fn of(flags: SegmentFlags) -> Option<Perms> {
        let (read, write, exec) = match flags {
            SegmentFlags::Elf { p_flags } => (p_flags & 4 != 0, p_flags & 2 != 0, p_flags & 1 != 0),
            SegmentFlags::MachO { initprot, .. } => (initprot & 1 != 0, initprot & 2 != 0, initprot & 4 != 0),
            SegmentFlags::Coff { characteristics } => (
                characteristics & object::pe::IMAGE_SCN_MEM_READ != 0,
                characteristics & object::pe::IMAGE_SCN_MEM_WRITE != 0,
                characteristics & object::pe::IMAGE_SCN_MEM_EXECUTE != 0,
            ),
            _ => return None,
        };
        Some(Perms { read, write, exec })
    }

    /// `r-x`, the way `otool -l` and `vmmap` spell it.
    fn spelled(self) -> String {
        [(self.read, 'r'), (self.write, 'w'), (self.exec, 'x')]
            .iter()
            .map(|&(on, c)| if on { c } else { '-' })
            .collect()
    }
}

/// One segment as the image declares it.
#[derive(Debug, Clone)]
struct Segment {
    /// The Mach-O segment or PE section name; `None` for an ELF `PT_LOAD` and
    /// the PE header page.
    name: Option<String>,
    /// What a note calls it.
    label: String,
    vma: u64,
    /// The memory it declares: ELF `p_memsz`, Mach-O `vmsize`, PE `VirtualSize`.
    size: u64,
    file_offset: u64,
    /// How many bytes from `file_offset` the loader copies in.
    file_size: u64,
    /// How far the loader serves bytes from it: the file bytes plus the
    /// zero-filled tail, `0` for a segment it does not load.
    served: u64,
    perms: Option<Perms>,
}

/// One maximal run of the span with one source in one segment.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
    source: Source,
    file_offset: Option<u64>,
    segment: Option<usize>,
}

/// The ranges one byte view's span is made of.
pub(super) struct Provenance {
    segments: Vec<Segment>,
    ranges: Vec<Range>,
}

/// Where each byte of `[start, end)` comes from. `view` is the image's bytes and
/// object view, `slice` where those bytes start in the file (a fat Mach-O).
pub(super) fn of(
    view: Option<(&[u8], &object::File)>,
    slice: u64,
    prog: &ConsoleProgram,
    start: u64,
    end: u64,
) -> Provenance {
    let unknown = || Provenance {
        segments: Vec::new(),
        ranges: vec![Range { start, end, source: Source::Unknown, file_offset: None, segment: None }],
    };
    if end <= start {
        return Provenance { segments: Vec::new(), ranges: Vec::new() };
    }
    let Some((bytes, file)) = view else { return unknown() };
    let Some(segments) = segment_map(file, bytes, slice) else { return unknown() };
    if !same_extents(&segments, &prog.segments()) {
        return unknown();
    }
    let ranges = cut(&segments, start, end);
    Provenance { segments, ranges }
}

/// The image's segments in the loader's order, with the extent the loader
/// serves from each (`ObjectLoadImage::from_bytes` and its
/// `clamp_virtual_tails`): an executable segment serves only its file bytes, any
/// other its whole memory size up to the next loaded segment, and a PE's header
/// page its `SizeOfHeaders` bytes. `None` where the loader would refuse the
/// image.
fn segment_map(file: &object::File, bytes: &[u8], slice: u64) -> Option<Vec<Segment>> {
    let elf = file.format() == object::BinaryFormat::Elf;
    let mut segments = Vec::new();
    for seg in file.segments() {
        let file_size = seg.data().ok()?.len() as u64; // cast: byte count
        let perms = Perms::of(seg.flags());
        let memsz = if perms.is_some_and(|p| p.exec) { 0 } else { seg.size() };
        let loaded = file_size != 0 || memsz != 0;
        if seg.size() == 0 && !loaded {
            continue;
        }
        let name = seg.name().ok().flatten().filter(|n| !n.is_empty()).map(str::to_owned);
        let label = match &name {
            Some(name) => name.clone(),
            None if elf => format!("the PT_LOAD at 0x{:x}", seg.address()),
            None => format!("the segment at 0x{:x}", seg.address()),
        };
        segments.push(Segment {
            name,
            label,
            vma: seg.address(),
            size: seg.size(),
            file_offset: slice + seg.file_range().0,
            file_size,
            served: if loaded { file_size.max(memsz) } else { 0 },
            perms,
        });
    }
    if let Some(region) = detect(file).ok().and_then(|fmt| fmt.header_region(file, bytes)) {
        let len = region.len as u64; // cast: clamped to the file length
        segments.push(Segment {
            name: None,
            label: "the header page".to_string(),
            vma: region.vma,
            size: len,
            file_offset: slice,
            file_size: len,
            served: len,
            perms: Some(Perms { read: true, write: false, exec: false }),
        });
    }
    segments.sort_by_key(|s| s.vma);
    let loaded: Vec<usize> = (0..segments.len()).filter(|&i| segments[i].served > 0).collect();
    for pair in loaded.windows(2) {
        let room = segments[pair[1]].vma.saturating_sub(segments[pair[0]].vma);
        let seg = &mut segments[pair[0]];
        seg.served = seg.served.min(room).max(seg.file_size);
    }
    Some(segments)
}

/// Is `segments`'s declared map the loader's own? The loader lists every
/// segment that declares memory, and the PE header page.
fn same_extents(segments: &[Segment], loader: &[(u64, u64, u32)]) -> bool {
    let mut ours: Vec<(u64, u64)> = segments.iter().filter(|s| s.size != 0).map(|s| (s.vma, s.size)).collect();
    let mut theirs: Vec<(u64, u64)> = loader.iter().map(|&(vma, size, _)| (vma, size)).collect();
    ours.sort_unstable();
    theirs.sort_unstable();
    !theirs.is_empty() && ours == theirs
}

/// The source of the byte at `addr`: the first loaded segment serving it (the
/// loader's `find_section`), else the segment declaring it, whose bytes there
/// the loader reads as zero.
fn classify(segments: &[Segment], addr: u64) -> (Source, Option<u64>, Option<usize>) {
    let serving = segments.iter().position(|s| s.served > 0 && addr >= s.vma && addr - s.vma < s.served);
    if let Some(i) = serving {
        let rel = addr - segments[i].vma;
        if rel < segments[i].file_size {
            return (Source::File, Some(segments[i].file_offset + rel), Some(i));
        }
        return (Source::ZeroFill, None, Some(i));
    }
    let declaring = segments.iter().position(|s| addr >= s.vma && addr - s.vma < s.size);
    (Source::ZeroFill, None, declaring)
}

/// `[start, end)` cut at every point where the source or the segment changes.
fn cut(segments: &[Segment], start: u64, end: u64) -> Vec<Range> {
    let mut points = vec![start, end];
    for s in segments {
        for len in [0, s.file_size, s.served, s.size] {
            let edge = s.vma.saturating_add(len);
            if edge > start && edge < end {
                points.push(edge);
            }
        }
    }
    points.sort_unstable();
    points.dedup();
    let mut ranges: Vec<Range> = Vec::new();
    for piece in points.windows(2) {
        let (source, file_offset, segment) = classify(segments, piece[0]);
        if let Some(last) = ranges.last_mut() {
            let len = last.end - last.start;
            let continues = last.file_offset.zip(file_offset).is_none_or(|(a, b)| a + len == b);
            if last.source == source && last.segment == segment && continues {
                last.end = piece[1];
                continue;
            }
        }
        ranges.push(Range { start: piece[0], end: piece[1], source, file_offset, segment });
    }
    ranges
}

impl Provenance {
    /// The stderr / `notes` lines: one for zero fill, one for bytes in a segment
    /// that declares no read access.
    pub(super) fn notes(&self) -> Vec<String> {
        let mut notes = Vec::new();
        let filled: Vec<&Range> = self.ranges.iter().filter(|r| r.source == Source::ZeroFill).collect();
        if !filled.is_empty() {
            let parts = self.spell(&filled, |seg| {
                format!("0x{:x} bytes in the file, 0x{:x} in memory", seg.file_size, seg.size)
            });
            notes.push(format!(
                "zero fill, not file bytes: {parts} -- kuna reads a segment's memory past what the file holds \
                 for it as zeroes; `--json` lists each range's source in `provenance`"
            ));
        }
        let sealed: Vec<&Range> = self
            .ranges
            .iter()
            .filter(|r| r.segment.and_then(|i| self.segments[i].perms).is_some_and(|p| !p.read))
            .collect();
        if !sealed.is_empty() {
            let parts = self.spell(&sealed, |seg| {
                format!("permissions {}", seg.perms.map_or("unknown".to_string(), Perms::spelled))
            });
            notes.push(format!("no read access: {parts} -- the image declares the segment unreadable; kuna lists its bytes anyway"));
        }
        notes
    }

    /// `0x0..0x10 in __PAGEZERO (detail)`, for up to [`NOTE_RANGES`] ranges.
    fn spell(&self, ranges: &[&Range], detail: impl Fn(&Segment) -> String) -> String {
        let mut parts: Vec<String> = ranges
            .iter()
            .take(NOTE_RANGES)
            .map(|r| match r.segment.map(|i| &self.segments[i]) {
                Some(seg) => format!("0x{:x}..0x{:x} in {} ({})", r.start, r.end, seg.label, detail(seg)),
                None => format!("0x{:x}..0x{:x}", r.start, r.end),
            })
            .collect();
        if ranges.len() > NOTE_RANGES {
            parts.push(format!("and {} more", ranges.len() - NOTE_RANGES));
        }
        parts.join("; ")
    }

    /// `provenance`, for the byte view's JSON document.
    pub(super) fn json_fields(&self) -> Vec<(String, Json)> {
        let number = |v: u64| Json::Number(v.to_string());
        let hex = |v: u64| Json::Str(format!("0x{v:x}"));
        let rows = self
            .ranges
            .iter()
            .map(|r| {
                let segment = r.segment.map_or(Json::Null, |i| {
                    let s = &self.segments[i];
                    Json::Object(vec![
                        ("name".into(), s.name.clone().map_or(Json::Null, Json::Str)),
                        ("start".into(), number(s.vma)),
                        ("start_hex".into(), hex(s.vma)),
                        ("size".into(), number(s.size)),
                        ("file_offset".into(), number(s.file_offset)),
                        ("file_size".into(), number(s.file_size)),
                        ("permissions".into(), s.perms.map_or(Json::Null, |p| Json::Str(p.spelled()))),
                    ])
                });
                Json::Object(vec![
                    ("start".into(), number(r.start)),
                    ("start_hex".into(), hex(r.start)),
                    ("end".into(), number(r.end)),
                    ("end_hex".into(), hex(r.end)),
                    ("size".into(), number(r.end - r.start)),
                    ("source".into(), Json::Str(r.source.as_str().into())),
                    ("file_offset".into(), r.file_offset.map_or(Json::Null, number)),
                    ("file_offset_hex".into(), r.file_offset.map_or(Json::Null, hex)),
                    ("segment".into(), segment),
                ])
            })
            .collect();
        vec![("provenance".into(), Json::Array(rows))]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seg(vma: u64, size: u64, file_size: u64, served: u64) -> Segment {
        Segment {
            name: None,
            label: format!("the PT_LOAD at 0x{vma:x}"),
            vma,
            size,
            file_offset: 0x100,
            file_size,
            served,
            perms: Some(Perms { read: true, write: true, exec: false }),
        }
    }

    fn range(start: u64, end: u64, source: Source, file_offset: Option<u64>, segment: Option<usize>) -> Range {
        Range { start, end, source, file_offset, segment }
    }

    #[test]
    fn a_span_is_cut_where_the_file_content_ends() {
        let segments = [seg(0x1000, 0x40, 0x10, 0x40)];
        assert_eq!(
            cut(&segments, 0x1008, 0x1020),
            [
                range(0x1008, 0x1010, Source::File, Some(0x108), Some(0)),
                range(0x1010, 0x1020, Source::ZeroFill, None, Some(0)),
            ]
        );
    }

    #[test]
    fn a_code_segments_tail_is_zero_fill_the_loader_does_not_serve() {
        // An executable segment serves only its file bytes; the rest of its
        // declared memory still reads back as zero inside a listing.
        let segments = [seg(0x1000, 0x40, 0x10, 0x10)];
        assert_eq!(classify(&segments, 0x1010), (Source::ZeroFill, None, Some(0)));
    }

    #[test]
    fn the_first_serving_segment_owns_a_byte_two_segments_claim() {
        let segments = [seg(0x1000, 0x100, 0x10, 0x20), seg(0x1020, 0x10, 0x10, 0x10)];
        assert_eq!(classify(&segments, 0x1020), (Source::File, Some(0x100), Some(1)));
    }

    #[test]
    fn the_loaders_map_must_match_or_nothing_is_claimed() {
        let segments = [seg(0x1000, 0x40, 0x10, 0x40)];
        assert!(same_extents(&segments, &[(0x1000, 0x40, 8)]));
        assert!(!same_extents(&segments, &[(0x1000, 0x30, 8)]));
        assert!(!same_extents(&segments, &[]));
    }

    #[test]
    fn permissions_spell_like_otool() {
        assert_eq!(Perms::of(SegmentFlags::MachO { flags: 0, maxprot: 0, initprot: 0 }).unwrap().spelled(), "---");
        assert_eq!(Perms::of(SegmentFlags::MachO { flags: 0, maxprot: 7, initprot: 5 }).unwrap().spelled(), "r-x");
        assert_eq!(Perms::of(SegmentFlags::Elf { p_flags: 6 }).unwrap().spelled(), "rw-");
        assert_eq!(Perms::of(SegmentFlags::Coff { characteristics: 0x0800_0040 }).unwrap().spelled(), "---");
    }
}
