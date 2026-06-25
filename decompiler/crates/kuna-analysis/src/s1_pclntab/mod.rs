//! Go `pclntab` function-name recovery — the kuna analog of Ghidra's
//! `GolangSymbolAnalyzer` (the name-recovery half).
//!
//! A Go binary embeds a `pclntab` (the "program-counter → line/name" table) so the
//! runtime can produce stack traces; because the runtime needs it, it survives
//! stripping. Without parsing it, every Go function renders as `sub_<addr>`. This
//! pass locates the table, decodes each function's entry PC and name, and emits a
//! [`SymFact`] `{ addr: func_entry, name, kind: Function }` per function — so
//! `main.main`, `runtime.*`, and package functions render named.
//!
//! Origin (upstream Ghidra, the tree kuna was ported from):
//! - header:   `Ghidra/Features/Base/.../golang/rtti/GoPcHeader.java`
//!   (the four magics `GO_1_2/1_16/1_18/1_20`, `isPcHeader` validation, the
//!   `*Offset`-relative-to-header layout, `readMagic`'s LE-then-BE endian probe).
//! - moduledata: `.../golang/rtti/GoModuledata.java` (the funcnametab/ftab/pclntable
//!   slices; `getText()`, `getFuncDataInstance(off)` = pclntable + funcoff;
//!   `getFunctabEntriesSlice` chops the trailing sentinel entry).
//! - functab:  `.../golang/rtti/GoFunctabEntry.java` (1.18+ `entryoff:u32` relative
//!   to text + `funcoff:u32`; ≤1.17 `entry:uintptr` absolute + `funcoff:uintptr`).
//! - `_func`:  `.../golang/rtti/GoFuncData.java` (`getName` reads the
//!   NUL-terminated UTF-8 string at `funcnametab + nameoff`; for ≤1.15 the name
//!   table is the pclntable blob itself).
//! - analyzer: `.../app/plugin/core/analysis/GolangSymbolAnalyzer.java`
//!   (`markupGoFunctions`: funcAddr → recovered name → `createFunctionIfMissing`).
//!
//! The byte-level layouts here are also cross-checked against the Go runtime
//! source (`runtime/symtab.go` `pcHeader`/`functab`, `runtime/runtime2.go`
//! `_func`) at go1.18.
//!
//! # Faithfulness / scope
//!
//! This is the **name-recovery** half only — the entry-PC→name mapping the
//! decompiler needs. Ghidra's `GolangSymbolAnalyzer` does much more (RTTI types,
//! method definitions, recovered signatures, source-file maps, no-return flags).
//! The Go no-return list is already covered by the (separate) known-no-return pass
//! (its `Compiler::Go` arm). Names that obviously are not functions
//! (`go:buildid` / `go.buildid` sentinels) are skipped, mirroring
//! `GolangSymbolAnalyzer.FUNCNAMES_TO_IGNORE`.
//!
//! Defensive by construction (the [`AnalysisPass`] never-fail contract): an
//! unknown magic, a malformed header, an out-of-range offset, or a missing table
//! all yield an **empty** output — never a panic, never an error.

use object::read::{Object, ObjectSection, ObjectSymbol};

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Stage, SymFact, SymKind};

/// The four `pcHeader` magics Ghidra recognizes (`GoPcHeader.java:45-48`), each
/// keyed to the layout family it selects.
const MAGIC_GO_1_2: u32 = 0xffff_fffb;
const MAGIC_GO_1_16: u32 = 0xffff_fffa;
const MAGIC_GO_1_18: u32 = 0xffff_fff0;
const MAGIC_GO_1_20: u32 = 0xffff_fff1;

/// Names the analyzer skips (not real functions) —
/// `GolangSymbolAnalyzer.FUNCNAMES_TO_IGNORE`.
const FUNCNAMES_TO_IGNORE: [&str; 2] = ["go:buildid", "go.buildid"];

/// The decoded pclntab layout family (selected by the header magic). Newer Go
/// minor versions reuse the `1.20` magic (`0xfffffff1`); Ghidra treats them all as
/// the 1.20 family, and so do we (the name-recovery layout is identical to 1.18).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum GoVer {
    /// go1.2 … go1.15 — the legacy direct-functab layout (no separate
    /// `funcnametab`; names live in the pclntab blob).
    Go12,
    /// go1.16 / go1.17 — `funcnametab`/`pctab`/… offsets, absolute `entry` in the
    /// functab, no `textStart` in the header.
    Go116,
    /// go1.18 / go1.19 (and go1.20+ via the shared magic) — 32-bit PC-relative
    /// `entryoff` from `textStart`, the header carries `textStart`.
    Go118,
}

/// `true` when the layout uses the 1.18+ shape: a `textStart` field in the header
/// and 32-bit PC-relative `entryoff` functab entries.
impl GoVer {
    fn is_118plus(self) -> bool {
        matches!(self, GoVer::Go118)
    }
}

/// The endianness recovered from the magic probe (LE first, then BE — faithful to
/// `GoPcHeader.readMagic`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum End {
    Little,
    Big,
}

/// A view over the loaded image's bytes addressable by virtual address: the
/// loadable sections, sorted, so a `read(vma, len)` resolves through whichever
/// section covers `[vma, vma+len)`. This is the kuna analog of reading program
/// memory through Ghidra's `BinaryReader`/`MemoryByteProvider`.
struct ImageBytes<'a> {
    /// `(vma, bytes)` for each section with file-backed data, sorted by vma.
    sections: Vec<(u64, &'a [u8])>,
}

impl<'a> ImageBytes<'a> {
    /// Build the address-resolvable byte view from a parsed object. Only sections
    /// with a nonzero address and readable data are included (BSS / `.tbss` carry
    /// no bytes; the pclntab is always in an initialized section).
    fn new(file: &'a object::File<'a>) -> Self {
        let mut sections: Vec<(u64, &[u8])> = Vec::new();
        for sec in file.sections() {
            let addr = sec.address();
            if addr == 0 {
                continue;
            }
            if let Ok(data) = sec.data() {
                if !data.is_empty() {
                    sections.push((addr, data));
                }
            }
        }
        sections.sort_by_key(|&(a, _)| a);
        ImageBytes { sections }
    }

    /// Return the `len` bytes at virtual address `vma`, or `None` if no single
    /// loaded section covers the whole span. (The pclntab and its referenced
    /// tables are contiguous within one section, so a single-section read is
    /// sufficient and avoids stitching across gaps.)
    fn read(&self, vma: u64, len: usize) -> Option<&'a [u8]> {
        for &(base, data) in &self.sections {
            if vma < base {
                continue;
            }
            let off = (vma - base) as usize;
            if let Some(end) = off.checked_add(len) {
                if end <= data.len() {
                    return Some(&data[off..end]);
                }
            }
        }
        None
    }

    /// Build an [`ImageBytes`] from explicit `(vma, bytes)` spans (test-only): lets
    /// the parser primitives be exercised over hand-constructed pclntab layouts
    /// without synthesizing a whole ELF.
    #[cfg(test)]
    fn from_spans(mut sections: Vec<(u64, &'a [u8])>) -> Self {
        sections.sort_by_key(|&(a, _)| a);
        ImageBytes { sections }
    }

    /// Read a NUL-terminated string starting at `vma`, capped at `max` bytes.
    /// Returns the decoded UTF-8 string (lossless — Go names are ASCII/UTF-8) or
    /// `None` if unmapped / no terminator within the cap.
    fn read_cstr(&self, vma: u64, max: usize) -> Option<String> {
        for &(base, data) in &self.sections {
            if vma < base {
                continue;
            }
            let off = (vma - base) as usize;
            if off >= data.len() {
                continue;
            }
            let slice = &data[off..];
            let cap = slice.len().min(max);
            if let Some(nul) = slice[..cap].iter().position(|&b| b == 0) {
                return std::str::from_utf8(&slice[..nul]).ok().map(str::to_string);
            }
            return None;
        }
        None
    }
}

/// Read a little/big-endian `u32` from the front of `b` (caller guarantees
/// `b.len() >= 4`).
fn rd_u32(b: &[u8], end: End) -> u32 {
    let a = [b[0], b[1], b[2], b[3]];
    match end {
        End::Little => u32::from_le_bytes(a),
        End::Big => u32::from_be_bytes(a),
    }
}

/// Read a little/big-endian `u64` from the front of `b` (caller guarantees
/// `b.len() >= 8`).
fn rd_u64(b: &[u8], end: End) -> u64 {
    let mut a = [0u8; 8];
    a.copy_from_slice(&b[..8]);
    match end {
        End::Little => u64::from_le_bytes(a),
        End::Big => u64::from_be_bytes(a),
    }
}

/// Read a `ptrSize`-wide unsigned word (uintptr) from the front of `b`.
fn rd_ptr(b: &[u8], ptr_size: u8, end: End) -> u64 {
    if ptr_size == 8 {
        rd_u64(b, end)
    } else {
        rd_u32(b, end) as u64
    }
}

/// Map a 32-bit magic to its [`GoVer`] layout family, or `None` if unknown.
fn magic_to_ver(magic: u32) -> Option<GoVer> {
    match magic {
        MAGIC_GO_1_2 => Some(GoVer::Go12),
        MAGIC_GO_1_16 => Some(GoVer::Go116),
        // 1.18 and 1.20 (and the 1.19/1.21+ that share the 1.20 magic) use the
        // same name-recovery layout.
        MAGIC_GO_1_18 | MAGIC_GO_1_20 => Some(GoVer::Go118),
        _ => None,
    }
}

/// Probe the 8-byte header prefix for a valid `pcHeader`, returning the
/// `(version, endianness, minLC, ptrSize)` on success. Faithful to
/// `GoPcHeader.isPcHeader` + `readMagic`: pad bytes 0, `minLC ∈ {1,2,4}`,
/// `ptrSize ∈ {4,8}`, magic matches a known version read LE-then-BE.
fn probe_header(hdr: &[u8]) -> Option<(GoVer, End, u8, u8)> {
    if hdr.len() < 8 {
        return None;
    }
    // pad1/pad2 must be zero; minLC in {1,2,4}; ptrSize in {4,8}.
    if hdr[4] != 0 || hdr[5] != 0 {
        return None;
    }
    let min_lc = hdr[6];
    let ptr_size = hdr[7];
    if !matches!(min_lc, 1 | 2 | 4) || !matches!(ptr_size, 4 | 8) {
        return None;
    }
    // Magic: try little-endian first, then big-endian (readMagic order).
    if let Some(ver) = magic_to_ver(rd_u32(hdr, End::Little)) {
        return Some((ver, End::Little, min_lc, ptr_size));
    }
    if let Some(ver) = magic_to_ver(rd_u32(hdr, End::Big)) {
        return Some((ver, End::Big, min_lc, ptr_size));
    }
    None
}

/// The located + validated pclntab header: where it is (`header_vma`), what it
/// looks like, and the resolved table bases (all already converted to virtual
/// addresses).
#[derive(Clone, Copy, Debug)]
struct PcHeader {
    ver: GoVer,
    end: End,
    ptr_size: u8,
    /// Base PC for 1.18+ `entryoff` resolution (header `textStart`); 0 / unused on
    /// older layouts (which carry absolute `entry`s).
    text_start: u64,
    /// VMA of the `funcnametab` blob (1.16+); for go1.2 this is the pclntab blob
    /// itself (names are read from the pclntab body).
    funcname_vma: u64,
    /// VMA of the functab (`ftab`/`pcln`) — the array of `(entry, funcoff)` pairs.
    functab_vma: u64,
    /// Number of functions (`nfunc` in the header for 1.16+; for go1.2 it is the
    /// first word after the 8-byte header).
    nfunc: u64,
    /// VMA the `_func` structs are addressed against: `pclntab_base + funcoff`. For
    /// 1.16+ this is `header_vma + pclnOffset`; for go1.2 it is the pclntab blob
    /// base (`header_vma`).
    funcdata_base: u64,
}

/// Parse the header at `header_vma` into a [`PcHeader`], resolving the table
/// bases. `None` on any malformation (the never-fail contract — the caller turns
/// `None` into an empty output).
fn parse_header(img: &ImageBytes, header_vma: u64) -> Option<PcHeader> {
    let hdr8 = img.read(header_vma, 8)?;
    let (ver, end, _min_lc, ptr_size) = probe_header(hdr8)?;
    let ps = ptr_size as usize;

    if ver == GoVer::Go12 {
        // Legacy (go1.2 … go1.15): 8-byte header, then `nfunc` (uintptr) at +8,
        // then the functab `(entry, funcoff)` pairs begin at +8+ptrSize. The name
        // table is the pclntab blob itself, indexed by `nameoff` from each `_func`.
        let nfunc_bytes = img.read(header_vma + 8, ps)?;
        let nfunc = rd_ptr(nfunc_bytes, ptr_size, end);
        let functab_vma = header_vma + 8 + ptr_size as u64;
        return Some(PcHeader {
            ver,
            end,
            ptr_size,
            text_start: 0,
            funcname_vma: header_vma,
            functab_vma,
            nfunc,
            funcdata_base: header_vma,
        });
    }

    // 1.16+ header (GoPcHeader.java field order). After the 8 fixed bytes:
    //   nfunc (intptr), nfiles (intptr),
    //   [textStart (uintptr) — 1.18+ only],
    //   funcnameOffset, cuOffset, filetabOffset, pctabOffset, pclnOffset (uintptr).
    // All `*Offset`s are relative to the header start.
    let mut cur = header_vma + 8;
    let nfunc = rd_ptr(img.read(cur, ps)?, ptr_size, end);
    cur += ptr_size as u64; // nfunc
    cur += ptr_size as u64; // nfiles (skip)

    let text_start = if ver.is_118plus() {
        let t = rd_ptr(img.read(cur, ps)?, ptr_size, end);
        cur += ptr_size as u64; // textStart
        t
    } else {
        0
    };

    let funcname_off = rd_ptr(img.read(cur, ps)?, ptr_size, end);
    cur += ptr_size as u64; // funcnameOffset
    cur += ptr_size as u64; // cuOffset (skip)
    cur += ptr_size as u64; // filetabOffset (skip)
    cur += ptr_size as u64; // pctabOffset (skip)
    let pcln_off = rd_ptr(img.read(cur, ps)?, ptr_size, end); // pclnOffset

    let funcname_vma = header_vma.checked_add(funcname_off)?;
    let functab_vma = header_vma.checked_add(pcln_off)?;
    // The `_func` structs (pointed at by each functab `funcoff`) live in the same
    // pclntab blob the functab is part of: `pclntable.array + funcoff`. The
    // functab IS the head of that blob, so the funcdata base is the functab base.
    let funcdata_base = functab_vma;

    Some(PcHeader {
        ver,
        end,
        ptr_size,
        text_start,
        funcname_vma,
        functab_vma,
        nfunc,
        funcdata_base,
    })
}

/// One recovered Go function: its entry PC and its (already NUL-stripped) name.
#[derive(Clone, Debug, PartialEq, Eq)]
struct GoFunc {
    addr: u64,
    name: String,
}

/// Decode the function table into `(entry_pc, name)` pairs. Walks the `nfunc`
/// functab entries (Ghidra drops the trailing sentinel; we bound the loop by
/// `nfunc`, which already excludes it), follows each `funcoff` to its `_func`,
/// reads `nameoff`, and pulls the NUL-terminated name from the name table.
///
/// Caps `nfunc` defensively (a corrupt header could claim millions of functions);
/// a malformed entry is skipped, not fatal.
fn decode_functions(img: &ImageBytes, h: &PcHeader) -> Vec<GoFunc> {
    let mut out = Vec::new();
    // Hard cap: refuse to spin on an absurd nfunc (the real limit is the section
    // size; this bound is generous yet keeps a corrupt value from hanging).
    let nfunc = h.nfunc.min(2_000_000) as usize;

    // Functab entry stride and field widths differ by version:
    //   1.18+: (entryoff: u32, funcoff: u32)   stride 8
    //   ≤1.17: (entry: uintptr, funcoff: uintptr) stride 2*ptrSize
    let (entry_w, stride): (usize, usize) = if h.ver.is_118plus() {
        (4, 8)
    } else {
        (h.ptr_size as usize, 2 * h.ptr_size as usize)
    };

    for i in 0..nfunc {
        let ent_vma = match h.functab_vma.checked_add((i * stride) as u64) {
            Some(v) => v,
            None => break,
        };
        let Some(ent) = img.read(ent_vma, stride) else { break };

        // Field 0: entry (absolute, ≤1.17) or entryoff (relative to textStart, 1.18+).
        let (func_pc, funcoff) = if h.ver.is_118plus() {
            let entryoff = rd_u32(ent, h.end);
            // 0xffffffff marks an inline/sentinel entry — skip (GoFuncData.isInline).
            if entryoff == u32::MAX {
                continue;
            }
            let funcoff = rd_u32(&ent[entry_w..], h.end) as u64;
            (h.text_start.wrapping_add(entryoff as u64), funcoff)
        } else {
            let entry = rd_ptr(ent, h.ptr_size, h.end);
            let funcoff = rd_ptr(&ent[entry_w..], h.ptr_size, h.end);
            (entry, funcoff)
        };

        // Follow funcoff to the `_func`. nameoff is the 2nd field (after the
        // entry field, whose width matches the functab entry field width):
        //   1.18+: entryoff(u32) @0, nameoff(i32) @4
        //   ≤1.17: entry(uintptr) @0, nameoff(i32) @ptrSize
        let func_vma = match h.funcdata_base.checked_add(funcoff) {
            Some(v) => v,
            None => continue,
        };
        let name_field_off = if h.ver.is_118plus() { 4 } else { h.ptr_size as usize };
        let Some(nb) = img.read(func_vma + name_field_off as u64, 4) else { continue };
        // nameoff is i32; a negative value is invalid here (no name).
        let nameoff = rd_u32(nb, h.end);
        if nameoff == u32::MAX {
            continue;
        }

        let name_vma = match h.funcname_vma.checked_add(nameoff as u64) {
            Some(v) => v,
            None => continue,
        };
        // Names are bounded; 4096 is far beyond any real Go symbol.
        let Some(name) = img.read_cstr(name_vma, 4096) else { continue };
        if name.is_empty() || FUNCNAMES_TO_IGNORE.contains(&name.as_str()) {
            continue;
        }
        out.push(GoFunc { addr: func_pc, name });
    }
    out
}

/// Locate the pclntab header VMA. In precedence order (faithful to
/// `GoPcHeader.getPcHeaderAddress` + `GoModuledata.getFirstModuledata`, plus a
/// scan fallback for stripped binaries):
/// 1. the `.gopclntab` (or `gopclntab`) section start;
/// 2. the `runtime.pclntab` / `runtime.firstmoduledata`-adjacent symbol — kuna
///    resolves the `runtime.pclntab` symbol directly when present;
/// 3. a byte scan of the read-only data sections for the validated header magic.
fn locate_header(file: &object::File, img: &ImageBytes) -> Option<u64> {
    // (1) The named section. Go's ELF uses `.gopclntab`; some toolchains/PE use
    // `gopclntab` (no dot).
    for name in [".gopclntab", "gopclntab"] {
        if let Some(sec) = file.section_by_name(name) {
            let vma = sec.address();
            if let Some(hdr) = img.read(vma, 8) {
                if probe_header(hdr).is_some() {
                    return Some(vma);
                }
            }
        }
    }

    // (2) The `runtime.pclntab` symbol (the header is at its address). Some
    // binaries carry it even without a `.gopclntab` section.
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if let Ok(n) = sym.name() {
            if n == "runtime.pclntab" {
                let vma = sym.address();
                if let Some(hdr) = img.read(vma, 8) {
                    if probe_header(hdr).is_some() {
                        return Some(vma);
                    }
                }
            }
        }
    }

    // (3) Scan the read-only data sections for the header magic + valid prefix
    // (the kuna analog of `findPcHeaderAddress` for stripped binaries). Restrict
    // to the sections Go places the pclntab in (`.rodata`, `.data.rel.ro`,
    // `.noptrdata`) to keep the scan bounded; the validation predicate
    // (`probe_header`) rejects coincidental magic-shaped bytes.
    for name in [".rodata", ".data.rel.ro", ".noptrdata", ".data"] {
        if let Some(sec) = file.section_by_name(name) {
            let base = sec.address();
            let Ok(data) = sec.data() else { continue };
            // The header is 8-byte-aligned in practice; step by 4 to be safe
            // (cheap on a few-MB section) and validate each candidate.
            let mut off = 0usize;
            while off + 8 <= data.len() {
                if probe_header(&data[off..off + 8]).is_some() {
                    return Some(base + off as u64);
                }
                off += 8;
            }
        }
    }
    None
}

/// Parse a Go binary's pclntab and return the recovered `(entry_pc, name)`
/// functions. Pure and never-failing: any malformation yields an empty vec. This
/// is the hermetically testable core (the [`AnalysisPass`] just wraps it).
fn recover_go_functions(file: &object::File) -> Vec<GoFunc> {
    let img = ImageBytes::new(file);
    let Some(header_vma) = locate_header(file, &img) else {
        return Vec::new();
    };
    let Some(h) = parse_header(&img, header_vma) else {
        return Vec::new();
    };
    decode_functions(&img, &h)
}

/// Go pclntab function-name recovery pass. Gated on the source language being Go
/// (registered only for `Compiler::Go` in `passes_for`); a strict no-op on every
/// other binary. Emits a [`SymFact`] `{ kind: Function }` per recovered function,
/// committed by the existing symbol arm (which no-ops idempotently if a real
/// `.symtab` name already occupies the address — so a non-stripped Go binary's
/// real symbols still win, and only a *stripped* binary's `sub_<addr>` functions
/// get the recovered name).
#[derive(Clone, Copy, Default)]
pub struct GoPclntabPass;

impl AnalysisPass for GoPclntabPass {
    fn stage(&self) -> Stage {
        Stage::S1
    }

    fn id(&self) -> &'static str {
        "gopclntab"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        // ELF-only (the only format kuna loads); a non-ELF view yields nothing.
        if !matches!(ctx.file.format(), object::BinaryFormat::Elf) {
            return out;
        }
        for f in recover_go_functions(ctx.file) {
            out.symbols.push(SymFact { addr: f.addr, name: f.name, kind: SymKind::Function });
        }
        out
    }
}

#[cfg(test)]
mod tests;
