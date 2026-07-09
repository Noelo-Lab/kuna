//! The **full upstream byte-pattern function-start set** — the faithful port of
//! Ghidra's `FunctionStartAnalyzer` byte matcher over the vendored
//! `Ghidra/Processors/<P>/data/patterns/*.xml` `<patternlist>`/`<patternpairs>`.
//!
//! This is the engine behind oracle 5's *full* mode (the
//! [`FuncStartPatternPass`](crate::entry::FuncStartPatternPass), gated
//! default-**off** by `--option funcstart_patterns on`). The existing oracle 5
//! (`entry::prologue_pattern_starts`) vendors a hand-written **minimal** set of
//! three bare x86-64 prologues; this module vendors and parses the **entire**
//! upstream pattern corpus for x86/x86-64 (and AArch64/ARM/RISC-V/MIPS/PPC) and
//! applies it with the upstream pre/post matching semantics, so a stripped binary
//! recovers many more function starts.
//!
//! ## What is ported (and what is a documented LOSS)
//!
//! Ghidra's pattern XML has two faithfully-byte-matchable shapes and several that
//! need a `PseudoDisassembler` / `Listing` (which the analyzer tier does not have,
//! the same wall `entry` documents for the `after="defined"`/`validcode` rules):
//!
//! - **`<patternpairs>`** (ported) — a set of `<prepatterns>` DittedBitSequences
//!   and a set of `<postpatterns>` whose terminal action is a bare `<funcstart/>`.
//!   Faithful to `PatternPairSet.createFinalPatterns`
//!   (`Ghidra/Features/Base/.../bytesearch/PatternPairSet.java:73`): each post
//!   pattern is concatenated *after* each pre sequence into one ditted pattern,
//!   with the **mark** at `prepattern.getSize()` — i.e. a candidate is a function
//!   start iff a **prepattern matches the bytes immediately before it** AND a
//!   **postpattern matches at it**. This is pure byte matching (no disassembler),
//!   the high-value half: the prepattern context (after a RET/JMP/NOP/…) makes the
//!   discovery both broader (every prologue shape) and self-gating (the prepattern
//!   gate), exactly as Ghidra intends. We port ONLY the `<patternpairs>` whose
//!   post action is `<funcstart/>` (not `<possiblefuncstart/>`, which Ghidra treats
//!   as a weaker hint requiring later confirmation, nor `<codeboundary/>`-only
//!   blocks, which mark code but not a function start).
//! - **bare `<pattern>` with a bare `<funcstart/>`** (ported) — a single
//!   DittedBitSequence whose only post action is an unconditional `<funcstart/>`
//!   (no `after`/`validcode`/`thunk`/`label`). These are rare but real; the
//!   x86 `get_pc_thunk` shapes carry `validcode`/`label`, so they are *excluded*.
//! - bare `<pattern>` with `after="defined"` / `validcode="N"` / `<possiblefuncstart/>`
//!   / `thunk` / `label` / `section` (LOSS) — the `after`/`validcode` post-rules
//!   need a `PseudoDisassembler` to check "is there defined code/data immediately
//!   before?" or "do N valid instructions disassemble here?"; `thunk`/`label`
//!   create a thunk/symbol; `<possiblefuncstart/>` is a weaker hint. None are
//!   byte-decidable at this tier, so they are dropped (the same documented LOSS
//!   `entry`'s module docs record). We keep the `<funcstart/>` post action of a
//!   `<patternpairs>` even though Ghidra's `FunctionStartAction` *also* runs the
//!   `checkPreRequisites` Listing checks at apply time, because for a
//!   `<patternpairs>` `<funcstart/>` with no `after`/`validcode` attributes those
//!   checks are vacuous (`afterName == null`, `validCodeMin == 0`) — the only
//!   gates are the prepattern (byte) and the alignment, both of which we apply.
//!
//! ## Provenance
//!
//! The XMLs under `entry/patterns/*.xml` are vendored **verbatim** from
//! upstream Ghidra (`GHIDRA_REV` in `docs/UPSTREAM.md`,
//! `cef869af04c4740a71ad31a55704045b1b0d1644`), copied from
//! `Ghidra/Processors/{x86,AARCH64,ARM,RISCV,MIPS,PowerPC}/data/patterns/`. They
//! are embedded via `include_str!` (no runtime file IO; the analyzer tier has no
//! spec-root handle). The parser is the faithful port of
//! `DittedBitSequence.initFromDittedStringData`
//! (`DittedBitSequence.java:365`, hex/binary ditted tokens) +
//! `PatternPairSet.restoreXml` (`PatternPairSet.java:109`).

use std::sync::OnceLock;

// ===========================================================================
// DittedBitSequence — the byte matcher (full hex+binary ditted parser)
// ===========================================================================

/// A ditted bit sequence: `bits[i]` is the required value (already masked by
/// `dits[i]`), `dits[i]` the care-mask. `isMatch(pos,val) == (val & dits) == bits`
/// (faithful to `DittedBitSequence.isMatch`, `DittedBitSequence.java:218`).
#[derive(Clone, Debug)]
pub struct Ditted {
    bits: Vec<u8>,
    dits: Vec<u8>,
    /// Byte offset of the `*`-marked function-start byte within this sequence (the
    /// `markOffset` of `DittedBitSequence.initFromDittedStringData`). `0` when the
    /// `<data>` carries no `*` (the whole sequence is the prologue). For a
    /// `<pattern>` like `0x909090 * 0x4883ec`, `mark == 3`: the bytes before the
    /// mark (the 3 NOPs) are *context* that must precede the candidate, and the
    /// candidate is the byte at `mark`.
    mark: usize,
}

impl Ditted {
    /// Parse a ditted `<data>` string mixing hex (`0x..`) and binary (`01.0....`)
    /// tokens. Faithful port of `DittedBitSequence.initFromDittedStringData`
    /// (`DittedBitSequence.java:365`):
    ///   - whitespace resets the token mode (a new token starts),
    ///   - `0x` enters hex mode: each following nibble pair (or `.` don't-care
    ///     nibble) → one byte, masks per nibble; consecutive hex chars (no space)
    ///     continue the same byte run,
    ///   - a leading `0`/`1`/`.` enters binary mode: the next 8 chars → one byte,
    ///   - `#` skips to end-of-line (comment),
    ///   - `*` sets a mark offset (unused here — `<patternpairs>` mark comes from
    ///     `prepattern.getSize()`; we ignore it).
    ///
    /// Returns `None` on a malformed sequence (Ghidra throws; we drop the pattern).
    pub fn parse(text: &str) -> Option<Ditted> {
        let chars: Vec<char> = text.chars().collect();
        let mut bits: Vec<u8> = Vec::new();
        let mut dits: Vec<u8> = Vec::new();
        let mut mark: usize = 0;
        // mode: -1 looking for token start, -2 skip-to-EOL, 0 hex, 1 binary.
        let mut mode: i32 = -1;
        let mut i = 0usize;
        while i < chars.len() {
            let c1 = chars[i];
            if mode == -2 {
                if c1 != '\n' {
                    i += 1;
                    continue;
                }
                // fallthrough to whitespace handling resets mode
            }
            if c1.is_whitespace() {
                mode = -1;
                i += 1;
                continue;
            }
            if c1 == '#' {
                mode = -2;
                i += 1;
                continue;
            }
            if mode == -1 {
                if c1 == '0' && i + 1 < chars.len() && chars[i + 1] == 'x' {
                    mode = 0;
                    i += 2;
                    continue;
                } else if c1 == '*' {
                    // Mark the function-start byte at the current byte count
                    // (`markOffset = ditarray.size()`, DittedBitSequence.java:399).
                    mark = bits.len();
                    i += 1;
                    continue;
                } else if c1 == '0' || c1 == '1' || c1 == '.' {
                    mode = 1;
                } else {
                    return None; // bad ditted bit sequence
                }
            }
            if mode == 0 {
                // Hex: need two nibble chars.
                if i + 1 >= chars.len() {
                    return None;
                }
                let c2 = chars[i + 1];
                i += 2;
                let mut val: u8 = 0;
                let mut mask: u8 = 0xff;
                if c1 == '.' {
                    mask ^= 0xf0;
                } else {
                    val = (hex_val(c1)? << 4) as u8;
                }
                if c2 == '.' {
                    mask ^= 0x0f;
                } else {
                    val |= hex_val(c2)? as u8;
                }
                bits.push(val);
                dits.push(mask);
            } else {
                // Binary: 8 chars.
                if i + 8 > chars.len() {
                    return None;
                }
                let mut val: u8 = 0;
                let mut mask: u8 = 0;
                for j in 0..8 {
                    let cc = chars[i + j];
                    match cc {
                        '0' => {
                            val <<= 1;
                            mask = (mask << 1) | 1;
                        }
                        '.' => {
                            val <<= 1;
                            mask <<= 1;
                        }
                        '1' => {
                            val = (val << 1) | 1;
                            mask = (mask << 1) | 1;
                        }
                        _ => return None,
                    }
                }
                i += 8;
                bits.push(val);
                dits.push(mask);
            }
        }
        if bits.is_empty() {
            return None;
        }
        Some(Ditted { bits, dits, mark })
    }

    /// Length in bytes.
    pub fn len(&self) -> usize {
        self.bits.len()
    }

    /// `(val & dits[k]) == bits[k]` for every byte; `false` if `window` is shorter
    /// than the sequence (faithful to `DittedBitSequence.isMatch`).
    pub fn matches(&self, window: &[u8]) -> bool {
        if window.len() < self.bits.len() {
            return false;
        }
        for k in 0..self.bits.len() {
            if window[k] & self.dits[k] != self.bits[k] {
                return false;
            }
        }
        true
    }

    /// Count of fixed (non-ditted) bits — the upstream `getNumFixedBits`
    /// (`DittedBitSequence.java:258`). Used only for diagnostics/tests here.
    #[cfg(test)]
    pub fn num_fixed_bits(&self) -> u32 {
        self.dits.iter().map(|d| d.count_ones()).sum()
    }
}

fn hex_val(c: char) -> Option<u32> {
    c.to_digit(16)
}

// ===========================================================================
// The parsed pattern set: pre/post pairs + bare funcstart patterns, per arch
// ===========================================================================

/// One `<patternpairs>` ⇒ a set of pre-sequences and a set of post-sequences, all
/// carrying a `<funcstart/>` action. A candidate at `vma` is a function start iff
/// some `post` matches at `vma` AND some `pre` matches ending exactly at `vma`
/// (`PatternPairSet.createFinalPatterns`: the concat's mark is `pre.getSize()`).
struct PatternPairs {
    pre: Vec<Ditted>,
    post: Vec<Ditted>,
}

/// The compiled function-start pattern set for one architecture.
pub struct FuncStartPatterns {
    /// `<patternpairs>` whose post action is a bare `<funcstart/>`.
    pairs: Vec<PatternPairs>,
    /// Bare `<pattern>`s whose only action is an unconditional `<funcstart/>`
    /// (no `after`/`validcode`/`thunk`/`label`). Matched at any aligned offset.
    bare: Vec<Ditted>,
    /// Instruction alignment for the arch (`FunctionStartAnalyzer.applyActionToSet`
    /// rejects an unaligned mark: `addr % getInstructionAlignment() != 0`).
    align: u64,
}

impl FuncStartPatterns {
    /// Scan `data` (loaded at `sec_addr`) and return every VMA that is a function
    /// start under the full pattern set. Faithful to `FunctionStartAnalyzer`
    /// applied to a byte range:
    ///   - for each `<patternpairs>`: a post matches at the candidate offset AND a
    ///     pre matches the bytes immediately before it (the pre's bytes end at the
    ///     candidate),
    ///   - for each bare `<funcstart/>` pattern: the sequence matches at the
    ///     candidate offset,
    ///   - the candidate VMA must be instruction-aligned.
    ///
    /// Returns absolute VMAs (`sec_addr + offset`), sorted/deduped.
    pub fn scan(&self, sec_addr: u64, data: &[u8]) -> Vec<u64> {
        let mut out: Vec<u64> = Vec::new();
        let n = data.len();
        for off in 0..n {
            let vma = sec_addr + off as u64;
            if vma % self.align != 0 {
                continue;
            }
            let here = &data[off..];
            let mut hit = false;
            // Bare <funcstart/> patterns. A pattern may carry a `*` mark: the bytes
            // before the mark are *context* that must precede the candidate, and the
            // candidate is the marked byte. So the full sequence must match starting
            // at `off - mark`, placing the marked byte at `off`. With `mark == 0`
            // (no `*`) this is just "the sequence matches at the candidate".
            for b in &self.bare {
                if b.mark <= off && b.matches(&data[off - b.mark..]) {
                    hit = true;
                    break;
                }
            }
            // <patternpairs>: a post at the candidate + a pre ending at it.
            if !hit {
                'pairs: for pp in &self.pairs {
                    // Find a post matching at the candidate first (cheap reject).
                    if !pp.post.iter().any(|p| p.matches(here)) {
                        continue;
                    }
                    // Then require a pre matching the bytes immediately before.
                    for pre in &pp.pre {
                        let plen = pre.len();
                        if plen <= off {
                            let pre_slice = &data[off - plen..off];
                            if pre.matches(pre_slice) {
                                hit = true;
                                break 'pairs;
                            }
                        }
                    }
                }
            }
            if hit {
                out.push(vma);
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }

    /// True if the set has any usable pattern (so the pass can early-out).
    fn is_empty(&self) -> bool {
        self.pairs.is_empty() && self.bare.is_empty()
    }
}

// ===========================================================================
// XML parsing (a tiny tag walker — no XML crate dependency at this tier)
// ===========================================================================

/// Parse a vendored `<patternlist>` XML string into pre/post pairs + bare
/// funcstart sequences. A faithful-but-minimal walker: it does NOT need a general
/// XML parser (the pattern files are flat and machine-generated). It extracts:
///   - each `<patternpairs>…</patternpairs>` block → its `<prepatterns>` `<data>`
///     sequences and, when the `<postpatterns>` carries a `<funcstart/>` (and not
///     a `<possiblefuncstart/>`), its `<data>` post sequences;
///   - each top-level `<pattern>…</pattern>` whose body has a `<funcstart` (no
///     `after`/`validcode`/`thunk`/`label`/`section` attribute) and not a
///     `<possiblefuncstart` → its single `<data>` sequence as a bare pattern.
fn parse_patternlist(xml: &str, pairs: &mut Vec<PatternPairs>, bare: &mut Vec<Ditted>) {
    // Walk <patternpairs> blocks.
    let mut rest = xml;
    while let Some(open) = rest.find("<patternpairs") {
        let after_open = &rest[open..];
        let Some(close_rel) = after_open.find("</patternpairs>") else { break };
        let block = &after_open[..close_rel];
        parse_patternpairs(block, pairs);
        rest = &after_open[close_rel + "</patternpairs>".len()..];
    }
    // Walk top-level <pattern> … </pattern> blocks. The substring search also sees
    // the <patternpairs> open tags, so we skip any "<pattern" immediately followed
    // by 'p'/'l'/'c' (<patternpairs>/<patternlist>/<patternconstraints>); a real
    // <pattern> is followed by '>' or whitespace. The per-block guard additionally
    // rejects any block that contains <prepatterns> (a patternpairs body).
    let mut rest = xml;
    while let Some(open) = rest.find("<pattern") {
        let tail = &rest[open + "<pattern".len()..];
        let next_char = tail.chars().next().unwrap_or(' ');
        if next_char == 'p' || next_char == 'l' || next_char == 'c' {
            rest = &rest[open + "<pattern".len()..];
            continue;
        }
        let after_open = &rest[open..];
        let Some(close_rel) = after_open.find("</pattern>") else { break };
        let block = &after_open[..close_rel];
        parse_bare_pattern(block, bare);
        rest = &after_open[close_rel + "</pattern>".len()..];
    }
}

/// Parse one `<patternpairs>` block: collect the `<prepatterns>` `<data>` and, IFF
/// the `<postpatterns>` carries an unconditional `<funcstart/>` (not
/// `<possiblefuncstart/>`), the `<postpatterns>` `<data>` post sequences.
fn parse_patternpairs(block: &str, pairs: &mut Vec<PatternPairs>) {
    let pre = match (block.find("<prepatterns>"), block.find("</prepatterns>")) {
        (Some(a), Some(b)) if b > a => collect_data(&block[a + "<prepatterns>".len()..b]),
        _ => Vec::new(),
    };
    // A block may carry multiple <postpatterns> sub-blocks; handle all.
    let mut rest = block;
    while let Some(a) = rest.find("<postpatterns>") {
        let after = &rest[a + "<postpatterns>".len()..];
        let Some(b) = after.find("</postpatterns>") else { break };
        let post_block = &after[..b];
        if post_has_funcstart(post_block) {
            let post = collect_data(post_block);
            if !pre.is_empty() && !post.is_empty() {
                pairs.push(PatternPairs { pre: pre.clone(), post });
            }
        }
        rest = &after[b + "</postpatterns>".len()..];
    }
}

/// True if a `<postpatterns>`/`<pattern>` body carries an unconditional
/// `<funcstart` action with NO `after`/`validcode`/`thunk`/`label`/`section`
/// attribute, and is NOT a `<possiblefuncstart`. Note `body.find("<funcstart")`
/// never matches inside `<possiblefuncstart` (that tag has no `<funcstart`
/// substring), so a found `<funcstart` is already the strict action.
fn post_has_funcstart(body: &str) -> bool {
    let Some(fs) = body.find("<funcstart") else { return false };
    let tag_tail = &body[fs..];
    let Some(end) = tag_tail.find('>') else { return false };
    let tag = &tag_tail[..end];
    if tag.contains("after=")
        || tag.contains("validcode")
        || tag.contains("thunk")
        || tag.contains("label")
        || tag.contains("section")
        || tag.contains("noreturn")
    {
        return false;
    }
    true
}

/// Parse one bare `<pattern>` block: if it has an unconditional `<funcstart/>`
/// (and no `<prepatterns>`, i.e. it is a single-data pattern), add its `<data>`.
fn parse_bare_pattern(block: &str, bare: &mut Vec<Ditted>) {
    if block.contains("<prepatterns>") {
        return; // a patternpairs body that slipped through — skip.
    }
    if !post_has_funcstart(block) {
        return;
    }
    for d in collect_data(block) {
        bare.push(d);
    }
}

/// Collect every `<data> … </data>` sequence in `body`, parsed to a [`Ditted`].
/// Malformed sequences are dropped (faithful: Ghidra would throw and skip).
fn collect_data(body: &str) -> Vec<Ditted> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(a) = rest.find("<data>") {
        let after = &rest[a + "<data>".len()..];
        let Some(b) = after.find("</data>") else { break };
        let text = &after[..b];
        if let Some(d) = Ditted::parse(text) {
            out.push(d);
        }
        rest = &after[b + "</data>".len()..];
    }
    out
}

// ===========================================================================
// Per-architecture pattern set selection (the patternconstraints.xml analog)
// ===========================================================================

// The vendored upstream pattern XMLs (provenance: docs/UPSTREAM.md GHIDRA_REV).
// Embedded so the analyzer tier needs no spec-root file handle.
const X86_64_GCC: &str = include_str!("x86-64gcc_patterns.xml");
const X86_64_WIN: &str = include_str!("x86-64win_patterns.xml");
const X86_GCC: &str = include_str!("x86gcc_patterns.xml");
const X86_GCC_PRE: &str = include_str!("x86gcc_prepatterns.xml");
const X86_WIN: &str = include_str!("x86win_patterns.xml");
const AARCH64_LE: &str = include_str!("AARCH64_LE_patterns.xml");
const ARM_LE: &str = include_str!("ARM_LE_patterns.xml");
const ARM_BE: &str = include_str!("ARM_BE_patterns.xml");
const RISCV_GC: &str = include_str!("riscv_gc_patterns.xml");
const MIPS_LE: &str = include_str!("MIPS_LE_patterns.xml");
const MIPS_BE: &str = include_str!("MIPS_BE_patterns.xml");
const PPC_LE: &str = include_str!("PPC_LE_patterns.xml");
const PPC_BE: &str = include_str!("PPC_BE_patterns.xml");

/// Compile a set of vendored XMLs into one [`FuncStartPatterns`] with the given
/// instruction alignment.
fn compile(xmls: &[&str], align: u64) -> FuncStartPatterns {
    let mut pairs = Vec::new();
    let mut bare = Vec::new();
    for xml in xmls {
        parse_patternlist(xml, &mut pairs, &mut bare);
    }
    FuncStartPatterns { pairs, bare, align }
}

/// Return the compiled full pattern set for `arch` (little/big-endian aware), or
/// `None` if kuna vendors no *usable* pattern set for the architecture. Faithful
/// to the `patternconstraints.xml` language→patternfile dispatch (we select by the
/// `object` architecture + endianness, the analyzer-tier analog of the SLEIGH
/// language id). Cached per arch so the parse runs once.
///
/// **`None` also when the vendored XMLs parse to an empty set** — i.e. when an
/// arch's `<patternpairs>` carry only `<possiblefuncstart/>` (AArch64, PowerPC).
/// Those are a weaker hint Ghidra confirms later with the Listing, not a
/// byte-decidable function start, so they contribute nothing here (a documented
/// LOSS). The arches that yield a non-empty set are x86-64, i386, ARM, RISC-V,
/// and MIPS.
pub fn for_arch(
    arch: object::Architecture,
    little_endian: bool,
) -> Option<&'static FuncStartPatterns> {
    use object::Architecture as A;
    macro_rules! cached {
        ($cell:ident, $build:expr) => {{
            static $cell: OnceLock<FuncStartPatterns> = OnceLock::new();
            let set = $cell.get_or_init(|| $build);
            if set.is_empty() {
                None
            } else {
                Some(set)
            }
        }};
    }
    match arch {
        // x86-64: gcc + windows patterns, unioned (the bytes are x86 opcodes
        // either way; upstream selects by compiler id, but at the analyzer tier we
        // union so a stripped binary matches regardless of the unknown toolchain).
        // 1-byte instruction alignment (x86).
        A::X86_64 | A::X86_64_X32 => cached!(X8664, compile(&[X86_64_GCC, X86_64_WIN], 1)),
        // 32-bit x86: gcc (incl. the prepatterns file) + windows.
        A::I386 => cached!(X86, compile(&[X86_GCC, X86_GCC_PRE, X86_WIN], 1)),
        // AArch64: 4-byte fixed-width A64 instructions.
        A::Aarch64 | A::Aarch64_Ilp32 => cached!(AARCH64, compile(&[AARCH64_LE], 4)),
        // ARM/Thumb: 2-byte alignment covers Thumb (an A32 start is still 4-aligned
        // by its byte content; a 2-byte grid only widens the candidate set).
        A::Arm => cached!(ARMLE, compile(&[ARM_LE], 2)),
        // RISC-V: 2-byte alignment (the compressed C extension).
        A::Riscv32 | A::Riscv64 => cached!(RISCV, compile(&[RISCV_GC], 2)),
        // MIPS: 4-byte fixed-width; pick endianness.
        A::Mips | A::Mips64 => {
            if little_endian {
                cached!(MIPSLE, compile(&[MIPS_LE], 4))
            } else {
                cached!(MIPSBE, compile(&[MIPS_BE], 4))
            }
        }
        // PowerPC: 4-byte fixed-width; pick endianness.
        A::PowerPc | A::PowerPc64 => {
            if little_endian {
                cached!(PPCLE, compile(&[PPC_LE], 4))
            } else {
                cached!(PPCBE, compile(&[PPC_BE], 4))
            }
        }
        _ => {
            // Reference the otherwise-unused BE-ARM blob so a future BE-ARM arm can
            // pick it up without a dead-const warning; no current arch selects it.
            let _ = ARM_BE;
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ditted_hex_and_binary_parse() {
        // Hex: 0x534889fb -> four fully-fixed bytes.
        let d = Ditted::parse("0x534889fb").unwrap();
        assert_eq!(d.len(), 4);
        assert!(d.matches(&[0x53, 0x48, 0x89, 0xfb, 0x00]));
        assert!(!d.matches(&[0x53, 0x48, 0x89, 0xfc]));
        assert_eq!(d.num_fixed_bits(), 32);

        // Ditted nibble: 0x0....000 (low 3 bits zero, bit7 zero on a byte).
        let d = Ditted::parse("0x53 0x48 0x83 0xec 0....000").unwrap();
        assert_eq!(d.len(), 5);
        // push rbx; sub rsp, 0x18 (0x18 = 0001_1000: bit7=0, low3=000) → match.
        assert!(d.matches(&[0x53, 0x48, 0x83, 0xec, 0x18]));
        // sub rsp, 0x19 (low bit set) → no match (low 3 bits must be 000).
        assert!(!d.matches(&[0x53, 0x48, 0x83, 0xec, 0x19]));

        // Binary token: "01010101" = 0x55.
        let d = Ditted::parse("01010101").unwrap();
        assert_eq!(d.len(), 1);
        assert!(d.matches(&[0x55]));
        assert!(!d.matches(&[0x54]));

        // Mixed binary don't-care: "0101.1.." matches a class of bytes.
        let d = Ditted::parse("0101.1..").unwrap();
        assert!(d.matches(&[0b0101_1100]));
        assert!(!d.matches(&[0b0100_1100]));

        // 8-byte NOP prepattern from the vendored set.
        let d = Ditted::parse("0x0f1f840000000000").unwrap();
        assert_eq!(d.len(), 8);
        assert!(d.matches(&[0x0f, 0x1f, 0x84, 0, 0, 0, 0, 0]));
    }

    #[test]
    fn x86_64_set_parses_and_has_pairs() {
        let set = for_arch(object::Architecture::X86_64, true).expect("x86-64 patterns");
        assert!(!set.pairs.is_empty(), "x86-64 must have <patternpairs>");
        // The PUSH RBX; MOV RBX,RDI postpattern (0x534889fb) is in the gcc set.
        let has = set
            .pairs
            .iter()
            .any(|pp| pp.post.iter().any(|p| p.matches(&[0x53, 0x48, 0x89, 0xfb])));
        assert!(has, "0x534889fb post pattern must be present");
    }

    #[test]
    fn x86_64_pairs_catch_push_rbx_after_nop() {
        // Simulate widget@0x1130: 8-byte NOP prepattern ending at the candidate,
        // then the PUSH RBX; MOV RBX,RDI postpattern at it. Aligned at 0x1130.
        let set = for_arch(object::Architecture::X86_64, true).unwrap();
        let mut data = vec![0u8; 0x20];
        // place the 8-byte nop at 0x08..0x10 (so it ends at offset 0x10).
        data[0x08..0x10].copy_from_slice(&[0x0f, 0x1f, 0x84, 0, 0, 0, 0, 0]);
        // widget prologue at 0x10.
        data[0x10..0x18].copy_from_slice(&[0x53, 0x48, 0x89, 0xfb, 0x48, 0x8d, 0x04, 0x1b]);
        let hits = set.scan(0x1120, &data);
        assert!(hits.contains(&0x1130), "push rbx after nop should be a start, got {hits:#x?}");
    }

    #[test]
    fn no_match_without_prepattern() {
        // Same postpattern but NO prepattern before it (random non-pre bytes) →
        // the patternpair must NOT fire (the prepattern gate is load-bearing).
        let set = for_arch(object::Architecture::X86_64, true).unwrap();
        let mut data = vec![0xABu8; 0x20];
        data[0x10..0x18].copy_from_slice(&[0x53, 0x48, 0x89, 0xfb, 0x48, 0x8d, 0x04, 0x1b]);
        let hits = set.scan(0x1120, &data);
        assert!(!hits.contains(&0x1130), "no prepattern → no patternpair hit, got {hits:#x?}");
    }

    #[test]
    fn star_mark_offset_is_parsed_and_applied() {
        // x86-64win bare pattern `0x909090 * 0x4883ec`: 3 NOPs (context) then SUB
        // RSP at the mark. The mark byte is the function start; the 3 NOPs must
        // precede it.
        let d = Ditted::parse("0x909090 * 0x4883ec").unwrap();
        assert_eq!(d.len(), 6, "3 nop + 3 sub bytes");
        assert_eq!(d.mark, 3, "the * marks the 4th byte (offset 3)");

        // A direct scan: NOP NOP NOP SUB RSP at 0x..3 → the start is at the SUB.
        let set = for_arch(object::Architecture::X86_64, true).unwrap();
        // data: [90 90 90 48 83 ec ..]; the candidate start is at offset 3.
        let mut data = vec![0u8; 0x10];
        data[0..6].copy_from_slice(&[0x90, 0x90, 0x90, 0x48, 0x83, 0xec]);
        let hits = set.scan(0x1000, &data);
        assert!(
            hits.contains(&0x1003),
            "the *-marked SUB RSP after 3 NOPs is the start (0x1003), got {hits:#x?}"
        );
    }

    #[test]
    fn arches_with_unconditional_funcstart_have_a_set() {
        // Only arches whose vendored `<patternpairs>` carry an unconditional
        // `<funcstart/>` (high-confidence) yield a non-empty set; the others use
        // `<possiblefuncstart/>` (a weaker hint Ghidra confirms later — a
        // documented LOSS at this disassembler-free tier). x86-64/ARM/RISC-V/MIPS
        // carry `<funcstart/>`; AArch64/PPC/x86-32 carry only `<possiblefuncstart/>`.
        for (a, le) in [
            (object::Architecture::Riscv64, true),
            (object::Architecture::Mips, false),
            (object::Architecture::Mips, true),
            (object::Architecture::Arm, true),
            // I386: the vendored x86win patterns carry bare `<funcstart/>`
            // patternpairs + `*`-marked bare patterns (the x86-64win MSVC shapes).
            (object::Architecture::I386, true),
        ] {
            assert!(for_arch(a, le).is_some(), "{a:?} should have a <funcstart/> pattern set");
        }
        // AArch64/PPC carry only `<possiblefuncstart/>` patternpairs (+ conditioned
        // bare patterns) ⇒ no unconditional funcstart ⇒ no set (None), faithfully
        // (a documented LOSS at this disassembler-free tier, not a bug).
        for (a, le) in [
            (object::Architecture::Aarch64, true),
            (object::Architecture::PowerPc64, false),
            (object::Architecture::PowerPc64, true),
        ] {
            assert!(
                for_arch(a, le).is_none(),
                "{a:?} has only <possiblefuncstart/> patternpairs ⇒ no funcstart set"
            );
        }
    }
}
