//! (kuna) `addrtakenentry` — code whose address the program takes, and that no
//! flow reaches, is a function entry (P1 code/data partition, x86-64).
//!
//! The Listing walk makes a function only at a CALL target. A function whose
//! address is taken — `lea rax,[rip+handler]` stored to a global, `mov
//! edi,offset handler` passed to a registration call — and that is only ever
//! called through that pointer is reached by no flow at all: the walk never
//! decodes it, and the extent bounder folds its bytes into the function before
//! it. On an x64 PE a frame-less leaf also has no `.pdata` record, so no
//! load-time oracle names it either.
//!
//! Two kinds of evidence name such an address:
//!
//! 1. an instruction the walk decoded materializes it: a RIP-relative `lea`, a
//!    `mov` of an immediate into a register or memory, or a `push` of an
//!    immediate;
//! 2. on a PE, a base-relocated word holds it and it lies before the first
//!    `RUNTIME_FUNCTION` of its section — the span the relocation oracle's
//!    `.pdata` hull guard leaves out (see `entry::kuna_pereloccode`).
//!
//! Either way the address may still be a label or data, so a candidate becomes
//! an entry only when all of these hold:
//!
//! - it is undecoded executable memory: no decoded instruction covers it, so no
//!   flow the walk followed reaches it;
//! - it starts where a function starts: the bytes in front of it, back to the
//!   last decoded instruction (or one this pass accepted, or the start of its
//!   section), are padding or zero fill, and that instruction does not fall
//!   through;
//! - it is code that ends like a function: decoded inside its gap, every path
//!   ends in a return, a computed jump, a trap (`int3`, `ud2`), a call followed
//!   by padding, or a jump to a known function entry; no path falls out of the
//!   gap into other code or jumps into the middle of decoded code; no `ret` is
//!   reached with the stack above the entry and no `leave` without a frame
//!   pointer the path set (a frame torn down that the routine never built); it
//!   does not open with `int3`, and no instruction of it is zero fill or `hlt`
//!   — the
//!   shape of a data table in `.text` (a MinGW delay-import descriptor decodes
//!   to `add [rax],eax ; add [rax],al ; ... ; ret 0`);
//! - it is not a label of a function that takes it: inside a taker's region
//!   that holds a register- or memory-indirect `jmp`, an address before more of
//!   the region's code, or one whose own code dispatches, is a computed-`goto`
//!   or threaded-interpreter label, not a registered function.
//!
//! The accepted entries re-seed the walk, so their callees and the addresses
//! their own code takes are found too; acceptance repeats over the rebuilt
//! Listing for at most [`MAX_ROUNDS`] rounds.
//!
//! x86-64 only. On i386 every address is an absolute immediate, and the same
//! shape names non-functions: a MinGW SJLJ landing pad sits after its
//! function's `ret` and alignment padding, and the function stores its address
//! with `mov [ebp-0x1c],imm` — measured, 40 i386 PEs gained 210 true starts,
//! lost 574 and grew 4,974 mid-body entries.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use kuna_base::space::AddrSpace;
use kuna_sleigh::translate::Translate;
use object::read::Object;

use crate::listing::{FlowKind, Listing};

use super::kuna_aifnoppad::{is_padding, is_zero_fill};
use super::{GapDecoder, ProbedInsn, MAX_FOLLOW_INSNS};

/// How many re-walks the accepted entries may trigger.
pub(crate) const MAX_ROUNDS: usize = 4;

/// The longest filler run read in front of a candidate.
const MAX_PADDING_INSNS: usize = 64;

/// The evidence that does not change between rounds: the executable bytes the
/// instruction scan reads, and a PE's pre-record relocated code pointers.
pub(crate) struct AddrTaken {
    texts: Vec<(u64, u64, Vec<u8>)>,
    relocated: Vec<u64>,
}

impl AddrTaken {
    /// `None` off x86-64, and on a Go binary: its `pclntab` already names every
    /// function, so a candidate could only re-seed a known one, and the wider walk
    /// then turns the calls into the middle of `runtime.duffzero` into entries.
    pub(crate) fn new(file: &object::File, bytes: &[u8]) -> Option<AddrTaken> {
        if file.architecture() != object::Architecture::X86_64
            || crate::sourcelang::effective_compiler(file, bytes).is_golang()
        {
            return None;
        }
        let texts = crate::entry::executable_sections(file);
        let relocated = crate::entry::kuna_pereloccode::pre_record_code_pointers(bytes, &texts);
        Some(AddrTaken { texts, relocated })
    }

    /// The candidates over `listing`, each with the decoded instructions that
    /// take it: every undecoded executable address a decoded instruction
    /// materializes, and every pre-record relocated code pointer (taken by none).
    /// Also returns the decoded register- or memory-indirect `jmp`s, sorted.
    fn candidates(&self, listing: &Listing) -> (BTreeMap<u64, Vec<u64>>, Vec<u64>) {
        let mut out: BTreeMap<u64, Vec<u64>> = self
            .relocated
            .iter()
            .filter(|&&t| listing.is_undefined(t))
            .map(|&t| (t, Vec::new()))
            .collect();
        let mut dispatches = Vec::new();
        for (&at, insn) in listing.instructions() {
            let Some(bytes) = self.bytes(at, insn.len) else { continue };
            if is_indirect_jmp(bytes) {
                dispatches.push(at);
            } else if let Some(target) = materialized(at, bytes) {
                if target != at.wrapping_add(u64::from(insn.len)) && listing.is_undefined(target) {
                    out.entry(target).or_default().push(at);
                }
            }
        }
        (out, dispatches)
    }

    fn bytes(&self, at: u64, len: u32) -> Option<&[u8]> {
        let (lo, _, data) = self.texts.iter().find(|&&(lo, hi, _)| at >= lo && at < hi)?;
        let off = usize::try_from(at - lo).ok()?;
        data.get(off..off.checked_add(len as usize)?)
    }

    /// The candidates of `listing` that pass every guard, address-sorted.
    pub(crate) fn accept(
        &self,
        listing: &Listing,
        translate: &dyn Translate,
        code_space: Rc<AddrSpace>,
    ) -> Vec<u64> {
        let (candidates, dispatches) = self.candidates(listing);
        if candidates.is_empty() {
            return Vec::new();
        }
        let ranges = listing.exec_ranges();
        let mut decoder = GapDecoder::new(translate, code_space, ranges);
        let mut claimed: BTreeMap<u64, Claimed> = BTreeMap::new();
        let mut accepted: BTreeSet<u64> = BTreeSet::new();
        for (target, takers) in candidates {
            let Some(&(lo, hi)) = ranges.iter().find(|&&(lo, hi)| target >= lo && target < hi) else {
                continue;
            };
            let label = own_label(listing, &dispatches, target, &takers);
            if splits_claimed(&claimed, target) || label == Some(Label::Between) {
                continue;
            }
            let Some(padded) = isolated(listing, &mut decoder, &claimed, lo, target) else {
                continue;
            };
            let gap_hi = listing.next_instruction_start_after(target).map_or(hi, |n| n.min(hi));
            let Some(body) = routine(listing, &mut decoder, &accepted, target, gap_hi, padded) else {
                continue;
            };
            let dispatches_again = || {
                body.iter().any(|(&at, c)| self.bytes(at, (c.end - at) as u32).is_some_and(is_indirect_jmp))
            };
            if label == Some(Label::After) && dispatches_again() {
                continue;
            }
            accepted.insert(target);
            claimed.extend(body);
        }
        accepted.into_iter().collect()
    }
}

/// One instruction of an accepted routine: where it ends, and whether it falls
/// through.
#[derive(Clone, Copy)]
struct Claimed {
    end: u64,
    falls: bool,
}

/// The address the decoded x86-64 instruction `bytes` at `at` materializes as a
/// value, when it is a RIP-relative `lea`, a `mov` of an immediate, or a `push`
/// of an immediate. Operand-size overrides narrow the value below an address and
/// are refused.
fn materialized(at: u64, bytes: &[u8]) -> Option<u64> {
    let mut i = 0;
    while let Some(&prefix) = bytes.get(i) {
        match prefix {
            0x66 | 0x67 => return None,
            0xF0 | 0xF2 | 0xF3 | 0x26 | 0x2E | 0x36 | 0x3E | 0x64 | 0x65 => i += 1,
            _ => break,
        }
    }
    let mut rex_w = false;
    if bytes.get(i).is_some_and(|&rex| rex & 0xF0 == 0x40) {
        rex_w = bytes[i] & 0x08 != 0;
        i += 1;
    }
    let len = bytes.len();
    let tail32 = || Some(u32::from_le_bytes(bytes.get(len.checked_sub(4)?..)?.try_into().ok()?));
    match *bytes.get(i)? {
        0x8D => {
            if bytes.get(i + 1)? & 0xC7 != 0x05 || len != i + 6 {
                return None;
            }
            let disp = tail32()? as i32 as i64 as u64;
            let target = at.wrapping_add(len as u64).wrapping_add(disp);
            Some(if rex_w { target } else { target & 0xFFFF_FFFF })
        }
        0xB8..=0xBF if rex_w => {
            let imm = bytes.get(i + 1..)?;
            (imm.len() == 8).then(|| u64::from_le_bytes(imm.try_into().unwrap()))
        }
        0xB8..=0xBF => (len == i + 5).then(|| tail32().map(u64::from)).flatten(),
        0xC7 => {
            if (bytes.get(i + 1)? >> 3) & 7 != 0 {
                return None;
            }
            let imm = tail32()?;
            Some(if rex_w { imm as i32 as i64 as u64 } else { u64::from(imm) })
        }
        0x68 => {
            if len != i + 5 {
                return None;
            }
            let imm = tail32()?;
            Some(imm as i32 as i64 as u64)
        }
        _ => None,
    }
}

/// Where a candidate sits in a function that takes it and dispatches through a
/// register- or memory-indirect `jmp` (see [`own_label`]).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Label {
    /// Before more of that function's decoded code: one of its blocks.
    Between,
    /// After all of it: one of its blocks if the candidate's own code dispatches;
    /// a function the `jmp` tail-calls if it does not.
    After,
}

/// Whether `target` may be a label of a function that takes it: the target lies
/// in a taker's function region (up to the next known entry) and the region holds
/// a register- or memory-indirect `jmp` (`dispatches`). That is the computed-goto
/// and threaded-interpreter shape — `lea rax,[label]` and later `jmp rax` — whose
/// labels sit between the function's blocks (clang `-O0`) or after the dispatch
/// jump (gcc), fall through into nothing and end in another dispatch, a jump back
/// or a return, as a function would. A lazy-binding stub stores the address of a
/// fallback that follows it and tail-calls through `jmp rax` too, so after the
/// function's code only a candidate that dispatches itself is refused.
fn own_label(listing: &Listing, dispatches: &[u64], target: u64, takers: &[u64]) -> Option<Label> {
    let next = listing.next_instruction_start_after(target);
    let mut label = None;
    for &taker in takers {
        let Some(entry) = listing.function_containing(taker).map(|f| f.entry) else { continue };
        let hi = listing.next_function_after(entry).map_or(u64::MAX, |f| f.entry);
        if target < entry || target >= hi {
            continue;
        }
        let first = dispatches.partition_point(|&at| at < entry);
        if !dispatches.get(first).is_some_and(|&at| at < hi) {
            continue;
        }
        if next.is_some_and(|at| at < hi) {
            return Some(Label::Between);
        }
        label = Some(Label::After);
    }
    label
}

/// Whether the decoded instruction `bytes` is a register- or memory-indirect
/// near `jmp` (`ff /4`), after any prefixes and a REX byte, through anything but
/// a fixed RIP-relative slot: `jmp [rip+slot]` is a tail call through an import
/// or a pointer, not a dispatch.
fn is_indirect_jmp(bytes: &[u8]) -> bool {
    let mut i = 0;
    while bytes.get(i).is_some_and(|b| matches!(b, 0x66 | 0x67 | 0xF0 | 0xF2 | 0xF3 | 0x26 | 0x2E | 0x36 | 0x3E | 0x64 | 0x65)) {
        i += 1;
    }
    if bytes.get(i).is_some_and(|&rex| rex & 0xF0 == 0x40) {
        i += 1;
    }
    bytes.get(i) == Some(&0xFF) && bytes.get(i + 1).is_some_and(|&m| (m >> 3) & 7 == 4 && m & 0xC7 != 0x05)
}

/// Whether `target` lies strictly inside an instruction a routine accepted this
/// round.
fn splits_claimed(claimed: &BTreeMap<u64, Claimed>, target: u64) -> bool {
    claimed.range(..target).next_back().is_some_and(|(_, c)| c.end > target)
}

/// Whether `target` starts where a function starts: the instruction before it
/// (decoded, or accepted this round, inside its range) does not fall through
/// and only filler lies between the two; with no instruction before it, only
/// filler lies between the range start and `target`. Returns whether any filler
/// was read.
fn isolated(
    listing: &Listing,
    decoder: &mut GapDecoder,
    claimed: &BTreeMap<u64, Claimed>,
    lo: u64,
    target: u64,
) -> Option<bool> {
    let decoded = listing
        .instruction_before(target)
        .filter(|insn| insn.addr >= lo)
        .map(|insn| (insn.addr, Claimed {
            end: insn.addr.wrapping_add(u64::from(insn.len)),
            falls: insn.fall_through.is_some(),
        }));
    let overlay = claimed.range(lo..target).next_back().map(|(&at, &c)| (at, c));
    let mut at = match decoded.into_iter().chain(overlay).max_by_key(|&(at, _)| at) {
        Some((_, prev)) if prev.end > target || prev.falls => return None,
        Some((_, prev)) => prev.end,
        None => lo,
    };
    let start = at;
    for _ in 0..=MAX_PADDING_INSNS {
        if at == target {
            return Some(at != start);
        }
        let insn = decoder.probe(at)?;
        if !is_padding(&insn, true) && !is_zero_fill(&insn) {
            return None;
        }
        at = at.checked_add(u64::from(insn.len)).filter(|&next| next <= target)?;
    }
    None
}

/// The instructions of the routine at `target`, decoded inside `[target, gap_hi)`,
/// when every path ends like a function's; `None` when one does not.
///
/// `padded` says filler precedes `target`: a candidate that is itself filler is
/// then the middle of that run, not a function opening with a `nop`.
fn routine(
    listing: &Listing,
    decoder: &mut GapDecoder,
    accepted: &BTreeSet<u64>,
    target: u64,
    gap_hi: u64,
    padded: bool,
) -> Option<BTreeMap<u64, Claimed>> {
    let first = decoder.probe(target)?;
    if is_zero_fill(&first) || is_trap(&first) || (padded && is_padding(&first, true)) {
        return None;
    }
    let is_entry = |at: u64| listing.function_at(at).is_some() || accepted.contains(&at);
    let mut body: BTreeMap<u64, Claimed> = BTreeMap::new();
    let mut work: Vec<(u64, bool, Stack)> = vec![(target, false, Stack { depth: Some(0), frame: Frame::None })];
    while let Some((at, after_call, stack)) = work.pop() {
        if body.contains_key(&at) {
            continue;
        }
        if body.len() >= MAX_FOLLOW_INSNS {
            return None;
        }
        let insn = decoder.probe(at)?;
        if after_call && (is_padding(&insn, true) || is_trap(&insn)) {
            continue;
        }
        if is_zero_fill(&insn) || insn.mnemonic.eq_ignore_ascii_case("hlt") {
            return None;
        }
        let end = at.checked_add(u64::from(insn.len)).filter(|&end| end <= gap_hi)?;
        body.insert(at, Claimed { end, falls: insn.fall_through.is_some() });
        if insn.is_terminal && stack.depth.is_some_and(|d| d > 0) {
            return None;
        }
        let stack = stack.step(&insn)?;
        if insn.is_terminal || insn.kind == FlowKind::ComputedJump || is_trap(&insn) {
            continue;
        }
        for &to in &insn.flows {
            if insn.is_call {
                if !decoder.in_exec(to) {
                    return None;
                }
            } else if to >= target && to < gap_hi {
                work.push((to, false, stack));
            } else if !is_entry(to) {
                return None;
            }
        }
        if let Some(fall) = insn.fall_through {
            if fall >= gap_hi {
                return None;
            }
            work.push((fall, insn.is_call, stack));
        }
    }
    let mut last_end = 0;
    for (&at, c) in &body {
        if at < last_end {
            return None;
        }
        last_end = c.end;
    }
    Some(body)
}

/// What a path through a candidate routine knows about its stack: how far `RSP`
/// is above the entry (`None` once it is written in a way not followed), and
/// whether the path set the frame pointer.
#[derive(Clone, Copy)]
struct Stack {
    depth: Option<i64>,
    frame: Frame,
}

/// Whether a path set `RBP` as a frame pointer, and at what depth.
#[derive(Clone, Copy)]
enum Frame {
    None,
    At(Option<i64>),
}

impl Stack {
    /// The stack after `insn`, or `None` when `insn` is a `leave` on a path that
    /// never set the frame pointer: like a `ret` above the entry, that tears down
    /// a frame the routine did not build.
    fn step(self, insn: &ProbedInsn) -> Option<Stack> {
        let m = &*insn.mnemonic;
        let writes_rbp = insn.operands.trim_start().to_ascii_lowercase().starts_with("rbp,");
        if m.eq_ignore_ascii_case("leave") {
            let Frame::At(at) = self.frame else { return None };
            return Some(Stack { depth: at.map(|d| d + 8), frame: Frame::None });
        }
        let frame = if m.eq_ignore_ascii_case("enter") {
            Frame::At(self.depth.map(|d| d - 8))
        } else if m.eq_ignore_ascii_case("pop") && insn.operands.trim().eq_ignore_ascii_case("rbp") {
            Frame::None
        } else if writes_rbp && !m.eq_ignore_ascii_case("cmp") && !m.eq_ignore_ascii_case("test") {
            let mov_rsp = m.eq_ignore_ascii_case("mov")
                && insn.operands.chars().filter(|c| !c.is_whitespace()).collect::<String>().eq_ignore_ascii_case("rbp,rsp");
            Frame::At(if mov_rsp { self.depth } else { None })
        } else {
            self.frame
        };
        let depth = if m.eq_ignore_ascii_case("enter") {
            None
        } else {
            self.depth.and_then(|d| stack_step(insn).map(|step| d + step))
        };
        Some(Stack { depth, frame })
    }
}

/// How far `insn` moves the stack pointer up (a pop is `+8`), `Some(0)` when it
/// does not write `RSP`, and `None` when it writes it in a way not followed
/// (`mov rsp,r11`, `and rsp,-16`). A routine whose `ret` is reached with the
/// stack above its entry tore down a frame it never built: it is the epilogue of
/// the function around it, not a function.
fn stack_step(insn: &ProbedInsn) -> Option<i64> {
    let m = &*insn.mnemonic;
    let operands: String = insn.operands.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_ascii_lowercase();
    if m.eq_ignore_ascii_case("push") || m.eq_ignore_ascii_case("pushfq") {
        return Some(-8);
    }
    if m.eq_ignore_ascii_case("pop") || m.eq_ignore_ascii_case("popfq") {
        return Some(8);
    }
    let Some(source) = operands.strip_prefix("rsp,") else {
        return Some(0);
    };
    let imm = |text: &str| {
        let (negative, digits) = text.strip_prefix('-').map_or((false, text), |rest| (true, rest));
        let value = i64::from_str_radix(digits.strip_prefix("0x")?, 16).ok()?;
        Some(if negative { -value } else { value })
    };
    if m.eq_ignore_ascii_case("add") {
        imm(source)
    } else if m.eq_ignore_ascii_case("sub") {
        imm(source).map(|v| -v)
    } else if m.eq_ignore_ascii_case("lea") {
        imm(source.strip_prefix("[rsp+")?.strip_suffix(']')?)
    } else if m.eq_ignore_ascii_case("cmp") || m.eq_ignore_ascii_case("test") {
        Some(0)
    } else {
        None
    }
}

/// `int3` and `ud2`: flow does not continue past them.
fn is_trap(insn: &ProbedInsn) -> bool {
    ["int3", "ud2"].iter().any(|m| insn.mnemonic.eq_ignore_ascii_case(m))
}

#[cfg(test)]
mod tests {
    use super::materialized;

    #[test]
    fn rip_relative_lea_names_its_target() {
        // lea rax,[rip+0x9] at 0x140001030 (the GH-1039 registration stub)
        let lea = [0x48, 0x8d, 0x05, 0x09, 0x00, 0x00, 0x00];
        assert_eq!(materialized(0x1_4000_1030, &lea), Some(0x1_4000_1040));
        // lea rcx,[rip-0x10]
        let back = [0x48, 0x8d, 0x0d, 0xf0, 0xff, 0xff, 0xff];
        assert_eq!(materialized(0x1000, &back), Some(0x1007 - 0x10));
    }

    #[test]
    fn register_based_lea_names_nothing() {
        // lea eax,[rcx+rcx*4]
        assert_eq!(materialized(0x1000, &[0x8d, 0x04, 0x89]), None);
        // lea rax,[rbx+0x10]
        assert_eq!(materialized(0x1000, &[0x48, 0x8d, 0x43, 0x10]), None);
    }

    #[test]
    fn immediates_are_read_at_their_width() {
        // mov edi,0x401136
        assert_eq!(materialized(0x1000, &[0xbf, 0x36, 0x11, 0x40, 0x00]), Some(0x401136));
        // movabs rax,0x140001040
        let movabs = [0x48, 0xb8, 0x40, 0x10, 0x00, 0x40, 0x01, 0x00, 0x00, 0x00];
        assert_eq!(materialized(0x1000, &movabs), Some(0x1_4000_1040));
        // mov qword [rip+0x100],0x401136
        let store = [0x48, 0xc7, 0x05, 0x00, 0x01, 0x00, 0x00, 0x36, 0x11, 0x40, 0x00];
        assert_eq!(materialized(0x1000, &store), Some(0x401136));
        // push 0x401000
        assert_eq!(materialized(0x1000, &[0x68, 0x00, 0x10, 0x40, 0x00]), Some(0x401000));
        // lea eax,[rip+0x10]: a 32-bit lea keeps the low half
        assert_eq!(materialized(0x1_0000_1000, &[0x8d, 0x05, 0x10, 0x00, 0x00, 0x00]), Some(0x1016));
    }

    fn insn(mnemonic: &str, operands: &str) -> super::ProbedInsn {
        super::ProbedInsn {
            len: 1, kind: crate::listing::FlowKind::Fallthrough, is_call: false, is_terminal: false,
            fall_through: None, flows: Vec::new(), mnemonic: std::rc::Rc::from(mnemonic),
            operands: operands.to_string(), reusable: None,
        }
    }

    fn walk(insns: &[(&str, &str)]) -> Option<Option<i64>> {
        let mut stack = super::Stack { depth: Some(0), frame: super::Frame::None };
        for (m, o) in insns {
            stack = stack.step(&insn(m, o))?;
        }
        Some(stack.depth)
    }

    /// gcc `-O0`'s computed-goto exit label reads the frame and `leave`s it
    /// without having built it; a function that builds its frame balances.
    #[test]
    fn a_frame_torn_down_but_never_built_is_refused() {
        assert_eq!(walk(&[("MOV", "RAX,qword ptr [RBP + -0x30]"), ("LEAVE", "")]), None);
        assert_eq!(walk(&[("POP", "RBP")]), Some(Some(8)));
        assert_eq!(walk(&[("ADD", "RSP,0x38")]), Some(Some(0x38)));
        assert_eq!(
            walk(&[("PUSH", "RBP"), ("MOV", "RBP,RSP"), ("SUB", "RSP,0x20"), ("LEAVE", "")]),
            Some(Some(0))
        );
        assert_eq!(walk(&[("SUB", "RSP,0x28"), ("ADD", "RSP,0x28")]), Some(Some(0)));
    }

    #[test]
    fn indirect_jumps_are_read_off_the_opcode() {
        assert!(super::is_indirect_jmp(&[0xff, 0xe0])); // jmp rax
        assert!(super::is_indirect_jmp(&[0x41, 0xff, 0xe3])); // jmp r11
        assert!(super::is_indirect_jmp(&[0x3e, 0xff, 0xe0])); // notrack jmp rax
        assert!(super::is_indirect_jmp(&[0xff, 0x24, 0xc5, 0, 0, 0, 0])); // jmp [rax*8+disp]
        assert!(!super::is_indirect_jmp(&[0xff, 0xd0])); // call rax
        assert!(!super::is_indirect_jmp(&[0x0f, 0x0b])); // ud2
        assert!(!super::is_indirect_jmp(&[0xe9, 0, 0, 0, 0])); // jmp rel32
        assert!(!super::is_indirect_jmp(&[0x48, 0xff, 0x25, 0, 0, 0, 0])); // jmp [rip+slot]
    }

    #[test]
    fn narrowed_and_unrelated_forms_name_nothing() {
        // mov ax,0x1234
        assert_eq!(materialized(0x1000, &[0x66, 0xb8, 0x34, 0x12]), None);
        // xbegin (c7 f8 rel32)
        assert_eq!(materialized(0x1000, &[0xc7, 0xf8, 0x00, 0x00, 0x00, 0x00]), None);
        // call rel32
        assert_eq!(materialized(0x1000, &[0xe8, 0x00, 0x00, 0x00, 0x00]), None);
    }
}
