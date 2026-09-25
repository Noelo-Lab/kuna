//! The instruction walker behind `kuna disassemble` and the in-browser study
//! view: a straight-line (or, with follow, a flow-anchored) decode of a window
//! of the image into rows, with undecodable bytes spelled as `.byte`, proved
//! literal-pool words folded into `.word` rows ([`crate::litpool`]), and the
//! branch targets no row starts at reported.

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use kuna_sleigh::loadimage::section_flags;

use crate::engine::{ConsoleProgram, FixedRefs};

/// Where a walk starts and stops.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Window {
    pub start: u64,
    /// Exclusive stop. `None` when only a row count bounds the walk.
    pub end: Option<u64>,
    /// Was the stop derived from the inventory rather than asked for? Such a
    /// walk is capped at [`DERIVED_INSTRUCTION_CAP`] rows.
    pub derived: bool,
    /// Exclusive end of the mapped run holding `start` ([`mapped_run_end`]).
    pub mapped_end: Option<u64>,
}

impl Window {
    /// Where the walk actually stops: the tighter of the caller's own stop and
    /// the end of mapped memory.
    pub fn stop(&self) -> Option<u64> {
        match (self.end, self.mapped_end) {
            (Some(end), Some(mapped)) => Some(end.min(mapped)),
            (end, mapped) => end.or(mapped),
        }
    }
}

/// Safety stop for a listing whose length was DERIVED from the inventory rather
/// than asked for.
///
/// The extent is an upper bound clipped at the next discovered entry or the end
/// of the containing CODE section (`kuna-console/src/funcextent.rs`), so where
/// discovery is thin the "function" runs to the end of `.text`: `main` in the
/// unpacked `Sh4ll6` crackme clips to 19,106 instructions, about 1.2 MB of
/// listing for what the caller asked to see one function of. Truncating there is
/// the useful answer — the header and the JSON `truncated` flag say so, and
/// `end` is the address to resume from. An explicit `--count`, `--bytes` or
/// address range is honored verbatim, however long.
pub const DERIVED_INSTRUCTION_CAP: usize = 1024;

/// The mnemonic given to bytes the translator would not decode.
pub const BAD_BYTE_MNEMONIC: &str = ".byte";

/// The widest resume grid [`resume_grid`] will infer from a listing's own rows.
/// No instruction set aligns further than this, so a wider shared alignment is
/// a coincidence of a short listing, not a grid.
const MAX_RESUME_GRID: u64 = 16;

/// One listed instruction.
pub struct Row {
    pub addr: u64,
    pub size: u64,
    pub bytes: Vec<u8>,
    pub mnemonic: String,
    pub operands: String,
}

impl Row {
    /// `MNEMONIC operands` with ONE space — the same instruction spelling
    /// `kuna xrefs` puts in its `instruction` field, so the two surfaces are
    /// greppable with one pattern (`CALL 0x140002490`). The console's own
    /// listing pads the mnemonic to a fixed column instead; that padding is a
    /// display choice, not part of the instruction.
    pub fn text(&self) -> String {
        if self.operands.is_empty() {
            self.mnemonic.clone()
        } else {
            format!("{} {}", self.mnemonic, self.operands)
        }
    }

    pub fn hex(&self) -> String {
        hex(&self.bytes)
    }
}

pub fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

/// (kuna, `disassembly-fabricates-zero-byte`) The exclusive end of the
/// contiguous mapped run containing `vma`, and the next mapped address above it.
///
/// The load image answers a read that STARTS on mapped memory for its whole
/// length, zero-filling every byte past the last segment it crosses — the
/// upstream BFD contract (`LoadImageBfd::loadFill`, `loadimage_object.rs`). So
/// the start address is the only thing that path checks, and a listing that asks
/// for more bytes than the run holds gets the rest of them invented: `kuna
/// disassemble 0x80d190b` on an unmapped address correctly refuses, while
/// `kuna disassemble 0x80d18b0 --count 30` on the same image walked 23 bytes
/// past the same segment's end and reported eight `ADD byte ptr [EAX],AL` rows
/// that are not in the file. The bound has to be carried here instead: the start
/// is checked against the image, and the length is clipped to the run holding
/// it, so the windowed listing agrees with the direct query.
///
/// Adjacent and overlapping segments merge, because a listing that crosses from
/// one `PT_LOAD` into the next at the very byte the first ends has crossed
/// nothing — only a genuine hole stops it. `None` when the loader publishes no
/// segments at all (the XML `<binaryimage>` corpus, a relocatable object): that
/// is silence, not a bound, and the walk stands as it was.
pub fn mapped_run_end(prog: &ConsoleProgram, vma: u64) -> Option<(u64, Option<u64>)> {
    mapped_run(&prog.segments(), vma)
}

/// [`mapped_run_end`] over a segment list, so the merge is testable without a
/// loaded program.
pub fn mapped_run(segments: &[(u64, u64, u32)], vma: u64) -> Option<(u64, Option<u64>)> {
    let mut runs: Vec<(u64, u64)> = segments
        .iter()
        .filter(|&&(_, size, _)| size > 0)
        .map(|&(start, size, _)| (start, start.saturating_add(size)))
        .collect();
    runs.sort_unstable();
    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(runs.len());
    for (start, end) in runs {
        match merged.last_mut() {
            Some(last) if start <= last.1 => last.1 = last.1.max(end),
            _ => merged.push((start, end)),
        }
    }
    let hit = merged.iter().position(|&(start, end)| vma >= start && vma < end)?;
    Some((merged[hit].1, merged.get(hit + 1).map(|&(start, _)| start)))
}

/// Decode forward from `region.start` until a stop is reached: the region's end,
/// the instruction budget, the derived-length cap, or memory that will not read.
///
/// An address the translator rejects is listed as a `.byte` row rather than
/// ending the listing, because the common reason for one is a listing that ran
/// into inline data and there is usually code again after it. Where that code
/// can resume is [`resume_grid`]: one byte later on a variable-length
/// architecture, the next alignment boundary on a fixed-width one.
///
/// A word the listing's OWN instructions read at a fixed address is then folded
/// back into a data row rather than left decoded as the instruction its bytes
/// happen to spell — the literal pool an ARM/MIPS/PPC function carries inside
/// its own extent. What is folded, and what refuses a fold, is
/// [`crate::litpool`]; the fold never moves a row, so the listing's addresses
/// are the same either way.
///
/// Under `--follow` the straight line is not the only anchor: the walk also
/// starts at every address the range's own branches name, so a jump over a
/// decoy byte cannot pull the rest of the listing out of phase
/// ([`follow_rows`]).
///
/// Either way the branch targets no row starts at are reported
/// ([`skipped_targets`]) — the fact an agent reading an obfuscated listing
/// cannot infer, because a desynchronized decode looks exactly like an ordinary
/// one.
pub fn walk(
    prog: &ConsoleProgram,
    region: &Window,
    count: Option<usize>,
    follow: bool,
) -> CodeWalk {
    // What the straight line makes of this window is the baseline either way:
    // without `--follow` it IS the listing, and with it, its skipped targets are
    // the count of what following recovered.
    let (rows, truncated, refs) = decode_rows(prog, region, count, 0);
    let plain = skipped_targets(&rows, &refs);
    let (rows, truncated, refs, anchored) = if follow {
        let (rows, truncated, refs) = follow_rows(prog, region, count);
        (rows, truncated, refs, plain.len())
    } else {
        (rows, truncated, refs, 0)
    };
    // Before the fold, so the targets are tested against the boundaries the
    // decode actually produced rather than against a pool word's merged span.
    let skipped = if follow { skipped_targets(&rows, &refs) } else { plain };
    let (rows, folded) = fold_pool_words(prog, region, rows, &refs);
    CodeWalk { rows, truncated, folded, skipped, anchored }
}

/// What one code-view walk produced: the rows, whether it was cut short, how
/// many literal-pool words were folded, which branch targets it has no row for,
/// and how many of those the straight line would have walked over (0 without
/// `--follow`, where they are reported rather than decoded).
pub struct CodeWalk {
    pub rows: Vec<Row>,
    pub truncated: bool,
    pub folded: usize,
    pub skipped: Vec<u64>,
    pub anchored: usize,
}

/// (kuna, `linear-disassembly-silently-skips`) The addresses this listing's own
/// branches name that no row in it starts at.
///
/// Rows tile their span, so an in-range target no row starts at lies strictly
/// inside one — which means the instruction printed over it is a decode of bytes
/// the program never executes as that instruction, and so is everything after it
/// until the decode happens to re-synchronize. That is the whole `EB 01`
/// jump-over-a-decoy-byte idiom, and a straight-line listing of it is not merely
/// missing a row: it prints calls and jumps the bytes do not contain.
///
/// Only targets INSIDE the listed span are reported. A branch out of the range
/// is not evidence about the range, and the listing is not claiming to have
/// decoded where it points.
pub fn skipped_targets(rows: &[Row], evidence: &FixedRefs) -> Vec<u64> {
    let (Some(first), Some(last)) = (rows.first(), rows.last()) else {
        return Vec::new();
    };
    let span = first.addr..(last.addr + last.size);
    let starts: BTreeSet<u64> = rows.iter().map(|r| r.addr).collect();
    let mut out: Vec<u64> = evidence
        .flow_targets
        .iter()
        .copied()
        .filter(|t| span.contains(t) && !starts.contains(t))
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// `--follow`: the same window, decoded from every address its own branches name
/// rather than from one end only.
///
/// Two passes. The first is an ordinary recursive descent seeded at
/// `region.start`: each run steps forward while the instruction falls through
/// (`FixedRefs::falls_through`, the `xref_control_flow` last-op rule), stops at
/// an address some earlier run already claimed, and hands every in-window
/// address it branches or calls to back to the worklist. Seeds are taken in
/// address order, so a bounded walk spends its budget on the rows it is going to
/// print. Where two runs contest the same bytes the earlier claim stands, which
/// is what makes the pass terminate: no address is ever decoded twice.
///
/// The second pass fills what flow never reached — the decoy byte a jump was
/// over, the padding after a return, a handler nothing in this window calls — by
/// running the ordinary straight-line [`decode_rows`] across each gap, clipped so
/// that no instruction crosses into a claimed row. So `--follow` never lists
/// LESS than the straight line does: it is the same listing with every branch
/// target forced to start a row, and the bytes that then do not tile an
/// instruction spelled out as `.byte`.
///
/// Returns the rows, whether the listing was cut short, and the fixed-address
/// evidence from both passes.
fn follow_rows(
    prog: &ConsoleProgram,
    region: &Window,
    count: Option<usize>,
) -> (Vec<Row>, bool, FixedRefs) {
    let cap = if region.derived { Some(DERIVED_INSTRUCTION_CAP) } else { None };
    let stop = region.stop();
    let in_window = |a: u64| a >= region.start && stop.is_none_or(|end| a < end);
    let mut evidence = FixedRefs::default();
    let mut claimed: BTreeMap<u64, Row> = BTreeMap::new();
    // Address order, so a listing bounded by `--count` claims the low addresses
    // it will actually print before it spends the budget on a far callee.
    let mut seeds: BinaryHeap<Reverse<u64>> = BinaryHeap::new();
    seeds.push(Reverse(region.start));
    // A row can only be printed once, so the printed budget bounds the claims;
    // an explicit range that asked for neither is bounded by the window itself.
    let claim_budget = count.or(cap).unwrap_or(usize::MAX);
    let (mut mnem, mut body, mut raw) = (String::new(), String::new(), Vec::new());
    while let Some(Reverse(seed)) = seeds.pop() {
        if !in_window(seed) || claimed.contains_key(&seed) {
            continue;
        }
        let mut addr = seed;
        while in_window(addr) && !claimed.contains_key(&addr) && claimed.len() < claim_budget {
            // No instruction may reach past mapped memory or into bytes an
            // earlier run already spelled as an instruction. The window's own
            // end is NOT a ceiling: the straight line lists the instruction
            // straddling it whole, and `--follow` must not list less.
            let ceiling = claimed
                .range(addr.saturating_add(1)..)
                .next()
                .map(|(&a, _)| a)
                .into_iter()
                .chain(region.mapped_end)
                .min();
            let decoded = prog
                .disassemble_at_into(addr, &mut mnem, &mut body)
                .ok()
                .filter(|&n| n > 0)
                .filter(|&n| ceiling.is_none_or(|c| addr.saturating_add(n as u64) <= c));
            let Some(len) = decoded else { break };
            if !prog.read_bytes_into(addr, len as usize, &mut raw) {
                break;
            }
            let known = evidence.flow_targets.len();
            prog.add_fixed_refs_at(addr, &mut evidence);
            for &target in &evidence.flow_targets[known..] {
                if in_window(target) {
                    seeds.push(Reverse(target));
                }
            }
            claimed.insert(
                addr,
                Row {
                    addr,
                    size: len as u64,
                    bytes: raw.clone(),
                    mnemonic: mnem.clone(),
                    operands: body.clone(),
                },
            );
            if !evidence.falls_through {
                break;
            }
            addr = addr.saturating_add(len as u64);
        }
    }
    let (rows, truncated) = fill_between(prog, region, claimed, count, cap, &mut evidence);
    (rows, truncated, evidence)
}

/// Merge the claimed rows with a straight-line decode of everything between
/// them, in address order, until the row budget or the window runs out.
fn fill_between(
    prog: &ConsoleProgram,
    region: &Window,
    claimed: BTreeMap<u64, Row>,
    count: Option<usize>,
    cap: Option<usize>,
    evidence: &mut FixedRefs,
) -> (Vec<Row>, bool) {
    let stop = region.stop();
    // The row budget both bounds impose, whichever is tighter.
    let budget = match (count, cap) {
        (Some(asked), Some(derived)) => Some(asked.min(derived)),
        (asked, derived) => asked.or(derived),
    };
    // Every claimed row is alignment evidence for the gaps between them, which
    // have none of their own: a gap that opens on bytes the translator refuses
    // must still resume on the architecture's grid.
    let witness = claimed.values().fold(0u64, |w, r| w | r.addr | r.size);
    let mut out: Vec<Row> = Vec::new();
    let mut cursor = region.start;
    let mut claims = claimed.into_values().peekable();
    loop {
        let left = match budget {
            // Out of budget with something still to list: the answer is short by
            // the ask, not by the image.
            Some(n) if out.len() >= n => {
                return (out, claims.peek().is_some() || stop.is_none_or(|end| cursor < end));
            }
            Some(n) => Some(n - out.len()),
            // Unbounded only where the window has an end of its own, so the gap
            // decode below still stops.
            None => None,
        };
        match claims.peek() {
            Some(row) if row.addr <= cursor => {
                let row = claims.next().expect("peeked");
                cursor = cursor.max(row.addr.saturating_add(row.size));
                out.push(row);
            }
            next => {
                // The gap runs to the next claim, or to the end of the window.
                let gap_end = next.map(|r| r.addr).or(stop);
                if gap_end.is_some_and(|end| end <= cursor) {
                    return (out, false);
                }
                let gap = Window {
                    start: cursor,
                    end: gap_end,
                    derived: false,
                    // The clip: nothing decoded in a gap may reach into a
                    // claimed row or past mapped memory. The window's own end
                    // is not one, so the last row of a `--follow` listing is
                    // the same row the straight line ends on.
                    mapped_end: next.map(|r| r.addr).or(region.mapped_end),
                };
                let (filled, _, refs) = decode_rows(prog, &gap, left, witness);
                let Some(last) = filled.last() else {
                    return (out, false);
                };
                cursor = last.addr.saturating_add(last.size);
                evidence.reads.extend(refs.reads);
                evidence.flow_targets.extend(refs.flow_targets);
                out.extend(filled);
            }
        }
    }
}

/// The straight-line decode itself: rows in address order, whether the walk was
/// truncated, and the fixed-address evidence the rows carry.
///
/// `seed_witness` is the alignment evidence a caller already holds — the OR of
/// the addresses and sizes of rows it decoded elsewhere in this same listing.
/// A walk that starts on undecodable bytes has none of its own, and on a
/// fixed-width architecture that is the difference between resuming on the
/// instruction grid and resuming one byte in ([`resume_grid`]). `0` for a walk
/// that is the whole listing.
fn decode_rows(
    prog: &ConsoleProgram,
    region: &Window,
    count: Option<usize>,
    seed_witness: u64,
) -> (Vec<Row>, bool, FixedRefs) {
    let cap = if region.derived { Some(DERIVED_INSTRUCTION_CAP) } else { None };
    let stop = region.stop();
    let alignment = u64::try_from(prog.arch().translate().get_alignment()).unwrap_or(1);
    let mut rows: Vec<Row> = Vec::new();
    let mut evidence = FixedRefs::default();
    let mut truncated = false;
    let mut addr = region.start;
    // The alignment every decoded row so far shares, folded into one OR of
    // their addresses and sizes; `resume_grid` reads its low zero bits.
    let mut witness = seed_witness;
    let (mut mnem, mut body, mut raw) = (String::new(), String::new(), Vec::new());
    loop {
        if count.is_some_and(|n| rows.len() >= n) {
            break;
        }
        if stop.is_some_and(|end| addr >= end) {
            break;
        }
        if cap.is_some_and(|c| rows.len() >= c) {
            truncated = true;
            break;
        }
        // An instruction that runs off the end of mapped memory was decoded out
        // of the load image's zero fill, not out of the file, so it is not an
        // instruction: the mapped bytes under it are listed as `.byte` instead.
        let decoded = prog
            .disassemble_at_into(addr, &mut mnem, &mut body)
            .ok()
            .filter(|&n| n > 0)
            .filter(|&n| {
                !region.mapped_end.is_some_and(|m| addr.saturating_add(n as u64) > m)
            });
        // The bytes are read back separately, so a row is only reported as an
        // instruction when BOTH the decode and the read succeeded — a row can
        // never claim a length it cannot show the bytes for.
        match decoded {
            Some(len) if prog.read_bytes_into(addr, len as usize, &mut raw) => {
                prog.add_fixed_refs_at(addr, &mut evidence);
                witness |= addr | len as u64;
                rows.push(Row {
                    addr,
                    size: len as u64,
                    bytes: raw.clone(),
                    mnemonic: mnem.clone(),
                    operands: body.clone(),
                });
                addr = addr.saturating_add(len as u64);
            }
            _ => {
                let want = recovery_span(addr, resume_grid(alignment, witness), stop);
                // A span that will not read whole falls back to the one byte
                // that is always safe; a row never claims bytes it cannot show.
                if !prog.read_bytes_into(addr, want as usize, &mut raw)
                    && !prog.read_bytes_into(addr, 1, &mut raw)
                {
                    break;
                }
                rows.push(Row {
                    addr,
                    size: raw.len() as u64,
                    bytes: raw.clone(),
                    mnemonic: BAD_BYTE_MNEMONIC.to_string(),
                    operands: bad_byte_operand(&raw),
                });
                addr = addr.saturating_add(raw.len() as u64);
            }
        }
    }
    (rows, truncated, evidence)
}

/// The address grid a listing resumes on after bytes the translator refused.
///
/// One byte is the right answer on a variable-length architecture: any address
/// can start an instruction there, and inline data is usually followed by code
/// again. On an architecture whose instructions must be aligned it is the wrong
/// answer, because code can only resume on the grid — stepping one byte puts
/// every row after the data off it, which is how an ARM function's five-word
/// literal pool came back as one `.byte` and four invented instructions.
///
/// `alignment` is the architecture's declared minimum (SLEIGH `define
/// alignment`); `1` means nothing is aligned and the byte-at-a-time recovery
/// stands. Above that the grid is the alignment the rows already decoded all
/// share — `witness`, the OR of each decoded row's address and size — so an ARM
/// listing of 4-byte rows resumes on 4 and a Thumb listing that has decoded a
/// 2-byte row resumes on 2. With nothing decoded yet there is no grid to infer
/// and the walk steps one byte.
fn resume_grid(alignment: u64, witness: u64) -> u64 {
    if alignment <= 1 || witness == 0 {
        return 1;
    }
    (1u64 << witness.trailing_zeros().min(u64::BITS - 1)).min(MAX_RESUME_GRID)
}

/// How many bytes one recovery row covers: forward to the next `grid`
/// boundary — a whole grid step when `addr` is already on one, since an
/// undecodable slot on an aligned architecture is one instruction's worth of
/// bytes — clipped to the region's end and never zero.
fn recovery_span(addr: u64, grid: u64, end: Option<u64>) -> u64 {
    let span = if grid > 1 { grid - addr % grid } else { 1 };
    match end {
        Some(end) => span.min(end.saturating_sub(addr)).max(1),
        None => span,
    }
}

/// The operand of a `.byte` row: every byte it covers, comma-separated in the
/// spelling an assembler takes back.
fn bad_byte_operand(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("0x{b:02x}")).collect::<Vec<_>>().join(",")
}

/// Replace each proved literal-pool word with one data row.
///
/// The rows a word covers are folded into a single `.word 0x...` row over the
/// same bytes, so the listing's addresses are untouched — [`crate::litpool`]
/// only proves a word whose width tiles whole decoded rows, which is what makes
/// that true.
fn fold_pool_words(
    prog: &ConsoleProgram,
    region: &Window,
    rows: Vec<Row>,
    evidence: &FixedRefs,
) -> (Vec<Row>, usize) {
    let Some(&Row { addr: first, .. }) = rows.first() else {
        return (rows, 0);
    };
    let last = rows.last().map_or(first, |r| r.addr + r.size);
    // A word only qualifies where the program says data can live and does not
    // say code does: a mapped non-writable section (a GOT slot is read by
    // address too, and a writable `.text` is a packer), and no function symbol
    // installed at that very address. The extent clip already keeps a *named*
    // target's listing off the next function, but an explicit multi-function
    // range walks straight through entries and must not eat one.
    let sections = prog.sections();
    let is_pool_slot = |vma: u64| {
        sections
            .iter()
            .find(|(start, size, _)| vma >= *start && vma - start < *size)
            .is_some_and(|(_, _, flags)| flags & section_flags::READONLY != 0)
            && prog.function_named_at(vma).is_none()
    };
    let boundaries: Vec<crate::litpool::Boundary> = rows.iter().map(|r| (r.addr, r.size)).collect();
    let pool = crate::litpool::pool_words(
        &boundaries,
        &evidence.reads,
        &evidence.flow_targets,
        (region.start.max(first), last),
        &is_pool_slot,
    );
    if pool.is_empty() {
        return (rows, 0);
    }
    let big_endian = prog.arch().translate().is_big_endian();
    let mut out: Vec<Row> = Vec::with_capacity(rows.len());
    let mut folded = 0;
    let mut it = rows.into_iter();
    while let Some(row) = it.next() {
        let Some(&width) = pool.get(&row.addr) else {
            out.push(row);
            continue;
        };
        let (addr, mut bytes) = (row.addr, row.bytes);
        while (bytes.len() as u64) < width {
            match it.next() {
                Some(next) => bytes.extend_from_slice(&next.bytes),
                None => break,
            }
        }
        out.push(Row {
            addr,
            size: bytes.len() as u64,
            mnemonic: crate::litpool::word_mnemonic(width).to_string(),
            operands: crate::litpool::word_operand(&bytes, big_endian),
            bytes,
        });
        folded += 1;
    }
    (out, folded)
}

/// Read at most `want` bytes at `vma`, shortening at the first byte the image
/// will not hand over — a mapped segment ends where it ends, which is far more
/// often mid-row than on a 16-byte boundary. Returns how many were read.
pub fn read_upto(prog: &ConsoleProgram, vma: u64, want: usize, out: &mut Vec<u8>) -> usize {
    if want == 0 {
        return 0;
    }
    if prog.read_bytes_into(vma, want, out) {
        return want;
    }
    let mut one: Vec<u8> = Vec::new();
    let mut got: Vec<u8> = Vec::with_capacity(want);
    for i in 0..want {
        if !prog.read_bytes_into(vma.saturating_add(i as u64), 1, &mut one) {
            break;
        }
        got.push(one[0]);
    }
    *out = got;
    out.len()
}

#[cfg(test)]
mod tests;
