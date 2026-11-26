//! (kuna `coverage`) The bytes each decompiled function covers, and how much of
//! a program's executable code a batch of decompiles lifted and printed. The
//! executable code is the CODE spans [`crate::funcextent::spans`] clips extents
//! to; NUL bytes no function lifted are left out of a share's total.

use kuna_decomp::funcdata::Funcdata;

use crate::engine::ConsoleProgram;
use crate::project::FuncResult;

/// What a batch of decompiled functions covers of a program's executable bytes.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Coverage {
    /// Bytes the loader serves for the executable spans.
    pub executable_bytes: u64,
    /// Executable bytes that are NUL and that no function lifted.
    pub unlifted_nul_bytes: u64,
    /// Executable bytes some function's flow decoded.
    pub lifted_bytes: u64,
    /// Executable bytes covered by a function decompiled without an error.
    pub decompiled_bytes: u64,
    /// Functions measured.
    pub functions: usize,
    /// Functions decompiled without an error.
    pub decompiled_functions: usize,
    /// Functions with no ranges: their decompile failed.
    pub unlifted_functions: usize,
}

impl Coverage {
    /// The executable bytes less the NUL bytes no function lifted: what a share
    /// is of.
    pub fn share_total(&self) -> u64 {
        self.executable_bytes - self.unlifted_nul_bytes
    }

    /// `lifted_bytes` over [`Self::share_total`], `None` when that is 0.
    pub fn lifted_share(&self) -> Option<f64> {
        self.share(self.lifted_bytes)
    }

    /// `decompiled_bytes` over [`Self::share_total`], `None` when that is 0.
    pub fn decompiled_share(&self) -> Option<f64> {
        self.share(self.decompiled_bytes)
    }

    fn share(&self, bytes: u64) -> Option<f64> {
        let total = self.share_total();
        (total > 0).then(|| bytes as f64 / total as f64)
    }
}

/// The bytes of every instruction the flow decoded for `fd`, dead code and
/// delay slots included ([`Funcdata::flow_ranges`]), in `prog`'s target address
/// units, as a result reports them.
pub fn output_ranges(prog: &ConsoleProgram, fd: &Funcdata) -> Vec<(u64, u64)> {
    joined(
        fd.flow_ranges()
            .iter()
            .map(|&(start, end)| (prog.output_code_offset(start), prog.output_code_end_offset(end)))
            .collect(),
    )
}

/// How many executable bytes one read takes.
const CHUNK: u64 = 1 << 20;

/// A program's executable bytes, kept as one bit per byte, set where the byte
/// is NUL, so a batch can be measured after the program is gone: a `--jobs`
/// parent drops its own before the workers start.
#[derive(Debug, Clone, Default)]
pub struct Code {
    stretches: Vec<Stretch>,
    /// How many bytes one target address unit is.
    unit: u64,
}

/// Executable bytes `[start, end)` and their NUL bits.
#[derive(Debug, Clone)]
struct Stretch {
    start: u64,
    end: u64,
    nul: Vec<u64>,
}

impl Stretch {
    fn of(start: u64, bytes: &[u8]) -> Self {
        let mut nul = vec![0u64; bytes.len().div_ceil(64)];
        for (i, _) in bytes.iter().enumerate().filter(|&(_, &byte)| byte == 0) {
            nul[i / 64] |= 1 << (i % 64);
        }
        Self { start, end: start + bytes.len() as u64, nul }
    }

    /// The NUL bytes in `[from, to)`, which lies inside the stretch.
    fn nul_in(&self, from: u64, to: u64) -> u64 {
        let (mut at, end) = ((from - self.start) as usize, (to - self.start) as usize);
        let mut count = 0;
        while at < end {
            let take = (64 - at % 64).min(end - at);
            let mask = if take == 64 { u64::MAX } else { (1u64 << take) - 1 };
            count += u64::from((self.nul[at / 64] >> (at % 64) & mask).count_ones());
            at += take;
        }
        count
    }
}

impl Code {
    /// The bytes the loader serves for `prog`'s executable spans, read a MiB at
    /// a time. A read the loader refuses (its first byte unmapped) is left out,
    /// and a zero-filled tail reads as NUL bytes.
    pub fn of(prog: &ConsoleProgram) -> Self {
        let mut stretches = Vec::new();
        let mut bytes = Vec::new();
        for (span_start, span_end) in joined(crate::funcextent::spans(&prog.sections(), &prog.segments())) {
            let mut start = span_start;
            while start < span_end {
                let end = span_end.min(start.saturating_add(CHUNK));
                if prog.read_bytes_into(start, (end - start) as usize, &mut bytes) {
                    stretches.push(Stretch::of(start, &bytes));
                }
                start = end;
            }
        }
        Self { stretches, unit: prog.code_unit_bytes() }
    }

    /// Measure `results`. A result is decompiled when it carries no error, as an
    /// export's `ok` count reads it: a streamed export drops each body once it is
    /// written.
    pub fn measure_results(&self, results: &[FuncResult]) -> Coverage {
        self.measure(results.iter().map(|r| (r.ranges.as_slice(), r.error.is_none())))
    }

    /// Measure `functions`, each given as its ranges in target address units and
    /// whether it decompiled.
    pub fn measure<'a>(&self, functions: impl IntoIterator<Item = (&'a [(u64, u64)], bool)>) -> Coverage {
        let mut coverage = Coverage::default();
        let (mut lifted, mut decompiled) = (Vec::new(), Vec::new());
        for (ranges, ok) in functions {
            coverage.functions += 1;
            coverage.decompiled_functions += usize::from(ok);
            coverage.unlifted_functions += usize::from(ranges.is_empty());
            let bytes = ranges
                .iter()
                .map(|&(start, end)| (start.saturating_mul(self.unit), end.saturating_mul(self.unit)));
            if ok {
                decompiled.extend(bytes.clone());
            }
            lifted.extend(bytes);
        }
        let (lifted, decompiled) = (joined(lifted), joined(decompiled));
        for stretch in &self.stretches {
            let (start, end) = (stretch.start, stretch.end);
            coverage.executable_bytes += end - start;
            coverage.lifted_bytes += overlap(start, end, &lifted);
            coverage.decompiled_bytes += overlap(start, end, &decompiled);
            coverage.unlifted_nul_bytes +=
                gaps(start, end, &lifted).map(|(from, to)| stretch.nul_in(from, to)).sum::<u64>();
        }
        coverage
    }
}

/// `ranges` with the empty ones dropped, ascending, and joined where they touch
/// or overlap.
fn joined(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    ranges.retain(|&(start, end)| start < end);
    ranges.sort_unstable();
    let mut out: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());
    for (start, end) in ranges {
        match out.last_mut() {
            Some(last) if start <= last.1 => last.1 = last.1.max(end),
            _ => out.push((start, end)),
        }
    }
    out
}

/// The parts of ascending, disjoint `ranges` inside `[start, end)`.
fn within(start: u64, end: u64, ranges: &[(u64, u64)]) -> impl Iterator<Item = (u64, u64)> + '_ {
    let first = ranges.partition_point(|&(_, range_end)| range_end <= start);
    ranges[first..]
        .iter()
        .take_while(move |&&(range_start, _)| range_start < end)
        .map(move |&(range_start, range_end)| (range_start.max(start), range_end.min(end)))
}

/// How many bytes of `[start, end)` ascending, disjoint `ranges` cover.
fn overlap(start: u64, end: u64, ranges: &[(u64, u64)]) -> u64 {
    within(start, end, ranges).map(|(from, to)| to - from).sum()
}

/// The parts of `[start, end)` ascending, disjoint `ranges` leave uncovered.
fn gaps(start: u64, end: u64, ranges: &[(u64, u64)]) -> impl Iterator<Item = (u64, u64)> + '_ {
    let mut at = start;
    within(start, end, ranges)
        .map(Some)
        .chain(std::iter::once(None))
        .filter_map(move |covered| {
            let (gap_end, next) = covered.unwrap_or((end, end));
            let gap = (at < gap_end).then_some((at, gap_end));
            at = next;
            gap
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn code(stretches: Vec<(u64, Vec<u8>)>, unit: u64) -> Code {
        let stretches = stretches.iter().map(|(start, bytes)| Stretch::of(*start, bytes)).collect();
        Code { stretches, unit }
    }

    #[test]
    fn ranges_are_joined_where_they_touch_or_overlap() {
        assert_eq!(
            joined(vec![(0x20, 0x30), (0x10, 0x18), (0x18, 0x1c), (0x24, 0x40), (0x50, 0x50)]),
            vec![(0x10, 0x1c), (0x20, 0x40)]
        );
    }

    #[test]
    fn the_unlifted_nul_bytes_are_the_padding_between_two_functions() {
        // 0x1000..0x1010: f (16 bytes, decompiled); 0x1010..0x1018: eight NUL
        // bytes of fill; 0x1018..0x1020: g (8 bytes, lifted, failed); then four
        // bytes no function covers, one of them NUL.
        let mut bytes = vec![0x90; 0x24];
        bytes[0x10..0x18].fill(0);
        bytes[0x21] = 0;
        let f = [(0x1000, 0x1010)];
        let g = [(0x1018, 0x1020)];
        let coverage = code(vec![(0x1000, bytes)], 1).measure([
            (&f[..], true),
            (&g[..], false),
            (&[][..], false),
        ]);
        assert_eq!(
            coverage,
            Coverage {
                executable_bytes: 0x24,
                unlifted_nul_bytes: 9,
                lifted_bytes: 0x18,
                decompiled_bytes: 0x10,
                functions: 3,
                decompiled_functions: 1,
                unlifted_functions: 1,
            }
        );
        assert_eq!(coverage.share_total(), 0x1b);
        assert_eq!(coverage.lifted_share(), Some(24.0 / 27.0));
        assert_eq!(coverage.decompiled_share(), Some(16.0 / 27.0));
    }

    #[test]
    fn nul_bytes_are_counted_across_bit_words() {
        // 200 NUL bytes from 3 to 203: across four 64-bit words, ending mid-word.
        let mut bytes = vec![0xcc; 256];
        bytes[3..203].fill(0);
        let stretch = Stretch::of(0x1000, &bytes);
        assert_eq!(stretch.nul_in(0x1000, 0x1100), 200);
        assert_eq!(stretch.nul_in(0x1040, 0x1080), 64);
        assert_eq!(stretch.nul_in(0x10ca, 0x10cc), 1);
        assert_eq!(stretch.nul_in(0x1010, 0x1010), 0);
    }

    #[test]
    fn bytes_outside_the_code_are_not_counted() {
        // f's block runs from 8 bytes before the code to 8 bytes past it.
        let f = [(0x0ff8, 0x1018)];
        let coverage = code(vec![(0x1000, vec![0xc3; 0x10])], 1).measure([(&f[..], true)]);
        assert_eq!(coverage.executable_bytes, 0x10);
        assert_eq!((coverage.lifted_bytes, coverage.decompiled_bytes), (0x10, 0x10));
    }

    #[test]
    fn a_range_across_two_stretches_counts_in_each() {
        let stretches = vec![(0x1000, vec![0x55; 0x10]), (0x1010, vec![0; 0x10])];
        let f = [(0x100c, 0x1014)];
        let coverage = code(stretches, 1).measure([(&f[..], true)]);
        assert_eq!(coverage.executable_bytes, 0x20);
        assert_eq!(coverage.lifted_bytes, 8);
        assert_eq!(coverage.unlifted_nul_bytes, 0xc);
        assert_eq!(coverage.lifted_share(), Some(8.0 / 20.0));
    }

    #[test]
    fn a_word_addressed_image_counts_whole_units_in_bytes() {
        // Two-byte code units: f's range 0x800..0x804 is the bytes 0x1000..0x1008.
        let f = [(0x800, 0x804)];
        let coverage = code(vec![(0x1000, vec![1; 0x10])], 2).measure([(&f[..], true)]);
        assert_eq!((coverage.executable_bytes, coverage.lifted_bytes), (0x10, 8));
    }

    #[test]
    fn no_code_has_no_share() {
        let coverage = code(Vec::new(), 1).measure([(&[(0x10, 0x20)][..], true)]);
        assert_eq!((coverage.executable_bytes, coverage.lifted_bytes), (0, 0));
        assert_eq!(coverage.lifted_share(), None);
    }
}
