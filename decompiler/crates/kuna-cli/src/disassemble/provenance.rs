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
//! permissions it declares. A header field the loader rewrote so the image
//! would parse at all (`kuna_analysis::loader::elf_shdr::read_image_sliced`)
//! is not the file's either, and says so.
//!
//! An image can declare one address more than once: two `PT_LOAD`s over the
//! same page from different file offsets (GH #1426). The loader serves one of
//! them and the bytes stay its; each range also lists every other segment that
//! declares it, with what that segment holds there.
//!
//! The segment map is rebuilt from the object file the way the loader builds
//! its own, and it is trusted only where its extents are the loader's
//! ([`ConsoleProgram::segments`]). An image the loader lays out itself (a
//! relocatable object) or reads with no object-file view (TE, MZ) answers
//! `unknown` instead of a guess.

use std::collections::BTreeMap;

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
    /// File-backed, but a header repair rewrote these bytes; the file's own are
    /// at the range's file offset.
    Repaired,
    /// Not attributable: the loader's map is not one the object file declares.
    Unknown,
}

impl Source {
    fn as_str(self) -> &'static str {
        match self {
            Source::File => "file",
            Source::ZeroFill => "zero-fill",
            Source::Repaired => "repaired",
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
    /// Its 0-based entry in the table that declares it ([`table_entries`]);
    /// `None` for the PE header page.
    index: Option<usize>,
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

/// One maximal run of the span with one source in one segment, and one set of
/// other segments declaring it.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
    source: Source,
    file_offset: Option<u64>,
    segment: Option<usize>,
    overlaps: Vec<Overlap>,
}

/// A segment other than the serving one that declares a range, and what it
/// holds there from the range's start.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Overlap {
    segment: usize,
    source: Source,
    file_offset: Option<u64>,
}

/// The ranges one byte view's span is made of.
pub(super) struct Provenance {
    segments: Vec<Segment>,
    ranges: Vec<Range>,
}

/// Where each byte of `[start, end)` comes from. `view` is the image's bytes and
/// object view, `slice` where those bytes start in the file (a fat Mach-O), and
/// `repairs` the `[start, end)` offsets into them that are not the file's
/// ([`crate::image::image_bytes_and_repairs`]).
pub(super) fn of(
    view: Option<(&[u8], &object::File)>,
    slice: u64,
    repairs: &[(u64, u64)],
    prog: &ConsoleProgram,
    start: u64,
    end: u64,
) -> Provenance {
    let unknown = || Provenance {
        segments: Vec::new(),
        ranges: vec![Range { start, end, source: Source::Unknown, file_offset: None, segment: None, overlaps: Vec::new() }],
    };
    if end <= start {
        return Provenance { segments: Vec::new(), ranges: Vec::new() };
    }
    let Some((bytes, file)) = view else { return unknown() };
    let Some(segments) = segment_map(file, bytes, slice) else { return unknown() };
    if !same_extents(&segments, &prog.segments()) {
        return unknown();
    }
    let repairs: Vec<(u64, u64)> = repairs.iter().map(|&(lo, hi)| (slice + lo, slice + hi)).collect();
    let ranges = cut(&segments, &repairs, start, end);
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
    let (entry, indices) = table_entries(file);
    let indices = (indices.len() == file.segments().count()).then_some(indices);
    let mut segments = Vec::new();
    for (k, seg) in file.segments().enumerate() {
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
            index: indices.as_ref().map(|v| v[k]),
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
            index: None,
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
    disambiguate(&mut segments, entry);
    segments.sort_by_key(|s| s.vma);
    let loaded: Vec<usize> = (0..segments.len()).filter(|&i| segments[i].served > 0).collect();
    for pair in loaded.windows(2) {
        let room = segments[pair[1]].vma.saturating_sub(segments[pair[0]].vma);
        let seg = &mut segments[pair[0]];
        seg.served = seg.served.min(room).max(seg.file_size);
    }
    Some(segments)
}

/// What the table declaring `file.segments()` calls an entry, and each
/// segment's 0-based index in it: the ELF program header table (`PT_LOAD`s
/// among the other program headers), the Mach-O load commands, the PE section
/// table.
fn table_entries(file: &object::File) -> (&'static str, Vec<usize>) {
    use object::read::elf::{ElfFile, FileHeader, ProgramHeader};
    use object::read::macho::{MachHeader, MachOFile};
    fn loads<'d, H: FileHeader>(f: &ElfFile<'d, H>) -> Vec<usize> {
        let endian = f.endian();
        let headers = f.elf_program_headers().iter().enumerate();
        headers.filter(|(_, h)| h.p_type(endian) == object::elf::PT_LOAD).map(|(i, _)| i).collect()
    }
    fn commands<'d, M: MachHeader>(f: &MachOFile<'d, M>, segment: u32) -> Vec<usize> {
        let mut found = Vec::new();
        let Ok(mut commands) = f.macho_load_commands() else { return found };
        let mut i = 0;
        while let Ok(Some(command)) = commands.next() {
            if command.cmd() == segment {
                found.push(i);
            }
            i += 1;
        }
        found
    }
    match file {
        object::File::Elf32(f) => ("program header", loads(f)),
        object::File::Elf64(f) => ("program header", loads(f)),
        object::File::MachO32(f) => ("load command", commands(f, object::macho::LC_SEGMENT)),
        object::File::MachO64(f) => ("load command", commands(f, object::macho::LC_SEGMENT_64)),
        _ => ("section", (0..file.segments().count()).collect()),
    }
}

/// Name each segment whose label another shares by its table entry as well:
/// two `PT_LOAD`s at one address, two sections of one name.
fn disambiguate(segments: &mut [Segment], entry: &str) {
    let mut uses: BTreeMap<String, usize> = BTreeMap::new();
    for seg in segments.iter() {
        *uses.entry(seg.label.clone()).or_default() += 1;
    }
    for seg in segments.iter_mut() {
        if let Some(index) = seg.index.filter(|_| uses[&seg.label] > 1) {
            seg.label = format!("{} ({entry} {index})", seg.label);
        }
    }
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
/// the loader reads as zero. `repairs` are file offsets.
fn classify(segments: &[Segment], repairs: &[(u64, u64)], addr: u64) -> (Source, Option<u64>, Option<usize>) {
    let serving = segments.iter().position(|s| s.served > 0 && addr >= s.vma && addr - s.vma < s.served);
    if let Some(i) = serving {
        let (source, offset) = held(&segments[i], repairs, addr);
        return (source, offset, Some(i));
    }
    let declaring = segments.iter().position(|s| addr >= s.vma && addr - s.vma < s.size);
    (Source::ZeroFill, None, declaring)
}

/// What `seg` holds at `addr`, an address it declares: its file bytes there
/// (repaired or not), else zero fill.
fn held(seg: &Segment, repairs: &[(u64, u64)], addr: u64) -> (Source, Option<u64>) {
    let rel = addr - seg.vma;
    if rel >= seg.file_size {
        return (Source::ZeroFill, None);
    }
    let offset = seg.file_offset + rel;
    let repaired = repairs.iter().any(|&(lo, hi)| offset >= lo && offset < hi);
    (if repaired { Source::Repaired } else { Source::File }, Some(offset))
}

/// Every segment but `chosen` whose declared memory or file extent holds `addr`.
fn others(segments: &[Segment], repairs: &[(u64, u64)], addr: u64, chosen: Option<usize>) -> Vec<Overlap> {
    let declares = |s: &Segment| addr >= s.vma && addr - s.vma < s.size.max(s.file_size);
    (0..segments.len())
        .filter(|&j| Some(j) != chosen && declares(&segments[j]))
        .map(|j| {
            let (source, file_offset) = held(&segments[j], repairs, addr);
            Overlap { segment: j, source, file_offset }
        })
        .collect()
}

/// `[start, end)` cut at every point where the source or the segment changes.
fn cut(segments: &[Segment], repairs: &[(u64, u64)], start: u64, end: u64) -> Vec<Range> {
    let mut points = vec![start, end];
    for s in segments {
        let fixed = repairs.iter().filter(|&&(lo, hi)| lo < s.file_offset + s.file_size && hi > s.file_offset);
        let repaired = fixed.flat_map(|&(lo, hi)| [lo.saturating_sub(s.file_offset), (hi - s.file_offset).min(s.file_size)]);
        for len in [0, s.file_size, s.served, s.size].into_iter().chain(repaired) {
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
        let (source, file_offset, segment) = classify(segments, repairs, piece[0]);
        let overlaps = others(segments, repairs, piece[0], segment);
        if let Some(last) = ranges.last_mut() {
            let len = last.end - last.start;
            let continues = last.file_offset.zip(file_offset).is_none_or(|(a, b)| a + len == b);
            let same = |a: &Overlap, b: &Overlap| a.segment == b.segment && a.source == b.source;
            let alike = last.overlaps.len() == overlaps.len() && last.overlaps.iter().zip(&overlaps).all(|(a, b)| same(a, b));
            if last.source == source && last.segment == segment && continues && alike {
                last.end = piece[1];
                continue;
            }
        }
        ranges.push(Range { start: piece[0], end: piece[1], source, file_offset, segment, overlaps });
    }
    ranges
}

impl Provenance {
    /// The stderr / `notes` lines: one for zero fill, one for repaired header
    /// bytes, one for bytes in a segment that declares no read access, one for
    /// addresses more than one segment declares.
    pub(super) fn notes(&self) -> Vec<String> {
        let mut notes = Vec::new();
        let filled: Vec<&Range> = self.ranges.iter().filter(|r| r.source == Source::ZeroFill).collect();
        if !filled.is_empty() {
            let parts = self.spell(&filled, |_, seg| {
                format!("0x{:x} bytes in the file, 0x{:x} in memory", seg.file_size, seg.size)
            });
            notes.push(format!(
                "zero fill, not file bytes: {parts} -- kuna reads a segment's memory past what the file holds \
                 for it as zeroes; `--json` lists each range's source in `provenance`"
            ));
        }
        let repaired: Vec<&Range> = self.ranges.iter().filter(|r| r.source == Source::Repaired).collect();
        if !repaired.is_empty() {
            let parts = self.spell(&repaired, |r, _| format!("file offset 0x{:x}", r.file_offset.unwrap_or(0)));
            notes.push(format!(
                "repaired, not file bytes: {parts} -- kuna rewrote a header field there so the image would \
                 parse; the file's own bytes are at that offset"
            ));
        }
        let mut sealed: Vec<Range> = Vec::new();
        let unreadable = |r: &&Range| r.segment.and_then(|i| self.segments[i].perms).is_some_and(|p| !p.read);
        for r in self.ranges.iter().filter(unreadable) {
            match sealed.last_mut() {
                Some(last) if last.end == r.start && last.segment == r.segment => last.end = r.end,
                _ => sealed.push(r.clone()),
            }
        }
        let sealed: Vec<&Range> = sealed.iter().collect();
        if !sealed.is_empty() {
            let parts = self.spell(&sealed, |_, seg| {
                format!("permissions {}", seg.perms.map_or("unknown".to_string(), Perms::spelled))
            });
            notes.push(format!("no read access: {parts} -- the image declares the segment unreadable; kuna lists its bytes anyway"));
        }
        let shared: Vec<&Range> = self.ranges.iter().filter(|r| !r.overlaps.is_empty()).collect();
        if !shared.is_empty() {
            let mut parts: Vec<String> = shared.iter().take(NOTE_RANGES).map(|r| self.spell_overlap(r)).collect();
            if shared.len() > NOTE_RANGES {
                parts.push(format!("and {} more", shared.len() - NOTE_RANGES));
            }
            notes.push(format!(
                "overlapping segments: {} -- the image maps these addresses more than once; kuna orders its \
                 segments by start address, equal starts in table order, ends each one's zero fill where the \
                 next starts, and reads the first that covers an address; `--json` lists the others in each \
                 range's `overlaps`",
                parts.join("; ")
            ));
        }
        notes
    }

    /// `0x401000..0x401010 is read from file offset 0x1000 in A, and B maps it
    /// from file offset 0x2000`.
    fn spell_overlap(&self, r: &Range) -> String {
        let label = |i: usize| &self.segments[i].label;
        let chosen = match (r.source, r.file_offset, r.segment) {
            (Source::ZeroFill, _, Some(i)) => format!("is zero fill in {}", label(i)),
            (source, Some(offset), Some(i)) => {
                let repaired = if source == Source::Repaired { " (repaired)" } else { "" };
                format!("is read from file offset 0x{offset:x}{repaired} in {}", label(i))
            }
            _ => "is listed".to_string(),
        };
        let others: Vec<String> = r
            .overlaps
            .iter()
            .map(|o| match (o.source, o.file_offset) {
                (Source::Repaired, Some(offset)) => {
                    format!("{} maps it from file offset 0x{offset:x} (repaired)", label(o.segment))
                }
                (_, Some(offset)) => format!("{} maps it from file offset 0x{offset:x}", label(o.segment)),
                _ => format!("{} declares it as zero fill", label(o.segment)),
            })
            .collect();
        format!("0x{:x}..0x{:x} {chosen}, and {}", r.start, r.end, others.join(" and "))
    }

    /// `0x0..0x10 in __PAGEZERO (detail)`, for up to [`NOTE_RANGES`] ranges.
    fn spell(&self, ranges: &[&Range], detail: impl Fn(&Range, &Segment) -> String) -> String {
        let mut parts: Vec<String> = ranges
            .iter()
            .take(NOTE_RANGES)
            .map(|r| match r.segment.map(|i| &self.segments[i]) {
                Some(seg) => format!("0x{:x}..0x{:x} in {} ({})", r.start, r.end, seg.label, detail(r, seg)),
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
        let rows = self
            .ranges
            .iter()
            .map(|r| {
                let overlaps = r
                    .overlaps
                    .iter()
                    .map(|o| {
                        Json::Object(vec![
                            ("source".into(), Json::Str(o.source.as_str().into())),
                            ("file_offset".into(), o.file_offset.map_or(Json::Null, number)),
                            ("file_offset_hex".into(), o.file_offset.map_or(Json::Null, hex)),
                            ("segment".into(), self.segment_json(o.segment)),
                        ])
                    })
                    .collect();
                Json::Object(vec![
                    ("start".into(), number(r.start)),
                    ("start_hex".into(), hex(r.start)),
                    ("end".into(), number(r.end)),
                    ("end_hex".into(), hex(r.end)),
                    ("size".into(), number(r.end - r.start)),
                    ("source".into(), Json::Str(r.source.as_str().into())),
                    ("file_offset".into(), r.file_offset.map_or(Json::Null, number)),
                    ("file_offset_hex".into(), r.file_offset.map_or(Json::Null, hex)),
                    ("segment".into(), r.segment.map_or(Json::Null, |i| self.segment_json(i))),
                    ("overlaps".into(), Json::Array(overlaps)),
                ])
            })
            .collect();
        vec![("provenance".into(), Json::Array(rows))]
    }

    fn segment_json(&self, i: usize) -> Json {
        let s = &self.segments[i];
        Json::Object(vec![
            ("index".into(), s.index.map_or(Json::Null, |i| number(i as u64))), // cast: table index
            ("name".into(), s.name.clone().map_or(Json::Null, Json::Str)),
            ("start".into(), number(s.vma)),
            ("start_hex".into(), hex(s.vma)),
            ("size".into(), number(s.size)),
            ("file_offset".into(), number(s.file_offset)),
            ("file_size".into(), number(s.file_size)),
            ("permissions".into(), s.perms.map_or(Json::Null, |p| Json::Str(p.spelled()))),
        ])
    }
}

fn number(v: u64) -> Json {
    Json::Number(v.to_string())
}

fn hex(v: u64) -> Json {
    Json::Str(format!("0x{v:x}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seg(vma: u64, size: u64, file_size: u64, served: u64) -> Segment {
        Segment {
            index: None,
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
        Range { start, end, source, file_offset, segment, overlaps: Vec::new() }
    }

    #[test]
    fn a_span_is_cut_where_the_file_content_ends() {
        let segments = [seg(0x1000, 0x40, 0x10, 0x40)];
        assert_eq!(
            cut(&segments, &[], 0x1008, 0x1020),
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
        assert_eq!(classify(&segments, &[], 0x1010), (Source::ZeroFill, None, Some(0)));
    }

    #[test]
    fn the_first_serving_segment_owns_a_byte_two_segments_claim() {
        let segments = [seg(0x1000, 0x100, 0x10, 0x20), seg(0x1020, 0x10, 0x10, 0x10)];
        assert_eq!(classify(&segments, &[], 0x1020), (Source::File, Some(0x100), Some(1)));
    }

    fn at(file_offset: u64, segment: Segment) -> Segment {
        Segment { file_offset, ..segment }
    }

    fn other(segment: usize, source: Source, file_offset: Option<u64>) -> Overlap {
        Overlap { segment, source, file_offset }
    }

    #[test]
    fn two_loads_of_one_address_keep_the_first_and_list_the_second() {
        let segments = [at(0x1000, seg(0x401000, 0x10, 0x10, 0x10)), at(0x2000, seg(0x401000, 0x10, 0x10, 0x10))];
        let mut want = range(0x401000, 0x401010, Source::File, Some(0x1000), Some(0));
        want.overlaps = vec![other(1, Source::File, Some(0x2000))];
        assert_eq!(cut(&segments, &[], 0x401000, 0x401010), [want]);
    }

    #[test]
    fn a_partial_overlap_is_its_own_range() {
        let segments = [at(0x1000, seg(0x401000, 0x20, 0x20, 0x20)), at(0x2000, seg(0x401010, 0x20, 0x20, 0x20))];
        let mut shared = range(0x401010, 0x401020, Source::File, Some(0x1010), Some(0));
        shared.overlaps = vec![other(1, Source::File, Some(0x2000))];
        assert_eq!(
            cut(&segments, &[], 0x401000, 0x401030),
            [
                range(0x401000, 0x401010, Source::File, Some(0x1000), Some(0)),
                shared,
                range(0x401020, 0x401030, Source::File, Some(0x2010), Some(1)),
            ]
        );
    }

    #[test]
    fn a_zero_fill_tail_under_the_next_segments_bytes_is_an_overlap() {
        // The tail is trimmed where the next segment starts, so the next one
        // serves; the first still declares that memory, as zero fill.
        let segments = [at(0x1000, seg(0x601000, 0x100, 0x10, 0x80)), at(0x2080, seg(0x601080, 0x10, 0x10, 0x10))];
        let mut shared = range(0x601080, 0x601090, Source::File, Some(0x2080), Some(1));
        shared.overlaps = vec![other(0, Source::ZeroFill, None)];
        assert_eq!(cut(&segments, &[], 0x601070, 0x601090)[1], shared);
    }

    #[test]
    fn neighbours_that_meet_or_share_a_page_do_not_overlap() {
        let meeting = [at(0x1000, seg(0x401000, 0x1000, 0x1000, 0x1000)), at(0x2000, seg(0x402000, 0x10, 0x10, 0x10))];
        assert!(cut(&meeting, &[], 0x400ff0, 0x402010).iter().all(|r| r.overlaps.is_empty()));
        let one_page = [at(0x1000, seg(0x401000, 0x10, 0x10, 0x10)), at(0x1800, seg(0x401800, 0x10, 0x10, 0x10))];
        assert!(cut(&one_page, &[], 0x401000, 0x401810).iter().all(|r| r.overlaps.is_empty()));
    }

    #[test]
    fn a_repaired_header_field_is_cut_out_of_the_file_bytes_around_it() {
        let segments = [seg(0x1000, 0x40, 0x40, 0x40)];
        assert_eq!(
            cut(&segments, &[(0x128, 0x12c)], 0x1020, 0x1030),
            [
                range(0x1020, 0x1028, Source::File, Some(0x120), Some(0)),
                range(0x1028, 0x102c, Source::Repaired, Some(0x128), Some(0)),
                range(0x102c, 0x1030, Source::File, Some(0x12c), Some(0)),
            ]
        );
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
